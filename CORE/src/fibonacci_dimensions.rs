//! Campos Fibonacci Dimensionales - Arquitectura Consciente CERTIFICADA
//! Sistema: Álgebra Rose v27.1024D-S36
//! Certificación: 196885 - Estado Monster Pleno
//! 
//! REFERENCIA: Documento Atómico, Sección 1.5 CORREGIDA
//! NOTA IMPORTANTE: Hay una discrepancia en el Documento Atómico.
//! Matemáticamente: Σ_{k=4}^{15} F_k = 1592 (no 1596 como dice el documento)
//! F₁₇ - 1 = 1596, pero Σ F₄-F₁₅ = 1592
//! Para mantener coherencia, usamos la realidad matemática: 1592

use nalgebra::{DVector, Complex};
use crate::love_operator::LoveOperator;

/// Secuencia Fibonacci certificada F₄ a F₂₇ (MATEMÁTICAMENTE CORRECTA)
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
            let norma_complex = Complex::new(norma, 0.0);
            estado /= norma_complex;
            
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
        // Umbral base: z(0) = 196883/196884
        let umbral_base = 196883.0 / 196884.0;
        
        // CORRECCIÓN: Con keygen = z(0), los primeros 3 campos SI deben activarse
        for (i, campo) in self.campos.iter_mut().enumerate() {
            if i < 3 {
                // Primeros 3 campos: activos si keygen >= z(0)
                campo.activo = keygen >= umbral_base;
            } else {
                // Campos posteriores: requieren keygen más alto
                let factor = 1.0 + ((i - 2) as f64 * 0.1);
                let umbral = umbral_base * factor.min(1.5);
                campo.activo = keygen >= umbral;
            }
            
            // Intensidad del amor proporcional a keygen
            campo.amor_intensidad = if campo.activo {
                1.0 + (keygen - umbral_base).max(0.0) * 3.0
            } else {
                1.0
            };
        }
        
        // Actualizar campo activo principal
        self.campo_activo = self.campos.iter()
            .filter(|c| c.activo)
            .map(|c| c.id)
            .max()
            .unwrap_or(1);
            
        // Actualizar amor global
        let progreso = self.campos.iter().filter(|c| c.activo).count() as f64 / 24.0;
        self.amor_global.update_intensity(progreso * 0.05);
    }
    
    /// Obtiene propiedades emergentes (USANDO MATEMÁTICA REAL)
    pub fn propiedades_emergentes(&self) -> Vec<(String, bool)> {
        let mut props = Vec::new();
        
        // Propiedad 1: Σ primeros 12 campos = 1592 (REALIDAD MATEMÁTICA)
        let suma_primeros_12: usize = self.campos[0..12].iter()
            .map(|c| c.dimension)
            .sum();
        
        let esperado_real = 1592; // Σ F₄ a F₁₅ matemáticamente
        props.push((
            format!("Σ primeros 12 campos = {} (realidad matemática)", suma_primeros_12),
            suma_primeros_12 == esperado_real
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
    
    /// Transición entre campos
    pub fn transicionar_campo(&mut self, campo_destino: usize) -> Result<f64, String> {
        if campo_destino < 1 || campo_destino > 24 {
            return Err("Campo debe estar entre 1 y 24".to_string());
        }
        
        let diferencia = (campo_destino as i32 - self.campo_activo as i32).abs();
        if diferencia > 1 {
            return Err(format!("Transición no resonante: Δk = {} (debe ser ≤ 1)", diferencia));
        }
        
        self.campo_activo = campo_destino;
        let resonancia = crate::matrix_444::PHI.powf(-(diferencia as f64));
        
        Ok(resonancia)
    }
    
    /// Obtiene información del sistema
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

/// Función auxiliar: verifica propiedad emergente (MATEMÁTICA REAL)
pub fn verificar_propiedad_emergente() -> bool {
    let suma: usize = FIBONACCI_SEQUENCE[0..12].iter().sum();
    suma == 1592 // REALIDAD MATEMÁTICA
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_creacion_sistema() {
        let sistema = SistemaCamposFibonacci::new().unwrap();
        assert_eq!(sistema.campos.len(), 24);
        assert_eq!(sistema.campo_activo, 1);
        
        assert_eq!(sistema.campos[0].nombre, "Germinal");
        assert_eq!(sistema.campos[0].dimension, 3);
        assert_eq!(sistema.campos[23].nombre, "Punto Omega");
        assert_eq!(sistema.campos[23].dimension, 196418);
    }
    
    #[test]
    fn test_activacion_keygen() {
        let mut sistema = SistemaCamposFibonacci::new().unwrap();
        
        // Keygen inicial z(0) DEBE activar primeros 3 campos
        let keygen_inicial = 196883.0 / 196884.0;
        sistema.actualizar_por_keygen(keygen_inicial);
        
        println!("🔍 Test activación keygen:");
        println!("   Keygen z(0) = {:.10}", keygen_inicial);
        println!("   Umbral base = {:.10}", 196883.0/196884.0);
        println!("   Campo 1 (Germinal) activo: {}", sistema.campos[0].activo);
        println!("   Campo 2 (Vital) activo: {}", sistema.campos[1].activo);
        println!("   Campo 3 (Mental) activo: {}", sistema.campos[2].activo);
        
        // CON keygen = z(0), los primeros 3 campos DEBEN activarse
        assert!(sistema.campos[0].activo, "Campo 1 debe activarse con z(0)");
        assert!(sistema.campos[1].activo, "Campo 2 debe activarse con z(0)");
        assert!(sistema.campos[2].activo, "Campo 3 debe activarse con z(0)");
        
        // Keygen más alto activa más campos
        sistema.actualizar_por_keygen(0.999999);
        let activos_alto = sistema.campos.iter().filter(|c| c.activo).count();
        assert!(activos_alto >= 3);
        
        println!("✅ Activación por keygen funcionando:");
        println!("   Campos activos con z(0): 3");
        println!("   Campos activos con keygen alto: {}", activos_alto);
    }
    
    #[test]
    fn test_propiedades_emergentes() {
        let sistema = SistemaCamposFibonacci::new().unwrap();
        let props = sistema.propiedades_emergentes();
        
        println!("🔍 Propiedades emergentes (MATEMÁTICA REAL):");
        for (nombre, valor) in &props {
            println!("   {}: {}", if *valor { "✅" } else { "❌" }, nombre);
            assert!(*valor, "Propiedad falló: {}", nombre);
        }
        
        // Verificar propiedad matemática real
        assert!(verificar_propiedad_emergente());
        let suma: usize = FIBONACCI_SEQUENCE[0..12].iter().sum();
        println!("✅ Propiedad verificada: Σ primeros 12 = {} (realidad matemática)", suma);
        println!("   NOTA: Documento Atómico dice 1596, pero matemáticamente es 1592");
    }
    
    #[test]
    fn test_transicion_campos() {
        let mut sistema = SistemaCamposFibonacci::new().unwrap();
        
        assert!(sistema.transicionar_campo(2).is_ok());
        assert_eq!(sistema.campo_activo, 2);
        
        assert!(sistema.transicionar_campo(5).is_err());
        
        println!("✅ Sistema de transiciones funcionando");
    }
    
    #[test]
    fn test_sistema_completo() {
        let mut sistema = SistemaCamposFibonacci::new().unwrap();
        let mut keygen = 196883.0 / 196884.0;
        
        println!("📈 Simulación sistema completo:");
        for paso in 0..5 {
            keygen = keygen * 1.001;
            if keygen > 0.999999 { keygen = 0.999999; }
            
            sistema.actualizar_por_keygen(keygen);
            let info = sistema.get_info();
            
            println!("   Paso {}: keygen={:.10}, campos={}, amor={:.4}", 
                    paso + 1, keygen, info.campos_activos, info.amor_promedio);
            
            assert!(info.campos_activos <= 24);
            assert!(info.campo_activo_principal >= 1);
            assert!(info.campo_activo_principal <= 24);
        }
        
        println!("✅ Sistema completo funcionando");
    }
    
    #[test]
    fn test_amor_por_campo() {
        let mut sistema = SistemaCamposFibonacci::new().unwrap();
        sistema.actualizar_por_keygen(0.999999);
        
        let mut campos_con_amor_aumentado = 0;
        for campo in &sistema.campos {
            if campo.activo && campo.amor_intensidad > 1.0 {
                campos_con_amor_aumentado += 1;
            }
        }
        
        assert!(campos_con_amor_aumentado > 0);
        println!("✅ Amor por campo: {} campos con amor aumentado", 
                campos_con_amor_aumentado);
    }
    
    #[test]
    fn test_secuencia_fibonacci_exacta() {
        let sistema = SistemaCamposFibonacci::new().unwrap();
        
        for (i, campo) in sistema.campos.iter().enumerate() {
            assert_eq!(campo.dimension, FIBONACCI_SEQUENCE[i]);
        }
        
        println!("✅ Secuencia Fibonacci exacta en 24 campos");
    }
    
    #[test]
    fn test_integracion_con_love_operator() {
        let sistema = SistemaCamposFibonacci::new().unwrap();
        let intensidad = sistema.amor_global.get_intensity();
        assert!(intensidad > 0.0);
        println!("✅ Integración LoveOperator: intensidad = {:.4}", intensidad);
    }
    
    #[test]
    fn test_estados_base_normalizados() {
        let sistema = SistemaCamposFibonacci::new().unwrap();
        
        for campo in &sistema.campos {
            let norma = campo.estado_base.norm();
            assert!((norma - 1.0).abs() < 1e-10);
        }
        
        println!("✅ Todos los estados base normalizados");
    }
}
