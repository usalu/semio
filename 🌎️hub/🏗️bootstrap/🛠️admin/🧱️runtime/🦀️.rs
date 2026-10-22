//! 🧱️ The `admin` vocabulary that carries no `HubState`: types, ledgers and helpers the state and the routes build on.

use semio_hub_directory_contract::{ADMIN_PAGE_MAX, ADMIN_RESPONSE_MAX_BYTES, AUTH_TEXT_MAX_BYTES, CAPABILITY_MAX_TTL_SECS};
use directory::os_directory::{AdminConnectionSnapshotV1, AdminIntentOutcomeV1, AdminIntentReceiptV1, AdminIntentResultV1, AdminIntentStateV1, AdminIntentV1, AdminOperationAuditPhaseV1, AdminOperationAuditV1, AdminOperationProgressV1, AdminPageV1, AdminRecordedConnectionV1, DirectoryCommand, DirectorySpaceVisibility, MemberView, SpaceView};
use semio_hub_directory_contract::model::{AdminOperationAuditRecord, NewAdminOperationAuditRecord, NewAdminOperationEffectReceiptV1};
use semio_hub_ledgers::guard::AdminPrincipalV1;
use std::sync::{Arc, Mutex};
use std::collections::BTreeMap;
use serde::Deserialize;
use directory::{DslValue, ToValue, os_directory};
use crate::directory_commands::FencedDirectoryCommandErrorV1;
use semio_hub_runtime::readiness::HUB_PROCESS_START;
use semio_hub_observability::{HubObservabilityV1, HubRouteMetricsV1};
use semio_hub_directory::{ProjectionRebuildControl, ProjectionRebuildProgress};
use semio_framework_hash::Sha256;
use semio_framework_async_slots::ShardedMap;
use semio_hub_ledgers::socket_grants::SocketBindingKeyV1;
use axum::http::StatusCode;
use semio_framework_trace::record::Tracer;
use semio_hub_trusted_catalog::VerifiedTrustedCatalog;
use semio_hub_runtime::support::now_ms;

//#region 🔖️State
pub const ADMIN_OPERATION_DEADLINE: std::time::Duration = std::time::Duration::from_secs(10);

pub const ADMIN_OPERATION_SHUTDOWN_DEADLINE: std::time::Duration = std::time::Duration::from_secs(11);

pub const ADMIN_TERMINAL_RETRY_DEADLINE: std::time::Duration = std::time::Duration::from_secs(2);

pub const ADMIN_EFFECT_PRE_EFFECT: u8 = 0;

pub const ADMIN_EFFECT_ADMITTED: u8 = 1;

const ADMIN_EFFECT_CANCELLED: u8 = 2;
//#endregion 🔖️State

//#region 🔖️Admin
/// 🧮️ Best-effort recursive directory size in bytes — used only for the admin overview's
/// `dataDirBytes`; any unreadable entry is silently skipped rather than failing the whole overview.
pub fn dir_size(path: &std::path::Path) -> u64 {
    let mut total = 0u64;
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                total += if metadata.is_dir() { dir_size(&entry.path()) } else { metadata.len() };
            }
        }
    }
    total
}

#[derive(Clone)]
pub struct AdminIntentMetadata {
    pub intent_kind: &'static str,
    pub target_kind: &'static str,
    pub target_id: String,
    pub reason_code: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdminPageQuery {
    pub cursor: Option<String>,
    pub limit: Option<usize>,
}

fn admin_cursor_mac(key: &[u8; 32], principal: &AdminPrincipalV1, payload: &[u8; 10], scope: Option<&str>) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(b"semio/hub/admin-page-cursor/v1\0");
    hash.update(key);
    hash.update(&(principal.auth_session_id.len() as u32).to_be_bytes());
    hash.update(principal.auth_session_id.as_bytes());
    hash.update(&principal.authorization_generation.to_be_bytes());
    hash.update(payload);
    hash.update(&[u8::from(scope.is_some())]);
    if let Some(scope) = scope {
        hash.update(&(scope.len() as u32).to_be_bytes());
        hash.update(scope.as_bytes());
    }
    hash.finalize()
}

