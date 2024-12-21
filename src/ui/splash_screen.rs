use std::time::Duration;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy::ui::TransparentUi;
use crate::manager::AppState;
use crate::ui::UIElement;

#[derive(Component, Resource)]
struct ScreenState {
    timer: Stopwatch,
    step : usize,
    next_step : Option<f32>
}

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScreenState {
            timer: Stopwatch::new(),
            step: 0,
            next_step: Some(3.0),
        });
        app.add_systems(OnEnter(AppState::SplashScreen), create_screen);
        app.add_systems(Update, update_screen.run_if(in_state(AppState::SplashScreen)));
        app.add_systems(OnExit(AppState::SplashScreen), destroy_screen);
    }
}

fn create_screen(mut commands: Commands, asset_server: Res<AssetServer>) {

    let bevy_logo = asset_server.load("logos/bevy_logo_dark.png");

    commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    })
        .insert(Name::new("SplashScreen"))
        .insert(BackgroundColor(Color::srgb(0.05, 0.05, 0.05)))
        .insert(UIElement)
        .with_children(|parent| {

            // Bevy Logo
            parent.spawn(Node::default())
                .insert(Name::new("Container 1"))
                .insert(UIElement)
                .insert(Visibility::Hidden)
                .with_children(|parent| {
                    parent.spawn(Node {
                        width: Val::Px(600.0),
                        height: Val::Px(175.0),
                        margin: UiRect::new(Val::Px(250.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
                        ..default()
                    })
                        .insert(ImageNode {
                            image: bevy_logo,
                            ..default()
                        })
                        .insert(Name::new("Bevy Logo"));
                });

            // Vogeez
            parent.spawn(Node::default())
                .insert(Name::new("Container 2"))
                .insert(UIElement)
                .insert(Visibility::Hidden)
                .with_children(|parent| {
                    parent.spawn(Text::new("Vogeez Studio"))
                        .insert(Name::new("Vogeez Studio"))
                        .insert(TextColor(Color::WHITE))
                        .insert(TextFont {
                            font_size: 40.0,
                            ..default()
                        });
                });
        });
}

fn update_screen(mut commands: Commands,
                 mut screen_state: ResMut<ScreenState>,
                 mut query: Query<(&mut Visibility, &Name, Entity), With<UIElement>>,
                 mut next_state: ResMut<NextState<AppState>>,
                 keyboard: Res<ButtonInput<KeyCode>>,
                 mouse: Res<ButtonInput<MouseButton>>
) {
    screen_state.timer.tick(Duration::from_secs_f32(1.0 / 60.0));

    if keyboard.just_pressed(KeyCode::Enter) || mouse.just_pressed(MouseButton::Left) {
        screen_state.next_step = Some(0.0);
    }

    if let Some(next) = screen_state.next_step {
        if screen_state.timer.elapsed_secs() >= next {
            screen_state.step += 1;
            screen_state.timer.reset();
            
            match screen_state.step {
                1 => {
                    for (mut vis, name, _) in query.iter_mut() {
                        if name.as_str().eq("Container 1") {
                            *vis = Visibility::Visible;
                        }
                    }
                    screen_state.next_step = Some(4.0);
                }

                2 => {
                    for (mut vis, name, entity) in query.iter_mut() {
                        if name.as_str().eq("Container 1") {
                            commands.entity(entity).despawn_recursive();
                        }
                        if name.as_str().eq("Container 2") {
                            *vis = Visibility::Visible;
                        }
                    }
                    screen_state.next_step = Some(4.0);
                }

                3 => {
                    next_state.set(AppState::TitleScreen);
                    screen_state.next_step = None;
                }

                _ => {}
            }
        }
    }
}

fn destroy_screen(mut commands: Commands, query: Query<Entity, With<UIElement>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}