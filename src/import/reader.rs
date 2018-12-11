/*!
The `imports::reader` module is for reading external data sources.
*/


use config::{Config, File, FileFormat};
use csv::Reader;
use std::error::Error;
use crate::datasets::City;

/// Read a CSV file using the `csv_path` given in the [Settings.toml](../../../Settings.toml) config file.
///
/// This function also deserializes the CSV records using a `City` struct and prints those out
/// for test purposes.
pub fn read_csv() -> Result<(), Box<Error>> {
    let mut c = Config::new();

    c.merge(File::new("Settings", FileFormat::Toml).required(false))
        .unwrap();
    let csv_path = c.get_str("csv_path").unwrap();
    println!("csv_path = {:?}", csv_path);

    println!("csv recs = ");
    let mut rdr = Reader::from_path(csv_path)?;
    for result in rdr.deserialize() {
        let record: City = result?;
        println!("{:?}", record);
    }

    Ok(())
}
