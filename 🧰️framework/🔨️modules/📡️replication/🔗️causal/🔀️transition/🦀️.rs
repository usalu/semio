//! 🔀️ History transitions: the structural half of an artifact's semantic event log. An edit's
//! operations travel as ordinary [`super::MutationEnvelope`]s whose `diff` carries the domain op;
//! every other history step (undo, redo, checkpoint commit, alternative branch, checkout, pin)
//! travels as a [`super::MutationEnvelope`] too, tagged with [`HISTORY_TRANSITION_SCHEMA`] and
//! carrying one encoded [`HistoryTransition`]. One causal, HLC-ordered stream therefore holds every
//! semantic change of a document; a replica materializes the document by folding that stream over
//! its genesis snapshot — never by merging another replica's snapshot.
//!
//! Identity across replicas is the per-operation [`crate::ids::MutationId`]: a receiver
//! materializes one local edit per wire operation, so transitions reference operations, never a
//! replica-local edit id. Checkpoint, change and alternative ids are minted once by the author
//! and carried verbatim.

use crate::ids::{ActorId, ArtifactId, HybridLogicalTimestamp, MutationId, SchemaId};

//#region 🔖️Vocabulary
/// @emoji 🏷️ `diff.schema` of every transition envelope. Operation envelopes carry their
/// artifact's own schema, so the tag alone routes an envelope to the transition fold.
pub const HISTORY_TRANSITION_SCHEMA: &str = "semio.history.transition";

/// @emoji 🧑‍🎨️ Author stamped on a committed checkpoint.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransitionAuthor {
    pub id: String,
    pub name: String,
    pub avatar: Option<String>,
}

/// @emoji 🚩️ The facts one checkpoint commit introduces: the change grouping the operations that
/// were uncommitted at the author, and the checkpoint stacking that change onto `parent_id`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransitionCheckpoint {
    pub checkpoint_id: String,
    pub parent_id: Option<String>,
    pub change_id: String,
    pub mutation_ids: Vec<MutationId>,
    pub description: Option<String>,
    pub saved_at: String,
    pub authors: Vec<TransitionAuthor>,
    pub message: Option<String>,
    pub timestamp: String,
}

/// @emoji 📌️ One owned child's checkpoint pin, as `(child artifact uri, child checkpoint id)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransitionPin {
    pub child_uri: String,
    pub checkpoint_id: String,
}

/// @emoji 🔀️ One structural history step. Every variant is a pure function of the fold state it
/// lands on, so replicas holding the same event set converge regardless of arrival order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HistoryTransition {
    /// ⏪️ The author withdraws its own operations (undo).
    Revert { mutation_ids: Vec<MutationId> },
    /// ⏩️ The author restores operations it reverted earlier (redo).
    Reinstate { mutation_ids: Vec<MutationId> },
    /// 🚩️ A checkpoint commit.
    Commit(TransitionCheckpoint),
    /// 🌿️ A new alternative rooted at `checkpoint_id`, which becomes the active alternative.
    Branch { alternative_id: String, name: String, checkpoint_id: String },
    /// 🎯️ Moves the document to `checkpoint_id`, activating `alternative_id` (if any).
    Checkout { checkpoint_id: String, alternative_id: Option<String> },
    /// 🧩️ Re-identifies `checkpoint_id` as `pinned_checkpoint_id` once its composed children's pins are known.
    Repin { checkpoint_id: String, pinned_checkpoint_id: String, pins: Vec<TransitionPin> },
}
//#endregion 🔖️Vocabulary

//#region 🔖️Codec
fn write_optional_str(out: &mut Vec<u8>, value: &Option<String>) {
    match value {
        Some(value) => {
            out.push(1);
            crate::write_str(out, value);
        }
        None => out.push(0),
    }
}

