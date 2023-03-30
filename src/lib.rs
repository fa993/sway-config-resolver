use std::path::Path;

use config::Config;
use error::SwayIOError;

pub mod error;
pub mod config;

pub fn parse_configs(
    files_to_parse: &mut impl Iterator<Item = String>,
    conf: &mut Config,
) -> Result<(), SwayIOError> {
    for t in files_to_parse {
        conf.read_config(Path::new(&t))?;
    }
    Ok(())
}
