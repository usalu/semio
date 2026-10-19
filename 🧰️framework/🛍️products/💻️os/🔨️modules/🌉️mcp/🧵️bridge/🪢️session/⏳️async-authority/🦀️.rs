//! ⏳️ Async authority of the bridge session: the async state, the async authority and its scheduling, retry, drive and broadcast completion paths. See [`super`].

use super::*;

//#region 🔖️BridgeHandle
pub(crate) struct BridgeAsyncState {
    pub(crate) broadcasts: Box<[Option<BridgeBroadcastCursor>]>,
    pub(crate) broadcast_head: usize,
    pub(crate) broadcast_len: usize,
    pub(crate) broadcast_reserved: usize,
    pub(crate) broadcast_driving: bool,
    pub(crate) completions: Box<[Option<BridgeBroadcastCompletion>]>,
    pub(crate) completion_head: usize,
    pub(crate) completion_len: usize,
    pub(crate) completion_reserved: usize,
    pub(crate) retirements: Box<[Option<BridgeRetirementCursor>]>,
    pub(crate) retirement_head: usize,
    pub(crate) retirement_len: usize,
    pub(crate) retirement_reserved: usize,
    pub(crate) retirement_generation: u64,
}

impl BridgeAsyncState {
    pub(crate) fn new() -> Self {
        Self {
            broadcasts: semio_framework_async::boxed_slots(BRIDGE_BROADCAST_MAX_PENDING, || None),
            broadcast_head: 0,
            broadcast_len: 0,
            broadcast_reserved: 0,
            broadcast_driving: false,
            completions: semio_framework_async::boxed_slots(BRIDGE_BROADCAST_MAX_PENDING, || None),
            completion_head: 0,
            completion_len: 0,
            completion_reserved: 0,
            retirements: semio_framework_async::boxed_slots(BRIDGE_RETIREMENT_MAX_PENDING, || None),
            retirement_head: 0,
            retirement_len: 0,
            retirement_reserved: 0,
            retirement_generation: 1,
        }
    }

    fn has_work(&self) -> bool {
        self.broadcast_len != 0 || self.retirement_len != 0
    }
}

pub(crate) struct BridgeAsyncAuthority {
    pub pool: WorkerPool,
    pub state: Mutex<BridgeAsyncState>,
    pub scheduled: AtomicBool,
    pub retry_armed: AtomicBool,
    pub retry_generation: AtomicU64,
    pub retry_job: Mutex<Option<Job>>,
    pub terminal_job: Mutex<Option<(WorkerSubmitErrorKind, Job)>>,
    pub terminal: AtomicBool,
}

impl BridgeAsyncAuthority {
    pub(crate) fn new(pool: WorkerPool) -> Arc<Self> {
        Arc::new(Self {
            pool,
            state: Mutex::new(BridgeAsyncState::new()),
            scheduled: AtomicBool::new(false),
            retry_armed: AtomicBool::new(false),
            retry_generation: AtomicU64::new(0),
            retry_job: Mutex::new(None),
            terminal_job: Mutex::new(None),
            terminal: AtomicBool::new(false),
        })
    }

    pub(crate) fn reserve_broadcast(&self) -> Result<(), ()> {
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        if self.terminal.load(Ordering::Acquire) || state.broadcast_len.saturating_add(state.broadcast_reserved) == BRIDGE_BROADCAST_MAX_PENDING || state.completion_len.saturating_add(state.completion_reserved) == BRIDGE_BROADCAST_MAX_PENDING {
            return Err(());
        }
        state.broadcast_reserved += 1;
        state.completion_reserved += 1;
        Ok(())
    }

    pub(crate) fn cancel_broadcast_reservation(&self) {
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        state.broadcast_reserved = state.broadcast_reserved.saturating_sub(1);
        state.completion_reserved = state.completion_reserved.saturating_sub(1);
    }

    pub(crate) fn reserve_retirement(&self) -> Result<BridgeRetirementGrant, ()> {
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        if self.terminal.load(Ordering::Acquire) || state.retirement_len.saturating_add(state.retirement_reserved) == BRIDGE_RETIREMENT_MAX_PENDING {
            return Err(());
        }
        state.retirement_reserved += 1;
        Ok(BridgeRetirementGrant { generation: state.retirement_generation })
    }

