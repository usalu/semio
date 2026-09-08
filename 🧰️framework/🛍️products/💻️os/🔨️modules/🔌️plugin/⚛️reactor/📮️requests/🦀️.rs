//! 📮️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2, design-abi.md §4). `host::request(build) ->
//! impl Future<Output = Result<Vec<u8>, Fault>>` — allocates a `RequestId` FIRST (so `build` can
//! embed it into the `Effect` it constructs — every completable `Effect` variant carries its own
//! `req` field), pushes the effect onto the outbound queue `reactor::poll` drains into
//! `turn-result.effects`, and parks the calling task's waker until `Event::Completed{req, result}`
//! (or `Event::HttpChunk`/`JobProgress`/`JobCompleted` for the streaming variants) arrives on a
//! later `poll` call and resolves it.
//!
//! Uses the REAL `semio_framework::kernel::{Effect, RequestId}` (packet A3 landed these in
//! `🎠️kernel/🦀️.rs` while this packet was in flight — confirmed present via
//! `grep -n "^pub enum Effect" 🧰️framework/🔨️modules/🎠️kernel/🦀️.rs` before this file was
//! written) — not a stand-in.

use semio_framework::kernel::{Effect, RequestId};
use semio_framework::Fault;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

enum Slot {
    Pending { waker: Option<Waker>, partial: Vec<u8> },
    Ready { result: Result<Vec<u8>, Fault> },
}

const REQUEST_SLOTS: usize = 1_024;
const REQUEST_OUTBOUND_SLOTS: usize = 1_024;
const REQUEST_SLOT_WORDS: usize = REQUEST_SLOTS / u64::BITS as usize;

struct SlotEntry {
    id: u64,
    instance: u32,
    value: Slot,
}

struct Inner {
    next_id: u64,
    slots: Box<[std::mem::MaybeUninit<SlotEntry>]>,
    occupied: [u64; REQUEST_SLOT_WORDS],
    /// 📤️ Effects allocated via `request()` since the last `drain()` — `reactor::poll` moves these
    /// into `turn-result.effects` (subject to `budget.max-effects`; overflow carries over to the
    /// next turn, see design-abi.md §4's `EffectSink` note).
    outbound: std::mem::ManuallyDrop<VecDeque<(u32, Effect)>>,
    allocation_admitted: bool,
}

impl Inner {
    fn new() -> Self {
        let mut slots = Vec::new();
        let slots_admitted = slots.try_reserve_exact(REQUEST_SLOTS).is_ok();
        if slots_admitted {
            slots.resize_with(REQUEST_SLOTS, std::mem::MaybeUninit::uninit);
        }
        let mut outbound = VecDeque::new();
        let outbound_admitted = outbound.try_reserve_exact(REQUEST_OUTBOUND_SLOTS).is_ok();
        Self { next_id: 0, slots: slots.into_boxed_slice(), occupied: [0; REQUEST_SLOT_WORDS], outbound: std::mem::ManuallyDrop::new(outbound), allocation_admitted: slots_admitted && outbound_admitted }
    }

    fn index(id: u64) -> usize {
        id as usize % REQUEST_SLOTS
    }

    fn occupied(&self, index: usize) -> bool {
        self.occupied[index / u64::BITS as usize] & (1u64 << (index % u64::BITS as usize)) != 0
    }

    fn set_occupied(&mut self, index: usize, occupied: bool) {
        let word = &mut self.occupied[index / u64::BITS as usize];
        let mask = 1u64 << (index % u64::BITS as usize);
        if occupied {
            *word |= mask;
        } else {
            *word &= !mask;
        }
    }

    fn get_at(&self, index: usize) -> Option<&SlotEntry> {
        if !self.occupied(index) {
            return None;
        }
        self.slots.get(index).map(|slot| {
            // SAFETY: occupancy is set only after `write` and cleared before `assume_init_read`.
            unsafe { slot.assume_init_ref() }
        })
    }

