//! 🎰️ Surface slots and reconcile grant: the ready patch capacity, the surface, terminal, rejected, ready and unadmitted slots, the mounted reconcile grant and owner, the closing instance, the tracker state and the patch tracker struct. See [`super`].

use super::*;

pub(super) const READY_PATCH_CAPACITY: usize = SURFACE_RECONCILE_ADMISSION_SLOTS;

pub(super) struct SurfaceSlot {
    pub key: NativeCloseKey,
    pub(super) output_index: Option<usize>,
    pub surface: ui_contract::SurfaceId,
    pub generation: u64,
    pub(super) acknowledged_revision: ui_contract::UiRevision,
    pub cancelled: bool,
    pub(super) pending_render: bool,
    pub(super) pending_render_source: Option<ui_contract::UiTextOriginalCopy>,
    pub(super) reconciler: Option<SurfaceReconciler>,
    pub producer: Option<MountedTreeProducer>,
    pub job: Option<SurfaceReconcileJob>,
}

pub(super) struct MountedTreeProducer {
    pub(super) reconciler: Option<SurfaceReconciler>,
    pub reservation: Option<SurfaceReconcileReservation>,
    pub(super) rejected_index: usize,
    pub authority: Box<ComponentTreeProducer>,
    pub outcome: Option<ComponentTreeProducerStep>,
}

pub(super) struct MountedTreeTerminal {
    pub key: NativeCloseKey,
    pub instance: Option<u32>,
    pub(super) surface_index: Option<usize>,
    pub surface: ui_contract::SurfaceId,
    pub(super) reconciler: Option<SurfaceReconciler>,
    pub reservation: Option<SurfaceReconcileReservation>,
    pub authority: Option<Box<ComponentTreeProducer>>,
    pub close: bool,
}

impl MountedTreeTerminal {
    /// 🧹️ The original source and native resources retire before the separately funded native shell release.
    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if let Some(authority) = self.authority.as_mut() {
            if !authority.close_step_on_original(cx)? || !authority.terminal_is_empty() { return Ok(false); }
            let capacity=ComponentTreeProducer::required_capacity_bytes();let grant=cx.retained_grant();
            if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_release_bytes<capacity{return Ok(false);}
            cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()})?;cx.consume_fuel(1);
            self.authority = None;
            return Ok(false);
        }
        if let Some(reconciler) = self.reconciler.as_mut() {
            if reconciler.close_native_resources_on_original(cx)? != Some(true) { return Ok(false); }
        }
        if cx.should_yield() || cx.retained_grant().maximum_items == 0 { return Ok(false); }
        if self.reservation.take().is_some() { cx.consume_fuel(1); return Ok(false); }
        if self.reconciler.take().is_some() { cx.consume_fuel(1); return Ok(false); }
        Ok(true)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.authority.is_none() && self.reservation.is_none() && self.reconciler.is_none()
    }
}

pub(super) struct TerminalSlot {
    pub key: NativeCloseKey,
    pub instance: Option<u32>,
    pub authority: SurfaceReconcileTerminal,
    pub close: bool,
}

pub(super) struct RejectedSlot {
    pub key: NativeCloseKey,
    pub surface: ui_contract::SurfaceId,
    pub authority: SurfaceReconcileRejected,
}

pub(super) struct ReadySlot {
    pub generation: u64,
    pub key: NativeCloseKey,
    pub outputs: SurfaceReconcileOutputs,
    pub reservation: Option<SurfaceReconcileOutputReservation>,
    pub published: bool,
    pub closing: bool,
}

