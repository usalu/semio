//! 🧯️ Exact closure of the owners a registered fixture job retains, quoted and granted per independent axis.

use semio_framework_plugin_ownership::bounded_ownership::{plugin_typed_owner_demand, plugin_typed_owner_step};
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_app::ArtifactOwnedContextHandle;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletion;
use semio_framework::action_bus::RetainedToolWireInput;
use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneProgress, StepContext, OriginalTurnFailure, close_original_retained_step};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};

/// 🧯️ The wire pages, the typed command, the completion cell and the admitted context of one fixture job, retired in that order.
type PendingFixtureEmission<A>=(semio_framework_plugin_decl_emission::ArtifactMutationOutcome<<A as ArtifactApp>::Mutation,<A as ArtifactApp>::ConfigMutation,<A as ArtifactApp>::DraftMutation>,semio_framework_plugin_decl_app::app_emission::EphemeralEmit<A>);

#[doc(hidden)]
pub struct FixtureJobOwners<A: ArtifactApp> {
    pub raw: Option<RetainedToolWireInput>,
    pub command: Option<Box<A::Command>>,
    pub completion: Option<ArtifactToolCompletion<A>>,
    pub context: Option<ArtifactOwnedContextHandle<A>>,
    command_active: Option<ControlledRetirement<Box<A::Command>>>,
    completion_active: Option<ControlledRetirement<ArtifactToolCompletion<A>>>,
    context_active: Option<ControlledRetirement<ArtifactOwnedContextHandle<A>>>,
    pub delivered:bool,
    pub pending:Option<PendingFixtureEmission<A>>,
    pending_active:Option<ControlledRetirement<PendingFixtureEmission<A>>>,
    pub closing: bool,
}

impl<A: ArtifactApp> FixtureJobOwners<A> {
    pub fn new(command: Box<A::Command>, completion: ArtifactToolCompletion<A>) -> Self {
        Self { raw: None, command: Some(command), completion: Some(completion), context: None, command_active: None, completion_active: None, context_active: None,delivered:false,pending:None,pending_active:None, closing: false }
    }

    /// 🔐️ Keeps the admitted original context beside the job until its issuers take it back.
    pub fn with_context(mut self, context: ArtifactOwnedContextHandle<A>) -> Self {
        self.context = Some(context);
        self
    }

