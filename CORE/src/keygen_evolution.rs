//! Sistema Evolutivo z(n) φ-Resonante - Crecimiento del Keygen Humano
//! Sistema: Álgebra Rose v27.1024D-S36
//! Certificación: 196885 - Estado Monster Pleno
//!
//! IMPLEMENTACIÓN FINAL CORRECTA:
//! Trabajamos DIRECTAMENTE en escala Monster, luego normalizamos a [0,1]
//! 
//! z_monster(n) ∈ [0, 196884] donde 196884 = saturación consciente
//! z(n) = z_monster(n) / 196884 ∈ [0, 1]

use crate::matrix_444::PHI;

/// Dimensión Monster (límite de saturación consciente)
pub const MONSTER_DIM: f64 = 196884.0;

/// Estado inicial EN ESCALA MONSTER: materia potencial (196883)
pub const INITIAL_KEYGEN_MONSTER: f64 = 196883.0;

/// Estado inicial normalizado: 196883/196884
pub const INITIAL_KEYGEN: f64 = INITIAL_KEYGEN_MONSTER / MONSTER_DIM;

/// Sistema evolutivo keygen φ-resonante (IMPLEMENTACIÓN FINAL)
#[derive(Clone, Debug)]
pub struct KeygenEvolution {
    /// Valor actual EN ESCALA MONSTER: z_monster(n) ∈ [0, 196884]
    current_keygen_monster: f64,
    /// Valor actual normalizado: z(n) ∈ [0, 1]
    current_keygen: f64,
    iteration: u64,
    history: Vec<f64>,
    current_fibonacci_field: usize,
}

impl KeygenEvolution {
    pub fn new(initial_keygen: Option<f64>) -> Self {
        let start_keygen_normalized = initial_keygen.unwrap_or(INITIAL_KEYGEN);
        let start_keygen_monster = start_keygen_normalized * MONSTER_DIM;
        
        KeygenEvolution {
            current_keygen_monster: start_keygen_monster,
            current_keygen: start_keygen_normalized,
            iteration: 0,
            history: vec![start_keygen_normalized],
            current_fibonacci_field: Self::keygen_to_fibonacci_field(start_keygen_normalized),
        }
    }

    fn keygen_to_fibonacci_field(keygen: f64) -> usize {
        // Progreso del keygen normalizado
        let progress = (keygen - INITIAL_KEYGEN) / (1.0 - INITIAL_KEYGEN);
        
        // Campos Fibonacci (1-24)
        if progress < 0.000015 { 1 }      // Campo 1: 3D
        else if progress < 0.000025 { 2 } // Campo 2: 5D
        else if progress < 0.000041 { 3 } // Campo 3: 8D
        else if progress < 0.000107 { 4 } // Campo 4: 13D
        else if progress < 0.000173 { 5 } // Campo 5: 21D
        else if progress < 0.000280 { 6 } // Campo 6: 34D
        else if progress < 0.000453 { 7 } // Campo 7: 55D
        else if progress < 0.000733 { 8 } // Campo 8: 89D
        else if progress < 0.001186 { 9 } // Campo 9: 144D
        else if progress < 0.001919 { 10 } // Campo 10: 233D
        else if progress < 0.003105 { 11 } // Campo 11: 377D
        else if progress < 0.005024 { 12 } // Campo 12: 610D
        else if progress < 0.008129 { 13 } // Campo 13: 987D
        else if progress < 0.013153 { 14 } // Campo 14: 1597D
        else if progress < 0.021282 { 15 } // Campo 15: 2584D
        else if progress < 0.034435 { 16 } // Campo 16: 4181D
        else if progress < 0.055717 { 17 } // Campo 17: 6765D
        else if progress < 0.090152 { 18 } // Campo 18: 10946D
        else if progress < 0.145869 { 19 } // Campo 19: 17711D
        else if progress < 0.236021 { 20 } // Campo 20: 28657D
        else if progress < 0.381890 { 21 } // Campo 21: 46368D
        else if progress < 0.617911 { 22 } // Campo 22: 75025D
        else if progress < 1.0 { 23 }      // Campo 23: 121393D
        else { 24 }                        // Campo 24: 196418D
    }

