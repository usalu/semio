//! 📝️ The document text format: parsing and printing `.dsl`/`.ops` text and `.spr` history into envelopes, edit lines and
//! the mirror files the codecs and the folder backbones write.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
use pack::snapshot::{ArtifactDsl,ArtifactPack};
use crate::os_vcs::io::binary::genesis::{AdmittedArtifactGenesis, ArtifactGenesisCodec};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use crate::os_spr::command::ArtifactMessageSource;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use crate::os_spr::{ActorId, ArtifactId, HybridLogicalTimestamp, MutationId, UndoPolicy};
use crate::os_spr::{DiffRegions, Edit, Mutation, MutationDiff, MutationMeta, OpBinary, OpText};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;
use crate::os_vcs::{Author, VcsError};
use crate::os_vcs::{
    Alternative, ArtifactHistoryLedger, ArtifactVcs, Change, Checkpoint, Identified, Patchable,
};
use super::store_schema::{ArtifactCursor, ArtifactEnvelope, ArtifactEnvelopeOwners, HistoryLane};
use super::artifact_child_owner_schema::OwnerRef;
use super::serialization::{ ArtifactPackFiles, ArtifactSqliteSnapshotHost, encode_document_pack_bytes};
#[cfg(target_arch = "wasm32")]
use super::serialization::{ArtifactCodecApplyFuture};
use semio_framework_diagnostic::TextError;
use semio_framework_diagnostic::TextSpan;
use crate::os_pack::pack_rt;
use super::command_format::{OpsAuthor, OpsHeaderLine, OpsPin, OpsSupersededInput, ops_line_provenance};
use super::ledger::{ArtifactEditMessageLedger, fold_envelope_history};
use super::ledger::{retire_replayed_projection, retire_scratch_operations, stamp_edit_semantics, validate_composition_pins};
use super::canonical_edit::{
    ArtifactCanonicalJson, ArtifactCanonicalJsonTree,
};
use super::replay::{EditReplay, EditReplayResult, EffectiveSupersessions, ReplayEdits, ReplayMode, admit_replacement};
use super::replay::{FoldProjection, retire_shared_projection};
#[cfg(target_arch = "wasm32")]
use super::backbone::resolve_backbone;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use super::backbone::local_storage_backbone_key;

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

