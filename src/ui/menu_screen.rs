use std::time::Duration;
use bevy::prelude::*;
use bevy_hui::prelude::{HtmlFunctions, HtmlNode};
use bevy_kira_audio::{AudioControl, AudioEasing, AudioTween, DynamicAudioChannels};
use crate::manager::{AppState, InGameState, MenuState};
use crate::services::loading_service::LoadingState;
use crate::ui::{destroy_screen, handle_fade_out_step, ScreenState, UiElement};

pub struct MenuScreenPlugin;

impl Plugin for MenuScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu(MenuState::MainUi)), create_screen);
        app.add_systems(Update, update_screen.run_if(in_state(AppState::MainMenu(MenuState::MainUi))));
        app.add_systems(OnEnter(AppState::InGame(InGameState::Playing)), destroy_screen);
    }
}

fn create_screen(mut commands: Commands, asset_server: Res<AssetServer>, mut html_functions: HtmlFunctions) {
    commands.spawn(HtmlNode(asset_server.load("ui/menu-screen.xml")));
    html_functions.register("start_new_game", start_new_game);
}

fn update_screen(mut screen_state: ResMut<ScreenState>,
                 mut background_query: Query<(&mut BackgroundColor, &Name), With<Name>>,
                 time: Res<Time>
) {
    screen_state.timer.tick(time.delta());

    match screen_state.step {
        0 => handle_fade_out_step(&mut screen_state, &mut background_query, time,1),
        _ => { }
    }
}

fn start_new_game(In(_entity): In<Entity>,
                  mut commands: Commands,
                  query: Query<Entity, With<UiElement>>,
                  mut next_state: ResMut<NextState<AppState>>,
                  mut loading_state: ResMut<LoadingState>,
                  audio: Res<DynamicAudioChannels>
) {
    for child_entity in query.iter() {
        commands.entity(child_entity).despawn_recursive();
    }

    next_state.set(AppState::InGame(InGameState::Playing));
    *loading_state = LoadingState::LevelLoading;

    audio.channel("title-audio").stop().fade_out(AudioTween::new(Duration::from_secs(2), AudioEasing::InPowi(2)));
}