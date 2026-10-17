//! 🪣️ Staged slice of the `command-routes` band of the mounted artifact instance: its `instance-jobs` members (WP6 step S0 of the instance family).
//!
//! 🧷️ Command routes of the mounted artifact instance: framework-reserved routes, action dispatch and command dispatch.
use semio_framework_plugin_ownership::bounded_ownership::{ARTIFACT_LIVE_OUTPUT_SLOTS, PluginLifecycleStep};
use semio_framework::{ActionKind, Fault, FaultCode, FaultFrom, FaultOrigin};
use semio_framework_plugin_decl_app::{ArtifactApp, ArtifactCompletionFault};
use semio_framework_plugin_decl_app::tool_contract::{ArtifactBoundedToolFault, ArtifactToolCompletion, ArtifactToolCompletionValue};
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_job::{ChildEmissionPreviewJob, agent_lane_uncarried_fault, drive_agent_lane_preview};
use semio_framework_plugin_instance_history::history_components::CommandHistory;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_job::reserved_jobs::{FrameworkReservedCommit, FrameworkReservedCommitProgress, FrameworkReservedCommitStage, PendingFrameworkReserved};
use std::future::Future;
use semio_framework_plugin_interaction::interaction_topology::INTERACTION_ACTION_IDS;
use semio_framework::kernel::InvocationResult;
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use crate::jobs_components::{PendingReserved, ToolOperations};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_ui_contract::*;

impl<A: ArtifactApp> PendingReserved<A> {
    /// 🖱️ Vortex pointermove is a storm of `interactionHover` (then a click `interactionSelect`)
    /// before the host finishes any Isolated reserved job. Exclusive `job % 64` slots made that
    /// storm `interactive-job.reserved-spawn-capacity`. Latest-wins: retire the same verb, and
    /// any other interaction occupying this hash slot, so hover preview and place can commit.
    pub fn retire_pending_reserved_latest_wins(&mut self, action: &str, job: u64) {
        if !INTERACTION_ACTION_IDS.contains(&action) {
            return;
        }
        let slot = job as usize % ARTIFACT_LIVE_OUTPUT_SLOTS;
        let mut ids = Vec::new();
        self.pending_reserved.each_id(|id| ids.push(id));
        let retire: Vec<u64> =
            ids.into_iter().filter(|&id| self.pending_reserved.get(id).is_some_and(|pending| pending.action == action || (id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS == slot && INTERACTION_ACTION_IDS.contains(&pending.action.as_str())))).collect();
        for id in retire {
            if let Some(pending) = self.pending_reserved.remove(id) {
                pending.permit.finish();
            }
        }
    }

    /// 🕰️ Hands one finished framework-reserved spawn-job to the app's commit queue. Nothing here
    /// suspends: the commit itself runs unit by unit in [`Self::step_framework_reserved_commit`].
    /// Answers `false` for a job this app never admitted.
    pub fn admit_framework_reserved_commit(&mut self, history: &mut CommandHistory<A>, job: u64, output: Result<Vec<u8>, Fault>) -> Result<bool, Fault> {
        let Some(pending) = self.pending_reserved.remove(job) else {
            return Ok(false);
        };
        if let Err(fault) = output {
            pending.permit.finish();
            return Err(fault);
        }
        if self.reserved_commits.len() >= ARTIFACT_LIVE_OUTPUT_SLOTS {
            pending.permit.finish();
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-commit-capacity"), format!("framework route '{}' found the reserved commit queue full", pending.action)));
        }
        let PendingFrameworkReserved { action, args, meta, permit } = pending;
        self.reserved_commits.push_back(FrameworkReservedCommit { action, args, meta, permit, log_generation_before: history.log_generation, stage: FrameworkReservedCommitStage::Validate, applied: 0, total: 1 });
        Ok(true)
    }

    /// 📊️ Progress of the commit at the head of the reserved-commit queue.
    pub fn framework_reserved_commit_progress(&self) -> Option<FrameworkReservedCommitProgress> {
        self.reserved_commits.front().map(|commit| FrameworkReservedCommitProgress { operation: commit.permit.operation.operation.0, applied: commit.applied, total: commit.total })
    }

    /// 🧾️ Takes the result of the last finished reserved commit.
    pub fn take_framework_reserved_commit_outcome(&mut self) -> Option<Result<InvocationResult, Fault>> {
        self.reserved_commit_outcome.take()
    }
}

impl<A: ArtifactApp> ToolOperations<A> {
    /// 📨️ Moves the original completion enum before validating its route-specific output kind.
    pub fn finish_mounted_reserved_completion(&mut self,id:u64){
        let mounted=self.tool_operations.get_mut(id).expect("original reserved completion retains its exact operation");
        let taken=mounted.completion.as_ref().map_or(Ok(None),ArtifactToolCompletion::take);
        let fault=match taken{
            Ok(Some(publication))=>{mounted.publication=Some(publication);match mounted.publication.as_ref().expect("original completion enum is retained"){
                ArtifactToolCompletionValue::Emit(Ok(_),_)=>None,
                ArtifactToolCompletionValue::Emit(Err(fault),_)=>Some(fault.bounded),
                ArtifactToolCompletionValue::Download(_,_)=>Some(ArtifactBoundedToolFault::from_fault(&plugin_sdk_fault("reserved producer completed with original download output"))),
            }},
            Ok(None)=>Some(ArtifactBoundedToolFault::from_fault(&plugin_sdk_fault("reserved producer has no completed original output"))),
            Err(fault)=>Some(ArtifactBoundedToolFault::from_fault(&fault)),
        };
        if let Some(fault)=fault{mounted.terminal_fault.get_or_insert(fault);mounted.cancellation_lease.as_ref().expect("original reserved completion lease").cancel();}
    }

