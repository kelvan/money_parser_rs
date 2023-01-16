use reader::{load_csv_content, MoneyReader};
use std::str::FromStr;

pub mod booking;
pub mod reader;
pub mod writer;

pub fn convert_csv_content(csv_content: String, informat: String) {
    let booking_lines = match load_csv_content(csv_content, MoneyReader::from_str(informat.as_str()).unwrap()) {
        Ok(lines) => lines,
        Err(err) => {
            !panic!("AAHHH");
        }
    };
}
