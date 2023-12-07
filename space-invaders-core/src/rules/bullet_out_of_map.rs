use crate::world::World;

use super::Rule;

pub struct OutOfMapBulletsRule {}
impl Rule for OutOfMapBulletsRule {
    fn apply(&mut self, world: &mut World) {
        for bullet in world.bullets.values_mut() {
            if bullet.position.y >= world.map.height {
                bullet.health = 0;
            }
        }
    }
}
