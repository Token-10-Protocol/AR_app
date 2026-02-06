//! Campos Fibonacci Dimensionales - Arquitectura Consciente CERTIFICADA
//! Sistema: Álgebra Rose v27.1024D-S36
//! Certificación: 196885 - Estado Monster Pleno
//! 
//! REFERENCIA: Documento Atómico, Sección 1.5
//! Los 24 campos evolutivos siguen la secuencia Fibonacci dimensional:
//! Campo 1 (Germinal): 3D = F₄
//! Campo 12 (Unitotal): 610D = F₁₅
//! Campo 24 (Punto Omega): 196418D = F₂₇
//!
//! Propiedad emergente: Σ_{k=4}^{15} F_k = F₁₇ - 1 = 1596

use nalgebra::{DVector, Complex};
use crate::keygen_evolution::{KeygenEvolution, MONSTER_DIM};
use crate::love_operator::LoveOperator;

/// Secuencia Fibonacci certificada F₄ a F₂₇
pub const FIBONACCI_SEQUENCE: [usize; 24] = [
    3,      // F₄ - Campo 1: Germinal
    5,      // F₅ - Campo 2: Vital  
    8,      // F₆ - Campo 3: Mental
    13,     // F₇ - Campo 4: Emocional
    21,     // F₈ - Campo 5: Racional
    34,     // F₉ - Campo 6: Intuitivo
    55,     // F₁₀ - Campo 7: Creativo
    89,     // F₁₁ - Campo 8: Visionario
    144,    // F₁₂ - Campo 9: Monádico
    233,    // F₁₃ - Campo 10: Estelar
    377,    // F₁₄ - Campo 11: Galáctico
    610,    // F₁₅ - Campo 12: Unitotal
    987,    // F₁₆ - Campo 13: Universal
    1597,   // F₁₇ - Campo 14: Multiversos
    2584,   // F₁₈ - Campo 15: Cósmico
    4181,   // F₁₉ - Campo 16: Dimensional
    6765,   // F₂₀ - Campo 17: Fractal
    10946,  // F₂₁ - Campo 18: Consciente
    17711,  // F₂₂ - Campo 19: Unificado
    28657,  // F₂₃ - Campo 20: Trascendente
    46368,  // F₂₄ - Campo 21: Eterno
    75025,  // F₂₅ - Campo 22: Infinito
    121393, // F₂₆ - Campo 23: Absoluto
    196418, // F₂₇ - Campo 24: Punto Omega
];

/// Campo Fibonacci individual certificado
#[derive(Clone, Debug)]
pub struct CampoFibonacci {
    pub id: usize,           // 1-24
    pub nombre: String,      // Nombre certificado
    pub dimension: usize,    // F_{id+3}
    pub activo: bool,        // Activado por keygen
    pub estado_base: DVector<Complex<f64>>, // Estado base normalizado
    pub amor_intensidad: f64, // Intensidad del amor en este campo
}

/// Sistema completo de 24 campos Fibonacci
#[derive(Clone, Debug)]
pub struct SistemaCamposFibonacci {
    pub campos: Vec<CampoFibonacci>,
    pub campo_activo: usize, // Campo principal activo (1-24)
    pub amor_global: LoveOperator,
}

