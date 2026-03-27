use std::time::Duration;

use bevy::prelude::*;

use crate::SpriteFrameUpdate;
#[cfg(feature = "bevy-inspector-egui")]
use bevy_inspector_egui::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy-inspector-egui", derive(Reflect, InspectorOptions))]
pub enum RepeatStrategy {
    Same,
    MirrorHorizontally,
    MirrorVertically,
    MirrorBoth,
}

impl RepeatStrategy {
    pub fn transform(&self, sprite: &mut Sprite, pos: (i32, i32)) {
        match self {
            Self::Same => (),
            Self::MirrorHorizontally => {
                let (x, _) = pos;
                sprite.flip_x ^= x % 2 != 0;
            }
            Self::MirrorVertically => {
                let (_, y) = pos;
                sprite.flip_y ^= y % 2 != 0;
            }
            Self::MirrorBoth => {
                let (x, y) = pos;
                sprite.flip_x ^= x % 2 != 0;
                sprite.flip_y ^= y % 2 != 0;
            }
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy-inspector-egui", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "bevy-inspector-egui", reflect(InspectorOptions))]
pub enum LayerRepeat {
    Horizontal(RepeatStrategy),
    Vertical(RepeatStrategy),
    Bidirectional(RepeatStrategy),
}

impl LayerRepeat {
    pub fn both(strategy: RepeatStrategy) -> Self {
        Self::Bidirectional(strategy)
    }

    pub fn horizontally(strategy: RepeatStrategy) -> Self {
        Self::Horizontal(strategy)
    }

    pub fn vertically(strategy: RepeatStrategy) -> Self {
        Self::Vertical(strategy)
    }

    pub fn horizontal() -> Self {
        Self::Horizontal(RepeatStrategy::Same)
    }

    pub fn vertical() -> Self {
        Self::Vertical(RepeatStrategy::Same)
    }

    pub fn has_vertical(&self) -> bool {
        matches!(self, Self::Vertical(_) | Self::Bidirectional(_))
    }

    pub fn has_horizontal(&self) -> bool {
        matches!(self, Self::Horizontal(_) | Self::Bidirectional(_))
    }

    pub fn get_strategy(&self) -> RepeatStrategy {
        match self {
            Self::Horizontal(strategy) => strategy.clone(),
            Self::Bidirectional(strategy) => strategy.clone(),
            Self::Vertical(strategy) => strategy.clone(),
        }
    }
}

impl Default for LayerRepeat {
    fn default() -> Self {
        Self::Bidirectional(RepeatStrategy::Same)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy-inspector-egui", derive(Reflect, InspectorOptions))]
pub enum Animation {
    FPS(f32),
    FrameDuration(Duration),
    TotalDuration(Duration),
}

impl Animation {
    pub fn to_sprite_update(&self, cols: usize, rows: usize, index: usize) -> SpriteFrameUpdate {
        let total = cols * rows;
        let duration = match self {
            Self::FPS(fps) => Duration::from_secs_f32(1. / fps),
            Self::FrameDuration(duration) => *duration,
            Self::TotalDuration(duration) => duration.div_f32(total as f32),
        };
        SpriteFrameUpdate {
            total,
            index,
            timer: Timer::new(duration, TimerMode::Repeating),
        }
    }
}

/// Core component for a parallax layer. Spawn as a child of a camera with `ParallaxCamera`.
#[derive(Component)]
#[cfg_attr(feature = "bevy-inspector-egui", derive(Reflect, InspectorOptions))]
pub struct ParallaxLayer {
    /// Path to the texture asset
    pub path: String,
    /// Parallax speed factor as a 2D vector.
    /// 0.0 = fixed to camera (moves with it), 1.0 = fixed in world (doesn't move with camera).
    pub speed: Vec2,
    /// How the layer repeats
    pub repeat: LayerRepeat,
    /// Size of a tile of the texture
    pub tile_size: UVec2,
    /// Columns in the texture file
    pub cols: usize,
    /// Rows in the texture file
    pub rows: usize,
    /// Scale of the texture
    pub scale: Vec2,
    /// Z position of the layer
    pub z: f32,
    /// Default initial position
    pub position: Vec2,
    /// Tint color
    pub color: Color,
    /// Sprite atlas index
    pub index: usize,
    /// Flip (horizontal, vertical)
    pub flip: (bool, bool),
    /// Optional animation
    pub animation: Option<Animation>,
    /// Whether this layer has been initialized (textures spawned)
    pub(crate) initialized: bool,
    /// Number of texture tiles in each direction
    pub(crate) texture_count: Vec2,
}

impl ParallaxLayer {
    /// Create a new parallax layer with the given texture path and speed.
    /// Speed is a scalar applied uniformly: 0.0 = moves with camera, 1.0 = stationary in world.
    pub fn new(path: impl Into<String>, speed: f32) -> Self {
        Self {
            path: path.into(),
            speed: Vec2::splat(speed),
            ..Default::default()
        }
    }

    pub fn with_tile_size(mut self, tile_size: UVec2) -> Self {
        self.tile_size = tile_size;
        self
    }

    pub fn with_repeat(mut self, repeat: LayerRepeat) -> Self {
        self.repeat = repeat;
        self
    }

    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_z(mut self, z: f32) -> Self {
        self.z = z;
        self
    }

    pub fn with_position(mut self, position: Vec2) -> Self {
        self.position = position;
        self
    }

    pub fn with_speed_2d(mut self, speed: Vec2) -> Self {
        self.speed = speed;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_flip(mut self, flip_x: bool, flip_y: bool) -> Self {
        self.flip = (flip_x, flip_y);
        self
    }

    pub fn with_cols(mut self, cols: usize) -> Self {
        self.cols = cols;
        self
    }

    pub fn with_rows(mut self, rows: usize) -> Self {
        self.rows = rows;
        self
    }

    pub fn with_index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }

    pub fn with_animation(mut self, animation: Animation) -> Self {
        self.animation = Some(animation);
        self
    }

    pub fn create_texture_atlas_layout(&self) -> TextureAtlasLayout {
        TextureAtlasLayout::from_grid(self.tile_size, self.cols as u32, self.rows as u32, None, None)
    }

    pub fn create_sprite(&self, image: Handle<Image>, atlas: TextureAtlas) -> Sprite {
        Sprite {
            image,
            texture_atlas: Some(atlas),
            color: self.color,
            flip_x: self.flip.0,
            flip_y: self.flip.1,
            ..Default::default()
        }
    }

    pub fn create_animation_bundle(&self) -> Option<impl Bundle> {
        self.animation
            .as_ref()
            .map(|animation| animation.to_sprite_update(self.cols, self.rows, self.index))
    }
}

impl Default for ParallaxLayer {
    fn default() -> Self {
        Self {
            path: String::new(),
            speed: Vec2::splat(1.0),
            repeat: LayerRepeat::default(),
            tile_size: UVec2::ZERO,
            cols: 1,
            rows: 1,
            scale: Vec2::ONE,
            z: 0.0,
            position: Vec2::ZERO,
            color: Color::WHITE,
            index: 0,
            flip: (false, false),
            animation: None,
            initialized: false,
            texture_count: Vec2::ONE,
        }
    }
}

/// Core component for layer texture
#[derive(Component)]
#[cfg_attr(feature = "bevy-inspector-egui", derive(Reflect, InspectorOptions))]
pub struct LayerTexture {
    /// Width of the texture
    pub width: f32,
    /// Height of the texture
    pub height: f32,
}
