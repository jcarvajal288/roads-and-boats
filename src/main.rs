use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

struct AxialCoordinate {
    q: f32,
    r: f32
}

const HEX_SIZE: f32 = 64.;

fn setup(mut commands: Commands, mut clear_color: ResMut<ClearColor>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    clear_color.0 = Color::hsl(210., 1., 0.8);
    let clear001: Handle<Image> = asset_server.load("Clear/Clear001.png");
    draw_hex(&mut commands, clear001.clone(), AxialCoordinate { q: 0., r: 0. });
    draw_hex(&mut commands, clear001.clone(), AxialCoordinate { q: 0., r: 1. });
    draw_hex(&mut commands, clear001.clone(), AxialCoordinate { q: 1., r: 1. });
}

fn draw_hex(commands: &mut Commands, hex_texture: Handle<Image>, axial_position: AxialCoordinate) {
    let pixel_position: Vec2 = axial_to_pixel(&axial_position);
    commands.spawn(SpriteBundle {
        texture: hex_texture,
        transform: Transform {
            translation: (pixel_position, 0.).into(),
            scale: (0.25, 0.25, 0.25).into(),
            ..default()
        },
        ..default()
    });
}

fn axial_to_pixel(axial: &AxialCoordinate) -> Vec2 {
    let x = HEX_SIZE * (3./2. * axial.q);
    let y = -1. * HEX_SIZE * (f32::sqrt(3.)/2. * axial.q + f32::sqrt(3.) * axial.r);
    return Vec2 { x, y };
}
