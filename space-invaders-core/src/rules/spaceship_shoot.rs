#![allow(implied_bounds_entailment)]

use crate::{World, Tick, Changes, BulletId, Bullet, Position, Velocity, MoveBulletRule};

use super::Rule;

pub struct SpaceshipShootRule;
impl Rule for SpaceshipShootRule {
    fn should_apply(&self, tick: &Tick) -> bool {
        tick.0 % 2 == 0
    }

    fn apply(&mut self, world: &mut World, _tick: &Tick) -> (Option<Vec<Changes>>, Option<Vec<Box<dyn Rule>>>, bool) {
        world.bullet_count += 1;
        let bullet_id = BulletId(world.bullet_count);
        world.bullets.insert(
            bullet_id,
            Bullet {
                position: Position {
                    x: world.spaceship.position.x + world.spaceship.dimension.width / 2,
                    y: world.spaceship.position.y + world.spaceship.dimension.height,
                },
                health: 1,
                velocity: Velocity { x: 0, y: 1 },
            },
        );

        (Some(vec![Changes::SpaceshipShoot(bullet_id)]), Some(
            vec![
                Box::new(MoveBulletRule { bullet_id })
            ]
        ), true)
    }
}