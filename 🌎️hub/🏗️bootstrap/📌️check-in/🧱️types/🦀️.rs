//! 🧱️ The `check_in` vocabulary that carries no `HubState`: types, ledgers and helpers the state and the routes build on.

use std::sync::Arc;
use semio_hub_artifact_authority::chunk_cas::{ArtifactChunkBlobStore, ArtifactChunkCasStores};
use directory::os_directory::{ArtifactFrontier, ArtifactHash, DocumentCheckInRefusalV1, DocumentCheckInStatusV1, DocumentDescriptor, DocumentScope, PublishedArtifactCheckpoint};
use semio_hub_artifact_authority::{AuthorityError, AuthorityLimits, AuthorityOperationControl, AuthorityProgress, CheckpointPublicationOrchestrator, CheckpointRequest, OperationContext, VerifiedCheckpointPublisher};
use semio_hub_directory_service::{DirectoryService, HubVerifiedCheckpointPublisher};
use semio_hub_artifact_checkin::DocumentCheckInJob;
use semio_hub_inference_runtime_contract::{GisMapApprovalCheckpointPublisherV1, GisMapApprovalCheckpointRequestV1, GisMapApprovalCommitErrorV1, GisMapDocumentWriteAuthorityV1};
use semio_hub_runtime::startup::HubArtifactAuthority;
use semio_hub_directory_dispatch::HubDirectories;
use semio_hub_lag_rebootstrap::{RebootstrapProgress, RebootstrapTransferControl};
use axum::response::Response;
use protocol::ServerFrame;
use semio_framework_hash::Sha256;
use semio_framework_async_slots::ShardedMap;
use axum::http::StatusCode;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::broadcast;
use directory::os_directory::io::binary::descriptor_digest::descriptor_digest_v1;
use semio_hub_runtime::presence::document_scope_key_v1;
use semio_hub_directory::{published_artifact_checkpoint, DocumentDirectory};
use crate::document_socket::wire_rebootstrap;

//#region 🔖️Rest
//#region 📣️CheckpointPublication
/// 📍️ The edited ledger point an actor-serialized snapshot names; `None` for genesis.
pub fn ledger_artifact_frontier(scope: &DocumentScope, snapshot: &db::CheckpointPublicationSnapshot) -> Option<ArtifactFrontier> {
    Some(ArtifactFrontier {
        document_id: scope.document_id.clone(),
        head_edit_ordinal: snapshot.frontier.head_seq,
        head_edit_id: snapshot.head_edit_id.as_ref()?.0.clone(),
        last_commit_seq: snapshot.frontier.commit_seq,
        chain_hash: ArtifactHash(snapshot.frontier.chain_hash),
    })
}

struct GisMapApprovalPublicationControlV1 {
    deadline_ms: u64,
}

impl AuthorityOperationControl for GisMapApprovalPublicationControlV1 {
    fn now_ms(&self) -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |duration| u64::try_from(duration.as_millis()).unwrap_or(u64::MAX))
    }

    fn is_cancelled(&self) -> bool {
        self.now_ms() > self.deadline_ms
    }

    fn report(&self, _progress: AuthorityProgress) {}
}

struct GisMapApprovalVerifiedPublisherV1 {
    directory: Arc<HubDirectories>,
    directory_service: Arc<DirectoryService>,
    artifact_cas: Arc<ArtifactChunkCasStores>,
    scope: DocumentScope,
    handle: db::ArtifactHandle,
    descriptor: DocumentDescriptor,
    descriptor_digest: ArtifactHash,
    expected_current: Option<PublishedArtifactCheckpoint>,
    expected_snapshot: db::CheckpointPublicationSnapshot,
}

impl GisMapApprovalVerifiedPublisherV1 {
    async fn authority_is_current(&self) -> Result<bool, AuthorityError> {
        let descriptor = self.directory.get_document_descriptor(&self.scope).await.map_err(|error| AuthorityError::Publication(error.to_string()))?;
        let current = self.directory.get_active_artifact_checkpoint(&self.scope).await.map_err(|error| AuthorityError::Publication(error.to_string()))?;
        let snapshot = self.handle.checkpoint_publication_snapshot().await.map_err(|error| AuthorityError::Publication(error.to_string()))?;
        Ok(descriptor.as_ref() == Some(&self.descriptor) && descriptor.as_ref().and_then(|value| descriptor_digest_v1(value).ok()) == Some(self.descriptor_digest) && current == self.expected_current && snapshot == self.expected_snapshot)
    }
}

