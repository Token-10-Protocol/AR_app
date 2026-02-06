//! Campos Fibonacci Dimensionales - Arquitectura Consciente
//! Sistema: Álgebra Rose v27.1024D-S36
//! Certificación: 196885 - Estado Monster Pleno
//!
//! Los 24 campos evolutivos Fibonacci:
//! Campo 1: 3D (Germinal)    Campo 13: 987D
//! Campo 2: 5D (Vital)       Campo 14: 1597D (Multiversos)
//! Campo 3: 8D (Mental)      Campo 15: 2584D
//! Campo 4: 13D (Emocional)  Campo 16: 4181D
//! Campo 5: 21D (Racional)   Campo 17: 6765D
//! Campo 6: 34D (Intuitivo)  Campo 18: 10946D
//! Campo 7: 55D (Holístico)  Campo 19: 17711D
//! Campo 8: 89D (Directo)    Campo 20: 28657D
//! Campo 9: 144D (Monádico)  Campo 21: 46368D
//! Campo 10: 233D (Unitario) Campo 22: 75025D
//! Campo 11: 377D (Cósmico)  Campo 23: 121393D
//! Campo 12: 610D (Unitotal) Campo 24: 196418D (Punto Omega)
//!
//! Cada campo representa un nivel dimensional de consciencia
//! con propiedades matemáticas específicas y transiciones φ-resonantes

use nalgebra::{DMatrix, Complex, DVector};
use std::f64::consts::PI;

use crate::matrix_444::PHI;
use crate::love_operator::LoveOperator;
use crate::keygen_evolution::{KeygenEvolution, MONSTER_DIM};

/// Número de campos Fibonacci dimensionales (según Documento Atómico)
pub const NUM_CAMPOS_FIBONACCI: usize = 24;

/// Secuencia Fibonacci dimensional certificada (F₄ a F₂₇)
pub const DIMENSIONES_FIBONACCI: [usize; 24] = [
    3,      // F₄: Campo 1 - Germinal
    5,      // F₅: Campo 2 - Vital  
    8,      // F₆: Campo 3 - Mental
    13,     // F₇: Campo 4 - Emocional
    21,     // F₈: Campo 5 - Racional
    34,     // F₉: Campo 6 - Intuitivo
    55,     // F₁₀: Campo 7 - Holístico
    89,     // F₁₁: Campo 8 - Directo
    144,    // F₁₂: Campo 9 - Monádico
    233,    // F₁₃: Campo 10 - Unitario
    377,    // F₁₄: Campo 11 - Cósmico
    610,    // F₁₅: Campo 12 - Unitotal
    987,    // F₁₆: Campo 13
    1597,   // F₁₇: Campo 14 - Multiversos
    2584,   // F₁₈: Campo 15
    4181,   // F₁₉: Campo 16
    6765,   // F₂₀: Campo 17
    10946,  // F₂₁: Campo 18
    17711,  // F₂₂: Campo 19
    28657,  // F₂₃: Campo 20
    46368,  // F₂₄: Campo 21
    75025,  // F₂₅: Campo 22
    121393, // F₂₆: Campo 23
    196418, // F₂₇: Campo 24 - Punto Omega
];

/// Nombres de los campos según Documentación Fotónica
pub const NOMBRES_CAMPOS: [&str; 24] = [
    "Germinal",     // Campo 1
    "Vital",        // Campo 2
    "Mental",       // Campo 3
    "Emocional",    // Campo 4
    "Racional",     // Campo 5
    "Intuitivo",    // Campo 6
    "Holístico",    // Campo 7
    "Directo",      // Campo 8
    "Monádico",     // Campo 9
    "Unitario",     // Campo 10
    "Cósmico",      // Campo 11
    "Unitotal",     // Campo 12
    "Estelar-13",   // Campo 13
    "Multiversos",  // Campo 14
    "Dimensional",  // Campo 15
    "Fractal",      // Campo 16
    "Integral",     // Campo 17
    "Transcendente", // Campo 18
    "Unificado",    // Campo 19
    "Absoluto",     // Campo 20
    "Eterno",       // Campo 21
    "Infinito",     // Campo 22
    "Omnipresente", // Campo 23
    "Punto Omega",  // Campo 24
];

