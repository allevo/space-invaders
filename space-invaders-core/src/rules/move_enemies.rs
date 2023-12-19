use crate::{world::World, Changes, Effects, Tick};

use super::Rule;

pub struct MoveEnemiesRule {
    pub low_bound: i32,
    pub high_bound: i32,
}
impl Rule for MoveEnemiesRule {
    fn apply(&mut self, world: &mut World, _tick: &Tick, effects: &mut Effects) -> bool {
        println!("{:?}", world.enemies);

        for enemy in world.enemies.values_mut() {
            let x = (enemy.position.x as i32) + enemy.velocity.x;
            println!("enemy.position.x: {}, x: {}", enemy.position.x, x);
            enemy.position.x = x.max(self.low_bound).min(self.high_bound) as u32;

            let mut y_delta = 0;
            if enemy.position.x as i32 == self.high_bound
                || enemy.position.x as i32 == self.low_bound
            {
                enemy.velocity.x = -enemy.velocity.x;
                y_delta = -1;
            }
            enemy.position.y = (enemy.position.y as i32 + y_delta) as u32;
        }

        effects.changes.insert(Changes::EnemiesMove);

        false
    }
}
