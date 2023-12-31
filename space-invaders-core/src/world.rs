use std::collections::HashMap;

#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
pub struct BulletId(pub u32);

#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
pub struct EnemyId(pub u32);

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

pub struct Map {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct Gun {}

#[derive(Debug)]
pub struct Dimension {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct Enemy {
    pub id: EnemyId,
    pub position: Position,
    pub dimension: Dimension,
    pub velocity: Velocity,
    pub health: u32,
    pub gun: Gun,
}

pub struct Spaceship {
    pub position: Position,
    pub dimension: Dimension,
    pub health: u32,
    pub gun: Gun,
}

pub struct Bullet {
    pub position: Position,
    pub velocity: Velocity,
    pub health: u32,
}

pub struct World {
    pub map: Map,
    pub enemies: HashMap<EnemyId, Enemy>,
    pub spaceship: Spaceship,
    pub bullets: HashMap<BulletId, Bullet>,
    pub bullet_count: u32,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            map: Map {
                width: 100,
                height: 100,
            },
            enemies: Default::default(),
            spaceship: Spaceship {
                position: Position { x: 50, y: 0 },
                dimension: Dimension {
                    width: 1,
                    height: 1,
                },
                health: 100,
                gun: Gun {},
            },
            bullets: HashMap::new(),
            bullet_count: 0,
        }
    }
}
