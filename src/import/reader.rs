/*!
The `imports::reader` module is for reading external data sources.
*/

use crate::datasets::*;
use csv::Reader;
use std::error::Error;

/// Read a CSV file for cities using the `csv_path` param.
///
/// This function also deserializes the CSV records using a `City` struct and prints those out
/// for test purposes.
pub fn read_csv_cities(csv_path: &str) -> Result<(), Box<Error>> {
    println!("csv recs = ");
    let mut rdr = Reader::from_path(csv_path)?;
    for result in rdr.deserialize() {
        let record: City = result?;
        // println!("\n");
        println!("{:?}", record);
        // println!("name = {}", record.name);
        // if record.population != None {
        //     println!("population = {}", record.population.unwrap());
        // }
        // if record.latitude != None {
        //     println!("latitude = {}", record.latitude.unwrap());
        // }
        // if record.longitude != None {
        //     println!("longitude = {}", record.longitude.unwrap());
        // }
    }
    Ok(())
}

/// Read a CSV file for properties using the `csv_path` param.
///
/// This function also deserializes the CSV records using a `Property` struct and prints those out
/// for test purposes.
pub fn read_csv_properties(csv_path: &str) -> Result<(), Box<Error>> {
    println!("csv recs = ");
    let mut rdr = Reader::from_path(csv_path)?;
    for result in rdr.deserialize() {
        let record: Property = result?;
        println!("{:?}", record);
    }
    Ok(())
}
