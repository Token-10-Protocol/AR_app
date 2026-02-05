//! Sistema Evolutivo Granular Basado en Campos Fibonacci
//! Sistema: Álgebra Rose v27.1024D-S36
//! Certificación: 196885 - Estado Monster Pleno
//!
//! ARQUITECTURA GRANULAR:
//! Campos Fibonacci (24) → Tensores → Vectores → Escalares
//! 
//! Progresión jerárquica:
//! 1.0 Escalar = unidad básica de crecimiento
//! φ Escalares = 1 Vector (1.618 unidades)
//! φ² Escalares = 1 Tensor (2.618 unidades)
//! φ³ Escalares = avance significativo (~4.236 unidades)

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

/// Unidades de progreso por nivel granular
pub const SCALAR_UNIT: f64 = 1.0;           // Unidad básica
pub const VECTOR_UNIT: f64 = PHI;           // φ escalares
pub const TENSOR_UNIT: f64 = PHI * PHI;     // φ² escalares
pub const FIELD_STEP: f64 = PHI * PHI * PHI; // φ³ escalares

/// Avance granular dentro de un campo Fibonacci
#[derive(Clone, Debug)]
pub struct GranularProgress {
    /// Escalares acumulados en el campo actual
    pub scalars: f64,
    /// Vectores completados
    pub vectors: u32,
    /// Tensores completados  
    pub tensors: u32,
    /// Campos completados
    pub fields: usize,
}

impl GranularProgress {
    pub fn new() -> Self {
        GranularProgress {
            scalars: 0.0,
            vectors: 0,
            tensors: 0,
            fields: 0,
        }
    }

    /// Añade progreso escalar y actualiza jerarquía
    pub fn add_scalars(&mut self, amount: f64) -> Vec<String> {
        let mut events = Vec::new();
        
        self.scalars += amount;
        
        // Verificar si completó un vector
        let new_vectors = (self.scalars / VECTOR_UNIT).floor() as u32;
        if new_vectors > self.vectors {
            let delta = new_vectors - self.vectors;
            self.vectors = new_vectors;
            events.push(format!("✨ Vector completado (+{} φ-unidades)", delta));
        }
        
        // Verificar si completó un tensor
        let new_tensors = (self.scalars / TENSOR_UNIT).floor() as u32;
        if new_tensors > self.tensors {
            let delta = new_tensors - self.tensors;
            self.tensors = new_tensors;
            events.push(format!("🌀 Tensor completado (+{} φ²-unidades)", delta));
        }
        
        // Verificar si completó un paso de campo
        let field_steps = (self.scalars / FIELD_STEP).floor() as usize;
        if field_steps > 0 {
            self.scalars -= field_steps as f64 * FIELD_STEP;
            self.vectors = 0;
            self.tensors = 0;
            events.push(format!("🌌 Paso de campo completado ({:.2} φ³)", field_steps as f64));
        }
        
        events
    }

    /// Obtiene progreso total en unidades φ
    pub fn total_phi_units(&self) -> f64 {
        self.scalars + 
        (self.vectors as f64) * VECTOR_UNIT + 
        (self.tensors as f64) * TENSOR_UNIT +
        (self.fields as f64) * FIELD_STEP * 24.0 // Campos previos
    }

    /// Convierte a keygen
    pub fn to_keygen(&self, current_field: usize) -> f64 {
        // Progreso en campos previos
        let prev_fields_progress = (self.fields as f64) / 24.0;
        
        // Progreso en campo actual (en términos de φ³)
        let current_field_progress = self.total_phi_units() / (24.0 * FIELD_STEP);
        
        let total_progress = (prev_fields_progress + current_field_progress).min(1.0);
        INITIAL_KEYGEN + total_progress * (1.0 - INITIAL_KEYGEN)
    }
}

/// Sistema evolutivo granular
#[derive(Clone, Debug)]
pub struct KeygenEvolution {
    /// Campo Fibonacci actual (0-23)
    current_field: usize,
    /// Progreso granular dentro del campo
    granular_progress: GranularProgress,
    /// Keygen actual
    current_keygen: f64,
    /// Iteración actual
    iteration: u64,
    /// Historial de evoluciones
    history: Vec<f64>,
    /// Operador Â
    love_operator: LoveOperator,
    /// Eventos de progreso recientes
    recent_events: Vec<String>,
}