impl ReadySlot {
    pub fn close_step(&mut self, items: usize, bytes: usize) -> Result<bool, &'static str> {
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

pub(super) struct UnadmittedSlot {
    pub key: NativeCloseKey,
    pub generation: u64,
}

/// 🎟️ Exact mounted render reservation; the tree cannot exist before its fixed slot does.
pub struct MountedReconcileGrant {
    pub key: NativeCloseKey,
    pub(super) output_index: usize,
    pub state: std::rc::Rc<RefCell<PatchTrackerState>>,
    pub index: usize,
    pub(super) surface_index: usize,
    pub(super) rejected_index: usize,
    pub generation: u64,
    pub owner: MountedReconcileOwner,
    pub active: bool,
}

#[expect(clippy::large_enum_variant, reason = "The grant keeps its reconciler and reservation inline until the exact ownership transfer; an extra heap owner would escape that pre-admitted storage.")]
pub(super) enum MountedReconcileOwner {
    Live { reconciler: SurfaceReconciler, reservation: SurfaceReconcileReservation },
    Transferred,
}

impl MountedReconcileGrant {
    #[expect(clippy::result_large_err, reason = "Admission failure returns the original bounded surface identity or tree owner without allocation.")]
    pub fn commit_source(&mut self, source: &mut Option<ComponentTree>, cx: &mut StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let capacity=ComponentTreeProducer::required_capacity_bytes();let grant=cx.retained_grant();
        if self.generation==0||source.is_none()||cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<capacity{return Ok(false);}
        let mut state = self.state.borrow_mut();
        if state.closing_instances.iter().flatten().any(|closing| closing.key == self.key) {
            return Ok(false);
        }
        if state.unadmitted[self.index].as_ref().is_none_or(|slot| slot.generation != self.generation) {
            return Ok(false);
        }
        let Some(slot) = state.slots[self.surface_index].as_ref() else { return Ok(false); };
        if slot.key!=self.key||slot.output_index!=Some(self.output_index)||slot.reconciler.is_some()||slot.producer.is_some()||slot.job.is_some()||!matches!(self.owner,MountedReconcileOwner::Live{..}) { return Ok(false); }
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()})?;
        cx.consume_fuel(1);
        let mut slot=state.slots[self.surface_index].take().expect("preflight original surface slot");
        let marker=state.unadmitted[self.index].take().expect("preflight original reservation marker");
        let MountedReconcileOwner::Live{reconciler,reservation}=std::mem::replace(&mut self.owner,MountedReconcileOwner::Transferred)else{unreachable!("preflight live original grant")};
        let producer=ComponentTreeProducer::try_new(source.take().expect("preflight original tree source"),self.generation).unwrap_or_else(|_|unreachable!("preflight nonzero original generation"));
        slot.generation = marker.generation;
        slot.cancelled = false;
        slot.producer = Some(MountedTreeProducer { reconciler: Some(reconciler), reservation: Some(reservation), rejected_index: self.rejected_index, authority: Box::new(producer), outcome: None });
        state.slots[self.surface_index] = Some(slot);
        self.active = false;
        Ok(true)
    }

    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn commit(mut self, tree: ComponentTree) {
        let owned = std::rc::Rc::clone(&self.state);
        let mut state = owned.borrow_mut();
        let owner = std::mem::replace(&mut self.owner, MountedReconcileOwner::Transferred);
        let MountedReconcileOwner::Live { reconciler, reservation } = owner else { return };
        let admission = SurfaceReconcileJob::try_new_reserved(reconciler, tree, reservation);
        let marker = state.unadmitted[self.index].take().filter(|slot| slot.generation == self.generation);
        let Some(marker) = marker else {
            drop(admission);
            self.release(&mut state);
            return;
        };
        let surface = {
            let Some(slot) = state.slots[self.surface_index].as_mut().filter(|slot| slot.key == self.key && slot.output_index == Some(self.output_index) && slot.reconciler.is_none() && slot.producer.is_none() && slot.job.is_none()) else {
                drop(admission);
                self.release(&mut state);
                return;
            };
            slot.generation = marker.generation;
        slot.cancelled = false;
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

    /// 🧹️ Releases EVERY slot this reservation holds — the unadmitted marker, the rejected-slot
    /// reservation, the reserved output, the surface slot's `output_index` and the checked-out
    /// reconciler — and disarms the [`Drop`] that would otherwise run it a second time. The ONE
    /// release, shared by [`MountedReconcileGrant::cancel`] and `Drop`.
    ///
    /// 🐛️ `Drop` used to release only the reconciler and the reservation, so a reservation that ended
    /// WITHOUT `cancel` — which is exactly what `commit_source`'s seven `Err(root)` exits do — left
    /// `slot.output_index` set and its `state.ready` output open. `reserve_mounted_owned` refuses any
    /// slot whose `output_index.is_some()`, so that surface became **permanently un-reservable**: every
    /// later dirty render was refused and only `defer`red, `deferred_surface_ready` answered `true` (no
    /// producer, no job, reconciler restored, revision acknowledged), the turn re-dirtied it, and the
    /// refusal repeated forever — a surface frozen at its last published revision while the actor
    /// answers `more-work` and publishes nothing. Each such exit also leaked one of
    /// [`READY_PATCH_CAPACITY`] output slots, so after 64 of them `reserve_mounted` refuses EVERY
    /// surface and the whole shell stops re-rendering.
    ///
    /// Measured live on `:6013` at wasm #58: after a pick on the 1-object document the host asked for
    /// all three world bodies, dropped none, and the guest answered `changed:[]` with every retained
    /// surface still at revision 1, so `data-guest-selection-json` kept `selectedIds:[]` — while
    /// `PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT`'s own doc records the same actor spinning with
    /// `acks=0` for over a thousand consecutive continuations (ticket
    /// 26/09/02/PUZZLE-3D-END-TO-END wave B46 §5 and §8.1, wave B48 §3).
    pub fn release(&mut self, state: &mut PatchTrackerState) {
        self.active = false;
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
        let owner = std::mem::replace(&mut self.owner, MountedReconcileOwner::Transferred);
        if let (MountedReconcileOwner::Live { reconciler, reservation }, Some(slot)) = (owner, state.slots[self.surface_index].as_mut()) {
            slot.reconciler = Some(reconciler);
            drop(reservation);
        }
    }

    pub fn cancel_and_defer(mut self) {
        let state = std::rc::Rc::clone(&self.state);
        let mut state = state.borrow_mut();
        if let Some(slot)=state.slots[self.surface_index].as_mut().filter(|slot|slot.key==self.key){slot.pending_render=true;}
        self.release(&mut state);
    }

    pub fn cancel(mut self) {
        let state = std::rc::Rc::clone(&self.state);
        let mut state = state.borrow_mut();
        self.release(&mut state);
    }
}

impl Drop for MountedReconcileGrant {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        let state = std::rc::Rc::clone(&self.state);
        let mut state = state.borrow_mut();
        self.release(&mut state);
    }
}

