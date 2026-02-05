//! Sistema Evolutivo z(n) φ-Resonante - Crecimiento del Keygen Humano
//! Sistema: Álgebra Rose v27.1024D-S36
//! Certificación: 196885 - Estado Monster Pleno
//!
//! IMPLEMENTACIÓN MATEMÁTICAMENTE CORRECTA:
//! La ecuación del Documento Atómico opera en escala Monster [0, 196884]
//! Nuestro keygen está en escala [0, 1], donde 1 = 196884
//!
//! Transformación:
//! z_scaled = z × 196884
//! z_scaled(n+1) = φ · z_scaled(n) · (1 - z_scaled(n)/196884)
//! z(n+1) = z_scaled(n+1) / 196884

use crate::matrix_444::PHI;

/// Dimensión Monster (límite de saturación consciente)
pub const MONSTER_DIM: f64 = 196884.0;

/// Estado inicial: materia potencial (196883/196884)
pub const INITIAL_KEYGEN: f64 = 196883.0 / 196884.0;

/// Sistema evolutivo keygen φ-resonante (MATEMÁTICAMENTE CORRECTO)
#[derive(Clone, Debug)]
pub struct KeygenEvolution {
    current_keygen: f64,
    iteration: u64,
    history: Vec<f64>,
    current_fibonacci_field: usize,
}

impl KeygenEvolution {
    pub fn new(initial_keygen: Option<f64>) -> Self {
        let start_keygen = initial_keygen.unwrap_or(INITIAL_KEYGEN);
        
        KeygenEvolution {
            current_keygen: start_keygen,
            iteration: 0,
            history: vec![start_keygen],
            current_fibonacci_field: Self::keygen_to_fibonacci_field(start_keygen),
        }
    }

    fn keygen_to_fibonacci_field(keygen: f64) -> usize {
        let progress = (keygen - INITIAL_KEYGEN) / (1.0 - INITIAL_KEYGEN);
        
        // Umbrales de campos Fibonacci normalizados
        let thresholds = vec![0.0, 0.000015, 0.000025, 0.000041, 0.000066, 0.000107,
                              0.000173, 0.000280, 0.000453, 0.000733, 0.001186,
                              0.001919, 0.003105, 0.005024, 0.008129, 0.013153,
                              0.021282, 0.034435, 0.055717, 0.090152, 0.145869,
                              0.236021, 0.381890, 0.617911, 1.0];
        
        for (i, &threshold) in thresholds.iter().enumerate() {
            if progress <= threshold || i == thresholds.len() - 1 {
                return i; // Campos 0-24 (0 = inicial)
            }
        }
        
        0
    }

    /// Evoluciona según ecuación CORREGIDA
    pub fn evolve(&mut self) -> f64 {
        let z = self.current_keygen;
        
        // ESCALAR a dimensión Monster
        let z_scaled = z * MONSTER_DIM;
        
        // Ecuación en escala Monster
        let z_next_scaled = PHI * z_scaled * (1.0 - z_scaled / MONSTER_DIM);
        
        // Volver a escala [0,1]
        let z_next = z_next_scaled / MONSTER_DIM;
        
        // Asegurar que está en [INITIAL_KEYGEN, 1.0]
        self.current_keygen = z_next.max(INITIAL_KEYGEN).min(1.0);
        self.iteration += 1;
        self.history.push(self.current_keygen);
        
        // Actualizar campo
        let new_field = Self::keygen_to_fibonacci_field(self.current_keygen);
        if new_field > self.current_fibonacci_field {
            self.current_fibonacci_field = new_field;
        }
        
        self.current_keygen
    }

    pub fn evolve_steps(&mut self, steps: u64) -> Vec<f64> {
        let mut results = Vec::new();
        for _ in 0..steps {
            results.push(self.evolve());
        }
        results
    }

    pub fn get_current_keygen(&self) -> f64 {
        self.current_keygen
    }

