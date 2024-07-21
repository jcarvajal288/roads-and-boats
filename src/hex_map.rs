use bevy::utils::HashMap;

use crate::cubic::{create_axial, Cubic};

pub struct HexMap {
    pub map: HashMap<Cubic, bool>
}

pub fn create_hex_map(left: i16, right: i16, top: i16, bottom: i16) -> HexMap {
    let mut new_map = HashMap::new();
    for q in left..=right {
        let q_offset = q >> 1;
        for r in (top - q_offset)..=(bottom - q_offset) {
            new_map.insert(create_axial([q, r]), true);
        }
    }
    return HexMap { map: new_map }
}
