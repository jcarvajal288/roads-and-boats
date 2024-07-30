use crate::hex_map::Terrain;

use rand::Rng;

pub struct Hex {
    pub terrain: Terrain,
    pub terrain_variant: usize,
}

pub fn create_hex(terrain: Terrain) -> Hex {
    let mut rng = rand::thread_rng();
    let terrain_variant = rng.gen_range(0..5);
    return Hex { terrain, terrain_variant };
}

