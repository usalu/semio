//! 🩹️ terra-brep-probe finding (see 📓️terra-brep-probe-report.md "Executor itself is broken"):
//! the LITERAL production file at
//! 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️executor/🦀️component.rs does NOT
//! compile standalone — confirmed by first attempting to `#[path]`-include it verbatim into this
//! probe and hitting 9 compiler errors. This is a byte-for-byte copy of that file with ONLY the
//! minimal fix applied, each one marked `// 🩹️ PATCH:` at the exact line. No other logic changed.
//!
//! ## The three-site patch
//! 1. `raw_waker` / `waker_clone` / `waker_wake` / `waker_wake_by_ref` / `waker_drop` were
//!    mechanically turned into `async fn`, but `core::task::RawWakerVTable::new` requires actual
//!    synchronous `unsafe fn(*const ()) -> T` function POINTERS — the std API itself calls these
//!    through a raw vtable, it can never `.await` them. This is a genuine "cannot be converted"
//!    site: reverted `raw_waker`/the four vtable fns back to plain `fn`/`unsafe fn`.
//! 2. `run_until_idle`'s two internal calls to now-async sibling methods (`self.waker_for(id)`,
//!    `self.has_pending()`) were missing the `.await` the conversion should have added. Added it.
//!
//! Everything else — struct shapes, the ready-queue/slot algorithm, doc comments — is preserved
//! verbatim from the original for anyone diffing this against the source of truth.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

pub type TaskId = u64;

type BoxedTask = Pin<Box<dyn Future<Output = ()>>>;

#[derive(Default)]
struct Inner {
    slots: Vec<Option<BoxedTask>>,
    free: Vec<usize>,
    ready: VecDeque<usize>,
}

#[derive(Clone, Default)]
pub struct LocalExecutor {
    inner: Rc<RefCell<Inner>>,
}

impl LocalExecutor {
    pub async fn new() -> Self {
        Self::default()
    }

    pub async fn spawn(&self, future: impl Future<Output = ()> + 'static) -> TaskId {
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

    pub async fn spawn_with_id(&self, make_future: impl FnOnce(TaskId) -> Pin<Box<dyn Future<Output = ()>>>) -> TaskId {
        let id = {
            let mut inner = self.inner.borrow_mut();
            if let Some(id) = inner.free.pop() {
                inner.slots[id] = None;
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

    pub async fn cancel(&self, id: TaskId) {
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

    pub async fn wake(&self, id: TaskId) {
        let mut inner = self.inner.borrow_mut();
        let id = id as usize;
        if id < inner.slots.len() && inner.slots[id].is_some() && !inner.ready.contains(&id) {
            inner.ready.push_back(id);
        }
    }

    pub async fn run_until_idle(&self, max_iterations: u32) -> bool {
        for _ in 0..max_iterations {
            let Some(id) = self.inner.borrow_mut().ready.pop_front() else {
                break;
            };
            let future = self.inner.borrow_mut().slots[id].take();
            let Some(mut future) = future else { continue };
            let waker = self.waker_for(id as TaskId).await; // 🩹️ PATCH: added `.await` (waker_for is async fn)
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
        self.has_pending().await // 🩹️ PATCH: added `.await` (has_pending is async fn)
    }

    pub async fn has_ready(&self) -> bool {
        !self.inner.borrow().ready.is_empty()
    }

    pub async fn has_pending(&self) -> bool {
        self.inner.borrow().slots.iter().any(Option::is_some)
    }

    async fn waker_for(&self, id: TaskId) -> Waker {
        let data = Rc::new(WakerData { inner: self.inner.clone(), id: id as usize });
        unsafe { Waker::from_raw(raw_waker(data)) } // (raw_waker is sync again, no await needed)
    }
}

struct WakerData {
    inner: Rc<RefCell<Inner>>,
    id: usize,
}

// 🩹️ PATCH: reverted to plain `fn` — see module doc point 1. A `RawWakerVTable` fn pointer must
// be callable synchronously by `core::task`'s own machinery; it can never be `async fn`.
fn raw_waker(data: Rc<WakerData>) -> RawWaker {
    RawWaker::new(Rc::into_raw(data) as *const (), &VTABLE)
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(waker_clone, waker_wake, waker_wake_by_ref, waker_drop);

// 🩹️ PATCH: reverted to plain `unsafe fn` — see module doc point 1.
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
