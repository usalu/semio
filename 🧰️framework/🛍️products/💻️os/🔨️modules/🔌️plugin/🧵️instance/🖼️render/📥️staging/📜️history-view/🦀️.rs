//! 🎳️ Staged `history-view` members of the `instance-render` use cases, as methods of the borrowed bundle (WP7 of the instance family).
//!
//! 📜️ The history projection: the command log backfill, the history view and its patches, the projection cache refresh, the history route commands and the reprojection status.
use semio_framework_plugin_decl_app::ArtifactApp;
use store::os_store::store_schema::ArtifactCommand;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_command::{CommandLogAppendKey, HistoryBackfillMark, HistoryRowReads, HistoryViewCache, HistoryViewPatch, HistoryViewStamp, backfilled_edit_label};
use semio_framework_plugin_decl_views::history_view::{CommandLogEntry, CommandView, HistoryView, MutationView};
use semio_framework_value::DslValue;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::{Fault, FaultFrom};
use std::future::Future;
use std::collections::{HashMap, HashSet};
use semio_framework::kernel::{HistoryEntry, HistoryPatch, InvocationResult};
use semio_framework_ui_locale::LocalizedLabel;
use store::{Mutation, build_history_columns};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use crate::render_bundle::{RenderCtx, RenderRef};
use semio_framework_plugin_command::lifted_command::history_mutation_entry;
use semio_framework_plugin_job::reserved_jobs::plugin_job_yield_once;
use semio_framework_ui_contract::*;
use semio_framework::kernel::HistoryReprojection;
use semio_framework::kernel::HistoryReprojectionKind;
use semio_framework::history_edit::HISTORY_EDIT_COMMIT_ACTION_ID;
use std::sync::Arc;
use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework_plugin_command::CommandLogAppend;
use semio_framework_plugin_document_media::ActiveDocumentArchiveLoad;
use semio_framework::ActionKind;
use store::MemberFactory;
use store::SpaceMember;
use semio_framework_plugin_time_travel as time_travel;

/// 🖋️ Decodes `args.authors` (`[{id, name, avatar?}]`, both shells' wire shape for a
/// checkpoint dispatch) into `vcs::Author`s — empty when absent or malformed, never a hard
/// error: an authorless checkpoint stays valid, it just can't say who checked in.
pub(crate) async fn history_command_authors(args: Option<&DslValue>) -> Vec<vcs::Author> {
    args.and_then(|value| value.get("authors")).and_then(|value| <Vec<vcs::Author> as semio_framework_value::FromValue>::from_value(value.clone()).ok()).unwrap_or_default()
}

/// 🕰️ Maps one of the six injected history action ids to its `ArtifactCommand`.
#[doc(hidden)]
pub async fn history_command<A: ArtifactApp>(action: &str, args: Option<&DslValue>) -> Option<ArtifactCommand<A::Mutation>> {
    let arg_str = |key: &str| args.and_then(|value| value.get(key)).and_then(DslValue::as_str).map(str::to_string);
    match action {
        "undo" => Some(ArtifactCommand::Undo),
        "redo" => Some(ArtifactCommand::Redo),
        "commitCheckpoint" => Some(ArtifactCommand::CommitCheckpoint { message: arg_str("message"), authors: history_command_authors(args).await }),
        "createAlternative" => Some(ArtifactCommand::CreateAlternative { name: arg_str("name").unwrap_or_else(|| "Alternative".into()) }),
        "switchAlternative" => arg_str("alternativeId").map(|alternative_id| ArtifactCommand::SwitchAlternative { alternative_id }),
        "checkoutCheckpoint" => arg_str("checkpointId").map(|checkpoint_id| ArtifactCommand::CheckoutCheckpoint { checkpoint_id }),
        _ => None,
    }
}

