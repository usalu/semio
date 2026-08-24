//! 🩹️ Retained fixed-admission reconciliation for mounted plugin surfaces.

use semio_framework_job::{CancelToken, Generation, OperationId, StepBudget, StepContext};
use semio_framework_ui_contract as ui_contract;
use semio_framework_ui_runtime::{
    ComponentTree, ComponentTreeProducer, ComponentTreeProducerStep, SurfaceReconcileJob, SurfaceReconcileJobStep, SurfaceReconcilePublishedAck, SurfaceReconcileReadyPatch, SurfaceReconcileRejected,
    SurfaceReconcileReservation, SurfaceReconcileTerminal, SurfaceReconciler, TreeNode, SURFACE_RECONCILE_ADMISSION_SLOTS,
};
use std::cell::RefCell;

const READY_PATCH_CAPACITY: usize = SURFACE_RECONCILE_ADMISSION_SLOTS;

struct SurfaceSlot {
    surface: String,
    surface_id: ui_contract::SurfaceId,
    generation: u64,
    operation: OperationId,
    preview_sequence: u64,
    acknowledged_revision: ui_contract::UiRevision,
    cancel: CancelToken,
    reconciler: Option<SurfaceReconciler>,
    producer: Option<MountedTreeProducer>,
    job: Option<SurfaceReconcileJob>,
}

struct MountedTreeProducer {
    reconciler: Option<SurfaceReconciler>,
    reservation: Option<SurfaceReconcileReservation>,
    rejected_index: usize,
    authority: ComponentTreeProducer,
}

struct MountedTreeTerminal {
    instance: Option<u32>,
    surface_index: Option<usize>,
    surface: String,
    reconciler: Option<SurfaceReconciler>,
    reservation: Option<SurfaceReconcileReservation>,
    authority: Option<ComponentTreeProducer>,
    close: bool,
}

impl MountedTreeTerminal {
    fn close_step(&mut self) -> bool {
        if let Some(authority) = self.authority.as_mut() {
            if !authority.close_step() {
                return false;
            }
            self.authority = None;
            return false;
        }
        if self.reservation.take().is_some() {
            return false;
        }
        if self.reconciler.take().is_some() {
            return false;
        }
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.authority.is_none() && self.reservation.is_none() && self.reconciler.is_none()
    }
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

struct ReadySlot {
    generation: u64,
    authority: SurfaceReconcileReadyPatch,
}

struct UnadmittedSlot {
    generation: u64,
    surface: String,
}

/// 🎟️ Exact mounted render reservation; the tree cannot exist before its fixed slot does.
pub struct MountedReconcileGrant<'a> {
    tracker: &'a PatchTracker,
    index: usize,
    surface_index: usize,
    rejected_index: usize,
    generation: u64,
    owner: MountedReconcileOwner,
    active: bool,
}

enum MountedReconcileOwner {
    Live { reconciler: SurfaceReconciler, reservation: SurfaceReconcileReservation },
    Transferred,
}