pub fn admin_cursor_decode_scoped(key: &[u8; 32], principal: &AdminPrincipalV1, route: u8, scope: Option<&str>, encoded: Option<&str>) -> Result<usize, StatusCode> {
    let Some(encoded) = encoded else {
        return Ok(0);
    };
    if encoded.len() != 84 || !encoded.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f')) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut raw = [0u8; 42];
    for (index, byte) in raw.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&encoded[index * 2..index * 2 + 2], 16).map_err(|_| StatusCode::BAD_REQUEST)?;
    }
    if raw[0] != 1 || raw[1] != route {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut payload = [0u8; 10];
    payload.copy_from_slice(&raw[..10]);
    let mut supplied = [0u8; 32];
    supplied.copy_from_slice(&raw[10..]);
    if !semio_hub_directory_contract::constant_time_digest_eq(&admin_cursor_mac(key, principal, &payload, scope), &supplied) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let offset = u64::from_be_bytes(payload[2..].try_into().map_err(|_| StatusCode::BAD_REQUEST)?);
    usize::try_from(offset).map_err(|_| StatusCode::BAD_REQUEST)
}

pub fn admin_cursor_decode(key: &[u8; 32], principal: &AdminPrincipalV1, route: u8, encoded: Option<&str>) -> Result<usize, StatusCode> {
    admin_cursor_decode_scoped(key, principal, route, None, encoded)
}

pub fn admin_cursor_encode_scoped(key: &[u8; 32], principal: &AdminPrincipalV1, route: u8, scope: Option<&str>, offset: usize) -> Result<String, StatusCode> {
    let mut payload = [0u8; 10];
    payload[0] = 1;
    payload[1] = route;
    payload[2..].copy_from_slice(&u64::try_from(offset).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.to_be_bytes());
    let mut raw = [0u8; 42];
    raw[..10].copy_from_slice(&payload);
    raw[10..].copy_from_slice(&admin_cursor_mac(key, principal, &payload, scope));
    Ok(os_directory::io::binary::artifact_hash::hex_lower(&raw))
}

pub fn admin_cursor_encode(key: &[u8; 32], principal: &AdminPrincipalV1, route: u8, offset: usize) -> Result<String, StatusCode> {
    admin_cursor_encode_scoped(key, principal, route, None, offset)
}

pub fn admin_page_limit(query: &AdminPageQuery) -> Result<usize, StatusCode> {
    match query.limit.unwrap_or(ADMIN_PAGE_MAX) {
        1..=ADMIN_PAGE_MAX => Ok(query.limit.unwrap_or(ADMIN_PAGE_MAX)),
        _ => Err(StatusCode::BAD_REQUEST),
    }
}

pub fn admin_fit_page<T, F>(mut rows: Vec<T>, storage_has_more: bool, observed_at_ms: i64, cursor: F) -> Result<AdminPageV1<T>, StatusCode>
where
    T: Clone + ToValue,
    F: Fn(&[T]) -> Result<String, StatusCode>,
{
    let fetched = rows.len();
    loop {
        let has_more = storage_has_more || rows.len() < fetched;
        if has_more && rows.is_empty() {
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }
        let next_cursor = has_more.then(|| cursor(&rows)).transpose()?;
        let candidate = AdminPageV1 { rows: rows.clone(), next_cursor, observed_at_ms };
        if semio_framework_pack_json::to_json_string(&candidate).len() <= ADMIN_RESPONSE_MAX_BYTES {
            return Ok(AdminPageV1 { rows, next_cursor: candidate.next_cursor, observed_at_ms });
        }
        if rows.len() <= 1 {
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }
        rows.pop();
    }
}

pub fn admin_fit_connection_snapshot(
    mut rows: Vec<AdminRecordedConnectionV1>,
    storage_has_more: bool,
    observed_at_ms: i64,
    head_seq: u64,
    cursor_key: &[u8; 32],
    principal: &AdminPrincipalV1,
    offset: usize,
) -> Result<AdminConnectionSnapshotV1, StatusCode> {
    let fetched = rows.len();
    loop {
        let has_more = storage_has_more || rows.len() < fetched;
        if has_more && rows.is_empty() {
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }
        let next_cursor = has_more.then(|| admin_cursor_encode(cursor_key, principal, 1, offset + rows.len())).transpose()?;
        let candidate = AdminConnectionSnapshotV1 { rows: rows.clone(), next_cursor, observed_at_ms, source: "recorded-sync-sessions".into(), head_seq };
        if semio_framework_pack_json::to_json_string(&candidate).len() <= ADMIN_RESPONSE_MAX_BYTES {
            return Ok(candidate);
        }
        if rows.len() <= 1 {
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }
        rows.pop();
    }
}

