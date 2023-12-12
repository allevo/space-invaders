#![allow(implied_bounds_entailment)]

use crate::{
    world::{BulletId, World},
    Changes, Tick, Effects,
};

use super::Rule;

pub struct MoveBulletRule {
    pub bullet_id: BulletId,
}
impl Rule for MoveBulletRule {
    fn apply(&mut self, world: &mut World, _tick: &Tick, effects: &mut Effects) -> bool {
        let bullet = match world.bullets.get_mut(&self.bullet_id) {
            Some(bullet) => bullet,
            // This rule contains a reference to a bullet that doesn't exist anymore.
            // This is ok, we just need to remove this rule from the rules list.
            None => return true,
        };
        bullet.position.x = ((bullet.position.x as i32) + bullet.velocity.x) as u32;
        bullet.position.y = ((bullet.position.y as i32) + bullet.velocity.y) as u32;

        effects.changes.insert(Changes::BulletMoved(self.bullet_id));

        false
    }
}
