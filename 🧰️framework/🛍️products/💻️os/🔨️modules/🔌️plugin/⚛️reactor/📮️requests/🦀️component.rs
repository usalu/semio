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
    Pending { waker: Option<Waker> },
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
}

/// 📮️ One per app instance (never shared across instances — a revoked capability or a fault must
/// only ever resolve the requests the OWNING instance is actually waiting on).
#[derive(Clone, Default)]
pub struct RequestRegistry {
    inner: Rc<RefCell<Inner>>,
}

impl RequestRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// 🔮️ Allocates a `RequestId`, calls `build(id)` to construct the `Effect` (every completable
    /// variant embeds its own `req: RequestId`), queues it for the next `turn-result.effects`
    /// drain, and returns a future that resolves once `resolve(id, ...)` is called from the
    /// `Event::Completed` (or job/http-chunk) routing step of `poll`.
    pub fn request(&self, build: impl FnOnce(RequestId) -> Effect) -> RequestFuture {
        let mut inner = self.inner.borrow_mut();
        inner.next_id += 1;
        let raw = inner.next_id;
        inner.slots.insert(raw, Slot::Pending { waker: None });
        inner.outbound.push(build(RequestId(raw)));
        RequestFuture { registry: self.inner.clone(), id: raw }
    }

    /// ✅️ Called from `poll`'s event-routing step when `Event::Completed{req, result}` (or an
    /// equivalent streaming completion) arrives. Wakes the parked task if one was already polled
    /// once (a request resolved before its future is ever polled just sits `Ready`).
    pub fn resolve(&self, id: RequestId, result: Result<Vec<u8>, Fault>) {
        let mut inner = self.inner.borrow_mut();
        let waker = match inner.slots.get_mut(&id.0) {
            Some(slot @ Slot::Pending { .. }) => {
                let Slot::Pending { waker } = std::mem::replace(slot, Slot::Ready { result }) else { unreachable!() };
                waker
            }
            _ => return,
        };
        drop(inner);
        if let Some(waker) = waker {
            waker.wake();
        }
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
            Some(Slot::Ready { result }) => Poll::Ready(result),
            Some(Slot::Pending { .. }) => {
                inner.slots.insert(self.id, Slot::Pending { waker: Some(cx.waker().clone()) });
                Poll::Pending
            }
            None => Poll::Ready(Err(Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("plugin.request-registry"), "request already consumed or unknown".to_string()))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_before_first_poll_leaves_the_future_immediately_ready() {
        let registry = RequestRegistry::new();
        let future = registry.request(|req| Effect::CancelJob { job: req.0 });
        assert_eq!(registry.drain().len(), 1, "request() must queue exactly one effect");
        registry.resolve(RequestId(1), Ok(b"ok".to_vec()));
        let mut future = Box::pin(future);
        let waker = futures_test_waker();
        let mut cx = Context::from_waker(&waker);
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(Ok(bytes)) => assert_eq!(bytes, b"ok"),
            Poll::Ready(Err(fault)) => panic!("expected Ok, got a fault: {fault:?}"),
            Poll::Pending => panic!("a request resolved before its first poll must be immediately ready"),
        }
    }

    #[test]
    fn pending_ids_reports_only_unresolved_requests() {
        let registry = RequestRegistry::new();
        let _first = registry.request(|req| Effect::CancelJob { job: req.0 });
        let _second = registry.request(|req| Effect::CancelJob { job: req.0 });
        assert_eq!(registry.pending_ids().len(), 2);
        registry.resolve(RequestId(1), Ok(Vec::new()));
        assert_eq!(registry.pending_ids(), vec![RequestId(2)]);
    }

    fn futures_test_waker() -> Waker {
        use std::task::{RawWaker, RawWakerVTable};
        fn noop(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            RawWaker::new(std::ptr::null(), &VTABLE)
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
        unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
    }
}