impl<P, Mutation: self::Mutation<P>> ParsedDocumentText<P, Mutation> {
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
pub(crate) fn print_clock(timestamp: &HybridLogicalTimestamp) -> String {
    format!("{}.{}.{}", timestamp.physical_ms, timestamp.logical, timestamp.actor)
}

pub(crate) fn parse_clock(text: &str) -> Result<HybridLogicalTimestamp, String> {
    let mut parts = text.split('.').map(str::parse::<u64>);
    match (parts.next(), parts.next(), parts.next(), parts.next()) {
        (Some(Ok(physical_ms)), Some(Ok(logical)), Some(Ok(actor)), None) => Ok(HybridLogicalTimestamp { actor, physical_ms, logical }),
        _ => Err(format!("invalid transition clock {text}")),
    }
}

pub(crate) fn operation_ids(ids: &[MutationId]) -> Vec<String> {
    ids.iter().map(|id| id.0.clone()).collect()
}

pub(crate) fn mutation_ids(ids: Vec<String>) -> Vec<MutationId> {
    ids.into_iter().map(MutationId).collect()
}

/// 🖨️ One transition envelope as its semantic `.ops` line.
pub(crate) fn ops_line_from_transition(envelope: &crate::os_spr::MutationEnvelope) -> Result<OpsHeaderLine, VcsError> {
    let transition = crate::os_spr::history_transition_from_envelope(envelope).map_err(|error| VcsError::Serialize(error.to_string()))?.ok_or_else(|| VcsError::Serialize(format!("{} is not a history transition", envelope.mutation_id.0)))?;
    let id = envelope.mutation_id.0.clone();
    let actor = envelope.actor.0.clone();
    let clock = print_clock(&envelope.timestamp);
    let after = operation_ids(&envelope.dependencies);
    Ok(match transition {
        crate::os_spr::HistoryTransition::Revert { mutation_ids } => OpsHeaderLine::Revert { id, actor, clock, after, operations: operation_ids(&mutation_ids) },
        crate::os_spr::HistoryTransition::Reinstate { mutation_ids } => OpsHeaderLine::Reinstate { id, actor, clock, after, operations: operation_ids(&mutation_ids) },
        crate::os_spr::HistoryTransition::Commit(checkpoint) => OpsHeaderLine::Commit {
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
        crate::os_spr::HistoryTransition::Branch { alternative_id, name, checkpoint_id } => OpsHeaderLine::Branch { id, actor, clock, after, alternative: alternative_id, name, checkpoint: checkpoint_id },
        crate::os_spr::HistoryTransition::Checkout { checkpoint_id, alternative_id } => OpsHeaderLine::Checkout { id, actor, clock, after, checkpoint: checkpoint_id, alternative: alternative_id },
        crate::os_spr::HistoryTransition::Repin { checkpoint_id, pinned_checkpoint_id, pins } => {
            OpsHeaderLine::Repin { id, actor, clock, after, checkpoint: checkpoint_id, pinned: pinned_checkpoint_id, pins: pins.into_iter().map(|pin| OpsPin { child: pin.child_uri, checkpoint: pin.checkpoint_id }).collect() }
        }
        crate::os_spr::HistoryTransition::Supersede(supersede) => {
            OpsHeaderLine::Supersede { id, actor, clock, after, scope: supersede.scope, inputs: supersede.inputs.into_iter().map(OpsSupersededInput::from).collect(), observed: envelope.observed.as_ref().map(|observed| observed.0.clone()) }
        }
    })
}

/// 🧾️ Rebuilds the transition envelope a `.ops` transition line describes, refusing a line
/// whose printed id is not the content-addressed id of what it states.
pub(crate) fn transition_from_ops_line(document_id: &str, id: String, actor: semio_framework_value::SharedUtf8, clock: &str, after: Vec<String>, transition: crate::os_spr::HistoryTransition) -> Result<crate::os_spr::MutationEnvelope, String> {
    let envelope = crate::os_spr::history_transition_envelope(&transition, &ArtifactId(document_id.to_string()), &ActorId(actor), mutation_ids(after), parse_clock(clock)?);
    if envelope.mutation_id.0 != id {
        return Err(format!("transition line {id} does not match its content-addressed id {}", envelope.mutation_id.0));
    }
    Ok(envelope)
}

/// 📚️ Materializes everything an envelope's event log projects to — change/checkpoint/
/// alternative ledgers, the active alternative and the cursor — from [`fold_envelope_history`].
/// The loaders call it right after reading the persisted events, which are all a document stores.
pub(crate) fn project_envelope_history<P, Mutation>(envelope: &mut ArtifactEnvelope<P, Mutation>) -> Result<EffectiveSupersessions, VcsError>
where
    Mutation: self::Mutation<P>,
{
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    let fold = fold_envelope_history(envelope)?;
    let changes = fold.changes.into_iter().map(|change| Change { id: change.id, edit_ids: change.edit_ids, description: change.description, saved_at: change.saved_at }).collect();
    let mut checkpoints = Vec::with_capacity(fold.checkpoints.len());
    for checkpoint in fold.checkpoints {
        let mut composition_pins = Vec::with_capacity(checkpoint.pins.len());
        for pin in checkpoint.pins {
            composition_pins.push(crate::os_vcs::CompositionPin { child_ref: semio_framework_artifact_reference::ArtifactRef::parse_uri(&pin.child_uri).map_err(VcsError::Deserialize)?, checkpoint_id: pin.checkpoint_id });
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
pub(crate) fn replay_loaded_history<P, Mutation, I, S>(envelope: &mut ArtifactEnvelope<P, Mutation>, applied: I, supersessions: EffectiveSupersessions) -> Result<P, VcsError>
where
    P: Clone + 'static,
    Mutation: self::Mutation<P> + OpBinary + 'static,
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let result = loaded_history_replay(envelope, applied, supersessions)?.run(&envelope.vcs.edits)?;
    let state = adopt_loaded_replay(envelope, result)?;
    Ok(Arc::try_unwrap(state).unwrap_or_else(|shared| (*shared).clone()))
}

/// ⏪️ The Report replay every load settles its applied history with ([`replay_loaded_history`]): from the genesis, under
/// the fold's effective supersessions. A retained loader steps it under its own deadline.
pub(crate) fn loaded_history_replay<P, Mutation, I, S>(envelope: &ArtifactEnvelope<P, Mutation>, applied: I, supersessions: EffectiveSupersessions) -> Result<EditReplay<P, Mutation>, VcsError>
where
    P: Clone,
    Mutation: self::Mutation<P> + OpBinary,
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let order = applied.into_iter().map(|edit_id| edit_id.as_ref().to_string()).collect();
    EditReplay::new(ReplayMode::Report, envelope.vcs.genesis.facts().share_snapshot(), order, 0, &envelope.schema, supersessions, &envelope.vcs.edits)
}

/// ✍️ Writes a finished load replay into `envelope` — every applied edit's rebased inverse and messages, every supersession
/// envelope's conflict target — and answers the replayed head projection.
pub(crate) fn adopt_loaded_replay<P, Mutation>(envelope: &mut ArtifactEnvelope<P, Mutation>, mut result: EditReplayResult<P, Mutation>) -> Result<Arc<P>, VcsError>
where
    P: Clone + 'static,
    Mutation: self::Mutation<P> + OpBinary + 'static,
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
pub(crate) fn take_loaded_replay<P, Mutation>(envelope: &mut ArtifactEnvelope<P, Mutation>, result: &mut EditReplayResult<P, Mutation>) -> Result<Arc<P>, VcsError>
where
    P: Clone,
    Mutation: self::Mutation<P> + OpBinary,
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
pub(crate) fn fill_supersession_targets<P, Mutation>(envelope: &mut ArtifactEnvelope<P, Mutation>) -> Result<(), VcsError>
where
    Mutation: self::Mutation<P> + OpBinary,
{
    let mut originals: HashMap<MutationId, &Mutation> = HashMap::new();
    for edit in envelope.vcs.edits.iter() {
        originals.extend(crate::os_spr::mutation_ids_for_edit::<P, Mutation>(edit).into_iter().zip(edit.forwards.iter()));
    }
    let mut targets = Vec::new();
    for (index, transition) in envelope.transitions.iter().enumerate() {
        if !transition.target.is_empty() {
            continue;
        }
        if let Some(crate::os_spr::HistoryTransition::Supersede(supersede)) = crate::os_spr::history_transition_from_envelope(transition).map_err(|error| VcsError::Deserialize(error.to_string()))? {
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
pub(crate) fn supersession_address<'a, P, Mutation>(supersede: &protocol::TransitionSupersede, schema: &str, original: impl Fn(&MutationId) -> Option<&'a Mutation>) -> Vec<String>
where
    Mutation: self::Mutation<P> + OpBinary + 'a,
{
    let mut address: Option<Vec<String>> = None;
    for input in &supersede.inputs {
        let operation = original(&input.target);
        let mut written = operation.map(self::Mutation::<P>::conflict_target).unwrap_or_default();
        if let Some(Ok(Some(replacement))) = operation.map(|operation| admit_replacement::<P, Mutation>(operation, &input.replacement, schema)) {
            written = common_address(&written, &replacement.conflict_target());
            self::Mutation::<P>::retire_cold(replacement);
        }
        address = Some(match address {
            Some(known) => common_address(&known, &written),
            None => written,
        });
    }
    address.unwrap_or_default()
}

/// 🎯️ The longest address prefix `left` and `right` share.
pub(crate) fn common_address(left: &[String], right: &[String]) -> Vec<String> {
    left.iter().zip(right.iter()).take_while(|(left, right)| left == right).map(|(segment, _)| segment.clone()).collect()
}

pub(crate) async fn print_ops_log<P, Mutation>(envelope: &ArtifactEnvelopeOwners<P, Mutation>) -> Result<String, VcsError>
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

/// 🎞️ `crate::os_spr::UndoPolicy` ordinal, matching `HistoryOpMeta.undo_policy`'s wire shape —
/// distinct from `undo_policy_ordinal` above, which maps THIS crate's `ArtifactCommand`-facing
/// `UndoPolicy` (currently `semio_framework_replication::UndoPolicy`; the two enums have identical
/// variants and will merge in the kernel-unification wave, see `protocol_core`'s own doc note).
// 🚫️async: E1 pure ordinal mapping, consumed by `history_op_meta_from_operation_meta` which is
// itself consumed only through `Iterator::map` fn-item arguments — see R9.
pub(crate) fn protocol_undo_policy_ordinal(policy: UndoPolicy) -> u8 {
    match policy {
        UndoPolicy::ExactBaseOnly => 0,
        UndoPolicy::TransformAgainstConcurrent => 1,
        UndoPolicy::SemanticUndo => 2,
        UndoPolicy::CompensatingAction => 3,
    }
}

// 🚫️async: E1 pure ordinal mapping, consumed by `mutation_meta_from_history_op_meta` — see R9.
pub(crate) fn protocol_undo_policy_from_ordinal(ordinal: u8) -> UndoPolicy {
    match ordinal {
        1 => UndoPolicy::TransformAgainstConcurrent,
        2 => UndoPolicy::SemanticUndo,
        3 => UndoPolicy::CompensatingAction,
        _ => UndoPolicy::ExactBaseOnly,
    }
}

// 🚫️async: E1 pure field-copy, consumed only through `Iterator::map` fn-item arguments — see R9.
pub(crate) fn history_message_from_mutation_message(message: &crate::os_spr::MutationMessage) -> crate::os_spr::history::HistoryMessage {
    crate::os_spr::history::HistoryMessage { level: message.level.as_u8(), code: message.code.0.clone(), message: message.message.clone(), target: message.target.clone(), op_index: message.op_index }
}

// 🚫️async: E1 pure field-copy, consumed only through `Iterator::map` fn-item arguments — see R9.
pub(crate) fn mutation_message_from_history_message(message: crate::os_spr::history::HistoryMessage) -> Result<crate::os_spr::MutationMessage, String> {
    let level = semio_framework_diagnostic::Severity::from_u8(message.level).ok_or_else(|| format!("unknown mutation message severity {}", message.level))?;
    Ok(crate::os_spr::MutationMessage { level, code: semio_framework_diagnostic::FaultCode(message.code), message: message.message, target: message.target, op_index: message.op_index })
}

// 🚫️async: E1 pure field-copy, consumed only through `Iterator::map` fn-item arguments — see R9.
pub(crate) fn history_op_meta_from_operation_meta(meta: &MutationMeta, messages: impl IntoIterator<Item = crate::os_spr::history::HistoryMessage>) -> crate::os_spr::HistoryOpMeta {
    crate::os_spr::HistoryOpMeta {
        op_id: meta.mutation_id.as_ref().map(|id| id.0.clone()),
        dependencies: meta.dependencies.iter().map(|id| id.0.clone()).collect(),
        base_version: meta.base_version,
        author_id: meta.author_id.as_ref().map(|id| id.0.clone()),
        hlt: Some((meta.timestamp.actor, meta.timestamp.physical_ms as i64, meta.timestamp.logical)),
        undo_policy: protocol_undo_policy_ordinal(meta.undo_policy),
        payload_hash: meta.payload_hash.as_ref().map(|hash| hash.0),
        group_id: meta.group_id.clone(),
        origin: meta.origin.clone(),
        transaction: meta.transaction.clone(),
        messages: messages.into_iter().collect(),
    }
}

// 🚫️async: E1 pure field-copy, consumed by `mutation_meta_from_history_op_meta`'s own caller
// (§) without `.await` — see R9.
pub(crate) fn mutation_meta_from_history_op_meta(meta: crate::os_spr::HistoryOpMeta) -> Result<(MutationMeta, Vec<crate::os_spr::MutationMessage>), String> {
    let (actor, physical_ms, logical) = meta.hlt.ok_or_else(|| "history operation metadata has no hybrid-clock timestamp".to_string())?;
    let physical_ms = u64::try_from(physical_ms).map_err(|_| "history operation metadata has a negative hybrid-clock physical time".to_string())?;
    if meta.op_id.as_ref().is_none_or(|id| id.trim().is_empty()) {
        return Err("history operation metadata has no stable operation identity".to_string());
    }
    let messages = meta.messages.into_iter().map(mutation_message_from_history_message).collect::<Result<Vec<_>, _>>()?;
    Ok((
        MutationMeta {
            mutation_id: meta.op_id.map(MutationId),
            dependencies: meta.dependencies.into_iter().map(MutationId).collect(),
            base_version: meta.base_version,
            author_id: meta.author_id.map(ActorId),
            timestamp: HybridLogicalTimestamp { actor, physical_ms, logical },
            undo_policy: protocol_undo_policy_from_ordinal(meta.undo_policy),
            payload_hash: meta.payload_hash.map(crate::os_spr::PayloadHash),
            semantic_kind: None,
            label: None,
            group_id: meta.group_id,
            origin: meta.origin,
            transaction: meta.transaction,
        },
        messages,
    ))
}

/// ⚖️ Persistence admits exactly the frozen outcome vocabulary ([`protocol::OUTCOME_CODES`] plus the
/// `mutation.apply.<detail>` family), each code at the one level it fixes.
pub(crate) async fn expected_mutation_message_level(code: &str) -> Option<semio_framework_diagnostic::Severity> {
    protocol::outcome_code_level(code)
}

pub(crate) async fn validate_persisted_message(message: &crate::os_spr::MutationMessage, operation_count: Option<usize>) -> Result<(), VcsError> {
    let Some(expected_level) = expected_mutation_message_level(&message.code.0).await else {
        return Err(VcsError::ValidationFailed(format!("history carries unknown mutation message code {}", message.code.0)));
    };
    if message.level != expected_level || message.message.trim().is_empty() || message.target.iter().any(|target| target.trim().is_empty()) {
        return Err(VcsError::ValidationFailed(format!("history carries malformed mutation message {}", message.code.0)));
    }
    if let Some(operation_count) = operation_count {
        if message.op_index.is_none_or(|index| index as usize >= operation_count) {
            return Err(VcsError::ValidationFailed(format!("history carries an invalid operation index for mutation message {}", message.code.0)));
        }
    }
    Ok(())
}

pub(crate) async fn stable_mutation_ids_for_edit<Mutation>(edit: &Edit<Mutation>) -> Result<Vec<MutationId>, VcsError> {
    if edit.mutation_meta.len() != edit.forwards.len() {
        return Err(VcsError::ValidationFailed(format!("edit {} does not carry one operation identity per forward mutation", edit.id)));
    }
    edit.mutation_meta.iter().map(|meta| meta.mutation_id.clone().filter(|id| !id.0.trim().is_empty()).ok_or_else(|| VcsError::ValidationFailed(format!("edit {} has an operation without a stable identity", edit.id)))).collect()
}

pub(crate) async fn conflict_messages_for_edits<Mutation>(edits: &[Edit<Mutation>], replayed: &[crate::os_spr::EditMessages]) -> Result<Vec<crate::os_spr::MutationMessage>, VcsError> {
    let replayed_by_edit: HashMap<&str, &crate::os_spr::EditMessages> = replayed.iter().map(|entry| (entry.edit_id.as_str(), entry)).collect();
    if replayed_by_edit.len() != replayed.len() {
        return Err(VcsError::ValidationFailed("conflict messages repeat an edit owner".to_string()));
    }
    let mut messages = Vec::new();
    let mut offset = 0u32;
    for edit in edits {
        if let Some(entry) = replayed_by_edit.get(edit.id.as_str()) {
            for message in &entry.messages {
                validate_persisted_message(message, Some(edit.forwards.len())).await?;
                let mut message = message.clone();
                let index = message.op_index.ok_or_else(|| VcsError::ValidationFailed(format!("conflict message {} has no operation index", message.code.0)))?;
                message.op_index = Some(offset.checked_add(index).ok_or_else(|| VcsError::ValidationFailed("conflict operation index overflow".to_string()))?);
                messages.push(message);
            }
        }
        offset = offset
            .checked_add(u32::try_from(edit.forwards.len()).map_err(|_| VcsError::ValidationFailed(format!("edit {} has too many operations", edit.id)))?)
            .ok_or_else(|| VcsError::ValidationFailed("conflict operation index overflow".to_string()))?;
    }
    Ok(messages)
}

pub(crate) async fn canonical_conflict_actors(actors: impl IntoIterator<Item = ActorId>) -> Vec<ActorId> {
    let mut actors: Vec<ActorId> = actors.into_iter().collect();
    actors.sort_by(|left, right| left.0.cmp(&right.0));
    actors.dedup_by(|left, right| left.0 == right.0);
    actors
}

/// 🎯️ A conflict's messages index its kind-specific flattened operation sequence: wire
/// envelopes in `Quarantined` order, or every forward operation of `Degraded` edits in
/// `edit_ids` order. This makes `op_index` globally unambiguous for a conflict rather
/// than accidentally treating every message as if it belonged to its first edit.
pub(crate) async fn validate_persisted_conflicts<P, Mutation>(envelope: &ArtifactEnvelopeOwners<P, Mutation>) -> Result<(), VcsError> {
    let mut conflict_ids = HashSet::new();
    let known_edits: HashMap<&str, &Edit<Mutation>> = envelope.vcs.edits.iter().map(|edit| (edit.id.as_str(), edit)).collect();
    for conflict in &envelope.conflicts {
        if conflict.id.0.trim().is_empty() || !conflict_ids.insert(conflict.id.0.as_str()) || conflict.messages.is_empty() {
            return Err(VcsError::ValidationFailed(format!("history repeats or omits conflict identity {}", conflict.id.0)));
        }
        if conflict.actors.is_empty() || conflict.actors.iter().any(|actor| actor.0.trim().is_empty()) || conflict.actors != canonical_conflict_actors(conflict.actors.clone()).await {
            return Err(VcsError::ValidationFailed(format!("conflict {} has malformed actor identities", conflict.id.0)));
        }
        let (mutation_ids, operation_count) = match &conflict.kind {
            crate::os_spr::ConflictKind::Quarantined { envelopes } => {
                if envelopes.is_empty() {
                    return Err(VcsError::ValidationFailed(format!("quarantined conflict {} has no mutation envelopes", conflict.id.0)));
                }
                let mut mutation_ids = Vec::with_capacity(envelopes.len());
                let expected_actors = canonical_conflict_actors(envelopes.iter().map(|remote| remote.actor.clone())).await;
                if conflict.actors != expected_actors {
                    return Err(VcsError::ValidationFailed(format!("quarantined conflict {} has actors inconsistent with its envelopes", conflict.id.0)));
                }
                for remote in envelopes {
                    let mut dependencies = HashSet::new();
                    if remote.document_id.0 != envelope.id
                        || remote.diff.schema.0 != envelope.schema
                        || remote.inverse.schema != remote.diff.schema
                        || remote.mutation_id.0.trim().is_empty()
                        || remote.actor.0.trim().is_empty()
                        || remote.dependencies.iter().any(|dependency| dependency.0.trim().is_empty() || dependency == &remote.mutation_id || !dependencies.insert(dependency.0.as_str()))
                    {
                        return Err(VcsError::ValidationFailed(format!("quarantined conflict {} has invalid mutation envelopes", conflict.id.0)));
                    }
                    if conflict.timestamp < remote.timestamp {
                        return Err(VcsError::ValidationFailed(format!("quarantined conflict {} predates its mutation envelope", conflict.id.0)));
                    }
                    mutation_ids.push(remote.mutation_id.clone());
                }
                let mut ids = HashSet::new();
                if mutation_ids.iter().any(|mutation_id| !ids.insert(mutation_id.0.as_str())) {
                    return Err(VcsError::ValidationFailed(format!("quarantined conflict {} repeats a mutation identity", conflict.id.0)));
                }
                (mutation_ids, envelopes.len())
            }
            crate::os_spr::ConflictKind::Degraded { edit_ids } => {
                if edit_ids.is_empty() {
                    return Err(VcsError::ValidationFailed(format!("degraded conflict {} has no edit references", conflict.id.0)));
                }
                let mut ids = HashSet::new();
                let mut mutation_ids = Vec::new();
                let mut expected_actors = Vec::new();
                for edit_id in edit_ids {
                    if !ids.insert(edit_id.as_str()) {
                        return Err(VcsError::ValidationFailed(format!("degraded conflict {} repeats edit reference {edit_id}", conflict.id.0)));
                    }
                    let edit = known_edits.get(edit_id.as_str()).ok_or_else(|| VcsError::ValidationFailed(format!("degraded conflict {} references unknown edit {edit_id}", conflict.id.0)))?;
                    let timestamp = edit.mutation_meta.first().map(|meta| meta.timestamp).ok_or_else(|| VcsError::ValidationFailed(format!("degraded conflict {} references edit {edit_id} without operation metadata", conflict.id.0)))?;
                    if conflict.timestamp < timestamp {
                        return Err(VcsError::ValidationFailed(format!("degraded conflict {} predates edit {edit_id}", conflict.id.0)));
                    }
                    expected_actors.push(ActorId(edit.actor.clone().ok_or_else(|| VcsError::ValidationFailed(format!("degraded conflict {} references actorless edit {edit_id}", conflict.id.0)))?));
                    mutation_ids.extend(stable_mutation_ids_for_edit(edit).await.map_err(|_| VcsError::ValidationFailed(format!("degraded conflict {} references edit {edit_id} without stable operation identities", conflict.id.0)))?);
                }
                if conflict.actors != canonical_conflict_actors(expected_actors).await {
                    return Err(VcsError::ValidationFailed(format!("degraded conflict {} has actors inconsistent with its edits", conflict.id.0)));
                }
                let operation_count = mutation_ids.len();
                (mutation_ids, operation_count)
            }
        };
        for message in &conflict.messages {
            validate_persisted_message(message, Some(operation_count)).await?;
        }
        if conflict.id != crate::os_spr::ConflictId::new(&conflict.kind, &ArtifactId(envelope.id.clone()), &mutation_ids, &conflict.timestamp).await {
            return Err(VcsError::ValidationFailed(format!("conflict {} does not match its content-addressed identity", conflict.id.0)));
        }
    }
    Ok(())
}

/// 🧱️ Validates the complete durable history graph at every persistence boundary.
/// Deliberately free of store construction bounds: codecs must reject malformed history before a
/// caller needs to construct an `ArtifactStore`.
pub(crate) async fn validate_durable_history<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> Result<(), VcsError> {
    let mut edit_ids = HashSet::new();
    let mut edit_sequences = HashSet::new();
    for edit in &envelope.vcs.edits {
        if edit.actor.as_deref().is_none_or(|actor| actor.trim().is_empty()) || edit.mutation_meta.iter().any(|meta| meta.author_id.as_ref().is_none_or(|actor| actor.0.trim().is_empty())) {
            return Err(VcsError::ValidationFailed(format!("history edit {} has no authoritative author", edit.id)));
        }
        if !edit_ids.insert(edit.id.as_str()) {
            return Err(VcsError::ValidationFailed(format!("history repeats authoritative edit {}", edit.id)));
        }
        if edit.sequence_number < 0 || !edit_sequences.insert(edit.sequence_number) {
            return Err(VcsError::ValidationFailed(format!("history has an invalid edit sequence {} for {}", edit.sequence_number, edit.id)));
        }
    }
    let mut message_edit_ids = HashSet::new();
    for entry in &envelope.edit_messages {
        if !message_edit_ids.insert(entry.edit_id.as_str()) || !edit_ids.contains(entry.edit_id.as_str()) {
            return Err(VcsError::ValidationFailed(format!("history has an invalid message ledger for edit {}", entry.edit_id)));
        }
        let operation_count = envelope.vcs.edits.find_near(|edit| edit.id == entry.edit_id).map(|edit| edit.forwards.len()).ok_or_else(|| VcsError::ValidationFailed(format!("history has an invalid message ledger for edit {}", entry.edit_id)))?;
        let mut any_malformed = false;
        for message in &entry.messages {
            if validate_persisted_message(message, Some(operation_count)).await.is_err() {
                any_malformed = true;
                break;
            }
        }
        if entry.messages.is_empty() || any_malformed {
            return Err(VcsError::ValidationFailed(format!("history has malformed messages for edit {}", entry.edit_id)));
        }
    }
    validate_persisted_conflicts(envelope).await?;
    let mut change_ids = HashSet::new();
    for change in &envelope.vcs.changes {
        if !change_ids.insert(change.id.as_str()) {
            return Err(VcsError::ValidationFailed(format!("history repeats authoritative change {}", change.id)));
        }
        let mut referenced_edits = HashSet::new();
        for edit_id in &change.edit_ids {
            if !referenced_edits.insert(edit_id.as_str()) || !edit_ids.contains(edit_id.as_str()) {
                return Err(VcsError::ValidationFailed(format!("change {} has an invalid edit reference {edit_id}", change.id)));
            }
        }
    }
    let mut checkpoint_ids = HashSet::new();
    for checkpoint in &envelope.vcs.checkpoints {
        if !checkpoint_ids.insert(checkpoint.id.as_str()) {
            return Err(VcsError::ValidationFailed(format!("history repeats authoritative checkpoint {}", checkpoint.id)));
        }
    }
    for checkpoint in &envelope.vcs.checkpoints {
        let mut referenced_changes = HashSet::new();
        for change_id in &checkpoint.change_ids {
            if !referenced_changes.insert(change_id.as_str()) || !change_ids.contains(change_id.as_str()) {
                return Err(VcsError::ValidationFailed(format!("checkpoint {} has an invalid change reference {change_id}", checkpoint.id)));
            }
        }
        if let Some(parent_id) = &checkpoint.parent_id {
            if !checkpoint_ids.contains(parent_id.as_str()) || parent_id == &checkpoint.id {
                return Err(VcsError::ValidationFailed(format!("checkpoint {} has an invalid parent reference {parent_id}", checkpoint.id)));
            }
        }
        validate_composition_pins(&checkpoint.composition_pins).await?;
    }
    let mut alternative_ids = HashSet::new();
    for alternative in &envelope.vcs.alternatives {
        if !alternative_ids.insert(alternative.id.as_str()) {
            return Err(VcsError::ValidationFailed(format!("history repeats authoritative alternative {}", alternative.id)));
        }
        let mut referenced_checkpoints = HashSet::new();
        for checkpoint_id in &alternative.checkpoint_ids {
            if !referenced_checkpoints.insert(checkpoint_id.as_str()) || !checkpoint_ids.contains(checkpoint_id.as_str()) {
                return Err(VcsError::ValidationFailed(format!("alternative {} has an invalid checkpoint reference {checkpoint_id}", alternative.id)));
            }
        }
    }
    if let Some(active_alternative_id) = &envelope.active_alternative_id {
        if !alternative_ids.contains(active_alternative_id.as_str()) {
            return Err(VcsError::ValidationFailed(format!("history names an unknown active alternative {active_alternative_id}")));
        }
    }
    if let Some(cursor) = &envelope.cursor {
        validate_history_lanes(envelope, &cursor.applied_edit_ids, &cursor.redo_edit_ids).await?;
        if let Some(checkpoint_id) = &cursor.checkpoint_id {
            if !checkpoint_ids.contains(checkpoint_id.as_str()) {
                return Err(VcsError::ValidationFailed(format!("history cursor names an unknown checkpoint {checkpoint_id}")));
            }
        }
    }
    Ok(())
}

pub(crate) async fn validate_history_lanes<P, Mutation, I, J, S, R>(envelope: &ArtifactEnvelope<P, Mutation>, applied_edit_ids: I, redo_edit_ids: J) -> Result<(), VcsError>
where
    I: IntoIterator<Item = S>,
    J: IntoIterator<Item = R>,
    S: AsRef<str>,
    R: AsRef<str>,
{
    let known = envelope.vcs.edits.iter().map(|edit| edit.id.as_str()).collect::<HashSet<_>>();
    let mut applied = HashSet::new();
    for edit_id in applied_edit_ids {
        let edit_id = edit_id.as_ref();
        if !applied.insert(edit_id.to_string()) {
            return Err(VcsError::ValidationFailed(format!("history repeats applied edit {edit_id}")));
        }
        if !known.contains(edit_id) {
            return Err(VcsError::UnknownEdit(edit_id.to_string()));
        }
    }
    let mut redo = HashSet::new();
    for edit_id in redo_edit_ids {
        let edit_id = edit_id.as_ref();
        if !redo.insert(edit_id.to_string()) {
            return Err(VcsError::ValidationFailed(format!("history repeats redo edit {edit_id}")));
        }
        if applied.contains(edit_id) {
            return Err(VcsError::ValidationFailed("history places an edit in both applied and redo lanes".to_string()));
        }
        if !known.contains(edit_id) {
            return Err(VcsError::UnknownEdit(edit_id.to_string()));
        }
    }
    Ok(())
}

/// 🎯️ Builds the binary op-log twin of `print_ops_log` — a `crate::os_spr::HistoryLog` carrying
/// REAL `inverse`/binary op payloads/explicit meta/cursor, encoded via `crate::os_spr::encode_history`
/// with `write_backwards_section: true`. Unlike the `.ops` text mirror (forwards-only, see
/// `print_ops_log`'s doc), this is the AUTHORITATIVE persisted form: `parse_document_spr` recovers
/// inverse/meta byte-for-byte instead of recomputing them via replay.
pub(crate) async fn history_op_payloads<'a, Mutation: OpBinary + 'a>(mutations: impl IntoIterator<Item = &'a Mutation>) -> Result<Vec<crate::os_spr::OpPayload>, VcsError> {
    let mut payloads = Vec::new();
    for op in mutations {
        let binary = op.encode_op().map_err(|error| VcsError::Serialize(error.to_string()))?;
        payloads.push(crate::os_spr::OpPayload { text: None, binary: Some(binary) });
    }
    Ok(payloads)
}

/// 🎯️ An empty `mutation_meta` (e.g. a hand-authored/externally-injected edit with no
/// explicit meta, distinct from a real dispatch which always populates one entry per
/// forward op) is treated as ABSENT, not as `Some(vec![])` — `encode_edit` requires
/// `metas.len() == ops.len()` when meta is present at all, and an empty-but-`Some` vec
/// would spuriously fail that check for a non-empty `ops`.
pub(crate) async fn history_edit_from_edit<Mutation: OpBinary>(edit: &Edit<Mutation>, messages: &[crate::os_spr::MutationMessage], lane: Option<HistoryLane>) -> Result<crate::os_spr::HistoryEdit, VcsError> {
    if messages.iter().any(|message| message.op_index.is_none_or(|index| index as usize >= edit.forwards.len())) {
        return Err(VcsError::ValidationFailed(format!("edit {} carries a message without a valid operation index", edit.id)));
    }
    Ok(crate::os_spr::HistoryEdit {
        id: edit.id.clone(),
        actor: edit.actor.clone(),
        started_at: edit.started_at.clone(),
        finished_at: edit.finished_at.clone(),
        verb: edit.verb.clone(),
        line: edit.line.clone(),
        ops: history_op_payloads(&edit.forwards).await?,
        inverse: history_op_payloads(&edit.inverse).await?,
        meta: if edit.mutation_meta.is_empty() {
            if messages.is_empty() {
                None
            } else {
                return Err(VcsError::ValidationFailed(format!("edit {} carries messages without operation metadata", edit.id)));
            }
        } else {
            Some(edit.mutation_meta.iter().enumerate().map(|(index, meta)| history_op_meta_from_operation_meta(meta, messages.iter().filter(move |message| message.op_index == Some(index as u32)).map(history_message_from_mutation_message))).collect())
        },
        lane: lane.filter(|lane| !matches!(lane, HistoryLane::Document)).map(|lane| match lane.to_value() {
            DslValue::String(name) => name,
            other => unreachable!("a HistoryLane always encodes as its own camelCase word, got {other:?}"),
        }),
    })
}

/// 🌀️ `encode_envelope` is async (📡️replication, out of this packet's scope); `map`'s
/// closure is sync (R10 shape 1), so it's hoisted into an explicit loop.
pub(crate) async fn history_conflict_from_conflict(conflict: &crate::os_spr::Conflict) -> crate::os_spr::history::HistoryConflict {
    let (kind, edit_ids, envelopes) = match &conflict.kind {
        crate::os_spr::ConflictKind::Quarantined { envelopes, .. } => {
            let mut encoded = Vec::with_capacity(envelopes.len());
            for envelope in envelopes {
                let mut bytes = Vec::new();
                crate::os_spr::encode_envelope(envelope, &mut bytes);
                encoded.push(bytes);
            }
            (0, Vec::new(), encoded)
        }
        crate::os_spr::ConflictKind::Degraded { edit_ids } => (1, edit_ids.clone(), Vec::new()),
    };
    crate::os_spr::history::HistoryConflict {
        id: conflict.id.0.clone(),
        kind,
        status: match conflict.status {
            crate::os_spr::ConflictStatus::Open => 0,
            crate::os_spr::ConflictStatus::Accepted => 1,
            crate::os_spr::ConflictStatus::Discarded => 2,
        },
        actors: conflict.actors.iter().map(|actor| actor.0.clone()).collect(),
        hlt: (conflict.timestamp.actor, conflict.timestamp.physical_ms, conflict.timestamp.logical),
        edit_ids,
        envelopes,
        messages: conflict.messages.iter().map(history_message_from_mutation_message).collect(),
    }
}

pub(crate) fn conflict_from_history_conflict(conflict: crate::os_spr::history::HistoryConflict) -> Result<crate::os_spr::Conflict, String> {
    let kind = match conflict.kind {
        0 => {
            let mut envelopes = Vec::with_capacity(conflict.envelopes.len());
            for bytes in conflict.envelopes {
                let mut position = 0;
                let envelope = crate::os_spr::decode_envelope(&bytes, &mut position).map_err(|error| error.to_string())?;
                if position != bytes.len() {
                    return Err("quarantined conflict envelope has trailing bytes".into());
                }
                envelopes.push(envelope);
            }
            crate::os_spr::ConflictKind::Quarantined { envelopes }
        }
        1 => crate::os_spr::ConflictKind::Degraded { edit_ids: conflict.edit_ids },
        value => return Err(format!("unknown conflict kind {value}")),
    };
    let status = match conflict.status {
        0 => crate::os_spr::ConflictStatus::Open,
        1 => crate::os_spr::ConflictStatus::Accepted,
        2 => crate::os_spr::ConflictStatus::Discarded,
        value => return Err(format!("unknown conflict status {value}")),
    };
    Ok(crate::os_spr::Conflict {
        id: crate::os_spr::ConflictId(conflict.id),
        kind,
        status,
        messages: conflict.messages.into_iter().map(mutation_message_from_history_message).collect::<Result<Vec<_>, _>>()?,
        actors: conflict.actors.into_iter().map(ActorId).collect(),
        timestamp: HybridLogicalTimestamp { actor: conflict.hlt.0, physical_ms: conflict.hlt.1, logical: conflict.hlt.2 },
    })
}

/// 🎯️ Encodes a bare, edit-free `.spr` op log for `schema` — the counterpart to a `.pack`
/// file carrying only an initial snapshot with no history yet (e.g. a single dropped `.pack`
/// file with no accompanying `.spr` sidecar). `doc_id` may be empty when the caller mints a fresh
/// id downstream (as `parse_document_spr` never cross-checks it against the pack). LAW:
/// `parse_document_spr(pack, &empty_document_spr(id, schema))` recovers exactly `P::decode_pack(pack)`
/// as both the initial and live snapshot, with zero edits.
pub async fn empty_document_spr(doc_id: &str, schema: &str) -> Vec<u8> {
    let log = crate::os_spr::HistoryLog { doc_id: doc_id.to_string(), schema: schema.to_string(), ..crate::os_spr::HistoryLog::default() };
    crate::os_spr::encode_history(&log, &crate::os_spr::EncodeOptions::default()).await.expect("encoding an edit-free HistoryLog is infallible")
}

/// 🪪️ Re-stamps an app-built document `.spr` with the identity of the live document it is
/// loaded into — id, schema, dialect, owner — keeping every history event.
/// A command replacing its whole document (`Effect::LoadDocument`) builds that log without knowing
/// where its store is mounted, and archive hydration rejects any log whose identity differs.
pub async fn stamp_document_spr_identity(spr: &[u8], doc_id: &str, schema: &str, dialect: &semio_framework_artifact_reference::ArtifactDialect, owner: Option<&OwnerRef>) -> Result<Vec<u8>, VcsError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    let mut log = crate::os_spr::decode_history(spr, &crate::os_spr::DecodeOptions::default()).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
    log.doc_id = doc_id.to_string();
    log.schema = schema.to_string();
    log.composition =
        Some(crate::os_spr::HistoryComposition { owner: owner.map(|owner| (owner.parent.to_uri(), owner.slot.clone(), owner.child_id.clone())), dialect: Some((dialect.artifact_kind.clone(), dialect.standard.clone(), dialect.subset.clone())) });
    let options = crate::os_spr::EncodeOptions { write_backwards_section: true, ..crate::os_spr::EncodeOptions::default() };
    crate::os_spr::encode_history(&log, &options).await.map_err(|error| VcsError::Serialize(error.to_string()))
}

/// 🌱️ A brand-new owned member's full envelope pack — `initial_pack` plus an edit-free `.spr`
/// stamped with the child's id, `schema`, `dialect` and `owner` through the same
/// `stamp_document_spr_identity` path a parent's own whole-document load takes — in exactly the
/// `encode_document_pack_bytes` framing `MemberFactory::open` and archive member admission accept.
/// Minted from bytes alone, never through a live `ArtifactStore`, so a caller completing a closure
/// (`VcsArtifactApp`'s derivable-child genesis) owes no bounded close protocol for it.
pub async fn genesis_member_envelope_pack(schema: &str, expected: &semio_framework_artifact_reference::ArtifactRef, owner: &OwnerRef, initial_pack: &[u8]) -> Result<Vec<u8>, VcsError> {
    if initial_pack.is_empty() {
        return Err(VcsError::Deserialize(format!("child genesis for {} carries an empty initial pack", owner.child_id)));
    }
    let spr = stamp_document_spr_identity(&empty_document_spr(&expected.artifact_id, schema).await, &expected.artifact_id, schema, &expected.dialect, Some(owner)).await?;
    Ok(encode_document_pack_bytes(initial_pack, &spr).await)
}

/// ➕️ Appends `edits` and `transitions` to an already-encoded `.spr` byte log — decode,
/// extend, re-encode. The log holds events only, so an appended event is live for the next reader
/// exactly as the fold orders it. O(history) per call — a caller appending many
/// batches back-to-back pays the whole decode/encode cost each time; a streaming variant
/// (`SprWriter::resume` + a seeded `DictBuilder`/`edit_ordinals`) is a follow-up optimization, not
/// required for correctness (this function's asymptotics match the JSON-envelope full rewrite it
/// replaces).
pub async fn append_history_events_to_spr(spr: &[u8], edits: &[crate::os_spr::HistoryEdit], transitions: &[crate::os_spr::HistoryTransitionRecord]) -> Result<Vec<u8>, VcsError> {
    let mut log = crate::os_spr::decode_history(spr, &crate::os_spr::DecodeOptions::default()).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
    log.edits.extend(edits.iter().cloned());
    log.transitions.extend(transitions.iter().cloned());
    let options = crate::os_spr::EncodeOptions { write_backwards_section: true, ..crate::os_spr::EncodeOptions::default() };
    crate::os_spr::encode_history(&log, &options).await.map_err(|error| VcsError::Serialize(error.to_string()))
}

/// 🧩️ Projects an envelope's composition facts (`owner`, `dialect`) into the durable
/// `HistoryComposition` overlay, or `None` when the document
/// has none — which is the overwhelming majority, so an ordinary leaf document's `.spr` gains not a
/// single byte from this record existing. `ArtifactRef`s cross the boundary as their `to_uri()`
/// wire string, the same "own the codec at this edge" convention `CompositionPin`/`ArtifactChild`
/// already use rather than coupling `ArtifactRef` into the protocol crate.
pub(crate) async fn history_composition_from_envelope<P, Mutation>(envelope: &ArtifactEnvelopeOwners<P, Mutation>) -> Option<crate::os_spr::HistoryComposition> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    let owner = envelope.owner.as_ref().map(|owner| (owner.parent.to_uri(), owner.slot.clone(), owner.child_id.clone()));
    let dialect = envelope.dialect.as_ref().map(|dialect| (dialect.artifact_kind.clone(), dialect.standard.clone(), dialect.subset.clone()));
    if owner.is_none() && dialect.is_none() {
        return None;
    }
    Some(crate::os_spr::HistoryComposition { owner, dialect })
}

/// 🧩️ Inverse of `history_composition_from_envelope`: malformed ownership rejects the
/// authoritative history rather than being silently discarded. Checkpoint pins are not part of the
/// overlay — they are facts of `Repin` transitions, folded like every other history fact.
pub(crate) async fn apply_history_composition<P, Mutation>(envelope: &mut ArtifactEnvelope<P, Mutation>, composition: &crate::os_spr::HistoryComposition) -> Result<(), VcsError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    envelope.owner = match &composition.owner {
        Some((parent, slot, child_id)) => Some(OwnerRef { parent: semio_framework_artifact_reference::ArtifactRef::parse_uri(parent).map_err(VcsError::Deserialize)?, slot: slot.clone(), child_id: child_id.clone() }),
        None => None,
    };
    envelope.dialect = composition.dialect.as_ref().map(|(artifact_kind, standard, subset)| semio_framework_artifact_reference::ArtifactDialect { artifact_kind: artifact_kind.clone(), standard: standard.clone(), subset: subset.clone() });
    Ok(())
}

/// 🌀️ `history_conflict_from_conflict` is async (calls the 📡️replication `encode_envelope`);
/// `Iterator::map`'s closure is sync (R10 shape 1), so it's hoisted into an explicit loop.
pub async fn print_document_spr<P, Mutation>(envelope: &ArtifactEnvelopeOwners<P, Mutation>) -> Result<Vec<u8>, VcsError>
where
    Mutation: OpBinary,
{
    validate_persisted_conflicts(envelope).await?;
    let mut message_ledger = BTreeMap::new();
    for entry in envelope.edit_messages.iter().filter(|entry| envelope.holds_committed_edit(&entry.edit_id)) {
        if message_ledger.insert(entry.edit_id.as_str(), entry.messages.as_slice()).is_some() {
            return Err(VcsError::ValidationFailed(format!("duplicate message ledger for edit {}", entry.edit_id)));
        }
        if !envelope.vcs.edits.iter().any(|edit| edit.id == entry.edit_id) {
            return Err(VcsError::ValidationFailed(format!("message ledger references unknown edit {}", entry.edit_id)));
        }
        let operation_count = envelope.vcs.edits.find_near(|edit| edit.id == entry.edit_id).map(|edit| edit.forwards.len()).ok_or_else(|| VcsError::ValidationFailed(format!("message ledger references unknown edit {}", entry.edit_id)))?;
        if entry.messages.is_empty() {
            return Err(VcsError::ValidationFailed(format!("message ledger for edit {} is empty", entry.edit_id)));
        }
        for message in entry.messages.iter() {
            validate_persisted_message(message, Some(operation_count)).await?;
        }
    }
    let mut edits = crate::os_spr::history::HistoryEdits::empty();
    for edit in envelope.vcs.edits.iter().filter(|edit| envelope.holds_committed_edit(&edit.id)) {
        edits.push(history_edit_from_edit::<Mutation>(edit, message_ledger.get(edit.id.as_str()).copied().unwrap_or(&[]), envelope.lanes.get(&edit.id).copied()).await?);
    }
    let mut conflicts = Vec::with_capacity(envelope.conflicts.len());
    for conflict in &envelope.conflicts {
        conflicts.push(history_conflict_from_conflict(conflict).await);
    }
    let log = crate::os_spr::HistoryLog {
        doc_id: envelope.id.clone(),
        schema: envelope.schema.clone(),
        edits,
        transitions: envelope.transitions.iter().map(crate::os_spr::HistoryTransitionRecord::from_envelope).collect(),
        composition: history_composition_from_envelope(envelope).await,
        conflicts,
        viewer_line: envelope.active_alternative_id.clone().filter(|line| line != &crate::os_spr::trunk_alternative_id(&ArtifactId(envelope.id.clone()))),
        viewer_checkpoint: envelope.viewer_checkpoint_id.clone(),
    };
    let options = crate::os_spr::EncodeOptions { write_backwards_section: true, ..crate::os_spr::EncodeOptions::default() };
    crate::os_spr::encode_history(&log, &options).await.map_err(|error| VcsError::Serialize(error.to_string()))
}

/// 🎯️ Inverse of [`print_document_spr`]: rebuilds an envelope's event log — `edits` (with
/// their persisted `inverse`/`mutation_meta`) and `transitions` — from a decoded `HistoryLog`, then
/// folds it into every derived position and fact ([`project_envelope_history`]). The initial
/// snapshot comes from `pack` via `ArtifactPack::decode_pack`, matching `parse_document_pack`'s contract.
pub async fn parse_document_spr<P, Mutation>(pack: &[u8], spr: &[u8]) -> Result<ParsedDocumentText<P, Mutation>, TextError>
where
    P: Clone + ArtifactPack + 'static,
    Mutation: OpText + OpBinary + self::Mutation<P> + 'static,
{
    let log = crate::os_spr::decode_history(spr, &crate::os_spr::DecodeOptions::default()).await.map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), TextSpan::at(1, 1)))?;
    parse_decoded_document_spr(pack, log).await
}

