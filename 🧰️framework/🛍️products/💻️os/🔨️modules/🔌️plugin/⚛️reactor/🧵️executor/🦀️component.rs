//! 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2, design-abi.md §4): a single-threaded task
//! executors for the guest runtime. `ReactorExecutor` owns only explicit `ReactorTask` state
//! machines whose work and disposal are both budgeted. `ColdFutureExecutor` is the deliberately
//! separate generic-future executor used by noninteractive cold jobs and legacy test fixtures; it
//! is never the production interactive reactor authority.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/// 🪪️ Stable slot index — reused across `poll` calls for as long as the task stays parked, so a
/// `RequestRegistry` completion or a fired timer can `wake()` it by id without walking a list.
pub type TaskId = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskPoll {
    Complete,
    Pending,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReactorTaskBudget {
    pub operation: u64,
    pub generation: u64,
    pub cancellation_generation: u64,
    pub maximum_units: usize,
    pub maximum_bytes: usize,
    pub deadline: std::time::Instant,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReactorTaskStep {
    Pending { processed_units: usize, processed_bytes: usize },
    Blocked { reason: &'static str },
    Complete,
}

pub trait ReactorTask {
    fn step(&mut self, budget: ReactorTaskBudget) -> ReactorTaskStep;
    fn close_step(&mut self, budget: ReactorTaskBudget) -> ReactorTaskStep;
    fn terminal_is_empty(&self) -> bool;
}

struct ReactorTaskSlot {
    generation: u32,
    instance: u32,
    operation: u64,
    authority_generation: u64,
    cancellation_generation: u64,
    task: std::mem::ManuallyDrop<Option<Box<dyn ReactorTask>>>,
    closing: bool,
}

impl Drop for ReactorTaskSlot {
    fn drop(&mut self) {
        debug_assert!(self.task.is_none(), "reactor task slot reached Drop before its bounded terminal disposal");
    }
}

struct ReactorInner {
    slots: VecDeque<ReactorTaskSlot>,
    free: VecDeque<usize>,
    scan_cursor: usize,
    shutdown_cursor: usize,
    shutdown: bool,
    live: usize,
    allocation_admitted: bool,
}

impl ReactorInner {
    fn new() -> Self {
        Self { slots: VecDeque::new(), free: VecDeque::new(), scan_cursor: 0, shutdown_cursor: 0, shutdown: false, live: 0, allocation_admitted: false }
    }
}

#[derive(Clone)]
pub struct ReactorExecutor {
    inner: Rc<RefCell<ReactorInner>>,
}

pub struct RejectedReactorTask {
    task: std::mem::ManuallyDrop<Option<Box<dyn ReactorTask>>>,
}

impl RejectedReactorTask {
    pub fn close_step(&mut self, budget: ReactorTaskBudget) -> ReactorTaskStep {
        let Some(task) = self.task.as_mut() else { return ReactorTaskStep::Complete };
        let step = task.close_step(budget);
        if step == ReactorTaskStep::Complete {
            if !task.terminal_is_empty() {
                return ReactorTaskStep::Blocked { reason: "rejected reactor task reported Complete before its terminal shell was empty" };
            }
            drop(self.task.take());
        }
        step
    }
}

impl Drop for RejectedReactorTask {
    fn drop(&mut self) {
        debug_assert!(self.task.is_none(), "rejected reactor task reached Drop without bounded disposal");
    }
}

impl Default for ReactorExecutor {
    fn default() -> Self {
        Self { inner: Rc::new(RefCell::new(ReactorInner::new())) }
    }
}

impl ReactorExecutor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pre_admit(&self) -> bool {
        let mut inner = self.inner.borrow_mut();
        if inner.allocation_admitted {
            return true;
        }
        if inner.live != 0 || !inner.slots.is_empty() || !inner.free.is_empty() || inner.shutdown {
            return false;
        }
        let mut slots = VecDeque::new();
        if slots.try_reserve_exact(LOCAL_EXECUTOR_TASK_SLOTS).is_err() {
            return false;
        }
        slots.extend((0..LOCAL_EXECUTOR_TASK_SLOTS).map(|_| ReactorTaskSlot { generation: 0, instance: 0, operation: 0, authority_generation: 0, cancellation_generation: 0, task: std::mem::ManuallyDrop::new(None), closing: false }));
        let mut free = VecDeque::new();
        if free.try_reserve_exact(LOCAL_EXECUTOR_TASK_SLOTS).is_err() {
            return false;
        }
        free.extend(0..LOCAL_EXECUTOR_TASK_SLOTS);
        inner.slots = slots;
        inner.free = free;
        inner.allocation_admitted = true;
        true
    }

    pub fn admit(&self, instance: u32, operation: u64, authority_generation: u64, cancellation_generation: u64, task: Box<dyn ReactorTask>) -> Result<TaskId, RejectedReactorTask> {
        let mut inner = self.inner.borrow_mut();
        if !inner.allocation_admitted || inner.shutdown {
            return Err(RejectedReactorTask { task: std::mem::ManuallyDrop::new(Some(task)) });
        }
        let Some(index) = inner.free.pop_front() else { return Err(RejectedReactorTask { task: std::mem::ManuallyDrop::new(Some(task)) }) };
        let generation = inner.slots[index].generation.wrapping_add(1).max(1);
        debug_assert!(inner.slots[index].task.is_none(), "free reactor slot retained a task owner");
        inner.slots[index] = ReactorTaskSlot { generation, instance, operation, authority_generation, cancellation_generation, task: std::mem::ManuallyDrop::new(Some(task)), closing: false };
        inner.live += 1;
        Ok(((generation as u64) << 32) | index as u64)
    }

    pub fn cancel(&self, id: TaskId, cancellation_generation: u64) -> bool {
        let index = id as u32 as usize;
        let generation = (id >> 32) as u32;
        let mut inner = self.inner.borrow_mut();
        let Some(slot) = inner.slots.get_mut(index) else { return false };
        if slot.generation != generation || slot.task.is_none() {
            return false;
        }
        slot.cancellation_generation = cancellation_generation;
        slot.closing = true;
        true
    }

    pub fn run_until_deadline(&self, maximum_units: usize, maximum_bytes: usize, deadline: std::time::Instant) -> bool {
        let mut remaining_units = maximum_units;
        let mut remaining_bytes = maximum_bytes;
        while remaining_units != 0 && std::time::Instant::now() < deadline {
            let (index, generation, authority, closing, mut task) = {
                let mut inner = self.inner.borrow_mut();
                if inner.live == 0 || inner.slots.is_empty() {
                    break;
                }
                let index = inner.scan_cursor % inner.slots.len();
                inner.scan_cursor = (index + 1) % inner.slots.len();
                let generation = inner.slots[index].generation;
                let Some(task) = inner.slots[index].task.take() else {
                    remaining_units -= 1;
                    continue;
                };
                let slot = &inner.slots[index];
                let authority = (slot.operation, slot.authority_generation, slot.cancellation_generation);
                (index, generation, authority, slot.closing, task)
            };
            let budget = ReactorTaskBudget { operation: authority.0, generation: authority.1, cancellation_generation: authority.2, maximum_units: remaining_units, maximum_bytes: remaining_bytes, deadline };
            let step = if closing { task.close_step(budget) } else { task.step(budget) };
            let mut inner = self.inner.borrow_mut();
            if inner.slots.get(index).is_none_or(|slot| slot.generation != generation || slot.task.is_some()) {
                return true;
            }
            match step {
                ReactorTaskStep::Complete if inner.slots[index].closing && task.terminal_is_empty() => {
                    drop(task);
                    inner.slots[index].closing = false;
                    inner.live = inner.live.saturating_sub(1);
                    inner.free.push_back(index);
                    remaining_units -= 1;
                }
                ReactorTaskStep::Complete => {
                    inner.slots[index].closing = true;
                    inner.slots[index].task = Some(task);
                    remaining_units -= 1;
                }
                ReactorTaskStep::Pending { processed_units, processed_bytes } if processed_units != 0 && processed_units <= remaining_units && processed_bytes <= remaining_bytes => {
                    remaining_units -= processed_units;
                    remaining_bytes -= processed_bytes;
                    inner.slots[index].task = Some(task);
                }
                ReactorTaskStep::Pending { .. } | ReactorTaskStep::Blocked { .. } => {
                    inner.slots[index].task = Some(task);
                    remaining_units -= 1;
                }
            }
        }
        self.inner.borrow().live != 0
    }

    pub fn close_instance_step(&self, instance: u32, cursor: &mut usize, budget: ReactorTaskBudget) -> ReactorTaskStep {
        if budget.maximum_units == 0 || std::time::Instant::now() >= budget.deadline {
            return ReactorTaskStep::Pending { processed_units: 0, processed_bytes: 0 };
        }
        if *cursor >= LOCAL_EXECUTOR_TASK_SLOTS {
            return ReactorTaskStep::Complete;
        }
        let index = *cursor;
        let (generation, authority, mut task) = {
            let mut inner = self.inner.borrow_mut();
            let Some(slot) = inner.slots.get_mut(index) else { return ReactorTaskStep::Complete };
            if slot.instance != instance || slot.task.is_none() {
                *cursor += 1;
                return if *cursor >= LOCAL_EXECUTOR_TASK_SLOTS { ReactorTaskStep::Complete } else { ReactorTaskStep::Pending { processed_units: 1, processed_bytes: 0 } };
            }
            slot.closing = true;
            (slot.generation, (slot.operation, slot.authority_generation, slot.cancellation_generation), slot.task.take().expect("checked exact reactor task owner"))
        };
        let budget = ReactorTaskBudget { operation: authority.0, generation: authority.1, cancellation_generation: authority.2, ..budget };
        let step = task.close_step(budget);
        let mut inner = self.inner.borrow_mut();
        let Some(slot) = inner.slots.get_mut(index) else { return ReactorTaskStep::Blocked { reason: "reactor task slot disappeared during close" } };
        if slot.generation != generation || slot.task.is_some() {
            return ReactorTaskStep::Blocked { reason: "reactor task generation changed during close" };
        }
        match step {
            ReactorTaskStep::Complete if task.terminal_is_empty() => {
                drop(task);
                slot.closing = false;
                inner.live = inner.live.saturating_sub(1);
                inner.free.push_back(index);
                *cursor += 1;
                if *cursor >= LOCAL_EXECUTOR_TASK_SLOTS { ReactorTaskStep::Complete } else { ReactorTaskStep::Pending { processed_units: 1, processed_bytes: 0 } }
            }
            ReactorTaskStep::Complete => {
                slot.task = Some(task);
                ReactorTaskStep::Blocked { reason: "reactor task reported Complete before its terminal shell was empty" }
            }
            ReactorTaskStep::Pending { processed_units, processed_bytes } if processed_units == 0 || processed_units > budget.maximum_units || processed_bytes > budget.maximum_bytes => {
                slot.task = Some(task);
                ReactorTaskStep::Blocked { reason: "reactor task close step violated its admitted unit or byte budget" }
            }
            other => {
                slot.task = Some(task);
                other
            }
        }
    }

    pub fn has_pending(&self) -> bool {
        self.inner.borrow().live != 0
    }

    pub fn begin_shutdown(&self) {
        let mut inner = self.inner.borrow_mut();
        inner.shutdown = true;
        inner.shutdown_cursor = 0;
    }

    pub fn shutdown_step(&self, budget: ReactorTaskBudget) -> ReactorTaskStep {
        if budget.maximum_units == 0 || std::time::Instant::now() >= budget.deadline {
            return ReactorTaskStep::Pending { processed_units: 0, processed_bytes: 0 };
        }
        let detached = {
            let mut inner = self.inner.borrow_mut();
            if !inner.shutdown {
                return ReactorTaskStep::Blocked { reason: "reactor executor shutdown was not atomically begun" };
            }
            if inner.shutdown_cursor < inner.slots.len() {
                let index = inner.shutdown_cursor;
                let slot = &mut inner.slots[index];
                slot.closing = true;
                slot.task.take().map(|task| (index, slot.generation, task))
            } else if inner.live != 0 {
                return ReactorTaskStep::Blocked { reason: "reactor executor shutdown cursor lost a live task owner" };
            } else if inner.free.pop_front().is_some() {
                return ReactorTaskStep::Pending { processed_units: 1, processed_bytes: 0 };
            } else if inner.slots.pop_back().is_some() {
                return ReactorTaskStep::Pending { processed_units: 1, processed_bytes: 0 };
            } else {
                return ReactorTaskStep::Complete;
            }
        };
        let Some((index, generation, mut task)) = detached else {
            self.inner.borrow_mut().shutdown_cursor += 1;
            return ReactorTaskStep::Pending { processed_units: 1, processed_bytes: 0 };
        };
        let step = task.close_step(budget);
        let mut inner = self.inner.borrow_mut();
        let Some(slot) = inner.slots.get_mut(index) else { return ReactorTaskStep::Blocked { reason: "reactor executor shutdown lost its exact slot" } };
        if slot.generation != generation || slot.task.is_some() {
            return ReactorTaskStep::Blocked { reason: "reactor executor shutdown observed a stale generation" };
        }
        match step {
            ReactorTaskStep::Complete if task.terminal_is_empty() => {
                drop(task);
                slot.closing = false;
                inner.live = inner.live.saturating_sub(1);
                inner.shutdown_cursor += 1;
                ReactorTaskStep::Pending { processed_units: 1, processed_bytes: 0 }
            }
            ReactorTaskStep::Complete => {
                slot.task = Some(task);
                ReactorTaskStep::Blocked { reason: "reactor task reported Complete before its shutdown terminal shell was empty" }
            }
            ReactorTaskStep::Pending { processed_units, processed_bytes } if processed_units == 0 || processed_units > budget.maximum_units || processed_bytes > budget.maximum_bytes => {
                slot.task = Some(task);
                ReactorTaskStep::Blocked { reason: "reactor task shutdown step violated its admitted unit or byte budget" }
            }
            other => {
                slot.task = Some(task);
                other
            }
        }
    }
}

type BoxedTask = Pin<Box<dyn Future<Output = ()>>>;

const LOCAL_EXECUTOR_TASK_SLOTS: usize = 1_024;

struct TaskSlot {
    generation: u32,
    future: Option<BoxedTask>,
    queued: bool,
    active: bool,
    reserved: bool,
    ready_previous: Option<usize>,
    ready_next: Option<usize>,
}

struct Inner {
    slots: Box<[TaskSlot]>,
    free: VecDeque<usize>,
    ready_head: Option<usize>,
    ready_tail: Option<usize>,
    ready_len: usize,
    live: usize,
    allocation_admitted: bool,
}

impl Inner {
    fn new() -> Self {
        let mut slots = Vec::new();
        let slots_admitted = slots.try_reserve_exact(LOCAL_EXECUTOR_TASK_SLOTS).is_ok();
        if slots_admitted {
            slots.resize_with(LOCAL_EXECUTOR_TASK_SLOTS, || TaskSlot { generation: 0, future: None, queued: false, active: false, reserved: false, ready_previous: None, ready_next: None });
        }
        let mut free = VecDeque::new();
        let free_admitted = free.try_reserve_exact(LOCAL_EXECUTOR_TASK_SLOTS).is_ok();
        if slots_admitted && free_admitted {
            free.extend(0..LOCAL_EXECUTOR_TASK_SLOTS);
        }
        Self { slots: slots.into_boxed_slice(), free, ready_head: None, ready_tail: None, ready_len: 0, live: 0, allocation_admitted: slots_admitted && free_admitted }
    }

    fn task_id(index: usize, generation: u32) -> TaskId {
        ((generation as u64) << 32) | index as u64
    }

    fn task_parts(id: TaskId) -> Option<(usize, u32)> {
        let index = id as u32 as usize;
        (index < LOCAL_EXECUTOR_TASK_SLOTS).then_some((index, (id >> 32) as u32))
    }

    fn matches(&self, id: TaskId) -> Option<usize> {
        let (index, generation) = Self::task_parts(id)?;
        self.slots.get(index).filter(|slot| slot.generation == generation && slot.active).map(|_| index)
    }

    fn enqueue_ready(&mut self, index: usize) -> bool {
        if index >= self.slots.len() || self.slots[index].queued || !self.slots[index].active || self.ready_len >= LOCAL_EXECUTOR_TASK_SLOTS {
            return false;
        }
        let previous = self.ready_tail;
        self.slots[index].queued = true;
        self.slots[index].ready_previous = previous;
        self.slots[index].ready_next = None;
        if let Some(previous) = previous {
            self.slots[previous].ready_next = Some(index);
        } else {
            self.ready_head = Some(index);
        }
        self.ready_tail = Some(index);
        self.ready_len += 1;
        true
    }

    fn remove_ready(&mut self, index: usize) -> bool {
        if index >= self.slots.len() || !self.slots[index].queued {
            return false;
        }
        let previous = self.slots[index].ready_previous;
        let next = self.slots[index].ready_next;
        if let Some(previous) = previous {
            self.slots[previous].ready_next = next;
        } else {
            self.ready_head = next;
        }
        if let Some(next) = next {
            self.slots[next].ready_previous = previous;
        } else {
            self.ready_tail = previous;
        }
        self.slots[index].queued = false;
        self.slots[index].ready_previous = None;
        self.slots[index].ready_next = None;
        self.ready_len = self.ready_len.saturating_sub(1);
        true
    }

    fn pop_ready(&mut self) -> Option<TaskId> {
        let index = self.ready_head?;
        let generation = self.slots[index].generation;
        self.remove_ready(index);
        Some(Self::task_id(index, generation))
    }
}

/// 🧵️ `Clone` is cheap (an `Rc` bump) — every `AsyncTask`/`host::*` future closure captures its
/// own clone to call `.wake(id)` on completion, never a raw pointer.
#[derive(Clone)]
pub struct ColdFutureExecutor {
    inner: Rc<RefCell<Inner>>,
}

#[cfg(test)]
pub struct TaskReservation {
    inner: Rc<RefCell<Inner>>,
    id: TaskId,
    installed: bool,
}

#[cfg(test)]
impl TaskReservation {
    pub fn id(&self) -> TaskId {
        self.id
    }

    pub fn install(mut self, future: Pin<Box<dyn Future<Output = ()>>>) -> TaskId {
        let (index, generation) = Inner::task_parts(self.id).expect("reservation id is direct-indexed");
        let mut inner = self.inner.borrow_mut();
        {
            let slot = &mut inner.slots[index];
            assert!(slot.generation == generation && slot.reserved && !slot.active, "executor reservation authority changed before install");
            slot.future = Some(future);
            slot.active = true;
            slot.reserved = false;
        }
        assert!(inner.enqueue_ready(index), "executor fixed ready authority rejected a reserved task");
        inner.live += 1;
        self.installed = true;
        self.id
    }
}

#[cfg(test)]
impl Drop for TaskReservation {
    fn drop(&mut self) {
        if self.installed {
            return;
        }
        let Some((index, generation)) = Inner::task_parts(self.id) else { return };
        let mut inner = self.inner.borrow_mut();
        if inner.slots[index].generation == generation && inner.slots[index].reserved && !inner.slots[index].active {
            inner.slots[index].reserved = false;
            inner.free.push_back(index);
        }
    }
}

impl Default for ColdFutureExecutor {
    fn default() -> Self {
        Self { inner: Rc::new(RefCell::new(Inner::new())) }
    }
}

impl ColdFutureExecutor {
    pub fn new() -> Self {
        Self::default()
    }

    /// 🌱️ Spawns a task and schedules it ready for the next `run_until_idle` — used both for a
    /// fresh `Emit::tasks` follow-up and for the top-level `app-command` dispatch future.
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static) -> Result<TaskId, &'static str> {
        self.spawn_boxed(Box::pin(future))
    }

    fn spawn_boxed(&self, future: BoxedTask) -> Result<TaskId, &'static str> {
        let mut inner = self.inner.borrow_mut();
        if !inner.allocation_admitted {
            return Err("executor fixed storage was not admitted");
        }
        let index = inner.free.pop_front().ok_or("executor fixed task capacity is saturated")?;
        let generation = inner.slots[index].generation.wrapping_add(1).max(1);
        let id = Inner::task_id(index, generation);
        inner.slots[index] = TaskSlot { generation, future: Some(future), queued: false, active: true, reserved: false, ready_previous: None, ready_next: None };
        assert!(inner.enqueue_ready(index), "executor fixed ready authority rejected a new task");
        inner.live += 1;
        Ok(id)
    }

    /// 🌱️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): two-phase spawn — reserves a
    /// slot id FIRST and hands it to `make_future`, so an `AsyncTask`'s completion wrapper can
    /// embed its OWN id (post-completion bookkeeping keyed by `TaskId` — `⚛️reactor::spawn_task`'s
    /// `TASK_RECORDS`/`TASK_KEYS` cleanup) without a capture-after-construct `Cell` dance. `spawn`
    /// above stays as the simpler entry point for callers that never need their own id.
    #[cfg(test)]
    pub fn spawn_with_id(&self, make_future: impl FnOnce(TaskId) -> Pin<Box<dyn Future<Output = ()>>>) -> Result<TaskId, &'static str> {
        let (index, generation, id) = {
            let mut inner = self.inner.borrow_mut();
            if !inner.allocation_admitted {
                return Err("executor fixed storage was not admitted");
            }
            let index = inner.free.pop_front().ok_or("executor fixed task capacity is saturated")?;
            let generation = inner.slots[index].generation.wrapping_add(1).max(1);
            let id = Inner::task_id(index, generation);
            inner.slots[index] = TaskSlot { generation, future: None, queued: false, active: false, reserved: true, ready_previous: None, ready_next: None };
            (index, generation, id)
        };
        let future = make_future(id);
        let mut inner = self.inner.borrow_mut();
        if inner.slots[index].generation != generation || inner.slots[index].future.is_some() {
            inner.slots[index].reserved = false;
            inner.free.push_back(index);
            return Err("executor reservation authority changed before install");
        }
        inner.slots[index].future = Some(future);
        inner.slots[index].active = true;
        inner.slots[index].reserved = false;
        assert!(inner.enqueue_ready(index), "executor fixed ready authority rejected an installed task");
        inner.live += 1;
        Ok(id)
    }

    #[cfg(test)]
    pub fn reserve(&self) -> Result<TaskReservation, &'static str> {
        let mut inner = self.inner.borrow_mut();
        if !inner.allocation_admitted {
            return Err("executor fixed storage was not admitted");
        }
        let index = inner.free.pop_front().ok_or("executor fixed task capacity is saturated")?;
        let generation = inner.slots[index].generation.wrapping_add(1).max(1);
        let id = Inner::task_id(index, generation);
        inner.slots[index] = TaskSlot { generation, future: None, queued: false, active: false, reserved: true, ready_previous: None, ready_next: None };
        Ok(TaskReservation { inner: self.inner.clone(), id, installed: false })
    }

    /// 🚫️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): cancels a spawned task
    /// outright — drops its future (and everything IT owns, including any parked `RequestFuture`
    /// from `⚛️reactor/📮️requests`) without ever polling it again, and frees its slot for reuse.
    /// Used for key-dedupe (spawning onto a live `(instance, key)` cancels the stale task first)
    /// and `Event::InstanceClose` (every task that instance owns). Idempotent: cancelling an
    /// already-finished or unknown id is a no-op.
    #[cfg(test)]
    pub fn detach(&self, id: TaskId) -> Option<BoxedTask> {
        let mut inner = self.inner.borrow_mut();
        let index = inner.matches(id)?;
        if inner.slots[index].future.is_none() {
            return None;
        }
        inner.remove_ready(index);
        let future = inner.slots[index].future.take();
        inner.slots[index].active = false;
        inner.live = inner.live.saturating_sub(1);
        inner.free.push_back(index);
        future
    }

    /// 🔔️ Re-queues `id`. Idempotent within one turn (never double-queues an already-ready task).
    pub fn wake(&self, id: TaskId) {
        let mut inner = self.inner.borrow_mut();
        let Some(index) = inner.matches(id) else { return };
        inner.enqueue_ready(index);
    }

    /// ▶️ Drains the ready queue, polling each task at most once per pass, until either the queue
    /// is empty or `max_iterations` polls have run (a defensive bound against a task that keeps
    /// re-waking itself forever inside one turn — `reactor::poll` treats hitting the cap as
    /// `turn-status::more-work`, not `idle`). Returns whether any task is still alive (ready or
    /// parked) when it returns.
    pub fn run_until_deadline(&self, maximum_fuel: u32, deadline: std::time::Instant) -> bool {
        for _ in 0..maximum_fuel {
            if std::time::Instant::now() >= deadline {
                break;
            }
            let Some(ticket) = self.inner.borrow_mut().pop_ready() else {
                break;
            };
            let future = {
                let mut inner = self.inner.borrow_mut();
                let Some(index) = inner.matches(ticket) else { continue };
                inner.slots[index].future.take().map(|future| (index, future))
            };
            let Some((index, mut future)) = future else { continue };
            let generation = self.inner.borrow().slots[index].generation;
            let waker = self.waker_for(ticket);
            let mut cx = Context::from_waker(&waker);
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(()) => {
                    let mut inner = self.inner.borrow_mut();
                    if inner.slots[index].generation == generation && inner.slots[index].active {
                        inner.remove_ready(index);
                        inner.slots[index].active = false;
                        inner.live = inner.live.saturating_sub(1);
                        inner.free.push_back(index);
                    }
                }
                Poll::Pending => {
                    let mut inner = self.inner.borrow_mut();
                    if inner.slots[index].generation == generation && inner.slots[index].active {
                        inner.slots[index].future = Some(future);
                    }
                }
            }
        }
        self.has_pending()
    }

    #[cfg(test)]
    pub fn run_until_idle(&self, maximum_fuel: u32) -> bool {
        self.run_until_deadline(maximum_fuel, std::time::Instant::now() + std::time::Duration::from_millis(8))
    }

    pub fn poll_one(&self, id: TaskId) -> TaskPoll {
        let Some((index, generation)) = Inner::task_parts(id) else { return TaskPoll::Unknown };
        let future = {
            let mut inner = self.inner.borrow_mut();
            if inner.matches(id) != Some(index) {
                return TaskPoll::Unknown;
            }
            inner.remove_ready(index);
            inner.slots[index].future.take()
        };
        let Some(mut future) = future else { return TaskPoll::Pending };
        let waker = self.waker_for(id);
        let mut cx = Context::from_waker(&waker);
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(()) => {
                let mut inner = self.inner.borrow_mut();
                if inner.slots[index].generation == generation && inner.slots[index].active {
                    inner.remove_ready(index);
                    inner.slots[index].active = false;
                    inner.live = inner.live.saturating_sub(1);
                    inner.free.push_back(index);
                }
                TaskPoll::Complete
            }
            Poll::Pending => {
                let mut inner = self.inner.borrow_mut();
                if inner.slots[index].generation == generation && inner.slots[index].active {
                    inner.slots[index].future = Some(future);
                    TaskPoll::Pending
                } else {
                    TaskPoll::Unknown
                }
            }
        }
    }

    pub fn has_ready(&self) -> bool {
        self.inner.borrow().ready_head.is_some()
    }

    pub fn has_pending(&self) -> bool {
        self.inner.borrow().live != 0
    }

    fn waker_for(&self, id: TaskId) -> Waker {
        let data = Rc::new(WakerData { inner: self.inner.clone(), id });
        unsafe { Waker::from_raw(raw_waker(data)) }
    }
}

