#![allow(implied_bounds_entailment)]

use crate::{world::World, Tick, Changes};

use super::Rule;

pub struct BulletsHitEnemiesRule {}
impl Rule for BulletsHitEnemiesRule {
    fn should_apply(&self, tick: &Tick) -> bool {
        tick.0 % 2 == 0
    }

    fn apply(&mut self, world: &mut World, _tick: &Tick) -> (Option<Vec<Changes>>, Option<Vec<Box<dyn Rule>>>, bool) {
        let mut enemies_to_remove = vec![];

        for bullet in world.bullets.values_mut() {
            if bullet.health == 0 {
                continue;
            }

            for enemy in &mut world.enemies {
                let x_range = enemy.position.x..(enemy.position.x + enemy.dimension.width);
                let y_range = enemy.position.y..(enemy.position.y + enemy.dimension.height);

                if x_range.contains(&bullet.position.x) && y_range.contains(&bullet.position.y) {
                    println!("BulletsHitEnemiesRule: a bullet hits the enemy {:?}", enemy.id);
                    bullet.health -= 1;

                    if enemy.health == 0 {
                        enemies_to_remove.push(enemy.id);
                        continue;
                    }
                    enemy.health -= 1;
                }
            }
        }

        if enemies_to_remove.is_empty() {
            return (None, None, false);
        }

        let changes = vec![crate::Changes::EnemiesDead(enemies_to_remove)];

        println!("BulletsHitEnemiesRule: {:?}", changes);

        (Some(changes), None, false)
    }
}
