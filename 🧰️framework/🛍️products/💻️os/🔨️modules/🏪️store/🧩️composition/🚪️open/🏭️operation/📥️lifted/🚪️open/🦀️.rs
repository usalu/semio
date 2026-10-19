//! 📥️ Items of `member_open` that belong to `os-store-member-open-store` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🧩️composition/🚪️open/🦀️.rs`.
use semio_framework_value::ErasedSnapshotRetirement;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_job::StepContext;
use std::mem::ManuallyDrop;
use semio_framework_os_store_member_open::MemberOpenDiagnostic;
use semio_framework_os_store_member_open::MemberOpenRequest;
pub struct MemberStoreOpenRetained<P, M>
where
    P: super::ToValue,
    M: super::ToValue + super::FromValue + super::Mutation<P>,
{
    pub request: ManuallyDrop<Option<MemberOpenRequest>>,
    pub owners: ManuallyDrop<Option<semio_framework_os_store_engine_owners::DocumentStoreOwners<P, M>>>,
    pub history: ManuallyDrop<Option<semio_framework_os_spr::HistoryLog>>,
    pub initial: ManuallyDrop<Option<P>>,
    pub pending_edit: ManuallyDrop<Option<semio_framework_os_spr::Edit<M>>>,
    pub envelope: ManuallyDrop<Option<semio_framework_os_store_space_schema::ArtifactEnvelopeOwners<P, M>>>,
    pub runtime: ManuallyDrop<Option<semio_framework_os_store_initialization_runtime::lifted_store::ArtifactStoreInitializationRuntime<P>>>,
    pub active: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub diagnostic: Option<MemberOpenDiagnostic>,
    pub terminal: bool,
}

impl<P, M> MemberStoreOpenRetained<P, M>
where
    P: super::ToValue + Send + Sync + 'static,
    M: super::ToValue + super::FromValue + super::Mutation<P> + Send + 'static,
{
    pub fn new(request: MemberOpenRequest, owners: semio_framework_os_store_engine_owners::DocumentStoreOwners<P, M>) -> Self {
        Self {
            request: ManuallyDrop::new(Some(request)),
            owners: ManuallyDrop::new(Some(owners)),
            history: ManuallyDrop::new(None),
            initial: ManuallyDrop::new(None),
            pending_edit: ManuallyDrop::new(None),
            envelope: ManuallyDrop::new(None),
            runtime: ManuallyDrop::new(None),
            active: ManuallyDrop::new(None),
            diagnostic: None,
            terminal: false,
        }
    }

    pub fn stage_history(&mut self, history: semio_framework_os_spr::HistoryLog) -> Result<(), semio_framework_os_spr::HistoryLog> {
        if self.diagnostic.is_some() || self.terminal || self.history.is_some() || self.initial.is_some() || self.envelope.is_some() {
            return Err(history);
        }
        *self.history = Some(history);
        Ok(())
    }

    pub fn stage_initial(&mut self, initial: P) -> Result<(), P> {
        if self.diagnostic.is_some() || self.terminal || self.initial.is_some() || self.envelope.is_some() {
            return Err(initial);
        }
        *self.initial = Some(initial);
        Ok(())
    }

    pub fn stage_edit(&mut self, edit: semio_framework_os_spr::Edit<M>) -> Result<(), semio_framework_os_spr::Edit<M>> {
        if self.diagnostic.is_some() || self.terminal || self.pending_edit.is_some() {
            return Err(edit);
        }
        *self.pending_edit = Some(edit);
        Ok(())
    }

    pub fn stage_envelope(&mut self, envelope: semio_framework_os_store_space_schema::ArtifactEnvelopeOwners<P, M>) -> Result<(), semio_framework_os_store_space_schema::ArtifactEnvelopeOwners<P, M>> {
        if self.diagnostic.is_some() || self.terminal || self.envelope.is_some() || self.initial.is_some() {
            return Err(envelope);
        }
        *self.envelope = Some(envelope);
        Ok(())
    }

    pub fn stage_runtime(&mut self, runtime: semio_framework_os_store_initialization_runtime::lifted_store::ArtifactStoreInitializationRuntime<P>) -> Result<(), semio_framework_os_store_initialization_runtime::lifted_store::ArtifactStoreInitializationRuntime<P>> {
        if self.diagnostic.is_some() || self.terminal || self.runtime.is_some() || self.envelope.is_none() {
            return Err(runtime);
        }
        *self.runtime = Some(runtime);
        Ok(())
    }

    pub fn check_step_authority(&mut self, cx: &StepContext<'_>) -> Result<(), MemberOpenDiagnostic> {
        if let Some(diagnostic) = self.diagnostic {
            return Err(diagnostic);
        }
        let result = self.request.as_ref().ok_or(MemberOpenDiagnostic::Stale)?.check_step_authority(cx);
        if let Err(diagnostic) = result {
            self.reject(diagnostic);
        }
        result
    }

    pub fn reject(&mut self, diagnostic: MemberOpenDiagnostic) {
        self.diagnostic.get_or_insert(diagnostic);
    }

    pub fn retained_input_bytes(&self) -> usize {
        self.request.as_ref().map_or(0, MemberOpenRequest::retained_input_bytes)
    }

    pub fn retained_typed_owners(&self) -> (bool, bool, bool, bool) {
        (self.initial.is_some(), self.pending_edit.is_some(), self.envelope.is_some(), self.runtime.is_some())
    }
}