pub fn admin_directory_command(intent: &AdminIntentV1) -> Option<DirectoryCommand> {
    Some(match intent {
        AdminIntentV1::CreateSpace { .. } => return None,
        AdminIntentV1::RenameSpace { space_id, name, .. } => DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: name.clone() },
        AdminIntentV1::SetSpaceVisibility { space_id, visibility, .. } => DirectoryCommand::SetVisibility { space_id: space_id.clone(), visibility: *visibility },
        AdminIntentV1::ArchiveSpace { space_id, .. } => DirectoryCommand::ArchiveSpace { space_id: space_id.clone() },
        AdminIntentV1::DeleteSpace { space_id, .. } => DirectoryCommand::DeleteSpace { space_id: space_id.clone() },
        AdminIntentV1::UpsertSpaceMember { space_id, email, role, .. } => DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: email.clone(), role: *role },
        AdminIntentV1::RemoveSpaceMember { space_id, user_id, .. } => DirectoryCommand::RemoveMember { space_id: space_id.clone(), user_id: user_id.clone() },
        AdminIntentV1::CreateSpaceInvite { space_id, role, ttl_secs, .. } => DirectoryCommand::CreateInvite { space_id: space_id.clone(), role: *role, ttl_secs: u64::from(*ttl_secs) },
        AdminIntentV1::RevokeSpaceInvite { space_id, invite_id, .. } => DirectoryCommand::RevokeInvite { space_id: space_id.clone(), invite_id: invite_id.clone() },
        _ => return None,
    })
}

pub fn admin_create_space_id(request_id: &str) -> String {
    let mut hash = Sha256::new();
    hash.update(b"semio/hub/admin-create-space/v1\0");
    hash.update(&(request_id.len() as u32).to_be_bytes());
    hash.update(request_id.as_bytes());
    format!("admin-space:{}", os_directory::io::binary::artifact_hash::hex_lower(&hash.finalize()))
}

pub fn admin_intent_metadata(intent: &AdminIntentV1) -> AdminIntentMetadata {
    match intent {
        AdminIntentV1::CreateSpace { request_id, .. } => AdminIntentMetadata { intent_kind: "create-space", target_kind: "space", target_id: admin_create_space_id(request_id), reason_code: None },
        AdminIntentV1::RenameSpace { space_id, .. } => AdminIntentMetadata { intent_kind: "rename-space", target_kind: "space", target_id: space_id.clone(), reason_code: None },
        AdminIntentV1::SetSpaceVisibility { space_id, .. } => AdminIntentMetadata { intent_kind: "set-space-visibility", target_kind: "space", target_id: space_id.clone(), reason_code: None },
        AdminIntentV1::ArchiveSpace { space_id, .. } => AdminIntentMetadata { intent_kind: "archive-space", target_kind: "space", target_id: space_id.clone(), reason_code: None },
        AdminIntentV1::DeleteSpace { space_id, .. } => AdminIntentMetadata { intent_kind: "delete-space", target_kind: "space", target_id: space_id.clone(), reason_code: None },
        AdminIntentV1::UpsertSpaceMember { space_id, .. } => AdminIntentMetadata { intent_kind: "upsert-space-member", target_kind: "space", target_id: space_id.clone(), reason_code: None },
        AdminIntentV1::RemoveSpaceMember { space_id, .. } => AdminIntentMetadata { intent_kind: "remove-space-member", target_kind: "space", target_id: space_id.clone(), reason_code: None },
        AdminIntentV1::CreateSpaceInvite { space_id, .. } => AdminIntentMetadata { intent_kind: "create-space-invite", target_kind: "space", target_id: space_id.clone(), reason_code: None },
        AdminIntentV1::RevokeSpaceInvite { space_id, .. } => AdminIntentMetadata { intent_kind: "revoke-space-invite", target_kind: "space", target_id: space_id.clone(), reason_code: None },
        AdminIntentV1::IssueDocumentShare { scope, .. } => AdminIntentMetadata { intent_kind: "issue-document-share", target_kind: "document", target_id: format!("{}/{}", scope.space_id, scope.document_id), reason_code: None },
        AdminIntentV1::RevokeDocumentShare { share_id, reason_code, .. } => AdminIntentMetadata { intent_kind: "revoke-document-share", target_kind: "share", target_id: share_id.clone(), reason_code: Some(reason_code.clone()) },
        AdminIntentV1::RevokeUserSessions { user_id, reason_code, .. } => AdminIntentMetadata { intent_kind: "revoke-user-sessions", target_kind: "user", target_id: user_id.clone(), reason_code: Some(reason_code.clone()) },
        AdminIntentV1::KickConnection { sync_session_id, reason_code, .. } => AdminIntentMetadata { intent_kind: "kick-connection", target_kind: "sync-session", target_id: sync_session_id.clone(), reason_code: Some(reason_code.clone()) },
        AdminIntentV1::RebuildDirectoryProjections { .. } => AdminIntentMetadata { intent_kind: "rebuild-directory-projections", target_kind: "directory", target_id: "directory".into(), reason_code: None },
    }
}

