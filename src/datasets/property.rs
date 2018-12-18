/*!
The `property` module defines the `Property` struct and implementation.
*/

//extern crate serde;

/// A `Property` struct for a CSV record.
#[derive(Debug, Deserialize)]
#[derive(PartialEq, PartialOrd)]
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

// Implement the Default trait.
impl Default for Property {
    fn default() -> Property {
        Property {
            bbl: Some(0),
            bin: Some(0),
            borough: Some(0),
            census_tract: Some(0),
            community_board: Some(0),
            comparable_rental_1_address: "".to_string(),
            comparable_rental_1_boro_block_lot: "".to_string(),
            comparable_rental_1_building_classification: "".to_string(),
            comparable_rental_1_dist_from_coop_in_miles: Some(0.0),
            comparable_rental_1_est_gross_income: Some(0),
            comparable_rental_1_full_market_value: Some(0),
            comparable_rental_1_gross_income_per_sqft: Some(0.0),
            comparable_rental_1_gross_sqft: Some(0),
            comparable_rental_1_market_value_per_sqft: Some(0.0),
            comparable_rental_1_neighborhood: "".to_string(),
            comparable_rental_1_total_units: Some(0),
            comparable_rental_1_year_built: Some(0),
            comparable_rental_2_address: "".to_string(),
            comparable_rental_2_boro_block_lot: "".to_string(),
            comparable_rental_2_building_classification: "".to_string(),
            comparable_rental_2_dist_from_coop_in_miles: Some(0.0),
            comparable_rental_2_est_gross_income: Some(0),
            comparable_rental_2_full_market_value: Some(0),
            comparable_rental_2_gross_income_per_sqft: Some(0.0),
            comparable_rental_2_gross_sqft: Some(0),
            comparable_rental_2_market_value_per_sqft: Some(0.0),
            comparable_rental_2_neighborhood: "".to_string(),
            comparable_rental_2_total_units: Some(0),
            comparable_rental_2_year_built: Some(0),
            council_district: Some(0),
            latitude: Some(0.0),
            longitude: Some(0.0),
            manhattan_condominium_property_address: "".to_string(),
            manhattan_condominium_property_boro_block_lot: "".to_string(),
            manhattan_condominium_property_building_classification: "".to_string(),
            manhattan_condominium_property_condo_section: "".to_string(),
            manhattan_condominium_property_est_gross_income: Some(0),
            manhattan_condominium_property_full_market_value: Some(0),
            manhattan_condominium_property_gross_income_per_sqft: Some(0.0),
            manhattan_condominium_property_gross_sqft: Some(0),
            manhattan_condominium_property_market_value_per_sqft: Some(0.0),
            manhattan_condominium_property_neighborhood: "".to_string(),
            manhattan_condominium_property_total_units: Some(0),
            manhattan_condominium_property_year_built: Some(0),
            nta: "".to_string(),
            postcode: Some(0),
        }
    }
}