    fn get_at_mut(&mut self, index: usize) -> Option<&mut SlotEntry> {
        if !self.occupied(index) {
            return None;
        }
        self.slots.get_mut(index).map(|slot| {
            // SAFETY: occupancy is set only after `write` and cleared before `assume_init_read`.
            unsafe { slot.assume_init_mut() }
        })
    }

    fn get(&self, id: u64) -> Option<&SlotEntry> {
        self.get_at(Self::index(id)).filter(|entry| entry.id == id)
    }

    fn get_mut(&mut self, id: u64) -> Option<&mut SlotEntry> {
        self.get_at_mut(Self::index(id)).filter(|entry| entry.id == id)
    }

    fn take(&mut self, id: u64) -> Option<SlotEntry> {
        let index = Self::index(id);
        if self.get_at(index).is_none_or(|entry| entry.id != id) {
            return None;
        }
        self.set_occupied(index, false);
        // SAFETY: exact identity and occupancy were checked, and occupancy is now cleared.
        Some(unsafe { self.slots[index].assume_init_read() })
    }

    fn insert_admitted(&mut self, entry: SlotEntry) {
        let index = Self::index(entry.id);
        debug_assert!(self.allocation_admitted && !self.occupied(index));
        self.slots[index].write(entry);
        self.set_occupied(index, true);
    }

    /// 🔁️ The replace-and-take-waker step `resolve`/`append_chunk` both need — factored out so the
    /// chunk-reassembly cap/done paths reuse the EXACT same resolution mechanics `resolve` already
    /// had, rather than a second hand-rolled copy.
    // 🚫️async: E1 pure in-memory slot mutation consumed by `RequestRegistry`'s sync API below — R9.
    fn complete(&mut self, id: u64, result: Result<Vec<u8>, Fault>) -> Option<Waker> {
        match self.get_mut(id).map(|entry| &mut entry.value) {
            Some(slot @ Slot::Pending { .. }) => {
                let Slot::Pending { waker, .. } = std::mem::replace(slot, Slot::Ready { result }) else { unreachable!() };
                waker
            }
            _ => None,
        }
    }
}

impl Drop for Inner {
    fn drop(&mut self) {}
}

/// 📮️ One shared queue per actor (today: one actor per app instance is the default granularity —
/// design-abi.md §4 — so `instance_of` is future-proofing for the opt-in multi-instance actor, not
/// a change in today's fan-out). `instance`: which instance THIS HANDLE tags newly allocated
/// requests with — `RequestRegistry::new()`/`Default` default to instance 0 (the bare per-actor
/// registry `⚛️reactor`'s `REGISTRY` thread-local holds); `for_instance` mints a handle sharing the
/// SAME underlying queue but tagging its own allocations, which is what
/// `⚛️reactor::host_for_instance(instance)` hands to each `TaskCtx`.
#[derive(Clone)]
pub struct RequestRegistry {
    inner: Rc<RefCell<Inner>>,
    instance: u32,
}

impl Default for RequestRegistry {
    fn default() -> Self {
        Self { inner: Rc::new(RefCell::new(Inner::new())), instance: 0 }
    }
}

pub struct RequestCloseCursor {
    instance: u32,
    index: usize,
    outbound_remaining: usize,
    outbound_initialized: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RequestCloseStep {
    Pending,
    Complete,
}

impl RequestRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// 🧵️ A handle over the SAME underlying queue/slots (ids stay globally unique — there is still
    /// only one counter), but every request IT allocates is tagged as belonging to `instance`. See
    /// the struct doc for why this exists instead of a per-request `instance` parameter on
    /// `request()` itself: `request()`'s signature is a live cross-crate contract
    /// (`🌐host/🦀️.rs`'s `Host::call`) this packet must not break.
    pub fn for_instance(&self, instance: u32) -> Self {
        Self { inner: self.inner.clone(), instance }
    }