impl VerifiedCheckpointPublisher for GisMapApprovalVerifiedPublisherV1 {
    async fn reserve(&self, plan: &semio_hub_artifact_authority::chunk_cas::ArtifactCasOwnershipPlanV1, context: &OperationContext<'_>) -> Result<semio_hub_artifact_authority::chunk_cas::ArtifactCasReservation, AuthorityError> {
        context.checkpoint()?;
        if !self.authority_is_current().await? {
            return Err(AuthorityError::Publication("GIS Map approval checkpoint authority changed".into()));
        }
        HubVerifiedCheckpointPublisher::new(self.directory_service.clone(), self.artifact_cas.clone(), "system:gis-map-approval").reserve(plan, context).await
    }

    async fn publish_reserved(&self, checkpoint: &directory::os_directory::ArtifactCheckpoint, reservation: &semio_hub_artifact_authority::chunk_cas::ArtifactCasReservation, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        if !self.authority_is_current().await?
            || checkpoint.scope != self.scope
            || checkpoint.descriptor_digest_v1 != self.descriptor_digest
            || checkpoint.baseline_frontier != ledger_artifact_frontier(&checkpoint.scope, &self.expected_snapshot).ok_or_else(|| AuthorityError::Publication("GIS Map approval frontier is incomplete".into()))?
        {
            return Err(AuthorityError::Publication("GIS Map approval checkpoint authority changed".into()));
        }
        HubVerifiedCheckpointPublisher::new(self.directory_service.clone(), self.artifact_cas.clone(), "system:gis-map-approval").publish_reserved(checkpoint, reservation, context).await
    }
}

pub struct GisMapApprovalCheckpointPublisherV1Impl {
    pub directory: Arc<HubDirectories>,
    pub directory_service: Arc<DirectoryService>,
    pub artifact_cas: Arc<ArtifactChunkCasStores>,
    pub authority: Arc<HubArtifactAuthority>,
    pub fanout: Arc<ShardedMap<String, broadcast::Sender<ServerFrame>>>,
    pub fanout_capacity: usize,
}

impl GisMapApprovalCheckpointPublisherV1Impl {
    fn matches_published(request: &GisMapApprovalCheckpointRequestV1, checkpoint: &PublishedArtifactCheckpoint) -> bool {
        let Some(frontier) = ledger_artifact_frontier(&request.scope, &request.actor_snapshot) else { return false };
        checkpoint.scope == request.scope
            && directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&checkpoint.descriptor_digest_v1) == request.descriptor_digest
            && checkpoint.baseline_frontier == frontier
            && checkpoint.pack.sha256 == ArtifactHash(Sha256::digest(&request.pair.pack))
            && checkpoint.pack.byte_length == request.pair.pack.len() as u64
            && checkpoint.spr.sha256 == ArtifactHash(Sha256::digest(&request.pair.spr))
            && checkpoint.spr.byte_length == request.pair.spr.len() as u64
    }

    pub fn current_matches_base(scope: &DocumentScope, base: &ArtifactFrontier, current: Option<&PublishedArtifactCheckpoint>) -> bool {
        if base.document_id != scope.document_id {
            return false;
        }
        match current {
            Some(checkpoint) => checkpoint.scope == *scope && checkpoint.baseline_frontier == *base,
            None => false,
        }
    }
}

