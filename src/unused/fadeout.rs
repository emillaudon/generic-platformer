use bevy::prelude::*;

use crate::{GameState, ascii::{AsciiSheet, get_zero}};

pub struct FadeoutPlugin;

#[derive(Component)]
pub struct ScreenFade {
    alpha: f32,
    sent: bool,
    next_state: GameState,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct FadeoutTimer {
    timer: Timer,
}

impl Plugin for FadeoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(fadeout);
    }
}

fn fadeout(
    mut commands: Commands,
    mut fade_query: Query<(Entity, &mut ScreenFade, &mut FadeoutTimer, &mut TextureAtlasSprite)>,
    mut state: ResMut<State<GameState>>,
    time: Res<Time>,
) {
    for(entity, mut fade, mut fadeout_timer, mut sprite) in fade_query.iter_mut() {
        fadeout_timer.timer.tick(time.delta());
        if fadeout_timer.timer.percent() < 0.5 {
            fade.alpha = fadeout_timer.timer.percent() * 2.0;
        } else {
            fade.alpha = fadeout_timer.timer.percent_left() * 2.0;
        }
        sprite.color.set_a(fade.alpha);

        if fadeout_timer.timer.percent() > 0.5 && !fade.sent {
            state.set(fade.next_state).unwrap();
            fade.sent = true;
        }
        if fadeout_timer.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn create_fadeout(commands: &mut Commands, next_state: GameState, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.color = Color::rgba(0.1, 0.1, 0.15, 0.0);
    sprite.custom_size = Some(Vec2::splat(1000000.0));

    commands.spawn_bundle(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: get_zero(ascii),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 999.0),
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(FadeoutTimer {
            timer: Timer::from_seconds(1.0, false)
        })
        .insert(ScreenFade {
            alpha: 0.0,
            sent: false,
            next_state: next_state
        })
            .insert(Name::new("Fadeout"));
}