/// 🧊️ The validation replay's final projection is a SCRATCH value nothing below reads — the
/// authoritative one is rebuilt from the cursor further down. Shadowing it there left this
/// binding to a bare drop at the end of the function, which aborts the process for any artifact
/// whose projection owns a fail-closed root: every `.pack`/`.spr` load of a generation3d document
/// with a layout entry died in `OrderedMap<WidgetLayout>::drop`
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). `ReplayProjection` now retires it on every early
/// `?` above as well, which the explicit call below never covered.
pub async fn parse_decoded_document_spr<P, Mutation>(pack: &[u8], mut log: crate::os_spr::HistoryLog) -> Result<ParsedDocumentText<P, Mutation>, TextError>
where
    P: Clone + ArtifactPack + 'static,
    Mutation: OpText + OpBinary + self::Mutation<P> + 'static,
{
    let initial_snapshot = P::decode_pack(pack).map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), TextSpan::at(1, 1)))?;

    async fn decode_op<P, Mutation: OpText + OpBinary + self::Mutation<P>>(payload: &crate::os_spr::OpPayload) -> Result<Mutation, TextError> {
        match (&payload.binary, &payload.text) {
            (Some(bytes), _) => Mutation::decode_op(bytes).map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), TextSpan::at(1, 1))),
            (None, Some(text)) => Mutation::parse_op(text),
            (None, None) => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "op payload carries neither binary nor text".to_string(), TextSpan::at(1, 1))),
        }
    }

    async fn decode_ops<P, Mutation: OpText + OpBinary + self::Mutation<P>>(payloads: &[crate::os_spr::OpPayload]) -> Result<Vec<Mutation>, TextError> {
        let mut out = Vec::with_capacity(payloads.len());
        for payload in payloads {
            out.push(decode_op::<P, Mutation>(payload).await?);
        }
        Ok(out)
    }

    let schema = log.schema.clone();
    let mut edits: Vec<Edit<Mutation>> = Vec::with_capacity(log.edits.len());
    let mut edit_messages = Vec::new();
    let mut lanes = protocol::HistoryFoldIndex::<String, HistoryLane>::new();
    for (index, history_edit) in log.edits.into_iter().enumerate() {
        let edit_id = history_edit.id.clone();
        if let Some(lane) = &history_edit.lane {
            let lane = HistoryLane::from_value(DslValue::String(lane.clone()))
                .map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("history edit {edit_id} names an unknown lane: {error}"), TextSpan::at(1, 1)))?;
            if !lane.is_document().await {
                lanes.insert(edit_id.clone(), lane);
            }
        }
        let forwards = decode_ops::<P, Mutation>(&history_edit.ops).await?;
        let inverse = decode_ops::<P, Mutation>(&history_edit.inverse).await?;
        let metas = history_edit.meta.ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("history edit {edit_id} has no authoritative operation metadata"), TextSpan::at(1, 1)))?;
        if metas.len() != forwards.len() {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("history edit {edit_id} has {} metadata entries for {} forward operations", metas.len(), forwards.len()), TextSpan::at(1, 1)));
        }
        let mut durable_messages = Vec::new();
        let mutation_meta = metas
            .into_iter()
            .map(|meta| {
                let (meta, messages) = mutation_meta_from_history_op_meta(meta)?;
                durable_messages.extend(messages);
                Ok(meta)
            })
            .collect::<Result<Vec<_>, String>>()
            .map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, TextSpan::at(1, 1)))?;
        if !durable_messages.is_empty() {
            edit_messages.push(crate::os_spr::EditMessages { edit_id: edit_id.clone(), messages: durable_messages });
        }
        let mut edit = Edit {
            id: history_edit.id,
            actor: history_edit.actor,
            forwards,
            inverse: inverse.into(),
            mutation_meta,
            verb: history_edit.verb,
            line: history_edit.line,
            sequence_number: index as i32 + 1,
            started_at: history_edit.started_at,
            finished_at: history_edit.finished_at,
        };
        stamp_edit_semantics::<P, Mutation>(&mut edit, &schema);
        edits.push(edit);
    }

    let mut conflicts = Vec::with_capacity(log.conflicts.len());
    for conflict in std::mem::take(&mut log.conflicts) {
        conflicts.push(conflict_from_history_conflict(conflict).map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, TextSpan::at(1, 1)))?);
    }
    let edits = ArtifactHistoryLedger::try_from_preflighted(edits).map_err(|_| TextError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history edit capacity exceeded".to_string(), TextSpan::at(1, 1)))?;
    let mut transitions: Vec<crate::os_spr::MutationEnvelope> = log.transitions.iter().map(|record| record.to_envelope(&log.doc_id)).collect();
    transitions.sort_by(|left, right| (left.timestamp.cmp_key(), left.mutation_id.0.as_str()).cmp(&(right.timestamp.cmp_key(), right.mutation_id.0.as_str())));
    let mut envelope = ArtifactEnvelope::from_owners(ArtifactEnvelopeOwners {
        schema: log.schema,
        id: log.doc_id,
        vcs: ArtifactVcs { genesis: AdmittedArtifactGenesis::from_decoded_pack(initial_snapshot, pack.to_vec()), edits, changes: ArtifactHistoryLedger::new(), checkpoints: ArtifactHistoryLedger::new(), alternatives: ArtifactHistoryLedger::new() },
        backbone: None,
        active_alternative_id: None,
        cursor: None,
        dialect: None,
        migrated_from: None,
        owner: None,
        lanes,
        edit_messages: ArtifactEditMessageLedger::from_preflighted_entries(edit_messages),
        conflicts,
        transitions,
        history_shape: crate::os_spr::HistoryShape::Document,
        open_transaction: None,
        viewer_checkpoint_id: None,
    });
    let trunk = crate::os_spr::trunk_alternative_id(&ArtifactId(envelope.id.clone()));
    envelope.active_alternative_id = log.viewer_line.clone().filter(|line| line != &trunk);
    envelope.viewer_checkpoint_id = log.viewer_checkpoint.clone();
    let composed = match &log.composition {
        Some(composition) => apply_history_composition(&mut envelope, composition).await.map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), TextSpan::at(1, 1))),
        None => Ok(()),
    };
    match composed {
        Ok(()) => settle_parsed_envelope(envelope).await,
        Err(error) => {
            discard_parsed_envelope(envelope);
            Err(error)
        }
    }
}