    pub(crate) fn cancel_retirement(&self, grant: BridgeRetirementGrant) {
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        if grant.generation == state.retirement_generation {
            state.retirement_reserved = state.retirement_reserved.saturating_sub(1);
        }
    }

    pub(crate) fn enqueue_broadcast(self: &Arc<Self>, cursor: BridgeBroadcastCursor) {
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        state.broadcast_reserved = state.broadcast_reserved.saturating_sub(1);
        let index = (state.broadcast_head + state.broadcast_len) % BRIDGE_BROADCAST_MAX_PENDING;
        state.broadcasts[index] = Some(cursor);
        state.broadcast_len += 1;
        drop(state);
        self.request_schedule();
    }

    fn retain_completion(state: &mut BridgeAsyncState, completion: BridgeBroadcastCompletion) {
        state.completion_reserved = state.completion_reserved.saturating_sub(1);
        let index = (state.completion_head + state.completion_len) % BRIDGE_BROADCAST_MAX_PENDING;
        state.completions[index] = Some(completion);
        state.completion_len += 1;
    }

    pub(crate) fn publish_retirement(self: &Arc<Self>, grant: BridgeRetirementGrant, pages: [Option<Box<[u8; BRIDGE_OUTBOX_PAGE_BYTES]>>; BRIDGE_OUTBOX_MAX_PAGES]) {
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        if grant.generation != state.retirement_generation || state.retirement_reserved == 0 {
            drop(state);
            drop(pages);
            return;
        }
        state.retirement_reserved -= 1;
        let index = (state.retirement_head + state.retirement_len) % BRIDGE_RETIREMENT_MAX_PENDING;
        state.retirements[index] = Some(BridgeRetirementCursor { pages, page: 0 });
        state.retirement_len += 1;
        drop(state);
        self.request_schedule();
    }

    pub fn request_schedule(self: &Arc<Self>) {
        if self.terminal.load(Ordering::Acquire) || self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let authority = Arc::clone(self);
        self.submit_exact(Box::new(move || authority.drive_one()));
    }

