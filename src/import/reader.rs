/*!
The `imports::reader` module is for reading external data sources.
*/

use crate::datasets::City;
use csv::Reader;
use std::error::Error;

/// Read a CSV file using the `csv_path` param.
///
/// This function also deserializes the CSV records using a `City` struct and prints those out
/// for test purposes.
pub fn read_csv(csv_path: &str) -> Result<(), Box<Error>> {
    println!("csv recs = ");
    let mut rdr = Reader::from_path(csv_path)?;
    for result in rdr.deserialize() {
        let record: City = result?;
        println!("{:?}", record);
    }
    Ok(())
}
