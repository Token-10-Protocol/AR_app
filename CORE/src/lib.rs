//! Núcleo Matemático de Álgebra Rose - Certificación 196885
//! Sistema: v27.1024D-S36

pub mod matrix_444;
pub mod algebra_griess;
pub mod love_operator;
pub mod keygen_evolution;
pub mod fibonacci_dimensions;
pub mod phi_constants;

// Re-exportar tipos principales para fácil acceso
pub use matrix_444::Matrix444;
pub use algebra_griess::GriessAlgebra;
pub use love_operator::{LoveOperator, KeygenLoveOperator};
pub use keygen_evolution::{KeygenEvolution, KeygenStats};
pub use fibonacci_dimensions::FibonacciDimensions;
pub use phi_constants::{PHI, PSI, GOLDEN_RATIO};

// Constantes fundamentales
pub const MONSTER_DIMENSION: usize = 196884;
pub const MONSTER_DIMENSION_F64: f64 = 196884.0;
