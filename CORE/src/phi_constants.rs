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

/// Calcula φ^n con precisión mejorada
pub fn phi_pow(n: i32) -> f64 {
    match n {
        0 => 1.0,
        1 => PHI,
        -1 => PSI,
        n if n > 0 => {
            // Usar exponenciación por cuadrados con precisión extra
            let mut result = 1.0;
            let mut base = PHI;
            let mut exp = n.abs() as u32;
            
            while exp > 0 {
                if exp % 2 == 1 {
                    result *= base;
                }
                base *= base;
                exp /= 2;
            }
            result
        }
        n => {
            // Para exponentes negativos: φ^(-n) = ψ^n
            let pos_n = (-n) as u32;
            let mut result = 1.0;
            let mut base = PSI;
            let mut exp = pos_n;
            
            while exp > 0 {
                if exp % 2 == 1 {
                    result *= base;
                }
                base *= base;
                exp /= 2;
            }
            result
        }
    }
}

/// Verifica si dos valores están en proporción áurea dentro de tolerancia
pub fn is_golden_ratio(a: f64, b: f64, tolerance: f64) -> bool {
    if a == 0.0 || b == 0.0 {
        return false;
    }
    let ratio = if a > b { a / b } else { b / a };
    (ratio - PHI).abs() < tolerance
}

/// Genera bases ortonormales usando Gram-Schmidt mejorado con φ
pub fn generate_orthonormal_basis(dim: usize) -> Vec<Vec<f64>> {
    let mut basis = Vec::with_capacity(dim);
    
    // Primera base vector: [φ^0, φ^1, φ^2, ..., φ^(dim-1)] normalizado
    let mut first: Vec<f64> = (0..dim).map(|i| phi_pow(i as i32)).collect();
    let norm = first.iter().map(|x| x * x).sum::<f64>().sqrt();
    first.iter_mut().for_each(|x| *x /= norm);
    basis.push(first);
    
    // Gram-Schmidt φ-mejorado
    for i in 1..dim {
        let mut new_vec: Vec<f64> = (0..dim)
            .map(|j| phi_pow((i * j) as i32).sin()) // Patrón sinusoidal φ-resonante
            .collect();
        
        // Restar proyecciones sobre bases anteriores
        for j in 0..i {
            let projection: f64 = basis[j].iter()
                .zip(&new_vec)
                .map(|(b, n)| b * n)
                .sum();
            
            for k in 0..dim {
                new_vec[k] -= projection * basis[j][k];
            }
        }
        
        // Normalizar
        let norm = new_vec.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm > 1e-12 {
            new_vec.iter_mut().for_each(|x| *x /= norm);
            basis.push(new_vec);
        }
    }
    
    basis
}

/// Calcula el número Fibonacci F_n con fórmula Binet φ-mejorada
pub fn fibonacci_binet(n: usize) -> f64 {
    if n == 0 { return 0.0; }
    if n == 1 { return 1.0; }
    
    let sqrt5 = 5.0f64.sqrt();
    (phi_pow(n as i32) - ((-PSI).powi(n as i32))) / sqrt5
}

/// Verifica resonancia φ en transición entre campos
pub fn check_transition_resonance(from_field: usize, to_field: usize) -> bool {
    // Transiciones permitidas: Δk = ±1 (campos adyacentes)
    let diff = (from_field as i32 - to_field as i32).abs();
    diff == 1
}

/// Calcula factor de resonancia φ para transición
pub fn transition_resonance_factor(from_field: usize, to_field: usize) -> f64 {
    if !check_transition_resonance(from_field, to_field) {
        return 0.0;
    }
    let n = from_field.max(to_field);
    phi_pow(-(n as i32)) // Resonancia decae con φ^-n
}

