use std::fs::read_to_string;
use rand::Rng;

use bevy::asset::Handle;
use bevy::log::info;
use bevy::prelude::{Commands, default, Image, Res, SpriteBundle, Transform, Vec2};
use bevy::utils::HashMap;

use crate::cubic::{create_axial, Cubic, cubic_from_pixel};
use crate::hex::Hex;
use crate::images::Images;

pub const HEX_SIZE: f32 = 64.;
const HEX_SCALE: f32 = 0.25;

pub enum Terrain {
    WOODS,
    PASTURE,
    ROCK,
    MOUNTAINS,
    DESERT,
    SEA,
    CITY
}

pub struct HexMap {
    pub map: HashMap<Cubic, Hex>
}

impl HexMap {
    pub fn draw(&self, mut commands: Commands, images: &Res<Images>) {
        for hex in &self.map {
            draw_hex(&mut commands, hex, images);
        }
    }

}

pub fn read_map_from_file(map_filename: &str) -> HexMap {
    let mut rng = rand::thread_rng();
    let mut new_map = HashMap::new();
    for line in read_to_string(map_filename).unwrap().lines() {
        if line.is_empty() {
            continue;
        }
        let split_line: Vec<&str> = line.split(':').collect();  
        let terrain = parse_terrain_type(split_line[1]);
        let coords: Vec<&str> = split_line[0].split(',').collect();
        let q = coords[0].parse::<i16>().unwrap();
        let r = coords[1].parse::<i16>().unwrap();
        let terrain_variant = rng.gen_range(0..5);
        let hex = Hex { terrain, terrain_variant };
        new_map.insert(create_axial([q, r]), hex);
    }
    return HexMap { map: new_map };
}

pub fn handle_left_click(mouse_position: Vec2) {
    let clicked_cubic_position = cubic_from_pixel(mouse_position);
    info!("[{},{}]", clicked_cubic_position.q(), clicked_cubic_position.r());
}

fn parse_terrain_type(terrain_code: &str) -> Terrain {
    return match terrain_code {
        "W" => Terrain::WOODS,
        "P" => Terrain::PASTURE,
        "R" => Terrain::ROCK,
        "M" => Terrain::MOUNTAINS,
        "D" => Terrain::DESERT,
        "S" => Terrain::SEA,
        "C" => Terrain::CITY,
        _ => Terrain::SEA
    }
}

fn draw_hex(commands: &mut Commands, hex: (&Cubic, &Hex), images: &Res<Images>) {
    let pixel_position: Vec2 = hex.0.to_pixel(HEX_SIZE);
    commands.spawn(SpriteBundle {
        texture: get_image(&hex.1.terrain, hex.1.terrain_variant, images),
        transform: Transform {
            translation: (pixel_position, 0.).into(),
            scale: (HEX_SCALE, HEX_SCALE, HEX_SCALE).into(),
            ..default()
        },
        ..default()
    });
}

fn get_image(terrain: &Terrain, variant: usize, images: &Res<Images>) -> Handle<Image> {
    return match terrain {
        Terrain::WOODS => images.woods[variant].clone(),
        Terrain::PASTURE => images.pasture[variant].clone(),
        Terrain::ROCK => images.rock[variant].clone(),
        Terrain::MOUNTAINS => images.mountains[variant].clone(),
        Terrain::DESERT => images.desert[variant].clone(),
        Terrain::SEA => images.sea[variant].clone(),
        Terrain::CITY => images.city[variant].clone(),
    }
}
