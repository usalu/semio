//! ⏯️ OS runtime of an interactive, transactional tool run with a visible process: the per-instance
//! [`ToolRunLedger`] (provisional document ops folded into an overlay the renderer reads as the document), the
//! [`ToolRunDriver`] turn stepping run and revalidate jobs, the one-edit finalize publication and the
//! framework ToolRun panel body.
//!
//! Contract: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/📋️tool-run-contract.md`
//! §2.7, §2.8, §3.3, §4.1 (layers 1 and 3). Domain-neutral types: `semio_framework_tool_run`.
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;
use semio_framework_plugin_decl_views::tool_run_contract::FRAMEWORK_TOOL_RUN_BODY_KEY;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunJobPurpose;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunJob;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunTraceKeys;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunJobPort;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunView;

use semio_framework_plugin_decl_app::ArtifactApp;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{ColdMutation, Mutation, MutationDiff};
use protocol::{OpBinary, OpText};
#[cfg(test)]
use semio_framework_plugin_reactor_app::PluginApp;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_plugin_interaction::ui_assembly_error;
use semio_framework_plugin_window_transient::{paged_text_carrier, ui_assembly_error_because};
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_ui_contract::*;
use semio_framework_tool_run::{tool_run_format, ToolRunAction, ToolRunCounter, ToolRunDefinition, ToolRunEffect, ToolRunEvent, ToolRunId, ToolRunIdentity, ToolRunLabel, ToolRunLane, ToolRunMachine, ToolRunRebasePolicy, ToolRunRejection, ToolRunSlot, ToolRunState, ToolRunStep, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTick, ToolRunTraceCursor, ToolRunTraceDelta, ToolRunTraceOp, ToolRunTraceStore, ToolRunVerdict, TOOL_RUN_ARG_GENERATION, TOOL_RUN_ARG_RUN_ID, TOOL_RUN_ARG_TOOL_ID, TOOL_RUN_MEMBER_OPS_MAX, TOOL_RUN_PROVISIONAL_OPS_MAX, TOOL_RUN_REASON_PROVISIONAL_CAP, TOOL_RUN_REASON_TRACE_TRUNCATED, TOOL_RUN_STATUS_ANNOUNCE_INTERVAL_MS};
use store::ErasedSnapshotRetirement;
use semio_framework_tool_run::{
    
    
    TOOL_RUN_ACTION_IDS, 
    
};
use semio_framework_ui_scene::{scene_lane_hash, Board2dScene, Board2dSceneLane, Canvas2dScene, Canvas2dSceneLane, SceneDoc, SceneLaneRef, World3dScene, World3dSceneLane};
use std::collections::BTreeMap;
use std::sync::Arc;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, admit_retained_clone_close};
//#region 🔖️Limits

#[path = "⏱️limits-and-job-slot/🦀️.rs"]
mod limits_and_job_slot;
pub use limits_and_job_slot::*;

#[path="🎟️publication/🦀️.rs"]
pub mod publication_ingress;
use crate::publication_ingress::ToolRunPublicationIngress;
#[path="👷️job/🎟️receiving/🦀️.rs"]
mod original_job_receiving;
pub(crate) use crate::original_job_receiving::OriginalToolRunPayload;
//#endregion 🔖️Limits

//#region 🔖️Selection
/// 🎯️ The run the ledger currently operates on (`ToolRunLedger::selected`), borrowing only `entries`.
#[macro_export]
#[doc(hidden)]
macro_rules! selected_entry {
    ($ledger:expr) => {
        $ledger.selected.and_then(|index| $ledger.entries.get(index)).map(|entry| &**entry)
    };
}

/// 🎯️ The run the ledger currently operates on, mutably, borrowing only `entries`.
#[macro_export]
#[doc(hidden)]
macro_rules! selected_entry_mut {
    ($ledger:expr) => {
        match $ledger.selected {
            Some(index) => $ledger.entries.get_mut(index).map(|entry| &mut **entry),
            None => None,
        }
    };
}

#[path="🎞️tick/📦️payload/🦀️.rs"]
pub mod original_tick_payload;
#[cfg(test)]
pub use crate::original_tick_payload::{advance_original_tick_payload, close_original_tick_payload};

#[cfg(test)]
#[path="📮️port/♻️retirement/🧪️tests/🦀️.rs"]
mod original_port_retirement_tests;
#[cfg(test)]
#[path="📮️port/🎟️admission/🧪️tests/🦀️.rs"]
mod original_port_admission_tests;

//#endregion 🔖️Selection

#[path = "🔁️refold-finalize-and-entry/🦀️.rs"]
mod refold_finalize_and_entry;
pub use refold_finalize_and_entry::*;

#[path = "📒️driver-and-ledger/🦀️.rs"]
mod driver_and_ledger;
pub use driver_and_ledger::*;

#[path = "👥️member-owners-and-visitors/🦀️.rs"]
mod member_owners_and_visitors;
pub use member_owners_and_visitors::*;

#[path = "🏷️action-outcome-and-labels/🦀️.rs"]
mod action_outcome_and_labels;
pub use action_outcome_and_labels::*;

#[path = "🪧️panel-groups/🦀️.rs"]
mod panel_groups;
use panel_groups::*;

#[cfg(test)]
#[path="♻️retirement/🧪️tests/🦀️.rs"]
mod granted_ledger_retirement_tests;

pub use semio_framework_plugin_job::lifted_job::*;

#[path = "../🛠️tool-machine/🦀️.rs"]
pub mod tool_jobs_tool_machine;
