/*!
The `city` module defines the `City` struct and implementation.
*/

//extern crate serde;

/// A `City` struct for a CSV record.
#[derive(Debug, Deserialize)]
#[derive(PartialEq, PartialOrd)]
pub struct City {
    // pub id: i32,
    // #[serde(default)]
    pub name: String,
    // #[serde(default)]
    pub population: Option<i32>,
    // #[serde(default)]
    pub latitude: Option<f64>,
    // #[serde(default)]
    pub longitude: Option<f64>,
}

// Implement the Default trait.
impl Default for City {
    fn default() -> City {
        City {
            name: "".to_string(),
            population: Some(0),
            latitude: Some(0.0),
            longitude: Some(0.0),
        }
    }
}

// Methods
impl City {
    /// Demo method `lat_long()` defined on `City` to return a lat/long tuple.
    pub fn lat_long(&self) -> (f64, f64) {
        (self.latitude.unwrap(), self.longitude.unwrap())
    }

    /// Method to get the name field.
    pub fn get_name(&self) -> &str {
        &self.name
    }
    /// Method to set the name field.
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

// Associated functions.
impl City {
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

// Tests.
#[cfg(test)]
mod tests {
    use crate::datasets::city::City;

    #[test]
    fn test_city_default() {
        let city_a = City::default();
        let city_b = City::new(
            "".to_string(),
            Some(0),
            Some(0.0),
            Some(0.0),
        );
        assert_eq!(city_a, city_b);
    }
}
