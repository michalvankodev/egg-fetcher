use bevy::prelude::*;

use crate::{
    resources::prelude::*,
    ui::{Housing, Overlay, SimpleText},
};

use super::{CollectedCoins, LevelTimer};

#[derive(Component)]
pub struct ScopedMarker;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct TimeText;

fn spawn_camera(commands: &mut Commands) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(ScopedMarker);
}

pub fn spawn(commands: &mut Commands, fonts: &Fonts) {
    let font = &fonts.fredoka;
    let overlay = Overlay::new();
    let mut top = Housing::percent(100.0, 100.0);
    top.align_items(AlignItems::FlexEnd)
        .flex_direction(FlexDirection::Row)
        .justify_content(JustifyContent::SpaceBetween);

    let mut loading_text = SimpleText::big("Try to collect as many coins as possible", font);
    let mut score_text = SimpleText::big("Score", font);
    let mut time_text = SimpleText::big("TODO time", font);

    loading_text.color(Colors::PRIMARY);
    score_text.color(Colors::PRIMARY);
    time_text.color(Colors::PRIMARY);

    overlay.spawn(
        commands,
        |parent| {
            top.spawn(parent, |parent| {
                parent.spawn_bundle(score_text.bundle).insert(ScoreText);
                loading_text.spawn(parent);
                parent.spawn_bundle(time_text.bundle).insert(TimeText);
            });
        },
        ScopedMarker,
    );

    spawn_camera(commands);
}

pub fn update_score_system(
    score: Res<CollectedCoins>,
    mut text: Query<&mut Text, With<ScoreText>>,
) {
    let mut score_text = text.single_mut();
    score_text.sections[0].value = "Score: ".to_owned() + &score.0.to_string();
}

pub fn update_time_system(
    mut level_timer: ResMut<LevelTimer>,
    mut text: Query<&mut Text, With<TimeText>>,
    time: Res<Time>,
) {
    level_timer.timer.tick(time.delta());
    let time_remaining =
        level_timer.timer.duration().as_secs_f32() - level_timer.timer.elapsed_secs();

    let mut timer_text = text.single_mut();
    timer_text.sections[0].value = "Time left: ".to_owned() + &time_remaining.floor().to_string();
}
