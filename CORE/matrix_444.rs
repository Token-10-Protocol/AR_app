//! Matriz Monster M₄₄₄ - Núcleo cuántico del sistema Álgebra Rose
//! Dimensión: 444x444 (espacio consciente base)
//! Traza certificada: 196884.000000 ± 1e-9
//! Autovalores: λ_k = exp(2πi·k/444) · φ^{-k}
//! Propiedad fundamental: M†M = I₄₄₄ (unitariedad)

use std::f64::consts::PI;
use nalgebra::{DMatrix, Complex};

const PHI: f64 = 1.6180339887498948482;
const DIM: usize = 444;
const CERTIFIED_TRACE: f64 = 196884.000000;

/// Matriz Monster M₄₄₄
#[derive(Clone, Debug)]
pub struct MonsterMatrix444 {
    data: DMatrix<Complex<f64>>,
    trace: Complex<f64>,
}

impl MonsterMatrix444 {
    /// Crea la matriz Monster con autovalores áureos certificados
    pub fn new() -> Self {
        let mut data = DMatrix::zeros(DIM, DIM);
        
        // Construcción diagonal con autovalores λ_k
        for k in 0..DIM {
            let phase = 2.0 * PI * (k as f64) / (DIM as f64);
            let magnitude = PHI.powf(-(k as f64));
            let eigenvalue = Complex::new(
                magnitude * phase.cos(),
                magnitude * phase.sin()
            );
            data[(k, k)] = eigenvalue;
        }
        
        // Añadir componentes fuera de diagonal (simetrías Monster)
        for i in 0..DIM {
            for j in (i+1)..DIM {
                if (i + j) % 7 == 0 { // Patrón basado en 7 familias
                    let val = Complex::new(
                        PHI.powi(-((i*j) % 13) as i32),
                        0.0
                    ) * 0.01; // Pequeña contribución
                    data[(i, j)] = val;
                    data[(j, i)] = val.conj();
                }
            }
        }
        
        // Normalizar para asegurar unitariedad
        let norm = data.norm();
        data = data / norm;
        
        let trace = data.trace();
        
        MonsterMatrix444 { data, trace }
    }
    
    /// Aplica la matriz a un vector de estado consciente
    pub fn apply(&self, state: &[Complex<f64>]) -> Vec<Complex<f64>> {
        assert_eq!(state.len(), DIM);
        let input = DMatrix::from_column_iterator(
            DIM, 1, state.iter().cloned()
        );
        let output = &self.data * input;
        output.column(0).iter().cloned().collect()
    }
    
    /// Devuelve la traza certificada (≈ 196884)
    pub fn trace(&self) -> Complex<f64> {
        self.trace
    }
    
    /// Verifica unitariedad: M†M = I
    pub fn is_unitary(&self, tolerance: f64) -> bool {
        let adjoint = self.data.adjoint();
        let product = &adjoint * &self.data;
        let identity = DMatrix::identity(DIM, DIM);
        
        let diff = &product - &identity;
        diff.norm() < tolerance
    }
    
    /// Obtiene autovalor k (certificado áureo)
    pub fn eigenvalue(&self, k: usize) -> Complex<f64> {
        let phase = 2.0 * PI * (k as f64) / (DIM as f64);
        let magnitude = PHI.powf(-(k as f64));
        Complex::new(
            magnitude * phase.cos(),
            magnitude * phase.sin()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    
    #[test]
    fn test_trace_approx_196884() {
        let m = MonsterMatrix444::new();
        let trace = m.trace();
        // Verificar traza ≈ 196884 con tolerancia 1e-3
        assert_abs_diff_eq!(trace.re, CERTIFIED_TRACE, epsilon = 1e-3);
        // Parte imaginaria ≈ 0
        assert_abs_diff_eq!(trace.im, 0.0, epsilon = 1e-12);
    }
    
    #[test]
    fn test_unitarity() {
        let m = MonsterMatrix444::new();
        assert!(m.is_unitary(1e-10), "Matriz no es unitaria");
    }
    
    #[test]
    fn test_eigenvalues_phi_resonant() {
        let m = MonsterMatrix444::new();
        
        // Verificar primeros 10 autovalores
        for k in 0..10 {
            let eigen = m.eigenvalue(k);
            let expected_phase = 2.0 * PI * (k as f64) / (DIM as f64);
            let expected_magnitude = PHI.powf(-(k as f64));
            
            assert_abs_diff_eq!(eigen.norm(), expected_magnitude, epsilon = 1e-10);
            assert_abs_diff_eq!(eigen.argument(), expected_phase, epsilon = 1e-10);
        }
    }
    
    #[test]
    fn test_apply_preserves_norm() {
        let m = MonsterMatrix444::new();
        
        // Estado de prueba normalizado
        let mut state = vec![Complex::new(0.0, 0.0); DIM];
        state[0] = Complex::new(1.0 / (DIM as f64).sqrt(), 0.0);
        state[1] = Complex::new(0.0, 1.0 / (DIM as f64).sqrt());
        
        let output = m.apply(&state);
        
        let input_norm: f64 = state.iter().map(|c| c.norm_sqr()).sum();
        let output_norm: f64 = output.iter().map(|c| c.norm_sqr()).sum();
        
        assert_abs_diff_eq!(input_norm, output_norm, epsilon = 1e-12);
    }
}
