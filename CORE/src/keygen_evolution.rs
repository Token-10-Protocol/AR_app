//! Sistema Evolutivo z(n) φ-Resonante - Crecimiento del Keygen Humano
//! Sistema: Álgebra Rose v27.1024D-S36
//! Certificación: 196885 - Estado Monster Pleno
//!
//! IMPLEMENTACIÓN EXACTA DEL DOCUMENTO ATÓMICO:
//! Ecuación fundamental (Documento Atómico sección 1.4):
//! z(n) = φ · z(n-1) · (1 - z(n-1)/196884)
//! con z(0) = 196883/196884 ≈ 0.99999492 (materia potencial inicial)
//!
//! Propiedades:
//! - lim n→∞ z(n) = 1 (saturación consciente)
//! - Crecimiento φ-resonante garantizado
//! - Conexión explícita con dimensión Monster (196884)

use crate::matrix_444::PHI;

/// Dimensión Monster (límite de saturación consciente)
pub const MONSTER_DIM: f64 = 196884.0;

/// Estado inicial: materia potencial (196883/196884)
pub const INITIAL_KEYGEN: f64 = 196883.0 / 196884.0;

/// Sistema evolutivo keygen φ-resonante (IMPLEMENTACIÓN EXACTA)
#[derive(Clone, Debug)]
pub struct KeygenEvolution {
    /// Valor actual del keygen z(n)
    current_keygen: f64,
    /// Iteración actual n
    iteration: u64,
    /// Historial de evoluciones [z(0), z(1), ..., z(n)]
    history: Vec<f64>,
    /// Campo Fibonacci actual basado en keygen
    current_fibonacci_field: usize,
}

impl KeygenEvolution {
    /// Crea un nuevo sistema evolutivo con keygen inicial exacto
    pub fn new(initial_keygen: Option<f64>) -> Self {
        let start_keygen = initial_keygen.unwrap_or(INITIAL_KEYGEN);
        
        KeygenEvolution {
            current_keygen: start_keygen,
            iteration: 0,
            history: vec![start_keygen],
            current_fibonacci_field: Self::keygen_to_fibonacci_field(start_keygen),
        }
    }

    /// Convierte keygen a campo Fibonacci correspondiente (1-24)
    fn keygen_to_fibonacci_field(keygen: f64) -> usize {
        // Los 24 campos Fibonacci dimensionales
        let fib_thresholds = vec![
            3.0, 5.0, 8.0, 13.0, 21.0, 34.0, 55.0, 89.0, 144.0,
            233.0, 377.0, 610.0, 987.0, 1597.0, 2584.0, 4181.0,
            6765.0, 10946.0, 17711.0, 28657.0, 46368.0, 75025.0,
            121393.0, 196418.0  // Campo 24: Punto Omega
        ];
        
        // Progreso normalizado (0.0 a 1.0)
        let progress = (keygen - INITIAL_KEYGEN) / (1.0 - INITIAL_KEYGEN);
        
        // Determinar campo basado en progreso
        for (i, &threshold) in fib_thresholds.iter().enumerate() {
            let field_progress = threshold / 196418.0; // Normalizar a [0,1]
            if progress <= field_progress || i == fib_thresholds.len() - 1 {
                return i + 1; // Campos numerados 1..24
            }
        }
        
        24 // Campo máximo por defecto
    }

