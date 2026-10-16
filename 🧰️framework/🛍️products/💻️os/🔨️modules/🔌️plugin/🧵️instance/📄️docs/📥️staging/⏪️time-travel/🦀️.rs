//! 🦠️ Staged slice of the `time-travel` band of the mounted artifact instance: its `instance-docs` members (WP6 step S0 of the instance family).
//!
//! ⏪️ The mounted instance's time-travel turns (history editing sessions, replay and supersession) and the
//! [`TimeTravelExtension`] that registers them with the instance through [`InstanceExtension`].
use super::*;
use crate::tool_jobs::time_travel::*;
#[cfg(test)] use semio_framework_os_kernel::HistoryPageStack;
use semio_framework::kernel::{HistoryReprojection, HistoryReprojectionKind};
use semio_framework::{mutation_input_instance, reference_id_text, registered_input_schema_document, ArgSchema, SnapSource, DIALOG_CHOICE_ARG};
use semio_framework::history_edit::{HISTORY_EDIT_ACCEPT_ACTION_ID, HISTORY_EDIT_ARG_EDIT, HISTORY_EDIT_ARG_MUTATION_ID, HISTORY_EDIT_ARG_NAME, HISTORY_EDIT_ARG_PATH, HISTORY_EDIT_ARG_STORE, HISTORY_EDIT_ARG_VALUE, HISTORY_EDIT_BACK_ACTION_ID, HISTORY_EDIT_BEGIN_ACTION_ID, HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, HISTORY_EDIT_CHOICE_OVERWRITE, HISTORY_EDIT_COMMIT_ACTION_ID, HISTORY_EDIT_DISCARD_ACTION_ID, HISTORY_EDIT_EXIT_ACTION_ID, HISTORY_EDIT_FINALIZE_ACTION_ID, HISTORY_EDIT_FINALIZE_DIALOG_ID, HISTORY_EDIT_INPUT_ACTION_ID, HISTORY_EDIT_INPUT_INSERT, HISTORY_EDIT_INPUT_REMOVE, HISTORY_EDIT_RERUN_ACTION_ID, HISTORY_EDIT_RESTORE_ACTION_ID, HISTORY_EDIT_USE_SELECTION_ACTION_ID, HISTORY_EDIT_WITHDRAW_ACTION_ID};
use semio_framework_time_travel::{
    is_time_travel_alternative_name, TimeTravelBase, TimeTravelChoice, TimeTravelEffect, TimeTravelEvent, TimeTravelLabel, TimeTravelRefusal, TimeTravelStage, TimeTravelTarget, TIME_TRAVEL_BUSY_CODE,
    TIME_TRAVEL_COMMIT_FAILED_CODE, TIME_TRAVEL_MEMBER_GONE_CODE,
    TIME_TRAVEL_REPLAY_FAULTED_CODE,
};
use std::collections::VecDeque;
use std::sync::Arc;
use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use crate::artifact_instance::InstanceExtension;
use crate::artifact_instance::{history_changed_event, VcsArtifactApp};
use crate::app_emission::ActionMeta;
use crate::command_history::{CommandLogAppend, HISTORY_STEP_BLOCKED_CODE};
use crate::document_ingress::{ActiveDocumentArchiveLoad, ActiveDocumentArchiveLoadState};
use crate::interaction_topology::INTERACTION_ACTION_IDS;
use crate::reserved_jobs::HISTORY_ACTION_IDS;
use crate::tree_kit::TREE_WINDOW_DEFAULT_ROWS;
use semio_framework::{ActionKind, ViewModel, REVERT_TO_COMMAND_ACTION_ID};
use store::{MemberFactory, SpaceMember};
use crate::app_contract::is_time_travel_action_id;
use semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> EnvelopeJobs<A, M> {
    /// 🛬️ The whole-document load (document archive operation) still running on this instance, if any
    /// (`📓️api-stepped-document-load.md`).
    pub(crate) fn live_document_load(&self) -> Option<u64> {
        let mut live = None;
        self.document_archive_loads.each_id(|operation| {
            if live.is_none() && self.document_archive_loads.get(operation).is_some_and(ActiveDocumentArchiveLoad::loading) {
                live = Some(operation);
            }
        });
        live
    }
}