/// 📤️ Pack counterpart of `print_document_text`: identical op-log TEXT body (`print_ops_log`)
/// for the human-readable mirror, but the initial snapshot is encoded to pack bytes
/// (`ArtifactPack::encode_pack`) instead of printed to DSL text — plus the AUTHORITATIVE `.spr`
/// binary op log (`print_document_spr`), which carries real inverse/binary payloads/cursor.
pub async fn print_document_pack<P, Mutation>(envelope: &ArtifactEnvelopeOwners<P, Mutation>) -> Result<ArtifactPackFiles, VcsError>
where
    P: ArtifactPack,
    Mutation: OpText + OpBinary,
{
    let pack = envelope.vcs.genesis.pack().to_vec();
    let spr = print_document_spr(envelope).await?;
    let ops = print_ops_log(envelope).await?;
    Ok(ArtifactPackFiles { pack, spr, ops })
}

/// 📥️ Parses the explicit `.ops` records against an already-obtained initial snapshot.
/// Text persistence is intentionally strict: it requires inverse operations, one authoritative
/// metadata record per forward operation, durable messages/conflicts, and exactly one cursor.
pub(crate) async fn replay_ops<P, Mutation>(initial_snapshot: P, ops: &str) -> Result<ParsedDocumentText<P, Mutation>, TextError>
where
    P: Clone + ArtifactGenesisCodec + 'static,
    Mutation: OpText + OpBinary + self::Mutation<P> + 'static,
{
    let mut schema = String::new();
    let mut id = String::new();
    let mut edits: Vec<Edit<Mutation>> = Vec::new();
    let mut transitions: Vec<crate::os_spr::MutationEnvelope> = Vec::new();
    let mut inverse_by_edit: HashMap<String, semio_framework_value::list::PagedList<Mutation, {usize::MAX}>> = HashMap::new();
    let mut metadata_by_edit: HashMap<String, HashMap<u32, MutationMeta>> = HashMap::new();
    let mut messages_by_edit: HashMap<String, Vec<crate::os_spr::MutationMessage>> = HashMap::new();
    let mut message_order = Vec::new();
    let mut conflicts = Vec::new();
    let mut saw_doc = false;

    struct PendingEdit {
        id: String,
        sequence_number: i32,
        actor: Option<semio_framework_value::SharedUtf8>,
        started_at: String,
        finished_at: Option<String>,
        verb: Option<String>,
        line: Option<String>,
    }
    let mut pending_edit: Option<PendingEdit> = None;
    let mut pending_forwards: Vec<Mutation> = Vec::new();

    let flush_pending_edit = |pending_edit: &mut Option<PendingEdit>, pending_forwards: &mut Vec<Mutation>, edits: &mut Vec<Edit<Mutation>>| -> Result<(), TextError> {
        let Some(header) = pending_edit.take() else {
            return Ok(());
        };
        let forwards = std::mem::take(pending_forwards);
        edits.push(Edit {
            id: header.id,
            actor: header.actor,
            forwards,
            inverse: Vec::new().into(),
            mutation_meta: Vec::new(),
            verb: header.verb,
            line: header.line,
            sequence_number: header.sequence_number,
            started_at: header.started_at,
            finished_at: header.finished_at,
        });
        Ok(())
    };

    for (index, raw_line) in ops.lines().enumerate() {
        let line_no = index as u32 + 1;
        let trimmed = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if raw_line.starts_with("  ") && pending_edit.is_some() {
            let operation = Mutation::parse_op(trimmed).map_err(|error| TextError { span: TextSpan::at(line_no, error.span.column), ..error })?;
            pending_forwards.push(operation);
            continue;
        }
        flush_pending_edit(&mut pending_edit, &mut pending_forwards, &mut edits)?;
        let line = OpsHeaderLine::parse_op(trimmed).map_err(|error| TextError { span: TextSpan::at(line_no, error.span.column), ..error })?;
        match line {
            OpsHeaderLine::Doc { id: doc_id, schema: doc_schema } => {
                if saw_doc {
                    return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "ops text repeats its document header".to_string(), TextSpan::at(line_no, 1)));
                }
                saw_doc = true;
                schema = doc_schema;
                id = doc_id;
            }
            OpsHeaderLine::Edit { id: edit_id, sequence, started, actor, finished, verb, line } => {
                if edits.iter().any(|edit| edit.id == edit_id) || pending_edit.as_ref().is_some_and(|edit| edit.id == edit_id) {
                    return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("ops text repeats edit {edit_id}"), TextSpan::at(line_no, 1)));
                }
                pending_edit = Some(PendingEdit { id: edit_id, sequence_number: sequence, actor, started_at: started, finished_at: finished, verb, line: ops_line_provenance(line)? });
                pending_forwards = Vec::new();
            }
            OpsHeaderLine::Revert { id: transition_id, actor, clock, after, operations } => {
                transitions.push(
                    transition_from_ops_line(&id, transition_id, actor, &clock, after, crate::os_spr::HistoryTransition::Revert { mutation_ids: mutation_ids(operations) })
                        .map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, TextSpan::at(line_no, 1)))?,
                );
            }
            OpsHeaderLine::Reinstate { id: transition_id, actor, clock, after, operations } => {
                transitions.push(
                    transition_from_ops_line(&id, transition_id, actor, &clock, after, crate::os_spr::HistoryTransition::Reinstate { mutation_ids: mutation_ids(operations) })
                        .map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, TextSpan::at(line_no, 1)))?,
                );
            }
            OpsHeaderLine::Commit { id: transition_id, actor, clock, after, checkpoint, parent, change, operations, description, saved, by, message, at, line } => {
                let authors = by.into_iter().map(|author| crate::os_spr::TransitionAuthor { id: author.id, name: author.name, avatar: author.avatar }).collect();
                let checkpoint = crate::os_spr::TransitionCheckpoint {
                    checkpoint_id: checkpoint,
                    parent_id: parent,
                    change_id: change,
                    mutation_ids: mutation_ids(operations),
                    description,
                    saved_at: saved,
                    authors,
                    message,
                    timestamp: at,
                    line_id: ops_line_provenance(line)?,
                };
                transitions.push(
                    transition_from_ops_line(&id, transition_id, actor, &clock, after, crate::os_spr::HistoryTransition::Commit(checkpoint))
                        .map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, TextSpan::at(line_no, 1)))?,
                );
            }
            OpsHeaderLine::Branch { id: transition_id, actor, clock, after, alternative, name, checkpoint } => {
                transitions.push(
                    transition_from_ops_line(&id, transition_id, actor, &clock, after, crate::os_spr::HistoryTransition::Branch { alternative_id: alternative, name, checkpoint_id: checkpoint })
                        .map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, TextSpan::at(line_no, 1)))?,
                );
            }
            OpsHeaderLine::Checkout { id: transition_id, actor, clock, after, checkpoint, alternative } => {
                transitions.push(
                    transition_from_ops_line(&id, transition_id, actor, &clock, after, crate::os_spr::HistoryTransition::Checkout { checkpoint_id: checkpoint, alternative_id: alternative })
                        .map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, TextSpan::at(line_no, 1)))?,
                );
            }
            OpsHeaderLine::Repin { id: transition_id, actor, clock, after, checkpoint, pinned, pins } => {
                let pins = pins.into_iter().map(|pin| crate::os_spr::TransitionPin { child_uri: pin.child, checkpoint_id: pin.checkpoint }).collect();
                transitions.push(
                    transition_from_ops_line(&id, transition_id, actor, &clock, after, crate::os_spr::HistoryTransition::Repin { checkpoint_id: checkpoint, pinned_checkpoint_id: pinned, pins })
                        .map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, TextSpan::at(line_no, 1)))?,
                );
            }
            OpsHeaderLine::Supersede { id: transition_id, actor, clock, after, scope, inputs, observed } => {
                let inputs = inputs.into_iter().map(crate::os_spr::SupersededInput::try_from).collect::<Result<Vec<_>, _>>().map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, TextSpan::at(line_no, 1)))?;
                let supersede = crate::os_spr::TransitionSupersede { scope, inputs };
                supersede.validate().map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), TextSpan::at(line_no, 1)))?;
                let mut envelope = transition_from_ops_line(&id, transition_id, actor, &clock, after, crate::os_spr::HistoryTransition::Supersede(supersede))
                    .map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, TextSpan::at(line_no, 1)))?;
                envelope.observed = observed.map(MutationId);
                transitions.push(envelope);
            }
            OpsHeaderLine::Inverse { edit, ops } => {
                let mut inverse = Vec::with_capacity(ops.len());
                for operation in &ops {
                    inverse.push(Mutation::parse_op(operation).map_err(|error| TextError { span: TextSpan::at(line_no, error.span.column), ..error })?);
                }
                if inverse_by_edit.insert(edit.clone(), inverse.into()).is_some() {
                    return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("ops text repeats inverse record for edit {edit}"), TextSpan::at(line_no, 1)));
                }
            }
            OpsHeaderLine::Metadata { edit, index, data } => {
                let metadata = semio_framework_pack_json::from_json_str(&data, semio_framework_pack_json::JsonMemberPolicy::Reject)
                    .map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid metadata record for edit {edit}: {error}"), TextSpan::at(line_no, 1)))?;
                if metadata_by_edit.entry(edit.clone()).or_default().insert(index, metadata).is_some() {
                    return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("ops text repeats metadata index {index} for edit {edit}"), TextSpan::at(line_no, 1)));
                }
            }
            OpsHeaderLine::Message { edit, data } => {
                let message = semio_framework_pack_json::from_json_str(&data, semio_framework_pack_json::JsonMemberPolicy::Reject)
                    .map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid message record for edit {edit}: {error}"), TextSpan::at(line_no, 1)))?;
                if !messages_by_edit.contains_key(&edit) {
                    message_order.push(edit.clone());
                }
                messages_by_edit.entry(edit).or_default().push(message);
            }
            OpsHeaderLine::Conflict { data } => {
                let conflict = semio_framework_pack_json::from_json_str(&data, semio_framework_pack_json::JsonMemberPolicy::Reject)
                    .map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid conflict record: {error}"), TextSpan::at(line_no, 1)))?;
                conflicts.push(conflict);
            }
        }
    }
    flush_pending_edit(&mut pending_edit, &mut pending_forwards, &mut edits)?;
    if !saw_doc {
        return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "ops text has no document header".to_string(), TextSpan::at(1, 1)));
    }
    for edit in &mut edits {
        edit.inverse = inverse_by_edit.remove(&edit.id).ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("ops text has no inverse record for edit {}", edit.id), TextSpan::at(1, 1)))?;
        let metadata = metadata_by_edit.remove(&edit.id).ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("ops text has no metadata records for edit {}", edit.id), TextSpan::at(1, 1)))?;
        if metadata.len() != edit.forwards.len() || (0..edit.forwards.len() as u32).any(|index| !metadata.contains_key(&index)) {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("ops text metadata does not exactly cover edit {}", edit.id), TextSpan::at(1, 1)));
        }
        edit.mutation_meta = (0..edit.forwards.len() as u32)
            .map(|index| metadata.get(&index).cloned().ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("ops text metadata is missing index {index} for edit {}", edit.id), TextSpan::at(1, 1))))
            .collect::<Result<Vec<_>, _>>()?;
        stamp_edit_semantics::<P, Mutation>(edit, &schema);
    }
    if let Some(edit) = inverse_by_edit.keys().next() {
        return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("ops text has an inverse for unknown edit {edit}"), TextSpan::at(1, 1)));
    }
    if let Some(edit) = metadata_by_edit.keys().next() {
        return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("ops text has metadata for unknown edit {edit}"), TextSpan::at(1, 1)));
    }
    let mut edit_messages = Vec::new();
    for edit_id in message_order {
        let messages = messages_by_edit.remove(&edit_id).ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, format!("ops text lost message ownership for edit {edit_id}"), TextSpan::at(1, 1)))?;
        let edit = edits.iter().find(|edit| edit.id == edit_id).ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("ops text has messages for unknown edit {edit_id}"), TextSpan::at(1, 1)))?;
        for message in &messages {
            validate_persisted_message(message, Some(edit.forwards.len())).await.map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), TextSpan::at(1, 1)))?;
        }
        edit_messages.push(crate::os_spr::EditMessages { edit_id, messages });
    }
    let edits = ArtifactHistoryLedger::try_from_preflighted(edits).map_err(|_| TextError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, "ops edit capacity exceeded".to_string(), TextSpan::at(1, 1)))?;
    transitions.sort_by(|left, right| (left.timestamp.cmp_key(), left.mutation_id.0.as_str()).cmp(&(right.timestamp.cmp_key(), right.mutation_id.0.as_str())));
    let envelope = ArtifactEnvelope::from_owners(ArtifactEnvelopeOwners {
        schema,
        id,
        vcs: ArtifactVcs { genesis: AdmittedArtifactGenesis::born(initial_snapshot).map_err(|error| TextError::new(error.kind, error.message, TextSpan::at(1, 1)))?, edits, changes: ArtifactHistoryLedger::new(), checkpoints: ArtifactHistoryLedger::new(), alternatives: ArtifactHistoryLedger::new() },
        backbone: None,
        active_alternative_id: None,
        cursor: None,
        dialect: None,
        migrated_from: None,
        owner: None,
        lanes: protocol::HistoryFoldIndex::new(),
        edit_messages: ArtifactEditMessageLedger::from_preflighted_entries(edit_messages),
        conflicts,
        transitions,
        history_shape: crate::os_spr::HistoryShape::Document,
        open_transaction: None,
        viewer_checkpoint_id: None,
    });
    settle_parsed_envelope(envelope).await
}

