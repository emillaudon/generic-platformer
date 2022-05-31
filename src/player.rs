use std::time::Duration;

use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::{prelude::*, rapier::prelude::RigidBodyVelocity};
use benimator::*;

use crate::{ spriteloader::{AsciiSheet, spawn_sprite}, TILE_SIZE, tilemap::{TileCollider, EncounterSpawner}, GameState};

pub struct PlayerPlugin;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct EncounterTracker {
    timer: Timer,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct AttackTimer {
    timer: Timer,
}

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
    active: bool,
    just_moved: bool,
    jumping: bool,
    facing_right: bool,
    player_action: PlayerAction,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerBullet {
    speed: f32,
    timer: Timer
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Inspectable)]
pub enum PlayerAction {
    RunningRight,
    RunningLeft,
    Idle,
    Jumping,
    Attacking,
}

#[derive(Component, Default)]
pub struct Animations {
    running: Handle<SpriteSheetAnimation>,
    idle: Handle<SpriteSheetAnimation>,
    jumping: Handle<SpriteSheetAnimation>,
    attacking: Handle<SpriteSheetAnimation>,
}


pub struct PlayerSheet(Handle<TextureAtlas>);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_enter(GameState::Overworld).with_system(show_player))
        .add_system_set(
            SystemSet::on_exit(GameState::Overworld).with_system(hide_player))
        .add_system_set(
            SystemSet::on_update(GameState::Overworld)
                .with_system(player_movement)
                .with_system(cancel_jump)
                .with_system(player_jump)
                .with_system(shooting)
                .with_system(move_bullets)
                .with_system(player_encounter_checking)
                .with_system(camera_follow)
                .with_system(change_animation)
                .with_system(melee_attack)
    )
    .add_startup_system(load_sheet);
    }
}

//Camera
fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y + 0.32;
}

//Player Movement
fn player_movement(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    mut velocities: Query<&mut Velocity>,
    time: Res<Time>
) {
    let (mut player, mut transform) = player_query.single_mut();
    if !player.active {
        return;
    }
        let movement = 1.2;
        let mut y_delta = 0.0;
        let mut x_delta = 0.0;

        if keyboard.pressed(KeyCode::S) {
            for mut vel in velocities.iter_mut() {
                vel.linvel = Vec2::new(0.0, 0.0);
            }

        } else if keyboard.pressed(KeyCode::A) {
            for mut vel in velocities.iter_mut() {
                vel.linvel = Vec2::new(-movement, vel.linvel.y);
                player.facing_right = false;
                transform.rotation = Quat::from_rotation_y(3.0);
                if !player.jumping && player.player_action != PlayerAction::Attacking {
                    player.player_action = PlayerAction::RunningLeft;
                }
            }
            //x_delta -= 0.1 * player.speed * TILE_SIZE * time.delta_seconds();
        } else if keyboard.pressed(KeyCode::D) {
            for mut vel in velocities.iter_mut() {
                vel.linvel = Vec2::new(movement, vel.linvel.y);
                player.facing_right = true;
                if !player.jumping && player.player_action != PlayerAction::Attacking {
                    player.player_action = PlayerAction::RunningRight;
                }
            }

        } else {
            //Idle
            if player.facing_right == false {
                transform.rotation = Quat::from_rotation_y(3.0);
            }
            if !player.jumping && player.player_action != PlayerAction::Attacking {
                player.player_action = PlayerAction::Idle;
            }
        }

        if x_delta != 0.0 || y_delta != 0.0 {
            player.just_moved = true;
        }

        let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);

        transform.translation = target;
}

fn player_jump(
 mut player: Query<&mut Player>,
 keyboard: Res<Input<KeyCode>>,
 mut velocities: Query<&mut Velocity, With<Player>>
) {
    let mut player = player.single_mut();

    if keyboard.pressed(KeyCode::Space) && !player.jumping {
        for mut vel in velocities.iter_mut() {
            player.player_action = PlayerAction::Jumping;
            vel.linvel = Vec2::new(0.0, 2.5);
            player.jumping = true;
        }
    }
}

