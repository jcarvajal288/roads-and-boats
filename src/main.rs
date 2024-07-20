use bevy::asset::ron::de::Position;
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

fn setup(mut commands: Commands, mut clear_color: ResMut<ClearColor>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    clear_color.0 = Color::hsl(210., 1., 0.8);
    let clear001: Handle<Image> = asset_server.load("Clear/Clear001.png");
    let position = AxialCoordinate { q: 0., r: 0. };
    draw_hex(commands, clear001, position)
}

fn draw_hex(mut commands: Commands, hex_texture: Handle<Image>, position: AxialCoordinate) {
    commands.spawn(SpriteBundle {
        texture: hex_texture,
        transform: Transform {
            translation: (position.q, position.r, 0.).into(),
            scale: (0.25, 0.25, 0.25).into(),
            ..default()
        },
        ..default()
    });
}
