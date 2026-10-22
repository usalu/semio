//! 🗒️ Text files, parsed document and ops log: the artifact text files, the parsed document text, the edit line print, the clocks, the ops lines, the loaded history replay, the supersession addresses and the ops log and document text print. See [`super`].

use super::*;

/// 📄️ The two files a textual VCS document is made of: the DSL text (initial snapshot) and
/// the append-only op log (every edit ever created, forwards-only — see {@link parse_document_text}).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ArtifactTextFiles {
    pub dsl: String,
    pub ops: String,
}

/// 🧩️ The result of loading a document from text: the reconstructed envelope plus the live
/// snapshot folded from every edit, so a caller never has to replay again after loading.
#[derive(Debug, PartialEq)]
pub struct ParsedDocumentText<P, Mutation> {
    pub envelope: ArtifactEnvelope<P, Mutation>,
    pub snapshot: P,
}

impl<P, Mutation: super::Mutation<P>> ParsedDocumentText<P, Mutation> {
    /// 🧊️ Takes the envelope and RETIRES the replayed projection beside it. Every load path wants the
    /// envelope only — the store rebuilds its own head projection from the cursor — and a bare
    /// `parsed.envelope` partial move leaves `snapshot` to drop, which aborts the process for any
    /// artifact whose projection owns a fail-closed root (`🌱️value/🗂️ordered/🦀️.rs`'s `Drop`). That is
    /// what killed every `.pack`/`.spr` load of a generation3d document
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn into_envelope(self) -> ArtifactEnvelope<P, Mutation> {
        let Self { envelope, snapshot } = self;
        retire_replayed_projection::<P, Mutation>(snapshot);
        envelope
    }

    /// 🔭️ The mirror of [`Self::into_envelope`]: takes the LIVE replayed projection and discards the
    /// history beside it through the same bounded path a refused parse already uses. For a reader
    /// that wants to LOOK at a document it will never edit — an inference computing over the
    /// artifact a caller named, which owns no store to put an envelope in — and which must not let
    /// the envelope fall out of scope, because every terminal shell below it asserts in `Drop` that
    /// its owners were retired first.
    pub fn into_snapshot(self) -> P {
        let Self { envelope, snapshot } = self;
        discard_parsed_envelope::<P, Mutation>(envelope);
        snapshot
    }
}

/// 📤️ Prints one edit as the complete hot-path append unit: `edit ...` header, one
/// two-space-indented `print_op` line per forward operation, then the matching inverse record and
/// one authoritative metadata record per forward — what `append_ops` / `replay_ops` require. The
/// metadata records carry only persisted facts, like `.spr`: the semantic kind and label every load
/// re-derives from the operation stay out of the text mirror.
pub async fn print_edit_lines<Mutation: OpText>(edit: &Edit<Mutation>) -> Result<String, VcsError> {
    if edit.mutation_meta.len() != edit.forwards.len() {
        return Err(VcsError::ValidationFailed(format!("edit {} has {} metadata entries for {} forward operations", edit.id, edit.mutation_meta.len(), edit.forwards.len())));
    }
    let header = OpsHeaderLine::Edit {
        id: edit.id.clone(),
        sequence: edit.sequence_number,
        started: edit.started_at.clone(),
        actor: edit.actor.clone(),
        finished: edit.finished_at.clone(),
        verb: edit.verb.clone(),
        line: edit.line.iter().cloned().collect(),
    };
    let mut out = header.print_op();
    out.push('\n');
    for operation in &edit.forwards {
        let printed = operation.print_op();
        if printed.contains('\n') {
            return Err(VcsError::Serialize("op-text print_op must not contain a newline".into()));
        }
        out.push_str("  ");
        out.push_str(&printed);
        out.push('\n');
    }
    let mut inverse: Vec<String> = Vec::with_capacity(edit.inverse.len());
    for operation in &edit.inverse {
        let text = operation.print_op();
        if text.contains('\n') {
            return Err(VcsError::Serialize("op-text print_op must not contain a newline".into()));
        }
        inverse.push(text);
    }
    out.push_str(&OpsHeaderLine::Inverse { edit: edit.id.clone(), ops: inverse }.print_op());
    out.push('\n');
    for (index, meta) in edit.mutation_meta.iter().enumerate() {
        let data = semio_framework_pack_json::to_json_string(&MutationMeta { semantic_kind: None, label: None, ..meta.clone() });
        out.push_str(&OpsHeaderLine::Metadata { edit: edit.id.clone(), index: index as u32, data }.print_op());
        out.push('\n');
    }
    Ok(out)
}

