#![allow(implied_bounds_entailment)]

use crate::{
    world::{BulletId, World},
    Changes, Tick,
};

use super::Rule;

pub struct MoveBulletRule {
    pub bullet_id: BulletId,
}
impl Rule for MoveBulletRule {
    fn should_apply(&self, tick: &Tick) -> bool {
        tick.0 % 2 == 0
    }

    fn apply(&mut self, world: &mut World, _tick: &Tick) -> (Option<Vec<Changes>>, Option<Vec<Box<dyn Rule>>>, bool) {
        let bullet = match world.bullets.get_mut(&self.bullet_id) {
            Some(bullet) => bullet,
            // This rule contains a reference to a bullet that doesn't exist anymore.
            // This is ok, we just need to remove this rule from the rules list.
            None => return (None, None, true),
        };
        bullet.position.x = ((bullet.position.x as i32) + bullet.velocity.x) as u32;
        bullet.position.y = ((bullet.position.y as i32) + bullet.velocity.y) as u32;

        (Some(vec![Changes::BulletMoved(self.bullet_id)]), None, false)
    }
}
