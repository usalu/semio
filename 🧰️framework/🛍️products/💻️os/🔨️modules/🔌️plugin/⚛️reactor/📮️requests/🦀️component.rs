//! 📮️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2, design-abi.md §4). `host::request(build) ->
//! impl Future<Output = Result<Vec<u8>, Fault>>` — allocates a `RequestId` FIRST (so `build` can
//! embed it into the `Effect` it constructs — every completable `Effect` variant carries its own
//! `req` field), pushes the effect onto the outbound queue `reactor::poll` drains into
//! `turn-result.effects`, and parks the calling task's waker until `Event::Completed{req, result}`
//! (or `Event::HttpChunk`/`JobProgress`/`JobCompleted` for the streaming variants) arrives on a
//! later `poll` call and resolves it.
//!
//! Uses the REAL `semio_framework::kernel::{Effect, RequestId}` (packet A3 landed these in
//! `🎠️kernel/🦀️component.rs` while this packet was in flight — confirmed present via
//! `grep -n "^pub enum Effect" 🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs` before this file was
//! written) — not a stand-in.

use semio_framework::kernel::{Effect, RequestId};
use semio_framework::Fault;
use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

enum Slot {
    Pending { waker: Option<Waker>, partial: Vec<u8> },
    Ready { result: Result<Vec<u8>, Fault> },
}

#[derive(Default)]
struct Inner {
    next_id: u64,
    slots: HashMap<u64, Slot>,
    /// 📤️ Effects allocated via `request()` since the last `drain()` — `reactor::poll` moves these
    /// into `turn-result.effects` (subject to `budget.max-effects`; overflow carries over to the
    /// next turn, see design-abi.md §4's `EffectSink` note).
    outbound: Vec<Effect>,
    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): which instance allocated
    /// each still-tracked request id — every `RequestFuture` is created inside an `AsyncTask`'s
    /// `TaskCtx.host` (no plugin code can await anything outside a spawned task), so this is
    /// populated purely from the `RequestRegistry` HANDLE's own `instance` tag (see
    /// `for_instance`), never guessed from timing. Read by `cancel_instance` on `Event::
    /// InstanceClose` so a dying instance's pending host round-trips leave no slot behind.
    instance_of: HashMap<u64, u32>,
}

impl Inner {
    /// 🔁️ The replace-and-take-waker step `resolve`/`append_chunk` both need — factored out so the
    /// chunk-reassembly cap/done paths reuse the EXACT same resolution mechanics `resolve` already
    /// had, rather than a second hand-rolled copy.
    // 🚫️async: E1 pure in-memory slot mutation consumed by `RequestRegistry`'s sync API below — R9.
    fn complete(&mut self, id: u64, result: Result<Vec<u8>, Fault>) -> Option<Waker> {
        match self.slots.get_mut(&id) {
            Some(slot @ Slot::Pending { .. }) => {
                let Slot::Pending { waker, .. } = std::mem::replace(slot, Slot::Ready { result }) else { unreachable!() };
                waker
            }
            _ => None,
        }
    }
}

/// 📮️ One shared queue per actor (today: one actor per app instance is the default granularity —
/// design-abi.md §4 — so `instance_of` is future-proofing for the opt-in multi-instance actor, not
/// a change in today's fan-out). `instance`: which instance THIS HANDLE tags newly allocated
/// requests with — `RequestRegistry::new()`/`Default` default to instance 0 (the bare per-actor
/// registry `⚛️reactor`'s `REGISTRY` thread-local holds); `for_instance` mints a handle sharing the
/// SAME underlying queue but tagging its own allocations, which is what
/// `⚛️reactor::host_for_instance(instance)` hands to each `TaskCtx`.
#[derive(Clone, Default)]
pub struct RequestRegistry {
    inner: Rc<RefCell<Inner>>,
    instance: u32,
}

