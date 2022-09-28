use chrono::{NaiveDate};
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct BookingLine {
    // "main" date to use
    pub date: NaiveDate,
    pub booking_date: Option<NaiveDate>,
    pub value_date: Option<NaiveDate>,
    // text of booking
    pub text: String,
    pub amount: Decimal,
    pub debit: Option<Decimal>,
    pub credit: Option<Decimal>,
    pub balance: Option<Decimal>,
    pub currency: Option<String>
}
