//! 🏃️ Reconcile job: the `SurfaceReconcileJob` with its debug, step and drop. See [`super`].

use super::*;

//#region ⏭️ResumableReconcile
//#region 🎟️RetainedAuthority
/// 🧵️ Generation-keyed by-value reconciliation job advanced once per worker grant.
pub struct SurfaceReconcileJob {
    pub(super) state: Option<Box<SurfaceReconcileRetained>>,
}

impl std::fmt::Debug for SurfaceReconcileJob {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SurfaceReconcileJob")
            .field("phase", &self.state.as_ref().map(|state| state.phase))
            .field("usage", &self.state.as_ref().map(|state| state.usage))
            .field("credit", &self.state.as_ref().and_then(|state| state.credit.as_ref()))
            .field("fault", &self.fault())
            .finish()
    }
}

impl SurfaceReconcileJob {
    pub fn try_new(current: SurfaceReconciler, tree: ComponentTree, generation: u64) -> Result<Self, SurfaceReconcileRejected> {
        Self::try_new_with_limits(current, tree, generation, SurfaceReconcileLimits::default())
    }

    pub fn try_new_with_limits(mut current: SurfaceReconciler, tree: ComponentTree, generation: u64, limits: SurfaceReconcileLimits) -> Result<Self, SurfaceReconcileRejected> {
        let surface_bytes = current.shadow.surface_identifier_bytes();
        let handback = acquire_surface_reconcile_handback(&mut current.handback, generation);
        let credit = if surface_bytes <= limits.max_identifier_bytes { reserve_surface_reconcile(limits) } else { None };
        let output_handback = reserve_surface_reconcile_handback(generation);
        let (credit, handback, output_handback) = match (credit, handback, output_handback) {
            (Some(credit), Some(handback), Some(output_handback)) => (credit, handback, output_handback),
            (credit, handback, output_handback) => {
                return Err(SurfaceReconcileRejected {
                    state: Some(Box::new(SurfaceReconcileRetained {
                        output_handback,
                        generation,
                        phase: SurfaceReconcileJobPhase::Fault,
                        current: Some(current),
                        source: Some(tree),
                        cursor: None,
                        candidate: None,
                        patch: ui_contract::UiPendingPatch::default(),
                        retire_tree: SurfaceTreeRetireCursor::default(),
                        fault: Some(SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: 0, items: 1, bytes: surface_bytes }, limits }),
                        usage: SurfaceReconcileUsage::default(),
                        credit,
                        handback,
                    })),
                })
            }
        };
        let cursor = SurfaceReconcileCursor::new_admitted(tree, &current.shadow, limits, generation, Some(credit));
        Ok(Self {
            state: Some(Box::new(SurfaceReconcileRetained {
                output_handback: Some(output_handback),
                generation,
                phase: SurfaceReconcileJobPhase::Drive,
                current: Some(current),
                source: None,
                cursor: Some(cursor),
                candidate: None,
                patch: ui_contract::UiPendingPatch::default(),
                retire_tree: SurfaceTreeRetireCursor::default(),
                fault: None,
                usage: SurfaceReconcileUsage::default(),
                credit: None,
                handback: Some(handback),
            })),
        })
    }

    pub fn try_new_reserved(mut current: SurfaceReconciler, tree: ComponentTree, mut reservation: SurfaceReconcileReservation) -> Result<Self, SurfaceReconcileRejected> {
        let generation = reservation.generation;
        let limits = reservation.limits;
        let surface_bytes = current.shadow.surface_identifier_bytes();
        let handback = acquire_reserved_surface_reconcile_handback(&mut current.handback, &mut reservation.handback, generation);
        if surface_bytes > limits.max_identifier_bytes {
            return Err(SurfaceReconcileRejected {
                state: Some(Box::new(SurfaceReconcileRetained {
                    output_handback: reservation.output_handback.take(),
                    generation,
                    phase: SurfaceReconcileJobPhase::Fault,
                    current: Some(current),
                    source: Some(tree),
                    cursor: None,
                    candidate: None,
                    patch: ui_contract::UiPendingPatch::default(),
                    retire_tree: SurfaceTreeRetireCursor::default(),
                    fault: Some(SurfaceReconcileFault::IdentifierBytes { actual: surface_bytes, max: limits.max_identifier_bytes }),
                    usage: SurfaceReconcileUsage::default(),
                    credit: reservation.credit.take(),
                    handback,
                })),
            });
        }
        let Some(handback) = handback else {
            return Err(SurfaceReconcileRejected {
                state: Some(Box::new(SurfaceReconcileRetained {
                    output_handback: reservation.output_handback.take(),
                    generation,
                    phase: SurfaceReconcileJobPhase::Fault,
                    current: Some(current),
                    source: Some(tree),
                    cursor: None,
                    candidate: None,
                    patch: ui_contract::UiPendingPatch::default(),
                    retire_tree: SurfaceTreeRetireCursor::default(),
                    fault: Some(SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: 0, items: 1, bytes: surface_bytes }, limits }),
                    usage: SurfaceReconcileUsage::default(),
                    credit: reservation.credit.take(),
                    handback: None,
                })),
            });
        };
        let cursor = SurfaceReconcileCursor::new_admitted(tree, &current.shadow, limits, generation, reservation.credit.take());
        Ok(Self {
            state: Some(Box::new(SurfaceReconcileRetained {
                output_handback: reservation.output_handback.take(),
                generation,
                phase: SurfaceReconcileJobPhase::Drive,
                current: Some(current),
                source: None,
                cursor: Some(cursor),
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

    pub fn generation(&self) -> u64 {
        self.state.as_ref().map_or(0, |state| state.generation)
    }

    pub fn handback_key(&self) -> Option<SurfaceReconcileHandbackKey> {
        self.state.as_ref()?.handback.as_ref().map(|handback| handback.key)
    }

    pub fn base_revision(&self) -> ui_contract::UiRevision {
        self.state.as_ref().and_then(|state| state.current.as_ref()).map_or(ui_contract::UiRevision::default(), SurfaceReconciler::revision)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn transaction_usage(&self) -> SurfaceReconcileUsage {
        self.state.as_ref().map_or_else(SurfaceReconcileUsage::default, |state| state.cursor.as_ref().map_or(state.usage, SurfaceReconcileCursor::usage))
    }

    pub fn drive_one(&mut self, generation: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<SurfaceReconcileJobStep, semio_framework_job::OriginalTurnFailure> {
        let Some(state) = self.state.as_mut() else { return Ok(SurfaceReconcileJobStep::Fault) };
        if state.phase == SurfaceReconcileJobPhase::Fault || state.phase == SurfaceReconcileJobPhase::Closing {
            return Ok(SurfaceReconcileJobStep::Fault);
        }
        if generation != state.generation {
            state.fault = Some(SurfaceReconcileFault::StaleGeneration { expected: state.generation, actual: generation });
            state.phase = SurfaceReconcileJobPhase::Fault;
            return Ok(SurfaceReconcileJobStep::Fault);
        }
        if cx.is_cancelled() {
            state.fault = Some(SurfaceReconcileFault::Cancelled);
            state.phase = SurfaceReconcileJobPhase::Fault;
            return Ok(SurfaceReconcileJobStep::Fault);
        }
        if cx.should_yield() {
            return Ok(SurfaceReconcileJobStep::MoreWork);
        }
        if state.phase == SurfaceReconcileJobPhase::Ready {
            return Ok(SurfaceReconcileJobStep::Ready);
        }
        let outcome = match state.phase {
            SurfaceReconcileJobPhase::Drive => {
                let Some(current) = state.current.as_ref() else {
                    state.phase = SurfaceReconcileJobPhase::Fault;
                    return Ok(SurfaceReconcileJobStep::Fault);
                };
                let Some(cursor) = state.cursor.as_mut() else {
                    state.phase = SurfaceReconcileJobPhase::Fault;
                    return Ok(SurfaceReconcileJobStep::Fault);
                };
                match cursor.step(&current.shadow) {
                    SurfaceReconcileStep::Yield { .. } => SurfaceReconcileJobStep::MoreWork,
                    SurfaceReconcileStep::Complete { shadow, patch } => {
                        state.usage = cursor.usage();
                        state.candidate = Some(SurfaceReconciler::from_shadow(shadow));
                        *state.patch.source_mut().expect("ready job retains writable patch owner") = patch;
                        state.phase = SurfaceReconcileJobPhase::RetireCursor;
                        SurfaceReconcileJobStep::MoreWork
                    }
                    SurfaceReconcileStep::Fault(fault) => {
                        state.fault = Some(fault);
                        state.phase = SurfaceReconcileJobPhase::Fault;
                        SurfaceReconcileJobStep::Fault
                    }
                }
            }
            SurfaceReconcileJobPhase::RetireCursor => {
                if let Some(cursor) = state.cursor.as_mut() { if cursor.close_native_resources_on_original(cx)? != Some(true) { return Ok(SurfaceReconcileJobStep::MoreWork); } }
                if state.cursor.as_mut().is_some_and(|cursor| cursor.retire_one()) {
                    state.cursor = None;
                    state.phase = SurfaceReconcileJobPhase::RetirePrevious;
                }
                SurfaceReconcileJobStep::MoreWork
            }
            SurfaceReconcileJobPhase::RetirePrevious => {
                if let Some(current) = state.current.as_mut() { if current.close_native_resources_on_original(cx)? != Some(true) { return Ok(SurfaceReconcileJobStep::MoreWork); } }
                if state.current.as_mut().is_some_and(|current| current.retire_one()) {
                    state.current = None;
                    state.phase = SurfaceReconcileJobPhase::SealCandidate;
                    SurfaceReconcileJobStep::MoreWork
                } else {
                    SurfaceReconcileJobStep::MoreWork
                }
            }
            SurfaceReconcileJobPhase::SealCandidate => {
                let candidate = state.candidate.as_mut().expect("completed cursor retains candidate assembly");
                match candidate.seal_step(state.usage, &mut state.credit, state.patch.get().is_some()) {
                    Ok(true) => { state.phase = SurfaceReconcileJobPhase::Ready; SurfaceReconcileJobStep::Ready }
                    Ok(false) => SurfaceReconcileJobStep::MoreWork,
                    Err(error) if error.kind == ui_contract::UiDocumentAssemblyErrorKind::Contended => SurfaceReconcileJobStep::MoreWork,
                    Err(_) => { state.fault = Some(SurfaceReconcileFault::AliasCapacity); state.phase = SurfaceReconcileJobPhase::Fault; SurfaceReconcileJobStep::Fault }
                }
            }
            SurfaceReconcileJobPhase::Ready => SurfaceReconcileJobStep::Ready,
            SurfaceReconcileJobPhase::Fault | SurfaceReconcileJobPhase::Closing => SurfaceReconcileJobStep::Fault,
        };
        cx.consume_fuel(1);
        if cx.is_cancelled() {
            state.fault = Some(SurfaceReconcileFault::Cancelled);
            state.phase = SurfaceReconcileJobPhase::Fault;
            return Ok(SurfaceReconcileJobStep::Fault);
        }
        Ok(outcome)
    }

    pub fn fault(&self) -> Option<&SurfaceReconcileFault> {
        self.state.as_ref().and_then(|state| state.fault.as_ref())
    }

    pub fn is_ready(&self) -> bool { self.state.as_ref().is_some_and(|state| state.phase == SurfaceReconcileJobPhase::Ready) }

    pub const fn required_ready_transfer_bytes() -> usize {
        2 * size_of::<SurfaceReconciler>() + size_of::<SurfaceReconcileReadyPatch>() + 3 * size_of::<ui_contract::UiPendingPatch>() + 2 * size_of::<ui_contract::UiResidentPermit>() + 4 * size_of::<SurfaceReconcileHandbackReservation>()
    }

    /// 📬️ Preadmitted receivers take the exact roots while the original job shell remains retained.
    pub fn take_ready_into(&mut self, current: &mut Option<SurfaceReconciler>, ready: &mut Option<SurfaceReconcileReadyPatch>, admitted_bytes: usize) -> Result<bool, &'static str> {
        if admitted_bytes < Self::required_ready_transfer_bytes() || current.is_some() || ready.is_some() { return Ok(false); }
        let Some(state) = self.state.as_mut() else { return Ok(false); };
        if state.phase != SurfaceReconcileJobPhase::Ready { return Ok(false); }
        let Some(candidate) = state.candidate.as_ref() else { return state.refuse_publication("ready job has no canonical candidate"); };
        if !candidate.shadow.has_document() || candidate.handback.is_some() || state.handback.is_none() { return state.refuse_publication("ready job canonical owner authority is incomplete"); }
        if state.current.is_some() || state.cursor.is_some() || state.source.is_some() || !state.retire_tree.is_empty() { return state.refuse_publication("ready job still owns unfinished source retirement"); }
        let has_patch = state.patch.get().is_some();
        if has_patch && (state.credit.is_none() || state.output_handback.is_none()) { return state.refuse_publication("ready patch is missing its paired publication authority"); }
        if has_patch {
            *ready = Some(SurfaceReconcileReadyPatch { generation: state.generation, patch: ui_contract::UiPendingPatch::default(), credit: None, handback: None });
            let target = ready.as_mut().expect("preadmitted structural output receiver");
            std::mem::swap(&mut target.patch, &mut state.patch);
            target.credit = state.credit.take();
            target.handback = state.output_handback.take();
        }
        state.candidate.as_mut().expect("preflight canonical candidate").handback = state.handback.take();
        *current = state.candidate.take();
        state.phase = SurfaceReconcileJobPhase::Closing;
        Ok(true)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub(super) fn take_ready(mut self) -> Result<(SurfaceReconciler, Option<SurfaceReconcileReadyPatch>), Self> {
        let mut current = None;
        let mut ready = None;
        if self.take_ready_into(&mut current, &mut ready, Self::required_ready_transfer_bytes()) != Ok(true) { return Err(self); }
        let mut terminal = self.into_terminal();
        while !terminal.close_step() {}
        Ok((current.expect("granted exact root transfer"), ready))
    }

    pub fn into_terminal(mut self) -> SurfaceReconcileTerminal {
        SurfaceReconcileTerminal { state: self.state.take() }
    }
}

impl Drop for SurfaceReconcileJob {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            handback_surface_reconcile(state);
        }
    }
}
//#endregion 🎟️RetainedAuthority
//#endregion ⏭️ResumableReconcile