impl MountedReconcileGrant<'_> {
    pub fn commit_source(mut self, root: TreeNode) -> Result<(), TreeNode> {
        let mut state = self.tracker.state.borrow_mut();
        if state.unadmitted[self.index].as_ref().is_none_or(|slot| slot.generation != self.generation) {
            return Err(root);
        }
        let Some(mut slot) = state.slots[self.surface_index].take() else { return Err(root) };
        if slot.reconciler.is_some() || slot.producer.is_some() || slot.job.is_some() {
            state.slots[self.surface_index] = Some(slot);
            return Err(root);
        }
        let Some(marker) = state.unadmitted[self.index].take() else {
            state.slots[self.surface_index] = Some(slot);
            return Err(root);
        };
        let owner = std::mem::replace(&mut self.owner, MountedReconcileOwner::Transferred);
        let MountedReconcileOwner::Live { reconciler, reservation } = owner else {
            state.unadmitted[self.index] = Some(marker);
            state.slots[self.surface_index] = Some(slot);
            return Err(root);
        };
        let producer = match ComponentTreeProducer::try_new(root, self.generation) {
            Ok(producer) => producer,
            Err(root) => {
                self.owner = MountedReconcileOwner::Live { reconciler, reservation };
                state.unadmitted[self.index] = Some(marker);
                state.slots[self.surface_index] = Some(slot);
                return Err(root);
            }
        };
        slot.generation = marker.generation;
        slot.operation = semio_framework_job::allocate_operation_id();
        slot.preview_sequence = 0;
        slot.cancel = semio_framework_job::root_cancel_token();
        slot.producer = Some(MountedTreeProducer {
            reconciler: Some(reconciler),
            reservation: Some(reservation),
            rejected_index: self.rejected_index,
            authority: producer,
        });
        state.slots[self.surface_index] = Some(slot);
        self.active = false;
        Ok(())
    }

    #[cfg(test)]
    pub fn commit(mut self, tree: ComponentTree) {
        let mut state = self.tracker.state.borrow_mut();
        let owner = std::mem::replace(&mut self.owner, MountedReconcileOwner::Transferred);
        let MountedReconcileOwner::Live { reconciler, reservation } = owner else { return };
        let admission = SurfaceReconcileJob::try_new_reserved(reconciler, tree, reservation);
        let marker = state.unadmitted[self.index].take().filter(|slot| slot.generation == self.generation);
        let Some(marker) = marker else {
            drop(state);
            drop(admission);
            self.active = false;
            return;
        };
        let surface = marker.surface.clone();
        {
            let Some(slot) = state.slots[self.surface_index].as_mut().filter(|slot| slot.surface == marker.surface && slot.reconciler.is_none()) else {
                drop(state);
                drop(admission);
                self.active = false;
                return;
            };
            slot.generation = marker.generation;
            slot.operation = semio_framework_job::allocate_operation_id();
            slot.preview_sequence = 0;
            slot.cancel = semio_framework_job::root_cancel_token();
        }
        match admission {
            Ok(job) => {
                state.rejected_reserved[self.rejected_index] = None;
                if let Some(slot) = state.slots[self.surface_index].as_mut() {
                    slot.job = Some(job);
                } else {
                    drop(job);
                }
            }
            Err(authority) => {
                state.rejected_reserved[self.rejected_index] = None;
                state.rejected[self.rejected_index] = Some(RejectedSlot { surface, authority });
            }
        }
        self.active = false;
    }

    pub fn cancel(mut self) {
        let mut state = self.tracker.state.borrow_mut();
        if state.unadmitted[self.index].as_ref().is_some_and(|slot| slot.generation == self.generation) {
            state.unadmitted[self.index] = None;
        }
        if state.rejected_reserved[self.rejected_index] == Some(self.generation) {
            state.rejected_reserved[self.rejected_index] = None;
        }
        let owner = std::mem::replace(&mut self.owner, MountedReconcileOwner::Transferred);
        if let (MountedReconcileOwner::Live { reconciler, reservation }, Some(slot)) = (owner, state.slots[self.surface_index].as_mut()) {
            slot.reconciler = Some(reconciler);
            drop(reservation);
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
        if state.unadmitted[self.index].as_ref().is_some_and(|slot| slot.generation == self.generation) {
            state.unadmitted[self.index] = None;
        }
        if state.rejected_reserved[self.rejected_index] == Some(self.generation) {
            state.rejected_reserved[self.rejected_index] = None;
        }
        let owner = std::mem::replace(&mut self.owner, MountedReconcileOwner::Transferred);
        if let (MountedReconcileOwner::Live { reconciler, reservation }, Some(slot)) = (owner, state.slots[self.surface_index].as_mut()) {
            slot.reconciler = Some(reconciler);
            drop(reservation);
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
    producer_terminals: [Option<MountedTreeTerminal>; SURFACE_RECONCILE_ADMISSION_SLOTS],
    rejected_reserved: [Option<u64>; SURFACE_RECONCILE_ADMISSION_SLOTS],
    deferred: [Option<String>; SURFACE_RECONCILE_ADMISSION_SLOTS],
    unadmitted: [Option<UnadmittedSlot>; SURFACE_RECONCILE_ADMISSION_SLOTS + 1],
    closing_instances: [Option<ClosingInstance>; SURFACE_RECONCILE_ADMISSION_SLOTS],
    ready: [Option<ReadySlot>; READY_PATCH_CAPACITY],
    next_generation: u64,
    generation_exhausted: bool,
    drive_cursor: usize,
    close_cursor: usize,
}

impl Default for PatchTrackerState {
    fn default() -> Self {
        Self {
            slots: std::array::from_fn(|_| None),
            rejected: std::array::from_fn(|_| None),
            terminals: std::array::from_fn(|_| None),
            producer_terminals: std::array::from_fn(|_| None),
            rejected_reserved: [None; SURFACE_RECONCILE_ADMISSION_SLOTS],
            deferred: std::array::from_fn(|_| None),
            unadmitted: std::array::from_fn(|_| None),
            closing_instances: [None; SURFACE_RECONCILE_ADMISSION_SLOTS],
            ready: std::array::from_fn(|_| None),
            next_generation: 0,
            generation_exhausted: false,
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
            return slot.producer.is_none()
                && slot.job.is_none()
                && slot.reconciler.as_ref().is_some_and(|reconciler| slot.acknowledged_revision.0 >= reconciler.revision().0);
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
        let index = state.deferred.iter().position(|entry| {
            entry.as_ref().is_some_and(|surface| {
                state.slots.iter().flatten().find(|slot| slot.surface == *surface).is_none_or(|slot| {
                    slot.producer.is_none()
                        && slot.job.is_none()
                        && slot.reconciler.as_ref().is_some_and(|reconciler| slot.acknowledged_revision.0 >= reconciler.revision().0)
                })
            })
        })?;
        state.deferred[index].take()
    }

    #[cfg(test)]
    pub fn begin(&self, surface: String, tree: ComponentTree) -> Result<u64, (String, ComponentTree)> {
        if surface.len() > 256 {
            return Err((surface, tree));
        }
        let mut state = self.state.borrow_mut();
        let reusable = state.slots.iter().flatten().find(|slot| slot.surface == surface);
        if reusable.is_some_and(|slot| slot.producer.is_some() || slot.job.is_some() || slot.reconciler.is_none()) || reusable.is_none() && state.slots.iter().all(Option::is_some) {
            return Err((surface, tree));
        }
        let Some(generation) = next_generation(&state) else { return Err((surface, tree)) };
        let Some(reservation) = SurfaceReconcileReservation::try_new(generation) else { return Err((surface, tree)) };
        commit_generation(&mut state, generation);
        drop(state);
        self.begin_generation(surface, tree, generation, reservation)?;
        Ok(generation)
    }

    #[cfg(test)]
    fn begin_generation(&self, surface: String, tree: ComponentTree, generation: u64, reservation: SurfaceReconcileReservation) -> Result<(), (String, ComponentTree)> {
        let Some(surface_text) = ui_contract::UiText::try_from_str(&surface) else { return Err((surface, tree)) };
        let mut state = self.state.borrow_mut();
        let index = if let Some(index) = state.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.surface == surface)) {
            index
        } else if let Some(index) = state.slots.iter().position(Option::is_none) {
            state.slots[index] = Some(SurfaceSlot {
                reconciler: Some(SurfaceReconciler::new(surface.as_str())),
                surface: surface.clone(),
                surface_id: ui_contract::SurfaceId(surface_text),
                generation: 0,
                operation: semio_framework_job::allocate_operation_id(),
                preview_sequence: 0,
                acknowledged_revision: ui_contract::UiRevision::default(),
                cancel: semio_framework_job::root_cancel_token(),
                producer: None,
                job: None,
            });
            index
        } else {
            return Err((surface, tree));
        };
        if state.slots[index].as_ref().is_some_and(|slot| slot.producer.is_some() || slot.job.is_some() || slot.reconciler.is_none()) {
            return Err((surface, tree));
        }
        let Some(rejected_index) = state.rejected.iter().position(Option::is_none) else { return Err((surface, tree)) };
        let Some(current) = state.slots[index].as_mut().and_then(|slot| slot.reconciler.take()) else { return Err((surface, tree)) };
        let admission = SurfaceReconcileJob::try_new_reserved(current, tree, reservation);
        match admission {
            Ok(job) => {
                let Some(slot) = state.slots[index].as_mut() else {
                    drop(job);
                    return Ok(());
                };
                slot.generation = generation;
                slot.operation = semio_framework_job::allocate_operation_id();
                slot.preview_sequence = 0;
                slot.cancel = semio_framework_job::root_cancel_token();
                slot.job = Some(job);
            }
            Err(rejected) => {
                if let Some(slot) = state.slots[index].as_mut() {
                    slot.generation = generation;
                    state.rejected[rejected_index] = Some(RejectedSlot { surface: surface.clone(), authority: rejected });
                } else {
                    drop(rejected);
                }
            }
        }
        Ok(())
    }

    #[cfg(test)]
    pub fn retain_unadmitted(&self, surface: String, tree: ComponentTree) -> Result<u64, (String, ComponentTree)> {
        self.begin(surface, tree)
    }

    pub fn reserve_mounted(&self, surface: String) -> Result<MountedReconcileGrant<'_>, String> {
        if surface.len() > 256 {
            return Err(surface);
        }
        let Some(surface_text) = ui_contract::UiText::try_from_str(&surface) else { return Err(surface) };
        let mut state = self.state.borrow_mut();
        if state.closing_instances.iter().flatten().any(|closing| surface_instance(&surface) == Some(closing.instance)) {
            return Err(surface);
        }
        let Some(index) = state.unadmitted.iter().position(Option::is_none) else { return Err(surface) };
        let surface_index = if let Some(index) = state.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.surface == surface)) {
            index
        } else if let Some(index) = state.slots.iter().position(Option::is_none) {
            state.slots[index] = Some(SurfaceSlot {
                reconciler: Some(SurfaceReconciler::new(ui_contract::SurfaceId(surface_text.clone()))),
                surface: surface.clone(),
                surface_id: ui_contract::SurfaceId(surface_text),
                generation: 0,
                operation: semio_framework_job::allocate_operation_id(),
                preview_sequence: 0,
                acknowledged_revision: ui_contract::UiRevision::default(),
                cancel: semio_framework_job::root_cancel_token(),
                producer: None,
                job: None,
            });
            index
        } else {
            return Err(surface);
        };
        if state.slots[surface_index].as_ref().is_none_or(|slot| slot.producer.is_some() || slot.job.is_some() || slot.reconciler.is_none()) {
            return Err(surface);
        }
        let Some(generation) = next_generation(&state) else { return Err(surface) };
        let Some(rejected_index) = state.rejected.iter().enumerate().find_map(|(index, slot)| {
            (slot.is_none() && state.rejected_reserved[index].is_none()).then_some(index)
        }) else {
            return Err(surface);
        };
        let Some(reconciler) = state.slots[surface_index].as_mut().and_then(|slot| slot.reconciler.take()) else { return Err(surface) };
        let Some(reservation) = SurfaceReconcileReservation::try_new(generation) else {
            if let Some(slot) = state.slots[surface_index].as_mut() {
                slot.reconciler = Some(reconciler);
            }
            return Err(surface);
        };
        commit_generation(&mut state, generation);
        state.rejected_reserved[rejected_index] = Some(generation);
        state.unadmitted[index] = Some(UnadmittedSlot { generation, surface });
        drop(state);
        Ok(MountedReconcileGrant {
            tracker: self,
            index,
            surface_index,
            rejected_index,
            generation,
            owner: MountedReconcileOwner::Live { reconciler, reservation },
            active: true,
        })
    }

    pub fn drive_one(&self) -> bool {
        let mut state = self.state.borrow_mut();
        let index = state.drive_cursor;
        state.drive_cursor = (state.drive_cursor + 1) % SURFACE_RECONCILE_ADMISSION_SLOTS;
        let ready_index = state.ready.iter().position(Option::is_none);
        let ready_has_capacity = ready_index.is_some();
        let Some(mut slot) = state.slots[index].take() else { return has_work(&state) };
        if let Some(mut producer) = slot.producer.take() {
            let mut preview_sequence = slot.preview_sequence;
            let mut context = StepContext::new(
                slot.operation,
                Generation(slot.generation),
                StepBudget::new(1, u64::MAX),
                slot.cancel.clone(),
                semio_framework_job::default_now_ms,
                &mut preview_sequence,
            );
            let outcome = producer.authority.step(slot.generation, context.is_cancelled(), context.deadline_exceeded());
            context.consume_fuel(1);
            slot.preview_sequence = preview_sequence;
            match outcome {
                ComponentTreeProducerStep::MoreWork => slot.producer = Some(producer),
                ComponentTreeProducerStep::Complete => {
                    let sources = match (producer.reconciler.take(), producer.reservation.take(), producer.authority.take_complete()) {
                        (Some(reconciler), Some(reservation), Some(tree)) => Some((reconciler, reservation, tree)),
                        _ => None,
                    };
                    if let Some((reconciler, reservation, tree)) = sources {
                        match SurfaceReconcileJob::try_new_reserved(reconciler, tree, reservation) {
                            Ok(job) => {
                                if state.rejected_reserved[producer.rejected_index] == Some(slot.generation) {
                                    state.rejected_reserved[producer.rejected_index] = None;
                                }
                                slot.job = Some(job);
                            }
                            Err(authority) => {
                                state.rejected_reserved[producer.rejected_index] = None;
                                state.rejected[producer.rejected_index] = Some(RejectedSlot { surface: slot.surface.clone(), authority });
                            }
                        }
                    } else if let Some(target) = state.producer_terminals.iter().position(Option::is_none) {
                        state.rejected_reserved[producer.rejected_index] = None;
                        state.producer_terminals[target] = Some(MountedTreeTerminal {
                            instance: surface_instance(&slot.surface),
                            surface_index: Some(index),
                            surface: slot.surface.clone(),
                            reconciler: producer.reconciler.take(),
                            reservation: producer.reservation.take(),
                            authority: Some(producer.authority),
                            close: true,
                        });
                    } else {
                        slot.producer = Some(producer);
                    }
                }
                ComponentTreeProducerStep::Fault(_) => {
                    if let Some(target) = state.producer_terminals.iter().position(Option::is_none) {
                        state.rejected_reserved[producer.rejected_index] = None;
                        state.producer_terminals[target] = Some(MountedTreeTerminal {
                            instance: surface_instance(&slot.surface),
                            surface_index: Some(index),
                            surface: slot.surface.clone(),
                            reconciler: producer.reconciler.take(),
                            reservation: producer.reservation.take(),
                            authority: Some(producer.authority),
                            close: true,
                        });
                    } else {
                        slot.producer = Some(producer);
                    }
                }
            }
            state.slots[index] = Some(slot);
            return has_work(&state);
        }
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
        let mut context = StepContext::new(slot.operation, Generation(slot.generation), StepBudget::new(1, u64::MAX), slot.cancel.clone(), semio_framework_job::default_now_ms, &mut preview_sequence);
        let outcome = job.drive_one(&mut context);
        slot.preview_sequence = preview_sequence;
        match outcome {
            SurfaceReconcileJobStep::MoreWork => slot.job = Some(job),
            SurfaceReconcileJobStep::Ready => match job.take_ready() {
                Ok((reconciler, patch)) => {
                    if patch.is_none() {
                        slot.acknowledged_revision = reconciler.revision();
                    }
                    slot.reconciler = Some(reconciler);
                    if let Some(patch) = patch {
                        if let Some(ready_index) = ready_index {
                            state.ready[ready_index] = Some(ReadySlot { generation: slot.generation, authority: patch });
                        } else {
                            drop(patch);
                        }
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

    pub fn take_ready_patch(&self) -> Option<SurfaceReconcileReadyPatch> {
        let mut state = self.state.borrow_mut();
        let ready_generation = state
            .ready
            .iter()
            .flatten()
            .filter(|ready| !state.closing_instances.iter().flatten().any(|closing| ready.authority.surface().and_then(|surface| surface_instance(&surface.0)) == Some(closing.instance)))
            .map(|ready| ready.generation)
            .min()?;
        let pending_generation = state.slots.iter().flatten().filter(|slot| slot.producer.is_some() || slot.job.is_some()).map(|slot| slot.generation).min();
        if pending_generation.is_some_and(|pending| pending < ready_generation) {
            return None;
        }
        let index = state.ready.iter().position(|ready| ready.as_ref().is_some_and(|ready| ready.generation == ready_generation))?;
        state.ready[index].take().map(|ready| ready.authority)
    }

    pub fn return_ready_patch(&self, authority: SurfaceReconcileReadyPatch) -> Result<(), SurfaceReconcileReadyPatch> {
        let generation = authority.generation();
        let mut state = self.state.borrow_mut();
        let Some(target) = state.ready.iter_mut().find(|slot| slot.is_none()) else { return Err(authority) };
        *target = Some(ReadySlot { generation, authority });
        Ok(())
    }

    pub fn mark_rejected(&self, surface: &str) {
        let mut state = self.state.borrow_mut();
        let Some(index) = state.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.surface == surface)) else { return };
        let Some(target_index) = state.terminals.iter().position(Option::is_none) else { return };
        if state.slots[index].as_ref().is_some_and(|slot| slot.producer.is_some() || slot.job.is_some()) {
            let Some(slot) = state.slots[index].as_mut() else { return };
            slot.cancel.cancel_now();
            if slot.producer.is_some() {
                return;
            }
            let Some(job) = slot.job.take() else { return };
            let terminal = job.into_terminal();
            slot.reconciler = Some(SurfaceReconciler::new(slot.surface_id.clone()));
            state.terminals[target_index] = Some(TerminalSlot { instance: surface_instance(surface), authority: terminal, close: true });
            return;
        }
        let Some(generation) = next_generation(&state) else { return };
        let Some(reconciler) = state.slots[index].as_mut().and_then(|slot| slot.reconciler.take()) else { return };
        let terminal = match SurfaceReconcileTerminal::try_from_reconciler(reconciler, generation) {
            Ok(terminal) => terminal,
            Err(reconciler) => {
                if let Some(slot) = state.slots[index].as_mut() {
                    slot.reconciler = Some(reconciler);
                } else {
                    drop(reconciler);
                }
                return;
            }
        };
        commit_generation(&mut state, generation);
        let Some(slot) = state.slots[index].as_mut() else {
            drop(terminal);
            return;
        };
        slot.cancel.cancel_now();
        slot.reconciler = Some(SurfaceReconciler::new(slot.surface_id.clone()));
        state.terminals[target_index] = Some(TerminalSlot { instance: surface_instance(surface), authority: terminal, close: true });
    }

    pub fn mark_published_ack(&self, ack: SurfaceReconcilePublishedAck) -> Result<(), SurfaceReconcilePublishedAck> {
        let generation = ack.generation();
        let revision = ack.revision().0;
        let Some(surface) = ack.surface().map(|surface| surface.0.clone()) else { return Err(ack) };
        let mut state = self.state.borrow_mut();
        let Some(slot) = state.slots.iter_mut().flatten().find(|slot| slot.surface == surface && slot.generation == generation) else { return Err(ack) };
        let current = slot.reconciler.as_ref().map_or_else(
            || {
                slot.producer
                    .as_ref()
                    .and_then(|producer| producer.reconciler.as_ref())
                    .map_or_else(|| slot.job.as_ref().map_or(ui_contract::UiRevision::default(), SurfaceReconcileJob::base_revision), SurfaceReconciler::revision)
            },
            SurfaceReconciler::revision,
        );
        if revision == current.0 && revision > slot.acknowledged_revision.0 {
            slot.acknowledged_revision = ui_contract::UiRevision(revision);
            return Ok(());
        }
        Err(ack)
    }

    pub fn revision(&self, surface: &str) -> ui_contract::UiRevision {
        self.state
            .borrow()
            .slots
            .iter()
            .flatten()
            .find(|slot| slot.surface == surface)
            .map(|slot| {
                slot.reconciler.as_ref().map_or_else(
                    || {
                        slot.producer
                            .as_ref()
                            .and_then(|producer| producer.reconciler.as_ref())
                            .map_or_else(|| slot.job.as_ref().map_or(ui_contract::UiRevision::default(), SurfaceReconcileJob::base_revision), SurfaceReconciler::revision)
                    },
                    SurfaceReconciler::revision,
                )
            })
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
            if terminal_target.is_none() {
                if let Some(index) = state.terminals.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.instance == Some(closing.instance))) {
                    let terminal = state.terminals[index].as_mut().expect("matching capacity-producing terminal");
                    terminal.close = true;
                    if terminal.authority.close_step() && terminal.authority.terminal_is_empty() {
                        state.terminals[index] = None;
                    }
                    return false;
                }
            }
            if let Some(ready_index) = state.ready.iter().position(|ready| {
                ready
                    .as_ref()
                    .is_some_and(|ready| ready.authority.surface().and_then(|surface| surface_instance(&surface.0)) == Some(closing.instance))
            }) {
                let ready = state.ready[ready_index].as_mut().expect("matching ready patch");
                if ready.authority.close_step() {
                    state.ready[ready_index] = None;
                }
                return false;
            }
            if let Some(index) = state.deferred.iter().position(|entry| entry.as_ref().is_some_and(|surface| surface_instance(surface) == Some(closing.instance))) {
                state.deferred[index].take();
                return false;
            }
            if let Some(index) = state.unadmitted.iter().position(|entry| entry.as_ref().is_some_and(|entry| surface_instance(&entry.surface) == Some(closing.instance))) {
                state.unadmitted[index] = None;
                return false;
            }
            if let Some(index) = state.rejected.iter().position(|entry| entry.as_ref().is_some_and(|entry| surface_instance(&entry.surface) == Some(closing.instance))) {
                if let Some(target) = terminal_target {
                    let rejected = state.rejected[index].take().expect("matching rejected owner");
                    state.terminals[target] = Some(TerminalSlot { instance: Some(closing.instance), authority: rejected.authority.into_terminal(), close: true });
                }
                return false;
            }
            if let Some(index) = state.producer_terminals.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.instance == Some(closing.instance))) {
                let terminal = state.producer_terminals[index].as_mut().expect("matching producer terminal");
                terminal.close = true;
                if terminal.close_step() && terminal.terminal_is_empty() {
                    state.producer_terminals[index] = None;
                }
                return false;
            }
            if let Some(surface_index) = state.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| surface_instance(&slot.surface) == Some(closing.instance))) {
                if state.slots[surface_index].as_ref().is_some_and(|slot| slot.producer.is_some()) {
                    let Some(target) = state.producer_terminals.iter().position(Option::is_none) else { return false };
                    let Some(mut surface) = state.slots[surface_index].take() else { return false };
                    surface.cancel.cancel_now();
                    let Some(mut producer) = surface.producer.take() else {
                        state.slots[surface_index] = Some(surface);
                        return false;
                    };
                    if state.rejected_reserved[producer.rejected_index] == Some(surface.generation) {
                        state.rejected_reserved[producer.rejected_index] = None;
                    }
                    state.producer_terminals[target] = Some(MountedTreeTerminal {
                        instance: Some(closing.instance),
                        surface_index: None,
                        surface: surface.surface,
                        reconciler: producer.reconciler.take(),
                        reservation: producer.reservation.take(),
                        authority: Some(producer.authority),
                        close: true,
                    });
                    return false;
                }
                let Some(target) = terminal_target else { return false };
                let mut surface = state.slots[surface_index].take().expect("matching closing surface");
                surface.cancel.cancel_now();
                let terminal = if let Some(job) = surface.job.take() {
                    Some(job.into_terminal())
                } else if let Some(reconciler) = surface.reconciler.take() {
                    match SurfaceReconcileTerminal::try_from_reconciler(reconciler, surface.generation) {
                        Ok(terminal) => Some(terminal),
                        Err(reconciler) => {
                            surface.reconciler = Some(reconciler);
                            state.slots[surface_index] = Some(surface);
                            return false;
                        }
                    }
                } else {
                    state
                        .rejected
                        .iter_mut()
                        .find(|entry| entry.as_ref().is_some_and(|rejected| rejected.authority.generation() == surface.generation))
                        .and_then(Option::take)
                        .map(|rejected| rejected.authority.into_terminal())
                };
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
        if let Some(terminal) = state.producer_terminals[index].as_mut().filter(|slot| slot.close) {
            if terminal.close_step() && terminal.terminal_is_empty() {
                let Some(terminal) = state.producer_terminals[index].take() else { return false };
                if let Some(surface_index) = terminal.surface_index {
                    if let Some(surface) = state.slots[surface_index].as_mut().filter(|slot| slot.surface == terminal.surface && slot.reconciler.is_none() && slot.producer.is_none() && slot.job.is_none()) {
                        surface.reconciler = Some(SurfaceReconciler::new(surface.surface_id.clone()));
                    }
                }
            }
            return !state.terminals.iter().flatten().any(|slot| slot.close) && !state.producer_terminals.iter().flatten().any(|slot| slot.close);
        }
        let Some(terminal) = state.terminals[index].as_mut().filter(|slot| slot.close) else {
            return !state.terminals.iter().flatten().any(|slot| slot.close) && !state.producer_terminals.iter().flatten().any(|slot| slot.close);
        };
        if terminal.authority.close_step() && terminal.authority.terminal_is_empty() {
            state.terminals[index] = None;
        }
        !state.terminals.iter().flatten().any(|slot| slot.close) && !state.producer_terminals.iter().flatten().any(|slot| slot.close)
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
            && state.producer_terminals.iter().all(Option::is_none)
            && state.rejected_reserved.iter().all(Option::is_none)
            && state.unadmitted.iter().all(Option::is_none)
            && state.deferred.iter().all(Option::is_none)
            && state.ready.iter().all(Option::is_none)
            && state.closing_instances.iter().all(Option::is_none)
    }
}

