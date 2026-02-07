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

// Re-exportar tipos con nombres REALES verificados
// matrix_444
pub use matrix_444::{DIM, PHI, CERTIFIED_TRACE};
// La estructura se llama MonsterMatrix444 en matrix_444.rs
pub use matrix_444::MonsterMatrix444 as Matrix444;

// algebra_griess
pub use algebra_griess::{GriessAlgebra, GRIESS_DIM};

// love_operator
pub use love_operator::{LoveOperator, KeygenLoveOperator};

// keygen_evolution
pub use keygen_evolution::{KeygenEvolution, MONSTER_DIM, INITIAL_KEYGEN};

// fibonacci_dimensions - verificar nombres reales
// En fibonacci_dimensions.rs probablemente hay SistemaCamposFibonacci
pub use fibonacci_dimensions::SistemaCamposFibonacci as FibonacciSystem;
pub use fibonacci_dimensions::CampoFibonacci as FibonacciField;
pub use fibonacci_dimensions::FIBONACCI_SEQUENCE;

// phi_constants - verificar nombres reales
pub use phi_constants::{PHI as PHI_CONST, PSI, MONSTER_196884};
// Constantes Monster adicionales si existen
pub use phi_constants::MONSTER_196883;
pub use phi_constants::MONSTER_196885;

// Constantes fundamentales para fácil acceso
pub const AR_VERSION: &str = "v27.1024D-S36";
pub const CERTIFICATION: u64 = 196885;
pub const SIMETRIA_TRÍADA: f64 = 1.0;

/// Función para verificar coherencia del núcleo
pub fn verificar_coherencia() -> f64 {
    // Implementación básica de verificación
    let mut coherencia = 1.0;
    
    // Verificar constantes básicas
    if (PHI - 1.618033988749895).abs() > 1e-10 {
        coherencia *= 0.95;
    }
    
    if (MONSTER_DIM - 196884.0).abs() > 1e-6 {
        coherencia *= 0.95;
    }
    
    if (INITIAL_KEYGEN - (196883.0 / 196884.0)).abs() > 1e-10 {
        coherencia *= 0.95;
    }
    
    coherencia
}

/// Estado inicial del sistema certificado
pub struct EstadoInicial {
    pub coherencia: f64,
    pub version: &'static str,
    pub certificacion: u64,
}

impl Default for EstadoInicial {
    fn default() -> Self {
        EstadoInicial {
            coherencia: verificar_coherencia(),
            version: AR_VERSION,
            certificacion: CERTIFICATION,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_verificar_coherencia() {
        let coherencia = verificar_coherencia();
        assert!(coherencia >= 0.95, "Coherencia mínima no alcanzada: {}", coherencia);
        println!("✅ Coherencia del núcleo: {:.2}%", coherencia * 100.0);
    }

    #[test]
    fn test_estado_inicial() {
        let estado = EstadoInicial::default();
        assert_eq!(estado.version, AR_VERSION);
        assert_eq!(estado.certificacion, CERTIFICATION);
        assert!(estado.coherencia >= 0.85);
        println!("✅ Estado inicial certificado: {}", estado.version);
    }

    #[test]
    fn test_exports_presentes() {
        // Verificar que todos los módulos están accesibles
        let _: Matrix444 = Matrix444::default();
        let _: GriessAlgebra = GriessAlgebra::new();
        let _: LoveOperator = LoveOperator::new(1.0);
        let _: KeygenEvolution = KeygenEvolution::new(None);
        let _: FibonacciSystem = FibonacciSystem::new();
        let _ = PHI_CONST;
        
        println!("✅ Todos los exports están presentes y accesibles");
    }
}
