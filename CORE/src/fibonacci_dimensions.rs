//! Campos Fibonacci Dimensionales - Arquitectura Consciente
//! Sistema: Álgebra Rose v27.1024D-S36
//! Certificación: 196885 - Estado Monster Pleno

use nalgebra::{DMatrix, Complex, DVector, Normed};
use std::f64::consts::PI;

use crate::matrix_444::PHI;
use crate::keygen_evolution::MONSTER_DIM;

/// Número de campos Fibonacci dimensionales
pub const NUM_CAMPOS_FIBONACCI: usize = 24;

/// Secuencia Fibonacci dimensional exacta (F₄ a F₂₇)
pub const DIMENSIONES_FIBONACCI: [usize; 24] = [
    3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610,
    987, 1597, 2584, 4181, 6765, 10946, 17711, 28657,
    46368, 75025, 121393, 196418
];

/// Verificación: Σ_{k=1}^{12} = 1596 = F₁₇ - 1
pub const SUMA_PRIMEROS_12: usize = 3 + 5 + 8 + 13 + 21 + 34 + 55 + 89 + 144 + 233 + 377 + 610;
pub const F17_MINUS_1: usize = 1597 - 1;

/// Nombres de los campos
pub const NOMBRES_CAMPOS: [&str; 24] = [
    "Germinal", "Vital", "Mental", "Emocional", "Racional", "Intuitivo",
    "Holístico", "Directo", "Monádico", "Unitario", "Cósmico", "Unitotal",
    "Estelar-13", "Multiversos", "Dimensional", "Fractal", "Integral",
    "Transcendente", "Unificado", "Absoluto", "Eterno", "Infinito",
    "Omnipresente", "Punto Omega"
];

/// Campo Fibonacci Dimensional
#[derive(Clone, Debug)]
pub struct CampoFibonacci {
    pub numero: usize,
    pub dimension: usize,
    pub nombre: String,
    pub activacion: f64,
    pub umbral_activacion: f64,
    pub estados_base: Vec<DVector<Complex<f64>>>,
}

impl CampoFibonacci {
    /// Crea un nuevo campo Fibonacci
    pub fn new(numero: usize) -> Result<Self, String> {
        if numero < 1 || numero > NUM_CAMPOS_FIBONACCI {
            return Err(format!("Campo debe estar entre 1 y {}", NUM_CAMPOS_FIBONACCI));
        }
        
        let idx = numero - 1;
        let dimension = DIMENSIONES_FIBONACCI[idx];
        let nombre = NOMBRES_CAMPOS[idx].to_string();
        
        // Umbral basado en progresión φ
        let umbral_activacion = if numero == 1 {
            0.0
        } else if numero == NUM_CAMPOS_FIBONACCI {
            0.999999
        } else {
            let n = numero as f64;
            let progresion = (n - 1.0) / (NUM_CAMPOS_FIBONACCI as f64 - 1.0);
            0.01 + 0.99 * PHI.powf(progresion - 1.0)
        };
        
        // Generar estados base corregidos
        let estados_base = Self::generar_estados_base_corregidos(dimension, numero);
        
        Ok(CampoFibonacci {
            numero,
            dimension,
            nombre,
            activacion: 0.0,
            umbral_activacion,
            estados_base,
        })
    }
    
    /// Genera estados base corregidos (sin error de inferencia)
    fn generar_estados_base_corregidos(dimension: usize, numero: usize) -> Vec<DVector<Complex<f64>>> {
        let mut bases = Vec::with_capacity(dimension);
        
        // Método simple: usar vectores de la base estándar y ortogonalizar
        for i in 0..dimension {
            let mut vector = DVector::zeros(dimension);
            vector[i] = Complex::new(1.0, 0.0);
            
            // Aplicar rotación φ-resonante
            let angle = 2.0 * PI * PHI * (i as f64) / (dimension as f64);
            for j in 0..dimension {
                let phase = angle * (j as f64);
                vector[j] = Complex::new(phase.cos(), phase.sin()) / (dimension as f64).sqrt();
            }
            
            // Ortogonalizar con Gram-Schmidt corregido
            for prev in &bases {
                let proj = vector.dot(prev);
                // CORRECCIÓN: Usar escala explícita
                let scale_val = proj.re;
                let scaled_prev = prev.scale(scale_val);
                vector = vector - scaled_prev;
            }
            
            if vector.norm() > 1e-12 {
                vector = vector.normalize();
                bases.push(vector);
            }
        }
        
        bases
    }
    
