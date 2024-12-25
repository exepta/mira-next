use bevy::prelude::*;
use bevy_hui::prelude::{HtmlNode};
use crate::manager::{AppState, MenuState};
use crate::ui::{handle_fade_out_step, ScreenState};

pub struct MenuScreenPlugin;

impl Plugin for MenuScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu(MenuState::MainUi)), create_screen);
        app.add_systems(Update, update_screen.run_if(in_state(AppState::MainMenu(MenuState::MainUi))));
    }
}

fn create_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(HtmlNode(asset_server.load("ui/menu-screen.xml")));
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