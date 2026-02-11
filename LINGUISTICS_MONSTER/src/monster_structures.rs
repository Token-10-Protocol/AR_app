//! Estructuras fundamentales para lingüística Monster
//! PRINCIPIO: No asignar τ, extraer τ naturales de subestructuras

use num_complex::Complex64;
use num_complex::ComplexFloat; // Para .abs(), .norm(), etc.
use std::error::Error;
use std::fmt;

/// Parámetros articulatorios continuos en ℝ⁵
/// Cada parámetro ∈ [0.0, 1.0]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParametrosArticulatorios {
    /// [0.0, 1.0] labial → glotal
    pub punto_articulacion: f64,
    /// [0.0, 1.0] oclusiva → vocal
    pub modo_articulacion: f64,
    /// [0.0, 1.0] sorda → sonora
    pub sonoridad: f64,
    /// [0.0, 1.0] oral → nasal
    pub nasalidad: f64,
    /// [0.0, 1.0] no redondeado → redondeado
    pub redondeamiento: f64,
}

impl ParametrosArticulatorios {
    /// Validar que todos los parámetros estén en [0, 1]
    pub fn validar(&self) -> Result<(), String> {
        let params = [
            ("punto_articulacion", self.punto_articulacion),
            ("modo_articulacion", self.modo_articulacion),
            ("sonoridad", self.sonoridad),
            ("nasalidad", self.nasalidad),
            ("redondeamiento", self.redondeamiento),
        ];

        for (nombre, valor) in params {
            if !(0.0..=1.0).contains(&valor) {
                return Err(format!("{} = {} fuera de [0, 1]", nombre, valor));
            }
        }
        Ok(())
    }

    /// Fonema /a/ - vocal abierta central
    pub fn fonema_a() -> Self {
        Self {
            punto_articulacion: 0.5,    // central
            modo_articulacion: 1.0,     // vocal pura
            sonoridad: 1.0,             // sonora
            nasalidad: 0.0,             // oral
            redondeamiento: 0.0,        // no redondeado
        }
    }

    /// Fonema /m/ - nasal bilabial
    pub fn fonema_m() -> Self {
        Self {
            punto_articulacion: 0.0,    // bilabial (labial extremo)
            modo_articulacion: 0.3,     // nasal (entre oclusiva y aproximante)
            sonoridad: 1.0,             // sonora
            nasalidad: 1.0,             // nasal completo
            redondeamiento: 0.0,        // no redondeado (labios juntos)
        }
    }

    /// Fonema /o/ - vocal media posterior redondeada
    pub fn fonema_o() -> Self {
        Self {
            punto_articulacion: 0.8,    // posterior
            modo_articulacion: 0.9,     // vocal (casi pura)
            sonoridad: 1.0,             // sonora
            nasalidad: 0.0,             // oral
            redondeamiento: 1.0,        // redondeado completo
        }
    }

    /// Fonema /r/ - vibrante alveolar (ɾ en español)
    pub fn fonema_r() -> Self {
        Self {
            punto_articulacion: 0.3,    // alveolar
            modo_articulacion: 0.6,     // vibrante (modo complejo)
            sonoridad: 1.0,             // sonora
            nasalidad: 0.0,             // oral
            redondeamiento: 0.2,        // ligeramente redondeado
        }
    }

    /// Convertir a ecuación característica φ-polinomial
    /// Paso 1: Parámetros → coeficientes basados en φ
    pub fn a_ecuacion_caracteristica(&self) -> EcuacionPolinomica {
        // φ = (1 + √5)/2 ≈ 1.618033988749895
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        
        // Cada parámetro contribuye a coeficientes de diferentes grados
        let complejidad = self.complejidad_articulatoria();
        let grado = (complejidad * 8.0).ceil() as usize + 3; // Grado mínimo 3
        
        let mut coeficientes = vec![Complex64::new(0.0, 0.0); grado + 1];
        
        // Término constante: función de todos los parámetros
        coeficientes[0] = Complex64::new(
            (self.sonoridad * phi - self.nasalidad * std::f64::consts::PI / 10.0).tanh(),
            self.redondeamiento * 0.1
        );
        
        // Término lineal: función del punto y modo
        coeficientes[1] = Complex64::new(
            self.punto_articulacion * phi - self.modo_articulacion * 0.5,
            self.redondeamiento * std::f64::consts::PI / 8.0
        );
        
        // Término cuadrático: interacción entre parámetros
        coeficientes[2] = Complex64::new(
            (self.punto_articulacion * self.modo_articulacion * phi).sin(),
            (self.sonoridad * self.nasalidad * 0.3).cos()
        );
        
        // Términos superiores: emergen de combinaciones no lineales
        for i in 3..=grado {
            let combinacion = self.combinacion_no_lineal(i);
            let fase = (i as f64) * std::f64::consts::PI / (grado as f64);
            coeficientes[i] = Complex64::new(
                combinacion * phi.powi((i / 2) as i32) * fase.cos(),
                combinacion * phi.powi(((i+1)/2) as i32) * fase.sin()
            );
        }
        
        // Asegurar que el coeficiente de mayor grado no sea cero
        if coeficientes[grado].norm() < 1e-12 {
            coeficientes[grado] = Complex64::new(phi.recip(), phi.recip() * 0.1);
        }
        
        EcuacionPolinomica { coeficientes, grado }
    }
    
