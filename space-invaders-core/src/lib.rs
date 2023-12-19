#![feature(get_mut_unchecked)]

use std::{collections::HashSet, sync::Arc};

mod levels;
mod rules;
mod world;

pub use levels::*;
pub use rules::*;
pub use world::*;

pub struct Tick(pub u32, bool);
impl Tick {
    pub(crate) fn should_play(&self) -> bool {
        self.1
    }
}

pub struct TickGenerator {
    frame: u32,
    count: u32,
}
impl TickGenerator {
    pub fn new(frame: u32) -> Self {
        TickGenerator { frame, count: 0 }
    }

    pub fn tick(&mut self) -> Tick {
        let tick = Tick(self.count, self.count % self.frame == 0);
        self.count += 1;
        tick
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Changes {
    SpaceshipMove(Position),
    SpaceshipShoot(BulletId),
    BulletsDead(Vec<BulletId>),
    EnemiesDead(Vec<EnemyId>),
    NewEnemyBullet(BulletId),
    BulletMoved(BulletId),
    EnemiesMove,
}

pub struct Effects {
    changes: HashSet<Changes>,
    new_rules: Vec<Box<dyn Rule>>,
}

pub struct Game {
    rules: Vec<Box<dyn Rule>>,
    shoot_rule: Option<Arc<SpaceshipShootRule>>,
}
impl Game {
    pub fn tick(&mut self, world: &mut World, tick: Tick) -> HashSet<Changes> {
        let mut indexes_to_remove = Vec::new();

        let mut effects = Effects {
            changes: HashSet::new(),
            new_rules: Vec::new(),
        };

        for (index, rule) in self.rules.iter_mut().enumerate() {
            if !rule.should_apply(&tick) {
                continue;
            }

            let to_remove = rule.apply(world, &tick, &mut effects);

            if to_remove {
                indexes_to_remove.push(index);
            }
        }

        self.rules.extend(effects.new_rules);

        for index in indexes_to_remove.into_iter().rev() {
            self.rules.remove(index);
        }

        effects.changes
    }

    pub fn move_spaceship(&mut self, delta: i32) {
        self.rules.push(Box::new(MoveSpaceshipRule { delta }));
    }

    pub fn shoot(&mut self) {
        let shoot_rule = match self.shoot_rule.as_mut() {
            None => {
                println!("shoot rule is None");

                return;
            }
            Some(shoot_rule) => shoot_rule,
        };

        // Safety: bevy runs this method and the tick method not in parallel.
        unsafe {
            Arc::get_mut_unchecked(shoot_rule).shooting = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rules::*;
    use crate::world::*;

    use super::*;

    #[test]
    fn move_enemies() {
        let mut world = World::new();
        world.enemies.insert(
            EnemyId(0),
            Enemy {
                id: EnemyId(0),
                position: Position { x: 0, y: 200 },
                dimension: Dimension {
                    width: 1,
                    height: 1,
                },
                velocity: Velocity { x: 1, y: 0 },
                health: 100,
                gun: Gun {},
            },
        );
        let mut tick_generator = TickGenerator::new(1);

        let mut game = Game {
            rules: vec![Box::new(MoveEnemiesRule {
                low_bound: 0,
                high_bound: 10,
            })],
            shoot_rule: None,
        };

        assert_eq!(
            world.enemies[&EnemyId(0)].position,
            Position { x: 0, y: 200 }
        );
        game.tick(&mut world, tick_generator.tick());
        assert_eq!(
            world.enemies[&EnemyId(0)].position,
            Position { x: 1, y: 200 }
        );
        for _ in 0..9 {
            game.tick(&mut world, tick_generator.tick());
        }
        assert_eq!(
            world.enemies[&EnemyId(0)].position,
            Position { x: 10, y: 199 }
        );

        game.tick(&mut world, tick_generator.tick());
        assert_eq!(
            world.enemies[&EnemyId(0)].position,
            Position { x: 9, y: 199 }
        );

        for _ in 0..9 {
            game.tick(&mut world, tick_generator.tick());
        }
        assert_eq!(
            world.enemies[&EnemyId(0)].position,
            Position { x: 0, y: 198 }
        );

        game.tick(&mut world, tick_generator.tick());
        assert_eq!(
            world.enemies[&EnemyId(0)].position,
            Position { x: 1, y: 198 }
        );
    }

    #[test]
    fn enemies_fire_bulltes() {
        struct R {}
        impl RandomInRange for R {
            fn random_boolean(&mut self, ids: Vec<EnemyId>) -> Option<EnemyId> {
                ids.first().copied()
            }
        }

        let mut world = World::new();
        world.enemies.insert(
            EnemyId(0),
            Enemy {
                id: EnemyId(0),
                position: Position { x: 0, y: 0 },
                dimension: Dimension {
                    width: 1,
                    height: 1,
                },
                velocity: Velocity { x: 0, y: 0 },
                health: 100,
                gun: Gun {},
            },
        );
        let mut tick_generator = TickGenerator::new(1);

        let mut game = Game {
            rules: vec![Box::new(EnemiesFireBulletsRule {
                random_boolean: Box::new(R {}),
            })],
            shoot_rule: None,
        };

        game.tick(&mut world, tick_generator.tick());

        assert_eq!(world.bullets.len(), 1);
        assert_eq!(
            world.bullets[&BulletId(1)].position,
            Position { x: 0, y: 0 }
        );
    }

    #[test]
    fn bullets_move() {
        let mut world = World::new();
        world.bullets.insert(
            BulletId(0),
            Bullet {
                position: Position { x: 0, y: 0 },
                health: 1,
                velocity: Velocity { x: 0, y: 1 },
            },
        );
        let mut tick_generator = TickGenerator::new(1);

        let mut game = Game {
            rules: vec![Box::new(MoveBulletRule {
                bullet_id: BulletId(0),
            })],
            shoot_rule: None,
        };

        game.tick(&mut world, tick_generator.tick());

        assert_eq!(
            world.bullets[&BulletId(0)].position,
            Position { x: 0, y: 1 }
        );
    }

    #[test]
    fn check_collisions() {
        let mut world = World::new();
        world.bullets.insert(
            BulletId(0),
            Bullet {
                position: Position { x: 0, y: 0 },
                health: 1,
                velocity: Velocity { x: 0, y: 1 },
            },
        );
        world.enemies.insert(
            EnemyId(0),
            Enemy {
                id: EnemyId(0),
                position: Position { x: 0, y: 4 },
                dimension: Dimension {
                    width: 1,
                    height: 1,
                },
                velocity: Velocity { x: 0, y: 0 },
                health: 1,
                gun: Gun {},
            },
        );
        let mut tick_generator = TickGenerator::new(1);

        let mut game = Game {
            rules: vec![
                Box::new(BulletsHitEnemiesRule {}),
                Box::new(MoveBulletRule {
                    bullet_id: BulletId(0),
                }),
                Box::new(BulletsDeadRule {}),
            ],
            shoot_rule: None,
        };

        game.tick(&mut world, tick_generator.tick());
        assert_eq!(
            world.bullets[&BulletId(0)].position,
            Position { x: 0, y: 1 }
        );
        game.tick(&mut world, tick_generator.tick());
        assert_eq!(
            world.bullets[&BulletId(0)].position,
            Position { x: 0, y: 2 }
        );
        game.tick(&mut world, tick_generator.tick());
        assert_eq!(
            world.bullets[&BulletId(0)].position,
            Position { x: 0, y: 3 }
        );
        game.tick(&mut world, tick_generator.tick());
        assert_eq!(
            world.bullets[&BulletId(0)].position,
            Position { x: 0, y: 4 }
        );
        game.tick(&mut world, tick_generator.tick());
        assert_eq!(world.bullets.len(), 0);
        assert_eq!(world.enemies[&EnemyId(0)].health, 0);
    }

    #[test]
    fn move_spaceship() {
        let mut world = World::new();
        let mut game = Game {
            rules: vec![],
            shoot_rule: None,
        };
        let mut tick_generator = TickGenerator::new(1);

        let spaceship_x = world.spaceship.position.x;
        game.move_spaceship(-1);
        game.tick(&mut world, tick_generator.tick());
        assert_eq!(world.spaceship.position.x, spaceship_x - 1);

        game.move_spaceship(2);
        game.tick(&mut world, tick_generator.tick());
        assert_eq!(world.spaceship.position.x, spaceship_x + 1);

        for _ in 0..2_000 {
            game.move_spaceship(-1);
            game.tick(&mut world, tick_generator.tick());
        }
        assert_eq!(world.spaceship.position.x, 0);

        for _ in 0..2_000 {
            game.move_spaceship(1);
            game.tick(&mut world, tick_generator.tick());

            println!("spaceship_x: {}", world.spaceship.position.x);
        }
        assert_eq!(world.spaceship.position.x, world.map.width - 1);
    }

    #[test]
    fn spaceship_shoots() {
        let mut world = World::new();

        let shoot_rule = SpaceshipShootRule {
            shooting: false,
            last_shoot_tick_time: -1000,
            shoot_tick_period: 50,
        };
        let shoot_rule = Arc::new(shoot_rule);
        let shoot_rule_boxed = Box::new(shoot_rule.clone());
        let mut game = Game {
            rules: vec![shoot_rule_boxed],
            shoot_rule: Some(shoot_rule.clone()),
        };
        let mut tick_generator = TickGenerator::new(1);

        // When the game starts, the spaceship can shoot.
        game.shoot();
        game.tick(&mut world, tick_generator.tick());
        assert_eq!(world.bullets.len(), 1);

        // The spaceship can't shoot again until the shoot_tick_period has passed.
        game.shoot();
        game.tick(&mut world, tick_generator.tick());
        assert_eq!(world.bullets.len(), 1);

        // Wait for a while (shoot_rule.shoot_tick_period ticks).
        for _ in 0..shoot_rule.shoot_tick_period {
            game.tick(&mut world, tick_generator.tick());
        }

        // The spaceship can shoot again.
        game.shoot();
        game.tick(&mut world, tick_generator.tick());
        assert_eq!(world.bullets.len(), 2);
    }

}
