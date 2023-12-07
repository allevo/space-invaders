use crate::world::{BulletId, World};

use super::Rule;

pub struct BulletDeadRule {}
impl Rule for BulletDeadRule {
    fn apply(&mut self, world: &mut World) {
        let bullets_to_remove: Vec<BulletId> = world
            .bullets
            .iter()
            .filter(|(_, bullet)| bullet.health == 0)
            .map(|(bullet_id, _)| *bullet_id)
            .collect();

        for bullet_id in bullets_to_remove {
            world.bullets.remove(&bullet_id);
        }
    }
}
