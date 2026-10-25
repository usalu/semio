//! 🏊️ Drawing mutation arena pool: the arena owner and builder, the pool with its slots and bootstrap job, the process state and the borrow functions.

use super::*;

/// 🎟️ Unbounded retained grant: the pre-migration step budget carried no retained limit.
const UNBOUNDED_STEP_GRANT: semio_framework_job::RetainedCloneGrant = semio_framework_job::RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX };

pub struct DrawingMutationArenaOwner {
    pub reverse: Vec<DrawingLayerNode>,
    pub output: Vec<DrawingLayerNode>,
    pub pages: Vec<String>,
    pub duplicate_id: String,
}

impl DrawingMutationArenaOwner {
    pub fn configured_totals() -> Result<(usize, usize), &'static str> {
        let items = DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY
            .checked_mul(2)
            .and_then(|items| items.checked_add(DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY))
            .and_then(|items| items.checked_add(1))
            .ok_or("drawing-store.mutation-arena-item-overflow")?;
        let bytes = size_of::<Self>()
            .checked_add(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY.checked_mul(size_of::<DrawingLayerNode>()).ok_or("drawing-store.mutation-arena-byte-overflow")?)
            .and_then(|bytes| bytes.checked_add(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY.checked_mul(size_of::<DrawingLayerNode>())?))
            .and_then(|bytes| bytes.checked_add(DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY.checked_mul(size_of::<String>())?))
            .and_then(|bytes| bytes.checked_add(DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY.checked_mul(DRAWING_MUTATION_RETAINED_PAGE_BYTES)?))
            .and_then(|bytes| bytes.checked_add(DRAWING_DUPLICATE_ID_BYTES))
            .ok_or("drawing-store.mutation-arena-byte-overflow")?;
        Ok((items, bytes))
    }

    pub fn admitted_totals(&self) -> Result<(usize, usize), &'static str> {
        Self::retained_totals(&self.reverse, &self.output, &self.pages, &self.duplicate_id)
    }

    pub fn retained_totals(reverse: &Vec<DrawingLayerNode>, output: &Vec<DrawingLayerNode>, pages: &Vec<String>, duplicate_id: &String) -> Result<(usize, usize), &'static str> {
        let items = reverse.capacity().checked_add(output.capacity()).and_then(|items| items.checked_add(pages.capacity())).and_then(|items| items.checked_add(1)).ok_or("drawing-store.mutation-arena-item-overflow")?;
        let bytes = size_of::<Self>()
            .checked_add(reverse.capacity().checked_mul(size_of::<DrawingLayerNode>()).ok_or("drawing-store.mutation-arena-byte-overflow")?)
            .and_then(|bytes| bytes.checked_add(output.capacity().checked_mul(size_of::<DrawingLayerNode>())?))
            .and_then(|bytes| bytes.checked_add(pages.capacity().checked_mul(size_of::<String>())?))
            .and_then(|bytes| pages.iter().try_fold(bytes, |total, page| total.checked_add(page.capacity())))
            .and_then(|bytes| bytes.checked_add(duplicate_id.capacity()))
            .ok_or("drawing-store.mutation-arena-byte-overflow")?;
        Ok((items, bytes))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.reverse.is_empty()
            && self.reverse.capacity() >= DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY
            && self.output.is_empty()
            && self.output.capacity() >= DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY
            && self.pages.len() == DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY
            && self.pages.capacity() >= DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY
            && self.pages.iter().all(|page| page.is_empty() && page.capacity() >= DRAWING_MUTATION_RETAINED_PAGE_BYTES)
            && self.duplicate_id.is_empty()
            && self.duplicate_id.capacity() >= DRAWING_DUPLICATE_ID_BYTES
    }
}

