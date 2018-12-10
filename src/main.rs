#[macro_use]
extern crate serde_derive;
use config::{Config, File, FileFormat};
use csv::Reader;
use std::error::Error;
use std::process;

#[derive(Debug, Deserialize)]
struct City {
    name: String,
    population: Option<i32>,
    latitude: Option<f32>,
    longitude: Option<f32>,
}

fn read_csv() -> Result<(), Box<Error>> {
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

fn main() {
    if let Err(err) = read_csv() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
