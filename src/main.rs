mod reader;
use std::process;
use reader::{load_csv, MoneyReader};


fn main() {
    if let Err(err) = load_csv("example.csv".to_string(), MoneyReader::Sgkb) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
