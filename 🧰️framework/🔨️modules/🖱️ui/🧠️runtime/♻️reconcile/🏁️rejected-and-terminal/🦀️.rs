//! 🏁️ Rejected and terminal: the rejected reconcile and the terminal with their retirement, and the terminal take and handback close. See [`super`].

use super::*;

//#region ⏭️ResumableReconcile
//#region 🎟️RetainedAuthority
/// 🔄️ Exact pre-admission owner returned without snapshot/tree cloning.
pub struct SurfaceReconcileRejected {
    pub(super) state: Option<Box<SurfaceReconcileRetained>>,
}

impl std::fmt::Debug for SurfaceReconcileRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("SurfaceReconcileRejected").field("generation", &self.generation()).finish()
    }
}

impl SurfaceReconcileRejected {
    pub fn generation(&self) -> u64 {
        self.state.as_ref().map_or(0, |state| state.generation)
    }

    pub fn retry(mut self, limits: SurfaceReconcileLimits) -> Result<SurfaceReconcileJob, Self> {
        let Some(mut state) = self.state.take() else { return Err(self) };
        let (Some(mut current), Some(tree)) = (state.current.take(), state.source.take()) else {
            self.state = Some(state);
            return Err(self);
        };
        if let Some(credit) = state.credit.take() { release_surface_reconcile(credit); }
        if let Some(handback) = state.output_handback.take() { release_surface_reconcile_handback(handback); }
        current.handback = state.handback.take();
        match SurfaceReconcileJob::try_new_with_limits(current, tree, state.generation, limits) {
            Ok(job) => Ok(job),
            Err(mut rejected) => {
                if let Some(next) = rejected.state.as_mut() {
                    next.fault = state.fault.take();
                }
                Err(rejected)
            }
        }
    }

    pub fn take_sources(&mut self) -> Option<(SurfaceReconciler, ComponentTree)> {
        let state = self.state.as_mut()?;
        let mut current = state.current.take()?;
        let Some(tree) = state.source.take() else {
            state.current = Some(current);
            return None;
        };
        state.fault = None;
        if let Some(credit) = state.credit.take() { release_surface_reconcile(credit); }
        current.handback = state.handback.take();
        Some((current, tree))
    }

    pub fn handback_key(&self) -> Option<SurfaceReconcileHandbackKey> {
        self.state.as_ref()?.handback.as_ref().map(|handback| handback.key)
    }

    pub fn close_step(&mut self) -> bool {
        self.state.as_mut().is_none_or(|state| state.close_admitted_step())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.as_ref().is_none_or(|state| state.terminal_is_empty())
    }

    pub fn into_terminal(mut self) -> SurfaceReconcileTerminal {
        SurfaceReconcileTerminal { state: self.state.take() }
    }
}

impl Drop for SurfaceReconcileRejected {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            handback_surface_reconcile(state);
        }
    }
}

/// 🧹️ Public fault/cancel/close authority; each close grant retires one retained owner.
pub struct SurfaceReconcileTerminal {
    pub(super) state: Option<Box<SurfaceReconcileRetained>>,
}

impl SurfaceReconcileTerminal {
    #[expect(clippy::result_large_err, reason = "Rejected admission returns the exact reconciler, tree and reservation without a fallible allocation.")]
    pub fn try_from_reserved_sources(mut current: SurfaceReconciler, tree: ComponentTree, mut reservation: SurfaceReconcileReservation) -> Result<Self, (SurfaceReconciler, ComponentTree, SurfaceReconcileReservation)> {
        let generation = reservation.generation;
        let Some(handback) = acquire_reserved_surface_reconcile_handback(&mut current.handback, &mut reservation.handback, generation) else {
            return Err((current, tree, reservation));
        };
        Ok(Self {
            state: Some(Box::new(SurfaceReconcileRetained {
                output_handback: reservation.output_handback.take(),
                generation,
                phase: SurfaceReconcileJobPhase::Closing,
                current: Some(current),
                source: Some(tree),
                cursor: None,
                candidate: None,
                patch: ui_contract::UiPendingPatch::default(),
                retire_tree: SurfaceTreeRetireCursor::default(),
                fault: None,
                usage: SurfaceReconcileUsage::default(),
                credit: reservation.credit.take(),
                handback: Some(handback),
            })),
        })
    }

