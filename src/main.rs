#![allow(clippy::redunant_field_names)]
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_rapier2d::prelude::*;
use benimator::{*, AnimationPlugin};
use bevy_kira_audio::{Audio, AudioPlugin};
use bevy_parallax::{
    LayerData, ParallaxCameraComponent, ParallaxMoveEvent, ParallaxPlugin, ParallaxResource,
};

pub const CLEAR: Color = Color::rgb(0.2, 0.2, 0.2);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

mod player;
mod debug;
mod spriteloader;
mod tilemap;

use bevy_rapier2d::{plugin::{RapierPhysicsPlugin, NoUserData}, prelude::{RapierDebugRenderPlugin, Restitution}};
use player::{PlayerPlugin, Animations};
use debug::DebugPlugin;
use spriteloader::AsciiPlugin;
use tilemap::TileMapPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Overworld,
    Combat
}

fn main() {
    let height = 900.0;
    App::new()
    .init_resource::<Animations>()
    .add_state(GameState::Overworld)
    .insert_resource(ClearColor(CLEAR))
    .insert_resource(ParallaxResource {
        layer_data: vec![
            LayerData {
                speed: 0.09,
                path: "bgrounds/city/sky.png".to_string(),
                tile_size: Vec2::new(480.0, 270.0),
                cols: 1,
                rows: 1,
                scale: 0.013,
                z: 0.0,
                ..Default::default()
            },
            LayerData {
                speed: 0.09,
                path: "bgrounds/city/far.png".to_string(),
                tile_size: Vec2::new(480.0, 270.0),
                cols: 1,
                rows: 1,
                scale: 0.013,
                z: 1.0,
                ..Default::default()
            },
            LayerData {
                speed: 0.06,
                path: "bgrounds/city/mid.png".to_string(),
                tile_size: Vec2::new(480.0, 270.0),
                cols: 1,
                rows: 1,
                scale: 0.013,
                z: 2.0,
                ..Default::default()
            },
            LayerData {
                speed: 0.01,
                path: "bgrounds/city/close.png".to_string(),
                tile_size: Vec2::new(480.0, 270.0),
                cols: 1,
                rows: 1,
                scale: 0.013,
                z: 3.0,
                ..Default::default()
            },
        ],
        ..Default::default()
    })
    .insert_resource(WindowDescriptor {
        width: RESOLUTION * height,
        height: height,
        title: "Jumper".to_string(),
        resizable: false,
        .. Default::default()
    }) 
    .add_plugin(ParallaxPlugin)
    .add_plugins(DefaultPlugins)
    .add_startup_system(spawn_camera)
    .add_startup_system(start_background_audio)
    .add_system(move_parallax_system)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0))
    //.add_plugin(RapierDebugRenderPlugin::default())
    .add_plugin(AsciiPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(DebugPlugin)
    .add_plugin(TileMapPlugin)
    .add_plugin(AudioPlugin)
    .add_plugin(AnimationPlugin::default()) 
    .run();
}

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_looped(asset_server.load("sound/bgm.mp3"));
}

pub fn move_parallax_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
) {
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        move_event_writer.send(ParallaxMoveEvent {
            camera_move_speed: 0.01,
        });
    } else if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        move_event_writer.send(ParallaxMoveEvent {
            camera_move_speed: -0.01,
        });
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera).insert(ParallaxCameraComponent);
}