    /// Medida de complejidad articulatoria ∈ [0, 1]
    fn complejidad_articulatoria(&self) -> f64 {
        // Distancia del punto medio (0.5) indica complejidad
        let distancia_punto = (self.punto_articulacion - 0.5).abs();
        let distancia_modo = (self.modo_articulacion - 0.5).abs();
        let extremalidad = (distancia_punto + distancia_modo) / 2.0;
        
        // Nasalidad y redondeamiento añaden complejidad
        let rasgos_especiales = (self.nasalidad + self.redondeamiento) / 2.0;
        
        (extremalidad * 0.6 + rasgos_especiales * 0.4).min(1.0)
    }
    
    /// Combinación no lineal para términos de grado i
    fn combinacion_no_lineal(&self, grado: usize) -> f64 {
        use std::f64::consts::E;
        
        let base = match grado % 5 {
            0 => self.punto_articulacion,
            1 => self.modo_articulacion,
            2 => self.sonoridad,
            3 => self.nasalidad,
            4 => self.redondeamiento,
            _ => unreachable!(),
        };
        
        // Función que varía suavemente con el grado
        let oscilacion = (grado as f64 * std::f64::consts::PI / 7.0).sin();
        (base * E.ln() * oscilacion).abs() / (grado as f64).ln_1p()
    }
}

/// Ecuación polinómica con coeficientes complejos
#[derive(Debug, Clone, PartialEq)]
pub struct EcuacionPolinomica {
    pub coeficientes: Vec<Complex64>,
    pub grado: usize,
}

impl EcuacionPolinomica {
    /// Calcular la raíz principal (τ ∈ [0, 1))
    /// Usamos búsqueda con múltiples semillas incluyendo φ⁻¹
    pub fn raiz_principal(&self) -> f64 {
        if self.grado < 2 {
            return 0.0;
        }
        
        // SEMILLAS ESTRATÉGICAS (incluyendo φ⁻¹)
        let semillas = [
            0.3819660112501051,  // φ⁻²
            0.6180339887498948,  // φ⁻¹ (OBJETIVO PRINCIPAL)
            0.5,                 // Punto medio
            0.25, 0.75,          // Cuartiles
            0.1, 0.9,            // Extremos suaves
        ];
        
        let mut mejor_raiz = 0.5;
        let mut mejor_valor = f64::INFINITY;
        
        // Probar cada semilla
        for &semilla in &semillas {
            if let Some(raiz) = self.buscar_raiz_desde(semilla) {
                let valor = self.evaluar_modulo(raiz);
                if valor < mejor_valor {
                    mejor_valor = valor;
                    mejor_raiz = raiz;
                }
            }
        }
        
        // Asegurar que esté en [0, 1)
        (mejor_raiz % 1.0).abs()
    }
    
    /// Buscar raíz usando Newton desde una semilla
    fn buscar_raiz_desde(&self, semilla: f64) -> Option<f64> {
        let mut x = semilla;
        let mut intentos = 0;
        
        while intentos < 50 {
            let (f, df) = self.evaluar_con_derivada(x);
            
            // Si estamos cerca de una raíz
            if f.norm() < 1e-12 {
                return Some(x);
            }
            
            // Si la derivada es muy pequeña, cambiar de dirección
            if df.norm() < 1e-12 {
                x = (x + 0.123456789) % 1.0;  // Salto pseudo-aleatorio
                intentos += 1;
                continue;
            }
            
            // Paso de Newton
            let delta = f / df;
            let nuevo_x = x - delta.re;  // Usar solo parte real
            
            // Verificar convergencia
            if (nuevo_x - x).abs() < 1e-12 {
                return Some(nuevo_x);
            }
            
            // Mantener en [0, 1)
            x = nuevo_x.max(0.0).min(0.999999);
            
            // Si empezamos a oscilar, salir
            if intentos > 10 && (nuevo_x - semilla).abs() > 0.5 {
                break;
            }
            
            intentos += 1;
        }
        
        // Verificar si encontramos una raíz aceptable
        if self.evaluar_modulo(x) < 0.01 {
            Some(x)
        } else {
            None
        }
    }
    
