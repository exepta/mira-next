use bevy::prelude::*;
use bevy_hui::prelude::{HtmlFunctions, HtmlNode, HtmlStyle};
use crate::services::loading_service::LoadingState;

pub struct LoadingScreenPlugin;

#[derive(Component)]
struct LoadingScreen;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_screen);
        app.add_systems(Update, update_screen);
    }
}

fn create_screen(mut commands: Commands, asset_server: Res<AssetServer>, mut html_functions: HtmlFunctions) {
    commands.spawn(HtmlNode(asset_server.load("ui/loading-screen.xml")));

    html_functions.register("setup_screen", setup_screen);
}

fn update_screen(loading_state: Res<LoadingState>,
                 mut display_query: Query<&mut HtmlStyle, With<LoadingScreen>>) {
    for mut style in display_query.iter_mut() {
        match loading_state.as_ref() {
            LoadingState::LevelReady => {
                style.computed.node.display = Display::None;
            },
            LoadingState::LevelLoading => {
                style.computed.node.display = Display::Flex;
            }
        }
    }
}

fn setup_screen(In(entity): In<Entity>,
                mut commands: Commands) {
    commands.entity(entity).insert(LoadingScreen)
        .insert(Name::new("LoadingScreen")).insert(ZIndex(2));
}