fn cancel_jump(
    rapier_context: Res<RapierContext>,
    mut player_query: Query<Entity, With<Player>>,
    mut player: Query<&mut Player>,
    mut wall_queries: Query<Entity, With<TileCollider>>
) {
    if let mut entity1 = player_query.single_mut() {
        for entity2 in wall_queries.iter_mut() {
            if let Some(contact_pair) = rapier_context.contact_pair(entity1, entity2) {
                if contact_pair.has_any_active_contacts() {
                    //println!("The entities {:?} and {:?} have intersecting colliders!", entity1, entity2);
                    let mut player = player.single_mut();
                    player.jumping = false;
                }
            }

        }
    }

}

//Melee
fn melee_attack(
    time: Res<Time>,
    mut velocities: Query<&mut Velocity, With<Player>>,
    mut attack_timer_query: Query<&mut AttackTimer>,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    mut player_query: Query<&mut Player>,
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
) {
    let mut player = player_query.single_mut();
    let player_facing_right = player.facing_right;
    let mut attack_timer = attack_timer_query.single_mut();
    
    let player_action = player.player_action;

    if player_facing_right {

    }

    let mut player_transform = player_transform_query.single_mut();

    if player_action == PlayerAction::Attacking {

        attack_timer.timer.tick(time.delta());
        println!("Ticks");

        if attack_timer.timer.just_finished() {
            println!("Done");
            player.player_action = PlayerAction::Idle;
        }
    } else if buttons.just_pressed(MouseButton::Left) {
        if player_action != PlayerAction::Attacking {
            player.player_action = PlayerAction::Attacking;

            let mut velocity = 2.0;
            if !player_facing_right {
                velocity = -velocity
            }

            for mut vel in velocities.iter_mut() {
                vel.linvel = Vec2::new(velocity, 0.0);
            }

            attack_timer.timer.reset();
        } 
    }  
}

//Bullets and bullet movement
fn shooting(
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    player_query: Query<&Player>,
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
) {
    let player = player_query.single();
    let player_facing_right = player.facing_right;

    let mut speed = -2.0;

    if player_facing_right {
        speed = 2.0;
    }

    let mut player_transform = player_transform_query.single_mut();

    if buttons.just_pressed(MouseButton::Right) {
        println!("Fired");
        commands.spawn_bundle(
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.7, 0.1, 0.1),
                    ..Default::default()
                },
                transform: Transform {
                    translation: player_transform.translation,
                    scale: Vec3::new(0.01, 0.01, 0.01),
                    ..Default::default()
                },
                ..Default::default()
            }
        )
        .insert(PlayerBullet {
            speed: speed,
            timer: Timer::from_seconds(2.0, false),
        });
    }
}

fn move_bullets(
    mut commands: Commands,
    time: Res<Time>,
    player_query: Query<&Player>,
    mut bullet_queries: Query<(Entity, &mut PlayerBullet, &mut Transform)>
) {
    let player = player_query.single();
    let player_facing_right = player.facing_right;

    for (ent, mut bullet, mut bullet_transform) in bullet_queries.iter_mut() {
        bullet_transform.translation += Vec3::new(bullet.speed * time.delta_seconds(), 0.0, 0.0);

        if bullet.timer.finished() {
            commands.entity(ent).despawn_recursive();
        } else {
            bullet.timer.tick(time.delta());
        }
    }
}

//Player wall collision
fn wall_collision_check(
    target_player_pos: Vec3,
    wall_translation: Vec3
) -> bool {
    let collision = collide(
        target_player_pos, 
        Vec2::splat(TILE_SIZE * 0.9),
        wall_translation,
        Vec2::splat(TILE_SIZE)
    );

    collision.is_some()
}

//Sprites and animations
fn load_sheet(
    mut handles: ResMut<Animations>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    animations: ResMut<Assets<SpriteSheetAnimation>>) {
        let image = assets.load("character.png");

        let mut atlas = TextureAtlas::from_grid_with_padding(
            image,
            Vec2::splat(32.0),
            2,
            5,
            Vec2::splat(0.1)
        );

        let atlas_handle = texture_atlases.add(atlas);

        for f in texture_atlases.iter() {
            println!("{}", 1);


        }
        commands.insert_resource(PlayerSheet(atlas_handle.clone()));

        spawn_player(handles, commands, &PlayerSheet(atlas_handle), animations);
}

