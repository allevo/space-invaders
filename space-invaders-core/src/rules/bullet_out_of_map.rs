#![allow(implied_bounds_entailment)]

use crate::{world::World, Tick, Changes};

use super::Rule;

pub struct OutOfMapBulletsRule {}
impl Rule for OutOfMapBulletsRule {
    fn apply(&mut self, world: &mut World, _tick: &Tick) -> (Option<Vec<Changes>>, Option<Vec<Box<dyn Rule>>>, bool) {
        for bullet in world.bullets.values_mut() {
            if bullet.position.y >= world.map.height {
                bullet.health = 0;
            }
        }

        (None, None, false)
    }
}