    #[expect(clippy::result_large_err, reason = "Rejected admission preserves the caller's exact reconciler without allocating on failure.")]
    pub fn try_from_reconciler(mut reconciler: SurfaceReconciler, generation: u64) -> Result<Self, SurfaceReconciler> {
        let Some(handback) = acquire_surface_reconcile_handback(&mut reconciler.handback, generation) else {
            return Err(reconciler);
        };
        let credit = None;
        Ok(Self {
            state: Some(Box::new(SurfaceReconcileRetained {
                output_handback: None,
                generation,
                phase: SurfaceReconcileJobPhase::Closing,
                current: Some(reconciler),
                source: None,
                cursor: None,
                candidate: None,
                patch: ui_contract::UiPendingPatch::default(),
                retire_tree: SurfaceTreeRetireCursor::default(),
                fault: None,
                usage: SurfaceReconcileUsage::default(),
                credit,
                handback: Some(handback),
            })),
        })
    }

    pub fn handback_key(&self) -> Option<SurfaceReconcileHandbackKey> {
        self.state.as_ref()?.handback.as_ref().map(|handback| handback.key)
    }

    pub fn generation(&self) -> u64 {
        self.state.as_ref().map_or(0, |state| state.generation)
    }

    pub fn fault(&self) -> Option<&SurfaceReconcileFault> {
        self.state.as_ref().and_then(|state| state.fault.as_ref())
    }

    pub fn resume(mut self, generation: u64) -> Result<SurfaceReconcileJob, Self> {
        let Some(state) = self.state.as_mut() else { return Err(self) };
        if state.generation != generation || state.cursor.is_none() || state.current.is_none() {
            return Err(self);
        }
        state.fault = None;
        state.cursor.as_mut().map(SurfaceReconcileCursor::clear_fault);
        state.phase = SurfaceReconcileJobPhase::Drive;
        Ok(SurfaceReconcileJob { state: self.state.take() })
    }

    pub fn close_step(&mut self) -> bool {
        self.close_step_with_grant(1, SURFACE_RECONCILE_PAGE_BYTES)
    }

    /// 📪️ Retires this terminal against the caller's PAGE grant — the same pricing
    /// [`SurfaceReconcileReadyPatch::close_step_with_grant`] and the pending-patch authority already
    /// take. A terminal holding a document-scaled retained surface keeps the reactor turn in
    /// `MoreWork`, and the host answers every `MoreWork` with one more round trip, so one owner per
    /// call is one host round trip per retained value (ticket 26/09/02, W-B2).
    pub fn close_step_on_original(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let Some(state) = self.state.as_mut() else { return Ok(true); };
        if state.close_native_resources_on_original(cx)? != Some(true) { return Ok(false); }
        if cx.should_yield() { return Ok(false); }
        let grant = cx.retained_grant();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(false); }
        let complete = state.close_admitted_run(1, grant.maximum_release_bytes.min(grant.maximum_copy_bytes));
        cx.consume_fuel(1);
        Ok(complete)
    }

    pub fn close_step_with_grant(&mut self, items: usize, bytes: usize) -> bool {
        self.state.as_mut().is_none_or(|state| state.close_admitted_run(items, bytes))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.as_ref().is_none_or(|state| state.terminal_is_empty())
    }
}

impl Drop for SurfaceReconcileTerminal {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            handback_surface_reconcile(state);
        }
    }
}

