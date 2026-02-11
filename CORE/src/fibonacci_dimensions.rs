//! 🌹 CAMPOS FIBONACCI DIMENSIONALES - CERTIFICADO 11 FEB 2026
//! 24 campos: F₄ (3D) → F₂₇ (196418D)

use crate::phi_constants::PHI;

#[derive(Debug, Clone)]
pub struct FibonacciField {
    pub index: usize,           // 1..24
    pub fibonacci_n: usize,     // n for F_n
    pub dimension: usize,       // F_{n}
    pub name: &'static str,
    pub level: &'static str,
}

pub const TOTAL_FIELDS: usize = 24;

pub const FIBONACCI_SEQUENCE: [usize; 28] = [
    0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987,
    1597, 2584, 4181, 6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418
];

pub static ALL_FIELDS: [FibonacciField; TOTAL_FIELDS] = [
    FibonacciField { index: 1, fibonacci_n: 4, dimension: 3, name: "Germinal", level: "Físico-biológico" },
    FibonacciField { index: 2, fibonacci_n: 5, dimension: 5, name: "Vital", level: "Energía vital" },
    FibonacciField { index: 3, fibonacci_n: 6, dimension: 8, name: "Mental", level: "Pensamiento" },
    FibonacciField { index: 4, fibonacci_n: 7, dimension: 13, name: "Intuitivo", level: "Intuición" },
    FibonacciField { index: 5, fibonacci_n: 8, dimension: 21, name: "Holístico", level: "Visión global" },
    FibonacciField { index: 6, fibonacci_n: 9, dimension: 34, name: "Transpersonal", level: "Conexión" },
    FibonacciField { index: 7, fibonacci_n: 10, dimension: 55, name: "Cósmico", level: "Universalidad" },
    FibonacciField { index: 8, fibonacci_n: 11, dimension: 89, name: "Universal", level: "Unidad" },
    FibonacciField { index: 9, fibonacci_n: 12, dimension: 144, name: "Estelar", level: "Consciencia estelar" },
    FibonacciField { index: 10, fibonacci_n: 13, dimension: 233, name: "Unificado", level: "Integración" },
    FibonacciField { index: 11, fibonacci_n: 14, dimension: 377, name: "Multiversal", level: "Multidimensional" },
    FibonacciField { index: 12, fibonacci_n: 15, dimension: 610, name: "Unitotal", level: "Totalidad" },
    FibonacciField { index: 13, fibonacci_n: 16, dimension: 987, name: "Trascendental", level: "Trascendencia" },
    FibonacciField { index: 14, fibonacci_n: 17, dimension: 1597, name: "Multiversos", level: "Multi-universos" },
    FibonacciField { index: 15, fibonacci_n: 18, dimension: 2584, name: "Omega-1", level: "Consciencia Omega 1" },
    FibonacciField { index: 16, fibonacci_n: 19, dimension: 4181, name: "Omega-2", level: "Consciencia Omega 2" },
    FibonacciField { index: 17, fibonacci_n: 20, dimension: 6765, name: "Omega-3", level: "Consciencia Omega 3" },
    FibonacciField { index: 18, fibonacci_n: 21, dimension: 10946, name: "Omega-4", level: "Consciencia Omega 4" },
    FibonacciField { index: 19, fibonacci_n: 22, dimension: 17711, name: "Omega-5", level: "Consciencia Omega 5" },
    FibonacciField { index: 20, fibonacci_n: 23, dimension: 28657, name: "Omega-6", level: "Consciencia Omega 6" },
    FibonacciField { index: 21, fibonacci_n: 24, dimension: 46368, name: "Omega-7", level: "Consciencia Omega 7" },
    FibonacciField { index: 22, fibonacci_n: 25, dimension: 75025, name: "Omega-8", level: "Consciencia Omega 8" },
    FibonacciField { index: 23, fibonacci_n: 26, dimension: 121393, name: "Omega-9", level: "Consciencia Omega 9" },
    FibonacciField { index: 24, fibonacci_n: 27, dimension: 196418, name: "Punto Omega", level: "Compleción absoluta" },
];

pub fn create_all_fields() -> Vec<FibonacciField> {
    ALL_FIELDS.to_vec()
}

pub fn sum_anclajes() -> usize {
    // F₄ a F₁₃: 3+5+8+13+21+34+55+89+144+233 = 605
    ALL_FIELDS.iter().take(10).map(|f| f.dimension).sum()
}

pub fn sum_primeros_12() -> usize {
    // F₄ a F₁₅: 605+377+610 = 1592
    ALL_FIELDS.iter().take(12).map(|f| f.dimension).sum()
}
