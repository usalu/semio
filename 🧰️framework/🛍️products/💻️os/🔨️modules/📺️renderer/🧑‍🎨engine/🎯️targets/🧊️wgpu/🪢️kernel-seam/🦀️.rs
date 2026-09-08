//! 🪢️ Test fixture for a bounded intent/outcome mailbox on the process worker pool.
//! Its injected exchanges exercise ownership, backpressure and host wakeups.
//! Production rendering dispatches through RuntimeMailbox; this fixture has no production router.

use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use ui_contract::UiIntent;

const OUTCOME_CAPACITY: usize = 64;

//#region 🔖️KernelSeam

//#region 📨️HostWaker

/// 📨️ A cheap, `Clone`-free handle the seam calls the instant an outcome lands, so the host's event
/// loop wakes up even though it is sitting in `ControlFlow::WaitUntil`/`Wait`. Deliberately NOT the
/// raw `std::task::Waker` the pool future itself needs. This wrapper is the host-side
/// "please redraw/re-check me" signal and is safe to call from the completing worker.
#[cfg(not(target_arch = "wasm32"))]
type HostWake = Arc<dyn Fn() + Send + Sync>;

#[cfg(target_arch = "wasm32")]
type HostWake = Rc<dyn Fn()>;

#[derive(Clone)]
pub struct HostWaker(HostWake);

