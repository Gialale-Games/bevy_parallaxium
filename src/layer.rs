#[cfg(feature = "animation")]
use std::time::Duration;

use bevy::prelude::*;

#[cfg(feature = "animation")]
use crate::SpriteFrameUpdate;

/// Controls how repeated tiles are flipped when tiling a layer.
#[derive(Debug, Clone)]
pub enum RepeatStrategy {
    /// No tiling — the sprite is rendered exactly once with no repetition.
    None,
    /// All tiles use the same orientation.
    Same,
    /// Every other column is flipped horizontally.
    MirrorHorizontally,
    /// Every other row is flipped vertically.
    MirrorVertically,
    /// Every other tile is flipped both horizontally and vertically.
    MirrorBoth,
}

impl RepeatStrategy {
    pub fn transform(&self, sprite: &mut Sprite, pos: (i32, i32)) {
        match self {
            Self::None | Self::Same => (),
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

/// Defines the axes along which a parallax layer tiles.
#[derive(Debug, Clone)]
pub enum LayerRepeat {
    /// No tiling — the sprite is rendered exactly once.
    None,
    /// Tiles only along the X axis.
    Horizontal(RepeatStrategy),
    /// Tiles only along the Y axis.
    Vertical(RepeatStrategy),
    /// Tiles along both axes.
    Bidirectional(RepeatStrategy),
}

impl LayerRepeat {
    /// No tiling.
    pub fn none() -> Self {
        Self::None
    }

    /// Tile in both directions using the given strategy.
    pub fn both(strategy: RepeatStrategy) -> Self {
        Self::Bidirectional(strategy)
    }

    /// Tile horizontally using the given strategy.
    pub fn horizontally(strategy: RepeatStrategy) -> Self {
        Self::Horizontal(strategy)
    }

    /// Tile vertically using the given strategy.
    pub fn vertically(strategy: RepeatStrategy) -> Self {
        Self::Vertical(strategy)
    }

    /// Tile horizontally with no mirroring.
    pub fn horizontal() -> Self {
        Self::Horizontal(RepeatStrategy::Same)
    }

    /// Tile vertically with no mirroring.
    pub fn vertical() -> Self {
        Self::Vertical(RepeatStrategy::Same)
    }

    /// Returns `true` if this repeat mode tiles along the Y axis.
    pub fn has_vertical(&self) -> bool {
        matches!(self, Self::Vertical(_) | Self::Bidirectional(_))
    }

    /// Returns `true` if this repeat mode tiles along the X axis.
    pub fn has_horizontal(&self) -> bool {
        matches!(self, Self::Horizontal(_) | Self::Bidirectional(_))
    }

    /// Returns the [`RepeatStrategy`] associated with this repeat mode.
    /// Returns [`RepeatStrategy::None`] when the layer does not tile.
    pub fn get_strategy(&self) -> &RepeatStrategy {
        match self {
            Self::None => &RepeatStrategy::None,
            Self::Horizontal(strategy) | Self::Bidirectional(strategy) | Self::Vertical(strategy) => strategy,
        }
    }
}

impl Default for LayerRepeat {
    fn default() -> Self {
        Self::Bidirectional(RepeatStrategy::Same)
    }
}

/// Specifies the playback speed of a sprite-sheet animation.
#[cfg(feature = "animation")]
#[derive(Debug, Clone)]
pub enum Animation {
    /// Frames per second (e.g. `Animation::FPS(12.0)`).
    FPS(f32),
    /// Fixed duration per frame.
    FrameDuration(Duration),
    /// Total duration for one full loop across all frames.
    TotalDuration(Duration),
}

#[cfg(feature = "animation")]
impl Animation {
    /// Converts this animation spec into a [`SpriteFrameUpdate`] for the given grid dimensions
    /// and starting frame index.
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
    #[cfg(feature = "animation")]
    pub animation: Option<Animation>,
    /// Number of texture tiles in each direction
    pub(crate) texture_count: Vec2,
}

impl ParallaxLayer {
    /// Creates a new parallax layer with the given texture path and uniform speed.
    ///
    /// `speed` is applied to both axes: `0.0` means the layer scrolls with the camera
    /// (appears fixed on screen), `1.0` means the layer is fixed in world space.
    pub fn new(path: impl Into<String>, speed: f32) -> Self {
        Self {
            path: path.into(),
            speed: Vec2::splat(speed),
            ..Default::default()
        }
    }

    /// Sets the size of a single tile within the texture atlas.
    pub fn with_tile_size(mut self, tile_size: UVec2) -> Self {
        self.tile_size = tile_size;
        self
    }

    /// Sets the tiling/repeat behavior for this layer.
    pub fn with_repeat(mut self, repeat: LayerRepeat) -> Self {
        self.repeat = repeat;
        self
    }

    /// Sets the world-space scale of each tile.
    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }

    /// Sets the Z depth of this layer.
    pub fn with_z(mut self, z: f32) -> Self {
        self.z = z;
        self
    }

    /// Sets the initial world-space position offset of this layer.
    pub fn with_position(mut self, position: Vec2) -> Self {
        self.position = position;
        self
    }

    /// Sets independent parallax speed factors for X and Y.
    pub fn with_speed_2d(mut self, speed: Vec2) -> Self {
        self.speed = speed;
        self
    }

    /// Sets the tint color applied to all tiles on this layer.
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Sets the initial horizontal and vertical flip state of tiles.
    pub fn with_flip(mut self, flip_x: bool, flip_y: bool) -> Self {
        self.flip = (flip_x, flip_y);
        self
    }

    /// Sets the number of columns in the texture atlas.
    pub fn with_cols(mut self, cols: usize) -> Self {
        self.cols = cols;
        self
    }

    /// Sets the number of rows in the texture atlas.
    pub fn with_rows(mut self, rows: usize) -> Self {
        self.rows = rows;
        self
    }

    /// Sets the starting sprite-atlas frame index.
    pub fn with_index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }

    /// Attaches a sprite-sheet animation to this layer.
    #[cfg(feature = "animation")]
    pub fn with_animation(mut self, animation: Animation) -> Self {
        self.animation = Some(animation);
        self
    }

    /// Builds a [`TextureAtlasLayout`] from this layer's tile size, columns, and rows.
    pub fn create_texture_atlas_layout(&self) -> TextureAtlasLayout {
        TextureAtlasLayout::from_grid(self.tile_size, self.cols as u32, self.rows as u32, None, None)
    }

    /// Creates a [`Sprite`] from this layer's color, flip, and atlas settings.
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

    /// Returns a bundle that drives sprite-sheet animation, or `None` if no animation is set.
    #[cfg(feature = "animation")]
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
            #[cfg(feature = "animation")]
            animation: None,
            texture_count: Vec2::ONE,
        }
    }
}

/// Core component for layer texture
#[derive(Component)]
pub struct LayerTexture {
    /// Width of a single tile in local (unscaled) space
    pub width: f32,
    /// Height of a single tile in local (unscaled) space
    pub height: f32,
    /// Half-width of a tile in world space (precomputed: width * layer.scale.x / 2)
    pub half_width: f32,
    /// Half-height of a tile in world space (precomputed: height * layer.scale.y / 2)
    pub half_height: f32,
}
