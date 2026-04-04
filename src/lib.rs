use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub mod camera;
pub mod layer;
pub mod parallax;
#[cfg(feature = "animation")]
pub mod sprite;

pub use camera::*;
pub use layer::*;
pub use parallax::*;
#[cfg(feature = "animation")]
pub use sprite::*;

pub struct ParallaxPlugin;

impl Plugin for ParallaxPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ParallaxMoveEvent>()
            .add_systems(PreUpdate, initialize_layers_system);
        #[cfg(feature = "animation")]
        app.add_systems(Update, sprite_frame_update_system);
        app.add_systems(
            Update,
            (camera_follow_system, move_layers_system, update_layer_textures_system)
                .chain()
                .in_set(ParallaxSystems),
        );
    }
}

#[derive(SystemSet, Clone, PartialEq, Eq, Debug, Hash)]
pub struct ParallaxSystems;

/// Initialize newly added `ParallaxLayer` components by spawning their texture grids.
/// Layers must be children of a camera entity with `ParallaxCamera`.
fn initialize_layers_system(
    mut layer_query: Query<(Entity, &mut ParallaxLayer, &ChildOf), Added<ParallaxLayer>>,
    primary_window: Single<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&ParallaxCamera, &Camera, &Transform)>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) -> Result {
    let mut window_size = Vec2::new(primary_window.width(), primary_window.height());

    for (layer_entity, mut layer, child_of) in layer_query.iter_mut() {
        let camera_entity = child_of.parent();
        let (parallax_camera, camera, camera_transform) = camera_query.get(camera_entity)?;
        let cam_pos = camera_transform.translation.truncate();
        if let Some(viewport) = &camera.viewport {
            window_size = viewport.physical_size.as_vec2();
        }

        let render_layer = parallax_camera.render_layer;
        let texture: Handle<Image> = asset_server.load(&layer.path);
        let texture_atlas = layer.create_texture_atlas_layout();
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let sprite_bundle = layer.create_sprite(texture, TextureAtlas::from(texture_atlas_handle));

        let max_length = window_size.length();

        let y_max_index = if layer.repeat.has_vertical() {
            f32::ceil(max_length / (layer.tile_size.y as f32 * layer.scale.y)) as i32
        } else {
            0
        };

        let x_max_index = if layer.repeat.has_horizontal() {
            f32::ceil(max_length / (layer.tile_size.x as f32 * layer.scale.x)) as i32
        } else {
            0
        };

        let texture_count = Vec2::new(f32::max(2.0 * x_max_index as f32, 1.), f32::max(2.0 * y_max_index as f32, 1.));

        let x_range = if layer.repeat.has_horizontal() {
            (-x_max_index + 1)..=x_max_index
        } else {
            0..=0
        };
        let y_range = if layer.repeat.has_vertical() {
            (-y_max_index + 1)..=y_max_index
        } else {
            0..=0
        };

        // Set runtime fields on the layer
        layer.texture_count = texture_count;

        // Set the layer's own transform and render layer
        commands
            .entity(layer_entity)
            .insert(RenderLayers::from_layers(&[render_layer.into()]))
            .insert((
                Transform {
                    translation: Vec3::new(
                        layer.position.x - cam_pos.x * (1.0 - layer.speed.x),
                        layer.position.y - cam_pos.y * (1.0 - layer.speed.y),
                        layer.z,
                    ),
                    scale: layer.scale.extend(1.0),
                    ..default()
                },
                Visibility::Visible,
            ))
            .with_children(|parent| {
                for x in x_range {
                    for y in y_range.clone() {
                        let mut adjusted_sprite = sprite_bundle.clone();
                        layer.repeat.get_strategy().transform(&mut adjusted_sprite, (x, y));

                        let mut transform = Transform::default();
                        transform.translation.x = layer.tile_size.x as f32 * x as f32;
                        transform.translation.y = layer.tile_size.y as f32 * y as f32;
                        let mut child_commands = parent.spawn((adjusted_sprite, transform));
                        let tile_w = layer.tile_size.x as f32;
                        let tile_h = layer.tile_size.y as f32;
                        child_commands
                            .insert(RenderLayers::from_layers(&[render_layer.into()]))
                            .insert(LayerTexture {
                                width: tile_w,
                                height: tile_h,
                                half_width: tile_w * layer.scale.x / 2.0,
                                half_height: tile_h * layer.scale.y / 2.0,
                            });
                        #[cfg(feature = "animation")]
                        if let Some(animation_bundle) = layer.create_animation_bundle() {
                            child_commands.insert(animation_bundle);
                        }
                    }
                }
            });
    }
    Ok(())
}

