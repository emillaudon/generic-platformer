use bevy::prelude::*;

use crate::TILE_SIZE;

pub struct AsciiPlugin;

pub struct AsciiSheet(Handle<TextureAtlas>);

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_stage_sprite);
    }
}

pub fn get_zero(ascii: Res<'_, AsciiSheet>) -> Handle<TextureAtlas> {
    ascii.0.clone()
}

pub fn spawn_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    index: usize,
    color: Color,
    translation: Vec3
) -> Entity { 
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = color;
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE * 3.0));

    commands.spawn_bundle(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: ascii.0.clone(),
        transform: Transform {
            translation: translation,
            ..Default::default()
        },
        ..Default::default()
    }).id()
}

fn load_stage_sprite(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
        let image = assets.load("ground.png");
        let atlas = TextureAtlas::from_grid_with_padding(
            image,
            Vec2::splat(32.0),
            2,
            2,
            Vec2::splat(2.0)
        );

        let atlas_handle = texture_atlases.add(atlas);

        for f in texture_atlases.iter() {
            println!("{}", 1);
        }
        commands.insert_resource(AsciiSheet(atlas_handle));
}