use std::collections::HashMap;

use crate::*;

pub fn level1() -> (World, Game, TickGenerator) {
    let mut world = World {
        map: Map {
            width: 150,
            height: 300,
        },
        enemies: Vec::new(),
        spaceship: Spaceship {
            position: Position { x: 75, y: 0 },
            dimension: Dimension {
                width: 1,
                height: 1,
            },
            health: 100,
            gun: Gun {},
        },
        bullets: HashMap::new(),
        bullet_count: 0,
    };
    world.enemies.push(Enemy {
        id: EnemyId(0),
        position: Position { x: 75, y: 299 },
        dimension: Dimension {
            width: 1,
            height: 1,
        },
        health: 1,
        gun: Gun {},
    });

    struct R {}
    impl RandomInRange for R {
        fn random_boolean(&mut self, _min: u32, _max: u32) -> Option<u32> {
            Some(0)
        }
    }

    let game = Game {
        rules: vec![
            /*
            Box::new(MoveEnemiesRule {
                direction: Direction::Right,
            }),
            Box::new(EnemiesFireBulletsRule {
                random_boolean: Box::new(R {}),
            }),
            */
            Box::new(OutOfMapBulletsRule {}),
            Box::new(BulletsDeadRule {}),
            Box::new(BulletsHitEnemiesRule {}),
        ],
    };

    (world, game, TickGenerator::new())
}
