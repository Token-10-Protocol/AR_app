//! Fonética paramétrica para múltiples idiomas
//! Extensión del sistema básico español

use crate::monster_structures::ParametrosArticulatorios;

impl ParametrosArticulatorios {
    // FONEMAS ESPAÑOLES (ya existentes)
    
    /// Fonema /l/ - lateral alveolar (para "love", "amour", etc.)
    pub fn fonema_l() -> Self {
        Self {
            punto_articulacion: 0.3,    // alveolar
            modo_articulacion: 0.7,     // lateral aproximante
            sonoridad: 0.9,             // sonora
            nasalidad: 0.0,             // oral
            redondeamiento: 0.1,        // neutral
        }
    }
    
    /// Fonema /v/ - fricativo labiodental (inglés/francés)
    pub fn fonema_v() -> Self {
        Self {
            punto_articulacion: 0.1,    // labiodental
            modo_articulacion: 0.65,    // fricativo
            sonoridad: 0.85,            // sonora
            nasalidad: 0.0,             // oral
            redondeamiento: 0.3,        // labios ligeramente redondeados
        }
    }
    
    /// Fonema /u/ - vocal cerrada posterior redondeada (francés "amour")
    pub fn fonema_u() -> Self {
        Self {
            punto_articulacion: 0.9,    // posterior extremo
            modo_articulacion: 0.95,    // vocal cerrada
            sonoridad: 1.0,             // sonora
            nasalidad: 0.0,             // oral
            redondeamiento: 1.0,        // muy redondeado
        }
    }
    
    /// Fonema /e/ - vocal media anterior no redondeada (italiano "amore")
    pub fn fonema_e() -> Self {
        Self {
            punto_articulacion: 0.4,    // anterior
            modo_articulacion: 0.85,    // vocal media
            sonoridad: 0.95,            // sonora
            nasalidad: 0.0,             // oral
            redondeamiento: 0.0,        // no redondeado
        }
    }
    
    /// Fonema /ɔ/ - vocal media-abierta posterior redondeada (portugués)
    pub fn fonema_open_o() -> Self {
        Self {
            punto_articulacion: 0.75,   // posterior
            modo_articulacion: 0.6,     // vocal abierta
            sonoridad: 0.9,             // sonora
            nasalidad: 0.0,             // oral
            redondeamiento: 0.8,        // redondeado
        }
    }
    
    /// Fonema /ʃ/ - fricativo postalveolar (francés "amour" - sonido "ch")
    pub fn fonema_sh() -> Self {
        Self {
            punto_articulacion: 0.45,   // postalveolar
            modo_articulacion: 0.7,     // fricativo
            sonoridad: 0.0,             // sorda (en francés)
            nasalidad: 0.0,             // oral
            redondeamiento: 0.5,        // labios protruidos
        }
    }
    
    /// Fonema /ʁ/ - fricativo uvular (francés "r" gutural)
    pub fn fonema_french_r() -> Self {
        Self {
            punto_articulacion: 0.95,   // uvular (garganta)
            modo_articulacion: 0.75,    // fricativo
            sonoridad: 0.7,             // parcialmente sonora
            nasalidad: 0.0,             // oral
            redondeamiento: 0.3,        // labios ligeramente redondeados
        }
    }
    
    /// Fonema /ð/ - fricativo dental (inglés "th" en "the")
    pub fn fonema_th_voiced() -> Self {
        Self {
            punto_articulacion: 0.25,   // dental
            modo_articulacion: 0.68,    // fricativo
            sonoridad: 0.8,             // sonora
            nasalidad: 0.0,             // oral
            redondeamiento: 0.0,        // no redondeado
        }
    }
    
    /// Fonema /ŋ/ - nasal velar (inglés final en "sing")
    pub fn fonema_ng() -> Self {
        Self {
            punto_articulacion: 0.85,   // velar
            modo_articulacion: 0.35,    // nasal
            sonoridad: 0.9,             // sonora
            nasalidad: 1.0,             // nasal completo
            redondeamiento: 0.0,        // no redondeado
        }
    }
}
