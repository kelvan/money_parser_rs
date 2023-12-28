use crate::booking;
use chrono::NaiveDate;
use csv::Reader;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde_trim::string_trim;
use std::cmp;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct RevolutLine {
    #[serde(rename = "Type")]
    _booking_type: String,
    #[serde(rename = "Product")]
    _product: String,
    #[serde(with = "revolut_date_format_option", rename = "Completed Date")]
    booking_date: Option<NaiveDate>,
    #[serde(with = "revolut_date_format", rename = "Started Date")]
    value_date: NaiveDate,
    #[serde(deserialize_with = "string_trim", rename = "Description")]
    text: String,
    #[serde(rename = "Amount", default, with = "decimal_format")]
    amount: Decimal,
    #[serde(rename = "Fee", default, with = "decimal_format")]
    fee: Decimal,
    #[serde(rename = "Balance", default, with = "decimal_format")]
    balance: Decimal,
    #[serde(rename = "Currency")]
    currency: String,
    #[serde(rename = "State")]
    _state: String,
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

mod revolut_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

mod revolut_date_format_option {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            return Ok(None);
        }
        NaiveDate::parse_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
            .map(Some)
    }
}

pub fn parse_file(path: String) -> Result<Vec<booking::BookingLine>, Box<dyn Error>> {
    let rdr = csv::ReaderBuilder::new().delimiter(b',').from_path(path)?;
    parse(rdr)
}

pub fn parse_string(csv_content: String) -> Result<Vec<booking::BookingLine>, Box<dyn Error>> {
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(csv_content.as_bytes());
    parse(rdr)
}

fn parse<T>(rdr: Reader<T>) -> Result<Vec<booking::BookingLine>, Box<dyn Error>>
where
    T: std::io::Read,
{
    let mut lines: Vec<booking::BookingLine> = Vec::new();

    for result in rdr.into_deserialize() {
        let line: RevolutLine = result?;
        let total_amount = line.amount - line.fee;

        lines.push(booking::BookingLine {
            date: line.value_date,
            booking_date: line.booking_date,
            value_date: Some(line.value_date),
            text: line.text,
            amount: total_amount,
            credit: Some(cmp::max(Decimal::new(0, 0), total_amount)),
            debit: Some(cmp::min(Decimal::new(0, 0), total_amount).abs()),
            balance: Some(line.balance),
            currency: Some(line.currency),
        });
    }
    Ok(lines)
}
