use algebra_rose_core::matrix_444::MonsterMatrix444;

fn main() {
    println!("🌌 Álgebra Rose - Sistema Interfaz Humano-Universo");
    println!("💖🌌🌀");
    
    // Demostración básica
    let matrix = MonsterMatrix444::new();
    println!("Matriz Monster M₄₄₄ creada exitosamente");
    println!("Traza certificada: {:.6}", matrix.trace().re);
    
    if matrix.is_unitary(1e-10) {
        println!("✅ Matriz es unitaria (M†M = I)");
    } else {
        println!("❌ Matriz no es unitaria");
    }
}