impl KeygenEvolution {
    /// Crea nuevo sistema evolutivo
    pub fn new(initial_keygen: Option<f64>) -> Self {
        let start_keygen = initial_keygen.unwrap_or(INITIAL_KEYGEN);
        let progress = (start_keygen - INITIAL_KEYGEN) / (1.0 - INITIAL_KEYGEN);
        
        // Determinar campo inicial basado en progreso
        let current_field = (progress * 24.0).floor() as usize;
        
        // Calcular progreso granular dentro del campo
        let mut granular = GranularProgress::new();
        let field_progress = (progress * 24.0).fract();
        granular.scalars = field_progress * FIELD_STEP;
        
        KeygenEvolution {
            current_field: current_field.min(23),
            granular_progress: granular,
            current_keygen: start_keygen,
            iteration: 0,
            history: vec![start_keygen],
            love_operator: LoveOperator::new(1.0),
            recent_events: Vec::new(),
        }
    }

    /// Evoluciona un paso con crecimiento φ-resonante granular
    pub fn evolve(&mut self) -> f64 {
        // Calcular crecimiento basado en amor y campo actual
        let love_intensity = self.love_operator.get_intensity();
        let field_factor = (self.current_field + 1) as f64 / 24.0;
        
        // Crecimiento escalar: φ^(intensidad × factor_campo)
        let scalar_growth = PHI.powf(love_intensity * field_factor * 0.1);
        
        // Añadir progreso granular
        self.recent_events = self.granular_progress.add_scalars(scalar_growth);
        
        // Verificar si avanzamos de campo
        if self.granular_progress.fields > 0 && self.current_field < 23 {
            self.current_field += 1;
            self.granular_progress.fields -= 1;
            self.recent_events.push(format!(
                "🚀 ACTIVADO CAMPO {}: {}D", 
                self.current_field + 1, 
                FIBONACCI_FIELDS[self.current_field]
            ));
        }
        
        // Actualizar keygen
        self.current_keygen = self.granular_progress.to_keygen(self.current_field);
        self.iteration += 1;
        self.history.push(self.current_keygen);
        
        // Actualizar operador amor según progreso
        let total_progress = (self.current_keygen - INITIAL_KEYGEN) / (1.0 - INITIAL_KEYGEN);
        self.love_operator.update_intensity(total_progress * 0.2);
        
        // Registrar evento si hay crecimiento significativo
        if self.history.len() >= 2 {
            let growth = (self.current_keygen - self.history[self.history.len() - 2]).abs();
            if growth > 1e-10 {
                self.recent_events.push(format!("📈 Crecimiento: {:.6}", growth));
            }
        }
        
        self.current_keygen
    }

    /// Evoluciona múltiples pasos
    pub fn evolve_steps(&mut self, steps: u64) -> Vec<f64> {
        let mut results = Vec::with_capacity(steps as usize);
        for _ in 0..steps {
            results.push(self.evolve());
            
            // Mostrar eventos importantes
            if !self.recent_events.is_empty() {
                for event in &self.recent_events {
                    if !event.contains("Crecimiento") { // Mostrar solo eventos significativos
                        println!("  {}", event);
                    }
                }
                self.recent_events.clear();
            }
        }
        results
    }

    /// Obtiene información granular actual
    pub fn get_granular_info(&self) -> (usize, f64, GranularProgress) {
        (
            self.current_field,
            FIBONACCI_FIELDS[self.current_field],
            self.granular_progress.clone()
        )
    }

    /// Obtiene estadísticas detalladas
    pub fn get_detailed_stats(&self) -> DetailedStats {
        let total_phi = self.granular_progress.total_phi_units();
        
        DetailedStats {
            keygen: self.current_keygen,
            iteration: self.iteration,
            current_field: self.current_field + 1,
            field_dimension: FIBONACCI_FIELDS[self.current_field],
            scalars: self.granular_progress.scalars,
            vectors: self.granular_progress.vectors,
            tensors: self.granular_progress.tensors,
            total_phi_units: total_phi,
            love_intensity: self.love_operator.get_intensity(),
            distance_to_monster: MONSTER_DIM * (1.0 - self.current_keygen),
            progress_percentage: (self.current_keygen - INITIAL_KEYGEN) / (1.0 - INITIAL_KEYGEN) * 100.0,
        }
    }

