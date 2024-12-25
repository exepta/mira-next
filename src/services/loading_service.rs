use bevy::prelude::*;
use crate::services::loading_service::pipelines_ready::{PipelinesReady, PipelinesReadyPlugin};

pub struct LoadingServicePlugin;

// A `Resource` that holds the current loading state.
#[derive(Resource, Default)]
pub enum LoadingState {
    #[default]
    LevelReady,
    LevelLoading,
}

// A resource that holds the current loading data.
#[derive(Resource, Debug, Default)]
pub struct LoadingData {
    // This will hold the currently unloaded/loading assets.
    pub loading_assets: Vec<UntypedHandle>,
    // Number of frames that everything needs to be ready for.
    // This is to prevent going into the fully loaded state in instances
    // where there might be a some frames between certain loading/pipelines action.
    pub confirmation_frames_target: usize,
    // Current number of confirmation frames.
    pub confirmation_frames_count: usize,
}

impl LoadingData {
    fn new(confirmation_frames_target: usize) -> Self {
        Self {
            loading_assets: Vec::new(),
            confirmation_frames_target,
            confirmation_frames_count: 0,
        }
    }
}

impl Plugin for LoadingServicePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PipelinesReadyPlugin);
        app.insert_resource(LoadingState::default());
        app.insert_resource(LoadingData::new(5));

        app.add_systems(Update, update_loading_data);
    }
}

fn update_loading_data(mut loading_data: ResMut<LoadingData>,
                       mut loading_state: ResMut<LoadingState>,
                       asset_server: Res<AssetServer>,
                       pipelines_ready: Res<PipelinesReady>,
) {
    if !loading_data.loading_assets.is_empty() || !pipelines_ready.0 {
        loading_data.confirmation_frames_count = 0;

        let mut pop_list: Vec<usize> = Vec::new();
        for (index, asset) in loading_data.loading_assets.iter().enumerate() {
            if let Some(state) = asset_server.get_load_states(asset) {
                if state.2.is_loaded() {
                    pop_list.push(index);
                }
            }
        }

        for index in pop_list.iter() {
            loading_data.loading_assets.remove(*index);
        }
    } else {
        loading_data.confirmation_frames_count += 1;
        if loading_data.confirmation_frames_count == loading_data.confirmation_frames_target {
            *loading_state = LoadingState::LevelReady;
        }
    }
}

mod pipelines_ready {
    use bevy::{
        prelude::*,
        render::{render_resource::*, *}
    };

    pub struct PipelinesReadyPlugin;

    #[derive(Resource, Debug, Default)]
    pub struct PipelinesReady(pub bool);

    impl Plugin for PipelinesReadyPlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(PipelinesReady::default());

            app.sub_app_mut(RenderApp)
                .add_systems(ExtractSchedule, update_pipelines_ready);
        }
    }

    fn update_pipelines_ready(mut main_world: ResMut<MainWorld>, pipelines: Res<PipelineCache>) {
        if let Some(mut pipelines_ready) = main_world.get_resource_mut::<PipelinesReady>() {
            pipelines_ready.0 = pipelines.waiting_pipelines().count() == 0;
        }
    }
}