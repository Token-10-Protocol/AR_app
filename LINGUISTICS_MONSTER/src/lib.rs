//! Módulo de Lingüística Monster
//! Análisis y síntesis de lenguaje basado en Monster Group
//! PRINCIPIO: No asignar τ, extraer τ naturales de subestructuras

// Re-exportar solo la funcionalidad esencial
pub mod monster_structures;

// Funciones públicas principales
pub use monster_structures::{
    analizar_palabra,
    analizar_palabra_extendida,
    tau_natural,
    tau_natural_extendido,
    SubestructuraMonster,
    ErrorLinguistica,
    ParametrosArticulatorios,
};

// Constantes fundamentales
pub const PHI_INV_MOD_1: f64 = 0.6180339887498948; // φ⁻¹ mod 1 ≈ 0.618
pub const TOLERANCIA_TAU: f64 = 1e-6; // Tolerancia para comparaciones

// Test simple integrado
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_amor_5_idiomas_simple() {
        println!("\n🌍 PRUEBA MULTILINGÜE SIMPLIFICADA:");
        println!("{}", "=".repeat(50));
        
        let pruebas = [
            ("Español", "amor"),
            ("Inglés", "love"),
            ("Francés", "amour"),
            ("Italiano", "amore"),
            ("Portugués", "amor"),
        ];
        
        let phi_inv = 0.6180339887498948;
        let mut resultados = Vec::new();
        
        for (idioma, palabra) in pruebas {
            match tau_natural_extendido(palabra) {
                Ok(tau) => {
                    let diff = (tau - phi_inv).abs();
                    let ok = diff < 0.05; // 5% de tolerancia
                    println!("{} '{}': τ = {:.6}, diff = {:.6} {}", 
                            idioma, palabra, tau, diff, if ok { "✅" } else { "⚠️" });
                    resultados.push((idioma, ok));
                    
                    // Español y Portugués deben ser exactos
                    if idioma == "Español" || idioma == "Portugués" {
                        assert!(diff < 1e-6, "{} debería ser exacto", idioma);
                    }
                }
                Err(e) => {
                    println!("{} '{}': ERROR: {}", idioma, palabra, e);
                    resultados.push((idioma, false));
                }
            }
        }
        
        let exitos = resultados.iter().filter(|(_, ok)| *ok).count();
        println!("\n✅ {}/5 palabras cerca de φ⁻¹", exitos);
        
        // Al menos 3 de 5 deberían estar cerca de φ⁻¹
        assert!(exitos >= 3, "Solo {}/5 cerca de φ⁻¹", exitos);
    }
    
    #[test]
    fn test_fonemas_basicos() {
        let fonemas = [
            ParametrosArticulatorios::fonema_a(),
            ParametrosArticulatorios::fonema_m(),
            ParametrosArticulatorios::fonema_o(),
            ParametrosArticulatorios::fonema_r(),
            ParametrosArticulatorios::fonema_l(),
            ParametrosArticulatorios::fonema_v(),
            ParametrosArticulatorios::fonema_u(),
            ParametrosArticulatorios::fonema_e(),
        ];
        
        for (i, fonema) in fonemas.iter().enumerate() {
            let nombre = match i {
                0 => "a", 1 => "m", 2 => "o", 3 => "r",
                4 => "l", 5 => "v", 6 => "u", 7 => "e",
                _ => "?",
            };
            
            assert!(fonema.validar().is_ok(), "Fonema /{}/ inválido", nombre);
            
            let tau = fonema.a_subestructura_monster().tau;
            assert!((0.0..1.0).contains(&tau), 
                   "τ(/{}/) = {} fuera de [0,1)", nombre, tau);
            
            println!("✅ /{}/ → τ = {:.6}", nombre, tau);
        }
    }
}
