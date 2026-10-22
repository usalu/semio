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
use semio_framework_async_slots::ShardedMap;
use semio_framework_hash::Sha256;
use semio_framework_trace::record::Tracer;
use semio_hub_artifact_authority::chunk_cas::ArtifactChunkCasStorage;
use semio_hub_artifact_creation::ArtifactCreationCommitAuthorityV1;
use semio_hub_observability::HubObservabilityV1;
use semio_hub_artifact_authority::AuthorityOperationControl;
use semio_hub_directory_contract::model::{AdminEffectCommitV1, AdminOperationAuditRecord, NewAdminOperationAuditRecord, SocketSessionBindingStatus, SpaceRole};
use directory::os_directory::DocumentScope;
use semio_hub_directory::{AdminOperationDirectory, AuthSessionDirectory, DocumentDirectory, EventLog, ProjectionRebuild, ProjectionRebuildControl, ShareTokenDirectory, SpaceDirectory, SyncSessionDirectory, UserDirectory};
use semio_hub_directory_contract::{ACTIVE_SYNC_SESSION_READ_MAX, ADMIN_INTENT_REQUEST_MAX_BYTES, ADMIN_RESPONSE_MAX_BYTES, IdentityVerificationControl, AUTH_TEXT_MAX_BYTES};
use semio_hub_inference_runtime_contract::GisMapApprovalIngressAuthorityV1;
use semio_hub_lag_rebootstrap::RebootstrapTransferControl;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use semio_hub_runtime::support::{DirectoryJson, now_ms, directory_error_status};
use semio_hub_state::HubState;
use semio_hub_ledgers::socket_grants::{SocketBindingKeyV1, SocketBindingModeV1, SocketBindingGuardV1};
use semio_hub_ledgers::guard::{AdminPrincipalV1, admin_provider_digest};
use semio_hub_session_routes::guard::authenticate_admin_principal;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_session_routes::access_grants::{pause_directory_command_authority, pause_directory_command_membership_fence, pause_admin_effect_started};
use semio_hub_route_models::directory_commands::{role_wire, FencedDirectoryCommandErrorV1};
use semio_hub_session_routes::directory_commands::{document_view, authorize_directory_command, invalidate_directory_event_authority};
#[cfg(not(any(test, feature = "testkit")))]
use semio_hub_session_routes::directory_commands::{pause_directory_command_authority, pause_directory_command_membership_fence, pause_admin_effect_started};
use db::storage::PayloadStorage as _;
use server::storage::SessionStore as _;
use semio_hub_route_models::admin::{ADMIN_EFFECT_ADMITTED, ADMIN_EFFECT_PRE_EFFECT, ADMIN_OPERATION_DEADLINE, ADMIN_TERMINAL_RETRY_DEADLINE, AdminIntentExecution, AdminIntentSecretResult, AdminOperationCleanup, AdminOperationRuntime, AdminPageQuery, AdminSpaceDetailResponse, DocumentsQuery, admin_audit_receipt, admin_audit_visibility, admin_create_space_id, admin_cursor_decode, admin_cursor_decode_scoped, admin_cursor_encode, admin_cursor_encode_scoped, admin_directory_authority_refusal, admin_directory_command, admin_effect_receipt_claim, admin_effect_rejected_before_commit, admin_effect_uncertain, admin_execution_receipt, admin_fit_connection_snapshot, admin_fit_page, admin_fit_space_detail, admin_intent_bindings, admin_intent_changes_own_principal, admin_intent_digest, admin_intent_metadata, admin_page_limit, admin_space_summary_view, dir_size, new_admin_audit_fact, observability_view, public_admin_audit, validate_admin_intent};

//#region 🔖️Admin

