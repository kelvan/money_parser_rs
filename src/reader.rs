mod sgkb;
mod easybank;
use std::{error::Error};
use strum_macros::EnumString;

use crate::booking;

#[derive(EnumString)]
pub enum MoneyReader {
    #[strum(ascii_case_insensitive)]
    Sgkb,
    #[strum(ascii_case_insensitive)]
    Easybank
}


pub fn load_csv(path: String, source_type: MoneyReader) -> Result<Vec<booking::BookingLine>, Box<dyn Error>> {
    let result = match source_type {
        MoneyReader::Sgkb => sgkb::parse_from_file(path),
        MoneyReader::Easybank => easybank::parse_from_file(path)
    };
    result
}
