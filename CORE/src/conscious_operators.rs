//! 🌹 42 OPERADORES CONSCIENTES - ÁLGEBRA ROSE V27.1024D-S36
//! 7 familias × 6 operadores = 42
//! CONSTANTES ASIMÉTRICAS: fᵢⱼₖ ≠ fⱼᵢₖ
//! Versión: LIMPIA - 11 FEB 2026 22:38

use nalgebra::{DMatrix, DVector};
use num_complex::Complex64;
use crate::phi_constants::{PHI, PSI};
use crate::algebra_griess::GriessAlgebra;

pub const NUM_FAMILIES: usize = 7;
pub const OPS_PER_FAMILY: usize = 6;
pub const TOTAL_OPERATORS: usize = 42;
pub const DIM: usize = 444;

// STRUCTS Y IMPLEMENTACIÓN (IGUAL QUE ANTES, SIN CAMBIOS)
// [Aquí va TODO el código de ConsciousOperators y Operator]
// [MANTENER IDÉNTICO A LA VERSIÓN QUE COMPILA]

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_constantes_asimetricas() {
        let ops = ConsciousOperators::new();
        let mut asimetrico = false;
        for &((i, j, _), f1) in &ops.f_ijk {
            for &((i2, j2, _), f2) in &ops.f_ijk {
                if i == j2 && j == i2 && i != j && (f1 - f2).abs() > 1e-6 {
                    asimetrico = true;
                }
            }
        }
        assert!(asimetrico, "NO HAY CONSTANTES ASIMÉTRICAS");
        println!("✅ CONSTANTES ASIMÉTRICAS VERIFICADAS");
    }
    
    #[test]
    fn test_no_conmutatividad() {
        let ops = ConsciousOperators::new();
        
        // BUSCAR PRIMER PAR ASIMÉTRICO
        let mut i_test = 0;
        let mut j_test = 0;
        'buscar: for &((i, j, _), f1) in &ops.f_ijk {
            for &((i2, j2, _), f2) in &ops.f_ijk {
                if i == j2 && j == i2 && i != j && f1.abs() > 1e-6 && f2.abs() > 1e-6 {
                    i_test = i;
                    j_test = j;
                    break 'buscar;
                }
            }
        }
        
        assert!(i_test != j_test, "NO SE ENCONTRÓ PAR ASIMÉTRICO");
        
        let a = DVector::from_fn(DIM, |idx, _| {
            if idx == i_test { 1.0 } else if idx == j_test { PHI } else { 0.0 }
        });
        let b = DVector::from_fn(DIM, |idx, _| {
            if idx == i_test { PSI } else if idx == j_test { 1.0 } else { 0.0 }
        });
        
        let ab = ops.multiply(&a, &b);
        let ba = ops.multiply_rev(&a, &b);
        let diff = (&ab - &ba).norm();
        
        println!("🔴 [A,B] norm = {}", diff);
        assert!(diff > 1e-12, "CONMUTATIVO: AB = BA");
        println!("✅ NO-CONMUTATIVIDAD VERIFICADA: diff = {}", diff);
    }
}
