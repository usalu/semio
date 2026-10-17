//! 🔩️ Staged slice of the `dispatch` band of the mounted artifact instance: its `instance-jobs` members (WP6 step S0 of the instance family).
//!
//! 📨️ Dispatch of the mounted artifact instance: test snapshot helpers, command log, history view, patch and refresh, preview, `dispatch_emit` and transaction groups.
use semio_framework::{ActionKind, Fault, FaultCode, FaultFrom, FaultOrigin};
use semio_framework_plugin_decl_emission::{ActionMeta, Emit};
use semio_framework::kernel::{AppEvent, Effect, InvocationId, InvocationResult, UiDirtyScope, UndoGroup};
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_instance_history::history_components::{CommandHistory, HostEvents};
use semio_framework_plugin_command::{CommandLogAppend, HISTORY_ROW_OPERATION_PREVIEW, HistoryRowReads, history_intent_label, history_leaf_row_label, history_row_applied, history_row_is_recorded, runtime_verb_label};
use semio_framework_plugin_decl_views::history_view::{CommandLogEntry, CommandView, InverseAction};
use semio_framework_value::DslValue;
use std::future::Future;
#[cfg(test)]
use semio_framework_plugin_command::HISTORY_ROWS_BUILT;
use std::collections::HashMap;
use semio_framework_plugin_instance_stores::stores_components::InstanceStores;
use semio_framework_ui_locale::LocalizedLabel;
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use crate::jobs_components::{PendingReserved, ToolOperations};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_time_travel::SupersedeRole;
use semio_framework_ui_contract::*;
use semio_framework_plugin_time_travel as time_travel;

impl<A: ArtifactApp> PendingReserved<A> {
    pub async fn reset_dispatch_report(&mut self, stores: &mut InstanceStores<A>) {
        self.dispatch_report = protocol::DispatchReport { policy: stores.store.merge_policy(), worst: None, messages: Vec::new() };
    }

    pub async fn record_dispatch_receipt(&mut self, stores: &mut InstanceStores<A>, receipt: store::CommandReceipt) {
        self.dispatch_report = protocol::DispatchReport { policy: stores.store.merge_policy(), worst: receipt.worst, messages: receipt.messages.into_iter().flat_map(|entry| entry.messages).collect() };
    }

    pub async fn record_rejected_dispatch(&mut self, policy: protocol::MergePolicy, messages: Vec<protocol::MutationMessage>) -> Fault {
        self.dispatch_report = protocol::DispatchReport { policy, worst: protocol::worst_level(&messages), messages };
        Fault::new(FaultOrigin::App, FaultCode::new("app.command.rejected"), "mutation rejected by the active merge policy")
    }

    pub async fn apply_merge_policy(&mut self, stores: &mut InstanceStores<A>, policy: protocol::MergePolicy) {
        stores.store.set_merge_policy(policy);
        self.reset_dispatch_report(stores).await;
    }

    pub async fn resolve_store_conflict(&mut self, stores: &mut InstanceStores<A>, history: &mut CommandHistory<A>, conflict_id: &str, resolution: protocol::ConflictResolution) -> Result<protocol::MergeReport, Fault> {
        let report = stores.store.resolve_conflict(conflict_id, resolution).await.map_err(|error| error.into_fault())?;
        self.dispatch_report = protocol::DispatchReport { policy: report.policy, worst: report.worst, messages: report.replayed.iter().flat_map(|entry| entry.messages.iter().cloned()).collect() };
        history.cache = None;
        Ok(report)
    }
}

impl<A: ArtifactApp> ToolOperations<A> {
    /// 🧾️ The single entry point every live dispatch logs through (`push_log_entry` remains for
    /// backfill, which never folds). Consecutive `View`/`🐚️Shell` dispatches of the SAME `(action_id,
    /// kind)` with no `edit_id` fold into one row — its `count` increments and its `label`/`timestamp`
    /// refresh, but its ORIGINAL `seq` is kept so the panel's tree-item id stays stable across
    /// re-renders. `Mutation`/`History`/`Clipboard` entries and anything with an `edit_id` never fold.
    /// `inverse` (computed from state BEFORE this dispatch) is only stored on a FRESH row — a folded
    /// row keeps its original inverse, since inverse on a folded "×N" row must undo the whole run,
    /// not just the last dispatch that folded into it.
    ///
    /// 🙈️ Interaction verbs (hover, pick, clear selection) and pure view verbs (a `View` that edited no store and
    /// carries no inverse) are never rows: they change no history and undo cannot reach them
    /// ([`history_row_is_recorded`]).
    ///
    /// 🌐️ The row keeps the action's FULL locale matrix — no resolve happens here, so the
    /// History panel renders every row in whatever locale the shell is showing right now.
    /// A row with no declaring definition falls back to its `action_id`, which is
    /// locale-invariant data, not untranslated English.
    pub fn record_command(&mut self, history: &mut CommandHistory<A>, action_id: &str, kind: ActionKind, label: Option<LocalizedLabel>, edit_id: Option<String>, child_edit_ids: Vec<String>, inverse: Option<InverseAction>) {
        if !history_row_is_recorded(kind, edit_id.is_some() || !child_edit_ids.is_empty(), inverse.is_some()) {
            return;
        }
        let label = label.or_else(|| self.verb_label(action_id)).unwrap_or_else(|| LocalizedLabel::data(action_id));
        history.push_log_entry(CommandLogAppend { action_id, label, kind, edit_id, transition_id: None, timestamp: None, inverse });
        if let Some(last) = history.command_log.last_mut() {
            last.child_edit_ids = child_edit_ids;
        }
        if !matches!(kind, ActionKind::History) {
            history.shell_redo.clear();
        }
    }

