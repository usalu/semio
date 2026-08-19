//! 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2, design-abi.md §4): a single-threaded task
//! executor for the guest's `poll` turn loop. wasm32-wasip2 plugin instances are always
//! single-threaded, so tasks and wakers live behind `Rc`/`RefCell` — no `Send`/`Sync` bound
//! anywhere in this module, which is what lets `host::*` futures capture non-`Send` state freely.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/// 🪪️ Stable slot index — reused across `poll` calls for as long as the task stays parked, so a
/// `RequestRegistry` completion or a fired timer can `wake()` it by id without walking a list.
pub type TaskId = u64;

type BoxedTask = Pin<Box<dyn Future<Output = ()>>>;

#[derive(Default)]
struct Inner {
    slots: Vec<Option<BoxedTask>>,
    free: Vec<usize>,
    ready: VecDeque<usize>,
}

/// 🧵️ `Clone` is cheap (an `Rc` bump) — every `AsyncTask`/`host::*` future closure captures its
/// own clone to call `.wake(id)` on completion, never a raw pointer.
#[derive(Clone, Default)]
pub struct LocalExecutor {
    inner: Rc<RefCell<Inner>>,
}

impl LocalExecutor {
    pub fn new() -> Self {
        Self::default()
    }

    /// 🌱️ Spawns a task and schedules it ready for the next `run_until_idle` — used both for a
    /// fresh `Emit::tasks` follow-up and for the top-level `app-command` dispatch future.
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static) -> TaskId {
        let mut inner = self.inner.borrow_mut();
        let id = if let Some(id) = inner.free.pop() {
            inner.slots[id] = Some(Box::pin(future));
            id
        } else {
            inner.slots.push(Some(Box::pin(future)));
            inner.slots.len() - 1
        };
        inner.ready.push_back(id);
        id as TaskId
    }

    /// 🌱️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): two-phase spawn — reserves a
    /// slot id FIRST and hands it to `make_future`, so an `AsyncTask`'s completion wrapper can
    /// embed its OWN id (post-completion bookkeeping keyed by `TaskId` — `⚛️reactor::spawn_task`'s
    /// `TASK_RECORDS`/`TASK_KEYS` cleanup) without a capture-after-construct `Cell` dance. `spawn`
    /// above stays as the simpler entry point for callers that never need their own id.
    pub fn spawn_with_id(&self, make_future: impl FnOnce(TaskId) -> Pin<Box<dyn Future<Output = ()>>>) -> TaskId {
        let id = {
            let mut inner = self.inner.borrow_mut();
            if let Some(id) = inner.free.pop() {
                inner.slots[id] = None; // placeholder — filled below, kept reserved meanwhile
                id
            } else {
                inner.slots.push(None);
                inner.slots.len() - 1
            }
        };
        let future = make_future(id as TaskId);
        let mut inner = self.inner.borrow_mut();
        inner.slots[id] = Some(future);
        inner.ready.push_back(id);
        id as TaskId
    }

    /// 🚫️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): cancels a spawned task
    /// outright — drops its future (and everything IT owns, including any parked `RequestFuture`
    /// from `⚛️reactor/📮️requests`) without ever polling it again, and frees its slot for reuse.
    /// Used for key-dedupe (spawning onto a live `(instance, key)` cancels the stale task first)
    /// and `Event::InstanceClose` (every task that instance owns). Idempotent: cancelling an
    /// already-finished or unknown id is a no-op.
    pub fn cancel(&self, id: TaskId) {
        let mut inner = self.inner.borrow_mut();
        let index = id as usize;
        if index < inner.slots.len() {
            inner.slots[index] = None;
            if !inner.free.contains(&index) {
                inner.free.push(index);
            }
        }
        inner.ready.retain(|&ready_index| ready_index != index);
    }

    /// 🔔️ Re-queues `id`. Idempotent within one turn (never double-queues an already-ready task).
    pub fn wake(&self, id: TaskId) {
        let mut inner = self.inner.borrow_mut();
        let id = id as usize;
        if id < inner.slots.len() && inner.slots[id].is_some() && !inner.ready.contains(&id) {
            inner.ready.push_back(id);
        }
    }

    /// ▶️ Drains the ready queue, polling each task at most once per pass, until either the queue
    /// is empty or `max_iterations` polls have run (a defensive bound against a task that keeps
    /// re-waking itself forever inside one turn — `reactor::poll` treats hitting the cap as
    /// `turn-status::more-work`, not `idle`). Returns whether any task is still alive (ready or
    /// parked) when it returns.
    pub fn run_until_idle(&self, max_iterations: u32) -> bool {
        for _ in 0..max_iterations {
            let Some(id) = self.inner.borrow_mut().ready.pop_front() else {
                break;
            };
            let future = self.inner.borrow_mut().slots[id].take();
            let Some(mut future) = future else { continue };
            let waker = self.waker_for(id as TaskId);
            let mut cx = Context::from_waker(&waker);
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(()) => {
                    let mut inner = self.inner.borrow_mut();
                    inner.slots[id] = None;
                    inner.free.push(id);
                }
                Poll::Pending => {
                    self.inner.borrow_mut().slots[id] = Some(future);
                }
            }
        }
        self.has_pending()
    }

    pub fn has_ready(&self) -> bool {
        !self.inner.borrow().ready.is_empty()
    }

    pub fn has_pending(&self) -> bool {
        self.inner.borrow().slots.iter().any(Option::is_some)
    }

    fn waker_for(&self, id: TaskId) -> Waker {
        let data = Rc::new(WakerData { inner: self.inner.clone(), id: id as usize });
        unsafe { Waker::from_raw(raw_waker(data)) }
    }
}

