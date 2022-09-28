use crate::booking;
use std::error::Error;
use serde::Serialize;
use chrono::NaiveDate;
use rust_decimal::Decimal;

#[derive(Debug, Serialize)]
pub struct HomebankLine {
    // "main" date to use
    #[serde(with = "homebank_date_format")]
    pub date: NaiveDate,
    pub payment_mode: Option<String>,
    // text of booking
    pub info: String,
    pub payee: Option<String>,
    pub memo: Option<String>,
    pub amount: Decimal,
    pub category: Option<String>,
    pub tags: Option<String>
}

mod homebank_date_format {
    use chrono::{NaiveDate};
    use serde::{self, Serializer};

    const FORMAT: &str = "%Y-%m-%d";

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
}

pub fn write_csv(booking_lines: Vec<booking::BookingLine>, path: String) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::WriterBuilder::new()
    .delimiter(b';')
    .has_headers(false)
    .from_path(path)?;

    for line in booking_lines {
        wtr.serialize(
            HomebankLine {
                date: line.date,
                payment_mode: None,
                info: line.text,
                payee: None,
                memo: None,
                amount: line.amount,
                category: None,
                tags: None
            }
        )?;
    }

    wtr.flush()?;
    Ok(())
}
