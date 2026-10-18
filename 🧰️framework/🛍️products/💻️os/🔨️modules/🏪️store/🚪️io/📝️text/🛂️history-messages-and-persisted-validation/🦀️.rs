//! 🛂️ History messages and persisted validation: the undo policy ordinals, the message and operation meta conversions, the persisted message, conflict, history and lane validation and the history edit and conflict conversions. See [`super`].

use super::*;

/// 🎞️ `crate::os_spr::UndoPolicy` ordinal, matching `HistoryOpMeta.undo_policy`'s wire shape —
/// distinct from `undo_policy_ordinal` above, which maps THIS crate's `ArtifactCommand`-facing
/// `UndoPolicy` (currently `semio_framework_replication::UndoPolicy`; the two enums have identical
/// variants and will merge in the kernel-unification wave, see `protocol_core`'s own doc note).
// 🚫️async: E1 pure ordinal mapping, consumed by `history_op_meta_from_operation_meta` which is
// itself consumed only through `Iterator::map` fn-item arguments — see R9.
pub fn protocol_undo_policy_ordinal(policy: UndoPolicy) -> u8 {
    match policy {
        UndoPolicy::ExactBaseOnly => 0,
        UndoPolicy::TransformAgainstConcurrent => 1,
        UndoPolicy::SemanticUndo => 2,
        UndoPolicy::CompensatingAction => 3,
    }
}

// 🚫️async: E1 pure ordinal mapping, consumed by `mutation_meta_from_history_op_meta` — see R9.
pub fn protocol_undo_policy_from_ordinal(ordinal: u8) -> UndoPolicy {
    match ordinal {
        1 => UndoPolicy::TransformAgainstConcurrent,
        2 => UndoPolicy::SemanticUndo,
        3 => UndoPolicy::CompensatingAction,
        _ => UndoPolicy::ExactBaseOnly,
    }
}

// 🚫️async: E1 pure field-copy, consumed only through `Iterator::map` fn-item arguments — see R9.
pub fn history_message_from_mutation_message(message: &semio_framework_os_spr::MutationMessage) -> semio_framework_os_spr::history::HistoryMessage {
    semio_framework_os_spr::history::HistoryMessage { level: message.level.as_u8(), code: message.code.0.clone(), message: message.message.clone(), target: message.target.clone(), op_index: message.op_index }
}

// 🚫️async: E1 pure field-copy, consumed only through `Iterator::map` fn-item arguments — see R9.
pub fn mutation_message_from_history_message(message: semio_framework_os_spr::history::HistoryMessage) -> Result<semio_framework_os_spr::MutationMessage, String> {
    let level = semio_framework_diagnostic::Severity::from_u8(message.level).ok_or_else(|| format!("unknown mutation message severity {}", message.level))?;
    Ok(semio_framework_os_spr::MutationMessage { level, code: semio_framework_diagnostic::FaultCode(message.code), message: message.message, target: message.target, op_index: message.op_index })
}

