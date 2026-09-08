
use super::*;

fn clock(start: i64, step_ms: i64) -> impl Fn() -> i64 {
    let tick = RefCell::new(start);
    move || {
        let mut tick = tick.borrow_mut();
        *tick += step_ms;
        *tick
    }
}

fn budget() -> TurnBudget {
    TurnBudget { fuel: 1_000_000, deadline_ms: 16, max_effects: 8, max_patch_bytes: 4096, max_frames: 8 }
}

#[test]
fn idle_profile_emits_nothing() {
    on_instance_open(br#"{"profile":"idle"}"#);
    let outcome = turn(budget(), clock(0, 1));
    assert!(outcome.effects.is_empty());
    assert!(outcome.patches.is_empty());
    assert!(!outcome.status_more_work);
}

#[test]
fn cpu_profile_consumes_at_most_its_declared_budget() {
    on_instance_open(br#"{"profile":"cpu","cpuBusyMs":5}"#);
    let outcome = turn(budget(), clock(0, 1));
    assert!(outcome.fuel_used <= u64::from(budget().deadline_ms));
    assert!(outcome.status_more_work);
}

#[test]
fn ui_profile_emits_monotonically_increasing_revisions() {
    on_instance_open(br#"{"profile":"ui","uiPatchesPerTurn":3}"#);
    let outcome = turn(budget(), clock(0, 1));
    assert_eq!(outcome.patches.len(), 3);
    let revisions: Vec<u64> = outcome.patches.iter().map(|p| p.revision).collect();
    assert_eq!(revisions, vec![1, 2, 3]);
    for patch in &outcome.patches {
        assert!((patch.bytes.len() as u32) <= budget().max_patch_bytes);
    }
}

#[test]
fn ui_profile_caps_patches_at_max_frames() {
    on_instance_open(br#"{"profile":"ui","uiPatchesPerTurn":1000}"#);
    let outcome = turn(budget(), clock(0, 1));
    assert_eq!(outcome.patches.len(), budget().max_frames as usize);
}

#[test]
fn io_profile_requests_once_then_goes_idle_until_completion() {
    on_instance_open(br#"{"profile":"io","ioCapabilityId":"scale-fixture.io"}"#);
    let first = turn(budget(), clock(0, 1));
    assert_eq!(first.effects.len(), 1);
    assert!(matches!(first.effects[0], PlainEffect::RequestCapability { ref id, .. } if id == "scale-fixture.io"));
    let second = turn(budget(), clock(0, 1));
    assert!(second.effects.is_empty());
    assert!(!second.status_more_work);
    on_completed(1);
    let third = turn(budget(), clock(0, 1));
    assert!(third.effects.is_empty());
}

#[test]
fn io_profile_re_requests_after_capability_revoked() {
    on_instance_open(br#"{"profile":"io","ioCapabilityId":"scale-fixture.io"}"#);
    let _ = turn(budget(), clock(0, 1)); // requests
    on_completed(1); // granted
    assert_eq!(revocation_count(), 0);
    on_capability_revoked("scale-fixture.io");
    assert_eq!(revocation_count(), 1);
    let after_revoke = turn(budget(), clock(0, 1));
    assert_eq!(after_revoke.effects.len(), 1, "revocation should trigger a fresh request-capability effect");
}

#[test]
fn capability_revoked_for_other_id_is_ignored() {
    on_instance_open(br#"{"profile":"io","ioCapabilityId":"scale-fixture.io"}"#);
    let _ = turn(budget(), clock(0, 1));
    on_completed(1);
    on_capability_revoked("some-other-plugin.io");
    assert_eq!(revocation_count(), 0);
    let outcome = turn(budget(), clock(0, 1));
    assert!(outcome.effects.is_empty(), "a revocation for a different capability id must not disturb this actor");
}

#[test]
fn hang_profile_overruns_its_own_deadline() {
    on_instance_open(br#"{"profile":"hang","hangOverrunMultiplier":3}"#);
    let start = 0i64;
    let clk = clock(start, 1);
    let outcome = turn(budget(), &clk);
    assert!(outcome.fuel_used > u64::from(budget().deadline_ms));
    assert!(outcome.status_more_work);
}

#[test]
#[should_panic(expected = "scale-fixture crash profile")]
fn crash_profile_traps_on_configured_turn() {
    on_instance_open(br#"{"profile":"crash","crashAfterTurns":2}"#);
    let _ = turn(budget(), clock(0, 1));
    let _ = turn(budget(), clock(0, 1));
}

#[test]
fn stateful_profile_checkpoint_restore_round_trips_exactly() {
    on_instance_open(br#"{"profile":"stateful"}"#);
    for _ in 0..5 {
        let _ = turn(budget(), clock(0, 1));
    }
    let snapshot = checkpoint().expect("checkpoint encodes");
    let before = STATE.with(|state| state.borrow().accumulated.clone());
    // 🔀️ Mutate further so restore has something real to undo.
    let _ = turn(budget(), clock(0, 1));
    assert_ne!(STATE.with(|state| state.borrow().accumulated.clone()), before);
    restore(&snapshot).expect("restore decodes");
    let after = STATE.with(|state| state.borrow().accumulated.clone());
    assert_eq!(after, before);
}

#[test]
fn job_echoes_input_immediately() {
    jobs::start_job(1, "semio.io-run", vec![1, 2, 3]);
    match jobs::step_job(1) {
        jobs::JobOutcome::Done(bytes) => assert_eq!(bytes, vec![1, 2, 3]),
        jobs::JobOutcome::Failed(_) => panic!("expected Done"),
    }
}

#[test]
fn unknown_job_fails() {
    match jobs::step_job(999) {
        jobs::JobOutcome::Failed(_) => {}
        jobs::JobOutcome::Done(_) => panic!("expected Failed for unknown job id"),
    }
}
