//! 🪬️ Staged slice of the `dispatch` band of the mounted artifact instance: its `instance-history` members (WP6 step S0 of the instance family).
//!
//! 📨️ Dispatch of the mounted artifact instance: test snapshot helpers, command log, history view, patch and refresh, preview, `dispatch_emit` and transaction groups.
#[cfg(any(test, feature = "artifact-app-testing"))]
use std::sync::Arc;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use std::collections::{BTreeSet, HashSet};
use crate::history_components::CommandHistory;
use semio_framework_plugin_command::{CommandLogAppend, HistoryViewPatch, HistoryViewStamp};
use semio_framework_plugin_decl_views::history_view::{CommandLogEntry, HistoryView};
use semio_framework_plugin_decl_emission::{Emit, TransactionPhase};
use semio_framework::{Fault, FaultFrom};
use std::future::Future;
use semio_framework_plugin_instance_stores::stores_components::InstanceStores;
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_ui_contract::*;
use semio_framework_plugin_time_travel as time_travel;

impl<A: ArtifactApp> CommandHistory<A> {
    /// 🧾️ Appends one entry to the session command log. `timestamp: None` stamps "now"
    /// (live dispatch); `Some(..)` preserves an edit's original `started_at` (backfill). Always a
    /// fresh row (`count: 1`) — folding consecutive `View`/`🐚️Shell` dispatches is `record_command`'s job.
    pub fn push_log_entry(&mut self, append: CommandLogAppend<'_>) {
        let CommandLogAppend { action_id, label, kind, edit_id, transition_id, timestamp, inverse } = append;
        self.next_command_seq += 1;
        let mut record = Some(CommandLogEntry {
            seq: self.next_command_seq,
            action_id: action_id.to_string(),
            label,
            kind,
            timestamp: timestamp.unwrap_or_else(store::os_store::document_materialize::now_iso),
            edit_id,
            child_edit_ids: Vec::new(),
            transition_id,
            count: 1,
            inverse,
        });
        let mut page = self.command_log.prepare_append().expect("live command history admits its exact next record");
        page.admit_step(RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: page.next_capacity_byte_demand(), maximum_copy_bytes: 0, maximum_release_bytes: 0, maximum_depth: 64 });
        self.command_log.commit_append(&mut page, &mut record).expect("preborn command page retains its original live prefix");
        self.history_dirty_sequences.insert(self.next_command_seq);
        self.log_generation += 1;
    }

    /// 🩹️ Which rows of the cached view a refresh rebuilds after the stores and the command log moved from `before` to
    /// `after` (design §20.14): every new log row, the previous newest row (the log amends only its newest row), the held
    /// rows of every edit and composed-member edit an appended, amended, undone or redone tail touched
    /// ([`time_travel::history_store_change`]) — found from the log's tail — and the shell rows an undo or redo of theirs
    /// flipped. `None` when the change is not such a patch (rows retired, a member came or went, a history edit, a replay,
    /// a checkout …): the view then rebuilds.
    pub fn history_view_patch(&self, stores: &InstanceStores<A>, before: &HistoryViewStamp, after: &HistoryViewStamp, members: &[(String, time_travel::HistoryStoreStamp, Option<Vec<(String, Option<usize>)>>)]) -> Option<HistoryViewPatch> {
        if before.supersede_records != after.supersede_records || before.members.len() != members.len() || before.members.iter().zip(members).any(|((held, _), (member, _, _))| held != member) {
            return None;
        }
        let fresh_count = self.command_log.iter().rev().take_while(|entry| entry.seq > before.newest_seq).count();
        let fresh_from = self.command_log.len() - fresh_count;
        if fresh_from != before.rows || (fresh_from > 0 && self.command_log.iter().rev().nth(fresh_count).map(|entry| entry.seq) != Some(before.newest_seq)) {
            return None;
        }
        let positions = time_travel::history_store_change(&before.document, &after.document, stores.store.applied_edit_ids())?;
        let fresh = || self.command_log.iter().rev().take(fresh_count);
        let mut children: HashSet<&str> = HashSet::new();
        for (_, _, change) in members {
            children.extend(change.as_ref()?.iter().map(|(edit_id, _)| edit_id.as_str()));
        }
        children.retain(|edit_id| !fresh().any(|entry| entry.child_edit_ids.iter().any(|child| child == edit_id)));
        let mut edits: HashSet<&str> = positions.iter().map(|(edit_id, _)| edit_id.as_str()).filter(|edit_id| !fresh().any(|entry| entry.edit_id.as_deref() == Some(*edit_id))).collect();
        let mut rebuild: BTreeSet<u64> = BTreeSet::new();
        if fresh_from > 0 {
            rebuild.insert(before.newest_seq);
        }
        for entry in self.command_log.iter().rev().skip(fresh_count) {
            if edits.is_empty() && children.is_empty() {
                break;
            }
            let edited = entry.edit_id.as_deref().is_some_and(|edit_id| edits.remove(edit_id));
            let membered = entry.child_edit_ids.iter().fold(false, |found, child| children.remove(child.as_str()) || found);
            if edited || membered {
                rebuild.insert(entry.seq);
            }
        }
        if before.shell_undone != after.shell_undone {
            rebuild.extend(self.history_dirty_sequences.iter().copied().filter(|seq| self.command_log.iter().rev().find(|entry| entry.seq == *seq).is_some_and(|entry| entry.inverse.is_some())));
        }
        Some(HistoryViewPatch { fresh_from, rebuild, positions })
    }

    /// ↩️ The history body's undo and redo availability: a document edit, a composed member's applied tail, a live shell
    /// row (`inverse_rows` of them, the undone ones aside) or a history edit to undo; a redo of any of them.
    pub fn history_view_stacks(&self, stores: &InstanceStores<A>, child_applied_tails: &HashSet<String>, child_has_redo_tail: bool, inverse_rows: usize) -> (bool, bool) {
        let can_undo = !stores.store.applied_edit_ids().is_empty() || !child_applied_tails.is_empty() || inverse_rows > self.shell_undone.len() || self.supersede_can_undo(stores);
        let can_redo = !stores.store.redo_edit_ids().is_empty() || child_has_redo_tail || !self.shell_redo.is_empty() || self.supersede_redo_target(stores).is_some();
        (can_undo, can_redo)
    }

    /// 🎯️ Shares immutable cache inputs with a command job without cloning the document or history.
    pub fn command_cache_inputs(&self) -> (std::sync::Arc<A::Snapshot>, std::sync::Arc<A::Config>, std::sync::Arc<HistoryView>) {
        let (_, snapshot, config, history) = self.cache.as_ref().expect("cache refreshed before command scheduling");
        (std::sync::Arc::clone(snapshot), std::sync::Arc::clone(config), std::sync::Arc::clone(history))
    }

    /// 🪃️ Empty streamed aborts retain their original transaction and replay source until zero-trace publication and paid closure complete.
    pub fn close_streamed_transaction_unit(&mut self, stores: &mut InstanceStores<A>, emit: &mut Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>, pending:&mut Option<Box<semio_framework_os_store_engine_replay::abort::OriginalStreamAbort<A::Snapshot,A::Mutation>>>, completed:&mut bool, cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        if *completed{return Ok(true)}
        let Some(transaction) = emit.transaction.as_ref() else { return Ok(true) };
        let abort = emit.transaction_phase == TransactionPhase::Abort;
        if !abort && (emit.transaction_phase == TransactionPhase::Stream || !emit.artifact_mutations.is_empty()) {return Ok(true)}
        if abort{
            if let Some(owner)=pending.as_mut(){
                if owner.published(){
                    if !owner.terminal_is_empty(){cx.with_original_retirement(|cx|owner.close_step(&mut stores.store,cx))?;return Ok(false)}
                    let mut d=semio_framework_os_store_engine_replay::abort::OriginalStreamAbort::<A::Snapshot,A::Mutation>::boxed_release_demands();d.copy_bytes+=1;
                    cx.with_original_retirement(|cx|semio_framework_job::close_original_retained_step(cx,d,|_|{*pending=None;*completed=true;Ok(semio_framework_value::RetainedCloneStep::Complete(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))}))?;
                    return Ok(*completed)
                }
                if owner.prepared(){owner.publish(&mut stores.store,cx)?;}else{owner.advance(&mut stores.store,&transaction.id,cx)?;}
                return Ok(false)
            }
            if stores.store.open_transaction().is_none(){let mut admitted=None;semio_framework_job::admit_original_construction(cx,&mut admitted,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:1,capacity_bytes:0,release_bytes:0,depth:1},|_|{*completed=true;Ok(((),semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()}))})?;return Ok(*completed)}
            semio_framework_os_store_engine_replay::abort::OriginalStreamAbort::admit(&stores.store,&transaction.id,cx,pending)?;
            return Ok(false)
        }
        let open = stores.store.open_transaction().is_some_and(|open| open.transaction.id == transaction.id);
        let transaction_id = transaction.id.clone();
        if abort || emit.child_emits.is_empty() {
            emit.transaction = None;
            emit.transaction_phase = TransactionPhase::Commit;
        }
        if !open {
            return Ok(true);
        }
        stores.store.commit_open_transaction(&transaction_id).map_err(|error| error.into_fault())?;
        Ok(true)
    }
}