impl<P, M> MemberStoreOpenRetained<P, M>
where
    P: super::ToValue + Send + Sync + 'static,
    M: super::ToValue + super::FromValue + super::Mutation<P> + Send + 'static,
{
    pub fn retirement_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
        let nested = |mut demand: RetirementDemand| { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "member-open owner depth overflow"))?; Ok(demand) };
        if let Some(active) = self.active.as_ref() { return semio_framework_os_store_read::artifact_retirement_box_demands(active, copy); }
        if let Some(runtime) = self.runtime.as_ref() { return nested(RetirementDemand { copy_bytes: runtime.next_close_copy_byte_demand()?, capacity_bytes: runtime.next_close_capacity_byte_demand(copy)?, release_bytes: runtime.next_close_release_byte_demand()?, depth: runtime.next_close_depth_demand()? }); }
        if self.pending_edit.is_some() { return Ok(RetirementDemand { capacity_bytes: size_of::<semio_framework_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement<M>>(), depth: 2, ..Default::default() }); }
        if self.envelope.is_some() { return Ok(RetirementDemand { capacity_bytes: size_of::<semio_framework_os_store_read::ArtifactStoreEnvelopeRetirement<P, M>>(), depth: 2, ..Default::default() }); }
        if let Some(initial) = self.initial.as_ref() { return Ok(RetirementDemand { capacity_bytes: self.owners.as_ref().expect("member-open retains its initial factory").initial_snapshot_retirement.retirement_birth_bytes(initial), depth: 2, ..Default::default() }); }
        if self.history.is_some() { return Ok(RetirementDemand { capacity_bytes: semio_framework_value::retirement::controlled::controlled_retirement_birth_bytes::<semio_framework_os_spr::HistoryLog>(), depth: 2, ..Default::default() }); }
        if let Some(request) = self.request.as_ref() { return nested(RetirementDemand { copy_bytes: request.next_copy_byte_demand()?, capacity_bytes: request.next_capacity_byte_demand(copy)?, release_bytes: request.next_release_byte_demand()?, depth: request.next_depth_demand()? }); }
        self.owners.as_ref().map_or(Ok(Default::default()), |owners| nested(owners.uninstalled_owners_demands(copy)?))
    }
}

impl<P, M> ErasedSnapshotRetirement for MemberStoreOpenRetained<P, M>
where
    P: super::ToValue + Send + Sync + 'static,
    M: super::ToValue + super::FromValue + super::Mutation<P> + Send + 'static,
{
    fn next_demand(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.retirement_demands(body)}

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::admit_retained_clone_close};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "member-open exceeds admitted original owner depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.reject(MemberOpenDiagnostic::Cancelled);
        if self.active.is_some() { return semio_framework_os_store_read::artifact_retirement_box_close_step(&mut self.active, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        let owners = self.owners.as_ref().expect("member-open retains original catalog");
        if let Some(runtime) = self.runtime.as_mut() {
            let step = runtime.close_step(&owners.initial_snapshot_retirement, child)?;
            let terminal = runtime.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "member-open original initialization")?;
            if terminal { drop(self.runtime.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(edit) = self.pending_edit.take() {
            return match semio_framework_value::retirement::frame::admit_retirement_frame(edit, child, |edit| semio_framework_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement::new(edit, owners.mutation_retirement.clone())) {
                Ok((owner, progress)) => { *self.active = Some(owner); Ok(RetainedCloneStep::Progress(progress)) }
                Err((error, edit)) => { *self.pending_edit = Some(edit); Err(error) }
            };
        }
        if let Some(envelope) = self.envelope.take() {
            return match semio_framework_value::retirement::frame::admit_retirement_frame(envelope, child, |envelope| semio_framework_os_store_read::ArtifactStoreEnvelopeRetirement::new(semio_framework_os_store_space_schema::ArtifactEnvelope::from_owners(envelope), owners.initial_snapshot_retirement.clone(), owners.mutation_retirement.clone())) {
                Ok((owner, progress)) => { *self.active = Some(owner); Ok(RetainedCloneStep::Progress(progress)) }
                Err((error, envelope)) => { *self.envelope = Some(envelope); Err(error) }
            };
        }
        if let Some(initial) = self.initial.take() {
            return match owners.initial_snapshot_retirement.retire_owned(initial, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "member-open initial factory changed its constructor receipt")); } Ok(RetainedCloneStep::Progress(progress)) }
                Err((error, initial)) => { *self.initial = Some(initial); Err(error) }
            };
        }
        if let Some(history) = self.history.take() {
            return match semio_framework_value::retirement::controlled::admit_typed_controlled_retirement(history, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); Ok(RetainedCloneStep::Progress(progress)) }
                Err((error, history)) => { *self.history = Some(history); Err(error) }
            };
        }
        if let Some(request) = self.request.as_mut() {
            let step = request.close_step(child)?;
            let terminal = request.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "member-open original request")?;
            if terminal { drop(self.request.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        let owners = self.owners.as_mut().expect("member-open catalog remains until terminal");
        let step = owners.close_uninstalled_owners_step(child)?;
        let terminal = owners.uninstalled_owners_terminal_is_empty();
        admit_retained_clone_close(child, step, terminal, "member-open original catalog")?;
        if terminal { drop(self.owners.take()); self.terminal = true; }
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.request.is_none() && self.owners.is_none() && self.history.is_none() && self.initial.is_none() && self.pending_edit.is_none() && self.envelope.is_none() && self.runtime.is_none() && self.active.is_none()
    }
    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(copy)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(0)?.depth) }
}

impl<P, M> Drop for MemberStoreOpenRetained<P, M>
where
    P: super::ToValue,
    M: super::ToValue + super::FromValue + super::Mutation<P>,
{
    fn drop(&mut self) {
        assert!(
            std::thread::panicking() || (self.terminal && self.request.is_none() && self.owners.is_none() && self.history.is_none() && self.initial.is_none() && self.pending_edit.is_none() && self.envelope.is_none() && self.runtime.is_none() && self.active.is_none()),
            "member-open reached Drop before exact adoption or bounded rejection retirement"
        );
    }
}
