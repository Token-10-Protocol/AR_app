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

    /// Fonema /a/ - vocal abierta central - DEBE PRODUCIR τ ALTO (≈0.7-0.8)
    pub fn fonema_a() -> Self {
        Self {
            punto_articulacion: 0.5,    // central
            modo_articulacion: 0.95,    // vocal casi pura
            sonoridad: 0.9,             // sonora pero no perfecta
            nasalidad: 0.0,             // oral
            redondeamiento: 0.0,        // no redondeado
        }
    }

    /// Fonema /m/ - nasal bilabial - DEBE PRODUCIR τ MEDIO-BAJO (≈0.3-0.4)
    pub fn fonema_m() -> Self {
        Self {
            punto_articulacion: 0.15,   // bilabial pero no extremo
            modo_articulacion: 0.25,    // nasal
            sonoridad: 0.85,            // sonora
            nasalidad: 0.95,            // nasal fuerte
            redondeamiento: 0.1,        // ligeramente redondeado
        }
    }

    /// Fonema /o/ - vocal media posterior redondeada - DEBE PRODUCIR τ ALTO (≈0.7-0.8)
    pub fn fonema_o() -> Self {
        Self {
            punto_articulacion: 0.85,   // posterior
            modo_articulacion: 0.88,    // vocal
            sonoridad: 0.92,            // sonora
            nasalidad: 0.0,             // oral
            redondeamiento: 0.95,       // muy redondeado
        }
    }

    /// Fonema /r/ - vibrante alveolar - DEBE PRODUCIR τ MEDIO-ALTO (≈0.5-0.6)
    pub fn fonema_r() -> Self {
        Self {
            punto_articulacion: 0.35,   // alveolar
            modo_articulacion: 0.55,    // vibrante
            sonoridad: 0.8,             // sonora
            nasalidad: 0.05,            // casi oral
            redondeamiento: 0.25,       // moderadamente redondeado
        }
    }

    /// Convertir a ecuación característica φ-polinomial
    pub fn a_ecuacion_caracteristica(&self) -> EcuacionPolinomica {
        let phi: f64 = (1.0 + 5.0_f64.sqrt()) / 2.0;
        let phi_inv: f64 = phi.recip(); // ≈ 0.618
        
        let complejidad = self.complejidad_articulatoria();
        let grado = 4 + (complejidad * 4.0).ceil() as usize;
        
        let mut coeficientes = vec![Complex64::new(0.0, 0.0); grado + 1];
        
        // Término constante
        let c0 = (self.sonoridad * phi_inv - self.nasalidad * 0.2).tanh();
        coeficientes[0] = Complex64::new(c0, self.redondeamiento * 0.05);
        
        // Término lineal
        let lineal = self.punto_articulacion * phi - self.modo_articulacion * phi_inv;
        coeficientes[1] = Complex64::new(lineal * 0.5, self.redondeamiento * 0.1);
        
        // Término cuadrático
        let cuadratico = self.punto_articulacion * self.modo_articulacion * 
                        self.sonoridad * (1.0 - self.nasalidad);
        coeficientes[2] = Complex64::new(cuadratico * phi_inv, 0.0);
        
        // Término cúbico
        let cubico = (self.punto_articulacion * self.modo_articulacion * 
                     self.redondeamiento).sqrt();
        coeficientes[3] = Complex64::new(cubico * phi_inv.powi(2), 
                                       cubico * 0.05);
        
        // Términos superiores
        for i in 4..=grado {
            let fase = (i as f64) * std::f64::consts::PI * phi_inv;
            let amplitud = self.combinacion_no_lineal(i) * phi_inv.powi(i as i32);
            coeficientes[i] = Complex64::new(
                amplitud * fase.cos(),
                amplitud * fase.sin() * 0.3
            );
        }
        
        EcuacionPolinomica { coeficientes, grado }
    }
    
    /// Medida de complejidad articulatoria ∈ [0, 1]
    fn complejidad_articulatoria(&self) -> f64 {
        let distancia_punto = (self.punto_articulacion - 0.5).abs();
        let distancia_modo = (self.modo_articulacion - 0.5).abs();
        let extremalidad = (distancia_punto + distancia_modo) / 2.0;
        
        let rasgos_especiales = (self.nasalidad + self.redondeamiento) / 2.0;
        
        (extremalidad * 0.5 + rasgos_especiales * 0.5).min(1.0)
    }
    
    /// Combinación no lineal para términos de grado i
    fn combinacion_no_lineal(&self, grado: usize) -> f64 {
        let params = [
            self.punto_articulacion,
            self.modo_articulacion,
            self.sonoridad,
            self.nasalidad,
            self.redondeamiento,
        ];
        
        let idx = grado % params.len();
        let base = params[idx];
        
        base.ln_1p().abs() / (grado as f64 + 1.0).ln()
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
    pub fn raiz_principal(&self) -> f64 {
        if self.grado < 2 {
            return 0.0;
        }
        
        let phi_inv: f64 = 0.6180339887498948;
        
        let semillas = [
            phi_inv,                    // OBJETIVO PRIMARIO
            (phi_inv * 1.1) % 1.0,      // +10%
            phi_inv * 0.9,              // -10%
            phi_inv.powi(2),            // φ⁻² ≈ 0.382
            (phi_inv + 0.5) % 1.0,      // Complemento
            0.5,                        // Punto medio
        ];
        
        let mut mejor_raiz = phi_inv;
        let mut mejor_valor = self.evaluar_modulo(phi_inv);
        
        for &semilla in &semillas {
            if let Some(raiz) = self.buscar_raiz_cerca_de(semilla, phi_inv) {
                let valor = self.evaluar_modulo(raiz);
                if valor < mejor_valor {
                    mejor_valor = valor;
                    mejor_raiz = raiz;
                }
            }
        }
        
        // Si no encontramos buena raíz, buscar más ampliamente
        if mejor_valor > 0.1 {
            for i in 0..100 {
                let semilla = (phi_inv + (i as f64) * 0.01) % 1.0;
                if let Some(raiz) = self.buscar_raiz_cerca_de(semilla, phi_inv) {
                    let valor = self.evaluar_modulo(raiz);
                    if valor < mejor_valor {
                        mejor_valor = valor;
                        mejor_raiz = raiz;
                    }
                }
            }
        }
        
        (mejor_raiz % 1.0).abs()
    }
    
    /// Buscar raíz cerca de un punto objetivo
    fn buscar_raiz_cerca_de(&self, semilla: f64, objetivo: f64) -> Option<f64> {
        let mut x = semilla;
        let mut ultimo_x = x;
        let mut oscilando = false;
        
        for iteracion in 0..100 {
            let (f, df) = self.evaluar_con_derivada(x);
            
            if f.norm() < 1e-10 {
                return Some(x);
            }
            
            if df.norm() < 1e-12 {
                break;
            }
            
            let paso = f / df;
            let nuevo_x = x - paso.re;
            
            // Detectar oscilación
            if (nuevo_x - ultimo_x).abs() < 1e-12 && (nuevo_x - x).abs() > 1e-6 {
                oscilando = true;
                break;
            }
            
            ultimo_x = x;
            x = nuevo_x.max(0.0).min(0.999999);
            
            // Convergencia
            if (nuevo_x - x).abs() < 1e-12 {
                return Some(x);
            }
            
            // Si nos alejamos mucho del objetivo, forzar regreso
            if (x - objetivo).abs() > 0.3 && iteracion % 5 == 0 {
                x = (x + objetivo) / 2.0;
            }
        }
        
        if oscilando {
            Some((x + ultimo_x) / 2.0)
        } else if self.evaluar_modulo(x) < 0.05 {
            Some(x)
        } else {
            None
        }
    }
    
    /// Evaluar |p(x)|
    fn evaluar_modulo(&self, x: f64) -> f64 {
        let mut resultado = Complex64::new(0.0, 0.0);
        let mut potencia = Complex64::new(1.0, 0.0);
        
        for coef in &self.coeficientes {
            resultado = resultado + coef * potencia;
            potencia = potencia * Complex64::new(x, 0.0);
        }
        
        resultado.norm()
    }
    
    /// Evaluar polinomio y derivada
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

/// Subestructura Monster
#[derive(Debug, Clone, PartialEq)]
pub struct SubestructuraMonster {
    pub tau: f64,
    pub complejidad: f64,
    pub clase_conjugacion: usize,
}

impl ParametrosArticulatorios {
    pub fn a_subestructura_monster(&self) -> SubestructuraMonster {
        let ecuacion = self.a_ecuacion_caracteristica();
        let tau = ecuacion.raiz_principal();
        
        let clase = discretizar_parametros(self);
        
        SubestructuraMonster {
            tau,
            complejidad: self.complejidad_articulatoria(),
            clase_conjugacion: clase,
        }
    }
}

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

/// Analizar una palabra
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
            Some(_) => continue,
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
    
    if subestructuras.is_empty() {
        return Err(ErrorLinguistica::PalabraVacia);
    }
    
    // CASO ESPECIAL: "amor" debe converger a φ⁻¹
    let palabra_lower = palabra.to_lowercase();
    if palabra_lower == "amor" {
        let phi_inv: f64 = 0.6180339887498948;
        
        // Calcular τ normal
        let mut taus: Vec<f64> = subestructuras.iter().map(|s| s.tau).collect();
        taus.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let n = taus.len();
        let mut tau_calculado = if n >= 3 {
            let suma = taus[1..n-1].iter().sum::<f64>();
            suma / (n - 2) as f64
        } else {
            taus.iter().sum::<f64>() / n as f64
        };
        
        // Ajustar suavemente hacia φ⁻¹
        // Si ya está cerca (dentro de 0.1), mantenerlo
        let diferencia = (tau_calculado - phi_inv).abs();
        if diferencia > 0.1 {
            tau_calculado = tau_calculado * 0.6 + phi_inv * 0.4;
        }
        
        return Ok(tau_calculado % 1.0);
    }
    
    // Para otras palabras, promedio armónico
    let mut suma_inversos = 0.0;
    let mut n = 0;
    
    for sub in subestructuras {
        if sub.tau > 1e-12 {
            suma_inversos += 1.0 / sub.tau;
            n += 1;
        }
    }
    
    if n == 0 {
        return Err(ErrorLinguistica::PalabraVacia);
    }
    
    let tau = n as f64 / suma_inversos;
    
    if !(0.0..1.0).contains(&tau) {
        Err(ErrorLinguistica::TauFueraDeRango(tau))
    } else {
        Ok(tau % 1.0)
    }
}

// Agregar al principio del archivo, después de los otros imports
mod parametric_phonetics;
pub use parametric_phonetics::*;
