//! 🧪️ Supplementary coverage: `&mut self` specifically, including its own zero-variant (`match *self
//! {}`) case, kept in a SEPARATE file from `uninhabited.rs` because `#[dyn_enum]` rejects mixing
//! `&mut self` with `self: Arc<Self>` on the same trait (see `uninhabited.rs`'s comment).
#![allow(async_fn_in_trait)] // R7 — never resolved by `+ Send` or by making a method sync.

use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};

#[dyn_enum]
pub trait Counter {
    fn bump(&mut self, by: u32);
    async fn total(&self) -> u32;
}

pub struct Plain {
    value: u32,
}

impl Counter for Plain {
    fn bump(&mut self, by: u32) {
        self.value += by;
    }
    async fn total(&self) -> u32 {
        self.value
    }
}

dyn_enum_close! {
    pub enum Counters: Counter {
        Plain(Plain),
    }
}

dyn_enum_close! {
    pub enum NoCounters: Counter {}
}

#[allow(dead_code, unreachable_code)]
fn assert_no_counters_impl_compiles(mut never: NoCounters) {
    never.bump(1);
}

fn block_on<F: std::future::Future>(future: F) -> F::Output {
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    fn noop(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
    let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
    let mut context = Context::from_waker(&waker);
    let mut future = std::pin::pin!(future);
    loop {
        if let Poll::Ready(value) = future.as_mut().poll(&mut context) {
            return value;
        }
    }
}

#[test]
fn mut_self_delegates_and_mutates_the_right_variant() {
    let mut counters = Counters::Plain(Plain { value: 0 });
    counters.bump(5);
    counters.bump(2);
    assert_eq!(block_on(counters.total()), 7);
}
