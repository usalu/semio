//! 🧳️ Retained parameter decisions, diagnostic ownership and exact field inverses.

use crate::{RetainedParameterDiagnosticOwner, RetainedParameterDisposition, RetainedParameterScalar};
use protocol::{Mutation, MutationMessage, RetainedDiagnosticPreparation, RetainedDiagnosticSpec, RetainedLeafWrapper, RetainedMutationAdvance, RetainedMutationCloseContext, RetainedMutationDecision, RetainedMutationLeafPreparation, RetainedSetField, RetainedSetFieldLeaf};
use semio_framework_value::{OriginalRetirementFailure, OriginalRetirementOutcome, RetainedWorkOutcome, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneBinding, RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep};
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use std::mem::{size_of, ManuallyDrop};

fn invalid() -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, "retained parameter preparation lost its source or phase") }
fn scalar(copy_bytes: usize) -> RetirementDemand { RetirementDemand { copy_bytes, depth: 1, ..Default::default() } }
fn nested(mut demand: RetirementDemand) -> Result<RetirementDemand, ValueError> { demand.copy_bytes = demand.copy_bytes.checked_add(size_of::<u8>()).ok_or_else(invalid)?; demand.depth = demand.depth.checked_add(1).ok_or_else(invalid)?; Ok(demand) }
fn child(grant: RetainedCloneGrant) -> RetainedCloneGrant { RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - size_of::<u8>(), maximum_depth: grant.maximum_depth - 1, ..grant } }
fn parent(mut progress: RetainedCloneProgress) -> Result<RetainedCloneProgress, ValueError> { progress.copied_bytes = progress.copied_bytes.checked_add(size_of::<u8>()).ok_or_else(invalid)?; Ok(progress) }
fn parent_error(error: OriginalRetirementFailure) -> OriginalRetirementFailure { let progress = parent(error.retained_progress()).unwrap_or_else(|_| error.retained_progress()); error.with_retained_progress(progress) }

/// 🧩️ Keeps the actual parameter payload and original snapshot bound across every decision and inverse turn.
pub struct RetainedParameterPreparation<L, M>
where
    L: RetainedParameterScalar + RetainedSetField<RemodelingSnapshot, RemodelingDiff, M>,
    M: Mutation<RemodelingSnapshot, Diff = RemodelingDiff> + RetainedLeafWrapper<L> + Send + 'static,
{
    payload: Option<RetainedCloneBinding>,
    original: Option<RetainedCloneBinding>,
    field: RetainedSetFieldLeaf<L, RemodelingSnapshot, RemodelingDiff, M>,
    numeric: RetainedParameterDiagnosticOwner,
    untargeted: RetainedDiagnosticPreparation<0>,
    targeted: RetainedDiagnosticPreparation<1>,
    discarded: ManuallyDrop<Option<RemodelingDiff>>,
    diff: ManuallyDrop<Option<RemodelingDiff>>,
    messages: ManuallyDrop<Option<Vec<MutationMessage>>>,
    ledger: Option<protocol::MutationMessageLedgerRetirement>,
    retirement: RetainedCloneClose,
    disposition: Option<RetainedParameterDisposition>,
    phase: u8,
    closing: bool,
}

impl<L, M> RetainedParameterPreparation<L, M>
where
    L: RetainedParameterScalar + RetainedSetField<RemodelingSnapshot, RemodelingDiff, M>,
    M: Mutation<RemodelingSnapshot, Diff = RemodelingDiff> + RetainedLeafWrapper<L> + Send + 'static,
{
    fn check(&self, payload: RetainedCloneRef<'_, L>, original: RetainedCloneRef<'_, RemodelingSnapshot>) -> Result<(), ValueError> { payload.check_binding(&self.payload)?; original.check_binding(&self.original)?; Ok(()) }
    fn kind(&self) -> Result<RetainedParameterDisposition, ValueError> { self.disposition.ok_or_else(invalid) }
    fn target<'a>(original: RetainedCloneRef<'a, RemodelingSnapshot>) -> [RetainedCloneRef<'a, String>; 1] { [original.project(0, |value| &value.id)] }
    fn spec(&self) -> Result<RetainedDiagnosticSpec, ValueError> {
        match self.kind()? {
            RetainedParameterDisposition::Unchanged => Ok(RetainedDiagnosticSpec { level: semio_framework_diagnostic::Severity::Warning, code: "mutation.no-op", message: L::NOOP_MESSAGE }),
            RetainedParameterDisposition::TargetedRefusal | RetainedParameterDisposition::UntargetedRefusal => Ok(RetainedDiagnosticSpec { level: semio_framework_diagnostic::Severity::Fatal, code: "mutation.invariant", message: L::REFUSAL_MESSAGE.ok_or_else(invalid)? }),
            _ => Err(invalid()),
        }
    }
}

