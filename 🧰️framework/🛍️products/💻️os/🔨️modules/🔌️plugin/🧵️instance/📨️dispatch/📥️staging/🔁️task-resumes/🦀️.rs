//! 🔁️ Staged `task-resumes` members of the `instance-dispatch` use cases: the resolved task follow-ups are delivered through the runtime-command lane one funded step per publication unit.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_command::command_admission::original_typed_command::OriginalTypedCommandSource;
use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework_plugin_command::command_admission::original_typed_command::TaskResumeStep;
use crate::dispatch_bundle::DispatchCtx;
use semio_framework::{Fault, FaultCode, FaultOrigin};
use semio_framework_value::RetainedCloneProgress;
use store::{MemberFactory, ReferenceSpaceMember};

/// 🛑️ The fault a refused delivery answers its task with, when the failure owes the turn nothing else: one that carries partial retained progress,
/// a receipt refusal or a context refusal keeps travelling as the turn's own failure so its accounting is never dropped.
fn delivery_fault(failure: &semio_framework_job::OriginalTurnFailure) -> Option<Fault> {
    if failure.retained_progress() != RetainedCloneProgress::default() || failure.receipt_refusal.is_some() || failure.context_refusal.is_some() {
        return None;
    }
    Some(failure.primary_fault().cloned().unwrap_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.task-resume-delivery"), format!("a resolved task follow-up could not be delivered: {}", failure.primary.message()))))
}

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> DispatchCtx<'ctx, A, M, X> {
    /// 🔁️ One funded step of the task-resume queue through the instance's own typed source: the front is produced, admitted and mounted like any
    /// typed command, its receipt is left for the host ([`TaskResumes::take_outcome`](semio_framework_plugin_command::command_admission::original_typed_command::TaskResumes::take_outcome)),
    /// and a cancelled or failed front returns every owner through paid closure before the next one starts.
    pub async fn task_resume_step(&mut self, source: &mut OriginalTypedCommandSource<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
        match self.host_events.resumes.plan(cx.is_cancelled(), source.pending(), self.host_events.delivering) {
            TaskResumeStep::Idle => Ok(()),
            TaskResumeStep::Close => cx.with_original_retirement(|cx| source.close_step(cx)).map(drop),
            TaskResumeStep::Drive => self.drive_task_resume(source, identity, cx).await,
        }
    }

    /// 🏃️ Drives the queue front: its bytes decode into the typed command only once the producer is funded to take it.
    async fn drive_task_resume(&mut self, source: &mut OriginalTypedCommandSource<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
        let (mut bytes, mut meta) = self.host_events.resumes.lend();
        let mut tag = None;
        let driven = self.drive_runtime_typed_command(source, || bytes.take().and_then(|bytes| <A::Command as protocol::OpBinary>::decode_op(&bytes).ok()).map(Box::new), &mut tag, &mut meta, identity, cx).await;
        match driven {
            Ok(Some(receipt)) => self.host_events.resumes.delivered(Ok(receipt)),
            Ok(None) => self.host_events.resumes.restore(bytes, meta),
            Err(failure) => match delivery_fault(&failure) {
                Some(fault) => self.host_events.resumes.failed(Some(Err(fault))),
                None => {
                    self.host_events.resumes.failed(None);
                    return Err(failure);
                }
            },
        }
        Ok(())
    }
}