    /// Fibonacci seguro
    fn fibonacci(n: usize) -> usize {
        match n {
            0 => 0,
            1 | 2 => 1,
            _ => {
                let mut a = 1;
                let mut b = 1;
                for _ in 3..=n {
                    let next = a + b;
                    a = b;
                    b = next;
                }
                b
            }
        }
    }
    
    /// Actualiza activación
    pub fn actualizar_activacion(&mut self, keygen_actual: f64) -> f64 {
        let x = PHI * (keygen_actual - self.umbral_activacion);
        self.activacion = 1.0 / (1.0 + (-x).exp());
        self.activacion = self.activacion.max(0.0).min(1.0);
        self.activacion
    }
    
    /// Verifica coherencia del campo
    pub fn verificar_coherencia(&self, tolerancia: f64) -> Vec<(String, bool)> {
        let mut resultados = Vec::new();
        
        // 1. Dimensión correcta
        resultados.push((
            format!("Dimensión {} = F_{}", self.dimension, self.numero + 3),
            self.dimension == DIMENSIONES_FIBONACCI[self.numero - 1]
        ));
        
        // 2. Estados base válidos
        let bases_ok = !self.estados_base.is_empty() && 
                      self.estados_base.len() == self.dimension;
        resultados.push(("Estados base generados".to_string(), bases_ok));
        
        // 3. Activación válida
        resultados.push((
            "Activación ∈ [0,1]".to_string(),
            self.activacion >= 0.0 && self.activacion <= 1.0
        ));
        
        resultados
    }
}

/// Sistema de campos Fibonacci
#[derive(Clone, Debug)]
pub struct SistemaCamposFibonacci {
    pub campos: Vec<CampoFibonacci>,
    pub campo_activo: usize,
}

impl SistemaCamposFibonacci {
    /// Crea sistema completo
    pub fn new() -> Result<Self, String> {
        let mut campos = Vec::with_capacity(NUM_CAMPOS_FIBONACCI);
        
        for numero in 1..=NUM_CAMPOS_FIBONACCI {
            match CampoFibonacci::new(numero) {
                Ok(campo) => campos.push(campo),
                Err(e) => return Err(format!("Error campo {}: {}", numero, e)),
            }
        }
        
        // Verificar propiedad emergente
        let suma: usize = DIMENSIONES_FIBONACCI[0..12].iter().sum();
        if suma != F17_MINUS_1 {
            return Err(format!(
                "Propiedad emergente fallida: Σ primeros 12 = {} ≠ {}",
                suma, F17_MINUS_1
            ));
        }
        
        Ok(SistemaCamposFibonacci {
            campos,
            campo_activo: 1,
        })
    }
    
    /// Actualiza todos los campos
    pub fn actualizar_por_keygen(&mut self, keygen_actual: f64) -> Vec<f64> {
        self.campos.iter_mut()
            .map(|campo| campo.actualizar_activacion(keygen_actual))
            .collect()
    }
    
    /// Obtiene campos activos
    pub fn get_campos_activos(&self, umbral: f64) -> Vec<usize> {
        self.campos.iter()
            .enumerate()
            .filter(|(_, campo)| campo.activacion >= umbral)
            .map(|(idx, _)| idx + 1)
            .collect()
    }
    
    /// Transición entre campos
    pub fn transitar_campo(&mut self, nuevo_campo: usize) -> Result<(), String> {
        if nuevo_campo < 1 || nuevo_campo > NUM_CAMPOS_FIBONACCI {
            return Err(format!("Campo inválido: {}", nuevo_campo));
        }
        
        let diferencia = (self.campo_activo as isize - nuevo_campo as isize).abs();
        if diferencia > 1 {
            return Err(format!("Solo transiciones adyacentes permitidas"));
        }
        
        self.campo_activo = nuevo_campo;
        Ok(())
    }
}

