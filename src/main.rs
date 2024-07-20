use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut clear_color: ResMut<ClearColor>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    clear_color.0 = Color::hsl(210., 1., 0.8);
    commands.spawn(SpriteBundle {
        texture: asset_server.load("Clear/Clear001.png"),
        transform: Transform::from_scale((0.25, 0.25, 0.25).into()),
        ..default()
    });
}