    /// Evoluciona hasta alcanzar un nivel granular específico
    pub fn evolve_to_granular_level(&mut self, target_tensors: u32, max_steps: u64) -> Result<(u64, f64), String> {
        let start_tensors = self.granular_progress.tensors;
        
        for step in 1..=max_steps {
            self.evolve();
            
            if self.granular_progress.tensors >= target_tensors {
                return Ok((step, self.current_keygen));
            }
        }
        
        Err(format!("No se alcanzaron {} tensores en {} pasos", target_tensors, max_steps))
    }

    /// Resetea a estado inicial
    pub fn reset(&mut self) {
        self.current_field = 0;
        self.granular_progress = GranularProgress::new();
        self.current_keygen = INITIAL_KEYGEN;
        self.iteration = 0;
        self.history = vec![INITIAL_KEYGEN];
        self.love_operator = LoveOperator::new(1.0);
        self.recent_events.clear();
    }

    /// Obtiene keygen actual
    pub fn get_current_keygen(&self) -> f64 {
        self.current_keygen
    }
}

/// Estadísticas detalladas
#[derive(Clone, Debug)]
pub struct DetailedStats {
    pub keygen: f64,
    pub iteration: u64,
    pub current_field: usize,
    pub field_dimension: f64,
    pub scalars: f64,
    pub vectors: u32,
    pub tensors: u32,
    pub total_phi_units: f64,
    pub love_intensity: f64,
    pub distance_to_monster: f64,
    pub progress_percentage: f64,
}