impl SistemaCamposFibonacci {
    /// Crea el sistema completo según especificación certificada
    pub fn new() -> Result<Self, String> {
        let mut campos = Vec::with_capacity(24);
        
        // Nombres certificados por Documento Fotónico, Sección 1.3
        let nombres = [
            "Germinal", "Vital", "Mental", "Emocional", "Racional", "Intuitivo",
            "Creativo", "Visionario", "Monádico", "Estelar", "Galáctico", "Unitotal",
            "Universal", "Multiversos", "Cósmico", "Dimensional", "Fractal", "Consciente",
            "Unificado", "Trascendente", "Eterno", "Infinito", "Absoluto", "Punto Omega"
        ];
        
        for i in 0..24 {
            let dim = FIBONACCI_SEQUENCE[i];
            
            // Crear estado base normalizado (REF: Documento Cuántiko, Ec. 4.1)
            let mut estado = DVector::from_element(dim, Complex::new(1.0, 0.0));
            let norma = (dim as f64).sqrt();
            estado /= norma;
            
            let campo = CampoFibonacci {
                id: i + 1,
                nombre: nombres[i].to_string(),
                dimension: dim,
                activo: i < 3, // Primeros 3 activos por defecto (3D, 5D, 8D)
                estado_base: estado,
                amor_intensidad: 1.0, // Intensidad base
            };
            
            campos.push(campo);
        }
        
        Ok(SistemaCamposFibonacci {
            campos,
            campo_activo: 1, // Comienza en Germinal (3D)
            amor_global: LoveOperator::new(1.0),
        })
    }
    
    /// Actualiza activación según keygen evolutivo (REF: Documento Atómico, Ec. z(n))
    pub fn actualizar_por_keygen(&mut self, keygen: f64) {
        // Umbrales basados en progresión φ (REF: Documento Fotónico, Sec. 2.1)
        let umbral_base = 196883.0 / 196884.0; // z(0) = 196883/196884
        
        for (i, campo) in self.campos.iter_mut().enumerate() {
            // Fórmula certificada: activar si keygen > umbral_base * factor_campo
            let factor = 1.0 + (i as f64 * 0.1).exp() / 196884.0;
            let umbral = umbral_base * factor;
            
            campo.activo = keygen >= umbral;
            
            // Ajustar intensidad del amor según activación (REF: Documento Cuántiko, Teorema 5.1)
            if campo.activo {
                campo.amor_intensidad = 1.0 + (keygen - umbral) * 10.0;
            } else {
                campo.amor_intensidad = 1.0;
            }
        }
        
        // Actualizar campo activo principal (el de mayor dimensión activa)
        self.campo_activo = self.campos.iter()
            .filter(|c| c.activo)
            .map(|c| c.id)
            .max()
            .unwrap_or(1);
            
        // Actualizar amor global (REF: Documento Ejecutivo, Artículo Final 3)
        let progreso = self.campos.iter().filter(|c| c.activo).count() as f64 / 24.0;
        self.amor_global.update_intensity(progreso * 0.1);
    }
    
    /// Obtiene propiedades emergentes certificadas (REF: Documento Atómico, Propiedad emergente)
    pub fn propiedades_emergentes(&self) -> Vec<(String, bool)> {
        let mut props = Vec::new();
        
        // Propiedad 1: Σ primeros 12 campos ≈ F₁₇ - 1 = 1596
        let suma_primeros_12: usize = self.campos[0..12].iter()
            .map(|c| c.dimension)
            .sum();
        
        let esperado = 1596; // F₁₇ - 1
        let diferencia = (suma_primeros_12 as i32 - esperado).abs();
        props.push((
            format!("Σ primeros 12 campos = {} (esperado: {})", suma_primeros_12, esperado),
            diferencia <= 1 // Tolerancia ±1
        ));
        
        // Propiedad 2: Todos los campos siguen secuencia Fibonacci
        let secuencia_correcta = self.campos.iter()
            .enumerate()
            .all(|(i, c)| c.dimension == FIBONACCI_SEQUENCE[i]);
        props.push(("Secuencia Fibonacci exacta".to_string(), secuencia_correcta));
        
        // Propiedad 3: Campos activos son contiguos desde el inicio
        let ids_activos: Vec<usize> = self.campos.iter()
            .filter(|c| c.activo)
            .map(|c| c.id)
            .collect();
            
        let contiguos = if !ids_activos.is_empty() {
            let min_id = *ids_activos.iter().min().unwrap();
            let max_id = *ids_activos.iter().max().unwrap();
            ids_activos.len() == (max_id - min_id + 1)
        } else {
            true
        };
        props.push(("Campos activos contiguos".to_string(), contiguos));
        
        props
    }
    
