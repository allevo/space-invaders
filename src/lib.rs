use rules::Rule;
use world::*;

mod rules;
mod world;

pub struct Game {
    rules: Vec<Box<dyn Rule>>,
}
impl Game {
    fn tick(&mut self, world: &mut World) {
        for rule in &mut self.rules {
            rule.apply(world);
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
        world.enemies.push(Enemy {
            position: Position { x: 0, y: 0 },
            dimension: Dimension {
                width: 1,
                height: 1,
            },
            health: 100,
            gun: Gun {},
        });

        let mut game = Game {
            rules: vec![Box::new(MoveEnemiesRule {
                ticks: 0,
                direction: Direction::Right,
            })],
        };

        assert_eq!(world.enemies[0].position, Position { x: 0, y: 0 });
        game.tick(&mut world);
        assert_eq!(world.enemies[0].position, Position { x: 1, y: 0 });
        for _ in 0..9 {
            game.tick(&mut world);
        }
        assert_eq!(world.enemies[0].position, Position { x: 10, y: 0 });
        game.tick(&mut world);
        assert_eq!(world.enemies[0].position, Position { x: 9, y: 1 });
        for _ in 0..9 {
            game.tick(&mut world);
        }
        assert_eq!(world.enemies[0].position, Position { x: 0, y: 1 });
        game.tick(&mut world);
        assert_eq!(world.enemies[0].position, Position { x: 1, y: 2 });
    }

    #[test]
    fn enemies_fire_bulltes() {
        struct R {}
        impl RandomInRange for R {
            fn random_boolean(&mut self, _min: u32, _max: u32) -> Option<u32> {
                Some(0)
            }
        }

        let mut world = World::new();
        world.enemies.push(Enemy {
            position: Position { x: 0, y: 0 },
            dimension: Dimension {
                width: 1,
                height: 1,
            },
            health: 100,
            gun: Gun {},
        });

        let mut game = Game {
            rules: vec![Box::new(RandomlyEnemyFireBulletsRule {
                random_boolean: Box::new(R {}),
            })],
        };

        game.tick(&mut world);

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

        let mut game = Game {
            rules: vec![Box::new(MoveBulletRule {
                bullet_id: BulletId(0),
            })],
        };

        game.tick(&mut world);

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
        world.enemies.push(Enemy {
            position: Position { x: 0, y: 4 },
            dimension: Dimension {
                width: 1,
                height: 1,
            },
            health: 1,
            gun: Gun {},
        });

        let mut game = Game {
            rules: vec![
                Box::new(BulletsHitEnemiesRule {}),
                Box::new(MoveBulletRule {
                    bullet_id: BulletId(0),
                }),
                Box::new(BulletDeadRule {}),
            ],
        };

        game.tick(&mut world);
        assert_eq!(
            world.bullets[&BulletId(0)].position,
            Position { x: 0, y: 1 }
        );
        game.tick(&mut world);
        assert_eq!(
            world.bullets[&BulletId(0)].position,
            Position { x: 0, y: 2 }
        );
        game.tick(&mut world);
        assert_eq!(
            world.bullets[&BulletId(0)].position,
            Position { x: 0, y: 3 }
        );
        game.tick(&mut world);
        assert_eq!(
            world.bullets[&BulletId(0)].position,
            Position { x: 0, y: 4 }
        );
        game.tick(&mut world);
        assert_eq!(world.bullets.len(), 0);
        assert_eq!(world.enemies[0].health, 0);
    }
}
