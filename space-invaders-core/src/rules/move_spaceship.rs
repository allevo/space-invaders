#![allow(implied_bounds_entailment)]

use crate::{Changes, Tick, World};

use super::Rule;

pub struct MoveSpaceshipRule {
    pub delta: i32,
}

impl Rule for MoveSpaceshipRule {
    fn should_apply(&self, _tick: &Tick) -> bool {
        true
    }

    fn apply(&mut self, world: &mut World, _tick: &Tick) -> (Option<Vec<Changes>>, Option<Vec<Box<dyn Rule>>>, bool) {
        if world.spaceship.position.x == 0 && self.delta < 0 {
            return (None, None, true);
        }
        if world.spaceship.position.x == world.map.width - 1 && self.delta > 0 {
            return (None, None, true);
        }

        world.spaceship.position.x = (world.spaceship.position.x as i32 + self.delta) as u32;

        (
            Some(vec![Changes::SpaceshipMove(world.spaceship.position)]),
            None,
            true,
        )
    }
}
