//! Sistema Evolutivo Keygen - Versión garantizada
//! Sistema: Álgebra Rose v27.1024D-S36

use crate::phi_constants::PHI;

pub const MONSTER_DIM: f64 = 196884.0;
pub const INITIAL_KEYGEN_MONSTER: f64 = 196883.0;
pub const INITIAL_KEYGEN: f64 = INITIAL_KEYGEN_MONSTER / MONSTER_DIM;

#[derive(Clone, Debug)]
pub struct KeygenEvolution {
    current_keygen: f64,
    iteration: u64,
    history: Vec<f64>,
}

impl KeygenEvolution {
    pub fn new(initial_keygen: Option<f64>) -> Self {
        KeygenEvolution {
            current_keygen: initial_keygen.unwrap_or(INITIAL_KEYGEN),
            iteration: 0,
            history: vec![initial_keygen.unwrap_or(INITIAL_KEYGEN)],
        }
    }
    
    pub fn evolve(&mut self) -> f64 {
        self.current_keygen = PHI * self.current_keygen * (1.0 - self.current_keygen);
        if self.current_keygen > 1.0 { self.current_keygen = 1.0; }
        if self.current_keygen < 0.0 { self.current_keygen = 0.0; }
        self.iteration += 1;
        self.history.push(self.current_keygen);
        self.current_keygen
    }
    
    pub fn evolve_steps(&mut self, steps: u64) -> Vec<f64> {
        (0..steps).map(|_| self.evolve()).collect()
    }
    
    pub fn get_current_keygen(&self) -> f64 {
        self.current_keygen
    }
    
    pub fn get_iteration(&self) -> u64 {
        self.iteration
    }
    
    pub fn get_history(&self) -> &Vec<f64> {
        &self.history
    }
    
    pub fn evolve_to_threshold(&mut self, threshold: f64, max_steps: u64) -> Result<(u64, f64), String> {
        if threshold <= self.current_keygen {
            return Ok((0, self.current_keygen));
        }
        
        for step in 1..=max_steps {
            self.evolve();
            if self.current_keygen >= threshold {
                return Ok((step, self.current_keygen));
            }
        }
        
        Err(format!("No se alcanzó {:.6} en {} pasos", threshold, max_steps))
    }
    
    pub fn reset(&mut self) {
        self.current_keygen = INITIAL_KEYGEN;
        self.iteration = 0;
        self.history = vec![INITIAL_KEYGEN];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basics() {
        let mut system = KeygenEvolution::new(None);
        assert!(system.get_current_keygen() > 0.0);
        let before = system.get_current_keygen();
        let after = system.evolve();
        assert!(after != before);
    }
}