// 🚫️async: E1 pure field-copy, consumed only through `Iterator::map` fn-item arguments — see R9.
pub fn history_op_meta_from_operation_meta(meta: &MutationMeta, messages: impl IntoIterator<Item = semio_framework_os_spr::history::HistoryMessage>) -> semio_framework_os_spr::HistoryOpMeta {
    semio_framework_os_spr::HistoryOpMeta {
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
pub fn mutation_meta_from_history_op_meta(meta: semio_framework_os_spr::HistoryOpMeta) -> Result<(MutationMeta, Vec<semio_framework_os_spr::MutationMessage>), String> {
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
            payload_hash: meta.payload_hash.map(semio_framework_os_spr::PayloadHash),
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
pub async fn expected_mutation_message_level(code: &str) -> Option<semio_framework_diagnostic::Severity> {
    protocol::outcome_code_level(code)
}

pub async fn validate_persisted_message(message: &semio_framework_os_spr::MutationMessage, operation_count: Option<usize>) -> Result<(), VcsError> {
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

pub async fn stable_mutation_ids_for_edit<Mutation>(edit: &Edit<Mutation>) -> Result<Vec<MutationId>, VcsError> {
    if edit.mutation_meta.len() != edit.forwards.len() {
        return Err(VcsError::ValidationFailed(format!("edit {} does not carry one operation identity per forward mutation", edit.id)));
    }
    edit.mutation_meta.iter().map(|meta| meta.mutation_id.clone().filter(|id| !id.0.trim().is_empty()).ok_or_else(|| VcsError::ValidationFailed(format!("edit {} has an operation without a stable identity", edit.id)))).collect()
}

pub async fn conflict_messages_for_edits<Mutation>(edits: &[Edit<Mutation>], replayed: &[semio_framework_os_spr::EditMessages]) -> Result<Vec<semio_framework_os_spr::MutationMessage>, VcsError> {
    let replayed_by_edit: HashMap<&str, &semio_framework_os_spr::EditMessages> = replayed.iter().map(|entry| (entry.edit_id.as_str(), entry)).collect();
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

pub async fn canonical_conflict_actors(actors: impl IntoIterator<Item = ActorId>) -> Vec<ActorId> {
    let mut actors: Vec<ActorId> = actors.into_iter().collect();
    actors.sort_by(|left, right| left.0.cmp(&right.0));
    actors.dedup_by(|left, right| left.0 == right.0);
    actors
}

/// 🎯️ A conflict's messages index its kind-specific flattened operation sequence: wire
/// envelopes in `Quarantined` order, or every forward operation of `Degraded` edits in
/// `edit_ids` order. This makes `op_index` globally unambiguous for a conflict rather
/// than accidentally treating every message as if it belonged to its first edit.
pub async fn validate_persisted_conflicts<P, Mutation>(envelope: &ArtifactEnvelopeOwners<P, Mutation>) -> Result<(), VcsError> {
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
            semio_framework_os_spr::ConflictKind::Quarantined { envelopes } => {
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
            semio_framework_os_spr::ConflictKind::Degraded { edit_ids } => {
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
        if conflict.id != semio_framework_os_spr::ConflictId::new(&conflict.kind, &ArtifactId(envelope.id.clone()), &mutation_ids, &conflict.timestamp).await {
            return Err(VcsError::ValidationFailed(format!("conflict {} does not match its content-addressed identity", conflict.id.0)));
        }
    }
    Ok(())
}

/// 🧱️ Validates the complete durable history graph at every persistence boundary.
/// Deliberately free of store construction bounds: codecs must reject malformed history before a
/// caller needs to construct an `ArtifactStore`.
pub async fn validate_durable_history<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> Result<(), VcsError> {
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

pub async fn validate_history_lanes<P, Mutation, I, J, S, R>(envelope: &ArtifactEnvelope<P, Mutation>, applied_edit_ids: I, redo_edit_ids: J) -> Result<(), VcsError>
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
pub async fn history_op_payloads<'a, Mutation: OpBinary + 'a>(mutations: impl IntoIterator<Item = &'a Mutation>) -> Result<Vec<semio_framework_os_spr::OpPayload>, VcsError> {
    let mut payloads = Vec::new();
    for op in mutations {
        let binary = op.encode_op().map_err(|error| VcsError::Serialize(error.to_string()))?;
        payloads.push(semio_framework_os_spr::OpPayload { text: None, binary: Some(binary) });
    }
    Ok(payloads)
}

/// 🎯️ An empty `mutation_meta` (e.g. a hand-authored/externally-injected edit with no
/// explicit meta, distinct from a real dispatch which always populates one entry per
/// forward op) is treated as ABSENT, not as `Some(vec![])` — `encode_edit` requires
/// `metas.len() == ops.len()` when meta is present at all, and an empty-but-`Some` vec
/// would spuriously fail that check for a non-empty `ops`.
pub async fn history_edit_from_edit<Mutation: OpBinary>(edit: &Edit<Mutation>, messages: &[semio_framework_os_spr::MutationMessage], lane: Option<HistoryLane>) -> Result<semio_framework_os_spr::HistoryEdit, VcsError> {
    if messages.iter().any(|message| message.op_index.is_none_or(|index| index as usize >= edit.forwards.len())) {
        return Err(VcsError::ValidationFailed(format!("edit {} carries a message without a valid operation index", edit.id)));
    }
    Ok(semio_framework_os_spr::HistoryEdit {
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
pub async fn history_conflict_from_conflict(conflict: &semio_framework_os_spr::Conflict) -> semio_framework_os_spr::history::HistoryConflict {
    let (kind, edit_ids, envelopes) = match &conflict.kind {
        semio_framework_os_spr::ConflictKind::Quarantined { envelopes, .. } => {
            let mut encoded = Vec::with_capacity(envelopes.len());
            for envelope in envelopes {
                let mut bytes = Vec::new();
                semio_framework_os_spr::encode_envelope(envelope, &mut bytes);
                encoded.push(bytes);
            }
            (0, Vec::new(), encoded)
        }
        semio_framework_os_spr::ConflictKind::Degraded { edit_ids } => (1, edit_ids.clone(), Vec::new()),
    };
    semio_framework_os_spr::history::HistoryConflict {
        id: conflict.id.0.clone(),
        kind,
        status: match conflict.status {
            semio_framework_os_spr::ConflictStatus::Open => 0,
            semio_framework_os_spr::ConflictStatus::Accepted => 1,
            semio_framework_os_spr::ConflictStatus::Discarded => 2,
        },
        actors: conflict.actors.iter().map(|actor| actor.0.clone()).collect(),
        hlt: (conflict.timestamp.actor, conflict.timestamp.physical_ms, conflict.timestamp.logical),
        edit_ids,
        envelopes,
        messages: conflict.messages.iter().map(history_message_from_mutation_message).collect(),
    }
}

pub fn conflict_from_history_conflict(conflict: semio_framework_os_spr::history::HistoryConflict) -> Result<semio_framework_os_spr::Conflict, String> {
    let kind = match conflict.kind {
        0 => {
            let mut envelopes = Vec::with_capacity(conflict.envelopes.len());
            for bytes in conflict.envelopes {
                let mut position = 0;
                let envelope = semio_framework_os_spr::decode_envelope(&bytes, &mut position).map_err(|error| error.to_string())?;
                if position != bytes.len() {
                    return Err("quarantined conflict envelope has trailing bytes".into());
                }
                envelopes.push(envelope);
            }
            semio_framework_os_spr::ConflictKind::Quarantined { envelopes }
        }
        1 => semio_framework_os_spr::ConflictKind::Degraded { edit_ids: conflict.edit_ids },
        value => return Err(format!("unknown conflict kind {value}")),
    };
    let status = match conflict.status {
        0 => semio_framework_os_spr::ConflictStatus::Open,
        1 => semio_framework_os_spr::ConflictStatus::Accepted,
        2 => semio_framework_os_spr::ConflictStatus::Discarded,
        value => return Err(format!("unknown conflict status {value}")),
    };
    Ok(semio_framework_os_spr::Conflict {
        id: semio_framework_os_spr::ConflictId(conflict.id),
        kind,
        status,
        messages: conflict.messages.into_iter().map(mutation_message_from_history_message).collect::<Result<Vec<_>, _>>()?,
        actors: conflict.actors.into_iter().map(ActorId).collect(),
        timestamp: HybridLogicalTimestamp { actor: conflict.hlt.0, physical_ms: conflict.hlt.1, logical: conflict.hlt.2 },
    })
}
