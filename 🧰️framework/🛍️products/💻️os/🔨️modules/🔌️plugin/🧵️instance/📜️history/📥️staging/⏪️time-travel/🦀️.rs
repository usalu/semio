//! 🪷️ Staged slice of the `time-travel` band of the mounted artifact instance: its `instance-history` members (WP6 step S0 of the instance family).
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

impl<A: ArtifactApp> CommandHistory<A> {
    /// 📚️ Rebuilds the supersede ledger when the store's transitions changed.
    pub(crate) fn refresh_supersede_ledger(&mut self, stores: &mut InstanceStores<A>) {
        let envelope = stores.store.envelope();
        self.supersedes.refresh(&envelope.transitions, |targets| {
            let mut originals = HashMap::new();
            if targets.is_empty() {
                return originals;
            }
            for edit in envelope.vcs.edits.iter() {
                for (mutation_id, op) in store::os_spr::mutation_ids_for_edit::<A::Snapshot, A::Mutation>(edit).into_iter().zip(edit.forwards.iter()) {
                    if targets.contains(&mutation_id) {
                        if let Ok(payload) = <A::Mutation as ::protocol::OpBinary>::encode_op(op) {
                            originals.insert(mutation_id, protocol::InputReplacement::Input { schema: envelope.schema.clone(), payload });
                        }
                    }
                }
                if originals.len() == targets.len() {
                    break;
                }
            }
            originals
        });
    }

    /// 🗂️ Retires the command-log rows of the document a whole-document replacement displaced (an archive or a pack load): a
    /// row naming an edit or a history transition the published store does not hold belonged to that document, so the history
    /// lists exactly the loaded document's rows, which the next read backfills (ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING
    /// follow-up 3). Rows naming neither — configuration and shell rows — stay.
    pub(crate) fn retire_displaced_document_rows(&mut self) {
        self.command_prune_generation = self.command_prune_generation.checked_add(1).expect("mounted command visibility generation remains representable");
        self.history_backfill = None;
        self.log_generation += 1;
    }

    /// 🏷️ The history label of supersede record `index`, naming its alternative when it is scoped to one.
    pub(crate) fn supersede_row_label(&self, stores: &InstanceStores<A>, index: usize) -> LocalizedLabel {
        let envelope = stores.store.envelope();
        let alternative = self.supersedes.records[index].scope.as_deref().and_then(|scope| envelope.vcs.alternatives.iter().find(|alternative| alternative.id == scope)).map(|alternative| alternative.name.as_str());
        self.supersedes.label(index, alternative)
    }

    /// ⏪️ The history edit a plain `undo` takes back: the newest own undo target of the command log, when it is a history
    /// edit row (the edit or its redo) still in effect. A newer own document, configuration, child or shell target keeps
    /// the plain undo on its own lane; rows of other authors are passed over.
    pub(crate) fn supersede_undo_target(&self, stores: &InstanceStores<A>, child_applied_tails: &HashSet<String>) -> Option<usize> {
        let local = stores.supersede_author();
        let applied_entries = self.supersedes.applied_entries(stores.store.supersessions().iter());
        let applied: HashSet<&str> = stores.store.applied_edit_ids().iter().map(String::as_str).collect();
        let envelope = stores.store.envelope();
        let own_edit = |edit_id: &str| envelope.vcs.edits.iter().find(|edit| edit.id == edit_id).is_some_and(|edit| edit.actor.as_deref().is_none_or(|actor| actor == local));
        for entry in self.command_log.iter().rev() {
            if let Some(transition_id) = entry.transition_id.as_deref() {
                let Some((_, record)) = self.supersedes.record(transition_id) else { continue };
                if record.actor.as_str() == local && record.role != SupersedeRole::Undo && self.supersedes.undoable(local, record.entry, &applied_entries) {
                    return Some(record.entry);
                }
                continue;
            }
            let shell = entry.inverse.is_some() && entry.edit_id.is_none() && !self.shell_undone.contains(&entry.seq);
            let document = entry.edit_id.as_deref().is_some_and(|id| applied.contains(id) && own_edit(id));
            let child = entry.child_edit_ids.iter().any(|id| child_applied_tails.contains(id));
            if shell || document || child {
                return None;
            }
        }
        None
    }

    /// ⏩️ The history edit a plain `redo` re-authors: this replica's newest undone one, while its undo still holds every
    /// input it restored, no own document edit is newer than that undo, and no pending document redo was undone later.
    pub(crate) fn supersede_redo_target(&self, stores: &InstanceStores<A>) -> Option<(usize, usize)> {
        let local = stores.supersede_author();
        let (entry, undo) = self.supersedes.redo_top(local)?;
        let undone_at = self.supersedes.records[undo].timestamp;
        if !stores.store.redo_edit_ids().is_empty() && self.supersedes.newest_revert(local).is_some_and(|revert| revert > undone_at) {
            return None;
        }
        let applied: HashSet<&str> = stores.store.applied_edit_ids().iter().map(String::as_str).collect();
        let newer_edit = stores.store.envelope().vcs.edits.iter().any(|edit| applied.contains(edit.id.as_str()) && edit.actor.as_deref().is_none_or(|actor| actor == local) && edit.mutation_meta.first().is_some_and(|meta| meta.timestamp > undone_at));
        (!newer_edit).then_some((entry, undo))
    }

    /// ↩️ Whether a plain `undo` would take a history edit back now.
    pub(crate) fn supersede_can_undo(&self, stores: &InstanceStores<A>) -> bool {
        let local = stores.supersede_author();
        let applied_entries = self.supersedes.applied_entries(stores.store.supersessions().iter());
        self.supersedes.records.iter().enumerate().any(|(index, record)| record.role == SupersedeRole::Edit && record.actor.as_str() == local && self.supersedes.undoable(local, index, &applied_entries))
    }
}
