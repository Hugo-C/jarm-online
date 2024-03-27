use std::{env, io};
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use rocket_db_pools::Connection as RocketConnection;
use rocket_db_pools::deadpool_redis::redis::{AsyncCommands, RedisError};
use serde::Serialize;
use tempfile::NamedTempFile;
use crate::rocket::futures::StreamExt;

use crate::Db;
use crate::deadpool_redis::Connection;

// Env var key
const FORCE_TRANCO_TOP1M_RAW_DATA_PATH: &str = "FORCE_TRANCO_TOP1M_RAW_DATA_PATH";
const TRANCO_TOP1M_S3_URL: &str = "TRANCO_TOP1M_S3_URL";

// Redis keys
const TRANCO_TOP_1M_JARM_PREFIX_KEY: &str = "trancotop1m:jarm:";
const TRANCO_TOP_1M_INITIALIZED_KEY: &str = "trancotop1m:initialized";

#[derive(PartialEq, Serialize, Clone, Debug)]
pub struct RankedDomain {
    pub rank: u64,
    pub domain: String,
}

pub struct TrancoTop1M {
    redis_client: Connection,
}

impl TrancoTop1M {
    pub fn new(redis_client: Connection) -> TrancoTop1M {
        TrancoTop1M {
            redis_client
        }
    }

    pub fn from(redis_client: RocketConnection<Db>) -> TrancoTop1M {
        TrancoTop1M {
            redis_client: redis_client.into_inner()
        }
    }

    pub async fn get(&mut self, jarm_hash: String) -> Vec<RankedDomain> {
        let key = format!("{TRANCO_TOP_1M_JARM_PREFIX_KEY}:{jarm_hash}");
        // Fetch all keys
        let values: Vec<String> = self.redis_client.lrange(key, 0, -1).await.unwrap();
        let mut res = vec![];
        for value in values {
            let (rank_as_str, domain_as_str) = value.split_once('#').unwrap();
            let rank = rank_as_str.parse::<u64>().unwrap();
            let domain = domain_as_str.to_string();
            res.push(RankedDomain { rank, domain })
        }
        res
    }

    pub async fn is_initialized(&mut self) -> bool {
        let result: bool = self.redis_client.exists(TRANCO_TOP_1M_INITIALIZED_KEY).await.unwrap();
        result
    }

    pub async fn initialize(&mut self) {
        if self.is_initialized().await {
            info!("Tranco already initialized, skipping.");
            return;
        }
        // tmp file get cleared once out of scope, so we need a var to hold it
        let mut downloaded_s3_file = NamedTempFile::new().unwrap();

        let path = if let Ok(path) = env::var(FORCE_TRANCO_TOP1M_RAW_DATA_PATH) {
            info!("Forcing tranco initialisation on {path}");
            path
        } else {
            let url = match env::var(TRANCO_TOP1M_S3_URL) {
                Ok(url) => url,
                Err(_) => {
                    warn!("TRANCO_TOP1M_S3_URL env var has to be set for tranco initialization");
                    warn!("Skipping tranco initialization");
                    return;
                }
            };
            info!("Downloading tranco initialisation file");
            let download_path = downloaded_s3_file.path();
            let path = download_path.to_str().unwrap().to_string();
            info!("Using temporary file path: {path}");

            match Self::download_top_1m_file(url, downloaded_s3_file.as_file_mut()).await {
                Ok(_) => info!("Tranco top 1M file downloaded successfully!"),
                Err(err) => {
                    sentry::capture_error(&err);
                    error!("Failed to download tranco file, aborting.");
                    return;
                }
            }
            path
        };
        self.destroy_db().await.unwrap();
        let count = match self.add_domains_from_path(path).await {
            Ok(count) => count,
            Err(_) => {
                error!("Failed to add tranco hashes from path");
                return;
            }
        };
        match count {
            0..=10 => {
                error!("Only {count} values found during initialisation, most likely something went wrong.");
                error!("Initialisation key will not be set so as to retry on next start.");
            }
            _ => {
                let _: () = self.redis_client.set(TRANCO_TOP_1M_INITIALIZED_KEY, 1).await.unwrap();
                info!("Tranco DB successfully initialized, {count} website's jarm hashes loaded");
            }
        }
    }

    async fn download_top_1m_file(url: String, file: &mut File) -> Result<(), reqwest::Error> {
        let mut byte_stream = reqwest::get(url).await?.bytes_stream();

        while let Some(item) = byte_stream.next().await {
            io::copy(&mut item?.as_ref(), file).unwrap();
        }
        Ok(())
    }

    /// Remove all keys related to Tranco in the Redis DB
    pub async fn destroy_db(&mut self) -> Result<(), RedisError> {
        // First remove the init key so the other keys are not used in a partial state
        let _: () = self.redis_client.del(TRANCO_TOP_1M_INITIALIZED_KEY).await?;

        // Then we remove all the jarm hash keys
        let pattern = format!("{TRANCO_TOP_1M_JARM_PREFIX_KEY}*");
        let keys: Vec<String> = self.redis_client.keys(pattern).await?;
        for key in keys {
            let _: () = self.redis_client.del(key).await?;
        }
        Ok(())
    }

    async fn add_domains_from_path(&mut self, path: String) -> Result<u64, Box<dyn Error>> {
        let mut count = 0;
        let mut reader = ReaderBuilder::new().has_headers(false).from_path(path)?;
        for result in reader.records() {
            let record = result?;

            let rank = record.get(0).ok_or("No rank provided")?;
            let domain = record.get(1).ok_or("No domain provided")?;
            let jarm_hash = record.get(2).ok_or("No jarm hash provided")?;

            let key = format!("{TRANCO_TOP_1M_JARM_PREFIX_KEY}:{jarm_hash}");
            let value = format!("{rank}#{domain}");
            let _: () = self.redis_client.rpush(key, value).await?;
            count += 1;
        }
        Ok(count)
    }
}