    /// Evaluar |p(x)| (módulo del polinomio)
    fn evaluar_modulo(&self, x: f64) -> f64 {
        let mut resultado = Complex64::new(0.0, 0.0);
        let mut potencia = Complex64::new(1.0, 0.0);
        
        for coef in &self.coeficientes {
            resultado = resultado + coef * potencia;
            potencia = potencia * Complex64::new(x, 0.0);
        }
        
        resultado.norm()
    }
    
    /// Evaluar polinomio y su derivada en x real
    fn evaluar_con_derivada(&self, x: f64) -> (Complex64, Complex64) {
        let mut f = Complex64::new(0.0, 0.0);
        let mut df = Complex64::new(0.0, 0.0);
        let mut potencia = Complex64::new(1.0, 0.0);
        
        for (i, coef) in self.coeficientes.iter().enumerate() {
            if i == 0 {
                f = f + coef;
            } else {
                f = f + coef * potencia;
                df = df + coef * (i as f64) * potencia / x.max(1e-12);
                potencia = potencia * Complex64::new(x, 0.0);
            }
        }
        
        (f, df)
    }
}

/// Subestructura Monster (placeholder - se expandirá en FASE β)
#[derive(Debug, Clone, PartialEq)]
pub struct SubestructuraMonster {
    pub tau: f64,
    pub complejidad: f64,
    pub clase_conjugacion: usize,
}

impl ParametrosArticulatorios {
    /// Paso 2: τ → Subestructura Monster (versión simplificada para FASE α)
    pub fn a_subestructura_monster(&self) -> SubestructuraMonster {
        let ecuacion = self.a_ecuacion_caracteristica();
        let tau = ecuacion.raiz_principal();
        
        // Clase de conjugación basada en parámetros discretizados
        let clase = discretizar_parametros(self);
        
        SubestructuraMonster {
            tau,
            complejidad: self.complejidad_articulatoria(),
            clase_conjugacion: clase,
        }
    }
}

/// Discretizar parámetros para clase de conjugación
fn discretizar_parametros(p: &ParametrosArticulatorios) -> usize {
    let bits = [
        (p.punto_articulacion > 0.5) as usize,
        (p.modo_articulacion > 0.5) as usize,
        (p.sonoridad > 0.5) as usize,
        (p.nasalidad > 0.5) as usize,
        (p.redondeamiento > 0.5) as usize,
    ];
    
    bits.iter().enumerate().map(|(i, &b)| b << i).sum()
}

/// Error de lingüística Monster
#[derive(Debug, Clone)]
pub enum ErrorLinguistica {
    ParametrosInvalidos(String),
    TauFueraDeRango(f64),
    PalabraVacia,
}

impl fmt::Display for ErrorLinguistica {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParametrosInvalidos(msg) => write!(f, "Parámetros inválidos: {}", msg),
            Self::TauFueraDeRango(tau) => write!(f, "τ = {} fuera de [0, 1)", tau),
            Self::PalabraVacia => write!(f, "Palabra vacía"),
        }
    }
}

impl Error for ErrorLinguistica {}

/// Analizar una palabra (español básico)
pub fn analizar_palabra(palabra: &str) -> Result<Vec<SubestructuraMonster>, ErrorLinguistica> {
    if palabra.is_empty() {
        return Err(ErrorLinguistica::PalabraVacia);
    }
    
    let mut resultados = Vec::new();
    
    for c in palabra.chars() {
        let parametros = match c.to_lowercase().next() {
            Some('a') => ParametrosArticulatorios::fonema_a(),
            Some('m') => ParametrosArticulatorios::fonema_m(),
            Some('o') => ParametrosArticulatorios::fonema_o(),
            Some('r') => ParametrosArticulatorios::fonema_r(),
            Some(_) => continue, // Ignorar caracteres no implementados
            None => continue,
        };
        
        resultados.push(parametros.a_subestructura_monster());
    }
    
    if resultados.is_empty() {
        Err(ErrorLinguistica::PalabraVacia)
    } else {
        Ok(resultados)
    }
}

/// Extraer τ natural de una palabra
pub fn tau_natural(palabra: &str) -> Result<f64, ErrorLinguistica> {
    let subestructuras = analizar_palabra(palabra)?;
    
    // τ combinado: promedio geométrico (más sensible a valores extremos)
    let mut producto = 1.0;
    let mut n = 0;
    
    for sub in subestructuras {
        if sub.tau > 1e-12 {  // Evitar ceros
            producto *= sub.tau;
            n += 1;
        }
    }
    
    if n == 0 {
        return Err(ErrorLinguistica::PalabraVacia);
    }
    
    let tau = producto.powf(1.0 / (n as f64));
    
    if !(0.0..1.0).contains(&tau) {
        Err(ErrorLinguistica::TauFueraDeRango(tau))
    } else {
        Ok(tau % 1.0)
    }
}
