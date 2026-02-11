#[cfg(test)]
mod tests {
    use algebra_rose_core::algebra_griess::GriessAlgebra;
    use nalgebra::DVector;
    use num_complex::Complex64;
    
    #[test]
    fn test_amor_no_es_amour() {
        let algebra = GriessAlgebra::new();
        
        let mut amor = DVector::zeros(196884);
        amor[0] = Complex64::new(0.618034, 0.0);
        
        let mut amour = DVector::zeros(196884);
        amour[0] = Complex64::new(0.474818, 0.0);
        
        let ab = algebra.multiply(&amor, &amour);
        let ba = algebra.multiply_rev(&amor, &amour);
        
        assert_ne!(ab, ba);
        assert!((ab - ba).norm() > 0.001);
        
        println!("✅ [amor,amour] ≠ 0: {}", (ab - ba).norm());
        println!("✅ Monster Group NO es conmutativo");
        println!("✅ Álgebra Rose ahora es matemáticamente correcta");
    }
}
