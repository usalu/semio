//! 🧑‍🏭️ Creation coordinates durable facts, one factory execution and the sole genesis publication.

use super::*;
use crate::artifact_authority::{CheckpointPublicationOrchestrator, VerifiedCheckpointPublisher};
use crate::artifact_authority::chunk_cas::{ArtifactCasOwnershipPlanV1, ArtifactCasReservation, ArtifactChunkBlobStore, ArtifactChunkCasStores};
use crate::artifact_authority::trusted_catalog::VerifiedTrustedCatalog;
use crate::directory::{DirectoryService, HubDirectory, HubVerifiedCheckpointPublisher};
use crate::directory::error::{DirectoryError, DirectoryResult};
use crate::directory::model::{SocketSessionBindingStatus, SpaceRole};
use directory::os_directory::schema::space_artifact_creation::{SpaceArtifactCreateV1, SpaceArtifactCreationCatalogV1, SpaceArtifactCreationPhaseV1, SpaceArtifactCreationStatusV1};
use std::sync::Arc;

/// 🏭️ An immutable admitted catalog and durable directory own all creation state.
pub struct ArtifactCreationServiceV1 {
    directory: Arc<DirectoryService>,
    catalog: Arc<VerifiedTrustedCatalog>,
    storage: Arc<ArtifactChunkCasStores>,
}

impl ArtifactCreationServiceV1 {
    /// 🧩️ No runtime status map or client-supplied factory participates in construction.
    pub fn new(directory: Arc<DirectoryService>, catalog: Arc<VerifiedTrustedCatalog>, storage: Arc<ArtifactChunkCasStores>) -> Self { Self { directory, catalog, storage } }

    async fn authorize(&self, actor: &ArtifactCreationActorV1, space_id: &str, now_ms: u64) -> DirectoryResult<()> {
        let now_ms = i64::try_from(now_ms).map_err(|_| DirectoryError::Unauthorized)?;
        if !matches!(self.directory.backend().socket_session_binding(&actor.session_id, &actor.user_id, actor.authorization_generation, Some(space_id), now_ms).await?, SocketSessionBindingStatus::Active { role: Some(SpaceRole::Author), .. }) || self.directory.backend().get_space(space_id).await?.is_none_or(|space| space.kind == "archive") { return Err(DirectoryError::Unauthorized); }
        Ok(())
    }

    /// 🗂️ Presents only current, verified, factory-backed choices after fresh Space authorization.
    pub async fn creation_catalog(&self, actor: &ArtifactCreationActorV1, space_id: &str, now_ms: u64) -> DirectoryResult<SpaceArtifactCreationCatalogV1> {
        self.authorize(actor, space_id, now_ms).await?;
        self.catalog.artifact_creation_catalog(space_id).ok_or_else(|| DirectoryError::Conflict("selected catalog has no verified creatable artifact kind".into()))
    }

    async fn read(&self, user_id: &str, space_id: &str, request_id: &str) -> DirectoryResult<ArtifactCreationOperationV1> {
        let facts = self.directory.backend().read_artifact_creation(user_id, request_id).await?;
        if facts.is_empty() { return Err(DirectoryError::NotFound("artifact creation request".into())); }
        let operation = ArtifactCreationOperationV1::fold(&facts)?;
        if operation.intent.scope.space_id != space_id || operation.intent.actor.user_id != user_id { return Err(DirectoryError::NotFound("artifact creation request".into())); }
        Ok(operation)
    }

    fn catalog_matches(&self, intent: &ArtifactCreationIntentV1) -> bool {
        self.catalog.generation_id() == intent.catalog_generation && self.catalog.artifact_creation_selection(&intent.request.kind_id).is_some_and(|selected| selected.package.plugin_id == intent.owner.plugin_id && selected.package.package_id == intent.owner.package_id && selected.package.version == intent.owner.version && selected.package.component_sha256 == intent.owner.package_hash && selected.artifact.schema == intent.artifact_schema && selected.artifact.pack_schema_hash == intent.pack_schema_hash && selected.parent_dialect == intent.parent_dialect)
    }

