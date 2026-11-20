//! 🏗️ Retains a tool's admission, candidate and failure through the original caller's build and close turns.

use super::*;
use semio_framework::action_bus::{OriginalToolConstructionScope, ToolJobAdmissionDemand};
use semio_framework::ErasedToolJob;
use semio_framework_job::{InteractiveJob, OriginalConstructionAdmission, OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneProgress, RetirementDemand, ValueError};
use semio_plugin_ownership::bounded_ownership::{plugin_typed_owner_demand, plugin_typed_owner_original};

impl<A: ArtifactApp> ArtifactOwnedToolJobRequest<A> {
    /// 📥️ Returns the request with its own empty construction control.
    pub fn new(inputs: ArtifactOwnedToolJobInputs<A>) -> Self {
        Self { inputs: Some(inputs), construction: Default::default(), construction_failure: None, failure_retirement: None }
    }

    /// 🎒️ Transfers the inputs to a custom factory that owns its own construction control.
    pub fn into_inputs(self) -> ArtifactOwnedToolJobInputs<A> {
        assert!(self.construction.terminal_is_empty() && self.construction_failure.is_none() && self.failure_retirement.is_none(), "custom factory owns its construction control");
        self.inputs.expect("custom factory receives the original tool inputs once")
    }

    /// 🫴️ Loans the original inputs to the same construction while keeping its candidate resident.
    pub fn admit_tool(
        &mut self,
        cx: &mut StepContext<'_>,
        demand: ToolJobAdmissionDemand,
        construct: impl FnOnce(&mut Option<ArtifactOwnedToolJobInputs<A>>, &mut OriginalToolConstructionScope<'_>) -> Result<(ErasedToolJob, RetainedCloneProgress), ValueError>,
    ) -> Result<bool, OriginalTurnFailure> {
        let inputs = &mut self.inputs;
        self.construction.admit_tool(cx, demand, |scope| construct(inputs, scope))
    }

    /// 📤️ Receives an already-born job only after its admission settles on the original source.
    pub fn finish_tool(request: &mut Option<Self>, output: &mut Option<ErasedToolJob>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let owner = request.as_mut().ok_or_else(|| ValueError::invariant("original tool request is absent"))?;
        if owner.inputs.is_some() || output.is_some() {
            return Err(ValueError::invariant("original tool handoff requires the consumed inputs and an empty output").into());
        }
        if !owner.construction.settle_tool(cx)? { return Ok(false); }
        let credit = cx.adopt_original_credit(())?;
        let mut candidate = owner.construction.take_candidate().ok_or_else(|| ValueError::invariant("settled original tool candidate is absent"))?;
        if let Ok(credit) = credit { candidate.attach_credit(credit); }
        *output = Some(candidate);
        *request = None;
        Ok(true)
    }

    /// 🔁️ Resumes the retained handoff before another app callback can borrow the consumed inputs.
    pub fn build_or_resume(request: &mut Option<Self>, output: &mut Option<ErasedToolJob>, admission: &mut OriginalConstructionAdmission, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if request.as_ref().is_some_and(|owner| owner.inputs.is_none()) {
            Self::finish_tool(request, output, cx)
        } else {
            A::build_tool_job(request, output, admission, cx)
        }
    }

    /// 📏️ Quotes live construction custody before the remaining request inputs can retire.
    pub fn construction_retirement_demands(&self, body: usize) -> Result<Option<RetirementDemand>, ValueError> {
        if !self.construction.control_is_empty() { return Ok(Some(self.construction.control_close_demand())); }
        if self.construction_failure.is_some() || self.failure_retirement.is_some() {
            return plugin_typed_owner_demand(&self.construction_failure, &self.failure_retirement, body).map(Some);
        }
        let Some(job) = self.construction.candidate() else { return Ok(None) };
        Ok(Some(if job.terminal_is_empty() {
            RetirementDemand { copy_bytes: std::mem::size_of::<Option<ErasedToolJob>>(), depth: 1, ..Default::default() }
        } else {
            RetirementDemand { copy_bytes: job.next_close_copy_byte_demand()?, capacity_bytes: job.next_close_capacity_byte_demand(body)?, release_bytes: job.next_close_release_byte_demand()?, depth: job.next_close_depth_demand()?.max(1) }
        }))
    }

    /// ♻️ Retires the control, original failure and candidate under the same caller before input disposal.
    pub fn retire_construction(&mut self, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        let admission = self.construction.admission();
        if admission.failure_untaken() {
            admission.take_failure_into_step(&mut self.construction_failure, cx)?;
        } else if !admission.terminal_is_empty() {
            admission.retire_step(cx)?;
        } else if self.construction_failure.is_some() || self.failure_retirement.is_some() {
            let demand = plugin_typed_owner_demand(&self.construction_failure, &self.failure_retirement, cx.retained_grant().maximum_copy_bytes)?;
            semio_framework_job::close_original_effect_step(cx, demand, |effect| plugin_typed_owner_original(&mut self.construction_failure, &mut self.failure_retirement, effect))?;
        } else {
            self.construction.close_candidate(cx)?;
        }
        Ok(())
    }
}
