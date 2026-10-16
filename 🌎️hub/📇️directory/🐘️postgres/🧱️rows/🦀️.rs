//! 🧱️ Row decoders, select lists and the small value helpers every section shares.

use super::*;

pub(super) fn array32(bytes: Vec<u8>, field: &str) -> DirectoryResult<[u8; 32]> {
    bytes.try_into().map_err(|bytes: Vec<u8>| DirectoryError::Backend(format!("{field} requires 32 bytes, got {}", bytes.len())))
}

pub(super) fn user_from_row(row: (String, String, String, Option<String>, Option<String>, Option<String>, i64)) -> UserRecord {
    let (id, email, display_name, password_hash, sso_subject, sso_provider, created_at) = row;
    UserRecord { id, email, display_name, password_hash, sso_subject, sso_provider, created_at }
}

pub(super) type InviteRow = (String, String, Vec<u8>, String, String, i64, i64, Option<i64>, Option<String>, Option<i64>, Option<String>);

pub(super) type ShareRow = (String, String, Vec<u8>, String, String, i64, i64, Option<i64>, Option<String>);

pub(super) type AuthSessionRow = (String, String, Vec<u8>, String, String, Vec<u8>, i64, i64, Option<i64>, Option<String>, i64, String, String);

pub(super) type AuthAuditRow = (String, i64, String, Option<String>, Option<String>, Option<String>, Option<String>, String, Option<String>, String, String);

pub(super) const DIRECTORY_COMMAND_RECEIPT_SELECT: &str = "actor_user_id, request_id, command_sha256, result_kind, disposition, event_seq_first, event_seq_last, receipt_sha256, claimed_at, completed_at";

pub(super) type DirectoryCommandReceiptRow = (String, String, String, String, String, Option<i64>, Option<i64>, Option<String>, i64, Option<i64>);

pub(super) type CheckpointPublicationReceiptRowV1 = (String, String, String, String, Option<Vec<u8>>, i64, Option<i64>);

pub(super) fn checkpoint_publication_receipt_from_row(row: CheckpointPublicationReceiptRowV1) -> DirectoryResult<CheckpointPublicationReceiptRecordV1> {
    let disposition = match row.3.as_str() {
        "pending" => CheckpointPublicationDispositionV1::Pending,
        "completed" => CheckpointPublicationDispositionV1::Completed,
        other => return Err(DirectoryError::Backend(format!("unknown checkpoint publication disposition '{other}'"))),
    };
    let checkpoint_id = row.4.map(|bytes| bytes.try_into().map(ArtifactHash).map_err(|_| DirectoryError::Backend("checkpoint publication id is not 32 bytes".into()))).transpose()?;
    Ok(CheckpointPublicationReceiptRecordV1 { actor_user_id: row.0, correlation_id: row.1, command_sha256: row.2, disposition, checkpoint_id, claimed_at: row.5, completed_at: row.6 })
}

pub(super) fn directory_command_receipt_from_row(row: DirectoryCommandReceiptRow) -> DirectoryResult<DirectoryCommandReceiptRecord> {
    let disposition = match row.4.as_str() {
        "pending" => DirectoryCommandDispositionV1::Pending,
        "completed" => DirectoryCommandDispositionV1::Completed,
        other => return Err(DirectoryError::Backend(format!("unknown command disposition '{other}'"))),
    };
    Ok(DirectoryCommandReceiptRecord {
        actor_user_id: row.0,
        request_id: row.1,
        command_sha256: row.2,
        result_kind: directory_command_result_kind_from_str(&row.3)?,
        disposition,
        event_seq_first: row.5.map(|seq| seq.unsigned_abs()),
        event_seq_last: row.6.map(|seq| seq.unsigned_abs()),
        receipt_sha256: row.7,
        claimed_at: row.8,
        completed_at: row.9,
    })
}

pub(super) struct AdminOperationAuditRow {
    pub(super) sequence: i64,
    pub(super) request_id: String,
    pub(super) intent_digest: String,
    pub(super) operation_id: String,
    pub(super) occurred_at: i64,
    pub(super) phase: String,
    pub(super) intent_kind: String,
    pub(super) target_kind: String,
    pub(super) target_id: String,
    pub(super) principal_user_id: String,
    pub(super) principal_session_id: String,
    pub(super) principal_generation: i64,
    pub(super) correlation_id: String,
    pub(super) event_seq_first: Option<i64>,
    pub(super) event_seq_last: Option<i64>,
    pub(super) outcome_code: String,
    pub(super) reason_code: Option<String>,
}

impl<'row> sqlx_core::from_row::FromRow<'row, sqlx_postgres::PgRow> for AdminOperationAuditRow {
    fn from_row(row: &'row sqlx_postgres::PgRow) -> Result<Self, sqlx_core::error::Error> {
        use sqlx_core::row::Row as _;
        Ok(Self {
            sequence: row.try_get("sequence")?,
            request_id: row.try_get("request_id")?,
            intent_digest: row.try_get("intent_digest")?,
            operation_id: row.try_get("operation_id")?,
            occurred_at: row.try_get("occurred_at")?,
            phase: row.try_get("phase")?,
            intent_kind: row.try_get("intent_kind")?,
            target_kind: row.try_get("target_kind")?,
            target_id: row.try_get("target_id")?,
            principal_user_id: row.try_get("principal_user_id")?,
            principal_session_id: row.try_get("principal_session_id")?,
            principal_generation: row.try_get("principal_generation")?,
            correlation_id: row.try_get("correlation_id")?,
            event_seq_first: row.try_get("event_seq_first")?,
            event_seq_last: row.try_get("event_seq_last")?,
            outcome_code: row.try_get("outcome_code")?,
            reason_code: row.try_get("reason_code")?,
        })
    }
}

