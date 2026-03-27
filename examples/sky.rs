use bevy::{prelude::*, window::WindowResolution};
use bevy_parallaxium::{ParallaxCamera, ParallaxLayer, ParallaxMoveEvent, ParallaxPlugin, ParallaxSystems};

fn main() {
    let primary_window = Window {
        title: "Sky".to_string(),
        resolution: WindowResolution::new(1280, 720),
        resizable: false,
        ..default()
    };

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(primary_window),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(ParallaxPlugin)
        .add_systems(Startup, initialize_camera_system)
        .add_systems(Update, move_camera_system.before(ParallaxSystems))
        .run();
}

pub fn initialize_camera_system(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        ParallaxCamera::default(),
        children![
            ParallaxLayer::new("sky-stars.png", 0.9)
                .with_speed_2d(Vec2::new(0.9, 0.9))
                .with_tile_size(UVec2::new(53, 55))
                .with_scale(Vec2::splat(3.0))
                .with_z(0.0),
            ParallaxLayer::new("sky-clouds.png", 0.1)
                .with_speed_2d(Vec2::new(0.1, 0.5))
                .with_tile_size(UVec2::new(109, 43))
                .with_scale(Vec2::splat(4.0))
                .with_z(1.0),
        ],
    ));
}

pub fn move_camera_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    camera_query: Query<Entity, With<Camera>>,
    mut move_event_writer: MessageWriter<ParallaxMoveEvent>,
) {
    let camera = camera_query.single().unwrap();
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        move_event_writer.write(ParallaxMoveEvent {
            translation: Vec2::new(3.0, 0.0),
            rotation: 0.,
            camera,
        });
    } else if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        move_event_writer.write(ParallaxMoveEvent {
            translation: Vec2::new(-3.0, 0.0),
            rotation: 0.,
            camera,
        });
    }
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        move_event_writer.write(ParallaxMoveEvent {
            translation: Vec2::new(0.0, 3.0),
            rotation: 0.,
            camera,
        });
    } else if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        move_event_writer.write(ParallaxMoveEvent {
            translation: Vec2::new(0.0, -3.0),
            rotation: 0.,
            camera,
        });
    }
}
