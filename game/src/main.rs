use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
enum Collider {
    Player,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("crab.png"),
        transform: Transform {
            translation: Vec3::new(0.0, -215.0, 0.0),
            // scale: Vec3::new(30.0, 30.0, 0.0),
            ..Default::default()
        },
        sprite: Sprite {
            // color: Color::rgb(0.5, 0.5, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player { speed: 500.0 })
    .insert(Collider::Player);
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup);
    }
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (paddle, mut transform) = query.single_mut();
    
    let mut direction_x = 0.0;
    let mut direction_y = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction_x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction_x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        direction_y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        direction_y -= 1.0;
    }

    let translation = &mut transform.translation;
    // move the paddle horizontally
    translation.x += direction_x * paddle.speed * TIME_STEP;
    // bound the paddle within the walls
    translation.x = translation.x.min(380.0).max(-380.0);
    // move the player in y also because Marisa said so
    translation.y += direction_y * paddle.speed * TIME_STEP;
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_movement_system)
        )
        .run();
}
