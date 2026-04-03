use bevy::prelude::*;

/// Event used to update parallax
#[derive(Message, Debug)]
pub struct ParallaxMoveEvent {
    /// camera translation
    pub translation: Vec2,

    /// camera rotation
    pub rotation: f32,

    pub camera: Entity,
}

impl ParallaxMoveEvent {
    pub fn new(camera: Entity, translation: Vec2, rotation: f32) -> Self {
        Self { camera, translation, rotation }
    }

    pub fn translate(camera: Entity, translation: Vec2) -> Self {
        Self { camera, translation, rotation: 0.0 }
    }

    pub fn rotate(camera: Entity, rotation: f32) -> Self {
        Self { camera, translation: Vec2::ZERO, rotation }
    }

    pub fn has_translation(&self) -> bool {
        self.translation != Vec2::ZERO
    }

    pub fn has_right_translation(&self) -> bool {
        self.translation.x > 0.
    }

    pub fn has_left_translation(&self) -> bool {
        self.translation.x < 0.
    }

    pub fn has_up_translation(&self) -> bool {
        self.translation.y > 0.
    }

    pub fn has_down_translation(&self) -> bool {
        self.translation.y < 0.
    }
}

/// Attach to a camera to enable parallax scrolling on its child `ParallaxLayer` entities.
#[derive(Component)]
pub struct ParallaxCamera {
    pub render_layer: u8,
    pub limits: Vec2Limit,
}

#[derive(Debug, Clone, Copy)]
pub struct Limit {
    pub min: f32,
    pub max: f32,
}

impl Default for Limit {
    fn default() -> Self {
        Self {
            min: f32::NEG_INFINITY,
            max: f32::INFINITY,
        }
    }
}

impl Limit {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn zero_to_infinity() -> Self {
        Self {
            min: 0.,
            max: f32::INFINITY,
        }
    }

    pub fn zero_to(max: f32) -> Self {
        Self { min: 0., max }
    }

    pub fn fix(&self, value: f32) -> f32 {
        value.clamp(self.min, self.max)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2Limit {
    pub x: Limit,
    pub y: Limit,
}

impl Vec2Limit {
    pub fn new(x: Limit, y: Limit) -> Self {
        Self { x, y }
    }

    pub fn fix(&self, vec: Vec2) -> Vec2 {
        Vec2::new(self.x.fix(vec.x), self.y.fix(vec.y))
    }
}

impl Default for Vec2Limit {
    fn default() -> Self {
        Self {
            x: default(),
            y: default(),
        }
    }
}

impl ParallaxCamera {
    pub fn inside_limits(&self, translation: Vec2) -> Vec2 {
        self.limits.fix(translation)
    }

    pub fn new(render_layer: u8) -> Self {
        Self { render_layer, ..default() }
    }
}

impl Default for ParallaxCamera {
    fn default() -> Self {
        Self {
            render_layer: 0,
            limits: default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::{ecs::entity::EntityRow, prelude::*};

    use crate::ParallaxMoveEvent;

    #[test]
    fn test_parallax_event() {
        let camera = Entity::from_row(EntityRow::from_raw_u32(0).unwrap());

        let no_movement = ParallaxMoveEvent {
            translation: Vec2::ZERO,
            rotation: 0.,
            camera,
        };
        assert!(!no_movement.has_translation());
        assert!(!no_movement.has_up_translation());
        assert!(!no_movement.has_down_translation());
        assert!(!no_movement.has_left_translation());
        assert!(!no_movement.has_right_translation());

        let up = ParallaxMoveEvent {
            translation: Vec2::new(0., 1.),
            rotation: 0.,
            camera,
        };
        assert!(up.has_translation());
        assert!(up.has_up_translation());
        assert!(!up.has_down_translation());
        assert!(!up.has_left_translation());
        assert!(!up.has_right_translation());

        let down = ParallaxMoveEvent {
            translation: Vec2::new(0., -1.),
            rotation: 0.,
            camera,
        };
        assert!(down.has_translation());
        assert!(!down.has_up_translation());
        assert!(down.has_down_translation());
        assert!(!down.has_left_translation());
        assert!(!down.has_right_translation());

        let left = ParallaxMoveEvent {
            translation: Vec2::new(-1., 0.),
            rotation: 0.,
            camera,
        };
        assert!(left.has_translation());
        assert!(!left.has_up_translation());
        assert!(!left.has_down_translation());
        assert!(left.has_left_translation());
        assert!(!left.has_right_translation());

        let right = ParallaxMoveEvent {
            translation: Vec2::new(1., 0.),
            rotation: 0.,
            camera,
        };
        assert!(right.has_translation());
        assert!(!right.has_up_translation());
        assert!(!right.has_down_translation());
        assert!(!right.has_left_translation());
        assert!(right.has_right_translation());

        let left_down = ParallaxMoveEvent {
            translation: Vec2::new(-1., -1.),
            rotation: 0.,
            camera,
        };
        assert!(left_down.has_translation());
        assert!(!left_down.has_up_translation());
        assert!(left_down.has_down_translation());
        assert!(left_down.has_left_translation());
        assert!(!left_down.has_right_translation());

        let up_right = ParallaxMoveEvent {
            translation: Vec2::new(1., 1.),
            rotation: 0.,
            camera,
        };
        assert!(up_right.has_translation());
        assert!(up_right.has_up_translation());
        assert!(!up_right.has_down_translation());
        assert!(!up_right.has_left_translation());
        assert!(up_right.has_right_translation());
    }
}
