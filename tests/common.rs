use std::env;
use std::path::Path;
use rstest::*;

#[fixture]
#[once]
pub fn alexa_top1m_path() -> &'static Path {
    Path::new("tests/fixtures_data/alexa_top1M.csv")
}

#[fixture]
#[once]
pub fn set_env_var_alexa_top1m_path(alexa_top1m_path: &'static Path) -> &'static Path {
    env::set_var("ALEXA_TOP1M_RAW_DATA_PATH", alexa_top1m_path.to_str().expect("valid path"));
    alexa_top1m_path
}
