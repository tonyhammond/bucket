use config::{Config, File, FileFormat};
use csv::Reader;
use std::error::Error;
use std::process;

fn example() -> Result<(), Box<Error>> {
    let mut c = Config::new();

    c.merge(File::new("Settings", FileFormat::Toml).required(false))
        .unwrap();
    let csv_path = c.get_str("csv_path").unwrap();
    println!("csv_path = {:?}", csv_path);

    println!("csv recs = ");
    let mut rdr = Reader::from_path(csv_path)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