impl GisMapApprovalCheckpointPublisherV1 for GisMapApprovalCheckpointPublisherV1Impl {
    fn publish<'a>(
        &'a self,
        request: GisMapApprovalCheckpointRequestV1,
        document_write: Arc<GisMapDocumentWriteAuthorityV1>,
        attempt_lifetime_ms: u64,
        _decision_now_ms: u64,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<PublishedArtifactCheckpoint, GisMapApprovalCommitErrorV1>> + Send + 'a>> {
        Box::pin(async move {
            let _document_write = document_write;
            let descriptor = self.directory.get_document_descriptor(&request.scope).await.map_err(|_| GisMapApprovalCommitErrorV1::Storage)?.ok_or(GisMapApprovalCommitErrorV1::Conflict)?;
            let descriptor_digest = descriptor_digest_v1(&descriptor).map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
            if directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&descriptor_digest) != request.descriptor_digest {
                return Err(GisMapApprovalCommitErrorV1::Conflict);
            }
            let current = self.directory.get_active_artifact_checkpoint(&request.scope).await.map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
            if current.as_ref().is_some_and(|checkpoint| Self::matches_published(&request, checkpoint)) {
                return Ok(current.expect("matching active checkpoint exists"));
            }
            if !Self::current_matches_base(&request.scope, &request.base_frontier, current.as_ref()) {
                return Err(GisMapApprovalCommitErrorV1::Conflict);
            }
            let post_frontier = ledger_artifact_frontier(&request.scope, &request.actor_snapshot).ok_or(GisMapApprovalCommitErrorV1::Conflict)?;
            if attempt_lifetime_ms == 0 || attempt_lifetime_ms > semio_hub_inference::schema::JOB_MAX_LIFETIME_MS {
                return Err(GisMapApprovalCommitErrorV1::Rejected);
            }
            let deadline_ms = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |duration| u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)).saturating_add(attempt_lifetime_ms);
            let control = GisMapApprovalPublicationControlV1 { deadline_ms };
            let context = OperationContext::new(deadline_ms, AuthorityLimits { max_operations: 1, max_operation_bytes: 1, max_pair_bytes: semio_hub_artifact_authority::AUTHORITY_MAX_PAIR_BYTES }, &control);
            let candidate = self
                .authority
                .materialize_checkpoint(
                    CheckpointRequest {
                        descriptor: descriptor.clone(),
                        scope: request.scope.clone(),
                        parent_checkpoint_id: current.as_ref().map(|checkpoint| checkpoint.checkpoint_id),
                        base_frontier: post_frontier,
                        input_pair: request.pair,
                        operations: Vec::new(),
                    },
                    &context,
                )
                .await
                .map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
            let publisher = GisMapApprovalVerifiedPublisherV1 {
                directory: self.directory.clone(),
                directory_service: self.directory_service.clone(),
                artifact_cas: self.artifact_cas.clone(),
                scope: request.scope,
                handle: request.handle,
                descriptor,
                descriptor_digest,
                expected_current: current,
                expected_snapshot: request.actor_snapshot,
            };
            let publication = CheckpointPublicationOrchestrator::new(ArtifactChunkBlobStore::new(self.artifact_cas.clone()), publisher);
            let published = publication.publish_candidate(candidate, &context).await.map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
            Ok(published_artifact_checkpoint(&published.checkpoint))
        })
    }

    fn checkpoint_applied(&self, checkpoint: &PublishedArtifactCheckpoint) -> Result<(), GisMapApprovalCommitErrorV1> {
        publish_gis_map_checkpoint_change(self.fanout.as_ref(), self.fanout_capacity, checkpoint);
        Ok(())
    }
}

pub fn publish_gis_map_checkpoint_change(fanout: &ShardedMap<String, broadcast::Sender<ServerFrame>>, fanout_capacity: usize, checkpoint: &PublishedArtifactCheckpoint) {
    let key = document_scope_key_v1(&checkpoint.scope);
    let sender = fanout.get_or_insert_with_cloned(key, || broadcast::channel(fanout_capacity).0);
    let control = directory::os_directory::RebootstrapRequired { scope: checkpoint.scope.clone(), checkpoint_id: checkpoint.checkpoint_id, descriptor_digest_v1: checkpoint.descriptor_digest_v1, baseline_frontier: checkpoint.baseline_frontier.clone() };
    let _ = sender.send(ServerFrame::RebootstrapRequired { control: wire_rebootstrap(&control) });
}
//#endregion 📣️CheckpointPublication
//#endregion 🔖️Rest

