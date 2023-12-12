#![allow(implied_bounds_entailment)]

use crate::{Changes, Tick, World, Effects};

use super::Rule;

pub struct MoveSpaceshipRule {
    pub delta: i32,
}

impl Rule for MoveSpaceshipRule {
    fn should_apply(&self, _tick: &Tick) -> bool {
        true
    }

    fn apply(&mut self, world: &mut World, _tick: &Tick, effects: &mut Effects) -> bool {
        if world.spaceship.position.x == 0 && self.delta < 0 {
            return true;
        }
        if world.spaceship.position.x == world.map.width - 1 && self.delta > 0 {
            return true;
        }

        world.spaceship.position.x = (world.spaceship.position.x as i32 + self.delta) as u32;

        effects.changes.insert(Changes::SpaceshipMove(world.spaceship.position));

        true
    }
}
