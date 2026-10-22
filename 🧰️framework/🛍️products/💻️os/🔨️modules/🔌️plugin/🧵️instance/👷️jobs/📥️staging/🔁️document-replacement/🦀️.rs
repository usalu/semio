//! 🫏️ Staged slice of the `document-replacement` band of the mounted artifact instance: its `instance-jobs` members (WP6 step S0 of the instance family).
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_instance_history::history_components::CommandHistory;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use std::future::Future;
use semio_framework_plugin_instance_stores::stores_components::InstanceStores;
use std::mem::ManuallyDrop;
use protocol::OpText;
use crate::jobs_components::{PendingReserved, ToolOperations};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwnerRegistry;
use semio_framework_ui_contract::*;

/// 🪪️ A persistent original loan owns one window renewal admission until its atomic publication and paid return.
#[doc(hidden)]
pub struct DocumentWindowResetAdmission{pub source:std::mem::ManuallyDrop<Option<semio_framework_job::OriginalCallerSourceLoan>>,pub generation:Option<u64>,pub committed:bool,pub returning:bool}

impl DocumentWindowResetAdmission{
 pub fn validate(&self,scope_generation:u64,cx:&semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{if !self.returning{self.source.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original window renewal loan is absent"))?.validate_context(cx)?;}if self.generation.is_some_and(|generation|generation!=scope_generation){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original ready window renewal changed its exact generation").into())}Ok(())}
}

/// 🔁️ Shared document replacement invalidates concrete window transients and retains displaced owners.
impl<A: ArtifactApp> PendingReserved<A> {
    pub fn commit_document_window_reset(&mut self, stores: &mut InstanceStores<A>, history: &mut CommandHistory<A>,replacement:&mut Option<WindowTransientOwnerRegistry>,scope_generation:u64,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        let admission=self.pending_document_window_admission.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"window publication has no original renewal admission"))?;admission.validate(scope_generation,cx)?;if admission.generation.is_none()||admission.committed{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"window publication requires one ready unused original renewal" ).into())}
        let replacement=replacement.take().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"document reset lost its retained replacement"))?;
        let generation=replacement.document_generation();let displaced=std::mem::replace(&mut stores.window_transient_store,replacement);if !displaced.terminal_is_empty(){stores.retired_window_transient_stores.insert_admitted(generation,displaced)}self.pending_document_window_admission.as_mut().unwrap().committed=true;history.cache=None;Ok(())
    }

    pub fn document_window_admission_demands(&self)->semio_framework_value::RetirementDemand{self.pending_document_window_admission.as_ref().map_or(Default::default(),|admission|admission.source.as_ref().map_or(semio_framework_value::RetirementDemand{depth:1,..Default::default()},semio_framework_job::OriginalCallerSourceLoan::retirement_demands))}

    pub fn close_document_window_admission(&mut self,cx:&mut semio_framework_job::StepContext<'_>,closing:bool)->Result<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let Some(admission)=self.pending_document_window_admission.as_mut()else{return Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(Default::default()))};if !closing&&!admission.committed{return Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::AwaitingInput{reason:"original window renewal awaits its candidate publication"})}
        if let Some(source)=admission.source.as_mut(){admission.returning=true;let step=source.close_step(cx.original_source_parent()?,cx)?;if source.terminal_is_empty(){admission.source.take();}return Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(step.progress()))}
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(Default::default()))}self.pending_document_window_admission.take();cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}))
    }
}

/// 🔁️ Shared document replacement invalidates concrete window transients and retains displaced owners.
impl<A: ArtifactApp> ToolOperations<A> {
    pub fn admit_document_window_reset(&mut self, pending: &mut PendingReserved<A>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if pending.pending_document_window_admission.is_none(){let Some(source)=cx.original_source_parent()?.admit_loan(cx)?else{return Ok(false)};*pending.pending_document_window_admission=Some(DocumentWindowResetAdmission{source:std::mem::ManuallyDrop::new(Some(source)),generation:None,committed:false,returning:false});return Ok(false)}
        let admission=pending.pending_document_window_admission.as_ref().unwrap();admission.validate(self.tool_cancellations.scope_generation(),cx)?;if admission.committed{return Ok(false)}if admission.generation.is_some(){return Ok(true)}
        if !self.tool_cancellations.renew_scope_generation(cx)?{return Ok(false)}pending.pending_document_window_admission.as_mut().unwrap().generation=Some(self.tool_cancellations.scope_generation());Ok(false)
    }
}
