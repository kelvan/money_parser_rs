use crate::booking;
use chrono::NaiveDate;
use csv::Reader;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde_trim::string_trim;
use std::cmp;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct EasybankLine {
    _iban: String,
    #[serde(deserialize_with = "string_trim")]
    text: String,
    #[serde(with = "easybank_date_format", rename = "Booking date")]
    booking_date: NaiveDate,
    #[serde(with = "easybank_date_format", rename = "Value date")]
    value_date: NaiveDate,
    #[serde(default, with = "decimal_format")]
    amount: Decimal,
    currency: String,
}

mod decimal_format {
    use rust_decimal::prelude::FromStr;
    use rust_decimal::Decimal;
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            return Ok(Decimal::new(0, 0));
        }
        Decimal::from_str(&s.replace(['+', '.'], "").replace(',', "."))
            .map_err(serde::de::Error::custom)
    }
}

mod easybank_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%d.%m.%Y";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

pub fn parse_file(path: String) -> Result<Vec<booking::BookingLine>, Box<dyn Error>> {
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_path(path)?;
    parse(rdr)
}

pub fn parse_string(csv_content: String) -> Result<Vec<booking::BookingLine>, Box<dyn Error>> {
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(csv_content.as_bytes());
    parse(rdr)
}

fn parse<T>(rdr: Reader<T>) -> Result<Vec<booking::BookingLine>, Box<dyn Error>>
where
    T: std::io::Read,
{
    let mut lines: Vec<booking::BookingLine> = Vec::new();

    for result in rdr.into_deserialize() {
        let line: EasybankLine = result?;

        lines.push(booking::BookingLine {
            date: line.booking_date,
            booking_date: Some(line.booking_date),
            value_date: Some(line.value_date),
            text: line.text,
            amount: line.amount,
            credit: Some(cmp::max(Decimal::new(0, 0), line.amount)),
            debit: Some(cmp::min(Decimal::new(0, 0), line.amount).abs()),
            balance: None,
            currency: Some(line.currency),
        });
    }
    Ok(lines)
}