    /// 🔮️ Allocates a `RequestId`, calls `build(id)` to construct the `Effect` (every completable
    /// variant embeds its own `req: RequestId`), queues it for the next `turn-result.effects`
    /// drain, and returns a future that resolves once `resolve(id, ...)` is called from the
    /// `Event::Completed` (or job/http-chunk) routing step of `poll`.
    pub fn request(&self, build: impl FnOnce(RequestId) -> Effect) -> RequestFuture {
        let mut inner = self.inner.borrow_mut();
        inner.next_id = inner.next_id.saturating_add(1);
        let raw = inner.next_id;
        if !inner.allocation_admitted || inner.outbound.len() >= REQUEST_OUTBOUND_SLOTS || inner.occupied(Inner::index(raw)) {
            return RequestFuture { registry: self.inner.clone(), id: 0, admission_failed: true };
        }
        let effect = build(RequestId(raw));
        inner.insert_admitted(SlotEntry { id: raw, instance: self.instance, value: Slot::Pending { waker: None, partial: Vec::new() } });
        inner.outbound.push_back((self.instance, effect));
        RequestFuture { registry: self.inner.clone(), id: raw, admission_failed: false }
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
    /// `⚛️reactor/🦀️.rs`'s `Event::HttpChunk` routing step used to have — it kept only the
    /// LAST chunk's `bytes`, silently dropping every earlier one). `cap` is the owning instance's
    /// `QuotaSchema.message_bytes` (default 64 MiB, resolved by the caller via `instance_of` +
    /// `INSTANCE_QUOTAS` — this registry has no quota table of its own): exceeding it resolves the
    /// request with a typed fault immediately rather than silently truncating, and it stays resolved
    /// (a fault is terminal) even if more chunks for the same `id` arrive afterward — the underlying
    /// HTTP task is not itself cancelled by this, it just has nowhere left to deliver into. A chunk
    /// for an id that is not `Pending` (already resolved, or cancelled/dropped — see `RequestFuture`'s
    /// `Drop` impl) is a harmless no-op, same as `resolve` on an unknown id.
    pub fn append_chunk(&self, id: RequestId, bytes: &[u8], done: bool, cap: usize) {
        let mut inner = self.inner.borrow_mut();
        let outcome = match inner.get_mut(id.0).map(|entry| &mut entry.value) {
            Some(Slot::Pending { partial, .. }) => {
                partial.extend_from_slice(bytes);
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
    /// (`⚛️reactor/🦀️.rs`'s `Event::HttpChunk` routing step) uses this to look up the
    /// owning instance's `QuotaSchema.message_bytes` cap. `None` once the slot is gone (already
    /// resolved or cancelled), same lifetime as every other per-id lookup here.
    pub fn instance_of(&self, id: RequestId) -> Option<u32> {
        self.inner.borrow().get(id.0).map(|entry| entry.instance)
    }

    /// 🔥️ Fire-and-forget: queues `effect` (a variant with no `req`/no completion — `Notify`,
    /// `ClipboardWrite`, `Navigate`, ...) without allocating a `RequestId` or parking anything.
    pub fn emit(&self, effect: Effect) {
        let mut inner = self.inner.borrow_mut();
        if inner.allocation_admitted && inner.outbound.len() < REQUEST_OUTBOUND_SLOTS {
            inner.outbound.push_back((self.instance, effect));
        }
    }

    /// 📤️ Drains and returns every effect queued since the last drain, in request order —
    /// `reactor::poll` calls this once per turn after the executor idles.
    pub fn drain(&self) -> Vec<Effect> {
        self.inner.borrow_mut().outbound.drain(..).map(|(_, effect)| effect).collect()
    }

    /// 📸️ `⚛️reactor/📸️checkpoint`'s `pending_requests`: the ids still `Pending` — carried in the
    /// checkpoint pack so a restored actor's host round-trips can be identified as stale/re-run
    /// (see design-abi.md §4: async tasks are never serialised, only marked re-run-on-restore).
    pub fn pending_ids(&self) -> Vec<RequestId> {
        let inner = self.inner.borrow();
        (0..REQUEST_SLOTS).filter_map(|index| inner.get_at(index)).filter(|entry| matches!(entry.value, Slot::Pending { .. })).map(|entry| RequestId(entry.id)).collect()
    }

    /// 🚫️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): `Event::InstanceClose`
    /// cancellation — removes every slot (`Pending` or an already-`Ready`-but-never-polled result)
    /// tagged as belonging to `instance`. No wake/notify: `⚛️reactor::cancel_instance_tasks` drops
    /// that instance's owning `AsyncTask` futures from the `LocalExecutor` in the SAME step (see
    /// that function's call site in `poll`'s `Event::InstanceClose` handling), so nothing is left
    /// to observe a "cancelled" resolution — this is cleanup, not notification. Returns the number
    /// of slots removed (diagnostic only). Idempotent: an instance with no pending requests removes
    /// nothing.
    pub fn begin_cancel_instance(&self, instance: u32) -> RequestCloseCursor {
        RequestCloseCursor { instance, index: 0, outbound_remaining: 0, outbound_initialized: false }
    }

    pub fn cancel_instance_step(&self, cursor: &mut RequestCloseCursor) -> RequestCloseStep {
        if cursor.index < REQUEST_SLOTS {
            let detached = {
                let Ok(mut inner) = self.inner.try_borrow_mut() else { return RequestCloseStep::Pending };
                let index = cursor.index;
                cursor.index += 1;
                if inner.get_at(index).is_some_and(|entry| entry.instance == cursor.instance) {
                    let id = inner.get_at(index).expect("checked request close slot").id;
                    inner.take(id)
                } else {
                    None
                }
            };
            drop(detached);
            return RequestCloseStep::Pending;
        }
        if !cursor.outbound_initialized {
            let Ok(inner) = self.inner.try_borrow() else { return RequestCloseStep::Pending };
            cursor.outbound_remaining = inner.outbound.len();
            cursor.outbound_initialized = true;
            return RequestCloseStep::Pending;
        }
        if cursor.outbound_remaining > 0 {
            let detached = {
                let Ok(mut inner) = self.inner.try_borrow_mut() else { return RequestCloseStep::Pending };
                cursor.outbound_remaining -= 1;
                match inner.outbound.pop_front() {
                    Some((owner, effect)) if owner == cursor.instance => Some(effect),
                    Some(entry) => {
                        inner.outbound.push_back(entry);
                        None
                    }
                    None => None,
                }
            };
            drop(detached);
            return RequestCloseStep::Pending;
        }
        RequestCloseStep::Complete
    }
}

/// ⏳️ Awaiting this future is how a `host::*` call parks — see `pure.wit`'s doc comment for why
/// only `log`/`now-ms`/`trace-span` stay synchronous and everything else goes through here.
pub struct RequestFuture {
    registry: Rc<RefCell<Inner>>,
    id: u64,
    admission_failed: bool,
}

impl Future for RequestFuture {
    type Output = Result<Vec<u8>, Fault>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.admission_failed {
            return Poll::Ready(Err(Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("plugin.request-registry.capacity"), "fixed request authority is saturated".to_string())));
        }
        let mut inner = self.registry.borrow_mut();
        match inner.take(self.id) {
            Some(SlotEntry { value: Slot::Ready { result }, .. }) => Poll::Ready(result),
            Some(SlotEntry { instance, value: Slot::Pending { partial, .. }, .. }) => {
                inner.insert_admitted(SlotEntry { id: self.id, instance, value: Slot::Pending { waker: Some(cx.waker().clone()), partial } });
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
        if !self.admission_failed {
            drop(self.registry.borrow_mut().take(self.id));
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
