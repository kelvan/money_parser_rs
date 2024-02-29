use crate::booking;
use chrono::NaiveDate;
use csv::Reader;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde_trim::string_trim;
use std::cmp;
use std::error::Error;

use super::Parser;

#[derive(Deserialize, Debug)]
pub struct NeonLine {
    #[serde(with = "neon_date_format", rename = "Date")]
    date: NaiveDate,
    #[serde(deserialize_with = "string_trim", rename = "Description")]
    text: String,
    #[serde(rename = "Amount", default, with = "decimal_format")]
    amount: Decimal,
}

pub struct Neon;
impl Parser for Neon {
    fn parse<T>(&self, rdr: Reader<T>) -> Result<Vec<booking::BookingLine>, Box<dyn Error>>
    where
        T: std::io::Read,
    {
        let mut lines: Vec<booking::BookingLine> = Vec::new();

        for result in rdr.into_deserialize() {
            let line: NeonLine = result?;

            lines.push(booking::BookingLine {
                date: line.date,
                booking_date: None,
                value_date: None,
                text: line.text,
                amount: line.amount,
                credit: Some(cmp::max(Decimal::new(0, 0), line.amount)),
                debit: Some(cmp::min(Decimal::new(0, 0), line.amount).abs()),
                balance: None,
                currency: Some("CHF".to_string()),
            });
        }
        Ok(lines)
    }

    fn parse_file(&self, path: String) -> Result<Vec<booking::BookingLine>, Box<dyn Error>> {
        let rdr = csv::ReaderBuilder::new().delimiter(b';').from_path(path)?;
        self.parse(rdr)
    }

    fn parse_string(&self, csv_content: String) -> Result<Vec<booking::BookingLine>, Box<dyn Error>> {
        let rdr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(csv_content.as_bytes());
        self.parse(rdr)
    }
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
        Decimal::from_str(&s).map_err(serde::de::Error::custom)
    }
}

mod neon_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%Y-%m-%d";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
