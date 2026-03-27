use bevy::{prelude::*, window::WindowResolution};
#[cfg(feature = "bevy-inspector-egui")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_parallaxium::{
    Animation, LayerRepeat, ParallaxCamera, ParallaxLayer, ParallaxMoveEvent, ParallaxPlugin, ParallaxSystems, RepeatStrategy,
};

fn main() {
    let primary_window = Window {
        title: "Mills".to_string(),
        resolution: WindowResolution::new(1280, 720),
        resizable: false,
        ..Default::default()
    };

    let mut app = App::new();
    app.add_plugins(
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
    .insert_resource(ClearColor(Color::srgb_u8(156, 219, 248)));
    #[cfg(feature = "bevy-inspector-egui")]
    app.add_plugins(WorldInspectorPlugin::new());

    app.run();
}

pub fn initialize_camera_system(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        ParallaxCamera::default(),
        children![
            ParallaxLayer::new("mills-back.png", 0.99)
                .with_speed_2d(Vec2::new(0.99, 0.99))
                .with_repeat(LayerRepeat::horizontally(RepeatStrategy::Same))
                .with_tile_size(UVec2::new(1123, 794))
                .with_cols(6)
                .with_scale(Vec2::splat(0.15))
                .with_z(0.6)
                .with_position(Vec2::new(0., 50.))
                .with_color(Color::BLACK)
                .with_animation(Animation::FPS(30.)),
            ParallaxLayer::new("mills-back.png", 0.98)
                .with_speed_2d(Vec2::new(0.98, 0.98))
                .with_repeat(LayerRepeat::horizontally(RepeatStrategy::Same))
                .with_tile_size(UVec2::new(1123, 794))
                .with_cols(6)
                .with_scale(Vec2::splat(0.25))
                .with_z(0.7)
                .with_position(Vec2::new(0., 50.))
                .with_color(bevy::color::palettes::css::DARK_GRAY.into())
                .with_index(2)
                .with_animation(Animation::FPS(28.)),
            ParallaxLayer::new("mills-back.png", 0.95)
                .with_speed_2d(Vec2::new(0.95, 0.95))
                .with_repeat(LayerRepeat::horizontally(RepeatStrategy::Same))
                .with_tile_size(UVec2::new(1123, 794))
                .with_cols(6)
                .with_scale(Vec2::splat(0.5))
                .with_z(0.8)
                .with_position(Vec2::new(0., 25.))
                .with_color(bevy::color::palettes::css::GRAY.into())
                .with_index(5)
                .with_animation(Animation::FPS(26.)),
            ParallaxLayer::new("mills-back.png", 0.9)
                .with_speed_2d(Vec2::new(0.9, 0.9))
                .with_repeat(LayerRepeat::horizontally(RepeatStrategy::MirrorBoth))
                .with_tile_size(UVec2::new(1123, 794))
                .with_cols(6)
                .with_scale(Vec2::splat(0.8))
                .with_z(0.9)
                .with_index(1)
                .with_animation(Animation::FPS(24.)),
            ParallaxLayer::new("mills-front.png", 0.8)
                .with_speed_2d(Vec2::new(0.8, 0.8))
                .with_repeat(LayerRepeat::horizontally(RepeatStrategy::MirrorBoth))
                .with_tile_size(UVec2::new(750, 434))
                .with_cols(6)
                .with_z(1.0)
                .with_scale(Vec2::splat(1.5))
                .with_position(Vec2::new(0., -100.))
                .with_index(3)
                .with_animation(Animation::FPS(20.)),
        ],
    ));
}

pub fn move_camera_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut move_event_writer: MessageWriter<ParallaxMoveEvent>,
    mut camera_query: Query<(Entity, &mut Transform), With<Camera>>,
) {
    let (camera, mut camera_transform) = camera_query.single_mut().unwrap();
    let speed = 20.;
    let mut direction = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += Vec2::new(1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction += Vec2::new(-1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction += Vec2::new(0.0, 1.0);
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction += Vec2::new(0.0, -1.0);
    }
    if keyboard_input.pressed(KeyCode::KeyE) {
        camera_transform.rotate_z(0.1);
    }
    if keyboard_input.pressed(KeyCode::KeyQ) {
        camera_transform.rotate_z(-0.1);
    }
    move_event_writer.write(ParallaxMoveEvent {
        translation: direction.normalize_or_zero() * speed,
        camera,
        rotation: 0.,
    });
}
