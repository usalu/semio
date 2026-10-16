//! 🪻️ Staged slice of the `tool-run` band of the mounted artifact instance: its `instance-jobs` members (WP6 step S0 of the instance family).
//!
//! ⏯️ The mounted instance's tool-run turns (ledger driving, tick folding and finalize publication) and the
//! [`ToolRunExtension`] that registers them with the instance through [`InstanceExtension`].
use super::*;
use crate::tool_jobs::tool_run::*;
use semio_framework_tool_run::{
    tool_run_pointer_value, ToolRunAction, ToolRunEffect, ToolRunEvent, ToolRunId, ToolRunIdentity, ToolRunLane, ToolRunMachine, ToolRunRebasePolicy, ToolRunReconfigurePolicy,
    ToolRunRejection, ToolRunSettingsReads, ToolRunState, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTick, ToolRunTraceStore, TOOL_RUN_ARG_GENERATION, TOOL_RUN_ARG_RUN_ID, TOOL_RUN_ARG_TOOL_ID, TOOL_RUN_ARG_WINDOW_ID, TOOL_RUN_REASON_CONFLICT, TOOL_RUN_REASON_REBASING, TOOL_RUN_TICK_BYTES_MAX,
};
use std::sync::Arc;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_close}};
use crate::artifact_instance::InstanceExtension;
use crate::artifact_instance::VcsArtifactApp;
use crate::app_emission::ActionMeta;
use crate::child_view::MemberKeyRef;
use crate::tool_run_contract::ToolRunJobRequest;
use semio_framework::{ActionKind, ViewModel};
use store::{os_store::store_schema::HistoryLane, MemberFactory, SpaceMember};
use semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use crate::tool_run_contract::ToolRunJobPurpose;

impl<A: ArtifactApp> ToolOperations<A> {
    /// 🧵️ Builds the run's job for `purpose`: the app's retargetable job when it builds one, else its plain job.
    pub(crate) fn build_tool_run_job_slot<M: SpaceMember + MemberFactory + 'static>(&self, stores: &InstanceStores<A>, child_composition: &ChildComposition<M>, entry: &ToolRunEntry<A>, purpose: ToolRunJobPurpose) -> Result<Option<ToolRunJobSlot<A::Config>>, Fault> {
        let request = || ToolRunJobRequest::<A> {
            receiving: &entry.definition.receiving,
            tool_id: &entry.tool_id,
            definition: &entry.definition,
            purpose,
            identity: entry.identity,
            snapshot: match purpose {
                ToolRunJobPurpose::Run => Arc::clone(&entry.base),
                ToolRunJobPurpose::Revalidate => stores.store.snapshot_owner(),
            },
            config: stores.config_store.snapshot_owner(),
            window_id: entry.window.as_ref().map(|(window_id, _)| window_id.as_str()),
            window_config: entry.window.as_ref().and_then(|(window_id, window_kind_id)| stores.window_config_store.snapshot(window_kind_id, window_id)),
            checkpoint: entry.checkpoint.as_deref(),
            provisional: &entry.provisional,
            instance_owner: self.instance_operation_owner.clone(),
            port: entry.port.clone(),
            trace_keys: entry.trace_keys.clone(),
            entity_marks: &entry.entity_marks,
            children: ChildContentView::clone(&child_composition.child_content_root),
            member_ops: entry.member.as_ref().map_or(&[], |member| member.ops.as_slice()),
        };
        let generation = entry.identity.generation;
        if let Some(job) = A::build_retargetable_tool_run_job(request())? {
            return Ok(Some(ToolRunJobSlot::new(ToolRunJobHandle::Retargetable(job), purpose, generation, entry.definition.receiving)));
        }
        Ok(A::build_tool_run_job(request())?.map(|job| ToolRunJobSlot::new(ToolRunJobHandle::Plain(job), purpose, generation, entry.definition.receiving)))
    }

    /// 🏷️ The history row label of a transaction a finalized run published (`<appId>#<toolId>` of a declared tool
    /// run): the tool's own label in every locale, so the row reads the same after a reload.
    pub(crate) fn tool_run_transaction_label(&self, app_id: &str, transaction: &protocol::TransactionRef) -> Option<LocalizedLabel> {
        let tool_id = transaction.tool.strip_prefix(app_id)?.strip_prefix('#')?;
        self.registry.tool_run(tool_id).map(|(label, _)| label.clone())
    }
}