fn read_optional_str(bytes: &[u8], pos: &mut usize) -> Result<Option<String>, crate::ProtocolError> {
    match read_u8(bytes, pos)? {
        0 => Ok(None),
        1 => Ok(Some(crate::read_str(bytes, pos)?)),
        other => Err(malformed(*pos, format!("invalid option tag {other}"))),
    }
}

fn read_u8(bytes: &[u8], pos: &mut usize) -> Result<u8, crate::ProtocolError> {
    let value = *bytes.get(*pos).ok_or_else(|| malformed(*pos, "truncated"))?;
    *pos += 1;
    Ok(value)
}

fn malformed(offset: usize, detail: impl Into<String>) -> crate::ProtocolError {
    crate::ProtocolError::Malformed { what: "history transition", offset: offset as u64, detail: detail.into() }
}

fn write_ids(out: &mut Vec<u8>, ids: &[MutationId]) {
    crate::wire::write_varint_u64(out, ids.len() as u64);
    for id in ids {
        crate::write_str(out, &id.0);
    }
}

fn read_ids(bytes: &[u8], pos: &mut usize) -> Result<Vec<MutationId>, crate::ProtocolError> {
    let count = crate::wire::read_varint_u64(bytes, pos)?;
    if count > (bytes.len() - (*pos).min(bytes.len())) as u64 {
        return Err(malformed(*pos, "id count exceeds payload"));
    }
    let mut ids = Vec::with_capacity(count as usize);
    for _ in 0..count {
        ids.push(MutationId(crate::read_str(bytes, pos)?));
    }
    Ok(ids)
}

/// @emoji 🎯️ `tag varint | variant fields in declaration order` — the transition payload bytes.
pub fn encode_history_transition(transition: &HistoryTransition) -> Vec<u8> {
    let mut out = Vec::new();
    match transition {
        HistoryTransition::Revert { mutation_ids } => {
            crate::wire::write_varint_u64(&mut out, 0);
            write_ids(&mut out, mutation_ids);
        }
        HistoryTransition::Reinstate { mutation_ids } => {
            crate::wire::write_varint_u64(&mut out, 1);
            write_ids(&mut out, mutation_ids);
        }
        HistoryTransition::Commit(checkpoint) => {
            crate::wire::write_varint_u64(&mut out, 2);
            crate::write_str(&mut out, &checkpoint.checkpoint_id);
            write_optional_str(&mut out, &checkpoint.parent_id);
            crate::write_str(&mut out, &checkpoint.change_id);
            write_ids(&mut out, &checkpoint.mutation_ids);
            write_optional_str(&mut out, &checkpoint.description);
            crate::write_str(&mut out, &checkpoint.saved_at);
            crate::wire::write_varint_u64(&mut out, checkpoint.authors.len() as u64);
            for author in &checkpoint.authors {
                crate::write_str(&mut out, &author.id);
                crate::write_str(&mut out, &author.name);
                write_optional_str(&mut out, &author.avatar);
            }
            write_optional_str(&mut out, &checkpoint.message);
            crate::write_str(&mut out, &checkpoint.timestamp);
        }
        HistoryTransition::Branch { alternative_id, name, checkpoint_id } => {
            crate::wire::write_varint_u64(&mut out, 3);
            crate::write_str(&mut out, alternative_id);
            crate::write_str(&mut out, name);
            crate::write_str(&mut out, checkpoint_id);
        }
        HistoryTransition::Checkout { checkpoint_id, alternative_id } => {
            crate::wire::write_varint_u64(&mut out, 4);
            crate::write_str(&mut out, checkpoint_id);
            write_optional_str(&mut out, alternative_id);
        }
        HistoryTransition::Repin { checkpoint_id, pinned_checkpoint_id, pins } => {
            crate::wire::write_varint_u64(&mut out, 5);
            crate::write_str(&mut out, checkpoint_id);
            crate::write_str(&mut out, pinned_checkpoint_id);
            crate::wire::write_varint_u64(&mut out, pins.len() as u64);
            for pin in pins {
                crate::write_str(&mut out, &pin.child_uri);
                crate::write_str(&mut out, &pin.checkpoint_id);
            }
        }
    }
    out
}