    /// 🏷️ The kind an action or command `verb` is declared with; `None` for a verb this app does not declare.
    pub fn declared_action_kind(&self, verb: &str) -> Option<ActionKind> {
        self.registry.get(verb).map(|definition| definition.kind).or_else(|| self.registry.get_command(verb).map(|command| command.kind))
    }

    pub fn advance_agent_lane_preview(&mut self,verb:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<(Emit<A::Mutation,A::ConfigMutation,A::DraftMutation>,(usize,usize,usize),u64)>,semio_framework_job::OriginalTurnFailure>{
        if cx.should_yield(){return Ok(None)}
        let state=self.agent_lane_preview.as_mut().ok_or_else(||plugin_sdk_fault("original preview source is absent"))?;
        if state.starting||state.closing{return Ok(None)}
        if !state.complete{state.caller.as_ref().ok_or_else(||plugin_sdk_fault("original preview caller source is absent"))?.validate_context(cx)?;}
        if let Some(producer)=state.producer.as_mut(){let Some(steps)=(match drive_agent_lane_preview(producer,verb,cx){Ok(result)=>result,Err(failure)=>{state.closing=true;return Err(failure)}})else{return Ok(None)};state.steps=steps;state.producer.take();return Ok(None)}
        if let Some(children)=state.children.as_mut(){let Some(steps)=(match drive_agent_lane_preview(children,verb,cx){Ok(result)=>result,Err(failure)=>{state.closing=true;return Err(failure)}})else{return Ok(None)};state.steps=state.steps.saturating_add(steps);state.children.take();state.complete=true;return Ok(None)}
        if !state.complete{
            let completion=state.completion.as_ref().ok_or_else(||plugin_sdk_fault("original preview completion source is absent"))?;
            match completion.take()?{
                Some(ArtifactToolCompletionValue::Emit(emit,ephemeral))=>{let emit=emit.map_err(ArtifactCompletionFault::into_fault)?;let job=ChildEmissionPreviewJob::<A>{emit:Some(emit),ephemeral:Some(ephemeral),completion:Some(completion.clone()),retirement:None,retirement_payload:None,retirement_fault:None,rejection:None,completion_retirement:None,maximum_bytes:state.maximum_bytes,wire_mode:true,fault_publication:semio_framework_job_fault_publication::RetainedFaultPublication::new(),closing:false};state.children=Some(semio_framework_plugin_job::OriginalPreviewDrive::new(job,state.config,cx));return Ok(None)},
                Some(value@ArtifactToolCompletionValue::Download(..))=>{state.publication=Some(value);state.complete=true;},
                None=>return Err(plugin_sdk_fault("preview producer completed without its original output").into()),
            }
        }
        if state.publication.is_none(){state.publication=state.completion.as_ref().ok_or_else(||plugin_sdk_fault("original preview completion source is absent"))?.take()?;return Ok(None)}
        if state.completion.is_some()||state.completion_close.is_some(){let step=semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut state.completion,&mut state.completion_close,cx.retained_grant())?;cx.consume_retained(step.progress())?;return Ok(None)}
        if let Some(owner)=state.lease_close.as_mut(){let step=if owner.terminal_is_empty(){semio_framework_plugin_operation::original_lease_cursor_release(&mut state.lease_close,cx.retained_grant()).map(RetainedCloneStep::Progress)?}else{owner.close_step(&self.tool_cancellations,cx.retained_grant())?};cx.consume_retained(step.progress())?;return Ok(None)}
        if state.lease.is_some(){let(owner,progress)=semio_framework_plugin_operation::original_lease_cursor_admit(&mut state.lease,cx.retained_grant())?;state.lease_close=owner;cx.consume_retained(progress)?;return Ok(None)}
        if let Some(caller)=state.caller.as_mut(){caller.close_step(cx.original_source_parent()?,cx)?;if caller.terminal_is_empty(){state.caller.take();}return Ok(None)}
        if cx.retained_grant().maximum_items<2||cx.retained_grant().maximum_depth==0{return Ok(None)}
        cx.consume_retained(RetainedCloneProgress{copied_items:2,..Default::default()})?;
        state.key.take();
        let state=self.agent_lane_preview.take().expect("original preview publication owner remains exact");
        match state.publication{
            Some(ArtifactToolCompletionValue::Emit(emit,ephemeral))=>emit.map(|emit|Some((emit,(ephemeral.presence.len(),ephemeral.transient.len(),ephemeral.window_transient.len()),state.steps))).map_err(|source|ArtifactCompletionFault::into_fault(source).into()),
            Some(ArtifactToolCompletionValue::Download(..))=>Err(agent_lane_uncarried_fault(verb,"a file download").into()),
            None=>Err(plugin_sdk_fault("preview closed without its original publication").into()),
        }
    }

    pub fn close_agent_lane_preview(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let Some(state)=self.agent_lane_preview.as_mut()else{return Ok(PluginLifecycleStep::Complete(Default::default()))};
        if state.terminal_is_empty(){if grant.maximum_items==0||grant.maximum_depth==0||cx.should_yield(){return Ok(PluginLifecycleStep::Progress(Default::default()))}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;self.agent_lane_preview.take();return Ok(PluginLifecycleStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))}
        state.close_step(cx,self.tool_cancellations.handle())
    }
}