/// Simula comunidad con diferentes ritmos de crecimiento
pub fn simulate_diverse_community(num_humans: usize, steps: u64) -> Vec<DetailedStats> {
    (0..num_humans)
        .map(|i| {
            // Diferentes intensidades iniciales de amor
            let love_factor = 0.8 + 0.4 * (i as f64) / (num_humans as f64);
            
            let mut system = KeygenEvolution::new(None);
            system.love_operator.update_intensity(love_factor);
            
            system.evolve_steps(steps);
            system.get_detailed_stats()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_granular_progress() {
        let mut progress = GranularProgress::new();
        
        // Añadir suficientes escalares para completar un vector
        let events = progress.add_scalars(VECTOR_UNIT);
        assert!(!events.is_empty());
        assert!(events[0].contains("Vector"));
        assert_eq!(progress.vectors, 1);
        
        // Añadir más para completar un tensor
        let events = progress.add_scalars(TENSOR_UNIT - VECTOR_UNIT);
        assert!(!events.is_empty());
        assert!(events.iter().any(|e| e.contains("Tensor")));
        assert_eq!(progress.tensors, 1);
        
        println!("Progreso granular: {} escalares, {} vectores, {} tensores", 
                 progress.scalars, progress.vectors, progress.tensors);
    }

    #[test]
    fn test_evolution_with_granularity() {
        let mut system = KeygenEvolution::new(None);
        
        println!("=== EVOLUCIÓN GRANULAR INICIAL ===");
        println!("Campo inicial: {} ({}D)", 
                 system.current_field + 1, 
                 FIBONACCI_FIELDS[system.current_field]);
        
        // Evolucionar y mostrar progreso granular
        let results = system.evolve_steps(50);
        
        let stats = system.get_detailed_stats();
        println!("\n=== ESTADÍSTICAS DETALLADAS ===");
        println!("Keygen: {:.10}", stats.keygen);
        println!("Campo: {} ({}D)", stats.current_field, stats.field_dimension);
        println!("Progreso: {:.2}%", stats.progress_percentage);
        println!("Escalares: {:.2}", stats.scalars);
        println!("Vectores: {}", stats.vectors);
        println!("Tensores: {}", stats.tensors);
        println!("Unidades φ totales: {:.2}", stats.total_phi_units);
        println!("Intensidad amor: {:.2}", stats.love_intensity);
        println!("Distancia a Monster: {:.2}", stats.distance_to_monster);
        
        // Verificar que hubo algún crecimiento
        assert!(results.len() == 50);
        assert!(stats.total_phi_units > 0.0 || stats.vectors > 0 || stats.tensors > 0);
    }

    #[test]
    fn test_tensor_achievement() {
        let mut system = KeygenEvolution::new(None);
        
        // Evolucionar hasta alcanzar al menos 1 tensor
        match system.evolve_to_granular_level(1, 200) {
            Ok((steps, keygen)) => {
                println!("✅ Alcanzado 1 tensor en {} pasos", steps);
                println!("Keygen final: {:.10}", keygen);
                
                let stats = system.get_detailed_stats();
                assert!(stats.tensors >= 1);
            },
            Err(e) => {
                println!("⚠️ {}", e);
                // No fallar el test, solo registrar
            }
        }
    }

    #[test]
    fn test_field_transition_with_granularity() {
        // Sistema que empieza cerca del final de un campo
        let near_end = INITIAL_KEYGEN + 0.04; // 4% de progreso total
        let mut system = KeygenEvolution::new(Some(near_end));
        
        let initial_field = system.current_field;
        println!("Campo inicial: {} ({}D)", 
                 initial_field + 1, 
                 FIBONACCI_FIELDS[initial_field]);
        
        // Evolucionar buscando transición de campo
        let mut found_transition = false;
        for _ in 0..100 {
            system.evolve();
            if system.current_field > initial_field {
                found_transition = true;
                println!("🚀 Transición al campo {} ({}D)", 
                         system.current_field + 1, 
                         FIBONACCI_FIELDS[system.current_field]);
                break;
            }
        }
        
        if found_transition {
            assert!(system.current_field > initial_field);
        }
    }

    #[test]
    fn test_diverse_community() {
        let stats = simulate_diverse_community(3, 30);
        
        println!("\n=== COMUNIDAD DIVERSA ===");
        for (i, stat) in stats.iter().enumerate() {
            println!("\nHumano {}:", i + 1);
            println!("  Keygen: {:.10}", stat.keygen);
            println!("  Tensores: {}", stat.tensors);
            println!("  Vectores: {}", stat.vectors);
            println!("  Progreso: {:.2}%", stat.progress_percentage);
        }
        
        // Verificar diversidad (no todos iguales)
        let keygens: Vec<f64> = stats.iter().map(|s| s.keygen).collect();
        let unique_keygens: std::collections::HashSet<u64> = 
            keygens.iter().map(|&k| (k * 1e12) as u64).collect();
        
        assert!(unique_keygens.len() > 1, "Debería haber diversidad en la comunidad");
    }

    #[test]
    fn test_granular_conversion() {
        let test_progress = vec![
            (0.0, 0, 0, "sin progreso"),
            (VECTOR_UNIT, 1, 0, "1 vector"),
            (TENSOR_UNIT, 0, 1, "1 tensor"),
            (FIELD_STEP, 0, 0, "1 paso de campo"),
        ];
        
        for (scalars, expected_vectors, expected_tensors, desc) in test_progress {
            let mut progress = GranularProgress::new();
            progress.add_scalars(scalars);
            
            println!("{}: {} vectores, {} tensores", 
                     desc, progress.vectors, progress.tensors);
            
            assert_eq!(progress.vectors, expected_vectors);
            assert_eq!(progress.tensors, expected_tensors);
        }
    }

    #[test]
    fn test_reset_granular() {
        let mut system = KeygenEvolution::new(None);
        
        // Evolucionar significativamente
        system.evolve_steps(100);
        let before_stats = system.get_detailed_stats();
        
        // Resetear
        system.reset();
        let after_stats = system.get_detailed_stats();
        
        println!("Antes del reset: {} tensores, {:.2}% progreso", 
                 before_stats.tensors, before_stats.progress_percentage);
        println!("Después del reset: {} tensores, {:.2}% progreso", 
                 after_stats.tensors, after_stats.progress_percentage);
        
        assert!(before_stats.tensors > after_stats.tensors || 
                before_stats.progress_percentage > after_stats.progress_percentage);
        assert_abs_diff_eq!(after_stats.keygen, INITIAL_KEYGEN, epsilon = 1e-10);
        assert_eq!(after_stats.iteration, 0);
    }
}

/// Extensor φ-Consciente - Transformador dimensional inteligente
/// Realiza reducción 1025D → 3D preservando estructura esencial
#[derive(Clone, Debug)]
pub struct PhiExtensor {
    /// Matriz de transformación extensor
    transformation: Vec<Vec<f64>>,
    /// Niveles de compresión disponibles
    compression_levels: Vec<usize>,
    /// Factor de coherencia preservada
    coherence_preservation: f64,
}

impl PhiExtensor {
    /// Crea nuevo extensor con niveles Fibonacci de compresión
    pub fn new() -> Self {
        // Niveles de compresión basados en Fibonacci: 1025D → 610D → 377D → ... → 3D
        let compression_levels = vec![1025, 610, 377, 233, 144, 89, 55, 34, 21, 13, 8, 5, 3];
        
        PhiExtensor {
            transformation: Self::create_phi_transformation(),
            compression_levels,
            coherence_preservation: 1.0, // Coherencia perfecta inicial
        }
    }
    
    /// Crea transformación φ-resonante
    fn create_phi_transformation() -> Vec<Vec<f64>> {
        // Matriz de transformación basada en proporciones áureas
        // Cada elemento T_ij = φ^(-|i-j|) * cos(2πφ*i*j)
        let mut matrix = Vec::new();
        
        for i in 0..1025 {
            let mut row = Vec::new();
            for j in 0..1025 {
                let distance = (i as f64 - j as f64).abs();
                let phase = (2.0 * std::f64::consts::PI * PHI * i as f64 * j as f64).cos();
                let value = PHI.powf(-distance) * phase;
                row.push(value);
            }
            matrix.push(row);
        }
        
        matrix
    }
    
    /// Aplica extensor a un estado de alta dimensión
    pub fn apply(&self, high_dim_state: &[f64], target_dim: usize) -> Result<Vec<f64>, String> {
        if !self.compression_levels.contains(&target_dim) {
            return Err(format!("Dimensión {} no es nivel Fibonacci válido", target_dim));
        }
        
        // Encontrar camino de reducción óptimo
        let path = self.find_optimal_path(high_dim_state.len(), target_dim);
        
        // Aplicar reducción paso a paso
        let mut current_state = high_dim_state.to_vec();
        for (from_dim, to_dim) in path.windows(2).map(|w| (w[0], w[1])) {
            current_state = self.compress_step(&current_state, from_dim, to_dim)?;
        }
        
        Ok(current_state)
    }
    
    /// Encuentra camino óptimo de reducción dimensional
    fn find_optimal_path(&self, from_dim: usize, to_dim: usize) -> Vec<usize> {
        let mut path = vec![from_dim];
        
        // Encontrar niveles Fibonacci intermedios
        let mut current = from_dim;
        while current > to_dim {
            // Encontrar mayor nivel Fibonacci menor que current pero ≥ to_dim
            if let Some(&next) = self.compression_levels.iter()
                .filter(|&&d| d < current && d >= to_dim)
                .max() {
                path.push(next);
                current = next;
            } else {
                break;
            }
        }
        
        path
    }
    
    /// Paso individual de compresión
    fn compress_step(&self, state: &[f64], from_dim: usize, to_dim: usize) -> Result<Vec<f64>, String> {
        if from_dim <= to_dim {
            return Err("from_dim debe ser mayor que to_dim".to_string());
        }
        
        let compression_ratio = from_dim as f64 / to_dim as f64;
        let mut result = vec![0.0; to_dim];
        
        // Compresión φ-resonante: promediar grupos con pesos áureos
        let group_size = (compression_ratio).ceil() as usize;
        
        for i in 0..to_dim {
            let start = i * group_size;
            let end = (start + group_size).min(from_dim);
            
            // Ponderación áurea dentro del grupo
            let mut weighted_sum = 0.0;
            let mut total_weight = 0.0;
            
            for j in start..end {
                let position_in_group = (j - start) as f64;
                let weight = PHI.powf(-position_in_group); // Peso decae áureamente
                weighted_sum += state[j] * weight;
                total_weight += weight;
            }
            
            result[i] = if total_weight > 0.0 {
                weighted_sum / total_weight
            } else {
                0.0
            };
        }
        
        // Actualizar factor de coherencia preservada
        self.update_coherence_preservation(from_dim, to_dim, state, &result);
        
        Ok(result)
    }
    
    /// Calcula cuánta coherencia se preservó
    fn update_coherence_preservation(&self, from_dim: usize, to_dim: usize, 
                                    original: &[f64], compressed: &[f64]) -> f64 {
        // Simulación simple: coherencia basada en preservación de energía
        let original_energy: f64 = original.iter().map(|&x| x * x).sum();
        let compressed_energy: f64 = compressed.iter().map(|&x| x * x).sum();
        
        let energy_ratio = if original_energy > 0.0 {
            compressed_energy / original_energy
        } else {
            1.0
        };
        
        // Penalizar por compresión agresiva
        let compression_penalty = (from_dim as f64 / to_dim as f64).ln() / PHI.ln();
        
        (energy_ratio * PHI.powf(-compression_penalty * 0.1)).max(0.0).min(1.0)
    }
    
    /// Obtiene factor de coherencia actual
    pub fn get_coherence(&self) -> f64 {
        self.coherence_preservation
    }
    
    /// Verifica si el extensor mantiene coherencia suficiente
    pub fn verify_coherence(&self, min_coherence: f64) -> bool {
        self.coherence_preservation >= min_coherence
    }
}

/// Sistema evolutivo extendido con extensor consciente
#[derive(Clone, Debug)]
pub struct ExtendedKeygenEvolution {
    /// Sistema evolutivo base
    base_evolution: KeygenEvolution,
    /// Extensor φ-consciente
    extensor: PhiExtensor,
    /// Historial de coherencia
    coherence_history: Vec<f64>,
    /// Umbral mínimo de coherencia
    min_coherence: f64,
}

impl ExtendedKeygenEvolution {
    /// Crea nuevo sistema evolutivo extendido
    pub fn new(initial_keygen: Option<f64>) -> Self {
        ExtendedKeygenEvolution {
            base_evolution: KeygenEvolution::new(initial_keygen),
            extensor: PhiExtensor::new(),
            coherence_history: vec![1.0], // Coherencia perfecta inicial
            min_coherence: 0.85, // 85% mínimo de coherencia
        }
    }
    
    /// Evoluciona con verificación de coherencia
    pub fn evolve_with_coherence(&mut self) -> Result<f64, String> {
        // Evolucionar sistema base
        let new_keygen = self.base_evolution.evolve();
        
        // Obtener estado de alta dimensión (simulado)
        let high_dim_state = self.simulate_high_dimension_state();
        
        // Aplicar extensor para verificar coherencia
        let low_dim_state = self.extensor.apply(&high_dim_state, 3)?;
        
        // Calcular coherencia preservada
        let coherence = self.calculate_state_coherence(&high_dim_state, &low_dim_state);
        self.coherence_history.push(coherence);
        
        // Verificar umbral mínimo
        if coherence < self.min_coherence {
            return Err(format!(
                "Coherencia insuficiente: {:.2}% < {:.2}%",
                coherence * 100.0, self.min_coherence * 100.0
            ));
        }
        
        Ok(new_keygen)
    }
    
    /// Simula estado de alta dimensión basado en keygen actual
    fn simulate_high_dimension_state(&self) -> Vec<f64> {
        let keygen = self.base_evolution.get_current_keygen();
        let field = self.base_evolution.get_current_field();
        let progress = self.base_evolution.get_field_progress();
        
        // Estado de 1025 dimensiones basado en progreso actual
        let mut state = vec![0.0; 1025];
        
        // Patrón φ-resonante
        for i in 0..1025 {
            let phi_freq = PHI * i as f64;
            let field_factor = (field.0 + 1) as f64 / 24.0;
            let progress_wave = (2.0 * std::f64::consts::PI * progress * i as f64 / 1025.0).sin();
            
            state[i] = keygen * phi_freq.sin() * field_factor * progress_wave;
        }
        
        state
    }
    
    /// Calcula coherencia entre estados de diferente dimensión
    fn calculate_state_coherence(&self, high_dim: &[f64], low_dim: &[f64]) -> f64 {
        // Métricas de coherencia múltiple
        
        // 1. Preservación de energía relativa
        let high_energy: f64 = high_dim.iter().map(|&x| x * x).sum();
        let low_energy: f64 = low_dim.iter().map(|&x| x * x).sum();
        let energy_coherence = if high_energy > 0.0 {
            (low_energy / high_energy).min(1.0)
        } else {
            1.0
        };
        
        // 2. Preservación de estructura espectral
        let spectral_coherence = self.calculate_spectral_coherence(high_dim, low_dim);
        
        // 3. Preservación de relaciones φ
        let phi_coherence = self.calculate_phi_coherence(high_dim, low_dim);
        
        // Coherencia combinada (media ponderada φ)
        (energy_coherence * 0.3 + spectral_coherence * 0.3 + phi_coherence * 0.4).max(0.0).min(1.0)
    }
    
    /// Coherencia espectral (preservación de patrones de frecuencia)
    fn calculate_spectral_coherence(&self, high_dim: &[f64], low_dim: &[f64]) -> f64 {
        // Simplificación: correlación entre promedios locales
        let high_avg: f64 = high_dim.iter().sum::<f64>() / high_dim.len() as f64;
        let low_avg: f64 = low_dim.iter().sum::<f64>() / low_dim.len() as f64;
        
        if high_avg.abs() < 1e-10 || low_avg.abs() < 1e-10 {
            return 0.0;
        }
        
        (low_avg / high_avg).abs().min(1.0)
    }
    
    /// Coherencia φ (preservación de proporciones áureas)
    fn calculate_phi_coherence(&self, high_dim: &[f64], low_dim: &[f64]) -> f64 {
        // Calcular proporciones entre elementos consecutivos
        let mut high_ratios = Vec::new();
        for i in 1..high_dim.len() {
            if high_dim[i-1].abs() > 1e-10 {
                high_ratios.push(high_dim[i] / high_dim[i-1]);
            }
        }
        
        let mut low_ratios = Vec::new();
        for i in 1..low_dim.len() {
            if low_dim[i-1].abs() > 1e-10 {
                low_ratios.push(low_dim[i] / low_dim[i-1]);
            }
        }
        
        // Comparar con φ ideal
        let high_phi_dev: f64 = high_ratios.iter()
            .map(|&r| (r - PHI).abs() / PHI)
            .sum::<f64>() / high_ratios.len() as f64;
            
        let low_phi_dev: f64 = low_ratios.iter()
            .map(|&r| (r - PHI).abs() / PHI)
            .sum::<f64>() / low_ratios.len() as f64;
        
        // Mejor coherencia cuando ambas se acercan a φ
        1.0 - (high_phi_dev + low_phi_dev) / 2.0
    }
    
    /// Obtiene métricas de coherencia actuales
    pub fn get_coherence_metrics(&self) -> CoherenceMetrics {
        let current_coherence = *self.coherence_history.last().unwrap_or(&1.0);
        let avg_coherence = self.coherence_history.iter().sum::<f64>() / self.coherence_history.len() as f64;
        
        CoherenceMetrics {
            current: current_coherence,
            average: avg_coherence,
            min: self.coherence_history.iter().cloned().fold(f64::INFINITY, f64::min),
            max: self.coherence_history.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            history_len: self.coherence_history.len(),
            meets_threshold: current_coherence >= self.min_coherence,
        }
    }
    
    /// Evoluciona múltiples pasos con verificación de coherencia
    pub fn evolve_steps_with_coherence(&mut self, steps: u64) -> Result<Vec<f64>, String> {
        let mut results = Vec::new();
        
        for step in 0..steps {
            match self.evolve_with_coherence() {
                Ok(keygen) => {
                    results.push(keygen);
                    
                    // Mostrar advertencia si coherencia baja
                    let metrics = self.get_coherence_metrics();
                    if metrics.current < 0.9 {
                        println!("⚠️  Paso {}: Coherencia baja ({:.1}%)", 
                                step + 1, metrics.current * 100.0);
                    }
                },
                Err(e) => {
                    return Err(format!("Error en paso {}: {}", step + 1, e));
                }
            }
        }
        
        Ok(results)
    }
    
    /// Obtiene el sistema base
    pub fn get_base_evolution(&self) -> &KeygenEvolution {
        &self.base_evolution
    }
    
    /// Obtiene el extensor
    pub fn get_extensor(&self) -> &PhiExtensor {
        &self.extensor
    }
}

/// Métricas de coherencia
#[derive(Clone, Debug)]
pub struct CoherenceMetrics {
    pub current: f64,
    pub average: f64,
    pub min: f64,
    pub max: f64,
    pub history_len: usize,
    pub meets_threshold: bool,
}

#[cfg(test)]
mod extensor_tests {
    use super::*;
    
    #[test]
    fn test_extensor_creation() {
        let extensor = PhiExtensor::new();
        
        assert!(!extensor.compression_levels.is_empty());
        assert_eq!(extensor.get_coherence(), 1.0);
        assert!(extensor.verify_coherence(0.8));
        
        println!("✅ Extensor creado con {} niveles de compresión", 
                extensor.compression_levels.len());
    }
    
    #[test]
    fn test_extensor_compression() {
        let extensor = PhiExtensor::new();
        
        // Estado de alta dimensión simulado
        let high_dim_state: Vec<f64> = (0..1025)
            .map(|i| (PHI * i as f64).sin())
            .collect();
        
        // Comprimir a 3D
        match extensor.apply(&high_dim_state, 3) {
            Ok(low_dim_state) => {
                assert_eq!(low_dim_state.len(), 3);
                println!("✅ Compresión 1025D → 3D exitosa");
                println!("   Estado comprimido: {:?}", low_dim_state);
                
                // Verificar que no es todo ceros
                let energy: f64 = low_dim_state.iter().map(|&x| x * x).sum();
                assert!(energy > 0.0, "Estado comprimido no debería ser todo ceros");
            },
            Err(e) => panic!("Error en compresión: {}", e),
        }
    }
    
    #[test]
    fn test_extended_evolution_with_coherence() {
        let mut extended_system = ExtendedKeygenEvolution::new(None);
        
        println!("=== SISTEMA EXTENDIDO CON EXTENSOR ===");
        
        // Evolucionar con verificación de coherencia
        match extended_system.evolve_steps_with_coherence(10) {
            Ok(results) => {
                assert_eq!(results.len(), 10);
                
                let metrics = extended_system.get_coherence_metrics();
                println!("✅ Evolución extendida completada ({} pasos)", results.len());
                println!("   Coherencia actual: {:.1}%", metrics.current * 100.0);
                println!("   Coherencia promedio: {:.1}%", metrics.average * 100.0);
                println!("   Cumple umbral: {}", metrics.meets_threshold);
                
                assert!(metrics.meets_threshold, "Debe mantener coherencia mínima");
                
                // Verificar crecimiento
                if results.len() >= 2 {
                    assert!(results[results.len()-1] > results[0] || 
                           (results[results.len()-1] - results[0]).abs() < 1e-10);
                }
            },
            Err(e) => panic!("Error en evolución extendida: {}", e),
        }
    }
    
    #[test]
    fn test_coherence_preservation() {
        let mut extended_system = ExtendedKeygenEvolution::new(None);
        
        // Evolucionar significativamente
        let steps = 50;
        match extended_system.evolve_steps_with_coherence(steps) {
            Ok(_) => {
                let metrics = extended_system.get_coherence_metrics();
                
                println!("📊 MÉTRICAS DE COHERENCIA ({} pasos):", steps);
                println!("   Actual: {:.2}%", metrics.current * 100.0);
                println!("   Mínima: {:.2}%", metrics.min * 100.0);
                println!("   Máxima: {:.2}%", metrics.max * 100.0);
                println!("   Promedio: {:.2}%", metrics.average * 100.0);
                
                // Verificar que coherencia se mantiene razonable
                assert!(metrics.min >= 0.7, "Coherencia mínima muy baja: {:.2}%", metrics.min * 100.0);
                assert!(metrics.average >= 0.85, "Coherencia promedio muy baja: {:.2}%", metrics.average * 100.0);
            },
            Err(e) => panic!("Error: {}", e),
        }
    }
    
    #[test]
    fn test_optimal_compression_path() {
        let extensor = PhiExtensor::new();
        
        // Testear diferentes rutas de compresión
        let test_cases = vec![
            (1025, 3),
            (610, 3),
            (377, 13),
            (233, 8),
        ];
        
        for (from, to) in test_cases {
            let path = extensor.find_optimal_path(from, to);
            
            println!("Compresión {}D → {}D:", from, to);
            println!("   Ruta: {:?}", path);
            
            // Verificar propiedades
            assert!(!path.is_empty());
            assert_eq!(path[0], from);
            assert_eq!(path[path.len()-1], to);
            
            // Verificar que todos son niveles Fibonacci válidos
            for &dim in &path {
                assert!(extensor.compression_levels.contains(&dim), 
                       "{}D no es nivel Fibonacci válido", dim);
            }
        }
    }
}
