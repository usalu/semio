//! 🩹️ Retained fixed-admission reconciliation for mounted plugin surfaces.

use super::instance_lifetime::NativeCloseKey;
use semio_framework_job::{CancelToken, Generation, OperationId, StepBudget, StepContext};
use semio_framework_ui_contract as ui_contract;
#[cfg(test)]
use semio_framework_ui_runtime::ComponentTree;
use semio_framework_ui_runtime::{
    ComponentTreeProducer, ComponentTreeProducerStep, SurfaceReconcileJob, SurfaceReconcileJobStep, SurfaceReconcilePublishedAck, SurfaceReconcileReadyPatch, SurfaceReconcileRejected, SurfaceReconcileReservation, SurfaceReconcileTerminal,
    SurfaceReconciler, TreeNode, SURFACE_RECONCILE_ADMISSION_SLOTS,
};
use semio_framework_ui_runtime::{SurfaceReconcileOutputReservation, SurfaceReconcileOutputTransfer, SurfaceReconcileOutputs};
use std::cell::RefCell;

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
    /// 🧹️ Retires this mounted tree terminal against the caller's ITEM grant. Its only owners are the
    /// tree producer's node pages and two single handles — nothing on this ladder is byte-priced, so
    /// it takes the grant's `items` and not its `bytes`; the producer spends the whole grant in one
    /// call and the two handles cost one item each (ticket 26/09/02, W-B2).
    fn close_step(&mut self, items: usize) -> bool {
        if let Some(authority) = self.authority.as_mut() {
            if !authority.close_step_with_grant(items) {
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
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<bool, &'static str> {
        self.closing = true;
        if let Some(reservation) = self.reservation.as_mut() {
            if reservation.close_step(items)?.complete {
                self.reservation = None;
            }
            return Ok(false);
        }
        Ok(self.outputs.close_step(items, bytes)?.complete && self.outputs.terminal_is_empty())
    }
}

struct UnadmittedSlot {
    key: NativeCloseKey,
    generation: u64,
}

/// 🎟️ Exact mounted render reservation; the tree cannot exist before its fixed slot does.
pub struct MountedReconcileGrant {
    key: NativeCloseKey,
    output_index: usize,
    state: std::rc::Rc<RefCell<PatchTrackerState>>,
    index: usize,
    surface_index: usize,
    rejected_index: usize,
    generation: u64,
    owner: MountedReconcileOwner,
    active: bool,
}

#[expect(clippy::large_enum_variant, reason = "The grant keeps its reconciler and reservation inline until the exact ownership transfer; an extra heap owner would escape that pre-admitted storage.")]
enum MountedReconcileOwner {
    Live { reconciler: SurfaceReconciler, reservation: SurfaceReconcileReservation },
    Transferred,
}

impl MountedReconcileGrant {
    #[expect(clippy::result_large_err, reason = "Admission failure returns the original bounded surface identity or tree owner without allocation.")]
    pub fn commit_source(mut self, root: TreeNode) -> Result<(), TreeNode> {
        let mut state = self.state.borrow_mut();
        if state.closing_instances.iter().flatten().any(|closing| closing.key == self.key) {
            return Err(root);
        }
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
        let mut state = self.state.borrow_mut();
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
        let surface = {
            let Some(slot) = state.slots[self.surface_index].as_mut().filter(|slot| slot.key == self.key && slot.output_index == Some(self.output_index) && slot.reconciler.is_none() && slot.producer.is_none() && slot.job.is_none()) else {
                drop(state);
                drop(admission);
                self.active = false;
                return;
            };
            slot.generation = marker.generation;
            slot.operation = semio_framework_job::allocate_operation_id();
            slot.preview_sequence = 0;
            slot.cancel = semio_framework_job::root_cancel_token();
            slot.surface.clone()
        };
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
        let mut state = self.state.borrow_mut();
        if state.unadmitted[self.index].as_ref().is_some_and(|slot| slot.generation == self.generation) {
            state.unadmitted[self.index] = None;
        }
        if state.rejected_reserved[self.rejected_index] == Some(self.generation) {
            state.rejected_reserved[self.rejected_index] = None;
        }
        if let Some(output) = state.ready[self.output_index].as_mut().filter(|output| output.key == self.key && output.generation == self.generation) {
            output.closing = true;
        }
        if let Some(slot) = state.slots[self.surface_index].as_mut().filter(|slot| slot.key == self.key && slot.output_index == Some(self.output_index)) {
            slot.output_index = None;
        }
        if let Some(output) = state.ready[self.output_index].as_mut().filter(|output| output.key == self.key && output.generation == self.generation) {
            output.closing = true;
        }
        if let Some(slot) = state.slots[self.surface_index].as_mut().filter(|slot| slot.key == self.key && slot.output_index == Some(self.output_index)) {
            slot.output_index = None;
        }
        let owner = std::mem::replace(&mut self.owner, MountedReconcileOwner::Transferred);
        if let (MountedReconcileOwner::Live { reconciler, reservation }, Some(slot)) = (owner, state.slots[self.surface_index].as_mut()) {
            slot.reconciler = Some(reconciler);
            drop(reservation);
        }
        self.active = false;
    }
}

impl Drop for MountedReconcileGrant {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        let mut state = self.state.borrow_mut();
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
    key: NativeCloseKey,
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
    state: std::rc::Rc<RefCell<PatchTrackerState>>,
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

    #[expect(clippy::result_large_err, reason = "Admission failure returns the original bounded surface identity or tree owner without allocation.")]
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
        let index = state.deferred.iter().position(|entry| entry.as_ref().is_some_and(|surface| deferred_surface_ready(&state, surface)))?;
        state.deferred[index].take()
    }

    #[cfg(test)]
    pub fn begin(&self, surface: String, tree: ComponentTree) -> Result<u64, (String, ComponentTree)> {
        let surface_id = match ui_contract::SurfaceId::try_from(surface) {
            Ok(surface) => surface,
            Err(surface) => return Err((surface, tree)),
        };
        let key = NativeCloseKey::fixture(surface_instance(surface_id.as_ref()).unwrap_or(0), 1);
        let grant = match self.reserve_mounted_owned(surface_id, key) {
            Ok(grant) => grant,
            Err(surface) => return Err((surface.0.to_string(), tree)),
        };
        let generation = grant.generation;
        grant.commit(tree);
        Ok(generation)
    }

    #[cfg(test)]
    pub fn retain_unadmitted(&self, surface: String, tree: ComponentTree) -> Result<u64, (String, ComponentTree)> {
        self.begin(surface, tree)
    }

    #[expect(clippy::result_large_err, reason = "Admission failure returns the original bounded surface identity or tree owner without allocation.")]
    pub(crate) fn reserve_mounted(&self, surface: ui_contract::SurfaceId, key: NativeCloseKey) -> Result<MountedReconcileGrant, ui_contract::SurfaceId> {
        if surface_instance(surface.as_ref()) != Some(key.instance()) {
            return Err(surface);
        }
        self.reserve_mounted_owned(surface, key)
    }

    #[expect(clippy::result_large_err, reason = "Admission failure returns the original bounded surface identity or tree owner without allocation.")]
    fn reserve_mounted_owned(&self, surface: ui_contract::SurfaceId, key: NativeCloseKey) -> Result<MountedReconcileGrant, ui_contract::SurfaceId> {
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
            Err(fault) => {
                state.output_fault = Some((key, fault, false));
                return Err(surface);
            }
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
                surface,
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
        state.unadmitted[index] = Some(UnadmittedSlot { key, generation });
        drop(state);
        Ok(MountedReconcileGrant { key, output_index, state: self.state.clone(), index, surface_index, rejected_index, generation, owner: MountedReconcileOwner::Live { reconciler, reservation }, active: true })
    }

    pub fn drive_one(&self) -> bool {
        let mut state = self.state.borrow_mut();
        if state.output_fault.is_some() {
            return true;
        }
        let Some(index) = (0..SURFACE_RECONCILE_ADMISSION_SLOTS)
            .map(|offset| (state.drive_cursor + offset) % SURFACE_RECONCILE_ADMISSION_SLOTS)
            .find(|index| state.slots[*index].as_ref().is_some_and(|slot| !state.closing_instances.iter().flatten().any(|closing| closing.key == slot.key) && (slot.producer.is_some() || slot.job.is_some())))
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
            if outcome != ComponentTreeProducerStep::MoreWork {
                producer.outcome = Some(outcome);
            }
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
                if let Some(output) = slot.output_index.take().and_then(|index| state.ready[index].as_mut()) {
                    output.closing = true;
                }
            }
            state.slots[index] = Some(slot);
            return has_work(&state);
        }
        state.slots[index] = Some(slot);
        has_work(&state)
    }

    /// 🐞️ Temporary trace summary of every retained slot family, read by the reactor's more-work streak trace.
    pub fn debug_state(&self) -> String {
        let state = self.state.borrow();
        let slots: Vec<String> = state
            .slots
            .iter()
            .flatten()
            .map(|slot| {
                format!(
                    "{}#g{}:{}{}{}:ack{}/rev{}:out{:?}",
                    slot.surface.as_ref(),
                    slot.generation,
                    if slot.producer.is_some() { "P" } else { "-" },
                    if slot.job.is_some() { "J" } else { "-" },
                    if slot.reconciler.is_some() { "R" } else { "-" },
                    slot.acknowledged_revision.0,
                    slot.reconciler.as_ref().map_or(0, |reconciler| reconciler.revision().0),
                    slot.output_index
                )
            })
            .collect();
        let ready: Vec<String> = state.ready.iter().flatten().map(|ready| format!("g{}:{}{}{}{}", ready.generation, if ready.published { "p" } else { "-" }, if ready.closing { "c" } else { "-" }, if ready.reservation.is_some() { "r" } else { "-" }, if ready.outputs.terminal_is_empty() { "e" } else { "-" })).collect();
        let terminals: Vec<String> = state.terminals.iter().flatten().map(|terminal| format!("g{}:{}{}{}", terminal.authority.generation(), if terminal.close { "c" } else { "-" }, if terminal.authority.fault().is_some() { "F" } else { "-" }, if terminal.authority.terminal_is_empty() { "e" } else { "-" })).collect();
        let producer_terminals: Vec<String> = state.producer_terminals.iter().flatten().map(|terminal| format!("{}:{}{}{}{}", terminal.surface.as_ref(), if terminal.close { "c" } else { "-" }, if terminal.authority.is_some() { "A" } else { "-" }, if terminal.reconciler.is_some() { "R" } else { "-" }, if terminal.reservation.is_some() { "V" } else { "-" })).collect();
        let deferred: Vec<String> = state.deferred.iter().flatten().map(|surface| surface.as_ref().to_owned()).collect();
        format!(
            "slots=[{}] ready=[{}] terminals=[{}] producer_terminals=[{}] deferred=[{}] rejected={} unadmitted={} closing={} output_fault={} generation_exhausted={} close_cursor={}",
            slots.join(","),
            ready.join(","),
            terminals.join(","),
            producer_terminals.join(","),
            deferred.join(","),
            state.rejected.iter().flatten().count(),
            state.unadmitted.iter().flatten().count(),
            state.closing_instances.iter().flatten().count(),
            state.output_fault.as_ref().map_or("none", |fault| fault.1),
            state.generation_exhausted,
            state.close_cursor
        )
    }

    /// 📤️ Work that can still produce or publish a patch (producers, jobs, ready outputs, deferred
    /// surfaces whose slot is ready to re-admit them, unadmitted surfaces, a pending output fault).
    /// Terminal and producer-terminal retirement is background maintenance driven by `close_step`
    /// every turn and must not hold the actor in `more-work`, otherwise a host drain waits for the
    /// retirement of trees it will never see. A deferred surface whose slot still awaits the host's
    /// acknowledgement of an earlier revision is host-blocked the same way: the acknowledgement
    /// arrives as its own lifecycle turn, so counting it here only makes the drain spin on itself.
    pub fn has_publishable_work(&self) -> bool {
        let state = self.state.borrow();
        state.output_fault.is_some()
            || state.slots.iter().flatten().any(|slot| slot.producer.is_some() || slot.job.is_some())
            || state.deferred.iter().flatten().any(|surface| deferred_surface_ready(&state, surface))
            || state.unadmitted.iter().any(Option::is_some)
            || state.closing_instances.iter().any(Option::is_some)
            || state.ready.iter().flatten().any(|ready| ready.published && !ready.closing)
    }

    pub fn has_work(&self) -> bool {
        has_work(&self.state.borrow())
    }

    /// 🚨️ Returns one mounted surface failure while retaining its incremental cleanup owner.
    pub fn take_render_fault(&self) -> Option<(u32, String)> {
        let mut state = self.state.borrow_mut();
        if let Some((key, fault, reported)) = state.output_fault.as_mut() {
            if !*reported {
                *reported = true;
                return Some((key.instance(), (*fault).to_owned()));
            }
        }
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
        let surface = state.slots.iter().flatten().find(|slot| slot.generation == generation).map_or("unknown surface", |slot| slot.surface.as_ref());
        Some((instance, format!("{surface}: {fault:?}")))
    }

    pub(crate) fn ready_patch_key(&self) -> Result<Option<(NativeCloseKey, u64)>, &'static str> {
        let state = self.state.try_borrow().map_err(|_| "patch publication target is busy")?;
        Ok(next_ready_index(&state).map(|index| {
            let ready = state.ready[index].as_ref().expect("selected output");
            (ready.key, ready.generation)
        }))
    }

    pub(crate) fn take_ready_patch_into(&self, key: NativeCloseKey, generation: u64, target: &mut Option<SurfaceReconcileReadyPatch>, admitted_bytes: usize) -> Result<bool, &'static str> {
        if target.is_some() {
            return Ok(false);
        }
        let metadata = size_of::<ReadySlot>();
        let Some(bytes) = admitted_bytes.checked_sub(metadata) else { return Ok(false) };
        let mut state = self.state.try_borrow_mut().map_err(|_| "patch publication target is busy")?;
        let Some(index) = next_ready_index(&state) else { return Ok(false) };
        let output = state.ready[index].as_mut().expect("selected retained output");
        if output.key != key || output.generation != generation {
            return Err("patch publication belongs to another allocation or generation");
        }
        if !output.outputs.take_front_into(target, bytes)? {
            return Ok(false);
        }
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

    pub(crate) fn mark_rejected(&self, surface: &str, expected_generation: u64) -> bool {
        let mut state = self.state.borrow_mut();
        let Some(index) = state.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.surface.as_ref() == surface)) else { return false };
        let current_generation = state.slots[index].as_ref().expect("matched rejection surface").generation;
        if current_generation != expected_generation {
            return expected_generation != 0 && current_generation > expected_generation;
        }
        let Some(target_index) = state.terminals.iter().position(Option::is_none) else { return false };
        if state.slots[index].as_ref().is_some_and(|slot| slot.producer.is_some() || slot.job.is_some()) {
            let Some(slot) = state.slots[index].as_mut() else { return false };
            slot.cancel.cancel_now();
            if slot.producer.is_some() {
                return false;
            }
            let Some(job) = slot.job.take() else { return false };
            let terminal = job.into_terminal();
            if slot.reconciler.is_none() {
                slot.reconciler = Some(SurfaceReconciler::new(slot.surface.clone()));
            }
            state.terminals[target_index] = Some(TerminalSlot { key: slot.key, instance: surface_instance(surface), authority: terminal, close: true });
            close_output(&mut state, index);
            return true;
        }
        let Some(generation) = next_generation(&state) else { return false };
        let Some(reconciler) = state.slots[index].as_mut().and_then(|slot| slot.reconciler.take()) else { return false };
        let terminal = match SurfaceReconcileTerminal::try_from_reconciler(reconciler, generation) {
            Ok(terminal) => terminal,
            Err(reconciler) => {
                if let Some(slot) = state.slots[index].as_mut() {
                    slot.reconciler = Some(reconciler);
                } else {
                    drop(reconciler);
                }
                return false;
            }
        };
        commit_generation(&mut state, generation);
        let Some(slot) = state.slots[index].as_mut() else {
            drop(terminal);
            return false;
        };
        slot.cancel.cancel_now();
        slot.reconciler = Some(SurfaceReconciler::new(slot.surface.clone()));
        state.terminals[target_index] = Some(TerminalSlot { key: slot.key, instance: surface_instance(surface), authority: terminal, close: true });
        true
    }

    pub fn mark_published_ack(&self, ack: &SurfaceReconcilePublishedAck) -> Result<bool, &'static str> {
        let generation = ack.generation();
        let revision = ack.revision().0;
        let Some(surface) = ack.surface().map(|surface| surface.0.as_str()) else { return Ok(false) };
        let mut state = self.state.try_borrow_mut().map_err(|_| "published ACK target is busy")?;
        let Some(slot) = state.slots.iter_mut().flatten().find(|slot| slot.surface.0.as_str() == surface) else { return Ok(false) };
        if slot.generation != generation {
            return Ok(generation != 0 && slot.generation > generation);
        }
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

    pub(crate) fn preflight_close_instance(&self, key: NativeCloseKey) -> Result<(), &'static str> {
        let state = self.state.try_borrow().map_err(|_| "patch close reservation is busy")?;
        if let Some(closing) = state.closing_instances.iter().flatten().find(|closing| closing.instance == key.instance()) {
            return if closing.key == key { Ok(()) } else { Err("patch close reservation belongs to another allocation") };
        }
        if state.slots.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.ready.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.rejected.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.terminals.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.producer_terminals.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.unadmitted.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
        {
            return Err("patch descendants belong to another allocation");
        }
        if state.closing_instances.iter().all(Option::is_some) {
            return Err("patch close reservation is full");
        }
        Ok(())
    }

    pub(crate) fn reserve_close_instance(&self, key: NativeCloseKey) -> Result<(), &'static str> {
        let mut state = self.state.try_borrow_mut().map_err(|_| "patch close reservation is busy")?;
        if let Some(closing) = state.closing_instances.iter().flatten().find(|closing| closing.instance == key.instance()) {
            return if closing.key == key { Ok(()) } else { Err("patch close reservation belongs to another allocation") };
        }
        if state.slots.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.ready.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.rejected.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.terminals.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.producer_terminals.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
            || state.unadmitted.iter().flatten().any(|slot| slot.key.instance() == key.instance() && slot.key != key)
        {
            return Err("patch descendants belong to another allocation");
        }
        let slot = state.closing_instances.iter_mut().find(|slot| slot.is_none()).ok_or("patch close reservation is full")?;
        *slot = Some(ClosingInstance { instance: key.instance(), key, active: false, complete: false });
        Ok(())
    }

    pub(crate) fn activate_close_instance(&self, key: NativeCloseKey) -> Result<(), &'static str> {
        let mut state = self.state.try_borrow_mut().map_err(|_| "patch close reservation is busy")?;
        let closing = state.closing_instances.iter_mut().flatten().find(|closing| closing.key == key).ok_or("exact patch close reservation missing")?;
        closing.active = true;
        Ok(())
    }

    pub(crate) fn close_instance_complete(&self, key: NativeCloseKey) -> Result<bool, &'static str> {
        let state = self.state.try_borrow().map_err(|_| "patch close receipt is busy")?;
        state.closing_instances.iter().flatten().find(|closing| closing.key == key).map(|closing| closing.complete).ok_or("exact patch close receipt missing")
    }

    pub(crate) fn release_close_instance(&self, key: NativeCloseKey) -> Result<(), &'static str> {
        let mut state = self.state.try_borrow_mut().map_err(|_| "patch close receipt is busy")?;
        let closing = state.closing_instances.iter_mut().find(|closing| closing.is_some_and(|closing| closing.key == key && closing.complete)).ok_or("exact patch close receipt is not terminal")?;
        *closing = None;
        Ok(())
    }

    /// 🧹️ Retires one unit of the oldest closing owner against the caller's grant — see
    /// [`super::pending::PendingPatchAuthority::close_step`] for why a retirement unit must be priced
    /// per PAGE and not per item: every unit the reactor cannot finish this turn is answered as
    /// `MoreWork`, and every `MoreWork` is one host round trip the user waits through.
    pub fn close_step(&self, items: usize, bytes: usize) -> bool {
        let Ok(mut state) = self.state.try_borrow_mut() else { return false };
        if state.output_fault.is_some() {
            return false;
        }
        close_stranded_outputs(&mut state);
        if let Some(index) = state.ready.iter().position(|output| output.as_ref().is_some_and(|output| output.closing)) {
            match state.ready[index].as_mut().expect("retained output close").close_step(items, bytes) {
                Ok(true) => state.ready[index] = None,
                Ok(false) => {}
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
                    if terminal.authority.close_step_with_grant(items, bytes) && terminal.authority.terminal_is_empty() {
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
                if terminal.close_step(items) && terminal.terminal_is_empty() {
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
                if terminal.authority.close_step_with_grant(items, bytes) && terminal.authority.terminal_is_empty() {
                    state.terminals[index] = None;
                }
                return false;
            }
            state.closing_instances[closing_index].as_mut().expect("exact retained close receipt").complete = true;
            return false;
        }
        let Some(index) = (0..SURFACE_RECONCILE_ADMISSION_SLOTS).map(|offset| (state.close_cursor + offset) % SURFACE_RECONCILE_ADMISSION_SLOTS).find(|index| {
            state.producer_terminals[*index].as_ref().is_some_and(|slot| slot.close) || state.terminals[*index].as_ref().is_some_and(|slot| slot.close)
        }) else {
            return true;
        };
        state.close_cursor = (index + 1) % SURFACE_RECONCILE_ADMISSION_SLOTS;
        if let Some(terminal) = state.producer_terminals[index].as_mut().filter(|slot| slot.close) {
            if terminal.close_step(items) && terminal.terminal_is_empty() {
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
        if terminal.authority.close_step_with_grant(items, bytes) && terminal.authority.terminal_is_empty() {
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
    let (index, ready) = state
        .ready
        .iter()
        .enumerate()
        .filter_map(|(index, ready)| ready.as_ref().filter(|ready| ready.published && !ready.closing && !state.closing_instances.iter().flatten().any(|closing| closing.key == ready.key)).map(|ready| (index, ready)))
        .min_by_key(|(_, ready)| ready.generation)?;
    let pending = state.slots.iter().flatten().filter(|slot| slot.producer.is_some() || slot.job.is_some()).map(|slot| slot.generation).min();
    if pending.is_some_and(|generation| generation < ready.generation) {
        return None;
    }
    Some(index)
}

/// 🧹️ An unpublished, not-yet-closing output whose surface slot holds no producer or job any more can
/// never be published (its job was retired into a terminal by an instance close or a supersession), so
/// it is closed here — otherwise `has_work` reports it forever and every drain spins on `reconcile`.
fn close_stranded_outputs(state: &mut PatchTrackerState) {
    for output_index in 0..state.ready.len() {
        let Some(ready) = state.ready[output_index].as_ref() else { continue };
        if ready.published || ready.closing {
            continue;
        }
        let publisher_alive = state.slots.iter().flatten().any(|slot| slot.output_index == Some(output_index) && (slot.producer.is_some() || slot.job.is_some()));
        if publisher_alive {
            continue;
        }
        for slot in state.slots.iter_mut().flatten() {
            if slot.output_index == Some(output_index) {
                slot.output_index = None;
            }
        }
        state.ready[output_index].as_mut().expect("stranded output").closing = true;
    }
}

fn close_output(state: &mut PatchTrackerState, index: usize) {
    if let Some(output) = state.slots[index].as_mut().and_then(|slot| slot.output_index.take()).and_then(|output| state.ready[output].as_mut()) {
        output.closing = true;
    }
}

fn drive_job_one(state: &mut PatchTrackerState, index: usize) {
    let Some(slot) = state.slots[index].as_ref() else { return };
    let Some(job) = slot.job.as_ref() else { return };
    if slot.reconciler.is_some() && !job.is_ready() {
        let Some(target) = state.terminals.iter().position(Option::is_none) else { return };
        let stranded = slot.output_index.and_then(|output| state.ready[output].as_ref()).is_some_and(|ready| !ready.published);
        if stranded {
            close_output(state, index);
        }
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
        if slot.job.as_mut().expect("retained ready job authority").drive_one(&mut context) != SurfaceReconcileJobStep::Ready {
            return;
        }
        let receiver_bytes = size_of::<ReadySlot>();
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
            Err(fault) => {
                state.output_fault = Some((slot.key, fault, false));
                return;
            }
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

/// 🎟️ Whether a deferred surface can be re-admitted now: no slot holds it, or its slot has no live
/// producer/job and the host has acknowledged the slot's current revision (the predicate
/// `take_deferred_ready` releases on).
fn deferred_surface_ready(state: &PatchTrackerState, surface: &ui_contract::SurfaceId) -> bool {
    state
        .slots
        .iter()
        .flatten()
        .find(|slot| slot.surface == *surface)
        .is_none_or(|slot| slot.producer.is_none() && slot.job.is_none() && slot.reconciler.as_ref().is_some_and(|reconciler| slot.acknowledged_revision.0 >= reconciler.revision().0))
}

fn has_work(state: &PatchTrackerState) -> bool {
    state.output_fault.is_some()
        || state.slots.iter().flatten().any(|slot| slot.producer.is_some() || slot.job.is_some())
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
