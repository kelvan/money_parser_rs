use chrono::NaiveDate;
use serde::Deserialize;
use serde_trim::string_trim;
use crate::booking;
use std::error::Error;
use std::cmp;
use rust_decimal::Decimal;


#[derive(Deserialize, Debug)]
pub struct NeonLine {
    #[serde(with = "neon_date_format", rename = "Date")]
    date: NaiveDate,
    #[serde(deserialize_with = "string_trim", rename = "Description")]
    text: String,
    #[serde(rename = "Amount", default, with = "decimal_format")]
    amount: Decimal,
}

mod decimal_format {
    use rust_decimal::Decimal;
    use rust_decimal::prelude::FromStr;
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Decimal, D::Error>
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
    use chrono::{NaiveDate};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%Y-%m-%d";

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}


pub fn parse_from_file(path: String) -> Result<Vec<booking::BookingLine>, Box<dyn Error>> {
    let mut lines: Vec<booking::BookingLine> = Vec::new();

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path(path)?;

    for result in rdr.deserialize() {
        let line: NeonLine = result?;

        lines.push(
            booking::BookingLine {
                date: line.date,
                booking_date: None,
                value_date: None,
                text: line.text,
                amount: line.amount,
                credit: Some(cmp::max(Decimal::new(0, 0), line.amount)),
                debit: Some(cmp::min(Decimal::new(0, 0), line.amount).abs()),
                balance: None,
                currency: Some("CHF".to_string())
            }
        );
    }
    Ok(lines)
}
