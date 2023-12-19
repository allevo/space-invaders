mod bullet_out_of_map;
mod bullets_dead;
mod bullets_hit_enemies;
mod enemies_fire;
mod move_bullet;
mod move_enemies;
mod move_spaceship;
mod spaceship_shoot;

use std::sync::Arc;

use crate::{world::World, Effects, Tick};

pub trait Rule: Send + Sync {
    fn should_apply(&self, tick: &Tick) -> bool {
        tick.should_play()
    }

    fn apply(&mut self, world: &mut World, tick: &Tick, effects: &mut Effects) -> bool;
}

impl<T: Rule> Rule for Arc<T> {
    fn should_apply(&self, tick: &Tick) -> bool {
        self.as_ref().should_apply(tick)
    }

    fn apply(&mut self, world: &mut World, tick: &Tick, effects: &mut Effects) -> bool {
        unsafe { Arc::get_mut_unchecked(self).apply(world, tick, effects) }
    }
}

pub use bullet_out_of_map::*;
pub use bullets_dead::*;
pub use bullets_hit_enemies::*;
pub use enemies_fire::*;
pub use move_bullet::*;
pub use move_enemies::*;
pub use move_spaceship::*;
pub use spaceship_shoot::*;
