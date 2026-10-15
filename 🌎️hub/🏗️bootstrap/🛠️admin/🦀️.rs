//! 🛠️ `admin` — administrator API: intents, operations, overview and listings.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use axum::body::Bytes;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use directory::{os_directory, DslValue, ToValue};
use directory::os_directory::{AdminConnectionSnapshotV1, AdminIntentOutcomeV1, AdminIntentReceiptV1, AdminIntentResultV1, AdminIntentStateV1, AdminIntentV1, AdminOperationAuditPhaseV1, AdminOperationAuditV1, AdminOperationProgressV1, AdminOperationStatusV1, AdminPageV1, AdminRecordedConnectionV1, DirectoryCommand, DirectoryEvent, DirectorySpaceRole, DirectorySpaceVisibility, DocumentView, MemberView, SpaceView};
use futures::{SinkExt, StreamExt};
use semio_framework_async::ShardedMap;
use semio_framework_hash::Sha256;
use semio_framework_trace::record::Tracer;
use semio_hub::artifact_authority::chunk_cas::ArtifactChunkCasStorage;
use semio_hub::artifact_authority::creation::ArtifactCreationCommitAuthorityV1;
use semio_hub::artifact_authority::trusted_catalog::VerifiedTrustedCatalog;
use semio_hub::observability::{HubObservabilityV1, HubRouteMetricsV1};
use semio_hub::artifact_authority::AuthorityOperationControl;
use semio_hub::directory::model::{AdminEffectCommitV1, AdminOperationAuditRecord, DocumentScope, NewAdminOperationAuditRecord, NewAdminOperationEffectReceiptV1, SocketSessionBindingStatus, SpaceRole};
use semio_hub::directory::{HubDirectory, ProjectionRebuildControl, ProjectionRebuildProgress, ACTIVE_SYNC_SESSION_READ_MAX, ADMIN_INTENT_REQUEST_MAX_BYTES, ADMIN_PAGE_MAX, ADMIN_RESPONSE_MAX_BYTES, CAPABILITY_MAX_TTL_SECS, IdentityVerificationControl, AUTH_TEXT_MAX_BYTES};
use semio_hub::inference::runtime::GisMapApprovalIngressAuthorityV1;
use semio_hub::lag_rebootstrap::RebootstrapTransferControl;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use crate::support::{DirectoryJson, now_ms, directory_error_status};
use crate::state::HubState;
use crate::socket_grants::{SocketBindingKeyV1, SocketBindingModeV1, SocketBindingGuardV1};
use crate::guard::{AdminPrincipalV1, admin_provider_digest, authenticate_admin_principal};
use crate::readiness::HUB_PROCESS_START;
#[cfg(test)]
use crate::access_grants::{pause_directory_command_authority, pause_directory_command_membership_fence, pause_admin_effect_started};
use crate::directory_commands::{role_wire, document_view, authorize_directory_command, FencedDirectoryCommandErrorV1, invalidate_directory_event_authority};
#[cfg(not(test))]
use crate::directory_commands::{pause_directory_command_authority, pause_directory_command_membership_fence, pause_admin_effect_started};
use db::db_storage::PayloadStorage as _;
use server::storage::SessionStore as _;

//#region 🔖️State
pub(crate) const ADMIN_OPERATION_DEADLINE: std::time::Duration = std::time::Duration::from_secs(10);

pub(crate) const ADMIN_OPERATION_SHUTDOWN_DEADLINE: std::time::Duration = std::time::Duration::from_secs(11);

const ADMIN_TERMINAL_RETRY_DEADLINE: std::time::Duration = std::time::Duration::from_secs(2);

pub(crate) const ADMIN_EFFECT_PRE_EFFECT: u8 = 0;

pub(crate) const ADMIN_EFFECT_ADMITTED: u8 = 1;

const ADMIN_EFFECT_CANCELLED: u8 = 2;
//#endregion 🔖️State

