//! 🧯️ Exact closure of the owners a registered fixture job retains, quoted and granted per independent axis.

use crate::app::{plugin_turn_admitted, plugin_typed_owner_demand, plugin_typed_owner_step, ArtifactApp, ArtifactOwnedContextHandle, ArtifactToolCompletion};
use semio_framework::action_bus::RetainedToolWireInput;
use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneProgress};
use semio_framework_value::{retained_clone::{RetainedCloneGrant, RetainedCloneStep}, retirement::controlled::ControlledRetirement, RetirementDemand, ValueError, ValueRefusalKind};

/// 🧯️ The wire pages, the typed command, the completion cell and the admitted context of one fixture job, retired in that order.
pub(crate) struct FixtureJobOwners<A: ArtifactApp> {
    pub(crate) raw: Option<RetainedToolWireInput>,
    pub(crate) command: Option<Box<A::Command>>,
    pub(crate) completion: Option<ArtifactToolCompletion<A>>,
    context: Option<ArtifactOwnedContextHandle<A>>,
    command_active: Option<ControlledRetirement<Box<A::Command>>>,
    completion_active: Option<ControlledRetirement<ArtifactToolCompletion<A>>>,
    context_active: Option<ControlledRetirement<ArtifactOwnedContextHandle<A>>>,
    closing: bool,
}

impl<A: ArtifactApp> FixtureJobOwners<A> {
    pub(crate) fn new(command: Box<A::Command>, completion: ArtifactToolCompletion<A>) -> Self {
        Self { raw: None, command: Some(command), completion: Some(completion), context: None, command_active: None, completion_active: None, context_active: None, closing: false }
    }

    /// 🔐️ Keeps the admitted original context beside the job until its issuers take it back.
    pub(crate) fn with_context(mut self, context: ArtifactOwnedContextHandle<A>) -> Self {
        self.context = Some(context);
        self
    }

    pub(crate) fn begin_close(&mut self) {
        self.closing = true;
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.closing && self.raw.is_none() && self.command.is_none() && self.command_active.is_none() && self.completion.is_none() && self.completion_active.is_none() && self.context.is_none() && self.context_active.is_none()
    }

    /// 🔎️ True once the command's frame and its original box are both physically gone.
    pub(crate) fn command_is_released(&self) -> bool {
        self.command.is_none() && self.command_active.is_none()
    }

    pub(crate) fn demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(raw) = self.raw.as_ref().filter(|raw| !raw.terminal_is_empty()) {
            return Ok(RetirementDemand { copy_bytes: raw.next_close_copy_byte_demand()?, capacity_bytes: raw.next_close_capacity_byte_demand(body)?, release_bytes: raw.next_close_release_byte_demand()?, depth: raw.next_close_depth_demand()? });
        }
        if self.command.is_some() || self.command_active.is_some() {
            return plugin_typed_owner_demand(&self.command, &self.command_active, body);
        }
        if self.completion.is_some() || self.completion_active.is_some() {
            return plugin_typed_owner_demand(&self.completion, &self.completion_active, body);
        }
        plugin_typed_owner_demand(&self.context, &self.context_active, body)
    }

    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        let demand = match self.demand(grant.maximum_copy_bytes) {
            Ok(demand) => demand,
            Err(error) => return InteractiveJobCloseStep::Refused{kind:error.kind,progress:Default::default()},
        };
        match plugin_turn_admitted(demand, grant) {
            Err(_) => return InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::DepthLimit,progress:Default::default()},
            Ok(false) => return InteractiveJobCloseStep::Pending { progress: Default::default() },
            Ok(true) => {}
        }
        if let Some(raw) = self.raw.as_mut() {
            let step = if raw.terminal_is_empty() { InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } } } else { raw.close_step(grant) };
            if raw.terminal_is_empty() {
                self.raw = None;
            }
            return Self::pending(step).admit(grant, true);
        }
        if self.command.is_some() || self.command_active.is_some() {
            return Self::retained(plugin_typed_owner_step(&mut self.command, &mut self.command_active, grant), grant);
        }
        if self.completion.is_some() || self.completion_active.is_some() {
            return Self::retained(plugin_typed_owner_step(&mut self.completion, &mut self.completion_active, grant), grant);
        }
        if self.context.is_some() || self.context_active.is_some() {
            return Self::retained(plugin_typed_owner_step(&mut self.context, &mut self.context_active, grant), grant);
        }
        InteractiveJobCloseStep::Complete { progress: Default::default() }
    }

    fn pending(step: InteractiveJobCloseStep) -> InteractiveJobCloseStep {
        match step {
            InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
            other => other,
        }
    }

    fn retained(step: Result<RetainedCloneStep, ValueError>, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        match step {
            Ok(RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) => InteractiveJobCloseStep::Pending { progress }.admit(grant, true),
            Err(error) => InteractiveJobCloseStep::Refused{kind:error.kind,progress:Default::default()},
        }
    }
}

/// 📏️ Reads one job's four independent close quotes, capacity priced at the quoted copy page.
pub(crate) fn job_quote<J: semio_framework_job::InteractiveJob + ?Sized>(job: &J) -> RetirementDemand {
    let copy_bytes = job.next_close_copy_byte_demand().expect("copy quote");
    RetirementDemand {
        copy_bytes,
        capacity_bytes: job.next_close_capacity_byte_demand(copy_bytes).expect("capacity quote"),
        release_bytes: job.next_close_release_byte_demand().expect("release quote"),
        depth: job.next_close_depth_demand().expect("depth quote"),
    }
}

/// 🎟️ Funds exactly one quoted turn of `items` admitted items.
pub(crate) fn job_grant(items: usize, demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: items, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }
}

/// 🧯️ Retires one completion handle under its own quotes; a fixture that dropped it would trip its explicit-return assertion.
pub(crate) fn retire_completion<A: ArtifactApp>(completion: ArtifactToolCompletion<A>) {
    let (mut original, mut active) = (Some(completion), None);
    for _ in 0..64 {
        if original.is_none() && active.is_none() {
            return;
        }
        let demand = plugin_typed_owner_demand(&original, &active, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).expect("completion quote");
        plugin_typed_owner_step(&mut original, &mut active, crate::app::plugin_demand_grant(demand)).expect("an emptied completion retires under its quote");
    }
    panic!("the completion did not retire");
}
