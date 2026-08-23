//! 🩹️ Retained fixed-admission reconciliation for mounted plugin surfaces.

use semio_framework_job::{CancelToken, Generation, OperationId, StepBudget, StepContext};
use semio_framework_ui_contract as ui_contract;
use semio_framework_ui_runtime::{ComponentTree, SurfaceReconcileJob, SurfaceReconcileJobStep, SurfaceReconcileRejected, SurfaceReconcileTerminal, SurfaceReconciler, SURFACE_RECONCILE_ADMISSION_SLOTS};
use std::cell::RefCell;
use std::collections::VecDeque;

const READY_PATCH_CAPACITY: usize = SURFACE_RECONCILE_ADMISSION_SLOTS;

struct SurfaceSlot {
    surface: String,
    generation: u64,
    operation: OperationId,
    preview_sequence: u64,
    cancel: CancelToken,
    reconciler: Option<SurfaceReconciler>,
    job: Option<SurfaceReconcileJob>,
}

struct TerminalSlot {
    instance: Option<u32>,
    authority: SurfaceReconcileTerminal,
    close: bool,
}

struct RejectedSlot {
    surface: String,
    authority: SurfaceReconcileRejected,
}

struct UnadmittedSlot {
    generation: u64,
    surface: String,
    tree: Option<ComponentTree>,
}

/// 🎟️ Exact mounted render reservation; the tree cannot exist before its fixed slot does.
pub struct MountedReconcileGrant<'a> {
    tracker: &'a PatchTracker,
    index: usize,
    generation: u64,
    active: bool,
}

impl MountedReconcileGrant<'_> {
    pub fn commit(mut self, tree: ComponentTree) {
        let mut state = self.tracker.state.borrow_mut();
        let slot = state.unadmitted[self.index].as_mut().filter(|slot| slot.generation == self.generation && slot.tree.is_none()).expect("mounted reservation remains generation-owned until commit");
        slot.tree = Some(tree);
        self.active = false;
    }

    pub fn cancel(mut self) {
        let mut state = self.tracker.state.borrow_mut();
        if state.unadmitted[self.index].as_ref().is_some_and(|slot| slot.generation == self.generation && slot.tree.is_none()) {
            state.unadmitted[self.index] = None;
        }
        self.active = false;
    }
}

impl Drop for MountedReconcileGrant<'_> {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        let mut state = self.tracker.state.borrow_mut();
        if state.unadmitted[self.index].as_ref().is_some_and(|slot| slot.generation == self.generation && slot.tree.is_none()) {
            state.unadmitted[self.index] = None;
        }
    }
}

#[derive(Clone, Copy)]
struct ClosingInstance {
    instance: u32,
}

struct PatchTrackerState {
    slots: [Option<SurfaceSlot>; SURFACE_RECONCILE_ADMISSION_SLOTS],
    rejected: [Option<RejectedSlot>; SURFACE_RECONCILE_ADMISSION_SLOTS],
    terminals: [Option<TerminalSlot>; SURFACE_RECONCILE_ADMISSION_SLOTS],
    deferred: [Option<String>; SURFACE_RECONCILE_ADMISSION_SLOTS],
    unadmitted: [Option<UnadmittedSlot>; SURFACE_RECONCILE_ADMISSION_SLOTS + 1],
    closing_instances: [Option<ClosingInstance>; SURFACE_RECONCILE_ADMISSION_SLOTS],
    ready: VecDeque<(u64, ui_contract::UiPatch)>,
    next_generation: u64,
    drive_cursor: usize,
    close_cursor: usize,
}

impl Default for PatchTrackerState {
    fn default() -> Self {
        Self {
            slots: std::array::from_fn(|_| None),
            rejected: std::array::from_fn(|_| None),
            terminals: std::array::from_fn(|_| None),
            deferred: std::array::from_fn(|_| None),
            unadmitted: std::array::from_fn(|_| None),
            closing_instances: [None; SURFACE_RECONCILE_ADMISSION_SLOTS],
            ready: VecDeque::with_capacity(READY_PATCH_CAPACITY),
            next_generation: 0,
            drive_cursor: 0,
            close_cursor: 0,
        }
    }
}

/// 🧵️ Fixed surface, rejected-owner, terminal-owner, and ready-publication authority.
#[derive(Default)]
pub struct PatchTracker {
    state: RefCell<PatchTrackerState>,
}

