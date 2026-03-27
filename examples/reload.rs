use bevy::{prelude::*, window::WindowResolution};
use bevy_parallaxium::{ParallaxCamera, ParallaxLayer, ParallaxMoveEvent, ParallaxPlugin, ParallaxSystems};

fn main() {
    let primary_window = Window {
        title: "Reload".to_string(),
        resolution: WindowResolution::new(1280, 720),
        resizable: false,
        ..Default::default()
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
        .add_systems(Update, reload_system)
        .run();
}

fn spawn_layers(commands: &mut ChildSpawnerCommands) {
    commands.spawn(
        ParallaxLayer::new("cyberpunk_back.png", 0.9)
            .with_tile_size(UVec2::new(96, 160))
            .with_scale(Vec2::splat(4.5))
            .with_z(0.0),
    );
    commands.spawn(
        ParallaxLayer::new("cyberpunk_middle.png", 0.6)
            .with_tile_size(UVec2::new(144, 160))
            .with_scale(Vec2::splat(4.5))
            .with_z(1.0),
    );
    commands.spawn(
        ParallaxLayer::new("cyberpunk_front.png", 0.1)
            .with_tile_size(UVec2::new(272, 160))
            .with_scale(Vec2::splat(4.5))
            .with_z(2.0),
    );
}

pub fn initialize_camera_system(mut commands: Commands) {
    commands.spawn((Camera2d, ParallaxCamera::default())).with_children(spawn_layers);
}

pub fn reload_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    camera_query: Query<(Entity, &Children), With<ParallaxCamera>>,
) {
    if keyboard_input.just_released(KeyCode::KeyR) {
        let (camera, children) = camera_query.single().unwrap();
        // Despawn all existing layer entities
        for child in children.iter() {
            commands.entity(child).despawn();
        }
        // Spawn new layers
        commands.entity(camera).with_children(spawn_layers);
    }
}

pub fn move_camera_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut move_event_writer: MessageWriter<ParallaxMoveEvent>,
    camera_query: Query<Entity, With<Camera>>,
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
}