    #[cfg(test)]
    pub(crate) fn pending_emission(&self)->Option<&PendingFixtureEmission<A>>{self.pending.as_ref()}
    pub fn completion_is_delivered(&self)->bool{self.delivered}
    pub fn pending_is_empty(&self)->bool{self.pending.is_none()}
    pub fn prepare_emission(&mut self,cx:&mut StepContext<'_>,copied:usize,extra:usize,build:impl FnOnce()->semio_framework_plugin_decl_emission::ArtifactMutationOutcome<A::Mutation,A::ConfigMutation,A::DraftMutation>)->Result<bool,OriginalTurnFailure>{
        let capacity=std::mem::size_of::<A::Mutation>().checked_add(extra).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original fixture emission extent overflows"))?;let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copied||grant.maximum_capacity_bytes<capacity{return Ok(false)}
        cx.consume_fuel(1);assert!(self.pending.is_none());self.pending=Some((build(),semio_framework_plugin_decl_app::app_emission::EphemeralEmit::default()));cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copied,retained_capacity_bytes:capacity,..Default::default()})?;Ok(true)
    }
    pub(crate) fn completion_demand()->RetirementDemand{
        let bytes=("interactive-job.media-completion-busy".len()+"media completion authority is busy or poisoned".len()).max("interactive-job.duplicate-output".len()+"app-owned tool job attempted to complete more than once".len());
        RetirementDemand{copy_bytes:bytes,capacity_bytes:bytes+std::mem::size_of::<semio_framework::FaultScope>(),depth:1,..Default::default()}
    }
    pub fn complete(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        let demand=Self::completion_demand();let grant=cx.retained_grant();if self.pending.is_none()||cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes{return Ok(false)}
        cx.consume_fuel(1);let(emit,ephemeral)=self.pending.take().unwrap();match self.completion.as_ref().expect("original fixture completion remains owned").complete(emit,ephemeral){
            Ok(())=>{self.delivered=true;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;Ok(true)},
            Err(rejected)=>{let semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionRejection{emit,ephemeral,fault}=rejected;self.pending=Some((emit,ephemeral));let progress=RetainedCloneProgress{copied_items:1,copied_bytes:fault.code.0.len()+fault.message.len(),retained_capacity_bytes:fault.code.0.capacity()+fault.message.capacity()+std::mem::size_of::<semio_framework::FaultScope>(),..Default::default()};let original=fault.with_retained_progress(progress);Err(OriginalTurnFailure::debit_original(original,cx))},
        }
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.pending.is_none() && self.pending_active.is_none() && self.raw.is_none() && self.command.is_none() && self.command_active.is_none() && self.completion.is_none() && self.completion_active.is_none() && self.context.is_none() && self.context_active.is_none()
    }

    /// 🔎️ True once the command's frame and its original box are both physically gone.
    pub(crate) fn command_is_released(&self) -> bool {
        self.command.is_none() && self.command_active.is_none()
    }

    pub fn demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(raw) = self.raw.as_ref() {
            if raw.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()})}
            return Ok(RetirementDemand { copy_bytes: raw.next_close_copy_byte_demand()?, capacity_bytes: raw.next_close_capacity_byte_demand(body)?, release_bytes: raw.next_close_release_byte_demand()?, depth: raw.next_close_depth_demand()? });
        }
        if self.pending.is_some()||self.pending_active.is_some(){return plugin_typed_owner_demand(&self.pending,&self.pending_active,body)}
        if self.command.is_some() || self.command_active.is_some() {
            return plugin_typed_owner_demand(&self.command, &self.command_active, body);
        }
        if self.completion.is_some() || self.completion_active.is_some() {
            return plugin_typed_owner_demand(&self.completion, &self.completion_active, body);
        }
        plugin_typed_owner_demand(&self.context, &self.context_active, body)
    }

    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
        if !self.closing{return Ok(InteractiveJobCloseStep::Blocked)}
        let demand=self.demand(cx.retained_grant().maximum_copy_bytes)?;
        if let Some(raw)=self.raw.as_mut(){
            if !raw.terminal_is_empty(){let step=close_original_retained_step(cx,demand,|grant|raw.close_step(grant).map(|step|if raw.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())}))?;return Ok(match step{InteractiveJobCloseStep::Complete{progress}=>InteractiveJobCloseStep::Pending{progress},step=>step})}
            return close_original_retained_step(cx,demand,|_|{self.raw=None;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))});
        }
        let step=close_original_retained_step(cx,demand,|grant|if self.pending.is_some()||self.pending_active.is_some(){plugin_typed_owner_step(&mut self.pending,&mut self.pending_active,grant)}else if self.command.is_some()||self.command_active.is_some(){plugin_typed_owner_step(&mut self.command,&mut self.command_active,grant)}else if self.completion.is_some()||self.completion_active.is_some(){plugin_typed_owner_step(&mut self.completion,&mut self.completion_active,grant)}else if self.context.is_some()||self.context_active.is_some(){plugin_typed_owner_step(&mut self.context,&mut self.context_active,grant)}else{Ok(RetainedCloneStep::Complete(Default::default()))})?;
        Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{match step{InteractiveJobCloseStep::Complete{progress}=>InteractiveJobCloseStep::Pending{progress},step=>step}})
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

/// 🧯️ Retires one completion handle under its unchanged caller policy; a fixture that dropped it would trip its explicit-return assertion.
#[doc(hidden)]
pub fn retire_completion<A: ArtifactApp>(completion: ArtifactToolCompletion<A>,cx:&mut StepContext<'_>,grant:RetainedCloneGrant) {
    let(mut original,mut active)=(Some(completion),None);
    for _ in 0..semio_framework_plugin_testing_caller::OriginalFixtureCaller::maximum_turns(){
        if original.is_none()&&active.is_none(){return}
        cx.with_retained_loan(grant,|same|{let demand=plugin_typed_owner_demand(&original,&active,grant.maximum_copy_bytes)?;close_original_retained_step(same,demand,|incoming|plugin_typed_owner_step(&mut original,&mut active,incoming))}).expect("completion closes through its original caller");
    }
    panic!("completion retained its original close frontier");
}
