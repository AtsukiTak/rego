#[macro_use]
extern crate serde;

pub mod app;
pub mod domain;
mod err;
pub mod infra;

pub use err::Error;
