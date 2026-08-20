//! 🪢️ The narrow interface that ends the renderer owning the actor kernel (ticket
//! `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`, packet `os-host`). `submit_intents`/
//! `drain_outcomes`/`set_waker` are implemented over the **existing** `kernel_runtime`
//! statics/`spawn_app_task` plumbing already in `📦️glue.rs` — wrapping, not rewriting: no
//! `kernel_runtime` code below the `//#region 🔖️AppKernelSeam` line is new, only reused.
//!
//! **U3 (no `dyn KernelSeam`).** The seam's "one impl per platform" resolves to a SINGLE concrete
//! type, [`AppKernelSeam`], not a cfg-selected pair: `crate::spawn_app_task` (this crate's existing
//! `📦️glue.rs` root fn) already branches `kernel_runtime::spawn_task` (native) vs
//! `wasm_bindgen_futures::spawn_local` (wasm) internally, so [`AppKernelSeam`] built on top of it
//! needs no cfg branching of its own — a better outcome than the two-impl table in `📌️important.md`
//! anticipated, per R11's own "note it produces a better design here anyway".
//!
//! **Honest gap — `submit_intents` has no real router yet.** [`ui_contract::UiIntent`] addresses a
//! `surface: SurfaceId`, not a plugin instance `u32`; the existing `kernel_runtime::KernelClient`
//! only knows how to exchange events against an instance id, and the surface→instance map lives
//! privately inside `kernel_runtime::KernelThreadState` on the kernel thread, unreachable from here.
//! Building the real router means either exposing that map (a `kernel_runtime` change, in scope) or
//! routing through `ProgramBridge`'s own dispatch (`🧱️elements/ProgramBridge/`, **forbidden** — this
//! packet's OWNS list is new sibling files plus surgical `📦️glue.rs` edits only, never a co-location
//! element dir). [`default_intent_exchange`] is therefore an explicit stub — see its own docstring —
//! and real routing is deferred to whichever packet lands `Event::UiIntent` on the wire (master plan
//! §3, "protocol flip"). `AppKernelSeam` itself is fully wired and tested independent of that stub —
//! see this file's own tests, which pass a fake `exchange` fn.
//!
//! **Waker correctness this file exists to fix.** `kernel_runtime::poll_tasks()` today drives every
//! queued future with `Waker::noop()` — harmless under the old `ControlFlow::Poll` loop (`poll_tasks`
//! reruns every single tick regardless of any real wake signal) but a real bug once `winit_app.rs`
//! switches to `WaitUntil`/`Wait`: a pending kernel round trip's `KernelFuture` would then only get
//! re-polled to completion whenever some UNRELATED event/deadline happens to wake the loop, not
//! promptly on its own completion. [`set_waker`][KernelSeam::set_waker] plus the surgical
//! `📦️glue.rs` edit installing a real `std::task::Waker` (built from `ui_host::WakeProxy`, itself
//! `Send + Sync` — exactly the cross-thread wake transport this needs) into `kernel_runtime` closes
//! that gap; see `📓️terra-os-host-report.md`'s redraw audit for the exact edit.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use ui_contract::UiIntent;

//#region 🔖️KernelSeam

//#region 📨️HostWaker

/// 📨️ A cheap, `Clone`-free handle the seam calls the instant an outcome lands, so the host's event
/// loop wakes up even though it is sitting in `ControlFlow::WaitUntil`/`Wait`. Deliberately NOT the
/// raw `std::task::Waker` `KernelFuture` itself needs (that one must be `Send + Sync` to be called
/// from the kernel thread inside `ResponseSlot::deliver` — see `kernel_seam::install` in
/// `os_host.rs`); this wrapper is the winit-thread-side "please redraw/re-check me" signal a
/// [`KernelSeam`] implementation calls after pushing into its own outcome queue, always from the
/// thread that owns that queue.
pub struct HostWaker(Rc<dyn Fn()>);

impl HostWaker {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(wake: impl Fn() + 'static) -> Self {
        Self(Rc::new(wake))
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn wake(&self) {
        (self.0)()
    }
}

//#endregion 📨️HostWaker

//#region 📤️KernelOutcome

/// 📤️ What one completed kernel round trip hands back to the host. `detail` is deliberately opaque
/// (`Box<dyn std::any::Any>` — U3's own `dyn Any` carve-out) until the protocol-flip packet lands
/// `UiPatch`-shaped outcomes on the wire (master plan §3); see this file's own module docstring for
/// the honest gap this papers over. A caller downcasts via `detail.downcast_ref::<T>()`.
pub struct KernelOutcome {
    pub surface: String,
    pub detail: Box<dyn std::any::Any>,
}

//#endregion 📤️KernelOutcome

//#region 🧩️KernelSeam trait

/// 🧩️ `KernelSeam { submit_intents, drain_outcomes, set_waker }` — the exact shape ticket
/// `26/08/20/…/📋️master.md` §5 specifies. No `async fn` (U1: this trait's methods are called from
/// inside `winit_app.rs`'s sync `ApplicationHandler` callbacks); the kernel round trip itself still
/// genuinely awaits, just entirely behind this interface.
pub trait KernelSeam {
    /// 📮️ Enqueues `intents` for exchange with the kernel; returns immediately, never blocks the
    /// caller. Results surface later through [`Self::drain_outcomes`], after [`Self::set_waker`]'s
    /// waker fires.
    fn submit_intents(&self, intents: Vec<UiIntent>);

    /// 📭️ Drains every outcome that has landed since the last call — called on wake, never per-frame.
    fn drain_outcomes(&self) -> Vec<KernelOutcome>;

