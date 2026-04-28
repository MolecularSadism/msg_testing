//! Testing utilities for Bevy applications with fixed timestep support.
//!
//! This crate provides convenient helpers for testing Bevy systems that rely on
//! fixed timesteps, physics updates, and time-dependent behavior.
//!
//! # Features
//!
//! - **`physics_app()`** - Pre-configured test app with fixed timestep
//! - **`paused_app()`** - Test app with frozen time
//! - **`AppTesting`** trait - Extension methods for app testing
//!   - `fixed_update()` - Step through one fixed update
//!   - `update_n()` / `fixed_update_n()` - Run multiple update cycles
//!   - `advance_time()` / `advance_time_secs()` - Manipulate virtual time
//! - **`assert_approx_eq!`** - Floating point equality assertions (re-exported from float-cmp)
//!
//! # Quick Start
//!
//! ```
//! use bevy::prelude::*;
//! use msg_testing::{physics_app, AppTesting};
//!
//! #[derive(Resource, Default)]
//! struct Counter(usize);
//!
//! fn increment(mut counter: ResMut<Counter>) {
//!     counter.0 += 1;
//! }
//!
//! let mut app = physics_app();
//! app.insert_resource(Counter::default());
//! app.add_systems(FixedUpdate, increment);
//!
//! // Advance time by one fixed timestep
//! app.fixed_update();
//!
//! let counter = app.world().resource::<Counter>();
//! assert_eq!(counter.0, 1);
//!
//! // Or run multiple updates
//! app.fixed_update_n(10);
//! assert_eq!(app.world().resource::<Counter>().0, 11);
//! ```
//!
//! # Testing Fixed Update Systems
//!
//! Use [`physics_app()`] for tests that need to run systems in the `FixedUpdate` schedule:
//!
//! ```
//! use bevy::prelude::*;
//! use msg_testing::{physics_app, AppTesting};
//!
//! #[derive(Component)]
//! struct Velocity(f32);
//!
//! #[derive(Component)]
//! struct Position(f32);
//!
//! fn apply_velocity(mut query: Query<(&Velocity, &mut Position)>) {
//!     for (vel, mut pos) in &mut query {
//!         pos.0 += vel.0;
//!     }
//! }
//!
//! let mut app = physics_app();
//! app.add_systems(FixedUpdate, apply_velocity);
//!
//! let entity = app.world_mut().spawn((Position(0.0), Velocity(1.0))).id();
//!
//! // Step through 10 fixed updates
//! app.fixed_update_n(10);
//!
//! let pos = app.world().entity(entity).get::<Position>().unwrap();
//! assert_eq!(pos.0, 10.0);
//! ```
//!
//! # Testing with Paused Time
//!
//! Use [`paused_app()`] to test that systems behave correctly when time is frozen:
//!
//! ```
//! use bevy::prelude::*;
//! use msg_testing::paused_app;
//!
//! #[derive(Resource, Default)]
//! struct FixedCounter(usize);
//!
//! fn increment_fixed(mut counter: ResMut<FixedCounter>) {
//!     counter.0 += 1;
//! }
//!
//! let mut app = paused_app();
//! app.insert_resource(FixedCounter::default());
//! app.add_systems(FixedUpdate, increment_fixed);
//!
//! // Run many update cycles - fixed update should never run
//! for _ in 0..100 {
//!     app.update();
//! }
//!
//! let counter = app.world().resource::<FixedCounter>();
//! assert_eq!(counter.0, 0, "Fixed update should not run when time is paused");
//! ```

use bevy::app::App;
use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::prelude::{Fixed, MinimalPlugins};
use bevy::time::{Real, Time, TimeUpdateStrategy, Virtual};
use std::time::Duration;

// Re-export float-cmp for floating point comparisons in tests
pub use float_cmp::{approx_eq, assert_approx_eq};

/// Extension trait for App to add testing utilities.
///
/// This trait provides convenient methods for manipulating time, running update cycles,
/// and testing time-dependent behavior in Bevy applications.
pub trait AppTesting {
    /// Advance time by one fixed timestep and run fixed update.
    ///
    /// This is necessary for `FixedUpdate` schedule to run.
    ///
    /// # Example
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use msg_testing::{physics_app, AppTesting};
    ///
    /// #[derive(Resource, Default)]
    /// struct Counter(usize);
    ///
    /// fn increment(mut counter: ResMut<Counter>) {
    ///     counter.0 += 1;
    /// }
    ///
    /// let mut app = physics_app();
    /// app.insert_resource(Counter::default());
    /// app.add_systems(FixedUpdate, increment);
    ///
    /// app.fixed_update();
    ///
    /// let counter = app.world().resource::<Counter>();
    /// assert_eq!(counter.0, 1);
    /// ```
    fn fixed_update(&mut self);

    /// Run the `Update` schedule multiple times.
    ///
    /// # Example
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use msg_testing::{physics_app, AppTesting};
    ///
    /// #[derive(Resource, Default)]
    /// struct Counter(usize);
    ///
    /// fn increment(mut counter: ResMut<Counter>) {
    ///     counter.0 += 1;
    /// }
    ///
    /// let mut app = physics_app();
    /// app.insert_resource(Counter::default());
    /// app.add_systems(Update, increment);
    ///
    /// app.update_n(50);
    ///
    /// assert_eq!(app.world().resource::<Counter>().0, 50);
    /// ```
    fn update_n(&mut self, count: usize);