    /// 📨️ A duplicate request returns durable status; only the new accepted fact mints an execution grant.
    pub async fn accept(&self, actor: &ArtifactCreationActorV1, space_id: &str, request: SpaceArtifactCreateV1, context: &OperationContext<'_>) -> DirectoryResult<ArtifactCreationAcceptanceV1> {
        context.checkpoint().map_err(authority_error)?;
        self.authorize(actor, space_id, context.now_ms()).await?;
        let command_sha256 = artifact_creation_command_digest_v1(space_id, &request)?;
        let existing = self.directory.backend().read_artifact_creation(&actor.user_id, &request.request_id).await?;
        if !existing.is_empty() {
            let operation = ArtifactCreationOperationV1::fold(&existing)?;
            if operation.intent.command_sha256 != command_sha256 || operation.intent.scope.space_id != space_id { return Err(DirectoryError::Conflict("artifact creation request already names another intent".into())); }
            return Ok(ArtifactCreationAcceptanceV1 { status: operation.status(), execution: None });
        }
        let selected = self.catalog.artifact_creation_selection(&request.kind_id).ok_or_else(|| DirectoryError::Conflict("artifact creation kind has no verified native editor".into()))?;
        let mut entropy = [0; 16]; directory::os_identity::fill_entropy(&mut entropy).map_err(|_| DirectoryError::Backend("artifact creation identity entropy is unavailable".into()))?;
        if entropy == [0; 16] { return Err(DirectoryError::Backend("artifact creation identity entropy is empty".into())); }
        let accepted_at_ms = context.now_ms();
        let intent = ArtifactCreationIntentV1 { actor: actor.clone(), scope: DocumentScope::new(space_id, format!("artifact-{}", directory::os_directory::hex_lower(&entropy))), request, catalog_generation: self.catalog.generation_id().into(), owner: DocumentOwner { plugin_id: selected.package.plugin_id.clone(), package_id: selected.package.package_id.clone(), version: selected.package.version.clone(), package_hash: selected.package.component_sha256.clone() }, artifact_schema: selected.artifact.schema.clone(), pack_schema_hash: selected.artifact.pack_schema_hash.clone(), parent_dialect: selected.parent_dialect.clone(), command_sha256, accepted_at_ms, deadline_ms: accepted_at_ms.checked_add(ARTIFACT_CREATION_DEADLINE_MS).ok_or_else(|| DirectoryError::Conflict("artifact creation deadline overflows".into()))? };
        intent.validate()?;
        match self.directory.backend().claim_artifact_creation(&intent).await? {
            ArtifactCreationClaimV1::Accepted(operation) => Ok(ArtifactCreationAcceptanceV1 { status: operation.status(), execution: Some(ArtifactCreationExecutionV1 { intent: operation.intent }) }),
            ArtifactCreationClaimV1::Existing(operation) => Ok(ArtifactCreationAcceptanceV1 { status: operation.status(), execution: None }),
        }
    }

    /// 🔎️ Status reauthorizes its current caller and contains no private operation material.
    pub async fn status(&self, actor: &ArtifactCreationActorV1, space_id: &str, request_id: &str, now_ms: u64) -> DirectoryResult<SpaceArtifactCreationStatusV1> {
        self.authorize(actor, space_id, now_ms).await?;
        Ok(self.read(&actor.user_id, space_id, request_id).await?.status())
    }

    /// 🛑️ Cancellation competes with commit under the durable request writer, never overwriting Ready.
    pub async fn cancel(&self, actor: &ArtifactCreationActorV1, space_id: &str, request_id: &str, now_ms: u64) -> DirectoryResult<SpaceArtifactCreationStatusV1> {
        self.authorize(actor, space_id, now_ms).await?;
        let operation = self.read(&actor.user_id, space_id, request_id).await?;
        let append = ArtifactCreationFactAppendV1 { actor: actor.clone(), space_id: space_id.into(), request_id: request_id.into(), command_sha256: operation.intent.command_sha256, expected_revision: operation.revision, recorded_at_ms: now_ms, body: ArtifactCreationFactBodyV1::Cancelled };
        Ok(self.directory.backend().append_artifact_creation_fact(&append).await?.status())
    }

    async fn terminal(&self, intent: &ArtifactCreationIntentV1, cancelled: bool, now_ms: u64) -> DirectoryResult<SpaceArtifactCreationStatusV1> {
        let operation = self.read(&intent.actor.user_id, &intent.scope.space_id, &intent.request.request_id).await?;
        let append = ArtifactCreationFactAppendV1 { actor: intent.actor.clone(), space_id: intent.scope.space_id.clone(), request_id: intent.request.request_id.clone(), command_sha256: intent.command_sha256.clone(), expected_revision: operation.revision, recorded_at_ms: now_ms, body: if cancelled { ArtifactCreationFactBodyV1::Cancelled } else { ArtifactCreationFactBodyV1::Failed } };
        match self.directory.backend().append_artifact_creation_fact(&append).await {
            Ok(operation) => Ok(operation.status()),
            Err(DirectoryError::Unauthorized) => Ok(self.directory.backend().artifact_creation_terminate_uncommitted(intent, now_ms).await?.status()),
            Err(error) => Err(error),
        }
    }

