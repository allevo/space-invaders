mod bullet_dead;
mod bullet_out_of_map;
mod bullets_hit_enemies;
mod enemies_fires;
mod move_bullet;
mod move_enemies;

use crate::world::World;

pub trait Rule {
    fn apply(&mut self, world: &mut World);
}

pub use bullet_dead::*;
pub use bullet_out_of_map::*;
pub use bullets_hit_enemies::*;
pub use enemies_fires::*;
pub use move_bullet::*;
pub use move_enemies::*;
