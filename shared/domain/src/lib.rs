// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
#[cfg(feature = "query")]
#[macro_use]
extern crate diesel_derive_newtype;

pub mod user;

pub mod ids;

pub use user::{Username, Password};
