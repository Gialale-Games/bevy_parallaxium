use bevy::{
    camera::{Viewport, visibility::RenderLayers},
    prelude::*,
    window::WindowResolution,
};
use bevy_parallaxium::{LayerRepeat, ParallaxCamera, ParallaxLayer, ParallaxMoveEvent, ParallaxPlugin, ParallaxSystems, RepeatStrategy};

fn main() {
    let primary_window = Window {
        title: "Split Screen".to_string(),
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
        Camera {
            order: 0,
            viewport: Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(1280 / 2, 720),
                ..default()
            }),
            ..default()
        },
        ParallaxCamera::new(1),
        RenderLayers::from_layers(&[0, 1]),
        InputMap::awsd(),
        children![
            ParallaxLayer::new("cyberpunk_back.png", 0.9)
                .with_repeat(LayerRepeat::horizontally(RepeatStrategy::Same))
                .with_tile_size(UVec2::new(96, 160))
                .with_scale(Vec2::splat(4.5))
                .with_z(0.0),
            ParallaxLayer::new("cyberpunk_middle.png", 0.6)
                .with_repeat(LayerRepeat::horizontally(RepeatStrategy::Same))
                .with_tile_size(UVec2::new(144, 160))
                .with_scale(Vec2::splat(4.5))
                .with_z(1.0),
            ParallaxLayer::new("cyberpunk_front.png", 0.1)
                .with_repeat(LayerRepeat::horizontally(RepeatStrategy::Same))
                .with_tile_size(UVec2::new(272, 160))
                .with_scale(Vec2::splat(4.5))
                .with_z(2.0),
        ],
    ));
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            viewport: Some(Viewport {
                physical_position: UVec2::new(1280 / 2, 0),
                physical_size: UVec2::new(1280 / 2, 720),
                ..default()
            }),
            clear_color: ClearColorConfig::None,
            ..default()
        },
        ParallaxCamera::new(2),
        RenderLayers::from_layers(&[0, 2]),
        InputMap::arrows(),
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

#[derive(Component)]
pub struct InputMap {
    right: KeyCode,
    left: KeyCode,
    up: KeyCode,
    down: KeyCode,
}

impl InputMap {
    pub fn arrows() -> Self {
        Self {
            right: KeyCode::ArrowRight,
            left: KeyCode::ArrowLeft,
            up: KeyCode::ArrowUp,
            down: KeyCode::ArrowDown,
        }
    }

    pub fn awsd() -> Self {
        Self {
            right: KeyCode::KeyD,
            left: KeyCode::KeyA,
            up: KeyCode::KeyW,
            down: KeyCode::KeyS,
        }
    }
}

pub fn move_camera_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut move_event_writer: MessageWriter<ParallaxMoveEvent>,
    camera_query: Query<(Entity, &InputMap), With<Camera>>,
) {
    for (camera, input_map) in camera_query.iter() {
        if keyboard_input.pressed(input_map.right) {
            move_event_writer.write(ParallaxMoveEvent {
                translation: Vec2::new(9.0, 0.0),
                rotation: 0.,
                camera,
            });
        } else if keyboard_input.pressed(input_map.left) {
            move_event_writer.write(ParallaxMoveEvent {
                translation: Vec2::new(-9.0, 0.0),
                rotation: 0.,
                camera,
            });
        }
        if keyboard_input.pressed(input_map.up) {
            move_event_writer.write(ParallaxMoveEvent {
                translation: Vec2::new(0.0, 9.0),
                rotation: 0.,
                camera,
            });
        } else if keyboard_input.pressed(input_map.down) {
            move_event_writer.write(ParallaxMoveEvent {
                translation: Vec2::new(0.0, -9.0),
                rotation: 0.,
                camera,
            });
        }
    }
}