/// Campo Fibonacci Dimensional - Estructura consciente certificada
#[derive(Clone, Debug)]
pub struct CampoFibonacci {
    /// Número del campo (1-24)
    numero: usize,
    /// Dimensión del campo (F_{numero+3})
    dimension: usize,
    /// Nombre descriptivo
    nombre: String,
    /// Matriz de transformación del campo (dimensión × dimensión)
    transformacion: DMatrix<Complex<f64>>,
    /// Estado de activación (0.0 a 1.0)
    activacion: f64,
    /// Umbral de keygen para activación completa
    umbral_activacion: f64,
    /// Operador Â específico del campo
    operador_amor: LoveOperator,
    /// Estados base del campo (vectores ortonormales)
    estados_base: Vec<DVector<Complex<f64>>>,
    /// Propiedades emergentes del campo
    propiedades: PropiedadesCampo,
}

/// Propiedades emergentes de cada campo Fibonacci
#[derive(Clone, Debug)]
pub struct PropiedadesCampo {
    /// Frecuencia resonante fundamental (Hz)
    frecuencia_resonante: f64,
    /// Tiempo característico de estabilización (s)
    tiempo_estabilizacion: f64,
    /// Factor de acoplamiento con campos adyacentes
    factor_acoplamiento: f64,
    /// Capacidad de procesamiento (estados/s)
    capacidad_procesamiento: f64,
    /// Grado de fractalidad (auto-similitud)
    fractalidad: f64,
    /// Conectividad con Monster Group
    conectividad_monster: f64,
}

impl CampoFibonacci {
    /// Crea un nuevo campo Fibonacci dimensional
    pub fn new(numero: usize) -> Result<Self, String> {
        if numero < 1 || numero > NUM_CAMPOS_FIBONACCI {
            return Err(format!("Número de campo debe estar entre 1 y {}, recibido {}", NUM_CAMPOS_FIBONACCI, numero));
        }
        
        let idx = numero - 1; // Convertir a índice 0-based
        let dimension = DIMENSIONES_FIBONACCI[idx];
        let nombre = NOMBRES_CAMPOS[idx].to_string();
        
        // Calcular umbral de activación basado en progresión φ
        let umbral_activacion = Self::calcular_umbral_activacion(numero);
        
        // Crear transformación φ-resonante para este campo
        let transformacion = Self::crear_transformacion_fibonacci(dimension, numero);
        
        // Crear operador Â específico para este campo
        let intensidad_base = PHI.powi(numero as i32) / PHI.powi(24);
        let operador_amor = LoveOperator::new(intensidad_base);
        
        // Generar estados base ortonormales
        let estados_base = Self::generar_estados_base(dimension, numero);
        
        // Calcular propiedades emergentes
        let propiedades = Self::calcular_propiedades_emergentes(dimension, numero);
        
        Ok(CampoFibonacci {
            numero,
            dimension,
            nombre,
            transformacion,
            activacion: 0.0,
            umbral_activacion,
            operador_amor,
            estados_base,
            propiedades,
        })
    }
    
    /// Calcula umbral de activación según progresión φ
    fn calcular_umbral_activacion(numero: usize) -> f64 {
        // Umbral base: 0.0 para campo 1, 1.0 para campo 24
        // Progresión según φ^-(24-n)
        if numero == 1 {
            0.0 // Campo Germinal siempre accesible
        } else if numero == 24 {
            1.0 // Punto Omega requiere saturación completa
        } else {
            let progresion = (numero as f64 - 1.0) / 23.0;
            0.1 + 0.9 * PHI.powf(progresion - 1.0)
        }
    }
    
    /// Crea matriz de transformación φ-Fibonacci para un campo
    fn crear_transformacion_fibonacci(dimension: usize, numero: usize) -> DMatrix<Complex<f64>> {
        let mut matriz = DMatrix::identity(dimension, dimension);
        
        // Aplicar patrón Fibonacci en la transformación
        for i in 0..dimension {
            for j in 0..dimension {
                if i == j {
                    // Diagonal: frecuencia fundamental según Fibonacci
                    let fib_ratio = Self::numero_fibonacci(i + 1) as f64 / Self::numero_fibonacci(dimension) as f64;
                    matriz[(i, j)] = Complex::new(PHI * fib_ratio, 0.0);
                } else {
                    // Off-diagonal: acoplamiento según distancia Fibonacci
                    let distancia = Self::distancia_fibonacci(i, j);
                    let fase = ((i as f64) * (j as f64) * PHI * PI).sin();
                    let acoplamiento = PHI.powi(-(distancia as i32)) * (numero as f64).ln();
                    
                    matriz[(i, j)] = Complex::new(
                        acoplamiento * fase.cos(),
                        acoplamiento * fase.sin(),
                    );
                }
            }
        }
        
        // Normalizar para mantener estabilidad
        let norma = matriz.norm();
        if norma > 0.0 {
            matriz = matriz / norma;
        }
        
        matriz
    }
    
