use bevy::ecs::system::ResMut;
use bevy::prelude::{Fixed, FixedUpdate, Reflect, Res, Resource, Time, Update};
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

#[test]
fn physics_app_with_timestep_runs_one_fixed_step_per_update_at_that_rate() {
    use std::time::Duration;

    let timestep = Duration::from_secs_f64(1.0 / 60.0);
    let mut app = msg_testing::physics_app_with_timestep(timestep);
    app.insert_resource(TickCounter::default());
    app.add_systems(FixedUpdate, fixed_update);

    app.fixed_update_n(60);

    assert_eq!(app.world().resource::<TickCounter>().fixed_update, 60);
    assert_eq!(app.world().resource::<Time<Fixed>>().timestep(), timestep);
}

#[test]
fn update_until_stops_once_the_condition_holds() {
    use std::time::Duration;

    let mut app = msg_testing::minimal_app();
    app.insert_resource(TickCounter::default());
    app.add_systems(Update, update);

    let settled = app.update_until(Duration::from_secs(10), |app| {
        app.world().resource::<TickCounter>().update >= 5
    });

    assert!(settled);
    assert_eq!(app.world().resource::<TickCounter>().update, 5);
}

#[test]
fn update_until_reports_an_exhausted_budget() {
    use std::time::Duration;

    let mut app = msg_testing::minimal_app();
    let settled = app.update_until(Duration::from_millis(20), |_| false);
    assert!(!settled);
}

#[test]
fn advance_time_reaches_a_system_run_once_on_the_generic_clock() {
    use bevy::ecs::system::RunSystemOnce;

    #[derive(Resource, Default)]
    struct Seen(f32);

    fn read_delta(time: Res<Time>, mut seen: ResMut<Seen>) {
        seen.0 = time.delta_secs();
    }

    let mut app = msg_testing::physics_app();
    app.insert_resource(Seen::default());
    app.advance_time_secs(1.5);
    app.world_mut().run_system_once(read_delta).unwrap();

    msg_testing::assert_approx_eq!(app.world().resource::<Seen>().0, 1.5);
}