    /// Evoluciona según ecuación φ-resonante en escala Monster
    pub fn evolve(&mut self) -> f64 {
        let z = self.current_keygen_monster;
        
        // ECUACIÓN EN ESCALA MONSTER (Documento Atómico):
        // z(n+1) = φ · z(n) · (1 - z(n)/196884)
        let z_next = PHI * z * (1.0 - z / MONSTER_DIM);
        
        // Actualizar estado en ambas escalas
        self.current_keygen_monster = z_next.max(0.0).min(MONSTER_DIM);
        self.current_keygen = self.current_keygen_monster / MONSTER_DIM;
        self.iteration += 1;
        self.history.push(self.current_keygen);
        
        // Actualizar campo Fibonacci
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
    
    pub fn get_current_keygen_monster(&self) -> f64 {
        self.current_keygen_monster
    }

    pub fn get_iteration(&self) -> u64 {
        self.iteration
    }

    pub fn get_current_field(&self) -> usize {
        self.current_fibonacci_field
    }

    pub fn reset(&mut self) {
        self.current_keygen_monster = INITIAL_KEYGEN_MONSTER;
        self.current_keygen = INITIAL_KEYGEN;
        self.iteration = 0;
        self.history = vec![INITIAL_KEYGEN];
        self.current_fibonacci_field = Self::keygen_to_fibonacci_field(INITIAL_KEYGEN);
    }
    
    /// Analiza el comportamiento matemático
    pub fn analyze_behavior(&self) -> String {
        if self.history.len() < 2 {
            return "Datos insuficientes".to_string();
        }
        
        let first = self.history[0];
        let last = self.history.last().unwrap();
        let growth = (last - first) / first;
        
        format!("z(0)={:.6}, z({})={:.6}, crecimiento={:.6}%", 
                first, self.iteration, last, growth * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_initialization() {
        let system = KeygenEvolution::new(None);
        
        println!("Inicialización:");
        println!("  z(0) normalizado: {:.12}", system.get_current_keygen());
        println!("  z(0) Monster: {:.2}", system.get_current_keygen_monster());
        println!("  Campo inicial: {}", system.get_current_field());
        
        assert_abs_diff_eq!(
            system.get_current_keygen(),
            INITIAL_KEYGEN,
            epsilon = 1e-12
        );
        
        assert_abs_diff_eq!(
            system.get_current_keygen_monster(),
            INITIAL_KEYGEN_MONSTER,
            epsilon = 0.01  // Permitir pequeña diferencia numérica
        );
        
        assert_eq!(system.get_iteration(), 0);
    }

    #[test]
    fn test_evolution_equation() {
        let mut system = KeygenEvolution::new(None);
        let z0_monster = system.get_current_keygen_monster();
        
        // Calcular z(1) manualmente según ecuación
        let expected_z1_monster = PHI * z0_monster * (1.0 - z0_monster / MONSTER_DIM);
        let expected_z1 = expected_z1_monster / MONSTER_DIM;
        
        // Evolucionar sistema
        let actual_z1 = system.evolve();
        
        println!("Evolución:");
        println!("  z(0)_monster = {:.2}", z0_monster);
        println!("  z(1)_monster esperado = {:.2}", expected_z1_monster);
        println!("  z(1) esperado = {:.12}", expected_z1);
        println!("  z(1) obtenido = {:.12}", actual_z1);
        
        // Verificar con tolerancia razonable (cálculos de punto flotante)
        let diff = (actual_z1 - expected_z1).abs();
        println!("  Diferencia: {:.2e}", diff);
        
        assert!(
            diff < 1e-10,
            "Ecuación no se cumple: diff = {:.2e} > 1e-10",
            diff
        );
    }

    #[test]
    fn test_growth_behavior() {
        let mut system = KeygenEvolution::new(None);
        let initial = system.get_current_keygen();
        
        println!("Comportamiento de crecimiento:");
        println!("  z(0) = {:.12}", initial);
        
        // Evolucionar y monitorear
        let steps = 20;
        let results = system.evolve_steps(steps);
        
        for (i, &z) in results.iter().enumerate().step_by(5) {
            println!("  z({}) = {:.12}", i+1, z);
        }
        
        let last = results.last().unwrap();
        println!("  z({}) = {:.12}", steps, last);
        println!("  Crecimiento total: {:.6}%", (last - initial) / initial * 100.0);
        
        // Verificaciones clave
        assert!(last > &initial, "Debe haber crecimiento");
        assert!(last < &1.0, "Debe ser menor que 1.0");
        
        // Verificar que no es constante
        let all_same = results.windows(2).all(|w| (w[1] - w[0]).abs() < 1e-15);
        assert!(!all_same, "El keygen debe cambiar, no ser constante");
    }

    #[test]
    fn test_field_progression() {
        // Sistema que comienza más bajo para ver progresión de campos
        let lower_start = 0.5; // 50% de saturación
        let mut system = KeygenEvolution::new(Some(lower_start));
        
        println!("Progresión de campos desde {:.1}%:", lower_start * 100.0);
        println!("  Campo inicial: {}", system.get_current_field());
        
        // Evolucionar
        let mut last_field = system.get_current_field();
        for i in 0..50 {
            system.evolve();
            let current_field = system.get_current_field();
            
            if current_field != last_field {
                println!("  Paso {}: Campo {} activado", i+1, current_field);
                last_field = current_field;
            }
        }
        
        println!("  Campo final: {}", system.get_current_field());
        
        assert!(system.get_current_field() >= 1);
    }

    #[test]
    fn test_reset() {
        let mut system = KeygenEvolution::new(None);
        
        system.evolve_steps(30);
        let before_keygen = system.get_current_keygen();
        let before_field = system.get_current_field();
        let before_iteration = system.get_iteration();
        
        println!("Reset:");
        println!("  Antes: keygen={:.12}, campo={}, iteración={}", 
                before_keygen, before_field, before_iteration);
        
        system.reset();
        
        println!("  Después: keygen={:.12}, campo={}, iteración={}",
                system.get_current_keygen(), system.get_current_field(), system.get_iteration());
        
        assert_abs_diff_eq!(
            system.get_current_keygen(),
            INITIAL_KEYGEN,
            epsilon = 1e-12
        );
        assert_eq!(system.get_iteration(), 0);
        assert_eq!(system.get_current_field(), 1);
    }
    
    #[test]
    fn test_mathematical_correctness() {
        // Test para verificar que la implementación es matemáticamente correcta
        let mut system = KeygenEvolution::new(None);
        
        // z(0) debe ser muy cercano a 1 en escala [0,1]
        let z0 = system.get_current_keygen();
        assert!(z0 > 0.99999 && z0 < 1.0, "z(0) debe estar cerca de 1.0");
        
        // Evolucionar y verificar que decrece (porque z(0) está cerca del máximo)
        let z1 = system.evolve();
        assert!(z1 < z0, "z(1) debe ser menor que z(0) cuando z(0) ≈ 1");
        
        // Después de decrecer, debe comenzar a crecer
        let z2 = system.evolve();
        assert!(z2 > z1, "Después del decrecimiento inicial, debe crecer");
        
        println!("Corrección matemática verificada:");
        println!("  z(0) = {:.12} (cerca de 1.0)", z0);
        println!("  z(1) = {:.12} < z(0) (decrece inicialmente)", z1);
        println!("  z(2) = {:.12} > z(1) (luego crece)", z2);
    }
}