struct WakerData {
    inner: Rc<RefCell<Inner>>,
    id: TaskId,
}

// 🚫️async: E4 fn-pointer slot — RawWakerVTable::new requires bare `unsafe fn(*const ()) -> T`
// pointers; core::task calls these through a raw vtable and can never `.await` them.
fn raw_waker(data: Rc<WakerData>) -> RawWaker {
    RawWaker::new(Rc::into_raw(data) as *const (), &VTABLE)
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(waker_clone, waker_wake, waker_wake_by_ref, waker_drop);

// 🚫️async: E4 fn-pointer slot
unsafe fn waker_clone(ptr: *const ()) -> RawWaker {
    let data = unsafe { Rc::from_raw(ptr as *const WakerData) };
    let cloned = data.clone();
    std::mem::forget(data);
    raw_waker(cloned)
}

// 🚫️async: E4 fn-pointer slot
unsafe fn waker_wake(ptr: *const ()) {
    let data = unsafe { Rc::from_raw(ptr as *const WakerData) };
    let mut inner = data.inner.borrow_mut();
    let Some(index) = inner.matches(data.id) else { return };
    inner.enqueue_ready(index);
}

// 🚫️async: E4 fn-pointer slot
unsafe fn waker_wake_by_ref(ptr: *const ()) {
    let data = unsafe { &*(ptr as *const WakerData) };
    let mut inner = data.inner.borrow_mut();
    let Some(index) = inner.matches(data.id) else { return };
    inner.enqueue_ready(index);
}

// 🚫️async: E4 fn-pointer slot
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

    #[semio_framework_async_macros::async_test]
    async fn spawn_runs_a_ready_task_to_completion() {
        let executor = ColdFutureExecutor::new();
        let ran = Rc::new(Cell::new(false));
        let ran_inner = ran.clone();
        executor
            .spawn(async move {
                ran_inner.set(true);
            })
            .expect("fixed executor admission");
        let pending = executor.run_until_idle(8);
        assert!(ran.get(), "task body must have run");
        assert!(!pending, "no task should remain pending");
    }

    #[semio_framework_async_macros::async_test]
    async fn a_self_waking_task_is_polled_again_within_the_same_pass() {
        let executor = ColdFutureExecutor::new();
        executor.spawn(YieldOnce { yielded: false }).expect("fixed executor admission");
        let pending = executor.run_until_idle(8);
        assert!(!pending, "YieldOnce must complete within the iteration budget");
    }

    #[semio_framework_async_macros::async_test]
    async fn a_task_that_never_wakes_stays_pending_until_woken() {
        let executor = ColdFutureExecutor::new();
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
        executor.spawn(ParkForever { cell: waker_cell_inner, done: done_inner }).expect("fixed executor admission");
        let pending = executor.run_until_idle(8);
        assert!(pending, "task must stay parked until its waker fires");
        assert!(!executor.has_ready(), "a parked task must not remain in the ready queue");

        done.set(true);
        waker_cell.borrow().as_ref().expect("poll must have captured a waker").wake_by_ref();
        let pending = executor.run_until_idle(8);
        assert!(!pending, "waking must let the task observe `done` and complete");
    }

    #[semio_framework_async_macros::async_test]
    async fn cancel_before_the_first_run_until_idle_drops_the_future_without_ever_polling_it() {
        struct DropFlag(Rc<Cell<bool>>);
        impl Drop for DropFlag {
            fn drop(&mut self) {
                self.0.set(true);
            }
        }
        let executor = ColdFutureExecutor::new();
        let polled = Rc::new(Cell::new(false));
        let dropped = Rc::new(Cell::new(false));
        let polled_inner = polled.clone();
        let flag = DropFlag(dropped.clone());
        let id = executor
            .spawn(async move {
                let _flag = flag;
                polled_inner.set(true);
            })
            .expect("fixed executor admission");
        let detached = executor.detach(id).expect("exact detached future");
        let pending = executor.run_until_idle(8);
        assert!(!polled.get(), "a cancelled task's body must never run");
        assert!(!dropped.get(), "detaching must not synchronously drop the future");
        drop(detached);
        assert!(dropped.get(), "the explicit disposal owner controls the future's final drop");
        assert!(!pending, "nothing should remain pending after cancelling the only task");
    }

    #[semio_framework_async_macros::async_test]
    async fn cancel_of_a_parked_task_drops_it_and_frees_its_slot_for_reuse() {
        let executor = ColdFutureExecutor::new();
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
        let id = executor.spawn(ParkForever { cell: waker_cell_inner }).expect("fixed executor admission");
        let pending = executor.run_until_idle(8);
        assert!(pending, "task must be parked");
        drop(executor.detach(id).expect("exact detached future"));
        assert!(!executor.has_pending(), "cancelling the only parked task must clear has_pending");
        // 🔁️ The freed slot is reused by the next spawn — cancel must not leak the index forever.
        let reused = executor.spawn(async move {}).expect("reused fixed executor slot");
        assert_ne!(reused, id, "slot reuse must advance generation authority");
        assert_eq!(reused as u32, id as u32, "a detached slot must be reusable by a later spawn");
    }

    #[semio_framework_async_macros::async_test]
    async fn spawn_with_id_hands_the_reserved_id_to_the_future_builder_before_it_ever_runs() {
        let executor = ColdFutureExecutor::new();
        let seen_id: Rc<Cell<Option<TaskId>>> = Rc::new(Cell::new(None));
        let seen_id_inner = seen_id.clone();
        let id = executor
            .spawn_with_id(move |id| {
                Box::pin(async move {
                    seen_id_inner.set(Some(id));
                })
            })
            .expect("fixed executor admission");
        let pending = executor.run_until_idle(8);
        assert!(!pending);
        assert_eq!(seen_id.get(), Some(id), "the future must observe the SAME id spawn_with_id returned");
    }

    #[semio_framework_async_macros::async_test]
    async fn cancel_is_idempotent_for_an_unknown_or_already_finished_id() {
        let executor = ColdFutureExecutor::new();
        assert!(executor.detach(999).is_none());
        let id = executor.spawn(async move {}).expect("fixed executor admission");
        let _ = executor.run_until_idle(8); // finishes and frees the slot
        assert!(executor.detach(id).is_none());
        assert!(executor.detach(id).is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn detach_and_reuse_ten_times_capacity_never_accumulates_stale_ready_authority() {
        let executor = ColdFutureExecutor::new();
        for _ in 0..LOCAL_EXECUTOR_TASK_SLOTS * 10 {
            let id = executor.spawn(async move {}).expect("one fixed task slot remains reusable");
            drop(executor.detach(id).expect("ready task detaches by exact generation"));
            let inner = executor.inner.borrow();
            assert_eq!(inner.ready_len, 0);
            assert_eq!(inner.live, 0);
            assert_eq!(inner.free.len(), LOCAL_EXECUTOR_TASK_SLOTS);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn self_detach_during_poll_cannot_steal_or_drop_the_in_flight_future() {
        struct SelfDetach {
            executor: ColdFutureExecutor,
            id: TaskId,
            attempted: Rc<Cell<bool>>,
        }
        impl Future for SelfDetach {
            type Output = ();

            fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> StdPoll<()> {
                self.attempted.set(true);
                assert!(self.executor.detach(self.id).is_none(), "an in-flight future is detached from its slot while polled");
                StdPoll::Ready(())
            }
        }

        let executor = ColdFutureExecutor::new();
        let attempted = Rc::new(Cell::new(false));
        let reservation = executor.reserve().expect("fixed reservation");
        let id = reservation.id();
        reservation.install(Box::pin(SelfDetach { executor: executor.clone(), id, attempted: attempted.clone() }));
        assert!(!executor.run_until_idle(1));
        assert!(attempted.get());
        assert!(!executor.has_pending());
    }

    #[semio_framework_async_macros::async_test]
    async fn reactor_task_close_releases_one_nested_owner_per_step_and_only_then_drops_terminal_shell() {
        struct DropItem(Rc<Cell<usize>>);
        impl Drop for DropItem {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }
        struct BoundedTask {
            items: Vec<DropItem>,
        }
        impl ReactorTask for BoundedTask {
            fn step(&mut self, _budget: ReactorTaskBudget) -> ReactorTaskStep {
                ReactorTaskStep::Complete
            }

            fn close_step(&mut self, budget: ReactorTaskBudget) -> ReactorTaskStep {
                if budget.maximum_units == 0 || std::time::Instant::now() >= budget.deadline {
                    return ReactorTaskStep::Pending { processed_units: 0, processed_bytes: 0 };
                }
                if self.items.pop().is_some() { ReactorTaskStep::Pending { processed_units: 1, processed_bytes: 0 } } else { ReactorTaskStep::Complete }
            }

            fn terminal_is_empty(&self) -> bool {
                self.items.is_empty()
            }
        }

        let executor = ReactorExecutor::new();
        assert!(executor.pre_admit());
        let dropped = Rc::new(Cell::new(0));
        let task = BoundedTask { items: (0..3).map(|_| DropItem(dropped.clone())).collect() };
        assert!(executor.admit(17, 91, 4, 4, Box::new(task)).is_ok(), "fixed reactor task admission");
        assert!(executor.run_until_deadline(1, 0, std::time::Instant::now() + std::time::Duration::from_millis(8)));
        let mut cursor = 0;
        let zero = ReactorTaskBudget { operation: 0, generation: 0, cancellation_generation: 0, maximum_units: 0, maximum_bytes: 0, deadline: std::time::Instant::now() + std::time::Duration::from_millis(8) };
        assert_eq!(executor.close_instance_step(17, &mut cursor, zero), ReactorTaskStep::Pending { processed_units: 0, processed_bytes: 0 });
        assert_eq!(dropped.get(), 0, "zero fuel cannot release nested owners");
        for expected in 1..=3 {
            let budget = ReactorTaskBudget { operation: 0, generation: 0, cancellation_generation: 0, maximum_units: 1, maximum_bytes: 0, deadline: std::time::Instant::now() + std::time::Duration::from_millis(8) };
            assert_eq!(executor.close_instance_step(17, &mut cursor, budget), ReactorTaskStep::Pending { processed_units: 1, processed_bytes: 0 });
            assert_eq!(dropped.get(), expected, "one close step releases exactly one nested owner");
        }
        let budget = ReactorTaskBudget { operation: 0, generation: 0, cancellation_generation: 0, maximum_units: 1, maximum_bytes: 0, deadline: std::time::Instant::now() + std::time::Duration::from_millis(8) };
        assert!(matches!(executor.close_instance_step(17, &mut cursor, budget), ReactorTaskStep::Pending { processed_units: 1, processed_bytes: 0 } | ReactorTaskStep::Complete));
        assert!(!executor.has_pending());
        assert_eq!(dropped.get(), 3, "terminal task drop is empty and constant-time");
    }

    #[semio_framework_async_macros::async_test]
    async fn rejected_reactor_task_is_bounded_disposed_without_drop() {
        struct RejectedTask {
            remaining: usize,
        }
        impl ReactorTask for RejectedTask {
            fn step(&mut self, _budget: ReactorTaskBudget) -> ReactorTaskStep {
                ReactorTaskStep::Blocked { reason: "not admitted" }
            }

            fn close_step(&mut self, _budget: ReactorTaskBudget) -> ReactorTaskStep {
                if self.remaining == 0 {
                    ReactorTaskStep::Complete
                } else {
                    self.remaining -= 1;
                    ReactorTaskStep::Pending { processed_units: 1, processed_bytes: 0 }
                }
            }

            fn terminal_is_empty(&self) -> bool {
                self.remaining == 0
            }
        }

        let executor = ReactorExecutor::new();
        executor.inner.borrow_mut().allocation_admitted = false;
        let mut rejected = match executor.admit(1, 2, 3, 3, Box::new(RejectedTask { remaining: 2 })) {
            Ok(_) => panic!("forced admission failure was accepted"),
            Err(rejected) => rejected,
        };
        let budget = ReactorTaskBudget { operation: 2, generation: 3, cancellation_generation: 3, maximum_units: 1, maximum_bytes: 0, deadline: std::time::Instant::now() + std::time::Duration::from_millis(8) };
        assert!(matches!(rejected.close_step(budget), ReactorTaskStep::Pending { .. }));
        assert!(matches!(rejected.close_step(budget), ReactorTaskStep::Pending { .. }));
        assert_eq!(rejected.close_step(budget), ReactorTaskStep::Complete);
    }

    #[semio_framework_async_macros::async_test]
    async fn blocked_reactor_task_does_not_starve_ready_peer() {
        struct BlockedTask;
        impl ReactorTask for BlockedTask {
            fn step(&mut self, _budget: ReactorTaskBudget) -> ReactorTaskStep {
                ReactorTaskStep::Blocked { reason: "external wait" }
            }

            fn close_step(&mut self, _budget: ReactorTaskBudget) -> ReactorTaskStep {
                ReactorTaskStep::Complete
            }

            fn terminal_is_empty(&self) -> bool {
                true
            }
        }
        struct ReadyTask(Rc<Cell<bool>>);
        impl ReactorTask for ReadyTask {
            fn step(&mut self, _budget: ReactorTaskBudget) -> ReactorTaskStep {
                self.0.set(true);
                ReactorTaskStep::Complete
            }

            fn close_step(&mut self, _budget: ReactorTaskBudget) -> ReactorTaskStep {
                ReactorTaskStep::Complete
            }

            fn terminal_is_empty(&self) -> bool {
                true
            }
        }

        let executor = ReactorExecutor::new();
        assert!(executor.pre_admit());
        let blocked_id = match executor.admit(1, 1, 1, 1, Box::new(BlockedTask)) {
            Ok(id) => id,
            Err(_) => panic!("blocked fixture admission failed"),
        };
        let ran = Rc::new(Cell::new(false));
        assert!(executor.admit(1, 2, 1, 1, Box::new(ReadyTask(ran.clone()))).is_ok());
        executor.run_until_deadline(4, 0, std::time::Instant::now() + std::time::Duration::from_millis(8));
        assert!(ran.get(), "a blocked slot cannot end the bounded scan before an unrelated ready slot");
        assert!(executor.cancel(blocked_id, 2));
        let mut cursor = 0;
        let budget = ReactorTaskBudget { operation: 0, generation: 0, cancellation_generation: 0, maximum_units: 1, maximum_bytes: 0, deadline: std::time::Instant::now() + std::time::Duration::from_millis(8) };
        executor.close_instance_step(1, &mut cursor, budget);
        executor.close_instance_step(1, &mut cursor, budget);
        assert!(!executor.has_pending());
    }

    #[semio_framework_async_macros::async_test]
    async fn stale_generation_cannot_commit() {
        struct GenerationTask(Rc<Cell<bool>>);
        impl ReactorTask for GenerationTask {
            fn step(&mut self, _budget: ReactorTaskBudget) -> ReactorTaskStep {
                panic!("cancelled generation reached normal step")
            }

            fn close_step(&mut self, budget: ReactorTaskBudget) -> ReactorTaskStep {
                self.0.set(budget.generation == 7 && budget.cancellation_generation == 8);
                ReactorTaskStep::Complete
            }

            fn terminal_is_empty(&self) -> bool {
                true
            }
        }

        let executor = ReactorExecutor::new();
        assert!(executor.pre_admit());
        let observed = Rc::new(Cell::new(false));
        let id = match executor.admit(1, 9, 7, 7, Box::new(GenerationTask(observed.clone()))) {
            Ok(id) => id,
            Err(_) => panic!("fixed generation task admission failed"),
        };
        assert!(executor.cancel(id, 8));
        executor.run_until_deadline(1, 0, std::time::Instant::now() + std::time::Duration::from_millis(8));
        assert!(observed.get());
        assert!(!executor.has_pending());
    }

    #[semio_framework_async_macros::async_test]
    async fn reactor_executor_shutdown_drains_every_slot_before_terminal_drop() {
        struct EmptyTask;
        impl ReactorTask for EmptyTask {
            fn step(&mut self, _budget: ReactorTaskBudget) -> ReactorTaskStep {
                ReactorTaskStep::Pending { processed_units: 1, processed_bytes: 0 }
            }

            fn close_step(&mut self, _budget: ReactorTaskBudget) -> ReactorTaskStep {
                ReactorTaskStep::Complete
            }

            fn terminal_is_empty(&self) -> bool {
                true
            }
        }

        let executor = ReactorExecutor::new();
        assert!(executor.pre_admit());
        for operation in 0..LOCAL_EXECUTOR_TASK_SLOTS as u64 {
            assert!(executor.admit(1, operation, 1, 1, Box::new(EmptyTask)).is_ok());
        }
        executor.begin_shutdown();
        let budget = ReactorTaskBudget { operation: 0, generation: 0, cancellation_generation: 0, maximum_units: 1, maximum_bytes: 0, deadline: std::time::Instant::now() + std::time::Duration::from_secs(5) };
        for _ in 0..LOCAL_EXECUTOR_TASK_SLOTS * 3 + 1 {
            if executor.shutdown_step(budget) == ReactorTaskStep::Complete {
                break;
            }
        }
        let inner = executor.inner.borrow();
        assert_eq!(inner.live, 0);
        assert!(inner.free.is_empty());
        assert!(inner.slots.is_empty());
    }
}
