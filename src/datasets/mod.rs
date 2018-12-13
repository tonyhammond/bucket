/*!
The `datasets` module is for describing data structures.
*/

/// A `City` struct for a CSV record.
#[derive(Debug, Deserialize)]
pub struct City {
    // pub id: i32,
    pub name: String,
    pub population: Option<i32>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

impl City {
    /// Demo method `lat_long()` defined on `City` to return a lat/long tuple.
    pub fn lat_long(&self) -> (f64, f64) {
        (self.latitude.unwrap(), self.longitude.unwrap())
    }

    // Constructor as associated function.
    pub fn new(
        name: String,
        population: Option<i32>,
        latitude: Option<f64>,
        longitude: Option<f64>,
    ) -> City {
        City {
            name,
            population,
            latitude,
            longitude,
        }
    }
}

// MANHATTAN CONDOMINIUM PROPERTY Boro-Block-Lot
// MANHATTAN CONDOMINIUM PROPERTY Condo Section
// MANHATTAN CONDOMINIUM PROPERTY Address
// Borough
// Postcode
// Latitude
// Longitude
// Community Board
// Council District
// Census Tract
// BIN
// BBL
// NTA
// MANHATTAN CONDOMINIUM PROPERTY Neighborhood
// MANHATTAN CONDOMINIUM PROPERTY Building Classification
// MANHATTAN CONDOMINIUM PROPERTY Total Units
// MANHATTAN CONDOMINIUM PROPERTY Year Built
// MANHATTAN CONDOMINIUM PROPERTY Gross SqFt
// MANHATTAN CONDOMINIUM PROPERTY Est. Gross Income
// MANHATTAN CONDOMINIUM PROPERTY Gross Income per SqFt
// MANHATTAN CONDOMINIUM PROPERTY Full Market Value
// MANHATTAN CONDOMINIUM PROPERTY Market Value per SqFt
//  COMPARABLE RENTAL 1 Boro-Block-Lot
// COMPARABLE RENTAL 1 Address
// COMPARABLE RENTAL 1  Neighborhood
// COMPARABLE RENTAL 1  Building Classification
// COMPARABLE RENTAL 1  Total Units
// COMPARABLE RENTAL 1  Year Built
// COMPARABLE RENTAL 1  Gross SqFt
// COMPARABLE RENTAL 1  Est. Gross Income
// COMPARABLE RENTAL 1  Gross Income per SqFt
// COMPARABLE RENTAL 1 Full Market Value
// COMPARABLE RENTAL 1  Market Value per SqFt
// COMPARABLE RENTAL 1  Dist. from Coop in miles
// COMPARABLE RENTAL 2  Boro-Block-Lot
// COMPARABLE RENTAL 2  Address
// COMPARABLE RENTAL 2  Neighborhood
// COMPARABLE RENTAL 2  Building Classification
// COMPARABLE RENTAL 2  Total Units
// COMPARABLE RENTAL 2  Year Built
// COMPARABLE RENTAL 2  Gross SqFt
// COMPARABLE RENTAL 2  Est. Gross Income
// COMPARABLE RENTAL 2  Gross Income per SqFt
// COMPARABLE RENTAL 2  Full Market Value
// COMPARABLE RENTAL 2  Market Value per SqFt
// COMPARABLE RENTAL 2  Dist. from Coop in miles

// manhattan_condominium_property_boro_block_lot
// manhattan_condominium_property_condo_section
// manhattan_condominium_property_address
// borough
// postcode
// latitude
// longitude
// community_board
// council_district
// census_tract
// bin
// bbl
// nta
// manhattan_condominium_property_neighborhood
// manhattan_condominium_property_building_classification
// manhattan_condominium_property_total_units
// manhattan_condominium_property_year_built
// manhattan_condominium_property_gross_sqft
// manhattan_condominium_property_est_gross_income
// manhattan_condominium_property_gross_income_per_sqft
// manhattan_condominium_property_full_market_value
// manhattan_condominium_property_market_value_per_sqft
// comparable_rental_1_boro_block_lot
// comparable_rental_1_address
// comparable_rental_1_neighborhood
// comparable_rental_1_building_classification
// comparable_rental_1_total_units
// comparable_rental_1_year_built
// comparable_rental_1_gross_sqft
// comparable_rental_1_est_gross_income
// comparable_rental_1_gross_income_per_sqft
// comparable_rental_1_full_market_value
// comparable_rental_1_market_value_per_sqft
// comparable_rental_1_dist_from_coop_in_miles
// comparable_rental_2_boro_block_lot
// comparable_rental_2_address
// comparable_rental_2_neighborhood
// comparable_rental_2_building_classification
// comparable_rental_2_total_units
// comparable_rental_2_year_built
// comparable_rental_2_gross_sqft
// comparable_rental_2_est_gross_income
// comparable_rental_2_gross_income_per_sqft
// comparable_rental_2_full_market_value
// comparable_rental_2_market_value_per_sqft
// comparable_rental_2_dist_from_coop_in_miles

// bbl
// bin
// borough
// census_tract
// community_board
// comparable_rental_1_address
// comparable_rental_1_boro_block_lot
// comparable_rental_1_building_classification
// comparable_rental_1_dist_from_coop_in_miles
// comparable_rental_1_est_gross_income
// comparable_rental_1_full_market_value
// comparable_rental_1_gross_income_per_sqft
// comparable_rental_1_gross_sqft
// comparable_rental_1_market_value_per_sqft
// comparable_rental_1_neighborhood
// comparable_rental_1_total_units
// comparable_rental_1_year_built
// comparable_rental_2_address
// comparable_rental_2_boro_block_lot
// comparable_rental_2_building_classification
// comparable_rental_2_dist_from_coop_in_miles
// comparable_rental_2_est_gross_income
// comparable_rental_2_full_market_value
// comparable_rental_2_gross_income_per_sqft
// comparable_rental_2_gross_sqft
// comparable_rental_2_market_value_per_sqft
// comparable_rental_2_neighborhood
// comparable_rental_2_total_units
// comparable_rental_2_year_built
// council_district
// latitude
// longitude
// manhattan_condominium_property_address
// manhattan_condominium_property_boro_block_lot
// manhattan_condominium_property_building_classification
// manhattan_condominium_property_condo_section
// manhattan_condominium_property_est_gross_income
// manhattan_condominium_property_full_market_value
// manhattan_condominium_property_gross_income_per_sqft
// manhattan_condominium_property_gross_sqft
// manhattan_condominium_property_market_value_per_sqft
// manhattan_condominium_property_neighborhood
// manhattan_condominium_property_total_units
// manhattan_condominium_property_year_built
// nta
// postcode

/// A `Property` struct for a CSV record.
#[derive(Debug, Deserialize)]
pub struct Property {
    pub bbl: Option<i32>,
    pub bin: Option<i32>,
    pub borough: Option<i32>,
    pub census_tract: Option<i32>,
    pub community_board: Option<i32>,
    pub comparable_rental_1_address: String,
    pub comparable_rental_1_boro_block_lot: String,
    pub comparable_rental_1_building_classification: String,
    pub comparable_rental_1_dist_from_coop_in_miles: Option<f64>,
    pub comparable_rental_1_est_gross_income: Option<i32>,
    pub comparable_rental_1_full_market_value: Option<i32>,
    pub comparable_rental_1_gross_income_per_sqft: Option<f64>,
    pub comparable_rental_1_gross_sqft: Option<i32>,
    pub comparable_rental_1_market_value_per_sqft: Option<f64>,
    pub comparable_rental_1_neighborhood: String,
    pub comparable_rental_1_total_units: Option<i32>,
    pub comparable_rental_1_year_built: Option<i32>,
    pub comparable_rental_2_address: String,
    pub comparable_rental_2_boro_block_lot: String,
    pub comparable_rental_2_building_classification: String,
    pub comparable_rental_2_dist_from_coop_in_miles: Option<f64>,
    pub comparable_rental_2_est_gross_income: Option<i32>,
    pub comparable_rental_2_full_market_value: Option<i32>,
    pub comparable_rental_2_gross_income_per_sqft: Option<f64>,
    pub comparable_rental_2_gross_sqft: Option<i32>,
    pub comparable_rental_2_market_value_per_sqft: Option<f64>,
    pub comparable_rental_2_neighborhood: String,
    pub comparable_rental_2_total_units: Option<i32>,
    pub comparable_rental_2_year_built: Option<i32>,
    pub council_district: Option<i32>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub manhattan_condominium_property_address: String,
    pub manhattan_condominium_property_boro_block_lot: String,
    pub manhattan_condominium_property_building_classification: String,
    pub manhattan_condominium_property_condo_section: String,
    pub manhattan_condominium_property_est_gross_income: Option<i32>,
    pub manhattan_condominium_property_full_market_value: Option<i32>,
    pub manhattan_condominium_property_gross_income_per_sqft: Option<f64>,
    pub manhattan_condominium_property_gross_sqft: Option<i32>,
    pub manhattan_condominium_property_market_value_per_sqft: Option<f64>,
    pub manhattan_condominium_property_neighborhood: String,
    pub manhattan_condominium_property_total_units: Option<i32>,
    pub manhattan_condominium_property_year_built: Option<i32>,
    pub nta: String,
    pub postcode: Option<i32>,
}
