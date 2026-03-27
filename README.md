# bevy-parallax

A parallax plugin for the [Bevy Engine](https://bevyengine.org/). This plugin allows you to easily create scrolling
parallax backgrounds for your games.

![cyberpunk](assets/cyberpunk.gif)

![fishy](assets/fishy.gif)

## Usage

```rust,no_run
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_parallaxium::{
    ParallaxCamera, ParallaxLayer, ParallaxMoveEvent, ParallaxPlugin, ParallaxSystems,
};

fn main() {
    let primary_window = Window {
        title: "Window Name".to_string(),
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
            ParallaxLayer::new("cyberpunk_back.png", 0.9)
                .with_tile_size(UVec2::new(96, 160))
                .with_scale(Vec2::splat(4.5))
                .with_z(0.0),
            ParallaxLayer::new("cyberpunk_middle.png", 0.6)
                .with_tile_size(UVec2::new(144, 160))
                .with_scale(Vec2::splat(4.5))
                .with_z(1.0),
            ParallaxLayer::new("cyberpunk_front.png", 0.1)
                .with_tile_size(UVec2::new(272, 160))
                .with_scale(Vec2::splat(4.5))
                .with_z(2.0),
        ],
    ));
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
```

## Compatible Bevy versions

Compatibility of `bevy-parallax` versions:

| Bevy version | `bevy-parallax` version |
|:-------------|:------------------------|
| `0.17`       | `0.12`                  |
| `0.15`       | `11`                    |
| `0.14`       | `0.9` - `0.10`          |
| `0.13`       | `0.8`                   |
| `0.12`       | `0.7`                   |
| `0.11`       | `0.5` - `0.6`           |
| `0.10`       | `0.4`                   |

## Credits

- [Fish World Pack](https://spicylobster.itch.io/fish-world-pack)

- [Warped City 2](https://ansimuz.itch.io/warped-city-2)

- [Mills](https://www.freepik.com/free-vector/flat-wheat-background-with-field_1599667.htm#query=mill%20background%20flat&position=25&from_view=search&track=ais#position=25&query=mill%20background%20flat)
