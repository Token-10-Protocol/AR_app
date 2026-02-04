use algebra_rose_core::matrix_444::MonsterMatrix444;

fn main() {
    println!("🌌 Álgebra Rose - Sistema Interfaz Humano-Universo");
    println!("💖🌌🌀");
    
    // Demostración básica
    let matrix = MonsterMatrix444::new_simple();
    println!("Matriz Monster M₄₄₄ creada exitosamente");
    println!("Traza certificada: {:.6}", matrix.trace().re);
    
    if matrix.is_unitary(1e-5) {
        println!("✅ Matriz es unitaria (M†M ≈ I)");
    } else {
        println!("⚠️  Matriz no es perfectamente unitaria (tolerancia 1e-5)");
    }
    
    // Demostrar algunos autovalores
    println!("\nPrimeros 5 autovalores:");
    for k in 0..5 {
        let eigen = matrix.eigenvalue(k);
        println!("  λ_{} = {:.6} + {:.6}i (|λ| = {:.6})", 
                 k, eigen.re, eigen.im, eigen.norm());
    }
}