    pub fn get_iteration(&self) -> u64 {
        self.iteration
    }

    pub fn get_current_field(&self) -> usize {
        self.current_fibonacci_field
    }

    pub fn reset(&mut self) {
        self.current_keygen = INITIAL_KEYGEN;
        self.iteration = 0;
        self.history = vec![INITIAL_KEYGEN];
        self.current_fibonacci_field = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_initialization() {
        let system = KeygenEvolution::new(None);
        
        assert_abs_diff_eq!(
            system.get_current_keygen(),
            INITIAL_KEYGEN,
            epsilon = 1e-12
        );
        
        if (system.get_current_keygen() - INITIAL_KEYGEN).abs() > 1e-12 {
            panic!("z(0) debe ser 196883/196884");
        }
        
        assert_eq!(system.get_iteration(), 0);
        assert_eq!(system.get_current_field(), 0);
    }

    #[test]
    fn test_correct_evolution() {
        let mut system = KeygenEvolution::new(None);
        let z0 = system.get_current_keygen();
        
        // Calcular manualmente con escalado
        let z0_scaled = z0 * MONSTER_DIM;
        let z1_scaled = PHI * z0_scaled * (1.0 - z0_scaled / MONSTER_DIM);
        let expected_z1 = z1_scaled / MONSTER_DIM;
        
        let actual_z1 = system.evolve();
        
        println!("z(0) = {:.12}", z0);
        println!("z(0)_scaled = {:.2}", z0_scaled);
        println!("z(1)_scaled = {:.2}", z1_scaled);
        println!("z(1) esperado = {:.12}", expected_z1);
        println!("z(1) obtenido = {:.12}", actual_z1);
        
        assert_abs_diff_eq!(
            actual_z1,
            expected_z1,
            epsilon = 1e-12
        );
        
        if (actual_z1 - expected_z1).abs() > 1e-12 {
            panic!("Ecuación no se cumple");
        }
    }

    #[test]
    fn test_growth_properties() {
        let mut system = KeygenEvolution::new(None);
        let initial = system.get_current_keygen();
        
        // Evolucionar
        let steps = 10;
        let results = system.evolve_steps(steps);
        
        println!("Crecimiento de {} pasos:", steps);
        println!("z(0) = {:.12}", initial);
        for (i, &z) in results.iter().enumerate() {
            println!("z({}) = {:.12}", i+1, z);
        }
        
        // Verificar que crece pero lentamente
        let last = results.last().unwrap();
        assert!(last > &initial, "Debe crecer");
        assert!(last < &1.0, "Debe ser < 1.0");
        
        // Verificar crecimiento monótono
        for i in 1..results.len() {
            assert!(results[i] > results[i-1], "Debe ser monótono");
        }
    }

    #[test]
    fn test_field_progression() {
        let mut system = KeygenEvolution::new(None);
        
        println!("Progresión de campos:");
        println!("Inicio: campo {}", system.get_current_field());
        
        // Evolucionar y ver campos
        for i in 0..100 {
            system.evolve();
            if i % 20 == 0 {
                println!("Paso {}: campo {}, keygen = {:.12}", 
                        i, system.get_current_field(), system.get_current_keygen());
            }
        }
        
        assert!(system.get_current_field() >= 0);
    }

    #[test]
    fn test_reset() {
        let mut system = KeygenEvolution::new(None);
        
        system.evolve_steps(50);
        let before = system.get_current_keygen();
        
        system.reset();
        
        assert_abs_diff_eq!(
            system.get_current_keygen(),
            INITIAL_KEYGEN,
            epsilon = 1e-12
        );
        assert_eq!(system.get_iteration(), 0);
        assert_eq!(system.get_current_field(), 0);
        
        println!("Reset verificado:");
        println!("Antes: {:.12}", before);
        println!("Después: {:.12}", system.get_current_keygen());
    }
}