pub fn take_surface_reconcile_terminal(key: SurfaceReconcileHandbackKey) -> Result<Option<SurfaceReconcileTerminal>, &'static str> {
    let mut registry = match SURFACE_RECONCILE_HANDBACKS.try_lock() {
        Ok(registry) => registry,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(None),
        Err(std::sync::TryLockError::Poisoned(_)) => return Err("surface handback registry is poisoned"),
    };
    let Some(slot) = registry.slots.get_mut(key.slot) else { return Ok(None); };
    if !slot.reserved || slot.epoch != key.epoch || slot.generation != key.generation { return Ok(None); }
    if slot.state.as_ref().is_some_and(|state| state.handback.is_some()) { return Err("queued surface owner retains an external registry reservation"); }
    let Some(mut state) = slot.state.take() else { return Ok(None); };
    state.handback = Some(SurfaceReconcileHandbackReservation { key });
    Ok(Some(SurfaceReconcileTerminal { state: Some(state) }))
}

pub fn close_surface_reconcile_handback_one() -> Result<bool, &'static str> {
    static RESIDENT_TURN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if RESIDENT_TURN.fetch_xor(true, std::sync::atomic::Ordering::Relaxed) && ui_contract::UiResidentPermit::has_pending_returns() {
        ui_contract::UiResidentPermit::drain_one().map_err(|error| error.reason())?;
        return Ok(false);
    }
    let mut registry = match SURFACE_RECONCILE_HANDBACKS.try_lock() {
        Ok(registry) => registry,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(false),
        Err(std::sync::TryLockError::Poisoned(_)) => return Err("surface handback registry is poisoned"),
    };
    if registry.retirement_len == 0 { return Ok(!ui_contract::UiResidentPermit::has_pending_returns()); }
    let head = registry.retirement_head;
    let index = registry.retirement[head];
    {
        let slot = registry.slots.get_mut(index).ok_or("surface retirement queue contains an invalid slot")?;
        if !slot.queued { return Err("surface retirement queue lost its exact owner"); }
    }
    let mut owned = registry.slots[index].state.take();
    let complete = if let Some(state) = owned.as_mut() {
        if let Some(handback) = state.handback.take() {
            release_surface_reconcile_handback_in(&mut registry, handback);
        }
        if let Some(handback) = state.current.as_mut().and_then(|owner| owner.handback.take()) {
            release_surface_reconcile_handback_in(&mut registry, handback);
        }
        if let Some(handback) = state.candidate.as_mut().and_then(|owner| owner.handback.take()) {
            release_surface_reconcile_handback_in(&mut registry, handback);
        }
        if state.fault.is_none() && !state.patch.terminal_is_empty() {
            state.phase = SurfaceReconcileJobPhase::Closing;
            state.patch.close_step(1, SURFACE_RECONCILE_PAGE_BYTES)?;
            false
        } else { state.close_step_with(Some(&mut registry)) && state.terminal_is_empty() }
    } else { true };
    registry.slots[index].state = owned;
    if complete && (registry.slots[index].state.is_some() || !registry.slots[index].reserved) && registry.free_len >= SURFACE_RECONCILE_HANDBACK_SLOTS { return Err("surface handback free list exhausted"); }
    registry.retirement[head] = usize::MAX;
    registry.retirement_head = (head + 1) % SURFACE_RECONCILE_HANDBACK_SLOTS;
    registry.retirement_len -= 1;
    if complete {
        let slot = &mut registry.slots[index];
        slot.queued = false;
        if slot.state.is_some() {
            slot.state = None;
            slot.reserved = false;
            slot.generation = 0;
        }
        if !slot.reserved {
            let free = registry.free_len;
            registry.free[free] = index;
            registry.free_len += 1;
        }
    } else {
        let tail = (registry.retirement_head + registry.retirement_len) % SURFACE_RECONCILE_HANDBACK_SLOTS;
        registry.retirement[tail] = index;
        registry.retirement_len += 1;
    }
    Ok(registry.retirement_len == 0 && !ui_contract::UiResidentPermit::has_pending_returns())
}
//#endregion 🎟️RetainedAuthority
//#endregion ⏭️ResumableReconcile