/// @emoji 🎯️ Inverse of [`encode_history_transition`]; refuses trailing bytes.
pub fn decode_history_transition(bytes: &[u8]) -> Result<HistoryTransition, crate::ProtocolError> {
    let mut pos = 0usize;
    let transition = match crate::wire::read_varint_u64(bytes, &mut pos)? {
        0 => HistoryTransition::Revert { mutation_ids: read_ids(bytes, &mut pos)? },
        1 => HistoryTransition::Reinstate { mutation_ids: read_ids(bytes, &mut pos)? },
        2 => {
            let checkpoint_id = crate::read_str(bytes, &mut pos)?;
            let parent_id = read_optional_str(bytes, &mut pos)?;
            let change_id = crate::read_str(bytes, &mut pos)?;
            let mutation_ids = read_ids(bytes, &mut pos)?;
            let description = read_optional_str(bytes, &mut pos)?;
            let saved_at = crate::read_str(bytes, &mut pos)?;
            let author_count = crate::wire::read_varint_u64(bytes, &mut pos)?;
            if author_count > (bytes.len() - pos.min(bytes.len())) as u64 {
                return Err(malformed(pos, "author count exceeds payload"));
            }
            let mut authors = Vec::with_capacity(author_count as usize);
            for _ in 0..author_count {
                authors.push(TransitionAuthor { id: crate::read_str(bytes, &mut pos)?, name: crate::read_str(bytes, &mut pos)?, avatar: read_optional_str(bytes, &mut pos)? });
            }
            let message = read_optional_str(bytes, &mut pos)?;
            let timestamp = crate::read_str(bytes, &mut pos)?;
            HistoryTransition::Commit(TransitionCheckpoint { checkpoint_id, parent_id, change_id, mutation_ids, description, saved_at, authors, message, timestamp })
        }
        3 => HistoryTransition::Branch { alternative_id: crate::read_str(bytes, &mut pos)?, name: crate::read_str(bytes, &mut pos)?, checkpoint_id: crate::read_str(bytes, &mut pos)? },
        4 => HistoryTransition::Checkout { checkpoint_id: crate::read_str(bytes, &mut pos)?, alternative_id: read_optional_str(bytes, &mut pos)? },
        5 => {
            let checkpoint_id = crate::read_str(bytes, &mut pos)?;
            let pinned_checkpoint_id = crate::read_str(bytes, &mut pos)?;
            let pin_count = crate::wire::read_varint_u64(bytes, &mut pos)?;
            if pin_count > (bytes.len() - pos.min(bytes.len())) as u64 {
                return Err(malformed(pos, "pin count exceeds payload"));
            }
            let mut pins = Vec::with_capacity(pin_count as usize);
            for _ in 0..pin_count {
                pins.push(TransitionPin { child_uri: crate::read_str(bytes, &mut pos)?, checkpoint_id: crate::read_str(bytes, &mut pos)? });
            }
            HistoryTransition::Repin { checkpoint_id, pinned_checkpoint_id, pins }
        }
        other => return Err(malformed(0, format!("unknown transition tag {other}"))),
    };
    if pos != bytes.len() {
        return Err(malformed(pos, "trailing bytes"));
    }
    Ok(transition)
}
//#endregion 🔖️Codec

//#region 🔖️Envelope
/// @emoji 🪪️ Content-addressed transition id: `transition-{hex16(blake3(actor | hlc | payload))}`.
pub fn history_transition_id(actor: &ActorId, timestamp: &HybridLogicalTimestamp, payload: &[u8]) -> MutationId {
    let mut material = Vec::with_capacity(payload.len() + actor.0.len() + 32);
    crate::write_str(&mut material, &actor.0);
    crate::wire::write_varint_u64(&mut material, timestamp.actor);
    crate::wire::write_varint_u64(&mut material, timestamp.physical_ms);
    crate::wire::write_varint_u64(&mut material, timestamp.logical);
    crate::write_bytes(&mut material, payload);
    let digest = crate::wire::RecordHasher::hash(&crate::format::Blake3Hasher, &material);
    let mut id = String::with_capacity("transition-".len() + 16);
    id.push_str("transition-");
    for byte in &digest[..8] {
        id.push_str(&format!("{byte:02x}"));
    }
    MutationId(id)
}

