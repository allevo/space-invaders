#![allow(implied_bounds_entailment)]

use crate::{
    world::{Bullet, BulletId, Position, Velocity, World},
    Changes, Tick, Effects, EnemyId,
};

use super::Rule;

pub trait RandomInRange: Send + Sync {
    fn random_boolean(&mut self, ids: Vec<EnemyId>) -> Option<EnemyId>;
}
pub struct EnemiesFireBulletsRule {
    pub random_boolean: Box<dyn RandomInRange>,
}
impl Rule for EnemiesFireBulletsRule {
    fn apply(&mut self, world: &mut World, _tick: &Tick, effects: &mut Effects) -> bool {
        let index = self
            .random_boolean
            .random_boolean(world.enemies.keys().cloned().collect());

        let index = match index {
            Some(index) => index,
            None => return false,
        };

        let enemy = &world.enemies[&index];

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

        effects.changes.insert(Changes::NewEnemyBullet(bullet_id));

        false
    }
}
