//! ⏯️ The mounted instance's tool-run turns (ledger driving, tick folding and finalize publication) and the
//! [`ToolRunExtension`] that registers them with the instance through [`InstanceExtension`].
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;
use semio_framework_tool_run::{ToolRunAction, ToolRunEffect, ToolRunEvent, ToolRunId, ToolRunIdentity, ToolRunLane, ToolRunMachine, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunRejection, ToolRunState, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTraceStore, TOOL_RUN_ARG_GENERATION, TOOL_RUN_ARG_RUN_ID, TOOL_RUN_ARG_TOOL_ID, TOOL_RUN_ARG_WINDOW_ID, TOOL_RUN_REASON_CONFLICT, TOOL_RUN_REASON_REBASING};
use std::sync::Arc;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, admit_retained_clone_close};
use semio_framework_plugin_decl_app::ArtifactApp;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_decl_views::tool_run_contract::{FRAMEWORK_TOOL_RUN_BODY_KEY, ToolRunJobPort, ToolRunJobPurpose, ToolRunRetargetableJob, ToolRunTraceKeys};
use semio_framework::FaultFrom;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{Mutation, MutationDiff};
use protocol::{OpBinary, OpText};
#[cfg(test)]
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_ui_scene::SceneDoc;
use semio_framework_plugin_tool_run::{TOOL_RUN_JOB_SITE, TOOL_RUN_PANEL_TRACE_ROWS, TOOL_RUN_TRACE_DELTA_BYTES, ToolRunActionOutcome, ToolRunEntry, ToolRunFinalize, ToolRunFinalizePhase, ToolRunJobHandle, ToolRunLedger, ToolRunMemberEmissionOwner, ToolRunMemberEmissionRetire, ToolRunMemberEmit, ToolRunMemberGeneration, ToolRunMemberOwners, ToolRunMemberRetire, ToolRunMemberRun, ToolRunRefoldTurn, inject_tool_run_trace_lane_into, selected_entry, selected_entry_mut, tool_grant_funds, tool_run_arg_u64, tool_run_scene_surface};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_plugin_job::is_tool_run_action_id;
use semio_framework_ui_contract::*;
use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework::{ActionKind, ViewModel};
use store::os_store::store_schema::HistoryLane;
use store::{MemberFactory, SpaceMember};
use semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;

use semio_framework_plugin_instance_jobs::jobs_components::ToolOperations;
use semio_framework_plugin_instance_extension::{ExtensionCtx, ExtensionRef};
use crate::tool_run_bundle::{ToolRunCtx, ToolRunRef};
use semio_framework_plugin_time_travel::TimeTravelLedger;

#[path = "🚀️action-dispatch-and-start/🦀️.rs"]
mod action_dispatch_and_start;

#[path = "👀️generation-watch-and-turn-driver/🦀️.rs"]
mod generation_watch_and_turn_driver;

#[path = "🎟️job-admission-and-finalize-turn/🦀️.rs"]
mod job_admission_and_finalize_turn;

#[path = "📤️publication-and-member-refresh/🦀️.rs"]
mod publication_and_member_refresh;

#[path = "🧹️member-retirement-and-close/🦀️.rs"]
mod member_retirement_and_close;

#[path = "🔌️ref-reads-and-extension/🦀️.rs"]
mod ref_reads_and_extension;
pub use ref_reads_and_extension::*;

#[path = "📥️staging/🧺️bundle/🦀️.rs"]
pub mod tool_run_bundle;
