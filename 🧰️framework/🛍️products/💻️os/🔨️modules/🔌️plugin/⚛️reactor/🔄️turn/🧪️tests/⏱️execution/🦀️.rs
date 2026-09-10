//! ⏱️ The strict lifecycle authority must price the guest's own work, never the browser's or the
//! operating system's descheduling of a suspended turn.
use super::*;
use std::future::Future as _;

fn law() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🚪️lifetime/🧫️fixtures/⏱️execution.json")).unwrap()
}

fn drive_with_suspension(suspension_us: u64) -> (u64, u64) {
    let observed = std::rc::Rc::new(std::cell::Cell::new(None));
    let sink = observed.clone();
    let mut suspended = false;
    let driven = std::pin::pin!(std::future::poll_fn(move |_| {
        if !suspended {
            suspended = true;
            return std::task::Poll::Pending;
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
    let (first_us, _) = drive_with_suspension(law["suspensionUs"].as_u64().unwrap());
    let (second_us, _) = drive_with_suspension(law["suspensionUs"].as_u64().unwrap());
    let bound_us = law["perTurnBoundUs"].as_u64().unwrap();
    assert!(first_us < bound_us && second_us < bound_us, "first={first_us} us second={second_us} us must each stay under {bound_us} us");
    assert_eq!(guest_turn_executing_us(), Some(second_us), "a settled turn keeps exactly the microseconds it executed");
    eprintln!("[DEBUG] guest turn execution reset first_us={first_us} second_us={second_us}");
}
