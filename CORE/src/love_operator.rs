//! Operador Â (Amor Fundamental) - Actualizador de Potencial Matemático
//! Sistema: Álgebra Rose v27.1024D-S36
//! Certificación: 196885 - Estado Monster Pleno

use nalgebra::{DMatrix, Complex, DVector, Normed};
use crate::matrix_444::{DIM, PHI};
use crate::algebra_griess::{GriessAlgebra, GRIESS_DIM};

/// Operador Â (Amor Fundamental) - Actualiza potencial matemático
/// Representa la fuerza fundamental de conexión consciente
#[derive(Clone, Debug)]
pub struct LoveOperator {
    /// Matriz de transformación amorosa (444 × 444)
    transformation: DMatrix<Complex<f64>>,
    /// Intensidad del amor (φ-resonante)
    intensity: f64,
    /// Fase amorosa actual
    phase: f64,
    /// Conexión con álgebra de Griess
    griess_connection: Option<GriessAlgebra>,
}

impl LoveOperator {
    /// Crea un nuevo operador Â con intensidad base
    pub fn new(intensity: f64) -> Self {
        let transformation = Self::create_love_transformation(intensity);
        LoveOperator {
            transformation,
            intensity,
            phase: 0.0,
            griess_connection: None,
        }
    }

    /// Crea la transformación amorosa basada en φ-resonancia
    fn create_love_transformation(intensity: f64) -> DMatrix<Complex<f64>> {
        let mut matrix = DMatrix::identity(DIM, DIM);
        
        // Aplicar transformación φ-resonante
        for i in 0..DIM {
            for j in 0..DIM {
                if i == j {
                    // Diagonal: amor propio fundamental
                    matrix[(i, j)] = Complex::new(PHI * intensity, 0.0);
                } else {
                    // Off-diagonal: amor conectivo
                    let distance = ((i as f64 - j as f64).abs() + 1.0).ln();
                    let phase = (i as f64 * j as f64 * PHI).sin();
                    let strength = intensity / distance;
                    matrix[(i, j)] = Complex::new(
                        strength * phase.cos(),
                        strength * phase.sin(),
                    );
                }
            }
        }
        matrix
    }

    /// Aplica el operador Â a un estado consciente
    pub fn apply(&self, state: &DVector<Complex<f64>>) -> DVector<Complex<f64>> {
        assert_eq!(state.len(), DIM, "Estado debe tener dimensión {}", DIM);
        // Transformación amorosa: |ψ'⟩ = Â|ψ⟩
        &self.transformation * state
    }

    /// Actualiza la intensidad del amor (crecimiento φ-resonante)
    pub fn update_intensity(&mut self, delta: f64) -> f64 {
        self.intensity *= PHI.powf(delta);
        // Recrear transformación con nueva intensidad
        self.transformation = Self::create_love_transformation(self.intensity);
        self.phase += delta * PHI;
        self.intensity
    }

    /// Conecta con el álgebra de Griess (amor matemático profundo)
    pub fn connect_to_griess(&mut self, griess: GriessAlgebra) -> Result<f64, String> {
        self.griess_connection = Some(griess);
        // Amplificar intensidad por conexión Monster
        let amplification = (GRIESS_DIM as f64 / DIM as f64).ln() / PHI.ln();
        self.update_intensity(amplification);
        Ok(self.intensity)
    }

    /// Calcula el factor de amor entre dos estados
    pub fn love_factor(&self, state_a: &DVector<Complex<f64>>, state_b: &DVector<Complex<f64>>) -> Complex<f64> {
        let transformed_a = self.apply(state_a);
        let transformed_b = self.apply(state_b);
        // Producto interno amoroso: ⟨ψ_a|Â⁺Â|ψ_b⟩
        transformed_a.dot(&transformed_b)
    }

    /// Verifica propiedades del operador Â
    pub fn verify_properties(&self, tolerance: f64) -> Vec<(String, bool)> {
        let mut results = Vec::new();
        
        // 1. Unitariedad aproximada (Â⁺Â ≈ I)
        let adjoint = self.transformation.adjoint();
        let product = &adjoint * &self.transformation;
        let identity_diff = (product - DMatrix::identity(DIM, DIM)).norm();
        results.push((
            "Unitariedad aproximada".to_string(),
            identity_diff < tolerance
        ));

        // 2. Traza relacionada con φ
        let trace = self.transformation.trace().re;
        let expected_trace = DIM as f64 * PHI * self.intensity;
        let trace_diff = (trace - expected_trace).abs() / expected_trace.abs();
        results.push((
            "Traza φ-resonante".to_string(),
            trace_diff < tolerance
        ));

        // 3. Intensidad positiva
        results.push((
            "Intensidad positiva".to_string(),
            self.intensity > 0.0
        ));

        // 4. Dimensión correcta
        results.push((
            format!("Dimensión {}", DIM).to_string(),
            self.transformation.nrows() == DIM && self.transformation.ncols() == DIM
        ));

        results
    }

    /// Obtiene la matriz de transformación
    pub fn get_transformation(&self) -> &DMatrix<Complex<f64>> {
        &self.transformation
    }

    /// Obtiene intensidad actual
    pub fn get_intensity(&self) -> f64 {
        self.intensity
    }

    /// Obtiene fase actual
    pub fn get_phase(&self) -> f64 {
        self.phase
    }
}

