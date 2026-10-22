//! 📌️ `check_in` — checkpoint publication and document check-in.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use axum::body::Bytes;
use axum::extract::{OriginalUri, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use directory::os_directory;
use directory::os_directory::{ArtifactFrontier, ArtifactHash, DocumentCheckInPhaseV1, DocumentCheckInRefusalV1, DocumentCheckInStatusV1, DocumentCheckInV1, EditedArtifactFrontierV1, DirectoryActor, DirectoryActorKind, DocumentDescriptor, DocumentOpenPlanErrorCodeV1, PublishedArtifactCheckpoint};
use directory::os_directory::io::binary::descriptor_digest::descriptor_digest_v1;
use futures::{SinkExt, StreamExt};
use protocol::{MutationEnvelope, ServerFrame};
use semio_framework_async_slots::ShardedMap;
use semio_framework_hash::Sha256;
use semio_framework_trace::record::{TraceLevel, TraceOutcome, TraceRecord};
use semio_hub_artifact_authority::chunk_cas::{ArtifactChunkBlobStore, ArtifactChunkCasStorage};
use semio_hub_artifact_creation::ArtifactCreationCommitAuthorityV1;
use semio_hub_artifact_checkin::{check_in_refusal_of_authority_error, CheckInMaterialization, DocumentCheckInAdmission, DocumentCheckInJob, DocumentCheckInKey, ReplayingArtifactAuthority, DOCUMENT_CHECK_IN_STALL_BOUND_MS};
use semio_hub_artifact_authority::{AuthorityError, AuthorityLimits, AuthorityOperationControl, CheckpointPublicationOrchestrator, OperationContext, VerifiedCheckpointPublisher};
use directory::os_directory::access_policy::DirectoryAccessActionV1;
use semio_hub_directory_contract::model::{CheckpointPublicationClaimV1, CheckpointPublicationCompletionV1, CheckpointPublicationDispositionV1, NewCheckpointPublicationClaimV1};
use directory::os_directory::DocumentScope;
use semio_hub_directory::DocumentDirectory;
use semio_hub_directory_service::HubVerifiedCheckpointPublisher;
use semio_hub_directory_contract::{IdentityVerificationControl, LocalBootstrapTransport};
use semio_hub_inference_runtime_contract::GisMapApprovalIngressAuthorityV1;
use semio_hub_lag_rebootstrap::{RebootstrapContext, RebootstrapError, RebootstrapTransferControl, REBOOTSTRAP_DEADLINE_MS};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::broadcast;
use semio_hub_runtime::support::now_ms;
use semio_hub_runtime::presence::{db_artifact_id, db_core_document_id};
use semio_hub_state::HubState;
use semio_hub_ledgers::socket_grants::{SocketAudienceV1, SocketSubjectV1, SocketBindingValidityV1};
use semio_hub_session_routes::guard::access_permits_in_space;
use semio_hub_ledgers::access_grants::socket_issue_bearer;
use semio_hub_session_routes::access_grants::authenticate_document_credential;
use semio_hub_session_routes::directory_commands::resolve_bearer_user;
use db::storage::PayloadStorage as _;
use server::storage::SessionStore as _;
use semio_hub_route_models::check_in::{CheckInClaimGuardV1, CheckInEndV1, CheckInPairControlV1, check_in_ledger_point, check_in_refusal_of_ledger_error, check_in_status_response, ledger_artifact_frontier};

//#region 🔖️Rest
//#region 📌️CheckIn

/// 🔒️ Publishes a Check In candidate only while the author still writes, the descriptor is the one it
/// was materialized against, and the active checkpoint is still its parent; completes the durable
/// claim in the same directory transaction as the checkpoint event.
struct FencedCheckInPublisherV1 {
    state: HubState,
    subject: SocketSubjectV1,
    audience: SocketAudienceV1,
    scope: DocumentScope,
    descriptor: DocumentDescriptor,
    parent: PublishedArtifactCheckpoint,
    completion: CheckpointPublicationCompletionV1,
}

impl FencedCheckInPublisherV1 {
    pub(crate) fn changed() -> AuthorityError {
        AuthorityError::Publication("check-in authority changed".into())
    }

    async fn authority_is_current(&self) -> Result<bool, AuthorityError> {
        if self.subject.revalidate(self.state.directory.as_ref(), &self.audience, now_ms()).await != SocketBindingValidityV1::Active {
            return Ok(false);
        }
        let descriptor = self.state.directory.get_document_descriptor(&self.scope).await.map_err(|_| Self::changed())?;
        let current = self.state.directory.get_active_artifact_checkpoint(&self.scope).await.map_err(|_| Self::changed())?;
        Ok(descriptor.as_ref() == Some(&self.descriptor) && current.as_ref().map(|checkpoint| checkpoint.checkpoint_id) == Some(self.parent.checkpoint_id))
    }
}

impl VerifiedCheckpointPublisher for FencedCheckInPublisherV1 {
    async fn reserve(&self, plan: &semio_hub_artifact_authority::chunk_cas::ArtifactCasOwnershipPlanV1, context: &OperationContext<'_>) -> Result<semio_hub_artifact_authority::chunk_cas::ArtifactCasReservation, AuthorityError> {
        context.checkpoint()?;
        if !self.authority_is_current().await? {
            return Err(Self::changed());
        }
        HubVerifiedCheckpointPublisher::new(self.state.directory_service.clone(), self.state.artifact_cas.clone(), "system:check-in").reserve(plan, context).await
    }

    async fn publish_reserved(&self, checkpoint: &os_directory::ArtifactCheckpoint, reservation: &semio_hub_artifact_authority::chunk_cas::ArtifactCasReservation, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        let authorization = tokio::time::timeout(std::time::Duration::from_secs(2), self.state.socket_binding_gates.share_record(&self.subject, &self.audience)).await.map_err(|_| Self::changed())?;
        let document_write = tokio::time::timeout(std::time::Duration::from_secs(2), self.state.socket_binding_gates.document_write(&self.scope).lock_owned()).await.map_err(|_| Self::changed())?;
        context.checkpoint()?;
        if !self.authority_is_current().await? || checkpoint.scope != self.scope || checkpoint.parent_checkpoint_id != Some(self.parent.checkpoint_id) {
            return Err(Self::changed());
        }
        let mut completion = self.completion.clone();
        completion.checkpoint_id = checkpoint.checkpoint_id;
        completion.completed_at = i64::try_from(context.now_ms()).map_err(|_| Self::changed())?;
        let result = self
            .state
            .directory_service
            .publish_reserved_artifact_checkpoint_and_complete_checkpoint_publication(DirectoryActor { kind: DirectoryActorKind::System, id: "system:check-in".into() }, checkpoint.clone(), reservation.clone(), completion, context.now_ms())
            .await
            .map(|_| ())
            .map_err(|_| AuthorityError::Publication("check-in completion failed".into()));
        drop(document_write);
        drop(authorization);
        result
    }
}

/// 📌️ Runs one admitted Check In to its terminal status: read the active checkpoint and its pair,
/// read the committed ledger from its baseline to the named head, fold it through the package codec,
/// and publish the result behind the author/descriptor/active-checkpoint fence. A monitor revalidates
/// the author every 50 ms and revokes the job the moment write access is lost.
/// 📌️ Tallies one committed batch for the checkpoint policy and, when it reaches the policy, starts the policy Check In
/// of the document on behalf of `subject` in the background.
pub(crate) fn observe_checkpoint_policy(state: &HubState, scope: &DocumentScope, subject: &SocketSubjectV1, handle: &db::ArtifactHandle, envelopes: &[MutationEnvelope]) {
    let payload_bytes = envelopes.iter().map(|envelope| (envelope.diff.payload.len() + envelope.inverse.payload.len()) as u64).sum();
    if state.checkpoint_policy.observe(scope, envelopes.len() as u64, payload_bytes, state.readiness.features.checkpoint_policy) {
        tokio::spawn(run_policy_check_in(state.clone(), subject.clone(), scope.clone(), handle.clone()));
    }
}

/// 📌️ The checkpoint policy's Check In: `subject` — the author whose batch reached the policy, still allowed to check
/// in — checks `scope` in at its committed head through the ordinary claim, fold and fenced publication; answers the
/// job's terminal status (`None` when it could not start: no author session, no head, a claim of the same head exists).
pub async fn run_policy_check_in(state: HubState, subject: SocketSubjectV1, scope: DocumentScope, handle: db::ArtifactHandle) -> Option<DocumentCheckInStatusV1> {
    let status = start_policy_check_in(&state, subject, &scope, &handle).await;
    state.checkpoint_policy.finish(&scope);
    status
}

async fn start_policy_check_in(state: &HubState, subject: SocketSubjectV1, scope: &DocumentScope, handle: &db::ArtifactHandle) -> Option<DocumentCheckInStatusV1> {
    let SocketSubjectV1::Session { user_id, .. } = &subject else { return None };
    let user_id = user_id.clone();
    if state.artifact_authority.is_none() || !access_permits_in_space(state, &subject.access_roles(), DirectoryAccessActionV1::DocumentCheckIn, &scope.space_id).await {
        return None;
    }
    let snapshot = handle.checkpoint_publication_snapshot().await.ok()?;
    let head = directory::os_directory::io::binary::artifact_hash::encode_edited_artifact_frontier_v1(&ledger_artifact_frontier(scope, &snapshot)?)?;
    let request_id = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(format!("checkpoint-policy\0{}\0{}\0{}", scope.space_id, scope.document_id, head.head_edit_ordinal).as_bytes()))[..32].to_string();
    let request = DocumentCheckInV1 { schema: os_directory::DOCUMENT_CHECK_IN_SCHEMA_V1.into(), request_id: request_id.clone(), head };
    let source = request.canonical_json()?;
    let command_sha256 = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(source.as_bytes()));
    let key = DocumentCheckInKey { user_id: user_id.clone(), space_id: scope.space_id.clone(), document_id: scope.document_id.clone(), request_id: request_id.clone() };
    let DocumentCheckInAdmission::Owner(job) = state.check_ins.admit(key.clone(), &command_sha256) else { return None };
    let claim = NewCheckpointPublicationClaimV1 { actor_user_id: user_id.clone(), correlation_id: request_id.clone(), command_sha256: command_sha256.clone(), claimed_at: now_ms() };
    if !matches!(state.directory_service.claim_or_read_checkpoint_publication(&claim).await, Ok(CheckpointPublicationClaimV1::Claimed(_))) {
        state.check_ins.forget(&key);
        return None;
    }
    let guard = CheckInClaimGuardV1 { service: state.directory_service.clone(), actor_user_id: user_id, correlation_id: request_id, command_sha256, complete: false };
    let policy = state.readiness.features.checkpoint_policy;
    let mut record = TraceRecord::new("server.document.check-in", TraceOutcome::Started);
    record.level = TraceLevel::Info;
    record.principal = Some(subject.trace_principal());
    record.space = Some(scope.space_id.clone());
    record.artifact = Some(scope.document_id.clone());
    record.detail = Some(format!("checkpoint-policy head={} edits={} payloadBytes={}", request.head.head_edit_ordinal, policy.edits, policy.payload_bytes));
    state.tracer.emit(record);
    run_document_check_in(state.clone(), subject, scope.clone(), request, job.clone(), guard).await;
    Some(job.status())
}

