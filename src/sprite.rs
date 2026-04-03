use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct SpriteFrameUpdate {
    pub index: usize,
    pub total: usize,
    pub timer: Timer,
}

impl SpriteFrameUpdate {
    /// Advance the animation by `duration` and return the new atlas index.
    /// Only increments the frame when the timer fires.
    pub fn next_index(&mut self, duration: Duration) -> usize {
        if self.timer.tick(duration).just_finished() {
            self.index = (self.index + 1) % self.total;
        }
        self.index
    }
}

pub fn sprite_frame_update_system(time: Res<Time>, mut query: Query<(&mut Sprite, &mut SpriteFrameUpdate)>) -> Result {
    let delta = time.delta();
    for (mut sprite, mut frame) in query.iter_mut() {
        if frame.timer.tick(delta).just_finished() {
            frame.index = (frame.index + 1) % frame.total;
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = frame.index;
            }
        }
    }
    Ok(())
}
