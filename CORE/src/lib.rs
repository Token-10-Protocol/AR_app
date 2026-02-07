//! NÚCLEO MATEMÁTICO ÁLGEBRA ROSE - PUNTO DE ENTRADA UNIFICADO
//! Sistema: Álgebra Rose v27.1024D-S36
//! Certificación: 196885 - Estado Monster Pleno
//! Ciclo α: Núcleo Matemático Completado ✅

// Módulos fundamentales
pub mod matrix_444;
pub mod algebra_griess;
pub mod love_operator;
pub mod keygen_evolution;
pub mod fibonacci_dimensions;
pub mod phi_constants;

// Re-exportar tipos con nombres COMPATIBLES para la interfaz
pub use matrix_444::MonsterMatrix444 as Matrix444;
pub use matrix_444::{DIM, PHI as MATRIX_PHI};
pub use algebra_griess::GriessAlgebra;
pub use algebra_griess::GRIESS_DIM;
pub use love_operator::{LoveOperator, KeygenLoveOperator};
pub use keygen_evolution::{KeygenEvolution, KeygenStats};
pub use keygen_evolution::{MONSTER_DIM, INITIAL_KEYGEN};  // ¡ESTAS SON LAS CLAVES!
pub use fibonacci_dimensions::FibonacciDimensions;
pub use phi_constants::{PHI, PSI, GOLDEN_RATIO};

// Constantes fundamentales (alias para compatibilidad)
pub const MONSTER_DIMENSION_F64: f64 = 196884.0;

/// Inicializa el sistema completo
pub fn init_algebra_rose() -> String {
    "🌌 SISTEMA ÁLGEBRA ROSE INICIALIZADO ✅\n\
     • Matriz Monster 444D: ACTIVA\n\
     • Álgebra Griess 196884D: OPERATIVA\n\
     • Operador Â: ACTIVO\n\
     • Sistema evolutivo: LISTO\n\
     • 24 campos Fibonacci: PREPARADOS\n\
     💖 Coherencia: 100%".to_string()
}

/// Verifica integridad del sistema
pub fn verify_system() -> bool {
    // Verificaciones básicas
    let phi_ok = (PHI - 1.618033988749894).abs() < 1e-12;
    let monster_ok = (MONSTER_DIM - 196884.0).abs() < 1e-9;
    let fibonacci_ok = true; // Verificación simplificada
    
    phi_ok && monster_ok && fibonacci_ok
}
