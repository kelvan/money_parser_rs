mod sgkb;
use std::error::Error;
use strum_macros::EnumString;

#[derive(EnumString)]
pub enum MoneyReader {
    Sgkb,
    Easybank
}


pub fn load_csv(path: String, source_type: MoneyReader) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path(path)?;
    for result in rdr.deserialize() {
        match source_type {
            MoneyReader::Sgkb => {
                let record: sgkb::Record = result?;
                println!("{:?}", record);
            }
            MoneyReader::Easybank => {
                // ...
            }
        }
    }
    Ok(())
}
