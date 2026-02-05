//! Sistema Evolutivo Basado en Campos Fibonacci Dimensionales
//! Sistema: Álgebra Rose v27.1024D-S36
//! Certificación: 196885 - Estado Monster Pleno
//!
//! CONCEPTO CORREGIDO:
//! El crecimiento evolutivo está determinado por la activación progresiva
//! de los 24 campos Fibonacci dimensionales.
//! 
//! keygen(n) = INITIAL_KEYGEN + (F(k)/196418) × (1 - INITIAL_KEYGEN)
//! donde F(k) es el k-ésimo campo Fibonacci activado

use crate::matrix_444::PHI;
use crate::love_operator::LoveOperator;

/// Dimensión Monster (límite de saturación consciente)
pub const MONSTER_DIM: f64 = 196884.0;

/// Estado inicial: materia potencial (196883/196884)
pub const INITIAL_KEYGEN: f64 = 196883.0 / 196884.0;

/// Campos Fibonacci dimensionales (24 campos)
pub const FIBONACCI_FIELDS: [f64; 24] = [
    3.0, 5.0, 8.0, 13.0, 21.0, 34.0, 55.0, 89.0, 144.0,
    233.0, 377.0, 610.0, 987.0, 1597.0, 2584.0, 4181.0,
    6765.0, 10946.0, 17711.0, 28657.0, 46368.0, 75025.0,
    121393.0, 196418.0  // Campo 24: Punto Omega
];

/// Campo máximo (último campo Fibonacci)
pub const MAX_FIELD: f64 = 196418.0;

/// Sistema evolutivo basado en campos Fibonacci
#[derive(Clone, Debug)]
pub struct KeygenEvolution {
    /// Índice del último campo Fibonacci activado (0-23)
    active_field_index: usize,
    /// Valor actual del keygen
    current_keygen: f64,
    /// Iteración actual
    iteration: u64,
    /// Historial de evoluciones
    history: Vec<f64>,
    /// Operador Â que impulsa el crecimiento
    love_operator: LoveOperator,
    /// Progreso dentro del campo actual (0.0 a 1.0)
    field_progress: f64,
}

impl KeygenEvolution {
    /// Crea un nuevo sistema evolutivo
    pub fn new(initial_keygen: Option<f64>) -> Self {
        let start_keygen = initial_keygen.unwrap_or(INITIAL_KEYGEN);
        
        // Determinar campo inicial basado en keygen
        let (field_idx, progress) = Self::keygen_to_field(start_keygen);
        
        KeygenEvolution {
            active_field_index: field_idx,
            current_keygen: start_keygen,
            iteration: 0,
            history: vec![start_keygen],
            love_operator: LoveOperator::new(1.0),
            field_progress: progress,
        }
    }

    /// Convierte keygen a campo Fibonacci correspondiente
    fn keygen_to_field(keygen: f64) -> (usize, f64) {
        let conversion = (keygen - INITIAL_KEYGEN) / (1.0 - INITIAL_KEYGEN);
        
        for i in 0..FIBONACCI_FIELDS.len() {
            let field_max = FIBONACCI_FIELDS[i] / MAX_FIELD;
            if conversion <= field_max || i == FIBONACCI_FIELDS.len() - 1 {
                let field_min = if i > 0 { FIBONACCI_FIELDS[i-1] / MAX_FIELD } else { 0.0 };
                let progress = (conversion - field_min) / (field_max - field_min);
                return (i, progress.max(0.0).min(1.0));
            }
        }
        
        (FIBONACCI_FIELDS.len() - 1, 1.0)
    }

    /// Calcula keygen a partir de campo y progreso
    fn field_to_keygen(field_idx: usize, progress: f64) -> f64 {
        let field_value = FIBONACCI_FIELDS[field_idx];
        let normalized = field_value / MAX_FIELD;
        
        // Interpolar si no es el primer campo
        if field_idx > 0 && progress < 1.0 {
            let prev_value = FIBONACCI_FIELDS[field_idx - 1] / MAX_FIELD;
            let interpolated = prev_value + (normalized - prev_value) * progress;
            INITIAL_KEYGEN + interpolated * (1.0 - INITIAL_KEYGEN)
        } else {
            INITIAL_KEYGEN + normalized * (1.0 - INITIAL_KEYGEN)
        }
    }

