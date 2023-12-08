use bevy::prelude::*;
use space_invaders_core::{Game, TickGenerator, World, Changes, BulletId, Dimension};

#[derive(Resource)]
struct SpaceInvadersResource {
    pub world: World,
    pub game: Game,
    pub tick_generator: TickGenerator,
}

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
struct SpaceshipComponent;

#[derive(Component)]
struct BulletComponent {
    id: BulletId,
}

const ENEMY_Z : f32 = 1.0;
const SPACESHIP_Z: f32 = 1.0;
const BULLET_Z: f32 = 2.0;

fn run_tick(
    mut commands: Commands,
    mut space_invaders: ResMut<SpaceInvadersResource>,
    mut spaceship: Query<&mut Transform, (With<SpaceshipComponent>, Without<BulletComponent>)>,
    mut bullets: Query<(Entity, &BulletComponent, &mut Transform), Without<SpaceshipComponent>>,
    asset_server: Res<AssetServer>,
) {
    let space_invaders = &mut *space_invaders;
    let changes = space_invaders.game.tick(&mut space_invaders.world, space_invaders.tick_generator.tick());

    for change in changes {
        println!("change: {:?}", change);
        match change {
            Changes::SpaceshipMove(new_spaceship_position) => {
                let mut spaship_transform = spaceship.get_single_mut().expect("Spaceship has to exist");
                spaship_transform.translation = to_bevy_coords(
                    &space_invaders.world,
                    &new_spaceship_position, 
                    &space_invaders.world.spaceship.dimension,
                    SPACESHIP_Z
                );
            }
            Changes::BulletsDead(bullet_ids) => {
                for (entity, bullet, _) in bullets.iter() {
                    if bullet_ids.contains(&bullet.id) {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
            Changes::SpaceshipShoot(bullet_id) => {
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("bullet.png"),
                        transform: Transform::from_translation(to_bevy_coords(
                            &space_invaders.world,
                            &space_invaders.world.bullets[&bullet_id].position,
                            &Dimension { width: 1, height: 1 },
                            BULLET_Z,
                        )),
                        ..default()
                    },
                    BulletComponent { id: bullet_id },
                ));
            }
            Changes::BulletMoved(bullet_id) => {
                let bullet = &space_invaders.world.bullets[&bullet_id];
                let (_, _, mut bullet_transform) = bullets
                    .iter_mut()
                    .find(|(_, bullet, _)| bullet.id == bullet_id)
                    .expect("Bullet has to exist");
                bullet_transform.translation = to_bevy_coords(
                    &space_invaders.world,
                    &bullet.position,
                    &Dimension { width: 1, height: 1 },
                    BULLET_Z
                );
            }
            _ => {}
        };
    }
}

fn handle_spaceship_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut space_invaders: ResMut<SpaceInvadersResource>,
) {
    let delta = keyboard_input
        .get_pressed()
        .filter_map(|key| match key {
            KeyCode::Left => Some(-1),
            KeyCode::Right => Some(1),
            _ => None,
        })
        .sum::<i32>();
    if delta == 0 {
        return;
    }

    let space_invaders = &mut *space_invaders;
    space_invaders
        .game
        .move_spaceship(delta);
}

fn handle_spaceship_shot(
    keyboard_input: Res<Input<KeyCode>>,
    mut space_invaders: ResMut<SpaceInvadersResource>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    let space_invaders = &mut *space_invaders;
    space_invaders.game.shoot();
}

fn main() {
    let (world, game, tick_generator) = space_invaders_core::level1();

    App::new()
        .insert_resource(GreetTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .insert_resource(SpaceInvadersResource {
            world,
            game,
            tick_generator,
        })
        .add_plugins((
            DefaultPlugins,
            bevy_framepace::FramepacePlugin,
            bevy_framepace::debug::DiagnosticsPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Startup, set_framepace)
        .add_systems(Update, run_tick)
        .add_systems(Update, handle_spaceship_movement)
        .add_systems(Update, handle_spaceship_shot)
        .run();
}

fn set_framepace(mut framepace: ResMut<bevy_framepace::FramepaceSettings>) {
    framepace.limiter = bevy_framepace::Limiter::from_framerate(30.0)
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    space_invaders: Res<SpaceInvadersResource>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("spaceship.png"),
            transform: Transform::from_translation(to_bevy_coords(
                &space_invaders.world,
                &space_invaders.world.spaceship.position,
                &space_invaders.world.spaceship.dimension,
                SPACESHIP_Z,
            )),
            ..default()
        },
        SpaceshipComponent,
    ));

    let enemy_texture_handle = asset_server.load("enemy.png");
    for enemy in &space_invaders.world.enemies {
        commands.spawn(SpriteBundle {
            texture: enemy_texture_handle.clone(),
            transform: Transform::from_translation(to_bevy_coords(
                &space_invaders.world,
                &enemy.position,
                &enemy.dimension,
                ENEMY_Z,
            )),
            ..default()
        });
    }
}

fn to_bevy_coords(
    world: &space_invaders_core::World,
    position: &space_invaders_core::Position,
    dimension: &Dimension,
    z: f32,
) -> Vec3 {
    let x = position.x as f32 - (world.map.width as f32 / 2.0) - (dimension.width as f32 / 2.0);
    let y = position.y as f32 - (world.map.height as f32 / 2.0) - (dimension.height as f32 / 2.0);

    Vec3::new(x, y, z)
}