async fn run_document_check_in(state: HubState, subject: SocketSubjectV1, scope: DocumentScope, request: DocumentCheckInV1, job: Arc<DocumentCheckInJob>, mut claim: CheckInClaimGuardV1) {
    let audience = SocketAudienceV1::Document(scope.clone());
    let monitor = tokio::spawn({
        let state = state.clone();
        let subject = subject.clone();
        let audience = audience.clone();
        let job = job.clone();
        async move {
            while !job.is_terminal() {
                if subject.revalidate(state.directory.as_ref(), &audience, now_ms()).await != SocketBindingValidityV1::Active {
                    job.revoke();
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            }
        }
    });
    let span = state.span("server.document.check-in").principal(subject.trace_principal()).space(scope.space_id.clone()).artifact(scope.document_id.clone());
    let outcome = materialize_and_publish_check_in(&state, &subject, &audience, &scope, &request, &job, &claim).await;
    let mut end_cause = None;
    match outcome {
        Ok((checkpoint_id, parent_checkpoint_id, baseline)) => {
            claim.complete = checkpoint_id != parent_checkpoint_id;
            if !claim.complete {
                claim.release().await;
            }
            job.finish_ready(checkpoint_id, parent_checkpoint_id, baseline);
        }
        Err(CheckInEndV1 { refusal: Some(refusal), cause }) => {
            claim.release().await;
            job.finish_refused(refusal);
            end_cause = cause;
        }
        Err(CheckInEndV1 { refusal: None, cause }) => {
            claim.release().await;
            job.finish_cancelled();
            end_cause = cause;
        }
    }
    monitor.abort();
    let status = job.status();
    match (status.phase, status.refusal) {
        (DocumentCheckInPhaseV1::Ready, _) => span.ok(),
        (DocumentCheckInPhaseV1::Cancelled, _) => span.cancelled("cancelled"),
        (_, refusal) => {
            let code = match refusal {
                Some(DocumentCheckInRefusalV1::UnknownHead) => "unknown-head",
                Some(DocumentCheckInRefusalV1::StaleHead) => "stale-head",
                Some(DocumentCheckInRefusalV1::ActiveCheckpointChanged) => "active-checkpoint-changed",
                Some(DocumentCheckInRefusalV1::LedgerNotReplayable) => "ledger-not-replayable",
                Some(DocumentCheckInRefusalV1::CodecRefused) => "codec-refused",
                Some(DocumentCheckInRefusalV1::AuthorityChanged) => "authority-changed",
                Some(DocumentCheckInRefusalV1::Unavailable) | None => "unavailable",
            };
            span.refused(&end_cause.map_or_else(|| code.to_string(), |cause| format!("{code}: {cause}")));
        }
    }
}

async fn materialize_and_publish_check_in(
    state: &HubState,
    subject: &SocketSubjectV1,
    audience: &SocketAudienceV1,
    scope: &DocumentScope,
    request: &DocumentCheckInV1,
    job: &Arc<DocumentCheckInJob>,
    claim: &CheckInClaimGuardV1,
) -> Result<(ArtifactHash, ArtifactHash, EditedArtifactFrontierV1), CheckInEndV1> {
    let refuse = |error: AuthorityError| CheckInEndV1 { refusal: check_in_refusal_of_authority_error(&error), cause: Some(error.to_string()) };
    let context = OperationContext::stall_bounded(DOCUMENT_CHECK_IN_STALL_BOUND_MS, AuthorityLimits::maximum(), job.as_ref()).map_err(refuse)?;
    job.advance(DocumentCheckInPhaseV1::Materializing, 0);
    let Some(authority) = state.artifact_authority.as_ref() else { return Err(Some(DocumentCheckInRefusalV1::Unavailable).into()) };
    let head = directory::os_directory::io::binary::artifact_hash::decode_edited_artifact_frontier_v1(&request.head).ok_or(Some(DocumentCheckInRefusalV1::UnknownHead))?;
    let descriptor = match state.directory.get_document_descriptor(scope).await {
        Ok(Some(descriptor)) => descriptor,
        Ok(None) => return Err(Some(DocumentCheckInRefusalV1::AuthorityChanged).into()),
        Err(_) => return Err(Some(DocumentCheckInRefusalV1::Unavailable).into()),
    };
    let current = match state.directory.get_active_artifact_checkpoint(scope).await {
        Ok(Some(current)) => current,
        Ok(None) | Err(_) => return Err(Some(DocumentCheckInRefusalV1::Unavailable).into()),
    };
    let baseline = &current.baseline_frontier;
    if head == *baseline {
        let parent = current.parent_checkpoint_id.ok_or(Some(DocumentCheckInRefusalV1::Unavailable))?;
        return Ok((current.checkpoint_id, parent, request.head.clone()));
    }
    if head.head_edit_ordinal < baseline.head_edit_ordinal || head.last_commit_seq < baseline.last_commit_seq {
        return Err(Some(DocumentCheckInRefusalV1::StaleHead).into());
    }
    if head.head_edit_ordinal == baseline.head_edit_ordinal || head.last_commit_seq == baseline.last_commit_seq {
        return Err(Some(DocumentCheckInRefusalV1::UnknownHead).into());
    }
    context.checkpoint().map_err(refuse)?;
    let pair_control = CheckInPairControlV1(job.as_ref());
    let pair_context = RebootstrapContext::new(context.now_ms().saturating_add(REBOOTSTRAP_DEADLINE_MS), &pair_control);
    let active = match state.rebootstrap.active_pair(scope, &pair_context).await {
        Ok(active) => active,
        Err(RebootstrapError::Cancelled) => return Err(None.into()),
        Err(_) => return Err(Some(DocumentCheckInRefusalV1::Unavailable).into()),
    };
    if active.selection.active_checkpoint_id != current.checkpoint_id {
        return Err(Some(DocumentCheckInRefusalV1::ActiveCheckpointChanged).into());
    }
    job.advance(DocumentCheckInPhaseV1::Materializing, 1);
    let storage = state.db.storage().await;
    let wal = storage.wal().await;
    let commits = match db::document::artifact_ledger_tail(&wal, &db_core_document_id(&db_artifact_id(scope)), &check_in_ledger_point(baseline), &check_in_ledger_point(&head), job.cancellation()).await {
        Ok(commits) => commits,
        Err(_) if AuthorityOperationControl::is_cancelled(job.as_ref()) => return Err(None.into()),
        Err(error) => return Err(CheckInEndV1 { refusal: check_in_refusal_of_ledger_error(&error), cause: Some(error.to_string()) }),
    };
    drop(wal);
    drop(storage);
    let envelopes: Vec<MutationEnvelope> = commits.into_iter().flat_map(|commit| commit.envelopes).collect();
    job.advance(DocumentCheckInPhaseV1::Materializing, 2);
    let candidate = authority
        .materialize_check_in(
            CheckInMaterialization { descriptor: descriptor.clone(), scope: scope.clone(), parent_checkpoint_id: current.checkpoint_id, base_pair: active.pair().clone(), head: head.clone(), envelopes: directory::os_spr::encode_envelopes(&envelopes) },
            &context,
        )
        .await
        .map_err(refuse)?;
    #[cfg(test)]
    if let Some(gate) = state.live_gate.as_ref().filter(|gate| gate.check_in_pause_enabled.load(std::sync::atomic::Ordering::Acquire)) {
        gate.check_in_admitted.add_permits(1);
        gate.check_in_release.acquire().await.expect("check-in test release").forget();
    }
    context.checkpoint().map_err(refuse)?;
    let publisher = FencedCheckInPublisherV1 {
        state: state.clone(),
        subject: subject.clone(),
        audience: audience.clone(),
        scope: scope.clone(),
        descriptor,
        parent: current.clone(),
        completion: CheckpointPublicationCompletionV1 { actor_user_id: claim.actor_user_id.clone(), correlation_id: claim.correlation_id.clone(), command_sha256: claim.command_sha256.clone(), checkpoint_id: ArtifactHash([0; 32]), completed_at: 0 },
    };
    let published = match CheckpointPublicationOrchestrator::new(ArtifactChunkBlobStore::new(state.artifact_cas.clone()), publisher).publish_candidate(candidate, &context).await {
        Ok(published) => published,
        Err(_) if subject.revalidate(state.directory.as_ref(), audience, now_ms()).await != SocketBindingValidityV1::Active => return Err(Some(DocumentCheckInRefusalV1::AuthorityChanged).into()),
        Err(error) => return Err(refuse(error)),
    };
    Ok((published.checkpoint.checkpoint_id, current.checkpoint_id, request.head.clone()))
}

/// 🔐️ The author of one document write: a current space author's session, never a share and never a
/// spectator. `401` for no credential, `403` for a member who may not write.
pub async fn check_in_author(state: &HubState, scope: &DocumentScope, headers: &HeaderMap) -> Result<(SocketSubjectV1, String), StatusCode> {
    let bearer = socket_issue_bearer(headers)?;
    let subject = match authenticate_document_credential(state, scope, &bearer).await {
        Ok((subject, _)) => subject,
        Err(DocumentOpenPlanErrorCodeV1::DeadlineExceeded) => return Err(StatusCode::SERVICE_UNAVAILABLE),
        Err(_) if resolve_bearer_user(state, Some(&bearer)).await.is_some() => return Err(StatusCode::FORBIDDEN),
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };
    if !access_permits_in_space(state, &subject.access_roles(), DirectoryAccessActionV1::DocumentCheckIn, &scope.space_id).await {
        return Err(StatusCode::FORBIDDEN);
    }
    let SocketSubjectV1::Session { user_id, .. } = &subject else { return Err(StatusCode::FORBIDDEN) };
    let user_id = user_id.clone();
    Ok((subject, user_id))
}

/// 📌️ `POST /spaces/{space_id}/documents/{document_id}/check-ins` — Check In. The author names
/// one committed head of this document's ledger; the hub materializes the checkpoint from its own
/// log and makes it active. Answers the job's status (`202` while it runs, `200` once terminal); a
/// retry of the same request answers the same job, or after a restart the same durable checkpoint.
pub async fn post_document_check_in(Path((space_id, document_id)): Path<(String, String)>, OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Response {
    if uri.query().is_some() || headers.get_all(axum::http::header::CONTENT_TYPE).iter().count() != 1 || headers.get(axum::http::header::CONTENT_TYPE).and_then(|value| value.to_str().ok()) != Some("application/json") {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let Some(source) = std::str::from_utf8(&body).ok() else { return StatusCode::BAD_REQUEST.into_response() };
    let Some(request) = DocumentCheckInV1::parse_canonical_json(source) else { return StatusCode::BAD_REQUEST.into_response() };
    if request.head.document_id != document_id {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let scope = DocumentScope::new(space_id, document_id);
    let (subject, user_id) = match check_in_author(&state, &scope, &headers).await {
        Ok(author) => author,
        Err(status) => return status.into_response(),
    };
    if state.artifact_authority.is_none() {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }
    let command_sha256 = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(source.as_bytes()));
    let key = DocumentCheckInKey { user_id: user_id.clone(), space_id: scope.space_id.clone(), document_id: scope.document_id.clone(), request_id: request.request_id.clone() };
    let job = match state.check_ins.admit(key.clone(), &command_sha256) {
        DocumentCheckInAdmission::Owner(job) => job,
        DocumentCheckInAdmission::Existing(job) => return check_in_status_response(job.status()),
        DocumentCheckInAdmission::Conflict => return StatusCode::CONFLICT.into_response(),
        DocumentCheckInAdmission::Unavailable => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };
    let claim = NewCheckpointPublicationClaimV1 { actor_user_id: user_id.clone(), correlation_id: request.request_id.clone(), command_sha256: command_sha256.clone(), claimed_at: now_ms() };
    match state.directory_service.claim_or_read_checkpoint_publication(&claim).await {
        Ok(CheckpointPublicationClaimV1::Claimed(_)) => {}
        Ok(CheckpointPublicationClaimV1::Existing(record)) if record.disposition == CheckpointPublicationDispositionV1::Completed => {
            let completed = match record.checkpoint_id {
                Some(checkpoint_id) => state.directory.get_artifact_checkpoint(&scope, checkpoint_id).await.ok().flatten(),
                None => None,
            };
            match completed.and_then(|checkpoint| Some((checkpoint.checkpoint_id, checkpoint.parent_checkpoint_id?, directory::os_directory::io::binary::artifact_hash::encode_edited_artifact_frontier_v1(&checkpoint.baseline_frontier)?))) {
                Some((checkpoint_id, parent, baseline)) if baseline == request.head => job.finish_ready(checkpoint_id, parent, baseline),
                _ => {
                    state.check_ins.forget(&key);
                    return StatusCode::CONFLICT.into_response();
                }
            }
            return check_in_status_response(job.status());
        }
        Ok(_) => {
            state.check_ins.forget(&key);
            return StatusCode::CONFLICT.into_response();
        }
        Err(_) => {
            state.check_ins.forget(&key);
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        }
    }
    let guard = CheckInClaimGuardV1 { service: state.directory_service.clone(), actor_user_id: user_id, correlation_id: request.request_id.clone(), command_sha256, complete: false };
    let accepted = job.status();
    tokio::spawn(run_document_check_in(state.clone(), subject, scope, request, job, guard));
    check_in_status_response(accepted)
}

/// 📣️ `GET /spaces/{space_id}/documents/{document_id}/check-ins/{request_id}` — one of the
/// caller's own Check Ins; another author's request id is indistinguishable from an unknown one.
pub async fn get_document_check_in(Path((space_id, document_id, request_id)): Path<(String, String, String)>, OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>) -> Response {
    if uri.query().is_some() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let scope = DocumentScope::new(space_id, document_id);
    let (_, user_id) = match check_in_author(&state, &scope, &headers).await {
        Ok(author) => author,
        Err(status) => return status.into_response(),
    };
    match state.check_ins.get(&DocumentCheckInKey { user_id, space_id: scope.space_id, document_id: scope.document_id, request_id }) {
        Some(job) => check_in_status_response(job.status()),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

/// 🛑️ `POST /spaces/{space_id}/documents/{document_id}/check-ins/{request_id}/cancel` — stops
/// the caller's own Check In at its next checkpoint; a published checkpoint is never withdrawn.
pub async fn post_document_check_in_cancel(Path((space_id, document_id, request_id)): Path<(String, String, String)>, OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Response {
    if uri.query().is_some() || !body.is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let scope = DocumentScope::new(space_id, document_id);
    let (_, user_id) = match check_in_author(&state, &scope, &headers).await {
        Ok(author) => author,
        Err(status) => return status.into_response(),
    };
    match state.check_ins.get(&DocumentCheckInKey { user_id, space_id: scope.space_id, document_id: scope.document_id, request_id }) {
        Some(job) => {
            job.cancel();
            check_in_status_response(job.status())
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
//#endregion 📌️CheckIn
//#endregion 🔖️Rest
