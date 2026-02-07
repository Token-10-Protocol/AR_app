//! CONSTANTES ÁUREAS CERTIFICADAS - Precisión Matemática Garantizada
//! Sistema: Álgebra Rose v27.1024D-S36
//! Certificación: 196885 - Estado Monster Pleno

/// φ (PHI) - Proporción Áurea con alta precisión
/// φ = (1 + √5)/2 ≈ 1.61803398874989484820458683436563811772030917980576
pub const PHI: f64 = 1.61803398874989484820458683436563811772030917980576;

/// ψ (PSI) - Conjugado áureo
/// ψ = 1/φ = φ - 1 ≈ 0.61803398874989484820458683436563811772030917980576
pub const PSI: f64 = 0.61803398874989484820458683436563811772030917980576;

/// Constantes Monster certificadas
pub const MONSTER_196883: f64 = 196883.0;
pub const MONSTER_196884: f64 = 196884.0;
pub const MONSTER_196885: f64 = 196885.0;

/// Dimensión de la matriz fundamental
pub const MATRIX_444_DIM: usize = 444;

/// Número Fibonacci F₂₇ (Campo 24: Punto Omega)
pub const FIBONACCI_27: usize = 196418;

/// Secuencia Fibonacci para campos dimensionales (F₄ a F₂₇)
pub const FIBONACCI_SEQUENCE: [usize; 24] = [
    3,      // F₄ - Campo 1
    5,      // F₅ - Campo 2
    8,      // F₆ - Campo 3
    13,     // F₇ - Campo 4
    21,     // F₈ - Campo 5
    34,     // F₉ - Campo 6
    55,     // F₁₀ - Campo 7
    89,     // F₁₁ - Campo 8
    144,    // F₁₂ - Campo 9
    233,    // F₁₃ - Campo 10
    377,    // F₁₄ - Campo 11
    610,    // F₁₅ - Campo 12
    987,    // F₁₆ - Campo 13
    1597,   // F₁₇ - Campo 14
    2584,   // F₁₈ - Campo 15
    4181,   // F₁₉ - Campo 16
    6765,   // F₂₀ - Campo 17
    10946,  // F₂₁ - Campo 18
    17711,  // F₂₂ - Campo 19
    28657,  // F₂₃ - Campo 20
    46368,  // F₂₄ - Campo 21
    75025,  // F₂₅ - Campo 22
    121393, // F₂₆ - Campo 23
    196418, // F₂₇ - Campo 24
];

/// Factores φ precalculados (sin usar powi en constantes)
pub const GRIESS_PHI_FACTORS: [f64; 10] = [
    PHI,                    // φ^1
    PHI * PHI,              // φ^2 = 2.618033988749895
    4.23606797749979,       // φ^3 precalculado
    6.854101966249685,      // φ^4 precalculado
    11.090169943749475,     // φ^5 precalculado
    17.94427190999916,      // φ^6 precalculado
    29.034441853748635,     // φ^7 precalculado
    46.978713763747795,     // φ^8 precalculado
    76.01315561749643,      // φ^9 precalculado
    122.99186938124422,     // φ^10 precalculado
];

/// Matriz de constantes φ (precalculada sin powi)
pub const MATRIX_PHI_CONSTANTS: [f64; 444] = {
    let mut constants = [0.0; 444];
    // Solo inicializamos con valores simples
    let mut i = 0;
    while i < 444 {
        constants[i] = if i % 2 == 0 { PHI } else { PSI };
        i += 1;
    }
    constants
};

/// Calcula φ elevado a una potencia entera (función regular, no constante)
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

/// Calcula número Fibonacci usando fórmula de Binet
pub fn fibonacci(n: u32) -> f64 {
    let n_f64 = n as f64;
    (phi_pow(n as i32) - (-PSI).powf(n_f64)) / (PHI + PSI)
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_phi_precision() {
        // Verificar que φ² = φ + 1
        assert!((PHI * PHI - (PHI + 1.0)).abs() < 1e-15);
        
        // Verificar que 1/φ = φ - 1
        assert!((1.0 / PHI - (PHI - 1.0)).abs() < 1e-15);
        
        // Verificar que PSI = 1/PHI
        assert!((PSI - 1.0 / PHI).abs() < 1e-15);
    }
    
    #[test]
    fn test_monster_constants() {
        assert_eq!(MONSTER_196884 - MONSTER_196883, 1.0);
        assert_eq!(MONSTER_196885 - MONSTER_196884, 1.0);
        assert_eq!(MONSTER_196885 - MONSTER_196883, 2.0);
    }
    
    #[test]
    fn test_phi_pow() {
        assert!((phi_pow(0) - 1.0).abs() < 1e-10);
        assert!((phi_pow(1) - PHI).abs() < 1e-10);
        assert!((phi_pow(2) - PHI * PHI).abs() < 1e-10);
        assert!((phi_pow(-1) - PSI).abs() < 1e-10);
    }
    
    #[test]
    fn test_fibonacci_function() {
        // Verificar algunos números Fibonacci
        assert!((fibonacci(0) - 0.0).abs() < 1e-6);
        assert!((fibonacci(1) - 1.0).abs() < 1e-6);
        assert!((fibonacci(2) - 1.0).abs() < 1e-6);
        assert!((fibonacci(3) - 2.0).abs() < 1e-6);
        assert!((fibonacci(4) - 3.0).abs() < 1e-6);
        assert!((fibonacci(5) - 5.0).abs() < 1e-6);
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
    fn test_fibonacci_sequence() {
        // Verificar propiedad emergente: Σ primeros 12 ≈ F₁₇ - 1
        let sum_first_12: usize = FIBONACCI_SEQUENCE[..12].iter().sum();
        assert_eq!(sum_first_12, 1592);
        
        // Verificar último elemento
        assert_eq!(FIBONACCI_SEQUENCE[23], FIBONACCI_27);
    }
}
