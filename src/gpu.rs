use bevy::app::App;
use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::ecs::error::warn;
use bevy::image::ImagePlugin;
use bevy::prelude::*;
use bevy::render::pipelined_rendering::PipelinedRenderingPlugin;
use bevy::render::renderer::RenderAdapterInfo;
use bevy::window::{ExitCondition, WindowPlugin};
use bevy::winit::WinitPlugin;

pub fn gpu_app() -> App {
    let mut app = App::new();
    app.set_error_handler(warn);
    app.add_plugins(
        DefaultPlugins
            .build()
            .disable::<bevy::audio::AudioPlugin>()
            .disable::<WinitPlugin>()
            .disable::<PipelinedRenderingPlugin>()
            .set(WindowPlugin {
                primary_window: None,
                exit_condition: ExitCondition::DontExit,
                ..default()
            })
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                watch_for_changes_override: Some(false),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );
    app
}

pub fn gpu_app_ready(app: &mut App) {
    app.finish();
    app.cleanup();
}

pub fn is_software_renderer(app: &App) -> bool {
    let Some(render_app) = app.get_sub_app(bevy::render::RenderApp) else {
        return true;
    };
    let Some(info) = render_app.world().get_resource::<RenderAdapterInfo>() else {
        return true;
    };
    let name = info.name.to_lowercase();
    name.contains("llvmpipe")
        || name.contains("lavapipe")
        || name.contains("swiftshader")
        || name.contains("cpu")
}

pub struct GpuBenchConfig {
    pub sample_size: usize,
    pub warm_up_frames: usize,
    pub is_software: bool,
}

impl GpuBenchConfig {
    pub fn detect(app: &App) -> Self {
        let is_software = is_software_renderer(app);
        if is_software {
            eprintln!("WARNING: software renderer detected — GPU bench numbers are not meaningful");
            Self {
                sample_size: 10,
                warm_up_frames: 30,
                is_software,
            }
        } else {
            Self {
                sample_size: 30,
                warm_up_frames: 20,
                is_software,
            }
        }
    }
}
