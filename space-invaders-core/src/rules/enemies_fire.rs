#![allow(implied_bounds_entailment)]

use crate::{
    world::{Bullet, BulletId, Position, Velocity, World},
    Changes, Tick,
};

use super::Rule;

pub trait RandomInRange: Send + Sync {
    fn random_boolean(&mut self, min: u32, max: u32) -> Option<u32>;
}
pub struct EnemiesFireBulletsRule {
    pub random_boolean: Box<dyn RandomInRange>,
}
impl Rule for EnemiesFireBulletsRule {
    fn apply(&mut self, world: &mut World, _tick: &Tick) -> (Option<Vec<Changes>>, Option<Vec<Box<dyn Rule>>>, bool) {
        let index = self
            .random_boolean
            .random_boolean(0, world.enemies.len() as u32);

        let index = match index {
            Some(index) => index,
            None => return (None, None, false),
        };

        let enemy = &world.enemies[index as usize];

        world.bullet_count += 1;
        let bullet_id = BulletId(world.bullet_count);
        world.bullets.insert(
            bullet_id,
            Bullet {
                position: Position {
                    x: enemy.position.x,
                    y: enemy.position.y,
                },
                health: 1,
                velocity: Velocity { x: 0, y: 1 },
            },
        );

        (Some(vec![Changes::NewEnemyBullet(bullet_id)]), None, false)
    }
}