struct WakerData {
    inner: Rc<RefCell<Inner>>,
    id: usize,
}

fn raw_waker(data: Rc<WakerData>) -> RawWaker {
    RawWaker::new(Rc::into_raw(data) as *const (), &VTABLE)
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(waker_clone, waker_wake, waker_wake_by_ref, waker_drop);

unsafe fn waker_clone(ptr: *const ()) -> RawWaker {
    let data = unsafe { Rc::from_raw(ptr as *const WakerData) };
    let cloned = data.clone();
    std::mem::forget(data);
    raw_waker(cloned)
}

unsafe fn waker_wake(ptr: *const ()) {
    let data = unsafe { Rc::from_raw(ptr as *const WakerData) };
    let mut inner = data.inner.borrow_mut();
    if !inner.ready.contains(&data.id) {
        inner.ready.push_back(data.id);
    }
}

unsafe fn waker_wake_by_ref(ptr: *const ()) {
    let data = unsafe { &*(ptr as *const WakerData) };
    let mut inner = data.inner.borrow_mut();
    if !inner.ready.contains(&data.id) {
        inner.ready.push_back(data.id);
    }
}

unsafe fn waker_drop(ptr: *const ()) {
    drop(unsafe { Rc::from_raw(ptr as *const WakerData) });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::task::Poll as StdPoll;

    struct YieldOnce {
        yielded: bool,
    }

    impl Future for YieldOnce {
        type Output = ();
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> StdPoll<()> {
            if self.yielded {
                StdPoll::Ready(())
            } else {
                self.yielded = true;
                cx.waker().wake_by_ref();
                StdPoll::Pending
            }
        }
    }

    #[test]
    fn spawn_runs_a_ready_task_to_completion() {
        let executor = LocalExecutor::new();
        let ran = Rc::new(Cell::new(false));
        let ran_inner = ran.clone();
        executor.spawn(async move {
            ran_inner.set(true);
        });
        let pending = executor.run_until_idle(8);
        assert!(ran.get(), "task body must have run");
        assert!(!pending, "no task should remain pending");
    }

    #[test]
    fn a_self_waking_task_is_polled_again_within_the_same_pass() {
        let executor = LocalExecutor::new();
        executor.spawn(YieldOnce { yielded: false });
        let pending = executor.run_until_idle(8);
        assert!(!pending, "YieldOnce must complete within the iteration budget");
    }

    #[test]
    fn a_task_that_never_wakes_stays_pending_until_woken() {
        let executor = LocalExecutor::new();
        let waker_cell: Rc<RefCell<Option<Waker>>> = Rc::new(RefCell::new(None));
        let waker_cell_inner = waker_cell.clone();
        struct ParkForever {
            cell: Rc<RefCell<Option<Waker>>>,
            done: Rc<Cell<bool>>,
        }
        impl Future for ParkForever {
            type Output = ();
            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> StdPoll<()> {
                if self.done.get() {
                    StdPoll::Ready(())
                } else {
                    *self.cell.borrow_mut() = Some(cx.waker().clone());
                    StdPoll::Pending
                }
            }
        }
        let done = Rc::new(Cell::new(false));
        let done_inner = done.clone();
        executor.spawn(ParkForever { cell: waker_cell_inner, done: done_inner });
        let pending = executor.run_until_idle(8);
        assert!(pending, "task must stay parked until its waker fires");
        assert!(!executor.has_ready(), "a parked task must not remain in the ready queue");

        done.set(true);
        waker_cell.borrow().as_ref().expect("poll must have captured a waker").wake_by_ref();
        let pending = executor.run_until_idle(8);
        assert!(!pending, "waking must let the task observe `done` and complete");
    }

    #[test]
    fn cancel_before_the_first_run_until_idle_drops_the_future_without_ever_polling_it() {
        struct DropFlag(Rc<Cell<bool>>);
        impl Drop for DropFlag {
            fn drop(&mut self) {
                self.0.set(true);
            }
        }
        let executor = LocalExecutor::new();
        let polled = Rc::new(Cell::new(false));
        let dropped = Rc::new(Cell::new(false));
        let polled_inner = polled.clone();
        let flag = DropFlag(dropped.clone());
        let id = executor.spawn(async move {
            let _flag = flag;
            polled_inner.set(true);
        });
        executor.cancel(id);
        let pending = executor.run_until_idle(8);
        assert!(!polled.get(), "a cancelled task's body must never run");
        assert!(dropped.get(), "cancelling must drop the future (and everything it owns)");
        assert!(!pending, "nothing should remain pending after cancelling the only task");
    }

    #[test]
    fn cancel_of_a_parked_task_drops_it_and_frees_its_slot_for_reuse() {
        let executor = LocalExecutor::new();
        let waker_cell: Rc<RefCell<Option<Waker>>> = Rc::new(RefCell::new(None));
        let waker_cell_inner = waker_cell.clone();
        struct ParkForever {
            cell: Rc<RefCell<Option<Waker>>>,
        }
        impl Future for ParkForever {
            type Output = ();
            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> StdPoll<()> {
                *self.cell.borrow_mut() = Some(cx.waker().clone());
                StdPoll::Pending
            }
        }
        let id = executor.spawn(ParkForever { cell: waker_cell_inner });
        let pending = executor.run_until_idle(8);
        assert!(pending, "task must be parked");
        executor.cancel(id);
        assert!(!executor.has_pending(), "cancelling the only parked task must clear has_pending");
        // 🔁️ The freed slot is reused by the next spawn — cancel must not leak the index forever.
        let reused = executor.spawn(async move {});
        assert_eq!(reused, id, "a cancelled slot must be reusable by a later spawn");
    }

    #[test]
    fn spawn_with_id_hands_the_reserved_id_to_the_future_builder_before_it_ever_runs() {
        let executor = LocalExecutor::new();
        let seen_id: Rc<Cell<Option<TaskId>>> = Rc::new(Cell::new(None));
        let seen_id_inner = seen_id.clone();
        let id = executor.spawn_with_id(move |id| {
            Box::pin(async move {
                seen_id_inner.set(Some(id));
            })
        });
        let pending = executor.run_until_idle(8);
        assert!(!pending);
        assert_eq!(seen_id.get(), Some(id), "the future must observe the SAME id spawn_with_id returned");
    }

    #[test]
    fn cancel_is_idempotent_for_an_unknown_or_already_finished_id() {
        let executor = LocalExecutor::new();
        executor.cancel(999); // never spawned
        let id = executor.spawn(async move {});
        let _ = executor.run_until_idle(8); // finishes and frees the slot
        executor.cancel(id); // already finished
        executor.cancel(id); // cancel twice in a row
    }
}
