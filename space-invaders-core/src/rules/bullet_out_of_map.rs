#![allow(implied_bounds_entailment)]

use crate::{world::World, Tick, Effects};

use super::Rule;

pub struct OutOfMapBulletsRule {}
impl Rule for OutOfMapBulletsRule {
    fn apply(&mut self, world: &mut World, _tick: &Tick, _effects: &mut Effects) -> bool {
        for bullet in world.bullets.values_mut() {
            if bullet.position.y >= world.map.height {
                bullet.health = 0;
            }
        }

        false
    }
}