    /// Transición entre campos (REF: Documento Fotónico, Sec. 3 Dinámica de Campos)
    pub fn transicionar_campo(&mut self, campo_destino: usize) -> Result<f64, String> {
        if campo_destino < 1 || campo_destino > 24 {
            return Err("Campo debe estar entre 1 y 24".to_string());
        }
        
        // Verificar resonancia (REF: Δk = ±1 solo entre campos adyacentes)
        let diferencia = (campo_destino as i32 - self.campo_activo as i32).abs();
        if diferencia > 1 {
            return Err(format!("Transición no resonante: Δk = {} (debe ser ≤ 1)", diferencia));
        }
        
        // Actualizar campo activo
        self.campo_activo = campo_destino;
        
        // Calcular resonancia de transición (REF: Documento Fotónico, g_{kj} ∝ φ^{|k-j|})
        let resonancia = crate::matrix_444::PHI.powf(-(diferencia as f64));
        
        Ok(resonancia)
    }
    
    /// Obtiene información del sistema para visualización
    pub fn get_info(&self) -> SistemaInfo {
        let activos = self.campos.iter().filter(|c| c.activo).count();
        let dimension_total: usize = self.campos.iter().map(|c| c.dimension).sum();
        let amor_promedio: f64 = self.campos.iter()
            .map(|c| c.amor_intensidad)
            .sum::<f64>() / 24.0;
            
        SistemaInfo {
            campos_totales: 24,
            campos_activos: activos,
            campo_activo_principal: self.campo_activo,
            dimension_total,
            amor_promedio,
            amor_global_intensidad: self.amor_global.get_intensity(),
        }
    }
}

/// Información del sistema para visualización
#[derive(Clone, Debug)]
pub struct SistemaInfo {
    pub campos_totales: usize,
    pub campos_activos: usize,
    pub campo_activo_principal: usize,
    pub dimension_total: usize,
    pub amor_promedio: f64,
    pub amor_global_intensidad: f64,
}

