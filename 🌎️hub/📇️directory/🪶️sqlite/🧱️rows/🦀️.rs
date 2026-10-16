//! 🧱️ Row decoders, select lists and the small value helpers every section shares.

use super::*;

fn blob32(row: &rusqlite::Row<'_>, index: usize) -> rusqlite::Result<[u8; 32]> {
    let bytes: Vec<u8> = row.get(index)?;
    bytes.try_into().map_err(|bytes: Vec<u8>| rusqlite::Error::FromSqlConversionFailure(index, rusqlite::types::Type::Blob, format!("expected 32 bytes, got {}", bytes.len()).into()))
}

pub(super) const ADMIN_OPERATION_AUDIT_SELECT: &str = "sequence, request_id, intent_digest, operation_id, occurred_at, phase, terminal, intent_kind, target_kind, target_id, principal_user_id, principal_session_id, principal_generation, correlation_id, event_seq_first, event_seq_last, outcome_code, reason_code";

pub(super) const ADMIN_OPERATION_EFFECT_RECEIPT_SELECT: &str = "operation_id, intent_digest, committed_at, outcome_code, event_seq_first, event_seq_last";

pub(super) const DIRECTORY_COMMAND_RECEIPT_SELECT: &str = "actor_user_id, request_id, command_sha256, result_kind, disposition, event_seq_first, event_seq_last, receipt_sha256, claimed_at, completed_at";

pub(super) fn directory_command_receipt_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<DirectoryCommandReceiptRecord> {
    let convert = |index: usize, message: String| rusqlite::Error::FromSqlConversionFailure(index, rusqlite::types::Type::Text, Box::new(std::io::Error::other(message)));
    let result_kind_text: String = row.get(3)?;
    let result_kind = directory_command_result_kind_from_str(&result_kind_text).map_err(|error| convert(3, error.to_string()))?;
    let disposition_text: String = row.get(4)?;
    let disposition = match disposition_text.as_str() {
        "pending" => DirectoryCommandDispositionV1::Pending,
        "completed" => DirectoryCommandDispositionV1::Completed,
        other => return Err(convert(4, format!("unknown command disposition '{other}'"))),
    };
    let event_seq_first: Option<i64> = row.get(5)?;
    let event_seq_last: Option<i64> = row.get(6)?;
    Ok(DirectoryCommandReceiptRecord {
        actor_user_id: row.get(0)?,
        request_id: row.get(1)?,
        command_sha256: row.get(2)?,
        result_kind,
        disposition,
        event_seq_first: event_seq_first.map(|seq| seq.unsigned_abs()),
        event_seq_last: event_seq_last.map(|seq| seq.unsigned_abs()),
        receipt_sha256: row.get(7)?,
        claimed_at: row.get(8)?,
        completed_at: row.get(9)?,
    })
}

pub(super) fn checkpoint_publication_receipt_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<CheckpointPublicationReceiptRecordV1> {
    let disposition = match row.get::<_, String>(3)?.as_str() {
        "pending" => CheckpointPublicationDispositionV1::Pending,
        "completed" => CheckpointPublicationDispositionV1::Completed,
        other => return Err(rusqlite::Error::FromSqlConversionFailure(3, rusqlite::types::Type::Text, std::io::Error::other(format!("unknown checkpoint publication disposition '{other}'")).into())),
    };
    let checkpoint_id = row
        .get::<_, Option<Vec<u8>>>(4)?
        .map(|bytes| bytes.try_into().map(ArtifactHash).map_err(|bytes: Vec<u8>| rusqlite::Error::FromSqlConversionFailure(4, rusqlite::types::Type::Blob, std::io::Error::other(format!("expected 32 bytes, got {}", bytes.len())).into())))
        .transpose()?;
    Ok(CheckpointPublicationReceiptRecordV1 { actor_user_id: row.get(0)?, correlation_id: row.get(1)?, command_sha256: row.get(2)?, disposition, checkpoint_id, claimed_at: row.get(5)?, completed_at: row.get(6)? })
}

