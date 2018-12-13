/*!
The `bucket` crate is just a playground for Rust experiments.

Initially we're going to use the [csv](https://docs.rs/csv/1.0.2/csv/) crate to read from a `.csv` file and the [postgres](https://docs.rs/postgres/0.15.2/postgres/)
crate to read from a Postgres database.

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
src
├── config.rs
├── datasets
│   └── mod.rs
├── import
│   ├── mod.rs
│   └── reader.rs
├── main.rs
└── utils.rs
```

# Examples

## Reading CSV files
```
println!("\n** read_csv()\n");
let csv_path = get_config("csv_path_cities");
if let Err(err) = read_csv_cities(&csv_path) {
    println!("error running example: {}", err);
    exit(1);
}
if get_config("debug") == "true" {
    let csv_path = get_config("csv_path_properties");
    if let Err(err) = read_csv_properties(&csv_path) {
        println!("error running example: {}", err);
        exit(1);
    }
}
```

## Reading the database
```
println!("\n** init_database()\n");
let dsn = get_config("dsn");
// let conn = init_database();
let conn = match Connection::connect(dsn, TlsMode::None) {
    Ok(conn) => conn,
    Err(e) => {
        println!("! {:?}", e);
        return;
    }
};
let query = get_config("query_cities");
read_database_cities(&conn, &query);
```
*/

#[macro_use]
extern crate serde_derive;

pub mod config;
pub mod datasets;
pub mod import;
pub mod utils;

use self::config::*;
use self::datasets::*;
use self::import::reader::*;
use self::utils::*;

use postgres::{Connection, TlsMode};
use std::process::exit;

// /// Open connection to the Postgres database.
// pub fn init_database() -> Result<postgres::Connection, Box<Error>> {
//     println!("\n** init_database()\n");
//     let dsn = get_config("dsn");
//     Connection::connect(dsn, TlsMode::None)
//     // let conn = match Connection::connect(dsn, TlsMode::None) {
//     //     Ok(conn) => conn,
//     //     Err(e) => {
//     //         println!("! {:?}", e);
//     //         return;
//     //     }
//     // };
//     // conn
// }

fn main() {
    println!("\n** test things\n");
    let name = String::from("Nowhere");
    let city = City::new(name, Some(123), None, None);
    debug_show(&city);

    println!("\n** read_csv()\n");
    let csv_path = get_config("csv_path_cities");
    if let Err(err) = read_csv_cities(&csv_path) {
        println!("error running example: {}", err);
        exit(1);
    }
    if get_config("debug") == "true" {
        let csv_path = get_config("csv_path_properties");
        if let Err(err) = read_csv_properties(&csv_path) {
            println!("error running example: {}", err);
            exit(1);
        }
    }

    println!("\n** init_database()\n");
    let dsn = get_config("dsn");
    // let conn = init_database();
    let conn = match Connection::connect(dsn, TlsMode::None) {
        Ok(conn) => conn,
        Err(e) => {
            println!("! {:?}", e);
            return;
        }
    };
    // let query = get_config("query_cities");
    // read_database_cities(&conn, &query);
    let query = get_config("query_properties");
    read_database_properties(&conn, &query);
}