struct DrawingMutationArenaOwnerBuilder {
    pub reverse: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    pub output: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    pub pages: std::mem::ManuallyDrop<Option<Vec<String>>>,
    pub duplicate_id: std::mem::ManuallyDrop<Option<String>>,
    pub rejected_string: std::mem::ManuallyDrop<Option<String>>,
    pub phase: usize,
    pub terminal: bool,
}

impl DrawingMutationArenaOwnerBuilder {
    pub fn new() -> Self {
        Self {
            reverse: std::mem::ManuallyDrop::new(None),
            output: std::mem::ManuallyDrop::new(None),
            pages: std::mem::ManuallyDrop::new(None),
            duplicate_id: std::mem::ManuallyDrop::new(None),
            rejected_string: std::mem::ManuallyDrop::new(None),
            phase: 0,
            terminal: false,
        }
    }

    pub fn from_owner(owner: DrawingMutationArenaOwner) -> Self {
        Self {
            reverse: std::mem::ManuallyDrop::new(Some(owner.reverse)),
            output: std::mem::ManuallyDrop::new(Some(owner.output)),
            pages: std::mem::ManuallyDrop::new(Some(owner.pages)),
            duplicate_id: std::mem::ManuallyDrop::new(Some(owner.duplicate_id)),
            rejected_string: std::mem::ManuallyDrop::new(None),
            phase: 20,
            terminal: false,
        }
    }

    pub fn inject(allocation: &mut usize, failure_at: Option<usize>) -> bool {
        let current = *allocation;
        *allocation += 1;
        failure_at == Some(current)
    }

    pub fn step(&mut self, allocation: &mut usize, failure_at: Option<usize>) -> Result<bool, &'static str> {
        if self.phase >= 20 {
            return Ok(true);
        }
        if Self::inject(allocation, failure_at) {
            return Err("drawing-store.mutation-arena-bootstrap-injected-allocation");
        }
        match self.phase {
            0 => {
                let mut value = Vec::new();
                if value.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY).is_err() {
                    return Err("drawing-store.mutation-reverse-arena-admission");
                }
                *self.reverse = Some(value);
            }
            1 => {
                let mut value = Vec::new();
                if value.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY).is_err() {
                    return Err("drawing-store.mutation-output-arena-admission");
                }
                *self.output = Some(value);
            }
            2 => {
                let mut pages = Vec::new();
                if pages.try_reserve_exact(DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY).is_err() {
                    return Err("drawing-store.mutation-overlay-arena-admission");
                }
                *self.pages = Some(pages);
            }
            phase if phase < 3 + DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY => {
                let mut page = String::new();
                if page.try_reserve_exact(DRAWING_MUTATION_RETAINED_PAGE_BYTES).is_err() {
                    return Err("drawing-store.mutation-overlay-page-admission");
                }
                let Some(pages) = self.pages.as_mut() else {
                    *self.rejected_string = Some(page);
                    return Err("drawing-store.mutation-overlay-arena-missing");
                };
                if pages.len() >= pages.capacity() {
                    *self.rejected_string = Some(page);
                    return Err("drawing-store.mutation-overlay-arena-saturated");
                }
                pages.push(page);
            }
            19 => {
                let mut value = String::new();
                if value.try_reserve_exact(DRAWING_DUPLICATE_ID_BYTES).is_err() {
                    return Err("drawing-store.duplicate-id-owner-admission");
                }
                *self.duplicate_id = Some(value);
            }
            _ => return Err("drawing-store.mutation-arena-bootstrap-phase"),
        }
        self.phase += 1;
        Ok(self.phase == 20)
    }

    pub fn take(&mut self) -> Option<DrawingMutationArenaOwner> {
        if self.phase != 20 || self.terminal {
            return None;
        }
        if self.rejected_string.is_some() || self.reverse.is_none() || self.output.is_none() || self.pages.is_none() || self.duplicate_id.is_none() {
            return None;
        }
        let owner = DrawingMutationArenaOwner {
            reverse: self.reverse.take().expect("validated Drawing reverse bootstrap owner remains retained"),
            output: self.output.take().expect("validated Drawing output bootstrap owner remains retained"),
            pages: self.pages.take().expect("validated Drawing page bootstrap owner remains retained"),
            duplicate_id: self.duplicate_id.take().expect("validated Drawing duplicate bootstrap owner remains retained"),
        };
        self.terminal = true;
        Some(owner)
    }

    pub fn close_step(&mut self) -> RetainedCloneStep {
        let released = |released_bytes: usize| RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() });
        if let Some(value) = self.rejected_string.take() {
            let released_bytes = value.capacity();
            drop(value);
            return released(released_bytes);
        }
        if let Some(value) = self.duplicate_id.take() {
            let released_bytes = value.capacity();
            drop(value);
            return released(released_bytes);
        }
        if let Some(pages) = self.pages.as_mut() {
            if let Some(value) = pages.pop() {
                let released_bytes = value.capacity();
                drop(value);
                return released(released_bytes);
            }
        }
        if let Some(value) = self.pages.take() {
            let released_bytes = value.capacity().saturating_mul(size_of::<String>());
            drop(value);
            return released(released_bytes);
        }
        if let Some(value) = self.output.take() {
            let released_bytes = value.capacity().saturating_mul(size_of::<DrawingLayerNode>());
            drop(value);
            return released(released_bytes);
        }
        if let Some(value) = self.reverse.take() {
            let released_bytes = value.capacity().saturating_mul(size_of::<DrawingLayerNode>());
            drop(value);
            return released(released_bytes);
        }
        self.terminal = true;
        RetainedCloneStep::Complete(RetainedCloneProgress::default())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal && self.reverse.is_none() && self.output.is_none() && self.pages.is_none() && self.duplicate_id.is_none() && self.rejected_string.is_none()
    }
}