pub async fn reconcile_stale_admin_acceptance(state: &HubState, rows: Vec<AdminOperationAuditRecord>) -> Result<Vec<AdminOperationAuditRecord>, StatusCode> {
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

pub async fn admin_operation(Path(operation_id): Path<String>, headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<DirectoryJson<AdminOperationStatusV1>, StatusCode> {
    authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    semio_hub_directory_contract::validate_bounded_auth_text(&operation_id, "admin operation id", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
    let rows = state.directory.admin_operation_audit_for_operation(&operation_id).await.map_err(directory_error_status)?;
    let rows = reconcile_stale_admin_acceptance(&state, rows).await?;
    let receipt = admin_audit_receipt(&rows).ok_or(StatusCode::NOT_FOUND)?;
    let progress = state.admin_operations.with(&receipt.operation_id, |runtime| runtime.map(|runtime| runtime.progress()));
    let receipt = admin_audit_visibility(receipt, progress.is_some());
    Ok(DirectoryJson(AdminOperationStatusV1 { receipt, progress }))
}

pub async fn cancel_admin_operation(
    Path(operation_id): Path<String>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<DirectoryJson<AdminOperationStatusV1>, StatusCode> {
    authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    semio_hub_directory_contract::validate_bounded_auth_text(&operation_id, "admin operation id", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
    let rows = state.directory.admin_operation_audit_for_operation(&operation_id).await.map_err(directory_error_status)?;
    let receipt = admin_audit_receipt(&rows).ok_or(StatusCode::NOT_FOUND)?;
    let runtime = state.admin_operations.get_cloned(&receipt.operation_id).ok_or(StatusCode::CONFLICT)?;
    runtime.request_cancel();
    Ok(DirectoryJson(AdminOperationStatusV1 { receipt, progress: Some(runtime.progress()) }))
}

pub async fn admin_operation_audit(
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
            .any(|subject| semio_hub_directory_contract::constant_time_digest_eq(&subject.provider_digest, &provider_digest) && semio_hub_directory_contract::constant_time_digest_eq(&subject.subject_digest, &principal.identity_subject_digest))
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

pub async fn execute_admin_intent(
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

pub async fn admin_intents(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>, body: Bytes) -> Result<(StatusCode, DirectoryJson<AdminIntentReceiptV1>), StatusCode> {
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
pub async fn admin_overview(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<Json<serde_json::Value>, StatusCode> {
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

/// 📝️ `GET /admin/api/observability` — behind `authenticate_admin_principal`, exactly like
/// every other `/admin/api` route, so a hub bound to a network interface never exposes its own
/// internals unauthenticated. See [`observability_view`] for why the body is counters.
pub async fn admin_observability(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>) -> Result<Json<HubObservabilityV1>, StatusCode> {
    let _principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    let response = observability_view(&state.tracer, &state.route_metrics, state.verified_catalog.as_deref());
    if serde_json::to_vec(&response).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.len() > ADMIN_RESPONSE_MAX_BYTES {
        return Err(StatusCode::PAYLOAD_TOO_LARGE);
    }
    Ok(Json(response))
}

pub async fn admin_spaces(
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

pub async fn admin_space(
    Path(space_id): Path<String>,
    Query(query): Query<AdminPageQuery>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<DirectoryJson<AdminSpaceDetailResponse>, StatusCode> {
    let principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    semio_hub_directory_contract::validate_bounded_auth_text(&space_id, "admin detail space", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
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

pub async fn admin_users(
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

pub async fn admin_connections(
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

pub async fn admin_documents(
    Query(query): Query<DocumentsQuery>,
    headers: HeaderMap,
    axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>,
    State(state): State<HubState>,
) -> Result<DirectoryJson<AdminPageV1<DocumentView>>, StatusCode> {
    let principal = authenticate_admin_principal(&state, &headers, Some(peer)).await?;
    let page = AdminPageQuery { cursor: query.cursor, limit: query.limit };
    let limit = admin_page_limit(&page)?;
    if let Some(space_id) = query.space.as_deref() {
        semio_hub_directory_contract::validate_bounded_auth_text(space_id, "admin document space", AUTH_TEXT_MAX_BYTES).map_err(|_| StatusCode::BAD_REQUEST)?;
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

pub async fn admin_events(
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
