/*!
The `config` module is for reading config variables.
*/

use config::{Config, File, FileFormat};

/// Get a param's value from the [Settings.toml](../../../Settings.toml) config file.
pub fn get_config(param: &str) -> String {
    let mut c = Config::new();

    c.merge(File::new("Settings", FileFormat::Toml).required(false))
        .unwrap();
    let value = c.get_str(param).unwrap();
    println!("{:?} = {:?}", param, value);
    value
}

// Tests.
#[cfg(test)]
mod tests {
    use crate::config::get_config;

    #[test]
    fn test_get_config() {
        assert_eq!(get_config("config"), "true");
    }
}
