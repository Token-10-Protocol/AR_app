//! Test multilingüe para verificar "amor" en 5 idiomas

#[cfg(test)]
mod tests {
    use crate::monster_structures::ParametrosArticulatorios;
    use crate::tau_natural;

    const PHI_INV: f64 = 0.6180339887498948;
    const TOLERANCIA_AMOR: f64 = 0.05; // 5% de tolerancia para otras lenguas

    /// Mapeo de fonemas extendido para múltiples idiomas
    fn parametros_para_caracter(c: char) -> Option<ParametrosArticulatorios> {
        match c.to_lowercase().next()? {
            // Español básico (ya existen)
            'a' => Some(ParametrosArticulatorios::fonema_a()),
            'm' => Some(ParametrosArticulatorios::fonema_m()),
            'o' => Some(ParametrosArticulatorios::fonema_o()),
            'r' => Some(ParametrosArticulatorios::fonema_r()),
            
            // Nuevos fonemas internacionales
            'l' => Some(ParametrosArticulatorios::fonema_l()),
            'v' => Some(ParametrosArticulatorios::fonema_v()),
            'u' => Some(ParametrosArticulatorios::fonema_u()),
            'e' => Some(ParametrosArticulatorios::fonema_e()),
            'c' => Some(ParametrosArticulatorios::fonema_sh()), // para "ch" en francés
            'g' => Some(ParametrosArticulatorios::fonema_french_r()), // para "r" francés
            't' => Some(ParametrosArticulatorios::fonema_th_voiced()), // para "th"
            'n' => Some(ParametrosArticulatorios::fonema_ng()), // para "ng"
            's' => Some(ParametrosArticulatorios::fonema_sh()), // aproximación
            'h' => Some(ParametrosArticulatorios::fonema_sh()), // aproximación
            
            _ => None,
        }
    }
    
    fn tau_natural_extendido(palabra: &str) -> Option<f64> {
        let mut taus = Vec::new();
        
        for c in palabra.chars() {
            if let Some(parametros) = parametros_para_caracter(c) {
                let subestructura = parametros.a_subestructura_monster();
                if subestructura.tau > 1e-12 {
                    taus.push(subestructura.tau);
                }
            }
        }
        
        if taus.is_empty() {
            return None;
        }
        
        // Promedio geométrico
        let producto: f64 = taus.iter().product();
        Some(producto.powf(1.0 / taus.len() as f64) % 1.0)
    }

    #[test]
    fn test_amor_5_idiomas() {
        let palabras = [
            ("Español", "amor"),
            ("Inglés", "love"),
            ("Francés", "amour"),
            ("Italiano", "amore"),
            ("Portugués", "amor"), // igual que español
        ];
        
        println!("\n🌍 ANÁLISIS MULTILINGÜE DE 'AMOR':");
        println!("=" .repeat(50));
        
        for (idioma, palabra) in palabras {
            if let Some(tau) = tau_natural_extendido(palabra) {
                let diferencia = (tau - PHI_INV).abs();
                let porcentaje = (diferencia / PHI_INV) * 100.0;
                
                let resultado = if diferencia < TOLERANCIA_AMOR {
                    "✅"
                } else {
                    "⚠️ "
                };
                
                println!("{} {}: '{}'", resultado, idioma, palabra);
                println!("   τ = {:.12}", tau);
                println!("   φ⁻¹ = {:.12}", PHI_INV);
                println!("   Diferencia = {:.4} ({:.1}%)", diferencia, porcentaje);
                
                if idioma == "Español" || idioma == "Portugués" {
                    // Estos deberían ser exactos por nuestro algoritmo especial
                    assert!(
                        diferencia < 1e-6,
                        "{}: τ = {}, diferencia = {} > 1e-6",
                        idioma, tau, diferencia
                    );
                } else {
                    // Para otros idiomas, tolerancia del 5%
                    assert!(
                        diferencia < TOLERANCIA_AMOR,
                        "{}: τ = {}, diferencia = {} > {}",
                        idioma, tau, diferencia, TOLERANCIA_AMOR
                    );
                }
            } else {
                println!("❌ {}: '{}' - No se pudo analizar", idioma, palabra);
            }
            println!();
        }
        
        // Test adicional: verificar que "love" no sea igual a "amor" exactamente
        let tau_amor = tau_natural_extendido("amor").unwrap();
        let tau_love = tau_natural_extendido("love").unwrap();
        
        let diferencia_amor_love = (tau_amor - tau_love).abs();
        println!("Comparación 'amor' vs 'love':");
        println!("  τ('amor') = {:.12}", tau_amor);
        println!("  τ('love') = {:.12}", tau_love);
        println!("  Diferencia = {:.12}", diferencia_amor_love);
        
        // Deberían ser diferentes pero ambos cerca de φ⁻¹
        assert!(
            diferencia_amor_love > 0.001,
            "'amor' y 'love' son demasiado similares: diferencia = {}",
            diferencia_amor_love
        );
    }
    
    #[test]
    fn test_fonemas_internacionales_validos() {
        let fonemas = [
            ('l', ParametrosArticulatorios::fonema_l()),
            ('v', ParametrosArticulatorios::fonema_v()),
            ('u', ParametrosArticulatorios::fonema_u()),
            ('e', ParametrosArticulatorios::fonema_e()),
        ];
        
        for (nombre, fonema) in fonemas {
            match fonema.validar() {
                Ok(()) => println!("✅ Fonema /{}/ internacional válido", nombre),
                Err(e) => panic!("❌ Fonema /{}/ inválido: {}", nombre, e),
            }
            
            let tau = fonema.a_subestructura_monster().tau;
            assert!(
                (0.0..1.0).contains(&tau),
                "τ para /{}/ = {} fuera de [0, 1)",
                nombre, tau
            );
        }
    }
}
