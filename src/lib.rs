use reader::{load_csv_content, MoneyReader};
use std::{error::Error, str::FromStr};

pub mod booking;
pub mod reader;
pub mod writer;

pub fn convert_csv_content(
    csv_content: String,
    informat: String,
) -> Result<Vec<booking::BookingLine>, Box<dyn Error>> {
    load_csv_content(
        csv_content,
        MoneyReader::from_str(informat.as_str()).unwrap(),
    )
}
