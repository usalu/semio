//! ⏱️ The strict lifecycle authority must price the guest's own work, never the browser's or the
//! operating system's descheduling of a suspended turn.
use super::*;
use std::future::Future as _;

fn law() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🚪️lifetime/🧫️fixtures/⏱️execution.json")).unwrap()
}

fn drive_with_suspension(suspension_us: u64) -> (u64, u64) {
    drive_with_suspension_executing(suspension_us, 0)
}

/// 🔥️ The same harness with a turn that really executes: after the suspension it spins until its own
/// accumulator has passed `execute_at_least_us`, so a later turn's accumulator proves non-inheritance
/// by construction rather than by how quiet the machine happened to be.
fn drive_with_suspension_executing(suspension_us: u64, execute_at_least_us: u64) -> (u64, u64) {
    let observed = std::rc::Rc::new(std::cell::Cell::new(None));
    let sink = observed.clone();
    let mut suspended = false;
    let driven = std::pin::pin!(std::future::poll_fn(move |_| {
        if !suspended {
            suspended = true;
            return std::task::Poll::Pending;
        }
        while guest_turn_executing_us().is_some_and(|executing_us| executing_us < execute_at_least_us) {
            std::hint::spin_loop();
        }
        sink.set(guest_turn_executing_us());
        std::task::Poll::Ready(())
    }));
    let mut future = std::pin::pin!(with_turn_execution(driven));
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    let wall_started_us = semio_framework_job::default_now_us().expect("native clock");
    assert!(future.as_mut().poll(&mut context).is_pending(), "the law needs one real suspension");
    std::thread::sleep(std::time::Duration::from_micros(suspension_us));
    assert!(future.as_mut().poll(&mut context).is_ready());
    let wall_us = semio_framework_job::default_now_us().expect("native clock") - wall_started_us;
    (observed.get().expect("native clock"), wall_us)
}

/// ⚖️ LAW: a turn parked across a suspension longer than the whole strict ceiling still measures only
/// the microseconds it executed — the exact failure mode that killed generation3d's first browser step
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, boot #6).
#[test]
fn guest_turn_execution_excludes_every_suspension_gap() {
    let law = law();
    let suspension_us = law["suspensionUs"].as_u64().unwrap();
    let (executing_us, wall_us) = drive_with_suspension(suspension_us);
    assert!(wall_us >= suspension_us, "the harness must really suspend: wall={wall_us} us");
    assert!(executing_us * law["wallToExecutingRatio"].as_u64().unwrap() < wall_us, "executing={executing_us} us must stay far below wall={wall_us} us");
    assert!(!semio_framework_trace::guest_lifecycle_turn_contract_violated(executing_us));
    assert!(semio_framework_trace::guest_lifecycle_turn_contract_violated(executing_us + semio_framework_trace::GUEST_LIFECYCLE_TURN_CEILING_US));
    eprintln!("[DEBUG] guest turn execution executing_us={executing_us} wall_us={wall_us} suspension_us={suspension_us}");
}

/// ⚖️ LAW: the accumulator is per-turn — a fresh turn never inherits the previous turn's microseconds.
#[test]
fn guest_turn_execution_resets_for_every_turn() {
    let law = law();
    let suspension_us = law["suspensionUs"].as_u64().unwrap();
    let bound_us = law["perTurnBoundUs"].as_u64().unwrap();
    // 🧭️ Non-inheritance is proved by CONSTRUCTION, not by a wall-clock read gap. The first turn spins
    // until its own accumulator has passed the whole per-turn bound; a second turn that inherited it
    // could not then come back under that bound. The previous shape drove two equally cheap turns and
    // pinned `guest_turn_executing_us() == second_us` — an equality that holds only while the
    // microsecond between the in-turn sample and the settled read rounds to zero, so a loaded machine
    // failed it (`left: Some(3)`) while the product property held. Both clauses below are ratios
    // against a measured accumulator, so load inflates both sides.
    let (busy_us, _) = drive_with_suspension_executing(suspension_us, bound_us);
    assert!(busy_us >= bound_us, "the first turn must really execute past the per-turn bound: busy={busy_us} us bound={bound_us} us");
    let (second_us, _) = drive_with_suspension(suspension_us);
    assert!(second_us < bound_us, "a fresh turn inherited the previous turn's microseconds: second={second_us} us busy={busy_us} us bound={bound_us} us");
    let settled_us = guest_turn_executing_us().expect("a settled turn keeps its own accumulator");
    assert!(settled_us >= second_us && settled_us < bound_us, "a settled turn keeps exactly the microseconds it executed: settled={settled_us} us in-turn={second_us} us bound={bound_us} us");
}
