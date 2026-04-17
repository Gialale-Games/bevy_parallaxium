use bevy::prelude::*;
use bevy_parallaxium::{CameraFollow, LayerRepeat, ParallaxCamera, ParallaxLayer, ParallaxPlugin, ParallaxSystems, RepeatStrategy};

const ATLAS_COLUMNS: usize = 6;
const ATLAS_ROWS: usize = 7;
const FRAME_DURATION: f32 = 0.1;

const IDLE_ROW: usize = 0;
const IDLE_FRAMES: usize = 4;
const RUN_ROW: usize = 3;
const RUN_FRAMES: usize = 6;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bandit Demo".to_string(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(ParallaxPlugin)
        .insert_resource(SunTimer {
            elapsed: 0.0,
            duration: 220.0,
            start_y: 100.0,
            end_y: -110.0,
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (player_input_system, player_animation_system, sun_movement_system)
                .chain()
                .before(ParallaxSystems),
        )
        .run();
}

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct Sun;

#[derive(Resource)]
struct SunTimer {
    elapsed: f32,
    duration: f32,
    start_y: f32,
    end_y: f32,
}

#[derive(Component)]
struct PlayerAnim {
    row: usize,
    col: usize,
    timer: Timer,
    facing_right: bool,
}

impl Default for PlayerAnim {
    fn default() -> Self {
        Self {
            row: IDLE_ROW,
            col: 0,
            timer: Timer::from_seconds(FRAME_DURATION, TimerMode::Repeating),
            facing_right: true,
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut layouts: ResMut<Assets<TextureAtlasLayout>>) {
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(32),
        ATLAS_COLUMNS as u32,
        ATLAS_ROWS as u32,
        None,
        None,
    ));

    let player = commands
        .spawn((
            Sprite {
                image: asset_server.load("MiniKingMan.png"),
                texture_atlas: Some(TextureAtlas { layout, index: 1 }),
                ..default()
            },
            Transform::from_xyz(0., -80., 1.).with_scale(Vec3::splat(1.0)),
            Player { speed: 100.0 },
            PlayerAnim::default(),
        ))
        .id();

    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 1.0 / 3.0,
            ..OrthographicProjection::default_2d()
        }),
        ParallaxCamera::default(),
        CameraFollow::new(player).with_offset(Vec2::new(0., 40.)),
        children![
            // Sky — barely scrolls, nearly pinned to camera
            (
                ParallaxLayer::new("sunny.png", 0.95)
                    .with_tile_size(UVec2::new(50, 50))
                    .with_repeat(LayerRepeat::None)
                    .with_position(Vec2::new(20., 90.))
                    .with_z(-5.0),
                Sun
            ),
            ParallaxLayer::new("sky.png", 0.95)
                .with_tile_size(UVec2::new(384, 216))
                .with_repeat(LayerRepeat::horizontal())
                .with_position(Vec2::new(0., 30.))
                .with_z(-6.0),
            // Near clouds
            ParallaxLayer::new("near-clouds.png", 0.55)
                .with_tile_size(UVec2::new(144, 240))
                .with_repeat(LayerRepeat::horizontally(RepeatStrategy::MirrorHorizontally))
                .with_position(Vec2::new(0., 0.))
                .with_z(-4.0),
            // // Mountains
            ParallaxLayer::new("mountains.png", 0.4)
                .with_tile_size(UVec2::new(320, 240))
                .with_position(Vec2::new(0., 10.))
                .with_repeat(LayerRepeat::horizontally(RepeatStrategy::MirrorHorizontally))
                .with_z(-3.0),
            // Trees — close foreground, strong parallax
            ParallaxLayer::new("trees.png", 0.1)
                .with_tile_size(UVec2::new(240, 240))
                .with_position(Vec2::new(0., 0.))
                .with_repeat(LayerRepeat::horizontally(RepeatStrategy::MirrorHorizontally))
                .with_z(-2.0),
            ParallaxLayer::new("ground.png", 0.0)
                .with_tile_size(UVec2::new(640, 360))
                .with_repeat(LayerRepeat::horizontal())
                .with_position(Vec2::new(0., 35.))
                .with_z(-1.0),
        ],
    ));
}

/// Moves the player with A/D or arrow keys and switches animation row.
fn player_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut PlayerAnim, &Player)>,
) {
    let Ok((mut transform, mut anim, player)) = query.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
        anim.facing_right = false;
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
        anim.facing_right = true;
    }

    let target_row = if direction != Vec2::ZERO { RUN_ROW } else { IDLE_ROW };
    if anim.row != target_row {
        anim.row = target_row;
        anim.col = 0;
    }

    if direction != Vec2::ZERO {
        transform.translation += (direction.normalize() * player.speed * time.delta_secs()).extend(0.0);
    }
}

/// Advances the sprite-sheet frame and applies horizontal flip for direction.
fn player_animation_system(time: Res<Time>, mut query: Query<(&mut Sprite, &mut PlayerAnim)>) {
    let Ok((mut sprite, mut anim)) = query.single_mut() else {
        return;
    };

    if anim.timer.tick(time.delta()).just_finished() {
        let frames = if anim.row == RUN_ROW { RUN_FRAMES } else { IDLE_FRAMES };
        anim.col = (anim.col + 1) % frames;
    }

    if let Some(atlas) = &mut sprite.texture_atlas {
        atlas.index = anim.row * ATLAS_COLUMNS + anim.col;
    }

    sprite.flip_x = !anim.facing_right;
}

/// Moves the sun downward over 60 seconds.
fn sun_movement_system(time: Res<Time>, mut sun_timer: ResMut<SunTimer>, mut query: Query<&mut Transform, With<Sun>>) {
    if sun_timer.elapsed >= sun_timer.duration {
        return;
    }
    sun_timer.elapsed = (sun_timer.elapsed + time.delta_secs()).min(sun_timer.duration);
    let t = sun_timer.elapsed / sun_timer.duration;
    if let Ok(mut transform) = query.single_mut() {
        transform.translation.y = sun_timer.start_y + (sun_timer.end_y - sun_timer.start_y) * t;
    }
}