/// 🧮️ Finishes a load: folds the parsed event log into every derived position and fact,
/// validates the result and materializes the live snapshot by replaying the applied edits over the
/// genesis. A refused document is discarded through [`discard_parsed_envelope`], never dropped.
pub(crate) async fn settle_parsed_envelope<P, Mutation>(mut envelope: ArtifactEnvelope<P, Mutation>) -> Result<ParsedDocumentText<P, Mutation>, TextError>
where
    P: Clone + 'static,
    Mutation: self::Mutation<P> + OpBinary + 'static,
{
    let settled = match project_envelope_history(&mut envelope) {
        Ok(supersessions) => validate_durable_history(&envelope).await.map(|()| supersessions),
        Err(error) => Err(error),
    };
    let snapshot = match settled {
        Ok(supersessions) => {
            let applied_edit_ids = envelope.cursor.as_ref().map(|cursor| cursor.applied_edit_ids.clone()).unwrap_or_default();
            replay_loaded_history(&mut envelope, &applied_edit_ids, supersessions)
        }
        Err(error) => Err(error),
    };
    match snapshot {
        Ok(snapshot) => Ok(ParsedDocumentText { envelope, snapshot }),
        Err(error) => {
            discard_parsed_envelope(envelope);
            Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), TextSpan::at(1, 1)))
        }
    }
}