impl HostWaker {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(wake: impl Fn() + Send + Sync + 'static) -> Self {
        Self(Arc::new(wake))
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[cfg(target_arch = "wasm32")]
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
/// (`Box<dyn std::any::Any + Send>` — U3's own `dyn Any` carve-out) until the protocol-flip packet lands
/// `UiPatch`-shaped outcomes on the wire (master plan §3); see this file's own module docstring for
/// the honest gap this papers over. A caller downcasts via `detail.downcast_ref::<T>()`.
pub struct KernelOutcome {
    pub surface: String,
    pub detail: Box<dyn std::any::Any + Send>,
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
    /// waker fires. Intents returned from this method did not fit the lossless mailbox and remain
    /// owned by the caller for a later retry.
    fn submit_intents(&self, intents: Vec<UiIntent>) -> Vec<UiIntent>;

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
#[cfg(not(target_arch = "wasm32"))]
pub type IntentExchange = fn(UiIntent) -> Pin<Box<dyn Future<Output = KernelOutcome> + Send>>;

#[cfg(target_arch = "wasm32")]
pub type IntentExchange = fn(UiIntent) -> Pin<Box<dyn Future<Output = KernelOutcome>>>;

/// 🕳️ The honest-gap stub — see this file's own module docstring. Echoes the intent's surface back
/// with no real kernel round trip, so [`AppKernelSeam`] is constructible and its plumbing testable
/// before the real router lands.
// 🚫️async: U1 — the fn itself is sync; only the future it returns awaits, at the boundary (U1's own
// "async at boundaries only" rule), never inside a frame transaction.
#[cfg(not(target_arch = "wasm32"))]
pub fn default_intent_exchange(intent: UiIntent) -> Pin<Box<dyn Future<Output = KernelOutcome> + Send>> {
    Box::pin(async move { KernelOutcome { surface: intent.surface.0.as_str().to_string(), detail: Box::new(()) } })
}

#[cfg(target_arch = "wasm32")]
pub fn default_intent_exchange(intent: UiIntent) -> Pin<Box<dyn Future<Output = KernelOutcome>>> {
    Box::pin(async move { KernelOutcome { surface: intent.surface.0.as_str().to_string(), detail: Box::new(()) } })
}

/// 🚀️ The one [`KernelSeam`] impl (U3: no `dyn`, and — per this file's own docstring — no cfg pair
/// either). Ready outcomes and in-flight exchanges share one fixed-capacity mailbox. Commands are
/// never coalesced or evicted; callers retain and retry the returned overflow intents.
pub struct AppKernelSeam {
    outcomes: Arc<Mutex<OutcomeMailbox>>,
    waker: Arc<Mutex<Option<HostWaker>>>,
    exchange: IntentExchange,
}

impl AppKernelSeam {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(exchange: IntentExchange) -> Self {
        Self { outcomes: Arc::new(Mutex::new(OutcomeMailbox::new())), waker: Arc::new(Mutex::new(None)), exchange }
    }

    /// 🧪️ Test/host seam for inspecting queue depth without draining it.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[cfg(test)]
    pub fn pending_len(&self) -> usize {
        self.outcomes.lock().expect("kernel outcome mailbox lock").ready.len()
    }
}

struct OutcomeMailbox {
    ready: VecDeque<KernelOutcome>,
    in_flight: usize,
}

impl OutcomeMailbox {
    fn new() -> Self {
        Self { ready: VecDeque::with_capacity(OUTCOME_CAPACITY), in_flight: 0 }
    }

    fn reserve(&mut self) -> bool {
        if self.ready.len() + self.in_flight >= OUTCOME_CAPACITY {
            return false;
        }
        self.in_flight += 1;
        true
    }

    fn finish(&mut self, outcome: KernelOutcome) {
        assert!(self.in_flight > 0, "kernel outcome without reservation");
        self.in_flight -= 1;
        self.ready.push_back(outcome);
    }
}

impl KernelSeam for AppKernelSeam {
    // 🚫️async: U1 — sync fn; the future it spawns is the boundary-async exception U1 itself carves
    // out ("await between frames, never halfway through a frame").
    fn submit_intents(&self, intents: Vec<UiIntent>) -> Vec<UiIntent> {
        let mut rejected = Vec::new();
        for intent in intents {
            if !self.outcomes.lock().expect("kernel outcome mailbox lock").reserve() {
                rejected.push(intent);
                continue;
            }
            let outcomes = self.outcomes.clone();
            let waker = self.waker.clone();
            let pending = (self.exchange)(intent);
            crate::spawn_app_task(async move {
                let outcome = pending.await;
                let mut mailbox = outcomes.lock().expect("kernel outcome mailbox lock");
                mailbox.finish(outcome);
                drop(mailbox);
                if let Some(host_waker) = waker.lock().expect("kernel host waker lock").as_ref() {
                    host_waker.wake();
                }
            });
        }
        rejected
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn drain_outcomes(&self) -> Vec<KernelOutcome> {
        self.outcomes.lock().expect("kernel outcome mailbox lock").ready.drain(..).collect()
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn set_waker(&self, waker: HostWaker) {
        *self.waker.lock().expect("kernel host waker lock") = Some(waker);
    }
}

//#endregion 🚀️AppKernelSeam

//#endregion 🔖️KernelSeam

#[cfg(test)]
mod tests {
    use super::*;
    use ui_contract::{ActionId, SurfaceId, Trigger, UiNodeId, UiRevision};

    fn fake_intent(surface: &str) -> UiIntent {
        UiIntent { surface: SurfaceId::try_from(surface).expect("bounded fixture surface"), revision: UiRevision(1), seq: 1, node: UiNodeId(1), node_key: ui_contract::UiText::try_from_str("root").expect("bounded fixture node key"), trigger: Trigger::Activate, action: ActionId::default(), args: None, input: None }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn echo_exchange(intent: UiIntent) -> Pin<Box<dyn Future<Output = KernelOutcome> + Send>> {
        Box::pin(async move { KernelOutcome { surface: intent.surface.0.to_string(), detail: Box::new(intent.node_key) } })
    }

    #[cfg(target_arch = "wasm32")]
    fn echo_exchange(intent: UiIntent) -> Pin<Box<dyn Future<Output = KernelOutcome>>> {
        Box::pin(async move { KernelOutcome { surface: intent.surface.0.to_string(), detail: Box::new(intent.node_key) } })
    }

    #[test]
    fn a_fake_seam_receives_submitted_intents_and_outcomes_reach_the_host_on_wake() {
        let seam = AppKernelSeam::new(echo_exchange);
        let (woken, completion) = std::sync::mpsc::sync_channel(1);
        seam.set_waker(HostWaker::new(move || {
            let _ = woken.try_send(());
        }));

        assert!(seam.submit_intents(vec![fake_intent("surface-a")]).is_empty());
        #[cfg(not(target_arch = "wasm32"))]
        completion.recv_timeout(std::time::Duration::from_secs(2)).expect("worker completion wake");

        #[cfg(not(target_arch = "wasm32"))]
        {
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

    #[test]
    fn outcome_mailbox_is_fixed_capacity_and_returns_backpressure_without_eviction() {
        let mut mailbox = OutcomeMailbox::new();
        for _ in 0..OUTCOME_CAPACITY {
            assert!(mailbox.reserve());
        }
        assert!(!mailbox.reserve());
        mailbox.finish(KernelOutcome { surface: "surface-7".to_string(), detail: Box::new(999usize) });
        assert_eq!(mailbox.ready.len() + mailbox.in_flight, OUTCOME_CAPACITY);
        assert_eq!(*mailbox.ready.back().expect("lossless outcome").detail.downcast_ref::<usize>().expect("usize detail"), 999);
    }
}
