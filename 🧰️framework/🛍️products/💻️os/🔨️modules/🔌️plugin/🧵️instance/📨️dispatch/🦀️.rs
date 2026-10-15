//! 📨️ Dispatch of the mounted artifact instance: test snapshot helpers, command log, history view, patch and refresh, preview, `dispatch_emit` and transaction groups.

use super::*;

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    #[cfg(test)]
    pub(crate) async fn test_snapshot(&self) -> A::Snapshot {
        self.store.snapshot().expect("materialize snapshot")
    }

    /// 🧪️ The document store itself — needed to assert on checkpoint metadata
    /// (`composition_pins`), which no snapshot-level accessor exposes.
    #[cfg(test)]
    pub(crate) async fn test_store(&self) -> &ArtifactStore<A::Snapshot, A::Mutation> {
        &self.store
    }

    /// ⚔️ Test-only mutable store access for conflict lifecycle fixtures.
    #[cfg(test)]
    pub(crate) async fn test_store_mut(&mut self) -> &mut ArtifactStore<A::Snapshot, A::Mutation> {
        &mut self.store
    }

    /// 🧹️ Test-only probe: whether a cancelled segmented output still waits in bounded cleanup.
    #[cfg(test)]
    pub(crate) fn test_segmented_closure_contains(&self, operation_id: u64) -> bool {
        self.segmented_closures.contains(operation_id)
    }

    /// 🧪️ The config-store twin of `test_snapshot`.
    #[cfg(test)]
    pub(crate) async fn test_config(&self) -> A::Config {
        self.config_store.snapshot().expect("materialize config snapshot")
    }

    /// 🧪️ Direct access to the wrapped app — used to assert on app-private test fixtures (e.g.
    /// `TestApp::received_actions`) that a framework-owned interception must never populate.
    #[cfg(test)]
    pub(crate) async fn test_app(&self) -> &A {
        &self.app
    }

    /// 🧪️ Refreshes (backfilling the command log) and returns the current `HistoryView` —
    /// the merged command+operation timeline test harness accessor.
    #[cfg(test)]
    pub(crate) async fn test_history(&mut self) -> HistoryView {
        self.refresh_cache().await.expect("refresh cache");
        self.build_history_view(None).await
    }

    /// 🧪️ Lands one app `Emit` through the production `dispatch_emit` seam under a named
    /// verb — the harness for laws about what the reactor does with an emit's effect list before
    /// the host sees it (ticket 26/09/16/INPUT-CAUSALITY-LEDGER: the inline interaction fold),
    /// which no registry-less typed command can reach and no reserved route carries.
    #[cfg(test)]
    pub(crate) async fn test_dispatch_emit(&mut self, verb: &str, emit: Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        self.dispatch_emit(verb, emit, meta, identity, cx).await
    }

    /// 🧪️ The `KernelMutation`/`UndoGroup` half of an `InvocationResult` as it stands NOW,
    /// read from the store's own last edit. A migrated dispatch returns an admission receipt whose
    /// `mutations` are empty by construction — the work is handed to a worker and the document only
    /// advances at `settle_registered_typed_operation` — so a settling fixture rebuilds this half
    /// afterwards through the very same `result_from_last_edit` the unmigrated route uses, instead of
    /// every law reaching into the store by hand.
    #[cfg(test)]
    pub(crate) async fn test_result_from_last_edit(&self, verb: &str, meta: &ActionMeta, tail_offset: (usize, usize)) -> InvocationResult {
        self.result_from_last_edit(verb, meta, Vec::new(), Vec::new(), UiDirtyScope::default(), tail_offset).await
    }

    /// 🧪️ `(forwards, inverse)` lengths of the store's last edit — the `before` half of
    /// `dispatch_emit`'s own `tail_offset`, so a settling fixture can report only the operations ONE
    /// dispatch added to a coalesced edit.
    #[cfg(test)]
    pub(crate) fn test_edit_tail_lengths(&self) -> (usize, usize) {
        self.store.edit_mutations().map_or((0, 0), |(forwards, inverse, _)| (forwards.len(), inverse.len()))
    }

    /// 🧪️ The store's last edit id, or `None` on a store that has never committed one —
    /// `dispatch_emit`'s `amended_same_edit` identity test, for a settling fixture.
    #[cfg(test)]
    pub(crate) fn test_last_edit_id(&self) -> Option<String> {
        self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone())
    }

    /// 🧪️ The persisted-plus-leftover selection half the panels and every app read —
    /// what a folded interaction verb must have moved.
    #[cfg(test)]
    pub(crate) fn test_interaction_selection_snapshot(&self) -> protocol::InteractionState {
        self.interaction_selection_snapshot()
    }

    /// 📸️ Materializes and returns the current snapshot — the typed counterpart to
    /// `render`'s `UiNode` output, for callers (host code, downstream plugin crates' own tests) that
    /// need direct structural access to document state instead of a rendered node.
    pub fn snapshot(&self) -> Result<A::Snapshot, Fault> {
        self.store.snapshot().map_err(|error| error.into_fault())
    }

    /// 🧪️ Returns the fixed store revision that a host render or admitted command observes.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn test_document_revision(&self) -> [u8; 32] {
        self.store.content_revision_now()
    }

    /// 🔗️ The store's current backbone descriptor, `None` when unattached (the default).
    pub fn backbone_ref(&self) -> Option<&store::os_store::store_schema::ArtifactBackboneRef> {
        self.store.backbone_ref()
    }

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
    pub(crate) fn record_command(&mut self, action_id: &str, kind: ActionKind, label: Option<LocalizedLabel>, edit_id: Option<String>, child_edit_ids: Vec<String>, inverse: Option<InverseAction>) {
        if !history_row_is_recorded(kind, edit_id.is_some() || !child_edit_ids.is_empty(), inverse.is_some()) {
            return;
        }
        let label = label.or_else(|| self.verb_label(action_id)).unwrap_or_else(|| LocalizedLabel::data(action_id));
        self.push_log_entry(CommandLogAppend { action_id, label, kind, edit_id, transition_id: None, timestamp: None, inverse });
        if let Some(last) = self.command_log.last_mut() {
            last.child_edit_ids = child_edit_ids;
        }
        if !matches!(kind, ActionKind::History) {
            self.shell_redo.clear();
        }
    }

    /// 🏷️ The registry label (every locale) of the action or command `verb` names — how a persisted
    /// [`protocol::Edit::verb`] becomes a history label at projection time; `None` for a verb this app does not declare.
    pub(crate) fn verb_label(&self, verb: &str) -> Option<LocalizedLabel> {
        self.registry.get(verb).map(|definition| definition.label.clone()).or_else(|| self.registry.get_command(verb).map(|definition| definition.label.clone())).or_else(|| runtime_verb_label(verb))
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
        self.refresh_supersede_ledger();
        let (ledgers, members_held) = self.member_ledger_tails(self.history_backfill.as_ref().map(|mark| mark.members.as_slice()));
        let edits = &self.store.envelope().vcs.edits;
        let scope = self.history_backfill.as_ref().filter(|mark| {
            let prefix = mark.edits <= edits.len() && (mark.edits == 0 || edits.iter().rev().nth(edits.len() - mark.edits).map(|edit| &edit.id) == mark.last_edit.as_ref());
            prefix && mark.supersede_records == self.supersedes.records.len() && members_held && ledgers.iter().all(|(_, tail)| tail.fresh.is_some())
        });
        let (log_from, edits_from, records_from) = scope.map_or((0, 0, 0), |mark| (self.command_log.len() - self.command_log.iter().rev().take_while(|entry| entry.seq >= mark.log_seq).count(), mark.edits, mark.supersede_records));
        let wanted_members: Option<HashSet<&str>> = scope.map(|_| ledgers.iter().flat_map(|(_, tail)| tail.fresh.iter().flatten().map(String::as_str)).collect());
        let rows = || self.command_log.iter().rev().take(self.command_log.len() - log_from).rev();
        let logged: HashSet<&str> = rows().filter_map(|entry| entry.edit_id.as_deref()).collect();
        let logged_transitions: HashSet<&str> = rows().filter_map(|entry| entry.transition_id.as_deref()).collect();
        let mut transitions = self.supersedes.records.iter().enumerate().skip(records_from).filter(|(_, record)| !logged_transitions.contains(record.transition_id.as_str())).map(|(index, record)| (index, record.timestamp)).peekable();
        let logged_children: HashSet<&str> = rows().flat_map(|entry| entry.child_edit_ids.iter().map(String::as_str)).collect();
        let fresh_edits: Vec<&protocol::Edit<A::Mutation>> = {
            let mut fresh: Vec<&protocol::Edit<A::Mutation>> = edits.iter().rev().take(edits.len() - edits_from).filter(|edit| !logged.contains(edit.id.as_str())).collect();
            fresh.reverse();
            fresh
        };
        let parent_transactions: HashMap<String, String> = fresh_edits.iter().filter_map(|edit| edit.mutation_meta.first().and_then(|meta| meta.transaction.as_ref()).map(|transaction| (transaction.id.clone(), edit.id.clone()))).collect();
        let histories = self.member_edit_histories(&HashMap::new(), wanted_members.as_ref());
        let time_travel::MemberBackfill { mut attached, groups } = time_travel::member_backfill(histories.into_values(), &logged_children, &parent_transactions);
        let mut members = groups.into_iter().peekable();
        let mut missing: Vec<(CommandLogAppendKey, LocalizedLabel, String)> = Vec::new();
        let mark = HistoryBackfillMark {
            edits: edits.len(),
            last_edit: edits.last().map(|edit| edit.id.clone()),
            members: ledgers.iter().map(|(member, tail)| (member.clone(), tail.len, tail.last.clone())).collect(),
            supersede_records: self.supersedes.records.len(),
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
                        let record = &self.supersedes.records[index];
                        missing.push((CommandLogAppendKey::Transition(record.transition_id.clone()), self.supersede_row_label(index), record.timestamp.physical_ms.to_string()));
                    }
                    (None, Some(_)) => {
                        let group = members.next().expect("peeked member group");
                        missing.push((CommandLogAppendKey::Members { edit_ids: group.edit_ids }, group.label, group.started_at));
                    }
                    (None, None) => break,
                }
            }
            let label = backfilled_edit_label(edit, edit.verb.as_deref().and_then(|verb| self.verb_label(verb)), |op| protocol::SemanticMutation::<A::Snapshot>::label(op));
            missing.push((CommandLogAppendKey::Edit { edit_id: edit.id.clone(), verb: edit.verb.clone(), child_edit_ids: attached.remove(&edit.id).unwrap_or_default() }, label, edit.started_at.clone()));
        }
        for (index, _) in transitions {
            let record = &self.supersedes.records[index];
            missing.push((CommandLogAppendKey::Transition(record.transition_id.clone()), self.supersede_row_label(index), record.timestamp.physical_ms.to_string()));
        }
        missing.extend(members.map(|group| (CommandLogAppendKey::Members { edit_ids: group.edit_ids }, group.label, group.started_at)));
        for (key, label, timestamp) in missing {
            match key {
                CommandLogAppendKey::Edit { edit_id, verb, child_edit_ids } => {
                    self.push_log_entry(CommandLogAppend { action_id: verb.as_deref().unwrap_or("apply"), label, kind: ActionKind::Mutation, edit_id: Some(edit_id), transition_id: None, timestamp: Some(timestamp), inverse: None });
                    if let Some(last) = self.command_log.last_mut() {
                        last.child_edit_ids = child_edit_ids;
                    }
                }
                CommandLogAppendKey::Members { edit_ids } => {
                    self.push_log_entry(CommandLogAppend { action_id: "apply", label, kind: ActionKind::Mutation, edit_id: None, transition_id: None, timestamp: Some(timestamp), inverse: None });
                    if let Some(last) = self.command_log.last_mut() {
                        last.child_edit_ids = edit_ids;
                    }
                }
                CommandLogAppendKey::Transition(transition_id) => self.push_log_entry(CommandLogAppend {
                    action_id: semio_framework::HISTORY_EDIT_COMMIT_ACTION_ID,
                    label,
                    kind: ActionKind::History,
                    edit_id: None,
                    transition_id: Some(transition_id),
                    timestamp: Some(timestamp),
                    inverse: None,
                }),
            }
        }
        self.history_backfill = Some(HistoryBackfillMark { log_seq: self.command_log.iter().next_back().map_or(0, |entry| entry.seq), ..mark });
    }

    /// 🧩️ Every live child's TAIL applied edit id, plus whether any child has an edit on
    /// its redo stack — the composed-artifact halves of `can_undo`/`can_redo`/`revertible`. A
    /// `Child`-lane document verb (see `ArtifactToolPublicationLane::Child`) never touches the
    /// parent store at all, so without this the shell disables its own undo control over a
    /// perfectly undoable ledger row. Empty and free for every leaf app: `entries()` yields
    /// nothing when the registry holds no child.
    pub(super) async fn child_history_tails(&self) -> (HashSet<String>, bool) {
        let mut applied_tails = HashSet::new();
        let mut has_redo_tail = false;
        for entry in self.children.entries() {
            if let Some(edit_id) = entry.member.tail_edit_id().await {
                let _ = applied_tails.insert(edit_id);
            }
            if !has_redo_tail {
                has_redo_tail = entry.member.redo_tail().await.is_some();
            }
        }
        (applied_tails, has_redo_tail)
    }

    /// 🧩️↩️ The composite group id an `undo`/`redo` must target when the PARENT store never
    /// moved. `commit_framework_history_route` reads `self.store.tail_group_id()` first, which is
    /// `None` for a gesture that published only on the `Child` lane; every such gesture still
    /// stamped its `invocation_id` onto each touched child (`CompositionCoordinator::dispatch_group`
    /// phase 2), so the group is recoverable from the children's own tails. Ordered by the
    /// append-only command log rather than by any member's clock: for `undo` the LAST row naming a
    /// live child tail is the most recent still-applied group, and for `redo` the FIRST row naming
    /// a child redo-tail is the one a single `redo` reapplies. A child whose tail carries no group
    /// id is not a composite gesture and is never a target.
    pub(super) async fn child_group_history_target(&self, action: &str) -> Option<String> {
        if self.children.is_empty() {
            return None;
        }
        let mut candidates: Vec<(String, String)> = Vec::new();
        for entry in self.children.entries() {
            let candidate = if action == "undo" {
                match (entry.member.tail_edit_id().await, entry.member.tail_group_id().await) {
                    (Some(edit_id), Some(group_id)) => Some((edit_id, group_id)),
                    _ => None,
                }
            } else {
                match entry.member.redo_tail().await {
                    Some((edit_id, Some(group_id))) => Some((edit_id, group_id)),
                    _ => None,
                }
            };
            if let Some(candidate) = candidate {
                candidates.push(candidate);
            }
        }
        if candidates.is_empty() {
            return None;
        }
        let matched = |entry: &CommandLogEntry| candidates.iter().find(|(edit_id, _)| entry.child_edit_ids.iter().any(|logged| logged == edit_id)).map(|(_, group_id)| group_id.clone());
        let ordered = if action == "undo" { self.command_log.iter().rev().find_map(matched) } else { self.command_log.iter().find_map(matched) };
        ordered.or_else(|| candidates.first().map(|(_, group_id)| group_id.clone()))
    }

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
    pub(crate) async fn build_history_view(&self, previous: Option<&HistoryView>) -> HistoryView {
        let envelope = self.store.envelope();
        let labelled: HashMap<&str, &MutationView> =
            previous.map_or_else(HashMap::new, |previous| previous.commands.iter().flat_map(|row| row.mutations.iter()).filter(|mutation| !mutation.superseded).map(|mutation| (mutation.mutation_id.as_str(), mutation)).collect());
        let members = if self.command_log.iter().any(|entry| !entry.child_edit_ids.is_empty()) { self.member_edit_histories(&labelled, None) } else { HashMap::new() };
        let (child_applied_tails, child_has_redo_tail) = self.child_history_tails().await;
        let reads = HistoryRowReads {
            full: true,
            positions: self.store.applied_edit_ids().iter().enumerate().map(|(position, edit_id)| (edit_id.as_str(), Some(position))).collect(),
            edits: envelope.vcs.edits.iter().map(|edit| (edit.id.as_str(), edit)).collect(),
            previous: previous.map_or_else(HashMap::new, |previous| previous.commands.iter().map(|row| (row.seq, row)).collect()),
            labelled,
            members,
            child_applied_tails,
            applied_entries: self.supersedes.applied_entries(self.store.supersessions().iter()),
            app_id: self.app.instance_id().await.to_string(),
        };
        let commands: Vec<CommandView> = self.command_log.iter().rev().map(|entry| self.history_row(entry, &reads)).collect();
        let (can_undo, can_redo) = self.history_view_stacks(&reads.child_applied_tails, child_has_redo_tail, self.command_log.iter().filter(|entry| entry.inverse.is_some()).count());
        HistoryView {
            columns: build_history_columns(envelope).await,
            can_undo,
            can_redo,
            active_alternative_id: envelope.active_alternative_id.clone(),
            alternatives: self.history_alternatives(),
            current_checkpoint_id: self.store.current_checkpoint_id().map(str::to_string),
            commands,
            command_filter: self.history_filter,
        }
    }

    /// 🩹️ Rebuilds the rows `patch` names in the cached `view` and puts the new log rows in front (design §20.14:
    /// O(change)): each rebuilt row reads its own edit's slice and its composed members' edits, reusing its previous
    /// projection's printed lines and labels; the undo and redo state follows. Answers the rows whose projection changed.
    async fn patch_history_view(&self, view: &mut HistoryView, patch: HistoryViewPatch, inverse_rows: usize) -> Vec<u64> {
        let (child_applied_tails, child_has_redo_tail) = self.child_history_tails().await;
        let app_id = self.app.instance_id().await.to_string();
        let (rows, fresh, stacks) = {
            let held = |seq: &u64| view.commands.binary_search_by(|row| seq.cmp(&row.seq)).ok().map(|index| &view.commands[index]);
            let logged = |seq: &u64| self.command_log.iter().rev().find(|entry| entry.seq == *seq);
            let previous: HashMap<u64, &CommandView> = patch.rebuild.iter().filter_map(|seq| held(seq).map(|row| (*seq, row))).collect();
            let labelled: HashMap<&str, &MutationView> = previous.values().flat_map(|row| row.mutations.iter()).filter(|mutation| !mutation.superseded).map(|mutation| (mutation.mutation_id.as_str(), mutation)).collect();
            let rebuilt: Vec<&CommandLogEntry> = patch.rebuild.iter().filter_map(logged).collect();
            let fresh = || self.command_log.iter().rev().take(self.command_log.len() - patch.fresh_from).rev();
            let wanted: HashSet<&str> = rebuilt.iter().copied().chain(fresh()).flat_map(|entry| entry.child_edit_ids.iter().map(String::as_str)).collect();
            let members = self.member_edit_histories(&labelled, Some(&wanted));
            let reads = HistoryRowReads {
                full: false,
                positions: patch.positions.iter().map(|(edit_id, position)| (edit_id.as_str(), *position)).collect(),
                edits: HashMap::new(),
                previous,
                labelled,
                members,
                child_applied_tails,
                applied_entries: self.supersedes.applied_entries(self.store.supersessions().iter()),
                app_id,
            };
            let rows: Vec<(u64, CommandView)> = rebuilt.iter().map(|entry| (entry.seq, self.history_row(entry, &reads))).collect();
            let fresh: Vec<CommandView> = fresh().rev().map(|entry| self.history_row(entry, &reads)).collect();
            (rows, fresh, self.history_view_stacks(&reads.child_applied_tails, child_has_redo_tail, inverse_rows))
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
        view.command_filter = self.history_filter;
        changed
    }

    /// 🩹️ Which rows of the cached view a refresh rebuilds after the stores and the command log moved from `before` to
    /// `after` (design §20.14): every new log row, the previous newest row (the log amends only its newest row), the held
    /// rows of every edit and composed-member edit an appended, amended, undone or redone tail touched
    /// ([`time_travel::history_store_change`]) — found from the log's tail — and the shell rows an undo or redo of theirs
    /// flipped. `None` when the change is not such a patch (rows retired, a member came or went, a history edit, a replay,
    /// a checkout …): the view then rebuilds.
    fn history_view_patch(&self, before: &HistoryViewStamp, after: &HistoryViewStamp, members: &[(String, time_travel::HistoryStoreStamp, Option<Vec<(String, Option<usize>)>>)]) -> Option<HistoryViewPatch> {
        if before.supersede_records != after.supersede_records || before.members.len() != members.len() || before.members.iter().zip(members).any(|((held, _), (member, _, _))| held != member) {
            return None;
        }
        let fresh_count = self.command_log.iter().rev().take_while(|entry| entry.seq > before.newest_seq).count();
        let fresh_from = self.command_log.len() - fresh_count;
        if fresh_from != before.rows || (fresh_from > 0 && self.command_log.iter().rev().nth(fresh_count).map(|entry| entry.seq) != Some(before.newest_seq)) {
            return None;
        }
        let positions = time_travel::history_store_change(&before.document, &after.document, self.store.applied_edit_ids())?;
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

    /// 🧱️ One history row of `entry` over `reads` — what a full build and an incremental patch share (design §20.14): a
    /// supersede record's row, or a command row with its edit's printed lines, its applied and revertible state, its mutation
    /// rows (its edit's applied operations read as one slice, then its composed members'), its transaction and its label.
    fn history_row(&self, entry: &CommandLogEntry, reads: &HistoryRowReads<'_, A::Mutation>) -> CommandView {
        #[cfg(test)]
        HISTORY_ROWS_BUILT.with(|built| built.set(built.get() + 1));
        let previous = reads.previous.get(&entry.seq).copied();
        if let Some((index, record)) = entry.transition_id.as_deref().and_then(|transition_id| self.supersedes.record(transition_id)) {
            let supersede_author = self.supersede_author();
            return CommandView {
                seq: entry.seq,
                action_id: entry.action_id.clone(),
                label: self.supersede_row_label(index),
                kind: entry.kind,
                timestamp: entry.timestamp.clone(),
                edit_id: None,
                child_edit_ids: Vec::new(),
                transition_id: Some(record.transition_id.clone()),
                author: Some(record.actor.as_str().to_owned()),
                op_lines: Vec::new(),
                op_count: 0,
                applied: self.supersedes.row_applied(index, &reads.applied_entries),
                revertible: record.role != SupersedeRole::Undo && record.actor.as_str() == supersede_author && self.supersedes.undoable(supersede_author, record.entry, &reads.applied_entries),
                count: entry.count,
                inverse: None,
                transaction: None,
                mutations: Vec::new(),
            };
        }
        let edit = entry.edit_id.as_deref().and_then(|edit_id| match reads.full {
            true => reads.edits.get(edit_id).copied(),
            false => self.store.envelope().vcs.edits.iter().rev().find(|edit| edit.id == edit_id),
        });
        let position = self.history_row_position(entry, reads, previous);
        let applied_ops = position.and_then(|position| self.store.applied_edit_mutations(position).ok()).unwrap_or_default();
        let durable = position.and_then(|position| self.store.applied_edit_outcomes(position).ok()).unwrap_or_default();
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
        let local_actor = self.store.local_actor_id();
        let revertible = (document_applied && edit.is_some_and(|edit| edit.actor.as_deref() == Some(local_actor.0.as_str()))) || child_applied || (entry.inverse.is_some() && !self.shell_undone.contains(&entry.seq));
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

    /// 📍️ The applied position of `entry`'s document edit as `reads` know it: from their positions (a full build knows
    /// every applied edit), else — a patch, for a row whose edit no change touched — the position its previous projection
    /// names (its own mutation rows, checked against the applied stack; an edit it held unapplied stays unapplied, no change
    /// named it), else a search from the stack's tail (a row new to the view whose edit the change did not name, e.g. a
    /// backfilled one). An edit neither knows is not applied.
    fn history_row_position(&self, entry: &CommandLogEntry, reads: &HistoryRowReads<'_, A::Mutation>, previous: Option<&CommandView>) -> Option<usize> {
        let edit_id = entry.edit_id.as_deref()?;
        if let Some(position) = reads.positions.get(edit_id) {
            return *position;
        }
        if reads.full {
            return None;
        }
        let applied = self.store.applied_edit_ids();
        let held = previous.filter(|row| row.edit_id.as_deref() == Some(edit_id));
        if held.is_some_and(|row| !row.applied) {
            return None;
        }
        held.and_then(|row| row.mutations.iter().find(|mutation| mutation.store.is_none()))
            .map(|mutation| mutation.position as usize)
            .filter(|position| applied.get(*position).is_some_and(|held| held == edit_id))
            .or_else(|| self.store.applied_edit_position(edit_id))
    }

    /// ↩️ The history body's undo and redo availability: a document edit, a composed member's applied tail, a live shell
    /// row (`inverse_rows` of them, the undone ones aside) or a history edit to undo; a redo of any of them.
    fn history_view_stacks(&self, child_applied_tails: &HashSet<String>, child_has_redo_tail: bool, inverse_rows: usize) -> (bool, bool) {
        let can_undo = !self.store.applied_edit_ids().is_empty() || !child_applied_tails.is_empty() || inverse_rows > self.shell_undone.len() || self.supersede_can_undo();
        let can_redo = !self.store.redo_edit_ids().is_empty() || child_has_redo_tail || !self.shell_redo.is_empty() || self.supersede_redo_target().is_some();
        (can_undo, can_redo)
    }

    /// 🌿️ The alternatives the history body lists ([`history_alternative_views`]) over the log's branch and supersede
    /// transitions.
    fn history_alternatives(&self) -> Vec<AlternativeView> {
        let envelope = self.store.envelope();
        let transitions: Vec<(&store::os_spr::MutationEnvelope, store::os_spr::HistoryTransition)> =
            envelope.transitions.iter().filter_map(|transition| store::os_spr::history_transition_from_envelope(transition).ok().flatten().map(|decoded| (transition, decoded))).collect();
        let facts: Vec<AlternativeFact<'_>> = transitions
            .iter()
            .filter_map(|(transition, decoded)| match decoded {
                store::os_spr::HistoryTransition::Branch { alternative_id, .. } => Some(AlternativeFact::Branch { alternative_id, actor: &transition.actor.0, physical_ms: transition.timestamp.physical_ms }),
                store::os_spr::HistoryTransition::Supersede(supersede) => Some(AlternativeFact::Supersede { scope: supersede.scope.as_deref() }),
                _ => None,
            })
            .collect();
        let line = self.store.active_line_id();
        history_alternative_views(envelope.vcs.alternatives.iter().map(|alternative| (alternative.id.as_str(), alternative.name.as_str())), Some(line.as_str()), &facts)
    }

    /// 🗂️ The history view of the current command log and stores (design §20.14): the cached one patched in place where
    /// the change only appended, amended, undid or redid edits and rows ([`Self::history_view_patch`]) — O(change), and in
    /// the same allocation unless a command job still shares it — else rebuilt over the whole history, reusing the previous
    /// projection's printed lines and labels. Every row whose projection changed is marked for the next history patch, so
    /// the wire carries exactly the changed rows (an undo no longer re-sends the whole history).
    async fn refresh_history_view(&mut self) -> std::sync::Arc<HistoryView> {
        let retained = self.history_view.take();
        let members = self.member_history_stamps(retained.as_ref().map_or(&[][..], |cache| cache.stamp.members.as_slice()));
        let mut stamp = HistoryViewStamp {
            document: time_travel::history_store_stamp(&self.store),
            members: members.iter().map(|(member, stamp, _)| (member.clone(), stamp.clone())).collect(),
            newest_seq: self.command_log.iter().next_back().map_or(0, |entry| entry.seq),
            rows: self.command_log.len(),
            inverse_rows: 0,
            shell_undone: self.shell_undone.len(),
            supersede_records: self.supersedes.records.len(),
        };
        let patch = retained.as_ref().and_then(|cache| self.history_view_patch(&cache.stamp, &stamp, &members).map(|patch| (patch, cache.stamp.inverse_rows)));
        let (view, changed) = match (retained, patch) {
            (Some(HistoryViewCache { mut view, .. }), Some((patch, inverse_rows))) => {
                stamp.inverse_rows = inverse_rows + self.command_log.iter().rev().take(self.command_log.len() - patch.fresh_from).filter(|entry| entry.inverse.is_some()).count();
                let changed = self.patch_history_view(std::sync::Arc::make_mut(&mut view), patch, stamp.inverse_rows).await;
                (view, changed)
            }
            (retained, _) => {
                stamp.inverse_rows = self.command_log.iter().filter(|entry| entry.inverse.is_some()).count();
                let previous = retained.as_ref().map(|cache| cache.view.as_ref());
                let view = self.build_history_view(previous).await;
                let held: HashMap<u64, &CommandView> = previous.map_or_else(HashMap::new, |previous| previous.commands.iter().map(|row| (row.seq, row)).collect());
                let changed = view.commands.iter().filter(|row| held.get(&row.seq).is_none_or(|before| *before != *row)).map(|row| row.seq).collect();
                (std::sync::Arc::new(view), changed)
            }
        };
        self.history_dirty_sequences.extend(changed);
        self.history_view = Some(HistoryViewCache { view: std::sync::Arc::clone(&view), stamp });
        view
    }

    /// 🧾️ Converts the authoritative command log into a typed host projection patch. Rows are
    /// never folded or capped here; the host controls its own visible window. A delta patch reads only its dirty rows
    /// (each found by its `seq` in the newest-first view, design §20.14) and shares the cached view, never copies it.
    pub(crate) async fn history_patch(&mut self, snapshot: bool) -> Result<HistoryPatch, Fault> {
        self.refresh_cache().await?;
        let history = std::sync::Arc::clone(&self.cache.as_ref().expect("history cache refreshed").3);
        let dirty = std::mem::take(&mut self.history_dirty_sequences);
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
            cursor: self.log_generation,
            upserts,
            can_undo: history.can_undo,
            can_redo: history.can_redo,
            active_alternative_id: history.active_alternative_id.clone(),
            current_checkpoint_id: history.current_checkpoint_id.clone(),
            command_filter: history.command_filter.value().into(),
            time_travel: time_travel.map(|panel| panel.status),
            reprojection: self.reprojection_status(),
            edit_count: u32::try_from(self.store.envelope().vcs.edits.len()).unwrap_or(u32::MAX),
        })
    }

    /// 🗂️ Refreshes the snapshot cache if the store advanced, the command log grew/folded, or
    /// the history filter changed since the last materialization. The key is recomputed a SECOND time
    /// after `backfill_command_log` — backfill itself may `push_log_entry` (bumping `log_generation`),
    /// so keying only on the pre-backfill snapshot would store a stale key and thrash on every call.
    ///
    /// 🧾️ The history view is patched, not rebuilt ([`Self::refresh_history_view`], design §20.14): the cache's own
    /// share of it is dropped first, so the retained view stays the only owner and is patched in its own allocation.
    pub(crate) async fn refresh_cache(&mut self) -> Result<(), Fault> {
        while self.command_prune_requested() || self.pending_command_prune.is_some() {
            self.advance_command_prune_step(self.mounted_policy.maintenance, false)?;
            plugin_job_yield_once().await;
        }
        let key = (self.store.generation(), self.config_store.generation(), self.log_generation, self.history_filter);
        if self.cache.as_ref().map(|(cached_key, _, _, _)| *cached_key) == Some(key) {
            return Ok(());
        }
        self.backfill_command_log().await;
        let key = (self.store.generation(), self.config_store.generation(), self.log_generation, self.history_filter);
        let (snapshot, config) = match self.cache.take() {
            Some((cached_key, snapshot, config, history)) => {
                drop(history);
                (if cached_key.0 == key.0 { snapshot } else { self.store.snapshot_owner() }, if cached_key.1 == key.1 { config } else { self.config_store.snapshot_owner() })
            }
            None => (self.store.snapshot_owner(), self.config_store.snapshot_owner()),
        };
        let history = self.refresh_history_view().await;
        self.cache = Some((key, snapshot, config, history));
        self.follow_tool_machines(false);
        Ok(())
    }

    /// 🎯️ Shares immutable cache inputs with a command job without cloning the document or history.
    pub(crate) fn command_cache_inputs(&self) -> (std::sync::Arc<A::Snapshot>, std::sync::Arc<A::Config>, std::sync::Arc<HistoryView>) {
        let (_, snapshot, config, history) = self.cache.as_ref().expect("cache refreshed before command scheduling");
        (std::sync::Arc::clone(snapshot), std::sync::Arc::clone(config), std::sync::Arc::clone(history))
    }

    /// 🖋️ Decodes `args.authors` (`[{id, name, avatar?}]`, both shells' wire shape for a
    /// checkpoint dispatch) into `vcs::Author`s — empty when absent or malformed, never a hard
    /// error: an authorless checkpoint stays valid, it just can't say who checked in.
    async fn history_command_authors(args: Option<&DslValue>) -> Vec<vcs::Author> {
        args.and_then(|value| value.get("authors")).and_then(|value| <Vec<vcs::Author> as semio_framework_value::FromValue>::from_value(value.clone()).ok()).unwrap_or_default()
    }

    /// 🕰️ Maps one of the six injected history action ids to its `ArtifactCommand`.
    pub(super) async fn history_command(action: &str, args: Option<&DslValue>) -> Option<ArtifactCommand<A::Mutation>> {
        let arg_str = |key: &str| args.and_then(|value| value.get(key)).and_then(DslValue::as_str).map(str::to_string);
        match action {
            "undo" => Some(ArtifactCommand::Undo),
            "redo" => Some(ArtifactCommand::Redo),
            "commitCheckpoint" => Some(ArtifactCommand::CommitCheckpoint { message: arg_str("message"), authors: Self::history_command_authors(args).await }),
            "createAlternative" => Some(ArtifactCommand::CreateAlternative { name: arg_str("name").unwrap_or_else(|| "Alternative".into()) }),
            "switchAlternative" => arg_str("alternativeId").map(|alternative_id| ArtifactCommand::SwitchAlternative { alternative_id }),
            "checkoutCheckpoint" => arg_str("checkpointId").map(|checkpoint_id| ArtifactCommand::CheckoutCheckpoint { checkpoint_id }),
            _ => None,
        }
    }

    pub(super) async fn reset_dispatch_report(&mut self) {
        self.dispatch_report = protocol::DispatchReport { policy: self.store.merge_policy(), worst: None, messages: Vec::new() };
    }

    async fn record_dispatch_receipt(&mut self, receipt: store::CommandReceipt) {
        self.dispatch_report = protocol::DispatchReport { policy: self.store.merge_policy(), worst: receipt.worst, messages: receipt.messages.into_iter().flat_map(|entry| entry.messages).collect() };
    }

    async fn record_rejected_dispatch(&mut self, policy: protocol::MergePolicy, messages: Vec<protocol::MutationMessage>) -> Fault {
        self.dispatch_report = protocol::DispatchReport { policy, worst: protocol::worst_level(&messages), messages };
        Fault::new(FaultOrigin::App, FaultCode::new("app.command.rejected"), "mutation rejected by the active merge policy")
    }

    pub(super) async fn apply_merge_policy(&mut self, policy: protocol::MergePolicy) {
        self.store.set_merge_policy(policy);
        self.reset_dispatch_report().await;
    }

    pub(super) async fn resolve_store_conflict(&mut self, conflict_id: &str, resolution: protocol::ConflictResolution) -> Result<protocol::MergeReport, Fault> {
        let report = self.store.resolve_conflict(conflict_id, resolution).await.map_err(|error| error.into_fault())?;
        self.dispatch_report = protocol::DispatchReport { policy: report.policy, worst: report.worst, messages: report.replayed.iter().flat_map(|entry| entry.messages.iter().cloned()).collect() };
        self.cache = None;
        Ok(report)
    }

    pub(super) async fn current_open_conflicts(&self) -> Vec<protocol::Conflict> {
        self.store.open_conflicts().cloned().collect()
    }

    /// 🔍️ Pure dry run (§C9): folds `ops` over the CURRENT persisted document snapshot via
    /// `SpaceMember::preview_wire` — never applies anything, never touches `self.dispatch_report` —
    /// and packages the resulting messages under the store's active `MergePolicy` as a
    /// `DispatchReport`. This is the single predicate a host/UI consults to grey out an action or
    /// context-menu item BEFORE a real dispatch is even attempted, so the "alone ⇒ illegal actions
    /// are prevented up front" half of the ticket holds without a round trip through `dispatch_emit`.
    pub async fn preview(&self, ops: &[A::Mutation]) -> protocol::DispatchReport {
        let mut op_bytes: Vec<Vec<u8>> = Vec::with_capacity(ops.len());
        for op in ops.iter() {
            op_bytes.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
        }
        let messages = SpaceMember::preview_wire(&self.store, &op_bytes).await;
        protocol::DispatchReport { policy: self.store.merge_policy(), worst: protocol::worst_level(&messages), messages }
    }

    /// 📇️ An empty `InvocationResult` carrying only host effects/events (view/shell actions,
    /// no-operation commands, and history notifications produce no `KernelMutation`s).
    pub(super) async fn empty_result(verb: &str, meta: &ActionMeta, effects: Vec<Effect>, events: Vec<AppEvent>, ui_scope: UiDirtyScope) -> InvocationResult {
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

    /// 🧱️ Builds the `InvocationResult` for a just-dispatched edit: one `KernelMutation` per
    /// forward operation NEW in this dispatch (`tail_offset`), each carrying just this dispatch's
    /// `inverse` as its inverse diff. For a streamed transaction's open edit, `edit_mutations()` returns
    /// the WHOLE accumulated edit — without slicing to `tail_offset`, every dispatch would rebuild and
    /// serialize every `KernelMutation` since the gesture started (O(edit-size) per dispatch, O(edit-
    /// size²) over the whole gesture) purely to report operations the caller already knows about.
    ///
    /// 🎯️ B5: real binary — each backward op's own `OpBinary::encode_op()`, framed as a
    /// binary ops-vec (`protocol::encode_ops_vec`, replacing the old `json!({"inverse":
    /// [...]})` convention). Every op in `inverse` shares the same encoding; a decode
    /// failure here would mean a real corrupt/foreign operation, not a schema choice, so
    /// `unwrap_or_default` on an individual encode failure degrades to an empty inverse
    /// (same fallback behavior `payload: Vec::new()` already has elsewhere) rather than
    /// panicking mid-invocation.
    ///
    /// 🎯️ W6 kernel unification: `mutation_meta` entries are `protocol::MutationMeta`,
    /// and this kernel's own `ArtifactDiff`/`UndoPolicy`/`HybridLogicalTimestamp` are now
    /// `pub use` re-exports of the SAME `protocol`/`protocol_core` types (see
    /// `framework/core`'s kernel cut-over note) — no bridging left to do, just direct
    /// field moves. `ArtifactDiff.schema`/`.payload` are `SchemaId`/`Vec<u8>` now (was
    /// `schema_id`/`Value`), so the payload is JSON-encoded to bytes at construction.
    async fn result_from_last_edit(&self, verb: &str, meta: &ActionMeta, effects: Vec<Effect>, events: Vec<AppEvent>, ui_scope: UiDirtyScope, tail_offset: (usize, usize)) -> InvocationResult {
        let schema = A::DOCUMENT_SCHEMA.to_string();
        let invocation_id = InvocationId(format!("{verb}:{}:{}", meta.instance_id, self.store.generation()));
        let document = ArtifactHandle(meta.instance_id as u128);
        let mut mutations: Vec<KernelMutation> = Vec::new();
        if let Some((forwards, inverse, mutation_meta)) = self.store.edit_mutations() {
            let (forwards_offset, backwards_offset) = tail_offset;
            let forwards = &forwards[forwards_offset.min(forwards.len())..];
            let mutation_meta = &mutation_meta[forwards_offset.min(mutation_meta.len())..];
            let mut inverse_op_bytes = Vec::with_capacity(inverse.len().saturating_sub(backwards_offset));
            for op in inverse.iter().skip(backwards_offset) {
                inverse_op_bytes.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
            }
            let inverse_payload = protocol::encode_ops_vec(&inverse_op_bytes);
            for (index, forward) in forwards.iter().enumerate() {
                let entry = mutation_meta.get(index);
                let mutation_id = entry.and_then(|entry_meta| entry_meta.mutation_id.clone()).unwrap_or_else(|| MutationId(format!("{}:{index}", invocation_id.0)));
                let base_version = ArtifactVersion(entry.map_or(0, |entry_meta| entry_meta.base_version));
                let undo_policy = entry.map_or(UndoPolicy::ExactBaseOnly, |entry_meta| entry_meta.undo_policy);
                let author = entry.and_then(|entry_meta| entry_meta.author_id.clone()).unwrap_or_else(|| ActorId(meta.actor.clone()));
                let timestamp = entry.map_or_else(|| HybridLogicalTimestamp::new(0, 0), |entry_meta| entry_meta.timestamp);
                mutations.push(KernelMutation {
                    id: mutation_id.clone(),
                    document,
                    base_version,
                    invocation_id: invocation_id.clone(),
                    diff: ArtifactDiff { schema: SchemaId(format!("{schema}.operation")), payload: ::protocol::OpBinary::encode_op(forward).unwrap_or_default() },
                    inverse: InverseMutation {
                        target_mutation: mutation_id,
                        inverse_diff: ArtifactDiff { schema: SchemaId(format!("{schema}.operation.inverse")), payload: inverse_payload.clone() },
                        base_version,
                        dependencies: Vec::new(),
                        undo_policy,
                    },
                    dependencies: Vec::new(),
                    author,
                    timestamp,
                });
            }
        }
        let mutation_ids: Vec<MutationId> = mutations.iter().map(|operation| operation.id.clone()).collect();
        let inverse_mutations: Vec<InverseMutation> = mutations.iter().map(|operation| operation.inverse.clone()).collect();
        InvocationResult {
            output: DslValue::Null,
            mutations,
            inverse_group: UndoGroup { invocation_id, mutations: mutation_ids, inverse_mutations, member_edits: Vec::new() },
            diagnostics: Vec::new(),
            requested_effects: effects,
            events,
            ui_scope,
            history_patch: None,
        }
    }

    /// 🧬️ Shared dispatch tail for `handle_action`/`handle_command`/`import_media`: given the
    /// app's `ActionEmit`, either records the op-less dispatch and returns an empty result, or commits
    /// `Apply` or the transaction commands, records the resulting edit, and builds the `InvocationResult` from it.
    /// `verb` is the action/command id, used to resolve the registry kind/label and to synthesize the
    /// `InvocationId`.
    /// 🧬️ B1: the single dispatch tail for BOTH `dispatch_typed_command` (the app's own
    /// `Emit`) and the framework-reserved clipboard actions below — commits `artifact_mutations` to
    /// the document store and `config_mutations` to the config store (independently, each its own
    /// undo stack), records exactly one command-log row carrying whichever edit id(s) were produced,
    /// and builds the resulting `InvocationResult` from the document side (config-only dispatches
    /// never touch `KernelMutation`/space-sync — see `🔖️CommandLog`'s doc region for why).
    /// 🔀️ Builds a typed rejection `Fault` using one of the frozen transaction rejection codes
    /// (contract-freeze.md §5's "Rejection taxonomy") — shared by `dispatch_emit`'s freeze check
    /// and every `transaction_prepare`/`transaction_commit`/`transaction_rollback`/
    /// `transaction_undo`/`transaction_redo` rejection path below.
    /// 🧮️ B1: `materialize_args` (JSON-args default-fill + required-arg enforcement for app-declared
    /// actions/commands, plus its now-unused `effective_action_args`/`missing_required_args` imports)
    /// was deleted — dead code once `dispatch_action`'s generic app-action fallback and
    /// `dispatch_command`'s registry-backed dispatch were removed (an app's own behavior now dispatches
    /// exclusively through the typed `Self::Command` channel, where a Rust caller supplies a complete
    /// value — there is no "missing arg" to materialize).
    ///
    /// 🚫️async: E1 pure error constructor consumed pervasively by sync closures — see R9.
    pub(super) fn transaction_fault(origin: FaultOrigin, code: &'static str, message: impl Into<String>) -> Fault {
        Fault::new(origin, FaultCode::new(code), message)
    }

    /// 🔁️ Drains `emit.extension_invocations` through the guest request registry: each entry
    /// allocates a `RequestId`, records its `response_action` continuation against that id and
    /// queues the resulting `Effect::InvokeExtension` on the registry's own outbound queue (the
    /// one `reactor::poll` drains into `turn-result.effects` later in the SAME turn). Also the
    /// fail-closed gate on the defect this lane replaces: an app that smuggles a hand-built
    /// `Effect::InvokeExtension` into `emit.effects` is rejected outright rather than having its
    /// result silently dropped at `Event::Completed` for an id that owns no registry slot.
    pub(super) fn mint_extension_invocations(instance: u32, emit: &mut Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>) -> Result<(), Fault> {
        if emit.effects.iter().any(|effect| matches!(effect, Effect::InvokeExtension { .. })) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unminted-extension-invocation"), "Effect::InvokeExtension carries a registry-minted request id — emit it through Emit::extension_invocations instead"));
        }
        for invocation in std::mem::take(&mut emit.extension_invocations) {
            crate::reactor::queue_extension_invocation(instance, &invocation)?;
        }
        Ok(())
    }

    /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — the reactor-side fold of a guest emit's
    /// interaction verbs: every `Effect::ReplayShellCommand { action_id ∈ INTERACTION_ACTION_IDS }`
    /// and every `Effect::DispatchAction { action ∈ INTERACTION_ACTION_IDS }` (the same intent in
    /// `📏️layout`/`💡️reasoning`'s emission shape, any `delay_ms`) is peeled BEFORE the effects
    /// are handed out ([`take_inline_interaction_verbs`]), the emit
    /// lands exactly as before ([`Self::dispatch_emit_inner`]), and then — after this command's
    /// own store lanes and `interaction_writes` landed, so a just-created id is already in
    /// topology — each peeled verb is applied inline ([`Self::fold_inline_interaction_verbs`]).
    /// The result leaves with the verbs' scopes merged into its `ui_scope`, the last verb's
    /// leftover `InteractionView` as its `output` when the carrier produced none (the SAME
    /// `Invocation.output.interactionView` lane a host-dispatched verb publishes on, so the
    /// world overlay still lands), and one diagnostic per verb that could not be applied.
    /// A failed carrier applies no verb: its pick did not happen. The typed-operation ladder,
    /// which never enters here, has its own twin ([`Self::publish_mounted_typed_inline_interaction_unit`]).
    pub(crate) async fn dispatch_emit(&mut self, verb: &str, mut emit: Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        if !emit.owned_child_emits.is_empty()||emit.owned_child_emits.capacity()!=0||!emit.child_preparations.is_empty()||emit.child_preparations.capacity()!=0{
            return self.mount_original_emit_publication(verb, emit, meta, None, cx).await;
        }
        let folded = take_inline_interaction_verbs(&mut emit.effects);
        let mut result = self.dispatch_emit_inner(verb, emit, meta, identity).await?;
        if folded.is_empty() {
            return Ok(result);
        }
        let scope = std::mem::replace(&mut result.ui_scope, UiDirtyScope::None);
        let fold = self.fold_inline_interaction_verbs(verb, folded, meta, scope, identity).await;
        result.ui_scope = fold.scope;
        if let (DslValue::Null, Some(leftover)) = (&result.output, fold.leftover) {
            result.output = leftover;
        }
        result.diagnostics.extend(fold.diagnostics);
        Ok(result)
    }

    /// 🧵️ The task lane belongs to the MIGRATED publication ladder
    /// (`publish_mounted_typed_operation_unit`), which owns the per-operation cancellation
    /// lease a spawned task must be cancellable under and spends exactly one task per
    /// bounded publication unit. This unmigrated route has neither, so a task emitted here
    /// is refused by name rather than spawned unowned.
    ///
    /// 🔒️ Contract §2.3 clause 2 — hard backstop, unreachable through `ViewerApp` (its `handle`
    /// builds `Emit` solely from `ViewEmit`'s three fields, contract §2.2), but this is what
    /// makes a hand-written runtime `ArtifactApp` impl that sets `ROLE = Viewer` safe too.
    ///
    /// 🔀️ Transaction freeze (contract §5.10): while a transaction is pending on this
    /// instance, reject anything that would emit artifact mutations rather than applying or
    /// queuing it — config/draft-only and op-less dispatches are unaffected. Read-only
    /// commands (RefreshUi/ReadDocument/ContextMenu/ephemeral lanes) never reach
    /// `dispatch_emit` at all, so they are unaffected by construction.
    ///
    /// 🔀️ Composite-mutation proposal (contract §5.1): when any artifact mutation carries
    /// foreign steps (folded forward exactly like `ArtifactStore::replay_mutations` folds its
    /// own snapshot, via the same public `Mutation::diff`/`protocol::apply_diff` pair — the
    /// private `apply_mutation` helper lives across the crate boundary in `store`), apply
    /// NOTHING — stash a `TransactionProposalDraft` instead, drained once by `plugin_exchange`
    /// (`take_pending_transaction_proposal`) and framed as `AppFrame::TransactionProposal`
    /// rather than the ordinary `Invocation`/`Done` frame.
    ///
    /// 📝️ Draft lane — ephemeral; applied without command-log rows (never checkpoints).
    ///
    /// 🚦️ Kind discipline, enforced where the operations actually arrive: a migrated dispatch hands
    /// the reducer to a worker, so the `View`/`🐚️Shell`-kind refusal `ArtifactApp::command_id`'s own
    /// doc promises can only be made against the published emit, never against the admission receipt.
    ///
    /// 🧮️ Config side dispatches first, independent of whether this verb ALSO touches the document
    /// — captures the resulting (possibly amended) config edit id for the command-log row below.
    ///
    /// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1): a non-empty `child_emits` means this
    /// gesture must land as ONE atomic multi-document group — routed entirely through
    /// `dispatch_emit_group` (parent ops included), never through the solitary single-store
    /// path below. See that method's own doc comment for why the two paths stay genuinely
    /// separate rather than being unified into one (the group protocol has no streamed open edit).
    ///
    /// 🧾️ An app-declared action logs under its declared kind (so an `Mutation`-kind action
    /// that happened to produce zero document operations — e.g. paste with no clipboard
    /// fragment, or a pure config-op dispatch — still gets a real row, `edit_id: None`,
    /// correctly filed under "Without Operations"); a declared command logs under its own
    /// independently declared kind; anything unresolved (registry-less test construction, an ad-hoc
    /// verb like an import-media port id) logs as `View`.
    ///
    /// 🪟️ A `View`-kind dispatch whose ONLY emission is the per-window config lane is session
    /// view state of one window (its camera pose, grid/LOD/selectable-kind options), not a
    /// command: the window-config store keeps its own per-window lane and nothing here is
    /// revertible, so a row would only pad the artifact command history with one entry per
    /// orbit tick. A `View` action that reaches the document or the shared config store still
    /// logs — `select`, whose emission is a config op, is unaffected.
    ///
    /// 🪢️ Captured before dispatch so `result_from_last_edit` can report only the operations THIS dispatch
    /// added — if a streamed tick grows the same open edit (`before_edit_id` unchanged after dispatch), these
    /// are the tail offsets into that edit's now-longer forwards/inverse; if a new edit was created
    /// instead, the offsets are moot (checked via edit identity, not reused blindly).
    ///
    /// 🕹️ Task 4: after EVERY artifact (document) dispatch, re-derive fresh topology and prune
    /// any selection/hover id no longer present — a document edit that deleted a node must not
    /// leave it lingering in another window's selection.
    ///
    /// 🧾️ One command-log entry per VCS edit — a coalesced gesture (`amended_same_edit`) grows the
    /// existing entry's `op_lines` live (see `build_history_view`), it never appends a new entry. The
    /// entry is logged BEFORE the interaction state is revalidated: a `Topology` domain refreshes the
    /// history cache, whose backfill would otherwise file the new edit under `apply` first and this
    /// dispatch would log it a second time (a committed typing run showed as two rows of one edit).
    ///
    /// ⏪️ No memory `inverse` for an edit-linked row — the VCS edit's own `Mutation::inverse`
    /// is already the real inverse, and `revertToCommand`'s edit_id branch replays that.
    async fn dispatch_emit_inner(&mut self, verb: &str, mut emit: Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        Self::mint_extension_invocations(meta.instance_id, &mut emit)?;
        let Emit { artifact_mutations, config_mutations, window_config_mutations, draft_mutations, transaction, transaction_phase, effects, extension_invocations, events, ui_scope, child_emits, owned_child_emits, child_preparations, interaction_writes, tasks } = emit;
        assert!(owned_child_emits.is_empty()&&owned_child_emits.capacity()==0,"applying retained children require their staged publication owner");
        assert!(child_preparations.is_empty()&&child_preparations.capacity()==0,"dispatch owns only completely prepared child operation groups");
        if let Some(fault) = tool_transaction_shape_fault(verb, transaction.as_ref(), transaction_phase, !child_emits.is_empty(), !artifact_mutations.is_empty(), artifact_mutations.iter().any(Mutation::may_emit_foreign_steps)) {
            return Err(fault);
        }
        let effects = self.stamp_load_document_effects(effects)?;
        debug_assert!(extension_invocations.is_empty(), "mint_extension_invocations drains the lane before destructuring");

        if !tasks.is_empty() {
            return Err(Fault::new(
                FaultOrigin::Framework,
                FaultCode::new("interactive-job.task-lane-unmigrated-route"),
                format!(
                    "verb {verb:?} emitted {} async task(s) on the unmigrated dispatch route; the task lane is published by the migrated typed-operation ladder, which owns the cancellation lease every spawned task is retired under",
                    tasks.len()
                ),
            ));
        }

        if A::ROLE == AppRole::Viewer && !artifact_mutations.is_empty() {
            return Err(viewer_read_only_fault(verb));
        }

        if !artifact_mutations.is_empty() && self.tool_runs.freezes_local_emits() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.busy"), format!("verb {verb:?} would emit artifact mutations while a freezing tool run is non-terminal on this instance")));
        }

        if (!artifact_mutations.is_empty() || !child_emits.is_empty()) && self.time_travel.freezes_local_emits() {
            return Err(time_travel_frozen_fault(verb));
        }

        if !artifact_mutations.is_empty() && self.pending_transaction.is_some() {
            let pending_txn_id = self.pending_transaction.as_ref().map(|pending| pending.txn_id.clone()).unwrap_or_default();
            return Err(Self::transaction_fault(FaultOrigin::Plugin, "transaction.instance-busy", format!("verb {verb:?} would emit artifact mutations while transaction {pending_txn_id:?} is pending on this instance")));
        }

        if artifact_mutations.iter().any(Mutation::may_emit_foreign_steps) {
            let mut running = self.store.snapshot().map_err(|error| error.into_fault())?;
            let foreign = Vec::new();
            for op in &artifact_mutations {
                if op.foreign_step_source(&running, 0).map_err(|error| error.into_fault())?.is_some() { return Err(plugin_sdk_fault("foreign transaction output awaits its original staged preparation recipient")); }
                let outcome = op.diff(&running);
                if !outcome.is_applicable(protocol::MergePolicy::default()) {
                    return Err(Self::transaction_fault(FaultOrigin::App, "transaction.member-rejected", format!("mutation outcome was rejected: {:?}", outcome.messages())));
                }
                running = protocol::apply_diff(outcome.diff(), &running).map_err(|error| Self::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string()))?;
            }
            if !foreign.is_empty() {
                let mut local_ops = Vec::with_capacity(artifact_mutations.len());
                for op in artifact_mutations.iter() {
                    local_ops.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
                }
                self.pending_transaction_proposal = Some(TransactionProposalDraft { local_ops, foreign });
                return Ok(Self::empty_result(verb, meta, effects, events, ui_scope).await);
            }
        }

        let mut artifact_op_bytes = Vec::with_capacity(artifact_mutations.len());
        for op in artifact_mutations.iter() {
            artifact_op_bytes.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
        }
        let mut config_op_bytes = Vec::with_capacity(config_mutations.len());
        for op in config_mutations.iter() {
            config_op_bytes.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
        }
        let mut draft_op_bytes = Vec::with_capacity(draft_mutations.len());
        for op in draft_mutations.iter() {
            draft_op_bytes.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
        }
        self.last_emit_wire = Some(EmitWire { document: protocol::encode_ops_vec(&artifact_op_bytes), config: protocol::encode_ops_vec(&config_op_bytes), draft: protocol::encode_ops_vec(&draft_op_bytes), children: Vec::new() });

        if !draft_mutations.is_empty() {
            self.draft_store.dispatch(ArtifactCommand::Apply { mutations: draft_mutations, transaction: None }, identity).await.map_err(|error| error.into_fault())?;
        }

        let declared_kind = self.declared_dispatch_kind(verb);
        if !artifact_mutations.is_empty() {
            self.require_operation_emitting_kind(verb, declared_kind)?;
        }

        let config_edited = !config_mutations.is_empty();
        if config_edited {
            let config_command = ArtifactCommand::Apply { mutations: config_mutations, transaction: None };
            self.config_store.set_authoring_verb(Some(verb.to_string()));
            let dispatched = self.config_store.dispatch(config_command, identity).await;
            self.config_store.set_authoring_verb(None);
            dispatched.map_err(|error| error.into_fault())?;
        }

        if !window_config_mutations.is_empty() {
            let authority = self
                .window_config_store
                .capture(meta.view_state.as_ref())
                .await?
                .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.window-context"), "window config emission requires one exact registered ViewModel window authority"))?;
            for mutation in window_config_mutations {
                self.window_config_store.dispatch(&authority, &meta.actor, mutation, identity).await?;
            }
            self.tool_runs.note_window_config_published();
        }
        #[cfg(test)]
        debug_assert!(tasks.is_empty());

        if !child_emits.is_empty() {
            let result = self.dispatch_emit_group(verb, &artifact_mutations, &child_emits, effects, events, ui_scope, meta, None, transaction, identity).await?;
            self.apply_interaction_writes(&interaction_writes, meta, identity).await?;
            return Ok(result);
        }

        let in_transaction = transaction.is_some();
        let log_label = self.verb_label(verb);
        let vcs_commands = artifact_lane_commands(artifact_mutations, transaction, transaction_phase, self.store.open_transaction());
        if vcs_commands.is_empty() {
            let kind = match self.invocation_kind {
                Some(kind) => kind,
                None => match self.registry.get(verb).map(|definition| definition.kind) {
                    Some(kind) => kind,
                    None => match self.registry.get_command(verb) {
                        Some(command) => command.kind,
                        None => ActionKind::View,
                    },
                },
            };
            if !in_transaction && !config_edited {
                self.record_command(verb, kind, log_label, None, Vec::new(), None);
            }
            self.apply_interaction_writes(&interaction_writes, meta, identity).await?;
            return Ok(Self::empty_result(verb, meta, effects, events, ui_scope).await);
        }
        let aborting = matches!(vcs_commands.as_slice(), [ArtifactCommand::AbortTransaction { .. }]);
        let before_edit_id = self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone());
        let (before_forwards_len, before_backwards_len) = self.store.edit_mutations().map_or((0, 0), |(f, b, _)| (f.len(), b.len()));
        for vcs_command in vcs_commands {
            self.store.set_authoring_verb(Some(verb.to_string()));
            let dispatched = self.store.dispatch(vcs_command, identity).await;
            self.store.set_authoring_verb(None);
            match dispatched {
                Ok(receipt) => self.record_dispatch_receipt(receipt),
                Err(vcs::VcsError::Rejected { policy, messages }) => return Err(self.record_rejected_dispatch(policy, messages).await),
                Err(error) => return Err(error.into_fault()),
            }
            .await;
        }
        if aborting {
            self.revalidate_interaction_on_document_change(meta, identity).await?;
            self.apply_interaction_writes(&interaction_writes, meta, identity).await?;
            self.retire_displaced_document_rows();
            return Ok(Self::empty_result(verb, meta, effects, events, ui_scope).await);
        }
        let amended_same_edit = before_edit_id.is_some() && self.store.envelope().vcs.edits.last().map(|edit| &edit.id) == before_edit_id.as_ref();
        if !amended_same_edit {
            if let Some(edit_id) = self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone()) {
                let kind = match self.invocation_kind {
                    Some(kind) => kind,
                    None => match self.registry.get(verb) {
                        Some(def) => def.kind,
                        None => match self.registry.get_command(verb) {
                            Some(def) => def.kind,
                            None => ActionKind::Mutation,
                        },
                    },
                };
                self.record_command(verb, kind, log_label, Some(edit_id), Vec::new(), None);
            }
        }
        self.revalidate_interaction_on_document_change(meta, identity).await?;
        self.apply_interaction_writes(&interaction_writes, meta, identity).await?;
        let tail_offset = if amended_same_edit { (before_forwards_len, before_backwards_len) } else { (0, 0) };
        Ok(self.result_from_last_edit(verb, meta, effects, events, ui_scope, tail_offset).await)
    }

    /// 🎗️ The store half of a migrated emission that closes a streamed tool transaction without new operations (design
    /// §15): an abort reverts the transaction's open edit and retires its history row, a commit with no last operations
    /// closes the edit (its announcement leaves with the run's next flush). Either is consumed without suspending, so the
    /// one-page publication continues with the emission's other lanes; a closing commit with operations and every stream
    /// are left to the batched route. A commit carrying composed-child emissions keeps its ref: the child group stamps it
    /// on every member op (design §12), so a child-only tool transaction is still one transaction row.
    pub(super) fn close_streamed_transaction_unit(&mut self, emit: &mut Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>) -> Result<(), Fault> {
        let Some(transaction) = emit.transaction.as_ref() else { return Ok(()) };
        let abort = emit.transaction_phase == TransactionPhase::Abort;
        if !abort && (emit.transaction_phase == TransactionPhase::Stream || !emit.artifact_mutations.is_empty()) {
            return Ok(());
        }
        let open = self.store.open_transaction().is_some_and(|open| open.transaction.id == transaction.id);
        let transaction_id = transaction.id.clone();
        if abort || emit.child_emits.is_empty() {
            emit.transaction = None;
            emit.transaction_phase = TransactionPhase::Commit;
        }
        if !open {
            return Ok(());
        }
        if abort {
            self.store.abort_open_transaction(&transaction_id).map_err(|error| error.into_fault())?;
            self.retire_displaced_document_rows();
        } else {
            self.store.commit_open_transaction(&transaction_id).map_err(|error| error.into_fault())?;
        }
        Ok(())
    }

    /// 🚧️ The typed refusal of a batched artifact publication while a tool transaction it does not carry is open on the
    /// document store (the store refuses its admission too, untyped).
    pub(super) fn open_transaction_admission_fault(&self, transaction: Option<&protocol::TransactionRef>) -> Option<Fault> {
        let open = self.store.open_transaction()?;
        transaction.is_none_or(|transaction| transaction.id != open.transaction.id).then(|| vcs::VcsError::TransactionOpen { transaction_id: open.transaction.id.clone() }.into_fault())
    }

    /// 🌱️ Absorbs every `store::os_store::composition_coordinator::GroupReceipt::created_children` entry into the live
    /// `self.children` map — B2 flagged that a `ChildGenesis`-created member has no pre-existing
    /// caller-held reference the way an already-registered child does, so without this it would
    /// be silently dropped the moment `dispatch_group` returns, making `ChildGenesis` pointless
    /// (see `GroupReceipt::created_children`'s own doc comment). The slot name is recovered from
    /// `self.composition`'s own graph (`dispatch_group`'s phase 2 already called `insert_owns`
    /// for every genesis child before returning the receipt, so `slot_of` is always populated
    /// here). A free-standing method (not inlined into `dispatch_emit_group`) so it is directly
    /// unit-testable against a synthetic `created_children` list, independent of the `ChildEmit`
    /// wire path (which never emits a `ChildGenesis` itself today).
    pub(crate) async fn absorb_created_children(&mut self, created_children: Vec<(ArtifactRef, M)>) -> Result<(), Fault> {
        for (target, mut member) in created_children {
            let publication_generation = self.admit_child_content_publication()?;
            let slot = self.composition.graph().await.slot_of(&target.artifact_id).await.ok_or_else(|| plugin_sdk_fault("created child has no exact target ownership slot"))?.to_owned();
            let projection = semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(self.store.snapshot_ref()).map_err(|error| plugin_sdk_fault(format!("created child projection failed: {error}")))?;
            let child_id = (0..projection.len()).filter_map(|index| projection.get(index)).find(|(declared, fields)| *declared == slot && fields.artifact_id == target.artifact_id && fields.artifact_kind == target.dialect.artifact_kind && fields.standard == target.dialect.standard && fields.subset == target.dialect.subset).map(|(_, fields)| fields.child_id.to_owned()).ok_or_else(|| plugin_sdk_fault("created child is not declared by the published parent"))?;
            ChildContentView::hash(MemberKeyRef::root(&slot, &child_id))?;
            let admission = self.children.admit(&(slot.clone(), child_id.clone()))?;
            let parent = ArtifactRef { artifact_id: self.store.envelope().id.clone(), dialect: A::DIALECT.into() };
            let owner = store::OwnerRef { parent, slot: slot.clone(), child_id: child_id.clone() };
            member.set_owner(Some(owner.clone())).await;
            self.children.insert_admitted(admission, target, owner, member);
            self.publish_child_content_member(publication_generation, &slot, &child_id).await?;
        }
        Ok(())
    }

    /// 🧩️ `dispatch_emit`'s composite-gesture branch, taken whenever `emit.child_emits` is
    /// non-empty. Routes `artifact_mutations` (the parent's own ops) plus every `ChildEmit`
    /// through `store::os_store::composition_coordinator::CompositionCoordinator::dispatch_group` as ONE atomic multi-document
    /// gesture, so a single user action spanning a parent and N owned children still produces ONE
    /// `UndoGroup`: `inverse_group.member_edits` names every touched document (parent included),
    /// and `dispatch_action`'s history-action arm routes an undo/redo of a group-tailed edit
    /// through `CompositionCoordinator::undo_group`/`redo_group` — see that arm's own doc comment.
    /// Deliberately narrower than the solitary (zero-children) path above: `dispatch_group`'s
    /// phase 2 always issues a plain `Apply` on every touched member (there is no group-aware
    /// streamed open edit), so a composite gesture with children is
    /// always ONE described edit per member, never folded across ticks — only the solitary path
    /// streams a transaction's ticks into one open edit. `inverse_group.mutations`/`.inverse_mutations`
    /// are best-effort for CHILD members specifically: real per-op inverse bytes are retrievable
    /// only through `ArtifactStore<P, Mutation>`'s own inherent `edit_mutations()`, and each
    /// child's concrete `P`/`Mutation` is erased behind the generic `M: SpaceMember` — the REAL
    /// reversal mechanism for a child member is `CompositionCoordinator::undo_group` calling
    /// `undo()` directly on the live child store (driven by `inverse_group.member_edits`, not by
    /// replaying these bytes).
    /// 🚦️ Kind discipline where the operations actually arrive. A migrated dispatch hands its
    /// reducer to a worker, so the `View`/`🐚️Shell`-kind refusal `ArtifactApp::command_id`'s own doc
    /// promises can only be made against the published emit, never against the admission receipt the
    /// caller gets back. An unresolved verb (registry-less construction, an ad-hoc port id) is not a
    /// declared `View`, so it is not refused here.
    /// 🚫️async: E1 pure classification over an already-resolved kind
    pub(super) fn require_operation_emitting_kind(&self, verb: &str, declared_kind: Option<ActionKind>) -> Result<(), Fault> {
        match declared_kind {
            Some(kind @ (ActionKind::View | ActionKind::Shell)) => Err(Fault::from(format!("{kind:?}-kind command '{verb}' must not emit operations"))),
            _ => Ok(()),
        }
    }

    /// 🏷️ The kind a verb dispatches under: the in-flight invocation's own kind when one is
    /// mounted, otherwise the action declaration, otherwise the command declaration.
    /// 🚫️async: E1 pure registry lookup
    pub(super) fn declared_dispatch_kind(&self, verb: &str) -> Option<ActionKind> {
        match self.invocation_kind {
            Some(kind) => Some(kind),
            None => self.registry.get(verb).map(|definition| definition.kind).or_else(|| self.registry.get_command(verb).map(|command| command.kind)),
        }
    }

    /// 🧩️ Commits an agent transaction that carries owned-child op groups as ONE composite gesture
    /// (`dispatch_emit_group`, the same path the shell lane's child-group verbs take) whose group identity is
    /// `txn_id` on every member it touches, so the gateway's `TransactionUndo{group_id: txn_id}` moves them all.
    /// The parent's tail edit (when the group touched the parent) carries the prepared origin like a solitary
    /// commit does. Returns the parent's edit id, else the first touched child's.
    pub(super) async fn commit_transaction_group(&mut self, txn_id: &str, ops: Vec<A::Mutation>, children: Vec<ChildEmit>, origin: protocol::MutationOrigin, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, Fault> {
        let parent_edits_before = self.store.envelope().vcs.edits.len();
        let result = self
            .dispatch_emit_group(&format!("transaction:{txn_id}"), &ops, &children, Vec::new(), Vec::new(), UiDirtyScope::Full, meta, Some(txn_id.to_string()), None, identity)
            .await
            .map_err(|fault| Self::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", fault.message))?;
        if self.store.envelope().vcs.edits.len() > parent_edits_before {
            self.store.stamp_tail_origin(origin).await.map_err(|error| Self::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", format!("{error:?}")))?;
            return Ok(self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone()).unwrap_or_default());
        }
        Ok(result.inverse_group.member_edits.first().map(|edit| edit.edit_id.clone()).unwrap_or_default())
    }

    /// ↩️ `transaction_undo`/`transaction_redo` for a composing instance: moves every member (parent and
    /// owned children) whose tail carries `group_id` through `CompositionCoordinator::undo_group`/`redo_group`,
    /// republishing each moved child's content like the shell lane's group history does. A group no member
    /// carries is refused by name, exactly like the solitary path.
    pub(super) async fn transaction_group_history(&mut self, action: &str, group_id: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        self.admit_child_content_publication_span(self.children.len())?;
        let parent_id = self.store.envelope().id.clone();
        let parent_dialect: ArtifactDialect = A::DIALECT.into();
        if self.store.envelope().dialect.as_ref() != Some(&parent_dialect) {
            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.group-history-dialect"), "composition history requires the parent's exact declared dialect"));
        }
        let parent_ref = ArtifactRef { artifact_id: parent_id.clone(), dialect: parent_dialect };
        let child_refs: Vec<ArtifactRef> = self.children.entries_physical().map(|entry| entry.reference.clone()).collect();
        let mut members: Vec<(&ArtifactRef, &mut M)> = Vec::with_capacity(child_refs.len());
        for (reference, entry) in child_refs.iter().zip(self.children.entries_mut_physical()) {
            members.push((reference, &mut entry.member));
        }
        let report = if action == "undo" { CompositionCoordinator::undo_group(&parent_ref, &mut self.store, &mut members, group_id, identity).await } else { CompositionCoordinator::redo_group(&parent_ref, &mut self.store, &mut members, group_id, identity).await };
        drop(members);
        self.cache = None;
        for (reference, _) in report.undone.iter().filter(|(reference, _)| reference.artifact_id != parent_id) {
            let Some(key) = self.children.keyed_entries().find(|(_, entry)| entry.reference.artifact_id == reference.artifact_id).map(|(key, _)| key.to_key()) else {
                return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.group-history-root"), "group history moved a child without exact immutable-root authority"));
            };
            let publication_generation = self.admit_child_content_publication()?;
            self.publish_member_content(publication_generation, key.borrowed()).await?;
        }
        if report.undone.is_empty() {
            let skipped = report.skipped.iter().map(|(reference, error)| format!("{} ({error})", reference.artifact_id)).collect::<Vec<_>>().join(", ");
            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.group-history-tail"), format!(
                "transaction_{action}: no member of this instance carries group {group_id:?} at its {} tail{}",
                if action == "undo" { "applied" } else { "redo" },
                if skipped.is_empty() { String::new() } else { format!(" (skipped: {skipped})") }
            )));
        }
        self.record_command(action, ActionKind::History, None, None, Vec::new(), None);
        Ok(())
    }

    /// 🚧️ `ArtifactEnvelope.dialect` stays `Option<ArtifactDialect>` per B2's own DEFERRED
    /// scope decision (`📓️wave1-reports/b2-store-composition-report.md`) — this fallback (a
    /// synthetic "native" dialect from the app's own `DOCUMENT_SCHEMA`) is only ever consulted
    /// when no real dialect was ever threaded through `create_document_envelope`, and only
    /// matters for the OWNERSHIP-GRAPH bookkeeping `dispatch_group` needs a real `ArtifactRef`
    /// for, not for any wire/codec decision.
    ///
    /// 🛡️ Keys are unique and `self.children` is not resized until after `dispatch_group`, so
    /// these entry pointers stay valid and non-aliasing for the simultaneous child borrows
    /// `dispatch_group` requires.
    ///
    /// 🎛️ `meta.actor` is honored; a group is always one plain edit per member.
    ///
    /// 🎯️ RESOLVED (was BLOCKED — see `📓️terra-dedyn-fw-os-spacemember-report.md` §dispatch_group,
    /// and `📓️terra-dispatch-group-split-report.md` for the fix): `store::os_store::composition_coordinator::CompositionCoordinator::
    /// dispatch_group<Mp: SpaceMember, Mc: SpaceMember + MemberFactory>` now takes SEPARATE type
    /// parameters for `parent` and `children` — `self.store` (`ArtifactStore<A::Snapshot,
    /// A::Mutation>`, this app's own fixed type, only ever mutated) unifies as `Mp`; `dispatches`
    /// (`Vec<(&mut M, ChildDispatch<'_>)>`, this composition's CHILD-kind enum, the only side ever
    /// genesis-constructed) unifies as `Mc`. Dyn-free (O1-compliant) and now type-checks.
    ///
    /// 🌱️ Absorb any freshly-created children into the live map — extracted into its own
    /// method (below) both so it is directly unit-testable and so it needs no revisiting once
    /// a genesis-emitting `Emit` constructor lands (`genesis` is always `Vec::new()` on THIS
    /// call today — a `ChildEmit` only ever targets an ALREADY-live child).
    ///
    /// 🪆️ Child-lane replication: a composed member's edit is a real event of a real document,
    /// and the parent's backbone is the only endpoint it can cross. Announced here, at the one
    /// choke point every child edit lands at, so a replica folds it into ITS member of the
    /// same lane instead of diverging (ticket 26/09/19 `📓️flow.md` §5.3).
    ///
    /// 🪪️ The PARENT's handle must be the same value its own `KernelMutation.document` carries
    /// (`ArtifactHandle(meta.instance_id)`), not `artifact_handle_of(parent_id)` — otherwise one
    /// `InvocationResult` identifies the same document two different ways and any consumer
    /// correlating `mutations` with `member_edits` by handle silently fails to match the parent.
    /// Children keep the content-addressed `artifact_handle_of`, which is what their own
    /// `KernelMutation.document` uses.
    ///
    /// 🧱️ Parent side: `dispatch_wire` landed a REAL edit on `self.store` above (whenever
    /// `parent_ops` was non-empty), so its tail edit's real forward/inverse mutations are
    /// available exactly like the solitary path's `result_from_last_edit`.
    ///
    /// 🧩️ Child side: real per-op inverse bytes are not retrievable through the object-safe
    /// `SpaceMember` seam (see this fn's own doc comment) — each touched child's edit surfaces
    /// as ONE best-effort `KernelMutation` bundling all of that child's ops as its diff
    /// payload, with an empty (but validly-encoded, decodes to zero ops) inverse payload.
    /// `dispatch_group`'s phase 2 walks `children` (built here in the SAME order as
    /// `child_emits`) by index, skipping empty-ops entries — so the non-empty-ops subsequence
    /// of `child_emits`, in order, lines up 1:1 with the non-parent tail of `member_edits`.
    ///
    /// 🧾️ One command-log row for the whole group, carrying every touched child's edit in `child_edit_ids` (see
    /// `CommandLogEntry::child_edit_ids`'s own doc comment); a config-lane edit never joins it (design §20.13, L4).
    #[allow(clippy::too_many_arguments)]
    pub(super) async fn dispatch_emit_group(
        &mut self,
        verb: &str,
        artifact_mutations: &[A::Mutation],
        child_emits: &[ChildEmit],
        effects: Vec<Effect>,
        events: Vec<AppEvent>,
        ui_scope: UiDirtyScope,
        meta: &ActionMeta,
        group_id: Option<String>,
        transaction: Option<protocol::TransactionRef>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        if !artifact_mutations.is_empty() {
            self.require_operation_emitting_kind(verb, self.declared_dispatch_kind(verb))?;
        }
        let touched_child_count = child_emits.iter().filter(|child| !child.ops.is_empty()).count();
        let child_publication_generation = (touched_child_count != 0).then(|| self.admit_child_content_publication()).transpose()?;
        let parent_id = self.store.envelope().id.clone();
        let parent_dialect: ArtifactDialect = A::DIALECT.into();
        if self.store.envelope().dialect.as_ref() != Some(&parent_dialect) {
            return Err(plugin_sdk_fault("composition requires the parent's exact declared dialect"));
        }
        let parent_ref = ArtifactRef { artifact_id: parent_id.clone(), dialect: parent_dialect };
        let parent_ops=artifact_mutations.iter().map(::protocol::OpBinary::encode_op).collect::<Result<Vec<_>,_>>().map_err(|error|plugin_sdk_fault(error.to_string()))?;

        let mut seen_keys = ::std::collections::HashSet::with_capacity(child_emits.len());
        let mut child_ptrs: Vec<(*mut M, ChildDispatch<'_>)> = Vec::with_capacity(child_emits.len());
        for child_emit in child_emits {
            let key = (child_emit.slot.clone(), child_emit.child_id.clone());
            if !seen_keys.insert(key.clone()) {
                return Err(plugin_sdk_fault(format!("dispatch_emit: verb {verb:?} emitted duplicate ChildEmit for slot {:?} child {:?}", child_emit.slot, child_emit.child_id)));
            }
            let entry = self.children.get_mut(&key).ok_or_else(|| {
                plugin_sdk_fault(format!("dispatch_emit: verb {verb:?} emitted a ChildEmit for slot {:?} child {:?} with no live child store — call VcsArtifactApp::open_child/register_child first", child_emit.slot, child_emit.child_id))
            })?;
            let target = entry.reference.clone();
            let member_ptr: *mut M = &mut entry.member;
            child_ptrs.push((member_ptr, ChildDispatch::borrowed(target, &child_emit.ops, &child_emit.op_schema, &child_emit.labels)));
        }
        let child_refs: Vec<ArtifactRef> = child_ptrs.iter().map(|(_, dispatch)| dispatch.child.clone()).collect();
        let child_member_ptrs: Vec<*mut M> = child_ptrs.iter().map(|(member, _)| *member).collect();
        let mut dispatches: Vec<(&mut M, ChildDispatch<'_>)> = child_ptrs.into_iter().map(|(ptr, dispatch)| (unsafe { &mut *ptr }, dispatch)).collect();

        let group_meta = GroupMeta { actor: Some(meta.actor.clone()), group_id, transaction };
        self.store.set_authoring_verb(Some(verb.to_string()));
        let receipt = self.composition.dispatch_group(&parent_ref, &mut self.store, &mut dispatches, parent_ops, Vec::new(), group_meta, identity).await;
        self.store.set_authoring_verb(None);
        let receipt = receipt.map_err(|error| plugin_sdk_fault(error.to_string()))?;
        drop(dispatches);
        self.cache = None;
        if let Some(publication_generation) = child_publication_generation {
            let mut next = ChildContentView::clone(&self.child_content_root);
            for child_emit in child_emits.iter().filter(|child| !child.ops.is_empty()) {
                let captured = match self.children.get(&(child_emit.slot.clone(), child_emit.child_id.clone())) {
                    Some(entry) => next.with_member(MemberKeyRef::root(&child_emit.slot, &child_emit.child_id), &entry.reference, &entry.member).await,
                    None => Err(plugin_sdk_fault("committed child group lost its exact live member before immutable-root capture")),
                };
                match captured {
                    Ok(captured) => next = captured,
                    Err(fault) => {
                        if !next.is_empty() {
                            self.child_content_retirements.insert_admitted(publication_generation, ChildContentRetirement::new(next, false));
                        }
                        let mut members: Vec<(&ArtifactRef, &mut M)> = child_refs.iter().zip(child_member_ptrs.iter().copied()).map(|(reference, member)| (reference, unsafe { &mut *member })).collect();
                        let compensation = CompositionCoordinator::undo_group(&parent_ref, &mut self.store, &mut members, &receipt.invocation_id, identity).await;
                        drop(members);
                        if compensation.undone.len() != receipt.member_edits.len() {
                            return Err(plugin_sdk_fault(format!("child root capture failed ({}) and group compensation moved {}/{} members", fault.message, compensation.undone.len(), receipt.member_edits.len())));
                        }
                        return Err(fault);
                    }
                }
            }
            let previous = std::mem::replace(&mut *self.child_content_root, next);
            if !previous.is_empty() {
                self.child_content_retirements.insert_admitted(publication_generation, ChildContentRetirement::new(previous, false));
            }
            self.child_content_generation = publication_generation;
        }

        self.absorb_created_children(receipt.created_children).await?;

        let announced_lanes: Vec<(String, String)> = child_emits.iter().filter(|child_emit| !child_emit.ops.is_empty()).map(|child_emit| (child_emit.slot.clone(), child_emit.child_id.clone())).collect();
        self.announce_member_tail_edits(&announced_lanes).await?;

        let invocation_id = InvocationId(receipt.invocation_id.clone());
        let mut member_edits: Vec<EditRef> = Vec::with_capacity(receipt.member_edits.len());
        for (reference, edit_id) in receipt.member_edits.iter() {
            let document = if reference.artifact_id == parent_id { ArtifactHandle(meta.instance_id as u128) } else { artifact_handle_of(&reference.artifact_id).await };
            member_edits.push(EditRef { document, edit_id: edit_id.clone() });
        }

        let mut mutations: Vec<KernelMutation> = Vec::new();

        let parent_touched = receipt.member_edits.iter().any(|(reference, _)| reference.artifact_id == parent_id);
        if parent_touched {
            let operation_wires = self.store.authored_edit_operation_bytes(identity).map_err(|error| plugin_sdk_fault(error.to_string()))?;
            if let (Some((forward_wires, inverse_wires)), Some((forwards, _, mutation_meta))) = (operation_wires, self.store.edit_mutations()) {
                let inverse_payload = identity.encode(|control| ::protocol::os_spr::io::binary::operation_sequence::encode(&inverse_wires, control)).map_err(|error| plugin_sdk_fault(error.to_string()))?;
                let document = ArtifactHandle(meta.instance_id as u128);
                let schema = A::DOCUMENT_SCHEMA.to_string();
                for ((index, _forward), forward_wire) in forwards.iter().enumerate().zip(forward_wires) {
                    let entry = mutation_meta.get(index);
                    let mutation_id = entry.and_then(|entry_meta| entry_meta.mutation_id.clone()).unwrap_or_else(|| MutationId(format!("{}:{index}", invocation_id.0)));
                    let base_version = ArtifactVersion(entry.map_or(0, |entry_meta| entry_meta.base_version));
                    let undo_policy = entry.map_or(UndoPolicy::ExactBaseOnly, |entry_meta| entry_meta.undo_policy);
                    let author = entry.and_then(|entry_meta| entry_meta.author_id.clone()).unwrap_or_else(|| ActorId(meta.actor.clone()));
                    let timestamp = entry.map_or_else(|| HybridLogicalTimestamp::new(0, 0), |entry_meta| entry_meta.timestamp);
                    mutations.push(KernelMutation {
                        id: mutation_id.clone(),
                        document,
                        base_version,
                        invocation_id: invocation_id.clone(),
                        diff: ArtifactDiff { schema: SchemaId(format!("{schema}.operation")), payload: forward_wire },
                        inverse: InverseMutation {
                            target_mutation: mutation_id,
                            inverse_diff: ArtifactDiff { schema: SchemaId(format!("{schema}.operation.inverse")), payload: identity.encode(|control| control.copy_bytes(&inverse_payload)).map_err(|error| plugin_sdk_fault(error.to_string()))? },
                            base_version,
                            dependencies: Vec::new(),
                            undo_policy,
                        },
                        dependencies: Vec::new(),
                        author,
                        timestamp,
                    });
                }
            }
        }

        let child_member_edits: Vec<&(ArtifactRef, String)> = receipt.member_edits.iter().filter(|(reference, _)| reference.artifact_id != parent_id).collect();
        let touched_child_emits: Vec<&ChildEmit> = child_emits.iter().filter(|child_emit| !child_emit.ops.is_empty()).collect();
        debug_assert_eq!(touched_child_emits.len(), child_member_edits.len(), "dispatch_group's per-child edit order must match the non-empty-ops subsequence of child_emits");
        let mut child_edit_ids: Vec<String> = Vec::with_capacity(child_member_edits.len());
        for (child_emit, (reference, edit_id)) in touched_child_emits.into_iter().zip(child_member_edits) {
            child_edit_ids.push(edit_id.clone());
            let document = artifact_handle_of(&reference.artifact_id).await;
            let mutation_id = MutationId(edit_id.clone());
            mutations.push(KernelMutation {
                id: mutation_id.clone(),
                document,
                base_version: ArtifactVersion(0),
                invocation_id: invocation_id.clone(),
                diff: ArtifactDiff { schema: child_emit.op_schema.clone(), payload: protocol::encode_ops_vec(&child_emit.ops) },
                inverse: InverseMutation {
                    target_mutation: mutation_id,
                    inverse_diff: ArtifactDiff { schema: SchemaId(format!("{}.inverse", child_emit.op_schema.0)), payload: protocol::encode_ops_vec(&[]) },
                    base_version: ArtifactVersion(0),
                    dependencies: Vec::new(),
                    undo_policy: UndoPolicy::ExactBaseOnly,
                },
                dependencies: Vec::new(),
                author: ActorId(meta.actor.clone()),
                timestamp: HybridLogicalTimestamp::new(0, 0),
            });
        }

        let mutation_ids: Vec<MutationId> = mutations.iter().map(|mutation| mutation.id.clone()).collect();
        let inverse_mutations: Vec<InverseMutation> = mutations.iter().map(|mutation| mutation.inverse.clone()).collect();

        let parent_edit_id = receipt.member_edits.iter().find(|(reference, _)| reference.artifact_id == parent_id).map(|(_, edit_id)| edit_id.clone());
        let kind = self.registry.get(verb).map_or(ActionKind::Mutation, |def| def.kind);
        let label = self.verb_label(verb);
        self.record_command(verb, kind, label, parent_edit_id, child_edit_ids, None);

        Ok(InvocationResult {
            output: DslValue::Null,
            mutations,
            inverse_group: UndoGroup { invocation_id, mutations: mutation_ids, inverse_mutations, member_edits },
            diagnostics: Vec::new(),
            requested_effects: effects,
            events,
            ui_scope,
            history_patch: None,
        })
    }
}