fn change_animation(
    animations: Res<Animations>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    mut player_animation_query: Query<&mut Handle<SpriteSheetAnimation>>,
) {
    let (player, transform) = player_query.single_mut();
    let mut animation = player_animation_query.single_mut();

    let player_action = player.player_action;

    match player_action {
        PlayerAction::RunningRight => *animation = animations.running.clone(),
        PlayerAction::RunningLeft => *animation = animations.running.clone(),
        PlayerAction::Jumping => *animation = animations.jumping.clone(),
        PlayerAction::Idle => *animation = animations.idle.clone(),
        PlayerAction::Attacking => *animation = animations.attacking.clone(),

        _ => *animation = animations.idle.clone()
    }

    if player_action == PlayerAction::RunningRight {
        
    }
}


//Spawning player and loading animations
fn spawn_player(mut handles: ResMut<Animations>, mut commands: Commands, player_sheet: &PlayerSheet, mut animations: ResMut<Assets<SpriteSheetAnimation>>) {
    //Running animation
    let running = animations.add(
        SpriteSheetAnimation::from_range(
        1..=3,
        Duration::from_millis(100),
    ));
    
    //Jumping animation
    let jumping = animations.add(
        SpriteSheetAnimation::from_range(
        4..=4,
        Duration::from_millis(100),
    ));

    //Idle Animation
    let idle = animations.add(
        SpriteSheetAnimation::from_range(
            0..=0,
            Duration::from_millis(100),
        ));

    let attacking = animations.add(
        SpriteSheetAnimation::from_range(
            5..=8,
            Duration::from_millis(50),
        ));

    let animations_handler = Animations {
        running: running.clone(),
        idle: idle.clone(),
        jumping: jumping.clone(),
        attacking: attacking.clone()
    };

    handles.idle = idle.clone();
    handles.running = running.clone();
    handles.jumping = jumping.clone();
    handles.attacking = attacking.clone();

    let attack_timer = AttackTimer {
        timer: Timer::from_seconds(0.150, false)
    };
    
    commands.spawn_bundle(
        SpriteSheetBundle {
            texture_atlas: player_sheet.0.clone(),
            transform: Transform { 
                    scale:  Vec3::new(0.001, 0.0001, 0.001),
                    ..Default::default()
                },
            ..Default::default()
        })
        .insert(handles.idle.clone())
        //.insert(animations_handler)
        .insert(Play)
        .insert(Name::new("Player"))
        .insert(Player {
            just_moved: false,
            active: true,
            speed: 100.0,
            jumping: false,
            facing_right: true,
            player_action: PlayerAction::Idle,
        })
        .insert(attack_timer)
        .insert(EncounterTracker {
            timer: Timer::from_seconds(1.0, true)
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(6.7))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Restitution::coefficient(0.7))
        .insert(GravityScale(0.1))
        .insert(Ccd::enabled())
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0
        })
        .insert(Sleeping::disabled())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Transform {
            scale: Vec3::new(0.01, 0.01, 0.1),
            translation: Vec3::new(0.1, -0.2, 900.0),
            ..Default::default()
        });

}




//Unused
fn show_player(
    mut player_query: Query<(&mut Player, &mut Visibility)>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>, 
) {
    let (mut player, mut player_vis) = player_query.single_mut();
    player_vis.is_visible = true;
    player.active = true;

    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = true;
            }
        }
    }
}

fn hide_player(
    mut player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>, 
) {
    let mut player_vis = player_query.single_mut();
    player_vis.is_visible = false;

    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = false;
            }
        }
    }
}

fn player_encounter_checking(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut EncounterTracker, &Transform)>,
    encounter_query: Query<&mut Transform, (With<EncounterSpawner>, Without<Player>)>,
    mut state: ResMut<State<GameState>>,
    ascii: Res<AsciiSheet>,
    mut time: Res<Time>,
    
) {
    let (mut player, mut encounter_tracker, player_transform) = player_query.single_mut();
    let player_translation = player_transform.translation;
    if player.just_moved && encounter_query
    .iter()
    .any(|&transform| wall_collision_check(player_translation, transform.translation))
    {
        encounter_tracker.timer.tick(time.delta());

        if encounter_tracker.timer.just_finished() {
            player.active = false;
            //create_fadeout(&mut commands, GameState::Combat, ascii);
            println!("Change to combat");
        }
    }
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        
        println!("Received collision event: {:?}", collision_event);
    }
}
