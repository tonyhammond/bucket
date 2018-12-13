/*!
The `utils` module defines some helper functions.
*/

/// A trivial function using generics.
pub fn debug_show<T>(arg: T)
where
    T: std::fmt::Debug,
{
    println!("{:#?}", arg);
}
