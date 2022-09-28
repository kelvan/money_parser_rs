use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use serde_trim::string_trim;
use crate::booking;
use csv;
use std::error::Error;
use rust_decimal::Decimal;


#[derive(Serialize, Deserialize, Debug)]
pub struct SgkbRecord {
    #[serde(with = "sgkb_date_format", rename = "Booking date")]
    booking_date: NaiveDate,
    #[serde(with = "sgkb_date_format", rename = "Value date")]
    value_date: NaiveDate,
    #[serde(deserialize_with = "string_trim", rename = "Booking text")]
    text: String,
    #[serde(rename = "Debit", default, with = "decimal_format")]
    debit: Decimal,
    #[serde(rename = "Credit", default, with = "decimal_format")]
    credit: Decimal,
    #[serde(rename = "Balance", default, with = "decimal_format")]
    balance: Decimal
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
        Decimal::from_str(&s.replace("'", "")).map_err(serde::de::Error::custom)
    }
}

mod sgkb_date_format {
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
        .from_path(path)?;

    for result in rdr.deserialize() {
        let line: SgkbRecord = result?;

        lines.push(
            booking::BookingLine {
                date: line.booking_date,
                booking_date: Some(line.booking_date),
                value_date: Some(line.value_date),
                text: line.text,
                amount: line.credit - line.debit,
                credit: Some(line.credit),
                debit: Some(line.debit),
                balance: None,
                currency: None,
            }
        );
    }
    Ok(lines)
}
