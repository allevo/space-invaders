#![allow(implied_bounds_entailment)]

use crate::{world::World, Tick, Changes, Effects};

use super::Rule;

pub struct BulletsHitEnemiesRule {}
impl Rule for BulletsHitEnemiesRule {
    fn should_apply(&self, tick: &Tick) -> bool {
        tick.0 % 2 == 0
    }

    fn apply(&mut self, world: &mut World, _tick: &Tick, effects: &mut Effects) -> bool {
        let mut enemies_to_remove = vec![];

        for bullet in world.bullets.values_mut() {
            if bullet.health == 0 {
                continue;
            }

            for enemy in world.enemies.values_mut() {
                let x_range = enemy.position.x..(enemy.position.x + enemy.dimension.width);
                let y_range = enemy.position.y..(enemy.position.y + enemy.dimension.height);

                if x_range.contains(&bullet.position.x) && y_range.contains(&bullet.position.y) {
                    println!("BulletsHitEnemiesRule: a bullet hits the enemy {:?}", enemy.id);
                    bullet.health -= 1;

                    if enemy.health > 0 {
                        enemy.health -= 1;
                    }

                    if enemy.health == 0 {
                        enemies_to_remove.push(enemy.id);
                        continue;
                    }
                }
            }
        }

        if enemies_to_remove.is_empty() {
            return false
        }

        effects.changes.insert(Changes::EnemiesDead(enemies_to_remove));

        false
    }
}
