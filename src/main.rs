/*!
The `bucket` crate is just a playground for Rust experiments.

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
use config::{Config, File, FileFormat};

/// Get a param's value from the [Settings.toml](../../../Settings.toml) config file.
pub fn get_config(param: &str) -> String {
    println!("param = {:?}", param);
    let mut c = Config::new();

    c.merge(File::new("Settings", FileFormat::Toml).required(false))
        .unwrap();
    let value = c.get_str(param).unwrap();
    println!("value = {:?}", value);
    value
}

fn main() {
    let csv_path1 = get_config("csv_path1");
    if let Err(err) = import::reader::read_csv(&csv_path1) {
        println!("error running example: {}", err);
        std::process::exit(1);
    }
    let csv_path2 = get_config("csv_path2");
    if let Err(err) = import::reader::read_csv(&csv_path2) {
        println!("error running example: {}", err);
        std::process::exit(1);
    }
}
