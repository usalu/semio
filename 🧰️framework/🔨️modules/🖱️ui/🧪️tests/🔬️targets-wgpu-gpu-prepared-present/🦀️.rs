
use super::*;

static TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn guard() -> std::sync::MutexGuard<'static, ()> {
    match TEST_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn drain() {
    while !PreparedGpuPresentCursor::close_abandoned_step() {}
}

#[test]
fn interrupted_present_cursor_hands_back_generation_and_fixed_owners() {
    let _guard = guard();
    drain();
    let cursor = match PreparedGpuPresentCursor::begin(7, 3) {
        Some(cursor) => cursor,
        None => panic!("fixed present cursor admission"),
    };
    drop(cursor);
    assert!(!PreparedGpuPresentCursor::close_abandoned_step());
    assert!(PreparedGpuPresentCursor::close_abandoned_step());
    assert!(PREPARED_GPU_ABANDONMENT_STATE.iter().all(|state| state.load(Ordering::Acquire) == 0));
}

#[test]
fn present_cursor_generation_and_capacity_boundaries_refuse_before_ownership() {
    let _guard = guard();
    drain();
    assert!(PreparedGpuPresentCursor::begin(0, 3).is_none());
    assert!(PreparedGpuPresentCursor::begin(7, u64::MAX).is_none());
    let mut owners: [Option<PreparedGpuPresentCursor>; PREPARED_GPU_ABANDONMENT_SLOTS] = std::array::from_fn(|_| None);
    for owner in &mut owners {
        *owner = PreparedGpuPresentCursor::begin(7, 3);
        assert!(owner.is_some());
    }
    assert!(PreparedGpuPresentCursor::begin(7, 3).is_none());
    for owner in owners.iter_mut().filter_map(Option::as_mut) {
        owner.begin_close();
        while !owner.close_step() {}
        assert!(owner.terminal_is_empty());
    }
}