//#region 🔖️Admin
/// 🧮️ Best-effort recursive directory size in bytes — used only for the admin overview's
/// `dataDirBytes`; any unreadable entry is silently skipped rather than failing the whole overview.
fn dir_size(path: &std::path::Path) -> u64 {
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
pub(crate) struct AdminIntentMetadata {
    pub(crate) intent_kind: &'static str,
    pub(crate) target_kind: &'static str,
    pub(crate) target_id: String,
    pub(crate) reason_code: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AdminPageQuery {
    pub(crate) cursor: Option<String>,
    pub(crate) limit: Option<usize>,
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

pub(crate) fn admin_cursor_decode_scoped(key: &[u8; 32], principal: &AdminPrincipalV1, route: u8, scope: Option<&str>, encoded: Option<&str>) -> Result<usize, StatusCode> {
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
    if !semio_hub::directory::constant_time_digest_eq(&admin_cursor_mac(key, principal, &payload, scope), &supplied) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let offset = u64::from_be_bytes(payload[2..].try_into().map_err(|_| StatusCode::BAD_REQUEST)?);
    usize::try_from(offset).map_err(|_| StatusCode::BAD_REQUEST)
}

pub(crate) fn admin_cursor_decode(key: &[u8; 32], principal: &AdminPrincipalV1, route: u8, encoded: Option<&str>) -> Result<usize, StatusCode> {
    admin_cursor_decode_scoped(key, principal, route, None, encoded)
}

pub(crate) fn admin_cursor_encode_scoped(key: &[u8; 32], principal: &AdminPrincipalV1, route: u8, scope: Option<&str>, offset: usize) -> Result<String, StatusCode> {
    let mut payload = [0u8; 10];
    payload[0] = 1;
    payload[1] = route;
    payload[2..].copy_from_slice(&u64::try_from(offset).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.to_be_bytes());
    let mut raw = [0u8; 42];
    raw[..10].copy_from_slice(&payload);
    raw[10..].copy_from_slice(&admin_cursor_mac(key, principal, &payload, scope));
    Ok(os_directory::io::binary::artifact_hash::hex_lower(&raw))
}

pub(crate) fn admin_cursor_encode(key: &[u8; 32], principal: &AdminPrincipalV1, route: u8, offset: usize) -> Result<String, StatusCode> {
    admin_cursor_encode_scoped(key, principal, route, None, offset)
}

pub(crate) fn admin_page_limit(query: &AdminPageQuery) -> Result<usize, StatusCode> {
    match query.limit.unwrap_or(ADMIN_PAGE_MAX) {
        1..=ADMIN_PAGE_MAX => Ok(query.limit.unwrap_or(ADMIN_PAGE_MAX)),
        _ => Err(StatusCode::BAD_REQUEST),
    }
}

pub(crate) fn admin_fit_page<T, F>(mut rows: Vec<T>, storage_has_more: bool, observed_at_ms: i64, cursor: F) -> Result<AdminPageV1<T>, StatusCode>
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

pub(crate) fn admin_fit_connection_snapshot(
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

fn admin_directory_command(intent: &AdminIntentV1) -> Option<DirectoryCommand> {
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

pub(crate) fn admin_create_space_id(request_id: &str) -> String {
    let mut hash = Sha256::new();
    hash.update(b"semio/hub/admin-create-space/v1\0");
    hash.update(&(request_id.len() as u32).to_be_bytes());
    hash.update(request_id.as_bytes());
    format!("admin-space:{}", os_directory::io::binary::artifact_hash::hex_lower(&hash.finalize()))
}

fn admin_intent_metadata(intent: &AdminIntentV1) -> AdminIntentMetadata {
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

fn validate_admin_intent(intent: &AdminIntentV1) -> Result<(), StatusCode> {
    semio_hub::directory::validate_bounded_auth_text(intent.request_id(), "admin request id", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
    let metadata = admin_intent_metadata(intent);
    semio_hub::directory::validate_bounded_auth_text(&metadata.target_id, "admin target", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
    if let Some(reason) = metadata.reason_code.as_deref() {
        semio_hub::directory::validate_bounded_auth_text(reason, "admin reason", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
    }
    match intent {
        AdminIntentV1::CreateSpace { name, .. } => {
            semio_hub::directory::validate_bounded_auth_text(name, "admin space name", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
        }
        AdminIntentV1::RenameSpace { space_id, name, .. } => {
            for value in [space_id, name] {
                semio_hub::directory::validate_bounded_auth_text(value, "admin space field", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            }
        }
        AdminIntentV1::SetSpaceVisibility { space_id, .. } | AdminIntentV1::ArchiveSpace { space_id, .. } | AdminIntentV1::DeleteSpace { space_id, .. } => {
            semio_hub::directory::validate_bounded_auth_text(space_id, "admin space", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
        }
        AdminIntentV1::UpsertSpaceMember { space_id, email, .. } => {
            for value in [space_id, email] {
                semio_hub::directory::validate_bounded_auth_text(value, "admin membership field", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            }
        }
        AdminIntentV1::RemoveSpaceMember { space_id, user_id, .. } => {
            for value in [space_id, user_id] {
                semio_hub::directory::validate_bounded_auth_text(value, "admin membership field", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            }
        }
        AdminIntentV1::CreateSpaceInvite { space_id, ttl_secs, .. } => {
            semio_hub::directory::validate_bounded_auth_text(space_id, "admin invite space", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            if *ttl_secs == 0 || i64::from(*ttl_secs) > CAPABILITY_MAX_TTL_SECS {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
        AdminIntentV1::RevokeSpaceInvite { space_id, invite_id, .. } => {
            for value in [space_id, invite_id] {
                semio_hub::directory::validate_bounded_auth_text(value, "admin invite field", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            }
        }
        AdminIntentV1::IssueDocumentShare { scope, ttl_secs, .. } => {
            semio_hub::directory::validate_bounded_auth_text(&scope.space_id, "admin scope space", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            semio_hub::directory::validate_bounded_auth_text(&scope.document_id, "admin scope document", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            if *ttl_secs == 0 || i64::from(*ttl_secs) > CAPABILITY_MAX_TTL_SECS {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
        AdminIntentV1::RevokeDocumentShare { scope, share_id, .. } => {
            for value in [&scope.space_id, &scope.document_id, share_id] {
                semio_hub::directory::validate_bounded_auth_text(value, "admin share scope", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
            }
        }
        AdminIntentV1::RevokeUserSessions { user_id, .. } => {
            semio_hub::directory::validate_bounded_auth_text(user_id, "admin user", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
        }
        AdminIntentV1::KickConnection { sync_session_id, .. } => {
            semio_hub::directory::validate_bounded_auth_text(sync_session_id, "admin sync session", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
        }
        AdminIntentV1::RebuildDirectoryProjections { .. } => {}
    }
    Ok(())
}

pub(crate) fn admin_intent_digest(intent: &AdminIntentV1) -> String {
    let encoded = semio_framework_pack_json::to_json_string(intent);
    let mut hash = Sha256::new();
    hash.update(b"semio/hub/admin-intent/v1\0");
    hash.update(&(encoded.len() as u32).to_be_bytes());
    hash.update(encoded.as_bytes());
    os_directory::io::binary::artifact_hash::hex_lower(&hash.finalize())
}

pub(crate) fn new_admin_audit_fact(principal: &AdminPrincipalV1, request_id: &str, intent_digest: &str, operation_id: &str, metadata: &AdminIntentMetadata, phase: &str, event_range: Option<(u64, u64)>, outcome_code: &str) -> NewAdminOperationAuditRecord {
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

pub(crate) fn admin_audit_receipt(rows: &[AdminOperationAuditRecord]) -> Option<AdminIntentReceiptV1> {
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

pub(crate) fn admin_audit_visibility(mut receipt: AdminIntentReceiptV1, resolver_live: bool) -> AdminIntentReceiptV1 {
    if receipt.state == AdminIntentStateV1::Accepted && !resolver_live {
        receipt.state = AdminIntentStateV1::Indeterminate;
        receipt.outcome = AdminIntentOutcomeV1 { code: "admin-effect-outcome-indeterminate".into(), durable: false, kick_attempted: None, kick_signalled: None };
    }
    receipt
}

fn public_admin_audit(row: AdminOperationAuditRecord) -> Result<AdminOperationAuditV1, StatusCode> {
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

pub(crate) async fn reconcile_stale_admin_acceptance(state: &HubState, rows: Vec<AdminOperationAuditRecord>) -> Result<Vec<AdminOperationAuditRecord>, StatusCode> {
    if rows.iter().any(|row| row.fact.phase != "accepted") {
        return Ok(rows);
    }
    let Some(accepted) = rows.iter().find(|row| row.fact.phase == "accepted") else {
        return Ok(rows);
    };
    let Some(effect) = state.directory.admin_operation_effect_receipt(&accepted.fact.operation_id, &accepted.fact.intent_digest).await.map_err(directory_error_status)? else {
        return Ok(rows);
    };
    let expected_outcome = match accepted.fact.intent_kind.as_str() {
        "create-space" | "rename-space" | "set-space-visibility" | "archive-space" | "delete-space" | "upsert-space-member" | "remove-space-member" | "create-space-invite" | "revoke-space-invite" => "directory-events-appended",
        "issue-document-share" => "share-issued",
        "revoke-document-share" => "share-revoked",
        "revoke-user-sessions" => "sessions-revoked",
        _ => return Ok(rows),
    };
    if effect.outcome_code != expected_outcome {
        return Ok(rows);
    }
    match (accepted.fact.intent_kind.as_str(), accepted.fact.target_kind.as_str()) {
        ("revoke-document-share", "share") => {
            let binding = SocketBindingKeyV1::Share(accepted.fact.target_id.clone());
            state.socket_grants.invalidate_binding(binding.clone());
            state.document_open_plans.invalidate_binding(&binding);
        }
        ("revoke-user-sessions", "user") => {
            let binding = SocketBindingKeyV1::User(accepted.fact.target_id.clone());
            state.socket_grants.invalidate_binding(binding.clone());
            state.document_open_plans.invalidate_binding(&binding);
        }
        _ => {}
    }
    let mut terminal = accepted.fact.clone();
    terminal.occurred_at = effect.committed_at;
    terminal.phase = "succeeded".into();
    terminal.event_seq_first = effect.event_seq_first;
    terminal.event_seq_last = effect.event_seq_last;
    terminal.outcome_code = effect.outcome_code;
    if !append_admin_terminal_with_retry(state, &terminal).await {
        return Ok(rows);
    }
    state.directory.admin_operation_audit_for_operation(&accepted.fact.operation_id).await.map_err(directory_error_status)
}

async fn append_admin_terminal_with_retry(state: &HubState, terminal: &NewAdminOperationAuditRecord) -> bool {
    let deadline = tokio::time::Instant::now() + ADMIN_TERMINAL_RETRY_DEADLINE;
    loop {
        if state.directory.append_admin_operation_audit(terminal).await.is_ok() {
            return true;
        }
        if tokio::time::Instant::now() >= deadline {
            return false;
        }
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
}

pub(crate) async fn admin_operation(Path(operation_id): Path<String>, headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<DirectoryJson<AdminOperationStatusV1>, StatusCode> {
    authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    semio_hub::directory::validate_bounded_auth_text(&operation_id, "admin operation id", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
    let rows = state.directory.admin_operation_audit_for_operation(&operation_id).await.map_err(directory_error_status)?;
    let rows = reconcile_stale_admin_acceptance(&state, rows).await?;
    let receipt = admin_audit_receipt(&rows).ok_or(StatusCode::NOT_FOUND)?;
    let progress = state.admin_operations.with(&receipt.operation_id, |runtime| runtime.map(|runtime| runtime.progress()));
    let receipt = admin_audit_visibility(receipt, progress.is_some());
    Ok(DirectoryJson(AdminOperationStatusV1 { receipt, progress }))
}

pub(crate) async fn cancel_admin_operation(
    Path(operation_id): Path<String>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<DirectoryJson<AdminOperationStatusV1>, StatusCode> {
    authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    semio_hub::directory::validate_bounded_auth_text(&operation_id, "admin operation id", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
    let rows = state.directory.admin_operation_audit_for_operation(&operation_id).await.map_err(directory_error_status)?;
    let receipt = admin_audit_receipt(&rows).ok_or(StatusCode::NOT_FOUND)?;
    let runtime = state.admin_operations.get_cloned(&receipt.operation_id).ok_or(StatusCode::CONFLICT)?;
    runtime.request_cancel();
    Ok(DirectoryJson(AdminOperationStatusV1 { receipt, progress: Some(runtime.progress()) }))
}

pub(crate) async fn admin_operation_audit(
    Query(query): Query<AdminPageQuery>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<DirectoryJson<AdminPageV1<AdminOperationAuditV1>>, StatusCode> {
    let principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    let limit = admin_page_limit(&query)?;
    let after = admin_cursor_decode(&state.admin_cursor_key, &principal, 4, query.cursor.as_deref())?;
    let mut rows = state.directory.list_admin_operation_audit(u64::try_from(after).map_err(|_| StatusCode::BAD_REQUEST)?, limit.saturating_add(1)).await.map_err(directory_error_status)?;
    let storage_has_more = rows.len() > limit;
    rows.truncate(limit);
    let rows = rows.into_iter().map(public_admin_audit).collect::<Result<Vec<_>, _>>()?;
    let page = admin_fit_page(rows, storage_has_more, now_ms(), |rows| {
        let next = rows.last().map(|row| usize::try_from(row.sequence).map_err(|_| StatusCode::BAD_REQUEST)).transpose()?.unwrap_or(after);
        admin_cursor_encode(&state.admin_cursor_key, &principal, 4, next)
    })?;
    Ok(DirectoryJson(page))
}

pub(crate) struct AdminIntentExecution {
    pub(crate) phase: &'static str,
    event_range: Option<(u64, u64)>,
    pub(crate) secret: Option<AdminIntentSecretResult>,
    pub(crate) outcome: AdminIntentOutcomeV1,
}

pub(crate) enum AdminIntentSecretResult {
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
            #[cfg(test)]
            ADMIN_SECRET_WIPE_BYTES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

#[cfg(test)]
pub(crate) static ADMIN_SECRET_WIPE_BYTES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

pub(crate) struct AdminOperationRuntime {
    pub(crate) deadline: std::time::Instant,
    pub(crate) completed: std::sync::atomic::AtomicU64,
    pub(crate) total: std::sync::atomic::AtomicU64,
    pub(crate) effect_state: std::sync::atomic::AtomicU8,
    pub(crate) cooperative_cancel_requested: std::sync::atomic::AtomicBool,
}

impl AdminOperationRuntime {
    pub(crate) fn request_cancel(&self) -> bool {
        match self.effect_state.compare_exchange(ADMIN_EFFECT_PRE_EFFECT, ADMIN_EFFECT_CANCELLED, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire) {
            Ok(_) | Err(ADMIN_EFFECT_CANCELLED) => true,
            Err(_) => {
                self.cooperative_cancel_requested.store(true, std::sync::atomic::Ordering::Release);
                false
            }
        }
    }

    pub(crate) fn admit_effect(&self) -> bool {
        self.effect_state.compare_exchange(ADMIN_EFFECT_PRE_EFFECT, ADMIN_EFFECT_ADMITTED, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok()
    }

    fn cancelled_before_effect(&self) -> bool {
        self.effect_state.load(std::sync::atomic::Ordering::Acquire) == ADMIN_EFFECT_CANCELLED
    }

    pub(crate) fn progress(&self) -> AdminOperationProgressV1 {
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

pub(crate) struct AdminOperationCleanup {
    pub(crate) operations: Arc<ShardedMap<String, Arc<AdminOperationRuntime>>>,
    pub(crate) operation_id: String,
    pub(crate) _permit: tokio::sync::OwnedSemaphorePermit,
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

pub(crate) struct AdminOperationTaskOwner {
    pub(crate) state: Mutex<AdminOperationTaskOwnerState>,
    pub(crate) shutdown_deadline: std::time::Duration,
}

impl AdminOperationTaskOwner {
    pub(crate) fn new(shutdown_deadline: std::time::Duration) -> Self {
        Self { state: Mutex::new(AdminOperationTaskOwnerState { closing: false, tasks: BTreeMap::new() }), shutdown_deadline }
    }

    pub(crate) fn spawn<F>(&self, operation_id: String, runtime: Arc<AdminOperationRuntime>, future: F) -> Result<(), F>
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

    pub(crate) async fn shutdown(&self) {
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

    #[cfg(test)]
    pub(crate) fn task_count(&self) -> usize {
        self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).tasks.values().filter(|task| !task.task.is_finished()).count()
    }
}

/// 🗝️ Derives one sorted authority union from the closed short intent before any gate is acquired.
pub(crate) fn admin_intent_bindings(principal: &AdminPrincipalV1, intent: &AdminIntentV1) -> Option<Vec<SocketBindingKeyV1>> {
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
fn admin_intent_changes_own_principal(principal: &AdminPrincipalV1, intent: &AdminIntentV1) -> bool {
    matches!(intent, AdminIntentV1::RevokeUserSessions { user_id, .. } if *user_id == principal.user_id)
}

/// 🏛️ A configured administrator retains its exact durable principal through a short side effect.
async fn acquire_admin_intent_authority(state: &HubState, principal: &AdminPrincipalV1, intent: &AdminIntentV1, bindings: Vec<SocketBindingKeyV1>) -> Result<Vec<SocketBindingGuardV1>, FencedDirectoryCommandErrorV1> {
    let own = [SocketBindingKeyV1::User(principal.user_id.clone()), SocketBindingKeyV1::Session(principal.auth_session_id.clone())];
    let modes = bindings.into_iter().map(|binding| {
        let mode = if own.contains(&binding) && !admin_intent_changes_own_principal(principal, intent) { SocketBindingModeV1::Shared } else { SocketBindingModeV1::Exclusive };
        (binding, mode)
    });
    let admission = tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.acquire(modes.collect())).await.map_err(|_| FencedDirectoryCommandErrorV1::Unavailable)?;
    let provider_digest = admin_provider_digest(&principal.identity_provider);
    if principal.expires_at_ms <= now_ms()
        || !state
            .admin_subjects
            .iter()
            .any(|subject| semio_hub::directory::constant_time_digest_eq(&subject.provider_digest, &provider_digest) && semio_hub::directory::constant_time_digest_eq(&subject.subject_digest, &principal.identity_subject_digest))
    {
        return Err(FencedDirectoryCommandErrorV1::Denied(StatusCode::UNAUTHORIZED));
    }
    let binding = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.socket_session_binding(&principal.auth_session_id, &principal.user_id, principal.authorization_generation, None, now_ms())).await;
    match binding {
        Ok(Ok(SocketSessionBindingStatus::Active { role: None, expires_at_ms, .. })) if expires_at_ms == principal.expires_at_ms => {}
        Ok(Ok(SocketSessionBindingStatus::Unavailable)) | Ok(Err(_)) | Err(_) => return Err(FencedDirectoryCommandErrorV1::Unavailable),
        _ => return Err(FencedDirectoryCommandErrorV1::Denied(StatusCode::UNAUTHORIZED)),
    }
    pause_directory_command_authority(state, &principal.user_id, true).await;
    Ok(admission)
}

fn admin_directory_authority_refusal(error: FencedDirectoryCommandErrorV1) -> AdminIntentExecution {
    let code = if matches!(error, FencedDirectoryCommandErrorV1::Denied(_)) { "admin-authority-changed" } else { "admin-authority-unavailable" };
    AdminIntentExecution { phase: "cancelled", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: code.into(), durable: false, kick_attempted: None, kick_signalled: None } }
}

pub(crate) fn admin_effect_receipt_claim(operation_id: &str, intent_digest: &str, outcome_code: &str) -> NewAdminOperationEffectReceiptV1 {
    NewAdminOperationEffectReceiptV1 { operation_id: operation_id.into(), intent_digest: intent_digest.into(), committed_at: now_ms(), outcome_code: outcome_code.into() }
}

fn admin_effect_uncertain() -> AdminIntentExecution {
    AdminIntentExecution { phase: "uncertain", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "admin-effect-outcome-uncertain".into(), durable: false, kick_attempted: None, kick_signalled: None } }
}

fn admin_effect_rejected_before_commit() -> AdminIntentExecution {
    AdminIntentExecution { phase: "failed", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "admin-effect-rejected-before-commit".into(), durable: false, kick_attempted: None, kick_signalled: None } }
}

pub(crate) async fn execute_admin_intent(
    state: &HubState,
    principal: &AdminPrincipalV1,
    operation_id: &str,
    intent_digest: &str,
    intent: AdminIntentV1,
    operation_runtime: Option<Arc<AdminOperationRuntime>>,
    authority_owner: &mut Option<Vec<SocketBindingGuardV1>>,
) -> AdminIntentExecution {
    *authority_owner = None;
    if operation_runtime.as_ref().is_some_and(|runtime| runtime.cancelled_before_effect()) {
        return AdminIntentExecution { phase: "cancelled", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "admin-operation-cancelled-before-effect".into(), durable: false, kick_attempted: None, kick_signalled: None } };
    }
    *authority_owner = if let Some(bindings) = admin_intent_bindings(principal, &intent) {
        pause_directory_command_authority(state, &principal.user_id, false).await;
        if operation_runtime.as_ref().is_some_and(|runtime| runtime.cancelled_before_effect()) {
            return AdminIntentExecution { phase: "cancelled", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "admin-operation-cancelled-before-effect".into(), durable: false, kick_attempted: None, kick_signalled: None } };
        }
        match acquire_admin_intent_authority(state, principal, &intent, bindings).await {
            Ok(authority) => Some(authority),
            Err(error) => return admin_directory_authority_refusal(error),
        }
    } else {
        None
    };
    #[cfg(test)]
    if matches!(&intent, AdminIntentV1::RevokeUserSessions { .. }) {
        if let Some(gate) = &state.live_gate {
            if gate.socket_admin_revoke_pause_enabled.load(std::sync::atomic::Ordering::Acquire) {
                gate.socket_admin_revoke_admitted.add_permits(1);
                gate.socket_admin_revoke_release.acquire().await.expect("admin revoke test release").forget();
            }
        }
    }
    if operation_runtime.as_ref().is_some_and(|runtime| !runtime.admit_effect()) {
        return AdminIntentExecution { phase: "cancelled", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "admin-operation-cancelled-before-effect".into(), durable: false, kick_attempted: None, kick_signalled: None } };
    }
    pause_admin_effect_started(state).await;
    if let AdminIntentV1::CreateSpace { request_id, name, space_kind, visibility } = &intent {
        let target_id = admin_create_space_id(request_id);
        let effect = admin_effect_receipt_claim(operation_id, intent_digest, "directory-events-appended");
        return match state.directory_service.execute_create_space_with_id_and_admin_effect(principal.event_actor(), target_id, name.clone(), *space_kind, *visibility, &effect).await {
            AdminEffectCommitV1::Applied(events) => AdminIntentExecution {
                phase: "succeeded",
                event_range: events.first().zip(events.last()).map(|(first, last)| (first.seq, last.seq)),
                secret: None,
                outcome: AdminIntentOutcomeV1 { code: "directory-events-appended".into(), durable: true, kick_attempted: None, kick_signalled: None },
            },
            AdminEffectCommitV1::RejectedBeforeCommit => admin_effect_rejected_before_commit(),
            AdminEffectCommitV1::Indeterminate => admin_effect_uncertain(),
        };
    }
    if let Some(command) = admin_directory_command(&intent) {
        if authorize_directory_command(state, principal.principal(), true, &command).await.is_err() {
            return AdminIntentExecution { phase: "failed", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "directory-command-denied".into(), durable: false, kick_attempted: None, kick_signalled: None } };
        }
        if matches!(&command, DirectoryCommand::RemoveMember { .. }) {
            pause_directory_command_membership_fence(state).await;
        }
        let effect = admin_effect_receipt_claim(operation_id, intent_digest, "directory-events-appended");
        return match state.directory_service.execute_with_admin_effect(principal.event_actor(), command, &effect).await {
            AdminEffectCommitV1::Applied((events, result)) => {
                invalidate_directory_event_authority(state, &events);
                AdminIntentExecution {
                    phase: "succeeded",
                    event_range: events.first().zip(events.last()).map(|(first, last)| (first.seq, last.seq)),
                    secret: result.and_then(|result| result.invite_token).map(AdminIntentSecretResult::Invite),
                    outcome: AdminIntentOutcomeV1 { code: "directory-events-appended".into(), durable: true, kick_attempted: None, kick_signalled: None },
                }
            }
            AdminEffectCommitV1::RejectedBeforeCommit => admin_effect_rejected_before_commit(),
            AdminEffectCommitV1::Indeterminate => admin_effect_uncertain(),
        };
    }
    match intent {
        AdminIntentV1::IssueDocumentShare { scope, ttl_secs, .. } => {
            let effect = admin_effect_receipt_claim(operation_id, intent_digest, "share-issued");
            match state.directory.issue_share_token_as_with_admin_effect(&scope, i64::from(ttl_secs), Some(&principal.user_id), &principal.correlation_id, &effect).await {
                AdminEffectCommitV1::Applied(issued) => AdminIntentExecution {
                    phase: "succeeded",
                    event_range: None,
                    secret: Some(AdminIntentSecretResult::Share(issued.capability.expose_once())),
                    outcome: AdminIntentOutcomeV1 { code: "share-issued".into(), durable: true, kick_attempted: None, kick_signalled: None },
                },
                AdminEffectCommitV1::RejectedBeforeCommit => admin_effect_rejected_before_commit(),
                AdminEffectCommitV1::Indeterminate => admin_effect_uncertain(),
            }
        }
        AdminIntentV1::RevokeDocumentShare { scope, share_id, reason_code, .. } => {
            let binding = SocketBindingKeyV1::Share(share_id.clone());
            let effect = admin_effect_receipt_claim(operation_id, intent_digest, "share-revoked");
            match state.directory.revoke_share_token_as_with_admin_effect(&scope, &share_id, &reason_code, Some(&principal.user_id), &principal.correlation_id, &effect).await {
                AdminEffectCommitV1::Applied(()) => {}
                AdminEffectCommitV1::RejectedBeforeCommit => return admin_effect_rejected_before_commit(),
                AdminEffectCommitV1::Indeterminate => return admin_effect_uncertain(),
            }
            state.socket_grants.invalidate_binding(binding.clone());
            state.document_open_plans.invalidate_binding(&binding);
            AdminIntentExecution { phase: "succeeded", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "share-revoked".into(), durable: true, kick_attempted: None, kick_signalled: None } }
        }
        AdminIntentV1::RevokeUserSessions { user_id, reason_code, .. } => {
            let effect = admin_effect_receipt_claim(operation_id, intent_digest, "sessions-revoked");
            let revoked = match state.directory.revoke_auth_sessions_for_user_with_admin_effect(&user_id, &reason_code, Some(&principal.user_id), &principal.correlation_id, &effect).await {
                AdminEffectCommitV1::Applied(revoked) => revoked,
                AdminEffectCommitV1::RejectedBeforeCommit => return admin_effect_rejected_before_commit(),
                AdminEffectCommitV1::Indeterminate => return admin_effect_uncertain(),
            };
            for session in &revoked {
                let binding = SocketBindingKeyV1::Session(session.id.clone());
                state.socket_grants.invalidate_binding(binding.clone());
                state.document_open_plans.invalidate_binding(&binding);
            }
            let revoked_ids: BTreeSet<&str> = revoked.iter().map(|session| session.id.as_str()).collect();
            let sessions = state.directory.list_active_sync_sessions(None, ACTIVE_SYNC_SESSION_READ_MAX).await.unwrap_or_default();
            let mut attempted = 0u32;
            let mut signalled = 0u32;
            for session in sessions.iter().filter(|session| session.auth_session_id.as_deref().is_some_and(|id| revoked_ids.contains(id))) {
                attempted = attempted.saturating_add(1);
                if let Some(notify) = state.session_kicks.get_cloned(&session.id) {
                    notify.notify_one();
                    signalled = signalled.saturating_add(1);
                }
            }
            AdminIntentExecution { phase: "succeeded", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "sessions-revoked".into(), durable: true, kick_attempted: Some(attempted), kick_signalled: Some(signalled) } }
        }
        AdminIntentV1::KickConnection { sync_session_id, .. } => match state.session_kicks.get_cloned(&sync_session_id) {
            Some(notify) => {
                notify.notify_one();
                AdminIntentExecution { phase: "succeeded", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "connection-kick-signalled".into(), durable: false, kick_attempted: Some(1), kick_signalled: Some(1) } }
            }
            None => AdminIntentExecution { phase: "failed", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "connection-not-live".into(), durable: false, kick_attempted: Some(1), kick_signalled: Some(0) } },
        },
        AdminIntentV1::RebuildDirectoryProjections { expected_head_seq, .. } => {
            if !matches!(state.directory.head_seq().await, Ok(head) if head == expected_head_seq) {
                return AdminIntentExecution { phase: "failed", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "directory-head-changed".into(), durable: false, kick_attempted: None, kick_signalled: None } };
            }
            let control = operation_runtime.unwrap_or_else(|| {
                Arc::new(AdminOperationRuntime {
                    deadline: std::time::Instant::now() + ADMIN_OPERATION_DEADLINE,
                    completed: std::sync::atomic::AtomicU64::new(0),
                    total: std::sync::atomic::AtomicU64::new(0),
                    effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_ADMITTED),
                    cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
                })
            });
            match state.directory.rebuild_projections_controlled(control.as_ref()).await {
                Ok(_) => AdminIntentExecution { phase: "succeeded", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "directory-projections-rebuilt".into(), durable: true, kick_attempted: None, kick_signalled: None } },
                Err(_) if control.is_cancelled() => {
                    AdminIntentExecution { phase: "cancelled", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "directory-rebuild-cancelled".into(), durable: false, kick_attempted: None, kick_signalled: None } }
                }
                Err(_) => AdminIntentExecution { phase: "failed", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "directory-rebuild-rejected".into(), durable: false, kick_attempted: None, kick_signalled: None } },
            }
        }
        _ => unreachable!("closed admin directory intents were handled before dispatch"),
    }
}

fn admin_execution_receipt(operation_id: String, correlation_id: String, execution: AdminIntentExecution) -> AdminIntentReceiptV1 {
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

pub(crate) async fn admin_intents(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>, body: Bytes) -> Result<(StatusCode, DirectoryJson<AdminIntentReceiptV1>), StatusCode> {
    let principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    if body.is_empty() || body.len() > ADMIN_INTENT_REQUEST_MAX_BYTES || headers.get(axum::http::header::CONTENT_TYPE).and_then(|value| value.to_str().ok()) != Some("application/json") {
        return Err(StatusCode::BAD_REQUEST);
    }
    let encoded = std::str::from_utf8(&body).map_err(|_| StatusCode::BAD_REQUEST)?;
    let intent: AdminIntentV1 = semio_framework_pack_json::from_json_str(encoded, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| StatusCode::BAD_REQUEST)?;
    validate_admin_intent(&intent)?;
    let metadata = admin_intent_metadata(&intent);
    let digest = admin_intent_digest(&intent);
    let request_id = intent.request_id().to_string();
    let prior = state.directory.admin_operation_audit_for_request(&request_id).await.map_err(directory_error_status)?;
    let prior = reconcile_stale_admin_acceptance(&state, prior).await?;
    if let Some(receipt) = admin_audit_receipt(&prior) {
        let same = prior
            .first()
            .is_some_and(|row| row.fact.intent_digest == digest && row.fact.principal_user_id == principal.user_id && row.fact.principal_session_id == principal.auth_session_id && row.fact.principal_generation == principal.authorization_generation);
        let resolver_live = state.admin_operations.get_cloned(&receipt.operation_id).is_some();
        return if same { Ok((StatusCode::OK, DirectoryJson(admin_audit_visibility(receipt, resolver_live)))) } else { Err(StatusCode::CONFLICT) };
    }
    let proposed_operation_id = directory::os_identity::time_ordered_id();
    let accepted = new_admin_audit_fact(&principal, &request_id, &digest, &proposed_operation_id, &metadata, "accepted", None, "accepted");
    let established = state.directory.append_admin_operation_audit(&accepted).await.map_err(directory_error_status)?;
    if established.fact.operation_id != proposed_operation_id {
        let joined = state.directory.admin_operation_audit_for_request(&request_id).await.map_err(directory_error_status)?;
        return admin_audit_receipt(&joined)
            .map(|receipt| {
                let resolver_live = state.admin_operations.get_cloned(&receipt.operation_id).is_some();
                (StatusCode::OK, DirectoryJson(admin_audit_visibility(receipt, resolver_live)))
            })
            .ok_or(StatusCode::CONFLICT);
    }
    let operation_permit = match state.admin_operation_slots.clone().try_acquire_owned() {
        Ok(permit) => permit,
        Err(_) => {
            let execution = AdminIntentExecution { phase: "failed", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "admin-operation-capacity".into(), durable: false, kick_attempted: None, kick_signalled: None } };
            let terminal = new_admin_audit_fact(&principal, &request_id, &digest, &proposed_operation_id, &metadata, execution.phase, None, &execution.outcome.code);
            state.directory.append_admin_operation_audit(&terminal).await.map_err(directory_error_status)?;
            return Ok((StatusCode::OK, DirectoryJson(admin_execution_receipt(proposed_operation_id, principal.correlation_id, execution))));
        }
    };
    let runtime = Arc::new(AdminOperationRuntime {
        deadline: std::time::Instant::now() + ADMIN_OPERATION_DEADLINE,
        completed: std::sync::atomic::AtomicU64::new(0),
        total: std::sync::atomic::AtomicU64::new(0),
        effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_PRE_EFFECT),
        cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
    });
    state.admin_operations.insert(proposed_operation_id.clone(), runtime.clone());
    let (response_tx, response_rx) = tokio::sync::oneshot::channel();
    let task_state = state.clone();
    let task_principal = principal.clone();
    let task_request_id = request_id.clone();
    let task_digest = digest.clone();
    let task_operation_id = proposed_operation_id.clone();
    let task_metadata = metadata.clone();
    let task_runtime = runtime.clone();
    let task_headers = headers.clone();
    let asynchronous = matches!(&intent, AdminIntentV1::RebuildDirectoryProjections { .. });
    let retained = async move {
        let _cleanup = AdminOperationCleanup { operations: task_state.admin_operations.clone(), operation_id: task_operation_id.clone(), _permit: operation_permit };
        let mut authority = None;
        let execution = match authenticate_admin_principal(&task_state, &task_headers, Some(peer)).await {
            Ok(fresh) if task_principal.same_authority(&fresh) => Some(execute_admin_intent(&task_state, &task_principal, &task_operation_id, &task_digest, intent, Some(task_runtime.clone()), &mut authority).await),
            _ => Some(AdminIntentExecution { phase: "cancelled", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "admin-authority-changed".into(), durable: false, kick_attempted: None, kick_signalled: None } }),
        };
        let Some(execution) = execution else {
            let _ = response_tx.send(None);
            return;
        };
        if execution.phase == "uncertain" {
            drop(authority);
            let _ = response_tx.send(None);
            return;
        }
        let terminal = new_admin_audit_fact(&task_principal, &task_request_id, &task_digest, &task_operation_id, &task_metadata, execution.phase, execution.event_range, &execution.outcome.code);
        if !append_admin_terminal_with_retry(&task_state, &terminal).await {
            let _ = response_tx.send(None);
            return;
        }
        drop(authority);
        if response_tx.is_closed() {
            drop(execution);
            return;
        }
        if let Err(mut execution) = response_tx.send(Some(execution)) {
            execution.take();
        }
    };
    if state.admin_operation_tasks.spawn(proposed_operation_id.clone(), runtime.clone(), retained).is_err() {
        state.admin_operations.remove(&proposed_operation_id);
        let execution = AdminIntentExecution { phase: "failed", event_range: None, secret: None, outcome: AdminIntentOutcomeV1 { code: "admin-operation-shutting-down".into(), durable: false, kick_attempted: None, kick_signalled: None } };
        let terminal = new_admin_audit_fact(&principal, &request_id, &digest, &proposed_operation_id, &metadata, execution.phase, None, &execution.outcome.code);
        state.directory.append_admin_operation_audit(&terminal).await.map_err(directory_error_status)?;
        return Ok((StatusCode::OK, DirectoryJson(admin_execution_receipt(proposed_operation_id, principal.correlation_id, execution))));
    }
    if asynchronous {
        return Ok((
            StatusCode::ACCEPTED,
            DirectoryJson(AdminIntentReceiptV1 {
                operation_id: proposed_operation_id,
                correlation_id: principal.correlation_id,
                state: AdminIntentStateV1::Accepted,
                event_seq_first: None,
                event_seq_last: None,
                result: None,
                outcome: AdminIntentOutcomeV1 { code: "directory-rebuild-running".into(), durable: false, kick_attempted: None, kick_signalled: None },
            }),
        ));
    }
    match tokio::time::timeout_at(tokio::time::Instant::from_std(runtime.deadline), response_rx).await {
        Ok(Ok(Some(execution))) => Ok((StatusCode::OK, DirectoryJson(admin_execution_receipt(proposed_operation_id, principal.correlation_id, execution)))),
        Ok(Ok(None)) | Ok(Err(_)) | Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

/// 🌵️ `extensions_root` is `{data_dir}/extension-modules` (see `main`'s own construction) — its
/// parent is `data_dir` itself, the nearest thing `HubState` carries to `OS_HUB_DATA`'s root.
pub(crate) async fn admin_overview(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<Json<serde_json::Value>, StatusCode> {
    let _principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    let counts = state.directory.admin_overview_counts().await.map_err(directory_error_status)?;
    let head_seq = state.directory.head_seq().await.map_err(directory_error_status)?;
    let data_dir_bytes = state.extensions_root.parent().map(dir_size).unwrap_or(0);
    let response = serde_json::json!({
        "counts": { "spaces": counts.spaces, "users": counts.users, "connections": counts.connections },
        "backends": { "sqlite": true, "postgres": true, "neo4j": true },
        "dataDirBytes": data_dir_bytes,
        "headSeq": head_seq,
        "openArtifacts": state.db.catalog().await.artifacts.len(),
    });
    if serde_json::to_vec(&response).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.len() > ADMIN_RESPONSE_MAX_BYTES {
        return Err(StatusCode::PAYLOAD_TOO_LARGE);
    }
    Ok(Json(response))
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
pub(crate) fn observability_view(tracer: &Tracer, routes: &HubRouteMetricsV1, catalog: Option<&VerifiedTrustedCatalog>) -> HubObservabilityV1 {
    HubObservabilityV1::read(tracer, routes, catalog, HUB_PROCESS_START.elapsed().as_millis().min(u128::from(u64::MAX)) as u64)
}

/// 📝️ `GET /admin/api/observability` — behind `authenticate_admin_principal`, exactly like
/// every other `/admin/api` route, so a hub bound to a network interface never exposes its own
/// internals unauthenticated. See [`observability_view`] for why the body is counters.
pub(crate) async fn admin_observability(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<Json<HubObservabilityV1>, StatusCode> {
    let _principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    let response = observability_view(&state.tracer, &state.route_metrics, state.verified_catalog.as_deref());
    if serde_json::to_vec(&response).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.len() > ADMIN_RESPONSE_MAX_BYTES {
        return Err(StatusCode::PAYLOAD_TOO_LARGE);
    }
    Ok(Json(response))
}

pub(crate) fn admin_space_summary_view(summary: semio_hub::directory::model::AdminSpaceSummaryRecord) -> Result<SpaceView, StatusCode> {
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

pub(crate) async fn admin_spaces(
    Query(query): Query<AdminPageQuery>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<DirectoryJson<AdminPageV1<SpaceView>>, StatusCode> {
    let principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    let limit = admin_page_limit(&query)?;
    let offset = admin_cursor_decode(&state.admin_cursor_key, &principal, 6, query.cursor.as_deref())?;
    let mut summaries = state.directory.list_admin_space_summaries_page(None, offset, limit.saturating_add(1)).await.map_err(directory_error_status)?;
    let storage_has_more = summaries.len() > limit;
    summaries.truncate(limit);
    let mut rows = Vec::with_capacity(summaries.len());
    for summary in summaries {
        rows.push(admin_space_summary_view(summary)?);
    }
    let page = admin_fit_page(rows, storage_has_more, now_ms(), |rows| admin_cursor_encode(&state.admin_cursor_key, &principal, 6, offset + rows.len()))?;
    Ok(DirectoryJson(page))
}

pub(crate) struct AdminSpaceDetailResponse {
    pub(crate) view: SpaceView,
    pub(crate) members: AdminPageV1<MemberView>,
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

pub(crate) fn admin_fit_space_detail(view: SpaceView, mut rows: Vec<MemberView>, storage_has_more: bool, observed_at_ms: i64, cursor_key: &[u8; 32], principal: &AdminPrincipalV1, space_id: &str, offset: usize) -> Result<AdminSpaceDetailResponse, StatusCode> {
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

pub(crate) async fn admin_space(
    Path(space_id): Path<String>,
    Query(query): Query<AdminPageQuery>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<DirectoryJson<AdminSpaceDetailResponse>, StatusCode> {
    let principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    semio_hub::directory::validate_bounded_auth_text(&space_id, "admin detail space", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
    let limit = admin_page_limit(&query)?;
    let offset = admin_cursor_decode_scoped(&state.admin_cursor_key, &principal, 7, Some(&space_id), query.cursor.as_deref())?;
    let summary = state.directory.list_admin_space_summaries_page(Some(&space_id), 0, 1).await.map_err(directory_error_status)?.pop().ok_or(StatusCode::NOT_FOUND)?;
    let view = admin_space_summary_view(summary)?;
    let mut records = state.directory.list_admin_space_members_page(&space_id, offset, limit.saturating_add(1)).await.map_err(directory_error_status)?;
    let storage_has_more = records.len() > limit;
    records.truncate(limit);
    let rows: Vec<MemberView> = records.into_iter().map(|(user, role)| MemberView { user_id: user.id, email: user.email, display_name: user.display_name, role: role_wire(role) }).collect();
    Ok(DirectoryJson(admin_fit_space_detail(view, rows, storage_has_more, now_ms(), &state.admin_cursor_key, &principal, &space_id, offset)?))
}

pub(crate) async fn admin_users(
    Query(query): Query<AdminPageQuery>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<DirectoryJson<AdminPageV1<os_directory::UserView>>, StatusCode> {
    let principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    let limit = admin_page_limit(&query)?;
    let offset = admin_cursor_decode(&state.admin_cursor_key, &principal, 2, query.cursor.as_deref())?;
    let mut users = state.directory.list_users(i64::try_from(limit.saturating_add(1)).map_err(|_| StatusCode::BAD_REQUEST)?, i64::try_from(offset).map_err(|_| StatusCode::BAD_REQUEST)?).await.map_err(directory_error_status)?;
    let storage_has_more = users.len() > limit;
    users.truncate(limit);
    let rows = users.into_iter().map(|user| os_directory::UserView { id: user.id, email: user.email, display_name: user.display_name, created_at_ms: user.created_at }).collect();
    let page = admin_fit_page(rows, storage_has_more, now_ms(), |rows| admin_cursor_encode(&state.admin_cursor_key, &principal, 2, offset + rows.len()))?;
    Ok(DirectoryJson(page))
}

pub(crate) async fn admin_connections(
    Query(query): Query<AdminPageQuery>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<DirectoryJson<AdminConnectionSnapshotV1>, StatusCode> {
    let principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    let limit = admin_page_limit(&query)?;
    let offset = admin_cursor_decode(&state.admin_cursor_key, &principal, 1, query.cursor.as_deref())?;
    let mut sessions = state.directory.list_active_sync_sessions_page(None, offset, limit.saturating_add(1)).await.map_err(directory_error_status)?;
    let storage_has_more = sessions.len() > limit;
    sessions.truncate(limit);
    let rows = sessions
        .into_iter()
        .map(|session| AdminRecordedConnectionV1 {
            sync_session_id: session.id,
            scope: DocumentScope::new(&session.space_id, &session.document_id),
            authenticated_user_id: session.user_id,
            email: session.authenticated_email,
            role: session.space_role.map(|role| match role {
                SpaceRole::Author => DirectorySpaceRole::Author,
                SpaceRole::Spectator => DirectorySpaceRole::Spectator,
            }),
            connected_at_ms: session.connected_at,
            source: "recorded-sync-session".into(),
        })
        .collect();
    let observed_at_ms = now_ms();
    let head_seq = state.directory.head_seq().await.map_err(directory_error_status)?;
    Ok(DirectoryJson(admin_fit_connection_snapshot(rows, storage_has_more, observed_at_ms, head_seq, &state.admin_cursor_key, &principal, offset)?))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DocumentsQuery {
    pub(crate) space: Option<String>,
    pub(crate) cursor: Option<String>,
    pub(crate) limit: Option<usize>,
}

pub(crate) async fn admin_documents(
    Query(query): Query<DocumentsQuery>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<DirectoryJson<AdminPageV1<DocumentView>>, StatusCode> {
    let principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    let page = AdminPageQuery { cursor: query.cursor, limit: query.limit };
    let limit = admin_page_limit(&page)?;
    if let Some(space_id) = query.space.as_deref() {
        semio_hub::directory::validate_bounded_auth_text(space_id, "admin document space", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
    }
    let offset = admin_cursor_decode_scoped(&state.admin_cursor_key, &principal, 5, query.space.as_deref(), page.cursor.as_deref())?;
    let mut descriptors = state.directory.list_document_descriptors_page(query.space.as_deref(), offset, limit.saturating_add(1)).await.map_err(directory_error_status)?;
    let storage_has_more = descriptors.len() > limit;
    descriptors.truncate(limit);
    let mut rows = Vec::with_capacity(descriptors.len());
    for descriptor in descriptors {
        rows.push(document_view(&state, descriptor).await);
    }
    let page = admin_fit_page(rows, storage_has_more, now_ms(), |rows| admin_cursor_encode_scoped(&state.admin_cursor_key, &principal, 5, query.space.as_deref(), offset + rows.len()))?;
    Ok(DirectoryJson(page))
}

pub(crate) async fn admin_events(
    Query(query): Query<AdminPageQuery>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<DirectoryJson<AdminPageV1<DirectoryEvent>>, StatusCode> {
    let principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    let limit = admin_page_limit(&query)?;
    let since = admin_cursor_decode(&state.admin_cursor_key, &principal, 3, query.cursor.as_deref())?;
    let mut events = state.directory.events_since(u64::try_from(since).map_err(|_| StatusCode::BAD_REQUEST)?, limit.saturating_add(1)).await.map_err(directory_error_status)?;
    let storage_has_more = events.len() > limit;
    events.truncate(limit);
    let page = admin_fit_page(events, storage_has_more, now_ms(), |rows| {
        let next = rows.last().map(|event| usize::try_from(event.seq).map_err(|_| StatusCode::BAD_REQUEST)).transpose()?.unwrap_or(since);
        admin_cursor_encode(&state.admin_cursor_key, &principal, 3, next)
    })?;
    Ok(DirectoryJson(page))
}
//#endregion 🔖️Admin
