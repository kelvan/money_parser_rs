mod easybank;
mod neon;
mod revolut;
mod sgkb;
use std::error::Error;
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
    Neon,
}

pub fn load_csv_file(
    path: String,
    source_type: MoneyReader,
) -> Result<Vec<booking::BookingLine>, Box<dyn Error>> {
    let result = match source_type {
        MoneyReader::Sgkb => sgkb::parse_file(path),
        MoneyReader::Easybank => easybank::parse_file(path),
        MoneyReader::Revolut => revolut::parse_file(path),
        MoneyReader::Neon => neon::parse_file(path),
    };
    result
}

pub fn load_csv_content(
    csv_content: String,
    source_type: MoneyReader,
) -> Result<Vec<booking::BookingLine>, Box<dyn Error>> {
    let result = match source_type {
        MoneyReader::Sgkb => sgkb::parse_string(csv_content),
        MoneyReader::Easybank => easybank::parse_string(csv_content),
        MoneyReader::Revolut => revolut::parse_string(csv_content),
        MoneyReader::Neon => neon::parse_string(csv_content),
    };
    result
}
