//! Sistema Evolutivo z(n) φ-Resonante - Crecimiento del Keygen Humano
//! Sistema: Álgebra Rose v27.1024D-S36
//! Certificación: 196885 - Estado Monster Pleno

use crate::matrix_444::PHI;
use crate::love_operator::LoveOperator;

/// Dimensión Monster (límite de saturación consciente)
pub const MONSTER_DIM: f64 = 196884.0;

/// Estado inicial: materia potencial (196883/196884)
pub const INITIAL_KEYGEN: f64 = 196883.0 / 196884.0;

/// Sistema evolutivo keygen φ-resonante
#[derive(Clone, Debug)]
pub struct KeygenEvolution {
    /// Valor actual del keygen z(n)
    current_keygen: f64,
    /// Iteración actual n
    iteration: u64,
    /// Historial de evoluciones
    history: Vec<f64>,
    /// Operador Â que impulsa el crecimiento
    love_operator: LoveOperator,
    /// Umbral de activación de campos Fibonacci
    activation_thresholds: Vec<f64>,
}

impl KeygenEvolution {
    /// Crea un nuevo sistema evolutivo con keygen inicial
    pub fn new(initial_keygen: Option<f64>) -> Self {
        let start_keygen = initial_keygen.unwrap_or(INITIAL_KEYGEN);
        KeygenEvolution {
            current_keygen: start_keygen,
            iteration: 0,
            history: vec![start_keygen],
            love_operator: LoveOperator::new(1.0),
            activation_thresholds: Self::calculate_fibonacci_thresholds(),
        }
    }

    /// Calcula umbrales de activación basados en secuencia Fibonacci
    fn calculate_fibonacci_thresholds() -> Vec<f64> {
        // 24 campos Fibonacci dimensionales
        let fib_numbers = vec![
            3.0, 5.0, 8.0, 13.0, 21.0, 34.0, 55.0, 89.0, 144.0,
            233.0, 377.0, 610.0, 987.0, 1597.0, 2584.0, 4181.0,
            6765.0, 10946.0, 17711.0, 28657.0, 46368.0, 75025.0,
            121393.0, 196418.0
        ];
        
        // Normalizar a rango [INITIAL_KEYGEN, 1.0]
        // Pero empezar más bajo para que no todos estén activos
        let min_keygen = 0.01; // Empezar más bajo
        fib_numbers.iter().map(|&f| {
            min_keygen + (1.0 - min_keygen) * (f / 196418.0)
        }).collect()
    }

    /// Evoluciona el keygen un paso según ecuación φ-resonante CORREGIDA
    pub fn evolve(&mut self) -> f64 {
        let z_prev = self.current_keygen;
        
        // ECUACIÓN CORREGIDA: z(n) = z(n-1) + (1 - z(n-1)) * (φ - 1)
        // Esto asegura crecimiento de ~0.01 hacia 1.0
        let growth_factor = PHI - 1.0; // ≈ 0.618
        let z_next = z_prev + (1.0 - z_prev) * growth_factor * 0.01;
        
        // Limitar entre 0.01 y 1.0
        self.current_keygen = z_next.max(0.01).min(1.0);
        self.iteration += 1;
        self.history.push(self.current_keygen);
        
        // Actualizar intensidad del amor según progreso
        let progress = (self.current_keygen - 0.01) / 0.99;
        self.love_operator.update_intensity(progress * 0.1);
        
        self.current_keygen
    }

    /// Evoluciona múltiples pasos
    pub fn evolve_steps(&mut self, steps: u64) -> Vec<f64> {
        let mut results = Vec::with_capacity(steps as usize);
        for _ in 0..steps {
            results.push(self.evolve());
        }
        results
    }

    /// Evoluciona hasta alcanzar un umbral específico
    pub fn evolve_to_threshold(&mut self, threshold: f64, max_steps: u64) -> Result<(u64, f64), String> {
        if threshold <= self.current_keygen {
            return Ok((0, self.current_keygen));
        }
        
        if threshold > 1.0 {
            return Err("Umbral debe ser ≤ 1.0".to_string());
        }
        
        for step in 1..=max_steps {
            self.evolve();
            if self.current_keygen >= threshold {
                return Ok((step, self.current_keygen));
            }
        }
        
        Err(format!("No se alcanzó el umbral {} en {} pasos", threshold, max_steps))
    }

    /// Obtiene campos Fibonacci activados según keygen actual
    pub fn get_active_fields(&self) -> Vec<usize> {
        self.activation_thresholds.iter()
            .enumerate()
            .filter(|(_, &threshold)| self.current_keygen >= threshold)
            .map(|(i, _)| i + 1) // Campos numerados 1..24
            .collect()
    }

