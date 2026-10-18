//! 📦️ Document spr, pack and history composition: the empty document spr, the identity stamp, the genesis member pack, the history events, the history composition, the spr print and parse and the pack print. See [`super`].

use super::*;

/// 🎯️ Encodes a bare, edit-free `.spr` op log for `schema` — the counterpart to a `.pack`
/// file carrying only an initial snapshot with no history yet (e.g. a single dropped `.pack`
/// file with no accompanying `.spr` sidecar). `doc_id` may be empty when the caller mints a fresh
/// id downstream (as `parse_document_spr` never cross-checks it against the pack). LAW:
/// `parse_document_spr(pack, &empty_document_spr(id, schema))` recovers exactly `P::decode_pack(pack)`
/// as both the initial and live snapshot, with zero edits.
pub async fn empty_document_spr(doc_id: &str, schema: &str) -> Vec<u8> {
    let log = semio_framework_os_spr::HistoryLog { doc_id: doc_id.to_string(), schema: schema.to_string(), ..semio_framework_os_spr::HistoryLog::default() };
    semio_framework_os_spr::encode_history(&log, &semio_framework_os_spr::EncodeOptions::default()).await.expect("encoding an edit-free HistoryLog is infallible")
}

/// 🪪️ Re-stamps an app-built document `.spr` with the identity of the live document it is
/// loaded into — id, schema, dialect, owner — keeping every history event.
/// A command replacing its whole document (`Effect::LoadDocument`) builds that log without knowing
/// where its store is mounted, and archive hydration rejects any log whose identity differs.
pub async fn stamp_document_spr_identity(spr: &[u8], doc_id: &str, schema: &str, dialect: &semio_framework_artifact_reference::ArtifactDialect, owner: Option<&OwnerRef>) -> Result<Vec<u8>, VcsError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    let mut log = semio_framework_os_spr::decode_history(spr, &semio_framework_os_spr::DecodeOptions::default()).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
    log.doc_id = doc_id.to_string();
    log.schema = schema.to_string();
    log.composition =
        Some(semio_framework_os_spr::HistoryComposition { owner: owner.map(|owner| (owner.parent.to_uri(), owner.slot.clone(), owner.child_id.clone())), dialect: Some((dialect.artifact_kind.clone(), dialect.standard.clone(), dialect.subset.clone())) });
    let options = semio_framework_os_spr::EncodeOptions { write_backwards_section: true, ..semio_framework_os_spr::EncodeOptions::default() };
    semio_framework_os_spr::encode_history(&log, &options).await.map_err(|error| VcsError::Serialize(error.to_string()))
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
pub async fn append_history_events_to_spr(spr: &[u8], edits: &[semio_framework_os_spr::HistoryEdit], transitions: &[semio_framework_os_spr::HistoryTransitionRecord]) -> Result<Vec<u8>, VcsError> {
    let mut log = semio_framework_os_spr::decode_history(spr, &semio_framework_os_spr::DecodeOptions::default()).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
    log.edits.extend(edits.iter().cloned());
    log.transitions.extend(transitions.iter().cloned());
    let options = semio_framework_os_spr::EncodeOptions { write_backwards_section: true, ..semio_framework_os_spr::EncodeOptions::default() };
    semio_framework_os_spr::encode_history(&log, &options).await.map_err(|error| VcsError::Serialize(error.to_string()))
}

/// 🧩️ Projects an envelope's composition facts (`owner`, `dialect`) into the durable
/// `HistoryComposition` overlay, or `None` when the document
/// has none — which is the overwhelming majority, so an ordinary leaf document's `.spr` gains not a
/// single byte from this record existing. `ArtifactRef`s cross the boundary as their `to_uri()`
/// wire string, the same "own the codec at this edge" convention `CompositionPin`/`ArtifactChild`
/// already use rather than coupling `ArtifactRef` into the protocol crate.
pub async fn history_composition_from_envelope<P, Mutation>(envelope: &ArtifactEnvelopeOwners<P, Mutation>) -> Option<semio_framework_os_spr::HistoryComposition> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    let owner = envelope.owner.as_ref().map(|owner| (owner.parent.to_uri(), owner.slot.clone(), owner.child_id.clone()));
    let dialect = envelope.dialect.as_ref().map(|dialect| (dialect.artifact_kind.clone(), dialect.standard.clone(), dialect.subset.clone()));
    if owner.is_none() && dialect.is_none() {
        return None;
    }
    Some(semio_framework_os_spr::HistoryComposition { owner, dialect })
}