/// 🧹️ Discards a refused, never-published parsed envelope: its shell is detached and its
/// genesis retired through the technology's own diff vocabulary; the plain history data drops.
pub(crate) fn discard_parsed_envelope<P, Mutation>(envelope: ArtifactEnvelope<P, Mutation>)
where
    Mutation: self::Mutation<P>,
{
    let mut owners = envelope.into_owners();
    while owners.vcs.edits.pop().is_some() {}
    while owners.vcs.changes.pop().is_some() {}
    while owners.vcs.checkpoints.pop().is_some() {}
    while owners.vcs.alternatives.pop().is_some() {}
    while owners.edit_messages.pop().is_some() {}
    let (snapshot, pack) = owners.vcs.genesis.into_owners();
    retire_shared_projection::<P, Mutation>(snapshot);
    drop(pack);
}

/// 📥️ Parses the textual VCS document back into an envelope plus its live (fully-replayed)
/// snapshot — obtains the initial snapshot via `P::parse_dsl` then shares `replay_ops`.
pub async fn parse_document_text<P, Mutation>(dsl: &str, ops: &str) -> Result<ParsedDocumentText<P, Mutation>, TextError>
where
    P: Clone + ArtifactDsl + ArtifactGenesisCodec + 'static,
    Mutation: OpText + OpBinary + self::Mutation<P> + 'static,
{
    let initial_snapshot = P::parse_dsl(dsl)?;
    replay_ops(initial_snapshot, ops).await
}

/// 📥️ spr-first pack counterpart of `parse_document_text`: pack+spr are the AUTHORITATIVE
/// pair (see `ArtifactPackFiles`'s doc) — this is a thin forward onto `parse_document_spr`, which
/// recovers real `inverse`/`mutation_meta`/`cursor` instead of recomputing them via replay.
pub async fn parse_document_pack<P, Mutation>(pack: &[u8], spr: &[u8]) -> Result<ParsedDocumentText<P, Mutation>, TextError>
where
    P: Clone + ArtifactPack + 'static,
    Mutation: OpText + OpBinary + self::Mutation<P> + 'static,
{
    parse_document_spr(pack, spr).await
}
