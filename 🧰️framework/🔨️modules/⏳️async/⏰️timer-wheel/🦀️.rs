//! ⏰️ `TimerWheel`: the deadline heap the worker pool fires from, and its `TimerSleep` future.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Mutex, OnceLock, PoisonError};
use std::task::{Context, Poll, Waker};

//#region ⏰️TimerWheel
/// ⏰️ One registered [`TimerWheel::sleep_until`] wait: its logical deadline, the [`Waker`] to fire
/// once due, and whether it has already fired (checked by [`TimerSleep::poll`] on a spurious
/// re-poll before the waker's task actually runs again).
struct TimerRegistration {
    #[allow(dead_code)]
    pub(super) deadline_ms: u64,
    pub(super) waker: Option<Waker>,
    fired: bool,
}

type TimerCallback = Box<dyn FnOnce() + Send + 'static>;
pub(super) const TIMER_ACTIONS_PER_POOL_TURN: usize = 32;

struct TimerWheelState {
    heap: BinaryHeap<Reverse<(u64, u64)>>,
    pub(super) entries: HashMap<u64, TimerRegistration>,
    callbacks: HashMap<u64, TimerCallback>,
    pub(super) next_id: u64,
    last_now_ms: u64,
}

/// ⏰️ Min-heap-of-deadlines timer primitive shared by [`WorkerPool`] (native: an idle-parked worker
/// calls [`TimerWheel::fire_due`] with its own elapsed-wall-clock reading each time it wakes; wasm:
/// [`WorkerPool::pump`] calls it with the host-supplied `now_ms`) — the epoch-ticker OS thread Phase
/// 0's census found (`🧵️shard/…host.rs` `"semio-epoch-ticker"`, 1 ms poll loop) disappears; deadline
/// tracking becomes this reactive structure plus [`Lane::Timer`] instead of a dedicated thread.
/// Deliberately does NOT read a clock itself (this crate's "caller supplies `now_ms`" discipline,
/// same as [`HostAsyncRuntime::now_ms`]) — [`TimerWheel::fire_due`] is always told the current time
/// by whichever driver called it.
pub struct TimerWheel {
    pub(super) state: Mutex<TimerWheelState>,
    earlier_deadline: OnceLock<Box<dyn Fn() + Send + Sync>>,
}

impl TimerWheel {
    pub fn new() -> TimerWheel {
        TimerWheel { state: Mutex::new(TimerWheelState { heap: BinaryHeap::new(), entries: HashMap::new(), callbacks: HashMap::new(), next_id: 1, last_now_ms: 0 }), earlier_deadline: OnceLock::new() }
    }

    /// 🔔️ Installs the one callback told whenever a registration becomes the wheel's new earliest
    /// deadline — how a parked driver learns it must wake sooner than it planned. Called outside the
    /// wheel's lock. A second install is refused, so the driver that owns the wheel stays the only one.
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn install_earlier_deadline_hook(&self, hook: Box<dyn Fn() + Send + Sync>) {
        let _ = self.earlier_deadline.set(hook);
    }

    fn push_deadline(state: &mut TimerWheelState, deadline_ms: u64, id: u64) -> bool {
        let earliest = state.heap.peek().is_none_or(|Reverse((head, _))| deadline_ms < *head);
        state.heap.push(Reverse((deadline_ms, id)));
        earliest
    }

    fn announce_earlier_deadline(&self, earliest: bool) {
        if earliest {
            if let Some(hook) = self.earlier_deadline.get() {
                hook();
            }
        }
    }