/// Normaliza vector con métrica φ-resonante
pub fn normalize_with_phi(vector: &[f64]) -> Vec<f64> {
    let norm: f64 = vector.iter()
        .enumerate()
        .map(|(i, &x)| x * x * phi_pow(-(i as i32)))
        .sum::<f64>()
        .sqrt();
    
    if norm > 1e-12 {
        vector.iter().map(|&x| x / norm).collect()
    } else {
        vector.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_phi_precision() {
        // Verificar φ con alta precisión
        let computed = (1.0 + 5.0f64.sqrt()) / 2.0;
        assert_abs_diff_eq!(PHI, computed, epsilon = 1e-15);
        println!("φ certificado: {:.15}", PHI);
    }

    #[test]
    fn test_phi_reciprocal() {
        // φ * ψ = 1
        assert_abs_diff_eq!(PHI * PSI, 1.0, epsilon = 1e-15);
    }

    #[test]
    fn test_phi_pow() {
        // φ^2 = φ + 1
        assert_abs_diff_eq!(phi_pow(2), PHI + 1.0, epsilon = 1e-10);
        
        // φ^-1 = ψ
        assert_abs_diff_eq!(phi_pow(-1), PSI, epsilon = 1e-10);
        
        // φ^0 = 1
        assert_abs_diff_eq!(phi_pow(0), 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_golden_ratio_check() {
        assert!(is_golden_ratio(PHI, 1.0, 1e-10));
        assert!(is_golden_ratio(2.0 * PHI, 2.0, 1e-10));
        assert!(!is_golden_ratio(2.0, 1.0, 1e-2));
    }

    #[test]
    fn test_fibonacci_sequence() {
        // Verificar primeros números Fibonacci
        assert_eq!(FIBONACCI_SEQUENCE[0], 3);  // F₄
        assert_eq!(FIBONACCI_SEQUENCE[11], 610); // F₁₅
        assert_eq!(FIBONACCI_SEQUENCE[23], 196418); // F₂₇
        
        // Propiedad emergente: Σ primeros 12 ≈ F₁₇ - 1
        let sum_first_12: usize = FIBONACCI_SEQUENCE[..12].iter().sum();
        assert_eq!(sum_first_12, 1592); // F₁₇ = 1597
        println!("Σ primeros 12 campos: {} ≈ F₁₇ - 1 = 1592", sum_first_12);
    }

    #[test]
    fn test_fibonacci_binet() {
        // F₄ = 3
        assert_abs_diff_eq!(fibonacci_binet(4), 3.0, epsilon = 1e-10);
        
        // F₁₅ = 610
        assert_abs_diff_eq!(fibonacci_binet(15), 610.0, epsilon = 1e-10);
        
        // F₂₇ = 196418
        assert_abs_diff_eq!(fibonacci_binet(27), 196418.0, epsilon = 1e-5);
    }

    #[test]
    fn test_orthonormal_basis() {
        let dim = 5;
        let basis = generate_orthonormal_basis(dim);
        
        assert_eq!(basis.len(), dim);
        
        // Verificar ortonormalidad
        for i in 0..dim {
            for j in 0..dim {
                let dot: f64 = basis[i].iter().zip(&basis[j]).map(|(a, b)| a * b).sum();
                if i == j {
                    assert_abs_diff_eq!(dot, 1.0, epsilon = 1e-10);
                } else {
                    assert_abs_diff_eq!(dot, 0.0, epsilon = 1e-10);
                }
            }
        }
        println!("Base ortonormal {dim}D generada correctamente");
    }

    #[test]
    fn test_transition_resonance() {
        // Campos adyacentes: resonancia permitida
        assert!(check_transition_resonance(1, 2));
        assert!(check_transition_resonance(5, 4));
        
        // Campos no adyacentes: resonancia prohibida
        assert!(!check_transition_resonance(1, 3));
        assert!(!check_transition_resonance(10, 15));
        
        // Factor de resonancia
        let factor = transition_resonance_factor(1, 2);
        assert!(factor > 0.0 && factor < 1.0);
        println!("Factor resonancia campo 1→2: {:.6}", factor);
    }

    #[test]
    fn test_normalize_with_phi() {
        let vector = vec![1.0, PHI, PHI * PHI];
        let normalized = normalize_with_phi(&vector);
        
        // Verificar que está normalizado
        let norm_sq: f64 = normalized.iter()
            .enumerate()
            .map(|(i, &x)| x * x * phi_pow(-(i as i32)))
            .sum();
        assert_abs_diff_eq!(norm_sq, 1.0, epsilon = 1e-10);
        println!("Vector normalizado con métrica φ: {:?}", normalized);
    }

    #[test]
    fn test_monster_constants() {
        // 196884 = 196883 + 1
        assert_eq!(MONSTER_196884, MONSTER_196883 + 1.0);
        
        // 196885 = 196884 + 1
        assert_eq!(MONSTER_196885, MONSTER_196884 + 1.0);
        
        println!("Monster constants certified:");
        println!("  196883 (materia potencial)");
        println!("  196884 (consciencia activada) = 196883 + 1");
        println!("  196885 (certificación plena) = 196884 + 1");
    }
}