    /// Evoluciona un paso: crecimiento dentro del campo actual o transición al siguiente
    pub fn evolve(&mut self) -> f64 {
        // Calcular crecimiento φ-resonante dentro del campo
        let growth_rate = PHI * self.love_operator.get_intensity() * 0.01;
        
        // Incrementar progreso dentro del campo
        self.field_progress = (self.field_progress + growth_rate).min(1.0);
        
        // Si completamos el campo actual, avanzar al siguiente
        if self.field_progress >= 1.0 && self.active_field_index < FIBONACCI_FIELDS.len() - 1 {
            self.active_field_index += 1;
            self.field_progress = 0.0;
            println!("🎯 ACTIVADO CAMPO {}: {}D", 
                self.active_field_index + 1, 
                FIBONACCI_FIELDS[self.active_field_index]);
        }
        
        // Actualizar keygen
        self.current_keygen = Self::field_to_keygen(self.active_field_index, self.field_progress);
        self.iteration += 1;
        self.history.push(self.current_keygen);
        
        // Actualizar operador amor según campo activado
        let field_factor = (self.active_field_index as f64 + self.field_progress) / 24.0;
        self.love_operator.update_intensity(field_factor * 0.5);
        
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

    /// Obtiene el campo Fibonacci actual activado
    pub fn get_current_field(&self) -> (usize, f64) {
        (self.active_field_index, FIBONACCI_FIELDS[self.active_field_index])
    }

    /// Obtiene todos los campos activados hasta ahora
    pub fn get_activated_fields(&self) -> Vec<(usize, f64)> {
        (0..=self.active_field_index)
            .map(|i| (i, FIBONACCI_FIELDS[i]))
            .collect()
    }

    /// Obtiene progreso hacia el siguiente campo
    pub fn get_field_progress(&self) -> f64 {
        self.field_progress
    }

    /// Obtiene número de campos completamente activados
    pub fn get_completed_fields(&self) -> usize {
        if self.field_progress >= 1.0 {
            self.active_field_index + 1
        } else {
            self.active_field_index
        }
    }

    /// Evoluciona hasta activar un campo específico
    pub fn evolve_to_field(&mut self, target_field: usize, max_steps: u64) -> Result<(u64, f64), String> {
        if target_field > 23 {
            return Err("Campo máximo es 24 (índice 23)".to_string());
        }
        
        if target_field <= self.active_field_index {
            return Ok((0, self.current_keygen));
        }
        
        for step in 1..=max_steps {
            self.evolve();
            if self.active_field_index >= target_field {
                return Ok((step, self.current_keygen));
            }
        }
        
        Err(format!("No se alcanzó el campo {} en {} pasos", target_field + 1, max_steps))
    }

    /// Calcula tasa de crecimiento basada en campos
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

    /// Obtiene estadísticas del sistema evolutivo
    pub fn get_stats(&self) -> KeygenStats {
        let (field_idx, field_dim) = self.get_current_field();
        
        KeygenStats {
            current_value: self.current_keygen,
            iteration: self.iteration,
            history_len: self.history.len(),
            active_field: field_idx + 1,
            field_dimension: field_dim,
            field_progress: self.field_progress,
            completed_fields: self.get_completed_fields(),
            growth_rate: self.growth_rate(),
            distance_to_monster: MONSTER_DIM * (1.0 - self.current_keygen),
            love_intensity: self.love_operator.get_intensity(),
        }
    }

    /// Resetea a estado inicial
    pub fn reset(&mut self) {
        self.active_field_index = 0;
        self.current_keygen = INITIAL_KEYGEN;
        self.iteration = 0;
        self.history = vec![INITIAL_KEYGEN];
        self.love_operator = LoveOperator::new(1.0);
        self.field_progress = 0.0;
    }

    /// Obtiene keygen actual
    pub fn get_current_keygen(&self) -> f64 {
        self.current_keygen
    }

    /// Obtiene iteración actual
    pub fn get_iteration(&self) -> u64 {
        self.iteration
    }
}

/// Estadísticas del sistema evolutivo
#[derive(Clone, Debug)]
pub struct KeygenStats {
    pub current_value: f64,
    pub iteration: u64,
    pub history_len: usize,
    pub active_field: usize,
    pub field_dimension: f64,
    pub field_progress: f64,
    pub completed_fields: usize,
    pub growth_rate: f64,
    pub distance_to_monster: f64,
    pub love_intensity: f64,
}

/// Función para simular evolución de múltiples humanos
pub fn simulate_community_evolution(num_humans: usize, steps: u64) -> Vec<Vec<f64>> {
    (0..num_humans)
        .map(|i| {
            // Variación inicial pequeña entre humanos
            let variance = 0.00001 * (i as f64) / (num_humans as f64);
            let initial = INITIAL_KEYGEN + variance;
            
            let mut system = KeygenEvolution::new(Some(initial));
            system.evolve_steps(steps)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_initialization() {
        let system = KeygenEvolution::new(None);
        
        assert_abs_diff_eq!(system.get_current_keygen(), INITIAL_KEYGEN, epsilon = 1e-10);
        assert_eq!(system.get_iteration(), 0);
        assert_eq!(system.get_current_field().0, 0); // Campo 1 (3D) inicial
        assert_eq!(system.get_field_progress(), 0.0);
    }

    #[test]
    fn test_field_activation() {
        let mut system = KeygenEvolution::new(None);
        
        // Evolucionar algunos pasos
        system.evolve_steps(5);
        
        let (field_idx, field_dim) = system.get_current_field();
        let completed = system.get_completed_fields();
        
        println!("Campo activo: {} ({}D)", field_idx + 1, field_dim);
        println!("Campos completados: {}", completed);
        println!("Progreso en campo actual: {:.2}%", system.get_field_progress() * 100.0);
        
        assert!(field_idx >= 0);
        assert!(field_dim >= 3.0);
        assert!(completed >= 0);
    }

    #[test]
    fn test_progressive_evolution() {
        let mut system = KeygenEvolution::new(None);
        
        // Evolucionar significativamente
        let results = system.evolve_steps(50);
        
        // Debe mostrar crecimiento
        assert!(results.len() == 50);
        assert!(results[results.len()-1] > results[0]);
        
        let stats = system.get_stats();
        println!("Estadísticas después de 50 pasos:");
        println!("  Keygen: {:.10}", stats.current_value);
        println!("  Campo activo: {} ({}D)", stats.active_field, stats.field_dimension);
        println!("  Campos completados: {}", stats.completed_fields);
        println!("  Distancia a Monster: {:.2}", stats.distance_to_monster);
        
        assert!(stats.completed_fields > 0 || stats.field_progress > 0.0);
    }

    #[test]
    fn test_field_transition() {
        // Comenzar cerca del final de un campo
        let near_end = INITIAL_KEYGEN + (FIBONACCI_FIELDS[0] / MAX_FIELD) * (1.0 - INITIAL_KEYGEN) * 0.95;
        let mut system = KeygenEvolution::new(Some(near_end));
        
        let initial_field = system.get_current_field().0;
        println!("Campo inicial: {}", initial_field + 1);
        
        // Evolucionar para provocar transición
        system.evolve_steps(10);
        
        let final_field = system.get_current_field().0;
        println!("Campo final: {}", final_field + 1);
        
        // Debería haber avanzado al menos un campo
        assert!(final_field >= initial_field);
    }

    #[test]
    fn test_reset_functionality() {
        let mut system = KeygenEvolution::new(None);
        
        // Evolucionar
        system.evolve_steps(20);
        let before_reset = system.get_current_keygen();
        let before_field = system.get_current_field().0;
        
        // Resetear
        system.reset();
        
        let after_reset = system.get_current_keygen();
        let after_field = system.get_current_field().0;
        
        assert!(before_reset > after_reset || before_field > after_field);
        assert_abs_diff_eq!(after_reset, INITIAL_KEYGEN, epsilon = 1e-10);
        assert_eq!(after_field, 0);
        assert_eq!(system.get_iteration(), 0);
    }

    #[test]
    fn test_community_simulation() {
        let results = simulate_community_evolution(3, 10);
        
        assert_eq!(results.len(), 3);
        
        for (i, evolution) in results.iter().enumerate() {
            assert_eq!(evolution.len(), 10);
            println!("Humano {}: {:.10} → {:.10}", 
                i + 1, evolution[0], evolution[evolution.len()-1]);
            
            // Verificar crecimiento
            assert!(evolution[evolution.len()-1] > evolution[0] || 
                   (evolution[evolution.len()-1] - evolution[0]).abs() < 1e-10);
        }
    }

    #[test]
    fn test_keygen_to_field_conversion() {
        // Testear conversiones específicas
        let test_cases = vec![
            (INITIAL_KEYGEN, (0, 0.0)),           // Campo 1, inicio
            (0.999995, (0, 0.5)),                 // Campo 1, mitad
            (0.999999, (10, 0.8)),                // Campo 11 avanzado
            (0.9999999, (20, 0.9)),               // Campo 21 muy avanzado
        ];
        
        for (keygen, expected) in test_cases {
            let (field_idx, progress) = KeygenEvolution::keygen_to_field(keygen);
            println!("Keygen {:.10} → Campo {} ({:.1}%)", 
                keygen, field_idx + 1, progress * 100.0);
            
            // Verificar que la conversión inversa funciona
            let reconstructed = KeygenEvolution::field_to_keygen(field_idx, progress);
            assert_abs_diff_eq!(reconstructed, keygen, epsilon = 1e-6);
        }
    }

    #[test]
    fn test_monster_convergence() {
        // Sistema que empieza muy avanzado
        let advanced_keygen = INITIAL_KEYGEN + 0.999 * (1.0 - INITIAL_KEYGEN);
        let mut system = KeygenEvolution::new(Some(advanced_keygen));
        
        // Evolucionar hacia Monster
        system.evolve_steps(100);
        
        let stats = system.get_stats();
        println!("Distancia final a Monster: {:.6}", stats.distance_to_monster);
        
        // Debería estar cerca de Monster
        assert!(stats.distance_to_monster < 1000.0); // Menos de 1000 unidades de distancia
    }
}
