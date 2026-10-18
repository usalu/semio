//! 📒️ Store log operations: the clock, envelope reads, authoring verb, open transaction, operation wire, conflicts, time travel reads, history shape admission and the event log payloads of the store log. See [`super`].

use super::*;

//#region 🔖️EphemeralLanes
//#region 🔖️Backbone
//#region 🔖️Backbone
//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
impl<P, Mutation> StoreLog<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    /// ⏰️ Reads this Store's hybrid logical clock without suspension, so a server that mints a
    /// stamped publication clock can make it strictly after the document's own last edit instead of
    /// guessing from its wall clock.
    pub fn clock_now(&self) -> HybridLogicalTimestamp {
        self.clock
    }

    pub fn envelope(&self) -> &ArtifactEnvelope<P, Mutation> {
        &self.envelope
    }

    /// 👁️ Read-only envelope view — prefer this over mutating through public fields.
    pub fn envelope_view(&self) -> ArtifactEnvelopeView<'_, P, Mutation> {
        ArtifactEnvelopeView { envelope: &self.envelope }
    }

    /// 🧭️ The checkpoint new commits currently parent onto (defaults to the latest checkpoint
    /// on construction/`set_state`; advances on commit/checkout/switch).
    pub fn current_checkpoint_id(&self) -> Option<&str> {
        self.current_checkpoint_id.as_deref()
    }

    /// 🖋️ The local actor id used to distinguish this store's own edits from ingested ones.
    /// The constructor supplies this authority; replay and history authors preserve it.
    pub fn local_actor_id(&self) -> &ActorId {
        &self.local_actor_id
    }

    /// 🖋️ Binds the opened runtime instance actor explicitly; mutation authors remain durable edit metadata.

    /// 🏷️ The verb id the next locally minted edit is stamped with (see {@link set_authoring_verb}).
    pub fn authoring_verb(&self) -> Option<&str> {
        self.authoring_verb.as_deref()
    }

    /// 🏷️ Names the action or command whose `Apply` the caller dispatches next, so the edit it mints carries a
    /// locale-neutral label source every reload and peer resolves the same way; the caller clears it after dispatch.
    pub fn set_authoring_verb(&mut self, verb: Option<String>) {
        self.authoring_verb = verb.filter(|value| !value.is_empty());
    }

    /// 🎞️ The tool transaction open on this store, if any ([`ArtifactCommand::AppendTransaction`]).
    pub fn open_transaction(&self) -> Option<&OpenToolTransaction> {
        self.envelope.open_transaction.as_ref()
    }

    /// 🔧️ The most recently created/amended edit's `(forwards, inverse, per-operation meta)`.
    /// Used right after `dispatch(Apply{..})`/`AppendTransaction` to build a `KernelMutation`/`InvocationResult`
    /// with a true inverse from the just-recorded `Edit.inverse`.
    pub fn edit_mutations(&self) -> Option<(&[Mutation], &semio_framework_value::list::PagedList<Mutation, {usize::MAX}>, &[MutationMeta])> {
        self.envelope.vcs.edits.last().map(|edit| (edit.forwards.as_slice(), &edit.inverse, edit.mutation_meta.as_slice()))
    }

    /// 📜️ Ancestor-graph rows for this store's checkpoint history. See {@link build_history_columns}.
    pub async fn history_columns(&self) -> Vec<HistoryColumn> {
        build_history_columns(&self.envelope).await
    }

    /// 🎟️ Borrows the exact physical history cursor frame allocation before any owner transfers.
    pub fn history_read_retirement_birth_bytes()->usize {history_read_admission::history_read_retirement_birth_bytes::<P,Mutation>()}

    /// 🪪️ Installs a richer-than-catalog operation wire (the SDK's document stores carry the `(entity, kind)` schema pair), once, before the catalog.
    pub fn install_operation_wire(&mut self, wire: ArtifactOperationWire<Mutation>) {
        assert!(self.operation_wire.is_none(), "a store installs its canonical operation wire exactly once");
        self.operation_wire = Some(wire);
    }

    /// 🪪️ The canonical operation wire this store authors with, when one is installed.
    pub fn operation_wire(&self) -> Option<ArtifactOperationWire<Mutation>> {
        self.operation_wire
    }

    fn take_string_at_retained(target: &mut std::mem::ManuallyDrop<semio_framework_os_vcs::HistoryPageStack<String>>, position: usize) -> String {
        target.remove(position)
    }

    /// ♻️ Retires one snapshot alias a client held beside the store (a tool run's base or overlay, a folded intermediate):
    /// the last alias hands its snapshot to the exact owned-value retirement the store's own returned reads use, any other
    /// alias only drops its count. A client never drops an alias plainly — a snapshot whose roots must be retired (an
    /// `OrderedMap`) panics on a plain drop of its last owner.
    /// 📏️ Borrows the exact snapshot alias retirement frame before original ownership transfers.
    pub fn snapshot_alias_retirement_birth_bytes()->usize {ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes()}

    /// 🎯️ MEDIUM-3 (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` J1
    /// robustness pass): `envelope.conflicts` used to be appended on every rejection/degradation and
    /// truncated only incidentally (`resolve_conflict`'s `Quarantined`+`Accept` arm discarding the
    /// re-ingest's own transient conflicts). A peer that keeps sending a batch this replica keeps
    /// quarantining — `ingest_remote`'s dag never advances on quarantine, so the SAME envelope is
    /// eligible for redelivery forever — grows this list, and the persisted `.spr`, without bound.
    /// Two named caps, two different failure modes because `Open`/resolved conflicts carry different
    /// weight: a resolved (`Accepted`/`Discarded`) conflict is a closed historical fact a UI no
    /// longer needs enumerated, so it is PRUNABLE — `prune_resolved_conflicts` evicts the oldest
    /// (lowest push-index, i.e. first non-`Open` entry found scanning from the front) resolved
    /// conflicts once the total exceeds [`Self::RESOLVED_CONFLICT_CAP`]. An `Open` conflict is a
    /// pending decision a human/authority has not yet acted on — silently dropping one would erase
    /// unresolved history, so it is never evicted; instead [`Self::ensure_open_conflict_capacity`]
    /// is checked BEFORE any state mutation in every call site that might mint one, and refuses with
    /// a loud, typed `VcsError` (atomic — nothing applied) once open conflicts would exceed
    /// [`Self::OPEN_CONFLICT_CAP`], forcing existing ones to be resolved before backlog can grow
    /// further. `edit_messages` (the durable per-edit message ledger) was reviewed for the same
    /// pattern and deliberately gets NO cap: unlike `conflicts`, it only grows once per edit that
    /// ever reaches `committed_ids` (`record_edit_messages`/`replace_edit_messages` are never called
    /// for a quarantined id — its entry is explicitly cleared instead), so its size tracks genuine
    /// applied-history growth, not redelivery-of-rejected-content amplification; capping it would
    /// mean silently losing a still-applied edit's own diagnostic record, which is strictly worse.
    pub const OPEN_CONFLICT_CAP: usize = 256;

    /// 🎯️ MEDIUM-3: refuses to mint `additional_open` more `Open` conflicts once doing so would push
    /// this envelope past [`Self::OPEN_CONFLICT_CAP`]. Callers check this BEFORE mutating any store
    /// field for the call, so a refusal is atomic — see the const's own doc for why open conflicts
    /// are capped-but-never-silently-dropped rather than pruned like resolved ones.
    pub fn ensure_open_conflict_capacity(&self, additional_open: usize) -> Result<(), VcsError> {
        if additional_open == 0 {
            return Ok(());
        }
        let open_now = self.envelope.conflicts.iter().filter(|conflict| conflict.status == semio_framework_os_spr::ConflictStatus::Open).count();
        if open_now + additional_open > Self::OPEN_CONFLICT_CAP {
            return Err(VcsError::ValidationFailed(format!(
                "open conflict backlog is at capacity ({open_now} of {} open, {additional_open} more pending) — resolve existing open conflicts before this artifact can record another",
                Self::OPEN_CONFLICT_CAP
            )));
        }
        Ok(())
    }

    /// ⚔️ Every conflict this store has ever raised (`Open`, `Accepted`, and `Discarded`
    /// alike) — see {@link conflicts} field doc.
    pub fn conflicts(&self) -> &[semio_framework_os_spr::Conflict] {
        &self.envelope.conflicts
    }

    /// ⚔️ The subset of {@link conflicts} still `Open` — what a UI's Conflicts panel lists.
    pub fn open_conflicts(&self) -> impl Iterator<Item = &semio_framework_os_spr::Conflict> {
        self.envelope.conflicts.iter().filter(|conflict| conflict.status == semio_framework_os_spr::ConflictStatus::Open)
    }

    /// ⚖️ This store's local `crate::os_spr::MergePolicy` (see the field doc).
    pub fn merge_policy(&self) -> semio_framework_os_spr::MergePolicy {
        self.merge_policy
    }

    /// ⚖️ Sets this store's local `crate::os_spr::MergePolicy`. Local/authority state only —
    /// never wire-carried, never part of shared history; takes effect starting with the NEXT
    /// `dispatch`/`ingest_remote` call.
    pub fn set_merge_policy(&mut self, policy: semio_framework_os_spr::MergePolicy) {
        self.merge_policy = policy;
    }

    /// 📒️ Every `crate::os_spr::MutationMessage` `edit_id`'s own replay raised — empty for an
    /// edit that raised none (including an unknown `edit_id`), never an error (see the field doc).
    pub fn messages_for_edit(&self, edit_id: &str) -> &[semio_framework_os_spr::MutationMessage] {
        self.envelope.edit_messages.get_by_id(edit_id).map_or(&[], |entry| entry.messages.as_slice())
    }

    //#region 🔖️TimeTravelReads
    /// 🌳️ The id of this document's trunk — the implicit root line, a first-class alternative listed first in
    /// `envelope().vcs.alternatives` (with an empty name the UI localizes) once a commit made on it gives it a chain.
    pub fn trunk_alternative_id(&self) -> String {
        semio_framework_os_spr::trunk_alternative_id(&ArtifactId(self.envelope.id.clone()))
    }

    /// 🚉️ The line the document is on: the active branched alternative, or the trunk while none is.
    pub fn active_line_id(&self) -> String {
        self.envelope.active_alternative_id.clone().unwrap_or_else(|| self.trunk_alternative_id())
    }

    /// 🔖️ `alternative_id` as a `Checkout` names it: `None` for the trunk (the log never names it), the id otherwise.
    pub fn branched_alternative(&self, alternative_id: String) -> Option<String> {
        (alternative_id != self.trunk_alternative_id()).then_some(alternative_id)
    }

    /// 📖️ The edits a replay from {@link begin_report_replay} steps against.
    pub fn replay_edits(&self) -> &ArtifactHistoryLedger<Edit<Mutation>> {
        &self.envelope.vcs.edits
    }

    //#endregion 🔖️TimeTravelReads
    //#region 📬️OneItemPublication
    /// 🎥️ The edit a batch appends to instead of minting a ledger slot: the open edit of the tool transaction it carries
    /// (design §15). Outside a transaction every batch is its own edit.
    pub fn batch_amend_target_ref(&self, transaction: Option<&protocol::TransactionRef>) -> Option<&str> {
        self.envelope.open_transaction.as_ref().filter(|open| transaction.is_some_and(|transaction| open.transaction.id == transaction.id) && self.applied_edit_ids.last() == Some(&open.edit_id)).map(|open| open.edit_id.as_str())
    }

    pub fn batch_amend_target(&self, transaction: Option<&protocol::TransactionRef>) -> Option<String> {
        self.envelope.open_transaction.as_ref().filter(|open| transaction.is_some_and(|transaction| open.transaction.id == transaction.id) && self.applied_edit_ids.last() == Some(&open.edit_id)).map(|open| open.edit_id.clone())
    }

    pub fn cancel_apply_batch(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>) -> bool {
        if matches!(publication.phase, ArtifactStoreOneItemPublicationPhase::Complete | ArtifactStoreOneItemPublicationPhase::Closing) {
            return false;
        }
        publication.cancel_requested = true;
        publication.begin_close();
        true
    }

    //#endregion 📬️OneItemPublication
    //#region 🔖️EventSourcedHistory
    /// 🪪️ The wire operations `edit_id` owns — the replica-neutral identity transitions name.
    pub fn edit_mutation_ids(&self, edit_id: &str) -> Result<Vec<MutationId>, VcsError> {
        let edit = self.envelope.vcs.edits.find_near(|edit| edit.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.to_string()))?;
        Ok(semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit))
    }

    /// 🌱️ The transition that brought `checkpoint_id` into being — a `Commit`, or the
    /// `Repin` that re-identified it — so a transition naming the checkpoint depends on it causally.
    pub fn checkpoint_origin(&self, checkpoint_id: &str) -> Option<MutationId> {
        self.envelope
            .transitions
            .iter()
            .rev()
            .find(|envelope| match semio_framework_os_spr::history_transition_from_envelope(envelope) {
                Ok(Some(semio_framework_os_spr::HistoryTransition::Commit(checkpoint))) => checkpoint.checkpoint_id == checkpoint_id,
                Ok(Some(semio_framework_os_spr::HistoryTransition::Repin { pinned_checkpoint_id, .. })) => pinned_checkpoint_id == checkpoint_id,
                _ => false,
            })
            .map(|envelope| envelope.mutation_id.clone())
    }

    /// 🫚️ The `Branch` transition that registered `alternative_id` — so a commit on that line depends on it causally and no
    /// replica folds the commit before it lists the line. The trunk has no registration.
    fn alternative_origin(&self, alternative_id: &str) -> Option<MutationId> {
        self.envelope
            .transitions
            .iter()
            .rev()
            .find(|envelope| matches!(semio_framework_os_spr::history_transition_from_envelope(envelope), Ok(Some(semio_framework_os_spr::HistoryTransition::Branch { alternative_id: registered, .. })) if registered == alternative_id))
            .map(|envelope| envelope.mutation_id.clone())
    }

    /// ✉️ `edit`'s operations as causal wire envelopes — one per forward operation.
    pub fn operation_envelopes(&self, edit: &Edit<Mutation>) -> Result<Vec<semio_framework_os_spr::MutationEnvelope>, VcsError> {
        self.operation_envelopes_since(edit, 0)
    }

    /// ✂️ The envelopes of `edit`'s operations from position `from` on: the range an amend appended.
    pub fn operation_envelopes_since(&self, edit: &Edit<Mutation>, from: usize) -> Result<Vec<semio_framework_os_spr::MutationEnvelope>, VcsError> {
        semio_framework_os_spr::mutation_envelopes_from_edit_since::<P, Mutation>(edit, from, &ArtifactId(self.envelope.id.clone()), &SchemaId(self.envelope.schema.clone())).map_err(|error| VcsError::Serialize(error.to_string()))
    }

    /// 👁️ The newest operation of an author other than `actor` this replica has applied — how far
    /// an operation `actor` authors now has seen the other authors' work. Remote edits join the ledger in
    /// arrival order, which is the hub's commit order; an undone edit was seen all the same.
    pub fn observed_foreign_operation(&self, actor: &str) -> Option<MutationId> {
        let edit = self.envelope.vcs.edits.iter().rev().find(|edit| edit.actor.as_deref().is_some_and(|author| author != actor))?;
        semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit).pop()
    }

    /// 🌾️ Marks every recorded event applied in the causal graph, so a peer's later events that
    /// depend on them are never held back — whatever path (load, batched publication) recorded them.
    pub fn seed_known_events(&mut self) -> Result<(), VcsError> {
        let mut identities: Vec<MutationId> = self.envelope.transitions.iter().map(|transition| transition.mutation_id.clone()).collect();
        for edit in &self.envelope.vcs.edits {
            identities.extend(semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit));
        }
        for identity in identities {
            match self.dag.seed_applied(identity) {
                Ok(()) => {}
                Err(rejected) if rejected.error == semio_framework_os_spr::MutationDagError::Duplicate => {}
                Err(rejected) => return Err(VcsError::ValidationFailed(rejected.error.to_string())),
            }
        }
        Ok(())
    }

    /// 🗂️ Refuses the first of `envelopes` whose transition this store's [`crate::os_spr::HistoryShape`] does not hold —
    /// with `HistoryShape` — or that does not decode, before any of them is recorded.
    pub fn admit_history_shape(&self, envelopes: &[semio_framework_os_spr::MutationEnvelope]) -> Result<(), VcsError> {
        let shape = self.envelope.history_shape;
        for envelope in envelopes {
            if let Some(kind) = semio_framework_os_spr::history_transition_from_envelope(envelope).map_err(|error| VcsError::Deserialize(error.to_string()))?.map(|transition| transition.kind()).filter(|kind| !shape.admits(*kind)) {
                return Err(VcsError::HistoryShape { shape, kind });
            }
        }
        Ok(())
    }

    /// 🖊️ The actor this replica authors its history transitions as.
    pub fn transition_actor(&self) -> ActorId {
        (*self.local_actor_id).clone()
    }

    /// 📧️ The envelope of one locally authored transition at `timestamp`, declaring `target` as the address it writes.
    pub fn transition_envelope(&self, transition: &semio_framework_os_spr::HistoryTransition, dependencies: Vec<MutationId>, target: Vec<String>, timestamp: HybridLogicalTimestamp) -> semio_framework_os_spr::MutationEnvelope {
        let actor = self.transition_actor();
        let mut envelope = semio_framework_os_spr::history_transition_envelope(transition, &ArtifactId(self.envelope.id.clone()), &actor, dependencies, timestamp);
        if matches!(transition, semio_framework_os_spr::HistoryTransition::Supersede(_)) {
            envelope.observed = self.observed_foreign_operation(&actor.0);
        }
        envelope.target = target;
        envelope
    }

    /// 🧮️ The full fold of the event log at this replica's head ([`fold_envelope_history`]) — O(history), so never on the
    /// path of a local edit that joins the fold as its tail ([`Self::folded_tail`]).
    pub fn fold_live_log(&self) -> Result<semio_framework_os_spr::HistoryFold, VcsError> {
        #[cfg(test)]
        hot_path_census::count(|census| census.folds += 1);
        fold_envelope_history(&self.envelope)
    }

    /// 🧭️ The greatest `(hlc)` key of every event in the log: the transitions are kept in key order, the edits are not.
    pub fn event_frontier(&self) -> (u64, u64, u64) {
        let transitions = self.envelope.transitions.last().map(|transition| transition.timestamp.cmp_key());
        self.envelope.vcs.edits.iter().map(|edit| edit_timestamp(edit).cmp_key()).chain(transitions).max().unwrap_or_default()
    }

    //#endregion 🔖️EventSourcedHistory
    //#region 🔖️ToolTransactions
    /// 🧿️ The open tool transaction when `transaction_id` names it, else `UnknownTransaction`.
    pub fn open_transaction_named(&self, transaction_id: &str) -> Result<OpenToolTransaction, VcsError> {
        self.envelope.open_transaction.clone().filter(|open| open.transaction.id == transaction_id).ok_or_else(|| VcsError::UnknownTransaction(transaction_id.to_string()))
    }

    //#endregion 🔖️ToolTransactions
    //#region 🔖️DeferredReprojection
    /// 🪐️ The history fold of the event log with a waiting change joined — its remote transitions, and its local step's
    /// transitions at the head that step moves to — touching nothing.
    pub fn pending_fold(&self, pending: &PendingReprojection<P, Mutation>) -> Result<semio_framework_os_spr::HistoryFold, VcsError> {
        let local = pending.local.as_ref();
        let candidates: Vec<semio_framework_os_spr::MutationEnvelope> = pending.transitions.iter().chain(local.map_or(&[][..], |local| local.envelopes.as_slice())).cloned().collect();
        let head = local.map_or_else(|| envelope_viewer_head(&self.envelope), |local| self.viewer_head(&local.head));
        self.prospective_fold_at(&candidates, &head)
    }

    //#endregion 🔖️DeferredReprojection
    //#region 🔖️Supersede
    /// 🪴️ Authors into `envelopes` the commit of every pending edit (when one is pending or no checkpoint exists) and a
    /// `Branch` of `name` at the resulting head, answering the new alternative's id.
    pub async fn author_head_alternative(&self, name: String, clock: &mut HybridLogicalTimestamp, envelopes: &mut Vec<semio_framework_os_spr::MutationEnvelope>, identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
        let pending = uncommitted_edit_ids(&self.envelope, &*self.applied_edit_ids).await;
        let checkpoint_id = if !pending.is_empty() || self.envelope.vcs.checkpoints.is_empty() {
            if self.applied_edit_ids.is_empty() {
                return Err(VcsError::NoCheckpoint);
            }
            let (transition, dependencies, checkpoint_id) = self.pending_checkpoint_transition(None, Vec::new(), identity).await?;
            clock.tick(now_ms());
            envelopes.push(self.transition_envelope(&transition, dependencies, Vec::new(), *clock));
            checkpoint_id
        } else {
            (*self.current_checkpoint_id).clone().or_else(|| self.envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())).ok_or(VcsError::NoCheckpoint)?
        };
        let alternative_id = identity.encode(|control|mint_alternative_id(&name, std::slice::from_ref(&checkpoint_id), control))?;
        if self.envelope.vcs.alternatives.iter().any(|alternative| alternative.id == alternative_id) {
            return Err(VcsError::ValidationFailed(format!("alternative {alternative_id} already exists")));
        }
        let dependencies = match envelopes.last() {
            Some(commit) => vec![commit.mutation_id.clone()],
            None => self.checkpoint_origin(&checkpoint_id).into_iter().collect(),
        };
        clock.tick(now_ms());
        envelopes.push(self.transition_envelope(&semio_framework_os_spr::HistoryTransition::Branch { alternative_id: alternative_id.clone(), name, checkpoint_id }, dependencies, Vec::new(), *clock));
        Ok(alternative_id)
    }

    /// 🔮️ The history fold of the event log with `candidates` joined, touching nothing.
    pub fn prospective_fold(&self, candidates: &[semio_framework_os_spr::MutationEnvelope]) -> Result<semio_framework_os_spr::HistoryFold, VcsError> {
        self.prospective_fold_at(candidates, &envelope_viewer_head(&self.envelope))
    }

    /// 🧐️ [`Self::prospective_fold`] for `head` instead of this replica's head.
    pub fn prospective_fold_at(&self, candidates: &[semio_framework_os_spr::MutationEnvelope], head: &semio_framework_os_spr::ViewerHead) -> Result<semio_framework_os_spr::HistoryFold, VcsError> {
        let mut transitions = self.envelope.transitions.clone();
        transitions.extend(candidates.iter().cloned());
        fold_event_log::<P, Mutation>(&self.envelope.id, &self.envelope.vcs.edits.iter().collect::<Vec<_>>(), &transitions, &self.envelope.conflicts, head)
    }

    //#endregion 🔖️Supersede
    //#region 🔖️HistoryLane
    /// 🛤️ The lane `edit_id` was recorded under. Absence from `envelope.lanes` means
    /// `Document` by construction — `apply_command` below only ever inserts a map
    /// entry for a NON-`Document` lane, so an ordinary document edit (and every edit that predates
    /// this field) never gets one. `HistoryLane` is `Copy`, so this returns by value like
    /// `edit_is_local` returns `bool`.
    pub fn edit_lane(&self, edit_id: &str) -> HistoryLane {
        self.envelope.lanes.get(edit_id).copied().unwrap_or_default()
    }

    /// 🛤️ Nearest-to-top redo entry this actor authored whose lane satisfies `lane`. Another
    /// actor's reverted edit sits on the shared redo stack too, but only its author may reinstate it.
    pub fn redo_position(&self, lane: impl Fn(HistoryLane) -> bool) -> Option<usize> {
        self.redo_edit_ids.iter().rposition(|id| self.edit_is_local(id) && lane(self.edit_lane(id)))
    }

    /// 👀️ This replica's head `(line, checkpoint)` as the envelope holds it.
    pub fn viewer_head_now(&self) -> (Option<String>, Option<String>) {
        (self.envelope.active_alternative_id.clone(), self.envelope.viewer_checkpoint_id.clone())
    }

    /// 🧿️ The fold head of `(line, checkpoint)`: an absent line (or the trunk's id) is the trunk.
    pub fn viewer_head(&self, (line, checkpoint): &(Option<String>, Option<String>)) -> semio_framework_os_spr::ViewerHead {
        let trunk = self.trunk_alternative_id();
        semio_framework_os_spr::ViewerHead { line_id: line.clone().filter(|line| *line != trunk).unwrap_or(trunk), checkpoint_id: checkpoint.clone() }
    }

    /// 🚩️ The `Commit` transition (with its dependencies and new checkpoint id) grouping every applied-but-uncommitted edit.
    pub async fn pending_checkpoint_transition(&self, message: Option<String>, authors: Vec<Author>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(semio_framework_os_spr::HistoryTransition, Vec<MutationId>, String), VcsError> {
        let pending = uncommitted_edit_ids(&self.envelope, &*self.applied_edit_ids).await;
        if pending.is_empty() {
            return Err(VcsError::ValidationFailed(EMPTY_CHECKPOINT_MESSAGE.to_string()));
        }
        let change_id = identity.encode(|control|mint_change_id(&pending, message.as_deref(), control))?;
        let saved_at = now_iso();
        let parent = self.current_checkpoint_id.as_ref().and_then(|id| self.envelope.vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == *id));
        let mut change_ids = parent.map(|checkpoint| checkpoint.change_ids.clone()).unwrap_or_default();
        let parent_id = parent.map(|checkpoint| checkpoint.id.clone());
        change_ids.push(change_id.clone());
        let timestamp = now_iso();
        let checkpoint_id = content_addressed_checkpoint_id_with_pending_change(parent_id.as_deref(), &change_ids, &self.envelope.vcs.changes, &change_id, &pending, message.as_deref(), &saved_at, message.as_deref(), &authors, &timestamp, &[]);
        let mut mutation_ids = Vec::new();
        for edit_id in &pending {
            mutation_ids.extend(self.edit_mutation_ids(edit_id)?);
        }
        let mut dependencies = mutation_ids.clone();
        dependencies.extend(parent_id.as_deref().and_then(|parent_id| self.checkpoint_origin(parent_id)));
        dependencies.extend(self.envelope.active_alternative_id.as_deref().and_then(|line_id| self.alternative_origin(line_id)));
        let authors = authors.into_iter().map(|author| semio_framework_os_spr::TransitionAuthor { id: author.id, name: author.name, avatar: author.avatar }).collect();
        let checkpoint =
            semio_framework_os_spr::TransitionCheckpoint { checkpoint_id: checkpoint_id.clone(), parent_id, change_id, mutation_ids, description: message.clone(), saved_at, authors, message, timestamp, line_id: self.envelope.active_alternative_id.clone() };
        Ok((semio_framework_os_spr::HistoryTransition::Commit(checkpoint), dependencies, checkpoint_id))
    }

    //#endregion 🔖️HistoryLane
    /// 📸️ The whole document as its persisted `pack` (genesis) + `spr` (event log) bytes —
    /// for export and archive persistence; replicas never exchange it (they exchange events).
    pub async fn snapshot_pack(&self) -> Result<ArtifactPackFiles, VcsError>
    where P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
    {
        print_document_pack(&self.envelope).await
    }

    /// 📦️ Serializes the full document envelope (snapshot + VCS history) as JSON over the
    /// first-party `ToValue` bridge. `capture_read()` stays fallible (a group-visibility
    /// consistency check); the resulting `ArtifactEnvelopeRead` is then infallibly convertible —
    /// see its own `impl ToValue` docstring.
    pub fn envelope_json(&self) -> Result<String, VcsError>
    where
        P: ToValue,
        Mutation: ToValue,
    {
        let read = self.envelope.capture_read().map_err(|e| VcsError::Serialize(e.to_string()))?;
        Ok(semio_framework_pack_json::to_json_string(&read))
    }

    /// 📜️ Every event of this replica's log as causal envelopes: each edit's operations in
    /// ledger order, then every history transition in HLC order. An open tool transaction's edit is not in the shared log
    /// before its commit.
    pub fn event_log(&self) -> Result<Vec<semio_framework_os_spr::MutationEnvelope>, VcsError> {
        let mut events = Vec::new();
        for edit in self.envelope.vcs.edits.iter().filter(|edit| self.envelope.holds_committed_edit(&edit.id)) {
            events.extend(self.operation_envelopes(edit)?);
        }
        events.extend(self.envelope.transitions.iter().cloned());
        Ok(events)
    }

    /// 📦️ This replica's whole event log as ONE encoded envelope payload. A composed member has
    /// no transport of its own, so its composing parent announces this on the member's behalf under the
    /// member's `(slot, child_id)` lane the moment the parent attaches a backbone.
    pub fn event_log_payload(&self) -> Result<Vec<u8>, VcsError> {
        Ok(semio_framework_os_spr::encode_envelopes(&self.event_log()?))
    }

    /// 📤️ Seeds this replica's TAIL edit into its own causal dag and hands back exactly the
    /// envelopes {@link flush_apply_outbound} would have sent, for a caller that owns the transport —
    /// the composed-member half of outbound announcement. Empty when nothing has ever been applied.
    pub fn announce_tail_edit_payload(&mut self) -> Result<Vec<u8>, VcsError> {
        let Some(edit) = self.envelope.vcs.edits.last().filter(|edit| self.envelope.holds_committed_edit(&edit.id)) else { return Ok(Vec::new()) };
        let document_id = ArtifactId(self.envelope.id.clone());
        let schema = SchemaId(self.envelope.schema.clone());
        let op_envelopes = semio_framework_os_spr::mutation_envelope_from_edit::<P, Mutation>(edit, &document_id, &schema).map_err(|error| VcsError::Serialize(error.to_string()))?;
        for op_envelope in &op_envelopes {
            match self.dag.seed_applied(op_envelope.mutation_id.clone()) {
                Ok(()) => {}
                Err(rejected) if rejected.error == semio_framework_os_spr::MutationDagError::Duplicate => {}
                Err(rejected) => return Err(VcsError::ValidationFailed(rejected.error.to_string())),
            }
        }
        Ok(semio_framework_os_spr::encode_envelopes(&op_envelopes))
    }

    pub fn backbone_ref(&self) -> Option<&ArtifactBackboneRef> {
        self.envelope.backbone.as_ref()
    }

    pub fn assert_equivalent_remote_envelope(&self, existing: &Edit<Mutation>, incoming: &semio_framework_os_spr::MutationEnvelope) -> Result<(), VcsError> {
        let document_id = ArtifactId(self.envelope.id.clone());
        let schema = SchemaId(self.envelope.schema.clone());
        let established = semio_framework_os_spr::mutation_envelope_from_edit::<P, Mutation>(existing, &document_id, &schema).map_err(|error| VcsError::Serialize(error.to_string()))?;
        if established.len() == 1 && established.first().is_some_and(|candidate| Self::same_operation_identity_and_payload(candidate, incoming)) {
            return Ok(());
        }
        Err(VcsError::ValidationFailed(format!("remote mutation id {} conflicts with its established payload", incoming.mutation_id.0)))
    }

    pub fn assert_equivalent_remote_mutation(&self, incoming: &semio_framework_os_spr::MutationEnvelope) -> Result<(), VcsError> {
        if semio_framework_os_spr::is_history_transition(incoming) {
            return match self.envelope.transitions.iter().find(|known| known.mutation_id == incoming.mutation_id) {
                Some(known) if Self::same_operation_identity_and_payload(known, incoming) => Ok(()),
                Some(_) => Err(VcsError::ValidationFailed(format!("remote transition {} conflicts with its established payload", incoming.mutation_id.0))),
                None => Err(VcsError::ValidationFailed(format!("remote transition {} was marked applied without an established payload", incoming.mutation_id.0))),
            };
        }
        let document_id = ArtifactId(self.envelope.id.clone());
        let schema = SchemaId(self.envelope.schema.clone());
        for edit in &self.envelope.vcs.edits {
            for established in semio_framework_os_spr::mutation_envelope_from_edit::<P, Mutation>(edit, &document_id, &schema).map_err(|error| VcsError::Serialize(error.to_string()))? {
                if established.mutation_id == incoming.mutation_id {
                    return if Self::same_operation_identity_and_payload(&established, incoming) { Ok(()) } else { Err(VcsError::ValidationFailed(format!("remote mutation id {} conflicts with its established payload", incoming.mutation_id.0))) };
                }
            }
        }
        Err(VcsError::ValidationFailed(format!("remote mutation id {} was marked applied without an established payload", incoming.mutation_id.0)))
    }

    /// 🚫️async: E1 pure field comparison, consumed only through `is_some_and`/`Iterator::all`
    /// sync closures — see R9.
    pub fn same_operation_identity_and_payload(left: &semio_framework_os_spr::MutationEnvelope, right: &semio_framework_os_spr::MutationEnvelope) -> bool {
        left.mutation_id == right.mutation_id && left.document_id == right.document_id && left.diff.schema == right.diff.schema && left.diff.payload == right.diff.payload
    }

    fn take_candidate_envelope(&mut self) -> ArtifactEnvelope<P, Mutation> {
        assert!(!self.envelope_detached, "resolution candidate envelope authority was already detached");
        self.envelope_detached = true;
        unsafe { std::mem::ManuallyDrop::take(&mut self.envelope) }
    }

    /// 🧬️ A peer's `Genesis` must name this document's own initial snapshot: replicas of one
    /// document share their genesis, so a mismatch is a different document, never something to merge.
    pub fn verify_genesis(&self, pack: &[u8]) -> Result<(), VcsError> {
        if *semio_framework_hash::hash(pack).as_bytes() != self.initial_digest {
            return Err(VcsError::ValidationFailed("backbone genesis names a different initial snapshot; replicas of one document share their genesis".into()));
        }
        Ok(())
    }

    /// 🖋️ Whether `edit_id` was authored by this opened actor; every other actor is foreign.
    pub fn edit_is_local(&self, edit_id: &str) -> bool {
        self.envelope.vcs.edits.find_near(|edit| edit.id == edit_id).is_some_and(|edit| edit.actor.as_deref() == Some(self.local_actor_id().0.as_str()))
    }
}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority
//#endregion 🔖️Backbone
//#endregion 🔖️Backbone
//#endregion 🔖️EphemeralLanes
