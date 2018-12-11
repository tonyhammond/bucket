/*!
The `datasets` module is for describing data structures.
*/

/// A `City` struct for a CSV record.
#[derive(Debug, Deserialize)]
pub struct City {
    name: String,
    population: Option<i32>,
    latitude: Option<f32>,
    longitude: Option<f32>,
}
