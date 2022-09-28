mod homebank;
use std::{error::Error};
use strum_macros::EnumString;
use crate::booking;

#[derive(EnumString)]
pub enum MoneyWriter {
    #[strum(ascii_case_insensitive)]
    Homebank,
}


pub fn write_csv(booking_lines: Vec<booking::BookingLine>, path: String, source_type: MoneyWriter) -> Result<(), Box<dyn Error>> {
    let result = match source_type {
        MoneyWriter::Homebank => homebank::write_csv(booking_lines, path),
    };
    result
}