impl<L, M> RetainedMutationLeafPreparation<RemodelingSnapshot, M, L> for RetainedParameterPreparation<L, M>
where
    L: RetainedParameterScalar + RetainedSetField<RemodelingSnapshot, RemodelingDiff, M>,
    M: Mutation<RemodelingSnapshot, Diff = RemodelingDiff> + RetainedLeafWrapper<L> + Send + 'static,
{
    fn new() -> Self {
        Self { payload: None, original: None, field: RetainedMutationLeafPreparation::new(), numeric: RetainedParameterDiagnosticOwner::new(), untargeted: Default::default(), targeted: Default::default(), discarded: ManuallyDrop::new(None), diff: ManuallyDrop::new(None), messages: ManuallyDrop::new(None), ledger: None, retirement: Default::default(), disposition: None, phase: 0, closing: false }
    }

    fn birth_work() -> Result<u64, ValueError> { u64::try_from(size_of::<Self>()).ok().and_then(|n| n.checked_add(1)).ok_or_else(|| ValueError::literal(ValueRefusalKind::WorkLimit, "retained parameter birth work overflow")) }

    fn decision_demand(&self, payload: RetainedCloneRef<'_, L>, original: RetainedCloneRef<'_, RemodelingSnapshot>, body: usize, maximum_work_units: u64) -> Result<RetirementDemand, ValueError> {
        if self.closing || self.phase >= 12 { return Err(invalid()); }
        self.check(payload, original)?;
        match self.phase {
            0 => Ok(scalar(payload.binding_copy_bytes() + size_of::<u8>())),
            1 => Ok(scalar(original.binding_copy_bytes() + size_of::<u8>())),
            2 => Ok(scalar(size_of::<Option<RetainedParameterDisposition>>() + size_of::<u8>())),
            3 => nested(self.field.decision_demand(payload, original, body, maximum_work_units.saturating_sub(1))?),
            4 => Ok(scalar(size_of::<RetainedMutationDecision<RemodelingDiff>>() + 2 * size_of::<Option<RemodelingDiff>>() + 2 * size_of::<Option<Vec<MutationMessage>>>() + 2 * size_of::<u8>())),
            5 => nested(self.retirement.next_owner_demands::<RemodelingDiff>(self.discarded.is_some(), &None, body)?),
            6 => nested(match self.kind()? { RetainedParameterDisposition::TargetedRefusal => RetainedDiagnosticPreparation::<1>::start_demand(), _ => RetainedDiagnosticPreparation::<0>::start_demand() }),
            7 => nested(match self.kind()? { RetainedParameterDisposition::TargetedRefusal => self.targeted.demand(Self::target(original), body, maximum_work_units.saturating_sub(1))?, _ => self.untargeted.demand([], body, maximum_work_units.saturating_sub(1))? }),
            8 => Ok(scalar(size_of::<Vec<MutationMessage>>() + 2 * size_of::<Option<Vec<MutationMessage>>>() + size_of::<u8>())),
            9 => Ok(scalar(2 * size_of::<Option<RemodelingDiff>>() + size_of::<u8>())),
            10 => nested(self.numeric.demand(L::retained_parameter_numeric_source(payload).ok_or_else(invalid)?, body, maximum_work_units.saturating_sub(1))?),
            11 => { let mut d = self.numeric.take_demand(); d.copy_bytes = d.copy_bytes.checked_add(2 * size_of::<Option<Vec<MutationMessage>>>() + size_of::<u8>()).ok_or_else(invalid)?; Ok(d) },
            _ => Err(invalid()),
        }
    }

    fn decision_work_demand(&self, payload: RetainedCloneRef<'_, L>, original: RetainedCloneRef<'_, RemodelingSnapshot>, body: usize, maximum_work_units: u64) -> Result<u64, ValueError> {
        let demand = self.decision_demand(payload, original, body, maximum_work_units)?;
        if maximum_work_units == 0 { return Ok(0); }
        let work = match self.phase {
            2 => L::WORK_DEMAND,
            3 => { let child = self.field.decision_work_demand(payload, original, body, maximum_work_units.saturating_sub(1))?; if child == 0 { return Ok(0); } child.checked_add(1).ok_or_else(invalid)? },
            5 => self.retirement.next_owner_work::<RemodelingDiff>(self.discarded.is_some(), &None, body)?.checked_add(1).ok_or_else(invalid)?,
            6 => 2,
            7 => { let child = match self.kind()? { RetainedParameterDisposition::TargetedRefusal => self.targeted.advance_work_demand(Self::target(original), body, maximum_work_units.saturating_sub(1))?, _ => self.untargeted.advance_work_demand([], body, maximum_work_units.saturating_sub(1))? }; if child == 0 { return Ok(0); } child.checked_add(1).ok_or_else(invalid)? },
            9 => 9,
            10 => { let child = self.numeric.work_demand(L::retained_parameter_numeric_source(payload).ok_or_else(invalid)?, body, maximum_work_units.saturating_sub(1))?; if child == 0 { return Ok(0); } child.checked_add(1).ok_or_else(invalid)? },
            _ => { let _ = demand; 1 },
        };
        Ok(if work <= maximum_work_units { work } else { 0 })
    }

    fn decide(&mut self, payload: RetainedCloneRef<'_, L>, original: RetainedCloneRef<'_, RemodelingSnapshot>, body: usize, maximum_work_units: u64, grant: RetainedCloneGrant, actual: &mut Option<usize>) -> RetainedWorkOutcome<RetainedMutationAdvance, OriginalRetirementFailure> {
        let mut work = 0;
        let result: Result<_, OriginalRetirementFailure> = (|| {
            let selected = self.decision_work_demand(payload, original, body, maximum_work_units)?;
            if selected == 0 { return Ok(RetainedMutationAdvance::Progress(Default::default())); }
            let demand = self.decision_demand(payload, original, body, maximum_work_units)?;
            if !protocol::retained_mutation_demand_fits(demand, grant) { return Ok(RetainedMutationAdvance::Progress(Default::default())); }
            let mut progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
            work = 1;
            match self.phase {
                0 => { let turn = payload.bind(&mut self.payload, RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - size_of::<u8>(), ..grant }); work = turn.work; progress = turn.result?.ok_or_else(invalid)?; progress.copied_bytes += size_of::<u8>(); self.phase = 1; },
                1 => { let turn = original.bind(&mut self.original, RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - size_of::<u8>(), ..grant }); work = turn.work; progress = turn.result?.ok_or_else(invalid)?; progress.copied_bytes += size_of::<u8>(); self.phase = 2; },
                2 => { let (disposition, actual) = payload.get().retained_parameter_disposition(original.get()); self.disposition = Some(disposition); work = actual; self.phase = match disposition { RetainedParameterDisposition::Changed | RetainedParameterDisposition::Unchanged => 3, RetainedParameterDisposition::NumericRefusal => 10, _ => 6 }; if actual > selected { return Err(invalid().with_retained_progress(progress).into()); } },
                3 => { let turn = self.field.decide(payload, original, body, maximum_work_units - 1, child(grant), actual); work += turn.work; let step = turn.result.map_err(parent_error)?; progress = parent(step.progress())?; if matches!(step, RetainedMutationAdvance::Ready(_)) { self.phase = if self.kind()? == RetainedParameterDisposition::Changed { 12 } else { 4 }; } },
                4 => { match self.field.take_decision() { RetainedMutationDecision::Admitted(diff, rows) => { *self.discarded = Some(diff); if !rows.is_empty() || rows.capacity() != 0 { *self.messages = Some(rows); return Err(invalid().with_retained_progress(progress).into()); } self.phase = 5; }, RetainedMutationDecision::Refused(rows) => { *self.messages = Some(rows); return Err(invalid().with_retained_progress(progress).into()); } } },
                5 => { if self.discarded.is_none() && self.retirement.is_empty() { self.phase = 6; } else if self.retirement.is_empty() { let turn = self.retirement.begin_granted(&mut self.discarded, child(grant), actual); work += turn.work; let step = turn.result.map_err(|error| { let progress = parent(error.retained_progress()).unwrap_or_else(|_| error.retained_progress()); error.with_retained_progress(progress) })?.ok_or_else(invalid)?; progress = parent(step.progress())?; } else { let turn = self.retirement.step_granted(child(grant)); work += turn.work; let step = turn.result.map_err(parent_error)?; progress = parent(step.progress())?; } },
                6 => { let spec = self.spec()?; progress = parent(match self.kind()? { RetainedParameterDisposition::TargetedRefusal => self.targeted.start(spec, child(grant))?, _ => self.untargeted.start(spec, child(grant))? })?; self.phase = 7; work = 2; },
                7 => { let turn = match self.kind()? { RetainedParameterDisposition::TargetedRefusal => self.targeted.advance(Self::target(original), body, maximum_work_units - 1, child(grant)), _ => self.untargeted.advance([], body, maximum_work_units - 1, child(grant)) }; work += turn.work; let step = turn.result.map_err(parent_error)?; progress = parent(step.progress())?; if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 8; } },
                8 => { let extra = 2 * size_of::<Option<Vec<MutationMessage>>>() + size_of::<u8>(); let g = RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - extra, ..grant }; let turn = match self.kind()? { RetainedParameterDisposition::TargetedRefusal => self.targeted.take(g), _ => self.untargeted.take(g) }; work = turn.work; let (rows, p) = turn.result?.ok_or_else(invalid)?; *self.messages = Some(rows); progress = p; progress.copied_bytes += extra; self.phase = if self.kind()? == RetainedParameterDisposition::Unchanged { 9 } else { 12 }; },
                9 => { *self.diff = Some(RemodelingDiff::default()); work = 9; self.phase = 12; },
                10 => { let turn = self.numeric.advance(L::retained_parameter_numeric_source(payload).ok_or_else(invalid)?, body, maximum_work_units - 1, child(grant)); work += turn.work; let step = turn.result.map_err(parent_error)?; progress = parent(step.progress())?; if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 11; } },
                11 => { let extra = 2 * size_of::<Option<Vec<MutationMessage>>>() + size_of::<u8>(); let turn = self.numeric.take(RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - extra, ..grant }); work = turn.work; let (rows, p) = turn.result?.ok_or_else(invalid)?; *self.messages = Some(rows); progress = p; progress.copied_bytes += extra; self.phase = 12; },
                _ => return Err(invalid().into()),
            }
            if !progress.fits(grant) { return Err(invalid().with_retained_progress(progress).into()); }
            Ok(if self.phase == 12 { RetainedMutationAdvance::Ready(progress) } else { RetainedMutationAdvance::Progress(progress) })
        })();
        RetainedWorkOutcome { result, work }
    }

    fn take_decision(&mut self) -> RetainedMutationDecision<RemodelingDiff> {
        assert!(self.phase == 12 && !self.closing, "parameter decision must be ready before transfer");
        self.phase = 13;
        match self.kind().expect("prepared disposition") {
            RetainedParameterDisposition::Changed => self.field.take_decision(),
            RetainedParameterDisposition::Unchanged => RetainedMutationDecision::Admitted(self.diff.take().expect("prepared empty diff"), self.messages.take().expect("prepared warning")),
            _ => RetainedMutationDecision::Refused(self.messages.take().expect("prepared refusal")),
        }
    }

    fn inverse_demand(&self, payload: RetainedCloneRef<'_, L>, original: RetainedCloneRef<'_, RemodelingSnapshot>, diff: RetainedCloneRef<'_, RemodelingDiff>, body: usize, maximum_work_units: u64) -> Result<RetirementDemand, ValueError> {
        if self.closing || self.phase != 13 || !matches!(self.kind()?, RetainedParameterDisposition::Changed | RetainedParameterDisposition::Unchanged) { return Err(invalid()); }
        self.check(payload, original)?;
        nested(self.field.inverse_demand(payload, original, diff, body, maximum_work_units.saturating_sub(1))?)
    }
    fn inverse_work_demand(&self, payload: RetainedCloneRef<'_, L>, original: RetainedCloneRef<'_, RemodelingSnapshot>, diff: RetainedCloneRef<'_, RemodelingDiff>, body: usize, maximum_work_units: u64) -> Result<u64, ValueError> {
        self.inverse_demand(payload, original, diff, body, maximum_work_units)?;
        let child = self.field.inverse_work_demand(payload, original, diff, body, maximum_work_units.saturating_sub(1))?;
        if child == 0 { return Ok(0); }
        let work = child.checked_add(1).ok_or_else(invalid)?;
        Ok(if work <= maximum_work_units { work } else { 0 })
    }
    fn inverse(&mut self, payload: RetainedCloneRef<'_, L>, original: RetainedCloneRef<'_, RemodelingSnapshot>, diff: RetainedCloneRef<'_, RemodelingDiff>, body: usize, maximum_work_units: u64, grant: RetainedCloneGrant, actual: &mut Option<usize>) -> RetainedWorkOutcome<RetainedMutationAdvance, OriginalRetirementFailure> {
        let mut work = 0;
        let result: Result<_, OriginalRetirementFailure> = (|| {
            if self.inverse_work_demand(payload, original, diff, body, maximum_work_units)? == 0 { return Ok(RetainedMutationAdvance::Progress(Default::default())); }
            let demand = self.inverse_demand(payload, original, diff, body, maximum_work_units)?;
            if !protocol::retained_mutation_demand_fits(demand, grant) { return Ok(RetainedMutationAdvance::Progress(Default::default())); }
            let turn = self.field.inverse(payload, original, diff, body, maximum_work_units - 1, child(grant), actual); work = 1 + turn.work;
            let step = turn.result.map_err(parent_error)?;
            let progress = parent(step.progress())?;
            if !progress.fits(grant) { return Err(invalid().with_retained_progress(progress).into()); }
            Ok(if matches!(step, RetainedMutationAdvance::Ready(_)) { RetainedMutationAdvance::Ready(progress) } else { RetainedMutationAdvance::Progress(progress) })
        })();
        RetainedWorkOutcome { result, work }
    }
    fn take_inverse(&mut self) -> Vec<M> { self.field.take_inverse() }

    fn begin_close(&mut self) { self.closing = true; self.field.begin_close(); self.numeric.begin_close(); self.untargeted.begin_close(); self.targeted.begin_close(); }
    fn close_demand(&self, context: RetainedMutationCloseContext<'_, M>, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.closing { return Err(invalid()); }
        if !self.field.terminal_is_empty() { return nested(self.field.close_demand(context, body)?); }
        if !self.numeric.terminal_is_empty() { return nested(self.numeric.close_demand(body)?); }
        if !self.untargeted.terminal_is_empty() { return nested(self.untargeted.close_demand(body)?); }
        if !self.targeted.terminal_is_empty() { return nested(self.targeted.close_demand(body)?); }
        if !self.retirement.is_empty() { return self.retirement.next_owner_demands::<RemodelingDiff>(false, &None, body); }
        if self.discarded.is_some() || self.diff.is_some() { return self.retirement.next_owner_demands::<RemodelingDiff>(true, &None, body); }
        if self.messages.is_some() { return Ok(scalar(2 * size_of::<Option<Vec<MutationMessage>>>() + 2 * size_of::<Option<protocol::MutationMessageLedgerRetirement>>())); }
        if let Some(ledger) = self.ledger.as_ref() {
            if ledger.terminal_is_empty() { return Ok(scalar(2 * size_of::<Option<protocol::MutationMessageLedgerRetirement>>())); }
            return Ok(RetirementDemand { copy_bytes: ledger.next_copy_byte_demand(), capacity_bytes: ledger.next_capacity_byte_demand(body)?, release_bytes: ledger.next_release_byte_demand()?, depth: ledger.next_depth_demand()? });
        }
        RetainedCloneBinding::close_demand(if self.payload.is_some() { &self.payload } else { &self.original }, body)
    }
    fn close_work_demand(&self, context: RetainedMutationCloseContext<'_, M>, body: usize) -> Result<u64, ValueError> {
        use semio_framework_value::ErasedSnapshotRetirement;
        if !self.closing { return Err(invalid()); }
        if !self.field.terminal_is_empty() { return self.field.close_work_demand(context, body)?.checked_add(1).ok_or_else(invalid); }
        if !self.numeric.terminal_is_empty() { return self.numeric.close_work_demand(body)?.checked_add(1).ok_or_else(invalid); }
        if !self.untargeted.terminal_is_empty() { return self.untargeted.close_work_demand(body)?.checked_add(1).ok_or_else(invalid); }
        if !self.targeted.terminal_is_empty() { return self.targeted.close_work_demand(body)?.checked_add(1).ok_or_else(invalid); }
        if !self.retirement.is_empty() { return self.retirement.next_owner_work::<RemodelingDiff>(false, &None, body); }
        if self.discarded.is_some() || self.diff.is_some() { return self.retirement.next_owner_work::<RemodelingDiff>(true, &None, body); }
        if self.messages.is_some() { return Ok(1); }
        if let Some(ledger) = self.ledger.as_ref() { return if ledger.terminal_is_empty() { Ok(1) } else { ledger.next_computational_work(body) }; }
        RetainedCloneBinding::close_work_demand(if self.payload.is_some() { &self.payload } else { &self.original }, body)
    }
    fn close_step(&mut self, context: RetainedMutationCloseContext<'_, M>, grant: RetainedCloneGrant, actual: &mut Option<usize>) -> OriginalRetirementOutcome {
        let mut work = 0;
        let result: Result<_, OriginalRetirementFailure> = (|| {
            let demand = self.close_demand(context, grant.maximum_copy_bytes)?;
            if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
            if !protocol::retained_mutation_demand_fits(demand, grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
            if !self.field.terminal_is_empty() { let turn = self.field.close_step(context, child(grant), actual); work = 1 + turn.work; let step = turn.result.map_err(parent_error)?; return Ok(RetainedCloneStep::Progress(parent(step.progress())?)); }
            if !self.numeric.terminal_is_empty() { let turn = self.numeric.close_step(child(grant)); work = 1 + turn.work; let step = turn.result.map_err(parent_error)?; return Ok(RetainedCloneStep::Progress(parent(step.progress())?)); }
            if !self.untargeted.terminal_is_empty() { let turn = self.untargeted.close_step(child(grant)); work = 1 + turn.work; let step = turn.result.map_err(parent_error)?; return Ok(RetainedCloneStep::Progress(parent(step.progress())?)); }
            if !self.targeted.terminal_is_empty() { let turn = self.targeted.close_step(child(grant)); work = 1 + turn.work; let step = turn.result.map_err(parent_error)?; return Ok(RetainedCloneStep::Progress(parent(step.progress())?)); }
            if !self.retirement.is_empty() { let turn = self.retirement.step_granted(grant, actual); work = turn.work; return Ok(RetainedCloneStep::Progress(turn.result?.progress())); }
            for slot in [&mut self.discarded, &mut self.diff] { let turn = self.retirement.begin_granted(slot, grant, actual); work += turn.work; if let Some(step) = turn.result? { return Ok(RetainedCloneStep::Progress(step.progress())); } }
            if let Some(rows) = self.messages.take() { self.ledger = Some(protocol::MutationMessageLedgerRetirement::new(String::new(), rows)); work = 1; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })); }
            if let Some(ledger) = self.ledger.as_mut() { if ledger.terminal_is_empty() { self.ledger = None; work = 1; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })); } let step = ledger.close_step(grant); work = ledger.last_turn_work(); return Ok(RetainedCloneStep::Progress(step?.progress())); }
            let binding = if self.payload.is_some() { &mut self.payload } else { &mut self.original };
            let turn = RetainedCloneBinding::close_one(binding, grant); work = turn.work; let step = turn.result?;
            Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
        })();
        RetainedWorkOutcome { result, work }
    }
    fn terminal_is_empty(&self) -> bool { self.field.terminal_is_empty() && self.numeric.terminal_is_empty() && self.untargeted.terminal_is_empty() && self.targeted.terminal_is_empty() && self.discarded.is_none() && self.diff.is_none() && self.messages.is_none() && self.ledger.is_none() && self.retirement.is_empty() && self.payload.is_none() && self.original.is_none() }
}

impl<L, M> Drop for RetainedParameterPreparation<L, M>
where
    L: RetainedParameterScalar + RetainedSetField<RemodelingSnapshot, RemodelingDiff, M>,
    M: Mutation<RemodelingSnapshot, Diff = RemodelingDiff> + RetainedLeafWrapper<L> + Send + 'static,
{
    fn drop(&mut self) { let empty = <Self as RetainedMutationLeafPreparation<RemodelingSnapshot, M, L>>::terminal_is_empty(self); assert!(empty || std::thread::panicking(), "parameter preparation requires funded closure"); if empty { unsafe { ManuallyDrop::drop(&mut self.discarded); ManuallyDrop::drop(&mut self.diff); ManuallyDrop::drop(&mut self.messages); } } }
}
