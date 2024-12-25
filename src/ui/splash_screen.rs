use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy_hui::prelude::{HtmlFunctions, HtmlNode, HtmlStyle};
use crate::manager::AppState;
use crate::ui::{destroy_screen, handle_delay_step, handle_fade_in_step, handle_fade_out_step, play_life_cycle, ScreenState};

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScreenState {
            timer: Stopwatch::new(),
            step: 0,
        });
        app.add_systems(OnEnter(AppState::SplashScreen), create_screen);
        app.add_systems(Update, update_screen.run_if(in_state(AppState::SplashScreen)));
        app.add_systems(OnExit(AppState::SplashScreen), destroy_screen);
    }
}

fn create_screen(mut commands: Commands,
                 asset_server: Res<AssetServer>,
                 mut html_functions: HtmlFunctions
) {
    commands.spawn(HtmlNode(asset_server.load("ui/splash-screen.xml")));

    html_functions.register("play_life_cycle", play_life_cycle);
    html_functions.register("set_name_con_1", set_name_con_1);
    html_functions.register("set_name_con_2", set_name_con_2);
}

fn update_screen(
    mut screen_state: ResMut<ScreenState>,
    mut query: Query<(&mut BackgroundColor, &Name), With<Name>>,
    mut display_query: Query<(&mut HtmlStyle, &Name), With<Name>>,
    mut next_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
) {
    screen_state.timer.tick(time.delta());

    match screen_state.step {
        0 => handle_delay_step(&mut screen_state, 1, 2.0),
        1 => handle_fade_out_step(&mut screen_state, &mut query, time, 2),
        2 => handle_delay_step(&mut screen_state, 3, 2.0),
        3 => handle_fade_in_step(&mut screen_state, &mut query, time, 4),
        4 => handle_step_4(&mut screen_state, &mut display_query),
        5 => handle_fade_out_step(&mut screen_state, &mut query, time, 6),
        6 => handle_delay_step(&mut screen_state, 7, 2.0),
        7 => handle_fade_in_step(&mut screen_state, &mut query, time, 8),
        8 => handle_step_8(&mut screen_state, &mut next_state),
        _ => {}
    }
}

fn handle_step_4(
    screen_state: &mut ResMut<ScreenState>,
    display_query: &mut Query<(&mut HtmlStyle, &Name), With<Name>>,
) {
    if screen_state.timer.elapsed_secs() >= 2.0 {
        for (mut style, name) in display_query.iter_mut() {
            if name.as_str() == "Container-1" {
                style.computed.node.display = Display::None;
                continue;
            }

            if name.as_str() == "Container-2" {
                style.computed.node.display = Display::Flex;
            }
        }

        screen_state.step = 5;
        screen_state.timer.reset();
    }
}

fn handle_step_8(
    screen_state: &mut ResMut<ScreenState>,
    next_state: &mut ResMut<NextState<AppState>>,
) {
    if screen_state.timer.elapsed_secs() >= 3.0 {
        screen_state.step = 0;
        screen_state.timer.reset();
        next_state.set(AppState::TitleScreen);
    }
}

fn set_name_con_1(In(entity): In<Entity>,
                  mut commands: Commands
) {
    commands.entity(entity).insert(Name::new("Container-1"));
}

fn set_name_con_2(In(entity): In<Entity>,
                  mut commands: Commands
) {
    commands.entity(entity).insert(Name::new("Container-2"));
}