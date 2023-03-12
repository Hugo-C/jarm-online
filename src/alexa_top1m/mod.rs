use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use csv::ReaderBuilder;
use serde::Serialize;

#[derive(PartialEq, Serialize, Clone, Debug)]
pub struct RankedDomain {
    pub rank: u64,
    pub domain: String,
}

#[derive(Debug)]
pub struct AlexaTop1M {
    jarm_to_ranked_domain: HashMap<String, Vec<RankedDomain>>,
}

impl AlexaTop1M {
    pub fn new(path: &Path) -> Result<AlexaTop1M, Box<dyn Error>> {
        let mut reader = ReaderBuilder::new().has_headers(false).from_path(path)?;
        let mut map = HashMap::new();
        for result in reader.records() {
            let record = result?;

            let rank = record.get(0).ok_or("No rank provided")?;
            let domain = record.get(1).ok_or("No domain provided")?;
            let jarm_hash = record.get(3).ok_or("No jarm hash provided")?;

            // Retrieve existing domains or initialize an empty vec
            let domains = map
                .entry(jarm_hash.to_string())
                .or_insert(Vec::new());
            let ranked_domain = RankedDomain {
                rank: rank.parse::<u64>()?,
                domain: domain.to_string(),
            };
            domains.push(ranked_domain);
        }
        Ok(AlexaTop1M {
            jarm_to_ranked_domain: map
        })
    }

    pub fn is_empty(&self) -> bool {
        self.jarm_to_ranked_domain.is_empty()
    }

    pub fn len(&self) -> usize {
        self.jarm_to_ranked_domain.len()
    }

    pub fn get(&self, jarm_hash: &str) -> Option<&Vec<RankedDomain>> {
        self.jarm_to_ranked_domain.get(jarm_hash)
    }
}