/// 🧩️ Inverse of `history_composition_from_envelope`: malformed ownership rejects the
/// authoritative history rather than being silently discarded. Checkpoint pins are not part of the
/// overlay — they are facts of `Repin` transitions, folded like every other history fact.
pub async fn apply_history_composition<P, Mutation>(envelope: &mut ArtifactEnvelope<P, Mutation>, composition: &semio_framework_os_spr::HistoryComposition) -> Result<(), VcsError> {
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
    let mut edits = semio_framework_os_spr::history::HistoryEdits::empty();
    for edit in envelope.vcs.edits.iter().filter(|edit| envelope.holds_committed_edit(&edit.id)) {
        edits.push(history_edit_from_edit::<Mutation>(edit, message_ledger.get(edit.id.as_str()).copied().unwrap_or(&[]), envelope.lanes.get(&edit.id).copied()).await?);
    }
    let mut conflicts = Vec::with_capacity(envelope.conflicts.len());
    for conflict in &envelope.conflicts {
        conflicts.push(history_conflict_from_conflict(conflict).await);
    }
    let log = semio_framework_os_spr::HistoryLog {
        doc_id: envelope.id.clone(),
        schema: envelope.schema.clone(),
        edits,
        transitions: envelope.transitions.iter().map(semio_framework_os_spr::HistoryTransitionRecord::from_envelope).collect(),
        composition: history_composition_from_envelope(envelope).await,
        conflicts,
        viewer_line: envelope.active_alternative_id.clone().filter(|line| line != &semio_framework_os_spr::trunk_alternative_id(&ArtifactId(envelope.id.clone()))),
        viewer_checkpoint: envelope.viewer_checkpoint_id.clone(),
    };
    let options = semio_framework_os_spr::EncodeOptions { write_backwards_section: true, ..semio_framework_os_spr::EncodeOptions::default() };
    semio_framework_os_spr::encode_history(&log, &options).await.map_err(|error| VcsError::Serialize(error.to_string()))
}

/// 🎯️ Inverse of [`print_document_spr`]: rebuilds an envelope's event log — `edits` (with
/// their persisted `inverse`/`mutation_meta`) and `transitions` — from a decoded `HistoryLog`, then
/// folds it into every derived position and fact ([`project_envelope_history`]). The initial
/// snapshot comes from `pack` via `ArtifactPack::decode_pack`, matching `parse_document_pack`'s contract.
pub async fn parse_document_spr<P, Mutation>(pack: &[u8], spr: &[u8], producer:&(impl ArtifactGenesisProducer<P> + Sync)) -> Result<ParsedDocumentText<P, Mutation>, TextError>
where
    P: Clone + 'static,
    Mutation: OpText + OpBinary + super::Mutation<P> + 'static,
{
    let log = semio_framework_os_spr::decode_history(spr, &semio_framework_os_spr::DecodeOptions::default()).await.map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), TextSpan::at(1, 1)))?;
    parse_decoded_document_spr(pack, log, producer).await
}

/// 🧊️ The validation replay's final projection is a SCRATCH value nothing below reads — the
/// authoritative one is rebuilt from the cursor further down. Shadowing it there left this
/// binding to a bare drop at the end of the function, which aborts the process for any artifact
/// whose projection owns a fail-closed root: every `.pack`/`.spr` load of a generation3d document
/// with a layout entry died in `OrderedMap<WidgetLayout>::drop`
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). `ReplayProjection` now retires it on every early
/// `?` above as well, which the explicit call below never covered.
pub async fn parse_decoded_document_spr<P, Mutation>(pack: &[u8], mut log: semio_framework_os_spr::HistoryLog, producer:&(impl ArtifactGenesisProducer<P> + Sync)) -> Result<ParsedDocumentText<P, Mutation>, TextError>
where
    P: Clone + 'static,
    Mutation: OpText + OpBinary + super::Mutation<P> + 'static,
{
    let initial_snapshot = producer.decode_genesis_pack(pack).map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), TextSpan::at(1, 1)))?;

    async fn decode_op<P, Mutation: OpText + OpBinary + super::Mutation<P>>(payload: &semio_framework_os_spr::OpPayload) -> Result<Mutation, TextError> {
        match (&payload.binary, &payload.text) {
            (Some(bytes), _) => Mutation::decode_op(bytes).map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), TextSpan::at(1, 1))),
            (None, Some(text)) => Mutation::parse_op(text),
            (None, None) => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "op payload carries neither binary nor text".to_string(), TextSpan::at(1, 1))),
        }
    }

    async fn decode_ops<P, Mutation: OpText + OpBinary + super::Mutation<P>>(payloads: &[semio_framework_os_spr::OpPayload]) -> Result<Vec<Mutation>, TextError> {
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
            edit_messages.push(semio_framework_os_spr::EditMessages { edit_id: edit_id.clone(), messages: durable_messages });
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
    let mut transitions: Vec<semio_framework_os_spr::MutationEnvelope> = log.transitions.iter().map(|record| record.to_envelope(&log.doc_id)).collect();
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
        history_shape: semio_framework_os_spr::HistoryShape::Document,
        open_transaction: None,
        viewer_checkpoint_id: None,
    });
    let trunk = semio_framework_os_spr::trunk_alternative_id(&ArtifactId(envelope.id.clone()));
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
    Mutation: OpText + OpBinary,
{
    let pack = envelope.vcs.genesis.pack().to_vec();
    let spr = print_document_spr(envelope).await?;
    let ops = print_ops_log(envelope).await?;
    Ok(ArtifactPackFiles { pack, spr, ops })
}