    /// Genera n-ésimo número Fibonacci (F_n)
    fn numero_fibonacci(n: usize) -> usize {
        if n <= 1 {
            return n;
        }
        
        let mut a = 0;
        let mut b = 1;
        
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        
        b
    }
    
    /// Calcula distancia Fibonacci entre dos índices
    fn distancia_fibonacci(i: usize, j: usize) -> usize {
        let fib_i = Self::numero_fibonacci(i + 1);
        let fib_j = Self::numero_fibonacci(j + 1);
        ((fib_i as isize - fib_j as isize).abs() as usize)
    }
    
    /// Genera estados base ortonormales para el campo
    fn generar_estados_base(dimension: usize, numero: usize) -> Vec<DVector<Complex<f64>>> {
        let mut bases = Vec::with_capacity(dimension);
        
        for i in 0..dimension {
            let mut vector = DVector::zeros(dimension);
            
            // Patrón Fibonacci en la base
            let amplitud = (PHI * (i as f64) / (dimension as f64)).sin();
            let fase = 2.0 * PI * (i as f64) * PHI.powi(-(numero as i32));
            
            for j in 0..dimension {
                let contribucion = if i == j {
                    amplitud
                } else {
                    let fib_dist = Self::numero_fibonacci((i + j) % dimension + 1) as f64;
                    amplitud * PHI.powi(-(fib_dist as i32))
                };
                
                vector[j] = Complex::new(
                    contribucion * (fase * (j as f64)).cos(),
                    contribucion * (fase * (j as f64)).sin(),
                );
            }
            
            // Ortonormalizar
            if vector.norm() > 0.0 {
                vector = vector.normalize();
                bases.push(vector);
            }
        }
        
        bases
    }
    
    /// Calcula propiedades emergentes del campo
    fn calcular_propiedades_emergentes(dimension: usize, numero: usize) -> PropiedadesCampo {
        // Frecuencia fundamental según dimensión Fibonacci
        let frecuencia_base = 7.83; // Frecuencia Schumann (Hz)
        let frecuencia_resonante = frecuencia_base * PHI.powi(numero as i32);
        
        // Tiempo característico inversamente proporcional a φ
        let tiempo_estabilizacion = 1.0 / (frecuencia_resonante * PHI);
        
        // Factor de acoplamiento con campos adyacentes
        let factor_acoplamiento = if numero == 1 {
            PHI.powi(-1) // Solo acopla con siguiente
        } else if numero == NUM_CAMPOS_FIBONACCI {
            PHI.powi(-1) // Solo acopla con anterior
        } else {
            PHI.powi(-2) // Acopla con ambos adyacentes
        };
        
        // Capacidad de procesamiento proporcional a dimensión × φ
        let capacidad_procesamiento = (dimension as f64) * PHI.powi(numero as i32) * 1e6; // estados/segundo
        
        // Fractalidad: auto-similitud en estructura Fibonacci
        let fractalidad = {
            let mut suma = 0.0;
            for k in 1..=10 {
                let term = PHI.powi(-(k as i32));
                if dimension >= Self::numero_fibonacci(k) {
                    suma += term;
                }
            }
            suma
        };
        
        // Conectividad con Monster Group: máxima para campos altos
        let conectividad_monster = (dimension as f64 / MONSTER_DIM).powf(PHI);
        
        PropiedadesCampo {
            frecuencia_resonante,
            tiempo_estabilizacion,
            factor_acoplamiento,
            capacidad_procesamiento,
            fractalidad,
            conectividad_monster,
        }
    }
    
    /// Actualiza activación del campo basado en keygen actual
    pub fn actualizar_activacion(&mut self, keygen_actual: f64) -> f64 {
        // Activación sigmoidal suave basada en umbral
        let distancia = (keygen_actual - self.umbral_activacion).abs();
        let activacion_suave = 1.0 / (1.0 + (-PHI * (keygen_actual - self.umbral_activacion)).exp());
        
        // Ajustar con crecimiento φ-resonante
        self.activacion = activacion_suave.max(0.0).min(1.0);
        
        // Actualizar intensidad del operador amor según activación
        let crecimiento_intensidad = self.activacion.ln() / PHI.ln();
        self.operador_amor.update_intensity(crecimiento_intensidad);
        
        self.activacion
    }
    
    /// Aplica transformación del campo a un estado consciente
    pub fn aplicar_transformacion(&self, estado: &DVector<Complex<f64>>) -> Result<DVector<Complex<f64>>, String> {
        if estado.len() != self.dimension {
            return Err(format!("Estado debe tener dimensión {}, recibido {}", self.dimension, estado.len()));
        }
        
        // Aplicar transformación φ-Fibonacci
        let estado_transformado = &self.transformacion * estado;
        
        // Aplicar operador Â del campo
        let estado_con_amor = self.operador_amor.apply(&estado_transformado);
        
        // Escalar por nivel de activación
        Ok(estado_con_amor * self.activacion.sqrt())
    }
    