impl Drop for DrawingMutationArenaOwnerBuilder {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Drawing mutation arena owner builder reached Drop before exact construction handoff or retirement");
    }
}

pub struct DrawingMutationArenaPoolSlot {
    pub reverse: Option<Vec<DrawingLayerNode>>,
    pub output: Option<Vec<DrawingLayerNode>>,
    pub pages: Option<Vec<String>>,
    pub duplicate_id: Option<String>,
    pub generation: u64,
    pub leased: bool,
}

impl DrawingMutationArenaPoolSlot {
    pub fn new(owner: DrawingMutationArenaOwner) -> Self {
        Self { reverse: Some(owner.reverse), output: Some(owner.output), pages: Some(owner.pages), duplicate_id: Some(owner.duplicate_id), generation: 0, leased: false }
    }

    pub fn is_available(&self) -> bool {
        !self.leased && self.reverse.is_some() && self.output.is_some() && self.pages.is_some() && self.duplicate_id.is_some()
    }

    pub fn take(&mut self, generation: u64) -> Option<DrawingMutationArenaOwner> {
        if !self.is_available() {
            return None;
        }
        self.generation = generation;
        self.leased = true;
        Some(DrawingMutationArenaOwner {
            reverse: self.reverse.take().expect("available Drawing pool slot retains reverse owner"),
            output: self.output.take().expect("available Drawing pool slot retains output owner"),
            pages: self.pages.take().expect("available Drawing pool slot retains page owner"),
            duplicate_id: self.duplicate_id.take().expect("available Drawing pool slot retains duplicate owner"),
        })
    }
}

pub struct DrawingMutationArenaPoolState {
    pub slots: [DrawingMutationArenaPoolSlot; DRAWING_MUTATION_ARENA_POOL_CAPACITY],
}

pub struct DrawingMutationArenaPool {
    pub state: std::sync::Mutex<DrawingMutationArenaPoolState>,
    pub admitted_items: usize,
    pub admitted_bytes: usize,
}

