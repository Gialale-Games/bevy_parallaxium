use bevy::{prelude::*, window::WindowResolution};
#[cfg(feature = "bevy-inspector-egui")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_parallaxium::{
    CameraFollow, LayerRepeat, Limit, PID, ParallaxCamera, ParallaxLayer, ParallaxPlugin, ParallaxSystems, RepeatStrategy, Vec2Limit,
};

#[derive(Component)]
pub struct Player {
    lin_speed: f32,
    ang_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            lin_speed: 900.,
            ang_speed: 3.,
        }
    }
}

fn main() {
    let primary_window = Window {
        title: "Cyberpunk".to_string(),
        resolution: WindowResolution::new(1280, 720),
        resizable: false,
        ..default()
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
    .add_systems(Update, move_player_system.before(ParallaxSystems))
    .insert_resource(ClearColor(Color::srgb_u8(42, 0, 63)));
    #[cfg(feature = "bevy-inspector-egui")]
    app.add_plugins(WorldInspectorPlugin::new());
    app.run();
}

pub fn move_player_system(keyboard_input: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut player_query: Query<(&mut Transform, &Player)>) {
    let mut rotation: f32 = 0.;
    let mut direction = Vec2::ZERO;
    for (mut player_transform, player) in player_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction += Vec2::new(1., 0.);
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction += Vec2::new(-1., 0.)
        }
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction += Vec2::new(0., 1.);
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction += Vec2::new(0., -1.)
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            rotation -= 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyQ) {
            rotation += 1.;
        }
        direction = direction.normalize_or_zero() * player.lin_speed * time.delta_secs();
        rotation = rotation * player.ang_speed * time.delta_secs();
        player_transform.translation += direction.extend(0.);
        player_transform.rotate_z(rotation);
    }
}

pub fn initialize_camera_system(mut commands: Commands) {
    let player = commands
        .spawn((
            Name::new("Player"),
            Sprite {
                color: bevy::color::palettes::css::YELLOW.into(),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            Transform::from_translation(Vec2::ZERO.extend(3.)),
            Player::default(),
        ))
        .id();
    let y_limit = Limit::zero_to(500.);
    let x_pid = PID::new(0.1, 0.5, 0.01);
    let y_pid = x_pid.with_integral_limit(Limit::new(-25., 25.));
    let offset = Vec2::new(200., 0.);
    commands.spawn((
        Camera2d,
        Transform::from_translation(offset.extend(0.)),
        CameraFollow::pid_xyz(player, &x_pid, &y_pid, &x_pid).with_offset(offset),
        ParallaxCamera {
            limits: Vec2Limit::new(Limit::default(), y_limit),
            ..default()
        },
        children![
            ParallaxLayer::new("cyberpunk_back.png", 0.9)
                .with_speed_2d(Vec2::new(0.9, 0.9))
                .with_repeat(LayerRepeat::horizontally(RepeatStrategy::Same))
                .with_tile_size(UVec2::new(96, 160))
                .with_scale(Vec2::splat(4.5))
                .with_z(0.0),
            ParallaxLayer::new("cyberpunk_middle.png", 0.7)
                .with_speed_2d(Vec2::new(0.7, 0.85))
                .with_repeat(LayerRepeat::horizontally(RepeatStrategy::Same))
                .with_tile_size(UVec2::new(144, 160))
                .with_scale(Vec2::splat(4.5))
                .with_z(0.5)
                .with_flip(true, false)
                .with_position(Vec2::new(0., 48.)),
            ParallaxLayer::new("cyberpunk_middle.png", 0.6)
                .with_speed_2d(Vec2::new(0.6, 0.8))
                .with_repeat(LayerRepeat::horizontally(RepeatStrategy::Same))
                .with_tile_size(UVec2::new(144, 160))
                .with_scale(Vec2::splat(4.5))
                .with_z(1.0)
                .with_position(Vec2::new(0., -64.)),
            ParallaxLayer::new("cyberpunk_front.png", 0.1)
                .with_speed_2d(Vec2::new(0.1, 0.3))
                .with_repeat(LayerRepeat::both(RepeatStrategy::MirrorHorizontally))
                .with_tile_size(UVec2::new(272, 160))
                .with_scale(Vec2::splat(4.5))
                .with_z(2.0),
        ],
    ));
}