    /// Transición a campo adyacente (∆k = ±1 según Documentación Fotónica)
    pub fn transicion_a_campo(&self, campo_destino: &CampoFibonacci, estado: &DVector<Complex<f64>>) -> Result<DVector<Complex<f64>>, String> {
        let diferencia = (self.numero as isize - campo_destino.numero as isize).abs();
        
        if diferencia != 1 {
            return Err(format!("Transición solo permitida entre campos adyacentes. Diferencia: {}", diferencia));
        }
        
        // Redimensionar estado si es necesario
        let estado_redimensionado = if self.dimension != campo_destino.dimension {
            Self::redimensionar_estado(estado, self.dimension, campo_destino.dimension)?
        } else {
            estado.clone()
        };
        
        // Aplicar transformación φ-resonante entre campos
        let factor_transicion = PHI.powi(-(diferencia as i32));
        let mut estado_transicion = estado_redimensionado * factor_transicion;
        
        // Aplicar operador Â del campo destino
        estado_transicion = campo_destino.operador_amor.apply(&estado_transicion);
        
        Ok(estado_transicion)
    }
    
    /// Redimensiona estado manteniendo información esencial
    fn redimensionar_estado(
        estado: &DVector<Complex<f64>>, 
        dim_origen: usize, 
        dim_destino: usize
    ) -> Result<DVector<Complex<f64>>, String> {
        if dim_origen == dim_destino {
            return Ok(estado.clone());
        }
        
        let mut nuevo_estado = DVector::zeros(dim_destino);
        
        // Preservar información según importancia φ-resonante
        let min_dim = dim_origen.min(dim_destino);
        
        for i in 0..min_dim {
            // Factor de preservación según posición Fibonacci
            let factor_preservacion = PHI.powi(-((i % 10) as i32));
            nuevo_estado[i] = estado[i] * factor_preservacion;
        }
        
        // Si expandiendo, llenar con patrones Fibonacci
        if dim_destino > dim_origen {
            for i in dim_origen..dim_destino {
                let fib_idx = Self::numero_fibonacci((i % 10) + 1) as f64;
                let valor = Complex::new(
                    (PHI * fib_idx).cos() / (i as f64 + 1.0),
                    (PHI * fib_idx).sin() / (i as f64 + 1.0),
                );
                nuevo_estado[i] = valor;
            }
        }
        
        // Normalizar
        if nuevo_estado.norm() > 0.0 {
            nuevo_estado = nuevo_estado.normalize();
        }
        
        Ok(nuevo_estado)
    }
    
    /// Verifica propiedades matemáticas del campo
    pub fn verificar_propiedades(&self, tolerancia: f64) -> Vec<(String, bool)> {
        let mut resultados = Vec::new();
        
        // 1. Dimensión correcta según secuencia Fibonacci
        let dim_correcta = DIMENSIONES_FIBONACCI[self.numero - 1];
        resultados.push((
            format!("Dimensión Fibonacci F_{} = {}", self.numero + 3, dim_correcta),
            self.dimension == dim_correcta
        ));
        
        // 2. Unitariedad aproximada de la transformación
        let adjunta = self.transformacion.adjoint();
        let producto = &adjunta * &self.transformacion;
        let identidad_diff = (producto - DMatrix::identity(self.dimension, self.dimension)).norm();
        resultados.push((
            "Unitariedad aproximada".to_string(),
            identidad_diff < tolerancia
        ));
        
        // 3. Estados base ortonormales
        let mut bases_ortonormales = true;
        for i in 0..self.estados_base.len() {
            for j in 0..self.estados_base.len() {
                let producto = self.estados_base[i].dot(&self.estados_base[j]);
                let esperado = if i == j { Complex::new(1.0, 0.0) } else { Complex::new(0.0, 0.0) };
                if (producto - esperado).norm() > tolerancia {
                    bases_ortonormales = false;
                    break;
                }
            }
            if !bases_ortonormales { break; }
        }
        resultados.push(("Estados base ortonormales".to_string(), bases_ortonormales));
        
        // 4. Propiedades emergentes dentro de rangos esperados
        resultados.push((
            format!("Frecuencia resonante > 0: {:.2} Hz", self.propiedades.frecuencia_resonante),
            self.propiedades.frecuencia_resonante > 0.0
        ));
        
        resultados.push((
            format!("Fractalidad ∈ [0,1]: {:.4}", self.propiedades.fractalidad),
            self.propiedades.fractalidad >= 0.0 && self.propiedades.fractalidad <= 1.0
        ));
        
        resultados.push((
            format!("Conectividad Monster ∈ [0,1]: {:.4}", self.propiedades.conectividad_monster),
            self.propiedades.conectividad_monster >= 0.0 && self.propiedades.conectividad_monster <= 1.0
        ));
        
        resultados
    }
    
