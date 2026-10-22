//! 🏅️ Staged `retirement` members of the `instance-typed-ops` use cases, as methods of the borrowed bundle (WP7 of the instance family).
//!
//! ♻️ Typed operation retirement under the granted lifecycle step.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin};
use std::future::Future;
use semio_framework_plugin_job::{INTERACTIVE_TURN_WORKER_PUMPS, INTERACTIVE_TURN_WORKER_WALL_US};
use semio_framework_plugin_instance_extension::InstanceExtension;
use store::{MemberFactory, Mutation, ReferenceSpaceMember};
use semio_framework_plugin_retained_command::typed_operation::MountedTypedCommandFullOperation;
use semio_framework_plugin_composition_publication::MountedTypedCommandFullOperationStage;
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_ownership::bounded_ownership::{PluginLifecycleStep, plugin_grant_funds, plugin_retirement_fault};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use crate::typed_bundle::TypedCtx;
use semio_framework_value::ValueError;
use semio_framework_ui_contract::*;
use semio_framework_plugin_instance_docs::docs_maintenance_ladder::item;

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> TypedCtx<'ctx, A, M, X> {
    #[inline(never)]
    pub fn retire_typed_operation_unit(&mut self,operation_id:u64,cx:&mut semio_framework_job::StepContext<'_>)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let grant=cx.retained_grant();let demand=self.tools.typed_operation_retirement_demands(&*self.child_composition, operation_id, grant.maximum_copy_bytes)?;
        semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,actual|self.retire_typed_operation_granted(operation_id,same,actual).map(|step|(step,false)))
    }

    fn retire_typed_operation_granted(&mut self,operation_id:u64,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        self.as_render().request_gesture_cancellation(operation_id);
        let item = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        if self.tools.tool_operations.get(operation_id).is_some_and(MountedTypedCommandFullOperation::close_fault_pending){return self.tools.tool_operations.get_mut(operation_id).unwrap().close_fault_step(grant).map(|step|PluginLifecycleStep::Progress(step.progress())).map_err(Into::into);}
        if let Some(group) = self.tools.tool_operations.get(operation_id).and_then(|operation| operation.owned_child_group) {
            if self.child_composition.private_child_groups.get(group).is_some() {
                return Ok(match self.child_composition.close_private_child_group_operation_step(&mut *self.stores, group, cx, grant)? {
                    semio_framework_job::InteractiveJobCloseStep::Pending { progress } | semio_framework_job::InteractiveJobCloseStep::Complete { progress } => PluginLifecycleStep::Progress(progress),
                    semio_framework_job::InteractiveJobCloseStep::Blocked => PluginLifecycleStep::Blocked { reason: "private child group close awaits its owner" },
                });
            }
            if grant.maximum_items == 0 { return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress::default())); }
            self.tools.tool_operations.get_mut(operation_id).unwrap().owned_child_group = None;
            return Ok(PluginLifecycleStep::Progress(item));
        }
        if self.tools.tool_operations.get(operation_id).is_some_and(MountedTypedCommandFullOperation::lease_retirement_ready){let step=self.tools.tool_operations.get_mut(operation_id).unwrap().lease_retirement_step(&self.tools.tool_cancellations,grant).map_err(plugin_retirement_fault)?;return Ok(PluginLifecycleStep::Progress(step.progress()));}
        self.tools.record_settled_typed_operation_command(&mut *self.history, &mut *self.host_events, operation_id);
        let operation = self
            .tools.tool_operations
            .get_mut(operation_id)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-retirement-authority"), "typed operation owner changed during one bounded retirement unit"))?;
        let step = operation.retirement_step(cx)?;
        let PluginLifecycleStep::Complete(progress) = step else { return Ok(step) };
        if !self.tools.tool_operations.get(operation_id).is_some_and(MountedTypedCommandFullOperation::terminal_is_empty) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-terminal"), "typed operation reported Complete without exact terminal emptiness").into());
        }
        let operation =
            self.tools.tool_operations.remove(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-retirement-authority"), "terminal typed operation changed before exact removal"))?;
        self.tools.operation_progress_retired |= operation.progress.is_some();
        drop(operation);
        self.tools.typed_inline_interaction_verbs.retain(|(operation, _)| *operation != operation_id);
        Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress }))
    }

    /// ♻️ Drives ONE retiring typed operation for a bounded slice of this turn, the exact twin of the
    /// slice [`Self::drive_typed_operation_worker`] gives a Worker-stage operation — same ladder, same
    /// owner, same `INTERACTIVE_TURN_WORKER_PUMPS`/`INTERACTIVE_TURN_WORKER_WALL_US` bound. Without it a
    /// `Retiring` operation answered `has_runnable_typed_operations` forever while the turn driver advanced nothing
    /// for it: the slot was released only by `maintenance_step` stage 0 of [`MAINTENANCE_STAGES`], one
    /// unit per rotation on the shared live-cleanup job, so a sustained action rate filled all
    /// [`ARTIFACT_LIVE_OUTPUT_SLOTS`] classes and every later action was refused
    /// `interactive-job.typed-operation-capacity`.
    pub(super) fn retire_typed_operation_run(&mut self,operation_id:u64,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let started_us=cx.now_us();
        for _ in 0..INTERACTIVE_TURN_WORKER_PUMPS {
            if !self.tools.tool_operations.get(operation_id).is_some_and(|operation| operation.stage == MountedTypedCommandFullOperationStage::Retiring) {
                return Ok(());
            }
            if self.tools.tool_operations.get(operation_id).is_some_and(|operation|operation.granted_retirement_ready()&&operation.granted_retirement_pending()){
                let grant=semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(self.mounted_policy.close,cx.retained_grant());
                let demand=self.tools.tool_operations.get(operation_id).unwrap().granted_retirement_demands(grant.maximum_copy_bytes)?;let step=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,actual|{let step=self.tools.tool_operations.get_mut(operation_id).unwrap().granted_retirement_step(same,actual)?;Ok((PluginLifecycleStep::Progress(step.progress()),false))})?;let step=RetainedCloneStep::Progress(step.progress().unwrap_or_default());
                if !step.progress().fits(grant){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original typed operation retirement exceeded its issuer grant").with_retained_progress(step.progress()).into());}
                let progress=step.progress();if progress==RetainedCloneProgress::default(){return Ok(());}
                let owner=self.tools.tool_operations.get_mut(operation_id).unwrap();assert!(owner.original_retirement_receipt.is_none(),"original mounted Close receipt cannot be overwritten before acknowledgement");owner.original_retirement_receipt=Some((grant,progress));return Ok(());

            }
            let grant = semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(self.mounted_policy.close, cx.retained_grant());
            let demand = self.tools.typed_operation_retirement_demands(&*self.child_composition, operation_id, grant.maximum_copy_bytes)?;
            if !plugin_grant_funds(demand, grant) {
                return Ok(());
            }
            match cx.with_retained_loan(grant,|same|self.retire_typed_operation_unit(operation_id,same).map_err(Into::into))? {
                PluginLifecycleStep::Blocked { .. } | PluginLifecycleStep::AwaitingInput { .. } => return Ok(()),
                PluginLifecycleStep::Progress(progress) | PluginLifecycleStep::Complete(progress) => {
                    if !progress.fits(grant) {
                        return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"typed operation retirement exceeded its issuer grant").with_retained_progress(progress).into());
                    }
                    if progress == RetainedCloneProgress::default() {
                        return Ok(());
                    }
                }
            }
            if cx.should_yield()||started_us.is_some_and(|started_us|cx.now_us().is_some_and(|now_us|now_us.saturating_sub(started_us)>=INTERACTIVE_TURN_WORKER_WALL_US)) {
                return Ok(());
            }
        }
        Ok(())
    }
}