    /// 🔔️ Installs the waker this seam calls the instant an outcome becomes available.
    fn set_waker(&self, waker: HostWaker);
}

//#endregion 🧩️KernelSeam trait

//#region 🚀️AppKernelSeam

/// 🚀️ A single kernel-round-trip step: takes an intent, returns a future producing its outcome. A
/// bare `fn` pointer (not `Fn`/`dyn Fn`) is intentional and idiomatic here (this crate already leans
/// on file-scoped statics/thread-locals for the same reason `KernelClient::get()`'s `OnceLock` does)
/// — the real implementation reaches for `kernel_runtime::KernelClient::get()` plus whatever
/// surface→instance lookup eventually lands, rather than closing over captured state.
pub type IntentExchange = fn(UiIntent) -> Pin<Box<dyn Future<Output = KernelOutcome>>>;

/// 🕳️ The honest-gap stub — see this file's own module docstring. Echoes the intent's surface back
/// with no real kernel round trip, so [`AppKernelSeam`] is constructible and its plumbing testable
/// before the real router lands.
// 🚫️async: U1 — the fn itself is sync; only the future it returns awaits, at the boundary (U1's own
// "async at boundaries only" rule), never inside a frame transaction.
pub fn default_intent_exchange(intent: UiIntent) -> Pin<Box<dyn Future<Output = KernelOutcome>>> {
    Box::pin(async move { KernelOutcome { surface: intent.surface.0, detail: Box::new(()) } })
}

/// 🚀️ The one [`KernelSeam`] impl (U3: no `dyn`, and — per this file's own docstring — no cfg pair
/// either). Outcomes accumulate in an `Rc<RefCell<VecDeque<_>>>` shared with every in-flight
/// `spawn_app_task` future this seam spawned; `drain_outcomes` empties it, `set_waker` stores the
/// wake callback each future's completion calls after pushing its result.
pub struct AppKernelSeam {
    outcomes: Rc<RefCell<VecDeque<KernelOutcome>>>,
    waker: Rc<RefCell<Option<HostWaker>>>,
    exchange: IntentExchange,
}

impl AppKernelSeam {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(exchange: IntentExchange) -> Self {
        Self { outcomes: Rc::new(RefCell::new(VecDeque::new())), waker: Rc::new(RefCell::new(None)), exchange }
    }

    /// 🧪️ Test/host seam for inspecting queue depth without draining it.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn pending_len(&self) -> usize {
        self.outcomes.borrow().len()
    }
}

impl KernelSeam for AppKernelSeam {
    // 🚫️async: U1 — sync fn; the future it spawns is the boundary-async exception U1 itself carves
    // out ("await between frames, never halfway through a frame").
    fn submit_intents(&self, intents: Vec<UiIntent>) {
        for intent in intents {
            let outcomes = self.outcomes.clone();
            let waker = self.waker.clone();
            let pending = (self.exchange)(intent);
            crate::spawn_app_task(async move {
                let outcome = pending.await;
                outcomes.borrow_mut().push_back(outcome);
                if let Some(host_waker) = waker.borrow().as_ref() {
                    host_waker.wake();
                }
            });
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn drain_outcomes(&self) -> Vec<KernelOutcome> {
        self.outcomes.borrow_mut().drain(..).collect()
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn set_waker(&self, waker: HostWaker) {
        *self.waker.borrow_mut() = Some(waker);
    }
}

//#endregion 🚀️AppKernelSeam

//#endregion 🔖️KernelSeam

#[cfg(test)]
mod tests {
    use super::*;
    use ui_contract::{ActionId, SurfaceId, Trigger, UiNodeId, UiRevision};

    fn fake_intent(surface: &str) -> UiIntent {
        UiIntent {
            surface: SurfaceId(surface.to_string()),
            revision: UiRevision(1),
            node: UiNodeId(1),
            node_key: "root".to_string(),
            trigger: Trigger::Activate,
            action: ActionId::default(),
            args: None,
            input: None,
        }
    }

    fn echo_exchange(intent: UiIntent) -> Pin<Box<dyn Future<Output = KernelOutcome>>> {
        Box::pin(async move { KernelOutcome { surface: intent.surface.0, detail: Box::new(intent.node_key) } })
    }

    #[test]
    fn a_fake_seam_receives_submitted_intents_and_outcomes_reach_the_host_on_wake() {
        let seam = AppKernelSeam::new(echo_exchange);
        let woken = Rc::new(RefCell::new(false));
        let woken_clone = woken.clone();
        seam.set_waker(HostWaker::new(move || *woken_clone.borrow_mut() = true));

        seam.submit_intents(vec![fake_intent("surface-a")]);
        assert_eq!(seam.pending_len(), 0, "the outcome has not landed yet — the future is still queued");

        // 🌀️ `spawn_app_task` on native routes through `kernel_runtime::spawn_task`'s thread-local
        // pool; polling it directly here (rather than via `about_to_wait`) is the same pattern this
        // crate's own `poll_tasks` doc comment describes.
        #[cfg(not(target_arch = "wasm32"))]
        crate::kernel_runtime::poll_tasks();

        #[cfg(not(target_arch = "wasm32"))]
        {
            assert!(*woken.borrow(), "the waker must fire once the outcome lands");
            let outcomes = seam.drain_outcomes();
            assert_eq!(outcomes.len(), 1);
            assert_eq!(outcomes[0].surface, "surface-a");
            assert_eq!(seam.pending_len(), 0, "drain_outcomes empties the queue");
        }
    }

    #[test]
    fn drain_outcomes_is_empty_with_nothing_submitted() {
        let seam = AppKernelSeam::new(default_intent_exchange);
        assert!(seam.drain_outcomes().is_empty());
    }
}