/// 🧾️ Whether a verb of this declared kind may dirty the history panel. `dispatch_emit` records a
/// command-log row for EVERY dispatch, view actions included (`ActionKind::View` is also the
/// fallback for an unresolved verb), so the log generation advancing is not by itself evidence that
/// anything a user would call history happened. A `View` verb paints, it does not command; an
/// `Interaction` verb moves selection/hover, which `dispatch_interaction_action` has always
/// declared as kept out of the panel. Handing either one a history patch made every background
/// tick a history update — measured 2026-09-09 20:55 as 90 host `readHistory` round trips beside
/// 252 `fillBuildTick`s in 35 s.
/// 🚫️async: E1 pure classification consumed by sync-only `matches!` call sites — see R9
pub(crate) fn verb_dirties_history_panel<A: ArtifactApp>(kind: Option<ActionKind>) -> bool {
    !matches!(kind, Some(ActionKind::View | ActionKind::Interaction))
}

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> RenderCtx<'ctx, A, M, X> {
    /// 🧪️ Refreshes (backfilling the command log) and returns the current `HistoryView` —
    /// the merged command+operation timeline test harness accessor.
    #[cfg(test)]
    pub async fn test_history(&mut self) -> HistoryView {
        self.refresh_cache().await.expect("refresh cache");
        self.shared().build_history_view(None).await
    }

    /// 🕰️ Appends a command-log entry for every VCS edit not yet referenced by the log —
    /// covers seeded (`app.seed`), ingested (`ingest_operations*`), and loaded
    /// (document archive load) edits that never passed through `dispatch_emit`.
    /// Invariant: after this runs, every `envelope.vcs.edits` entry is referenced by exactly one
    /// `CommandLogEntry`. Idempotent — re-running finds nothing missing. Always `push_log_entry`
    /// (never `record_command`) — a backfilled edit is always its own distinct row, never folded.
    ///
    /// 🌉️ Hoisted out of a `.map()` closure into an explicit loop (sync — `Iterator::map`
    /// cannot take an async closure, and `OpText::print_op` is genuinely async) — see
    /// `build_history_view`'s sibling comment for the same pattern.
    ///
    /// 🧮️ Same backfill, for the CONFIG store's own edits — a config edit reached via `seed`/
    /// `load_config_pack`/ingest never passes through `dispatch_emit` either.
    ///
    /// 🧾️ O(change) (design §20.14): past a [`HistoryBackfillMark`] whose ledger prefixes still stand, only the edits, member
    /// edits and supersede records that came after it are read, against the rows logged since (the then-newest row
    /// included — a typed operation attaches its edit to it); anything else backfills over the whole history.
    async fn backfill_command_log(&mut self) {
        self.history.refresh_supersede_ledger(&mut *self.stores);
        let (ledgers, members_held) = self.child_composition.member_ledger_tails(self.history.history_backfill.as_ref().map(|mark| mark.members.as_slice()));
        let edits = &self.stores.store.envelope().vcs.edits;
        let scope = self.history.history_backfill.as_ref().filter(|mark| {
            let prefix = mark.edits <= edits.len() && (mark.edits == 0 || edits.iter().rev().nth(edits.len() - mark.edits).map(|edit| &edit.id) == mark.last_edit.as_ref());
            prefix && mark.supersede_records == self.history.supersedes.records.len() && members_held && ledgers.iter().all(|(_, tail)| tail.fresh.is_some())
        });
        let (log_from, edits_from, records_from) = scope.map_or((0, 0, 0), |mark| (self.history.command_log.len() - self.history.command_log.iter().rev().take_while(|entry| entry.seq >= mark.log_seq).count(), mark.edits, mark.supersede_records));
        let wanted_members: Option<HashSet<&str>> = scope.map(|_| ledgers.iter().flat_map(|(_, tail)| tail.fresh.iter().flatten().map(String::as_str)).collect());
        let rows = || self.history.command_log.iter().rev().take(self.history.command_log.len() - log_from).rev();
        let logged: HashSet<&str> = rows().filter_map(|entry| entry.edit_id.as_deref()).collect();
        let logged_transitions: HashSet<&str> = rows().filter_map(|entry| entry.transition_id.as_deref()).collect();
        let mut transitions = self.history.supersedes.records.iter().enumerate().skip(records_from).filter(|(_, record)| !logged_transitions.contains(record.transition_id.as_str())).map(|(index, record)| (index, record.timestamp)).peekable();
        let logged_children: HashSet<&str> = rows().flat_map(|entry| entry.child_edit_ids.iter().map(String::as_str)).collect();
        let fresh_edits: Vec<&protocol::Edit<A::Mutation>> = {
            let mut fresh: Vec<&protocol::Edit<A::Mutation>> = edits.iter().rev().take(edits.len() - edits_from).filter(|edit| !logged.contains(edit.id.as_str())).collect();
            fresh.reverse();
            fresh
        };
        let parent_transactions: HashMap<String, String> = fresh_edits.iter().filter_map(|edit| edit.mutation_meta.first().and_then(|meta| meta.transaction.as_ref()).map(|transaction| (transaction.id.clone(), edit.id.clone()))).collect();
        let histories = self.child_composition.member_edit_histories(&HashMap::new(), wanted_members.as_ref());
        let time_travel::MemberBackfill { mut attached, groups } = time_travel::member_backfill(histories.into_values(), &logged_children, &parent_transactions);
        let mut members = groups.into_iter().peekable();
        let mut missing: Vec<(CommandLogAppendKey, LocalizedLabel, String)> = Vec::new();
        let mark = HistoryBackfillMark {
            edits: edits.len(),
            last_edit: edits.last().map(|edit| edit.id.clone()),
            members: ledgers.iter().map(|(member, tail)| (member.clone(), tail.len, tail.last.clone())).collect(),
            supersede_records: self.history.supersedes.records.len(),
            log_seq: 0,
        };
        for edit in fresh_edits {
            let at = edit.mutation_meta.first().map(|meta| meta.timestamp);
            loop {
                let transition_at = transitions.peek().map(|(_, timestamp)| *timestamp).filter(|timestamp| at.is_some_and(|at| *timestamp < at));
                let member_at = members.peek().filter(|group| at.is_some_and(|at| group.at.is_some_and(|member| member < at))).and_then(|group| group.at);
                match (transition_at, member_at) {
                    (Some(transition), Some(member)) if member < transition => {
                        let group = members.next().expect("peeked member group");
                        missing.push((CommandLogAppendKey::Members { edit_ids: group.edit_ids }, group.label, group.started_at));
                    }
                    (Some(_), _) => {
                        let (index, _) = transitions.next().expect("peeked transition");
                        let record = &self.history.supersedes.records[index];
                        missing.push((CommandLogAppendKey::Transition(record.transition_id.clone()), self.history.supersede_row_label(&*self.stores, index), record.timestamp.physical_ms.to_string()));
                    }
                    (None, Some(_)) => {
                        let group = members.next().expect("peeked member group");
                        missing.push((CommandLogAppendKey::Members { edit_ids: group.edit_ids }, group.label, group.started_at));
                    }
                    (None, None) => break,
                }
            }
            let label = backfilled_edit_label(edit, edit.verb.as_deref().and_then(|verb| self.tools.verb_label(verb)), |op| protocol::SemanticMutation::<A::Snapshot>::label(op));
            missing.push((CommandLogAppendKey::Edit { edit_id: edit.id.clone(), verb: edit.verb.clone(), child_edit_ids: attached.remove(&edit.id).unwrap_or_default() }, label, edit.started_at.clone()));
        }
        for (index, _) in transitions {
            let record = &self.history.supersedes.records[index];
            missing.push((CommandLogAppendKey::Transition(record.transition_id.clone()), self.history.supersede_row_label(&*self.stores, index), record.timestamp.physical_ms.to_string()));
        }
        missing.extend(members.map(|group| (CommandLogAppendKey::Members { edit_ids: group.edit_ids }, group.label, group.started_at)));
        for (key, label, timestamp) in missing {
            match key {
                CommandLogAppendKey::Edit { edit_id, verb, child_edit_ids } => {
                    self.history.push_log_entry(CommandLogAppend { action_id: verb.as_deref().unwrap_or("apply"), label, kind: ActionKind::Mutation, edit_id: Some(edit_id), transition_id: None, timestamp: Some(timestamp), inverse: None });
                    if let Some(last) = self.history.command_log.last_mut() {
                        last.child_edit_ids = child_edit_ids;
                    }
                }
                CommandLogAppendKey::Members { edit_ids } => {
                    self.history.push_log_entry(CommandLogAppend { action_id: "apply", label, kind: ActionKind::Mutation, edit_id: None, transition_id: None, timestamp: Some(timestamp), inverse: None });
                    if let Some(last) = self.history.command_log.last_mut() {
                        last.child_edit_ids = edit_ids;
                    }
                }
                CommandLogAppendKey::Transition(transition_id) => self.history.push_log_entry(CommandLogAppend {
                    action_id: semio_framework::history_edit::HISTORY_EDIT_COMMIT_ACTION_ID,
                    label,
                    kind: ActionKind::History,
                    edit_id: None,
                    transition_id: Some(transition_id),
                    timestamp: Some(timestamp),
                    inverse: None,
                }),
            }
        }
        self.history.history_backfill = Some(HistoryBackfillMark { log_seq: self.history.command_log.iter().next_back().map_or(0, |entry| entry.seq), ..mark });
    }

    /// 🗂️ The history view of the current command log and stores (design §20.14): the cached one patched in place where
    /// the change only appended, amended, undid or redid edits and rows ([`Self::history_view_patch`]) — O(change), and in
    /// the same allocation unless a command job still shares it — else rebuilt over the whole history, reusing the previous
    /// projection's printed lines and labels. Every row whose projection changed is marked for the next history patch, so
    /// the wire carries exactly the changed rows (an undo no longer re-sends the whole history).
    async fn refresh_history_view(&mut self) -> std::sync::Arc<HistoryView> {
        let retained = self.history.history_view.take();
        let members = self.child_composition.member_history_stamps(retained.as_ref().map_or(&[][..], |cache| cache.stamp.members.as_slice()));
        let mut stamp = HistoryViewStamp {
            document: time_travel::history_store_stamp(&self.stores.store),
            members: members.iter().map(|(member, stamp, _)| (member.clone(), stamp.clone())).collect(),
            newest_seq: self.history.command_log.iter().next_back().map_or(0, |entry| entry.seq),
            rows: self.history.command_log.len(),
            inverse_rows: 0,
            shell_undone: self.history.shell_undone.len(),
            supersede_records: self.history.supersedes.records.len(),
        };
        let patch = retained.as_ref().and_then(|cache| self.history.history_view_patch(&*self.stores, &cache.stamp, &stamp, &members).map(|patch| (patch, cache.stamp.inverse_rows)));
        let (view, changed) = match (retained, patch) {
            (Some(HistoryViewCache { mut view, .. }), Some((patch, inverse_rows))) => {
                stamp.inverse_rows = inverse_rows + self.history.command_log.iter().rev().take(self.history.command_log.len() - patch.fresh_from).filter(|entry| entry.inverse.is_some()).count();
                let changed = self.shared().patch_history_view(std::sync::Arc::make_mut(&mut view), patch, stamp.inverse_rows).await;
                (view, changed)
            }
            (retained, _) => {
                stamp.inverse_rows = self.history.command_log.iter().filter(|entry| entry.inverse.is_some()).count();
                let previous = retained.as_ref().map(|cache| cache.view.as_ref());
                let view = self.shared().build_history_view(previous).await;
                let held: HashMap<u64, &CommandView> = previous.map_or_else(HashMap::new, |previous| previous.commands.iter().map(|row| (row.seq, row)).collect());
                let changed = view.commands.iter().filter(|row| held.get(&row.seq).is_none_or(|before| *before != *row)).map(|row| row.seq).collect();
                (std::sync::Arc::new(view), changed)
            }
        };
        self.history.history_dirty_sequences.extend(changed);
        self.history.history_view = Some(HistoryViewCache { view: std::sync::Arc::clone(&view), stamp });
        view
    }

    /// 🧾️ Converts the authoritative command log into a typed host projection patch. Rows are
    /// never folded or capped here; the host controls its own visible window. A delta patch reads only its dirty rows
    /// (each found by its `seq` in the newest-first view, design §20.14) and shares the cached view, never copies it.
    pub async fn history_patch(&mut self, snapshot: bool) -> Result<HistoryPatch, Fault> {
        self.refresh_cache().await?;
        let history = std::sync::Arc::clone(&self.history.cache.as_ref().expect("history cache refreshed").3);
        let dirty = std::mem::take(&mut self.history.history_dirty_sequences);
        let time_travel = self.time_travel.panel();
        let rows: Vec<&CommandView> = match snapshot {
            true => history.commands.iter().collect(),
            false => {
                let mut seqs: Vec<u64> = dirty.into_iter().collect();
                seqs.sort_unstable_by(|a, b| b.cmp(a));
                seqs.iter().filter_map(|seq| history.commands.binary_search_by(|row| seq.cmp(&row.seq)).ok().map(|index| &history.commands[index])).collect()
            }
        };
        let upserts = rows
            .into_iter()
            .map(|entry| (entry, entry.mutations.iter().map(|mutation| history_mutation_entry(mutation, time_travel.as_ref())).collect::<Vec<_>>()))
            .map(|(entry, mutations)| HistoryEntry {
                seq: entry.seq,
                action_id: entry.action_id.clone(),
                label: entry.label.clone(),
                kind: match entry.kind {
                    ActionKind::Mutation => "mutation",
                    ActionKind::History => "history",
                    ActionKind::Clipboard => "clipboard",
                    ActionKind::View => "view",
                    ActionKind::Shell => "shell",
                    ActionKind::Interaction => "interaction",
                }
                .into(),
                timestamp: entry.timestamp.clone(),
                op_lines: entry.op_lines.clone(),
                op_count: entry.op_count as u64,
                applied: entry.applied,
                revertible: entry.revertible,
                count: entry.count,
                edit_id: entry.edit_id.clone(),
                transaction: entry.transaction.as_ref().map(|transaction| semio_framework::kernel::HistoryTransaction { id: transaction.id.clone(), tool: transaction.tool.clone() }),
                transition_id: entry.transition_id.clone(),
                author: entry.author.clone(),
                worst: mutations.iter().filter_map(|mutation| mutation.worst).max(),
                mutations,
            })
            .collect();
        Ok(HistoryPatch {
            cursor: self.history.log_generation,
            upserts,
            can_undo: history.can_undo,
            can_redo: history.can_redo,
            active_alternative_id: history.active_alternative_id.clone(),
            current_checkpoint_id: history.current_checkpoint_id.clone(),
            command_filter: history.command_filter.value().into(),
            time_travel: time_travel.map(|panel| panel.status),
            reprojection: self.shared().reprojection_status(),
            edit_count: u32::try_from(self.stores.store.envelope().vcs.edits.len()).unwrap_or(u32::MAX),
        })
    }

    /// 🗂️ Refreshes the snapshot cache if the store advanced, the command log grew/folded, or
    /// the history filter changed since the last materialization. The key is recomputed a SECOND time
    /// after `backfill_command_log` — backfill itself may `push_log_entry` (bumping `log_generation`),
    /// so keying only on the pre-backfill snapshot would store a stale key and thrash on every call.
    ///
    /// 🧾️ The history view is patched, not rebuilt ([`Self::refresh_history_view`], design §20.14): the cache's own
    /// share of it is dropped first, so the retained view stays the only owner and is patched in its own allocation.
    pub async fn refresh_cache(&mut self) -> Result<(), Fault> {
        while self.history.command_prune_requested() || self.history.pending_command_prune.is_some() {
            ({ let __a0 = self.mounted_policy.maintenance; let __a1 = false; self.history.advance_command_prune_step(&mut *self.stores, __a0, __a1) })?;
            plugin_job_yield_once().await;
        }
        let key = (self.stores.store.generation(), self.stores.config_store.generation(), self.history.log_generation, self.history.history_filter);
        if self.history.cache.as_ref().map(|(cached_key, _, _, _)| *cached_key) == Some(key) {
            return Ok(());
        }
        self.backfill_command_log().await;
        let key = (self.stores.store.generation(), self.stores.config_store.generation(), self.history.log_generation, self.history.history_filter);
        let (snapshot, config) = match self.history.cache.take() {
            Some((cached_key, snapshot, config, history)) => {
                drop(history);
                (if cached_key.0 == key.0 { snapshot } else { self.stores.store.snapshot_owner() }, if cached_key.1 == key.1 { config } else { self.stores.config_store.snapshot_owner() })
            }
            None => (self.stores.store.snapshot_owner(), self.stores.config_store.snapshot_owner()),
        };
        let history = self.refresh_history_view().await;
        self.history.cache = Some((key, snapshot, config, history));
        self.follow_tool_machines(false);
        Ok(())
    }

    /// 🧾️ Attaches the command-log delta directly to the accepted invocation response. History is
    /// no longer a plugin-rendered panel body, so this deliberately leaves `ui_scope` untouched.
    /// `verb` is the DECLARED action/command id — every caller must pass the real one, never a generic
    /// placeholder, or the kind lookup below silently resolves to nothing and the skip is unreachable.
    pub async fn finish_recorded(&mut self, log_generation_before: u64, verb: &str, mut result: InvocationResult) -> InvocationResult {
        if self.history.log_generation == log_generation_before {
            return result;
        }
        let kind = self.tools.registry.get(verb).map(|definition| definition.kind).or_else(|| self.tools.registry.get_command(verb).map(|command| command.kind));
        if !verb_dirties_history_panel::<A>(kind) {
            return result;
        }
        result.history_patch = self.history_patch(false).await.ok();
        result
    }
}

