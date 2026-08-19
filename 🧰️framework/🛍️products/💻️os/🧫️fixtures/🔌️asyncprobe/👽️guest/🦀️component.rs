//! 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-S1 + terra-probe-spikes W6). Guest half of the
//! go/no-go probe for component-model-async / WASI 0.3: the original G3 shapes (async export taking
//! a scalar, async import awaited mid-call, async export reading a host-written `stream<u32>`) plus
//! the W6 S1-S6 runtime-behaviour spikes.

use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

wit_bindgen::generate!({
    path: "🧬️schema/📜️world.wit",
    world: "asyncprobe",
});

struct Component;

impl Guest for Component {
    async fn ping(n: u32) -> u32 {
        // 🎯️ the critical case: awaiting a HOST-implemented async import mid-call.
        let echoed = echo(format!("ping:{n}")).await;
        let _ = echoed;
        n + 1
    }

    async fn run(mut events: wit_bindgen::rt::async_support::StreamReader<u32>) -> u32 {
        let mut total: u32 = 0;
        while let Some(v) = events.next().await {
            total = total.wrapping_add(v);
        }
        total
    }

    // #region S1 — epoch-Yield fairness
    // 🧪️ CPU-bound, no internal `.await` at all. If epoch-Yield fairness works, wasmtime preempts
    // this loop transparently at wasm loop-back edges and interleaves it with the other concurrently
    // running `burn` call, purely via `Config::epoch_interruption` + `epoch_deadline_callback`. If it
    // does NOT work, one `burn` call will run to completion before the other starts at all — visible
    // in the host's progress log.
    async fn burn(guest_id: u32, iters: u32) -> u32 {
        let mut acc: u32 = 0;
        let mut i: u32 = 0;
        while i < iters {
            acc = acc.wrapping_add(i.wrapping_mul(2654435761));
            if i % 4096 == 0 {
                progress(guest_id, i / 4096);
            }
            i = i.wrapping_add(1);
        }
        acc
    }
    // #endregion

    // #region S1c — pure-CPU loop, ZERO host-import calls (isolates epoch/fuel-Yield preemption
    // from the `progress` import call's own guest->host call boundary, which S1/S1b did not
    // control for). Structurally identical to `burn` minus the `progress()` call; the accumulator
    // is returned so the loop cannot be optimised away.
    async fn burn_pure(_guest_id: u32, iters: u32) -> u32 {
        // 🧪️ S1c pitfall (found empirically, not anticipated): with ZERO host-import calls in the
        // loop body, this is a pure induction-variable sum LLVM can strength-reduce to a CLOSED
        // FORM (sum of `i * C` for i in 0..iters has an arithmetic-series closed form, even under
        // wrapping) and eliminate the loop entirely — first attempt returned in ~6 microseconds
        // for 300M "iterations" with 0 epoch-callback hits, i.e. no loop ever ran. `black_box`
        // forces each iteration to be treated as opaque so the actual work happens.
        let mut acc: u32 = 0;
        let mut i: u32 = 0;
        while i < iters {
            let step = std::hint::black_box(i).wrapping_mul(2654435761);
            acc = std::hint::black_box(acc.wrapping_add(step));
            i = i.wrapping_add(1);
        }
        acc
    }
    // #endregion

    // #region S3 — cross-export concurrency
    // 🧪️ Sync export invoked from the host while a separate `run` call is parked awaiting stream
    // items. Purely a liveness probe — the return value only proves this particular call executed.
    async fn checkpoint() -> u32 {
        42
    }
    // #endregion

    // #region S2 — host-import future drop on guest-side cancel
    async fn cancel_probe() -> bool {
        // Manually drive the `hang` import's future exactly one poll (with a no-op waker) so the
        // host-side async fn body is guaranteed to have started and reached its `pending().await`
        // point, then drop the future outright — this is the guest-side "cancel a subtask" act.
        let mut fut = Box::pin(hang(99));
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        match fut.as_mut().poll(&mut cx) {
            Poll::Pending => {}
            Poll::Ready(_) => return false, // unexpected: `hang` resolved on the first poll
        }
        drop(fut);
        // Give the host a chance to observe the cancellation by round-tripping through another
        // async import (`echo`) before checking the flag — this forces at least one more
        // executor tick to elapse on the host side.
        let _ = echo("post-cancel-sync".to_string()).await;
        was_hang_dropped()
    }
    // #endregion

    // #region S6 — nested guest-side executor
    async fn nested_exec_probe() -> String {
        // Hand-rolled single-task "local executor" shaped like an `Rc<RefCell>`-backed SDK
        // `LocalExecutor`: a queue of exactly one runnable task, polled by forwarding the OUTER
        // task's own `Context`/`Waker` straight through. This is the minimal honest simulation of
        // "spawn onto a local queue, then let the outer wit-bindgen task drive it" — if waker
        // propagation across this indirection were broken, `delayed_echo` (which deliberately
        // returns `Poll::Pending` once on the host side) would never resume this guest task.
        struct LocalExecutor<F: Future> {
            queue: Rc<RefCell<Option<Pin<Box<F>>>>>,
        }
        impl<F: Future> Future for LocalExecutor<F> {
            type Output = F::Output;
            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let mut slot = self.queue.borrow_mut();
                let task = slot.as_mut().expect("local executor task missing");
                match task.as_mut().poll(cx) {
                    Poll::Ready(v) => {
                        *slot = None;
                        Poll::Ready(v)
                    }
                    Poll::Pending => Poll::Pending,
                }
            }
        }

        let queue = Rc::new(RefCell::new(Some(Box::pin(delayed_echo(
            "nested".to_string(),
        )))));
        LocalExecutor { queue }.await
    }
    // #endregion

    // #region S7 — can a SYNC-lifted export drive an ASYNC-lowered import? (coordinator follow-up
    // on the real `jobs`/`checkpoint` schema, which are plain `func`, not `async func`)
    fn s7_sync_noop(id: u32) -> u32 {
        // 🧪️ Q1a: zero import calls — isolates "can this store even invoke a plain sync export at
        // all from within a `run_concurrent` session" from "can it await an import".
        id.wrapping_add(1)
    }

    fn s7_sync_awaits_import(id: u32) -> u32 {
        // 🧪️ Q1b/Q2: a plain (non-`async fn`) Guest trait method literally cannot write `.await` —
        // this is the ONLY way a sync export body could possibly drive an async import: construct
        // the import call's future by hand and poll it in a busy-spin loop. `spins` is capped so a
        // genuine deadlock produces a bounded, evidence-carrying sentinel (`u32::MAX`) instead of
        // hanging the whole probe process forever — per S1c's own lesson, don't trust an
        // impossibly-fast or impossibly-silent result, verify it terminated for the right reason.
        let mut fut = Box::pin(s7_slow_op(id));
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        let mut spins: u32 = 0;
        loop {
            match fut.as_mut().poll(&mut cx) {
                Poll::Ready(v) => return v,
                Poll::Pending => {
                    spins += 1;
                    if spins >= 2_000_000 {
                        return u32::MAX; // sentinel: gave up after the spin cap, suspected deadlock
                    }
                }
            }
        }
    }

    async fn s7_async_awaits_import(id: u32) -> u32 {
        // 🧪️ Q3 control group: the shape that's already known to work (identical to `checkpoint`).
        s7_slow_op(id).await
    }
    // #endregion
}

export!(Component);