    /// 🏷️ The registry label (every locale) of the action or command `verb` names — how a persisted
    /// [`protocol::Edit::verb`] becomes a history label at projection time; `None` for a verb this app does not declare.
    pub fn verb_label(&self, verb: &str) -> Option<LocalizedLabel> {
        self.registry.get(verb).map(|definition| definition.label.clone()).or_else(|| self.registry.get_command(verb).map(|definition| definition.label.clone())).or_else(|| runtime_verb_label(verb))
    }

    /// 🧱️ One history row of `entry` over `reads` — what a full build and an incremental patch share (design §20.14): a
    /// supersede record's row, or a command row with its edit's printed lines, its applied and revertible state, its mutation
    /// rows (its edit's applied operations read as one slice, then its composed members'), its transaction and its label.
    pub fn history_row(&self, stores: &InstanceStores<A>, history: &CommandHistory<A>, entry: &CommandLogEntry, reads: &HistoryRowReads<'_, A::Mutation>) -> CommandView {
        #[cfg(test)]
        HISTORY_ROWS_BUILT.with(|built| built.set(built.get() + 1));
        let previous = reads.previous.get(&entry.seq).copied();
        if let Some((index, record)) = entry.transition_id.as_deref().and_then(|transition_id| history.supersedes.record(transition_id)) {
            let supersede_author = stores.supersede_author();
            return CommandView {
                seq: entry.seq,
                action_id: entry.action_id.clone(),
                label: history.supersede_row_label(stores, index),
                kind: entry.kind,
                timestamp: entry.timestamp.clone(),
                edit_id: None,
                child_edit_ids: Vec::new(),
                transition_id: Some(record.transition_id.clone()),
                author: Some(record.actor.as_str().to_owned()),
                op_lines: Vec::new(),
                op_count: 0,
                applied: history.supersedes.row_applied(index, &reads.applied_entries),
                revertible: record.role != SupersedeRole::Undo && record.actor.as_str() == supersede_author && history.supersedes.undoable(supersede_author, record.entry, &reads.applied_entries),
                count: entry.count,
                inverse: None,
                transaction: None,
                mutations: Vec::new(),
            };
        }
        let edit = entry.edit_id.as_deref().and_then(|edit_id| match reads.full {
            true => reads.edits.get(edit_id).copied(),
            false => stores.store.envelope().vcs.edits.iter().rev().find(|edit| edit.id == edit_id),
        });
        let position = stores.history_row_position(entry, reads, previous);
        let applied_ops = position.and_then(|position| stores.store.applied_edit_mutations(position).ok()).unwrap_or_default();
        let durable = position.and_then(|position| stores.store.applied_edit_outcomes(position).ok()).unwrap_or_default();
        let outcomes: HashMap<&str, &protocol::MutationReplayOutcome> = durable.iter().map(|outcome| (outcome.mutation_id.0.as_str(), outcome)).collect();
        let ops: Vec<&store::AppliedMutation<'_, A::Mutation>> = applied_ops.iter().collect();
        let mut op_lines = Vec::new();
        let mut op_count = 0;
        if let Some(edit) = edit {
            op_count = edit.forwards.len();
            match previous.filter(|row| row.edit_id == entry.edit_id && row.op_count == op_count) {
                Some(row) => op_lines.extend_from_slice(&row.op_lines),
                None => op_lines.extend(edit.forwards[op_count.saturating_sub(HISTORY_ROW_OPERATION_PREVIEW)..].iter().map(|op| op.print_op())),
            }
        }
        let document_applied = position.is_some();
        let child_applied = entry.child_edit_ids.iter().any(|edit_id| reads.child_applied_tails.contains(edit_id));
        let local_actor = stores.store.local_actor_id();
        let revertible = (document_applied && edit.is_some_and(|edit| edit.actor.as_deref() == Some(local_actor.0.as_str()))) || child_applied || (entry.inverse.is_some() && !history.shell_undone.contains(&entry.seq));
        let child_edits: Vec<&time_travel::MemberEditHistory> = entry.child_edit_ids.iter().filter_map(|edit_id| reads.members.get(edit_id.as_str())).collect();
        if edit.is_none() {
            op_count = child_edits.iter().map(|child| child.op_count).sum();
            op_lines.extend(child_edits.iter().flat_map(|child| child.op_lines.iter().cloned()));
            op_lines.drain(..op_lines.len().saturating_sub(HISTORY_ROW_OPERATION_PREVIEW));
        }
        let mut mutations = time_travel::history_mutation_views::<A>(&ops, &outcomes, &reads.labelled);
        mutations.extend(child_edits.iter().flat_map(|child| child.mutations.iter().cloned()));
        let transaction = edit.and_then(|edit| edit.mutation_meta.first()).and_then(|meta| meta.transaction.clone()).or_else(|| child_edits.iter().find_map(|child| child.transaction.clone()));
        let tool_run_label = transaction.as_ref().and_then(|transaction| self.tool_run_transaction_label(&reads.app_id, transaction));
        let leaf_label = ops.first().map(|op| op.operation).or_else(|| edit.and_then(|edit| edit.forwards.first())).map(|op| protocol::SemanticMutation::<A::Snapshot>::label(op));
        let verb_label = edit.and_then(|edit| edit.verb.as_deref()).and_then(|verb| self.verb_label(verb));
        let intent = transaction.as_ref().and_then(|transaction| history_intent_label::<A>(&transaction.tool, &ops, &mutations)).or_else(|| child_edits.iter().find_map(|child| child.intent_label.clone()));
        let label = match (edit, mutations.first()) {
            _ if tool_run_label.is_some() => tool_run_label.expect("tool run label checked above"),
            (Some(_), Some(first)) if transaction.is_some() => {
                history_leaf_row_label(intent.as_ref().unwrap_or(&first.label), op_count)
            }
            (Some(_), _) if op_count == 1 && leaf_label.is_some() => history_leaf_row_label(&leaf_label.expect("leaf label checked above"), op_count),
            (Some(_), _) if verb_label.is_some() => verb_label.expect("verb label checked above"),
            (Some(_), _) if leaf_label.is_some() => history_leaf_row_label(&leaf_label.expect("leaf label checked above"), op_count),
            (None, Some(first)) if !child_edits.is_empty() => history_leaf_row_label(intent.as_ref().unwrap_or(&first.label), op_count),
            _ => entry.label.clone(),
        };
        CommandView {
            seq: entry.seq,
            action_id: entry.action_id.clone(),
            label,
            kind: entry.kind,
            timestamp: entry.timestamp.clone(),
            edit_id: entry.edit_id.clone(),
            child_edit_ids: entry.child_edit_ids.clone(),
            transition_id: None,
            author: edit.and_then(|edit| edit.actor.as_ref().map(|actor| actor.as_str().to_owned())),
            op_lines,
            op_count,
            applied: history_row_applied(entry.edit_id.is_some(), document_applied, child_applied),
            revertible,
            count: entry.count,
            inverse: entry.inverse.clone(),
            transaction,
            mutations,
        }
    }