/// 📤️ Builds just the op-log half of a textual/pack document — `doc` header, every edit ever
/// created as an `edit` block, then `change`/`checkpoint`/`alternative`/`active` records. Shared by
/// `print_document_text` and `print_document_pack`: the op-log grammar never touches
/// `initial_snapshot`, so it is provably format-invariant and both printers thin out to this plus
/// their own initial-snapshot encoding. Every replay-critical value is explicit: forward and
/// inverse operations, operation metadata, message ledger, conflicts, and cursor.
/// ⏰️ `physical.logical.actor` — the textual HLC every transition line carries.
pub fn print_clock(timestamp: &HybridLogicalTimestamp) -> String {
    format!("{}.{}.{}", timestamp.physical_ms, timestamp.logical, timestamp.actor)
}

pub fn parse_clock(text: &str) -> Result<HybridLogicalTimestamp, String> {
    let mut parts = text.split('.').map(str::parse::<u64>);
    match (parts.next(), parts.next(), parts.next(), parts.next()) {
        (Some(Ok(physical_ms)), Some(Ok(logical)), Some(Ok(actor)), None) => Ok(HybridLogicalTimestamp { actor, physical_ms, logical }),
        _ => Err(format!("invalid transition clock {text}")),
    }
}

pub fn operation_ids(ids: &[MutationId]) -> Vec<String> {
    ids.iter().map(|id| id.0.clone()).collect()
}

pub fn mutation_ids(ids: Vec<String>) -> Vec<MutationId> {
    ids.into_iter().map(MutationId).collect()
}

/// 🖨️ One transition envelope as its semantic `.ops` line.
pub fn ops_line_from_transition(envelope: &semio_framework_os_spr::MutationEnvelope) -> Result<OpsHeaderLine, VcsError> {
    let transition = semio_framework_os_spr::history_transition_from_envelope(envelope).map_err(|error| VcsError::Serialize(error.to_string()))?.ok_or_else(|| VcsError::Serialize(format!("{} is not a history transition", envelope.mutation_id.0)))?;
    let id = envelope.mutation_id.0.clone();
    let actor = envelope.actor.0.clone();
    let clock = print_clock(&envelope.timestamp);
    let after = operation_ids(&envelope.dependencies);
    Ok(match transition {
        semio_framework_os_spr::HistoryTransition::Revert { mutation_ids } => OpsHeaderLine::Revert { id, actor, clock, after, operations: operation_ids(&mutation_ids) },
        semio_framework_os_spr::HistoryTransition::Reinstate { mutation_ids } => OpsHeaderLine::Reinstate { id, actor, clock, after, operations: operation_ids(&mutation_ids) },
        semio_framework_os_spr::HistoryTransition::Commit(checkpoint) => OpsHeaderLine::Commit {
            id,
            actor,
            clock,
            after,
            checkpoint: checkpoint.checkpoint_id,
            parent: checkpoint.parent_id,
            change: checkpoint.change_id,
            operations: operation_ids(&checkpoint.mutation_ids),
            description: checkpoint.description,
            saved: checkpoint.saved_at,
            by: checkpoint.authors.into_iter().map(|author| OpsAuthor { id: author.id, name: author.name, avatar: author.avatar }).collect(),
            message: checkpoint.message,
            at: checkpoint.timestamp,
            line: checkpoint.line_id.into_iter().collect(),
        },
        semio_framework_os_spr::HistoryTransition::Branch { alternative_id, name, checkpoint_id } => OpsHeaderLine::Branch { id, actor, clock, after, alternative: alternative_id, name, checkpoint: checkpoint_id },
        semio_framework_os_spr::HistoryTransition::Checkout { checkpoint_id, alternative_id } => OpsHeaderLine::Checkout { id, actor, clock, after, checkpoint: checkpoint_id, alternative: alternative_id },
        semio_framework_os_spr::HistoryTransition::Repin { checkpoint_id, pinned_checkpoint_id, pins } => {
            OpsHeaderLine::Repin { id, actor, clock, after, checkpoint: checkpoint_id, pinned: pinned_checkpoint_id, pins: pins.into_iter().map(|pin| OpsPin { child: pin.child_uri, checkpoint: pin.checkpoint_id }).collect() }
        }
        semio_framework_os_spr::HistoryTransition::Supersede(supersede) => {
            OpsHeaderLine::Supersede { id, actor, clock, after, scope: supersede.scope, inputs: supersede.inputs.into_iter().map(OpsSupersededInput::from).collect(), observed: envelope.observed.as_ref().map(|observed| observed.0.clone()) }
        }
    })
}

