# Changelog

## 0.4.1

- Added `GpuAppTesting` (`gpu` feature): `offscreen_target(size)` gives a camera an
  image to render into, since a `gpu_app()` has no window, and `update_gpu()` runs
  `update()` then blocks until the GPU has executed the frame, so benches time the
  GPU work and not only its submission.
- `gpu_app()` installs `LogPlugin` only for the first app in a process; later apps
  no longer log an error for the already-set global subscriber.
- `gpu` feature enables `bevy/bevy_log`.

## 0.4.0

- Added `AppTesting::with_timestep(timestep)`: `physics_app().with_timestep(..)`
  runs `FixedUpdate` at an application-chosen rate, keeping one `FixedMain`
  run per `update()`.
- Added `AppTesting::update_until(budget, done)`: runs `update()` with a
  one-millisecond sleep between calls until `done` holds or the wall-clock
  `budget` elapses, for waiting on task-pool work without a fixed update count.
- `advance_time()` / `advance_time_secs()` now advance the generic `Time`
  clock alongside `Time<Virtual>`, so `run_system_once` on a system reading
  `Res<Time>` sees the delta.

## 0.3.1

- `gpu` feature no longer pulls in `bevy/default` (audio, 2d, 3d, ui); it now
  enables only `bevy_render`/`bevy_winit`/`bevy_image`/`bevy_window`.

## 0.3.0

- **Breaking:** `assert_approx_eq!` is now this crate's own value-first macro —
  `assert_approx_eq!(left, right)` with a default `1e-4` absolute tolerance, or
  `assert_approx_eq!(left, right, tolerance)` — replacing the re-export of
  float-cmp's type-first `assert_approx_eq!(f32, left, right)`. Call sites
  using the old form must drop the leading type argument (and pass an explicit
  tolerance where the ULP default mattered), or switch to the still re-exported
  `approx_eq!`.
- `assert_approx_eq!` treats exactly equal operands as equal, so equal
  infinities pass; both forms expand to expressions.
- Added `minimal_app()`, a bare `MinimalPlugins` test app builder;
  `physics_app()` and `paused_app()` now build on it.

## 0.2.0

- Added the `gpu` feature: `gpu_app()`, `gpu_app_ready()`,
  `is_software_renderer()`, and `GpuBenchConfig`.
- Added `fixture_dir()` for tests that feed themselves their own files.

## 0.1.0

- Initial release: `physics_app()`, `paused_app()`, `test_asset_plugin()`, and
  the `AppTesting` extension trait.