impl<'v, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> RenderRef<'v, A, M, X> {
    /// 🧾️ Projects the whole command log into its host view, reusing the op text a PREVIOUS view
    /// already printed for every edit that did not grow.
    ///
    /// ⏱️ Both reuses are what make this O(changed) instead of O(n²) (ticket
    /// 26/09/02/PUZZLE-3D-END-TO-END wave B54). A mutation bumps `log_generation`, so
    /// [`Self::refresh_cache`]'s incremental arm misses and this runs on every single completion —
    /// and it used to resolve each row's edit by a LINEAR scan of the whole edit list (161 rows ×
    /// 161 edits on the battery's brush-painted document) and re-`print_op` every operation of every
    /// edit in the log, both for rows whose text had not changed since the last projection.
    ///
    /// 🧩️ Child lane: a composed app whose document verbs declare
    /// `ArtifactToolPublicationLane::Child` writes every user edit into a CHILD store, so the
    /// parent's own applied stack stays empty for exactly the gesture the user wants back.
    /// `SpaceMember` exposes only each child's TAIL applied/redo edit object-safely — which is
    /// all `undo`/`redo` ever reach anyway, since both walk one member's stack one step.
    ///
    /// 🌉️ Hoisted out of the loop below — both envelopes are the SAME value for every row, so
    /// awaiting once here is strictly cheaper than re-awaiting per row.
    ///
    /// 🌉️ Rewritten from a `.map(|entry| {...}).collect()` into an explicit loop (sync —
    /// `Iterator::map` cannot take an async closure, and `OpText::print_op` is genuinely async).
    ///
    /// 🪞️ Mirrors `store::ArtifactStore::edit_is_local` (private to that crate): an edit
    /// with no recorded actor is treated as local, same as a real undo would.
    ///
    /// ⏪️ Three disjoint ways a row earns "inverse": document edit-linked (applied +
    /// locally authored), config edit-linked (same, on the config store — B1's replacement
    /// for the old memory-only "View"-kind inverse), or memory-only (a stored
    /// `InverseAction`, the remaining `🐚️Shell`-kind `noteShellCommand` path).
    ///
    /// ✅️ `applied` is "this row's edit is live somewhere", and a row publishes into
    /// exactly one of THREE lanes — the parent document store, the config store, or a
    /// composed child's store. Asking only the first made every `Child`- and `Config`-lane
    /// row report `applied: false`, which the host reads as UNDONE: the History panel
    /// dimmed the row and `uncommittedEditCount` (`#s-checkin`) never counted it. That is
    /// one root under three kinds — 🌊️flow's `addWidget` and 🎬️sequence's `addStep` are
    /// `Child`, 🌀️procedural's `generate` is `Config` — each of which moved its document
    /// and still read `edits 0` (ticket 26/09/18 S10; PB3 §3.6 named the child half).
    /// `revertible` below already consulted all three; this is the same law for `applied`.
    ///
    /// ⏪️ …but ONLY for a row that has no parent-document edit of its own. `undo`/`redo`
    /// dispatch against `self.store` alone — `interaction_store`'s own doc states it in as
    /// many words — so a MULTI-lane row (`Artifact` + `Config`, which is what
    /// 💠️lowpoly's `addPrimitive` and 🎥️shooting's `addShot` declare) keeps a live config
    /// edit after its document edit has been undone. Reading those two with `||` made such
    /// a row report `applied: true` for ever: the History panel never dimmed it and
    /// `#s-checkin`'s uncommitted count never came back down. Measured inside `s` on
    /// 2026-09-22 at three different machine loads as `edits [0,1,1,1]` with the ledger
    /// still growing on the undo AND the redo (`applied [0,1,3,6]` / `[1,2,4,7]`) — a
    /// regression of the fix above, in the two kinds that publish on more than one lane
    /// (ticket 26/09/18 S11). The parent document lane is authoritative whenever the row
    /// published into it; the other two answer only for a row that has no parent edit at
    /// all, which is exactly the Child/Config-only case the fix above exists for.
    pub async fn build_history_view(&self, previous: Option<&HistoryView>) -> HistoryView {
        let envelope = self.stores.store.envelope();
        let labelled: HashMap<&str, &MutationView> =
            previous.map_or_else(HashMap::new, |previous| previous.commands.iter().flat_map(|row| row.mutations.iter()).filter(|mutation| !mutation.superseded).map(|mutation| (mutation.mutation_id.as_str(), mutation)).collect());
        let members = if self.history.command_log.iter().any(|entry| !entry.child_edit_ids.is_empty()) { self.child_composition.member_edit_histories(&labelled, None) } else { HashMap::new() };
        let (child_applied_tails, child_has_redo_tail) = self.child_composition.child_history_tails().await;
        let reads = HistoryRowReads {
            full: true,
            positions: self.stores.store.applied_edit_ids().iter().enumerate().map(|(position, edit_id)| (edit_id.as_str(), Some(position))).collect(),
            edits: envelope.vcs.edits.iter().map(|edit| (edit.id.as_str(), edit)).collect(),
            previous: previous.map_or_else(HashMap::new, |previous| previous.commands.iter().map(|row| (row.seq, row)).collect()),
            labelled,
            members,
            child_applied_tails,
            applied_entries: self.history.supersedes.applied_entries(self.stores.store.supersessions().iter()),
            app_id: self.app.instance_id().await.to_string(),
        };
        let commands: Vec<CommandView> = self.history.command_log.iter().rev().map(|entry| self.tools.history_row(&*self.stores, &*self.history, entry, &reads)).collect();
        let (can_undo, can_redo) = ({ let __a0 = &reads.child_applied_tails; let __a1 = child_has_redo_tail; let __a2 = self.history.command_log.iter().filter(|entry| entry.inverse.is_some()).count(); self.history.history_view_stacks(&*self.stores, __a0, __a1, __a2) });
        HistoryView {
            columns: build_history_columns(envelope).await,
            can_undo,
            can_redo,
            active_alternative_id: envelope.active_alternative_id.clone(),
            alternatives: self.stores.history_alternatives(),
            current_checkpoint_id: self.stores.store.current_checkpoint_id().map(str::to_string),
            commands,
            command_filter: self.history.history_filter,
        }
    }

