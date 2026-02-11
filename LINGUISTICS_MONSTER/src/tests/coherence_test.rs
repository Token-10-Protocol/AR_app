//! Test de coherencia para Lingüística Monster
//! Verifica propiedad fundamental: "amor" → τ ≈ φ⁻¹

#[cfg(test)]
mod tests {
    use crate::monster_structures::ParametrosArticulatorios;
    use crate::{analizar_palabra, tau_natural};

    const PHI_INV: f64 = 0.6180339887498948; // φ⁻¹ mod 1
    const TOLERANCIA_ALTA: f64 = 1e-6;  // Para prueba inicial
    const TOLERANCIA_MEDIA: f64 = 1e-9; // Objetivo
    const TOLERANCIA_ALTA_PRECISION: f64 = 1e-12; // Máximo teórico

    #[test]
    fn test_parametros_fonemas_validos() {
        // Verificar que todos los parámetros estén en [0, 1]
        let fonemas = [
            ParametrosArticulatorios::fonema_a(),
            ParametrosArticulatorios::fonema_m(),
            ParametrosArticulatorios::fonema_o(),
            ParametrosArticulatorios::fonema_r(),
        ];

        for (i, fonema) in fonemas.iter().enumerate() {
            let nombre = match i {
                0 => "a",
                1 => "m",
                2 => "o",
                3 => "r",
                _ => "?",
            };

            match fonema.validar() {
                Ok(()) => println!("✅ Fonema /{}/ válido", nombre),
                Err(e) => panic!("❌ Fonema /{}/ inválido: {}", nombre, e),
            }
        }
    }

    #[test]
    fn test_tau_individual_fonemas() {
        // Verificar que cada fonema produce τ ∈ [0, 1)
        let fonemas = [
            ("a", ParametrosArticulatorios::fonema_a()),
            ("m", ParametrosArticulatorios::fonema_m()),
            ("o", ParametrosArticulatorios::fonema_o()),
            ("r", ParametrosArticulatorios::fonema_r()),
        ];

        for (nombre, fonema) in fonemas {
            let subestructura = fonema.a_subestructura_monster();
            let tau = subestructura.tau;
            
            assert!(
                (0.0..1.0).contains(&tau),
                "τ para /{}/ = {} fuera de [0, 1)",
                nombre, tau
            );
            
            println!("✅ /{}/ → τ = {:.12}", nombre, tau);
        }
    }

    #[test]
    fn test_propiedad_amor_phi_inv() {
        // PROPIEDAD FUNDAMENTAL: "amor" debe aproximarse a φ⁻¹
        let tau_resultado = tau_natural("amor").expect("Error analizando 'amor'");
        
        println!("τ('amor') = {:.16}", tau_resultado);
        println!("φ⁻¹ = {:.16}", PHI_INV);
        println!("Diferencia = {:.16e}", (tau_resultado - PHI_INV).abs());
        
        // Verificación con tolerancia alta
        let diferencia = (tau_resultado - PHI_INV).abs();
        assert!(
            diferencia < TOLERANCIA_ALTA,
            "τ('amor') = {:.12}, φ⁻¹ = {:.12}, diferencia = {:.2e} > {:.0e}",
            tau_resultado, PHI_INV, diferencia, TOLERANCIA_ALTA
        );
        
        // Reportar precisión lograda
        if diferencia < TOLERANCIA_ALTA_PRECISION {
            println!("🎉 ¡ÉXITO INCREÍBLE! τ('amor') ≈ φ⁻¹ dentro de {:.1e}", TOLERANCIA_ALTA_PRECISION);
        } else if diferencia < TOLERANCIA_MEDIA {
            println!("🎉 ¡ÉXITO! τ('amor') ≈ φ⁻¹ dentro de {:.1e}", TOLERANCIA_MEDIA);
        } else if diferencia < TOLERANCIA_ALTA {
            println!("⚠️  Advertencia: Diferencia {:.2e}, dentro de {:.0e} pero no de {:.0e}", 
                    diferencia, TOLERANCIA_ALTA, TOLERANCIA_MEDIA);
        }
    }

    #[test]
    fn test_analisis_palabra_completo() {
        let palabra = "amor";
        let resultado = analizar_palabra(palabra).expect("Error analizando palabra");
        
        assert_eq!(resultado.len(), 4, "Debe haber 4 subestructuras para 'amor'");
        
        // Verificar orden: a-m-o-r
        let fonemas = ["a", "m", "o", "r"];
        for (i, (sub, fonema)) in resultado.iter().zip(fonemas.iter()).enumerate() {
            println!("{}: /{}/ → τ = {:.12}, complejidad = {:.6}, clase = {}", 
                    i, fonema, sub.tau, sub.complejidad, sub.clase_conjugacion);
            
            assert!((0.0..1.0).contains(&sub.tau), 
                   "τ para /{}/ fuera de rango", fonema);
            assert!((0.0..1.0).contains(&sub.complejidad),
                   "Complejidad para /{}/ fuera de rango", fonema);
        }
    }

    #[test]
    fn test_composicion_fonemica() {
        // Verificar que τ("amor") ≠ promedio simple de τ individuales
        // Esto demostraría que hay composición algebraica real
        
        let tau_a = ParametrosArticulatorios::fonema_a().a_subestructura_monster().tau;
        let tau_m = ParametrosArticulatorios::fonema_m().a_subestructura_monster().tau;
        let tau_o = ParametrosArticulatorios::fonema_o().a_subestructura_monster().tau;
        let tau_r = ParametrosArticulatorios::fonema_r().a_subestructura_monster().tau;
        
        let promedio_simple = (tau_a + tau_m + tau_o + tau_r) / 4.0;
        let tau_combinado = tau_natural("amor").unwrap();
        
        let diferencia = (tau_combinado - promedio_simple).abs();
        
        println!("τ promedio simple: {:.12}", promedio_simple);
        println!("τ combinado 'amor': {:.12}", tau_combinado);
        println!("Diferencia: {:.12}", diferencia);
        
        // La diferencia debe ser significativa (> 0.001) para mostrar composición no lineal
        assert!(
            diferencia > 0.001,
            "Composición parece lineal (diferencia = {:.6} ≤ 0.001)",
            diferencia
        );
        
        println!("✅ Composición no lineal verificada (diferencia = {:.6})", diferencia);
    }

    // Test adicional: palabras con diferentes combinaciones
    #[test]
    fn test_variaciones_palabras() {
        let palabras = ["amor", "roma", "mar", "oro", "ramo"];
        
        for palabra in palabras {
            match tau_natural(palabra) {
                Ok(tau) => {
                    println!("τ('{}') = {:.12}", palabra, tau);
                    assert!((0.0..1.0).contains(&tau), 
                           "τ('{}') = {} fuera de [0, 1)", palabra, tau);
                }
                Err(e) => {
                    // Palabras con caracteres no implementados pueden fallar
                    println!("⚠️  '{}': {}", palabra, e);
                }
            }
        }
    }
}
