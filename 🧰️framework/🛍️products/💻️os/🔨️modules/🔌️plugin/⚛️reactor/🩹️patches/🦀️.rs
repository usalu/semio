//! 🩹️ Retained fixed-admission reconciliation for mounted plugin surfaces.

use semio_framework_job::{CancelToken, Generation, OperationId, StepBudget, StepContext};
use semio_framework_ui_contract as ui_contract;
#[cfg(test)]
use semio_framework_ui_runtime::ComponentTree;
use semio_framework_ui_runtime::{
    ComponentTreeProducer, ComponentTreeProducerStep, SurfaceReconcileJob, SurfaceReconcileJobStep, SurfaceReconcilePublishedAck, SurfaceReconcileReadyPatch, SurfaceReconcileRejected, SurfaceReconcileReservation, SurfaceReconcileTerminal,
    SurfaceReconciler, TreeNode, SURFACE_RECONCILE_ADMISSION_SLOTS,
};
use std::cell::RefCell;
use super::instance_lifetime::NativeCloseKey;
use semio_framework_ui_runtime::{SurfaceReconcileOutputReservation, SurfaceReconcileOutputs, SurfaceReconcileOutputTransfer};

const READY_PATCH_CAPACITY: usize = SURFACE_RECONCILE_ADMISSION_SLOTS;

struct SurfaceSlot {
    key: NativeCloseKey,
    output_index: Option<usize>,
    surface: ui_contract::SurfaceId,
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
    authority: Box<ComponentTreeProducer>,
    outcome: Option<ComponentTreeProducerStep>,
}

struct MountedTreeTerminal {
    key: NativeCloseKey,
    instance: Option<u32>,
    surface_index: Option<usize>,
    surface: ui_contract::SurfaceId,
    reconciler: Option<SurfaceReconciler>,
    reservation: Option<SurfaceReconcileReservation>,
    authority: Option<Box<ComponentTreeProducer>>,
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
    key: NativeCloseKey,
    instance: Option<u32>,
    authority: SurfaceReconcileTerminal,
    close: bool,
}

struct RejectedSlot {
    key: NativeCloseKey,
    surface: ui_contract::SurfaceId,
    authority: SurfaceReconcileRejected,
}

struct ReadySlot {
    generation: u64,
    key: NativeCloseKey,
    outputs: SurfaceReconcileOutputs,
    reservation: Option<SurfaceReconcileOutputReservation>,
    published: bool,
    closing: bool,
}

impl ReadySlot {
    fn close_step(&mut self) -> Result<bool, &'static str> {
        self.closing = true;
        if let Some(reservation) = self.reservation.as_mut() {
            if reservation.close_step(1)?.complete { self.reservation = None; }
            return Ok(false);
        }
        Ok(self.outputs.close_step(1, 4096)?.complete && self.outputs.terminal_is_empty())
    }
}

struct UnadmittedSlot {
    key: NativeCloseKey,
    generation: u64,
    surface: ui_contract::SurfaceId,
}

/// 🎟️ Exact mounted render reservation; the tree cannot exist before its fixed slot does.
pub struct MountedReconcileGrant<'a> {
    key: NativeCloseKey,
    output_index: usize,
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
        if state.closing_instances.iter().flatten().any(|closing| closing.key == self.key) { return Err(root); }
        if state.unadmitted[self.index].as_ref().is_none_or(|slot| slot.generation != self.generation) {
            return Err(root);
        }
        let Some(mut slot) = state.slots[self.surface_index].take() else { return Err(root) };
        if slot.key != self.key || slot.output_index != Some(self.output_index) || slot.reconciler.is_some() || slot.producer.is_some() || slot.job.is_some() {
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
        slot.producer = Some(MountedTreeProducer { reconciler: Some(reconciler), reservation: Some(reservation), rejected_index: self.rejected_index, authority: Box::new(producer), outcome: None });
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
                state.rejected[self.rejected_index] = Some(RejectedSlot { key: self.key, surface, authority });
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
        if let Some(output) = state.ready[self.output_index].as_mut().filter(|output| output.key == self.key && output.generation == self.generation) { output.closing = true; }
        if let Some(slot) = state.slots[self.surface_index].as_mut().filter(|slot| slot.key == self.key && slot.output_index == Some(self.output_index)) { slot.output_index = None; }
        if let Some(output) = state.ready[self.output_index].as_mut().filter(|output| output.key == self.key && output.generation == self.generation) { output.closing = true; }
        if let Some(slot) = state.slots[self.surface_index].as_mut().filter(|slot| slot.key == self.key && slot.output_index == Some(self.output_index)) { slot.output_index = None; }
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
    key: super::instance_lifetime::NativeCloseKey,
    active: bool,
    complete: bool,
}

struct PatchTrackerState {
    slots: Box<[Option<SurfaceSlot>]>,
    rejected: Box<[Option<RejectedSlot>]>,
    terminals: Box<[Option<TerminalSlot>]>,
    producer_terminals: Box<[Option<MountedTreeTerminal>]>,
    rejected_reserved: Box<[Option<u64>]>,
    deferred: Box<[Option<ui_contract::SurfaceId>]>,
    unadmitted: Box<[Option<UnadmittedSlot>]>,
    closing_instances: Box<[Option<ClosingInstance>]>,
    ready: Box<[Option<ReadySlot>]>,
    next_generation: u64,
    generation_exhausted: bool,
    drive_cursor: usize,
    close_cursor: usize,
    output_fault: Option<(NativeCloseKey, &'static str, bool)>,
}

impl Default for PatchTrackerState {
    fn default() -> Self {
        Self {
            slots: fixed_slots(SURFACE_RECONCILE_ADMISSION_SLOTS),
            rejected: fixed_slots(SURFACE_RECONCILE_ADMISSION_SLOTS),
            terminals: fixed_slots(SURFACE_RECONCILE_ADMISSION_SLOTS),
            producer_terminals: fixed_slots(SURFACE_RECONCILE_ADMISSION_SLOTS),
            rejected_reserved: fixed_slots(SURFACE_RECONCILE_ADMISSION_SLOTS),
            deferred: fixed_slots(SURFACE_RECONCILE_ADMISSION_SLOTS),
            unadmitted: fixed_slots(SURFACE_RECONCILE_ADMISSION_SLOTS + 1),
            closing_instances: fixed_slots(SURFACE_RECONCILE_ADMISSION_SLOTS),
            ready: fixed_slots(READY_PATCH_CAPACITY),
            next_generation: 0,
            generation_exhausted: false,
            drive_cursor: 0,
            close_cursor: 0,
            output_fault: None,
        }
    }
}

fn fixed_slots<T>(capacity: usize) -> Box<[Option<T>]> {
    std::iter::repeat_with(|| None).take(capacity).collect::<Vec<_>>().into_boxed_slice()
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
        if let Some(slot) = state.slots.iter().flatten().find(|slot| slot.surface.as_ref() == surface) {
            return slot.producer.is_none() && slot.job.is_none() && slot.reconciler.as_ref().is_some_and(|reconciler| slot.acknowledged_revision.0 >= reconciler.revision().0);
        }
        state.slots.iter().any(Option::is_none)
    }

    pub fn defer(&self, surface: ui_contract::SurfaceId) -> Result<(), ui_contract::SurfaceId> {
        let mut state = self.state.borrow_mut();
        if state.closing_instances.iter().flatten().any(|closing| surface_instance(surface.as_ref()) == Some(closing.instance)) {
            return Err(surface);
        }
        if state.deferred.iter().flatten().any(|queued| queued == &surface) {
            return Ok(());
        }
        let Some(slot) = state.deferred.iter_mut().find(|slot| slot.is_none()) else { return Err(surface) };
        *slot = Some(surface);
        Ok(())
    }

    pub fn take_deferred_ready(&self) -> Option<ui_contract::SurfaceId> {
        let mut state = self.state.borrow_mut();
        let index = state.deferred.iter().position(|entry| {
            entry.as_ref().is_some_and(|surface| {
                state
                    .slots
                    .iter()
                    .flatten()
                    .find(|slot| slot.surface == *surface)
                    .is_none_or(|slot| slot.producer.is_none() && slot.job.is_none() && slot.reconciler.as_ref().is_some_and(|reconciler| slot.acknowledged_revision.0 >= reconciler.revision().0))
            })
        })?;
        state.deferred[index].take()
    }

    #[cfg(test)]
    pub fn begin(&self, surface: String, tree: ComponentTree) -> Result<u64, (String, ComponentTree)> {
        let surface_id = match ui_contract::SurfaceId::try_from(surface) {
            Ok(surface) => surface,
            Err(surface) => return Err((surface, tree)),
        };
        let key = NativeCloseKey::fixture(surface_instance(surface_id.as_ref()).unwrap_or(0), 1);
        let grant = match self.reserve_mounted_owned(surface_id, key) { Ok(grant) => grant, Err(surface) => return Err((surface.0.to_string(), tree)) };
        let generation = grant.generation;
        grant.commit(tree);
        Ok(generation)
    }

    #[cfg(test)]
    pub fn retain_unadmitted(&self, surface: String, tree: ComponentTree) -> Result<u64, (String, ComponentTree)> {
        self.begin(surface, tree)
    }

