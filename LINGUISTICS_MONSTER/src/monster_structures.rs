//! Estructuras fundamentales para lingüística Monster
//! PRINCIPIO: No asignar τ, extraer τ naturales de subestructuras

use num_complex::Complex64;
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
        // La lógica: parámetros más extremos (cerca de 0 o 1) crean términos de mayor grado
        let complejidad = self.complejidad_articulatoria();
        let grado = (complejidad * 6.0).ceil() as usize + 2; // Grado mínimo 2
        
        let mut coeficientes = vec![Complex64::new(0.0, 0.0); grado + 1];
        
        // Término constante: función de la sonoridad y nasalidad
        coeficientes[0] = Complex64::new(
            self.sonoridad * phi.ln() - self.nasalidad * std::f64::consts::PI / 10.0,
            0.0
        );
        
        // Término lineal: función del punto y modo
        coeficientes[1] = Complex64::new(
            self.punto_articulacion * phi - self.modo_articulacion,
            self.redondeamiento * std::f64::consts::PI / 5.0
        );
        
        // Términos superiores: emergen de combinaciones no lineales
        for i in 2..=grado {
            let combinacion = self.combinacion_no_lineal(i);
            coeficientes[i] = Complex64::new(
                combinacion * phi.powi(i as i32),
                combinacion * std::f64::consts::PI / (i as f64)
            );
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
        (base * E.ln() * (grado as f64).sin()).abs() / (grado as f64).sqrt()
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
    /// Usamos método de Newton simplificado
    pub fn raiz_principal(&self) -> f64 {
        if self.grado < 2 {
            return 0.0;
        }
        
        // Semilla inicial basada en coeficientes
        let semilla = self.coeficientes[1].norm().min(0.9).max(0.1);
        let mut x = semilla;
        
        // 10 iteraciones de Newton
        for _ in 0..10 {
            let (f, df) = self.evaluar_con_derivada(x);
            if df.abs() < 1e-12 {
                break;
            }
            x = (x - f.re / df.re).max(0.0).min(0.999999);
        }
        
        x % 1.0  // Asegurar τ ∈ [0, 1)
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
                if i >= 1 {
                    df = df + coef * (i as f64) * potencia / x.max(1e-12);
                }
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
    
    // τ combinado: promedio ponderado por complejidad
    let mut suma_ponderada = 0.0;
    let mut suma_pesos = 0.0;
    
    for sub in subestructuras {
        let peso = sub.complejidad.max(0.1);
        suma_ponderada += sub.tau * peso;
        suma_pesos += peso;
    }
    
    if suma_pesos == 0.0 {
        return Err(ErrorLinguistica::PalabraVacia);
    }
    
    let tau = suma_ponderada / suma_pesos;
    
    if !(0.0..1.0).contains(&tau) {
        Err(ErrorLinguistica::TauFueraDeRango(tau))
    } else {
        Ok(tau % 1.0)
    }
}