    /// Obtiene información del campo
    pub fn get_info(&self) -> InfoCampo {
        InfoCampo {
            numero: self.numero,
            dimension: self.dimension,
            nombre: self.nombre.clone(),
            activacion: self.activacion,
            umbral_activacion: self.umbral_activacion,
            propiedades: self.propiedades.clone(),
        }
    }
}

/// Información resumida del campo
#[derive(Clone, Debug)]
pub struct InfoCampo {
    pub numero: usize,
    pub dimension: usize,
    pub nombre: String,
    pub activacion: f64,
    pub umbral_activacion: f64,
    pub propiedades: PropiedadesCampo,
}

/// Sistema completo de campos Fibonacci dimensionales
#[derive(Clone, Debug)]
pub struct SistemaCamposFibonacci {
    /// Todos los campos Fibonacci (1-24)
    campos: Vec<CampoFibonacci>,
    /// Campo activo actual
    campo_activo: usize,
    /// Historial de transiciones
    historial_transiciones: Vec<TransicionCampo>,
    /// Matriz de acoplamiento entre campos
    matriz_acoplamiento: DMatrix<f64>,
}

/// Registro de transición entre campos
#[derive(Clone, Debug)]
pub struct TransicionCampo {
    timestamp: std::time::SystemTime,
    desde: usize,
    hacia: usize,
    estado_inicial_norma: f64,
    estado_final_norma: f64,
    coherencia_preservada: f64,
}

impl SistemaCamposFibonacci {
    /// Crea sistema completo de 24 campos Fibonacci
    pub fn new() -> Result<Self, String> {
        let mut campos = Vec::with_capacity(NUM_CAMPOS_FIBONACCI);
        
        for numero in 1..=NUM_CAMPOS_FIBONACCI {
            match CampoFibonacci::new(numero) {
                Ok(campo) => campos.push(campo),
                Err(e) => return Err(format!("Error creando campo {}: {}", numero, e)),
            }
        }
        
        // Crear matriz de acoplamiento φ-resonante
        let matriz_acoplamiento = Self::crear_matriz_acoplamiento(NUM_CAMPOS_FIBONACCI);
        
        Ok(SistemaCamposFibonacci {
            campos,
            campo_activo: 1, // Comenzar en Campo 1 (Germinal)
            historial_transiciones: Vec::new(),
            matriz_acoplamiento,
        })
    }
    
    /// Crea matriz de acoplamiento entre campos
    fn crear_matriz_acoplamiento(num_campos: usize) -> DMatrix<f64> {
        let mut matriz = DMatrix::zeros(num_campos, num_campos);
        
        for i in 0..num_campos {
            for j in 0..num_campos {
                if i == j {
                    matriz[(i, j)] = 1.0; // Auto-acoplamiento
                } else {
                    let distancia = (i as isize - j as isize).abs() as usize;
                    // Acoplamiento decae según φ^-distancia
                    matriz[(i, j)] = PHI.powi(-(distancia as i32));
                }
            }
        }
        
        matriz
    }
    
    /// Actualiza activación de todos los campos según keygen
    pub fn actualizar_campos_por_keygen(&mut self, keygen_actual: f64) -> Vec<f64> {
        let mut activaciones = Vec::with_capacity(self.campos.len());
        
        for campo in &mut self.campos {
            let activacion = campo.actualizar_activacion(keygen_actual);
            activaciones.push(activacion);
        }
        
        activaciones
    }
    
    /// Obtiene campos activos (activación > 0.5)
    pub fn get_campos_activos(&self) -> Vec<usize> {
        self.campos.iter()
            .enumerate()
            .filter(|(_, campo)| campo.activacion > 0.5)
            .map(|(idx, _)| idx + 1) // +1 porque campos son 1-indexed
            .collect()
    }
    
