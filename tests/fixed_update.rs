use bevy::ecs::system::ResMut;
use bevy::prelude::{FixedUpdate, Reflect, Resource, Update};
use msg_testing::{AppTesting, paused_app, physics_app};

#[derive(Resource, Reflect, Debug, Default)]
struct TickCounter {
    pub update: usize,
    pub fixed_update: usize,
}

fn update(mut counter: ResMut<TickCounter>) {
    counter.update += 1;
}

fn fixed_update(mut counter: ResMut<TickCounter>) {
    counter.fixed_update += 1;
}

#[test]
fn update_counts() {
    let mut app = physics_app();
    app.init_resource::<TickCounter>();
    app.register_type::<TickCounter>();
    app.add_systems(Update, update);
    app.add_systems(FixedUpdate, fixed_update);

    app.update();

    let counter = app.world().resource::<TickCounter>();
    assert_eq!(counter.update, 1, "Update should run once");
    assert_eq!(
        counter.fixed_update, 1,
        "FixedUpdate runs once per update() on physics_app (ManualDuration ensures exactly one fixed step)"
    );
}

#[test]
fn fixed_update_counts() {
    let mut app = physics_app();
    app.init_resource::<TickCounter>();
    app.register_type::<TickCounter>();
    app.add_systems(Update, update);
    app.add_systems(FixedUpdate, fixed_update);

    app.fixed_update();

    let counter = app.world().resource::<TickCounter>();
    assert_eq!(counter.update, 1, "Update should run once");
    assert_eq!(
        counter.fixed_update, 1,
        "FixedUpdate should run once after fixed_update()"
    );
}

#[test]
fn fifty_ticks() {
    let mut app = physics_app();
    app.init_resource::<TickCounter>();
    app.register_type::<TickCounter>();
    app.add_systems(Update, update);
    app.add_systems(FixedUpdate, fixed_update);

    app.fixed_update_n(50);

    let counter = app.world().resource::<TickCounter>();
    assert_eq!(counter.update, 50, "Update should run 50 times");
    assert_eq!(counter.fixed_update, 50, "FixedUpdate should run 50 times");
}

#[test]
fn paused_time_never_runs_fixed_update() {
    let mut app = paused_app();
    app.init_resource::<TickCounter>();
    app.register_type::<TickCounter>();
    app.add_systems(Update, update);
    app.add_systems(FixedUpdate, fixed_update);

    // Run 200 update cycles with paused time (sufficient to verify FixedUpdate never fires)
    app.update_n(200);

    let counter = app.world().resource::<TickCounter>();
    assert_eq!(counter.update, 200, "Update should run 200 times");
    assert_eq!(
        counter.fixed_update, 0,
        "FixedUpdate should never run with paused time"
    );
}
