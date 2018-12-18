/*!
The `imports::reader` module is for reading external data sources.
*/

use crate::datasets::city::*;
use crate::datasets::property::*;
use csv::Reader;
use postgres::Connection;
use std::error::Error;

/// Read a CSV file for cities or propertiesw using the `csv_path` and `csv_type` params.
///
/// This function also deserializes the CSV records using a `City` struct or `Property` struct
/// baased on the `csv_type` param and prints those out for test purposes.
pub fn read_csv(csv_path: &str, csv_type: &str) -> Result<(), Box<Error>> {
    let mut rdr = Reader::from_path(csv_path)?;
    if csv_type == "City" {
        for result in rdr.deserialize() {
            let city: City = result?;
            // let mut city: City = result?;
            // city.name = city.get_name().trim_right().to_string();
            println!("{:?}", &city);
        }
    } else if csv_type == "Property" {
        for result in rdr.deserialize() {
            let property: Property = result?;
            println!("{:?}", &property);
        }
    }
    Ok(())
}

/// Read a CSV file for cities using the `csv_path` param.
///
/// This function also deserializes the CSV records using a `City` struct and prints those out
/// for test purposes.
pub fn read_csv_cities(csv_path: &str) -> Result<(), Box<Error>> {
    println!("csv recs = ");
    let mut rdr = Reader::from_path(csv_path)?;
    for result in rdr.deserialize() {
        let city: City = result?;
        // println!("\n");
        println!("{:?}", city);
        println!("Lat/long = {:?}", city.lat_long());
        // println!("name = {}", city.name);
        // if city.population != None {
        //     println!("population = {}", city.population.unwrap());
        // }
        // if city.latitude != None {
        //     println!("latitude = {}", city.latitude.unwrap());
        // }
        // if city.longitude != None {
        //     println!("longitude = {}", city.longitude.unwrap());
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
        let property: Property = result?;
        println!("{:?}", property);
    }
    Ok(())
}

/// Read the cities table from the Postgres database.
pub fn read_database_cities(conn: &Connection, query: &str) {
    println!("\n** read_database()\n");
    println!("db rows = ");
    for row in &conn.query(query, &[]).unwrap() {
        let city = City {
            name: row.get(0),
            population: row.get(1),
            latitude: row.get(2),
            longitude: row.get(3),
        };
        println!("{:?}", city);
    }
}

/// Read the properties table from the Postgres database.
pub fn read_database_properties(conn: &Connection, query: &str) {
    println!("\n** read_database()\n");
    println!("db rows = ");
    for row in &conn.query(query, &[]).unwrap() {
        let hood: String = row.get(0);
        let lat: f64 = row.get(1);
        let long: f64 = row.get(2);

        println!("{:?}, {:?}, {:?}", hood, lat, long);
        // let property = Property {
        // //
        // bbl: row.get(0),
        // bin: row.get(1),
        // borough: row.get(2),
        // census_tract: row.get(3),
        // community_board: row.get(4),
        // comparable_rental_1_address: row.get(5),
        // comparable_rental_1_boro_block_lot: row.get(6),
        // comparable_rental_1_building_classification: row.get(7),
        // comparable_rental_1_dist_from_coop_in_miles: row.get(8),
        // comparable_rental_1_est_gross_income: row.get(9),
        // comparable_rental_1_full_market_value: row.get(10),
        // comparable_rental_1_gross_income_per_sqft: row.get(11),
        // comparable_rental_1_gross_sqft: row.get(12),
        // comparable_rental_1_market_value_per_sqft: row.get(13),
        // comparable_rental_1_neighborhood: row.get(14),
        // comparable_rental_1_total_units: row.get(15),
        // comparable_rental_1_year_built: row.get(16),
        // comparable_rental_2_address: row.get(17),
        // comparable_rental_2_boro_block_lot: row.get(18),
        // comparable_rental_2_building_classification: row.get(19),
        // comparable_rental_2_dist_from_coop_in_miles: row.get(20),
        // comparable_rental_2_est_gross_income: row.get(21),
        // comparable_rental_2_full_market_value: row.get(22),
        // comparable_rental_2_gross_income_per_sqft: row.get(23),
        // comparable_rental_2_gross_sqft: row.get(24),
        // comparable_rental_2_market_value_per_sqft: row.get(25),
        // comparable_rental_2_neighborhood: row.get(26),
        // comparable_rental_2_total_units: row.get(27),
        // comparable_rental_2_year_built: row.get(28),
        // council_district: row.get(29),
        // latitude: row.get(30),
        // longitude: row.get(31),
        // manhattan_condominium_property_address: row.get(32),
        // manhattan_condominium_property_boro_block_lot: row.get(33),
        // manhattan_condominium_property_building_classification: row.get(34),
        // manhattan_condominium_property_condo_section: row.get(35),
        // manhattan_condominium_property_est_gross_income: row.get(36),
        // manhattan_condominium_property_full_market_value: row.get(37),
        // manhattan_condominium_property_gross_income_per_sqft: row.get(38),
        // manhattan_condominium_property_gross_sqft: row.get(39),
        // manhattan_condominium_property_market_value_per_sqft: row.get(40),
        // manhattan_condominium_property_neighborhood: row.get(41),
        // manhattan_condominium_property_total_units: row.get(42),
        // manhattan_condominium_property_year_built: row.get(43),
        // nta: row.get(44),
        // postcode: row.get(45),
        //};
        // println!("{:?}", property);
    }
}