pub(super) fn admin_operation_audit_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<AdminOperationAuditRecord> {
    let sequence: i64 = row.get(0)?;
    let principal_generation: i64 = row.get(12)?;
    let event_seq_first: Option<i64> = row.get(14)?;
    let event_seq_last: Option<i64> = row.get(15)?;
    Ok(AdminOperationAuditRecord {
        sequence: u64::try_from(sequence).map_err(|error| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Integer, Box::new(error)))?,
        fact: NewAdminOperationAuditRecord {
            request_id: row.get(1)?,
            intent_digest: row.get(2)?,
            operation_id: row.get(3)?,
            occurred_at: row.get(4)?,
            phase: row.get(5)?,
            intent_kind: row.get(7)?,
            target_kind: row.get(8)?,
            target_id: row.get(9)?,
            principal_user_id: row.get(10)?,
            principal_session_id: row.get(11)?,
            principal_generation: u64::try_from(principal_generation).map_err(|error| rusqlite::Error::FromSqlConversionFailure(12, rusqlite::types::Type::Integer, Box::new(error)))?,
            correlation_id: row.get(13)?,
            event_seq_first: event_seq_first.map(u64::try_from).transpose().map_err(|error| rusqlite::Error::FromSqlConversionFailure(14, rusqlite::types::Type::Integer, Box::new(error)))?,
            event_seq_last: event_seq_last.map(u64::try_from).transpose().map_err(|error| rusqlite::Error::FromSqlConversionFailure(15, rusqlite::types::Type::Integer, Box::new(error)))?,
            outcome_code: row.get(16)?,
            reason_code: row.get(17)?,
        },
    })
}

pub(super) fn admin_operation_effect_receipt_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<AdminOperationEffectReceiptV1> {
    let first: Option<i64> = row.get(4)?;
    let last: Option<i64> = row.get(5)?;
    Ok(AdminOperationEffectReceiptV1 {
        operation_id: row.get(0)?,
        intent_digest: row.get(1)?,
        committed_at: row.get(2)?,
        outcome_code: row.get(3)?,
        event_seq_first: first.map(u64::try_from).transpose().map_err(|error| rusqlite::Error::FromSqlConversionFailure(4, rusqlite::types::Type::Integer, Box::new(error)))?,
        event_seq_last: last.map(u64::try_from).transpose().map_err(|error| rusqlite::Error::FromSqlConversionFailure(5, rusqlite::types::Type::Integer, Box::new(error)))?,
    })
}

pub(super) fn user_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<UserRecord> {
    Ok(UserRecord { id: row.get(0)?, email: row.get(1)?, display_name: row.get(2)?, password_hash: row.get(3)?, sso_subject: row.get(4)?, sso_provider: row.get(5)?, created_at: row.get(6)? })
}

pub(super) fn sync_session_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<SyncSessionRecord> {
    let space_role: Option<String> = row.get(9)?;
    Ok(SyncSessionRecord {
        id: row.get(0)?,
        auth_session_id: row.get(1)?,
        authorization_generation: u64::try_from(row.get::<_, i64>(2)?).unwrap_or(0),
        actor_id: row.get(3)?,
        space_id: row.get(4)?,
        document_id: row.get(5)?,
        surface: row.get(6)?,
        user_id: row.get(7)?,
        authenticated_email: row.get(8)?,
        space_role: space_role.and_then(|r| SpaceRole::parse(&r)),
        client_label: row.get(10)?,
        connected_at: row.get(11)?,
        disconnected_at: row.get(12)?,
    })
}

pub(super) fn invite_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<InviteRecord> {
    let role: String = row.get(4)?;
    Ok(InviteRecord {
        id: row.get(0)?,
        selector: row.get(1)?,
        secret_digest: blob32(row, 2)?,
        space_id: row.get(3)?,
        role: SpaceRole::parse(&role).unwrap_or(SpaceRole::Spectator),
        created_at: row.get(5)?,
        expires_at: row.get(6)?,
        revoked_at: row.get(7)?,
        revoked_reason: row.get(8)?,
        accepted_at: row.get(9)?,
        accepted_event_id: row.get(10)?,
    })
}

pub(super) fn share_token_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ShareTokenRecord> {
    Ok(ShareTokenRecord {
        id: row.get(0)?,
        selector: row.get(1)?,
        secret_digest: blob32(row, 2)?,
        scope: DocumentScope::new(row.get::<_, String>(3)?, row.get::<_, String>(4)?),
        created_at: row.get(5)?,
        expires_at: row.get(6)?,
        revoked_at: row.get(7)?,
        revoked_reason: row.get(8)?,
    })
}

pub(super) fn auth_session_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<AuthSessionRecord> {
    let generation = row.get::<_, i64>(10)?;
    let session_kind: String = row.get(12)?;
    Ok(AuthSessionRecord {
        id: row.get(0)?,
        selector: row.get(1)?,
        secret_digest: blob32(row, 2)?,
        user_id: row.get(3)?,
        identity_provider: row.get(4)?,
        identity_subject_digest: blob32(row, 5)?,
        issued_at: row.get(6)?,
        expires_at: row.get(7)?,
        revoked_at: row.get(8)?,
        revoked_reason: row.get(9)?,
        authorization_generation: u64::try_from(generation).unwrap_or(0),
        device_instance_id: row.get(11)?,
        session_kind: AuthSessionKind::parse(&session_kind).unwrap_or(AuthSessionKind::External),
    })
}

