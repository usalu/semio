//! ⏪️ OS runtime of non-destructive history editing: the per-instance [`TimeTravelLedger`] hosting the pure
//! `⏪️time-travel` session reducer, the host-driven `historyEdit*` verbs, the session preview every render seam reads,
//! the Report replay stepped per reactor turn, the finalize commit, the per-mutation history overlay and the history
//! panel's time-travel sections.
//!
//! Contract: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §4, §7 and §10.
//! Domain-neutral session: `semio_framework_time_travel`; store read API: `ArtifactStore::{state_before,
//! begin_report_replay, replay_report, commit_finished_replay}`.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_os_kernel as pack;
use semio_framework::ActionArgDef;
use semio_framework::{Fault, FaultCode, FaultOrigin, ArtifactDialect};
use semio_framework::kernel::{HistoryPatch, MutationId, UiDirtyScope};
use semio_framework_plugin_decl_views::child_view::{ChildContentView, MemberKey};
use semio_framework_plugin_child::MemberPath;
use semio_framework_plugin_tool_run::ToolRunLedger;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::ManuallyDrop;
use semio_framework_ui_layout::layout::FRAMEWORK_HISTORY_BODY_KEY;
use semio_framework_plugin_command::HISTORY_UNIT_SPANS_DOCUMENTS_CODE;
use semio_framework_plugin_decl_app::ArtifactApp;
use pack::snapshot::ArtifactPack;
use store::ArtifactStore;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use semio_framework_value::DslValue;
use semio_framework_value_derive::FromValue;
use std::collections::HashMap;
use std::collections::HashSet;
use semio_framework::kernel::HybridLogicalTimestamp;
use semio_framework_ui_locale::LocalizedLabel;
use store::Mutation;
use protocol::OpBinary;
use protocol::OpText;
use semio_framework_value_derive::ToValue;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use store::MutationDiff;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_ui_contract::*;
use semio_framework::kernel::{HistoryReprojectionKind, HistoryTimeTravel, HistoryTimeTravelProblem, HistoryTimeTravelReview, HistoryTimeTravelStage, RequestId};
use semio_framework::{mutation_input_defs, reference_id_text, registered_input_schema_document, ArgSchema, InputSchemaError};
use semio_framework::history_edit::HISTORY_EDIT_ARG_GENERATION;
use semio_framework_time_travel::{TimeTravelBase, TimeTravelEvent, TimeTravelLabel, TimeTravelRefusal, TimeTravelReview, TimeTravelSession, TimeTravelStage, TimeTravelTarget, TIME_TRAVEL_BUSY_CODE, TIME_TRAVEL_EDITOR_CLOSED_CODE, TIME_TRAVEL_INVALID_INPUT_CODE, TIME_TRAVEL_NAME_INVALID_CODE, TIME_TRAVEL_NAME_REQUIRED_CODE, TIME_TRAVEL_NOT_EDITABLE_CODE, TIME_TRAVEL_NOT_WITHDRAWABLE_CODE, TIME_TRAVEL_NO_SELECTION_CODE, TIME_TRAVEL_SCHEMA_UNAVAILABLE_CODE, TIME_TRAVEL_UNKNOWN_INPUT_CODE, TIME_TRAVEL_UNKNOWN_MUTATION_CODE};
use store::ErasedSnapshotRetirement;
#[cfg(test)] use semio_framework_os_kernel::HistoryPageStack;
use semio_framework::kernel::{HistoryMutationEntry, HistoryMutationMessage, HistoryReprojection};
use semio_framework::{input_label_glossary, ActionArgControl, ActionArgOption, ArgPresentation, OptionSource, SnapSource};
use semio_framework::history_edit::{HISTORY_EDIT_ACCEPT_ACTION_ID, HISTORY_EDIT_ARG_EDIT, HISTORY_EDIT_ARG_MUTATION_ID, HISTORY_EDIT_ARG_PATH, HISTORY_EDIT_ARG_STORE, HISTORY_EDIT_ARG_VALUE, HISTORY_EDIT_BEGIN_ACTION_ID, HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, HISTORY_EDIT_DISCARD_ACTION_ID, HISTORY_EDIT_EXIT_ACTION_ID, HISTORY_EDIT_FINALIZE_ACTION_ID, HISTORY_EDIT_INPUT_ACTION_ID, HISTORY_EDIT_INPUT_INSERT, HISTORY_EDIT_INPUT_REMOVE, HISTORY_EDIT_RERUN_ACTION_ID, HISTORY_EDIT_USE_SELECTION_ACTION_ID, HISTORY_EDIT_WITHDRAW_ACTION_ID};
use semio_framework_time_travel::{
    
};
use std::collections::VecDeque;
use std::sync::Arc;
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
#[path="♻️retirement/🦀️.rs"]
mod original_retirement;
#[cfg(test)]
pub use crate::original_retirement::tests::assert_session_original;
#[cfg(test)]
pub use crate::original_retirement::tests::assert_pending_command_original;
#[cfg(test)]
pub use crate::original_retirement::tests::assert_discard_publication_original;
#[cfg(test)]
pub use crate::original_retirement::tests::native_original_actor_grant;

pub use semio_framework_plugin_time_travel_panel::panel_pointer::*;

pub use semio_framework_plugin_time_travel_panel::panel_sections::*;

pub use semio_framework_plugin_command::lifted_command::*;

#[path = "⏱️limits-and-outcomes/🦀️.rs"]
mod limits_and_outcomes;
pub use limits_and_outcomes::*;

#[path = "✏️editor-and-member-subjects/🦀️.rs"]
mod editor_and_member_subjects;
pub use editor_and_member_subjects::*;

#[path = "📒️time-travel-ledger/🦀️.rs"]
mod time_travel_ledger;
pub use time_travel_ledger::*;

//#region 🔖️StoreState
#[doc(hidden)]
#[path="🎮️decision/🗑️discard/🦀️.rs"]
pub mod discard_decision;

#[path = "🗄️store-state/🦀️.rs"]
mod store_state;
pub use store_state::*;

#[path = "🧱️store-owners-and-member-visitors/🦀️.rs"]
mod store_owners_and_member_visitors;
pub use store_owners_and_member_visitors::*;

//#endregion 🔖️StoreState

#[path = "🔀️supersession-ledger/🦀️.rs"]
mod supersession_ledger;
pub use supersession_ledger::*;
