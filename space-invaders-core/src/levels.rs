use std::collections::HashMap;

use crate::*;

pub fn level1() -> (World, Game, TickGenerator) {
    let mut world = World {
        map: Map {
            width: 150,
            height: 300,
        },
        enemies: Default::default(),
        spaceship: Spaceship {
            position: Position { x: 75, y: 0 },
            dimension: Dimension {
                width: 16,
                height: 16,
            },
            health: 100,
            gun: Gun {},
        },
        bullets: HashMap::new(),
        bullet_count: 0,
    };
    world.enemies.insert(EnemyId(0), Enemy {
        id: EnemyId(0),
        position: Position { x: 75, y: 299 },
        dimension: Dimension {
            width: 16,
            height: 16,
        },
        velocity: Velocity { x: 5, y: 0 },
        health: 1,
        gun: Gun {},
    });

    struct R {
        should_shoot_numbers: [bool; 500],
        should_shoot_index: usize,
        enemy_id_index_numbers: [usize; 500],
        enemy_id_index_index: usize,
    }
    impl R {
        fn new() -> Self {
            Self {
                should_shoot_numbers: rand::random(),
                should_shoot_index: 0,
                enemy_id_index_numbers: rand::random(),
                enemy_id_index_index: 0,
            }
        }
    }
    impl RandomInRange for R {
        fn random_boolean(&mut self, ids: Vec<EnemyId>) -> Option<EnemyId> {
            let should_shoot = self.should_shoot_numbers[self.should_shoot_index];
            self.should_shoot_index = (self.should_shoot_index + 1) % self.should_shoot_numbers.len();
            if should_shoot {
                let enemy_id_index = self.enemy_id_index_numbers[self.enemy_id_index_index];
                self.enemy_id_index_index = (self.enemy_id_index_index + 1) % self.enemy_id_index_numbers.len();
                ids.get(enemy_id_index).copied()
            } else {
                None
            }
        }
    }

    let game = Game {
        rules: vec![
            Box::new(MoveEnemiesRule {
                low_bound: 20,
                high_bound: 130,
            }),
            Box::new(EnemiesFireBulletsRule {
                random_boolean: Box::new(R::new()),
            }),
            Box::new(OutOfMapBulletsRule {}),
            Box::new(BulletsDeadRule {}),
            Box::new(BulletsHitEnemiesRule {}),
        ],
    };

    (world, game, TickGenerator::new(30))
}
