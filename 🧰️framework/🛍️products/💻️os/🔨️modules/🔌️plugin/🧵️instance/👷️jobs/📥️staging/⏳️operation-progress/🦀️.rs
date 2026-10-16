//! 🛖️ Staged slice of the `operation-progress` band of the mounted artifact instance: its `instance-jobs` members (WP6 step S0 of the instance family).
//!
//! ⏳️ The instance's operation-progress publication.
use super::*;
use crate::tool_jobs::operation_progress::*;

impl<A: ArtifactApp> ToolOperations<A> {
    pub(crate) fn operation_progress_snapshot(&self) -> Vec<ArtifactOperationProgress> {
        let mut result = Vec::new();
        self.tool_operations.each_id(|id| {
            let Some(operation) = self.tool_operations.get(id) else { return };
            let Some(completed_units) = operation.progress.filter(|_| !operation.terminal_seen) else { return };
            let Some(definition) = self.registry.get(&operation.verb) else { return };
            result.push(ArtifactOperationProgress {
                operation_id: operation.operation.operation.0,
                generation: operation.operation.generation.0,
                label: definition.label.clone(),
                completed_units,
                cancelling: operation.cancellation_lease.as_ref().is_some_and(|lease| lease.cancel_token().is_cancelled_now()),
            });
        });
        result
    }

    /// ⏳️ The app's declared operation-progress scope ([`ArtifactApp::operation_progress_scope`]) once an operation's
    /// progress changed or it retired; `None` when nothing changed or the app renders no progress.
    pub(crate) fn take_operation_progress_scope(&mut self) -> Option<UiDirtyScope> {
        let mut changed = std::mem::take(&mut self.operation_progress_retired);
        for index in 0..ARTIFACT_LIVE_OUTPUT_SLOTS {
            if let Some((_, operation)) = self.tool_operations.entry_mut(index) {
                changed |= std::mem::take(&mut operation.progress_pending);
            }
        }
        operation_progress_dirty_scope(changed, A::operation_progress_scope)
    }

    pub(crate) async fn dispatch_operation_cancellation(&mut self, args: Option<&DslValue>, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
        let (id, generation) = args.and_then(decode_operation_cancellation).ok_or_else(|| plugin_sdk_fault("operation cancellation requires exact operationId and generation"))?;
        let operation = self.tool_operations.get_mut(id).ok_or_else(|| plugin_sdk_fault("operation cancellation target is no longer live"))?;
        if operation.meta.instance_id != meta.instance_id || operation.meta.actor != meta.actor || operation.operation.generation.0 != generation {
            return Err(plugin_sdk_fault("operation cancellation authority does not match"));
        }
        if !operation_cancellation_target_admitted(true, operation.terminal_seen, operation.cancellation_lease.is_some()) {
            return Err(plugin_sdk_fault("operation cancellation target is no longer cancellable"));
        }
        operation.user_cancel_requested = true;
        operation.cancellation_lease.as_ref().expect("admitted cancellation has a lease").cancel();
        operation.progress_pending = true;
        Ok(ToolOperations::<A>::empty_result(CANCEL_TYPED_OPERATION_ACTION_ID, meta, Vec::new(), Vec::new(), A::operation_progress_scope()).await)
    }
}
