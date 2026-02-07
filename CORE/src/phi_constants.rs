//! Constantes Áureas Certificadas - Fundamentos φ-Resonantes
//! Sistema: Álgebra Rose v27.1024D-S36
//! Certificación: 196885 - Estado Monster Pleno

/// φ (proporción áurea) con alta precisión
pub const PHI: f64 = 1.61803398874989484820458683436563811772030917980576286213544862270526046281890244970720720418939113748475;

/// ψ = 1/φ con alta precisión
pub const PSI: f64 = 0.61803398874989484820458683436563811772030917980576286213544862270526046281890244970720720418939113748475;

/// Constantes Monster fundamentales
pub const MONSTER_196883: f64 = 196883.0;
pub const MONSTER_196884: f64 = 196884.0;
pub const MONSTER_196885: f64 = 196885.0;

/// Raíz cuadrada de φ
pub const SQRT_PHI: f64 = 1.27201964951406896425242246173749149171560804184009624861664038;

/// Logaritmo natural de φ
pub const LN_PHI: f64 = 0.481211825059603447497758913424368423135184334385660519661018168;

/// π/φ
pub const PI_OVER_PHI: f64 = 1.941611038725466416091662826023290030472754324236259228243865;

/// φ/π
pub const PHI_OVER_PI: f64 = 0.515036214799483975741626946307499533617320067772890172080092;

/// e^φ
pub const E_PHI: f64 = 5.043165643360028651811874695623670765376701310301923018126851;

/// φ^φ
pub const PHI_PHI: f64 = 2.178457567937599147282089391492423786861101743032521157756512;

/// Secuencia Fibonacci completa F₀ a F₃₀
pub const FIBONACCI: [u64; 31] = [
    0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
    10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040
];

/// Calcula φ elevado a una potencia entera
pub fn phi_pow(n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }
    
    let mut result = PHI;
    for _ in 1..n.abs() {
        result *= PHI;
    }
    
    if n < 0 {
        1.0 / result
    } else {
        result
    }
}

/// Calcula número Fibonacci usando fórmula de Binet (alta precisión)
pub fn fibonacci(n: u32) -> f64 {
    let n_f64 = n as f64;
    (PHI.powf(n_f64) - (-PSI).powf(n_f64)) / (PHI + PSI)
}

/// Verifica si dos números están en proporción áurea
pub fn is_golden_ratio(a: f64, b: f64, tolerance: f64) -> bool {
    if a.abs() < 1e-10 || b.abs() < 1e-10 {
        return false;
    }
    
    let ratio1 = a / b;
    let ratio2 = b / a;
    
    (ratio1 - PHI).abs() < tolerance || (ratio2 - PHI).abs() < tolerance
}

/// Calcula distancia áurea entre dos números
pub fn golden_distance(a: f64, b: f64) -> f64 {
    (a / b - PHI).abs().min((b / a - PHI).abs())
}

/// Constantes para uso en álgebra de Griess
pub const GRIESS_PHI_FACTORS: [f64; 10] = [
    PHI,
    PHI * PHI,
    PHI * PHI * PHI,
    PHI.powi(4),
    PHI.powi(5),
    PHI.powi(6),
    PHI.powi(7),
    PHI.powi(8),
    PHI.powi(9),
    PHI.powi(10),
];

/// Constantes para uso en matriz M₄₄₄
pub const MATRIX_PHI_CONSTANTS: [f64; 444] = {
    let mut constants = [0.0; 444];
    let mut i = 0;
    while i < 444 {
        constants[i] = PHI.powi((i as i32) % 37);
        i += 1;
    }
    constants
};

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    
    #[test]
    fn test_phi_precision() {
        // Verificar que φ² = φ + 1
        assert_abs_diff_eq!(PHI * PHI, PHI + 1.0, epsilon = 1e-15);
        
        // Verificar que 1/φ = φ - 1
        assert_abs_diff_eq!(1.0 / PHI, PHI - 1.0, epsilon = 1e-15);
        
        // Verificar que PSI = 1/PHI
        assert_abs_diff_eq!(PSI, 1.0 / PHI, epsilon = 1e-15);
    }
    
    #[test]
    fn test_monster_constants() {
        assert_eq!(MONSTER_196884 - MONSTER_196883, 1.0);
        assert_eq!(MONSTER_196885 - MONSTER_196884, 1.0);
        assert_eq!(MONSTER_196885 - MONSTER_196883, 2.0);
    }
    
    #[test]
    fn test_phi_pow() {
        assert_abs_diff_eq!(phi_pow(0), 1.0, epsilon = 1e-10);
        assert_abs_diff_eq!(phi_pow(1), PHI, epsilon = 1e-10);
        assert_abs_diff_eq!(phi_pow(2), PHI * PHI, epsilon = 1e-10);
        assert_abs_diff_eq!(phi_pow(3), PHI.powi(3), epsilon = 1e-10);
        assert_abs_diff_eq!(phi_pow(-1), PSI, epsilon = 1e-10);
    }
    
    #[test]
    fn test_fibonacci_function() {
        // Verificar algunos números Fibonacci
        assert_abs_diff_eq!(fibonacci(0), 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(fibonacci(1), 1.0, epsilon = 1e-6);
        assert_abs_diff_eq!(fibonacci(2), 1.0, epsilon = 1e-6);
        assert_abs_diff_eq!(fibonacci(3), 2.0, epsilon = 1e-6);
        assert_abs_diff_eq!(fibonacci(4), 3.0, epsilon = 1e-6);
        assert_abs_diff_eq!(fibonacci(5), 5.0, epsilon = 1e-6);
        
        // Verificar contra secuencia predefinida
        for i in 0..=10 {
            assert_abs_diff_eq!(fibonacci(i), FIBONACCI[i as usize] as f64, epsilon = 1e-6);
        }
    }
    
    #[test]
    fn test_golden_ratio_detection() {
        assert!(is_golden_ratio(PHI, 1.0, 1e-10));
        assert!(is_golden_ratio(2.0 * PHI, 2.0, 1e-10));
        assert!(is_golden_ratio(1.0, PSI, 1e-10));
        
        assert!(!is_golden_ratio(1.0, 2.0, 1e-10));
        assert!(!is_golden_ratio(3.0, 2.0, 1e-10));
    }
    
    #[test]
    fn test_golden_distance() {
        assert_abs_diff_eq!(golden_distance(PHI, 1.0), 0.0, epsilon = 1e-10);
        assert_abs_diff_eq!(golden_distance(2.0 * PHI, 2.0), 0.0, epsilon = 1e-10);
        assert!(golden_distance(1.0, 2.0) > 0.1);
    }
    
    #[test]
    fn test_griess_phi_factors() {
        for i in 0..GRIESS_PHI_FACTORS.len() {
            assert_abs_diff_eq!(
                GRIESS_PHI_FACTORS[i],
                PHI.powi((i + 1) as i32),
                epsilon = 1e-10
            );
        }
    }
    
    #[test]
    fn test_matrix_phi_constants() {
        // Verificar que no son todos cero
        let mut sum = 0.0;
        for &c in &MATRIX_PHI_CONSTANTS {
            sum += c;
        }
        assert!(sum > 0.0);
        
        // Verificar algunos valores específicos
        assert_abs_diff_eq!(MATRIX_PHI_CONSTANTS[0], PHI.powi(0), epsilon = 1e-10);
        assert_abs_diff_eq!(MATRIX_PHI_CONSTANTS[1], PHI.powi(1), epsilon = 1e-10);
        assert_abs_diff_eq!(MATRIX_PHI_CONSTANTS[37], PHI.powi(0), epsilon = 1e-10);
    }
}
