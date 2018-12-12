# bucket – Playground for Rust experiments

First off, some simple code for reading a CSV file just to have something to
play with. We're parsing this with the
[csv](https://docs.rs/csv/1.0.2/csv/) crate.

We've also added the CSV file paths in `csv_path*` properties in the config
file `Settings.toml`.

Also to get some feel for reading from a database we're using the
[postgres](https://docs.rs/postgres/0.15.2/postgres/) crate.

Again using the config file `Settings.toml` to read the `dsn` connection string.

Example of running this:
```
% cargo run
   Compiling bucket v0.1.0 (/Users/tony/Projects/github/tonyhammond/bucket)                                                              
    Finished dev [unoptimized + debuginfo] target(s) in 2.50s                                                                            
     Running `target/debug/bucket`

** read_csv()

"csv_path_cities" = "csv/test_cities.csv"
csv recs =
City { name: "New York ", population: Some(8287238), latitude: Some(40.7306), longitude: Some(-73.98658) }
City { name: "Los Angeles ", population: Some(3826423), latitude: Some(34.05372), longitude: Some(-118.24273) }
City { name: "Chicago ", population: Some(2705627), latitude: Some(41.875553), longitude: Some(-87.62442) }
City { name: "Houston ", population: Some(2129784), latitude: Some(29.758938), longitude: Some(-95.3677) }
City { name: "Philadelphia ", population: Some(1539313), latitude: Some(39.952335), longitude: Some(-75.16379) }
"debug" = "false"

** read_database()

"dsn" = "postgresql://postgres@localhost:5432/cosmo_dev"
db rows =
City { name: "New York", population: Some(8287238), latitude: None, longitude: None }
City { name: "Los Angeles", population: Some(3826423), latitude: None, longitude: None }
City { name: "Chicago", population: Some(2705627), latitude: None, longitude: None }
City { name: "Houston", population: Some(2129784), latitude: None, longitude: None }
City { name: "Philadelphia", population: Some(1539313), latitude: None, longitude: None }
```

Note that having some difficulty reading the lat/long floats as different sizes in the database from those declared in the  `City` struct, hence these are being set to `None` for now.

Something else to look at is the [sophia](https://docs.rs/sophia/0.1.0/sophia/) crate for RDF processing. (Source code in the [sophia_rs](https://github.com/pchampin/sophia_rs) project.)