pub struct DrawingMutationArenaPoolBootstrap {
    pub owners: std::mem::ManuallyDrop<[Option<DrawingMutationArenaOwner>; DRAWING_MUTATION_ARENA_POOL_CAPACITY]>,
    pub active: std::mem::ManuallyDrop<Option<DrawingMutationArenaOwnerBuilder>>,
    pub owner: usize,
    pub allocation: usize,
    pub failure_at: Option<usize>,
    pub failure_after_owner: Option<usize>,
    pub maximum_items: usize,
    pub maximum_bytes: usize,
    pub admitted_items: usize,
    pub admitted_bytes: usize,
    pub ready: bool,
    pub fault: Option<&'static str>,
    pub terminal: bool,
}

impl std::fmt::Debug for DrawingMutationArenaPoolBootstrap {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DrawingMutationArenaPoolBootstrap").field("owner", &self.owner).field("allocation", &self.allocation).field("fault", &self.fault).finish()
    }
}

impl DrawingMutationArenaPoolBootstrap {
    pub fn new(failure_at: Option<usize>, failure_after_owner: Option<usize>, maximum_items: usize, maximum_bytes: usize) -> Self {
        Self {
            owners: std::mem::ManuallyDrop::new(std::array::from_fn(|_| None)),
            active: std::mem::ManuallyDrop::new(None),
            owner: 0,
            allocation: 0,
            failure_at,
            failure_after_owner,
            maximum_items,
            maximum_bytes,
            admitted_items: 0,
            admitted_bytes: 0,
            ready: false,
            fault: None,
            terminal: false,
        }
    }

    pub fn production(admission: DrawingMutationArenaBootstrapAdmission) -> Self {
        Self::new(None, None, admission.maximum_items, admission.maximum_bytes)
    }

    pub fn fail(&mut self, fault: &'static str) -> Result<bool, &'static str> {
        self.fault = Some(fault);
        Err(fault)
    }

    pub fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if cx.should_yield() {
            return Ok(false);
        }
        if let Some(fault) = self.fault {
            return Err(fault);
        }
        if self.ready {
            return Ok(true);
        }
        if self.owner == DRAWING_MUTATION_ARENA_POOL_CAPACITY {
            if self.admitted_items > self.maximum_items || self.admitted_bytes > self.maximum_bytes {
                return self.fail("drawing-store.mutation-arena-pool-capacity");
            }
            self.ready = true;
            return Ok(true);
        }
        if self.active.is_none() {
            *self.active = Some(DrawingMutationArenaOwnerBuilder::new());
            return Ok(false);
        }
        let complete = match self.active.as_mut().expect("Drawing arena owner builder remains retained").step(&mut self.allocation, self.failure_at) {
            Ok(complete) => complete,
            Err(error) => return self.fail(error),
        };
        if !complete {
            return Ok(false);
        }
        let mut builder = self.active.take().expect("completed Drawing arena owner builder remains retained");
        let Some(owner) = builder.take() else {
            *self.active = Some(builder);
            return self.fail("drawing-store.mutation-arena-owner-false-terminal");
        };
        drop(builder);
        if !owner.terminal_is_empty() {
            self.owners[self.owner] = Some(owner);
            return self.fail("drawing-store.mutation-arena-pool-initial-owner");
        }
        let totals = owner.admitted_totals();
        let owner_index = self.owner;
        self.owners[owner_index] = Some(owner);
        self.owner += 1;
        if self.failure_after_owner == Some(owner_index) {
            return self.fail("drawing-store.mutation-arena-bootstrap-injected-owner");
        }
        let (items, bytes) = match totals {
            Ok(totals) => totals,
            Err(error) => return self.fail(error),
        };
        self.admitted_items = match self.admitted_items.checked_add(items) {
            Some(total) => total,
            None => return self.fail("drawing-store.mutation-arena-pool-item-overflow"),
        };
        self.admitted_bytes = match self.admitted_bytes.checked_add(bytes) {
            Some(total) => total,
            None => return self.fail("drawing-store.mutation-arena-pool-byte-overflow"),
        };
        Ok(false)
    }

