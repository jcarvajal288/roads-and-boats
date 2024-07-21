use bevy::utils::HashMap;

use crate::cubic::{create_axial, Cubic};

pub struct HexMap {
    pub width: i16,
    pub height: i16,
    map: HashMap<Cubic, bool>
}

pub fn create_hex_map(width: i16, height: i16) -> HexMap {
    let mut new_map = HashMap::new();
    for q in 0..height {
        for r in 0..width {
            new_map.insert(create_axial([q, r]), true);
        }
    }
    return HexMap { width, height, map: new_map }
}