impl RequestRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// 🧵️ A handle over the SAME underlying queue/slots (ids stay globally unique — there is still
    /// only one counter), but every request IT allocates is tagged as belonging to `instance`. See
    /// the struct doc for why this exists instead of a per-request `instance` parameter on
    /// `request()` itself: `request()`'s signature is a live cross-crate contract
    /// (`🌐host/🦀️component.rs`'s `Host::call`) this packet must not break.
    pub fn for_instance(&self, instance: u32) -> Self {
        Self { inner: self.inner.clone(), instance }
    }

    /// 🔮️ Allocates a `RequestId`, calls `build(id)` to construct the `Effect` (every completable
    /// variant embeds its own `req: RequestId`), queues it for the next `turn-result.effects`
    /// drain, and returns a future that resolves once `resolve(id, ...)` is called from the
    /// `Event::Completed` (or job/http-chunk) routing step of `poll`.
    pub fn request(&self, build: impl FnOnce(RequestId) -> Effect) -> RequestFuture {
        let mut inner = self.inner.borrow_mut();
        inner.next_id += 1;
        let raw = inner.next_id;
        inner.slots.insert(raw, Slot::Pending { waker: None, partial: Vec::new() });
        inner.instance_of.insert(raw, self.instance);
        inner.outbound.push(build(RequestId(raw)));
        RequestFuture { registry: self.inner.clone(), id: raw }
    }

    /// ✅️ Called from `poll`'s event-routing step when `Event::Completed{req, result}` (or an
    /// equivalent streaming completion) arrives. Wakes the parked task if one was already polled
    /// once (a request resolved before its future is ever polled just sits `Ready`).
    pub fn resolve(&self, id: RequestId, result: Result<Vec<u8>, Fault>) {
        let mut inner = self.inner.borrow_mut();
        let waker = inner.complete(id.0, result);
        drop(inner);
        if let Some(waker) = waker {
            waker.wake();
        }
    }

    /// 🌊️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): accumulates one `Event::HttpChunk`
    /// body chunk into `id`'s partial buffer instead of discarding every non-final chunk (the bug
    /// `⚛️reactor/🦀️component.rs`'s `Event::HttpChunk` routing step used to have — it kept only the
    /// LAST chunk's `bytes`, silently dropping every earlier one). `cap` is the owning instance's
    /// `QuotaSchema.message_bytes` (default 64 MiB, resolved by the caller via `instance_of` +
    /// `INSTANCE_QUOTAS` — this registry has no quota table of its own): exceeding it resolves the
    /// request with a typed fault immediately rather than silently truncating, and it stays resolved
    /// (a fault is terminal) even if more chunks for the same `id` arrive afterward — the underlying
    /// HTTP task is not itself cancelled by this, it just has nowhere left to deliver into. A chunk
    /// for an id that is not `Pending` (already resolved, or cancelled/dropped — see `RequestFuture`'s
    /// `Drop` impl) is a harmless no-op, same as `resolve` on an unknown id.
    pub fn append_chunk(&self, id: RequestId, bytes: Vec<u8>, done: bool, cap: usize) {
        let mut inner = self.inner.borrow_mut();
        let outcome = match inner.slots.get_mut(&id.0) {
            Some(Slot::Pending { partial, .. }) => {
                partial.extend_from_slice(&bytes);
                if partial.len() > cap {
                    Some(Err(Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("plugin.request-registry.body-too-large"), format!("http/blob body exceeded the {cap}-byte instance quota (message_bytes)"))))
                } else if done {
                    Some(Ok(std::mem::take(partial)))
                } else {
                    None
                }
            }
            _ => None,
        };
        let waker = match outcome {
            Some(result) => inner.complete(id.0, result),
            None => None,
        };
        drop(inner);
        if let Some(waker) = waker {
            waker.wake();
        }
    }

    /// 🏷️ Which instance's `for_instance` handle allocated `id` — `append_chunk`'s caller
    /// (`⚛️reactor/🦀️component.rs`'s `Event::HttpChunk` routing step) uses this to look up the
    /// owning instance's `QuotaSchema.message_bytes` cap. `None` once the slot is gone (already
    /// resolved or cancelled), same lifetime as every other per-id lookup here.
    pub fn instance_of(&self, id: RequestId) -> Option<u32> {
        self.inner.borrow().instance_of.get(&id.0).copied()
    }

    /// 🔥️ Fire-and-forget: queues `effect` (a variant with no `req`/no completion — `Notify`,
    /// `ClipboardWrite`, `Navigate`, ...) without allocating a `RequestId` or parking anything.
    pub fn emit(&self, effect: Effect) {
        self.inner.borrow_mut().outbound.push(effect);
    }

    /// 📤️ Drains and returns every effect queued since the last drain, in request order —
    /// `reactor::poll` calls this once per turn after the executor idles.
    pub fn drain(&self) -> Vec<Effect> {
        std::mem::take(&mut self.inner.borrow_mut().outbound)
    }

    /// 📸️ `⚛️reactor/📸️checkpoint`'s `pending_requests`: the ids still `Pending` — carried in the
    /// checkpoint pack so a restored actor's host round-trips can be identified as stale/re-run
    /// (see design-abi.md §4: async tasks are never serialised, only marked re-run-on-restore).
    pub fn pending_ids(&self) -> Vec<RequestId> {
        self.inner.borrow().slots.iter().filter(|(_, slot)| matches!(slot, Slot::Pending { .. })).map(|(id, _)| RequestId(*id)).collect()
    }

    /// 🚫️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): `Event::InstanceClose`
    /// cancellation — removes every slot (`Pending` or an already-`Ready`-but-never-polled result)
    /// tagged as belonging to `instance`. No wake/notify: `⚛️reactor::cancel_instance_tasks` drops
    /// that instance's owning `AsyncTask` futures from the `LocalExecutor` in the SAME step (see
    /// that function's call site in `poll`'s `Event::InstanceClose` handling), so nothing is left
    /// to observe a "cancelled" resolution — this is cleanup, not notification. Returns the number
    /// of slots removed (diagnostic only). Idempotent: an instance with no pending requests removes
    /// nothing.
    pub fn cancel_instance(&self, instance: u32) -> usize {
        let mut inner = self.inner.borrow_mut();
        let ids: Vec<u64> = inner.instance_of.iter().filter(|(_, owner)| **owner == instance).map(|(id, _)| *id).collect();
        for id in &ids {
            inner.slots.remove(id);
            inner.instance_of.remove(id);
        }
        ids.len()
    }
}