/// 🧾️ Rebuilds the transition envelope a `.ops` transition line describes, refusing a line
/// whose printed id is not the content-addressed id of what it states.
pub fn transition_from_ops_line(document_id: &str, id: String, actor: semio_framework_value::SharedUtf8, clock: &str, after: Vec<String>, transition: semio_framework_os_spr::HistoryTransition) -> Result<semio_framework_os_spr::MutationEnvelope, String> {
    let envelope = semio_framework_os_spr::history_transition_envelope(&transition, &ArtifactId(document_id.to_string()), &ActorId(actor), mutation_ids(after), parse_clock(clock)?);
    if envelope.mutation_id.0 != id {
        return Err(format!("transition line {id} does not match its content-addressed id {}", envelope.mutation_id.0));
    }
    Ok(envelope)
}

/// 📚️ Materializes everything an envelope's event log projects to — change/checkpoint/
/// alternative ledgers, the active alternative and the cursor — from [`fold_envelope_history`].
/// The loaders call it right after reading the persisted events, which are all a document stores.
pub fn project_envelope_history<P, Mutation>(envelope: &mut ArtifactEnvelope<P, Mutation>) -> Result<EffectiveSupersessions, VcsError>
where
    Mutation: super::Mutation<P>,
{
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    let fold = fold_envelope_history(envelope)?;
    let changes = fold.changes.into_iter().map(|change| Change { id: change.id, edit_ids: change.edit_ids, description: change.description, saved_at: change.saved_at }).collect();
    let mut checkpoints = Vec::with_capacity(fold.checkpoints.len());
    for checkpoint in fold.checkpoints {
        let mut composition_pins = Vec::with_capacity(checkpoint.pins.len());
        for pin in checkpoint.pins {
            composition_pins.push(semio_framework_os_vcs::CompositionPin { child_ref: semio_framework_artifact_reference::ArtifactRef::parse_uri(&pin.child_uri).map_err(VcsError::Deserialize)?, checkpoint_id: pin.checkpoint_id });
        }
        let authors = checkpoint.authors.into_iter().map(|author| Author { id: author.id, name: author.name, avatar: author.avatar }).collect();
        checkpoints.push(Checkpoint { id: checkpoint.id, change_ids: checkpoint.change_ids, parent_id: checkpoint.parent_id, authors, message: checkpoint.message, timestamp: checkpoint.timestamp, composition_pins });
    }
    let alternatives = fold.alternatives.into_iter().map(|alternative| Alternative { id: alternative.id, name: alternative.name, checkpoint_ids: alternative.checkpoint_ids }).collect();
    envelope.vcs.changes = ArtifactHistoryLedger::try_from_preflighted(changes).map_err(|_| VcsError::ValidationFailed("history change capacity exceeded".into()))?;
    envelope.vcs.checkpoints = ArtifactHistoryLedger::try_from_preflighted(checkpoints).map_err(|_| VcsError::ValidationFailed("history checkpoint capacity exceeded".into()))?;
    envelope.vcs.alternatives = ArtifactHistoryLedger::try_from_preflighted(alternatives).map_err(|_| VcsError::ValidationFailed("history alternative capacity exceeded".into()))?;
    envelope.active_alternative_id = fold.alternative;
    envelope.cursor = Some(ArtifactCursor::new(fold.applied, fold.redo, fold.checkpoint));
    Ok(fold.supersessions)
}

/// ✏️ Settles a loaded history: persisted inverses and messages of applied edits are derived caches a later supersession
/// or re-projection may have left stale (an append-only log never rewrites the edit record), so every load recomputes
/// them with one Report replay from the genesis — an operation that no longer applies is recorded as a `Fatal` no-op
/// whether or not supersessions exist — and every supersession envelope regains the conflict target its record does not
/// persist. Answers the replayed head projection.
pub fn replay_loaded_history<P, Mutation, I, S>(envelope: &mut ArtifactEnvelope<P, Mutation>, applied: I, supersessions: EffectiveSupersessions) -> Result<P, VcsError>
where
    P: Clone + 'static,
    Mutation: super::Mutation<P> + semio_framework_os_spr::ColdMutation<P> + OpBinary + 'static,
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let result = loaded_history_replay(envelope, applied, supersessions)?.run(&envelope.vcs.edits)?;
    let state = adopt_loaded_replay(envelope, result)?;
    Ok(Arc::try_unwrap(state).unwrap_or_else(|shared| (*shared).clone()))
}