    /// ⏰️ A future that resolves once this wheel has been [`TimerWheel::fire_due`]'d with a
    /// `now_ms >= deadline_ms` (or was already, before this fn was even called).
    pub fn sleep_until(&self, deadline_ms: u64) -> TimerSleep<'_> {
        TimerSleep { wheel: self, deadline_ms, id: None }
    }

    #[doc(hidden)] pub fn register(&self, deadline_ms: u64, waker: Waker) -> u64 {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        let id = state.next_id;
        state.next_id += 1;
        state.entries.insert(id, TimerRegistration { deadline_ms, waker: Some(waker), fired: false });
        let earliest = Self::push_deadline(&mut state, deadline_ms, id);
        drop(state);
        self.announce_earlier_deadline(earliest);
        id
    }

    fn update_waker(&self, id: u64, waker: Waker) {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(entry) = state.entries.get_mut(&id) {
            entry.waker = Some(waker);
        }
    }

    #[doc(hidden)] pub fn schedule_callback(&self, deadline_ms: u64, callback: TimerCallback) {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        let id = state.next_id;
        state.next_id += 1;
        state.callbacks.insert(id, callback);
        let earliest = Self::push_deadline(&mut state, deadline_ms, id);
        drop(state);
        self.announce_earlier_deadline(earliest);
    }

    fn is_fired(&self, id: u64) -> bool {
        let state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        state.entries.get(&id).is_none_or(|entry| entry.fired)
    }

    fn forget(&self, id: u64) {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        state.entries.remove(&id);
    }

    fn last_now_ms(&self) -> u64 {
        self.state.lock().unwrap_or_else(PoisonError::into_inner).last_now_ms
    }

    /// ⏰️ The earliest still-pending deadline, if any — used by [`WorkerPool`]'s idle-park loop to
    /// bound how long a worker sleeps before it must re-check timers.
    pub fn next_deadline_ms(&self) -> Option<u64> {
        let state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        state.heap.peek().map(|Reverse((deadline, _))| *deadline)
    }

    /// ⏰️ Wakes every registration whose deadline is `<= now_ms`, returning how many fired. Stale
    /// heap entries for already-[`TimerWheel::forget`]-ten ids are skipped lazily (never
    /// compacted eagerly — the heap only ever grows by live-registration count, bounded by however
    /// many `sleep_until` calls are outstanding at once).
    pub fn fire_due(&self, now_ms: u64) -> u32 {
        self.fire_due_batch(now_ms, usize::MAX)
    }

    /// ⏱️ Fires at most `max_items` heaped registrations, including stale entries, so a pool
    /// turn cannot hide an unbounded timer scan.
    pub fn fire_due_batch(&self, now_ms: u64, max_items: usize) -> u32 {
        let mut woken = Vec::new();
        let mut callbacks = Vec::new();
        {
            let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
            state.last_now_ms = state.last_now_ms.max(now_ms);
            let mut processed_items = 0usize;
            while processed_items < max_items {
                let Some(Reverse((deadline, id))) = state.heap.peek().copied() else { break };
                if deadline > now_ms {
                    break;
                }
                state.heap.pop();
                processed_items += 1;
                if let Some(entry) = state.entries.get_mut(&id) {
                    if !entry.fired {
                        entry.fired = true;
                        if let Some(waker) = entry.waker.take() {
                            woken.push(waker);
                        }
                    }
                } else if let Some(callback) = state.callbacks.remove(&id) {
                    callbacks.push(callback);
                }
            }
        }
        let count = (woken.len() + callbacks.len()) as u32;
        for waker in woken {
            waker.wake();
        }
        for callback in callbacks {
            callback();
        }
        count
    }
}

impl Default for TimerWheel {
    fn default() -> TimerWheel {
        TimerWheel::new()
    }
}

/// ⏰️ Future returned by [`TimerWheel::sleep_until`]. Registers itself into the wheel on first
/// poll (or resolves immediately if the wheel's last known `now_ms` already reached the deadline),
/// then relies entirely on [`TimerWheel::fire_due`] waking it — no busy-polling.
pub struct TimerSleep<'a> {
    pub(super) wheel: &'a TimerWheel,
    pub(super) deadline_ms: u64,
    pub(super) id: Option<u64>,
}

impl Future for TimerSleep<'_> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.get_mut();
        if this.wheel.last_now_ms() >= this.deadline_ms {
            return Poll::Ready(());
        }
        match this.id {
            Some(id) if this.wheel.is_fired(id) => Poll::Ready(()),
            Some(id) => {
                this.wheel.update_waker(id, cx.waker().clone());
                Poll::Pending
            }
            None => {
                this.id = Some(this.wheel.register(this.deadline_ms, cx.waker().clone()));
                Poll::Pending
            }
        }
    }
}

impl Drop for TimerSleep<'_> {
    fn drop(&mut self) {
        if let Some(id) = self.id {
            self.wheel.forget(id);
        }
    }
}
//#endregion ⏰️TimerWheel