/// ⏳️ Awaiting this future is how a `host::*` call parks — see `pure.wit`'s doc comment for why
/// only `log`/`now-ms`/`trace-span` stay synchronous and everything else goes through here.
pub struct RequestFuture {
    registry: Rc<RefCell<Inner>>,
    id: u64,
}

impl Future for RequestFuture {
    type Output = Result<Vec<u8>, Fault>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.registry.borrow_mut();
        match inner.slots.remove(&self.id) {
            Some(Slot::Ready { result }) => {
                inner.instance_of.remove(&self.id);
                Poll::Ready(result)
            }
            Some(Slot::Pending { partial, .. }) => {
                inner.slots.insert(self.id, Slot::Pending { waker: Some(cx.waker().clone()), partial });
                Poll::Pending
            }
            None => Poll::Ready(Err(Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("plugin.request-registry"), "request already consumed or unknown".to_string()))),
        }
    }
}

/// 🚫️ Drop-is-cancellation: releasing a `RequestFuture` without ever resolving it frees its slot
/// and its instance tag. This is what makes dropping a task's future a COMPLETE cancellation —
/// `⚛️reactor::cancel_instance_tasks` and the key-dedupe replacement path both cancel purely by
/// dropping the owning future, and without this impl each one silently leaked a `Pending` slot plus
/// one unit of the instance's `outstanding_requests` quota, so a plugin that re-keyed a task often
/// enough would eventually be refused its own quota with nothing pending. Mirrors the host side's
/// `CancelOnDrop` guard and the reference architecture's "guest future-drop => cancellation".
///
/// 🔒️ `borrow_mut` is sound here because no code path holds a registry borrow across a point where
/// a `RequestFuture` can be dropped — `resolve` deliberately releases its guard before `wake()`,
/// and `poll` returns its guard before the caller can drop the future.
impl Drop for RequestFuture {
    fn drop(&mut self) {
        let mut inner = self.registry.borrow_mut();
        inner.slots.remove(&self.id);
        inner.instance_of.remove(&self.id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn resolve_before_first_poll_leaves_the_future_immediately_ready() {
        let registry = RequestRegistry::new();
        let future = registry.request(|req| Effect::CancelJob { job: req.0 }).await;
        assert_eq!(registry.drain().len(), 1, "request() must queue exactly one effect");
        registry.resolve(RequestId(1), Ok(b"ok".to_vec()));
        let mut future = Box::pin(future);
        let waker = futures_test_waker();
        let mut cx = Context::from_waker(waker);
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(Ok(bytes)) => assert_eq!(bytes, b"ok"),
            Poll::Ready(Err(fault)) => panic!("expected Ok, got a fault: {fault:?}"),
            Poll::Pending => panic!("a request resolved before its first poll must be immediately ready"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn append_chunk_reassembles_a_multi_chunk_body_to_the_exact_original_bytes() {
        let registry = RequestRegistry::new();
        let future = registry.request(|req| Effect::CancelJob { job: req.0 }).await;
        let original: Vec<u8> = (0u8..=255).chain(0u8..=255).chain(0u8..100).collect(); // 710 bytes, non-trivial and non-uniform
        for (index, window) in original.chunks(97).enumerate() {
            let done = (index + 1) * 97 >= original.len();
            registry.append_chunk(RequestId(1), window.to_vec(), done, 1024 * 1024);
        }
        let mut future = Box::pin(future);
        let waker = futures_test_waker();
        let mut cx = Context::from_waker(waker);
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(Ok(bytes)) => assert_eq!(bytes, original, "every chunk (not just the final one) must survive reassembly"),
            Poll::Ready(Err(fault)) => panic!("expected Ok, got a fault: {fault:?}"),
            Poll::Pending => panic!("the request must be Ready once the final (done) chunk arrived"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn append_chunk_over_cap_faults_instead_of_silently_truncating() {
        let registry = RequestRegistry::new();
        let future = registry.request(|req| Effect::CancelJob { job: req.0 }).await;
        registry.append_chunk(RequestId(1), vec![0u8; 40], false, 64);
        registry.append_chunk(RequestId(1), vec![0u8; 40], false, 64); // 80 > 64 cap — must fault here, not wait for `done`
        let mut future = Box::pin(future);
        let waker = futures_test_waker();
        let mut cx = Context::from_waker(waker);
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(Err(fault)) => assert_eq!(fault.code.0, "plugin.request-registry.body-too-large"),
            Poll::Ready(Ok(bytes)) => panic!("must fault over cap, not silently truncate — got {} bytes", bytes.len()),
            Poll::Pending => panic!("the over-cap chunk must resolve the request immediately, not wait for `done`"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn append_chunk_on_an_unknown_or_already_resolved_id_is_a_harmless_no_op() {
        let registry = RequestRegistry::new();
        registry.append_chunk(RequestId(999), vec![1, 2, 3], false, 1024); // never requested
        let future = registry.request(|req| Effect::CancelJob { job: req.0 }).await;
        registry.resolve(RequestId(1), Ok(b"already done".to_vec()));
        registry.append_chunk(RequestId(1), vec![9, 9, 9], true, 1024); // arrives after resolve
        let mut future = Box::pin(future);
        let waker = futures_test_waker();
        let mut cx = Context::from_waker(waker);
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(Ok(bytes)) => assert_eq!(bytes, b"already done", "a late chunk must not clobber an already-resolved result"),
            other => panic!("expected the original resolution to stand, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn pending_ids_reports_only_unresolved_requests() {
        let registry = RequestRegistry::new();
        let _first = registry.request(|req| Effect::CancelJob { job: req.0 }).await;
        let _second = registry.request(|req| Effect::CancelJob { job: req.0 }).await;
        assert_eq!(registry.pending_ids().len(), 2);
        registry.resolve(RequestId(1), Ok(Vec::new()));
        assert_eq!(registry.pending_ids(), vec![RequestId(2)]);
    }

    #[semio_framework_async_macros::async_test]
    async fn cancel_instance_removes_only_that_instances_pending_requests() {
        let registry = RequestRegistry::new();
        let scoped_to_7 = registry.for_instance(7);
        let scoped_to_9 = registry.for_instance(9);
        let _seven_a = scoped_to_7.request(|req| Effect::CancelJob { job: req.0 }).await;
        let _seven_b = scoped_to_7.request(|req| Effect::CancelJob { job: req.0 }).await;
        let nine = scoped_to_9.request(|req| Effect::CancelJob { job: req.0 }).await;

        assert_eq!(registry.pending_ids().len(), 3, "all three requests share the one underlying queue");
        let removed = registry.cancel_instance(7);
        assert_eq!(removed, 2, "cancel_instance must report exactly the count it removed");
        assert_eq!(registry.pending_ids(), vec![RequestId(3)], "only instance 9's request must survive");

        // 🚫️ A cancelled instance's future observes neither Ready nor a wake — it simply never
        // resolves. The slot is gone outright (no leaked entry to poll against later).
        let mut nine = Box::pin(nine);
        let waker = futures_test_waker();
        let mut cx = Context::from_waker(waker);
        assert!(matches!(nine.as_mut().poll(&mut cx), Poll::Pending), "the surviving instance's request is unaffected");
    }

    #[semio_framework_async_macros::async_test]
    async fn cancel_instance_on_an_instance_with_no_pending_requests_is_a_harmless_no_op() {
        let registry = RequestRegistry::new();
        assert_eq!(registry.cancel_instance(42), 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn for_instance_shares_the_same_id_counter_as_the_registry_it_was_derived_from() {
        let registry = RequestRegistry::new();
        let scoped = registry.for_instance(3);
        let _first = registry.request(|req| Effect::CancelJob { job: req.0 }).await; // id 1, instance 0
        let _second = scoped.request(|req| Effect::CancelJob { job: req.0 }).await; // id 2, instance 3
        assert_eq!(registry.cancel_instance(3), 1, "only the request allocated through the scoped handle belongs to instance 3");
        assert_eq!(registry.pending_ids(), vec![RequestId(1)]);
    }

    // 🚫️async: E4 fn-pointer slot — Waker::noop() replaces the hand-rolled RawWakerVTable outright;
    // no vtable fn-pointer slot survives to tag.
    fn futures_test_waker() -> &'static Waker {
        Waker::noop()
    }
}