#[derive(Clone, Copy)]
pub(super) struct ClosingInstance {
    pub instance: u32,
    pub key: NativeCloseKey,
    pub active: bool,
    pub complete: bool,
}

pub(super) struct PatchTrackerState {
    pub slots: Box<[Option<SurfaceSlot>]>,
    pub rejected: Box<[Option<RejectedSlot>]>,
    pub(super) terminals: Box<[Option<TerminalSlot>]>,
    pub(super) producer_terminals: Box<[Option<MountedTreeTerminal>]>,
    pub(super) rejected_reserved: Box<[Option<u64>]>,
    pub deferred: Box<[Option<ui_contract::SurfaceId>]>,
    pub unadmitted: Box<[Option<UnadmittedSlot>]>,
    pub closing_instances: Box<[Option<ClosingInstance>]>,
    pub ready: Box<[Option<ReadySlot>]>,
    pub next_generation: u64,
    pub(super) generation_exhausted: bool,
    pub(super) drive_cursor: usize,
    pub close_cursor: usize,
    pub(super) output_fault: Option<(NativeCloseKey, &'static str, bool)>,
    /// 🩺️ Why the LAST mounted render reservation was refused, and for which surface. A refusal is not a
    /// fault — the turn defers that render and retries — but eight of the eleven refusal predicates in
    /// [`PatchTracker::reserve_mounted_owned`] produced no evidence at all, so a guest stuck in a
    /// refuse/defer cycle was indistinguishable from a guest with nothing to do. Read back through
    /// [`PatchTracker::debug_state`], which the reactor's own more-work streak trace prints.
    ///
    /// 🐛️ Measured live on `:6013` at wasm #58 on the Nakagin switch: `streak=57451` consecutive
    /// more-work turns, `effects=0`, ALL THIRTEEN surfaces of instance 1 in `deferred`, twelve of them
    /// with a clean slot (`--R:ack1/rev1:outNone`) that `reserve_mounted` nonetheless refused, and no
    /// record anywhere of which predicate refused them (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B48
    /// §6.4).
    pub(super) reserve_refusal: Option<(u8, &'static str, bool)>,
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
            reserve_refusal: None,
        }
    }
}

/// 🩺️ Which mounted slot a refused reservation named, as a `u8` so the fixed tracker state keeps its
/// 256-byte stack budget (`tracker_initialization_fits_the_component_stack_budget`). `u8::MAX` means the
/// surface holds no slot yet, which is itself part of the refusal's reading.
pub(super) fn refused_slot_index(state: &PatchTrackerState, surface: &ui_contract::SurfaceId) -> u8 {
    state.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| &slot.surface == surface)).and_then(|index| u8::try_from(index).ok()).unwrap_or(u8::MAX)
}

fn fixed_slots<T>(capacity: usize) -> Box<[Option<T>]> {
    std::iter::repeat_with(|| None).take(capacity).collect::<Vec<_>>().into_boxed_slice()
}

/// 🧵️ Fixed surface, rejected-owner, terminal-owner, and ready-publication authority.
#[derive(Default)]
pub struct PatchTracker {
    pub state: std::rc::Rc<RefCell<PatchTrackerState>>,
}
