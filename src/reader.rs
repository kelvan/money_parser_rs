mod sgkb;
mod easybank;
mod revolut;
mod neon;
use std::{error::Error};
use strum_macros::EnumString;

use crate::booking;

#[derive(EnumString)]
pub enum MoneyReader {
    #[strum(ascii_case_insensitive)]
    Sgkb,
    #[strum(ascii_case_insensitive)]
    Easybank,
    #[strum(ascii_case_insensitive)]
    Revolut,
    #[strum(ascii_case_insensitive)]
    Neon
}


pub fn load_csv(path: String, source_type: MoneyReader) -> Result<Vec<booking::BookingLine>, Box<dyn Error>> {
    let result = match source_type {
        MoneyReader::Sgkb => sgkb::parse_from_file(path),
        MoneyReader::Easybank => easybank::parse_from_file(path),
        MoneyReader::Revolut => revolut::parse_from_file(path),
        MoneyReader::Neon => neon::parse_from_file(path),
    };
    result
}