    pub fn take_pool(&mut self) -> Option<std::sync::Arc<DrawingMutationArenaPool>> {
        if !self.ready || self.terminal {
            return None;
        }
        let owners = std::mem::replace(&mut *self.owners, std::array::from_fn(|_| None));
        let slots = owners.map(|owner| DrawingMutationArenaPoolSlot::new(owner.expect("validated Drawing arena bootstrap retains every owner")));
        self.terminal = true;
        Some(std::sync::Arc::new(DrawingMutationArenaPool { state: std::sync::Mutex::new(DrawingMutationArenaPoolState { slots }), admitted_items: self.admitted_items, admitted_bytes: self.admitted_bytes }))
    }

    pub fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> RetainedCloneStep {
        if cx.should_yield() {
            return RetainedCloneStep::Progress(RetainedCloneProgress::default());
        }
        if let Some(active) = self.active.as_mut() {
            return match active.close_step() {
                RetainedCloneStep::Complete(_) if active.terminal_is_empty() => {
                    drop(self.active.take());
                    RetainedCloneStep::Progress(RetainedCloneProgress::default())
                }
                step => RetainedCloneStep::Progress(step.progress()),
            };
        }
        if self.owner > 0 {
            self.owner -= 1;
            let owner = self.owners[self.owner].take().expect("Drawing bootstrap retirement cursor locates the preceding retained owner");
            *self.active = Some(DrawingMutationArenaOwnerBuilder::from_owner(owner));
            return RetainedCloneStep::Progress(RetainedCloneProgress::default());
        }
        self.terminal = true;
        RetainedCloneStep::Complete(RetainedCloneProgress::default())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal && self.active.is_none() && self.owners.iter().all(Option::is_none)
    }
}

impl Drop for DrawingMutationArenaPoolBootstrap {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Drawing mutation arena pool bootstrap reached Drop before exact handoff or fault retirement");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawingMutationArenaBootstrapAdmission {
    pub maximum_items: usize,
    pub maximum_bytes: usize,
}

impl DrawingMutationArenaBootstrapAdmission {
    pub fn fixed() -> Result<Self, &'static str> {
        let (owner_items, owner_bytes) = DrawingMutationArenaOwner::configured_totals()?;
        Ok(Self {
            maximum_items: owner_items.checked_mul(DRAWING_MUTATION_ARENA_POOL_CAPACITY).ok_or("drawing-store.mutation-arena-bootstrap-item-claim")?,
            maximum_bytes: owner_bytes.checked_mul(DRAWING_MUTATION_ARENA_POOL_CAPACITY).ok_or("drawing-store.mutation-arena-bootstrap-byte-claim")?,
        })
    }
}