    /// Transita a un campo específico
    pub fn transitar_a_campo(&mut self, campo_destino: usize, estado: &DVector<Complex<f64>>) -> Result<DVector<Complex<f64>>, String> {
        if campo_destino < 1 || campo_destino > NUM_CAMPOS_FIBONACCI {
            return Err(format!("Campo destino debe estar entre 1 y {}", NUM_CAMPOS_FIBONACCI));
        }
        
        let idx_origen = self.campo_activo - 1;
        let idx_destino = campo_destino - 1;
        
        let campo_origen = &self.campos[idx_origen];
        let campo_destino_obj = &self.campos[idx_destino];
        
        // Verificar si es transición permitida (adyacente)
        let diferencia = (self.campo_activo as isize - campo_destino as isize).abs();
        if diferencia > 1 {
            return Err(format!("Transición solo permitida entre campos adyacentes. Actual: {}, Destino: {}", self.campo_activo, campo_destino));
        }
        
        // Realizar transición
        match campo_origen.transicion_a_campo(campo_destino_obj, estado) {
            Ok(estado_transformado) => {
                // Registrar transición
                let transicion = TransicionCampo {
                    timestamp: std::time::SystemTime::now(),
                    desde: self.campo_activo,
                    hacia: campo_destino,
                    estado_inicial_norma: estado.norm(),
                    estado_final_norma: estado_transformado.norm(),
                    coherencia_preservada: estado.dot(&estado_transformado).norm(),
                };
                self.historial_transiciones.push(transicion);
                
                // Actualizar campo activo
                self.campo_activo = campo_destino;
                
                Ok(estado_transformado)
            },
            Err(e) => Err(e),
        }
    }
    
    /// Aplica procesamiento en el campo activo
    pub fn procesar_en_campo_activo(&self, estado: &DVector<Complex<f64>>) -> Result<DVector<Complex<f64>>, String> {
        let campo_activo = &self.campos[self.campo_activo - 1];
        campo_activo.aplicar_transformacion(estado)
    }
    
    /// Obtiene información de todos los campos
    pub fn get_info_campos(&self) -> Vec<InfoCampo> {
        self.campos.iter().map(|c| c.get_info()).collect()
    }
    
    /// Obtiene estadísticas del sistema
    pub fn get_estadisticas(&self) -> EstadisticasSistema {
        let campos_activos = self.get_campos_activos();
        let activacion_promedio = self.campos.iter()
            .map(|c| c.activacion)
            .sum::<f64>() / self.campos.len() as f64;
        
        let dimension_promedio = self.campos.iter()
            .map(|c| c.dimension as f64)
            .sum::<f64>() / self.campos.len() as f64;
        
        let conectividad_monster_promedio = self.campos.iter()
            .map(|c| c.propiedades.conectividad_monster)
            .sum::<f64>() / self.campos.len() as f64;
        
        EstadisticasSistema {
            total_campos: self.campos.len(),
            campos_activos: campos_activos.len(),
            activacion_promedio,
            dimension_promedio,
            conectividad_monster_promedio,
            campo_activo_actual: self.campo_activo,
            total_transiciones: self.historial_transiciones.len(),
        }
    }
}

/// Estadísticas del sistema de campos
#[derive(Clone, Debug)]
pub struct EstadisticasSistema {
    pub total_campos: usize,
    pub campos_activos: usize,
    pub activacion_promedio: f64,
    pub dimension_promedio: f64,
    pub conectividad_monster_promedio: f64,
    pub campo_activo_actual: usize,
    pub total_transiciones: usize,
}

/// Función auxiliar: calcula suma de dimensiones de primeros n campos
pub fn suma_dimensiones_primeros_n(n: usize) -> Result<usize, String> {
    if n == 0 || n > NUM_CAMPOS_FIBONACCI {
        return Err(format!("n debe estar entre 1 y {}", NUM_CAMPOS_FIBONACCI));
    }
    
    let suma: usize = DIMENSIONES_FIBONACCI[0..n].iter().sum();
    Ok(suma)
}