    /// Advance time by multiple fixed timesteps and run fixed update.
    ///
    /// # Example
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use msg_testing::{physics_app, AppTesting};
    ///
    /// #[derive(Resource, Default)]
    /// struct Counter(usize);
    ///
    /// fn increment(mut counter: ResMut<Counter>) {
    ///     counter.0 += 1;
    /// }
    ///
    /// let mut app = physics_app();
    /// app.insert_resource(Counter::default());
    /// app.add_systems(FixedUpdate, increment);
    ///
    /// app.fixed_update_n(10);
    ///
    /// assert_eq!(app.world().resource::<Counter>().0, 10);
    /// ```
    fn fixed_update_n(&mut self, count: usize);

    /// Advance virtual time by the specified number of milliseconds.
    ///
    /// Useful for testing time-dependent systems and timers.
    /// Note: This only advances time; you must call `update()` to run systems.
    ///
    /// # Example
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use bevy::time::Virtual;
    /// use msg_testing::{physics_app, AppTesting};
    /// use std::time::Duration;
    ///
    /// let mut app = physics_app();
    ///
    /// let initial_time = app.world().resource::<Time<Virtual>>().elapsed();
    ///
    /// app.advance_time(500);
    ///
    /// let new_time = app.world().resource::<Time<Virtual>>().elapsed();
    /// assert_eq!(new_time - initial_time, Duration::from_millis(500));
    /// ```
    fn advance_time(&mut self, millis: u64);

    /// Advance virtual time by the specified number of seconds.
    ///
    /// Convenience method for `advance_time()` when working with seconds.
    /// Note: This only advances time; you must call `update()` to run systems.
    ///
    /// # Example
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use bevy::time::Virtual;
    /// use msg_testing::{physics_app, AppTesting};
    /// use std::time::Duration;
    ///
    /// let mut app = physics_app();
    ///
    /// let initial_time = app.world().resource::<Time<Virtual>>().elapsed();
    ///
    /// app.advance_time_secs(2.5);
    ///
    /// let new_time = app.world().resource::<Time<Virtual>>().elapsed();
    /// assert_eq!(new_time - initial_time, Duration::from_secs_f32(2.5));
    /// ```
    fn advance_time_secs(&mut self, secs: f32);
}

impl AppTesting for App {
    fn fixed_update(&mut self) {
        self.update();
    }

    fn update_n(&mut self, count: usize) {
        for _ in 0..count {
            self.update();
        }
    }

    fn fixed_update_n(&mut self, count: usize) {
        for _ in 0..count {
            self.fixed_update();
        }
    }

    fn advance_time(&mut self, millis: u64) {
        self.world_mut()
            .resource_mut::<Time<Virtual>>()
            .advance_by(Duration::from_millis(millis));
    }

    fn advance_time_secs(&mut self, secs: f32) {
        self.world_mut()
            .resource_mut::<Time<Virtual>>()
            .advance_by(Duration::from_secs_f32(secs));
    }
}

/// Create a test app with minimal plugins and default fixed timestep.
/// Use this for tests that need physics or fixed update schedules.
///
/// # Example
///
/// ```
/// use bevy::prelude::*;
/// use msg_testing::{physics_app, AppTesting};
///
/// let mut app = physics_app();
/// app.fixed_update();
/// ```
pub fn physics_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    let timestep = app.world().resource::<Time<Fixed>>().timestep();

    // ManualDuration makes time_system set Time<Real>.delta = timestep on every update(),
    // which propagates to Time<Virtual>.delta = timestep via update_virtual_time,
    // causing run_fixed_main_schedule to fire expend() exactly once per update().
    app.insert_resource(TimeUpdateStrategy::ManualDuration(timestep));

    // update_with_instant returns early (without calling advance_by) when last_update is None,
    // which would cause the first update() to have Time<Real>.delta = 0 and skip FixedMain.
    // Pre-warming sets last_update so the first update() gives delta = timestep like all others.
    app.world_mut()
        .resource_mut::<Time<Real>>()
        .update_with_duration(timestep);

    app
}

/// Create a test app with paused time.
/// Fixed update will never run regardless of how many update cycles are executed.
///
/// # Example
///
/// ```
/// use bevy::prelude::*;
/// use msg_testing::paused_app;
///
/// #[derive(Resource, Default)]
/// struct Counter(usize);
///
/// fn increment(mut counter: ResMut<Counter>) {
///     counter.0 += 1;
/// }
///
/// let mut app = paused_app();
/// app.insert_resource(Counter::default());
/// app.add_systems(FixedUpdate, increment);
///
/// // Run many update cycles
/// for _ in 0..100 {
///     app.update();
/// }
///
/// // Fixed update should never have run
/// let counter = app.world().resource::<Counter>();
/// assert_eq!(counter.0, 0);
/// ```
pub fn paused_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Pause virtual time so fixed update never runs
    app.world_mut().resource_mut::<Time<Virtual>>().pause();

    app
}

/// Create an [`AssetPlugin`] configured for fast tests.
///
/// Disables file watching and meta file checks to avoid filesystem overhead
/// that causes tests to take 60+ seconds on Windows.
///
/// # Example
///
/// ```
/// use bevy::prelude::*;
/// use msg_testing::test_asset_plugin;
///
/// let mut app = App::new();
/// app.add_plugins(MinimalPlugins);
/// app.add_plugins(test_asset_plugin());
/// app.update();
/// ```
pub fn test_asset_plugin() -> AssetPlugin {
    AssetPlugin {
        meta_check: AssetMetaCheck::Never,
        watch_for_changes_override: Some(false),
        ..Default::default()
    }
}