// Associated functions.
impl Property {
    // Constructor as associated function.
    pub fn new(
        bbl: Option<i32>,
        bin: Option<i32>,
        borough: Option<i32>,
        census_tract: Option<i32>,
        community_board: Option<i32>,
        comparable_rental_1_address: String,
        comparable_rental_1_boro_block_lot: String,
        comparable_rental_1_building_classification: String,
        comparable_rental_1_dist_from_coop_in_miles: Option<f64>,
        comparable_rental_1_est_gross_income: Option<i32>,
        comparable_rental_1_full_market_value: Option<i32>,
        comparable_rental_1_gross_income_per_sqft: Option<f64>,
        comparable_rental_1_gross_sqft: Option<i32>,
        comparable_rental_1_market_value_per_sqft: Option<f64>,
        comparable_rental_1_neighborhood: String,
        comparable_rental_1_total_units: Option<i32>,
        comparable_rental_1_year_built: Option<i32>,
        comparable_rental_2_address: String,
        comparable_rental_2_boro_block_lot: String,
        comparable_rental_2_building_classification: String,
        comparable_rental_2_dist_from_coop_in_miles: Option<f64>,
        comparable_rental_2_est_gross_income: Option<i32>,
        comparable_rental_2_full_market_value: Option<i32>,
        comparable_rental_2_gross_income_per_sqft: Option<f64>,
        comparable_rental_2_gross_sqft: Option<i32>,
        comparable_rental_2_market_value_per_sqft: Option<f64>,
        comparable_rental_2_neighborhood: String,
        comparable_rental_2_total_units: Option<i32>,
        comparable_rental_2_year_built: Option<i32>,
        council_district: Option<i32>,
        latitude: Option<f64>,
        longitude: Option<f64>,
        manhattan_condominium_property_address: String,
        manhattan_condominium_property_boro_block_lot: String,
        manhattan_condominium_property_building_classification: String,
        manhattan_condominium_property_condo_section: String,
        manhattan_condominium_property_est_gross_income: Option<i32>,
        manhattan_condominium_property_full_market_value: Option<i32>,
        manhattan_condominium_property_gross_income_per_sqft: Option<f64>,
        manhattan_condominium_property_gross_sqft: Option<i32>,
        manhattan_condominium_property_market_value_per_sqft: Option<f64>,
        manhattan_condominium_property_neighborhood: String,
        manhattan_condominium_property_total_units: Option<i32>,
        manhattan_condominium_property_year_built: Option<i32>,
        nta: String,
        postcode: Option<i32>,
    ) -> Property {
        Property {
            bbl,
            bin,
            borough,
            census_tract,
            community_board,
            comparable_rental_1_address,
            comparable_rental_1_boro_block_lot,
            comparable_rental_1_building_classification,
            comparable_rental_1_dist_from_coop_in_miles,
            comparable_rental_1_est_gross_income,
            comparable_rental_1_full_market_value,
            comparable_rental_1_gross_income_per_sqft,
            comparable_rental_1_gross_sqft,
            comparable_rental_1_market_value_per_sqft,
            comparable_rental_1_neighborhood,
            comparable_rental_1_total_units,
            comparable_rental_1_year_built,
            comparable_rental_2_address,
            comparable_rental_2_boro_block_lot,
            comparable_rental_2_building_classification,
            comparable_rental_2_dist_from_coop_in_miles,
            comparable_rental_2_est_gross_income,
            comparable_rental_2_full_market_value,
            comparable_rental_2_gross_income_per_sqft,
            comparable_rental_2_gross_sqft,
            comparable_rental_2_market_value_per_sqft,
            comparable_rental_2_neighborhood,
            comparable_rental_2_total_units,
            comparable_rental_2_year_built,
            council_district,
            latitude,
            longitude,
            manhattan_condominium_property_address,
            manhattan_condominium_property_boro_block_lot,
            manhattan_condominium_property_building_classification,
            manhattan_condominium_property_condo_section,
            manhattan_condominium_property_est_gross_income,
            manhattan_condominium_property_full_market_value,
            manhattan_condominium_property_gross_income_per_sqft,
            manhattan_condominium_property_gross_sqft,
            manhattan_condominium_property_market_value_per_sqft,
            manhattan_condominium_property_neighborhood,
            manhattan_condominium_property_total_units,
            manhattan_condominium_property_year_built,
            nta,
            postcode,
        }
    }
}

// Tests.
#[cfg(test)]
mod tests {
    use crate::datasets::property::Property;

    #[test]
    fn test_property_default() {
        let property_a = Property::default();
        let property_b = Property::new(
            Some(0),    // bbl
            Some(0),    // bin
            Some(0),    // borough
            Some(0),    // census_tract
            Some(0),    // community_board
            "".to_string(),    // comparable_rental_1_address
            "".to_string(),    // comparable_rental_1_boro_block_lot
            "".to_string(),    // comparable_rental_1_building_classification
            Some(0.0),    // comparable_rental_1_dist_from_coop_in_miles
            Some(0),    // comparable_rental_1_est_gross_income
            Some(0),    // comparable_rental_1_full_market_value
            Some(0.0),    // comparable_rental_1_gross_income_per_sqft
            Some(0),    // comparable_rental_1_gross_sqft
            Some(0.0),    // comparable_rental_1_market_value_per_sqft
            "".to_string(),    // comparable_rental_1_neighborhood
            Some(0),    // comparable_rental_1_total_units
            Some(0),    // comparable_rental_1_year_built
            "".to_string(),    // comparable_rental_2_address
            "".to_string(),    // comparable_rental_2_boro_block_lot
            "".to_string(),    // comparable_rental_2_building_classification
            Some(0.0),    // comparable_rental_2_dist_from_coop_in_miles
            Some(0),    // comparable_rental_2_est_gross_income
            Some(0),    // comparable_rental_2_full_market_value
            Some(0.0),    // comparable_rental_2_gross_income_per_sqft
            Some(0),    // comparable_rental_2_gross_sqft
            Some(0.0),    // comparable_rental_2_market_value_per_sqft
            "".to_string(),    // comparable_rental_2_neighborhood
            Some(0),    // comparable_rental_2_total_units
            Some(0),    // comparable_rental_2_year_built
            Some(0),    // council_district
            Some(0.0),    // latitude
            Some(0.0),    // longitude
            "".to_string(),    // manhattan_condominium_property_address
            "".to_string(),    // manhattan_condominium_property_boro_block_lot
            "".to_string(),    // manhattan_condominium_property_building_classification
            "".to_string(),    // manhattan_condominium_property_condo_section
            Some(0),    // manhattan_condominium_property_est_gross_income
            Some(0),    // manhattan_condominium_property_full_market_value
            Some(0.0),    // manhattan_condominium_property_gross_income_per_sqft
            Some(0),    // manhattan_condominium_property_gross_sqft
            Some(0.0),    // manhattan_condominium_property_market_value_per_sqft
            "".to_string(),    // manhattan_condominium_property_neighborhood
            Some(0),    // manhattan_condominium_property_total_units
            Some(0),    // manhattan_condominium_property_year_built
            "".to_string(),    // nta
            Some(0),    // postcode
        );
        assert_eq!(property_a, property_b);
    }

}
