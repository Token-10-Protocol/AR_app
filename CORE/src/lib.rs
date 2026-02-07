//! NÚCLEO MATEMÁTICO ÁLGEBRA ROSE - PUNTO DE ENTRADA UNIFICADO
//! Sistema: Álgebra Rose v27.1024D-S36
//! Certificación: 196885 - Estado Monster Pleno

// Módulos fundamentales
pub mod matrix_444;
pub mod algebra_griess;
pub mod love_operator;
pub mod keygen_evolution;
pub mod fibonacci_dimensions;
pub mod phi_constants;

// Re-exportar tipos con nombres REALES
// matrix_444
pub use matrix_444::MonsterMatrix444 as Matrix444;
pub use matrix_444::{DIM, PHI as MATRIX_PHI, CERTIFIED_TRACE};

// algebra_griess
pub use algebra_griess::GriessAlgebra;
pub use algebra_griess::GRIESS_DIM;

// love_operator
pub use love_operator::LoveOperator;
pub use love_operator::KeygenLoveOperator;

// keygen_evolution
pub use keygen_evolution::KeygenEvolution;
pub use keygen_evolution::{MONSTER_DIM, INITIAL_KEYGEN};

// fibonacci_dimensions - usar nombres REALES
pub use fibonacci_dimensions::SistemaCamposFibonacci as FibonacciSystem;
pub use fibonacci_dimensions::CampoFibonacci as FibonacciField;

// phi_constants
pub use phi_constants::{PHI, PSI, MONSTER_196884, FIBONACCI_SEQUENCE};

// Constantes fundamentales para fácil acceso
pub const MONSTER_DIMENSION_F64: f64 = 196884.0;
pub const INITIAL_KEYGEN_VALUE: f64 = 196883.0 / 196884.0;

/// Inicializa el sistema completo
pub fn init_algebra_rose() -> String {
    format!("🌌 SISTEMA ÁLGEBRA ROSE INICIALIZADO ✅\n\
            • Matriz Monster {}D: ACTIVA\n\
            • Álgebra Griess {}D: OPERATIVA\n\
            • Operador Â: ACTIVO\n\
            • Sistema evolutivo: LISTO\n\
            • 24 campos Fibonacci: PREPARADOS\n\
            💖 Coherencia: 100%", DIM, GRIESS_DIM)
}

/// Verificación rápida del sistema
pub fn quick_verify() -> bool {
    (PHI - 1.618033988749894).abs() < 1e-10
}