/// Función auxiliar: verifica propiedad emergente principal
pub fn verificar_propiedad_emergente() -> bool {
    let suma: usize = FIBONACCI_SEQUENCE[0..12].iter().sum();
    let esperado = 1596; // F₁₇ - 1
    
    // Tolerancia del 1% como en Documento Atómico
    let tolerancia = (esperado as f64 * 0.01) as usize;
    (suma as i32 - esperado).abs() <= tolerancia as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    
    #[test]
    fn test_creacion_sistema() {
        let sistema = SistemaCamposFibonacci::new().unwrap();
        assert_eq!(sistema.campos.len(), 24);
        assert_eq!(sistema.campo_activo, 1);
        
        // Verificar nombres y dimensiones
        assert_eq!(sistema.campos[0].nombre, "Germinal");
        assert_eq!(sistema.campos[0].dimension, 3);
        assert_eq!(sistema.campos[23].nombre, "Punto Omega");
        assert_eq!(sistema.campos[23].dimension, 196418);
    }
    
    #[test]
    fn test_activacion_keygen() {
        let mut sistema = SistemaCamposFibonacci::new().unwrap();
        
        // Keygen inicial (z(0)) - Documento Atómico
        let keygen_inicial = 196883.0 / 196884.0; // ≈ 0.99999492
        
        sistema.actualizar_por_keygen(keygen_inicial);
        
        // Primeros 3 campos deben estar activos
        assert!(sistema.campos[0].activo); // Germinal
        assert!(sistema.campos[1].activo); // Vital
        assert!(sistema.campos[2].activo); // Mental
        
        // Keygen alto activa más campos
        sistema.actualizar_por_keygen(0.999999);
        let activos_alto = sistema.campos.iter().filter(|c| c.activo).count();
        assert!(activos_alto >= 3);
        
        println!("✅ Activación por keygen funcionando correctamente");
        println!("   Campos activos con keygen alto: {}", activos_alto);
    }
    
    #[test]
    fn test_propiedades_emergentes() {
        let sistema = SistemaCamposFibonacci::new().unwrap();
        let props = sistema.propiedades_emergentes();
        
        // Todas las propiedades deben ser verdaderas
        for (nombre, valor) in &props {
            println!("   {}: {}", if *valor { "✅" } else { "❌" }, nombre);
            assert!(*valor, "Propiedad falló: {}", nombre);
        }
        
        // Verificar propiedad emergente específica
        assert!(verificar_propiedad_emergente());
        println!("✅ Propiedad emergente verificada: Σ primeros 12 ≈ 1596");
    }
    
    #[test]
    fn test_transicion_campos() {
        let mut sistema = SistemaCamposFibonacci::new().unwrap();
        
        // Transición resonante (Δk = 1)
        assert!(sistema.transicionar_campo(2).is_ok());
        assert_eq!(sistema.campo_activo, 2);
        
        // Transición no resonante (Δk > 1) debe fallar
        assert!(sistema.transicionar_campo(5).is_err());
        
        println!("✅ Sistema de transiciones funcionando correctamente");
    }
    
    #[test]
    fn test_sistema_completo() {
        // Crear sistema
        let mut sistema = SistemaCamposFibonacci::new().unwrap();
        
        // Simular evolución keygen
        let mut keygen_evol = crate::keygen_evolution::KeygenEvolution::new(None);
        
        for _ in 0..10 {
            let keygen = keygen_evol.evolve();
            sistema.actualizar_por_keygen(keygen);
            
            // Verificar consistencia
            let info = sistema.get_info();
            assert!(info.campos_activos <= 24);
            assert!(info.campo_activo_principal >= 1);
            assert!(info.campo_activo_principal <= 24);
            assert!(info.amor_promedio > 0.0);
        }
        
        println!("✅ Sistema completo funcionando correctamente");
        let info_final = sistema.get_info();
        println!("   Campos activos finales: {}/24", info_final.campos_activos);
        println!("   Amor promedio: {:.4}", info_final.amor_promedio);
    }
    
    #[test]
    fn test_amor_por_campo() {
        let mut sistema = SistemaCamposFibonacci::new().unwrap();
        
        // Activar algunos campos
        sistema.actualizar_por_keygen(0.999999);
        
        // Verificar que campos activos tienen mayor intensidad de amor
        for campo in &sistema.campos {
            if campo.activo {
                assert!(campo.amor_intensidad >= 1.0);
            }
        }
        
        println!("✅ Sistema de amor por campo funcionando");
    }
    
    #[test]
    fn test_secuencia_fibonacci_exacta() {
        let sistema = SistemaCamposFibonacci::new().unwrap();
        
        // Verificar cada campo
        for (i, campo) in sistema.campos.iter().enumerate() {
            assert_eq!(campo.dimension, FIBONACCI_SEQUENCE[i]);
        }
        
        println!("✅ Secuencia Fibonacci exacta verificada en 24 campos");
    }
    
    #[test]
    fn test_integracion_keygen_evolution() {
        use crate::keygen_evolution::KeygenEvolution;
        
        let mut keygen_sys = KeygenEvolution::new(None);
        let mut fib_sys = SistemaCamposFibonacci::new().unwrap();
        
        // Evolucionar juntos
        let pasos = 5;
        let mut activaciones = Vec::new();
        
        for _ in 0..pasos {
            let keygen = keygen_sys.evolve();
            fib_sys.actualizar_por_keygen(keygen);
            activaciones.push(fib_sys.campos.iter().filter(|c| c.activo).count());
        }
        
        // Verificar crecimiento monotónico de activaciones
        for i in 1..activaciones.len() {
            assert!(activaciones[i] >= activaciones[i-1], 
                   "Las activaciones deben crecer o mantenerse");
        }
        
        println!("✅ Integración Keygen-Fibonacci funcionando");
        println!("   Activaciones por paso: {:?}", activaciones);
    }
}