    /// 🩹️ Rebuilds the rows `patch` names in the cached `view` and puts the new log rows in front (design §20.14:
    /// O(change)): each rebuilt row reads its own edit's slice and its composed members' edits, reusing its previous
    /// projection's printed lines and labels; the undo and redo state follows. Answers the rows whose projection changed.
    async fn patch_history_view(&self, view: &mut HistoryView, patch: HistoryViewPatch, inverse_rows: usize) -> Vec<u64> {
        let (child_applied_tails, child_has_redo_tail) = self.child_composition.child_history_tails().await;
        let app_id = self.app.instance_id().await.to_string();
        let (rows, fresh, stacks) = {
            let held = |seq: &u64| view.commands.binary_search_by(|row| seq.cmp(&row.seq)).ok().map(|index| &view.commands[index]);
            let logged = |seq: &u64| self.history.command_log.iter().rev().find(|entry| entry.seq == *seq);
            let previous: HashMap<u64, &CommandView> = patch.rebuild.iter().filter_map(|seq| held(seq).map(|row| (*seq, row))).collect();
            let labelled: HashMap<&str, &MutationView> = previous.values().flat_map(|row| row.mutations.iter()).filter(|mutation| !mutation.superseded).map(|mutation| (mutation.mutation_id.as_str(), mutation)).collect();
            let rebuilt: Vec<&CommandLogEntry> = patch.rebuild.iter().filter_map(logged).collect();
            let fresh = || self.history.command_log.iter().rev().take(self.history.command_log.len() - patch.fresh_from).rev();
            let wanted: HashSet<&str> = rebuilt.iter().copied().chain(fresh()).flat_map(|entry| entry.child_edit_ids.iter().map(String::as_str)).collect();
            let members = self.child_composition.member_edit_histories(&labelled, Some(&wanted));
            let reads = HistoryRowReads {
                full: false,
                positions: patch.positions.iter().map(|(edit_id, position)| (edit_id.as_str(), *position)).collect(),
                edits: HashMap::new(),
                previous,
                labelled,
                members,
                child_applied_tails,
                applied_entries: self.history.supersedes.applied_entries(self.stores.store.supersessions().iter()),
                app_id,
            };
            let rows: Vec<(u64, CommandView)> = rebuilt.iter().map(|entry| (entry.seq, self.tools.history_row(&*self.stores, &*self.history, entry, &reads))).collect();
            let fresh: Vec<CommandView> = fresh().rev().map(|entry| self.tools.history_row(&*self.stores, &*self.history, entry, &reads)).collect();
            (rows, fresh, self.history.history_view_stacks(&*self.stores, &reads.child_applied_tails, child_has_redo_tail, inverse_rows))
        };
        let mut changed: Vec<u64> = fresh.iter().map(|row| row.seq).collect();
        for (seq, row) in rows {
            if let Ok(index) = view.commands.binary_search_by(|held| seq.cmp(&held.seq)) {
                if view.commands[index] != row {
                    changed.push(seq);
                    view.commands[index] = row;
                }
            }
        }
        view.commands.splice(0..0, fresh);
        (view.can_undo, view.can_redo) = stacks;
        view.command_filter = self.history.history_filter;
        changed
    }

