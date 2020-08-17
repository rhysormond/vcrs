//! Also known as `vcrs`.
//!
//! Code is in the repo. Smash your keyboard with a terminal open to use the CLI powered by the library these docs fail
//! to do justice to.
//!
//! # Basic usage
//! idk how this lib works
pub use cat_file::cat_file;
pub use checkout::checkout;
pub use hash_object::hash_object;
pub use init::init;
pub use log::log;

mod cat_file;
mod checkout;
mod hash_object;
mod init;
mod log;
mod object;
mod reference;
mod repository;
