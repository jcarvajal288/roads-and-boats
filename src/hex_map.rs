use std::fs::read_to_string;

use bevy::asset::Handle;
use bevy::prelude::{Commands, default, Image, Res, SpriteBundle, Transform, Vec2};
use bevy::utils::HashMap;

use crate::cubic::{create_axial, Cubic};
use crate::images::Images;

const HEX_SIZE: f32 = 64.;
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
    pub map: HashMap<Cubic, Terrain>
}

impl HexMap {
    pub fn draw(&self, mut commands: Commands, images: &Res<Images>) {
        for hex in &self.map {
            draw_hex(&mut commands, hex, images);
        }
    }

}

pub fn read_map_from_file(map_filename: &str) -> HexMap {
    let mut new_map = HashMap::new();
    for line in read_to_string(map_filename).unwrap().lines() {
        if line.is_empty() {
            continue;
        }
        let split_line: Vec<&str> = line.split(':').collect();  
        let terrain_type = parse_terrain_type(split_line[1]);
        let coords: Vec<&str> = split_line[0].split(',').collect();
        let q = coords[0].parse::<i16>().unwrap();
        let r = coords[1].parse::<i16>().unwrap();
        new_map.insert(create_axial([q, r]), terrain_type);
    }
    return HexMap { map: new_map };
}

pub fn create_hex_map(left: i16, right: i16, top: i16, bottom: i16) -> HexMap {
    let mut new_map = HashMap::new();
    for q in left..=right {
        let q_offset = q >> 1;
        for r in (top - q_offset)..=(bottom - q_offset) {
            let terrain = if (q + r % 2 == 0) { Terrain::PASTURE } else { Terrain::SEA };
            new_map.insert(create_axial([q, r]), terrain);
        }
    }
    return HexMap { map: new_map };
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

fn draw_hex(commands: &mut Commands, hex: (&Cubic, &Terrain), images: &Res<Images>) {
    let pixel_position: Vec2 = hex.0.to_pixel(HEX_SIZE);
    commands.spawn(SpriteBundle {
        texture: get_image(hex.1, images),
        transform: Transform {
            translation: (pixel_position, 0.).into(),
            scale: (HEX_SCALE, HEX_SCALE, HEX_SCALE).into(),
            ..default()
        },
        ..default()
    });
}

fn get_image(terrain: &Terrain, images: &Res<Images>) -> Handle<Image> {
    return match terrain {
        Terrain::WOODS => images.woods.clone(),
        Terrain::PASTURE => images.pasture.clone(),
        Terrain::ROCK => images.rock.clone(),
        Terrain::MOUNTAINS => images.mountains.clone(),
        Terrain::DESERT => images.desert.clone(),
        Terrain::SEA => images.sea.clone(),
        Terrain::CITY => images.city.clone(),
    }
}
