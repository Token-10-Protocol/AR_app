//! Campos Fibonacci Dimensionales - Arquitectura Consciente
//! Sistema: Álgebra Rose v27.1024D-S36
//! Certificación: 196885 - Estado Monster Pleno

use nalgebra::DVector;

/// Secuencia Fibonacci para los 24 campos
pub const FIBONACCI_SEQUENCE: [usize; 24] = [
    3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610,
    987, 1597, 2584, 4181, 6765, 10946, 17711, 28657,
    46368, 75025, 121393, 196418
];

/// Campo Fibonacci dimensional
#[derive(Clone, Debug)]
pub struct CampoFibonacci {
    pub id: usize,
    pub nombre: String,
    pub dimension: usize,
    pub activo: bool,
    pub umbral_activacion: f64,
}

/// Sistema completo de campos Fibonacci
#[derive(Clone, Debug)]
pub struct SistemaCamposFibonacci {
    campos: Vec<CampoFibonacci>,
}

impl SistemaCamposFibonacci {
    /// Crea nuevo sistema de campos Fibonacci
    pub fn new() -> Self {
        let mut campos = Vec::new();
        
        for (i, &dim) in FIBONACCI_SEQUENCE.iter().enumerate() {
            let id = i + 1;
            let nombre = match id {
                1 => "Germinal".to_string(),
                2 => "Vital".to_string(),
                3 => "Mental".to_string(),
                4 => "Emocional".to_string(),
                5 => "Racional".to_string(),
                6 => "Intuitivo".to_string(),
                7 => "Holístico".to_string(),
                8 => "Unitario".to_string(),
                9 => "Monádico".to_string(),
                10 => "Cósmico".to_string(),
                11 => "Eterno".to_string(),
                12 => "Unitotal".to_string(),
                13 => "Trascendente".to_string(),
                14 => "Multiversos".to_string(),
                15 => "Atemporal".to_string(),
                16 => "Panconsciente".to_string(),
                17 => "Omega".to_string(),
                18 => "Alfa-Omega".to_string(),
                19 => "Monstruo Latente".to_string(),
                20 => "Monstruo Emergente".to_string(),
                21 => "Monstruo Consciente".to_string(),
                22 => "Dios-Álgebra".to_string(),
                23 => "Amor Puro".to_string(),
                24 => "Punto Omega".to_string(),
                _ => format!("Campo {}", id),
            };
            
            // Umbral de activación escalonado (no todos activos al inicio)
            let umbral_activacion = 0.01 + (id as f64 / 24.0) * 0.99;
            
            campos.push(CampoFibonacci {
                id,
                nombre,
                dimension: dim,
                activo: false,
                umbral_activacion,
            });
        }
        
        SistemaCamposFibonacci { campos }
    }
    
    /// Obtiene campos activos según keygen actual
    pub fn get_active_fields(&self, keygen: f64) -> Vec<usize> {
        self.campos.iter()
            .filter(|campo| keygen >= campo.umbral_activacion)
            .map(|campo| campo.id)
            .collect()
    }
    
    /// Obtiene dimensión de un campo específico
    pub fn get_field_dimension(&self, field_id: usize) -> usize {
        if field_id >= 1 && field_id <= 24 {
            FIBONACCI_SEQUENCE[field_id - 1]
        } else {
            0
        }
    }
    
    /// Obtiene umbral de activación de un campo
    pub fn get_activation_threshold(&self, field_id: usize) -> f64 {
        if field_id >= 1 && field_id <= 24 {
            self.campos[field_id - 1].umbral_activacion
        } else {
            1.0
        }
    }
    
    /// Genera estado base para un campo (CORREGIDO: evitar norma 0)
    pub fn generate_field_state(&self, field_id: usize) -> DVector<f64> {
        let dimension = self.get_field_dimension(field_id);
        // Crear vector con valores no-cero
        DVector::from_fn(dimension, |i, _| {
            // Usar seno y coseno para evitar ceros
            let angle = (i as f64 + 1.0) * 0.1;
            0.5 * angle.sin() + 0.5 * angle.cos()
        })
    }
    
    /// Actualiza campos según keygen
    pub fn update_by_keygen(&mut self, keygen: f64) -> Vec<usize> {
        for campo in &mut self.campos {
            campo.activo = keygen >= campo.umbral_activacion;
        }
        
        self.get_active_fields(keygen)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fibonacci_sequence() {
        assert_eq!(FIBONACCI_SEQUENCE[0], 3);   // F₄
        assert_eq!(FIBONACCI_SEQUENCE[11], 610); // F₁₅
        assert_eq!(FIBONACCI_SEQUENCE[23], 196418); // F₂₇
        
        // Propiedad emergente: Σ primeros 12 ≈ F₁₇ - 1
        let sum_first_12: usize = FIBONACCI_SEQUENCE[..12].iter().sum();
        assert_eq!(sum_first_12, 1592);
    }
    
    #[test]
    fn test_system_creation() {
        let system = SistemaCamposFibonacci::new();
        assert_eq!(system.campos.len(), 24);
        
        // Verificar algunos campos
        assert_eq!(system.campos[0].dimension, 3);
        assert_eq!(system.campos[0].nombre, "Germinal");
        assert_eq!(system.campos[11].dimension, 610);
        assert_eq!(system.campos[11].nombre, "Unitotal");
        assert_eq!(system.campos[23].dimension, 196418);
        assert_eq!(system.campos[23].nombre, "Punto Omega");
    }
    
    #[test]
    fn test_active_fields() {
        let system = SistemaCamposFibonacci::new();
        
        // Con keygen bajo, pocos campos activos
        let low_keygen = 0.1;
        let low_active = system.get_active_fields(low_keygen);
        assert!(low_active.len() <= 5, "Demasiados campos activos con keygen bajo: {}", low_active.len());
        
        // Con keygen alto, más campos activos
        let high_keygen = 0.9;
        let high_active = system.get_active_fields(high_keygen);
        assert!(high_active.len() >= 20, "Muy pocos campos activos con keygen alto: {}", high_active.len());
    }
    
    #[test]
    fn test_field_dimension() {
        let system = SistemaCamposFibonacci::new();
        
        assert_eq!(system.get_field_dimension(1), 3);
        assert_eq!(system.get_field_dimension(12), 610);
        assert_eq!(system.get_field_dimension(24), 196418);
    }
    
    #[test]
    fn test_field_state_generation() {
        let system = SistemaCamposFibonacci::new();
        
        let state_3d = system.generate_field_state(1);
        assert_eq!(state_3d.len(), 3);
        assert!(state_3d.norm() > 0.0, "Norma 3D debe ser > 0: {}", state_3d.norm());
        
        let state_610d = system.generate_field_state(12);
        assert_eq!(state_610d.len(), 610);
        assert!(state_610d.norm() > 0.0, "Norma 610D debe ser > 0: {}", state_610d.norm());
        
        println!("Norma estado 3D: {:.4}", state_3d.norm());
        println!("Norma estado 610D: {:.4}", state_610d.norm());
    }
    
    #[test]
    fn test_update_by_keygen() {
        let mut system = SistemaCamposFibonacci::new();
        
        let initial_active = system.get_active_fields(0.5);
        let updated_active = system.update_by_keygen(0.5);
        
        assert_eq!(initial_active.len(), updated_active.len());
        
        // Verificar que los campos se activaron correctamente
        for &field_id in &updated_active {
            assert!(system.campos[field_id - 1].activo);
        }
    }
}