impl PatchTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn can_begin(&self, surface: &str) -> bool {
        if surface.len() > 256 {
            return false;
        }
        let state = self.state.borrow();
        if state.closing_instances.iter().flatten().any(|closing| surface_instance(surface) == Some(closing.instance)) || state.unadmitted.iter().all(Option::is_some) {
            return false;
        }
        if let Some(slot) = state.slots.iter().flatten().find(|slot| slot.surface == surface) {
            return slot.job.is_none() && slot.reconciler.is_some();
        }
        state.slots.iter().any(Option::is_none)
    }

    pub fn defer(&self, surface: String) -> bool {
        let mut state = self.state.borrow_mut();
        if state.closing_instances.iter().flatten().any(|closing| surface_instance(&surface) == Some(closing.instance)) {
            return false;
        }
        if state.deferred.iter().flatten().any(|queued| queued == &surface) {
            return true;
        }
        let Some(slot) = state.deferred.iter_mut().find(|slot| slot.is_none()) else { return false };
        *slot = Some(surface);
        true
    }

    pub fn take_deferred_ready(&self) -> Option<String> {
        let mut state = self.state.borrow_mut();
        let index = state.deferred.iter().position(|entry| entry.as_ref().is_some_and(|surface| state.slots.iter().flatten().find(|slot| slot.surface == *surface).is_none_or(|slot| slot.job.is_none() && slot.reconciler.is_some())))?;
        state.deferred[index].take()
    }

    pub fn begin(&self, surface: String, tree: ComponentTree) -> Result<u64, (String, ComponentTree)> {
        if surface.len() > 256 {
            return Err((surface, tree));
        }
        let generation = {
            let mut state = self.state.borrow_mut();
            state.next_generation = state.next_generation.checked_add(1).unwrap_or(u64::MAX);
            state.next_generation
        };
        self.begin_generation(surface, tree, generation)?;
        Ok(generation)
    }

    fn begin_generation(&self, surface: String, tree: ComponentTree, generation: u64) -> Result<(), (String, ComponentTree)> {
        let mut state = self.state.borrow_mut();
        let index = if let Some(index) = state.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.surface == surface)) {
            index
        } else if let Some(index) = state.slots.iter().position(Option::is_none) {
            state.slots[index] = Some(SurfaceSlot {
                reconciler: Some(SurfaceReconciler::new(surface.as_str())),
                surface: surface.clone(),
                generation: 0,
                operation: semio_framework_job::allocate_operation_id(),
                preview_sequence: 0,
                cancel: semio_framework_job::root_cancel_token(),
                job: None,
            });
            index
        } else {
            return Err((surface, tree));
        };
        if state.slots[index].as_ref().is_some_and(|slot| slot.job.is_some() || slot.reconciler.is_none()) {
            return Err((surface, tree));
        }
        let current = state.slots[index].as_mut().and_then(|slot| slot.reconciler.take()).expect("idle surface owns its reconciler");
        match SurfaceReconcileJob::try_new(current, tree, generation) {
            Ok(job) => {
                let slot = state.slots[index].as_mut().expect("surface slot was selected");
                slot.generation = generation;
                slot.operation = semio_framework_job::allocate_operation_id();
                slot.preview_sequence = 0;
                slot.cancel = semio_framework_job::root_cancel_token();
                slot.job = Some(job);
            }
            Err(rejected) => {
                state.slots[index].as_mut().expect("rejected surface slot").generation = generation;
                if let Some(rejected_slot) = state.rejected.iter_mut().find(|slot| slot.is_none()) {
                    *rejected_slot = Some(RejectedSlot { surface: surface.clone(), authority: rejected });
                }
            }
        }
        Ok(())
    }

    pub fn retain_unadmitted(&self, surface: String, tree: ComponentTree) -> Result<u64, (String, ComponentTree)> {
        if surface.len() > 256 {
            return Err((surface, tree));
        }
        let mut state = self.state.borrow_mut();
        let Some(index) = state.unadmitted.iter().position(Option::is_none) else { return Err((surface, tree)) };
        state.next_generation = state.next_generation.checked_add(1).unwrap_or(u64::MAX);
        let generation = state.next_generation;
        state.unadmitted[index] = Some(UnadmittedSlot { generation, surface, tree: Some(tree) });
        Ok(generation)
    }

    pub fn reserve_mounted(&self, surface: String) -> Result<MountedReconcileGrant<'_>, String> {
        if surface.len() > 256 {
            return Err(surface);
        }
        let mut state = self.state.borrow_mut();
        if state.closing_instances.iter().flatten().any(|closing| surface_instance(&surface) == Some(closing.instance)) {
            return Err(surface);
        }
        let Some(index) = state.unadmitted.iter().position(Option::is_none) else { return Err(surface) };
        state.next_generation = state.next_generation.checked_add(1).unwrap_or(u64::MAX);
        let generation = state.next_generation;
        state.unadmitted[index] = Some(UnadmittedSlot { generation, surface, tree: None });
        drop(state);
        Ok(MountedReconcileGrant { tracker: self, index, generation, active: true })
    }

    pub fn take_unadmitted(&self, generation: u64) -> Option<(String, ComponentTree)> {
        let mut state = self.state.borrow_mut();
        let slot = state.unadmitted.iter_mut().find(|slot| slot.as_ref().is_some_and(|entry| entry.generation == generation && entry.tree.is_some()))?;
        slot.take().and_then(|entry| entry.tree.map(|tree| (entry.surface, tree)))
    }

    pub fn drive_one(&self) -> bool {
        let retry = {
            let mut state = self.state.borrow_mut();
            state.unadmitted.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.tree.is_some())).and_then(|index| state.unadmitted[index].take().map(|owner| (index, owner)))
        };
        if let Some((index, mut owner)) = retry {
            let tree = owner.tree.take().expect("retry selected a rendered tree");
            let generation = owner.generation;
            let surface = owner.surface;
            if let Err((surface, tree)) = self.begin_generation(surface, tree, generation) {
                let mut state = self.state.borrow_mut();
                debug_assert!(state.unadmitted[index].is_none());
                state.unadmitted[index] = Some(UnadmittedSlot { generation, surface, tree: Some(tree) });
            }
            return true;
        }
        let mut state = self.state.borrow_mut();
        let index = state.drive_cursor;
        state.drive_cursor = (state.drive_cursor + 1) % SURFACE_RECONCILE_ADMISSION_SLOTS;
        let ready_has_capacity = state.ready.len() < READY_PATCH_CAPACITY;
        let Some(mut slot) = state.slots[index].take() else { return has_work(&state) };
        let Some(mut job) = slot.job.take() else {
            state.slots[index] = Some(slot);
            return has_work(&state);
        };
        if !ready_has_capacity && job.fault().is_none() {
            slot.job = Some(job);
            state.slots[index] = Some(slot);
            return true;
        }
        let mut preview_sequence = slot.preview_sequence;
        let context = StepContext::new(slot.operation, Generation(slot.generation), StepBudget::new(1, u64::MAX), slot.cancel.clone(), semio_framework_job::default_now_ms, &mut preview_sequence);
        let outcome = job.drive_one(&context);
        slot.preview_sequence = preview_sequence;
        match outcome {
            SurfaceReconcileJobStep::MoreWork => slot.job = Some(job),
            SurfaceReconcileJobStep::Ready => match job.take_ready() {
                Ok((reconciler, patch)) => {
                    slot.reconciler = Some(reconciler);
                    if let Some(patch) = patch {
                        state.ready.push_back((slot.generation, patch));
                    }
                }
                Err(job) => slot.job = Some(job),
            },
            SurfaceReconcileJobStep::Fault => {
                if let Some(terminal_slot) = state.terminals.iter_mut().find(|slot| slot.is_none()) {
                    *terminal_slot = Some(TerminalSlot { instance: surface_instance(&slot.surface), authority: job.into_terminal(), close: false });
                } else {
                    slot.job = Some(job);
                }
            }
        }
        state.slots[index] = Some(slot);
        has_work(&state)
    }

    pub fn has_work(&self) -> bool {
        has_work(&self.state.borrow())
    }

    pub fn take_ready_patch(&self) -> Option<ui_contract::UiPatch> {
        let mut state = self.state.borrow_mut();
        let ready_generation = state.ready.iter().filter(|(_, patch)| !state.closing_instances.iter().flatten().any(|closing| surface_instance(&patch.surface.0) == Some(closing.instance))).map(|(generation, _)| *generation).min()?;
        let pending_generation = state.slots.iter().flatten().filter(|slot| slot.job.is_some()).map(|slot| slot.generation).min();
        if pending_generation.is_some_and(|pending| pending < ready_generation) {
            return None;
        }
        let index = state.ready.iter().position(|(generation, _)| *generation == ready_generation)?;
        state.ready.remove(index).map(|(_, patch)| patch)
    }

    pub fn mark_rejected(&self, surface: &str) {
        let mut state = self.state.borrow_mut();
        let Some(index) = state.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.surface == surface)) else { return };
        state.next_generation = state.next_generation.checked_add(1).unwrap_or(u64::MAX);
        let generation = state.next_generation;
        let Some(target_index) = state.terminals.iter().position(Option::is_none) else { return };
        let terminal = {
            let slot = state.slots[index].as_mut().expect("surface slot existed");
            slot.cancel.cancel_now();
            let terminal = slot.job.take().map(SurfaceReconcileJob::into_terminal).or_else(|| slot.reconciler.take().map(|owner| SurfaceReconcileTerminal::from_reconciler(owner, generation)));
            slot.reconciler = Some(SurfaceReconciler::new(surface));
            terminal
        };
        if let Some(terminal) = terminal {
            state.terminals[target_index] = Some(TerminalSlot { instance: surface_instance(surface), authority: terminal, close: true });
        }
    }

    pub fn mark_ack(&self, _surface: &str, _revision: u64) {}

    pub fn revision(&self, surface: &str) -> ui_contract::UiRevision {
        self.state
            .borrow()
            .slots
            .iter()
            .flatten()
            .find(|slot| slot.surface == surface)
            .map(|slot| slot.reconciler.as_ref().map_or_else(|| slot.job.as_ref().map_or(ui_contract::UiRevision::default(), SurfaceReconcileJob::base_revision), SurfaceReconciler::revision))
            .unwrap_or_default()
    }

    pub fn begin_close_instance(&self, instance: u32) {
        let mut state = self.state.borrow_mut();
        if state.closing_instances.iter().flatten().any(|closing| closing.instance == instance) {
            return;
        }
        if let Some(slot) = state.closing_instances.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some(ClosingInstance { instance });
        }
    }

    pub fn close_step(&self) -> bool {
        let mut state = self.state.borrow_mut();
        if let Some(closing_index) = state.closing_instances.iter().position(Option::is_some) {
            let closing = state.closing_instances[closing_index].expect("closing instance existed");
            let terminal_target = state.terminals.iter().position(Option::is_none);
            if let (Some(ready_index), Some(target)) = (state.ready.iter().position(|(_, patch)| surface_instance(&patch.surface.0) == Some(closing.instance)), terminal_target) {
                let (generation, patch) = state.ready.remove(ready_index).expect("matching ready patch");
                state.terminals[target] = Some(TerminalSlot { instance: Some(closing.instance), authority: SurfaceReconcileTerminal::from_patch(patch, generation), close: true });
                return false;
            }
            if let Some(index) = state.deferred.iter().position(|entry| entry.as_ref().is_some_and(|surface| surface_instance(surface) == Some(closing.instance))) {
                state.deferred[index].take();
                return false;
            }
            if let Some(index) = state.unadmitted.iter().position(|entry| entry.as_ref().is_some_and(|entry| surface_instance(&entry.surface) == Some(closing.instance))) {
                if state.unadmitted[index].as_ref().is_some_and(|entry| entry.tree.is_none()) {
                    state.unadmitted[index] = None;
                    return false;
                }
                let Some(target) = terminal_target else { return false };
                let mut owner = state.unadmitted[index].take().expect("matching unadmitted tree");
                let tree = owner.tree.take().expect("rendered owner");
                let current = SurfaceReconciler::new(owner.surface.as_str());
                state.terminals[target] = Some(TerminalSlot { instance: Some(closing.instance), authority: SurfaceReconcileTerminal::from_sources(current, tree, owner.generation), close: true });
                return false;
            }
            if let Some(index) = state.rejected.iter().position(|entry| entry.as_ref().is_some_and(|entry| surface_instance(&entry.surface) == Some(closing.instance))) {
                if let Some(target) = terminal_target {
                    let rejected = state.rejected[index].take().expect("matching rejected owner");
                    state.terminals[target] = Some(TerminalSlot { instance: Some(closing.instance), authority: rejected.authority.into_terminal(), close: true });
                }
                return false;
            }
            if let Some(surface_index) = state.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| surface_instance(&slot.surface) == Some(closing.instance))) {
                let Some(target) = terminal_target else { return false };
                let mut surface = state.slots[surface_index].take().expect("matching closing surface");
                surface.cancel.cancel_now();
                let terminal = surface
                    .job
                    .take()
                    .map(SurfaceReconcileJob::into_terminal)
                    .or_else(|| surface.reconciler.take().map(|owner| SurfaceReconcileTerminal::from_reconciler(owner, surface.generation)))
                    .or_else(|| state.rejected.iter_mut().find(|entry| entry.as_ref().is_some_and(|rejected| rejected.authority.generation() == surface.generation)).and_then(Option::take).map(|rejected| rejected.authority.into_terminal));
                if let Some(terminal) = terminal {
                    state.terminals[target] = Some(TerminalSlot { instance: Some(closing.instance), authority: terminal, close: true });
                }
                return false;
            }
            if let Some(index) = state.terminals.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.instance == Some(closing.instance))) {
                let terminal = state.terminals[index].as_mut().expect("matching terminal");
                if terminal.authority.close_step() && terminal.authority.terminal_is_empty() {
                    state.terminals[index] = None;
                }
                return false;
            }
            state.closing_instances[closing_index] = None;
            return false;
        }
        let index = state.close_cursor;
        state.close_cursor = (state.close_cursor + 1) % SURFACE_RECONCILE_ADMISSION_SLOTS;
        let Some(terminal) = state.terminals[index].as_mut().filter(|slot| slot.close) else { return !state.terminals.iter().flatten().any(|slot| slot.close) };
        if terminal.authority.close_step() && terminal.authority.terminal_is_empty() {
            state.terminals[index] = None;
        }
        !state.terminals.iter().flatten().any(|slot| slot.close)
    }

    pub fn take_terminal(&self, generation: u64) -> Option<SurfaceReconcileTerminal> {
        let mut state = self.state.borrow_mut();
        let slot = state.terminals.iter_mut().find(|slot| slot.as_ref().is_some_and(|slot| slot.authority.generation() == generation))?;
        slot.take().map(|slot| slot.authority)
    }

    pub fn take_rejected(&self, generation: u64) -> Option<SurfaceReconcileRejected> {
        let mut state = self.state.borrow_mut();
        let slot = state.rejected.iter_mut().find(|slot| slot.as_ref().is_some_and(|slot| slot.authority.generation() == generation))?;
        slot.take().map(|slot| slot.authority)
    }

    pub fn resume_rejected(&self, rejected: SurfaceReconcileRejected) -> Result<(), SurfaceReconcileTerminal> {
        let generation = rejected.generation();
        let job = match rejected.retry(Default::default()) {
            Ok(job) => job,
            Err(rejected) => return Err(rejected.into_terminal()),
        };
        let mut state = self.state.borrow_mut();
        let Some(slot) = state.slots.iter_mut().flatten().find(|slot| slot.generation == generation && slot.job.is_none() && slot.reconciler.is_none()) else { return Err(job.into_terminal()) };
        slot.job = Some(job);
        Ok(())
    }

    pub fn resume_terminal(&self, terminal: SurfaceReconcileTerminal) -> Result<(), SurfaceReconcileTerminal> {
        let generation = terminal.generation();
        let job = terminal.resume(generation)?;
        let mut state = self.state.borrow_mut();
        let Some(slot) = state.slots.iter_mut().flatten().find(|slot| slot.generation == generation && slot.job.is_none()) else { return Err(job.into_terminal()) };
        slot.job = Some(job);
        Ok(())
    }

    pub fn terminal_is_empty(&self) -> bool {
        let state = self.state.borrow();
        state.slots.iter().all(Option::is_none)
            && state.rejected.iter().all(Option::is_none)
            && state.terminals.iter().all(Option::is_none)
            && state.unadmitted.iter().all(Option::is_none)
            && state.deferred.iter().all(Option::is_none)
            && state.ready.is_empty()
            && state.closing_instances.iter().all(Option::is_none)
    }
}

