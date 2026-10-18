//! 🚪️ Tracker admission and reservation: the patch tracker constructor, the begin, defer and retain admission, the pending render and deferred ready takes and the mounted output reservation with its refusals. See [`super`].

use super::*;

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

    pub fn take_pending_render_on_original(&self,cx:&mut StepContext<'_>)->Result<Option<ui_contract::SurfaceId>,semio_framework_job::OriginalTurnFailure>{
        let mut state=self.state.borrow_mut();
        let Some(slot)=state.slots.iter_mut().flatten().find(|slot|pending_render_ready(slot))else{return Ok(None)};
        let grant=cx.retained_grant();
        if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
        if slot.pending_render_source.is_none(){slot.pending_render_source=Some(ui_contract::UiTextOriginalCopy::new(&slot.surface.0));cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(None)}
        let source=slot.pending_render_source.as_mut().unwrap();let (ready,progress)=source.step(&slot.surface.0,grant)?;
        cx.consume_retained(progress)?;if progress!=Default::default(){cx.consume_fuel(1)}
        if let Some(surface)=ready{slot.pending_render_source=None;slot.pending_render=false;return Ok(Some(ui_contract::SurfaceId(surface)))}Ok(None)
    }

    pub fn take_deferred_ready(&self) -> Option<ui_contract::SurfaceId> {
        let mut state = self.state.borrow_mut();
        let index = state.deferred.iter().position(|entry| entry.as_ref().is_some_and(|surface| deferred_surface_ready(&state, surface)))?;
        state.deferred[index].take()
    }

    #[cfg(any(test, feature = "artifact-app-testing"))]
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

    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn retain_unadmitted(&self, surface: String, tree: ComponentTree) -> Result<u64, (String, ComponentTree)> {
        self.begin(surface, tree)
    }

    #[expect(clippy::result_large_err, reason = "Admission failure returns the original bounded surface identity or tree owner without allocation.")]
    pub fn reserve_mounted(&self, surface: ui_contract::SurfaceId, key: NativeCloseKey) -> Result<MountedReconcileGrant, ui_contract::SurfaceId> {
        if surface_instance(surface.as_ref()) != Some(key.instance()) {
            return Err(self.refuse_reserve(surface, "instance-mismatch"));
        }
        self.reserve_mounted_owned(surface, key)
    }

    /// 🩺️ Records WHY one mounted render reservation was refused and hands the surface identity back
    /// unchanged — see [`PatchTrackerState::reserve_refusal`]. Every refusal predicate goes through
    /// here, so a guest stuck in a refuse/defer cycle names the predicate holding it instead of looking
    /// identical to a guest with nothing to do.
    #[expect(clippy::result_large_err, reason = "Admission failure returns the original bounded surface identity without allocation.")]
    fn refuse_reserve(&self, surface: ui_contract::SurfaceId, reason: &'static str) -> ui_contract::SurfaceId {
        let mut state = self.state.borrow_mut();
        let index = refused_slot_index(&state, &surface);
        state.reserve_refusal = Some((index, reason, false));
        surface
    }

    #[expect(clippy::result_large_err, reason = "Admission failure returns the original bounded surface identity or tree owner without allocation.")]
    fn reserve_mounted_owned(&self, surface: ui_contract::SurfaceId, key: NativeCloseKey) -> Result<MountedReconcileGrant, ui_contract::SurfaceId> {
        let mut state = self.state.borrow_mut();
        let refuse = |state: &mut PatchTrackerState, surface: ui_contract::SurfaceId, reason: &'static str| -> ui_contract::SurfaceId {
            let index = refused_slot_index(state, &surface);
            state.reserve_refusal = Some((index, reason, false));
            surface
        };
        if state.closing_instances.iter().flatten().any(|closing| surface_instance(surface.as_ref()) == Some(closing.instance)) {
            return Err(refuse(&mut state, surface, "instance-closing"));
        }
        let Some(index) = state.unadmitted.iter().position(Option::is_none) else { return Err(refuse(&mut state, surface, "unadmitted-full")) };
        let Some(surface_index) = state.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.surface == surface)).or_else(|| state.slots.iter().position(Option::is_none)) else {
            return Err(refuse(&mut state, surface, "slots-full"));
        };
        if let Some(reason) = state.slots[surface_index].as_ref().and_then(|slot| {
            if slot.key != key {
                Some("slot-key-stale")
            } else if slot.output_index.is_some() {
                Some("slot-output-held")
            } else if slot.producer.is_some() {
                Some("slot-producer-live")
            } else if slot.job.is_some() {
                Some("slot-job-live")
            } else if slot.reconciler.is_none() {
                Some("slot-reconciler-checked-out")
            } else {
                None
            }
        }) {
            return Err(refuse(&mut state, surface, reason));
        }
        let Some(generation) = next_generation(&state) else { return Err(refuse(&mut state, surface, "generations-exhausted")) };
        let Some(rejected_index) = state.rejected.iter().enumerate().find_map(|(index, slot)| (slot.is_none() && state.rejected_reserved[index].is_none()).then_some(index)) else {
            return Err(refuse(&mut state, surface, "rejected-full"));
        };
        let reservation = match SurfaceReconcileReservation::try_new(generation) {
            Ok(reservation) => reservation,
            Err(refusal) => return Err(refuse(&mut state, surface, refusal.reason())),
        };
        let Some(output_index) = state.ready.iter().position(Option::is_none) else { return Err(refuse(&mut state, surface, "ready-full")) };
        let mut outputs = SurfaceReconcileOutputs::default();
        let output_reservation = match outputs.try_reserve(generation, semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES) {
            Ok(Some(owner)) => owner,
            Ok(None) => return Err(refuse(&mut state, surface, "output-reservation-unavailable")),
            Err(fault) => {
                state.output_fault = Some((key, fault, false));
                return Err(refuse(&mut state, surface, "output-reservation-fault"));
            }
        };
        state.reserve_refusal = None;
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
                acknowledged_revision: ui_contract::UiRevision::default(),
                cancelled: false,
                pending_render: false,
                pending_render_source: None,
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

    /// 🩺️ The refusal reason of the LAST mounted render reservation, answered once, so the turn that
    /// deferred that render can name the process-wide table holding it instead of deferring in silence. The
    /// census field itself is retained for [`PatchTracker::debug_state`] — only the "already reported" mark
    /// moves (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B58; wave B56 named the reasons, this reports them).
    pub fn take_unreported_reserve_refusal(&self) -> Option<&'static str> {
        let mut state = self.state.borrow_mut();
        let (_, reason, reported) = state.reserve_refusal.as_mut()?;
        if *reported {
            return None;
        }
        *reported = true;
        Some(*reason)
    }

}
