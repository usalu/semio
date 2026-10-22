//! 🔁️ Replay ops and document parsing: the replay of the ops over an envelope, the parsed envelope settlement and discard and the document text and pack parse. See [`super`].

use super::*;

/// 📥️ Parses the explicit `.ops` records against an already-obtained initial snapshot.
/// Text persistence is intentionally strict: it requires inverse operations, one authoritative
/// metadata record per forward operation, durable messages/conflicts, and exactly one cursor.
pub async fn replay_ops<P, Mutation>(initial_snapshot: P, ops: &str, producer:&(impl ArtifactGenesisProducer<P> + Sync)) -> Result<ParsedDocumentText<P, Mutation>, TextError>
where
    P: Clone + 'static,
    Mutation: OpText + OpBinary + super::Mutation<P> + semio_framework_os_spr::ColdMutation<P> + 'static,
{
    let mut schema = String::new();
    let mut id = String::new();
    let mut edits: Vec<Edit<Mutation>> = Vec::new();
    let mut transitions: Vec<semio_framework_os_spr::MutationEnvelope> = Vec::new();
    let mut inverse_by_edit: HashMap<String, semio_framework_value_persistent::list::PagedList<Mutation, {usize::MAX}>> = HashMap::new();
    let mut metadata_by_edit: HashMap<String, HashMap<u32, MutationMeta>> = HashMap::new();
    let mut messages_by_edit: HashMap<String, Vec<semio_framework_os_spr::MutationMessage>> = HashMap::new();
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
                    transition_from_ops_line(&id, transition_id, actor, &clock, after, semio_framework_os_spr::HistoryTransition::Revert { mutation_ids: mutation_ids(operations) })
                        .map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, TextSpan::at(line_no, 1)))?,
                );
            }
            OpsHeaderLine::Reinstate { id: transition_id, actor, clock, after, operations } => {
                transitions.push(
                    transition_from_ops_line(&id, transition_id, actor, &clock, after, semio_framework_os_spr::HistoryTransition::Reinstate { mutation_ids: mutation_ids(operations) })
                        .map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, TextSpan::at(line_no, 1)))?,
                );
            }
            OpsHeaderLine::Commit { id: transition_id, actor, clock, after, checkpoint, parent, change, operations, description, saved, by, message, at, line } => {
                let authors = by.into_iter().map(|author| semio_framework_os_spr::TransitionAuthor { id: author.id, name: author.name, avatar: author.avatar }).collect();
                let checkpoint = semio_framework_os_spr::TransitionCheckpoint {
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
                    transition_from_ops_line(&id, transition_id, actor, &clock, after, semio_framework_os_spr::HistoryTransition::Commit(checkpoint))
                        .map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, TextSpan::at(line_no, 1)))?,
                );
            }
            OpsHeaderLine::Branch { id: transition_id, actor, clock, after, alternative, name, checkpoint } => {
                transitions.push(
                    transition_from_ops_line(&id, transition_id, actor, &clock, after, semio_framework_os_spr::HistoryTransition::Branch { alternative_id: alternative, name, checkpoint_id: checkpoint })
                        .map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, TextSpan::at(line_no, 1)))?,
                );
            }
            OpsHeaderLine::Checkout { id: transition_id, actor, clock, after, checkpoint, alternative } => {
                transitions.push(
                    transition_from_ops_line(&id, transition_id, actor, &clock, after, semio_framework_os_spr::HistoryTransition::Checkout { checkpoint_id: checkpoint, alternative_id: alternative })
                        .map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, TextSpan::at(line_no, 1)))?,
                );
            }
            OpsHeaderLine::Repin { id: transition_id, actor, clock, after, checkpoint, pinned, pins } => {
                let pins = pins.into_iter().map(|pin| semio_framework_os_spr::TransitionPin { child_uri: pin.child, checkpoint_id: pin.checkpoint }).collect();
                transitions.push(
                    transition_from_ops_line(&id, transition_id, actor, &clock, after, semio_framework_os_spr::HistoryTransition::Repin { checkpoint_id: checkpoint, pinned_checkpoint_id: pinned, pins })
                        .map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, TextSpan::at(line_no, 1)))?,
                );
            }
            OpsHeaderLine::Supersede { id: transition_id, actor, clock, after, scope, inputs, observed } => {
                let inputs = inputs.into_iter().map(semio_framework_os_spr::SupersededInput::try_from).collect::<Result<Vec<_>, _>>().map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, TextSpan::at(line_no, 1)))?;
                let supersede = semio_framework_os_spr::TransitionSupersede { scope, inputs };
                supersede.validate().map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), TextSpan::at(line_no, 1)))?;
                let mut envelope = transition_from_ops_line(&id, transition_id, actor, &clock, after, semio_framework_os_spr::HistoryTransition::Supersede(supersede))
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
        edit_messages.push(semio_framework_os_spr::EditMessages { edit_id, messages });
    }
    let edits = ArtifactHistoryLedger::try_from_preflighted(edits).map_err(|_| TextError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, "ops edit capacity exceeded".to_string(), TextSpan::at(1, 1)))?;
    transitions.sort_by(|left, right| (left.timestamp.cmp_key(), left.mutation_id.0.as_str()).cmp(&(right.timestamp.cmp_key(), right.mutation_id.0.as_str())));
    let envelope = ArtifactEnvelope::from_owners(ArtifactEnvelopeOwners {
        schema,
        id,
        vcs: ArtifactVcs { genesis: AdmittedArtifactGenesis::born(initial_snapshot,producer).map_err(|error| TextError::new(error.kind, error.message, TextSpan::at(1, 1)))?, edits, changes: ArtifactHistoryLedger::new(), checkpoints: ArtifactHistoryLedger::new(), alternatives: ArtifactHistoryLedger::new() },
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
        history_shape: semio_framework_os_spr::HistoryShape::Document,
        open_transaction: None,
        viewer_checkpoint_id: None,
    });
    settle_parsed_envelope(envelope).await
}

