use chrono::{NaiveDate};
use serde::{Serialize, Deserialize};
use serde_trim::string_trim;

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    #[serde(with = "sgkb_date_format", rename = "Booking date")]
    date: NaiveDate,
    #[serde(with = "sgkb_date_format", rename = "Value date")]
    value_date: NaiveDate,
    #[serde(deserialize_with = "string_trim", rename = "Booking text")]
    text: String,
    #[serde(rename = "Debit")]
    debit: Option<String>,
    #[serde(rename = "Credit")]
    credit: Option<String>,
    #[serde(rename = "Balance")]
    balance: String
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
