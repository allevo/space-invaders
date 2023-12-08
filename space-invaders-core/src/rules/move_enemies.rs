#![allow(implied_bounds_entailment)]

use crate::{world::World, Tick, Changes};

use super::Rule;

pub enum Direction {
    Left,
    Right,
}

impl Direction {
    fn x_delta(&self) -> i32 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

pub struct MoveEnemiesRule {
    pub direction: Direction,
}
impl Rule for MoveEnemiesRule {
    fn apply(&mut self, world: &mut World, tick: &Tick) -> (Option<Vec<Changes>>, Option<Vec<Box<dyn Rule>>>, bool) {
        // This is wrong.
        // TODO: leverage the map width
        let max = 10;
        let mut y_delta = 0;

        if tick.0 > 0 && tick.0 % max == 0 {
            self.direction = self.direction.opposite();
            y_delta = 1;
        }

        for enemy in &mut world.enemies {
            let x = (enemy.position.x as i32) + self.direction.x_delta();
            enemy.position.x = x.max(0) as u32;
            enemy.position.y += y_delta;
        }

        (Some(vec![crate::Changes::EnemiesMove]), None, false)
    }
}