/// @emoji ✉️ Wraps `transition` as a causal envelope: schema-tagged payload, empty inverse (a
/// transition is undone by a later transition, never by an inverse payload).
pub fn history_transition_envelope(transition: &HistoryTransition, document_id: &ArtifactId, actor: &ActorId, dependencies: Vec<MutationId>, timestamp: HybridLogicalTimestamp) -> super::MutationEnvelope {
    let payload = encode_history_transition(transition);
    let mutation_id = history_transition_id(actor, &timestamp, &payload);
    let schema = SchemaId(HISTORY_TRANSITION_SCHEMA.to_string());
    super::MutationEnvelope {
        mutation_id,
        document_id: document_id.clone(),
        actor: actor.clone(),
        dependencies,
        diff: super::ArtifactDiff { schema: schema.clone(), payload },
        inverse: super::InverseMutation { schema, payload: Vec::new() },
        timestamp,
    }
}

/// @emoji 🔎️ Whether `envelope` is a history transition rather than a domain operation.
pub fn is_history_transition(envelope: &super::MutationEnvelope) -> bool {
    envelope.diff.schema.0 == HISTORY_TRANSITION_SCHEMA
}

/// @emoji 📤️ Decodes `envelope`'s transition, or `None` for a domain-operation envelope.
pub fn history_transition_from_envelope(envelope: &super::MutationEnvelope) -> Result<Option<HistoryTransition>, crate::ProtocolError> {
    if !is_history_transition(envelope) {
        return Ok(None);
    }
    decode_history_transition(&envelope.diff.payload).map(Some)
}
//#endregion 🔖️Envelope

//#region 🔖️Fold
/// @emoji ✏️ One edit as the fold sees it: its replica-local id, author, HLC and the wire
/// operations it owns.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoldEdit {
    pub id: String,
    pub actor: Option<String>,
    pub timestamp: HybridLogicalTimestamp,
    pub mutation_ids: Vec<MutationId>,
}

/// @emoji 📦️ A change fact the fold materialized from a [`HistoryTransition::Commit`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoldChange {
    pub id: String,
    pub edit_ids: Vec<String>,
    pub description: Option<String>,
    pub saved_at: String,
}

/// @emoji 🚩️ A checkpoint fact the fold materialized, with its full change chain and pins.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoldCheckpoint {
    pub id: String,
    pub change_ids: Vec<String>,
    pub parent_id: Option<String>,
    pub authors: Vec<TransitionAuthor>,
    pub message: Option<String>,
    pub timestamp: String,
    pub pins: Vec<TransitionPin>,
}

/// @emoji 🌿️ An alternative fact with the checkpoint chain the fold grew it to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoldAlternative {
    pub id: String,
    pub name: String,
    pub checkpoint_ids: Vec<String>,
}

/// @emoji 🧮️ Everything a document's history projects to: the active edits in HLC order, the redo
/// stack, the current checkpoint and alternative, and every change/checkpoint/alternative fact.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HistoryFold {
    pub applied: Vec<String>,
    pub redo: Vec<String>,
    pub checkpoint: Option<String>,
    pub alternative: Option<String>,
    pub changes: Vec<FoldChange>,
    pub checkpoints: Vec<FoldCheckpoint>,
    pub alternatives: Vec<FoldAlternative>,
}

enum FoldEvent<'a> {
    Edit(&'a FoldEdit),
    Transition(HistoryTransition),
}