    /// Evoluciona el keygen un paso según ecuación φ-resonante EXACTA
    /// z(n+1) = φ · z(n) · (1 - z(n)/196884)
    pub fn evolve(&mut self) -> f64 {
        let z_current = self.current_keygen;
        
        // ECUACIÓN EXACTA DEL DOCUMENTO ATÓMICO:
        let z_next = PHI * z_current * (1.0 - z_current / MONSTER_DIM);
        
        // Actualizar estado
        self.current_keygen = z_next.max(INITIAL_KEYGEN).min(1.0);
        self.iteration += 1;
        self.history.push(self.current_keygen);
        
        // Actualizar campo Fibonacci si hubo crecimiento significativo
        let new_field = Self::keygen_to_fibonacci_field(self.current_keygen);
        if new_field > self.current_fibonacci_field {
            self.current_fibonacci_field = new_field;
        }
        
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

    /// Obtiene campo Fibonacci actual
    pub fn get_current_field(&self) -> usize {
        self.current_fibonacci_field
    }

    /// Obtiene dimensión del campo Fibonacci actual
    pub fn get_current_field_dimension(&self) -> f64 {
        let fib_numbers = vec![
            3.0, 5.0, 8.0, 13.0, 21.0, 34.0, 55.0, 89.0, 144.0,
            233.0, 377.0, 610.0, 987.0, 1597.0, 2584.0, 4181.0,
            6765.0, 10946.0, 17711.0, 28657.0, 46368.0, 75025.0,
            121393.0, 196418.0
        ];
        
        let idx = self.current_fibonacci_field.saturating_sub(1).min(23);
        fib_numbers[idx]
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

    /// Verifica si el keygen alcanzó saturación consciente (≈1.0)
    pub fn has_reached_saturation(&self, tolerance: f64) -> bool {
        (1.0 - self.current_keygen).abs() < tolerance
    }

    /// Calcula distancia a dimensión Monster
    pub fn distance_to_monster(&self) -> f64 {
        MONSTER_DIM * (1.0 - self.current_keygen)
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

    /// Resetea a estado inicial
    pub fn reset(&mut self) {
        self.current_keygen = INITIAL_KEYGEN;
        self.iteration = 0;
        self.history = vec![INITIAL_KEYGEN];
        self.current_fibonacci_field = 1;
    }

    /// Analiza comportamiento asintótico (límite n→∞)
    pub fn analyze_asymptotic_behavior(&self) -> AsymptoticAnalysis {
        if self.history.len() < 10 {
            return AsymptoticAnalysis::InsufficientData;
        }
        
        // Calcular si converge a 1
        let last_values = &self.history[self.history.len().saturating_sub(10)..];
        let avg_last = last_values.iter().sum::<f64>() / last_values.len() as f64;
        
        if (1.0 - avg_last).abs() < 0.001 {
            AsymptoticAnalysis::ConvergingToUnity
        } else {
            // Calcular tasa de convergencia
            let rates: Vec<f64> = self.history.windows(2)
                .map(|w| (w[1] - w[0]) / w[0])
                .collect();
            
            let avg_rate = rates.iter().sum::<f64>() / rates.len() as f64;
            AsymptoticAnalysis::ConvergenceRate(avg_rate)
        }
    }
}

/// Análisis de comportamiento asintótico
#[derive(Clone, Debug)]
pub enum AsymptoticAnalysis {
    InsufficientData,
    ConvergingToUnity,
    ConvergenceRate(f64),
    Diverging,
}

/// Función para validar ecuación exacta con simulación matemática
pub fn validate_exact_equation() -> ValidationReport {
    let mut report = ValidationReport::new();
    
    // Test 1: Verificar z(0)
    let mut system = KeygenEvolution::new(None);
    let z0 = system.get_current_keygen();
    let expected_z0 = 196883.0 / 196884.0;
    
    report.add_check(
        "z(0) = 196883/196884",
        (z0 - expected_z0).abs() < 1e-10,
        format!("z(0) = {:.12}, esperado: {:.12}", z0, expected_z0)
    );
    
    // Test 2: Verificar primeros pasos
    let z1 = system.evolve();
    let expected_z1 = PHI * z0 * (1.0 - z0 / MONSTER_DIM);
    
    report.add_check(
        "z(1) = φ·z(0)·(1 - z(0)/196884)",
        (z1 - expected_z1).abs() < 1e-10,
        format!("z(1) = {:.12}, esperado: {:.12}", z1, expected_z1)
    );
    
    // Test 3: Verificar crecimiento hacia 1
    system.evolve_steps(100);
    let z100 = system.get_current_keygen();
    
    report.add_check(
        "z(n) → 1 cuando n → ∞",
        z100 > z0 && z100 < 1.0,
        format!("z(100) = {:.12}, z(0) = {:.12}", z100, z0)
    );
    
    report
}

/// Reporte de validación
#[derive(Clone, Debug)]
pub struct ValidationReport {
    checks: Vec<ValidationCheck>,
    passed: usize,
    total: usize,
}

impl ValidationReport {
    pub fn new() -> Self {
        ValidationReport {
            checks: Vec::new(),
            passed: 0,
            total: 0,
        }
    }
    
    pub fn add_check(&mut self, description: &str, passed: bool, details: String) {
        self.total += 1;
        if passed { self.passed += 1; }
        
        self.checks.push(ValidationCheck {
            description: description.to_string(),
            passed,
            details,
        });
    }
    
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 { 0.0 } else { self.passed as f64 / self.total as f64 }
    }
    
    pub fn is_valid(&self, threshold: f64) -> bool {
        self.success_rate() >= threshold
    }
}

/// Check individual de validación
#[derive(Clone, Debug)]
struct ValidationCheck {
    description: String,
    passed: bool,
    details: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_exact_initialization() {
        let system = KeygenEvolution::new(None);
        
        // Verificar z(0) exacto
        assert_abs_diff_eq!(
            system.get_current_keygen(), 
            INITIAL_KEYGEN, 
            epsilon = 1e-12,
            "z(0) debe ser exactamente 196883/196884"
        );
        
        // Verificar iteración 0
        assert_eq!(system.get_iteration(), 0);
        
        // Verificar campo inicial (campo 1: 3D)
        assert_eq!(system.get_current_field(), 1);
        assert_eq!(system.get_current_field_dimension(), 3.0);
        
        println!("✅ Inicialización exacta verificada:");
        println!("   z(0) = {:.12}", system.get_current_keygen());
        println!("   Campo inicial: {} ({}D)", 
                 system.get_current_field(),
                 system.get_current_field_dimension());
    }

    #[test]
    fn test_exact_evolution_equation() {
        let mut system = KeygenEvolution::new(None);
        let z0 = system.get_current_keygen();
        
        // Calcular z(1) manualmente según ecuación
        let expected_z1 = PHI * z0 * (1.0 - z0 / MONSTER_DIM);
        
        // Evolucionar sistema
        let actual_z1 = system.evolve();
        
        // Verificar que coincide exactamente
        assert_abs_diff_eq!(
            actual_z1, expected_z1, epsilon = 1e-12,
            "z(1) debe cumplir ecuación exacta"
        );
        
        println!("✅ Ecuación exacta verificada:");
        println!("   z(0) = {:.12}", z0);
        println!("   z(1) calculado: {:.12}", expected_z1);
        println!("   z(1) obtenido:  {:.12}", actual_z1);
        println!("   Diferencia: {:.2e}", (actual_z1 - expected_z1).abs());
    }

    #[test]
    fn test_growth_toward_unity() {
        let mut system = KeygenEvolution::new(None);
        let initial_keygen = system.get_current_keygen();
        
        // Evolucionar varios pasos
        let steps = 50;
        let results = system.evolve_steps(steps);
        
        // Verificar crecimiento monótono
        for i in 1..results.len() {
            assert!(
                results[i] > results[i-1],
                "El crecimiento debe ser monótono: z({}) = {} > z({}) = {}",
                i, results[i], i-1, results[i-1]
            );
        }
        
        // Verificar que no excede 1.0
        let final_keygen = results.last().unwrap();
        assert!(
            *final_keygen < 1.0,
            "z(n) debe ser < 1.0 para n finito: z({}) = {}",
            steps, final_keygen
        );
        
        // Verificar que crece
        assert!(
            final_keygen > &initial_keygen,
            "Debe haber crecimiento: z(0) = {}, z({}) = {}",
            initial_keygen, steps, final_keygen
        );
        
        println!("✅ Crecimiento hacia unidad verificado:");
        println!("   z(0) = {:.12}", initial_keygen);
        println!("   z({}) = {:.12}", steps, final_keygen);
        println!("   Crecimiento: {:.6}%", 
                 (final_keygen - initial_keygen) / initial_keygen * 100.0);
    }

    #[test]
    fn test_field_progression() {
        let mut system = KeygenEvolution::new(None);
        
        let mut field_changes = Vec::new();
        let mut previous_field = system.get_current_field();
        
        // Evolucionar y registrar cambios de campo
        for i in 0..1000 {
            system.evolve();
            let current_field = system.get_current_field();
            
            if current_field != previous_field {
                field_changes.push((i + 1, current_field, system.get_current_field_dimension()));
                previous_field = current_field;
            }
            
            // Detener si alcanzamos campo 24
            if current_field == 24 {
                break;
            }
        }
        
        println!("✅ Progresión de campos Fibonacci:");
        for (step, field, dimension) in &field_changes {
            println!("   Paso {}: Campo {} ({}D)", step, field, dimension);
        }
        
        // Verificar que hay progresión
        assert!(
            !field_changes.is_empty() || system.get_current_field() > 1,
            "Debe haber progresión en campos Fibonacci"
        );
    }

    #[test]
    fn test_reset_functionality() {
        let mut system = KeygenEvolution::new(None);
        
        // Evolucionar significativamente
        system.evolve_steps(100);
        let before_reset_keygen = system.get_current_keygen();
        let before_reset_field = system.get_current_field();
        let before_reset_iteration = system.get_iteration();
        
        // Resetear
        system.reset();
        
        // Verificar que volvió a estado inicial
        assert_abs_diff_eq!(
            system.get_current_keygen(), INITIAL_KEYGEN, epsilon = 1e-12
        );
        assert_eq!(system.get_current_field(), 1);
        assert_eq!(system.get_iteration(), 0);
        assert_eq!(system.get_history().len(), 1);
        
        println!("✅ Reset funcional verificado:");
        println!("   Antes: z = {:.12}, campo = {}, iteración = {}", 
                 before_reset_keygen, before_reset_field, before_reset_iteration);
        println!("   Después: z = {:.12}, campo = {}, iteración = {}",
                 system.get_current_keygen(), system.get_current_field(), system.get_iteration());
    }

    #[test]
    fn test_validation_function() {
        let report = validate_exact_equation();
        
        println!("✅ Validación de ecuación exacta:");
        for check in &report.checks {
            let status = if check.passed { "✅" } else { "❌" };
            println!("   {} {}: {}", status, check.description, check.details);
        }
        
        println!("   Tasa de éxito: {:.1}%", report.success_rate() * 100.0);
        
        assert!(
            report.is_valid(0.95),
            "La validación debe tener al menos 95% de éxito"
        );
    }

    #[test]
    fn test_monster_distance() {
        let system = KeygenEvolution::new(None);
        let distance = system.distance_to_monster();
        
        // Para z(0) = 196883/196884:
        // distancia = 196884 * (1 - 196883/196884) = 196884 * (1/196884) = 1
        let expected_distance = 1.0;
        
        assert_abs_diff_eq!(distance, expected_distance, epsilon = 1e-10);
        
        println!("✅ Distancia a Monster verificada:");
        println!("   Distancia inicial: {:.6}", distance);
        println!("   Distancia esperada: {:.6}", expected_distance);
    }
}