pub fn validate_admin_intent(intent: &AdminIntentV1) -> Result<(), StatusCode> {
    semio_hub_directory_contract::validate_bounded_auth_text(intent.request_id(), "admin request id", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
    let metadata = admin_intent_metadata(intent);
    semio_hub_directory_contract::validate_bounded_auth_text(&metadata.target_id, "admin target", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
    if let Some(reason) = metadata.reason_code.as_deref() {
        semio_hub_directory_contract::validate_bounded_auth_text(reason, "admin reason", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
    }
    match intent {
        AdminIntentV1::CreateSpace { name, .. } => {
            semio_hub_directory_contract::validate_bounded_auth_text(name, "admin space name", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
        }
        AdminIntentV1::RenameSpace { space_id, name, .. } => {
            for value in [space_id, name] {
                semio_hub_directory_contract::validate_bounded_auth_text(value, "admin space field", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            }
        }
        AdminIntentV1::SetSpaceVisibility { space_id, .. } | AdminIntentV1::ArchiveSpace { space_id, .. } | AdminIntentV1::DeleteSpace { space_id, .. } => {
            semio_hub_directory_contract::validate_bounded_auth_text(space_id, "admin space", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
        }
        AdminIntentV1::UpsertSpaceMember { space_id, email, .. } => {
            for value in [space_id, email] {
                semio_hub_directory_contract::validate_bounded_auth_text(value, "admin membership field", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            }
        }
        AdminIntentV1::RemoveSpaceMember { space_id, user_id, .. } => {
            for value in [space_id, user_id] {
                semio_hub_directory_contract::validate_bounded_auth_text(value, "admin membership field", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            }
        }
        AdminIntentV1::CreateSpaceInvite { space_id, ttl_secs, .. } => {
            semio_hub_directory_contract::validate_bounded_auth_text(space_id, "admin invite space", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            if *ttl_secs == 0 || i64::from(*ttl_secs) > CAPABILITY_MAX_TTL_SECS {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
        AdminIntentV1::RevokeSpaceInvite { space_id, invite_id, .. } => {
            for value in [space_id, invite_id] {
                semio_hub_directory_contract::validate_bounded_auth_text(value, "admin invite field", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            }
        }
        AdminIntentV1::IssueDocumentShare { scope, ttl_secs, .. } => {
            semio_hub_directory_contract::validate_bounded_auth_text(&scope.space_id, "admin scope space", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            semio_hub_directory_contract::validate_bounded_auth_text(&scope.document_id, "admin scope document", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            if *ttl_secs == 0 || i64::from(*ttl_secs) > CAPABILITY_MAX_TTL_SECS {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
        AdminIntentV1::RevokeDocumentShare { scope, share_id, .. } => {
            for value in [&scope.space_id, &scope.document_id, share_id] {
                semio_hub_directory_contract::validate_bounded_auth_text(value, "admin share scope", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            }
        }
        AdminIntentV1::RevokeUserSessions { user_id, .. } => {
            semio_hub_directory_contract::validate_bounded_auth_text(user_id, "admin user", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
        }
        AdminIntentV1::KickConnection { sync_session_id, .. } => {
            semio_hub_directory_contract::validate_bounded_auth_text(sync_session_id, "admin sync session", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
        }
        AdminIntentV1::RebuildDirectoryProjections { .. } => {}
    }
    Ok(())
}

pub fn admin_intent_digest(intent: &AdminIntentV1) -> String {
    let encoded = semio_framework_pack_json::to_json_string(intent);
    let mut hash = Sha256::new();
    hash.update(b"semio/hub/admin-intent/v1\0");
    hash.update(&(encoded.len() as u32).to_be_bytes());
    hash.update(encoded.as_bytes());
    os_directory::io::binary::artifact_hash::hex_lower(&hash.finalize())
}

pub fn new_admin_audit_fact(principal: &AdminPrincipalV1, request_id: &str, intent_digest: &str, operation_id: &str, metadata: &AdminIntentMetadata, phase: &str, event_range: Option<(u64, u64)>, outcome_code: &str) -> NewAdminOperationAuditRecord {
    NewAdminOperationAuditRecord {
        request_id: request_id.into(),
        intent_digest: intent_digest.into(),
        operation_id: operation_id.into(),
        occurred_at: now_ms(),
        phase: phase.into(),
        intent_kind: metadata.intent_kind.into(),
        target_kind: metadata.target_kind.into(),
        target_id: metadata.target_id.clone(),
        principal_user_id: principal.user_id.clone(),
        principal_session_id: principal.auth_session_id.clone(),
        principal_generation: principal.authorization_generation,
        correlation_id: principal.correlation_id.clone(),
        event_seq_first: event_range.map(|range| range.0),
        event_seq_last: event_range.map(|range| range.1),
        outcome_code: outcome_code.into(),
        reason_code: metadata.reason_code.clone(),
    }
}

pub fn admin_audit_receipt(rows: &[AdminOperationAuditRecord]) -> Option<AdminIntentReceiptV1> {
    let row = rows.iter().find(|row| row.fact.phase != "accepted").or_else(|| rows.first())?;
    let state = match row.fact.phase.as_str() {
        "succeeded" => AdminIntentStateV1::Succeeded,
        "failed" => AdminIntentStateV1::Failed,
        "cancelled" => AdminIntentStateV1::Cancelled,
        _ => AdminIntentStateV1::Accepted,
    };
    Some(AdminIntentReceiptV1 {
        operation_id: row.fact.operation_id.clone(),
        correlation_id: row.fact.correlation_id.clone(),
        state,
        event_seq_first: row.fact.event_seq_first,
        event_seq_last: row.fact.event_seq_last,
        result: None,
        outcome: AdminIntentOutcomeV1 { code: row.fact.outcome_code.clone(), durable: row.fact.intent_kind != "kick-connection" && row.fact.phase == "succeeded", kick_attempted: None, kick_signalled: None },
    })
}

pub fn admin_audit_visibility(mut receipt: AdminIntentReceiptV1, resolver_live: bool) -> AdminIntentReceiptV1 {
    if receipt.state == AdminIntentStateV1::Accepted && !resolver_live {
        receipt.state = AdminIntentStateV1::Indeterminate;
        receipt.outcome = AdminIntentOutcomeV1 { code: "admin-effect-outcome-indeterminate".into(), durable: false, kick_attempted: None, kick_signalled: None };
    }
    receipt
}

pub fn public_admin_audit(row: AdminOperationAuditRecord) -> Result<AdminOperationAuditV1, StatusCode> {
    let phase = match row.fact.phase.as_str() {
        "accepted" => AdminOperationAuditPhaseV1::Accepted,
        "succeeded" => AdminOperationAuditPhaseV1::Succeeded,
        "failed" => AdminOperationAuditPhaseV1::Failed,
        "cancelled" => AdminOperationAuditPhaseV1::Cancelled,
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    Ok(AdminOperationAuditV1 {
        sequence: row.sequence,
        operation_id: row.fact.operation_id,
        occurred_at_ms: row.fact.occurred_at,
        phase,
        intent_kind: row.fact.intent_kind,
        target_kind: row.fact.target_kind,
        target_id: row.fact.target_id,
        principal_user_id: row.fact.principal_user_id,
        principal_session_id: row.fact.principal_session_id,
        principal_generation: row.fact.principal_generation,
        correlation_id: row.fact.correlation_id,
        event_seq_first: row.fact.event_seq_first,
        event_seq_last: row.fact.event_seq_last,
        outcome_code: row.fact.outcome_code,
        reason_code: row.fact.reason_code,
    })
}

pub struct AdminIntentExecution {
    pub phase: &'static str,
    pub event_range: Option<(u64, u64)>,
    pub secret: Option<AdminIntentSecretResult>,
    pub outcome: AdminIntentOutcomeV1,
}

pub enum AdminIntentSecretResult {
    Invite(String),
    Share(String),
}

impl AdminIntentSecretResult {
    fn into_public(mut self) -> AdminIntentResultV1 {
        match &mut self {
            Self::Invite(value) => AdminIntentResultV1 { invite_token: Some(std::mem::take(value)), share_token: None },
            Self::Share(value) => AdminIntentResultV1 { invite_token: None, share_token: Some(std::mem::take(value)) },
        }
    }
}

impl Drop for AdminIntentSecretResult {
    fn drop(&mut self) {
        let value = match self {
            Self::Invite(value) | Self::Share(value) => value,
        };
        let bytes = unsafe { value.as_bytes_mut() };
        for byte in bytes {
            unsafe { std::ptr::write_volatile(byte, 0) };
            #[cfg(any(test, feature = "testkit"))]
            ADMIN_SECRET_WIPE_BYTES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

#[cfg(any(test, feature = "testkit"))]
pub static ADMIN_SECRET_WIPE_BYTES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

pub struct AdminOperationRuntime {
    pub deadline: std::time::Instant,
    pub completed: std::sync::atomic::AtomicU64,
    pub total: std::sync::atomic::AtomicU64,
    pub effect_state: std::sync::atomic::AtomicU8,
    pub cooperative_cancel_requested: std::sync::atomic::AtomicBool,
}

impl AdminOperationRuntime {
    pub fn request_cancel(&self) -> bool {
        match self.effect_state.compare_exchange(ADMIN_EFFECT_PRE_EFFECT, ADMIN_EFFECT_CANCELLED, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire) {
            Ok(_) | Err(ADMIN_EFFECT_CANCELLED) => true,
            Err(_) => {
                self.cooperative_cancel_requested.store(true, std::sync::atomic::Ordering::Release);
                false
            }
        }
    }

    pub fn admit_effect(&self) -> bool {
        self.effect_state.compare_exchange(ADMIN_EFFECT_PRE_EFFECT, ADMIN_EFFECT_ADMITTED, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok()
    }

    pub fn cancelled_before_effect(&self) -> bool {
        self.effect_state.load(std::sync::atomic::Ordering::Acquire) == ADMIN_EFFECT_CANCELLED
    }

    pub fn progress(&self) -> AdminOperationProgressV1 {
        AdminOperationProgressV1 {
            completed_events: self.completed.load(std::sync::atomic::Ordering::Acquire),
            total_events: self.total.load(std::sync::atomic::Ordering::Acquire),
            cancel_requested: self.cancelled_before_effect() || self.cooperative_cancel_requested.load(std::sync::atomic::Ordering::Acquire),
        }
    }
}

impl ProjectionRebuildControl for AdminOperationRuntime {
    fn is_cancelled(&self) -> bool {
        self.cancelled_before_effect() || self.cooperative_cancel_requested.load(std::sync::atomic::Ordering::Acquire) || std::time::Instant::now() >= self.deadline
    }

    fn report(&self, progress: ProjectionRebuildProgress) {
        self.completed.store(progress.completed_events, std::sync::atomic::Ordering::Release);
        self.total.store(progress.total_events, std::sync::atomic::Ordering::Release);
    }
}

pub struct AdminOperationCleanup {
    pub operations: Arc<ShardedMap<String, Arc<AdminOperationRuntime>>>,
    pub operation_id: String,
    pub _permit: tokio::sync::OwnedSemaphorePermit,
}

impl Drop for AdminOperationCleanup {
    fn drop(&mut self) {
        self.operations.remove(&self.operation_id);
    }
}

pub(crate) struct AdminOperationTask {
    pub(crate) task: tokio::task::JoinHandle<()>,
    pub(crate) runtime: Arc<AdminOperationRuntime>,
}

pub(crate) struct AdminOperationTaskOwnerState {
    pub(crate) closing: bool,
    pub(crate) tasks: BTreeMap<String, AdminOperationTask>,
}

pub struct AdminOperationTaskOwner {
    pub state: Mutex<AdminOperationTaskOwnerState>,
    pub(crate) shutdown_deadline: std::time::Duration,
}

impl AdminOperationTaskOwner {
    pub fn new(shutdown_deadline: std::time::Duration) -> Self {
        Self { state: Mutex::new(AdminOperationTaskOwnerState { closing: false, tasks: BTreeMap::new() }), shutdown_deadline }
    }

    pub fn spawn<F>(&self, operation_id: String, runtime: Arc<AdminOperationRuntime>, future: F) -> Result<(), F>
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.tasks.retain(|_, task| !task.task.is_finished());
        if state.closing || state.tasks.contains_key(&operation_id) {
            return Err(future);
        }
        let task = tokio::spawn(future);
        state.tasks.insert(operation_id, AdminOperationTask { task, runtime });
        Ok(())
    }

    pub async fn shutdown(&self) {
        let mut tasks = {
            let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            state.closing = true;
            for task in state.tasks.values() {
                task.runtime.request_cancel();
            }
            std::mem::take(&mut state.tasks).into_values().map(|task| task.task).collect::<Vec<_>>()
        };
        if tokio::time::timeout(self.shutdown_deadline, futures::future::join_all(tasks.iter_mut())).await.is_err() {
            for task in &tasks {
                task.abort();
            }
            for task in tasks {
                let _ = task.await;
            }
        }
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn task_count(&self) -> usize {
        self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).tasks.values().filter(|task| !task.task.is_finished()).count()
    }
}

/// 🗝️ Derives one sorted authority union from the closed short intent before any gate is acquired.
pub fn admin_intent_bindings(principal: &AdminPrincipalV1, intent: &AdminIntentV1) -> Option<Vec<SocketBindingKeyV1>> {
    let mut bindings = vec![SocketBindingKeyV1::User(principal.user_id.clone()), SocketBindingKeyV1::Session(principal.auth_session_id.clone())];
    match intent {
        AdminIntentV1::CreateSpace { request_id, .. } => bindings.push(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: admin_create_space_id(request_id) }),
        AdminIntentV1::RenameSpace { space_id, .. }
        | AdminIntentV1::SetSpaceVisibility { space_id, .. }
        | AdminIntentV1::ArchiveSpace { space_id, .. }
        | AdminIntentV1::DeleteSpace { space_id, .. }
        | AdminIntentV1::UpsertSpaceMember { space_id, .. }
        | AdminIntentV1::CreateSpaceInvite { space_id, .. }
        | AdminIntentV1::RevokeSpaceInvite { space_id, .. } => bindings.push(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space_id.clone() }),
        AdminIntentV1::RemoveSpaceMember { space_id, user_id, .. } => {
            bindings.push(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space_id.clone() });
            bindings.push(SocketBindingKeyV1::Membership { user_id: user_id.clone(), space_id: space_id.clone() });
        }
        AdminIntentV1::IssueDocumentShare { scope, .. } => bindings.push(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: scope.space_id.clone() }),
        AdminIntentV1::RevokeDocumentShare { scope, share_id, .. } => {
            bindings.push(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: scope.space_id.clone() });
            bindings.push(SocketBindingKeyV1::Share(share_id.clone()));
        }
        AdminIntentV1::RevokeUserSessions { user_id, .. } => bindings.push(SocketBindingKeyV1::User(user_id.clone())),
        AdminIntentV1::KickConnection { .. } => {}
        AdminIntentV1::RebuildDirectoryProjections { .. } => return None,
    }
    bindings.sort_unstable();
    bindings.dedup();
    Some(bindings)
}

/// 🪞️ Whether an administrator intent changes the administrator's own principal — revoking its own
/// user's sessions — so its own user binding is held exclusive rather than shared.
pub fn admin_intent_changes_own_principal(principal: &AdminPrincipalV1, intent: &AdminIntentV1) -> bool {
    matches!(intent, AdminIntentV1::RevokeUserSessions { user_id, .. } if *user_id == principal.user_id)
}

pub fn admin_directory_authority_refusal(error: FencedDirectoryCommandErrorV1) -> AdminIntentExecution {
    let code = if matches!(error, FencedDirectoryCommandErrorV1::Denied(_)) { "admin-authority-changed" } else { "admin-authority-unavailable" };
    AdminIntentExecution { phase: "cancelled", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: code.into(), durable: false, kick_attempted: None, kick_signalled: None } }
}

pub fn admin_effect_receipt_claim(operation_id: &str, intent_digest: &str, outcome_code: &str) -> NewAdminOperationEffectReceiptV1 {
    NewAdminOperationEffectReceiptV1 { operation_id: operation_id.into(), intent_digest: intent_digest.into(), committed_at: now_ms(), outcome_code: outcome_code.into() }
}

pub fn admin_effect_uncertain() -> AdminIntentExecution {
    AdminIntentExecution { phase: "uncertain", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "admin-effect-outcome-uncertain".into(), durable: false, kick_attempted: None, kick_signalled: None } }
}

pub fn admin_effect_rejected_before_commit() -> AdminIntentExecution {
    AdminIntentExecution { phase: "failed", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "admin-effect-rejected-before-commit".into(), durable: false, kick_attempted: None, kick_signalled: None } }
}

pub fn admin_execution_receipt(operation_id: String, correlation_id: String, execution: AdminIntentExecution) -> AdminIntentReceiptV1 {
    AdminIntentReceiptV1 {
        operation_id,
        correlation_id,
        state: match execution.phase {
            "succeeded" => AdminIntentStateV1::Succeeded,
            "cancelled" => AdminIntentStateV1::Cancelled,
            _ => AdminIntentStateV1::Failed,
        },
        event_seq_first: execution.event_range.map(|range| range.0),
        event_seq_last: execution.event_range.map(|range| range.1),
        result: execution.secret.map(AdminIntentSecretResult::into_public),
        outcome: execution.outcome,
    }
}

/// 📝️ What `GET /admin/api/observability` answers ([`HubObservabilityV1`]): the per-event table one
/// tracer has accumulated since boot, the per-route answer table, the compiled-guest residency and the DB I/O
/// census.
///
/// Counters, never records. Replaying the record stream over HTTP would put one tenant's
/// principals, space ids and artifact ids behind another operator's single admin capability, and it
/// is not what the question "what is failing, how often, and how slow is it" needs — the tables
/// answer that, and the records themselves go to the sink the operator configured, which is where
/// their retention and access policy already lives. Routes are keyed by their template, never a
/// concrete path, for the same reason.
///
/// `declaredEvents` ships the vocabulary alongside the live rows so a dashboard can show a declared
/// event that has not fired yet as a zero rather than as a missing series, and `droppedEvents` is
/// the bounded table's own overflow count — non-zero means some event name is being built from
/// request data, which is itself the thing to alert on.
pub fn observability_view(tracer: &Tracer, routes: &HubRouteMetricsV1, catalog: Option<&VerifiedTrustedCatalog>) -> HubObservabilityV1 {
    HubObservabilityV1::read(tracer, routes, catalog, HUB_PROCESS_START.elapsed().as_millis().min(u128::from(u64::MAX)) as u64)
}

pub fn admin_space_summary_view(summary: semio_hub_directory_contract::model::AdminSpaceSummaryRecord) -> Result<SpaceView, StatusCode> {
    let kind = match summary.space.kind.as_str() {
        "atelier" => os_directory::DirectorySpaceKind::Atelier,
        "studio" => os_directory::DirectorySpaceKind::Studio,
        "archive" => os_directory::DirectorySpaceKind::Archive,
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    let visibility = match summary.space.visibility.as_str() {
        "private" => DirectorySpaceVisibility::Private,
        "public" => DirectorySpaceVisibility::Public,
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    Ok(SpaceView {
        id: summary.space.id,
        name: summary.space.name,
        kind,
        visibility,
        owner_user_id: summary.space.owner_user_id,
        role: None,
        member_count: u32::try_from(summary.member_count).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        document_count: u32::try_from(summary.document_count).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        active_connections: u32::try_from(summary.active_connections).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        created_at_ms: summary.space.created_at,
        updated_at_ms: summary.updated_at,
    })
}

pub struct AdminSpaceDetailResponse {
    pub(crate) view: SpaceView,
    pub members: AdminPageV1<MemberView>,
}

impl ToValue for AdminSpaceDetailResponse {
    fn to_value(&self) -> DslValue {
        let mut entries = match self.view.to_value() {
            DslValue::Object(entries) => entries,
            other => vec![("space".into(), other)],
        };
        entries.push(("members".into(), self.members.to_value()));
        DslValue::Object(entries)
    }
}

pub fn admin_fit_space_detail(view: SpaceView, mut rows: Vec<MemberView>, storage_has_more: bool, observed_at_ms: i64, cursor_key: &[u8; 32], principal: &AdminPrincipalV1, space_id: &str, offset: usize) -> Result<AdminSpaceDetailResponse, StatusCode> {
    let fetched = rows.len();
    loop {
        let has_more = storage_has_more || rows.len() < fetched;
        if has_more && rows.is_empty() {
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }
        let next_cursor = has_more.then(|| admin_cursor_encode_scoped(cursor_key, principal, 7, Some(space_id), offset + rows.len())).transpose()?;
        let response = AdminSpaceDetailResponse { view: view.clone(), members: AdminPageV1 { rows: rows.clone(), next_cursor, observed_at_ms } };
        if semio_framework_pack_json::to_json_string(&response).len() <= ADMIN_RESPONSE_MAX_BYTES {
            return Ok(response);
        }
        if rows.len() <= 1 {
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }
        rows.pop();
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentsQuery {
    pub space: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<usize>,
}
//#endregion 🔖️Admin