/// 🤖️ Rehydrates one `hub_agent_delegation` row. A stored audience outside the enum is a corrupt
/// projection, never a silently-defaulted `read`.
pub(super) fn agent_delegation_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<AgentDelegationRecord> {
    let digest: Vec<u8> = row.get(2)?;
    let audience: String = row.get(6)?;
    Ok(AgentDelegationRecord {
        id: row.get(0)?,
        selector: row.get(1)?,
        secret_digest: <[u8; 32]>::try_from(digest.as_slice()).map_err(|_| rusqlite::Error::IntegralValueOutOfRange(2, 0))?,
        space_id: row.get(3)?,
        delegating_user_id: row.get(4)?,
        agent_label: row.get(5)?,
        audience: AgentAudience::parse(&audience).ok_or(rusqlite::Error::IntegralValueOutOfRange(6, 0))?,
        created_at: row.get(7)?,
        expires_at: row.get(8)?,
        revoked_at: row.get(9)?,
        revoked_reason: row.get(10)?,
    })
}

pub(super) fn auth_audit_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<AuthAuditRecord> {
    Ok(AuthAuditRecord {
        id: row.get(0)?,
        occurred_at: row.get(1)?,
        event_kind: row.get(2)?,
        auth_session_id: row.get(3)?,
        target_user_id: row.get(4)?,
        actor_user_id: row.get(5)?,
        provider: row.get(6)?,
        outcome_code: row.get(7)?,
        reason_code: row.get(8)?,
        correlation_id: row.get(9)?,
        peer_class: row.get(10)?,
    })
}

pub(super) fn document_descriptor_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<DocumentDescriptor> {
    Ok(DocumentDescriptor {
        space_id: row.get(0)?,
        document_id: row.get(1)?,
        artifact_kind: row.get(2)?,
        artifact_schema: row.get(3)?,
        owner: DocumentOwner { plugin_id: row.get(4)?, package_id: row.get(5)?, version: row.get(6)?, package_hash: row.get(7)? },
        pack_schema_hash: row.get(8)?,
        bootstrap_version: row.get::<_, i64>(9)? as u32,
        bootstrap_frontier: DocumentFrontier { head_seq: row.get::<_, i64>(10)? as u64, commit_seq: row.get::<_, i64>(11)? as u64, epoch: row.get::<_, i64>(12)? as u64 },
        bootstrap_snapshot_hash: row.get(13)?,
    })
}

pub(super) fn published_checkpoint_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<PublishedArtifactCheckpoint> {
    let payload: String = row.get(0)?;
    let value: serde_json::Value = serde_json::from_str(&payload).map_err(|error| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(error)))?;
    PublishedArtifactCheckpoint::from_value(DslValue::from(value)).map_err(|error| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(error)))
}

pub(super) fn verified_checkpoint_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ArtifactCheckpoint> {
    let payload: String = row.get(0)?;
    let value: serde_json::Value = serde_json::from_str(&payload).map_err(|error| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(error)))?;
    ArtifactCheckpoint::from_value(DslValue::from(value)).map_err(|error| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(error)))
}

pub(super) fn artifact_retention_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ArtifactRetention> {
    let payload: String = row.get(0)?;
    let value: serde_json::Value = serde_json::from_str(&payload).map_err(|error| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(error)))?;
    ArtifactRetention::from_value(DslValue::from(value)).map_err(|error| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(error)))
}

pub(super) fn event_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<DirectoryEvent> {
    let seq: i64 = row.get(0)?;
    let id: String = row.get(1)?;
    let hlc_physical: i64 = row.get(2)?;
    let hlc_logical: i64 = row.get(3)?;
    let actor_kind: String = row.get(4)?;
    let actor_id: String = row.get(5)?;
    let space_id: Option<String> = row.get(6)?;
    let user_id: Option<String> = row.get(7)?;
    let payload: String = row.get(8)?;
    let recorded_at_ms: i64 = row.get(9)?;
    let value: serde_json::Value = serde_json::from_str(&payload).map_err(|error| rusqlite::Error::FromSqlConversionFailure(8, rusqlite::types::Type::Text, Box::new(error)))?;
    let body = DirectoryEventBody::from_value(DslValue::from(value)).map_err(|error| rusqlite::Error::FromSqlConversionFailure(8, rusqlite::types::Type::Text, Box::new(error)))?;
    let seq = u64::try_from(seq).map_err(|error| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Integer, Box::new(error)))?;
    let logical = u32::try_from(hlc_logical).map_err(|error| rusqlite::Error::FromSqlConversionFailure(3, rusqlite::types::Type::Integer, Box::new(error)))?;
    Ok(DirectoryEvent { seq, id, hlc: Hlc { physical_ms: hlc_physical, logical }, actor: DirectoryActor { kind: actor_kind_from_str(&actor_kind), id: actor_id }, space_id, user_id, body, recorded_at_ms })
}