    /// 📋️ A bounded durable scan supplies recovery work without a process-local status registry.
    pub async fn recovery_candidates(&self, now_ms: u64, limit: usize) -> DirectoryResult<Vec<ArtifactCreationIntentV1>> { self.directory.backend().artifact_creation_recovery_candidates(now_ms, limit).await }

    /// 🧯️ Expiry or conclusively lost authority closes an uncommitted key; only Prepared may resume publication.
    pub async fn recover<A: ArtifactCreationCommitAuthorityV1>(&self, intent: ArtifactCreationIntentV1, authority: &A, context: &OperationContext<'_>) -> DirectoryResult<SpaceArtifactCreationStatusV1> {
        context.checkpoint().map_err(authority_error)?;
        let operation = self.read(&intent.actor.user_id, &intent.scope.space_id, &intent.request.request_id).await?;
        if operation.intent != intent { return Err(DirectoryError::Conflict("artifact creation recovery intent differs".into())); }
        if matches!(operation.phase, SpaceArtifactCreationPhaseV1::Ready | SpaceArtifactCreationPhaseV1::Cancelled | SpaceArtifactCreationPhaseV1::Failed) { return Ok(operation.status()); }
        let expired = context.now_ms() >= intent.deadline_ms;
        let revoked = match self.authorize(&intent.actor, &intent.scope.space_id, context.now_ms()).await { Ok(()) => false, Err(DirectoryError::Unauthorized) => true, Err(error) => return Err(error) };
        if expired || revoked { return Ok(self.directory.backend().artifact_creation_terminate_uncommitted(&intent, context.now_ms()).await?.status()); }
        if operation.phase == SpaceArtifactCreationPhaseV1::Accepted { return Ok(operation.status()); }
        self.recover_prepared(intent, authority, context).await
    }

    /// 🌱️ Consumes the sole execution grant; a failed or interrupted factory is never retried under this key.
    pub async fn execute<A: ArtifactCreationCommitAuthorityV1>(&self, execution: ArtifactCreationExecutionV1, authority: &A, context: &OperationContext<'_>) -> DirectoryResult<SpaceArtifactCreationStatusV1> {
        let intent = execution.intent;
        let operation = self.read(&intent.actor.user_id, &intent.scope.space_id, &intent.request.request_id).await?;
        if operation.intent != intent { return Err(DirectoryError::Conflict("artifact creation execution identity differs".into())); }
        if operation.phase != SpaceArtifactCreationPhaseV1::Accepted { return Ok(operation.status()); }
        if !self.catalog_matches(&intent) { return self.terminal(&intent, false, context.now_ms()).await; }
        let mut limits = context.limits(); limits.max_pair_bytes = limits.max_pair_bytes.min(ARTIFACT_CREATION_PAIR_MAX_BYTES as u64);
        let bounded = OperationContext::new(intent.deadline_ms, limits, context.control);
        let candidate = match ValidatingCanonicalArtifactAuthority::new(self.catalog.clone()).materialize_genesis(ArtifactGenesisRequest { scope: intent.scope.clone(), kind_id: intent.request.kind_id.clone() }, &bounded).await {
            Ok(candidate) => candidate,
            Err(error) => return self.terminal(&intent, matches!(error, AuthorityError::Cancelled), context.now_ms()).await,
        };
        let prepared = ArtifactCreationPreparedV1 { descriptor: candidate.descriptor, checkpoint: candidate.candidate.checkpoint, pack: candidate.candidate.pair.pack, spr: candidate.candidate.pair.spr };
        prepared.validate(&intent)?;
        let append = ArtifactCreationFactAppendV1 { actor: intent.actor.clone(), space_id: intent.scope.space_id.clone(), request_id: intent.request.request_id.clone(), command_sha256: intent.command_sha256.clone(), expected_revision: operation.revision, recorded_at_ms: context.now_ms(), body: ArtifactCreationFactBodyV1::Prepared { candidate: prepared } };
        let operation = self.directory.backend().append_artifact_creation_fact(&append).await?;
        self.publish_prepared(operation, authority, &bounded).await
    }