fn has_work(state: &PatchTrackerState) -> bool {
    state.slots.iter().flatten().any(|slot| slot.job.is_some())
        || state.terminals.iter().flatten().any(|slot| slot.close)
        || state.deferred.iter().any(Option::is_some)
        || state.unadmitted.iter().any(Option::is_some)
        || state.closing_instances.iter().any(Option::is_some)
        || !state.ready.is_empty()
}

fn surface_instance(surface: &str) -> Option<u32> {
    surface.split(':').next()?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_ui_runtime::TreeNode;

    fn leaf(key: &str, text: &str) -> ComponentTree {
        ComponentTree::new(TreeNode::new(key, ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label::from(text), emphasize: None, data_attributes: None })))
    }

    fn finish(tracker: &PatchTracker) -> Option<ui_contract::UiPatch> {
        for _ in 0..4_096 {
            tracker.drive_one();
            if let Some(patch) = tracker.take_ready_patch() {
                return Some(patch);
            }
            if !tracker.has_work() {
                return None;
            }
        }
        None
    }

    #[test]
    fn mounted_path_advances_one_reconcile_opportunity_per_grant() {
        let tracker = PatchTracker::new();
        tracker.begin("main".into(), leaf("root", "a")).expect("admitted");
        assert!(tracker.take_ready_patch().is_none());
        assert!(tracker.drive_one());
        assert!(tracker.take_ready_patch().is_none());
        let patch = finish(&tracker).expect("eventual patch");
        assert_eq!(patch.base_revision, ui_contract::UiRevision(0));
    }

    #[test]
    fn cap_plus_one_returns_the_exact_tree_owner() {
        let tracker = PatchTracker::new();
        {
            let mut state = tracker.state.borrow_mut();
            for index in 0..SURFACE_RECONCILE_ADMISSION_SLOTS {
                state.slots[index] = Some(SurfaceSlot {
                    surface: format!("{index}:main"),
                    generation: index as u64 + 1,
                    operation: semio_framework_job::allocate_operation_id(),
                    preview_sequence: 0,
                    cancel: semio_framework_job::root_cancel_token(),
                    reconciler: Some(SurfaceReconciler::new(format!("{index}:main"))),
                    job: None,
                });
            }
        }
        let tree = leaf("root", "exact");
        let pointer = tree.root.key.as_ptr();
        let (_, returned) = tracker.begin("65:main".into(), tree).expect_err("cap + 1");
        assert_eq!(returned.root.key.as_ptr(), pointer);
    }

    #[test]
    fn mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner() {
        let tracker = PatchTracker::new();
        let grant = tracker.reserve_mounted("4:mounted".into()).expect("fixed mounted reservation");
        let generation = grant.generation;
        assert_eq!(tracker.state.borrow().unadmitted.iter().flatten().filter(|owner| owner.tree.is_none()).count(), 1);
        let tree = leaf("root", "reserved");
        let pointer = tree.root.key.as_ptr();
        grant.commit(tree);
        let (_, returned) = tracker.take_unadmitted(generation).expect("generation-keyed mounted owner");
        assert_eq!(returned.root.key.as_ptr(), pointer);

        for index in 0..=SURFACE_RECONCILE_ADMISSION_SLOTS {
            tracker.retain_unadmitted(format!("{index}:queued"), leaf("root", "queued")).expect("fixed unadmitted slot");
        }
        let overflow = leaf("root", "overflow");
        let overflow_pointer = overflow.root.key.as_ptr();
        let (_, returned) = tracker.retain_unadmitted("66:queued".into(), overflow).expect_err("cap + 1 returns the exact tree");
        assert_eq!(returned.root.key.as_ptr(), overflow_pointer);
        assert!(tracker.reserve_mounted("67:mounted".into()).is_err(), "render cannot materialize before a fixed slot exists");
    }

    #[test]
    fn stale_generation_fault_is_publicly_retrievable() {
        let tracker = PatchTracker::new();
        let generation = tracker.begin("main".into(), leaf("root", "a")).expect("admitted");
        tracker.mark_rejected("main");
        let mut terminal = tracker.take_terminal(generation).expect("terminal owner");
        for _ in 0..32 {
            if terminal.close_step() && terminal.terminal_is_empty() {
                break;
            }
        }
        assert!(terminal.terminal_is_empty());
    }

    #[test]
    fn resize_storm_coalesces_to_one_deferred_surface_owner() {
        let tracker = PatchTracker::new();
        let generation = tracker.begin("7:main".into(), leaf("root", "a")).expect("admitted");
        for _ in 0..128 {
            assert!(tracker.defer("7:main".into()));
        }
        assert!(tracker.take_deferred_ready().is_none());
        tracker.mark_rejected("7:main");
        let mut terminal = tracker.take_terminal(generation).expect("cancelled owner");
        for _ in 0..32 {
            if terminal.close_step() && terminal.terminal_is_empty() {
                break;
            }
        }
        assert_eq!(tracker.take_deferred_ready().as_deref(), Some("7:main"));
        assert!(tracker.take_deferred_ready().is_none());
    }

    #[test]
    fn effects_publish_in_admission_order_even_when_later_tree_finishes_first() {
        let tracker = PatchTracker::new();
        tracker.begin("1:first".into(), leaf("root", "a")).expect("first");
        tracker.begin("1:second".into(), leaf("root", "b")).expect("second");
        let first = finish(&tracker).expect("first ready");
        let second = finish(&tracker).expect("second ready");
        assert_eq!(first.surface.0, "1:first");
        assert_eq!(second.surface.0, "1:second");
    }

    #[test]
    fn actor_close_retires_each_surface_and_old_generation_cannot_resume_reopened_slot() {
        let tracker = PatchTracker::new();
        let old = tracker.begin("9:first".into(), leaf("root", "a")).expect("old generation");
        tracker.mark_rejected("9:first");
        let terminal = tracker.take_terminal(old).expect("old terminal");
        tracker.begin("9:first".into(), leaf("root", "b")).expect("reopened generation");
        let terminal = tracker.resume_terminal(terminal).expect_err("old generation cannot mutate reopened slot");
        let mut terminal = terminal;
        for _ in 0..32 {
            if terminal.close_step() && terminal.terminal_is_empty() {
                break;
            }
        }
        tracker.begin_close_instance(9);
        for _ in 0..512 {
            if tracker.close_step() && tracker.terminal_is_empty() {
                break;
            }
        }
        assert!(tracker.terminal_is_empty());
    }

    #[test]
    fn close_retires_ready_deferred_unadmitted_active_and_terminal_owners_without_stale_publish() {
        let tracker = PatchTracker::new();
        tracker.begin("12:ready".into(), leaf("root", "ready")).expect("ready source");
        for _ in 0..128 {
            tracker.drive_one();
            if !tracker.state.borrow().ready.is_empty() {
                break;
            }
        }
        assert!(!tracker.state.borrow().ready.is_empty());
        assert!(tracker.defer("12:deferred".into()));
        tracker.retain_unadmitted("12:queued".into(), leaf("root", "queued")).expect("unadmitted");
        tracker.begin("12:active".into(), leaf("root", "active")).expect("active");
        {
            let mut state = tracker.state.borrow_mut();
            let target = state.terminals.iter_mut().find(|slot| slot.is_none()).expect("terminal capacity");
            *target = Some(TerminalSlot { instance: Some(12), authority: SurfaceReconcileTerminal::from_reconciler(SurfaceReconciler::new("12:terminal"), 90_012), close: true });
        }
        tracker.begin_close_instance(12);
        assert!(tracker.take_ready_patch().is_none(), "close prevents stale ready publication");
        for _ in 0..16_384 {
            tracker.close_step();
            if tracker.terminal_is_empty() {
                break;
            }
        }
        assert!(tracker.terminal_is_empty());
        assert!(tracker.take_ready_patch().is_none());
    }

    #[test]
    fn terminal_saturation_keeps_fault_job_in_its_surface_until_one_slot_is_freed() {
        let tracker = PatchTracker::new();
        {
            let mut state = tracker.state.borrow_mut();
            for (index, terminal) in state.terminals.iter_mut().enumerate() {
                *terminal = Some(TerminalSlot { instance: Some(44), authority: SurfaceReconcileTerminal::from_reconciler(SurfaceReconciler::new(format!("44:terminal-{index}")), 100_000 + index as u64), close: false });
            }
        }
        let generation = tracker.begin("44:active".into(), leaf("root", "active")).expect("active");
        tracker.mark_rejected("44:active");
        assert!(tracker.state.borrow().slots.iter().flatten().find(|slot| slot.surface == "44:active").is_some_and(|slot| slot.job.is_some()), "saturation retains the exact job locally");
        let mut released = tracker.take_terminal(100_000).expect("free one terminal grant");
        for _ in 0..32 {
            if released.close_step() && released.terminal_is_empty() {
                break;
            }
        }
        assert!(released.terminal_is_empty());
        tracker.mark_rejected("44:active");
        assert!(tracker.take_terminal(generation).is_some(), "freed capacity receives the original generation");
    }
}
