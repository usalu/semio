//! 🧪️ Laws of the per-thread isolation: one instance per thread and key, stable within a thread, absent without the feature.

use super::{ISOLATED, isolated_cell};

#[test]
fn one_thread_and_key_always_meet_the_same_instance() {
    let first = isolated_cell::<u32>(1, || 7);
    *first.lock().expect("lock") += 1;
    let second = isolated_cell::<u32>(1, || 7);
    assert!(std::ptr::eq(first, second));
    assert_eq!(*second.lock().expect("lock"), 8);
}

#[test]
fn another_key_is_another_instance() {
    let a = isolated_cell::<u32>(2, || 1);
    let b = isolated_cell::<u32>(3, || 1);
    assert!(!std::ptr::eq(a, b));
}

#[test]
fn another_thread_starts_from_its_own_initial_state() {
    *isolated_cell::<u32>(4, || 0).lock().expect("lock") = 41;
    let other = std::thread::spawn(|| *isolated_cell::<u32>(4, || 0).lock().expect("lock")).join().expect("join");
    assert_eq!(other, 0);
    assert_eq!(*isolated_cell::<u32>(4, || 0).lock().expect("lock"), 41);
}

#[cfg(not(feature = "isolated"))]
#[test]
fn isolation_is_off_unless_a_target_enables_the_feature() {
    assert!(!ISOLATED);
}

#[cfg(feature = "isolated")]
#[test]
fn isolation_is_on_when_a_target_enables_the_feature() {
    assert!(ISOLATED);
}
