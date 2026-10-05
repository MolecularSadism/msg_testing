use std::sync::atomic::{AtomicBool, Ordering};

use bevy::app::App;
use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::camera::RenderTarget;
use bevy::ecs::error::warn;
use bevy::image::ImagePlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::render::RenderApp;
use bevy::render::pipelined_rendering::PipelinedRenderingPlugin;
use bevy::render::render_resource::{PollType, TextureFormat};
use bevy::render::renderer::{RenderAdapterInfo, RenderDevice};
use bevy::window::{ExitCondition, WindowPlugin};
use bevy::winit::WinitPlugin;

/// Set once the first [`gpu_app`] in this process has installed the global log subscriber.
static LOG_INSTALLED: AtomicBool = AtomicBool::new(false);

/// A headless app with a real render backend and no window.
///
/// Cameras have no primary window to render to, so give each one a target from
/// [`GpuAppTesting::offscreen_target`]. Only the first app in a process installs
/// [`LogPlugin`]; the global subscriber can be set once, and benches build many apps.
pub fn gpu_app() -> App {
    let mut app = App::new();
    app.set_error_handler(warn);
    let mut plugins = DefaultPlugins
        .build()
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
        .set(ImagePlugin::default_nearest());
    if LOG_INSTALLED.swap(true, Ordering::SeqCst) {
        plugins = plugins.disable::<LogPlugin>();
    }
    app.add_plugins(plugins);
    app
}

pub fn gpu_app_ready(app: &mut App) {
    app.finish();
    app.cleanup();
}

/// Extension methods for apps built with [`gpu_app`].
pub trait GpuAppTesting {
    /// A render target backed by a new `size`-pixel colour image, for a camera to draw into.
    ///
    /// ```no_run
    /// use bevy::prelude::*;
    /// use msg_testing::{GpuAppTesting, gpu_app};
    ///
    /// let mut app = gpu_app();
    /// let target = app.offscreen_target(UVec2::new(1920, 1080));
    /// app.world_mut().spawn((Camera2d, target));
    /// ```
    fn offscreen_target(&mut self, size: UVec2) -> RenderTarget;

    /// Runs one `update()` and blocks until the GPU has executed everything it submitted.
    ///
    /// `update()` alone returns once the frame is queued, so wall-clock timing around it
    /// measures only the CPU side.
    fn update_gpu(&mut self);
}

impl GpuAppTesting for App {
    fn offscreen_target(&mut self, size: UVec2) -> RenderTarget {
        let image = Image::new_target_texture(size.x, size.y, TextureFormat::bevy_default(), None);
        let handle = self.world_mut().resource_mut::<Assets<Image>>().add(image);
        RenderTarget::Image(handle.into())
    }

    fn update_gpu(&mut self) {
        self.update();
        let device = self
            .get_sub_app(RenderApp)
            .and_then(|render| render.world().get_resource::<RenderDevice>())
            .expect("update_gpu needs a gpu_app made ready with gpu_app_ready");
        if let Err(error) = device.poll(PollType::wait_indefinitely()) {
            error!("waiting for the GPU failed: {error}");
        }
    }
}

pub fn is_software_renderer(app: &App) -> bool {
    let Some(render_app) = app.get_sub_app(RenderApp) else {
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
