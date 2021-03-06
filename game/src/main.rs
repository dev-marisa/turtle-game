use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
mod camera;
use camera::*;
use rand::Rng;

const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Component)]
struct Particle;

#[derive(Component)]
struct Barrier;

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
enum Collider {
    Player,
}

fn spawn_sand_particles(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    
    // right barier
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(570.0, 0.0, 0.0),
            scale: Vec3::new(400.0, 10000.0, 0.0),
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::rgb(0.66, 0.59, 0.4),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Barrier {});

    // left barier
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(-570.0, 0.0, 0.0),
            scale: Vec3::new(400.0, 10000.0, 0.0),
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::rgb(0.66, 0.59, 0.4),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Barrier {});

    // spawn some random particles
    for i in 0..1000 {
        let mut rand_x = rng.gen_range(-1000..1000);
        let mut rand_y = rng.gen_range(-1000..5000);
        commands.spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(rand_x as f32, rand_y as f32, 0.0),
                scale: Vec3::new(3.0, 3.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.2, 0.3, 0.2),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Particle {});
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // cameras
    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // commands.spawn_bundle(UiCameraBundle::default());
    // commands.spawn_bundle(new_camera_2d()); 

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("turtle_idle_01.png"),
        transform: Transform {
            translation: Vec3::new(0.0, -100.0, 0.0),
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
    .with_children(|parent| {
        parent.spawn_bundle(new_camera_2d());
    })    
    .insert(Collider::Player);
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_sand_particles)    
            .add_startup_system(setup);
    }
}

// fn player_movement_system(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut query: Query<(&Player, &mut Transform)>,
//     mut camera: Query<&mut Transform, With<Camera>>
// ) {
fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>
) {
    let (paddle, mut transform) = query.single_mut();
    
    let mut direction_x = 0.0;
    let mut direction_y = 0.0;
    let mut cam_dir_y = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction_x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction_x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        direction_y += 1.0;
        cam_dir_y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        direction_y -= 1.0;
        cam_dir_y -= 1.0;
    }

    let translation = &mut transform.translation;
    // move the paddle horizontally
    translation.x += direction_x * paddle.speed * TIME_STEP;
    // bound the paddle within the walls
    translation.x = translation.x.min(360.0).max(-360.0);
    // move the player in y also because Marisa said so
    translation.y += direction_y * paddle.speed * TIME_STEP;
    // let cam_trans = &mut camera;
    // cam_trans.y = cam_dir_y;
}



fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "crab gaem".to_string(),
            width: 900.0,
            height: 600.0,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        // Bevy, as bare bones it is, I've decided to just 
        // add a color for now, we can probably get a more
        // complicated texture, but just a color for now
        // Bevy "rgb" actually goes from 0-1, so
        // (255, 255, 255) is actually (1.0, 1.0, 1.0)
        // I just used https://www.calculator.net/percent-calculator.html
                                                  // nice
        .insert_resource(ClearColor(Color::rgb(0.76, 0.69, 0.5)))
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_movement_system)
        )      
        .run();
}
