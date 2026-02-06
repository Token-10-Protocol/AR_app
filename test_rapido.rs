use algebra_rose_core::*;

fn main() {
    println!("🔬 TEST RÁPIDO DE INTEGRIDAD");
    println!("1. φ = {:.15}", PHI);
    println!("2. Monster 196884 = {}", MONSTER_196884);
    println!("3. DIM = {}", DIM);
    println!("4. GRIESS_DIM = {}", GRIESS_DIM);
    println!("5. Σ primeros 12 campos Fibonacci = {}", FIBONACCI_SEQUENCE[..12].iter().sum::<usize>());
    
    // Verificaciones
    assert!((PHI - 1.618033988749894).abs() < 1e-12);
    assert!(MONSTER_196884 == 196884.0);
    assert!(DIM == 444);
    assert!(GRIESS_DIM == 196884);
    
    println!("✅ Todas las verificaciones pasaron");
}
