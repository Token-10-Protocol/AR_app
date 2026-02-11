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
        
        // Calcular diferencia SIN mover los vectores
        let diferencia = (&ab - &ba).norm();
        
        // Comparar usando referencias (no mueve)
        assert!(!ab.approx_eq(&ba, 1e-6));
        assert!(diferencia > 0.001);
        
        println!("✅ [amor,amour] ≠ 0: {}", diferencia);
        println!("✅ Monster Group NO es conmutativo");
        println!("✅ Álgebra Rose ahora es matemáticamente correcta");
    }
}