    pub fn submit_exact(self: &Arc<Self>, job: Job) {
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => match error.kind() {
                WorkerSubmitErrorKind::Contended | WorkerSubmitErrorKind::Saturated => {
                    *self.retry_job.lock().expect("bridge retry lock poisoned") = Some(error.into_job());
                    self.arm_retry();
                }
                kind @ (WorkerSubmitErrorKind::Shutdown | WorkerSubmitErrorKind::Poisoned) => {
                    self.terminal.store(true, Ordering::Release);
                    self.retry_generation.fetch_add(1, Ordering::AcqRel);
                    *self.terminal_job.lock().expect("bridge terminal lock poisoned") = Some((kind, error.into_job()));
                }
            },
        }
    }

    pub fn arm_retry(self: &Arc<Self>) {
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let generation = self.retry_generation.fetch_add(1, Ordering::AcqRel).wrapping_add(1);
        let authority = Arc::clone(self);
        self.pool.callback_at(self.pool.now_ms().saturating_add(BRIDGE_ASYNC_RETRY_MS), move || {
            if generation != authority.retry_generation.load(Ordering::Acquire) {
                return;
            }
            authority.retry_armed.store(false, Ordering::Release);
            let job = authority.retry_job.lock().expect("bridge retry lock poisoned").take();
            if let Some(job) = job {
                authority.submit_exact(job);
            }
        });
    }

    pub fn drive_one(self: Arc<Self>) {
        if self.terminal.load(Ordering::Acquire) {
            self.scheduled.store(false, Ordering::Release);
            return;
        }
        let broadcast = {
            let mut state = self.state.lock().expect("bridge async lock poisoned");
            if state.broadcast_len == 0 || state.broadcast_driving {
                None
            } else {
                let head = state.broadcast_head;
                state.broadcast_driving = true;
                state.broadcasts[head].take()
            }
        };
        if let Some(cursor) = broadcast {
            let step = cursor.step();
            let mut state = self.state.lock().expect("bridge async lock poisoned");
            state.broadcast_driving = false;
            let head = state.broadcast_head;
            match step {
                BridgeBroadcastStep::Pending(cursor) => state.broadcasts[head] = Some(cursor),
                BridgeBroadcastStep::Complete(completion) => {
                    state.broadcast_head = (head + 1) % BRIDGE_BROADCAST_MAX_PENDING;
                    state.broadcast_len -= 1;
                    Self::retain_completion(&mut state, completion);
                }
            }
        } else {
            let page = {
                let mut state = self.state.lock().expect("bridge async lock poisoned");
                if state.retirement_len == 0 {
                    None
                } else {
                    let head = state.retirement_head;
                    let cursor = state.retirements[head].as_mut().expect("bridge retirement cursor disappeared");
                    while cursor.page < BRIDGE_OUTBOX_MAX_PAGES && cursor.pages[cursor.page].is_none() {
                        cursor.page += 1;
                    }
                    if cursor.page == BRIDGE_OUTBOX_MAX_PAGES {
                        state.retirements[head] = None;
                        state.retirement_head = (head + 1) % BRIDGE_RETIREMENT_MAX_PENDING;
                        state.retirement_len -= 1;
                        None
                    } else {
                        let page = cursor.pages[cursor.page].take();
                        cursor.page += 1;
                        page
                    }
                }
            };
            drop(page);
        }
        self.scheduled.store(false, Ordering::Release);
        if self.state.lock().expect("bridge async lock poisoned").has_work() {
            self.request_schedule();
        }
    }

    pub(crate) fn take_terminal_job(&self) -> Option<(WorkerSubmitErrorKind, Job)> {
        self.terminal_job.lock().expect("bridge terminal lock poisoned").take()
    }

    pub fn cancel(&self) {
        self.terminal.store(true, Ordering::Release);
        self.retry_generation.fetch_add(1, Ordering::AcqRel);
    }

    pub(crate) fn take_broadcast_completion(&self) -> Option<BridgeBroadcastCompletion> {
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        if state.completion_len == 0 {
            return None;
        }
        let head = state.completion_head;
        let completion = state.completions[head].take();
        state.completion_head = (head + 1) % BRIDGE_BROADCAST_MAX_PENDING;
        state.completion_len -= 1;
        completion
    }

    pub(crate) fn close_one_terminal_broadcast_claim(&self) -> bool {
        if !self.terminal.load(Ordering::Acquire) {
            return false;
        }
        let cursor = {
            let mut state = self.state.lock().expect("bridge async lock poisoned");
            if state.broadcast_len == 0 || state.broadcast_driving {
                return false;
            }
            let head = state.broadcast_head;
            state.broadcast_driving = true;
            state.broadcasts[head].take().expect("terminal bridge broadcast cursor disappeared")
        };
        let step = cursor.close_one_claim();
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        state.broadcast_driving = false;
        let head = state.broadcast_head;
        match step {
            BridgeBroadcastStep::Pending(cursor) => state.broadcasts[head] = Some(cursor),
            BridgeBroadcastStep::Complete(completion) => {
                state.broadcast_head = (head + 1) % BRIDGE_BROADCAST_MAX_PENDING;
                state.broadcast_len -= 1;
                Self::retain_completion(&mut state, completion);
            }
        }
        true
    }

    pub(crate) fn close_one_terminal_retired_page(&self) -> bool {
        if !self.terminal.load(Ordering::Acquire) {
            return false;
        }
        let page = {
            let mut state = self.state.lock().expect("bridge async lock poisoned");
            if state.retirement_len == 0 {
                return false;
            }
            let head = state.retirement_head;
            let cursor = state.retirements[head].as_mut().expect("bridge retirement cursor disappeared");
            while cursor.page < BRIDGE_OUTBOX_MAX_PAGES && cursor.pages[cursor.page].is_none() {
                cursor.page += 1;
            }
            let page = if cursor.page < BRIDGE_OUTBOX_MAX_PAGES {
                let page = cursor.pages[cursor.page].take();
                cursor.page += 1;
                page
            } else {
                None
            };
            if cursor.page == BRIDGE_OUTBOX_MAX_PAGES || cursor.pages[cursor.page..].iter().all(Option::is_none) {
                state.retirements[head] = None;
                state.retirement_head = (head + 1) % BRIDGE_RETIREMENT_MAX_PENDING;
                state.retirement_len -= 1;
            }
            page
        };
        drop(page);
        true
    }
}
//#endregion 🔖️BridgeHandle
