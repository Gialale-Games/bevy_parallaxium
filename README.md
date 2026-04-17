# bevy_parallaxium

A parallax plugin for the [Bevy Engine](https://bevyengine.org/). This plugin allows you to easily create scrolling
parallax backgrounds for your games.

![bandit](/assets/bandit.gif)

> **Fork of [bevy-parallax](https://github.com/Corrosive-Games/bevy-parallax)** — rewritten for Bevy 0.17 with a new
> component-based API

## Usage

### Basic setup

Add the plugin and spawn a camera with `ParallaxCamera`. Attach `ParallaxLayer` components as children.

```rust,no_run
use bevy::prelude::*;
use bevy_parallaxium::{
    LayerRepeat, ParallaxCamera, ParallaxLayer, ParallaxMoveEvent, ParallaxPlugin, ParallaxSystems,
    RepeatStrategy,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(ParallaxPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, move_camera.before(ParallaxSystems))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        ParallaxCamera::default(),
        children![
            ParallaxLayer::new("sky.png", 0.9)
                .with_tile_size(UVec2::new(384, 216))
                .with_repeat(LayerRepeat::horizontal())
                .with_z(-2.0),
            ParallaxLayer::new("mountains.png", 0.5)
                .with_tile_size(UVec2::new(320, 240))
                .with_repeat(LayerRepeat::horizontally(RepeatStrategy::MirrorHorizontally))
                .with_z(-1.0),
            ParallaxLayer::new("ground.png", 0.0)
                .with_tile_size(UVec2::new(640, 360))
                .with_repeat(LayerRepeat::horizontal())
                .with_z(0.0),
        ],
    ));
}

fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut move_events: MessageWriter<ParallaxMoveEvent>,
    camera_query: Query<Entity, With<Camera>>,
) {
    let camera = camera_query.single().unwrap();
    let mut translation = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        translation.x += 3.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        translation.x -= 3.0;
    }
    if translation != Vec2::ZERO {
        move_events.write(ParallaxMoveEvent::translate(camera, translation));
    }
}
```

### ParallaxLayer builder API

`ParallaxLayer::new(path, speed)` creates a layer. `speed` is applied uniformly to both axes:
- `0.0` — layer is fixed to the camera (no parallax effect)
- `1.0` — layer is fixed in world space (full parallax)

| Method | Description |
|:-------|:------------|
| `.with_tile_size(UVec2)` | Size of a single tile in the texture atlas |
| `.with_repeat(LayerRepeat)` | Tiling axes and mirroring strategy |
| `.with_scale(Vec2)` | World-space scale of each tile |
| `.with_z(f32)` | Z depth of this layer |
| `.with_position(Vec2)` | Initial world-space position offset |
| `.with_speed_2d(Vec2)` | Independent X/Y parallax speed |
| `.with_color(Color)` | Tint color applied to all tiles |
| `.with_flip(bool, bool)` | Horizontal and vertical flip |
| `.with_cols(usize)` | Number of atlas columns (sprite-sheets) |
| `.with_rows(usize)` | Number of atlas rows (sprite-sheets) |
| `.with_index(usize)` | Starting atlas frame index |
| `.with_animation(Animation)` | Sprite-sheet animation |

### Tiling

`LayerRepeat` controls how tiles fill the screen:

```rust,no_run
LayerRepeat::none()                                        // single sprite, no tiling
LayerRepeat::horizontal()                                  // tile X axis, no mirroring
LayerRepeat::vertical()                                    // tile Y axis, no mirroring
LayerRepeat::horizontally(RepeatStrategy::MirrorHorizontally) // tile X, flip every other
LayerRepeat::both(RepeatStrategy::MirrorBoth)             // tile both axes, flip both
```

`RepeatStrategy` variants: `Same`, `MirrorHorizontally`, `MirrorVertically`, `MirrorBoth`.

### Animation

```rust,no_run
ParallaxLayer::new("spritesheet.png", 0.5)
    .with_tile_size(UVec2::new(32, 32))
    .with_cols(8)
    .with_rows(1)
    .with_animation(Animation::FPS(12.0))
```

`Animation` variants:
- `Animation::FPS(f32)` — frames per second
- `Animation::FrameDuration(Duration)` — fixed time per frame
- `Animation::TotalDuration(Duration)` — total loop duration

### Camera following

Add `CameraFollow` to a `ParallaxCamera` entity to track another entity:

```rust,no_run
use bevy_parallaxium::{CameraFollow, PID};

// Snap directly to target
CameraFollow::fixed(player_entity)

// Proportional smoothing
CameraFollow::proportional(player_entity, 0.1)

// Full PID controller
CameraFollow::pid(player_entity, &PID::new(0.1, 0.05, 0.001))

// Offset the follow target
CameraFollow::new(player_entity).with_offset(Vec2::new(0., 40.))
```

### Camera limits

Clamp camera movement to world bounds using `Vec2Limit`:

```rust,no_run
use bevy_parallaxium::{Limit, ParallaxCamera, Vec2Limit};

ParallaxCamera {
    limits: Vec2Limit::new(
        Limit::new(-500.0, 500.0), // X bounds
        Limit::new(-200.0, 200.0), // Y bounds
    ),
    ..default()
}
```

### Mirrored view

Flip the camera horizontally — as if viewing the scene from behind — with `ViewDirection`. Parallax, tiling, and `CameraFollow` all continue to work unchanged.

```rust,no_run
use bevy_parallaxium::{ParallaxCamera, ViewDirection};

// At spawn time:
ParallaxCamera::default().with_view_direction(ViewDirection::Mirrored)
```

Toggle at runtime by mutating the component — the plugin syncs `Transform.scale.x` for you:

```rust,no_run
fn toggle_mirror(mut cameras: Query<&mut ParallaxCamera>) {
    for mut cam in &mut cameras {
        cam.view_direction = match cam.view_direction {
            ViewDirection::Normal => ViewDirection::Mirrored,
            ViewDirection::Mirrored => ViewDirection::Normal,
        };
    }
}
```

> **Don't set `camera.transform.scale.x` yourself** — the plugin owns its sign. You can still set `scale.x`'s magnitude for zoom; only the sign is overwritten.

### Sending move events manually

Instead of `CameraFollow`, you can drive the camera yourself by sending `ParallaxMoveEvent`:

```rust,no_run
move_events.write(ParallaxMoveEvent::translate(camera, Vec2::new(3.0, 0.0)));
move_events.write(ParallaxMoveEvent::rotate(camera, 0.01));
move_events.write(ParallaxMoveEvent::new(camera, Vec2::new(3.0, 0.0), 0.01));
```

## Feature flags

| Feature | Default | Description |
|:--------|:-------:|:------------|
| `animation` | yes | Enables `Animation`, `SpriteFrameUpdate`, `.with_animation()`, and the built-in frame-advance system. Disable if you drive sprite-sheet animation yourself to avoid double-ticking. |

To disable the built-in animation system:

```toml
[dependencies]
bevy_parallaxium = { version = "0.1", default-features = false }
```

## Compatible Bevy versions

| Bevy version | `bevy_parallaxium` version |
|:-------------|:--------------------------|
| `0.17`       | `0.1`                     |

## Credits

Forked from [bevy-parallax](https://github.com/Corrosive-Games/bevy-parallax) by Corrosive Games.
