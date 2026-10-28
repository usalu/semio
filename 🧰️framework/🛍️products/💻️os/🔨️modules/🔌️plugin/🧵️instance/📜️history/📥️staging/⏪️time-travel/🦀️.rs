//! 🪷️ Staged slice of the `time-travel` band of the mounted artifact instance: its `instance-history` members (WP6 step S0 of the instance family).
//!
//! ⏪️ The mounted instance's time-travel turns (history editing sessions, replay and supersession) and the
//! [`TimeTravelExtension`] that registers them with the instance through [`InstanceExtension`].
use semio_framework_plugin_decl_app::ArtifactApp;
use std::collections::HashMap;
use std::collections::HashSet;
use semio_framework_ui_locale::LocalizedLabel;
use store::Mutation;
use semio_framework_os_kernel::OpBinary;
use semio_framework_plugin_time_travel::SupersedeRole;
use semio_framework_os_kernel as protocol;
use semio_framework_os_kernel as store;
use semio_framework_os_kernel as vcs;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use protocol::OpText;
use crate::history_components::CommandHistory;
use std::future::Future;
use semio_framework_plugin_instance_stores::stores_components::InstanceStores;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_ui_contract::*;
use semio_framework_plugin_time_travel::*;
use semio_framework_os_kernel::{MemberFactory, SpaceMember};

impl<A: ArtifactApp> CommandHistory<A> {
    /// 📚️ Rebuilds the supersede ledger when the store's transitions changed.
    pub fn refresh_supersede_ledger(&mut self, stores: &mut InstanceStores<A>) {
        let envelope = stores.store.envelope();
        self.supersedes.refresh(envelope.transitions.iter(), |targets| {
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
    pub fn retire_displaced_document_rows(&mut self) {
        self.command_prune_generation = self.command_prune_generation.checked_add(1).expect("mounted command visibility generation remains representable");
        self.history_backfill = None;
        self.log_generation += 1;
    }

    /// 🏷️ The history label of supersede record `index`, naming its alternative when it is scoped to one.
    pub fn supersede_row_label(&self, stores: &InstanceStores<A>, index: usize) -> LocalizedLabel {
        let envelope = stores.store.envelope();
        let alternative = self.supersedes.records[index].scope.as_deref().and_then(|scope| envelope.vcs.alternatives.iter().find(|alternative| alternative.id == scope)).map(|alternative| alternative.name.as_str());
        self.supersedes.label(index, alternative)
    }

    /// ⏪️ The history edit a plain `undo` takes back: the newest own undo target of the command log, when it is a history
    /// edit row (the edit or its redo) still in effect. A newer own document, configuration, child or shell target keeps
    /// the plain undo on its own lane; rows of other authors are passed over.
    pub fn supersede_undo_target(&self, stores: &InstanceStores<A>, child_applied_tails: &HashSet<String>) -> Option<usize> {
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
    pub fn supersede_redo_target(&self, stores: &InstanceStores<A>) -> Option<(usize, usize)> {
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
