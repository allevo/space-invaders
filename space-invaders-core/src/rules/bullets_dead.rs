#![allow(implied_bounds_entailment)]

use crate::{
    world::{BulletId, World},
    Changes, Tick, Effects,
};

use super::Rule;

pub struct BulletsDeadRule {}
impl Rule for BulletsDeadRule {
    fn apply(&mut self, world: &mut World, _tick: &Tick, effects: &mut Effects) -> bool {
        let bullets_to_remove: Vec<BulletId> = world
            .bullets
            .iter()
            .filter(|(_, bullet)| bullet.health == 0)
            .map(|(bullet_id, _)| *bullet_id)
            .collect();

        for bullet_id in &bullets_to_remove {
            world.bullets.remove(bullet_id);
        }

        if bullets_to_remove.is_empty() {
            return false
        }

        effects.changes.insert(Changes::BulletsDead(bullets_to_remove));

        false
    }
}