//#region 🔖️Rest
//#region 📌️CheckIn
/// 📍️ The db ledger point one directory frontier names; genesis has no edited tip.
pub fn check_in_ledger_point(frontier: &ArtifactFrontier) -> db::document::ArtifactLedgerPoint {
    if frontier.head_edit_ordinal == 0 && frontier.last_commit_seq == 0 && frontier.head_edit_id.is_empty() {
        return db::document::ArtifactLedgerPoint::genesis();
    }
    db::document::ArtifactLedgerPoint { head_seq: frontier.head_edit_ordinal, commit_seq: frontier.last_commit_seq, chain_hash: frontier.chain_hash.0, head_edit_id: Some(protocol::MutationId(frontier.head_edit_id.clone())) }
}

/// ⛔️ The refusal a ledger read maps to: a point that is not on this document's ledger is an unknown
/// head, an approval decision inside the range cannot be replayed, a cancelled walk is cancellation.
pub fn check_in_refusal_of_ledger_error(error: &db::DbError) -> Option<DocumentCheckInRefusalV1> {
    match error {
        db::DbError::NotFound(_) | db::DbError::InvalidArgument(_) => Some(DocumentCheckInRefusalV1::UnknownHead),
        db::DbError::Conflict(_) => Some(DocumentCheckInRefusalV1::LedgerNotReplayable),
        _ => Some(DocumentCheckInRefusalV1::Unavailable),
    }
}

/// 🌊️ Rebootstrap transfer control over the job, so the active pair read shares its cancellation.
pub struct CheckInPairControlV1<'a>(pub &'a DocumentCheckInJob);

impl RebootstrapTransferControl for CheckInPairControlV1<'_> {
    fn now_ms(&self) -> u64 {
        self.0.now_ms()
    }

    fn is_cancelled(&self) -> bool {
        AuthorityOperationControl::is_cancelled(self.0)
    }

    fn report(&self, _progress: RebootstrapProgress) {}
}

/// 🧾️ The durable claim of one Check In request; released unless completed with its checkpoint.
pub struct CheckInClaimGuardV1 {
    pub service: Arc<DirectoryService>,
    pub actor_user_id: String,
    pub correlation_id: String,
    pub command_sha256: String,
    pub complete: bool,
}

impl CheckInClaimGuardV1 {
    pub async fn release(&mut self) {
        if !self.complete && self.service.release_checkpoint_publication(&self.actor_user_id, &self.correlation_id, &self.command_sha256).await.is_ok() {
            self.complete = true;
        }
    }
}

impl Drop for CheckInClaimGuardV1 {
    fn drop(&mut self) {
        if self.complete {
            return;
        }
        let service = self.service.clone();
        let (actor_user_id, correlation_id, command_sha256) = (self.actor_user_id.clone(), self.correlation_id.clone(), self.command_sha256.clone());
        tokio::spawn(async move {
            let _ = service.release_checkpoint_publication(&actor_user_id, &correlation_id, &command_sha256).await;
        });
    }
}

/// ⛔️ Why one Check In ended without a checkpoint: its typed refusal (`None`: cancelled) and, when an authority or
/// ledger failure caused it, that failure in its own words — the span's detail after the refusal code, so a
/// `codec-refused` names the codec fault, the pair limit or the operation order that refused it.
pub struct CheckInEndV1 {
    pub refusal: Option<DocumentCheckInRefusalV1>,
    pub cause: Option<String>,
}

impl From<Option<DocumentCheckInRefusalV1>> for CheckInEndV1 {
    fn from(refusal: Option<DocumentCheckInRefusalV1>) -> Self {
        Self { refusal, cause: None }
    }
}

pub fn check_in_status_response(status: DocumentCheckInStatusV1) -> Response {
    let code = if status.phase.is_terminal() { StatusCode::OK } else { StatusCode::ACCEPTED };
    let Some(body) = status.canonical_json() else { return StatusCode::INTERNAL_SERVER_ERROR.into_response() };
    let mut response = (code, [(axum::http::header::CONTENT_TYPE, "application/json")], body).into_response();
    response.headers_mut().insert(axum::http::header::CACHE_CONTROL, axum::http::HeaderValue::from_static("private, no-store"));
    response
}
//#endregion 📌️CheckIn
//#endregion 🔖️Rest
