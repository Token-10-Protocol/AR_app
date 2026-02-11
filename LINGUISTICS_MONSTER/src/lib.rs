//! Módulo de Lingüística Monster
//! Análisis y síntesis de lenguaje basado en Monster Group
//! PRINCIPIO: No asignar τ, extraer τ naturales de subestructuras

// Re-exportar solo la funcionalidad esencial
pub mod monster_structures;

// Funciones públicas principales
pub use monster_structures::{
    analizar_palabra,
    tau_natural,
    SubestructuraMonster,
    ErrorLinguistica,
};

// Constantes fundamentales
pub const PHI_INV_MOD_1: f64 = 0.6180339887498948; // φ⁻¹ mod 1 ≈ 0.618
pub const TOLERANCIA_TAU: f64 = 1e-6; // Tolerancia para comparaciones
#[cfg(test)] mod tests;