pub enum DrawingMutationArenaProcessState {
    Inert,
    Building(DrawingMutationArenaPoolBootstrap),
    Ready(std::sync::Arc<DrawingMutationArenaPool>),
    Retiring(DrawingMutationArenaPoolBootstrap),
    Fault(&'static str),
}

pub static DRAWING_MUTATION_ARENA_POOL: std::sync::OnceLock<std::sync::Mutex<DrawingMutationArenaProcessState>> = std::sync::OnceLock::new();

static DRAWING_MUTATION_ARENA_BOOTSTRAP_REQUESTED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawingMutationArenaPoolAvailability {
    Ready,
    NotReady,
    Contended,
    Fault(&'static str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawingMutationArenaBorrowError {
    NotReady,
    Contended,
    Fault(&'static str),
    Invalid(&'static str),
}

impl DrawingMutationArenaBorrowError {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotReady => "drawing-store.mutation-arena-bootstrap-not-ready",
            Self::Contended => "drawing-store.mutation-arena-pool-contended",
            Self::Fault(fault) | Self::Invalid(fault) => fault,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawingMutationArenaBootstrapStep {
    Pending { advanced_items: u64 },
    Blocked,
    Ready,
    Cancelled,
    Fault(&'static str),
}

pub struct DrawingMutationArenaBootstrapJob {
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub admission: DrawingMutationArenaBootstrapAdmission,
    pub terminal: bool,
}

impl DrawingMutationArenaBootstrapJob {
    pub fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<Self, &'static str> {
        request_drawing_mutation_arena_pool();
        Ok(Self { operation, generation, admission: DrawingMutationArenaBootstrapAdmission::fixed()?, terminal: false })
    }

    pub fn inactive(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Self {
        Self { operation, generation, admission: DrawingMutationArenaBootstrapAdmission { maximum_items: 0, maximum_bytes: 0 }, terminal: true }
    }

    pub fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingMutationArenaBootstrapStep {
        if self.terminal {
            return DrawingMutationArenaBootstrapStep::Ready;
        }
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.terminal = true;
            return DrawingMutationArenaBootstrapStep::Fault("drawing-store.mutation-arena-bootstrap-stale-authority");
        }
        if cx.should_yield() {
            return DrawingMutationArenaBootstrapStep::Blocked;
        }
        let state = DRAWING_MUTATION_ARENA_POOL.get_or_init(|| std::sync::Mutex::new(DrawingMutationArenaProcessState::Inert));
        let Ok(mut state) = state.try_lock() else {
            return DrawingMutationArenaBootstrapStep::Blocked;
        };
        self.step_locked(&mut state, cx)
    }

    pub fn step_locked(&mut self, state: &mut DrawingMutationArenaProcessState, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingMutationArenaBootstrapStep {
        cx.set_stage("drawing-arena-bootstrap");
        if cx.is_cancelled() {
            match &*state {
                DrawingMutationArenaProcessState::Inert | DrawingMutationArenaProcessState::Ready(_) => {
                    self.terminal = true;
                    return DrawingMutationArenaBootstrapStep::Cancelled;
                }
                DrawingMutationArenaProcessState::Building(_) => {
                    let previous = std::mem::replace(&mut *state, DrawingMutationArenaProcessState::Fault("drawing-store.mutation-arena-bootstrap-transition"));
                    let DrawingMutationArenaProcessState::Building(mut bootstrap) = previous else { unreachable!("Drawing bootstrap cancellation preserves its exact building owner") };
                    bootstrap.fault = Some("drawing-store.mutation-arena-bootstrap-cancelled");
                    *state = DrawingMutationArenaProcessState::Retiring(bootstrap);
                    cx.consume_fuel(1);
                    return DrawingMutationArenaBootstrapStep::Pending { advanced_items: 1 };
                }
                DrawingMutationArenaProcessState::Retiring(_) => {}
                DrawingMutationArenaProcessState::Fault(fault) => {
                    self.terminal = true;
                    return DrawingMutationArenaBootstrapStep::Fault(fault);
                }
            }
        }
        let transition = match &mut *state {
            DrawingMutationArenaProcessState::Inert => {
                if !DRAWING_MUTATION_ARENA_BOOTSTRAP_REQUESTED.swap(false, std::sync::atomic::Ordering::AcqRel) {
                    return DrawingMutationArenaBootstrapStep::Blocked;
                }
                *state = DrawingMutationArenaProcessState::Building(DrawingMutationArenaPoolBootstrap::production(self.admission));
                cx.consume_fuel(1);
                return DrawingMutationArenaBootstrapStep::Pending { advanced_items: 1 };
            }
            DrawingMutationArenaProcessState::Building(bootstrap) => match bootstrap.step(cx) {
                Ok(true) => DrawingMutationArenaProcessTransition::Publish,
                Ok(false) => DrawingMutationArenaProcessTransition::None,
                Err(_) => DrawingMutationArenaProcessTransition::Retire,
            },
            DrawingMutationArenaProcessState::Ready(_) => {
                self.terminal = true;
                return DrawingMutationArenaBootstrapStep::Ready;
            }
            DrawingMutationArenaProcessState::Fault(error) => {
                self.terminal = true;
                return DrawingMutationArenaBootstrapStep::Fault(error);
            }
            DrawingMutationArenaProcessState::Retiring(bootstrap) => {
                let fault = bootstrap.fault.unwrap_or("drawing-store.mutation-arena-bootstrap-fault");
                if matches!(bootstrap.close_step(cx), RetainedCloneStep::Complete(_)) && bootstrap.terminal_is_empty() {
                    DrawingMutationArenaProcessTransition::Fault(fault)
                } else {
                    DrawingMutationArenaProcessTransition::None
                }
            }
        };
        cx.consume_fuel(1);
        match transition {
            DrawingMutationArenaProcessTransition::None => DrawingMutationArenaBootstrapStep::Pending { advanced_items: 1 },
            DrawingMutationArenaProcessTransition::Publish => {
                let previous = std::mem::replace(&mut *state, DrawingMutationArenaProcessState::Fault("drawing-store.mutation-arena-bootstrap-transition"));
                let DrawingMutationArenaProcessState::Building(mut bootstrap) = previous else { unreachable!("Drawing arena publish transition preserves the building owner") };
                let Some(pool) = bootstrap.take_pool() else {
                    *state = DrawingMutationArenaProcessState::Retiring(bootstrap);
                    return DrawingMutationArenaBootstrapStep::Pending { advanced_items: 1 };
                };
                drop(bootstrap);
                *state = DrawingMutationArenaProcessState::Ready(pool);
                self.terminal = true;
                DrawingMutationArenaBootstrapStep::Ready
            }
            DrawingMutationArenaProcessTransition::Retire => {
                let previous = std::mem::replace(&mut *state, DrawingMutationArenaProcessState::Fault("drawing-store.mutation-arena-bootstrap-transition"));
                let DrawingMutationArenaProcessState::Building(bootstrap) = previous else { unreachable!("Drawing arena fault transition preserves the building owner") };
                *state = DrawingMutationArenaProcessState::Retiring(bootstrap);
                DrawingMutationArenaBootstrapStep::Pending { advanced_items: 1 }
            }
            DrawingMutationArenaProcessTransition::Fault(fault) => {
                let previous = std::mem::replace(&mut *state, DrawingMutationArenaProcessState::Fault(fault));
                let DrawingMutationArenaProcessState::Retiring(bootstrap) = previous else { unreachable!("Drawing arena terminal fault transition preserves the retirement owner") };
                drop(bootstrap);
                self.terminal = true;
                if fault == "drawing-store.mutation-arena-bootstrap-cancelled" {
                    DrawingMutationArenaBootstrapStep::Cancelled
                } else {
                    DrawingMutationArenaBootstrapStep::Fault(fault)
                }
            }
        }
    }
}

impl DrawingMutationArenaPool {
    #[cfg(any(test, feature = "testkit"))]
    pub fn try_new() -> Result<std::sync::Arc<Self>, DrawingMutationArenaPoolBootstrap> {
        let mut bootstrap = DrawingMutationArenaPoolBootstrap::production(DrawingMutationArenaBootstrapAdmission::fixed().expect("fixed Drawing arena bootstrap claim"));
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        for _ in 0..=DRAWING_MUTATION_ARENA_POOL_CAPACITY * 24 {
            let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(7_901),
                semio_framework_job::Generation(79),
                semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT),
                cancel.clone(),
                semio_framework_job::default_now_us,
                &mut preview_sequence,
                &mut context_retained,
            );
            match bootstrap.step(&mut context) {
                Ok(true) => return Ok(bootstrap.take_pool().expect("completed Drawing arena bootstrap publishes exact pool")),
                Ok(false) => {}
                Err(_) => return Err(bootstrap),
            }
        }
        bootstrap.fault = Some("drawing-store.mutation-arena-bootstrap-turn-capacity");
        Err(bootstrap)
    }
}

enum DrawingMutationArenaProcessTransition {
    None,
    Publish,
    Retire,
    Fault(&'static str),
}

pub fn request_drawing_mutation_arena_pool() -> DrawingMutationArenaPoolAvailability {
    DRAWING_MUTATION_ARENA_BOOTSTRAP_REQUESTED.store(true, std::sync::atomic::Ordering::Release);
    let state = DRAWING_MUTATION_ARENA_POOL.get_or_init(|| std::sync::Mutex::new(DrawingMutationArenaProcessState::Inert));
    let Ok(state) = state.try_lock() else {
        return DrawingMutationArenaPoolAvailability::Contended;
    };
    match &*state {
        DrawingMutationArenaProcessState::Ready(_) => DrawingMutationArenaPoolAvailability::Ready,
        DrawingMutationArenaProcessState::Fault(fault) => DrawingMutationArenaPoolAvailability::Fault(fault),
        DrawingMutationArenaProcessState::Inert | DrawingMutationArenaProcessState::Building(_) | DrawingMutationArenaProcessState::Retiring(_) => DrawingMutationArenaPoolAvailability::NotReady,
    }
}

pub fn drawing_mutation_arena_pool_fault() -> Option<&'static str> {
    let state = DRAWING_MUTATION_ARENA_POOL.get()?;
    let state = state.try_lock().ok()?;
    match &*state {
        DrawingMutationArenaProcessState::Fault(fault) => Some(*fault),
        DrawingMutationArenaProcessState::Inert | DrawingMutationArenaProcessState::Building(_) | DrawingMutationArenaProcessState::Ready(_) | DrawingMutationArenaProcessState::Retiring(_) => None,
    }
}

pub fn borrow_drawing_mutation_arena_from(pool: std::sync::Arc<DrawingMutationArenaPool>) -> Result<(std::sync::Arc<DrawingMutationArenaPool>, usize, u64, DrawingMutationArenaOwner), &'static str> {
    if pool.admitted_items == 0 || pool.admitted_bytes == 0 {
        return Err("drawing-store.mutation-arena-pool-unadmitted");
    }
    let mut state = pool.state.try_lock().map_err(|_| "drawing-store.mutation-arena-pool-contended")?;
    let slot = state.slots.iter().position(DrawingMutationArenaPoolSlot::is_available).ok_or("drawing-store.mutation-arena-pool-saturated")?;
    let generation = state.slots[slot].generation.checked_add(1).ok_or("drawing-store.mutation-arena-generation-exhausted")?;
    let owner = state.slots[slot].take(generation).ok_or("drawing-store.mutation-arena-owner-missing")?;
    drop(state);
    Ok((pool, slot, generation, owner))
}

pub fn borrow_drawing_mutation_arena() -> Result<(std::sync::Arc<DrawingMutationArenaPool>, usize, u64, DrawingMutationArenaOwner), DrawingMutationArenaBorrowError> {
    match request_drawing_mutation_arena_pool() {
        DrawingMutationArenaPoolAvailability::Ready => {}
        DrawingMutationArenaPoolAvailability::NotReady => return Err(DrawingMutationArenaBorrowError::NotReady),
        DrawingMutationArenaPoolAvailability::Contended => return Err(DrawingMutationArenaBorrowError::Contended),
        DrawingMutationArenaPoolAvailability::Fault(fault) => return Err(DrawingMutationArenaBorrowError::Fault(fault)),
    }
    let state = DRAWING_MUTATION_ARENA_POOL.get().ok_or(DrawingMutationArenaBorrowError::Invalid("drawing-store.mutation-arena-pool-uninitialized"))?;
    let state = state.try_lock().map_err(|_| DrawingMutationArenaBorrowError::Contended)?;
    let DrawingMutationArenaProcessState::Ready(pool) = &*state else { return Err(DrawingMutationArenaBorrowError::NotReady) };
    let pool = pool.clone();
    drop(state);
    borrow_drawing_mutation_arena_from(pool).map_err(DrawingMutationArenaBorrowError::Invalid)
}