fn fold_error(detail: impl Into<String>) -> crate::ProtocolError {
    crate::ProtocolError::Malformed { what: "history fold", offset: 0, detail: detail.into() }
}

/// @emoji 🧮️ Folds `edits` and the transition envelopes in `transitions` in `(hlc, id)` order into
/// the document's history projection. A pure function of the event SET: replicas holding the same
/// events derive the same projection whatever order they received them in. `excluded` names edits
/// withheld by a merge policy (quarantined); they never become active.
pub fn fold_history(edits: &[FoldEdit], transitions: &[super::MutationEnvelope], excluded: &std::collections::HashSet<String>) -> Result<HistoryFold, crate::ProtocolError> {
    let mut owners: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
    let mut authors: std::collections::HashMap<&str, Option<&str>> = std::collections::HashMap::new();
    for edit in edits {
        authors.insert(edit.id.as_str(), edit.actor.as_deref());
        for mutation_id in &edit.mutation_ids {
            owners.insert(mutation_id.0.as_str(), edit.id.as_str());
        }
    }
    let mut events: Vec<((u64, u64, u64), &str, FoldEvent<'_>)> = Vec::with_capacity(edits.len() + transitions.len());
    for edit in edits {
        events.push((edit.timestamp.cmp_key(), edit.id.as_str(), FoldEvent::Edit(edit)));
    }
    let mut identities: std::collections::HashSet<&str> = edits.iter().map(|edit| edit.id.as_str()).collect();
    if identities.len() != edits.len() {
        return Err(fold_error("history repeats an edit"));
    }
    for envelope in transitions {
        if !identities.insert(envelope.mutation_id.0.as_str()) {
            return Err(fold_error(format!("history repeats transition {}", envelope.mutation_id.0)));
        }
        let transition = history_transition_from_envelope(envelope)?.ok_or_else(|| fold_error(format!("{} is not a history transition", envelope.mutation_id.0)))?;
        events.push((envelope.timestamp.cmp_key(), envelope.mutation_id.0.as_str(), FoldEvent::Transition(transition)));
    }
    events.sort_by(|left, right| (left.0, left.1).cmp(&(right.0, right.1)));
    let owned = |mutation_ids: &[MutationId]| -> Result<Vec<String>, crate::ProtocolError> {
        let mut edit_ids: Vec<String> = Vec::with_capacity(mutation_ids.len());
        for mutation_id in mutation_ids {
            let edit_id = owners.get(mutation_id.0.as_str()).ok_or_else(|| fold_error(format!("transition references unknown operation {}", mutation_id.0)))?;
            if !edit_ids.iter().any(|known| known == edit_id) {
                edit_ids.push((*edit_id).to_string());
            }
        }
        Ok(edit_ids)
    };
    let mut fold = HistoryFold::default();
    let mut active: std::collections::HashSet<String> = std::collections::HashSet::new();
    for (_, _, event) in events {
        match event {
            FoldEvent::Edit(edit) => {
                if excluded.contains(&edit.id) {
                    continue;
                }
                active.insert(edit.id.clone());
                fold.redo.retain(|redo| authors.get(redo.as_str()).copied().flatten() != edit.actor.as_deref());
            }
            FoldEvent::Transition(HistoryTransition::Revert { mutation_ids }) => {
                for edit_id in owned(&mutation_ids)? {
                    if active.remove(&edit_id) {
                        fold.redo.push(edit_id);
                    }
                }
            }
            FoldEvent::Transition(HistoryTransition::Reinstate { mutation_ids }) => {
                for edit_id in owned(&mutation_ids)? {
                    if let Some(position) = fold.redo.iter().position(|redo| *redo == edit_id) {
                        fold.redo.remove(position);
                        active.insert(edit_id);
                    }
                }
            }
            FoldEvent::Transition(HistoryTransition::Commit(checkpoint)) => {
                let mut change_ids = match &checkpoint.parent_id {
                    Some(parent_id) => fold.checkpoints.iter().find(|known| known.id == *parent_id).ok_or_else(|| fold_error(format!("checkpoint {} names unknown parent {parent_id}", checkpoint.checkpoint_id)))?.change_ids.clone(),
                    None => Vec::new(),
                };
                change_ids.push(checkpoint.change_id.clone());
                fold.changes.push(FoldChange { id: checkpoint.change_id, edit_ids: owned(&checkpoint.mutation_ids)?, description: checkpoint.description, saved_at: checkpoint.saved_at });
                fold.checkpoints.push(FoldCheckpoint { id: checkpoint.checkpoint_id.clone(), change_ids, parent_id: checkpoint.parent_id, authors: checkpoint.authors, message: checkpoint.message, timestamp: checkpoint.timestamp, pins: Vec::new() });
                if let Some(alternative_id) = &fold.alternative {
                    if let Some(alternative) = fold.alternatives.iter_mut().find(|alternative| alternative.id == *alternative_id) {
                        alternative.checkpoint_ids.push(checkpoint.checkpoint_id.clone());
                    }
                }
                fold.checkpoint = Some(checkpoint.checkpoint_id);
            }
            FoldEvent::Transition(HistoryTransition::Branch { alternative_id, name, checkpoint_id }) => {
                checkout(&mut fold, &mut active, &checkpoint_id, excluded)?;
                fold.alternatives.push(FoldAlternative { id: alternative_id.clone(), name, checkpoint_ids: vec![checkpoint_id] });
                fold.alternative = Some(alternative_id);
            }
            FoldEvent::Transition(HistoryTransition::Checkout { checkpoint_id, alternative_id }) => {
                checkout(&mut fold, &mut active, &checkpoint_id, excluded)?;
                fold.alternative = alternative_id;
            }
            FoldEvent::Transition(HistoryTransition::Repin { checkpoint_id, pinned_checkpoint_id, pins }) => {
                let checkpoint = fold.checkpoints.iter_mut().find(|known| known.id == checkpoint_id).ok_or_else(|| fold_error(format!("repin names unknown checkpoint {checkpoint_id}")))?;
                checkpoint.id = pinned_checkpoint_id.clone();
                checkpoint.pins = pins;
                for alternative in &mut fold.alternatives {
                    for id in &mut alternative.checkpoint_ids {
                        if *id == checkpoint_id {
                            *id = pinned_checkpoint_id.clone();
                        }
                    }
                }
                if fold.checkpoint.as_deref() == Some(checkpoint_id.as_str()) {
                    fold.checkpoint = Some(pinned_checkpoint_id);
                }
            }
        }
    }
    let mut ordered: Vec<&FoldEdit> = edits.iter().filter(|edit| active.contains(&edit.id)).collect();
    ordered.sort_by(|left, right| (left.timestamp.cmp_key(), left.id.as_str()).cmp(&(right.timestamp.cmp_key(), right.id.as_str())));
    fold.applied = ordered.into_iter().map(|edit| edit.id.clone()).collect();
    Ok(fold)
}

fn checkout(fold: &mut HistoryFold, active: &mut std::collections::HashSet<String>, checkpoint_id: &str, excluded: &std::collections::HashSet<String>) -> Result<(), crate::ProtocolError> {
    let checkpoint = fold.checkpoints.iter().find(|known| known.id == checkpoint_id).ok_or_else(|| fold_error(format!("checkout names unknown checkpoint {checkpoint_id}")))?;
    active.clear();
    for change_id in &checkpoint.change_ids {
        let change = fold.changes.iter().find(|change| change.id == *change_id).ok_or_else(|| fold_error(format!("checkpoint {checkpoint_id} names unknown change {change_id}")))?;
        active.extend(change.edit_ids.iter().filter(|edit_id| !excluded.contains(*edit_id)).cloned());
    }
    fold.redo.clear();
    fold.checkpoint = Some(checkpoint_id.to_string());
    Ok(())
}
//#endregion 🔖️Fold

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