/// ⏪️ The Report replay every load settles its applied history with ([`replay_loaded_history`]): from the genesis, under
/// the fold's effective supersessions. A retained loader steps it under its own deadline.
pub fn loaded_history_replay<P, Mutation, I, S>(envelope: &ArtifactEnvelope<P, Mutation>, applied: I, supersessions: EffectiveSupersessions) -> Result<EditReplay<P, Mutation>, VcsError>
where
    P: Clone,
    Mutation: super::Mutation<P> + semio_framework_os_spr::ColdMutation<P> + OpBinary,
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let order = applied.into_iter().map(|edit_id| edit_id.as_ref().to_string()).collect();
    EditReplay::new(ReplayMode::Report, envelope.vcs.genesis.facts().share_snapshot(), order, 0, &envelope.schema, supersessions, &envelope.vcs.edits)
}

/// ✍️ Writes a finished load replay into `envelope` — every applied edit's rebased inverse and messages, every supersession
/// envelope's conflict target — and answers the replayed head projection.
pub fn adopt_loaded_replay<P, Mutation>(envelope: &mut ArtifactEnvelope<P, Mutation>, mut result: EditReplayResult<P, Mutation>) -> Result<Arc<P>, VcsError>
where
    P: Clone + 'static,
    Mutation: super::Mutation<P> + semio_framework_os_spr::ColdMutation<P> + OpBinary + 'static,
{
    let adopted = take_loaded_replay(envelope, &mut result);
    let retired = result.retire_unscheduled().map_err(|error| VcsError::ValidationFailed(error.into_message()));
    let state = adopted?;
    if let Err(error) = retired {
        retire_shared_projection::<P, Mutation>(state);
        return Err(error);
    }
    Ok(state)
}

/// 🎴️ Writes a finished load replay into `envelope` and takes its reached projection; every untaken owner stays attached to `result`.
pub fn take_loaded_replay<P, Mutation>(envelope: &mut ArtifactEnvelope<P, Mutation>, result: &mut EditReplayResult<P, Mutation>) -> Result<Arc<P>, VcsError>
where
    P: Clone,
    Mutation: super::Mutation<P> + semio_framework_os_spr::ColdMutation<P> + OpBinary,
{
    fill_supersession_targets::<P, Mutation>(envelope)?;
    for (edit_id, inverse) in result.take_rebased_undo_rows() {
        match envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id) {
            Some(edit) => retire_scratch_operations::<P, Mutation>(std::mem::replace(&mut edit.inverse, inverse)),
            None => retire_scratch_operations::<P, Mutation>(inverse),
        }
    }
    for entry in result.take_replayed() {
        if entry.messages.is_empty() {
            envelope.edit_messages.remove_id(&entry.edit_id);
            continue;
        }
        match envelope.edit_messages.get_mut_by_id(&entry.edit_id) {
            Some(established) => *established = entry,
            None => envelope.edit_messages.admit(entry).map_err(|rejected| VcsError::ValidationFailed(format!("replayed messages of edit {} exceed the message ledger", rejected.edit_id)))?,
        }
    }
    let state = result.take_state().ok_or_else(|| VcsError::ValidationFailed("history replay lost its reached projection".into()))?;
    Ok(state)
}

/// 🎯️ Re-derives the conflict target of every supersession envelope that carries none — the persisted transition record
/// does not — from the superseded and replacing operations ([`supersession_address`]).
pub fn fill_supersession_targets<P, Mutation>(envelope: &mut ArtifactEnvelope<P, Mutation>) -> Result<(), VcsError>
where
    Mutation: super::Mutation<P> + OpBinary,
{
    let mut originals: HashMap<MutationId, &Mutation> = HashMap::new();
    for edit in envelope.vcs.edits.iter() {
        originals.extend(semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit).into_iter().zip(edit.forwards.iter()));
    }
    let mut targets = Vec::new();
    for (index, transition) in envelope.transitions.iter().enumerate() {
        if !transition.target.is_empty() {
            continue;
        }
        if let Some(semio_framework_os_spr::HistoryTransition::Supersede(supersede)) = semio_framework_os_spr::history_transition_from_envelope(transition).map_err(|error| VcsError::Deserialize(error.to_string()))? {
            targets.push((index, supersession_address::<P, Mutation>(&supersede, &envelope.schema, |target| originals.get(target).copied())));
        }
    }
    drop(originals);
    for (index, target) in targets {
        envelope.transitions[index].target = target;
    }
    Ok(())
}