/// Move camera and apply parallax offsets to child layers.
/// Layers as camera children inherit camera movement automatically.
/// To create parallax (layers moving slower than camera):
/// layer.translation -= delta * (1.0 - speed)
fn move_layers_system(
    mut camera_query: Query<(&mut Transform, &ParallaxCamera, &Children)>,
    mut layer_query: Query<(&mut Transform, &ParallaxLayer), Without<ParallaxCamera>>,
    mut move_events: MessageReader<ParallaxMoveEvent>,
) -> Result {
    for event in move_events.read() {
        if let Ok((mut camera_transform, parallax, children)) = camera_query.get_mut(event.camera) {
            let camera_translation = camera_transform.translation;
            camera_transform.translation = parallax
                .inside_limits(camera_transform.translation.truncate() + event.translation)
                .extend(camera_transform.translation.z);
            let real_translation = camera_transform.translation - camera_translation;
            camera_transform.rotate_z(event.rotation);

            for child in children.iter() {
                if let Ok((mut layer_transform, layer)) = layer_query.get_mut(child) {
                    // Counteract inherited camera movement proportional to (1 - speed)
                    layer_transform.translation.x -= real_translation.x * (1.0 - layer.speed.x);
                    layer_transform.translation.y -= real_translation.y * (1.0 - layer.speed.y);
                }
            }
        }
    }
    Ok(())
}

type LayerQuery<'w, 's> = Query<'w, 's, (&'static ParallaxLayer, &'static Children, &'static GlobalTransform), Without<ParallaxCamera>>;
type TextureQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static GlobalTransform,
        &'static mut Transform,
        &'static LayerTexture,
        &'static ViewVisibility,
    ),
    (Without<ParallaxCamera>, Without<ParallaxLayer>),
>;

/// Update layer texture positions for infinite scrolling.
/// Traverses camera → layer → texture hierarchy via Children.
fn update_layer_textures_system(
    camera_query: Query<(Entity, &Transform, &Camera, &Children), With<ParallaxCamera>>,
    layer_query: LayerQuery,
    mut texture_query: TextureQuery,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut move_events: MessageReader<ParallaxMoveEvent>,
) -> Result {
    // Compute once per frame — valid for all events this tick.
    let primary_window = window_query.single()?;
    let window_size = Vec2::new(primary_window.width(), primary_window.height());

    for event in move_events.read() {
        if !event.has_translation() {
            continue;
        }

        if let Ok((_camera_entity, camera_transform, camera, camera_children)) = camera_query.get(event.camera) {
            let view_size = match &camera.viewport {
                Some(viewport) => viewport.physical_size.as_vec2(),
                _ => window_size,
            };

            for camera_child in camera_children.iter() {
                let Ok((layer, layer_children, _layer_gtransform)) = layer_query.get(camera_child) else {
                    continue;
                };

                for texture_child in layer_children.iter() {
                    let Ok((texture_gtransform, mut texture_transform, layer_texture, computed_visibility)) =
                        texture_query.get_mut(texture_child)
                    else {
                        continue;
                    };

                    // Do not move visible textures
                    if computed_visibility.get() {
                        continue;
                    }

                    // Correct for stale GlobalTransform: compute the texture's position
                    // relative to the camera in world space. GlobalTransform::translation()
                    // is cheaper than compute_transform() as it avoids full matrix decomposition.
                    let stale_texture_translation = camera_transform.translation - texture_gtransform.translation();
                    // The layer counteracts (1-speed) of camera movement, so effective movement is speed
                    let correction = Vec3::new(event.translation.x * layer.speed.x, event.translation.y * layer.speed.y, 0.0);
                    let texture_translation = stale_texture_translation - correction;

                    if layer.repeat.has_horizontal() {
                        let x_delta = layer_texture.width * layer.texture_count.x;
                        let half_width = layer_texture.half_width;
                        if texture_translation.x + half_width < -view_size.x {
                            let distance_offscreen = -view_size.x - (texture_translation.x + half_width);
                            let num_of_jumps = (distance_offscreen / x_delta).ceil().max(1.0);
                            texture_transform.translation.x -= x_delta * num_of_jumps;
                        } else if texture_translation.x - half_width > view_size.x {
                            let distance_offscreen = texture_translation.x - half_width - view_size.x;
                            let num_of_jumps = (distance_offscreen / x_delta).ceil().max(1.0);
                            texture_transform.translation.x += x_delta * num_of_jumps;
                        }
                    }
                    if layer.repeat.has_vertical() {
                        let y_delta = layer_texture.height * layer.texture_count.y;
                        let half_height = layer_texture.half_height;
                        if texture_translation.y + half_height < -view_size.y {
                            let distance_offscreen = -view_size.y - (texture_translation.y + half_height);
                            let num_of_jumps = (distance_offscreen / y_delta).ceil().max(1.0);
                            texture_transform.translation.y -= y_delta * num_of_jumps;
                        } else if texture_translation.y - half_height > view_size.y {
                            let distance_offscreen = texture_translation.y - half_height - view_size.y;
                            let num_of_jumps = (distance_offscreen / y_delta).ceil().max(1.0);
                            texture_transform.translation.y += y_delta * num_of_jumps;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }
    external_doc_test!(include_str!("../README.md"));
}
