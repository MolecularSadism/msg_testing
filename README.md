# msg_testing

A testing utilities crate for Bevy applications, providing convenient helpers for testing systems that rely on fixed timesteps and time-dependent behavior.

## Features

- **Physics-enabled test apps** - Pre-configured apps with fixed timestep support
- **Paused time testing** - Test apps where time never advances
- **`AppTesting` trait** - Extension methods for app testing:
  - `fixed_update()` / `fixed_update_n()` - Step through fixed updates
  - `update()` / `update_n()` - Run multiple update cycles
  - `advance_time()` / `advance_time_secs()` - Manipulate virtual time
- **Float comparison helpers** - Re-exports `assert_approx_eq!` from float-cmp
- **Default timestep handling** - Uses Bevy's default fixed timestep automatically

## Installation

Add to your `Cargo.toml`:

```toml
[dev-dependencies]
msg_testing = "0.1"
```

## Quick Start

### Testing Fixed Update Systems

```rust
use bevy::prelude::*;
use msg_testing::{physics_app, AppTesting};

#[derive(Resource, Default)]
struct PhysicsCounter(usize);

fn physics_tick(mut counter: ResMut<PhysicsCounter>) {
    counter.0 += 1;
}

#[test]
fn test_physics_system() {
    let mut app = physics_app();
    app.insert_resource(PhysicsCounter::default());
    app.add_systems(FixedUpdate, physics_tick);

    // Advance time by one fixed timestep
    app.fixed_update();

    let counter = app.world().resource::<PhysicsCounter>();
    assert_eq!(counter.0, 1);

    // Or run multiple updates at once
    app.fixed_update_n(10);
    assert_eq!(app.world().resource::<PhysicsCounter>().0, 11);
}
```

### Testing with Paused Time

```rust
use bevy::prelude::*;
use msg_testing::{paused_app, AppTesting};

#[derive(Resource, Default)]
struct Counter(usize);

fn increment(mut counter: ResMut<Counter>) {
    counter.0 += 1;
}

#[test]
fn test_paused_time() {
    let mut app = paused_app();
    app.insert_resource(Counter::default());
    app.add_systems(FixedUpdate, increment);

    // Run many update cycles - fixed update should never run
    app.update_n(1000);

    let counter = app.world().resource::<Counter>();
    assert_eq!(counter.0, 0, "Fixed update should never run with paused time");
}
```

## API Overview

### `physics_app()`

Creates a minimal Bevy app with `MinimalPlugins` and default fixed timestep configuration. Use this for tests that need to run `FixedUpdate` systems.

```rust
let mut app = physics_app();
app.add_systems(FixedUpdate, my_physics_system);
app.fixed_update(); // Advance by one fixed timestep
```

### `paused_app()`

Creates a minimal Bevy app with paused virtual time. Fixed update will never run regardless of how many `app.update()` calls are made. Useful for testing that systems behave correctly when time is frozen.

```rust
let mut app = paused_app();
app.add_systems(FixedUpdate, should_not_run);

for _ in 0..1000 {
    app.update(); // Fixed update never runs
}
```

### `AppTesting` trait

Extension trait that adds convenient testing methods to Bevy's `App`:

- **`fixed_update()`** - Advance time by one fixed timestep and run FixedUpdate
- **`fixed_update_n(count)`** - Run multiple fixed updates
- **`update_n(count)`** - Run multiple update cycles
- **`advance_time(millis)`** - Advance virtual time by milliseconds
- **`advance_time_secs(secs)`** - Advance virtual time by seconds

```rust
use msg_testing::{physics_app, AppTesting};

let mut app = physics_app();

// Single fixed update
app.fixed_update();

// Multiple fixed updates
app.fixed_update_n(50);

// Multiple regular updates
app.update_n(100);

// Time manipulation
app.advance_time(500);  // Advance by 500ms
app.advance_time_secs(2.5);  // Advance by 2.5 seconds
```

## Use Cases

### Testing Physics Systems

When testing systems that run in `FixedUpdate` schedule (physics, gameplay logic, etc.):

```rust
use bevy::prelude::*;
use msg_testing::{physics_app, AppTesting};

#[test]
fn test_gravity() {
    let mut app = physics_app();
    // Add your physics systems
    app.add_systems(FixedUpdate, apply_gravity);

    // Step through multiple physics frames
    app.fixed_update_n(60);

    // Assert final state
}
```

### Testing Time-Dependent Behavior

Test that systems correctly handle paused time:

```rust
use bevy::prelude::*;
use msg_testing::paused_app;

#[test]
fn test_pause_handling() {
    let mut app = paused_app();
    app.add_systems(Update, countdown_timer);

    // Timer should not advance when paused
    for _ in 0..100 {
        app.update();
    }

    let timer = app.world().resource::<CountdownTimer>();
    assert_eq!(timer.remaining(), 10.0); // Unchanged
}
```

### Testing Multiple Ticks

Easily test behavior over many fixed update cycles:

```rust
use bevy::prelude::*;
use msg_testing::{physics_app, AppTesting};

#[test]
fn test_accumulation() {
    let mut app = physics_app();
    app.insert_resource(Accumulator(0));
    app.add_systems(FixedUpdate, accumulate);

    // Run 120 fixed updates (2 seconds at 60 FPS)
    app.fixed_update_n(120);

    let acc = app.world().resource::<Accumulator>();
    assert_eq!(acc.0, 120);
}
```

## Integration with Bevy

This crate uses Bevy's built-in time management:
- `Time<Fixed>` - Uses Bevy's default fixed timestep (64 Hz)
- `Time<Virtual>` - Used for time advancement and pausing
- `MinimalPlugins` - Provides core Bevy functionality without rendering

All time-related behavior matches what you'd see in a real Bevy application.

## Bevy compatibility

| `msg_testing` | Bevy   |
|--------------|--------|
| 0.1          | 0.18   |

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.

## Contributing

Issues and pull requests welcome at [https://github.com/MolecularSadism/msg_testing](https://github.com/MolecularSadism/msg_testing)