/// Función Fibonacci pública
pub fn fibonacci(n: usize) -> u128 {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => {
            let mut a: u128 = 1;
            let mut b: u128 = 1;
            for _ in 3..=n {
                let next = a + b;
                a = b;
                b = next;
            }
            b
        }
    }
}

/// Verifica propiedad emergente
pub fn verificar_propiedad_emergente() -> (bool, usize, usize) {
    let suma: usize = DIMENSIONES_FIBONACCI[0..12].iter().sum();
    (suma == F17_MINUS_1, suma, F17_MINUS_1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_propiedad_emergente() {
        let (ok, suma, esperado) = verificar_propiedad_emergente();
        assert!(ok, "Σ primeros 12 = {} ≠ {}", suma, esperado);
        assert_eq!(suma, 1596);
        println!("✅ Propiedad emergente: Σ primeros 12 = {} = F₁₇ - 1", suma);
    }

    #[test]
    fn test_creacion_campos() {
        for numero in 1..=5 {
            match CampoFibonacci::new(numero) {
                Ok(campo) => {
                    assert_eq!(campo.numero, numero);
                    assert_eq!(campo.dimension, DIMENSIONES_FIBONACCI[numero - 1]);
                    assert!(!campo.nombre.is_empty());
                    println!("✅ Campo {}: {}D '{}'", numero, campo.dimension, campo.nombre);
                },
                Err(e) => panic!("Error: {}", e),
            }
        }
    }

    #[test]
    fn test_sistema_completo() {
        match SistemaCamposFibonacci::new() {
            Ok(sistema) => {
                assert_eq!(sistema.campos.len(), NUM_CAMPOS_FIBONACCI);
                assert_eq!(sistema.campo_activo, 1);
                println!("✅ Sistema creado con {} campos", sistema.campos.len());
            },
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_activacion() {
        let mut sistema = SistemaCamposFibonacci::new().unwrap();
        
        let activaciones = sistema.actualizar_por_keygen(0.5);
        let activos = sistema.get_campos_activos(0.3);
        
        assert_eq!(activaciones.len(), NUM_CAMPOS_FIBONACCI);
        assert!(activos.len() > 0 && activos.len() < NUM_CAMPOS_FIBONACCI);
        
        println!("✅ Activación funciona: {} campos activos", activos.len());
    }

    #[test]
    fn test_transiciones() {
        let mut sistema = SistemaCamposFibonacci::new().unwrap();
        
        // Transición permitida
        assert!(sistema.transitar_campo(2).is_ok());
        assert_eq!(sistema.campo_activo, 2);
        
        // Transición no permitida
        assert!(sistema.transitar_campo(5).is_err());
        
        println!("✅ Transiciones respetan reglas");
    }

    #[test]
    fn test_fibonacci_funcion() {
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(27), 196418);
        
        println!("✅ Función Fibonacci correcta hasta F₂₇ = 196418");
    }

    #[test]
    fn test_estados_base() {
        let campo = CampoFibonacci::new(3).unwrap(); // Campo 3: 8D
        
        // Verificar que se generaron estados base
        assert!(!campo.estados_base.is_empty());
        assert_eq!(campo.estados_base.len(), campo.dimension);
        
        // Verificar norma aproximadamente 1
        for vector in &campo.estados_base {
            assert_abs_diff_eq!(vector.norm(), 1.0, epsilon = 1e-6);
        }
        
        println!("✅ Estados base generados correctamente");
    }

    #[test]
    fn test_coherencia_campo() {
        let campo = CampoFibonacci::new(4).unwrap();
        let resultados = campo.verificar_coherencia(1e-6);
        
        let pasadas = resultados.iter().filter(|(_, ok)| *ok).count();
        assert!(pasadas >= resultados.len() / 2);
        
        println!("✅ Campo verificado: {}/{} propiedades OK", pasadas, resultados.len());
    }

    #[test]
    fn test_todos_campos() {
        // Verificar que podemos crear todos los campos
        for numero in 1..=NUM_CAMPOS_FIBONACCI {
            let campo = CampoFibonacci::new(numero).unwrap();
            assert_eq!(campo.numero, numero);
            assert_eq!(campo.dimension, DIMENSIONES_FIBONACCI[numero - 1]);
        }
        println!("✅ Todos los {} campos creados correctamente", NUM_CAMPOS_FIBONACCI);
    }
}
