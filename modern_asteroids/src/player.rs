use bevy::prelude::*;
#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub score: u32,
    pub health: u32,
    pub name: String,
}

#[derive(Resource)]
pub struct OffsetedCursorPosition {
    pub x: f32,
    pub y: f32,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            speed: 500.0,
            score: 0,
            health: 100,
            name,
        }
    }

}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let player = Player::new("player".to_string());
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/player.png"),
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            ..default()
        },
        player,

    ));
}

pub fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1., 0., 0.);
        }
        if keys.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1., 0., 0.);
        }
        if keys.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0., 1., 0.);
        }
        if keys.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0., -1., 0.);
        }
        if direction.length() > 0. {
            direction = direction.normalize();
        }
        transform.translation += direction * 500.0 * time.delta_seconds();

    }
}