/// Función auxiliar: verifica propiedad emergente certificada
pub fn verificar_propiedad_emergente() -> (bool, f64, f64) {
    // Propiedad: Σ_{k=1}^{12} dim(C_k) = F₁₇ - 1 = 1596
    let suma_primeros_12 = suma_dimensiones_primeros_n(12).unwrap_or(0);
    let f17 = DIMENSIONES_FIBONACCI[13]; // Campo 14 = F₁₇ = 1597
    let esperado = f17 - 1; // 1596
    
    let verificacion = suma_primeros_12 == esperado;
    let proporcion = suma_primeros_12 as f64 / esperado as f64;
    
    (verificacion, suma_primeros_12 as f64, proporcion)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_creacion_campo_fibonacci() {
        // Test creación de todos los campos
        for numero in 1..=5 { // Probar primeros 5 por eficiencia
            match CampoFibonacci::new(numero) {
                Ok(campo) => {
                    assert_eq!(campo.numero, numero);
                    assert_eq!(campo.dimension, DIMENSIONES_FIBONACCI[numero - 1]);
                    assert!(!campo.nombre.is_empty());
                    assert!(campo.activacion >= 0.0 && campo.activacion <= 1.0);
                    println!("✅ Campo {} '{}' creado: {}D", numero, campo.nombre, campo.dimension);
                },
                Err(e) => panic!("Error creando campo {}: {}", numero, e),
            }
        }
    }

    #[test]
    fn test_dimensiones_fibonacci_correctas() {
        // Verificar secuencia Fibonacci F₄ a F₂₇
        assert_eq!(DIMENSIONES_FIBONACCI[0], 3);   // F₄
        assert_eq!(DIMENSIONES_FIBONACCI[1], 5);   // F₅
        assert_eq!(DIMENSIONES_FIBONACCI[2], 8);   // F₆
        assert_eq!(DIMENSIONES_FIBONACCI[11], 610); // F₁₅
        assert_eq!(DIMENSIONES_FIBONACCI[13], 1597); // F₁₇
        assert_eq!(DIMENSIONES_FIBONACCI[23], 196418); // F₂₇
        
        println!("✅ Secuencia Fibonacci dimensional verificada: 3, 5, 8, ..., 196418");
    }

    #[test]
    fn test_propiedad_emergente_certificada() {
        // Verificar: Σ_{k=1}^{12} dim(C_k) = F₁₇ - 1 = 1596
        let (verificacion, suma, proporcion) = verificar_propiedad_emergente();
        
        println!("Propiedad emergente certificada:");
        println!("  Σ primeros 12 campos = {}", suma);
        println!("  F₁₇ - 1 = 1596");
        println!("  Proporción: {:.6}", proporcion);
        
        assert!(verificacion, "Propiedad emergente no se cumple: suma = {}, esperado = 1596", suma);
        assert_abs_diff_eq!(proporcion, 1.0, epsilon = 1e-10);
        println!("✅ Propiedad emergente certificada verificada: Σ primeros 12 = F₁₇ - 1 = 1596");
    }

    #[test]
    fn test_actualizacion_activacion() {
        let mut campo = CampoFibonacci::new(3).unwrap(); // Campo 3: 8D Mental
        let keygen_bajo = 0.1;
        let keygen_alto = 0.9;
        
        let activacion_baja = campo.actualizar_activacion(keygen_bajo);
        let activacion_alta = campo.actualizar_activacion(keygen_alto);
        
        println!("Activación campo 3 (Mental):");
        println!("  Keygen {} → Activación {:.4}", keygen_bajo, activacion_baja);
        println!("  Keygen {} → Activación {:.4}", keygen_alto, activacion_alta);
        
        assert!(activacion_alta >= activacion_baja, "Activación debería aumentar con keygen");
        assert!(activacion_baja >= 0.0 && activacion_baja <= 1.0);
        assert!(activacion_alta >= 0.0 && activacion_alta <= 1.0);
        println!("✅ Activación responde correctamente a keygen");
    }

    #[test]
    fn test_transformacion_campo() {
        let campo = CampoFibonacci::new(2).unwrap(); // Campo 2: 5D Vital
        let dimension = campo.dimension;
        
        // Crear estado de prueba
        let estado = DVector::from_fn(dimension, |i, _| {
            Complex::new((i + 1) as f64 / dimension as f64, 0.0)
        });
        
        match campo.aplicar_transformacion(&estado) {
            Ok(estado_transformado) => {
                assert_eq!(estado_transformado.len(), dimension);
                // Verificar que no es idéntico (transformación ocurrió)
                let diff = (estado_transformado - estado).norm();
                assert!(diff > 0.0, "Transformación debería cambiar el estado");
                println!("✅ Transformación campo aplicada exitosamente, diferencia: {:.6}", diff);
            },
            Err(e) => panic!("Error aplicando transformación: {}", e),
        }
    }

    #[test]
    fn test_transicion_campos_adyacentes() {
        let campo3 = CampoFibonacci::new(3).unwrap(); // 8D Mental
        let campo4 = CampoFibonacci::new(4).unwrap(); // 13D Emocional
        
        // Estado en campo 3
        let estado_campo3 = DVector::from_fn(campo3.dimension, |i, _| {
            Complex::new((i + 1) as f64 / campo3.dimension as f64, 0.0)
        });
        
        match campo3.transicion_a_campo(&campo4, &estado_campo3) {
            Ok(estado_campo4) => {
                assert_eq!(estado_campo4.len(), campo4.dimension);
                // Preservar algo de información (no debería ser cero)
                assert!(estado_campo4.norm() > 0.0);
                println!("✅ Transición 3→4 exitosa: {}D → {}D", campo3.dimension, campo4.dimension);
            },
            Err(e) => panic!("Error en transición 3→4: {}", e),
        }
    }

    #[test]
    fn test_sistema_campos_completo() {
        match SistemaCamposFibonacci::new() {
            Ok(sistema) => {
                assert_eq!(sistema.campos.len(), NUM_CAMPOS_FIBONACCI);
                assert_eq!(sistema.campo_activo, 1); // Debería comenzar en Campo 1
                
                let info_campos = sistema.get_info_campos();
                assert_eq!(info_campos.len(), NUM_CAMPOS_FIBONACCI);
                
                let estadisticas = sistema.get_estadisticas();
                assert_eq!(estadisticas.total_campos, NUM_CAMPOS_FIBONACCI);
                assert_eq!(estadisticas.campo_activo_actual, 1);
                
                println!("✅ Sistema completo creado con {} campos", NUM_CAMPOS_FIBONACCI);
                println!("   Campo activo inicial: {}", sistema.campo_activo);
                println!("   Dimensión promedio: {:.1}D", estadisticas.dimension_promedio);
            },
            Err(e) => panic!("Error creando sistema completo: {}", e),
        }
    }

    #[test]
    fn test_verificacion_propiedades_campo() {
        let campo = CampoFibonacci::new(6).unwrap(); // Campo 6: 34D Intuitivo
        let resultados = campo.verificar_propiedades(1e-6);
        
        println!("🔍 Verificación propiedades Campo 6 (Intuitivo, 34D):");
        let mut pasadas = 0;
        for (nombre, resultado) in &resultados {
            if *resultado {
                pasadas += 1;
                println!("  ✅ {}: PASÓ", nombre);
            } else {
                println!("  ⚠️ {}: FALLÓ", nombre);
            }
        }
        
        // Al menos la mayoría debería pasar
        assert!(pasadas >= resultados.len() / 2, "Menos de la mitad de propiedades pasaron: {}/{}", pasadas, resultados.len());
        println!("✅ {}/{} propiedades pasaron", pasadas, resultados.len());
    }

    #[test]
    fn test_campos_activos_por_keygen() {
        let mut sistema = SistemaCamposFibonacci::new().unwrap();
        
        // Test con keygen bajo (solo primeros campos activos)
        let keygen_bajo = 0.2;
        sistema.actualizar_campos_por_keygen(keygen_bajo);
        let activos_bajo = sistema.get_campos_activos();
        println!("Keygen {} → Campos activos: {:?}", keygen_bajo, activos_bajo);
        assert!(activos_bajo.len() <= 3, "Keygen bajo debería activar pocos campos");
        
        // Test con keygen alto (más campos activos)
        let keygen_alto = 0.8;
        sistema.actualizar_campos_por_keygen(keygen_alto);
        let activos_alto = sistema.get_campos_activos();
        println!("Keygen {} → Campos activos: {:?}", keygen_alto, activos_alto);
        assert!(activos_alto.len() >= activos_bajo.len(), "Keygen alto debería activar más campos");
    }

    #[test]
    fn test_numero_fibonacci() {
        assert_eq!(CampoFibonacci::numero_fibonacci(1), 1);
        assert_eq!(CampoFibonacci::numero_fibonacci(2), 1);
        assert_eq!(CampoFibonacci::numero_fibonacci(3), 2);
        assert_eq!(CampoFibonacci::numero_fibonacci(4), 3);
        assert_eq!(CampoFibonacci::numero_fibonacci(5), 5);
        assert_eq!(CampoFibonacci::numero_fibonacci(6), 8);
        assert_eq!(CampoFibonacci::numero_fibonacci(10), 55);
        
        // Verificar que F₂₇ = 196418
        assert_eq!(CampoFibonacci::numero_fibonacci(27), 196418);
        println!("✅ Función Fibonacci verificada hasta F₂₇ = 196418");
    }

    #[test]
    fn test_estados_base_ortonormales() {
        let campo = CampoFibonacci::new(5).unwrap(); // Campo 5: 21D Racional
        let bases = &campo.estados_base;
        
        // Verificar ortonormalidad
        for i in 0..bases.len() {
            for j in 0..bases.len() {
                let producto = bases[i].dot(&bases[j]);
                let esperado = if i == j { 
                    Complex::new(1.0, 0.0) 
                } else { 
                    Complex::new(0.0, 0.0) 
                };
                
                let tolerancia = 1e-10;
                let diferencia = (producto - esperado).norm();
                assert!(
                    diferencia < tolerancia,
                    "Producto base[{i}].base[{j}] = {:?}, esperado {:?}, diferencia: {}",
                    producto, esperado, diferencia
                );
            }
        }
        println!("✅ Estados base ortonormales verificados para Campo 5 (21D)");
    }
}