    /// ♻️ Recovery consumes only exact durable Prepared bytes, never a lost factory execution grant.
    pub async fn recover_prepared<A: ArtifactCreationCommitAuthorityV1>(&self, intent: ArtifactCreationIntentV1, authority: &A, context: &OperationContext<'_>) -> DirectoryResult<SpaceArtifactCreationStatusV1> {
        let operation = self.read(&intent.actor.user_id, &intent.scope.space_id, &intent.request.request_id).await?;
        if operation.intent != intent || operation.phase == SpaceArtifactCreationPhaseV1::Accepted { return Err(DirectoryError::Conflict("artifact creation recovery cannot execute an unprepared factory".into())); }
        let mut limits = context.limits(); limits.max_pair_bytes = limits.max_pair_bytes.min(ARTIFACT_CREATION_PAIR_MAX_BYTES as u64);
        self.publish_prepared(operation, authority, &OperationContext::new(intent.deadline_ms, limits, context.control)).await
    }

    async fn publish_prepared<A: ArtifactCreationCommitAuthorityV1>(&self, operation: ArtifactCreationOperationV1, authority: &A, context: &OperationContext<'_>) -> DirectoryResult<SpaceArtifactCreationStatusV1> {
        if operation.phase != SpaceArtifactCreationPhaseV1::Preparing { return Ok(operation.status()); }
        let intent = &operation.intent;
        if !self.catalog_matches(intent) { return self.terminal(intent, false, context.now_ms()).await; }
        let prepared = operation.prepared.as_ref().ok_or_else(|| DirectoryError::Conflict("creation has no prepared bytes".into()))?;
        prepared.validate(intent)?;
        let publisher = GenesisPublisher { owner: self, intent, prepared, authority };
        let candidate = CheckpointCandidate { checkpoint: prepared.checkpoint.clone(), pair: ArtifactPair { pack: prepared.pack.clone(), spr: prepared.spr.clone() } };
        match CheckpointPublicationOrchestrator::new(ArtifactChunkBlobStore::new(self.storage.clone()), publisher).publish_candidate(candidate, context).await {
            Ok(_) => Ok(self.read(&intent.actor.user_id, &intent.scope.space_id, &intent.request.request_id).await?.status()),
            Err(AuthorityError::Publication(_)) => {
                let current = self.read(&intent.actor.user_id, &intent.scope.space_id, &intent.request.request_id).await?;
                if matches!(current.phase, SpaceArtifactCreationPhaseV1::Ready | SpaceArtifactCreationPhaseV1::Cancelled | SpaceArtifactCreationPhaseV1::Failed) { return Ok(current.status()); }
                Ok(SpaceArtifactCreationStatusV1 { schema: "semio.hub.space-artifact-creation-status/v1".into(), request_id: intent.request.request_id.clone(), space_id: intent.scope.space_id.clone(), phase: SpaceArtifactCreationPhaseV1::Indeterminate, ready: None })
            }
            Err(error) => self.terminal(intent, matches!(error, AuthorityError::Cancelled), context.now_ms()).await,
        }
    }
}

fn authority_error(error: AuthorityError) -> DirectoryError { DirectoryError::Backend(error.to_string()) }

struct GenesisPublisher<'a, A> { owner: &'a ArtifactCreationServiceV1, intent: &'a ArtifactCreationIntentV1, prepared: &'a ArtifactCreationPreparedV1, authority: &'a A }

impl<A: ArtifactCreationCommitAuthorityV1> VerifiedCheckpointPublisher for GenesisPublisher<'_, A> {
    async fn reserve(&self, plan: &ArtifactCasOwnershipPlanV1, context: &OperationContext<'_>) -> Result<ArtifactCasReservation, AuthorityError> {
        context.checkpoint()?;
        self.owner.authorize(&self.intent.actor, &self.intent.scope.space_id, context.now_ms()).await.map_err(|error| AuthorityError::Publication(error.to_string()))?;
        HubVerifiedCheckpointPublisher::new(self.owner.directory.clone(), self.owner.storage.clone(), "system:artifact-creation").reserve(plan, context).await
    }

    async fn publish_reserved(&self, checkpoint: &ArtifactCheckpoint, reservation: &ArtifactCasReservation, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        let lease = self.authority.acquire(&self.intent.actor, &self.intent.scope.space_id).await.map_err(|error| AuthorityError::Publication(error.to_string()))?;
        context.checkpoint()?;
        if !self.owner.catalog_matches(self.intent) { return Err(AuthorityError::Publication("artifact creation catalog changed".into())); }
        let result = self.owner.directory.publish_document_genesis(self.intent.clone(), self.prepared, checkpoint.clone(), reservation.clone(), context.now_ms()).await.map(|_| ()).map_err(|error| AuthorityError::Publication(error.to_string()));
        drop(lease); result
    }
}
