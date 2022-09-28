use chrono::{NaiveDate};
use serde::{Serialize, Deserialize};
use serde_trim::string_trim;
use crate::booking;
use csv;
use std::cmp;
use std::error::Error;
use rust_decimal::Decimal;


#[derive(Serialize, Deserialize, Debug)]
pub struct EasybankRecord {
    iban: String,
    #[serde(deserialize_with = "string_trim")]
    text: String,
    #[serde(with = "easybank_date_format", rename = "Booking date")]
    booking_date: NaiveDate,
    #[serde(with = "easybank_date_format", rename = "Value date")]
    value_date: NaiveDate,
    #[serde(default, with = "decimal_format")]
    amount: Decimal,
    currency: String
}

mod decimal_format {
    use rust_decimal::Decimal;
    use rust_decimal::prelude::FromStr;
    use serde::{self, Deserialize, Serializer, Deserializer};

    pub fn serialize<S>(
        value: &Decimal,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", value.to_string());
        serializer.serialize_str(&s)
    }
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
        Decimal::from_str(&s.replace("+", "").replace(".", "").replace(",", ".")).map_err(serde::de::Error::custom)
    }
}

mod easybank_date_format {
    use chrono::{NaiveDate};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%d.%m.%Y";

    pub fn serialize<S>(
        date: &NaiveDate,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }
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
        .has_headers(false)
        .from_path(path)?;

    for result in rdr.deserialize() {
        let line: EasybankRecord = result?;

        lines.push(
            booking::BookingLine {
                date: line.booking_date,
                booking_date: Some(line.booking_date),
                value_date: Some(line.value_date),
                text: line.text,
                amount: line.amount,
                credit: Some(cmp::max(Decimal::new(0, 0), line.amount)),
                debit: Some(cmp::min(Decimal::new(0, 0), line.amount).abs()),
                balance: None,
                currency: Some(line.currency),
            }
        );
    }
    Ok(lines)
}