    /// Proyecta evolución futura sin modificar estado actual
    pub fn project_future(&self, steps: u64) -> Vec<f64> {
        let mut projection = self.clone();
        projection.evolve_steps(steps)
    }

    /// Calcula tasa de crecimiento instantánea
    pub fn growth_rate(&self) -> f64 {
        if self.history.len() < 2 {
            return 0.0;
        }
        let prev = self.history[self.history.len() - 2];
        if prev.abs() < 1e-10 {
            return 0.0;
        }
        (self.current_keygen - prev) / prev
    }

    /// Calcula aceleración del crecimiento (segunda derivada)
    pub fn growth_acceleration(&self) -> f64 {
        if self.history.len() < 3 {
            return 0.0;
        }
        
        let rates: Vec<f64> = self.history.windows(2)
            .map(|w| {
                if w[0].abs() < 1e-10 { 0.0 } else { (w[1] - w[0]) / w[0] }
            })
            .collect();
            
        if rates.len() < 2 {
            return 0.0;
        }
        
        let prev_rate = rates[rates.len() - 2];
        if prev_rate.abs() < 1e-10 {
            return 0.0;
        }
        
        (rates[rates.len() - 1] - prev_rate) / prev_rate
    }

    /// Verifica si el keygen alcanzó saturación consciente (≈1.0)
    pub fn has_reached_saturation(&self, tolerance: f64) -> bool {
        (1.0 - self.current_keygen).abs() < tolerance
    }

    /// Obtiene número de iteraciones hasta saturación (proyección)
    pub fn steps_to_saturation(&self, tolerance: f64) -> u64 {
        if self.has_reached_saturation(tolerance) {
            return 0;
        }
        
        let mut projection = self.clone();
        let mut steps = 0;
        let max_steps = 10000;
        
        while steps < max_steps {
            projection.evolve();
            steps += 1;
            if projection.has_reached_saturation(tolerance) {
                return steps;
            }
        }
        
        max_steps
    }

    /// Obtiene estadísticas del sistema evolutivo
    pub fn get_stats(&self) -> KeygenStats {
        KeygenStats {
            current_value: self.current_keygen,
            iteration: self.iteration,
            history_len: self.history.len(),
            active_fields: self.get_active_fields().len(),
            growth_rate: self.growth_rate(),
            growth_acceleration: self.growth_acceleration(),
            distance_to_monster: MONSTER_DIM * (1.0 - self.current_keygen),
            love_intensity: self.love_operator.get_intensity(),
        }
    }

    /// Resetea a estado inicial manteniendo configuración
    pub fn reset(&mut self) {
        self.current_keygen = INITIAL_KEYGEN;
        self.iteration = 0;
        self.history = vec![INITIAL_KEYGEN];
        self.love_operator = LoveOperator::new(1.0);
    }

    /// Obtiene keygen actual
    pub fn get_current_keygen(&self) -> f64 {
        self.current_keygen
    }

    /// Obtiene iteración actual
    pub fn get_iteration(&self) -> u64 {
        self.iteration
    }

    /// Obtiene historial completo
    pub fn get_history(&self) -> &Vec<f64> {
        &self.history
    }

    /// Obtiene operador amor actual
    pub fn get_love_operator(&self) -> &LoveOperator {
        &self.love_operator
    }
}

/// Estadísticas del sistema evolutivo
#[derive(Clone, Debug)]
pub struct KeygenStats {
    pub current_value: f64,
    pub iteration: u64,
    pub history_len: usize,
    pub active_fields: usize,
    pub growth_rate: f64,
    pub growth_acceleration: f64,
    pub distance_to_monster: f64,
    pub love_intensity: f64,
}

/// Función auxiliar: evolución keygen batch para múltiples humanos
pub fn batch_evolution(initial_keygens: &[f64], steps: u64) -> Vec<Vec<f64>> {
    initial_keygens.iter()
        .map(|&initial| {
            let mut system = KeygenEvolution::new(Some(initial));
            system.evolve_steps(steps)
        })
        .collect()
}

