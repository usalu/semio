//! 🪼️ Staged slice of the `tool-run` band of the mounted artifact instance: its `instance-stores` members (WP6 step S0 of the instance family).
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

impl<A: ArtifactApp> InstanceStores<A> {
    /// 🎚️ What the run's declared settings reads name right now. A declared window kind that is the starting
    /// window's reads only that window's config, the one its jobs receive; any other declared kind reads every
    /// partition of that kind.
    pub(crate) fn tool_run_settings_values(&self, reads: &ToolRunSettingsReads, window: Option<&(String, String)>) -> ToolRunSettingsValues {
        let config = if reads.config.is_empty() {
            Vec::new()
        } else {
            let document = semio_framework_value::ToValue::to_value(self.config_store.snapshot_owner().as_ref());
            reads.config.iter().map(|pointer| tool_run_pointer_value(&document, pointer).cloned()).collect()
        };
        let window_config = reads
            .window_config
            .iter()
            .flat_map(|(window_kind_id, pointers)| {
                let started = window.filter(|(_, kind)| kind == window_kind_id).map(|(window_id, _)| window_id.as_str());
                self.window_config_store.pointer_values(window_kind_id, pointers).into_iter().filter(move |(window_id, _)| started.is_none_or(|started| started == window_id)).map(move |(window_id, values)| (window_kind_id.clone(), window_id, values))
            })
            .collect();
        ToolRunSettingsValues { config, window_config }
    }
}
