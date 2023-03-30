pub mod config;
pub mod error;

use std::{env::args, path::Path};

use config::Config;
use error::SwayIOError;

fn main() {
    let mut files_to_parse = args().skip(1);
    let mut conf = Config::default();
    parse_configs(&mut files_to_parse, &mut conf).expect("Error");
    println!("{conf:?}");
}

fn parse_configs(
    files_to_parse: &mut impl Iterator<Item = String>,
    conf: &mut Config,
) -> Result<(), SwayIOError> {
    for t in files_to_parse {
        conf.read_config(Path::new(&t))?;
    }
    Ok(())
}
