//! 🎟️ Retained authority: the `SurfaceReconcileRetained` custody of a reconcile across turns and the handback of a finished reconcile. See [`super`].

use super::*;

//#region ⏭️ResumableReconcile
//#region 🎟️RetainedAuthority
pub(super) struct SurfaceReconcileRetained {
    pub(super) generation: u64,
    pub(super) phase: SurfaceReconcileJobPhase,
    pub(super) current: Option<SurfaceReconciler>,
    pub(super) source: Option<ComponentTree>,
    pub(super) cursor: Option<SurfaceReconcileCursor>,
    pub(super) candidate: Option<SurfaceReconciler>,
    pub(super) patch: ui_contract::UiPendingPatch,
    pub(super) retire_tree: SurfaceTreeRetireCursor,
    pub(super) fault: Option<SurfaceReconcileFault>,
    pub(super) usage: SurfaceReconcileUsage,
    pub(super) credit: Option<ui_contract::UiResidentPermit>,
    pub(super) handback: Option<SurfaceReconcileHandbackReservation>,
    pub(super) output_handback: Option<SurfaceReconcileHandbackReservation>,
}

impl SurfaceReconcileRetained {
    pub(super) fn close_native_resources_on_original(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<bool>, semio_framework_job::OriginalTurnFailure> {
        if let Some(owner) = self.candidate.as_mut() { if owner.close_native_resources_on_original(cx)? != Some(true) { return Ok(None); } }
        if let Some(owner) = self.cursor.as_mut() { if owner.close_native_resources_on_original(cx)? != Some(true) { return Ok(None); } }
        if let Some(owner) = self.current.as_mut() { if owner.close_native_resources_on_original(cx)? != Some(true) { return Ok(None); } }
        if let Some(tree) = self.source.as_mut() {
            if let Some(resources) = tree.native_resources.as_mut() {
                if resources.close_step_on_original(cx)? != Some(true) { return Ok(None); }
                tree.native_resources = None;
                return Ok(None);
            }
        }
        self.retire_tree.close_native_resources_on_original(cx)
    }

    pub(super) fn refuse_publication(&mut self, reason: &'static str) -> Result<bool, &'static str> {
        self.fault = Some(SurfaceReconcileFault::PublicationAuthority(reason));
        self.phase = SurfaceReconcileJobPhase::Fault;
        Err(reason)
    }

    pub(super) fn close_step(&mut self) -> bool {
        self.close_step_with(None)
    }

    pub(super) fn close_step_with(&mut self, registry: Option<&mut SurfaceReconcileHandbackRegistry>) -> bool {
        self.close_unit(1, SURFACE_RECONCILE_PAGE_BYTES, registry).0
    }

    /// 🧹️ One retirement unit of this retained surface against the caller's grant, answering
    /// `(complete, consumed)`. A PAGED owner (the pending patch) spends the whole grant in one call
    /// and reports it; every other owner on the ladder is a single retained value and consumes one
    /// item, so [`Self::close_run`] can spend a page grant either way without squaring it.
    fn close_unit(&mut self, items: usize, bytes: usize, mut registry: Option<&mut SurfaceReconcileHandbackRegistry>) -> (bool, usize) {
        self.phase = SurfaceReconcileJobPhase::Closing;
        if self.fault.take().is_some() {
            return (false, 1);
        }
        if !self.patch.terminal_is_empty() {
            let _ = self.patch.close_step(items, bytes);
            return (false, items);
        }
        if let Some(candidate) = self.candidate.as_mut() {
            if !candidate.retire_one() {
                return (false, 1);
            }
            self.candidate = None;
            return (false, 1);
        }
        if let Some(cursor) = self.cursor.as_mut() {
            if !cursor.retire_one() {
                return (false, 1);
            }
            self.cursor = None;
            return (false, 1);
        }
        if let Some(current) = self.current.as_mut() {
            if !current.retire_one() {
                return (false, 1);
            }
            self.current = None;
            return (false, 1);
        }
        if !self.retire_tree.step() {
            return (false, 1);
        }
        if self.retire_tree.try_begin_tree(&mut self.source) {
            return (false, 1);
        }
        if let Some(credit) = self.credit.take() {
            release_surface_reconcile(credit);
            return (false, 1);
        }
        if let Some(handback) = self.handback.take() {
            match registry.as_deref_mut() {
                Some(registry) => release_surface_reconcile_handback_in(registry, handback),
                None => release_surface_reconcile_handback(handback),
            }
            return (false, 1);
        }
        if let Some(handback) = self.output_handback.take() {
            match registry.as_deref_mut() {
                Some(registry) => release_surface_reconcile_handback_in(registry, handback),
                None => release_surface_reconcile_handback(handback),
            }
            return (false, 1);
        }
        (true, 1)
    }

    /// 🧹️ Retires this retained surface for a bounded RUN priced by the caller's page grant instead
    /// of the single owner per call [`Self::close_step`] retires. Every unit the reactor cannot
    /// finish this turn is answered as `MoreWork` and costs the host one round trip, so a
    /// document-scaled retained surface must not be retired one value per turn (ticket 26/09/02,
    /// W-S2 §2.4 measured 1 092 units for one 180-object world publication, W-B2 1 093 turns for the
    /// mixed surface set the same interaction touches).
    fn close_run(&mut self, items: usize, bytes: usize) -> bool {
        let mut remaining = items.max(1);
        while remaining > 0 {
            let (complete, consumed) = self.close_unit(remaining, bytes, None);
            if complete {
                return true;
            }
            remaining = remaining.saturating_sub(consumed.max(1));
        }
        false
    }

    pub(super) fn close_admitted_step(&mut self) -> bool {
        self.close_admitted_run(1, SURFACE_RECONCILE_PAGE_BYTES)
    }

    pub(super) fn close_admitted_run(&mut self, items: usize, bytes: usize) -> bool {
        if self.source.is_some() && self.handback.is_none() {
            self.handback = self.output_handback.take().or_else(|| try_reserve_surface_reconcile_handback(self.generation));
            if self.handback.is_none() { return false; }
        }
        self.close_run(items, bytes)
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.current.is_none()
            && self.source.is_none()
            && self.cursor.is_none()
            && self.candidate.is_none()
            && self.patch.terminal_is_empty()
            && self.retire_tree.is_empty()
            && self.fault.is_none()
            && self.credit.is_none()
            && self.handback.is_none()
            && self.output_handback.is_none()
    }
}

pub(super) fn handback_surface_reconcile(mut state: Box<SurfaceReconcileRetained>) {
    if state.handback.is_none() {
        state.handback = state.current.as_mut().and_then(|owner| owner.handback.take()).or_else(|| state.candidate.as_mut().and_then(|owner| owner.handback.take())).or_else(|| state.output_handback.take());
    }
    let Some(reservation) = state.handback.take() else { return };
    let mut registry = SURFACE_RECONCILE_HANDBACKS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let enqueue = {
        let slot = &mut registry.slots[reservation.key.slot];
        let enqueue = !slot.queued;
        slot.state = Some(state);
        if enqueue {
            slot.queued = true;
        }
        enqueue
    };
    if enqueue {
        let tail = (registry.retirement_head + registry.retirement_len) % SURFACE_RECONCILE_HANDBACK_SLOTS;
        registry.retirement[tail] = reservation.key.slot;
        registry.retirement_len += 1;
    }
}
//#endregion 🎟️RetainedAuthority
//#endregion ⏭️ResumableReconcile