    /// 📇️ An empty `InvocationResult` carrying only host effects/events (view/shell actions,
    /// no-operation commands, and history notifications produce no `KernelMutation`s).
    pub async fn empty_result(verb: &str, meta: &ActionMeta, effects: Vec<Effect>, events: Vec<AppEvent>, ui_scope: UiDirtyScope) -> InvocationResult {
        let invocation_id = InvocationId(format!("{verb}:{}", meta.instance_id));
        InvocationResult {
            output: DslValue::Null,
            mutations: Vec::new(),
            inverse_group: UndoGroup { invocation_id, mutations: Vec::new(), inverse_mutations: Vec::new(), member_edits: Vec::new() },
            diagnostics: Vec::new(),
            requested_effects: effects,
            events,
            ui_scope,
            history_patch: None,
        }
    }

    /// 🏷️ The kind a verb dispatches under: the in-flight invocation's own kind when one is
    /// mounted, otherwise the action declaration, otherwise the command declaration.
    /// 🚫️async: E1 pure registry lookup
    pub fn declared_dispatch_kind(&self, host_events: &HostEvents, verb: &str) -> Option<ActionKind> {
        match host_events.invocation_kind {
            Some(kind) => Some(kind),
            None => self.registry.get(verb).map(|definition| definition.kind).or_else(|| self.registry.get_command(verb).map(|command| command.kind)),
        }
    }
}
