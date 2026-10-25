//! 🌉️ Entry-point-only `block_on`: parks the thread on native, spins a no-op waker on wasm.

use std::future::Future;
use std::task::{Context, Poll};
#[cfg(target_arch = "wasm32")]
use std::task::Waker;
#[cfg(not(target_arch="wasm32"))]
use super::original_thread_wake;

//#region 🌉️BlockOn
/// 🧱️ Drives `fut` to completion on the calling thread — the boundary where a plain OS thread
/// BECOMES an executor.
///
/// 🚫️async: E5 executor bridge — one of the few functions in the whole repo deliberately NOT
/// `async`: an `async fn` can only ever be *driven* by something already polling it, so the bridge
/// that turns a thread into that poller cannot itself be `async` without begging the question. At
/// most one such bridge per crate (R2); this is this crate's.
///
/// 🚪️ Phase 1 packet P1a: gated behind the `entrypoint` cargo feature (enabled by the companion crate
/// `semio-framework-async-tests`, same shape as `testkit`/[`ManualRuntime`]) — the ONLY
/// approved callers are process entry points (a CLI `main`) and test/testkit consumers that
/// explicitly opt in with `features = ["entrypoint"]`. Interactive-reachable code must never call
/// this: driving a future to completion on the calling thread is exactly the run-to-completion
/// shape the whole INTERACTIVE-JOB-RUNTIME-REFACTOR forbids on a thread that must stay responsive.
/// `semio_framework_async_macros::async_test`'s generated `#[test]` harness does NOT call this fn —
/// it carries its own self-contained, zero-dependency inline copy (that macro crate must not link
/// this one, a runtime dependency, from 65+ crates' dev-dependency-only edge) — so gating this fn
/// behind a feature never affects `#[async_test]` expansion.
///
/// On every target except `wasm32-unknown-unknown` this parks the calling [`std::thread::Thread`]
/// and re-polls only once its [`std::task::Wake`] waker fires. `wasm32-unknown-unknown` has no
/// `Thread::unpark` to wake (single-threaded, no real OS thread to park), so that target instead
/// spins on [`std::task::Waker::noop`] — busy-polling is the correct fallback there since there is
/// no other thread that could ever wake a parked one.
#[cfg(feature = "entrypoint")]
pub fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = std::pin::pin!(fut);

    #[cfg(not(target_arch = "wasm32"))]
    {
        loop {
            let waker=original_thread_wake::original_thread_waker();
            let mut cx=Context::from_waker(&waker);
            match fut.as_mut().poll(&mut cx) {
                Poll::Ready(value) => return value,
                Poll::Pending => std::thread::park(),
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        loop {
            if let Poll::Ready(value) = fut.as_mut().poll(&mut cx) {
                return value;
            }
        }
    }
}
//#endregion 🌉️BlockOn