/// 🎯️ The address a supersession writes: the longest prefix every superseded operation's and every admitted
/// replacement's conflict target share — the same law authoring stamps and loading re-derives. A replacement breaking
/// the supersede law ([`admit_replacement`]) writes nothing beyond its target.
pub fn supersession_address<'a, P, Mutation>(supersede: &protocol::TransitionSupersede, schema: &str, original: impl Fn(&MutationId) -> Option<&'a Mutation>) -> Vec<String>
where
    Mutation: super::Mutation<P> + OpBinary + 'a,
{
    let mut address: Option<Vec<String>> = None;
    for input in &supersede.inputs {
        let operation = original(&input.target);
        let mut written = operation.map(super::Mutation::<P>::conflict_target).unwrap_or_default();
        if let Some(Ok(Some(replacement))) = operation.map(|operation| admit_replacement::<P, Mutation>(operation, &input.replacement, schema)) {
            written = common_address(&written, &replacement.conflict_target());
            super::Mutation::<P>::retire_cold(replacement);
        }
        address = Some(match address {
            Some(known) => common_address(&known, &written),
            None => written,
        });
    }
    address.unwrap_or_default()
}

/// 🎯️ The longest address prefix `left` and `right` share.
pub fn common_address(left: &[String], right: &[String]) -> Vec<String> {
    left.iter().zip(right.iter()).take_while(|(left, right)| left == right).map(|(segment, _)| segment.clone()).collect()
}

pub async fn print_ops_log<P, Mutation>(envelope: &ArtifactEnvelopeOwners<P, Mutation>) -> Result<String, VcsError>
where
    Mutation: OpText,
{
    let mut ops = String::new();
    ops.push_str(&OpsHeaderLine::Doc { id: envelope.id.clone(), schema: envelope.schema.clone() }.print_op());
    ops.push('\n');
    for edit in envelope.vcs.edits.iter().filter(|edit| envelope.holds_committed_edit(&edit.id)) {
        ops.push_str(&print_edit_lines(edit).await?);
    }
    for transition in &envelope.transitions {
        ops.push_str(&ops_line_from_transition(transition)?.print_op());
        ops.push('\n');
    }
    for entry in envelope.edit_messages.iter().filter(|entry| envelope.holds_committed_edit(&entry.edit_id)) {
        if !envelope.vcs.edits.iter().any(|edit| edit.id == entry.edit_id) {
            return Err(VcsError::ValidationFailed(format!("message ledger references unknown edit {}", entry.edit_id)));
        }
        for message in &entry.messages {
            let data = semio_framework_pack_json::to_json_string(message);
            ops.push_str(&OpsHeaderLine::Message { edit: entry.edit_id.clone(), data }.print_op());
            ops.push('\n');
        }
    }
    for conflict in &envelope.conflicts {
        let data = semio_framework_pack_json::to_json_string(conflict);
        ops.push_str(&OpsHeaderLine::Conflict { data }.print_op());
        ops.push('\n');
    }
    Ok(ops)
}

/// 📤️ Prints the full textual VCS document: the DSL text (initial snapshot) and the complete
/// event log (`doc` header, every edit with its inverse/meta/message records, every history
/// transition, every conflict) — events only; positions are re-derived by folding on load.
/// Replaces the JSON envelope as the canonical persisted form.
pub async fn print_document_text<P, Mutation>(envelope: &ArtifactEnvelopeOwners<P, Mutation>) -> Result<ArtifactTextFiles, VcsError>
where
    P: ArtifactDsl,
    Mutation: OpText,
{
    let dsl = envelope.vcs.genesis.facts().snapshot().print_dsl();
    let ops = print_ops_log(envelope).await?;
    Ok(ArtifactTextFiles { dsl, ops })
}