    pub(crate) fn reserve_mounted(&self, surface: ui_contract::SurfaceId, key: NativeCloseKey) -> Result<MountedReconcileGrant<'_>, ui_contract::SurfaceId> {
        if surface_instance(surface.as_ref()) != Some(key.instance()) { return Err(surface); }
        self.reserve_mounted_owned(surface, key)
    }

    fn reserve_mounted_owned(&self, surface: ui_contract::SurfaceId, key: NativeCloseKey) -> Result<MountedReconcileGrant<'_>, ui_contract::SurfaceId> {
        let mut state = self.state.borrow_mut();
        if state.closing_instances.iter().flatten().any(|closing| surface_instance(surface.as_ref()) == Some(closing.instance)) {
            return Err(surface);
        }
        let Some(index) = state.unadmitted.iter().position(Option::is_none) else { return Err(surface) };
        let Some(surface_index) = state.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.surface == surface)).or_else(|| state.slots.iter().position(Option::is_none)) else { return Err(surface) };
        if state.slots[surface_index].as_ref().is_some_and(|slot| slot.key != key || slot.output_index.is_some() || slot.producer.is_some() || slot.job.is_some() || slot.reconciler.is_none()) {
            return Err(surface);
        }
        let Some(generation) = next_generation(&state) else { return Err(surface) };
        let Some(rejected_index) = state.rejected.iter().enumerate().find_map(|(index, slot)| (slot.is_none() && state.rejected_reserved[index].is_none()).then_some(index)) else {
            return Err(surface);
        };
        let Some(reservation) = SurfaceReconcileReservation::try_new(generation) else { return Err(surface) };
        let Some(output_index) = state.ready.iter().position(Option::is_none) else { return Err(surface) };
        let mut outputs = SurfaceReconcileOutputs::default();
        let output_reservation = match outputs.try_reserve(generation, semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES) {
            Ok(Some(owner)) => owner,
            Ok(None) => return Err(surface),
            Err(fault) => { state.output_fault = Some((key, fault, false)); return Err(surface); }
        };
        state.ready[output_index] = Some(ReadySlot { generation, key, outputs, reservation: Some(output_reservation), published: false, closing: false });
        let reconciler = if let Some(slot) = state.slots[surface_index].as_mut() {
            slot.output_index = Some(output_index);
            slot.reconciler.take().expect("preflight retained current root")
        } else {
            let reconciler = SurfaceReconciler::new(surface.clone());
            state.slots[surface_index] = Some(SurfaceSlot {
                key,
                output_index: Some(output_index),
                reconciler: None,
                surface: surface.clone(),
                generation,
                operation: semio_framework_job::allocate_operation_id(),
                preview_sequence: 0,
                acknowledged_revision: ui_contract::UiRevision::default(),
                cancel: semio_framework_job::root_cancel_token(),
                producer: None,
                job: None,
            });
            reconciler
        };
        commit_generation(&mut state, generation);
        state.rejected_reserved[rejected_index] = Some(generation);
        state.unadmitted[index] = Some(UnadmittedSlot { key, generation, surface });
        drop(state);
        Ok(MountedReconcileGrant { key, output_index, tracker: self, index, surface_index, rejected_index, generation, owner: MountedReconcileOwner::Live { reconciler, reservation }, active: true })
    }

    pub fn drive_one(&self) -> bool {
        let mut state = self.state.borrow_mut();
        if state.output_fault.is_some() { return true; }
        let Some(index) =
            (0..SURFACE_RECONCILE_ADMISSION_SLOTS).map(|offset| (state.drive_cursor + offset) % SURFACE_RECONCILE_ADMISSION_SLOTS).find(|index| state.slots[*index].as_ref().is_some_and(|slot| !state.closing_instances.iter().flatten().any(|closing| closing.key == slot.key) && (slot.producer.is_some() || slot.job.is_some())))
        else {
            return has_work(&state);
        };
        state.drive_cursor = (index + 1) % SURFACE_RECONCILE_ADMISSION_SLOTS;
        if state.slots[index].as_ref().is_some_and(|slot| slot.job.is_some()) {
            drive_job_one(&mut state, index);
            return has_work(&state);
        }
        let outcome = state.slots[index].as_ref().and_then(|slot| slot.producer.as_ref()).and_then(|producer| producer.outcome);
        let Some(outcome) = outcome else {
            let slot = state.slots[index].as_mut().expect("selected structural producer slot");
            let Some(producer) = slot.producer.as_mut() else { return has_work(&state) };
            let mut preview_sequence = slot.preview_sequence;
            let mut context = StepContext::new(slot.operation, Generation(slot.generation), StepBudget::new(1, u64::MAX), slot.cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            let outcome = producer.authority.step(slot.generation, context.is_cancelled(), context.deadline_exceeded());
            #[cfg(test)]
            tests::after_producer_step();
            context.consume_fuel(1);
            slot.preview_sequence = preview_sequence;
            if outcome != ComponentTreeProducerStep::MoreWork { producer.outcome = Some(outcome); }
            return has_work(&state);
        };
        let Some(mut slot) = state.slots[index].take() else { return has_work(&state) };
        if let Some(mut producer) = slot.producer.take() {
            match outcome {
                ComponentTreeProducerStep::MoreWork => slot.producer = Some(producer),
                ComponentTreeProducerStep::Complete => {
                    let sources = if producer.reconciler.is_some() && producer.reservation.is_some() && producer.authority.has_complete() {
                        Some((producer.reconciler.take().expect("preflight current root"), producer.reservation.take().expect("preflight producer reservation"), producer.authority.take_complete().expect("preflight completed tree")))
                    } else {
                        None
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
                                state.rejected[producer.rejected_index] = Some(RejectedSlot { key: slot.key, surface: slot.surface.clone(), authority });
                            }
                        }
                    } else if let Some(target) = state.producer_terminals.iter().position(Option::is_none) {
                        state.rejected_reserved[producer.rejected_index] = None;
                        state.producer_terminals[target] = Some(MountedTreeTerminal {
                            key: slot.key,
                            instance: surface_instance(slot.surface.as_ref()),
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
                            key: slot.key,
                            instance: surface_instance(slot.surface.as_ref()),
                            surface_index: Some(index),
                            surface: slot.surface.clone(),
                            reconciler: producer.reconciler.take(),
                            reservation: producer.reservation.take(),
                            authority: Some(producer.authority),
                            close: false,
                        });
                    } else {
                        slot.producer = Some(producer);
                    }
                }
            }
            if slot.producer.is_none() && slot.job.is_none() && state.rejected.iter().flatten().all(|rejected| rejected.authority.generation() != slot.generation) {
                if let Some(output) = slot.output_index.take().and_then(|index| state.ready[index].as_mut()) { output.closing = true; }
            }
            state.slots[index] = Some(slot);
            return has_work(&state);
        }
        state.slots[index] = Some(slot);
        has_work(&state)
    }

    pub fn has_work(&self) -> bool {
        has_work(&self.state.borrow())
    }

    /// 🚨️ Returns one mounted surface failure while retaining its incremental cleanup owner.
    pub fn take_render_fault(&self) -> Option<(u32, String)> {
        let mut state = self.state.borrow_mut();
        if let Some((key, fault, reported)) = state.output_fault.as_mut() { if !*reported { *reported = true; return Some((key.instance(), (*fault).to_owned())); } }
        for terminal in state.producer_terminals.iter_mut().flatten().filter(|terminal| !terminal.close) {
            if let Some(fault) = terminal.authority.as_ref().and_then(|authority| authority.fault()) {
                terminal.close = true;
                return Some((terminal.instance.unwrap_or(0), format!("{}: {fault:?}", terminal.surface.as_ref())));
            }
        }
        let terminal = state.terminals.iter_mut().flatten().find(|terminal| !terminal.close && terminal.authority.fault().is_some())?;
        let fault = terminal.authority.fault().cloned()?;
        let generation = terminal.authority.generation();
        let instance = terminal.instance.unwrap_or(0);
        terminal.close = true;
        let surface = state.slots.iter().flatten().find(|slot| slot.generation == generation).map(|slot| slot.surface.as_ref()).unwrap_or("unknown surface");
        Some((instance, format!("{surface}: {fault:?}")))
    }

    pub(crate) fn ready_patch_key(&self) -> Result<Option<(NativeCloseKey, u64)>, &'static str> {
        let state = self.state.try_borrow().map_err(|_| "patch publication target is busy")?;
        Ok(next_ready_index(&state).map(|index| { let ready = state.ready[index].as_ref().expect("selected output"); (ready.key, ready.generation) }))
    }

    pub(crate) fn take_ready_patch_into(&self, key: NativeCloseKey, generation: u64, target: &mut Option<SurfaceReconcileReadyPatch>, admitted_bytes: usize) -> Result<bool, &'static str> {
        if target.is_some() { return Ok(false); }
        let metadata = std::mem::size_of::<ReadySlot>();
        let Some(bytes) = admitted_bytes.checked_sub(metadata) else { return Ok(false) };
        let mut state = self.state.try_borrow_mut().map_err(|_| "patch publication target is busy")?;
        let Some(index) = next_ready_index(&state) else { return Ok(false) };
        let output = state.ready[index].as_mut().expect("selected retained output");
        if output.key != key || output.generation != generation { return Err("patch publication belongs to another allocation or generation"); }
        if !output.outputs.take_front_into(target, bytes)? { return Ok(false); }
        output.published = false;
        output.closing = true;
        Ok(true)
    }

    #[cfg(test)]
    pub fn take_ready_patch(&self) -> Option<SurfaceReconcileReadyPatch> {
        let (key, generation) = self.ready_patch_key().unwrap()?;
        let mut target = None;
        self.take_ready_patch_into(key, generation, &mut target, semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES).unwrap();
        target
    }

    pub fn mark_rejected(&self, surface: &str) {
        let mut state = self.state.borrow_mut();
        let Some(index) = state.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.surface.as_ref() == surface)) else { return };
        let Some(target_index) = state.terminals.iter().position(Option::is_none) else { return };
        if state.slots[index].as_ref().is_some_and(|slot| slot.producer.is_some() || slot.job.is_some()) {
            let Some(slot) = state.slots[index].as_mut() else { return };
            slot.cancel.cancel_now();
            if slot.producer.is_some() {
                return;
            }
            let Some(job) = slot.job.take() else { return };
            let terminal = job.into_terminal();
            if slot.reconciler.is_none() { slot.reconciler = Some(SurfaceReconciler::new(slot.surface.clone())); }
            state.terminals[target_index] = Some(TerminalSlot { key: slot.key, instance: surface_instance(surface), authority: terminal, close: true });
            close_output(&mut state, index);
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
        slot.reconciler = Some(SurfaceReconciler::new(slot.surface.clone()));
        state.terminals[target_index] = Some(TerminalSlot { key: slot.key, instance: surface_instance(surface), authority: terminal, close: true });
    }

    pub fn mark_published_ack(&self, ack: &SurfaceReconcilePublishedAck) -> Result<bool, &'static str> {
        let generation = ack.generation();
        let revision = ack.revision().0;
        let Some(surface) = ack.surface().map(|surface| surface.0.as_str()) else { return Ok(false) };
        let mut state = self.state.try_borrow_mut().map_err(|_| "published ACK target is busy")?;
        let Some(slot) = state.slots.iter_mut().flatten().find(|slot| slot.surface.0.as_str() == surface && slot.generation == generation) else { return Ok(false) };
        let current = slot.reconciler.as_ref().map_or_else(
            || slot.producer.as_ref().and_then(|producer| producer.reconciler.as_ref()).map_or_else(|| slot.job.as_ref().map_or(ui_contract::UiRevision::default(), SurfaceReconcileJob::base_revision), SurfaceReconciler::revision),
            SurfaceReconciler::revision,
        );
        if revision == current.0 && revision > slot.acknowledged_revision.0 {
            slot.acknowledged_revision = ui_contract::UiRevision(revision);
            return Ok(true);
        }
        Ok(false)
    }

    pub fn revision(&self, surface: &str) -> ui_contract::UiRevision {
        self.state
            .borrow()
            .slots
            .iter()
            .flatten()
            .find(|slot| slot.surface.as_ref() == surface)
            .map(|slot| {
                slot.reconciler.as_ref().map_or_else(
                    || slot.producer.as_ref().and_then(|producer| producer.reconciler.as_ref()).map_or_else(|| slot.job.as_ref().map_or(ui_contract::UiRevision::default(), SurfaceReconcileJob::base_revision), SurfaceReconciler::revision),
                    SurfaceReconciler::revision,
                )
            })
            .unwrap_or_default()
    }

    pub(crate) fn reserve_close_instance(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
        let mut state = self.state.try_borrow_mut().map_err(|_| "patch close reservation is busy")?;
        if let Some(closing) = state.closing_instances.iter().flatten().find(|closing| closing.instance == key.instance()) {
            return if closing.key == key { Ok(()) } else { Err("patch close reservation belongs to another allocation") };
        }
        if state.slots.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.ready.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.rejected.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.terminals.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.producer_terminals.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.unadmitted.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key) {
            return Err("patch descendants belong to another allocation");
        }
        let slot = state.closing_instances.iter_mut().find(|slot| slot.is_none()).ok_or("patch close reservation is full")?;
        *slot = Some(ClosingInstance { instance: key.instance(), key, active: false, complete: false });
        Ok(())
    }

    pub(crate) fn activate_close_instance(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
        let mut state = self.state.try_borrow_mut().map_err(|_| "patch close reservation is busy")?;
        let closing = state.closing_instances.iter_mut().flatten().find(|closing| closing.key == key).ok_or("exact patch close reservation missing")?;
        closing.active = true;
        Ok(())
    }

    pub(crate) fn close_instance_complete(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<bool, &'static str> {
        let state = self.state.try_borrow().map_err(|_| "patch close receipt is busy")?;
        state.closing_instances.iter().flatten().find(|closing| closing.key == key).map(|closing| closing.complete).ok_or("exact patch close receipt missing")
    }

    pub(crate) fn release_close_instance(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
        let mut state = self.state.try_borrow_mut().map_err(|_| "patch close receipt is busy")?;
        let closing = state.closing_instances.iter_mut().find(|closing| closing.is_some_and(|closing| closing.key == key && closing.complete)).ok_or("exact patch close receipt is not terminal")?;
        *closing = None;
        Ok(())
    }

    pub fn close_step(&self) -> bool {
        let Ok(mut state) = self.state.try_borrow_mut() else { return false };
        if state.output_fault.is_some() { return false; }
        if let Some(index) = state.ready.iter().position(|output| output.as_ref().is_some_and(|output| output.closing)) {
            match state.ready[index].as_mut().expect("retained output close").close_step() {
                Ok(true) => state.ready[index] = None,
                Ok(false) => {},
                Err(fault) => state.output_fault = Some((state.ready[index].as_ref().expect("faulted retained output").key, fault, false)),
            }
            return false;
        }
        if let Some(closing_index) = state.closing_instances.iter().position(|closing| closing.is_some_and(|closing| closing.active && !closing.complete)) {
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
            if let Some(ready_index) = state.ready.iter().position(|ready| ready.as_ref().is_some_and(|ready| ready.key == closing.key)) {
                let ready = state.ready[ready_index].as_mut().expect("matching ready patch");
                ready.closing = true;
                return false;
            }
            if let Some(index) = state.deferred.iter().position(|entry| entry.as_ref().is_some_and(|surface| surface_instance(surface.as_ref()) == Some(closing.instance))) {
                state.deferred[index].take();
                return false;
            }
            if state.unadmitted.iter().flatten().any(|entry| entry.key == closing.key) {
                return false;
            }
            if let Some(index) = state.rejected.iter().position(|entry| entry.as_ref().is_some_and(|entry| surface_instance(entry.surface.as_ref()) == Some(closing.instance))) {
                if let Some(target) = terminal_target {
                    let rejected = state.rejected[index].take().expect("matching rejected owner");
                    state.terminals[target] = Some(TerminalSlot { key: rejected.key, instance: Some(closing.instance), authority: rejected.authority.into_terminal(), close: true });
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
            if let Some(surface_index) = state.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.key == closing.key)) {
                if state.slots[surface_index].as_ref().is_some_and(|slot| slot.job.is_some() && slot.reconciler.is_some()) {
                    let Some(target) = terminal_target else { return false };
                    let PatchTrackerState { slots, terminals, .. } = &mut *state;
                    let surface = slots[surface_index].as_mut().expect("transferred job and canonical root retained");
                    terminals[target] = Some(TerminalSlot { key: surface.key, instance: Some(closing.instance), authority: surface.job.take().expect("retained transferred job shell").into_terminal(), close: true });
                    surface.output_index = None;
                    return false;
                }
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
                        key: surface.key,
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
                    state.rejected.iter_mut().find(|entry| entry.as_ref().is_some_and(|rejected| rejected.authority.generation() == surface.generation)).and_then(Option::take).map(|rejected| rejected.authority.into_terminal())
                };
                if let Some(terminal) = terminal {
                    state.terminals[target] = Some(TerminalSlot { key: surface.key, instance: Some(closing.instance), authority: terminal, close: true });
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
            state.closing_instances[closing_index].as_mut().expect("exact retained close receipt").complete = true;
            return false;
        }
        let index = state.close_cursor;
        state.close_cursor = (state.close_cursor + 1) % SURFACE_RECONCILE_ADMISSION_SLOTS;
        if let Some(terminal) = state.producer_terminals[index].as_mut().filter(|slot| slot.close) {
            if terminal.close_step() && terminal.terminal_is_empty() {
                let Some(terminal) = state.producer_terminals[index].take() else { return false };
                if let Some(surface_index) = terminal.surface_index {
                    if let Some(surface) = state.slots[surface_index].as_mut().filter(|slot| slot.surface == terminal.surface && slot.reconciler.is_none() && slot.producer.is_none() && slot.job.is_none()) {
                        surface.reconciler = Some(SurfaceReconciler::new(surface.surface.clone()));
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
            && state.output_fault.is_none()
    }
}

fn next_ready_index(state: &PatchTrackerState) -> Option<usize> {
    let (index, ready) = state.ready.iter().enumerate().filter_map(|(index, ready)| ready.as_ref().filter(|ready| ready.published && !ready.closing && !state.closing_instances.iter().flatten().any(|closing| closing.key == ready.key)).map(|ready| (index, ready))).min_by_key(|(_, ready)| ready.generation)?;
    let pending = state.slots.iter().flatten().filter(|slot| slot.producer.is_some() || slot.job.is_some()).map(|slot| slot.generation).min();
    if pending.is_some_and(|generation| generation < ready.generation) { return None; }
    Some(index)
}

fn close_output(state: &mut PatchTrackerState, index: usize) {
    if let Some(output) = state.slots[index].as_mut().and_then(|slot| slot.output_index.take()).and_then(|output| state.ready[output].as_mut()) { output.closing = true; }
}

fn drive_job_one(state: &mut PatchTrackerState, index: usize) {
    let Some(slot) = state.slots[index].as_ref() else { return };
    let Some(job) = slot.job.as_ref() else { return };
    if slot.reconciler.is_some() && !job.is_ready() {
        let Some(target) = state.terminals.iter().position(Option::is_none) else { return };
        let slot = state.slots[index].as_mut().expect("retained transferred job");
        state.terminals[target] = Some(TerminalSlot { key: slot.key, instance: surface_instance(slot.surface.as_ref()), authority: slot.job.take().expect("retained empty job shell").into_terminal(), close: true });
        slot.output_index = None;
        return;
    }
    let Some(output_index) = slot.output_index else { return };
    let Some(_) = state.ready[output_index].as_ref().filter(|output| output.key == slot.key && output.generation == slot.generation) else { return };
    if job.is_ready() {
        let slot = state.slots[index].as_mut().expect("retained ready job slot");
        let mut context = StepContext::new(slot.operation, Generation(slot.generation), StepBudget::new(1, u64::MAX), slot.cancel.clone(), semio_framework_job::default_now_us, &mut slot.preview_sequence);
        if slot.job.as_mut().expect("retained ready job authority").drive_one(&mut context) != SurfaceReconcileJobStep::Ready { return; }
        let receiver_bytes = std::mem::size_of::<ReadySlot>();
        let Some(grant) = semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES.checked_sub(receiver_bytes) else { return };
        let output = state.ready[output_index].as_mut().expect("output reserved before producer");
        let result = output.outputs.receive_job_into(&mut output.reservation, slot.job.as_mut().expect("structural job receiver"), &mut slot.reconciler, grant);
        match result {
            Ok(SurfaceReconcileOutputTransfer::Published) => output.published = true,
            Ok(SurfaceReconcileOutputTransfer::Empty) => {
                slot.acknowledged_revision = slot.reconciler.as_ref().expect("transferred canonical root").revision();
                output.closing = true;
            }
            Ok(SurfaceReconcileOutputTransfer::Pending) => return,
            Err(fault) => { state.output_fault = Some((slot.key, fault, false)); return; }
        }
        #[cfg(test)]
        tests::after_output_transfer();
        return;
    }
    let slot = state.slots[index].as_mut().expect("retained job slot");
    let mut context = StepContext::new(slot.operation, Generation(slot.generation), StepBudget::new(1, u64::MAX), slot.cancel.clone(), semio_framework_job::default_now_us, &mut slot.preview_sequence);
    let outcome = slot.job.as_mut().expect("retained job authority").drive_one(&mut context);
    if outcome == SurfaceReconcileJobStep::Fault {
        if let Some(target) = state.terminals.iter_mut().find(|slot| slot.is_none()) {
            *target = Some(TerminalSlot { key: slot.key, instance: surface_instance(slot.surface.as_ref()), authority: slot.job.take().expect("faulted job remains retained").into_terminal(), close: false });
        }
    }
}

fn has_work(state: &PatchTrackerState) -> bool {
    state.output_fault.is_some() || state.slots.iter().flatten().any(|slot| slot.producer.is_some() || slot.job.is_some())
        || state.terminals.iter().any(Option::is_some)
        || state.producer_terminals.iter().any(Option::is_some)
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

    thread_local! { static PANIC_AFTER_OUTPUT_TRANSFER: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
    thread_local! { static PANIC_AFTER_PRODUCER_STEP: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }

    pub(super) fn after_output_transfer() {
        PANIC_AFTER_OUTPUT_TRANSFER.with(|pending| { if pending.replace(false) { panic!("[DEBUG] actual mounted direct-output transfer unwind"); } });
    }

    pub(super) fn after_producer_step() {
        if PANIC_AFTER_PRODUCER_STEP.with(|pending| pending.replace(false)) { panic!("[DEBUG] actual mounted producer partial-step unwind"); }
    }

    fn reserve<'a>(tracker: &'a PatchTracker, surface: ui_contract::SurfaceId) -> Result<MountedReconcileGrant<'a>, ui_contract::SurfaceId> {
        let key = NativeCloseKey::fixture(surface_instance(surface.as_ref()).expect("numeric test surface"), 1);
        tracker.reserve_mounted(surface, key)
    }

    fn leaf(key: &str, text: &str) -> ComponentTree {
        let value = ui_contract::UiText::try_from_str(text).expect("bounded test text");
        let root = TreeNode::try_new(key, ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label(value), emphasize: None, data_attributes: None })).unwrap_or_else(|_| panic!("bounded test tree"));
        ComponentTree { root }
    }

    fn tree_with_owned_child(text: &str) -> ComponentTree {
        let mut tree = leaf("root", text);
        tree.root.children.try_push(leaf("owned-child", text).root).expect("bounded owned child");
        tree
    }

    fn publish_test(mut ready: SurfaceReconcileReadyPatch) -> (ui_contract::UiPatch, semio_framework_ui_runtime::SurfaceReconcilePublishedPatch) {
        let mut payload = ui_contract::UiPendingPatch::default();
        let mut published = None;
        assert!(ready.publish_into(&mut payload, &mut published, SurfaceReconcileReadyPatch::required_publish_bytes()).unwrap() > 0);
        assert!(ready.terminal_is_empty());
        (payload.source_mut().unwrap().take().unwrap(), published.unwrap())
    }

    fn close_published(mut published: semio_framework_ui_runtime::SurfaceReconcilePublishedPatch) {
        for turn in 0..65_536 {
            let step = published.close_step_with_grant(1, 4096).unwrap();
            assert!(step.released_items <= 1 && step.released_bytes <= 4096);
            if step.complete && published.terminal_is_empty() { return; }
            assert!(turn < 65_535);
        }
    }

    fn close_ack(mut ack: SurfaceReconcilePublishedAck) {
        for turn in 0..65_536 {
            let step = ack.close_step_with_grant(1, 4096).unwrap();
            assert!(step.released_items <= 1 && step.released_bytes <= 4096);
            if step.complete && ack.terminal_is_empty() { return; }
            assert!(turn < 65_535);
        }
    }

    fn close_test_patch(patch: ui_contract::UiPatch) {
        let mut owner = ui_contract::UiPendingPatch::default();
        *owner.source_mut().unwrap() = Some(patch);
        for turn in 0..65_536 {
            owner.close_step(1, 4096).unwrap();
            if owner.terminal_is_empty() { return; }
            assert!(turn < 65_535);
        }
    }

    fn finish(tracker: &PatchTracker) -> Option<ui_contract::UiPatch> {
        for _ in 0..4_096 {
            tracker.drive_one();
            if let Some(owner) = tracker.take_ready_patch() {
                let (patch, published) = publish_test(owner);
                close_published(published);
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
                let (patch, published) = publish_test(owner);
                close_test_patch(patch);
                return published;
            }
        }
        panic!("publication did not become ready")
    }

    fn saturate_terminals(tracker: &PatchTracker, instance: u32, generation: u64) {
        let mut state = tracker.state.borrow_mut();
        for (index, terminal) in state.terminals.iter_mut().enumerate() {
            *terminal = Some(TerminalSlot {
                key: NativeCloseKey::fixture(instance, 1),
                instance: Some(instance),
                authority: SurfaceReconcileTerminal::try_from_reconciler(SurfaceReconciler::new(ui_contract::SurfaceId::try_from(format!("{instance}:terminal-{index}")).expect("bounded surface fixture")), generation + index as u64)
                    .expect("fixed terminal admission"),
                close: true,
            });
        }
    }

    fn close_instance_to_empty(tracker: &PatchTracker, instance: u32) {
        let key = super::super::instance_lifetime::NativeCloseKey::fixture(instance, 1);
        tracker.reserve_close_instance(key).expect("exact close reservation");
        tracker.activate_close_instance(key).expect("activate retained close");
        for _ in 0..65_536 {
            tracker.close_step();
            if tracker.close_instance_complete(key).expect("exact close receipt") {
                tracker.release_close_instance(key).expect("final ACK releases close slot");
                assert!(tracker.terminal_is_empty());
                return;
            }
        }
        panic!("instance {instance} did not reach terminal empty");
    }

    #[test]
    fn mounted_output_admission_cancel_and_drop_keep_the_original_close_generation() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🖱️ui/🧠️runtime/📤️output/🧫️fixture/🔣️.json")).unwrap();
        let fixture = &fixture["cancelledAdmission"];
        for drop_grant in [false, true] {
            let tracker = PatchTracker::new();
            let surface = ui_contract::SurfaceId::try_from(fixture["surface"].as_str().unwrap()).unwrap();
            let grant = reserve(&tracker, surface.clone()).unwrap();
            let generation = grant.generation;
            if drop_grant { drop(grant); } else { grant.cancel(); }
            let preserved = tracker.state.borrow().slots.iter().flatten().find(|slot| slot.surface == surface).is_some_and(|slot| slot.generation == generation && generation != 0);
            let revision = tracker.revision(surface.as_ref()).0;
            close_instance_to_empty(&tracker, 74);
            assert_eq!(preserved, fixture["generationPreserved"].as_bool().unwrap());
            assert_eq!(revision, fixture["revision"].as_u64().unwrap());
            assert_eq!(tracker.terminal_is_empty(), fixture["terminal"].as_bool().unwrap());
            eprintln!("[DEBUG] mounted-uncommitted-close drop={drop_grant} generation={generation} preserved={preserved} revision={revision} terminal=true");
        }
    }

    #[test]
    fn mounted_output_admission_refuses_before_tree_when_shared_output_pool_is_full() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🖱️ui/🧠️runtime/📤️output/🧫️fixture/🔣️.json")).unwrap();
        let mut outputs = semio_framework_ui_runtime::SurfaceReconcileOutputs::default();
        let mut reservations = Vec::new();
        for generation in 1..=fixture["entrySlots"].as_u64().unwrap() {
            reservations.push(outputs.try_reserve(generation, fixture["physicalGrant"].as_u64().unwrap() as usize).unwrap().unwrap());
        }
        let tracker = PatchTracker::new();
        let admitted = match reserve(&tracker, ui_contract::SurfaceId::try_from("71:output-admission").unwrap()) {
            Ok(grant) => { grant.cancel(); true }
            Err(surface) => { assert_eq!(surface.as_ref(), "71:output-admission"); false }
        };
        for owner in &mut reservations { while !owner.close_step(1).unwrap().complete {} }
        while !outputs.close_step(1, 4096).unwrap().complete {}
        close_instance_to_empty(&tracker, 71);
        assert_eq!(admitted, fixture["extraInvocation"].as_bool().unwrap());
        eprintln!("[DEBUG] mounted-output-admission accepted={admitted} tree-constructed=false shared-entries=64");
    }

    #[test]
    fn mounted_output_admission_partial_producer_step_unwind_retains_original_slot_and_box() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🖱️ui/🧠️runtime/📤️output/🧫️fixture/🔣️.json")).unwrap();
        let tracker = PatchTracker::new();
        reserve(&tracker, ui_contract::SurfaceId::try_from("72:producer-unwind").unwrap()).unwrap().commit_source(leaf("root", "owned-é").root).unwrap();
        let (index, rejected_index, generation, pointer) = {
            let state = tracker.state.borrow();
            let index = state.slots.iter().position(Option::is_some).unwrap();
            let slot = state.slots[index].as_ref().unwrap();
            let producer = slot.producer.as_ref().unwrap();
            (index, producer.rejected_index, slot.generation, producer.authority.as_ref() as *const _)
        };
        PANIC_AFTER_PRODUCER_STEP.with(|pending| pending.set(true));
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tracker.drive_one()));
        assert!(caught.is_err());
        let retained = tracker.state.borrow().slots[index].as_ref().and_then(|slot| slot.producer.as_ref()).is_some_and(|producer| producer.authority.as_ref() as *const _ == pointer && producer.reservation.as_ref().is_some_and(|owner| owner.generation() == generation));
        if !retained {
            let mut state = tracker.state.borrow_mut();
            assert_eq!(state.rejected_reserved[rejected_index], Some(generation));
            state.rejected_reserved[rejected_index] = None;
        }
        close_instance_to_empty(&tracker, 72);
        assert_eq!(retained, fixture["partialProducerUnwindRetainsOwner"].as_bool().unwrap(), "actual producer step must not remove the structural slot before invoking the child");
        eprintln!("[DEBUG] mounted-producer-unwind exact-slot-and-box-retained={retained}");
    }

    #[test]
    fn mounted_output_admission_incomplete_producer_sources_preserve_remaining_owners() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🖱️ui/🧠️runtime/📤️output/🧫️fixture/🔣️.json")).unwrap();
        let tracker = PatchTracker::new();
        reserve(&tracker, ui_contract::SurfaceId::try_from("73:producer-source").unwrap()).unwrap().commit_source(leaf("owned-root", "é").root).unwrap();
        let (index, generation, pointer, reconciler) = {
            let mut state = tracker.state.borrow_mut();
            let index = state.slots.iter().position(Option::is_some).unwrap();
            let slot = state.slots[index].as_mut().unwrap();
            let producer = slot.producer.as_mut().unwrap();
            for turn in 0..64 {
                if producer.authority.step(slot.generation, false, false) == ComponentTreeProducerStep::Complete { break; }
                assert!(turn < 63);
            }
            (index, slot.generation, producer.authority.as_ref() as *const _, producer.reconciler.take().unwrap())
        };
        tracker.drive_one();
        tracker.drive_one();
        let (reservation_retained, tree_retained) = {
            let mut state = tracker.state.borrow_mut();
            let result = if let Some(producer) = state.slots[index].as_mut().and_then(|slot| slot.producer.as_mut()) {
                assert_eq!(producer.authority.as_ref() as *const _, pointer);
                (producer.reservation.as_ref().is_some_and(|owner| owner.generation() == generation), producer.authority.take_complete().is_some_and(|tree| tree.root.key.as_str() == "owned-root"))
            } else if let Some(terminal) = state.producer_terminals.iter_mut().flatten().find(|terminal| terminal.authority.as_ref().is_some_and(|owner| owner.as_ref() as *const _ == pointer)) {
                (terminal.reservation.as_ref().is_some_and(|owner| owner.generation() == generation), terminal.authority.as_mut().unwrap().take_complete().is_some_and(|tree| tree.root.key.as_str() == "owned-root"))
            } else { (false, false) };
            state.slots[index].as_mut().unwrap().reconciler = Some(reconciler);
            result
        };
        close_instance_to_empty(&tracker, 73);
        assert_eq!(reservation_retained && tree_retained, fixture["incompleteProducerSourcesPreserveRemainingOwners"].as_bool().unwrap(), "one missing source must not consume other completed owners");
        eprintln!("[DEBUG] mounted-producer-incomplete reservation-retained={reservation_retained} tree-retained={tree_retained}");
    }

    #[test]
    fn mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🖱️ui/🧠️runtime/📤️output/🧫️fixture/🔣️.json")).unwrap();
        let law = &fixture["capturedLifetime"];
        let instance = law["instance"].as_u64().unwrap() as u32;
        let key = NativeCloseKey::fixture(instance, law["original"].as_u64().unwrap());
        let foreign = NativeCloseKey::fixture(instance, law["reused"].as_u64().unwrap());
        let tracker = PatchTracker::new();
        let grant = tracker.reserve_mounted(ui_contract::SurfaceId::try_from(format!("{instance}:direct")).unwrap(), key).unwrap();
        let generation = grant.generation;
        grant.commit_source(leaf("owned-root", "界-é").root).unwrap();
        PANIC_AFTER_OUTPUT_TRANSFER.with(|pending| pending.set(true));
        let mut caught = false;
        for _ in 0..65_536 {
            if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tracker.drive_one())).is_err() { caught = true; break; }
        }
        assert!(caught, "actual live job-to-pool transfer callback ran");
        let exact_roots = {
            let state = tracker.state.borrow();
            let surface = state.slots.iter().flatten().find(|slot| slot.key == key).unwrap();
            let output = state.ready[surface.output_index.unwrap()].as_ref().unwrap();
            surface.job.is_some() && surface.reconciler.is_some() && output.published && output.reservation.is_none() && output.key == key && output.generation == generation
        };
        assert!(exact_roots);
        assert_eq!(tracker.reserve_close_instance(foreign).is_ok(), law["foreignCloseAccepted"].as_bool().unwrap());
        let mut target = None;
        assert!(tracker.take_ready_patch_into(foreign, generation, &mut target, 32768).is_err());
        assert!(tracker.take_ready_patch_into(key, generation + 1, &mut target, 32768).is_err());
        assert!(!tracker.take_ready_patch_into(key, generation, &mut target, 0).unwrap());
        let guard = tracker.state.borrow_mut();
        assert!(tracker.take_ready_patch_into(key, generation, &mut target, 32768).is_err());
        drop(guard);
        assert!(target.is_none());
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            assert!(tracker.take_ready_patch_into(key, generation, &mut target, 32768).unwrap());
            panic!("[DEBUG] actual Pending receiver callback retains exact Ready");
        }));
        assert!(caught.is_err());
        assert_eq!(target.as_ref().unwrap().generation(), generation);
        assert_eq!(target.as_ref().unwrap().surface().unwrap().as_ref(), format!("{instance}:direct"));
        while !target.as_mut().unwrap().close_step_with_grant(1, 4096).unwrap().complete {}
        close_instance_to_empty(&tracker, instance);
        assert_eq!(tracker.terminal_is_empty(), law["terminal"].as_bool().unwrap());
        eprintln!("[DEBUG] live-output exact-lifetime=true admission-generation={generation} producer-callback-roots={exact_roots} occupied-busy-zero-refusal=true pending-callback-retained=true terminal=true");
    }

    #[test]
    fn mounted_output_admission_close_waits_for_the_original_uncommitted_grant() {
        let tracker = PatchTracker::new();
        let key = NativeCloseKey::fixture(76, 1);
        let grant = tracker.reserve_mounted(ui_contract::SurfaceId::try_from("76:grant").unwrap(), key).unwrap();
        tracker.reserve_close_instance(key).unwrap();
        tracker.activate_close_instance(key).unwrap();
        for _ in 0..8 { tracker.close_step(); assert!(!tracker.close_instance_complete(key).unwrap()); }
        let root = tree_with_owned_child("returned");
        let pointer = root.root.children.get(0).unwrap().key.as_ptr();
        let returned = grant.commit_source(root.root).expect_err("closing lifetime rejects producer invocation");
        assert_eq!(returned.children.get(0).unwrap().key.as_ptr(), pointer);
        close_instance_to_empty(&tracker, 76);
        eprintln!("[DEBUG] live-output close-waits-for-original-grant=true rejected-tree-pointer-preserved=true");
    }

    #[test]
    fn mounted_output_admission_concurrent_trackers_share_one_fixed_pool_without_overadmission() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🖱️ui/🧠️runtime/📤️output/🧫️fixture/🔣️.json")).unwrap();
        let law = &fixture["concurrentAdmission"];
        let mut occupied = SurfaceReconcileOutputs::default();
        let mut reservations = Vec::new();
        for generation in 1..=law["occupied"].as_u64().unwrap() { reservations.push(occupied.try_reserve(generation, 32768).unwrap().unwrap()); }
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(3));
        let (tx, rx) = std::sync::mpsc::channel();
        let mut workers = Vec::new();
        for instance in [81, 82] {
            let barrier = barrier.clone();
            let tx = tx.clone();
            workers.push(std::thread::spawn(move || {
                let tracker = PatchTracker::new();
                let mut surface = ui_contract::SurfaceId::try_from(format!("{instance}:concurrent")).unwrap();
                let key = NativeCloseKey::fixture(instance, 1);
                let mut admitted = None;
                barrier.wait();
                for _ in 0..64 {
                    match tracker.reserve_mounted(surface, key) {
                        Ok(grant) => { admitted = Some(grant); break; }
                        Err(returned) => { assert_eq!(returned.as_ref(), format!("{instance}:concurrent")); surface = returned; std::thread::yield_now(); }
                    }
                }
                tx.send(admitted.is_some()).unwrap();
                barrier.wait();
                if let Some(grant) = admitted { grant.cancel(); }
                close_instance_to_empty(&tracker, instance);
            }));
        }
        barrier.wait();
        let accepted = usize::from(rx.recv().unwrap()) + usize::from(rx.recv().unwrap());
        barrier.wait();
        for worker in workers { worker.join().unwrap(); }
        for reservation in &mut reservations { while !reservation.close_step(1).unwrap().complete {} }
        while !occupied.close_step(1, 4096).unwrap().complete {}
        assert_eq!(accepted, law["accepted"].as_u64().unwrap() as usize);
        let mut reused = SurfaceReconcileOutputs::default();
        let mut restored = Vec::new();
        for generation in 1..=64 { restored.push(reused.try_reserve(generation, 32768).unwrap().unwrap()); }
        assert!(reused.try_reserve(65, 32768).unwrap().is_none());
        for reservation in &mut restored { while !reservation.close_step(1).unwrap().complete {} }
        while !reused.close_step(1, 4096).unwrap().complete {}
        eprintln!("[DEBUG] live-output same-process-workers=2 preoccupied=63 accepted={accepted} exact-refusal=true full64-restored=true");
    }

    #[test]
    fn tracker_initialization_fits_the_component_stack_budget() {
        let bytes = std::mem::size_of::<PatchTrackerState>();
        assert!(bytes <= 256, "PatchTrackerState requires {bytes} bytes");
        let state = PatchTrackerState::default();
        assert_eq!(state.slots.len(), SURFACE_RECONCILE_ADMISSION_SLOTS);
        assert_eq!(state.unadmitted.len(), SURFACE_RECONCILE_ADMISSION_SLOTS + 1);
        assert_eq!(state.ready.len(), READY_PATCH_CAPACITY);
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
    fn mounted_document_tree_publishes_nested_interactive_rows() {
        use ui_contract::{Buildable, HasBase, HasChildren};
        fn row(value: &serde_json::Value) -> ui_contract::BuiltNode {
            let id = value["id"].as_str().unwrap();
            let mut builder = ui_contract::tree_item(ui_contract::Label(ui_contract::UiText::try_from_str(id).unwrap())).try_id(id).ok().unwrap();
            let action = |name: &str| serde_json::from_value(serde_json::json!({ "scope": "fixture", "name": name, "version": 1 })).unwrap();
            let args = || serde_json::from_value(serde_json::json!({ "domainId": "fixture", "merge": "replace", "method": "pick", "targets": id })).unwrap();
            builder = builder.try_on_with(ui_contract::Trigger::Activate, action("select"), args()).ok().unwrap();
            for name in value["rowActions"].as_array().into_iter().flatten() {
                let name = name.as_str().unwrap();
                builder = builder
                    .try_row_action(ui_contract::RowAction {
                        icon: ui_contract::UiText::try_from_str(name).unwrap(),
                        label: Some(ui_contract::Label(ui_contract::UiText::try_from_str(name).unwrap())),
                        action: ui_contract::ActionBinding { trigger: ui_contract::Trigger::Activate, action: action(name), args: Some(args()), capability: None },
                        placement: ui_contract::RowActionPlacement::Row,
                    })
                    .ok()
                    .unwrap();
            }
            builder.try_children(value["children"].as_array().into_iter().flatten().map(row)).ok().unwrap().try_build().unwrap()
        }
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/📃️document-surface.json")).unwrap();
        assert_eq!(ui_contract::UI_NODE_BINDINGS as u64, fixture["limits"]["nodeBindings"].as_u64().unwrap());
        assert_eq!(semio_framework_ui_runtime::SurfaceReconcileLimits::default().max_bytes as u64, fixture["limits"]["surfaceBytes"].as_u64().unwrap());
        assert_eq!(semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES as u64, fixture["limits"]["pageBytes"].as_u64().unwrap());
        assert_eq!(semio_framework_ui_runtime::SURFACE_RECONCILE_AGGREGATE_BYTES as u64, fixture["limits"]["aggregateBytes"].as_u64().unwrap());
        let sections = fixture["sections"].as_array().unwrap().iter().map(|section| {
            ui_contract::tree_section(ui_contract::Label(ui_contract::UiText::try_from_str(section["id"].as_str().unwrap()).unwrap()))
                .try_id(section["id"].as_str().unwrap())
                .ok()
                .unwrap()
                .try_children(section["rows"].as_array().unwrap().iter().map(row))
                .ok()
                .unwrap()
                .try_build()
                .unwrap()
        });
        let root = ui_contract::tree().try_id("document").ok().unwrap().try_children(sections).ok().unwrap().try_build().unwrap();
        let tracker = PatchTracker::new();
        reserve(&tracker, ui_contract::SurfaceId::try_from(fixture["surface"].as_str().unwrap()).unwrap()).unwrap().commit_source(root).unwrap();
        let patch = finish(&tracker).unwrap_or_else(|| panic!("document never published: {:?}", tracker.state.borrow().terminals.iter().flatten().map(|slot| slot.authority.fault()).collect::<Vec<_>>()));
        let nodes = patch.ops.iter().filter_map(|op| if let ui_contract::UiPatchOp::Upsert(node) = op { Some(node) } else { None }).collect::<Vec<_>>();
        assert_eq!(nodes.len(), fixture["nodes"].as_u64().unwrap() as usize);
        assert_eq!(nodes.iter().filter(|node| !node.bindings.is_empty()).count(), fixture["interactiveRows"].as_u64().unwrap() as usize);
    }

    #[test]
    fn mounted_settings_controls_publish_with_authored_fields() {
        use ui_contract::{Buildable, HasBase, HasChildren};
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🎚️settings-surface.json")).expect("language-neutral settings");
        let fields = fixture["fields"].as_array().unwrap();
        let children = fields.iter().map(|field| {
            let id = field["id"].as_str().unwrap();
            let action = serde_json::from_value(serde_json::json!({ "scope": "fixture", "name": field["action"], "version": 1 })).unwrap();
            let mut control =
                ui_contract::BuiltNode::try_new(format!("{id}.control"), ui_contract::Component::NumberStepper(ui_contract::NumberStepperProps { value: field["value"].as_f64().unwrap(), step: field["step"].as_f64().unwrap(), uniform: false }))
                    .ok()
                    .unwrap();
            control.bindings.try_push(ui_contract::ActionBinding { trigger: ui_contract::Trigger::Change, action, args: None, capability: None }).ok().unwrap();
            ui_contract::field(ui_contract::Label(ui_contract::UiText::try_from_str(field["label"].as_str().unwrap()).unwrap())).try_id(id).ok().unwrap().try_child(control).ok().unwrap().try_build().unwrap()
        });
        let root = ui_contract::section(ui_contract::Label(ui_contract::UiText::try_from_str(fixture["label"].as_str().unwrap()).unwrap())).try_id("settings").ok().unwrap().default_open(true).try_children(children).ok().unwrap().try_build().unwrap();
        let tracker = PatchTracker::new();
        reserve(&tracker, ui_contract::SurfaceId::try_from(fixture["surface"].as_str().unwrap()).unwrap()).unwrap().commit_source(root).unwrap();
        let patch = finish(&tracker).unwrap_or_else(|| panic!("settings never published: {:?}", tracker.state.borrow().slots.iter().flatten().map(|slot| &slot.job).collect::<Vec<_>>()));
        let controls = patch
            .ops
            .iter()
            .filter_map(|op| match op {
                ui_contract::UiPatchOp::Upsert(node) => match &node.component {
                    ui_contract::Component::NumberStepper(props) => Some(serde_json::json!({ "value": props.value, "step": props.step, "action": node.bindings[0].action.name.as_str() })),
                    _ => None,
                },
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(controls, fields.iter().map(|field| serde_json::json!({ "value": field["value"], "step": field["step"], "action": field["action"] })).collect::<Vec<_>>());
    }

    #[test]
    fn mounted_catalogue_publishes_every_section_beyond_thirty_two_nodes() {
        use ui_contract::{Buildable, HasBase, HasChildren};
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🗂️catalogue-surface.json")).unwrap();
        let rows = fixture["rowsPerSection"].as_u64().unwrap();
        let mut expected = std::collections::BTreeSet::new();
        let sections = fixture["sections"].as_array().unwrap().iter().map(|section| {
            let section = section.as_str().unwrap();
            let children = (0..rows).map(|row| {
                let key = format!("{section}.{row}");
                expected.insert(key.clone());
                leaf(&key, &key).root
            });
            ui_contract::tree_section(ui_contract::Label(ui_contract::UiText::try_from_str(section).unwrap())).try_id(section).ok().unwrap().try_children(children).ok().unwrap().try_build().unwrap()
        });
        let root = ui_contract::tree().try_id("catalogue").ok().unwrap().try_children(sections).ok().unwrap().try_build().unwrap();
        assert_eq!(expected.len() as u64, fixture["expectedRows"].as_u64().unwrap());
        let tracker = PatchTracker::new();
        reserve(&tracker, ui_contract::SurfaceId::try_from(fixture["surface"].as_str().unwrap()).unwrap()).unwrap().commit_source(root).unwrap();
        let mut published = None;
        for _ in 0..65_536 {
            tracker.drive_one();
            if let Some(owner) = tracker.take_ready_patch() {
                let (patch, authority) = publish_test(owner);
                let mut authority = Some(authority);
                let mut acknowledgement = None;
                assert!(semio_framework_ui_runtime::SurfaceReconcilePublishedPatch::acknowledge_into(&mut authority, &mut acknowledgement, fixture["surface"].as_str().unwrap(), patch.revision.0, semio_framework_ui_runtime::SurfaceReconcilePublishedPatch::required_acknowledge_bytes()).unwrap());
                assert!(tracker.mark_published_ack(acknowledgement.as_ref().unwrap()).unwrap());
                close_ack(acknowledgement.take().unwrap());
                published = Some(patch);
                break;
            }
            if !tracker.has_work() { break; }
        }
        let fault = format!("{:?}", tracker.state.borrow().terminals.iter().flatten().map(|slot| slot.authority.fault()).collect::<Vec<_>>());
        close_instance_to_empty(&tracker, 1);
        let patch = published.unwrap_or_else(|| panic!("catalogue did not publish: {fault}"));
        let nodes = patch.ops.iter().filter_map(|op| if let ui_contract::UiPatchOp::Upsert(node) = op { Some(node) } else { None }).collect::<Vec<_>>();
        assert_eq!(nodes.len() as u64, fixture["expectedNodes"].as_u64().unwrap());
        assert_eq!(nodes.iter().filter(|node| matches!(node.component, ui_contract::Component::Text(_))).map(|node| node.key.to_string()).collect::<std::collections::BTreeSet<_>>(), expected);
    }

    #[test]
    fn mounted_catalogue_reports_producer_failure_once_before_cleanup() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🗂️catalogue-surface.json")).unwrap();
        let failure = &fixture["failure"];
        let key = failure["key"].as_str().unwrap();
        let mut root = leaf("catalogue", "Catalogue").root;
        root.children.try_push(leaf(key, key).root).unwrap();
        root.children.try_push(leaf(key, key).root).unwrap();
        let tracker = PatchTracker::new();
        reserve(&tracker, ui_contract::SurfaceId::try_from(failure["surface"].as_str().unwrap()).unwrap()).unwrap().commit_source(root).unwrap();
        let mut reported = None;
        for _ in 0..4096 {
            tracker.drive_one();
            if let Some(fault) = tracker.take_render_fault() { reported = Some(fault); break; }
        }
        assert!(tracker.take_ready_patch().is_none());
        assert!(tracker.take_render_fault().is_none());
        close_instance_to_empty(&tracker, failure["instance"].as_u64().unwrap() as u32);
        let (instance, reason) = reported.expect("producer fault must not disappear during cleanup");
        assert_eq!(instance as u64, failure["instance"].as_u64().unwrap());
        assert!(reason.contains(failure["surface"].as_str().unwrap()));
        assert!(reason.contains(failure["reason"].as_str().unwrap()));
    }

    #[test]
    fn mounted_catalogue_reports_reconcile_capacity_without_leaking_owners() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🗂️catalogue-surface.json")).unwrap();
        let failure = &fixture["capacityFailure"];
        assert_eq!(ui_contract::UI_DOCUMENT_NODES as u64, failure["nodeLimit"].as_u64().unwrap());
        let mut root = leaf("catalogue", "Catalogue").root;
        for section in 0..failure["sections"].as_u64().unwrap() {
            let mut child = leaf(&section.to_string(), "Section").root;
            for row in 0..failure["rowsPerSection"].as_u64().unwrap() {
                child.children.try_push(leaf(&row.to_string(), "Row").root).unwrap();
            }
            root.children.try_push(child).unwrap();
        }
        let tracker = PatchTracker::new();
        reserve(&tracker, ui_contract::SurfaceId::try_from(failure["surface"].as_str().unwrap()).unwrap()).unwrap().commit_source(root).unwrap();
        let mut reported = None;
        for _ in 0..65_536 {
            tracker.drive_one();
            if let Some(fault) = tracker.take_render_fault() { reported = Some(fault); break; }
        }
        assert!(tracker.take_ready_patch().is_none());
        assert!(tracker.take_render_fault().is_none());
        close_instance_to_empty(&tracker, failure["instance"].as_u64().unwrap() as u32);
        let (instance, reason) = reported.expect("reconcile failure must not become an idle empty surface");
        assert_eq!(instance as u64, failure["instance"].as_u64().unwrap());
        assert!(reason.contains(failure["surface"].as_str().unwrap()));
        assert!(reason.contains(failure["reason"].as_str().unwrap()));
    }

    #[test]
    fn mounted_sources_publish_every_window_and_panel_tree() {
        use ui_contract::{Buildable, HasBase, HasChildren};
        let fixtures: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🪟️mounted-surfaces.json")).expect("language-neutral surface fixtures");
        let fixtures = fixtures.as_array().expect("surface list");
        let tracker = PatchTracker::new();
        for fixture in fixtures {
            let surface = fixture["surface"].as_str().expect("surface key");
            let children = fixture["children"].as_array().expect("child keys").iter().map(|key| {
                let mut node = leaf(key.as_str().expect("child key"), "content").root;
                let action = serde_json::from_value(fixture["action"].clone()).expect("language-neutral action");
                node.bindings.try_push(ui_contract::ActionBinding { trigger: ui_contract::Trigger::Activate, action, args: None, capability: None }).expect("bounded binding");
                node
            });
            let root = ui_contract::column().try_id(surface).ok().expect("bounded key").try_children(children).ok().expect("bounded children").try_build().expect("bounded root");
            reserve(&tracker, ui_contract::SurfaceId::try_from(surface).expect("bounded surface")).expect("mounted reservation").commit_source(root).expect("mounted producer");
            let patch = finish(&tracker).unwrap_or_else(|| {
                let state = tracker.state.borrow();
                panic!(
                    "surface {} never published: {:?}; terminals={}, rejected={}",
                    fixture["surface"],
                    state.slots.iter().flatten().map(|slot| (&slot.surface, slot.producer.as_ref().map(|producer| producer.authority.fault()), &slot.job, slot.reconciler.is_some())).collect::<Vec<_>>(),
                    state.terminals.iter().flatten().count(),
                    state.rejected.iter().flatten().count()
                );
            });
            assert_eq!(patch.surface.0.as_str(), fixture["surface"].as_str().unwrap());
            assert_eq!(patch.ops.iter().filter(|op| matches!(op, ui_contract::UiPatchOp::Upsert(..))).count(), 1 + fixture["children"].as_array().unwrap().len());
            for op in &patch.ops {
                if let ui_contract::UiPatchOp::Upsert(record) = op {
                    if record.key.as_str() != surface {
                        assert_eq!(serde_json::to_value(&record.bindings[0].action).expect("action wire oracle"), fixture["action"]);
                    }
                }
            }
        }
    }

    #[test]
    fn one_active_surface_does_not_wait_behind_sixty_three_empty_slots_between_steps() {
        let steps = [0, SURFACE_RECONCILE_ADMISSION_SLOTS - 1].map(|index| {
            let tracker = PatchTracker::new();
            tracker.begin("main".into(), leaf("root", "a")).expect("admitted");
            tracker.state.borrow_mut().slots.swap(0, index);
            for step in 1..=4_096 {
                tracker.drive_one();
                if let Some(owner) = tracker.take_ready_patch() {
                    let (patch, published) = publish_test(owner);
                    close_test_patch(patch);
                    close_published(published);
                    return step;
                }
            }
            panic!("active surface never published");
        });
        assert_eq!(steps[0], steps[1], "empty slots consume no reconcile opportunities");
    }

    #[test]
    fn published_owner_first_ack_rejects_early_stale_duplicate_wrong_instance_and_aba_without_authority_loss() {
        let tracker = PatchTracker::new();
        let mut published = Some(published(&tracker, "71:ack"));
        let revision = published.as_ref().unwrap().revision().0;
        let mut ack = None;
        let admitted = semio_framework_ui_runtime::SurfaceReconcilePublishedPatch::required_acknowledge_bytes();
        assert!(!semio_framework_ui_runtime::SurfaceReconcilePublishedPatch::acknowledge_into(&mut published, &mut ack, "72:wrong", revision, admitted).unwrap());
        assert!(published.as_ref().unwrap().matches("71:ack", revision));
        assert!(semio_framework_ui_runtime::SurfaceReconcilePublishedPatch::acknowledge_into(&mut published, &mut ack, "71:ack", revision, admitted).unwrap());
        {
            let mut state = tracker.state.borrow_mut();
            let slot = state.slots.iter_mut().flatten().find(|slot| slot.surface.as_ref() == "71:ack").expect("published surface");
            slot.generation += 1;
        }
        assert!(!tracker.mark_published_ack(ack.as_ref().unwrap()).unwrap(), "ABA generation is inert");
        assert_eq!(ack.as_ref().unwrap().surface().unwrap().0.as_str(), "71:ack", "ABA refusal preserves the identical structural authority");
        {
            let mut state = tracker.state.borrow_mut();
            let slot = state.slots.iter_mut().flatten().find(|slot| slot.surface.as_ref() == "71:ack").expect("published surface");
            slot.generation = ack.as_ref().unwrap().generation();
            slot.acknowledged_revision = ui_contract::UiRevision(revision);
        }
        assert!(!tracker.mark_published_ack(ack.as_ref().unwrap()).unwrap(), "duplicate ACK is inert");
        assert_eq!(ack.as_ref().unwrap().surface().unwrap().0.as_str(), "71:ack", "duplicate refusal preserves exact authority");
        close_ack(ack.take().unwrap());
    }

    #[test]
    fn cap_plus_one_returns_the_exact_tree_owner() {
        let tracker = PatchTracker::new();
        {
            let mut state = tracker.state.borrow_mut();
            for index in 0..SURFACE_RECONCILE_ADMISSION_SLOTS {
                let surface = format!("{index}:main");
                state.slots[index] = Some(SurfaceSlot {
                    key: NativeCloseKey::fixture(index as u32, 1),
                    output_index: None,
                    surface: ui_contract::SurfaceId::try_from(surface).expect("bounded surface fixture"),
                    generation: index as u64 + 1,
                    operation: semio_framework_job::allocate_operation_id(),
                    preview_sequence: 0,
                    acknowledged_revision: ui_contract::UiRevision::default(),
                    cancel: semio_framework_job::root_cancel_token(),
                    reconciler: Some(SurfaceReconciler::new(ui_contract::SurfaceId::try_from(format!("{index}:main")).expect("bounded surface fixture"))),
                    producer: None,
                    job: None,
                });
            }
        }
        let tree = tree_with_owned_child("exact");
        let pointer = tree.root.children.get(0).unwrap().key.as_ptr();
        let (_, returned) = tracker.begin("65:main".into(), tree).expect_err("cap + 1");
        assert_eq!(returned.root.children.get(0).unwrap().key.as_ptr(), pointer);
    }

    #[test]
    fn mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🖱️ui/🧠️runtime/📤️output/🧫️fixture/🔣️.json")).unwrap();
        let law = &fixture["residentCapacity"];
        let aggregate = semio_framework_ui_runtime::SURFACE_RECONCILE_AGGREGATE_BYTES;
        let limits = semio_framework_ui_runtime::SurfaceReconcileLimits::default();
        assert_eq!(aggregate, fixture["aggregateCeilingBytes"].as_u64().unwrap() as usize);
        assert_eq!(limits.max_bytes, law["reservationBytes"].as_u64().unwrap() as usize);
        for row in law["cases"].as_array().unwrap() {
            let available = aggregate.checked_sub(usize::try_from(row["fixedBytes"].as_u64().unwrap()).unwrap()).unwrap();
            assert_eq!(available / limits.max_bytes, usize::try_from(row["capacity"].as_u64().unwrap()).unwrap());
        }
        let tracker = PatchTracker::new();
        let grant = reserve(&tracker, ui_contract::SurfaceId::try_from("4:mounted").expect("bounded surface")).expect("fixed mounted reservation");
        let generation = grant.generation;
        assert!(tracker.state.borrow().unadmitted.iter().flatten().any(|owner| owner.generation == generation));
        let tree = leaf("root", "reserved");
        grant.commit(tree);
        assert!(tracker.state.borrow().unadmitted.iter().all(Option::is_none));
        assert!(tracker.state.borrow().slots.iter().flatten().any(|slot| slot.generation == generation && slot.job.is_some()));

        let fixed_bytes = ui_contract::UiResidentPermit::fixed_backing_bytes().unwrap();
        let capacity = aggregate.checked_sub(fixed_bytes).unwrap() / limits.max_bytes;
        assert!(capacity > 0 && capacity < SURFACE_RECONCILE_ADMISSION_SLOTS);
        let first = ui_contract::UiResidentPermit::snapshot().unwrap();
        assert_eq!(first, ui_contract::UiResidentSnapshot { bytes: fixed_bytes + limits.max_bytes, items: limits.max_items, used_slots: 1 });
        for index in 0..capacity - 1 {
            tracker.retain_unadmitted(format!("{index}:queued"), leaf("root", "queued")).expect("fixed unadmitted slot");
            let admitted = index + 2;
            assert_eq!(ui_contract::UiResidentPermit::snapshot().unwrap(), ui_contract::UiResidentSnapshot {
                bytes: fixed_bytes.checked_add(admitted.checked_mul(limits.max_bytes).unwrap()).unwrap(),
                items: admitted.checked_mul(limits.max_items).unwrap(),
                used_slots: admitted,
            });
        }
        let full = ui_contract::UiResidentPermit::snapshot().unwrap();
        assert!(full.bytes <= aggregate && full.bytes.checked_add(limits.max_bytes).unwrap() > aggregate);
        let overflow = tree_with_owned_child("overflow");
        let overflow_pointer = overflow.root.children.get(0).unwrap().key.as_ptr();
        let (_, returned) = tracker.retain_unadmitted("66:queued".into(), overflow).expect_err("cap + 1 returns the exact tree");
        assert_eq!(returned.root.children.get(0).unwrap().key.as_ptr() == overflow_pointer, law["refusalPreservesTree"].as_bool().unwrap());
        assert!(reserve(&tracker, ui_contract::SurfaceId::try_from("67:mounted").expect("bounded surface")).is_err(), "render cannot materialize before a fixed slot exists");
        assert_eq!(ui_contract::UiResidentPermit::snapshot().unwrap(), full);
        let keys: Vec<_> = std::iter::once(4).chain((0..capacity - 1).map(|index| u32::try_from(index).unwrap())).map(|instance| NativeCloseKey::fixture(instance, 1)).collect();
        for key in &keys {
            tracker.reserve_close_instance(*key).unwrap();
            tracker.activate_close_instance(*key).unwrap();
        }
        for _ in 0..65_536 {
            tracker.close_step();
            if keys.iter().all(|key| tracker.close_instance_complete(*key).unwrap()) { break; }
        }
        for key in &keys {
            assert!(tracker.close_instance_complete(*key).unwrap());
            tracker.release_close_instance(*key).unwrap();
        }
        assert_eq!(tracker.terminal_is_empty(), law["terminal"].as_bool().unwrap());
        assert_eq!(ui_contract::UiResidentPermit::snapshot().unwrap(), ui_contract::UiResidentSnapshot { bytes: fixed_bytes, items: 0, used_slots: 0 });
        eprintln!("[DEBUG] mounted-resident-capacity fixed={fixed_bytes} per={} accepted={capacity} full={} cap-plus-one=false exact-refusal=true restored={fixed_bytes}", limits.max_bytes, full.bytes);
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
            assert!(tracker.defer(ui_contract::SurfaceId::try_from("7:main").expect("bounded surface")).is_ok());
        }
        assert!(tracker.take_deferred_ready().is_none());
        tracker.mark_rejected("7:main");
        let mut terminal = tracker.take_terminal(generation).expect("cancelled owner");
        for _ in 0..32 {
            if terminal.close_step() && terminal.terminal_is_empty() {
                break;
            }
        }
        assert_eq!(tracker.take_deferred_ready().as_ref().map(AsRef::as_ref), Some("7:main"));
        assert!(tracker.take_deferred_ready().is_none());
    }

    #[test]
    fn effects_publish_in_admission_order_even_when_later_tree_finishes_first() {
        let tracker = PatchTracker::new();
        tracker.begin("1:first".into(), leaf("root", "a")).expect("first");
        tracker.begin("1:second".into(), leaf("root", "b")).expect("second");
        let first = finish(&tracker).expect("first ready");
        let second = finish(&tracker).expect("second ready");
        assert_eq!(first.surface.0.as_str(), "1:first");
        assert_eq!(second.surface.0.as_str(), "1:second");
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
        close_instance_to_empty(&tracker, 9);
        assert!(tracker.terminal_is_empty());
    }

    #[test]
    fn close_retires_ready_deferred_unadmitted_active_and_terminal_owners_without_stale_publish() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🚪️surface-close.json")).unwrap();
        let instance = fixture["instance"].as_u64().unwrap() as u32;
        let surface = |key: &str| fixture["surfaces"][key].as_str().unwrap();
        let tracker = PatchTracker::new();
        tracker.begin(surface("ready").into(), leaf("root", "ready")).expect("ready source");
        for _ in 0..128 {
            tracker.drive_one();
            if tracker.state.borrow().ready.iter().any(Option::is_some) {
                break;
            }
        }
        assert!(tracker.state.borrow().ready.iter().any(Option::is_some));
        assert!(tracker.defer(ui_contract::SurfaceId::try_from(surface("deferred")).expect("bounded surface")).is_ok());
        tracker.retain_unadmitted(surface("queued").into(), leaf("root", "queued")).expect("unadmitted");
        tracker.begin(surface("active").into(), leaf("root", "active")).expect("active");
        {
            let mut state = tracker.state.borrow_mut();
            let target = state.terminals.iter_mut().find(|slot| slot.is_none()).expect("terminal capacity");
            let surface = ui_contract::SurfaceId::try_from(surface("terminal")).expect("bounded surface fixture");
            *target = Some(TerminalSlot { key: NativeCloseKey::fixture(instance, 1), instance: Some(instance), authority: SurfaceReconcileTerminal::try_from_reconciler(SurfaceReconciler::new(surface), 90_012).expect("fixed terminal admission"), close: true });
        }
        let key = super::super::instance_lifetime::NativeCloseKey::fixture(instance, 1);
        tracker.reserve_close_instance(key).expect("exact close reservation");
        tracker.activate_close_instance(key).expect("activate retained close");
        assert_eq!(tracker.take_ready_patch().is_some(), fixture["stalePatch"].as_bool().unwrap());
        for _ in 0..16_384 {
            tracker.close_step();
            if tracker.close_instance_complete(key).expect("exact close receipt") {
                break;
            }
        }
        assert!(!tracker.terminal_is_empty(), "completed close receipt remains owned until exact ACK");
        tracker.release_close_instance(key).expect("final ACK releases close slot");
        assert_eq!(tracker.terminal_is_empty(), fixture["terminalEmpty"].as_bool().unwrap());
        assert_eq!(tracker.take_ready_patch().is_some(), fixture["stalePatch"].as_bool().unwrap());
    }

    #[test]
    fn terminal_saturation_keeps_fault_job_in_its_surface_until_one_slot_is_freed() {
        let tracker = PatchTracker::new();
        {
            let mut state = tracker.state.borrow_mut();
            for (index, terminal) in state.terminals.iter_mut().enumerate() {
                *terminal = Some(TerminalSlot {
                    key: NativeCloseKey::fixture(44, 1),
                    instance: Some(44),
                    authority: SurfaceReconcileTerminal::try_from_reconciler(SurfaceReconciler::new(ui_contract::SurfaceId::try_from(format!("44:terminal-{index}")).expect("bounded surface fixture")), 100_000 + index as u64)
                        .expect("fixed terminal admission"),
                    close: false,
                });
            }
        }
        let generation = tracker.begin("44:active".into(), leaf("root", "active")).expect("active");
        tracker.mark_rejected("44:active");
        assert!(tracker.state.borrow().slots.iter().flatten().find(|slot| slot.surface.as_ref() == "44:active").is_some_and(|slot| slot.job.is_some()), "saturation retains the exact job locally");
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
        let identifier_bytes = semio_framework_ui_runtime::SurfaceReconcileLimits::default().max_identifier_bytes;
        tracker.begin(format!("52:{}", "x".repeat(identifier_bytes)), leaf("root", "rejected")).expect("fixed surface slot");
        assert!(tracker.state.borrow().rejected.iter().any(Option::is_some), "identifier cap produces an exact rejected owner");
        saturate_terminals(&tracker, 52, 520_000);
        close_instance_to_empty(&tracker, 52);
    }

    #[test]
    fn terminal_full_plus_matching_surface_advances_capacity_before_conversion() {
        let tracker = PatchTracker::new();
        {
            let mut state = tracker.state.borrow_mut();
            state.slots[0] = Some(SurfaceSlot {
                key: NativeCloseKey::fixture(53, 1),
                output_index: None,
                surface: ui_contract::SurfaceId::try_from("53:idle").expect("bounded surface fixture"),
                generation: 530_000,
                operation: semio_framework_job::allocate_operation_id(),
                preview_sequence: 0,
                acknowledged_revision: ui_contract::UiRevision::default(),
                cancel: semio_framework_job::root_cancel_token(),
                reconciler: Some(SurfaceReconciler::new(ui_contract::SurfaceId::try_from("53:idle").expect("bounded surface fixture"))),
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
        let refused = tree_with_owned_child("post-maximum");
        let refused_pointer = refused.root.children.get(0).unwrap().key.as_ptr();
        let (_, refused) = tracker.begin("61:refused".into(), refused).expect_err("first post-maximum refuses");
        assert_eq!(refused.root.children.get(0).unwrap().key.as_ptr(), refused_pointer);
        let repeated = tree_with_owned_child("repeated");
        let repeated_pointer = repeated.root.children.get(0).unwrap().key.as_ptr();
        let (_, repeated) = tracker.retain_unadmitted("61:repeated".into(), repeated).expect_err("repeated refusal is stable");
        assert_eq!(repeated.root.children.get(0).unwrap().key.as_ptr(), repeated_pointer);
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
                key: NativeCloseKey::fixture(62, 1),
                output_index: None,
                surface: ui_contract::SurfaceId::try_from("62:idle").expect("bounded surface fixture"),
                generation: u64::MAX - 1,
                operation: semio_framework_job::allocate_operation_id(),
                preview_sequence: 0,
                acknowledged_revision: ui_contract::UiRevision::default(),
                cancel: semio_framework_job::root_cancel_token(),
                reconciler: Some(SurfaceReconciler::new(ui_contract::SurfaceId::try_from("62:idle").expect("bounded surface fixture"))),
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
