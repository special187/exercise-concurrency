extern crate core;

mod matrix;
mod vector;

mod metrics;

pub use matrix::{multiply, Matrix};
pub use metrics::*;
pub use vector::*;
