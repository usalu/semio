//! 🪬️ Staged slice of the `dispatch` band of the mounted artifact instance: its `instance-history` members (WP6 step S0 of the instance family).
//!
//! 📨️ Dispatch of the mounted artifact instance: test snapshot helpers, command log, history view, patch and refresh, preview, `dispatch_emit` and transaction groups.
use super::*;
use crate::tool_jobs::time_travel;

impl<A: ArtifactApp> CommandHistory<A> {
    /// 🧾️ Appends one entry to the session command log. `timestamp: None` stamps "now"
    /// (live dispatch); `Some(..)` preserves an edit's original `started_at` (backfill). Always a
    /// fresh row (`count: 1`) — folding consecutive `View`/`🐚️Shell` dispatches is `record_command`'s job.
    pub(crate) fn push_log_entry(&mut self, append: CommandLogAppend<'_>) {
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
    pub(crate) fn history_view_patch(&self, stores: &InstanceStores<A>, before: &HistoryViewStamp, after: &HistoryViewStamp, members: &[(String, time_travel::HistoryStoreStamp, Option<Vec<(String, Option<usize>)>>)]) -> Option<HistoryViewPatch> {
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
    pub(crate) fn history_view_stacks(&self, stores: &InstanceStores<A>, child_applied_tails: &HashSet<String>, child_has_redo_tail: bool, inverse_rows: usize) -> (bool, bool) {
        let can_undo = !stores.store.applied_edit_ids().is_empty() || !child_applied_tails.is_empty() || inverse_rows > self.shell_undone.len() || self.supersede_can_undo(stores);
        let can_redo = !stores.store.redo_edit_ids().is_empty() || child_has_redo_tail || !self.shell_redo.is_empty() || self.supersede_redo_target(stores).is_some();
        (can_undo, can_redo)
    }

    /// 🎯️ Shares immutable cache inputs with a command job without cloning the document or history.
    pub(crate) fn command_cache_inputs(&self) -> (std::sync::Arc<A::Snapshot>, std::sync::Arc<A::Config>, std::sync::Arc<HistoryView>) {
        let (_, snapshot, config, history) = self.cache.as_ref().expect("cache refreshed before command scheduling");
        (std::sync::Arc::clone(snapshot), std::sync::Arc::clone(config), std::sync::Arc::clone(history))
    }

    /// 🎗️ The store half of a migrated emission that closes a streamed tool transaction without new operations (design
    /// §15): an abort reverts the transaction's open edit and retires its history row, a commit with no last operations
    /// closes the edit (its announcement leaves with the run's next flush). Either is consumed without suspending, so the
    /// one-page publication continues with the emission's other lanes; a closing commit with operations and every stream
    /// are left to the batched route. A commit carrying composed-child emissions keeps its ref: the child group stamps it
    /// on every member op (design §12), so a child-only tool transaction is still one transaction row.
    pub(crate) fn close_streamed_transaction_unit(&mut self, stores: &mut InstanceStores<A>, emit: &mut Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>) -> Result<(), Fault> {
        let Some(transaction) = emit.transaction.as_ref() else { return Ok(()) };
        let abort = emit.transaction_phase == TransactionPhase::Abort;
        if !abort && (emit.transaction_phase == TransactionPhase::Stream || !emit.artifact_mutations.is_empty()) {
            return Ok(());
        }
        let open = stores.store.open_transaction().is_some_and(|open| open.transaction.id == transaction.id);
        let transaction_id = transaction.id.clone();
        if abort || emit.child_emits.is_empty() {
            emit.transaction = None;
            emit.transaction_phase = TransactionPhase::Commit;
        }
        if !open {
            return Ok(());
        }
        if abort {
            stores.store.abort_open_transaction(&transaction_id).map_err(|error| error.into_fault())?;
            self.retire_displaced_document_rows();
        } else {
            stores.store.commit_open_transaction(&transaction_id).map_err(|error| error.into_fault())?;
        }
        Ok(())
    }
}
