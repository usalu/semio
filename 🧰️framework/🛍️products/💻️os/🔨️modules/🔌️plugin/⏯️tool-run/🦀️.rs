//! ⏯️ OS runtime of an interactive, transactional tool run with a visible process: the per-instance
//! [`ToolRunLedger`] (provisional document ops folded into an overlay the renderer reads as the document), the
//! [`ToolRunDriver`] turn stepping run and revalidate jobs, the one-edit finalize publication and the
//! framework ToolRun panel body.
//!
//! Contract: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/📋️tool-run-contract.md`
//! §2.7, §2.8, §3.3, §4.1 (layers 1 and 3). Domain-neutral types: `semio_framework_tool_run`.
extern crate semio_os_kernel as store;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as pack;
use semio_plugin_window_transient::{ui_assembly_error,ui_assembly_error_because,paged_text_carrier};
use semio_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_plugin_decl_views::tool_run_contract::FRAMEWORK_TOOL_RUN_BODY_KEY;
use semio_plugin_decl_views::tool_run_contract::ToolRunJobPurpose;
use semio_plugin_decl_views::tool_run_contract::ToolRunJob;
use semio_plugin_decl_views::tool_run_contract::ToolRunTraceKeys;
use semio_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_plugin_decl_views::tool_run_contract::ToolRunJobPort;
use semio_plugin_decl_views::tool_run_contract::ToolRunView;

use semio_plugin_decl_app::ArtifactApp;
#[cfg(test)]
use semio_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
#[cfg(test)]
use semio_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use semio_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{ColdMutation, Mutation, MutationDiff};
use protocol::{OpBinary, OpText};
#[cfg(test)]
use semio_plugin_reactor_app::PluginApp;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_plugin_window_config::WindowConfigOwner;
use semio_ui_contract::*;
use semio_framework_tool_run::{tool_run_format, ToolRunAction, ToolRunCounter, ToolRunDefinition, ToolRunEffect, ToolRunEvent, ToolRunId, ToolRunIdentity, ToolRunLabel, ToolRunLane, ToolRunMachine, ToolRunRebasePolicy, ToolRunRejection, ToolRunSlot, ToolRunState, ToolRunStep, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTick, ToolRunTraceCursor, ToolRunTraceDelta, ToolRunTraceOp, ToolRunTraceStore, ToolRunVerdict, TOOL_RUN_ARG_GENERATION, TOOL_RUN_ARG_RUN_ID, TOOL_RUN_ARG_TOOL_ID, TOOL_RUN_MEMBER_OPS_MAX, TOOL_RUN_PROVISIONAL_OPS_MAX, TOOL_RUN_REASON_PROVISIONAL_CAP, TOOL_RUN_REASON_TRACE_TRUNCATED, TOOL_RUN_STATUS_ANNOUNCE_INTERVAL_MS};
use store::ErasedSnapshotRetirement;
use semio_framework_tool_run::{
    
    
    TOOL_RUN_ACTION_IDS, 
    
};
use semio_plugin_tool_run_operations::tool_erased_retirement_demand;
use semio_ui_scene::{scene_lane_hash, Board2dScene, Board2dSceneLane, Canvas2dScene, Canvas2dSceneLane, SceneDoc, SceneLaneRef, World3dScene, World3dSceneLane};
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

/// 🚨️ The typed fault of a turn step that needs the selected run after the ledger lost it.
pub fn tool_run_slot_lost(step:&'static str)->semio_framework::Fault{plugin_sdk_fault(format!("tool run {step} requires its selected run, which the ledger no longer holds"))}

/// 🎯️ The selected run mutably, or the typed [`tool_run_slot_lost`] fault naming the step.
#[macro_export]
#[doc(hidden)]
macro_rules! selected_entry_required {
    ($ledger:expr,$step:expr) => {
        $crate::selected_entry_mut!($ledger).ok_or_else(||$crate::tool_run_slot_lost($step))
    };
}

#[cfg(test)]
mod selected_entry_required_tests {
    struct Ledger{selected:Option<usize>,entries:Vec<Box<u8>>}
    fn fault(ledger:&mut Ledger)->Result<u8,semio_framework::Fault>{selected_entry_required!(ledger,"probe").map(|entry|*entry)}
    #[test]
    fn selection_resolves_the_selected_run(){assert_eq!(fault(&mut Ledger{selected:Some(1),entries:vec![Box::new(7),Box::new(9)]}).ok(),Some(9));}
    #[test]
    fn missing_selection_is_a_typed_fault(){let error=fault(&mut Ledger{selected:None,entries:vec![Box::new(7)]}).unwrap_err();assert!(error.message.contains("probe"),"{}",error.message);}
    #[test]
    fn dangling_selection_is_a_typed_fault(){assert!(fault(&mut Ledger{selected:Some(3),entries:vec![Box::new(7)]}).is_err());}
}

/// 🧊️ Cold retirement of one shared original: the last alias is elected and its plain value dropped; never a bare drop of the owner.
pub(crate) fn retire_cold_shared<T>(mut owner:semio_framework_value::retirement::arc_custody::original::SharedOriginalStorage<T>){if let Some(mut original)=owner.elect_cold(){while !original.physical_is_unique(){std::hint::spin_loop()}drop(original.take_value());}}
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

pub use semio_plugin_job::lifted_job::*;


#[path="🎞️tick/🛬️native/🦀️.rs"]
pub mod original_native_tick;