/// Función auxiliar: encuentra punto fijo de la ecuación evolutiva
pub fn find_fixed_point(tolerance: f64, max_iterations: u64) -> Option<f64> {
    let mut z = INITIAL_KEYGEN;
    let mut system = KeygenEvolution::new(Some(z));
    
    for _ in 0..max_iterations {
        let z_next = system.evolve();
        if (z_next - z).abs() < tolerance {
            return Some(z_next);
        }
        z = z_next;
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_initialization() {
        let system = KeygenEvolution::new(None);
        let keygen = system.get_current_keygen();
        assert!(keygen > 0.0 && keygen < 1.0, "Keygen inicial fuera de rango: {}", keygen);
        assert_eq!(system.get_iteration(), 0);
        assert_eq!(system.get_history().len(), 1);
    }

    #[test]
    fn test_single_evolution() {
        let mut system = KeygenEvolution::new(None);
        let initial = system.get_current_keygen();
        let next = system.evolve();
        
        // Debe crecer
        assert!(next > initial, "Keygen debe crecer: {} > {}", next, initial);
        assert_eq!(system.get_iteration(), 1);
        assert_eq!(system.get_history().len(), 2);
    }

    #[test]
    fn test_multiple_evolutions() {
        let mut system = KeygenEvolution::new(None);
        let results = system.evolve_steps(10);
        
        assert_eq!(results.len(), 10);
        assert_eq!(system.get_iteration(), 10);
        assert_eq!(system.get_history().len(), 11);
        
        // Verificar crecimiento monótono
        for i in 1..results.len() {
            assert!(results[i] > results[i-1],
                   "El crecimiento debe ser monótono en paso {}: {} > {}",
                   i, results[i], results[i-1]);
        }
    }

    #[test]
    fn test_evolution_to_threshold() {
        let mut system = KeygenEvolution::new(None);
        let threshold = 0.5;
        
        match system.evolve_to_threshold(threshold, 1000) {
            Ok((steps, final_value)) => {
                assert!(final_value >= threshold);
                assert!(steps > 0);
                println!("Alcanzó umbral {} en {} pasos, valor final: {}", 
                        threshold, steps, final_value);
            },
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_active_fields() {
        let mut system = KeygenEvolution::new(None);
        
        // Inicialmente pocos campos activos
        let initial_fields = system.get_active_fields();
        println!("Campos activos iniciales: {:?}", initial_fields);
        assert!(initial_fields.len() <= 3, "Demasiados campos activos inicialmente: {}", initial_fields.len());
        
        // Evolucionar y ver más campos se activan
        system.evolve_steps(50);
        let later_fields = system.get_active_fields();
        println!("Campos activos después de 50 pasos: {:?}", later_fields);
        assert!(later_fields.len() >= initial_fields.len());
    }

    #[test]
    fn test_growth_metrics() {
        let mut system = KeygenEvolution::new(None);
        system.evolve_steps(5);
        
        let rate = system.growth_rate();
        let acceleration = system.growth_acceleration();
        
        println!("Tasa crecimiento: {:.6}", rate);
        println!("Aceleración: {:.6}", acceleration);
        
        // La tasa debe ser positiva
        assert!(rate > 0.0, "Tasa de crecimiento debe ser positiva: {}", rate);
    }

    #[test]
    fn test_projection() {
        let system = KeygenEvolution::new(None);
        let projection = system.project_future(20);
        
        assert_eq!(projection.len(), 20);
        
        // La proyección debe mostrar crecimiento
        for i in 1..projection.len() {
            assert!(projection[i] > projection[i-1],
                   "Proyección debe crecer en paso {}: {} > {}",
                   i, projection[i], projection[i-1]);
        }
        
        println!("Proyección 20 pasos: {:?}", projection);
    }

    #[test]
    fn test_stats_generation() {
        let mut system = KeygenEvolution::new(None);
        system.evolve_steps(5);
        
        let stats = system.get_stats();
        println!("Estadísticas: {:?}", stats);
        
        assert!(stats.current_value > 0.0);
        assert_eq!(stats.iteration, 5);
        assert!(stats.active_fields > 0);
        assert!(stats.growth_rate > 0.0);
        assert!(stats.love_intensity > 0.0);
    }

    #[test]
    fn test_batch_evolution() {
        let initials = vec![0.1, 0.5, 0.8];
        let results = batch_evolution(&initials, 10);
        
        assert_eq!(results.len(), 3);
        for (i, evolution) in results.iter().enumerate() {
            assert_eq!(evolution.len(), 10);
            println!("Batch {}: primeros valores: {:?}", i, &evolution[..3]);
        }
    }

    #[test]
    fn test_reset() {
        let mut system = KeygenEvolution::new(None);
        system.evolve_steps(25);
        let before_reset = system.get_current_keygen();
        
        system.reset();
        let after_reset = system.get_current_keygen();
        
        assert!(before_reset > after_reset);
        assert!((after_reset - INITIAL_KEYGEN).abs() < 1e-10);
        assert_eq!(system.get_iteration(), 0);
        assert_eq!(system.get_history().len(), 1);
    }

    #[test]
    fn test_saturation_detection() {
        // Sistema con keygen alto
        let mut system = KeygenEvolution::new(Some(0.999));
        system.evolve_steps(5);
        
        let saturated = system.has_reached_saturation(1e-3);
        let steps_to_sat = system.steps_to_saturation(1e-3);
        
        println!("¿Saturado? {}", saturated);
        println!("Pasos hasta saturación: {}", steps_to_sat);
        
        // steps_to_sat debe ser un número razonable
        assert!(steps_to_sat < 1000);
    }
}
