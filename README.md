# bucket – Playground for Rust experiments

The `bucket` crate is just a playground for Rust experiments. See
[doc](https://tonyhammond.github.io/bucket/doc/bucket/) for the crate docs.

First off, some simple code for reading a CSV file just to have something to
play with. We're parsing this with the
[csv](https://docs.rs/csv/1.0.2/csv/) crate.

We've also added the CSV file paths in `csv_path*` properties in the config
file `Settings.toml`.

Also to get some feel for reading from a database we're using the
[postgres](https://docs.rs/postgres/0.15.2/postgres/) crate.

Again using the config file `Settings.toml` to read the `dsn` connection string.

## Structs

Here's the simple `City` struct we're using.

```
/// A `City` struct for a CSV record.
#[derive(Debug, Deserialize)]
pub struct City {
    // pub id: i32,
    pub name: String,
    pub population: Option<i32>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}
```

## Examples

Example of running this:
```
% cargo run
   Compiling bucket v0.1.0 (/Users/tony/Projects/github/tonyhammond/bucket)                                                              
    Finished dev [unoptimized + debuginfo] target(s) in 2.34s                                                                            
     Running `target/debug/bucket`

** read_csv()

"csv_path_cities" = "csv/test_cities.csv"
csv recs =
City { name: "New York ", population: Some(8287238), latitude: Some(40.7305991), longitude: Some(-73.9865812) }
City { name: "Los Angeles ", population: Some(3826423), latitude: Some(34.053717), longitude: Some(-118.2427266) }
City { name: "Chicago ", population: Some(2705627), latitude: Some(41.8755546), longitude: Some(-87.6244212) }
City { name: "Houston ", population: Some(2129784), latitude: Some(29.7589382), longitude: Some(-95.3676974) }
City { name: "Philadelphia ", population: Some(1539313), latitude: Some(39.952335), longitude: Some(-75.163789) }
"debug" = "false"

** read_database()

"dsn" = "postgresql://postgres@localhost:5432/cosmo_dev"
db rows =
City { name: "New York", population: Some(8287238), latitude: Some(40.7305991), longitude: Some(-73.9865812) }
City { name: "Los Angeles", population: Some(3826423), latitude: Some(34.053717), longitude: Some(-118.2427266) }
City { name: "Chicago", population: Some(2705627), latitude: Some(41.8755546), longitude: Some(-87.6244212) }
City { name: "Houston", population: Some(2129784), latitude: Some(29.7589382), longitude: Some(-95.3676974) }
City { name: "Philadelphia", population: Some(1539313), latitude: Some(39.952335), longitude: Some(-75.163789) }
```

## Extra

Something else to look at is the [sophia](https://docs.rs/sophia/0.1.0/sophia/) crate for RDF processing. (Source code in the [sophia_rs](https://github.com/pchampin/sophia_rs) project.)