fn has_work(state: &PatchTrackerState) -> bool {
    state.slots.iter().flatten().any(|slot| slot.producer.is_some() || slot.job.is_some())
        || state.terminals.iter().flatten().any(|slot| slot.close)
        || state.producer_terminals.iter().flatten().any(|slot| slot.close)
        || state.deferred.iter().any(Option::is_some)
        || state.unadmitted.iter().any(Option::is_some)
        || state.closing_instances.iter().any(Option::is_some)
        || state.ready.iter().any(Option::is_some)
}

fn next_generation(state: &PatchTrackerState) -> Option<u64> {
    if state.generation_exhausted {
        return None;
    }
    state.next_generation.checked_add(1)
}

fn commit_generation(state: &mut PatchTrackerState, generation: u64) {
    debug_assert_eq!(state.next_generation.checked_add(1), Some(generation));
    state.next_generation = generation;
    state.generation_exhausted = generation == u64::MAX;
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
            if let Some(owner) = tracker.take_ready_patch() {
                let (patch, published) = owner.publish().expect("ready owner retains its patch");
                drop(published);
                return Some(patch);
            }
            if !tracker.has_work() {
                return None;
            }
        }
        None
    }

    fn published(tracker: &PatchTracker, surface: &str) -> semio_framework_ui_runtime::SurfaceReconcilePublishedPatch {
        tracker.begin(surface.to_owned(), leaf("root", "published")).expect("admitted publication");
        for _ in 0..4_096 {
            tracker.drive_one();
            if let Some(owner) = tracker.take_ready_patch() {
                return owner.publish().expect("ready owner retains exact publication").1;
            }
        }
        panic!("publication did not become ready")
    }

    fn saturate_terminals(tracker: &PatchTracker, instance: u32, generation: u64) {
        let mut state = tracker.state.borrow_mut();
        for (index, terminal) in state.terminals.iter_mut().enumerate() {
            *terminal = Some(TerminalSlot {
                instance: Some(instance),
                authority: SurfaceReconcileTerminal::try_from_reconciler(SurfaceReconciler::new(format!("{instance}:terminal-{index}")), generation + index as u64).expect("fixed terminal admission"),
                close: true,
            });
        }
    }

    fn close_instance_to_empty(tracker: &PatchTracker, instance: u32) {
        tracker.begin_close_instance(instance);
        for _ in 0..65_536 {
            tracker.close_step();
            if tracker.terminal_is_empty() {
                return;
            }
        }
        panic!("instance {instance} did not reach terminal empty");
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
    fn published_owner_first_ack_rejects_early_stale_duplicate_wrong_instance_and_aba_without_authority_loss() {
        let tracker = PatchTracker::new();
        let published = published(&tracker, "71:ack");
        let revision = published.revision().0;
        let published = published.acknowledge("72:wrong", revision).expect_err("wrong instance is inert");
        let ack = published.acknowledge("71:ack", revision).expect("exact published owner creates ACK authority");
        {
            let mut state = tracker.state.borrow_mut();
            let slot = state.slots.iter_mut().flatten().find(|slot| slot.surface == "71:ack").expect("published surface");
            slot.generation += 1;
        }
        let ack = tracker.mark_published_ack(ack).expect_err("ABA generation is inert");
        let published = ack.into_published();
        assert!(published.matches("71:ack", revision), "ABA refusal returns the identical published authority");
        {
            let mut state = tracker.state.borrow_mut();
            let slot = state.slots.iter_mut().flatten().find(|slot| slot.surface == "71:ack").expect("published surface");
            slot.generation = published.generation();
            slot.acknowledged_revision = ui_contract::UiRevision(revision);
        }
        let ack = published.acknowledge("71:ack", revision).expect("restored published owner remains exact");
        let ack = tracker.mark_published_ack(ack).expect_err("duplicate ACK is inert");
        assert!(ack.into_published().matches("71:ack", revision), "duplicate refusal preserves exact authority");
    }

    #[test]
    fn cap_plus_one_returns_the_exact_tree_owner() {
        let tracker = PatchTracker::new();
        {
            let mut state = tracker.state.borrow_mut();
            for index in 0..SURFACE_RECONCILE_ADMISSION_SLOTS {
                let surface = format!("{index}:main");
                state.slots[index] = Some(SurfaceSlot {
                    surface_id: ui_contract::SurfaceId::try_from(surface.clone()).expect("bounded surface fixture"),
                    surface,
                    generation: index as u64 + 1,
                    operation: semio_framework_job::allocate_operation_id(),
                    preview_sequence: 0,
                    acknowledged_revision: ui_contract::UiRevision::default(),
                    cancel: semio_framework_job::root_cancel_token(),
                    reconciler: Some(SurfaceReconciler::new(format!("{index}:main"))),
                    producer: None,
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
        assert!(tracker.state.borrow().unadmitted.iter().flatten().any(|owner| owner.generation == generation));
        let tree = leaf("root", "reserved");
        grant.commit(tree);
        assert!(tracker.state.borrow().unadmitted.iter().all(Option::is_none));
        assert!(tracker.state.borrow().slots.iter().flatten().any(|slot| slot.generation == generation && slot.job.is_some()));

        for index in 0..SURFACE_RECONCILE_ADMISSION_SLOTS - 1 {
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
            if tracker.state.borrow().ready.iter().any(Option::is_some) {
                break;
            }
        }
        assert!(tracker.state.borrow().ready.iter().any(Option::is_some));
        assert!(tracker.defer("12:deferred".into()));
        tracker.retain_unadmitted("12:queued".into(), leaf("root", "queued")).expect("unadmitted");
        tracker.begin("12:active".into(), leaf("root", "active")).expect("active");
        {
            let mut state = tracker.state.borrow_mut();
            let target = state.terminals.iter_mut().find(|slot| slot.is_none()).expect("terminal capacity");
            *target = Some(TerminalSlot {
                instance: Some(12),
                authority: SurfaceReconcileTerminal::try_from_reconciler(SurfaceReconciler::new("12:terminal"), 90_012).expect("fixed terminal admission"),
                close: true,
            });
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
                *terminal = Some(TerminalSlot {
                    instance: Some(44),
                    authority: SurfaceReconcileTerminal::try_from_reconciler(SurfaceReconciler::new(format!("44:terminal-{index}")), 100_000 + index as u64).expect("fixed terminal admission"),
                    close: false,
                });
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

    #[test]
    fn terminal_full_plus_matching_unadmitted_advances_capacity_before_conversion() {
        let tracker = PatchTracker::new();
        saturate_terminals(&tracker, 51, 510_000);
        tracker.retain_unadmitted("51:queued".into(), leaf("root", "queued")).expect("pre-admitted owner");
        close_instance_to_empty(&tracker, 51);
    }

    #[test]
    fn terminal_full_plus_matching_rejected_advances_capacity_before_conversion() {
        let tracker = PatchTracker::new();
        for index in 0..=4 {
            tracker.begin(format!("52:surface-{index}"), leaf("root", "rejected")).expect("fixed surface slot");
        }
        assert!(tracker.state.borrow().rejected.iter().any(Option::is_some), "aggregate cap produces an exact rejected owner");
        saturate_terminals(&tracker, 52, 520_000);
        close_instance_to_empty(&tracker, 52);
    }

    #[test]
    fn terminal_full_plus_matching_surface_advances_capacity_before_conversion() {
        let tracker = PatchTracker::new();
        {
            let mut state = tracker.state.borrow_mut();
            state.slots[0] = Some(SurfaceSlot {
                surface: "53:idle".into(),
                surface_id: ui_contract::SurfaceId::try_from("53:idle").expect("bounded surface fixture"),
                generation: 530_000,
                operation: semio_framework_job::allocate_operation_id(),
                preview_sequence: 0,
                acknowledged_revision: ui_contract::UiRevision::default(),
                cancel: semio_framework_job::root_cancel_token(),
                reconciler: Some(SurfaceReconciler::new("53:idle")),
                producer: None,
                job: None,
            });
        }
        saturate_terminals(&tracker, 53, 531_000);
        close_instance_to_empty(&tracker, 53);
    }

    #[test]
    fn generation_max_is_issued_once_and_repeated_exhaustion_returns_exact_owners_without_mutation() {
        let tracker = PatchTracker::new();
        {
            let mut state = tracker.state.borrow_mut();
            state.next_generation = u64::MAX - 2;
        }
        assert_eq!(tracker.begin("61:first".into(), leaf("root", "first")).expect("near maximum"), u64::MAX - 1);
        assert_eq!(tracker.retain_unadmitted("61:maximum".into(), leaf("root", "maximum")).expect("maximum once"), u64::MAX);
        let refused = leaf("root", "post-maximum");
        let refused_pointer = refused.root.key.as_ptr();
        let (_, refused) = tracker.begin("61:refused".into(), refused).expect_err("first post-maximum refuses");
        assert_eq!(refused.root.key.as_ptr(), refused_pointer);
        let repeated = leaf("root", "repeated");
        let repeated_pointer = repeated.root.key.as_ptr();
        let (_, repeated) = tracker.retain_unadmitted("61:repeated".into(), repeated).expect_err("repeated refusal is stable");
        assert_eq!(repeated.root.key.as_ptr(), repeated_pointer);
        let state = tracker.state.borrow();
        assert_eq!(state.next_generation, u64::MAX);
        assert!(state.generation_exhausted);
    }

    #[test]
    fn terminal_saturation_does_not_consume_maximum_generation_before_exact_owner_reservation() {
        let tracker = PatchTracker::new();
        {
            let mut state = tracker.state.borrow_mut();
            state.next_generation = u64::MAX - 1;
            state.slots[0] = Some(SurfaceSlot {
                surface: "62:idle".into(),
                surface_id: ui_contract::SurfaceId::try_from("62:idle").expect("bounded surface fixture"),
                generation: u64::MAX - 1,
                operation: semio_framework_job::allocate_operation_id(),
                preview_sequence: 0,
                acknowledged_revision: ui_contract::UiRevision::default(),
                cancel: semio_framework_job::root_cancel_token(),
                reconciler: Some(SurfaceReconciler::new("62:idle")),
                producer: None,
                job: None,
            });
        }
        saturate_terminals(&tracker, 62, 620_000);
        tracker.mark_rejected("62:idle");
        {
            let state = tracker.state.borrow();
            assert_eq!(state.next_generation, u64::MAX - 1);
            assert!(!state.generation_exhausted);
            assert!(state.slots[0].as_ref().is_some_and(|slot| slot.reconciler.is_some()));
        }
        let terminal = tracker.state.borrow_mut().terminals[0].take().expect("free one exact terminal reservation");
        drop(terminal);
        tracker.mark_rejected("62:idle");
        let state = tracker.state.borrow();
        assert_eq!(state.next_generation, u64::MAX);
        assert!(state.generation_exhausted);
        assert!(state.terminals.iter().flatten().any(|terminal| terminal.authority.generation() == u64::MAX));
    }
}
