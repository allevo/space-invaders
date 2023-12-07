use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use space_invaders_core::{Game, World};

#[derive(Resource)]
struct SpaceInvadersResource {
    pub world: World,
    pub game: Game,
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn run_tick(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    mut space_invaders: ResMut<SpaceInvadersResource>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let space_invaders = &mut *space_invaders;
        space_invaders.game.tick(&mut space_invaders.world);
    }
}

fn main() {
    let (world, game) = space_invaders_core::level1();

    App::new()
        .insert_resource(GreetTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .insert_resource(SpaceInvadersResource { world, game })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, run_tick)
        .run();
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    space_invaders: Res<SpaceInvadersResource>,
) {
    commands.spawn(Camera2dBundle::default());

    // Rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(space_invaders.world.map.width as f32, space_invaders.world.map.height as f32)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("spaceship.png"),
        transform: Transform::from_translation(to_bevy_coords(&space_invaders.world, &space_invaders.world.spaceship.position)),
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("bullet.png"),
        transform: Transform::from_translation(to_bevy_coords(&space_invaders.world, &space_invaders_core::Position { x: 0, y: 0 })),
        ..default()
    });

    let enemy_texture_handle = asset_server.load("enemy.png");
    for enemy in &space_invaders.world.enemies {
        commands.spawn(SpriteBundle {
            texture: enemy_texture_handle.clone(),
            transform: Transform::from_translation(to_bevy_coords(&space_invaders.world, &enemy.position)),
            ..default()
        });
    }
}

fn to_bevy_coords(
    world: &space_invaders_core::World,
    position: &space_invaders_core::Position,
) -> Vec3 {
    let x = position.x as f32 - (world.map.width as f32 / 2.0);
    let y = position.y as f32 - (world.map.height as f32 / 2.0);

    Vec3::new(x, y, 0.)
}