pub(super) type SyncSessionRow = (String, Option<String>, i64, String, String, String, String, Option<String>, Option<String>, Option<String>, String, i64, Option<i64>);

pub(super) fn invite_from_row(row: InviteRow) -> DirectoryResult<InviteRecord> {
    let (id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at, accepted_event_id) = row;
    Ok(InviteRecord {
        id,
        selector,
        secret_digest: array32(secret_digest, "invite digest")?,
        space_id,
        role: SpaceRole::parse(&role).unwrap_or(SpaceRole::Spectator),
        created_at,
        expires_at,
        revoked_at,
        revoked_reason,
        accepted_at,
        accepted_event_id,
    })
}

pub(super) fn share_from_row(row: ShareRow) -> DirectoryResult<ShareTokenRecord> {
    let (id, selector, secret_digest, space_id, document_id, created_at, expires_at, revoked_at, revoked_reason) = row;
    Ok(ShareTokenRecord { id, selector, secret_digest: array32(secret_digest, "share digest")?, scope: DocumentScope::new(space_id, document_id), created_at, expires_at, revoked_at, revoked_reason })
}

pub(super) fn auth_session_from_row(row: AuthSessionRow) -> DirectoryResult<AuthSessionRecord> {
    let (id, selector, secret_digest, user_id, identity_provider, identity_subject_digest, issued_at, expires_at, revoked_at, revoked_reason, authorization_generation, device_instance_id, session_kind) = row;
    Ok(AuthSessionRecord {
        id,
        selector,
        secret_digest: array32(secret_digest, "session digest")?,
        user_id,
        identity_provider,
        identity_subject_digest: array32(identity_subject_digest, "identity subject digest")?,
        issued_at,
        expires_at,
        revoked_at,
        revoked_reason,
        authorization_generation: u64::try_from(authorization_generation).map_err(backend)?,
        device_instance_id,
        session_kind: AuthSessionKind::parse(&session_kind).ok_or_else(|| DirectoryError::Backend("stored session kind is invalid".into()))?,
    })
}

pub(super) fn auth_audit_from_row(row: AuthAuditRow) -> AuthAuditRecord {
    let (id, occurred_at, event_kind, auth_session_id, target_user_id, actor_user_id, provider, outcome_code, reason_code, correlation_id, peer_class) = row;
    AuthAuditRecord { id, occurred_at, event_kind, auth_session_id, target_user_id, actor_user_id, provider, outcome_code, reason_code, correlation_id, peer_class }
}

pub(super) fn admin_operation_audit_from_row(row: AdminOperationAuditRow) -> DirectoryResult<AdminOperationAuditRecord> {
    Ok(AdminOperationAuditRecord {
        sequence: u64::try_from(row.sequence).map_err(backend)?,
        fact: NewAdminOperationAuditRecord {
            request_id: row.request_id,
            intent_digest: row.intent_digest,
            operation_id: row.operation_id,
            occurred_at: row.occurred_at,
            phase: row.phase,
            intent_kind: row.intent_kind,
            target_kind: row.target_kind,
            target_id: row.target_id,
            principal_user_id: row.principal_user_id,
            principal_session_id: row.principal_session_id,
            principal_generation: u64::try_from(row.principal_generation).map_err(backend)?,
            correlation_id: row.correlation_id,
            event_seq_first: row.event_seq_first.map(u64::try_from).transpose().map_err(backend)?,
            event_seq_last: row.event_seq_last.map(u64::try_from).transpose().map_err(backend)?,
            outcome_code: row.outcome_code,
            reason_code: row.reason_code,
        },
    })
}

pub(super) fn sync_session_from_row(row: SyncSessionRow) -> SyncSessionRecord {
    let (id, auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, authenticated_email, space_role, client_label, connected_at, disconnected_at) = row;
    SyncSessionRecord {
        id,
        auth_session_id,
        authorization_generation: u64::try_from(authorization_generation).unwrap_or(0),
        actor_id,
        space_id,
        document_id,
        surface,
        user_id,
        authenticated_email,
        space_role: space_role.and_then(|r| SpaceRole::parse(&r)),
        client_label,
        connected_at,
        disconnected_at,
    }
}

pub(super) fn event_from_row(row: (i64, String, i64, i64, String, String, Option<String>, Option<String>, serde_json::Value, i64)) -> DirectoryResult<DirectoryEvent> {
    let (seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, payload, recorded_at_ms) = row;
    let body = DirectoryEventBody::from_value(DslValue::from(payload)).map_err(backend)?;
    Ok(DirectoryEvent {
        seq: u64::try_from(seq).map_err(backend)?,
        id,
        hlc: Hlc { physical_ms: hlc_physical, logical: u32::try_from(hlc_logical).map_err(backend)? },
        actor: DirectoryActor { kind: actor_kind_from_str(&actor_kind), id: actor_id },
        space_id,
        user_id,
        body,
        recorded_at_ms,
    })
}
