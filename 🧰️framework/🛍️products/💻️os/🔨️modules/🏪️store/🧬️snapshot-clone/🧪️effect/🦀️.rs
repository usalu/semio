//! 🧪️ Snapshot-clone fixtures run controlled and reply-shaped children inside one actual original effect of a resident caller turn.
use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller};
use semio_framework_value::{RetainedAllocationEffect, retained_clone::RetainedCloneGrant, testing::fx};
use std::{cell::RefCell, mem::ManuallyDrop};

thread_local! { static CALLER: RefCell<Option<ManuallyDrop<OriginalJobFixtureCaller>>> = const { RefCell::new(None) }; }

fn stock() -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: 1 << 28, maximum_copy_bytes: 1 << 28, maximum_capacity_bytes: 1 << 26, maximum_release_bytes: 1 << 26, maximum_depth: 4096 } }

/// 🌱️ Births this test thread's caller before any heap observation starts; it is leaked with the thread because only its effects are under test.
pub fn prime() {
    CALLER.with(|slot| { slot.borrow_mut().get_or_insert_with(|| ManuallyDrop::new(OriginalJobFixtureCaller::admit(OperationId(1), Generation(1), stock(), || Some(1), u64::MAX, 1 << 40))); });
}

/// 🪢️ Runs one operation in a real caller turn's original effect; a grant the turn cannot afford runs on the plain fixture effect.
pub fn invoked<R>(grant: RetainedCloneGrant, operation: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> R) -> R {
    prime();
    let attempt = CALLER.with(|slot| slot.borrow_mut().as_mut().expect("primed original fixture caller").with_effect(grant, operation));
    match attempt { Ok(value) => value, Err(operation) => fx(grant, operation) }
}
