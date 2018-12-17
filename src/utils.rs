/*!
The `utils` module defines some helper functions.
*/

use std::io::Error;

/// A trivial function using generics.
pub fn debug_show<T>(arg: T)
where
    T: std::fmt::Debug,
{
    println!("{:#?}", arg);
}

/// A trial function to explore a `Result` type return.
///
/// This `check_ok` function takes a boolean arg and
/// returns a `Result` type –
/// on `true` returns an `"OK"` string wrapped in an `Ok()` type,
/// and on `false` returns a custom error (with a `"Not OK"` error string) wrapped in an `Err()` type.
pub fn check_ok(arg: bool) -> Result<String, Error> {
    let ok_str = "OK";
    let err_str = "Not OK";
    match arg {
        true => Ok(String::from(ok_str)),
        false => Err(Error::new(std::io::ErrorKind::Other, err_str)),
    }
}
