use crate::world::{BulletId, World};

use super::Rule;

pub struct MoveBulletRule {
    pub bullet_id: BulletId,
}
impl Rule for MoveBulletRule {
    fn apply(&mut self, world: &mut World) {
        let bullet = match world.bullets.get_mut(&self.bullet_id) {
            Some(bullet) => bullet,
            None => return,
        };
        bullet.position.x += bullet.velocity.x;
        bullet.position.y += bullet.velocity.y;
    }
}
