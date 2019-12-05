//! Advent of code utils

pub mod interpreter;
pub mod math;

pub use self::interpreter::Interpreter;
pub use self::math::{float_eq, float_eq_eps};
