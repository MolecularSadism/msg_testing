# Changelog

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