    /// 📡️ The history change this replica replays before adopting it, as the history wire and body show it: a whole-document
    /// load first (its archive operation's progress), else the document store's waiting reprojection — this replica's own
    /// history step or a remote change — with whether the user paused a remote one and the code of a refused adoption;
    /// `None` while nothing waits.
    pub fn reprojection_status(&self) -> Option<HistoryReprojection> {
        if let Some(load) = self.ingest.live_document_load().and_then(|operation| self.ingest.document_archive_loads.get(operation)).map(ActiveDocumentArchiveLoad::status) {
            return Some(HistoryReprojection { done: u32::try_from(load.completed).unwrap_or(u32::MAX), total: u32::try_from(load.total).unwrap_or(u32::MAX), processed: None, kind: HistoryReprojectionKind::Load, paused: false, fault: None });
        }
        let kind = if self.stores.store.local_step_pending() { HistoryReprojectionKind::Step } else { HistoryReprojectionKind::Remote };
        match (self.stores.store.reprojection_progress(), self.time_travel.reprojection_fault.as_ref()) {
            (Some(progress), fault) => {
                let processed = self.stores.store.reprojection_operation_preparation_progress().map(|work| work.items).filter(|items| *items > 0).map(|items| u32::try_from(items).unwrap_or(u32::MAX));
                Some(HistoryReprojection { done: progress.done, total: progress.total, processed, kind, paused: kind == HistoryReprojectionKind::Remote && self.time_travel.reprojection_paused, fault: fault.map(|(code, _)| code.clone()) })
            }
            (None, Some((code, kind))) => Some(HistoryReprojection { done: 0, total: 0, processed: None, kind: *kind, paused: false, fault: Some(code.clone()) }),
            (None, None) => None,
        }
    }
}
