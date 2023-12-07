use crate::world::World;

use super::Rule;

pub struct BulletsHitEnemiesRule {}
impl Rule for BulletsHitEnemiesRule {
    fn apply(&mut self, world: &mut World) {
        for bullet in world.bullets.values_mut() {
            for enemy in &mut world.enemies {
                let x_range = enemy.position.x..(enemy.position.x + enemy.dimension.width);
                let y_range = enemy.position.y..(enemy.position.y + enemy.dimension.height);

                if x_range.contains(&bullet.position.x) && y_range.contains(&bullet.position.y) {
                    bullet.health -= 1;

                    if enemy.health == 0 {
                        continue;
                    }
                    enemy.health -= 1;
                }
            }
        }
    }
}
