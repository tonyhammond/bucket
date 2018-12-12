/*!
The `bucket` crate is just a playground for Rust experiments. See
[doc](https://tonyhammond.github.io/bucket/doc/bucket/) for the crate docs.

Initially we're going to use the `csv` crate to read from a `.csv` file. We'll use
[csv/test.csv](../../../csv/test.csv) for now.

We're also going to experiment with file organization and module structure,
and with documenting the code.


```csv
name,population,latitude,longitude
New York ,8287238,40.7305991,-73.9865812
Los Angeles ,3826423,34.053717,-118.2427266
...
```

# Modules

For now have broken out the following structure:


```text
src/
├── datasets
│   └── mod.rs
├── import
│   ├── mod.rs
│   └── reader.rs
└── main.rs
```
*/

#[macro_use]
extern crate serde_derive;

pub mod datasets;
pub mod import;

use crate::datasets::*;
use config::{Config, File, FileFormat};
use postgres::{Connection, TlsMode};

/// Get a param's value from the [Settings.toml](../../../Settings.toml) config file.
pub fn get_config(param: &str) -> String {
    // println!("param = {:?}", param);
    let mut c = Config::new();

    c.merge(File::new("Settings", FileFormat::Toml).required(false))
        .unwrap();
    let value = c.get_str(param).unwrap();
    // println!("value = {:?}", value);
    println!("{:?} = {:?}", param, value);
    value
}

pub fn read_csv() {
    println!("\n** read_csv()\n");
    let csv_path = get_config("csv_path_cities");
    if let Err(err) = import::reader::read_csv_cities(&csv_path) {
        println!("error running example: {}", err);
        std::process::exit(1);
    }
    if get_config("debug") == "true" {
        let csv_path = get_config("csv_path_properties");
        if let Err(err) = import::reader::read_csv_properties(&csv_path) {
            println!("error running example: {}", err);
            std::process::exit(1);
        }
    }
}

pub fn read_database() {
    println!("\n** read_database()\n");
    let dsn = get_config("dsn");
    let conn = match Connection::connect(dsn, TlsMode::None) {
        Ok(conn) => conn,
        Err(e) => {
            println!("! {:?}", e);
            return;
        }
    };

    let query = "SELECT name, population, latitude, longitude FROM cities LIMIT 5";
    println!("db rows = ");
    for row in &conn.query(query, &[]).unwrap() {
      let city = City {
            name: row.get(0),
            population: row.get(1),
            // latitude: Some(0.0),
            // longitude: Some(0.0),
            latitude: None,
            longitude: None,
        };
        println!("{:?}", city);
    }
}

fn main() {
    read_csv();
    read_database();
}
