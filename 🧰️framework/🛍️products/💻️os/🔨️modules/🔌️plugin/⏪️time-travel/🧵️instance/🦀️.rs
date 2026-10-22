//! ⏪️ The mounted instance's time-travel turns (history editing sessions, replay and supersession) and the
//! [`TimeTravelExtension`] that registers them with the instance through [`InstanceExtension`].
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_os_kernel as pack;
use semio_framework::kernel::HistoryReprojectionKind;
use semio_framework::{mutation_input_instance, reference_id_text, registered_input_schema_document, ArgSchema, SnapSource, DIALOG_CHOICE_ARG};
use semio_framework::history_edit::{HISTORY_EDIT_ACCEPT_ACTION_ID, HISTORY_EDIT_ARG_EDIT, HISTORY_EDIT_ARG_MUTATION_ID, HISTORY_EDIT_ARG_NAME, HISTORY_EDIT_ARG_PATH, HISTORY_EDIT_ARG_STORE, HISTORY_EDIT_ARG_VALUE, HISTORY_EDIT_BACK_ACTION_ID, HISTORY_EDIT_BEGIN_ACTION_ID, HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, HISTORY_EDIT_CHOICE_OVERWRITE, HISTORY_EDIT_COMMIT_ACTION_ID, HISTORY_EDIT_DISCARD_ACTION_ID, HISTORY_EDIT_EXIT_ACTION_ID, HISTORY_EDIT_FINALIZE_ACTION_ID, HISTORY_EDIT_FINALIZE_DIALOG_ID, HISTORY_EDIT_INPUT_ACTION_ID, HISTORY_EDIT_INPUT_INSERT, HISTORY_EDIT_INPUT_REMOVE, HISTORY_EDIT_RERUN_ACTION_ID, HISTORY_EDIT_RESTORE_ACTION_ID, HISTORY_EDIT_USE_SELECTION_ACTION_ID, HISTORY_EDIT_WITHDRAW_ACTION_ID};
use semio_framework_time_travel::{is_time_travel_alternative_name, TimeTravelBase, TimeTravelChoice, TimeTravelEffect, TimeTravelEvent, TimeTravelLabel, TimeTravelRefusal, TimeTravelStage, TimeTravelTarget, TIME_TRAVEL_BUSY_CODE, TIME_TRAVEL_COMMIT_FAILED_CODE, TIME_TRAVEL_MEMBER_GONE_CODE, TIME_TRAVEL_REPLAY_FAULTED_CODE};
use std::collections::VecDeque;
use std::sync::Arc;
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework::{ActionArgDef, FaultFrom};
use semio_framework_plugin_decl_app::ArtifactApp;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use store::{ArtifactStore, Mutation, MutationDiff};
use semio_framework_value::DslValue;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_ui_locale::LocalizedLabel;
use protocol::{OpBinary, OpText};
use semio_framework_plugin_time_travel::{SupersedeAuthored, SupersedeAuthoring, SupersedeRole, TIME_TRAVEL_FINALIZE_REQUEST, TIME_TRAVEL_PROGRESS_REFRESH_MS, TIME_TRAVEL_PROGRESS_REFRESH_PERMILLE, TIME_TRAVEL_TURN_WALL_US, TimeTravelActionOutcome, TimeTravelActionRefusal, TimeTravelCommit, TimeTravelEditor, TimeTravelLedger, TimeTravelMemberCommit, TimeTravelMemberMetadata, TimeTravelMemberQuery, TimeTravelMemberRun, TimeTravelMemberSettle, TimeTravelMemberSubject, TimeTravelOwners, TimeTravelPreviewStep, TimeTravelQueryOutput, TimeTravelReplayStep, TimeTravelReviewChildren, TimeTravelStoreCommand, TimeTravelStoreOutput, TimeTravelStoreQuery, TimeTravelStoreState, discard_decision, history_planning_retirement_grant, history_retirement_frame_grant, time_travel_arg_generation, time_travel_selection_value};
use semio_framework_plugin_command::lifted_command::{TimeTravelPanel, time_travel_arg_text};
#[cfg(test)]
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_plugin_time_travel_panel::panel_pointer::{time_travel_coerce, time_travel_default_value, time_travel_entity_names, time_travel_input_at, time_travel_json, time_travel_pointer_get, time_travel_pointer_insert, time_travel_pointer_remove, time_travel_pointer_set, time_travel_reference_fallback_label, time_travel_resolve_options, time_travel_resolve_snaps, time_travel_switch_variant, time_travel_variant_selector};
use semio_framework_ui_contract::*;
use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework_plugin_instance_docs::docs_plugin_app_impl::history_changed_event;
use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework_plugin_command::{CommandLogAppend, HISTORY_STEP_BLOCKED_CODE};
use semio_framework_plugin_document_media::ActiveDocumentArchiveLoadState;
use semio_framework_plugin_interaction::interaction_topology::INTERACTION_ACTION_IDS;
use semio_framework_plugin_job::reserved_jobs::HISTORY_ACTION_IDS;
use semio_framework::{ActionKind, ViewModel, REVERT_TO_COMMAND_ACTION_ID};
use store::{MemberFactory, ReferenceSpaceMember, SpaceMember};
use semio_framework_plugin_decl_app::is_time_travel_action_id;
use semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;

use semio_framework_plugin_instance_docs::docs_maintenance_ladder::item;
use semio_framework_plugin_instance_jobs::jobs_components::ToolOperations;
use semio_framework_plugin_instance_extension::{ExtensionCtx, ExtensionRef};
use semio_framework_plugin_instance_docs::docs_components::MediaTransfers;
use crate::time_travel_bundle::{TimeTravelCtx, TimeTravelRef};
use semio_framework_plugin_tool_run::ToolRunLedger;

#[path = "🧹️owners-and-retirement/🦀️.rs"]
mod owners_and_retirement;

#[path = "🔄️reprojection-turns-and-base-watch/🦀️.rs"]
mod reprojection_turns_and_base_watch;

#[path = "🎬️action-dispatch-and-effects/🦀️.rs"]
mod action_dispatch_and_effects;

#[path = "🔎️preview-replay-and-session-start/🦀️.rs"]
mod preview_replay_and_session_start;

#[path = "✍️draft-inputs/🦀️.rs"]
mod draft_inputs;

#[path = "🏃️turn-driving-and-replay-steps/🦀️.rs"]
mod turn_driving_and_replay_steps;

#[path = "📖️ref-reads/🦀️.rs"]
mod ref_reads;

#[path = "🪄️supersede-authoring-and-extension/🦀️.rs"]
mod supersede_authoring_and_extension;
pub use supersede_authoring_and_extension::*;

#[path = "📥️staging/🧺️bundle/🦀️.rs"]
pub mod time_travel_bundle;
