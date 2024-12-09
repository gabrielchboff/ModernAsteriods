mod player;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup, (
                spawn_camera,
                player::spawn_player,
            )
        )
        .add_systems(Update, (
                player::move_player,
            ))
        .run();
}

fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(
                window.width() / 2.,
                window.height() / 2.,
                0.,
            ),
            ..default()
        },
    ));

}


