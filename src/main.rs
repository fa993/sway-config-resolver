use include_one::{config::Config, parse_configs};
use std::env::args;

fn main() {
    let mut files_to_parse = args().skip(1);
    let mut conf = Config::default();
    parse_configs(&mut files_to_parse, &mut conf).expect("Error");
    println!("{conf:?}");
}
