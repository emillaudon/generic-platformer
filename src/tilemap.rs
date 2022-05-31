use std::{fs::File, io::{BufReader, BufRead}};

use bevy::prelude::*;

use bevy_rapier2d::{prelude::{Collider, RigidBody, Velocity, GravityScale, Ccd, Sleeping, LockedAxes}};

use crate::{spriteloader::{AsciiSheet, spawn_sprite}, TILE_SIZE, GameState};

pub struct TileMapPlugin;

#[derive(Component)]
struct Map;

#[derive(Component)]
pub struct EncounterSpawner;

#[derive(Component)]
pub struct TileCollider;

#[derive(Component)]
pub struct WallCollider;

#[derive(Component)]
pub struct MovingCollider {
    movements_right: i32,
    movements_left: i32,
    timer: Timer
}

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(build_stage)
        .add_system_set(
            SystemSet::on_enter(GameState::Overworld)
                .with_system(show_map)
                .with_system(move_floating_platform)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Overworld)
                .with_system(move_floating_platform)
        )
            .add_system_set(SystemSet::on_exit(GameState::Overworld).with_system(hide_map));
    }
}

//Builds stage using chars from textfile
fn build_stage(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let file = File::open("assets/map.txt").expect("No map file found");
    let mut tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {

                let mut index = 0;

                match char {
                    '#' => index = 0,
                    '#' => index = 1,
                    '|'  => index = 2,
                    '-' => index = 0,

                    _ => index = 3,
                };

                if index != 3 {
                    let tile = spawn_sprite(
                        &mut commands,
                        &ascii,
                        index,
                        Color::rgb(1.0, 1.0, 1.0),
                        Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0)
                    );
                    if char == '*' {
                        commands.entity(tile).insert(EncounterSpawner);
                        
                    }
                    if char == '#' {
                        commands.entity(tile).insert(TileCollider)
                        .insert(RigidBody::Fixed)
                        .insert(Collider::cuboid(0.08, 0.06));
                    }

                    if char == '-' {
                        commands.entity(tile).insert(MovingCollider {
                            movements_right: 0,
                            movements_left: 0,
                            timer: Timer::from_seconds(1.5, true),
                        })
                        .insert(TileCollider)
                        .insert(RigidBody::KinematicPositionBased)
                        .insert(Velocity {
                            linvel: Vec2::new(0.0, 0.0),
                            angvel: 0.0
                        })
                        .insert(GravityScale(0.1))
                        .insert(Ccd::enabled())
                        .insert(Sleeping::disabled())
                        .insert(LockedAxes::ROTATION_LOCKED)
                        .insert(Collider::cuboid(0.08, 0.02));
                    }

                    if char == '|' {
                        commands.entity(tile).insert(WallCollider)
                        .insert(Collider::cuboid(0.05, 0.10));
                    }
                    tiles.push(tile);
                }
            }
        }
    }

    commands
        .spawn()
        .insert(Map)
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles);
}

//Movement for horizontal moving platforms
fn move_floating_platform(
    time: Res<Time>,
    mut transform_query: Query<&mut Transform, With<MovingCollider>>,
    mut query_entity: Query<Entity, With<MovingCollider>>,
    mut query_moving_collider: Query<&mut MovingCollider>,
    mut velocities: Query<&mut Velocity, With<MovingCollider>>
) {
    let speed = 0.3;

    for mut moving_collider in query_moving_collider.iter_mut() {
        moving_collider.timer.tick(time.delta());

        if moving_collider.timer.finished() {
            let mut velocity = 0.0;
            let mut right = false;
            
            if moving_collider.movements_right < 3 {
                right = true;
                moving_collider.movements_right += 1;
                velocity = speed;
            } else if moving_collider.movements_left < 3 {
                right = false;
                moving_collider.movements_left += 1;
                velocity = -speed;
                
                if moving_collider.movements_left == 3 {
                    moving_collider.movements_left = 0;
                    moving_collider.movements_right = 0;
                }
            }

            for mut transform in transform_query.iter_mut() {
                if right {
                    transform.translation.x = transform.translation.x + speed;
                } else {
                    transform.translation.x = transform.translation.x - speed;
                }
            }

            for mut vel in velocities.iter_mut() {
                vel.linvel = Vec2::new(velocity, 10.0);
            }
        }
    }
}

//Unused
fn show_map(
    children_query: Query<&Children, With<Map>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Map>>, 
) {
    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = true;
            }
        }
    }
}

fn hide_map(
    children_query: Query<&Children, With<Map>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Map>>, 
) {
    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = false;
            }
        }
    }
}