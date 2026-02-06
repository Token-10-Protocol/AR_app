pub mod matrix_444;

// Re-export constants and structs for easy access
pub use matrix_444::{DIM, CERTIFIED_TRACE, PHI, MonsterMatrix444};
pub mod algebra_griess;
pub mod love_operator;
pub mod keygen_evolution;
pub mod fibonacci_dimensions;