/// 🧮️ Finishes a load: folds the parsed event log into every derived position and fact,
/// validates the result and materializes the live snapshot by replaying the applied edits over the
/// genesis. A refused document is discarded through [`discard_parsed_envelope`], never dropped.
pub async fn settle_parsed_envelope<P, Mutation>(mut envelope: ArtifactEnvelope<P, Mutation>) -> Result<ParsedDocumentText<P, Mutation>, TextError>
where
    P: Clone + 'static,
    Mutation: super::Mutation<P> + semio_framework_os_spr::ColdMutation<P> + OpBinary + 'static,
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
pub fn discard_parsed_envelope<P, Mutation>(envelope: ArtifactEnvelope<P, Mutation>)
where
    Mutation: super::Mutation<P>,
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
pub async fn parse_document_text<P, Mutation>(dsl: &str, ops: &str, producer:&(impl ArtifactGenesisProducer<P> + Sync)) -> Result<ParsedDocumentText<P, Mutation>, TextError>
where
    P: Clone + ArtifactDsl + 'static,
    Mutation: OpText + OpBinary + super::Mutation<P> + semio_framework_os_spr::ColdMutation<P> + 'static,
{
    let initial_snapshot = P::parse_dsl(dsl)?;
    replay_ops(initial_snapshot, ops, producer).await
}

/// 📥️ spr-first pack counterpart of `parse_document_text`: pack+spr are the AUTHORITATIVE
/// pair (see `ArtifactPackFiles`'s doc) — this is a thin forward onto `parse_document_spr`, which
/// recovers real `inverse`/`mutation_meta`/`cursor` instead of recomputing them via replay.
pub async fn parse_document_pack<P, Mutation>(pack: &[u8], spr: &[u8], producer:&(impl ArtifactGenesisProducer<P> + Sync)) -> Result<ParsedDocumentText<P, Mutation>, TextError>
where
    P: Clone + 'static,
    Mutation: OpText + OpBinary + super::Mutation<P> + semio_framework_os_spr::ColdMutation<P> + 'static,
{
    parse_document_spr(pack, spr, producer).await
}
