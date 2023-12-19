use crate::{Bullet, BulletId, Changes, Effects, MoveBulletRule, Position, Tick, Velocity, World};

use super::Rule;

pub struct SpaceshipShootRule {
    pub shooting: bool,
    pub last_shoot_tick_time: i32,
    pub shoot_tick_period: i32,
}
impl Rule for SpaceshipShootRule {
    fn should_apply(&self, tick: &Tick) -> bool {
        tick.0 % 2 == 0
    }

    fn apply(&mut self, world: &mut World, tick: &Tick, effects: &mut Effects) -> bool {
        if !self.shooting {
            println!("not shooting");
            return false;
        }

        if self.last_shoot_tick_time + self.shoot_tick_period > tick.0 as i32 {
            println!("too early to shoot");
            self.shooting = false;
            return false;
        }

        println!("shooting");

        self.last_shoot_tick_time = tick.0 as i32;
        self.shooting = false;

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
                velocity: Velocity { x: 0, y: 10 },
            },
        );

        effects.changes.insert(Changes::SpaceshipShoot(bullet_id));
        effects
            .new_rules
            .push(Box::new(MoveBulletRule { bullet_id }));

        false
    }
}
