use bevy::{prelude::*, window::WindowResolution};
use bevy_parallaxium::{ParallaxCamera, ParallaxLayer, ParallaxMoveEvent, ParallaxPlugin, ParallaxSystems};

fn main() {
    let primary_window = Window {
        title: "Fishy".to_string(),
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
    let water_color = Color::srgba(0.5, 0.8, 1.0, 1.0);
    let sand_color = Color::srgba(0.7, 0.6, 0.4, 1.0);
    commands.spawn((
        Camera2d,
        ParallaxCamera::default(),
        children![
            ParallaxLayer::new("fishy_back.png", 0.9)
                .with_tile_size(UVec2::new(256, 596))
                .with_scale(Vec2::splat(1.25))
                .with_z(0.0)
                .with_color(water_color),
            ParallaxLayer::new("fishy_middle_1.png", 0.8)
                .with_tile_size(UVec2::new(256, 596))
                .with_scale(Vec2::splat(1.25))
                .with_z(1.0)
                .with_color(water_color),
            ParallaxLayer::new("fishy_middle_2.png", 0.6)
                .with_tile_size(UVec2::new(256, 596))
                .with_scale(Vec2::splat(1.25))
                .with_z(2.0)
                .with_color(sand_color),
            ParallaxLayer::new("fishy_middle_3.png", 0.5)
                .with_tile_size(UVec2::new(256, 596))
                .with_scale(Vec2::splat(1.25))
                .with_z(3.0)
                .with_color(sand_color),
            ParallaxLayer::new("fishy_middle_4.png", 0.4)
                .with_tile_size(UVec2::new(256, 596))
                .with_scale(Vec2::splat(1.25))
                .with_z(4.0)
                .with_color(sand_color),
            ParallaxLayer::new("fishy_front.png", 0.2)
                .with_tile_size(UVec2::new(256, 596))
                .with_scale(Vec2::splat(1.25))
                .with_z(5.0)
                .with_color(sand_color),
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
}
