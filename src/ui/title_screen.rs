use std::time::Duration;
use bevy::prelude::*;
use bevy_hui::prelude::{HtmlFunctions, HtmlNode, HtmlStyle};
use bevy_kira_audio::{Audio, AudioControl, AudioEasing, AudioTween};
use bevy_kira_audio::prelude::Volume;
use crate::manager::AppState;
use crate::ui::{destroy_screen, handle_delay_step, handle_fade_out_step, play_life_cycle, ScreenState};

#[derive(Component, Resource)]
struct TitleScreenState {
    timer: Timer,
    direction: f32,
}

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TitleScreenState {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            direction: 1.0
        });
        app.add_systems(OnEnter(AppState::TitleScreen), create_screen);
        app.add_systems(Update, update_screen.run_if(in_state(AppState::TitleScreen)));
        app.add_systems(OnEnter(AppState::TitleScreen), play_title_music);
        app.add_systems(OnExit(AppState::TitleScreen), destroy_screen);
    }
}

fn create_screen(mut commands: Commands, asset_server: Res<AssetServer>, mut html_functions: HtmlFunctions) {
    commands.spawn(HtmlNode(asset_server.load("ui/title-screen.xml")));

    html_functions.register("setup_mira_text", setup_mira_text);
    html_functions.register("play_life_cycle", play_life_cycle);

    commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    }).insert(ZIndex(1))
        .insert(Name::new("Overlay"))
        .insert(BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 1.0)));
}

fn update_screen(mut screen_state: ResMut<ScreenState>,
                 mut text_state: ResMut<TitleScreenState>,
                 mut background_query: Query<(&mut BackgroundColor, &Name), With<Name>>,
                 mut text_query: Query<&mut HtmlStyle, With<Name>>,
                 time: Res<Time>,
                 keyboard: Res<ButtonInput<KeyCode>>,
                 mouse: Res<ButtonInput<MouseButton>>
) {
    if keyboard.any_just_pressed([KeyCode::Enter]) || mouse.any_just_pressed([MouseButton::Left]) {
        info!("Go to MainMenu");
    }

    screen_state.timer.tick(time.delta());
    text_state.timer.tick(time.delta());

    match screen_state.step {
        0 => handle_delay_step(&mut screen_state, 1, 2.0),
        1 => handle_fade_out_step(&mut screen_state, &mut background_query, time,2),
        _ => { }
    }

    let total_duration = text_state.timer.duration().as_secs_f32();
    let elapsed = text_state.timer.elapsed_secs();

    let alpha = 0.5 * (1.0 + (elapsed / total_duration * std::f32::consts::PI * 2.0).sin());

    for mut style in text_query.iter_mut() {
        style.computed.font_color = Color::srgba(1.0, 1.0, 1.0, alpha);
    }

    if elapsed >= total_duration {
        text_state.timer.reset();
    }
}

fn play_title_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("audio/title-music.ogg"))
        .start_from(10.0)
        .loop_from(10.0)
        .loop_until(343.0)
        .fade_in(AudioTween::new(Duration::from_secs(2), AudioEasing::OutPowi(2)))
        .with_volume(Volume::from(0.08))
        .looped();
}

fn setup_mira_text(In(entity): In<Entity>,
                   mut commands: Commands
) {
    commands.entity(entity).insert(Name::new("Text-Mira"));
}