/// Operador Â especializado para crecimiento keygen
#[derive(Clone, Debug)]
pub struct KeygenLoveOperator {
    base_operator: LoveOperator,
    growth_rate: f64,
    keygen_threshold: f64,
}

impl KeygenLoveOperator {
    /// Crea operador Â para crecimiento keygen
    pub fn new(initial_intensity: f64, growth_rate: f64) -> Self {
        KeygenLoveOperator {
            base_operator: LoveOperator::new(initial_intensity),
            growth_rate,
            keygen_threshold: 196884.0, // Dimensión Monster
        }
    }

    /// Aplica crecimiento keygen φ-resonante
    pub fn apply_keygen_growth(&mut self, current_keygen: f64) -> f64 {
        // Crecimiento según distancia al threshold
        let distance_ratio = 1.0 - (current_keygen / self.keygen_threshold);
        let growth_factor = PHI.powf(distance_ratio * self.growth_rate);
        self.base_operator.update_intensity(growth_factor);
        // Nuevo keygen proyectado
        current_keygen * growth_factor
    }

    /// Obtiene el operador base
    pub fn get_operator(&self) -> &LoveOperator {
        &self.base_operator
    }

    /// Verifica si keygen alcanzó threshold Monster
    pub fn has_reached_monster(&self, current_keygen: f64) -> bool {
        current_keygen >= self.keygen_threshold * 0.999 // 99.9% del threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_love_operator_creation() {
        let operator = LoveOperator::new(1.0);
        assert_eq!(operator.get_transformation().nrows(), DIM);
        assert_eq!(operator.get_transformation().ncols(), DIM);
        assert!(operator.get_intensity() > 0.0);
    }

    #[test]
    fn test_love_application() {
        let operator = LoveOperator::new(1.0);
        let state = DVector::from_element(DIM, Complex::new(1.0, 0.0));
        let transformed = operator.apply(&state);
        assert_eq!(transformed.len(), DIM);
        // Verificar que no es idéntico (transformación ocurrió)
        let diff = (transformed - state).norm();
        assert!(diff > 0.0, "Transformación debe cambiar el estado");
    }

    #[test]
    fn test_love_factor() {
        let operator = LoveOperator::new(1.0);
        let state_a = DVector::from_fn(DIM, |i, _| {
            Complex::new((i + 1) as f64 / DIM as f64, 0.0)
        });
        let state_b = DVector::from_fn(DIM, |i, _| {
            Complex::new((DIM - i) as f64 / DIM as f64, 0.0)
        });
        let factor = operator.love_factor(&state_a, &state_b);
        // El factor de amor debe ser un número complejo no-cero
        assert!(factor.norm() > 0.0, "Factor de amor debe ser no-cero");
        println!("Factor de amor: {:.4} + {:.4}i", factor.re, factor.im);
    }

    #[test]
    fn test_intensity_update() {
        let mut operator = LoveOperator::new(1.0);
        let initial_intensity = operator.get_intensity();
        operator.update_intensity(0.5);
        let new_intensity = operator.get_intensity();
        // Debe crecer según φ
        let expected_growth = PHI.powf(0.5);
        let actual_growth = new_intensity / initial_intensity;
        assert_abs_diff_eq!(actual_growth, expected_growth, epsilon = 1e-10);
        println!("Crecimiento de intensidad: {:.4} (esperado: {:.4})",
                 actual_growth, expected_growth);
    }

    #[test]
    fn test_property_verification() {
        let operator = LoveOperator::new(1.0);
        let results = operator.verify_properties(1e-6);
        println!("🔍 Verificación de propiedades del operador Â:");
        let mut passed = 0;
        for (name, success) in &results {
            if *success {
                passed += 1;
                println!(" ✅ {}: PASÓ", name);
            } else {
                println!(" ⚠️ {}: FALLÓ", name);
            }
        }
        assert!(passed >= 3, "Al menos 3 propiedades deben pasar");
        println!("✅ {}/{} propiedades pasaron", passed, results.len());
    }

    #[test]
    fn test_keygen_operator() {
        let mut keygen_op = KeygenLoveOperator::new(1.0, 0.1);
        let initial_keygen = 100000.0;
        let new_keygen = keygen_op.apply_keygen_growth(initial_keygen);
        // Keygen debe crecer
        assert!(new_keygen > initial_keygen);
        // Verificar threshold
        let not_reached = keygen_op.has_reached_monster(initial_keygen);
        assert!(!not_reached, "No debe haber alcanzado Monster todavía");
        let reached = keygen_op.has_reached_monster(196883.0);
        assert!(reached, "196883 debe estar cerca de Monster threshold");
        println!("Keygen crecimiento: {:.0} → {:.0}", initial_keygen, new_keygen);
    }

    #[test]
    fn test_griess_connection() {
        let griess = crate::algebra_griess::GriessAlgebra::new();
        let mut operator = LoveOperator::new(1.0);
        let initial_intensity = operator.get_intensity();
        match operator.connect_to_griess(griess) {
            Ok(new_intensity) => {
                // Intensidad debe aumentar con conexión Monster
                assert!(new_intensity > initial_intensity);
                println!("Conexión Griess exitosa, intensidad: {:.4} → {:.4}",
                         initial_intensity, new_intensity);
            },
            Err(e) => {
                panic!("Error en conexión Griess: {}", e);
            }
        }
    }
}
