use crate::world::World;

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
    pub ticks: u32,
    pub direction: Direction,
}
impl Rule for MoveEnemiesRule {
    fn apply(&mut self, world: &mut World) {
        // This is wrong.
        // TODO: leverage the map width
        let max = 10;
        let mut y_delta = 0;

        if self.ticks > 0 && self.ticks % max == 0 {
            self.direction = self.direction.opposite();
            y_delta = 1;
        }

        for enemy in &mut world.enemies {
            let x = (enemy.position.x as i32) + self.direction.x_delta();
            enemy.position.x = x.max(0) as u32;
            enemy.position.y += y_delta;
        }

        self.ticks += 1;
    }
}
