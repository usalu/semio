//! 🌐️ `HubDirectory` over Neo4j (neo4rs). Users/Spaces/Memberships are real nodes+relationships —
//! where graph traversal earns its keep (role lookups, VFS tree walks). Document persistence and
//! blobs are no longer this crate's concern — `db::Database` and `db_storage_neo4j` own that now
//! (see `bin.rs`). `#[cfg(feature = "neo4j")]`-gated as a whole by the parent `directory` module
//! (see `📇️directory/🦀️.rs`'s `//#region 🔖️Backends`).

use crate::artifact_authority::chunk_cas::{ArtifactCasDeleteFence, ArtifactCasObjectKey, ArtifactCasOwnershipPlanV1, ArtifactCasReservation, decode_artifact_cas_ownership_v1, encode_artifact_cas_ownership_v1, validate_artifact_cas_publication_v1};
use crate::artifact_authority::creation::{
    ArtifactCreationActorV1, ArtifactCreationClaimV1, ArtifactCreationFactAppendV1, ArtifactCreationFactBodyV1, ArtifactCreationFactV1, ArtifactCreationIntentV1, ArtifactCreationOperationV1, DocumentGenesisAppendV1, DocumentGenesisCommitV1,
    decide_artifact_creation_fact_append_v1,
};
use crate::directory::error::{DirectoryError, DirectoryResult};
use crate::directory::model::*;
use crate::directory::{
    ADMIN_PAGE_MAX, ARTIFACT_CAS_RESERVATION_MAX_TTL_MS, ARTIFACT_CAS_SWEEP_PAGE_MAX, ARTIFACT_CHECKPOINT_LINEAGE_MAX, AUTH_AUDIT_PAGE_MAX, AUTH_TEXT_MAX_BYTES, ArtifactCasSweepCandidatePage, DIRECTORY_WIRE_INTEGER_MAX, DirectoryAppendOutcomeV1,
    DirectoryProjectionRejectionV1, HubClock, HubDirectory, InviteCapability, InviteRedemptionPreflight, InviteRedemptionScopeHintV1, InviteRedemptionSpaceStateV1, NewDirectoryEvent, ProjectionRebuildControl, SessionCapability, ShareCapability,
    UNCONTROLLED_PROJECTION_REBUILD, active_capability, admin_operation_effect_receipt_v1, auth_audit, bounded_event_read, checkpoint_projection_rebuild, decode_auth_digest_hex, directory_command_result_kind_from_str,
    directory_command_result_kind_str, directory_projection_rejection_v1, directory_projection_space_v1, document_genesis_completion_v1, encode_capability_bytes, invite_redemption_preflight, kind_to_str, prepare_auth_session, prepare_invite,
    prepare_share_token, role_from_wire, role_to_wire, same_admin_operation_request, validate_admin_operation_audit, validate_admin_operation_effect_receipt, validate_bounded_auth_text, validate_checkpoint_publication_claim,
    validate_checkpoint_publication_completion, validate_directory_command_claim, validate_document_genesis_append_v1, validate_verified_checkpoint_append, verify_invite_redemption_event, verify_invite_redemption_scope_hint, visibility_to_str,
};
use directory::os_directory::{
    ArtifactCheckpoint, ArtifactHash, ArtifactRetention, DirectoryActor, DirectoryActorKind, DirectoryEvent, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, DocumentDescriptor, Hlc, PublishedArtifactCheckpoint,
    hex_lower, validate_directory_event_page_event,
};
use directory::os_identity::time_ordered_id;
use directory::{FromValue, ToValue};
use neo4rs::{Graph, Txn, query};
use semio_framework_hash::Sha256;

fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
}

fn backend<E: std::fmt::Display>(err: E) -> DirectoryError {
    DirectoryError::Backend(err.to_string())
}

fn artifact_creation_request_key(actor_user_id: &str, request_id: &str) -> String {
    format!("v1:{}:{}:{}{}", actor_user_id.len(), request_id.len(), actor_user_id, request_id)
}

fn artifact_creation_phase(body: &ArtifactCreationFactBodyV1) -> &'static str {
    match body {
        ArtifactCreationFactBodyV1::Accepted { .. } => "accepted",
        ArtifactCreationFactBodyV1::Prepared { .. } => "prepared",
        ArtifactCreationFactBodyV1::Committed { .. } => "committed",
        ArtifactCreationFactBodyV1::Cancelled => "cancelled",
        ArtifactCreationFactBodyV1::Failed => "failed",
    }
}

async fn lock_artifact_creation_request(txn: &mut Txn, actor_user_id: &str, request_id: &str) -> DirectoryResult<String> {
    let key = artifact_creation_request_key(actor_user_id, request_id);
    let mut result = txn
        .execute(
            query("MERGE (r:ArtifactCreationRequest {key: $key}) ON CREATE SET r.actorUserId = $actor_user_id, r.requestId = $request_id, r.lockNonce = 0 SET r.lockNonce = r.lockNonce + 1 RETURN r.key AS key")
                .param("key", key.clone())
                .param("actor_user_id", actor_user_id)
                .param("request_id", request_id),
        )
        .await
        .map_err(backend)?;
    result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact creation request lock returned no row".into()))?;
    drop(result);
    Ok(key)
}

async fn artifact_creation_facts(txn: &mut Txn, request_key: &str) -> DirectoryResult<Vec<ArtifactCreationFactV1>> {
    let mut result = txn.execute(query("MATCH (:ArtifactCreationRequest {key: $key})-[:HAS_FACT]->(f:ArtifactCreationFact) RETURN f.payload AS payload ORDER BY f.revision LIMIT 4").param("key", request_key)).await.map_err(backend)?;
    let mut facts = Vec::new();
    while let Some(row) = result.next(txn.handle()).await.map_err(backend)? {
        facts.push(directory::os_pack::json::from_json_str(&row.get::<String>("payload").map_err(backend)?).map_err(backend)?);
    }
    Ok(facts)
}

async fn validate_artifact_creation_authority(txn: &mut Txn, actor: &ArtifactCreationActorV1, space_id: &str, observed_now_override: Option<u64>) -> DirectoryResult<u64> {
    let mut result = txn
        .execute(
            query(
                "MATCH (a:AuthSession {id: $session_id})-[:BELONGS_TO]->(u:User {id: $user_id}), (s:Space {id: $space_id})
         OPTIONAL MATCH (u)-[m:MEMBER_OF]->(s)
         SET u.creationLockNonce = coalesce(u.creationLockNonce, 0) + 1, a.creationLockNonce = coalesce(a.creationLockNonce, 0) + 1,
             s.creationLockNonce = coalesce(s.creationLockNonce, 0) + 1
         FOREACH (_ IN CASE WHEN m IS NULL THEN [] ELSE [1] END | SET m.creationLockNonce = coalesce(m.creationLockNonce, 0) + 1)
         RETURN a.authorizationGeneration AS generation, a.revokedAt AS revokedAt, a.expiresAt AS expiresAt, s.kind AS spaceKind, m.role AS role",
            )
            .param("session_id", actor.session_id.clone())
            .param("user_id", actor.user_id.clone())
            .param("space_id", space_id),
        )
        .await
        .map_err(backend)?;
    let row = result.next(txn.handle()).await.map_err(backend)?.ok_or(DirectoryError::Unauthorized)?;
    let generation: i64 = row.get("generation").map_err(backend)?;
    let revoked: Option<i64> = row.get("revokedAt").map_err(backend)?;
    let expires_at: i64 = row.get("expiresAt").map_err(backend)?;
    let space_kind: String = row.get("spaceKind").map_err(backend)?;
    let role: Option<String> = row.get("role").map_err(backend)?;
    drop(result);
    let observed_now = observed_now_override.unwrap_or(u64::try_from(now_ms()).map_err(backend)?);
    if generation != i64::try_from(actor.authorization_generation).map_err(backend)? || revoked.is_some() || space_kind == "archive" || role.as_deref() != Some("author") || expires_at <= i64::try_from(observed_now).map_err(backend)? {
        return Err(DirectoryError::Unauthorized);
    }
    Ok(observed_now)
}

async fn insert_artifact_creation_fact(txn: &mut Txn, request_key: &str, intent: &ArtifactCreationIntentV1, fact: &ArtifactCreationFactV1) -> DirectoryResult<()> {
    let payload = directory::os_pack::json::to_json_string(fact);
    if payload.len() > 8 * 1024 * 1024 {
        return Err(DirectoryError::Conflict("artifact creation fact exceeds its bounded envelope".into()));
    }
    let revision = i64::try_from(fact.revision).map_err(backend)?;
    txn.run(query(
        "MATCH (r:ArtifactCreationRequest {key: $request_key})
         CREATE (f:ArtifactCreationFact {requestRevisionKey: $request_revision_key, actorUserId: $actor_user_id, requestId: $request_id, revision: $revision, phase: $phase, spaceId: $space_id, documentId: $document_id, deadlineMs: $deadline_ms, recordedAtMs: $recorded_at_ms, payload: $payload})
         CREATE (r)-[:HAS_FACT]->(f)")
        .param("request_key", request_key)
        .param("request_revision_key", format!("{request_key}:{revision}"))
        .param("actor_user_id", fact.actor_user_id.clone())
        .param("request_id", fact.request_id.clone())
        .param("revision", revision)
        .param("phase", artifact_creation_phase(&fact.body))
        .param("space_id", intent.scope.space_id.clone())
        .param("document_id", intent.scope.document_id.clone())
        .param("deadline_ms", i64::try_from(intent.deadline_ms).map_err(backend)?)
        .param("recorded_at_ms", i64::try_from(fact.recorded_at_ms).map_err(backend)?)
        .param("payload", payload)).await.map_err(backend)?;
    Ok(())
}

enum AdminEffectPreflightFailure {
    RejectedBeforeCommit,
    Indeterminate,
}

impl<T> From<AdminEffectPreflightFailure> for AdminEffectCommitV1<T> {
    fn from(value: AdminEffectPreflightFailure) -> Self {
        match value {
            AdminEffectPreflightFailure::RejectedBeforeCommit => Self::RejectedBeforeCommit,
            AdminEffectPreflightFailure::Indeterminate => Self::Indeterminate,
        }
    }
}

fn admin_effect_preflight<T>(result: DirectoryResult<T>) -> Result<T, AdminEffectPreflightFailure> {
    match result {
        Ok(value) => Ok(value),
        Err(DirectoryError::Backend(_)) => Err(AdminEffectPreflightFailure::Indeterminate),
        Err(_) => Err(AdminEffectPreflightFailure::RejectedBeforeCommit),
    }
}

async fn admin_effect_rollback<T>(txn: Txn) -> AdminEffectCommitV1<T> {
    match txn.rollback().await {
        Ok(()) => AdminEffectCommitV1::RejectedBeforeCommit,
        Err(_) => AdminEffectCommitV1::Indeterminate,
    }
}

macro_rules! admin_effect_try {
    ($txn:ident, $result:expr) => {
        match $result {
            Ok(value) => value,
            Err(_) => return admin_effect_rollback($txn).await,
        }
    };
}

async fn insert_auth_audit(txn: &mut Txn, event: &AuthAuditRecord) -> DirectoryResult<()> {
    txn.run(
        query("CREATE (:AuthAudit {id: $id, occurredAt: $occurred_at, eventKind: $event_kind, authSessionId: $auth_session_id, targetUserId: $target_user_id, actorUserId: $actor_user_id, provider: $provider, outcomeCode: $outcome_code, reasonCode: $reason_code, correlationId: $correlation_id, peerClass: $peer_class})")
            .param("id", event.id.clone())
            .param("occurred_at", event.occurred_at)
            .param("event_kind", event.event_kind.clone())
            .param("auth_session_id", event.auth_session_id.clone().unwrap_or_default())
            .param("target_user_id", event.target_user_id.clone().unwrap_or_default())
            .param("actor_user_id", event.actor_user_id.clone().unwrap_or_default())
            .param("provider", event.provider.clone().unwrap_or_default())
            .param("outcome_code", event.outcome_code.clone())
            .param("reason_code", event.reason_code.clone().unwrap_or_default())
            .param("correlation_id", event.correlation_id.clone())
            .param("peer_class", event.peer_class.clone()),
    )
    .await
    .map_err(backend)?;
    Ok(())
}

async fn insert_admin_operation_effect_receipt(txn: &mut Txn, receipt: &AdminOperationEffectReceiptV1) -> DirectoryResult<()> {
    validate_admin_operation_effect_receipt(receipt)?;
    let mut result = txn
        .execute(
            query(
                "MERGE (r:AdminOperationEffectReceipt {operationId: $operation_id})
                 ON CREATE SET r.intentDigest = $intent_digest, r.committedAt = $committed_at, r.outcomeCode = $outcome_code, r.eventSeqFirst = $event_seq_first, r.eventSeqLast = $event_seq_last
                 RETURN r AS r",
            )
            .param("operation_id", receipt.operation_id.clone())
            .param("intent_digest", receipt.intent_digest.clone())
            .param("committed_at", receipt.committed_at)
            .param("outcome_code", receipt.outcome_code.clone())
            .param("event_seq_first", receipt.event_seq_first.map(|value| i64::try_from(value).map_err(backend)).transpose()?.unwrap_or(0))
            .param("event_seq_last", receipt.event_seq_last.map(|value| i64::try_from(value).map_err(backend)).transpose()?.unwrap_or(0)),
        )
        .await
        .map_err(backend)?;
    let row = result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Conflict("admin operation effect receipt disappeared".into()))?;
    let node: neo4rs::Node = row.get("r").map_err(backend)?;
    let established = AdminOperationEffectReceiptV1 {
        operation_id: node.get("operationId").map_err(backend)?,
        intent_digest: node.get("intentDigest").map_err(backend)?,
        committed_at: node.get("committedAt").map_err(backend)?,
        outcome_code: node.get("outcomeCode").map_err(backend)?,
        event_seq_first: match node.get::<i64>("eventSeqFirst").map_err(backend)? {
            0 => None,
            value => Some(u64::try_from(value).map_err(backend)?),
        },
        event_seq_last: match node.get::<i64>("eventSeqLast").map_err(backend)? {
            0 => None,
            value => Some(u64::try_from(value).map_err(backend)?),
        },
    };
    drop(result);
    if &established == receipt { Ok(()) } else { Err(DirectoryError::Conflict("admin operation effect receipt identity changed".into())) }
}

fn actor_kind_to_str(kind: DirectoryActorKind) -> &'static str {
    match kind {
        DirectoryActorKind::User => "user",
        DirectoryActorKind::Admin => "admin",
        DirectoryActorKind::System => "system",
    }
}

fn actor_kind_from_str(value: &str) -> DirectoryActorKind {
    match value {
        "admin" => DirectoryActorKind::Admin,
        "system" => DirectoryActorKind::System,
        _ => DirectoryActorKind::User,
    }
}

fn document_scope_key_v1(scope: &DocumentScope) -> String {
    format!("v1:{}:{}:{}{}", scope.space_id.len(), scope.document_id.len(), scope.space_id, scope.document_id)
}

fn checkpoint_key_v1(scope: &DocumentScope, checkpoint_id: ArtifactHash) -> String {
    let scope_key = document_scope_key_v1(scope);
    format!("v1:{}:{}{}", scope_key.len(), scope_key, hex_lower(&checkpoint_id.0))
}

fn cas_object_token(key: &ArtifactCasObjectKey) -> String {
    format!("{}:{}", key.kind.name(), hex_lower(&key.digest.0))
}

async fn cas_generation(txn: &mut Txn) -> DirectoryResult<i64> {
    let mut result = txn.execute(query("MERGE (h:ArtifactCasLedgerHead {id: 'singleton'}) ON CREATE SET h.generation = 0 SET h.generation = h.generation + 1 RETURN h.generation AS generation")).await.map_err(backend)?;
    let generation = result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS ledger counter returned no row".into()))?.get("generation").map_err(backend)?;
    drop(result);
    Ok(generation)
}

async fn cas_lock_space(txn: &mut Txn, space_id: &str) -> DirectoryResult<i64> {
    let mut result = txn
        .execute(query("MERGE (b:ArtifactCasSpaceBarrier {spaceId: $space_id}) ON CREATE SET b.lockNonce = 0 SET b.lockNonce = b.lockNonce + 1 RETURN coalesce(b.leaseExpiresAtMs, 0) AS leaseExpiresAtMs").param("space_id", space_id))
        .await
        .map_err(backend)?;
    let expires_at_ms = result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS space barrier returned no row".into()))?.get("leaseExpiresAtMs").map_err(backend)?;
    drop(result);
    Ok(expires_at_ms)
}

async fn cas_reservation_barrier(txn: &mut Txn, space_id: &str, now_ms: i64) -> DirectoryResult<([u8; 32], u64)> {
    let lease_expires_at_ms = cas_lock_space(txn, space_id).await?;
    if lease_expires_at_ms > now_ms {
        return Err(DirectoryError::Conflict("artifact CAS deletion lease is active for this space".into()));
    }
    let mut epoch_result = txn
        .execute(query("MATCH (b:ArtifactCasSpaceBarrier {spaceId: $space_id}) SET b.fenceEpoch = coalesce(b.fenceEpoch, 0) + 1 REMOVE b.leaseToken, b.leaseExpiresAtMs RETURN b.fenceEpoch AS epoch").param("space_id", space_id))
        .await
        .map_err(backend)?;
    let epoch: i64 = epoch_result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS barrier epoch returned no row".into()))?.get("epoch").map_err(backend)?;
    drop(epoch_result);
    let mut identity = txn.execute(query("MATCH (b:ArtifactCasBarrierIdentity {id: 'singleton'}) RETURN b.coordinatorId AS coordinator")).await.map_err(backend)?;
    let row = identity.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS barrier coordinator identity is missing".into()))?;
    let coordinator: neo4rs::BoltBytes = row.get("coordinator").map_err(backend)?;
    drop(identity);
    Ok((coordinator.value.to_vec().try_into().map_err(|_| DirectoryError::Backend("artifact CAS barrier coordinator identity is invalid".into()))?, u64::try_from(epoch).map_err(backend)?))
}

async fn cas_project_reserve(txn: &mut Txn, reservation: &ArtifactCasReservation) -> DirectoryResult<()> {
    let scope_key = checkpoint_key_v1(&reservation.plan.scope, reservation.plan.checkpoint_id);
    txn.run(query("MERGE (r:ArtifactCasReservation {scopeCheckpointKey: $key}) SET r.spaceId = $space_id, r.documentId = $document_id, r.checkpointId = $checkpoint_id, r.generation = $generation, r.writeEpoch = $write_epoch, r.expiresAtMs = $expires_at, r.plan = $plan, r.objects = $objects")
        .param("key", scope_key).param("space_id", reservation.plan.scope.space_id.clone()).param("document_id", reservation.plan.scope.document_id.clone()).param("checkpoint_id", hex_lower(&reservation.plan.checkpoint_id.0)).param("generation", i64::try_from(reservation.generation).map_err(backend)?).param("write_epoch", i64::try_from(reservation.write_epoch).map_err(backend)?).param("expires_at", i64::try_from(reservation.expires_at_ms).map_err(backend)?).param("plan", encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?).param("objects", reservation.plan.objects.iter().map(cas_object_token).collect::<Vec<_>>())).await.map_err(backend)?;
    Ok(())
}

async fn cas_project_publish(txn: &mut Txn, reservation: &ArtifactCasReservation, generation: i64) -> DirectoryResult<()> {
    let scope_key = checkpoint_key_v1(&reservation.plan.scope, reservation.plan.checkpoint_id);
    txn.run(query("MATCH (r:ArtifactCasReservation {scopeCheckpointKey: $key}) DELETE r").param("key", scope_key.clone())).await.map_err(backend)?;
    txn.run(
        query("CREATE (r:ArtifactCasReference {scopeCheckpointKey: $key, spaceId: $space_id, documentId: $document_id, checkpointId: $checkpoint_id, generation: $generation, writeEpoch: $write_epoch, plan: $plan, objects: $objects})")
            .param("key", scope_key)
            .param("space_id", reservation.plan.scope.space_id.clone())
            .param("document_id", reservation.plan.scope.document_id.clone())
            .param("checkpoint_id", hex_lower(&reservation.plan.checkpoint_id.0))
            .param("generation", generation)
            .param("write_epoch", i64::try_from(reservation.write_epoch).map_err(backend)?)
            .param("plan", encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?)
            .param("objects", reservation.plan.objects.iter().map(cas_object_token).collect::<Vec<_>>()),
    )
    .await
    .map_err(backend)?;
    Ok(())
}

async fn cas_project_release(txn: &mut Txn, operation: &str, space_id: &str, scope: Option<&DocumentScope>, checkpoint_id: Option<ArtifactHash>) -> DirectoryResult<()> {
    match operation {
        "retention" => {
            let scope = scope.ok_or_else(|| DirectoryError::Backend("artifact CAS retention scope missing".into()))?;
            let floor_key = checkpoint_key_v1(scope, checkpoint_id.ok_or_else(|| DirectoryError::Backend("artifact CAS retention checkpoint missing".into()))?);
            txn.run(query("MATCH (floor:ArtifactAuthorityEvent {scopeCheckpointKey: $floor_key}) MATCH (r:ArtifactCasReference {spaceId: $space_id, documentId: $document_id}) MATCH (published:ArtifactAuthorityEvent {scopeCheckpointKey: r.scopeCheckpointKey}) WHERE published.eventSeq < floor.eventSeq DETACH DELETE r")
                .param("floor_key", floor_key.clone()).param("space_id", space_id).param("document_id", scope.document_id.clone())).await.map_err(backend)?;
            txn.run(
                query("MATCH (floor:ArtifactAuthorityEvent {scopeCheckpointKey: $floor_key}) MATCH (p:ArtifactCheckpointPrivate {spaceId: $space_id, documentId: $document_id}) WHERE p.eventSeq < floor.eventSeq DETACH DELETE p")
                    .param("floor_key", floor_key)
                    .param("space_id", space_id)
                    .param("document_id", scope.document_id.clone()),
            )
            .await
            .map_err(backend)?;
        }
        "space-delete" => {
            txn.run(query("MATCH (r:ArtifactCasReservation {spaceId: $space_id}) DETACH DELETE r").param("space_id", space_id)).await.map_err(backend)?;
            txn.run(query("MATCH (r:ArtifactCasReference {spaceId: $space_id}) DETACH DELETE r").param("space_id", space_id)).await.map_err(backend)?;
        }
        _ => return Err(DirectoryError::Backend("invalid artifact CAS release operation".into())),
    }
    Ok(())
}

const CONSTRAINTS: &[&str] = &[
    "CREATE CONSTRAINT IF NOT EXISTS FOR (u:User) REQUIRE u.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (s:Space) REQUIRE s.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (d:DocumentDescriptor) REQUIRE d.scopeKey IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (i:DocumentIndex) REQUIRE i.scopeKey IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (c:ArtifactCheckpoint) REQUIRE c.scopeCheckpointKey IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (r:ArtifactRetention) REQUIRE r.scopeKey IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (p:ArtifactCheckpointPrivate) REQUIRE p.scopeCheckpointKey IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (a:ArtifactAuthorityEvent) REQUIRE a.eventSeq IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (a:ArtifactAuthorityEvent) REQUIRE a.scopeCheckpointKey IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (e:ArtifactCasLedgerEvent) REQUIRE e.generation IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (r:ArtifactCasReservation) REQUIRE r.scopeCheckpointKey IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (r:ArtifactCasReference) REQUIRE r.scopeCheckpointKey IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (b:ArtifactCasSpaceBarrier) REQUIRE b.spaceId IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (b:ArtifactCasBarrierIdentity) REQUIRE b.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (g:ShareGrant) REQUIRE g.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (g:ShareGrant) REQUIRE g.selector IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (a:AuthSession) REQUIRE a.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (a:AuthSession) REQUIRE a.selector IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (s:SyncSession) REQUIRE s.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (i:SpaceInvite) REQUIRE i.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (r:DirectoryCommandReceipt) REQUIRE r.key IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (r:CheckpointPublicationReceipt) REQUIRE r.key IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (i:SpaceInvite) REQUIRE i.selector IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (a:AuthAudit) REQUIRE a.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (a:AdminOperationAudit) REQUIRE a.sequence IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (a:AdminOperationAudit) REQUIRE a.requestTerminalKey IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (r:AdminOperationEffectReceipt) REQUIRE r.operationId IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (r:ArtifactCreationRequest) REQUIRE r.key IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (r:ArtifactCreationRequest) REQUIRE r.documentScopeKey IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (f:ArtifactCreationFact) REQUIRE f.requestRevisionKey IS UNIQUE",
    "CREATE INDEX IF NOT EXISTS FOR (f:ArtifactCreationFact) ON (f.phase, f.deadlineMs)",
    "CREATE INDEX IF NOT EXISTS FOR (a:AdminOperationAudit) ON (a.operationId)",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (c:AdminOperationAuditCounter) REQUIRE c.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (e:DirectoryEvent) REQUIRE e.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (e:DirectoryEvent) REQUIRE e.seq IS UNIQUE",
];

/// @emoji 🕸️ Neo4j-backed `HubDirectory`.
pub struct Neo4jDirectory {
    graph: Graph,
    #[cfg(test)]
    genesis_test_control: std::sync::Arc<ArtifactGenesisTestControlV1>,
}

#[cfg(test)]
#[derive(Default)]
struct ArtifactGenesisTestControlV1 {
    pause_before_authority: std::sync::atomic::AtomicBool,
    reached_before_authority: tokio::sync::Semaphore,
    resume_before_authority: tokio::sync::Semaphore,
    observed_now_ms: std::sync::atomic::AtomicU64,
    fail_commit_ack: std::sync::atomic::AtomicBool,
}

impl Neo4jDirectory {
    /// @emoji 🔌️ Connects to `uri` with `user`/`password` and bootstraps uniqueness constraints.
    pub async fn connect(uri: &str, user: &str, password: &str) -> DirectoryResult<Self> {
        let graph = Graph::new(uri, user, password).await.map_err(backend)?;
        for statement in CONSTRAINTS {
            graph.run(query(statement)).await.map_err(backend)?;
        }
        let mut identity = Sha256::new();
        identity.update(b"semio.hub.artifact-cas.barrier-identity.v1\0");
        identity.update(time_ordered_id().as_bytes());
        graph.run(query("MERGE (b:ArtifactCasBarrierIdentity {id: 'singleton'}) ON CREATE SET b.coordinatorId = $coordinator").param("coordinator", identity.finalize().to_vec())).await.map_err(backend)?;
        Ok(Self {
            graph,
            #[cfg(test)]
            genesis_test_control: std::sync::Arc::new(ArtifactGenesisTestControlV1::default()),
        })
    }

    #[cfg(test)]
    async fn pause_genesis_before_authority_for_test(&self) -> Option<u64> {
        if self.genesis_test_control.pause_before_authority.swap(false, std::sync::atomic::Ordering::SeqCst) {
            self.genesis_test_control.reached_before_authority.add_permits(1);
            self.genesis_test_control.resume_before_authority.acquire().await.expect("genesis test resume semaphore").forget();
        }
        let observed_now_ms = self.genesis_test_control.observed_now_ms.load(std::sync::atomic::Ordering::SeqCst);
        (observed_now_ms != 0).then_some(observed_now_ms)
    }

    async fn revoke_auth_sessions_by(
        &self,
        field: &str,
        key: &str,
        subject_digest: Option<[u8; 32]>,
        reason: &str,
        actor_user_id: Option<&str>,
        correlation_id: &str,
        admin_effect: Option<&NewAdminOperationEffectReceiptV1>,
    ) -> DirectoryResult<Vec<RevokedAuthSession>> {
        validate_bounded_auth_text(reason, "session revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut result = match field {
            "id" => {
                txn.execute(
                    query("MATCH (a:AuthSession {id: $key})-[:BELONGS_TO]->(u:User) WHERE a.revokedAt IS NULL SET a.revokedAt = $revoked_at, a.revokedReason = $reason, a.authorizationGeneration = a.authorizationGeneration + 1 RETURN a.id AS id, u.id AS userId, a.authorizationGeneration AS generation, a.identityProvider AS provider")
                        .param("key", key)
                        .param("revoked_at", revoked_at)
                        .param("reason", reason),
                )
                .await
                .map_err(backend)?
            }
            "user" => {
                txn.execute(
                    query("MATCH (a:AuthSession)-[:BELONGS_TO]->(u:User {id: $key}) WHERE a.revokedAt IS NULL SET a.revokedAt = $revoked_at, a.revokedReason = $reason, a.authorizationGeneration = a.authorizationGeneration + 1 RETURN a.id AS id, u.id AS userId, a.authorizationGeneration AS generation, a.identityProvider AS provider")
                        .param("key", key)
                        .param("revoked_at", revoked_at)
                        .param("reason", reason),
                )
                .await
                .map_err(backend)?
            }
            "identity" => {
                let digest = subject_digest.ok_or_else(|| DirectoryError::Backend("identity revocation requires subject digest".into()))?;
                txn.execute(
                    query("MATCH (a:AuthSession {identityProvider: $key, identitySubjectDigest: $digest})-[:BELONGS_TO]->(u:User) WHERE a.revokedAt IS NULL SET a.revokedAt = $revoked_at, a.revokedReason = $reason, a.authorizationGeneration = a.authorizationGeneration + 1 RETURN a.id AS id, u.id AS userId, a.authorizationGeneration AS generation, a.identityProvider AS provider")
                        .param("key", key)
                        .param("digest", encode_capability_bytes(&digest))
                        .param("revoked_at", revoked_at)
                        .param("reason", reason),
                )
                .await
                .map_err(backend)?
            }
            _ => return Err(DirectoryError::Backend("invalid auth revocation selector".into())),
        };
        let mut rows = Vec::new();
        while let Some(row) = result.next(txn.handle()).await.map_err(backend)? {
            let id: String = row.get("id").map_err(backend)?;
            let user_id: String = row.get("userId").map_err(backend)?;
            let provider: String = row.get("provider").map_err(backend)?;
            let generation: i64 = row.get("generation").map_err(backend)?;
            rows.push((id, user_id, provider, generation));
        }
        drop(result);
        let mut revoked = Vec::with_capacity(rows.len());
        for (id, user_id, provider, generation) in rows {
            let audit = auth_audit(revoked_at, "session-revoked", Some(&id), Some(&user_id), actor_user_id, Some(&provider), "success", Some(reason), correlation_id, "server")?;
            insert_auth_audit(&mut txn, &audit).await?;
            revoked.push(RevokedAuthSession { id, authorization_generation: u64::try_from(generation).map_err(backend)?, revoked_at });
        }
        if let Some(effect) = admin_effect {
            insert_admin_operation_effect_receipt(&mut txn, &admin_operation_effect_receipt_v1(effect, &[])?).await?;
        }
        txn.commit().await.map_err(backend)?;
        Ok(revoked)
    }

    async fn revoke_auth_sessions_by_user_with_admin_effect(&self, key: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<Vec<RevokedAuthSession>> {
        if let Err(outcome) = admin_effect_preflight(validate_bounded_auth_text(reason, "session revoke reason", AUTH_TEXT_MAX_BYTES)) {
            return outcome.into();
        }
        let revoked_at = now_ms();
        let mut txn = match self.graph.start_txn().await {
            Ok(txn) => txn,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let rows_result: DirectoryResult<Vec<(String, String, String, i64)>> = async {
            let mut result = txn
                .execute(
                    query("MATCH (a:AuthSession)-[:BELONGS_TO]->(u:User {id: $key}) WHERE a.revokedAt IS NULL SET a.revokedAt = $revoked_at, a.revokedReason = $reason, a.authorizationGeneration = a.authorizationGeneration + 1 RETURN a.id AS id, u.id AS userId, a.authorizationGeneration AS generation, a.identityProvider AS provider")
                        .param("key", key)
                        .param("revoked_at", revoked_at)
                        .param("reason", reason),
                )
                .await
                .map_err(backend)?;
            let mut rows = Vec::new();
            while let Some(row) = result.next(txn.handle()).await.map_err(backend)? {
                rows.push((row.get("id").map_err(backend)?, row.get("userId").map_err(backend)?, row.get("provider").map_err(backend)?, row.get("generation").map_err(backend)?));
            }
            drop(result);
            Ok(rows)
        }
        .await;
        let rows = admin_effect_try!(txn, rows_result);
        let mut revoked = Vec::with_capacity(rows.len());
        for (id, user_id, provider, generation) in rows {
            let audit = admin_effect_try!(txn, auth_audit(revoked_at, "session-revoked", Some(&id), Some(&user_id), actor_user_id, Some(&provider), "success", Some(reason), correlation_id, "server"));
            admin_effect_try!(txn, insert_auth_audit(&mut txn, &audit).await);
            let Ok(authorization_generation) = u64::try_from(generation) else { return admin_effect_rollback(txn).await };
            revoked.push(RevokedAuthSession { id, authorization_generation, revoked_at });
        }
        let receipt = admin_effect_try!(txn, admin_operation_effect_receipt_v1(effect, &[]));
        admin_effect_try!(txn, insert_admin_operation_effect_receipt(&mut txn, &receipt).await);
        match txn.commit().await {
            Ok(()) => AdminEffectCommitV1::Applied(revoked),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }

    /// @emoji 🌱️ Seeds a default `studio`/`private` space authored by a `seed` system user node,
    /// through the event log (`user.created` + `space.created` + `member.upserted`) like any other
    /// write.
    pub async fn seed(&self) -> DirectoryResult<()> {
        let mut existing = self.graph.execute(query("MATCH (u:User {id: 'seed'}) RETURN u.id AS id")).await.map_err(backend)?;
        if existing.next().await.map_err(backend)?.is_some() {
            return Ok(());
        }
        let actor = DirectoryActor { kind: DirectoryActorKind::System, id: "system:seed".into() };
        let mut clock = HubClock::new();
        let events = vec![
            NewDirectoryEvent { hlc: clock.tick(), actor: actor.clone(), space_id: None, user_id: Some("seed".into()), body: DirectoryEventBody::UserCreated { user_id: "seed".into(), email: "seed@localhost".into(), display_name: "System".into() } },
            NewDirectoryEvent {
                hlc: clock.tick(),
                actor: actor.clone(),
                space_id: Some("default".into()),
                user_id: Some("seed".into()),
                body: DirectoryEventBody::SpaceCreated { space_id: "default".into(), name: "Space".into(), space_kind: DirectorySpaceKind::Studio, visibility: DirectorySpaceVisibility::Private, owner_user_id: "seed".into() },
            },
            NewDirectoryEvent {
                hlc: clock.tick(),
                actor,
                space_id: Some("default".into()),
                user_id: Some("seed".into()),
                body: DirectoryEventBody::MemberUpserted { space_id: "default".into(), user_id: "seed".into(), role: DirectorySpaceRole::Author },
            },
        ];
        self.append_events(&events).await?;
        Ok(())
    }

    async fn project_verified_checkpoint(&self, txn: &mut Txn, event: &DirectoryEvent, checkpoint: &ArtifactCheckpoint) -> DirectoryResult<()> {
        let new_event = NewDirectoryEvent { hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone() };
        validate_verified_checkpoint_append(&new_event, checkpoint)?;
        let key = checkpoint_key_v1(&checkpoint.scope, checkpoint.checkpoint_id);
        txn.run(
            query(
                "MATCH (c:ArtifactCheckpoint {scopeCheckpointKey: $key})
                 CREATE (p:ArtifactCheckpointPrivate {scopeCheckpointKey: $key, spaceId: $space_id, documentId: $document_id, eventSeq: $event_seq, payload: $payload})
                 MERGE (c)-[:HAS_PRIVATE_AUTHORITY]->(p)",
            )
            .param("key", key)
            .param("space_id", checkpoint.scope.space_id.clone())
            .param("document_id", checkpoint.scope.document_id.clone())
            .param("event_seq", i64::try_from(event.seq).map_err(backend)?)
            .param("payload", directory::os_pack::json::to_json_string(checkpoint)),
        )
        .await
        .map_err(backend)?;
        Ok(())
    }

    //#region 🔖️Projections
    /// 🏛️ Reads projection authority after the Neo4j directory-counter writer lock.
    async fn projection_rejection(&self, txn: &mut Txn, body: &DirectoryEventBody) -> DirectoryResult<Option<DirectoryProjectionRejectionV1>> {
        let Some(space_id) = directory_projection_space_v1(body) else {
            return Ok(None);
        };
        let mut result = txn.execute(query("MATCH (s:Space {id: $id}) RETURN s.kind AS kind").param("id", space_id)).await.map_err(backend)?;
        let kind = result.next(txn.handle()).await.map_err(backend)?.map(|row| row.get::<String>("kind").map_err(backend)).transpose()?;
        Ok(directory_projection_rejection_v1(body, kind.as_deref()))
    }

    /// 🧮️ Applies one event and enforces intrinsic archive authority in the same transaction.
    async fn project(&self, txn: &mut Txn, event: &DirectoryEvent) -> DirectoryResult<()> {
        if let Some(reason) = self.projection_rejection(txn, &event.body).await? {
            return Err(reason.into_error());
        }
        match &event.body {
            DirectoryEventBody::UserCreated { user_id, email, display_name } => {
                txn.run(
                    query("MERGE (u:User {id: $id}) ON CREATE SET u.email = $email, u.displayName = $display_name, u.createdAt = $created_at")
                        .param("id", user_id.clone())
                        .param("email", email.clone())
                        .param("display_name", display_name.clone())
                        .param("created_at", event.recorded_at_ms),
                )
                .await
                .map_err(backend)?;
            }
            DirectoryEventBody::SpaceCreated { space_id, name, space_kind, visibility, owner_user_id } => {
                txn.run(
                    query("MERGE (s:Space {id: $id}) ON CREATE SET s.name = $name, s.ownerUserId = $owner_user_id, s.kind = $kind, s.visibility = $visibility, s.createdAt = $created_at")
                        .param("id", space_id.clone())
                        .param("name", name.clone())
                        .param("owner_user_id", owner_user_id.clone())
                        .param("kind", kind_to_str(*space_kind))
                        .param("visibility", visibility_to_str(*visibility))
                        .param("created_at", event.recorded_at_ms),
                )
                .await
                .map_err(backend)?;
            }
            DirectoryEventBody::SpaceRenamed { space_id, name } => {
                txn.run(query("MATCH (s:Space {id: $id}) SET s.name = $name").param("id", space_id.clone()).param("name", name.clone())).await.map_err(backend)?;
            }
            DirectoryEventBody::SpaceVisibilityChanged { space_id, visibility } => {
                txn.run(query("MATCH (s:Space {id: $id}) SET s.visibility = $visibility").param("id", space_id.clone()).param("visibility", visibility_to_str(*visibility))).await.map_err(backend)?;
            }
            DirectoryEventBody::SpaceArchived { space_id } => {
                txn.run(query("MATCH (s:Space {id: $id}) SET s.kind = 'archive'").param("id", space_id.clone())).await.map_err(backend)?;
                txn.run(query("MATCH (:User)-[m:MEMBER_OF]->(:Space {id: $id}) WHERE m.role = 'author' SET m.role = 'spectator'").param("id", space_id.clone())).await.map_err(backend)?;
            }
            DirectoryEventBody::SpaceDeleted { space_id } => {
                txn.run(query("MATCH (g:ShareGrant {spaceId: $id}) DETACH DELETE g").param("id", space_id.clone())).await.map_err(backend)?;
                txn.run(query("MATCH (r:ArtifactRetention {spaceId: $id}) DETACH DELETE r").param("id", space_id.clone())).await.map_err(backend)?;
                txn.run(query("MATCH (p:ArtifactCheckpointPrivate {spaceId: $id}) DETACH DELETE p").param("id", space_id.clone())).await.map_err(backend)?;
                txn.run(query("MATCH (c:ArtifactCheckpoint {spaceId: $id}) DETACH DELETE c").param("id", space_id.clone())).await.map_err(backend)?;
                txn.run(query("MATCH (i:DocumentIndex {spaceId: $id}) DETACH DELETE i").param("id", space_id.clone())).await.map_err(backend)?;
                txn.run(query("MATCH (d:DocumentDescriptor {spaceId: $id}) DETACH DELETE d").param("id", space_id.clone())).await.map_err(backend)?;
                txn.run(query("MATCH (i:SpaceInvite {spaceId: $id}) DETACH DELETE i").param("id", space_id.clone())).await.map_err(backend)?;
                txn.run(query("MATCH (s:Space {id: $id}) DETACH DELETE s").param("id", space_id.clone())).await.map_err(backend)?;
            }
            DirectoryEventBody::MemberUpserted { space_id, user_id, role } => {
                txn.run(
                    query(
                        "MATCH (u:User {id: $user_id}), (s:Space {id: $space_id})
                         MERGE (u)-[m:MEMBER_OF]->(s)
                         ON CREATE SET m.role = $role, m.createdAt = $created_at
                         ON MATCH SET m.role = $role",
                    )
                    .param("user_id", user_id.clone())
                    .param("space_id", space_id.clone())
                    .param("role", role_from_wire(*role).as_str())
                    .param("created_at", event.recorded_at_ms),
                )
                .await
                .map_err(backend)?;
            }
            DirectoryEventBody::MemberRemoved { space_id, user_id } => {
                txn.run(query("MATCH (:User {id: $user_id})-[m:MEMBER_OF]->(:Space {id: $space_id}) DELETE m").param("user_id", user_id.clone()).param("space_id", space_id.clone())).await.map_err(backend)?;
            }
            DirectoryEventBody::InviteRedeemed { space_id, user_id, role, .. } => {
                txn.run(
                    query(
                        "MATCH (u:User {id: $user_id}), (s:Space {id: $space_id})
                         MERGE (u)-[m:MEMBER_OF]->(s)
                         ON CREATE SET m.role = $role, m.createdAt = $created_at
                         ON MATCH SET m.role = $role",
                    )
                    .param("user_id", user_id.clone())
                    .param("space_id", space_id.clone())
                    .param("role", role_from_wire(*role).as_str())
                    .param("created_at", event.recorded_at_ms),
                )
                .await
                .map_err(backend)?;
            }
            DirectoryEventBody::DocumentAnnounced { descriptor } => {
                let scope_key = document_scope_key_v1(&DocumentScope::new(&descriptor.space_id, &descriptor.document_id));
                txn.run(
                    query("MATCH (s:Space {id: $space_id}) MERGE (d:DocumentDescriptor {scopeKey: $scope_key}) ON CREATE SET d.spaceId = $space_id, d.documentId = $document_id, d.descriptor = $descriptor, d.announcedAt = $announced_at MERGE (s)-[:CONTAINS_DOCUMENT]->(d)")
                        .param("space_id", descriptor.space_id.clone())
                        .param("scope_key", scope_key)
                        .param("document_id", descriptor.document_id.clone())
                        .param("descriptor", directory::os_pack::json::to_json_string(descriptor))
                        .param("announced_at", event.recorded_at_ms),
                )
                .await
                .map_err(backend)?;
            }
            DirectoryEventBody::DocumentIndexed { scope, .. } => {
                let scope_key = document_scope_key_v1(scope);
                let mut result = txn.execute(query("MATCH (d:DocumentDescriptor {scopeKey: $scope_key}) RETURN d.descriptor AS descriptor").param("scope_key", scope_key.clone())).await.map_err(backend)?;
                let row = result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::NotFound("indexed document descriptor".into()))?;
                let descriptor_json: String = row.get("descriptor").map_err(backend)?;
                drop(result);
                let descriptor: DocumentDescriptor = directory::os_pack::json::from_json_str(&descriptor_json).map_err(backend)?;
                let indexed = crate::directory::document_index_projection_v1(event, &descriptor)?;
                let payload = directory::os_pack::json::to_json_string(&indexed);
                let mut previous = txn.execute(query("MATCH (i:DocumentIndex {scopeKey: $scope_key}) RETURN i.payload AS payload").param("scope_key", scope_key.clone())).await.map_err(backend)?;
                if let Some(row) = previous.next(txn.handle()).await.map_err(backend)? {
                    let stored: String = row.get("payload").map_err(backend)?;
                    if stored != payload {
                        return Err(DirectoryError::Conflict("document index is already bound".into()));
                    }
                }
                drop(previous);
                txn.run(
                    query("MATCH (d:DocumentDescriptor {scopeKey: $scope_key}) MERGE (i:DocumentIndex {scopeKey: $scope_key}) ON CREATE SET i.spaceId = $space_id, i.documentId = $document_id, i.payload = $payload MERGE (d)-[:INDEXED_AS]->(i)")
                        .param("scope_key", scope_key)
                        .param("space_id", scope.space_id.clone())
                        .param("document_id", scope.document_id.clone())
                        .param("payload", payload),
                )
                .await
                .map_err(backend)?;
            }
            DirectoryEventBody::ArtifactCheckpointPublished { checkpoint } => {
                let scope_key = document_scope_key_v1(&checkpoint.scope);
                let mut descriptors = txn.execute(query("MATCH (d:DocumentDescriptor {scopeKey: $scope_key}) RETURN d.descriptor AS descriptor").param("scope_key", scope_key.clone())).await.map_err(backend)?;
                let descriptor_row = descriptors.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::NotFound("checkpoint document descriptor".into()))?;
                let descriptor: DocumentDescriptor = directory::os_pack::json::from_json_str(&descriptor_row.get::<String>("descriptor").map_err(backend)?).map_err(backend)?;
                drop(descriptors);
                let mut indexes = txn.execute(query("MATCH (i:DocumentIndex {scopeKey: $scope_key}) RETURN i.payload AS payload").param("scope_key", scope_key.clone())).await.map_err(backend)?;
                let index: Option<directory::os_directory::DirectoryIndexedDocumentViewV1> = match indexes.next(txn.handle()).await.map_err(backend)? {
                    Some(row) => Some(directory::os_pack::json::from_json_str(&row.get::<String>("payload").map_err(backend)?).map_err(backend)?),
                    None => None,
                };
                drop(indexes);
                crate::directory::validate_checkpoint_index_v1(index.as_ref(), &descriptor, checkpoint)?;
                let mut heads = txn.execute(query("MATCH (:DocumentDescriptor {scopeKey: $scope_key})-[:ACTIVE_CHECKPOINT]->(c:ArtifactCheckpoint) RETURN c.payload AS payload").param("scope_key", scope_key.clone())).await.map_err(backend)?;
                let active: Option<PublishedArtifactCheckpoint> = match heads.next(txn.handle()).await.map_err(backend)? {
                    Some(row) => Some(directory::os_pack::json::from_json_str(&row.get::<String>("payload").map_err(backend)?).map_err(backend)?),
                    None => None,
                };
                drop(heads);
                let mut counts = txn.execute(query("MATCH (:DocumentDescriptor {scopeKey: $scope_key})-[:HAS_CHECKPOINT]->(c:ArtifactCheckpoint) RETURN count(c) AS count").param("scope_key", scope_key.clone())).await.map_err(backend)?;
                let count: i64 = counts.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("checkpoint count is unavailable".into()))?.get("count").map_err(backend)?;
                drop(counts);
                crate::directory::validate_published_checkpoint_lineage(&descriptor, active.as_ref(), u64::try_from(count).map_err(backend)?, checkpoint)?;
                let checkpoint_key = checkpoint_key_v1(&checkpoint.scope, checkpoint.checkpoint_id);
                let payload = directory::os_pack::json::to_json_string(checkpoint);
                txn.run(
                    query(
                        "MATCH (d:DocumentDescriptor {scopeKey: $scope_key})
                         OPTIONAL MATCH (d)-[active:ACTIVE_CHECKPOINT]->(:ArtifactCheckpoint)
                         DELETE active
                         MERGE (c:ArtifactCheckpoint {scopeCheckpointKey: $checkpoint_key})
                         ON CREATE SET c.spaceId = $space_id, c.documentId = $document_id, c.checkpointId = $checkpoint_id, c.parentCheckpointId = $parent_checkpoint_id, c.descriptorDigest = $descriptor_digest, c.frontierDocumentId = $frontier_document_id, c.headEditOrdinal = $head_edit_ordinal, c.headEditId = $head_edit_id, c.lastCommitSeq = $last_commit_seq, c.chainHash = $chain_hash, c.packSha256 = $pack_sha256, c.packByteLength = $pack_byte_length, c.sprSha256 = $spr_sha256, c.sprByteLength = $spr_byte_length, c.aggregateSha256 = $aggregate_sha256, c.publishedAt = $published_at, c.eventSeq = $event_seq, c.payload = $payload
                         SET c.payload = $payload
                         MERGE (d)-[:HAS_CHECKPOINT]->(c)
                         MERGE (d)-[:ACTIVE_CHECKPOINT]->(c)",
                    )
                    .param("scope_key", scope_key)
                    .param("checkpoint_key", checkpoint_key)
                    .param("space_id", checkpoint.scope.space_id.clone())
                    .param("document_id", checkpoint.scope.document_id.clone())
                    .param("checkpoint_id", hex_lower(&checkpoint.checkpoint_id.0))
                    .param("parent_checkpoint_id", checkpoint.parent_checkpoint_id.map(|id| hex_lower(&id.0)).unwrap_or_default())
                    .param("descriptor_digest", hex_lower(&checkpoint.descriptor_digest_v1.0))
                    .param("frontier_document_id", checkpoint.baseline_frontier.document_id.clone())
                    .param("head_edit_ordinal", i64::try_from(checkpoint.baseline_frontier.head_edit_ordinal).map_err(backend)?)
                    .param("head_edit_id", checkpoint.baseline_frontier.head_edit_id.clone())
                    .param("last_commit_seq", i64::try_from(checkpoint.baseline_frontier.last_commit_seq).map_err(backend)?)
                    .param("chain_hash", hex_lower(&checkpoint.baseline_frontier.chain_hash.0))
                    .param("pack_sha256", hex_lower(&checkpoint.pack.sha256.0))
                    .param("pack_byte_length", i64::try_from(checkpoint.pack.byte_length).map_err(backend)?)
                    .param("spr_sha256", hex_lower(&checkpoint.spr.sha256.0))
                    .param("spr_byte_length", i64::try_from(checkpoint.spr.byte_length).map_err(backend)?)
                    .param("aggregate_sha256", hex_lower(&checkpoint.aggregate_sha256.0))
                    .param("published_at", i64::try_from(checkpoint.published_at_ms).map_err(backend)?)
                    .param("event_seq", i64::try_from(event.seq).map_err(backend)?)
                    .param("payload", payload),
                )
                .await
                .map_err(backend)?;
                if let Some(parent) = checkpoint.parent_checkpoint_id {
                    txn.run(
                        query(
                            "MATCH (p:ArtifactCheckpoint {scopeCheckpointKey: $parent_key}), (c:ArtifactCheckpoint {scopeCheckpointKey: $checkpoint_key})
                             MERGE (p)-[:NEXT_CHECKPOINT]->(c)",
                        )
                        .param("parent_key", checkpoint_key_v1(&checkpoint.scope, parent))
                        .param("checkpoint_key", checkpoint_key_v1(&checkpoint.scope, checkpoint.checkpoint_id)),
                    )
                    .await
                    .map_err(backend)?;
                }
            }
            DirectoryEventBody::ArtifactRetentionAdvanced { retention } => {
                let scope_key = document_scope_key_v1(&retention.scope);
                let checkpoint_key = checkpoint_key_v1(&retention.scope, retention.retained_checkpoint_id);
                let payload = directory::os_pack::json::to_json_string(retention);
                txn.run(
                    query(
                        "MATCH (d:DocumentDescriptor {scopeKey: $scope_key}), (c:ArtifactCheckpoint {scopeCheckpointKey: $checkpoint_key})
                         OPTIONAL MATCH (d)-[old:RETENTION_FLOOR]->(:ArtifactCheckpoint)
                         DELETE old
                         MERGE (r:ArtifactRetention {scopeKey: $scope_key})
                         SET r.spaceId = $space_id, r.documentId = $document_id, r.retainedCheckpointId = $retained_checkpoint_id, r.floorDocumentId = $floor_document_id, r.floorHeadEditOrdinal = $floor_head_edit_ordinal, r.floorHeadEditId = $floor_head_edit_id, r.floorLastCommitSeq = $floor_last_commit_seq, r.floorChainHash = $floor_chain_hash, r.checkpointLineageHead = $lineage_head, r.eventSeq = $event_seq, r.payload = $payload
                         MERGE (d)-[:HAS_RETENTION]->(r)
                         MERGE (d)-[:RETENTION_FLOOR]->(c)",
                    )
                    .param("scope_key", scope_key)
                    .param("checkpoint_key", checkpoint_key)
                    .param("space_id", retention.scope.space_id.clone())
                    .param("document_id", retention.scope.document_id.clone())
                    .param("retained_checkpoint_id", hex_lower(&retention.retained_checkpoint_id.0))
                    .param("floor_document_id", retention.retained_floor.document_id.clone())
                    .param("floor_head_edit_ordinal", i64::try_from(retention.retained_floor.head_edit_ordinal).map_err(backend)?)
                    .param("floor_head_edit_id", retention.retained_floor.head_edit_id.clone())
                    .param("floor_last_commit_seq", i64::try_from(retention.retained_floor.last_commit_seq).map_err(backend)?)
                    .param("floor_chain_hash", hex_lower(&retention.retained_floor.chain_hash.0))
                    .param("lineage_head", hex_lower(&retention.checkpoint_lineage_head.0))
                    .param("event_seq", i64::try_from(event.seq).map_err(backend)?)
                    .param("payload", payload),
                )
                .await
                .map_err(backend)?;
            }
        }
        Ok(())
    }
    //#endregion 🔖️Projections
}

impl HubDirectory for Neo4jDirectory {
    async fn claim_artifact_creation(&self, intent: &ArtifactCreationIntentV1) -> DirectoryResult<ArtifactCreationClaimV1> {
        intent.validate()?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let request_key = lock_artifact_creation_request(&mut txn, &intent.actor.user_id, &intent.request.request_id).await?;
        let observed_now = validate_artifact_creation_authority(&mut txn, &intent.actor, &intent.scope.space_id, None).await?;
        if intent.accepted_at_ms > observed_now || observed_now >= intent.deadline_ms {
            return Err(DirectoryError::Conflict("artifact creation acceptance is outside its live server deadline".into()));
        }
        let facts = artifact_creation_facts(&mut txn, &request_key).await?;
        if !facts.is_empty() {
            let operation = ArtifactCreationOperationV1::fold(&facts)?;
            if operation.intent.command_sha256 != intent.command_sha256 || operation.intent.scope.space_id != intent.scope.space_id {
                return Err(DirectoryError::Conflict("artifact creation request is already bound to another intent".into()));
            }
            txn.commit().await.map_err(backend)?;
            return Ok(ArtifactCreationClaimV1::Existing(operation));
        }
        let scope_key = document_scope_key_v1(&intent.scope);
        let mut occupied = txn
            .execute(
                query("OPTIONAL MATCH (d:DocumentDescriptor {scopeKey: $scope_key}) OPTIONAL MATCH (f:ArtifactCreationFact {spaceId: $space_id, documentId: $document_id, revision: 1}) RETURN count(d) + count(f) AS count")
                    .param("scope_key", scope_key)
                    .param("space_id", intent.scope.space_id.clone())
                    .param("document_id", intent.scope.document_id.clone()),
            )
            .await
            .map_err(backend)?;
        let count: i64 = occupied.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact creation occupancy returned no row".into()))?.get("count").map_err(backend)?;
        drop(occupied);
        if count != 0 {
            return Err(DirectoryError::Conflict("artifact creation document identity is already occupied".into()));
        }
        txn.run(
            query("MATCH (r:ArtifactCreationRequest {key: $key}) SET r.documentScopeKey = $scope_key, r.spaceId = $space_id, r.documentId = $document_id")
                .param("key", request_key.clone())
                .param("scope_key", document_scope_key_v1(&intent.scope))
                .param("space_id", intent.scope.space_id.clone())
                .param("document_id", intent.scope.document_id.clone()),
        )
        .await
        .map_err(backend)?;
        let fact = ArtifactCreationFactV1 {
            actor_user_id: intent.actor.user_id.clone(),
            request_id: intent.request.request_id.clone(),
            revision: 1,
            recorded_at_ms: intent.accepted_at_ms,
            body: ArtifactCreationFactBodyV1::Accepted { intent: intent.clone() },
        };
        let operation = ArtifactCreationOperationV1::fold(std::slice::from_ref(&fact))?;
        insert_artifact_creation_fact(&mut txn, &request_key, intent, &fact).await?;
        txn.commit().await.map_err(backend)?;
        Ok(ArtifactCreationClaimV1::Accepted(operation))
    }

    async fn read_artifact_creation(&self, actor_user_id: &str, request_id: &str) -> DirectoryResult<Vec<ArtifactCreationFactV1>> {
        let key = artifact_creation_request_key(actor_user_id, request_id);
        let mut result = self.graph.execute(query("MATCH (:ArtifactCreationRequest {key: $key})-[:HAS_FACT]->(f:ArtifactCreationFact) RETURN f.payload AS payload ORDER BY f.revision LIMIT 4").param("key", key)).await.map_err(backend)?;
        let mut facts = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            facts.push(directory::os_pack::json::from_json_str(&row.get::<String>("payload").map_err(backend)?).map_err(backend)?);
        }
        Ok(facts)
    }

    async fn append_artifact_creation_fact(&self, append: &ArtifactCreationFactAppendV1) -> DirectoryResult<ArtifactCreationOperationV1> {
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let request_key = lock_artifact_creation_request(&mut txn, &append.actor.user_id, &append.request_id).await?;
        let observed_now = validate_artifact_creation_authority(&mut txn, &append.actor, &append.space_id, None).await?;
        let mut facts = artifact_creation_facts(&mut txn, &request_key).await?;
        let operation = ArtifactCreationOperationV1::fold(&facts)?;
        if append.recorded_at_ms > observed_now || matches!(append.body, ArtifactCreationFactBodyV1::Prepared { .. }) && observed_now >= operation.intent.deadline_ms {
            return Err(DirectoryError::Conflict("artifact creation transition is outside its live server clock".into()));
        }
        if let Some(next) = decide_artifact_creation_fact_append_v1(&facts, append, observed_now)? {
            insert_artifact_creation_fact(&mut txn, &request_key, &operation.intent, &next).await?;
            facts.push(next);
        }
        let operation = ArtifactCreationOperationV1::fold(&facts)?;
        txn.commit().await.map_err(backend)?;
        Ok(operation)
    }

    async fn artifact_creation_terminate_uncommitted(&self, intent: &ArtifactCreationIntentV1, current_now_ms: u64) -> DirectoryResult<ArtifactCreationOperationV1> {
        intent.validate()?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let request_key = lock_artifact_creation_request(&mut txn, &intent.actor.user_id, &intent.request.request_id).await?;
        let mut facts = artifact_creation_facts(&mut txn, &request_key).await?;
        let operation = ArtifactCreationOperationV1::fold(&facts)?;
        if operation.intent != *intent {
            return Err(DirectoryError::Conflict("artifact creation supervisor intent differs".into()));
        }
        if operation.receipt.is_some()
            || matches!(operation.phase, directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationPhaseV1::Cancelled | directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationPhaseV1::Failed)
        {
            txn.commit().await.map_err(backend)?;
            return Ok(operation);
        }
        let authority = validate_artifact_creation_authority(&mut txn, &intent.actor, &intent.scope.space_id, None).await;
        let observed_now = match authority {
            Ok(observed_now) => {
                if observed_now < intent.deadline_ms {
                    return Err(DirectoryError::Conflict("artifact creation still has live execution authority".into()));
                }
                observed_now
            }
            Err(DirectoryError::Unauthorized) => u64::try_from(now_ms()).map_err(backend)?,
            Err(error) => return Err(error),
        };
        if current_now_ms > observed_now {
            return Err(DirectoryError::Conflict("artifact creation supervisor clock is in the future".into()));
        }
        let next = ArtifactCreationFactV1 { actor_user_id: intent.actor.user_id.clone(), request_id: intent.request.request_id.clone(), revision: operation.revision + 1, recorded_at_ms: observed_now, body: ArtifactCreationFactBodyV1::Failed };
        facts.push(next.clone());
        let operation = ArtifactCreationOperationV1::fold(&facts)?;
        insert_artifact_creation_fact(&mut txn, &request_key, intent, &next).await?;
        txn.commit().await.map_err(backend)?;
        Ok(operation)
    }

    async fn artifact_creation_recovery_candidates(&self, current_now_ms: u64, limit: usize) -> DirectoryResult<Vec<ArtifactCreationIntentV1>> {
        if limit == 0 || limit > 256 {
            return Err(DirectoryError::Conflict("artifact creation recovery page is out of bounds".into()));
        }
        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH (r:ArtifactCreationRequest)-[:HAS_FACT]->(a:ArtifactCreationFact {revision: 1})
             MATCH (r)-[:HAS_FACT]->(candidate:ArtifactCreationFact)
             WITH r, a, max(candidate.revision) AS latestRevision
             MATCH (r)-[:HAS_FACT]->(state:ArtifactCreationFact {revision: latestRevision})
             WHERE state.phase IN ['accepted','prepared'] AND (a.deadlineMs <= $now_ms OR state.phase = 'prepared')
             RETURN a.payload AS payload ORDER BY a.recordedAtMs, a.actorUserId, a.requestId LIMIT $limit",
                )
                .param("now_ms", i64::try_from(current_now_ms).map_err(backend)?)
                .param("limit", i64::try_from(limit).map_err(backend)?),
            )
            .await
            .map_err(backend)?;
        let mut intents = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let fact: ArtifactCreationFactV1 = directory::os_pack::json::from_json_str(&row.get::<String>("payload").map_err(backend)?).map_err(backend)?;
            let ArtifactCreationFactBodyV1::Accepted { intent } = fact.body else {
                return Err(DirectoryError::Backend("artifact creation recovery row is not accepted".into()));
            };
            intent.validate()?;
            intents.push(intent);
        }
        Ok(intents)
    }

    async fn append_document_genesis(&self, append: &DocumentGenesisAppendV1) -> DirectoryResult<DocumentGenesisCommitV1> {
        append.intent.validate()?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let request_key = lock_artifact_creation_request(&mut txn, &append.intent.actor.user_id, &append.intent.request.request_id).await?;
        let lease_expires_at_ms = cas_lock_space(&mut txn, &append.intent.scope.space_id).await?;
        let mut writer = txn.execute(query("MERGE (c:DirectoryCounter {id: 'singleton'}) ON CREATE SET c.seq = 0 SET c.claimNonce = coalesce(c.claimNonce, 0) + 1 RETURN c.seq AS seq")).await.map_err(backend)?;
        writer.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("directory writer lock returned no row".into()))?;
        drop(writer);
        #[cfg(test)]
        let observed_now_override = self.pause_genesis_before_authority_for_test().await;
        #[cfg(not(test))]
        let observed_now_override = None;
        let observed_now = validate_artifact_creation_authority(&mut txn, &append.intent.actor, &append.intent.scope.space_id, observed_now_override).await?;
        let mut facts = artifact_creation_facts(&mut txn, &request_key).await?;
        let operation = ArtifactCreationOperationV1::fold(&facts)?;
        if operation.intent != append.intent {
            return Err(DirectoryError::Conflict("genesis creation accepted identity differs".into()));
        }
        if operation.receipt.is_some() {
            txn.commit().await.map_err(backend)?;
            return Ok(DocumentGenesisCommitV1::Existing(operation));
        }
        if append.now_ms > observed_now || observed_now >= append.intent.deadline_ms {
            return Err(DirectoryError::Conflict("genesis publication is outside its live server deadline".into()));
        }
        validate_document_genesis_append_v1(&operation, append)?;
        let checkpoint = &append.checkpoint;
        let reservation = &append.reservation;
        let encoded = encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        let current_now = i64::try_from(observed_now).map_err(backend)?;
        let token_generation = i64::try_from(reservation.generation).map_err(backend)?;
        let token_epoch = i64::try_from(reservation.write_epoch).map_err(backend)?;
        let token_expiry = i64::try_from(reservation.expires_at_ms).map_err(backend)?;
        if lease_expires_at_ms > current_now {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is active for this space".into()));
        }
        let scope_key = document_scope_key_v1(&checkpoint.scope);
        let mut occupied = txn.execute(query("OPTIONAL MATCH (d:DocumentDescriptor {scopeKey: $scope_key}) OPTIONAL MATCH (i:DocumentIndex {scopeKey: $scope_key}) OPTIONAL MATCH (c:ArtifactCheckpoint {spaceId: $space_id, documentId: $document_id}) RETURN count(d) + count(i) + count(c) AS count")
            .param("scope_key", scope_key).param("space_id", checkpoint.scope.space_id.clone()).param("document_id", checkpoint.scope.document_id.clone())).await.map_err(backend)?;
        let count: i64 = occupied.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("genesis public occupancy returned no row".into()))?.get("count").map_err(backend)?;
        drop(occupied);
        if count != 0 {
            return Err(DirectoryError::Conflict("genesis scope is already publicly occupied".into()));
        }
        let mut published = txn
            .execute(query("MATCH (r:ArtifactCasReference {spaceId: $space_id, documentId: $document_id}) RETURN count(r) AS count").param("space_id", checkpoint.scope.space_id.clone()).param("document_id", checkpoint.scope.document_id.clone()))
            .await
            .map_err(backend)?;
        let published_count: i64 = published.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("genesis CAS reference count returned no row".into()))?.get("count").map_err(backend)?;
        drop(published);
        if published_count != 0 {
            return Err(DirectoryError::Conflict("genesis scope has a CAS reference without its creation receipt".into()));
        }
        let checkpoint_key = checkpoint_key_v1(&checkpoint.scope, checkpoint.checkpoint_id);
        let mut current = txn
            .execute(query("MATCH (r:ArtifactCasReservation {scopeCheckpointKey: $key}) RETURN r.generation AS generation, r.writeEpoch AS writeEpoch, r.expiresAtMs AS expiresAtMs, r.plan AS plan").param("key", checkpoint_key.clone()))
            .await
            .map_err(backend)?;
        let row = current.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Conflict("genesis CAS reservation is missing, expired or substituted".into()))?;
        let stored: neo4rs::BoltBytes = row.get("plan").map_err(backend)?;
        if row.get::<i64>("generation").map_err(backend)? != token_generation
            || row.get::<i64>("writeEpoch").map_err(backend)? != token_epoch
            || row.get::<i64>("expiresAtMs").map_err(backend)? != token_expiry
            || token_expiry <= current_now
            || stored.value.as_ref() != encoded
        {
            return Err(DirectoryError::Conflict("genesis CAS reservation is missing, expired or substituted".into()));
        }
        drop(current);
        let mut events = Vec::with_capacity(3);
        for event in &append.events {
            let id = time_ordered_id();
            let recorded_at_ms = now_ms();
            let payload = serde_json::Value::from(&event.body.to_value());
            let kind = payload.get("kind").and_then(|value| value.as_str()).unwrap_or_default().to_string();
            let mut counter = txn.execute(query("MATCH (c:DirectoryCounter {id: 'singleton'}) SET c.seq = c.seq + 1 RETURN c.seq AS seq")).await.map_err(backend)?;
            let sequence: i64 = counter.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("directory counter returned no row".into()))?.get("seq").map_err(backend)?;
            drop(counter);
            let seq = u64::try_from(sequence).map_err(backend)?;
            if seq > DIRECTORY_WIRE_INTEGER_MAX {
                return Err(DirectoryError::Conflict("directory event sequence exceeds the public integer boundary".into()));
            }
            let full = DirectoryEvent { seq, id: id.clone(), hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone(), recorded_at_ms };
            validate_directory_event_page_event(&full).map_err(|_| DirectoryError::Conflict("directory event violates the bounded event-page contract".into()))?;
            txn.run(query("CREATE (e:DirectoryEvent {seq: $seq, id: $id, hlcPhysical: $hlc_physical, hlcLogical: $hlc_logical, actorKind: $actor_kind, actorId: $actor_id, spaceId: $space_id, userId: $user_id, kind: $kind, payload: $payload, recordedAt: $recorded_at})")
                .param("seq", sequence).param("id", id).param("hlc_physical", event.hlc.physical_ms).param("hlc_logical", i64::from(event.hlc.logical)).param("actor_kind", actor_kind_to_str(event.actor.kind)).param("actor_id", event.actor.id.clone()).param("space_id", event.space_id.clone()).param("user_id", event.user_id.clone()).param("kind", kind).param("payload", payload.to_string()).param("recorded_at", recorded_at_ms)).await.map_err(backend)?;
            if events.len() == 2 {
                txn.run(
                    query("CREATE (:ArtifactAuthorityEvent {eventSeq: $event_seq, scopeCheckpointKey: $key, payload: $payload})")
                        .param("event_seq", sequence)
                        .param("key", checkpoint_key.clone())
                        .param("payload", directory::os_pack::json::to_json_string(checkpoint)),
                )
                .await
                .map_err(backend)?;
            }
            self.project(&mut txn, &full).await?;
            if events.len() == 2 {
                self.project_verified_checkpoint(&mut txn, &full, checkpoint).await?;
            }
            events.push(full);
        }
        let generation = cas_generation(&mut txn).await?;
        txn.run(query("CREATE (:ArtifactCasLedgerEvent {generation: $generation, operation: 'publish', scopeCheckpointKey: $key, spaceId: $space_id, documentId: $document_id, checkpointId: $checkpoint_id, writeEpoch: $write_epoch, expiresAtMs: $expires_at, eventSeq: $event_seq, plan: $plan})")
            .param("generation", generation).param("key", checkpoint_key).param("space_id", checkpoint.scope.space_id.clone()).param("document_id", checkpoint.scope.document_id.clone()).param("checkpoint_id", hex_lower(&checkpoint.checkpoint_id.0)).param("write_epoch", token_epoch).param("expires_at", token_expiry).param("event_seq", i64::try_from(events[2].seq).map_err(backend)?).param("plan", encoded)).await.map_err(backend)?;
        cas_project_publish(&mut txn, reservation, generation).await?;
        let completion = document_genesis_completion_v1(&operation, append, &events)?;
        insert_artifact_creation_fact(&mut txn, &request_key, &append.intent, &completion).await?;
        facts.push(completion);
        let operation = ArtifactCreationOperationV1::fold(&facts)?;
        match txn.commit().await {
            Ok(()) => {
                #[cfg(test)]
                if self.genesis_test_control.fail_commit_ack.swap(false, std::sync::atomic::Ordering::SeqCst) {
                    return Ok(DocumentGenesisCommitV1::Indeterminate);
                }
                Ok(DocumentGenesisCommitV1::Committed { events, operation })
            }
            Err(_) => Ok(DocumentGenesisCommitV1::Indeterminate),
        }
    }

    //#region ShareTokens
    async fn issue_share_token_as(&self, scope: &DocumentScope, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<IssuedShareToken> {
        let issued = prepare_share_token(scope, ttl_secs, now_ms())?;
        let audit = auth_audit(issued.record.created_at, "share-issued", Some(&issued.record.id), None, actor_user_id, None, "success", None, correlation_id, "server")?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let scope_key = document_scope_key_v1(scope);
        let mut result = txn
            .execute(
                query(
                    "MATCH (s:Space {id: $space_id}), (d:DocumentDescriptor {scopeKey: $scope_key, spaceId: $space_id, documentId: $document_id})
                 CREATE (s)-[:HAS_SHARE_GRANT]->(g:ShareGrant {id: $id, selector: $selector, secretDigest: $secret_digest, spaceId: $space_id, documentId: $document_id, createdAt: $created_at, expiresAt: $expires_at})-[:GRANTS_DOCUMENT]->(d)
                 RETURN count(g) AS c",
                )
                .param("id", issued.record.id.clone())
                .param("selector", issued.record.selector.clone())
                .param("secret_digest", encode_capability_bytes(&issued.record.secret_digest))
                .param("space_id", scope.space_id.clone())
                .param("document_id", scope.document_id.clone())
                .param("scope_key", scope_key)
                .param("created_at", issued.record.created_at)
                .param("expires_at", issued.record.expires_at),
            )
            .await
            .map_err(backend)?;
        let changed: i64 = result.next(txn.handle()).await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
        if changed != 1 {
            return Err(DirectoryError::NotFound(format!("document descriptor {}/{}", scope.space_id, scope.document_id)));
        }
        insert_auth_audit(&mut txn, &audit).await?;
        txn.commit().await.map_err(backend)?;
        Ok(issued)
    }

    async fn issue_share_token_as_with_admin_effect(&self, scope: &DocumentScope, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<IssuedShareToken> {
        let issued = match admin_effect_preflight(prepare_share_token(scope, ttl_secs, now_ms())) {
            Ok(value) => value,
            Err(outcome) => return outcome.into(),
        };
        let audit = match admin_effect_preflight(auth_audit(issued.record.created_at, "share-issued", Some(&issued.record.id), None, actor_user_id, None, "success", None, correlation_id, "server")) {
            Ok(value) => value,
            Err(outcome) => return outcome.into(),
        };
        let mut txn = match self.graph.start_txn().await {
            Ok(txn) => txn,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let changed_result: DirectoryResult<i64> = async {
            let mut result = txn.execute(
                query("MATCH (s:Space {id: $space_id}), (d:DocumentDescriptor {scopeKey: $scope_key, spaceId: $space_id, documentId: $document_id}) CREATE (s)-[:HAS_SHARE_GRANT]->(g:ShareGrant {id: $id, selector: $selector, secretDigest: $secret_digest, spaceId: $space_id, documentId: $document_id, createdAt: $created_at, expiresAt: $expires_at})-[:GRANTS_DOCUMENT]->(d) RETURN count(g) AS c")
                    .param("id", issued.record.id.clone())
                    .param("selector", issued.record.selector.clone())
                    .param("secret_digest", encode_capability_bytes(&issued.record.secret_digest))
                    .param("space_id", scope.space_id.clone())
                    .param("document_id", scope.document_id.clone())
                    .param("scope_key", document_scope_key_v1(scope))
                    .param("created_at", issued.record.created_at)
                    .param("expires_at", issued.record.expires_at),
            )
            .await
            .map_err(backend)?;
            let changed = result.next(txn.handle()).await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
            drop(result);
            Ok(changed)
        }
        .await;
        let changed = admin_effect_try!(txn, changed_result);
        if changed != 1 {
            return admin_effect_rollback(txn).await;
        }
        admin_effect_try!(txn, insert_auth_audit(&mut txn, &audit).await);
        let receipt = admin_effect_try!(txn, admin_operation_effect_receipt_v1(effect, &[]));
        admin_effect_try!(txn, insert_admin_operation_effect_receipt(&mut txn, &receipt).await);
        match txn.commit().await {
            Ok(()) => AdminEffectCommitV1::Applied(issued),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }

    async fn revoke_share_token_as(&self, scope: &DocumentScope, share_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()> {
        validate_bounded_auth_text(reason, "share revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let audit = auth_audit(revoked_at, "share-revoked", Some(share_id), None, actor_user_id, None, "success", Some(reason), correlation_id, "server")?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut result = txn
            .execute(
                query("MATCH (g:ShareGrant {id: $id, spaceId: $space_id, documentId: $document_id}) WHERE g.revokedAt IS NULL SET g.revokedAt = $revoked_at, g.revokedReason = $reason RETURN count(g) AS c")
                    .param("id", share_id)
                    .param("space_id", scope.space_id.clone())
                    .param("document_id", scope.document_id.clone())
                    .param("revoked_at", revoked_at)
                    .param("reason", reason),
            )
            .await
            .map_err(backend)?;
        let changed: i64 = result.next(txn.handle()).await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
        if changed == 0 {
            Err(DirectoryError::NotFound(format!("share grant {share_id}")))
        } else {
            insert_auth_audit(&mut txn, &audit).await?;
            txn.commit().await.map_err(backend)?;
            Ok(())
        }
    }

    async fn revoke_share_token_as_with_admin_effect(&self, scope: &DocumentScope, share_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<()> {
        if let Err(outcome) = admin_effect_preflight(validate_bounded_auth_text(reason, "share revoke reason", AUTH_TEXT_MAX_BYTES)) {
            return outcome.into();
        }
        let revoked_at = now_ms();
        let audit = match admin_effect_preflight(auth_audit(revoked_at, "share-revoked", Some(share_id), None, actor_user_id, None, "success", Some(reason), correlation_id, "server")) {
            Ok(value) => value,
            Err(outcome) => return outcome.into(),
        };
        let mut txn = match self.graph.start_txn().await {
            Ok(txn) => txn,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let changed_result: DirectoryResult<i64> = async {
            let mut result = txn
                .execute(
                    query("MATCH (g:ShareGrant {id: $id, spaceId: $space_id, documentId: $document_id}) WHERE g.revokedAt IS NULL SET g.revokedAt = $revoked_at, g.revokedReason = $reason RETURN count(g) AS c")
                        .param("id", share_id)
                        .param("space_id", scope.space_id.clone())
                        .param("document_id", scope.document_id.clone())
                        .param("revoked_at", revoked_at)
                        .param("reason", reason),
                )
                .await
                .map_err(backend)?;
            let changed = result.next(txn.handle()).await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
            drop(result);
            Ok(changed)
        }
        .await;
        let changed = admin_effect_try!(txn, changed_result);
        if changed != 1 {
            return admin_effect_rollback(txn).await;
        }
        admin_effect_try!(txn, insert_auth_audit(&mut txn, &audit).await);
        let receipt = admin_effect_try!(txn, admin_operation_effect_receipt_v1(effect, &[]));
        admin_effect_try!(txn, insert_admin_operation_effect_receipt(&mut txn, &receipt).await);
        match txn.commit().await {
            Ok(()) => AdminEffectCommitV1::Applied(()),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }

    async fn authenticate_share(&self, scope: &DocumentScope, capability: &ShareCapability) -> DirectoryResult<bool> {
        Ok(self.authenticate_share_binding(scope, capability).await?.is_some())
    }

    async fn authenticate_share_binding(&self, scope: &DocumentScope, capability: &ShareCapability) -> DirectoryResult<Option<ShareTokenRecord>> {
        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH (:Space {id: $space_id})-[:HAS_SHARE_GRANT]->(g:ShareGrant {selector: $selector, spaceId: $space_id, documentId: $document_id})-[:GRANTS_DOCUMENT]->(:DocumentDescriptor {scopeKey: $scope_key})
                     RETURN g AS g",
                )
                .param("selector", capability.selector())
                .param("space_id", scope.space_id.clone())
                .param("document_id", scope.document_id.clone())
                .param("scope_key", document_scope_key_v1(scope)),
            )
            .await
            .map_err(backend)?;
        let Some(row) = result.next().await.map_err(backend)? else { return Ok(None) };
        let record = share_from_node(&row)?;
        Ok(active_capability(&record.selector, &record.secret_digest, record.expires_at, record.revoked_at, capability.selector(), &capability.secret_digest(), now_ms()).then_some(record))
    }

    async fn socket_share_binding(&self, share_id: &str, selector: &str, scope: &DocumentScope, now_ms: i64) -> DirectoryResult<SocketShareBindingStatus> {
        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH (:Space {id: $space_id})-[:HAS_SHARE_GRANT]->(g:ShareGrant {id: $id, selector: $selector, spaceId: $space_id, documentId: $document_id})-[:GRANTS_DOCUMENT]->(:DocumentDescriptor {scopeKey: $scope_key})
                     RETURN g AS g",
                )
                .param("id", share_id)
                .param("selector", selector)
                .param("space_id", scope.space_id.clone())
                .param("document_id", scope.document_id.clone())
                .param("scope_key", document_scope_key_v1(scope)),
            )
            .await
            .map_err(backend)?;
        let Some(row) = result.next().await.map_err(backend)? else { return Ok(SocketShareBindingStatus::Unavailable) };
        let record = share_from_node(&row)?;
        Ok(if record.revoked_at.is_some() {
            SocketShareBindingStatus::Revoked
        } else if record.expires_at <= now_ms {
            SocketShareBindingStatus::Expired
        } else {
            SocketShareBindingStatus::Active { expires_at_ms: record.expires_at }
        })
    }
    //#endregion

    //#region Users
    async fn create_user(&self, email: &str, display_name: &str, password_hash: Option<&str>, sso_subject: Option<&str>, sso_provider: Option<&str>) -> DirectoryResult<UserRecord> {
        let id = time_ordered_id();
        let created_at = now_ms();
        self.graph
            .run(
                query(
                    "CREATE (u:User {id: $id, email: $email, displayName: $display_name, passwordHash: $password_hash,
                                      ssoSubject: $sso_subject, ssoProvider: $sso_provider, createdAt: $created_at})",
                )
                .param("id", id.clone())
                .param("email", email)
                .param("display_name", display_name)
                .param("password_hash", password_hash.unwrap_or_default())
                .param("sso_subject", sso_subject.unwrap_or_default())
                .param("sso_provider", sso_provider.unwrap_or_default())
                .param("created_at", created_at),
            )
            .await
            .map_err(backend)?;
        Ok(UserRecord {
            id,
            email: email.to_string(),
            display_name: display_name.to_string(),
            password_hash: password_hash.map(str::to_string),
            sso_subject: sso_subject.map(str::to_string),
            sso_provider: sso_provider.map(str::to_string),
            created_at,
        })
    }

    async fn get_user(&self, user_id: &str) -> DirectoryResult<Option<UserRecord>> {
        let mut result = self.graph.execute(query("MATCH (u:User {id: $id}) RETURN u AS u").param("id", user_id)).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => Ok(Some(user_from_node(&row)?)),
            None => Ok(None),
        }
    }

    async fn get_user_by_email(&self, email: &str) -> DirectoryResult<Option<UserRecord>> {
        let mut result = self.graph.execute(query("MATCH (u:User {email: $email}) RETURN u AS u").param("email", email)).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => Ok(Some(user_from_node(&row)?)),
            None => Ok(None),
        }
    }

    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> DirectoryResult<Option<UserRecord>> {
        let mut result = self.graph.execute(query("MATCH (u:User {ssoProvider: $provider, ssoSubject: $subject}) RETURN u AS u").param("provider", provider).param("subject", subject)).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => Ok(Some(user_from_node(&row)?)),
            None => Ok(None),
        }
    }

    async fn list_users(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<UserRecord>> {
        let mut result = self.graph.execute(query("MATCH (u:User) RETURN u AS u ORDER BY u.createdAt SKIP $offset LIMIT $limit").param("limit", limit).param("offset", offset)).await.map_err(backend)?;
        let mut users = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            users.push(user_from_node(&row)?);
        }
        Ok(users)
    }

    async fn admin_overview_counts(&self) -> DirectoryResult<AdminDirectoryOverviewCounts> {
        let mut result = self
            .graph
            .execute(query(
                "MATCH (s:Space) WITH count(s) AS spaces
                 OPTIONAL MATCH (u:User) WITH spaces, count(u) AS users
                 OPTIONAL MATCH (session:SyncSession) WHERE session.disconnectedAt IS NULL
                 RETURN spaces, users, count(session) AS connections",
            ))
            .await
            .map_err(backend)?;
        let row = result.next().await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("administrator overview count query returned no row".into()))?;
        let spaces: i64 = row.get("spaces").map_err(backend)?;
        let users: i64 = row.get("users").map_err(backend)?;
        let connections: i64 = row.get("connections").map_err(backend)?;
        Ok(AdminDirectoryOverviewCounts { spaces: u64::try_from(spaces).map_err(backend)?, users: u64::try_from(users).map_err(backend)?, connections: u64::try_from(connections).map_err(backend)? })
    }
    //#endregion

    //#region Spaces
    async fn get_space(&self, space_id: &str) -> DirectoryResult<Option<SpaceRecord>> {
        let mut result = self.graph.execute(query("MATCH (s:Space {id: $space_id}) RETURN s AS s").param("space_id", space_id)).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => Ok(Some(space_from_node(&row)?)),
            None => Ok(None),
        }
    }

    async fn list_spaces_for_user(&self, user_id: &str) -> DirectoryResult<Vec<(SpaceRecord, SpaceRole)>> {
        let mut result = self.graph.execute(query("MATCH (:User {id: $user_id})-[m:MEMBER_OF]->(s:Space) RETURN s AS s, m.role AS role ORDER BY s.createdAt").param("user_id", user_id)).await.map_err(backend)?;
        let mut studios = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let studio = space_from_node(&row)?;
            let role: String = row.get("role").map_err(backend)?;
            if let Some(role) = SpaceRole::parse(&role) {
                studios.push((studio, role));
            }
        }
        Ok(studios)
    }

    async fn list_spaces(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<SpaceRecord>> {
        let mut result = self.graph.execute(query("MATCH (s:Space) RETURN s AS s ORDER BY s.createdAt SKIP $offset LIMIT $limit").param("limit", limit).param("offset", offset)).await.map_err(backend)?;
        let mut studios = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            studios.push(space_from_node(&row)?);
        }
        Ok(studios)
    }

    async fn list_admin_space_summaries_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<AdminSpaceSummaryRecord>> {
        if limit == 0 || limit > super::ADMIN_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("administrator space page limit must be 1..={}", super::ADMIN_PAGE_FETCH_MAX)));
        }
        let cypher = "MATCH (s:Space) WHERE $space_id IS NULL OR s.id = $space_id
                      WITH s ORDER BY s.id SKIP $offset LIMIT $limit
                      CALL { WITH s OPTIONAL MATCH (:User)-[membership:MEMBER_OF]->(s) RETURN count(membership) AS memberCount }
                      CALL { WITH s OPTIONAL MATCH (s)-[:CONTAINS_DOCUMENT]->(document:DocumentDescriptor) RETURN count(document) AS documentCount }
                      CALL { WITH s OPTIONAL MATCH (session:SyncSession {spaceId: s.id}) WHERE session.disconnectedAt IS NULL RETURN count(session) AS activeConnections }
                      CALL { WITH s OPTIONAL MATCH (event:DirectoryEvent {spaceId: s.id}) RETURN coalesce(max(event.recordedAt), s.createdAt) AS updatedAt }
                      RETURN s AS s, memberCount, documentCount, activeConnections, updatedAt ORDER BY s.id";
        let mut result = self.graph.execute(query(cypher).param("space_id", space_id.map(str::to_string)).param("offset", i64::try_from(offset).map_err(backend)?).param("limit", i64::try_from(limit).map_err(backend)?)).await.map_err(backend)?;
        let mut summaries = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            summaries.push(AdminSpaceSummaryRecord {
                space: space_from_node(&row)?,
                member_count: u64::try_from(row.get::<i64>("memberCount").map_err(backend)?).map_err(backend)?,
                document_count: u64::try_from(row.get::<i64>("documentCount").map_err(backend)?).map_err(backend)?,
                active_connections: u64::try_from(row.get::<i64>("activeConnections").map_err(backend)?).map_err(backend)?,
                updated_at: row.get("updatedAt").map_err(backend)?,
            });
        }
        Ok(summaries)
    }

    async fn list_admin_space_members_page(&self, space_id: &str, offset: usize, limit: usize) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>> {
        if limit == 0 || limit > super::ADMIN_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("administrator member page limit must be 1..={}", super::ADMIN_PAGE_FETCH_MAX)));
        }
        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH (u:User)-[membership:MEMBER_OF]->(:Space {id: $space_id})
                     RETURN u AS u, membership.role AS role ORDER BY u.id SKIP $offset LIMIT $limit",
                )
                .param("space_id", space_id)
                .param("offset", i64::try_from(offset).map_err(backend)?)
                .param("limit", i64::try_from(limit).map_err(backend)?),
            )
            .await
            .map_err(backend)?;
        let mut members = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let role: String = row.get("role").map_err(backend)?;
            members.push((user_from_node(&row)?, SpaceRole::parse(&role).ok_or_else(|| DirectoryError::Backend("stored member role is invalid".into()))?));
        }
        Ok(members)
    }

    async fn list_members(&self, space_id: &str) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>> {
        let mut result = self.graph.execute(query("MATCH (u:User)-[m:MEMBER_OF]->(:Space {id: $space_id}) RETURN u AS u, m.role AS role ORDER BY m.createdAt").param("space_id", space_id)).await.map_err(backend)?;
        let mut members = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let user = user_from_node(&row)?;
            let role: String = row.get("role").map_err(backend)?;
            if let Some(role) = SpaceRole::parse(&role) {
                members.push((user, role));
            }
        }
        Ok(members)
    }

    async fn get_role(&self, space_id: &str, user_id: &str) -> DirectoryResult<Option<SpaceRole>> {
        let mut result = self.graph.execute(query("MATCH (:User {id: $user_id})-[m:MEMBER_OF]->(:Space {id: $space_id}) RETURN m.role AS role").param("user_id", user_id).param("space_id", space_id)).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => {
                let role: String = row.get("role").map_err(backend)?;
                Ok(SpaceRole::parse(&role))
            }
            None => Ok(None),
        }
    }

    async fn list_space_administration_members_page(&self, space_id: &str, after_user_id: Option<&str>, limit: usize) -> DirectoryResult<Vec<SpaceAdministrationMemberRow>> {
        if limit == 0 || limit > super::SPACE_ADMINISTRATION_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("space administration member page limit must be 1..={}", super::SPACE_ADMINISTRATION_PAGE_FETCH_MAX)));
        }
        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH (u:User)-[membership:MEMBER_OF]->(:Space {id: $space_id})
                     WHERE $after IS NULL OR u.id > $after
                     RETURN u.id AS userId, u.email AS email, u.displayName AS displayName, membership.role AS role
                     ORDER BY u.id LIMIT $limit",
                )
                .param("space_id", space_id)
                .param("after", after_user_id.map(str::to_owned))
                .param("limit", i64::try_from(limit).map_err(backend)?),
            )
            .await
            .map_err(backend)?;
        let mut members = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let role: String = row.get("role").map_err(backend)?;
            members.push(SpaceAdministrationMemberRow {
                user_id: row.get("userId").map_err(backend)?,
                email: row.get("email").map_err(backend)?,
                display_name: row.get("displayName").map_err(backend)?,
                role: SpaceRole::parse(&role).ok_or_else(|| DirectoryError::Backend("stored member role is invalid".into()))?,
            });
        }
        Ok(members)
    }

    async fn get_document_descriptor(&self, scope: &DocumentScope) -> DirectoryResult<Option<DocumentDescriptor>> {
        let scope_key = document_scope_key_v1(scope);
        let mut result = self.graph.execute(query("MATCH (d:DocumentDescriptor {scopeKey: $scope_key}) RETURN d.descriptor AS descriptor").param("scope_key", scope_key)).await.map_err(backend)?;
        let Some(row) = result.next().await.map_err(backend)? else { return Ok(None) };
        let encoded: String = row.get("descriptor").map_err(backend)?;
        directory::os_pack::json::from_json_str(&encoded).map(Some).map_err(backend)
    }

    async fn list_document_descriptors(&self, space_id: &str) -> DirectoryResult<Vec<DocumentDescriptor>> {
        let mut result = self.graph.execute(query("MATCH (d:DocumentDescriptor {spaceId: $space_id}) RETURN d.descriptor AS descriptor ORDER BY d.documentId").param("space_id", space_id)).await.map_err(backend)?;
        let mut descriptors = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let encoded: String = row.get("descriptor").map_err(backend)?;
            descriptors.push(directory::os_pack::json::from_json_str(&encoded).map_err(backend)?);
        }
        Ok(descriptors)
    }

    async fn list_document_descriptors_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<DocumentDescriptor>> {
        if limit == 0 || limit > super::ADMIN_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("administrator document page limit must be 1..={}", super::ADMIN_PAGE_FETCH_MAX)));
        }
        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH (d:DocumentDescriptor)
                     WHERE $space_id IS NULL OR d.spaceId = $space_id
                     RETURN d.descriptor AS descriptor
                     ORDER BY d.spaceId, d.documentId SKIP $offset LIMIT $limit",
                )
                .param("space_id", space_id.map(str::to_string))
                .param("offset", i64::try_from(offset).map_err(backend)?)
                .param("limit", i64::try_from(limit).map_err(backend)?),
            )
            .await
            .map_err(backend)?;
        let mut descriptors = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let encoded: String = row.get("descriptor").map_err(backend)?;
            descriptors.push(directory::os_pack::json::from_json_str(&encoded).map_err(backend)?);
        }
        Ok(descriptors)
    }

    async fn get_artifact_checkpoint(&self, scope: &DocumentScope, checkpoint_id: ArtifactHash) -> DirectoryResult<Option<PublishedArtifactCheckpoint>> {
        let mut result = self.graph.execute(query("MATCH (c:ArtifactCheckpoint {scopeCheckpointKey: $key}) RETURN c.payload AS payload").param("key", checkpoint_key_v1(scope, checkpoint_id))).await.map_err(backend)?;
        let Some(row) = result.next().await.map_err(backend)? else { return Ok(None) };
        let payload: String = row.get("payload").map_err(backend)?;
        directory::os_pack::json::from_json_str(&payload).map(Some).map_err(backend)
    }

    async fn get_verified_artifact_checkpoint(&self, scope: &DocumentScope, checkpoint_id: ArtifactHash) -> DirectoryResult<Option<ArtifactCheckpoint>> {
        let mut result = self.graph.execute(query("MATCH (p:ArtifactCheckpointPrivate {scopeCheckpointKey: $key}) RETURN p.payload AS payload").param("key", checkpoint_key_v1(scope, checkpoint_id))).await.map_err(backend)?;
        let Some(row) = result.next().await.map_err(backend)? else { return Ok(None) };
        let payload: String = row.get("payload").map_err(backend)?;
        directory::os_pack::json::from_json_str(&payload).map(Some).map_err(backend)
    }

    async fn get_active_artifact_checkpoint(&self, scope: &DocumentScope) -> DirectoryResult<Option<PublishedArtifactCheckpoint>> {
        let mut result =
            self.graph.execute(query("MATCH (:DocumentDescriptor {scopeKey: $scope_key})-[:ACTIVE_CHECKPOINT]->(c:ArtifactCheckpoint) RETURN c.payload AS payload").param("scope_key", document_scope_key_v1(scope))).await.map_err(backend)?;
        let Some(row) = result.next().await.map_err(backend)? else { return Ok(None) };
        let payload: String = row.get("payload").map_err(backend)?;
        directory::os_pack::json::from_json_str(&payload).map(Some).map_err(backend)
    }

    async fn get_artifact_retention(&self, scope: &DocumentScope) -> DirectoryResult<Option<ArtifactRetention>> {
        let mut result = self.graph.execute(query("MATCH (r:ArtifactRetention {scopeKey: $scope_key}) RETURN r.payload AS payload").param("scope_key", document_scope_key_v1(scope))).await.map_err(backend)?;
        let Some(row) = result.next().await.map_err(backend)? else { return Ok(None) };
        let payload: String = row.get("payload").map_err(backend)?;
        directory::os_pack::json::from_json_str(&payload).map(Some).map_err(backend)
    }

    async fn artifact_checkpoint_count(&self, scope: &DocumentScope) -> DirectoryResult<u64> {
        let mut result = self.graph.execute(query("MATCH (:DocumentDescriptor {scopeKey: $scope_key})-[:HAS_CHECKPOINT]->(c:ArtifactCheckpoint) RETURN count(c) AS count").param("scope_key", document_scope_key_v1(scope))).await.map_err(backend)?;
        let count: i64 = result.next().await.map_err(backend)?.and_then(|row| row.get("count").ok()).unwrap_or(0);
        u64::try_from(count).map_err(backend)
    }

    async fn list_artifact_checkpoint_lineage(&self, scope: &DocumentScope, limit: usize) -> DirectoryResult<Vec<PublishedArtifactCheckpoint>> {
        if limit == 0 || limit as u64 > ARTIFACT_CHECKPOINT_LINEAGE_MAX {
            return Err(DirectoryError::Conflict(format!("artifact checkpoint lineage limit must be 1..={ARTIFACT_CHECKPOINT_LINEAGE_MAX}")));
        }
        let mut result = self
            .graph
            .execute(
                query("MATCH (:DocumentDescriptor {scopeKey: $scope_key})-[:HAS_CHECKPOINT]->(c:ArtifactCheckpoint) RETURN c.payload AS payload ORDER BY c.eventSeq LIMIT $limit")
                    .param("scope_key", document_scope_key_v1(scope))
                    .param("limit", limit as i64),
            )
            .await
            .map_err(backend)?;
        let mut checkpoints = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let payload: String = row.get("payload").map_err(backend)?;
            checkpoints.push(directory::os_pack::json::from_json_str(&payload).map_err(backend)?);
        }
        Ok(checkpoints)
    }
    //#endregion

    //#region AuthSessions
    async fn issue_auth_session(&self, issue: &AuthSessionIssue) -> DirectoryResult<IssuedAuthSession> {
        let issued = prepare_auth_session(issue, now_ms())?;
        let audit = auth_audit(issued.record.issued_at, "session-issued", Some(&issued.record.id), Some(&issued.record.user_id), None, Some(&issued.record.identity_provider), "success", None, &issue.correlation_id, &issue.peer_class)?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        txn.run(
                query(
                    "MATCH (u:User {id: $user_id})
                     CREATE (a:AuthSession {id: $id, selector: $selector, secretDigest: $secret_digest, identityProvider: $identity_provider, identitySubjectDigest: $identity_subject_digest, issuedAt: $issued_at, expiresAt: $expires_at, authorizationGeneration: $generation, deviceInstanceId: $device_instance_id, sessionKind: $session_kind})-[:BELONGS_TO]->(u)",
                )
                .param("user_id", issued.record.user_id.clone())
                .param("id", issued.record.id.clone())
                .param("selector", issued.record.selector.clone())
                .param("secret_digest", encode_capability_bytes(&issued.record.secret_digest))
                .param("identity_provider", issued.record.identity_provider.clone())
                .param("identity_subject_digest", encode_capability_bytes(&issued.record.identity_subject_digest))
                .param("issued_at", issued.record.issued_at)
                .param("expires_at", issued.record.expires_at)
                .param("generation", i64::try_from(issued.record.authorization_generation).map_err(backend)?)
                .param("device_instance_id", issued.record.device_instance_id.clone())
                .param("session_kind", issued.record.session_kind.as_str()),
            )
            .await
            .map_err(backend)?;
        insert_auth_audit(&mut txn, &audit).await?;
        txn.commit().await.map_err(backend)?;
        Ok(issued)
    }

    async fn authenticate_session(&self, capability: &SessionCapability) -> DirectoryResult<Option<AuthSessionRecord>> {
        let mut result = self.graph.execute(query("MATCH (a:AuthSession {selector: $selector})-[:BELONGS_TO]->(u:User) RETURN a AS a, u.id AS userId").param("selector", capability.selector())).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => {
                let record = auth_session_from_node(&row)?;
                Ok(active_capability(&record.selector, &record.secret_digest, record.expires_at, record.revoked_at, capability.selector(), &capability.secret_digest(), now_ms()).then_some(record))
            }
            None => Ok(None),
        }
    }

    async fn socket_session_binding(&self, session_id: &str, user_id: &str, authorization_generation: u64, space_id: Option<&str>, now_ms: i64) -> DirectoryResult<SocketSessionBindingStatus> {
        let mut result = match space_id {
            Some(space_id) => self
                .graph
                .execute(
                    query("MATCH (a:AuthSession {id: $session_id})-[:BELONGS_TO]->(u:User) OPTIONAL MATCH (u)-[m:MEMBER_OF]->(:Space {id: $space_id}) RETURN a AS a, u.id AS userId, m.role AS role")
                        .param("session_id", session_id)
                        .param("space_id", space_id),
                )
                .await
                .map_err(backend)?,
            None => self.graph.execute(query("MATCH (a:AuthSession {id: $session_id})-[:BELONGS_TO]->(u:User) RETURN a AS a, u.id AS userId").param("session_id", session_id)).await.map_err(backend)?,
        };
        let Some(row) = result.next().await.map_err(backend)? else { return Ok(SocketSessionBindingStatus::Unavailable) };
        let record = auth_session_from_node(&row)?;
        if record.user_id != user_id {
            return Ok(SocketSessionBindingStatus::Unavailable);
        }
        if record.revoked_at.is_some() || record.authorization_generation != authorization_generation {
            return Ok(SocketSessionBindingStatus::Revoked);
        }
        if record.expires_at <= now_ms {
            return Ok(SocketSessionBindingStatus::Expired);
        }
        let role = space_id.and_then(|_| row.get::<String>("role").ok().and_then(|role| SpaceRole::parse(&role)));
        if space_id.is_some() && role.is_none() {
            return Ok(SocketSessionBindingStatus::MembershipLost);
        }
        Ok(SocketSessionBindingStatus::Active { role, expires_at_ms: record.expires_at })
    }

    async fn revoke_auth_session(&self, id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Option<RevokedAuthSession>> {
        let mut revoked = self.revoke_auth_sessions_by("id", id, None, reason, actor_user_id, correlation_id, None).await?;
        Ok(revoked.pop())
    }

    async fn revoke_auth_sessions_for_user(&self, user_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>> {
        self.revoke_auth_sessions_by("user", user_id, None, reason, actor_user_id, correlation_id, None).await
    }

    async fn revoke_auth_sessions_for_user_with_admin_effect(&self, user_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<Vec<RevokedAuthSession>> {
        self.revoke_auth_sessions_by_user_with_admin_effect(user_id, reason, actor_user_id, correlation_id, effect).await
    }

    async fn revoke_auth_sessions_for_identity(&self, provider: &str, subject_digest: [u8; 32], reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>> {
        self.revoke_auth_sessions_by("identity", provider, Some(subject_digest), reason, actor_user_id, correlation_id, None).await
    }

    async fn list_auth_audit(&self, limit: usize, offset: usize) -> DirectoryResult<Vec<AuthAuditRecord>> {
        if limit == 0 || limit > AUTH_AUDIT_PAGE_MAX {
            return Err(DirectoryError::Conflict(format!("auth audit limit must be 1..={AUTH_AUDIT_PAGE_MAX}")));
        }
        let mut result = self
            .graph
            .execute(query("MATCH (a:AuthAudit) RETURN a AS a ORDER BY a.occurredAt, a.id SKIP $offset LIMIT $limit").param("offset", i64::try_from(offset).map_err(backend)?).param("limit", i64::try_from(limit).map_err(backend)?))
            .await
            .map_err(backend)?;
        let mut records = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            records.push(auth_audit_from_node(&row)?);
        }
        Ok(records)
    }
    //#endregion

    //#region AdminOperations
    async fn claim_or_read_directory_command_receipt(&self, claim: &NewDirectoryCommandReceipt) -> DirectoryResult<DirectoryCommandClaimV1> {
        validate_directory_command_claim(claim)?;
        let key = directory_command_receipt_key(&claim.actor_user_id, &claim.request_id);
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut existing_result = txn.execute(query("MATCH (r:DirectoryCommandReceipt {key: $key}) RETURN r AS r LIMIT 1").param("key", key.clone())).await.map_err(backend)?;
        let existing = match existing_result.next(txn.handle()).await.map_err(backend)? {
            Some(row) => Some(directory_command_receipt_from_node(&row)?),
            None => None,
        };
        drop(existing_result);
        if let Some(record) = existing {
            txn.rollback().await.map_err(backend)?;
            return Ok(if record.command_sha256 == claim.command_sha256 { DirectoryCommandClaimV1::Existing(record) } else { DirectoryCommandClaimV1::Conflict });
        }
        txn.run(
            query("CREATE (:DirectoryCommandReceipt {key: $key, actorUserId: $actor_user_id, requestId: $request_id, commandSha256: $command_sha256, resultKind: $result_kind, disposition: 'pending', eventSeqFirst: 0, eventSeqLast: 0, receiptSha256: '', claimedAt: $claimed_at, completedAt: 0})")
                .param("key", key)
                .param("actor_user_id", claim.actor_user_id.clone())
                .param("request_id", claim.request_id.clone())
                .param("command_sha256", claim.command_sha256.clone())
                .param("result_kind", directory_command_result_kind_str(claim.result_kind))
                .param("claimed_at", claim.claimed_at),
        )
        .await
        .map_err(backend)?;
        txn.commit().await.map_err(backend)?;
        Ok(DirectoryCommandClaimV1::Claimed(DirectoryCommandReceiptRecord {
            actor_user_id: claim.actor_user_id.clone(),
            request_id: claim.request_id.clone(),
            command_sha256: claim.command_sha256.clone(),
            result_kind: claim.result_kind,
            disposition: DirectoryCommandDispositionV1::Pending,
            event_seq_first: None,
            event_seq_last: None,
            receipt_sha256: None,
            claimed_at: claim.claimed_at,
            completed_at: None,
        }))
    }

    async fn complete_directory_command_receipt(&self, completion: &DirectoryCommandReceiptCompletion) -> DirectoryResult<DirectoryCommandReceiptRecord> {
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut updated = txn
            .execute(
                query("MATCH (r:DirectoryCommandReceipt {key: $key, disposition: 'pending'}) SET r.disposition = 'completed', r.eventSeqFirst = $event_seq_first, r.eventSeqLast = $event_seq_last, r.receiptSha256 = $receipt_sha256, r.completedAt = $completed_at RETURN r AS r")
                    .param("key", directory_command_receipt_key(&completion.actor_user_id, &completion.request_id))
                    .param("event_seq_first", i64::try_from(completion.event_seq_first.unwrap_or(0)).map_err(backend)?)
                    .param("event_seq_last", i64::try_from(completion.event_seq_last.unwrap_or(0)).map_err(backend)?)
                    .param("receipt_sha256", completion.receipt_sha256.clone())
                    .param("completed_at", completion.completed_at),
            )
            .await
            .map_err(backend)?;
        let row = updated.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Conflict("directory command receipt claim is not pending".into()))?;
        let record = directory_command_receipt_from_node(&row)?;
        drop(updated);
        txn.commit().await.map_err(backend)?;
        Ok(record)
    }

    async fn release_directory_command_receipt(&self, actor_user_id: &str, request_id: &str, command_sha256: &str) -> DirectoryResult<()> {
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut result = txn
            .execute(
                query("MATCH (r:DirectoryCommandReceipt {key: $key, commandSha256: $command_sha256, disposition: 'pending'}) DELETE r RETURN count(*) AS changed")
                    .param("key", directory_command_receipt_key(actor_user_id, request_id))
                    .param("command_sha256", command_sha256),
            )
            .await
            .map_err(backend)?;
        let changed: i64 = result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("receipt release returned no count".into()))?.get("changed").map_err(backend)?;
        drop(result);
        if changed != 1 {
            txn.rollback().await.map_err(backend)?;
            return Err(DirectoryError::Conflict("exact pending directory command receipt is not owned".into()));
        }
        txn.commit().await.map_err(backend)?;
        Ok(())
    }

    async fn claim_or_read_checkpoint_publication(&self, claim: &NewCheckpointPublicationClaimV1) -> DirectoryResult<CheckpointPublicationClaimV1> {
        validate_checkpoint_publication_claim(claim)?;
        let key = checkpoint_publication_receipt_key(&claim.actor_user_id, &claim.correlation_id);
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut existing = txn.execute(query("MATCH (r:CheckpointPublicationReceipt {key: $key}) RETURN r AS r LIMIT 1").param("key", key.clone())).await.map_err(backend)?;
        let record = existing.next(txn.handle()).await.map_err(backend)?.map(|row| checkpoint_publication_receipt_from_node(&row)).transpose()?;
        drop(existing);
        if let Some(record) = record {
            txn.rollback().await.map_err(backend)?;
            return Ok(if record.command_sha256 == claim.command_sha256 { CheckpointPublicationClaimV1::Existing(record) } else { CheckpointPublicationClaimV1::Conflict });
        }
        txn.run(
            query("CREATE (:CheckpointPublicationReceipt {key: $key, actorUserId: $actor_user_id, correlationId: $correlation_id, commandSha256: $command_sha256, disposition: 'pending', checkpointId: '', claimedAt: $claimed_at, completedAt: 0})")
                .param("key", key)
                .param("actor_user_id", claim.actor_user_id.clone())
                .param("correlation_id", claim.correlation_id.clone())
                .param("command_sha256", claim.command_sha256.clone())
                .param("claimed_at", claim.claimed_at),
        )
        .await
        .map_err(backend)?;
        txn.commit().await.map_err(backend)?;
        Ok(CheckpointPublicationClaimV1::Claimed(CheckpointPublicationReceiptRecordV1 {
            actor_user_id: claim.actor_user_id.clone(),
            correlation_id: claim.correlation_id.clone(),
            command_sha256: claim.command_sha256.clone(),
            disposition: CheckpointPublicationDispositionV1::Pending,
            checkpoint_id: None,
            claimed_at: claim.claimed_at,
            completed_at: None,
        }))
    }

    async fn release_checkpoint_publication(&self, actor_user_id: &str, correlation_id: &str, command_sha256: &str) -> DirectoryResult<()> {
        self.graph
            .run(
                query("MATCH (r:CheckpointPublicationReceipt {key: $key, commandSha256: $command_sha256, disposition: 'pending'}) DELETE r")
                    .param("key", checkpoint_publication_receipt_key(actor_user_id, correlation_id))
                    .param("command_sha256", command_sha256.to_string()),
            )
            .await
            .map_err(backend)?;
        Ok(())
    }

    async fn append_admin_operation_audit(&self, fact: &NewAdminOperationAuditRecord) -> DirectoryResult<AdminOperationAuditRecord> {
        validate_admin_operation_audit(fact)?;
        let terminal = fact.phase != "accepted";
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut existing_result = txn.execute(query("MATCH (a:AdminOperationAudit {requestId: $request_id}) RETURN a AS a ORDER BY a.sequence LIMIT 2").param("request_id", fact.request_id.clone())).await.map_err(backend)?;
        let mut existing = Vec::new();
        while let Some(row) = existing_result.next(txn.handle()).await.map_err(backend)? {
            existing.push(admin_operation_audit_from_node(&row)?);
        }
        drop(existing_result);
        if !terminal {
            if let Some(established) = existing.first() {
                let outcome = if same_admin_operation_request(&established.fact, fact) { Ok(established.clone()) } else { Err(DirectoryError::Conflict("admin request id was reused for a different intent".into())) };
                txn.rollback().await.map_err(backend)?;
                return outcome;
            }
        } else {
            let accepted = existing.iter().find(|row| row.fact.phase == "accepted").ok_or_else(|| DirectoryError::Conflict("admin terminal fact requires accepted fact".into()))?;
            if accepted.fact.operation_id != fact.operation_id || !same_admin_operation_request(&accepted.fact, fact) {
                return Err(DirectoryError::Conflict("admin terminal fact changed operation identity".into()));
            }
            if let Some(established) = existing.iter().find(|row| row.fact.phase != "accepted") {
                let outcome = if established.fact.operation_id == fact.operation_id && established.fact.phase == fact.phase {
                    Ok(established.clone())
                } else {
                    Err(DirectoryError::Conflict("admin operation already terminated with a different outcome".into()))
                };
                txn.rollback().await.map_err(backend)?;
                return outcome;
            }
        }
        let mut sequence_result = txn.execute(query("MERGE (c:AdminOperationAuditCounter {id: 'singleton'}) ON CREATE SET c.sequence = 0 SET c.sequence = c.sequence + 1 RETURN c.sequence AS sequence")).await.map_err(backend)?;
        let sequence: i64 = sequence_result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("admin operation audit counter returned no row".into()))?.get("sequence").map_err(backend)?;
        drop(sequence_result);
        let mut merged = txn
            .execute(
            query("MERGE (a:AdminOperationAudit {requestTerminalKey: $request_terminal_key}) ON CREATE SET a.sequence = $sequence, a.requestId = $request_id, a.intentDigest = $intent_digest, a.operationId = $operation_id, a.occurredAt = $occurred_at, a.phase = $phase, a.intentKind = $intent_kind, a.targetKind = $target_kind, a.targetId = $target_id, a.principalUserId = $principal_user_id, a.principalSessionId = $principal_session_id, a.principalGeneration = $principal_generation, a.correlationId = $correlation_id, a.eventSeqFirst = $event_seq_first, a.eventSeqLast = $event_seq_last, a.outcomeCode = $outcome_code, a.reasonCode = $reason_code RETURN a AS a")
                .param("sequence", sequence)
                .param("request_id", fact.request_id.clone())
                .param("intent_digest", fact.intent_digest.clone())
                .param("request_terminal_key", format!("{}:{}", fact.request_id, u8::from(terminal)))
                .param("operation_id", fact.operation_id.clone())
                .param("occurred_at", fact.occurred_at)
                .param("phase", fact.phase.clone())
                .param("intent_kind", fact.intent_kind.clone())
                .param("target_kind", fact.target_kind.clone())
                .param("target_id", fact.target_id.clone())
                .param("principal_user_id", fact.principal_user_id.clone())
                .param("principal_session_id", fact.principal_session_id.clone())
                .param("principal_generation", i64::try_from(fact.principal_generation).map_err(backend)?)
                .param("correlation_id", fact.correlation_id.clone())
                .param("event_seq_first", fact.event_seq_first.map(i64::try_from).transpose().map_err(backend)?.unwrap_or_default())
                .param("event_seq_last", fact.event_seq_last.map(i64::try_from).transpose().map_err(backend)?.unwrap_or_default())
                .param("outcome_code", fact.outcome_code.clone())
                .param("reason_code", fact.reason_code.clone().unwrap_or_default()),
        )
        .await
        .map_err(backend)?;
        let established = merged.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("admin operation audit merge returned no row".into()))?;
        let established = admin_operation_audit_from_node(&established)?;
        drop(merged);
        txn.commit().await.map_err(backend)?;
        if same_admin_operation_request(&established.fact, fact) { Ok(established) } else { Err(DirectoryError::Conflict("admin request id race changed intent".into())) }
    }

    async fn admin_operation_audit_for_request(&self, request_id: &str) -> DirectoryResult<Vec<AdminOperationAuditRecord>> {
        validate_bounded_auth_text(request_id, "admin request id", AUTH_TEXT_MAX_BYTES)?;
        let mut result = self.graph.execute(query("MATCH (a:AdminOperationAudit {requestId: $request_id}) RETURN a AS a ORDER BY a.sequence LIMIT 2").param("request_id", request_id)).await.map_err(backend)?;
        let mut records = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            records.push(admin_operation_audit_from_node(&row)?);
        }
        Ok(records)
    }

    async fn admin_operation_audit_for_operation(&self, operation_id: &str) -> DirectoryResult<Vec<AdminOperationAuditRecord>> {
        validate_bounded_auth_text(operation_id, "admin operation id", AUTH_TEXT_MAX_BYTES)?;
        let mut result = self.graph.execute(query("MATCH (a:AdminOperationAudit {operationId: $operation_id}) RETURN a AS a ORDER BY a.sequence LIMIT 2").param("operation_id", operation_id)).await.map_err(backend)?;
        let mut records = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            records.push(admin_operation_audit_from_node(&row)?);
        }
        Ok(records)
    }

    async fn admin_operation_effect_receipt(&self, operation_id: &str, intent_digest: &str) -> DirectoryResult<Option<AdminOperationEffectReceiptV1>> {
        validate_bounded_auth_text(operation_id, "admin effect operation id", AUTH_TEXT_MAX_BYTES)?;
        if intent_digest.len() != 64 || !intent_digest.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()) {
            return Err(DirectoryError::Conflict("admin effect intent digest must be 64 lowercase hex digits".into()));
        }
        let mut result = self
            .graph
            .execute(query("MATCH (r:AdminOperationEffectReceipt {operationId: $operation_id, intentDigest: $intent_digest}) RETURN r AS r LIMIT 1").param("operation_id", operation_id).param("intent_digest", intent_digest))
            .await
            .map_err(backend)?;
        let Some(row) = result.next().await.map_err(backend)? else { return Ok(None) };
        let node: neo4rs::Node = row.get("r").map_err(backend)?;
        let first = node.get::<i64>("eventSeqFirst").ok().filter(|value| *value > 0).map(u64::try_from).transpose().map_err(backend)?;
        let last = node.get::<i64>("eventSeqLast").ok().filter(|value| *value > 0).map(u64::try_from).transpose().map_err(backend)?;
        let receipt = AdminOperationEffectReceiptV1 {
            operation_id: node.get("operationId").map_err(backend)?,
            intent_digest: node.get("intentDigest").map_err(backend)?,
            committed_at: node.get("committedAt").map_err(backend)?,
            outcome_code: node.get("outcomeCode").map_err(backend)?,
            event_seq_first: first,
            event_seq_last: last,
        };
        validate_admin_operation_effect_receipt(&receipt)?;
        Ok(Some(receipt))
    }

    async fn list_admin_operation_audit(&self, after_sequence: u64, limit: usize) -> DirectoryResult<Vec<AdminOperationAuditRecord>> {
        if limit == 0 || limit > ADMIN_PAGE_MAX {
            return Err(DirectoryError::Conflict(format!("admin audit limit must be 1..={ADMIN_PAGE_MAX}")));
        }
        let mut result = self
            .graph
            .execute(
                query("MATCH (a:AdminOperationAudit) WHERE a.sequence > $after RETURN a AS a ORDER BY a.sequence LIMIT $limit").param("after", i64::try_from(after_sequence).map_err(backend)?).param("limit", i64::try_from(limit).map_err(backend)?),
            )
            .await
            .map_err(backend)?;
        let mut records = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            records.push(admin_operation_audit_from_node(&row)?);
        }
        Ok(records)
    }
    //#endregion

    //#region Invites
    async fn issue_invite_as(&self, space_id: &str, role: SpaceRole, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<IssuedInvite> {
        let issued = prepare_invite(space_id, role, ttl_secs, now_ms())?;
        let audit = auth_audit(issued.record.created_at, "invite-issued", Some(&issued.record.id), None, actor_user_id, None, "success", None, correlation_id, "server")?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        txn.run(
            query("CREATE (i:SpaceInvite {id: $id, selector: $selector, secretDigest: $secret_digest, spaceId: $space_id, role: $role, createdAt: $created_at, expiresAt: $expires_at})")
                .param("id", issued.record.id.clone())
                .param("selector", issued.record.selector.clone())
                .param("secret_digest", encode_capability_bytes(&issued.record.secret_digest))
                .param("space_id", space_id)
                .param("role", role.as_str())
                .param("created_at", issued.record.created_at)
                .param("expires_at", issued.record.expires_at),
        )
        .await
        .map_err(backend)?;
        insert_auth_audit(&mut txn, &audit).await?;
        txn.commit().await.map_err(backend)?;
        Ok(issued)
    }

    async fn issue_invite_as_with_admin_effect(&self, space_id: &str, role: SpaceRole, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<IssuedInvite> {
        let issued = match admin_effect_preflight(prepare_invite(space_id, role, ttl_secs, now_ms())) {
            Ok(value) => value,
            Err(outcome) => return outcome.into(),
        };
        let audit = match admin_effect_preflight(auth_audit(issued.record.created_at, "invite-issued", Some(&issued.record.id), None, actor_user_id, None, "success", None, correlation_id, "server")) {
            Ok(value) => value,
            Err(outcome) => return outcome.into(),
        };
        let mut txn = match self.graph.start_txn().await {
            Ok(txn) => txn,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let changed_result: DirectoryResult<i64> = async {
            let mut result = txn
                .execute(
                    query("MATCH (s:Space {id: $space_id}) CREATE (i:SpaceInvite {id: $id, selector: $selector, secretDigest: $secret_digest, spaceId: $space_id, role: $role, createdAt: $created_at, expiresAt: $expires_at}) RETURN count(i) AS c")
                        .param("id", issued.record.id.clone())
                        .param("selector", issued.record.selector.clone())
                        .param("secret_digest", encode_capability_bytes(&issued.record.secret_digest))
                        .param("space_id", space_id)
                        .param("role", role.as_str())
                        .param("created_at", issued.record.created_at)
                        .param("expires_at", issued.record.expires_at),
                )
                .await
                .map_err(backend)?;
            let changed = result.next(txn.handle()).await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
            drop(result);
            Ok(changed)
        }
        .await;
        let changed = admin_effect_try!(txn, changed_result);
        if changed != 1 {
            return admin_effect_rollback(txn).await;
        }
        admin_effect_try!(txn, insert_auth_audit(&mut txn, &audit).await);
        let receipt = admin_effect_try!(txn, admin_operation_effect_receipt_v1(effect, &[]));
        admin_effect_try!(txn, insert_admin_operation_effect_receipt(&mut txn, &receipt).await);
        match txn.commit().await {
            Ok(()) => AdminEffectCommitV1::Applied(issued),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }

    async fn invite_redemption_scope_hint(&self, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str) -> DirectoryResult<InviteRedemptionScopeHintV1> {
        let mut result = self
            .graph
            .execute(query("MATCH (i:SpaceInvite {selector: $selector}) MATCH (s:Space {id: i.spaceId}) MATCH (u:User {id: $user_id}) RETURN i AS i").param("selector", capability.selector()).param("user_id", user_id))
            .await
            .map_err(backend)?;
        let record = result.next().await.map_err(backend)?.map(|row| invite_from_node(&row)).transpose()?;
        verify_invite_redemption_scope_hint(record.as_ref(), capability, actor, user_id)
    }

    async fn redeem_invite_atomic(&self, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str, hlc: Hlc) -> DirectoryResult<InviteRedemptionCommit> {
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let accepted_at_ms = now_ms();
        let mut counter_lock = txn.execute(query("MERGE (c:DirectoryCounter {id: 'singleton'}) ON CREATE SET c.seq = 0 SET c.claimNonce = coalesce(c.claimNonce, 0) + 1 RETURN c.seq AS seq")).await.map_err(backend)?;
        counter_lock.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("directory counter lock returned no row".into()))?;
        drop(counter_lock);
        let mut invite_lock = txn.execute(query("MATCH (i:SpaceInvite {selector: $selector}) SET i.claimNonce = coalesce(i.claimNonce, 0) + 1 RETURN i AS i").param("selector", capability.selector())).await.map_err(backend)?;
        let record = invite_lock.next(txn.handle()).await.map_err(backend)?.map(|row| invite_from_node(&row)).transpose()?;
        drop(invite_lock);
        let mut user_query = txn.execute(query("MATCH (u:User {id: $user_id}) RETURN count(u) AS count").param("user_id", user_id)).await.map_err(backend)?;
        let user_exists = user_query.next(txn.handle()).await.map_err(backend)?.and_then(|row| row.get::<i64>("count").ok()).unwrap_or(0) == 1;
        drop(user_query);
        let space_kind: Option<String> = match record.as_ref() {
            Some(invite) => {
                let mut result = txn.execute(query("MATCH (s:Space {id: $space_id}) RETURN s.kind AS kind").param("space_id", invite.space_id.clone())).await.map_err(backend)?;
                let kind = result.next(txn.handle()).await.map_err(backend)?.map(|row| row.get::<String>("kind").map_err(backend)).transpose()?;
                drop(result);
                kind
            }
            None => None,
        };
        let space_state = InviteRedemptionSpaceStateV1::from_kind(space_kind.as_deref())?;
        match invite_redemption_preflight(record.as_ref(), capability, actor, user_id, user_exists, space_state, accepted_at_ms) {
            InviteRedemptionPreflight::AlreadyCommitted => {
                let invite = record.as_ref().expect("committed preflight requires a record");
                let mut result = txn.execute(query("MATCH (e:DirectoryEvent {id: $event_id}) RETURN e AS e").param("event_id", invite.accepted_event_id.clone().unwrap_or_default())).await.map_err(backend)?;
                let event = result.next(txn.handle()).await.map_err(backend)?.map(|row| event_from_node(&row)).transpose()?;
                drop(result);
                let event = verify_invite_redemption_event(invite, event, user_id)?;
                txn.commit().await.map_err(backend)?;
                return Ok(InviteRedemptionCommit::AlreadyCommitted { event });
            }
            InviteRedemptionPreflight::Revoked => return Err(DirectoryError::Conflict("invite already revoked".into())),
            InviteRedemptionPreflight::Expired => return Err(DirectoryError::Conflict("invite expired".into())),
            InviteRedemptionPreflight::Denied => return Err(DirectoryError::Unauthorized),
            InviteRedemptionPreflight::Corrupt => return Err(DirectoryError::Backend("invite acceptance marker is incomplete".into())),
            InviteRedemptionPreflight::Claim => {}
        }
        let invite = record.expect("claim preflight requires a record");
        let id = time_ordered_id();
        let mut claimed = txn
            .execute(
                query("MATCH (i:SpaceInvite {id: $invite_id}) WHERE i.acceptedAt IS NULL AND i.acceptedEventId IS NULL AND i.revokedAt IS NULL AND i.expiresAt > $accepted_at SET i.acceptedAt = $accepted_at, i.acceptedEventId = $event_id RETURN count(i) AS count")
                    .param("invite_id", invite.id.clone())
                    .param("accepted_at", accepted_at_ms)
                    .param("event_id", id.clone()),
            )
            .await
            .map_err(backend)?;
        let changed = claimed.next(txn.handle()).await.map_err(backend)?.and_then(|row| row.get::<i64>("count").ok()).unwrap_or(0);
        drop(claimed);
        if changed != 1 {
            return Err(DirectoryError::Backend("Neo4j invitation claim lost its transaction fence".into()));
        }
        let event = NewDirectoryEvent {
            hlc,
            actor: actor.clone(),
            space_id: Some(invite.space_id.clone()),
            user_id: Some(user_id.to_string()),
            body: DirectoryEventBody::InviteRedeemed { space_id: invite.space_id, user_id: user_id.to_string(), invite_id: invite.id, role: role_to_wire(invite.role) },
        };
        let mut counter = txn.execute(query("MATCH (c:DirectoryCounter {id: 'singleton'}) SET c.seq = c.seq + 1 RETURN c.seq AS seq")).await.map_err(backend)?;
        let sequence: i64 = counter.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("directory counter query returned no row".into()))?.get("seq").map_err(backend)?;
        drop(counter);
        let sequence = u64::try_from(sequence).map_err(backend)?;
        if sequence > DIRECTORY_WIRE_INTEGER_MAX {
            return Err(DirectoryError::Conflict("directory event sequence exceeds the public integer boundary".into()));
        }
        let payload_value = serde_json::Value::from(&event.body.to_value());
        let kind = payload_value.get("kind").and_then(|value| value.as_str()).unwrap_or_default().to_string();
        let persisted = DirectoryEvent { seq: sequence, id: id.clone(), hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone(), recorded_at_ms: accepted_at_ms };
        validate_directory_event_page_event(&persisted).map_err(|_| DirectoryError::Conflict("directory event violates the bounded event-page contract".into()))?;
        txn.run(
            query("CREATE (e:DirectoryEvent {seq: $seq, id: $id, hlcPhysical: $hlc_physical, hlcLogical: $hlc_logical, actorKind: $actor_kind, actorId: $actor_id, spaceId: $space_id, userId: $user_id, kind: $kind, payload: $payload, recordedAt: $recorded_at})")
                .param("seq", i64::try_from(sequence).map_err(backend)?)
                .param("id", id.clone())
                .param("hlc_physical", event.hlc.physical_ms)
                .param("hlc_logical", i64::from(event.hlc.logical))
                .param("actor_kind", actor_kind_to_str(event.actor.kind))
                .param("actor_id", event.actor.id.clone())
                .param("space_id", event.space_id.clone())
                .param("user_id", event.user_id.clone())
                .param("kind", kind)
                .param("payload", payload_value.to_string())
                .param("recorded_at", accepted_at_ms),
        )
        .await
        .map_err(backend)?;
        if let Err(error) = self.project(&mut txn, &persisted).await {
            txn.rollback().await.map_err(backend)?;
            return Err(error);
        }
        txn.commit().await.map_err(backend)?;
        Ok(InviteRedemptionCommit::NewlyCommitted { event: persisted })
    }

    async fn revoke_invite_as(&self, space_id: &str, invite_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()> {
        validate_bounded_auth_text(reason, "invite revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let audit = auth_audit(revoked_at, "invite-revoked", Some(invite_id), None, actor_user_id, None, "success", Some(reason), correlation_id, "server")?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut result = txn
            .execute(
                query("MATCH (i:SpaceInvite {spaceId: $space_id, id: $id}) WHERE i.revokedAt IS NULL AND i.acceptedAt IS NULL SET i.revokedAt = $revoked_at, i.revokedReason = $reason RETURN count(i) AS c")
                    .param("id", invite_id)
                    .param("space_id", space_id)
                    .param("revoked_at", revoked_at)
                    .param("reason", reason),
            )
            .await
            .map_err(backend)?;
        let changed: i64 = result.next(txn.handle()).await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
        if changed == 0 {
            drop(result);
            let mut accepted = txn.execute(query("MATCH (i:SpaceInvite {spaceId: $space_id, id: $id}) RETURN i.acceptedAt AS acceptedAt").param("id", invite_id).param("space_id", space_id)).await.map_err(backend)?;
            let accepted = accepted.next(txn.handle()).await.map_err(backend)?.and_then(|row| row.get::<i64>("acceptedAt").ok()).is_some();
            return if accepted { Err(DirectoryError::Conflict("invite already accepted".into())) } else { Err(DirectoryError::NotFound(format!("invite {invite_id}"))) };
        }
        drop(result);
        insert_auth_audit(&mut txn, &audit).await?;
        txn.commit().await.map_err(backend)?;
        Ok(())
    }

    async fn revoke_invite_as_with_admin_effect(&self, space_id: &str, invite_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<()> {
        if let Err(outcome) = admin_effect_preflight(validate_bounded_auth_text(reason, "invite revoke reason", AUTH_TEXT_MAX_BYTES)) {
            return outcome.into();
        }
        let revoked_at = now_ms();
        let audit = match admin_effect_preflight(auth_audit(revoked_at, "invite-revoked", Some(invite_id), None, actor_user_id, None, "success", Some(reason), correlation_id, "server")) {
            Ok(value) => value,
            Err(outcome) => return outcome.into(),
        };
        let mut txn = match self.graph.start_txn().await {
            Ok(txn) => txn,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let changed_result: DirectoryResult<i64> = async {
            let mut result = txn
                .execute(
                    query("MATCH (i:SpaceInvite {spaceId: $space_id, id: $id}) WHERE i.revokedAt IS NULL AND i.acceptedAt IS NULL SET i.revokedAt = $revoked_at, i.revokedReason = $reason RETURN count(i) AS c")
                        .param("id", invite_id)
                        .param("space_id", space_id)
                        .param("revoked_at", revoked_at)
                        .param("reason", reason),
                )
                .await
                .map_err(backend)?;
            let changed = result.next(txn.handle()).await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
            drop(result);
            Ok(changed)
        }
        .await;
        let changed = admin_effect_try!(txn, changed_result);
        if changed != 1 {
            return admin_effect_rollback(txn).await;
        }
        admin_effect_try!(txn, insert_auth_audit(&mut txn, &audit).await);
        let receipt = admin_effect_try!(txn, admin_operation_effect_receipt_v1(effect, &[]));
        admin_effect_try!(txn, insert_admin_operation_effect_receipt(&mut txn, &receipt).await);
        match txn.commit().await {
            Ok(()) => AdminEffectCommitV1::Applied(()),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }

    async fn list_invites(&self, space_id: &str) -> DirectoryResult<Vec<InviteRecord>> {
        let mut result = self.graph.execute(query("MATCH (i:SpaceInvite {spaceId: $space_id}) RETURN i AS i ORDER BY i.createdAt DESC").param("space_id", space_id)).await.map_err(backend)?;
        let mut invites = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            invites.push(invite_from_node(&row)?);
        }
        Ok(invites)
    }

    async fn list_space_administration_invites_page(&self, space_id: &str, after: Option<(i64, &str)>, limit: usize) -> DirectoryResult<Vec<SpaceAdministrationInviteRow>> {
        if limit == 0 || limit > super::SPACE_ADMINISTRATION_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("space administration invite page limit must be 1..={}", super::SPACE_ADMINISTRATION_PAGE_FETCH_MAX)));
        }
        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH (i:SpaceInvite {spaceId: $space_id})
                     WHERE $after_created_at IS NULL OR i.createdAt < $after_created_at OR (i.createdAt = $after_created_at AND i.id < $after_id)
                     RETURN i.id AS inviteId, i.role AS role, i.createdAt AS createdAt, i.expiresAt AS expiresAt, i.revokedAt AS revokedAt, i.acceptedAt AS acceptedAt
                     ORDER BY i.createdAt DESC, i.id DESC LIMIT $limit",
                )
                .param("space_id", space_id)
                .param("after_created_at", after.map(|(created_at, _)| created_at))
                .param("after_id", after.map(|(_, invite_id)| invite_id.to_owned()))
                .param("limit", i64::try_from(limit).map_err(backend)?),
            )
            .await
            .map_err(backend)?;
        let mut invites = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let role: String = row.get("role").map_err(backend)?;
            invites.push(SpaceAdministrationInviteRow {
                invite_id: row.get("inviteId").map_err(backend)?,
                role: SpaceRole::parse(&role).ok_or_else(|| DirectoryError::Backend("stored invite role is invalid".into()))?,
                created_at_ms: row.get("createdAt").map_err(backend)?,
                expires_at_ms: row.get("expiresAt").map_err(backend)?,
                revoked: row.get::<i64>("revokedAt").is_ok(),
                accepted: row.get::<i64>("acceptedAt").is_ok(),
            });
        }
        Ok(invites)
    }
    //#endregion

    //#region SyncSessions
    async fn record_sync_session_open(
        &self,
        auth_session_id: Option<&str>,
        authorization_generation: u64,
        actor_id: &str,
        space_id: &str,
        document_id: &str,
        surface: &str,
        user_id: Option<&str>,
        authenticated_email: Option<&str>,
        space_role: Option<SpaceRole>,
        client_label: &str,
    ) -> DirectoryResult<SyncSessionRecord> {
        validate_bounded_auth_text(actor_id, "sync actor", AUTH_TEXT_MAX_BYTES)?;
        let id = time_ordered_id();
        let connected_at = now_ms();
        let role_str = space_role.map(|r| r.as_str().to_string()).unwrap_or_default();
        match user_id {
            Some(user_id) => {
                self.graph
                    .run(
                        query(
                            "MATCH (u:User {id: $user_id})
                             CREATE (s:SyncSession {id: $id, authSessionId: $auth_session_id, authorizationGeneration: $generation, actorId: $actor_id, spaceId: $space_id, documentId: $document_id, surface: $surface, authenticatedEmail: $authenticated_email, clientLabel: $client_label, spaceRole: $role, connectedAt: $connected_at})
                             CREATE (s)-[:AS_USER]->(u)",
                        )
                        .param("space_id", space_id)
                        .param("document_id", document_id)
                        .param("surface", surface)
                        .param("user_id", user_id)
                        .param("authenticated_email", authenticated_email.unwrap_or_default())
                        .param("id", id.clone())
                        .param("auth_session_id", auth_session_id.unwrap_or_default())
                        .param("generation", i64::try_from(authorization_generation).map_err(backend)?)
                        .param("actor_id", actor_id)
                        .param("client_label", client_label)
                        .param("role", role_str.clone())
                        .param("connected_at", connected_at),
                    )
                    .await
                    .map_err(backend)?;
            }
            None => {
                self.graph
                    .run(
                        query("CREATE (s:SyncSession {id: $id, authSessionId: $auth_session_id, authorizationGeneration: $generation, actorId: $actor_id, spaceId: $space_id, documentId: $document_id, surface: $surface, authenticatedEmail: $authenticated_email, clientLabel: $client_label, spaceRole: $role, connectedAt: $connected_at})")
                            .param("space_id", space_id)
                            .param("document_id", document_id)
                            .param("surface", surface)
                            .param("authenticated_email", authenticated_email.unwrap_or_default())
                            .param("id", id.clone())
                            .param("auth_session_id", auth_session_id.unwrap_or_default())
                            .param("generation", i64::try_from(authorization_generation).map_err(backend)?)
                            .param("actor_id", actor_id)
                            .param("client_label", client_label)
                            .param("role", role_str.clone())
                            .param("connected_at", connected_at),
                    )
                    .await
                    .map_err(backend)?;
            }
        }
        Ok(SyncSessionRecord {
            id,
            auth_session_id: auth_session_id.map(str::to_string),
            authorization_generation,
            actor_id: actor_id.to_string(),
            space_id: space_id.to_string(),
            document_id: document_id.to_string(),
            surface: surface.to_string(),
            user_id: user_id.map(str::to_string),
            authenticated_email: authenticated_email.map(str::to_string),
            space_role,
            client_label: client_label.to_string(),
            connected_at,
            disconnected_at: None,
        })
    }

    async fn record_sync_session_close(&self, sync_session_id: &str) -> DirectoryResult<()> {
        self.graph.run(query("MATCH (s:SyncSession {id: $id}) SET s.disconnectedAt = $disconnected_at").param("id", sync_session_id).param("disconnected_at", now_ms())).await.map_err(backend)?;
        Ok(())
    }

    async fn list_sync_sessions_for_document(&self, document_id: &str) -> DirectoryResult<Vec<SyncSessionRecord>> {
        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH (s:SyncSession {documentId: $document_id})
                     OPTIONAL MATCH (s)-[:AS_USER]->(u:User)
                     RETURN s.id AS id, s.authSessionId AS authSessionId, s.authorizationGeneration AS generation, s.actorId AS actorId,
                            s.spaceId AS spaceId, s.surface AS surface, u.id AS userId, s.authenticatedEmail AS authenticatedEmail, s.spaceRole AS role, s.clientLabel AS clientLabel,
                            s.connectedAt AS connectedAt, s.disconnectedAt AS disconnectedAt
                     ORDER BY s.connectedAt DESC",
                )
                .param("document_id", document_id),
            )
            .await
            .map_err(backend)?;
        let mut sessions = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            sessions.push(sync_session_from_row(&row, document_id)?);
        }
        Ok(sessions)
    }

    async fn list_active_sync_sessions_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<SyncSessionRecord>> {
        if limit == 0 || limit > super::ACTIVE_SYNC_SESSION_READ_MAX {
            return Err(DirectoryError::Conflict(format!("active sync-session limit must be 1..={}", super::ACTIVE_SYNC_SESSION_READ_MAX)));
        }
        let cypher = "MATCH (s:SyncSession)
                       WHERE s.disconnectedAt IS NULL AND ($space_id IS NULL OR s.spaceId = $space_id)
                       OPTIONAL MATCH (s)-[:AS_USER]->(u:User)
                       RETURN s.id AS id, s.authSessionId AS authSessionId, s.authorizationGeneration AS generation, s.actorId AS actorId,
                              s.spaceId AS spaceId, s.documentId AS documentId, s.surface AS surface, u.id AS userId, s.authenticatedEmail AS authenticatedEmail,
                              s.spaceRole AS role, s.clientLabel AS clientLabel, s.connectedAt AS connectedAt, s.disconnectedAt AS disconnectedAt
                       ORDER BY s.connectedAt DESC, s.id ASC SKIP $offset LIMIT $limit";
        let mut result = self.graph.execute(query(cypher).param("space_id", space_id.map(str::to_string)).param("offset", i64::try_from(offset).map_err(backend)?).param("limit", i64::try_from(limit).map_err(backend)?)).await.map_err(backend)?;
        let mut sessions = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let document_id: String = row.get("documentId").map_err(backend)?;
            sessions.push(sync_session_from_row(&row, &document_id)?);
        }
        Ok(sessions)
    }

    async fn close_all_sync_sessions(&self) -> DirectoryResult<()> {
        self.graph.run(query("MATCH (s:SyncSession) WHERE s.disconnectedAt IS NULL SET s.disconnectedAt = $now").param("now", now_ms())).await.map_err(backend)?;
        Ok(())
    }
    //#endregion

    async fn reserve_artifact_cas(&self, plan: &ArtifactCasOwnershipPlanV1, expires_at_ms: u64, now_ms: u64) -> DirectoryResult<ArtifactCasReservation> {
        let encoded = encode_artifact_cas_ownership_v1(plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        if expires_at_ms <= now_ms || expires_at_ms.checked_sub(now_ms).is_none_or(|ttl| ttl > ARTIFACT_CAS_RESERVATION_MAX_TTL_MS) {
            return Err(DirectoryError::Conflict(format!("artifact CAS reservation ttl must be 1..={ARTIFACT_CAS_RESERVATION_MAX_TTL_MS} milliseconds")));
        }
        let scope_key = checkpoint_key_v1(&plan.scope, plan.checkpoint_id);
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let (coordinator_id, physical_epoch) = cas_reservation_barrier(&mut txn, &plan.scope.space_id, i64::try_from(now_ms).map_err(backend)?).await?;
        let mut historical = txn.execute(query("MATCH (e:ArtifactCasLedgerEvent {scopeCheckpointKey: $key}) WHERE e.plan IS NOT NULL RETURN e.plan AS plan ORDER BY e.generation LIMIT 1").param("key", scope_key.clone())).await.map_err(backend)?;
        if let Some(row) = historical.next(txn.handle()).await.map_err(backend)? {
            let stored: neo4rs::BoltBytes = row.get("plan").map_err(backend)?;
            if stored.value.as_ref() != encoded {
                return Err(DirectoryError::Conflict("artifact CAS checkpoint identity names a different ownership plan".into()));
            }
        }
        drop(historical);
        let mut published = txn.execute(query("MATCH (r:ArtifactCasReference {scopeCheckpointKey: $key}) RETURN r.generation AS generation, r.writeEpoch AS writeEpoch, r.plan AS plan").param("key", scope_key.clone())).await.map_err(backend)?;
        if let Some(row) = published.next(txn.handle()).await.map_err(backend)? {
            let stored: neo4rs::BoltBytes = row.get("plan").map_err(backend)?;
            if stored.value.as_ref() != encoded {
                return Err(DirectoryError::Conflict("artifact CAS published ownership conflict".into()));
            }
            let reservation = ArtifactCasReservation::fenced(
                plan.clone(),
                u64::try_from(row.get::<i64>("generation").map_err(backend)?).map_err(backend)?,
                u64::try_from(row.get::<i64>("writeEpoch").map_err(backend)?).map_err(backend)?,
                i64::MAX as u64,
                coordinator_id,
                physical_epoch,
            );
            drop(published);
            txn.commit().await.map_err(backend)?;
            return Ok(reservation);
        }
        drop(published);
        let mut released = txn.execute(query("MATCH (e:ArtifactCasLedgerEvent {scopeCheckpointKey: $key, operation: 'publish'}) RETURN count(e) > 0 AS released").param("key", scope_key.clone())).await.map_err(backend)?;
        let was_released: bool = released.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS released checkpoint query returned no row".into()))?.get("released").map_err(backend)?;
        drop(released);
        if was_released {
            return Err(DirectoryError::Conflict("artifact CAS released checkpoint cannot be reserved again".into()));
        }
        let mut current = txn
            .execute(query("MATCH (r:ArtifactCasReservation {scopeCheckpointKey: $key}) RETURN r.generation AS generation, r.writeEpoch AS writeEpoch, r.expiresAtMs AS expiresAtMs, r.plan AS plan").param("key", scope_key.clone()))
            .await
            .map_err(backend)?;
        if let Some(row) = current.next(txn.handle()).await.map_err(backend)? {
            let stored: neo4rs::BoltBytes = row.get("plan").map_err(backend)?;
            if stored.value.as_ref() != encoded {
                return Err(DirectoryError::Conflict("artifact CAS reservation identity conflict".into()));
            }
            let expiry: i64 = row.get("expiresAtMs").map_err(backend)?;
            if expiry > i64::try_from(now_ms).map_err(backend)? {
                let reservation = ArtifactCasReservation::fenced(
                    plan.clone(),
                    u64::try_from(row.get::<i64>("generation").map_err(backend)?).map_err(backend)?,
                    u64::try_from(row.get::<i64>("writeEpoch").map_err(backend)?).map_err(backend)?,
                    u64::try_from(expiry).map_err(backend)?,
                    coordinator_id,
                    physical_epoch,
                );
                drop(current);
                txn.commit().await.map_err(backend)?;
                return Ok(reservation);
            }
        }
        drop(current);
        let mut epoch_result = txn.execute(query("MATCH (e:ArtifactCasLedgerEvent {scopeCheckpointKey: $key}) RETURN coalesce(max(e.writeEpoch), 0) AS writeEpoch").param("key", scope_key.clone())).await.map_err(backend)?;
        let previous_epoch: i64 = epoch_result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS write epoch query returned no row".into()))?.get("writeEpoch").map_err(backend)?;
        drop(epoch_result);
        let write_epoch = previous_epoch.checked_add(1).ok_or_else(|| DirectoryError::Conflict("artifact CAS write epoch overflow".into()))?;
        let generation = cas_generation(&mut txn).await?;
        txn.run(query("CREATE (:ArtifactCasLedgerEvent {generation: $generation, operation: 'reserve', scopeCheckpointKey: $key, spaceId: $space_id, documentId: $document_id, checkpointId: $checkpoint_id, writeEpoch: $write_epoch, expiresAtMs: $expires_at, plan: $plan})")
            .param("generation", generation).param("key", scope_key).param("space_id", plan.scope.space_id.clone()).param("document_id", plan.scope.document_id.clone()).param("checkpoint_id", hex_lower(&plan.checkpoint_id.0)).param("write_epoch", write_epoch).param("expires_at", i64::try_from(expires_at_ms).map_err(backend)?).param("plan", encoded)).await.map_err(backend)?;
        let reservation = ArtifactCasReservation::fenced(plan.clone(), u64::try_from(generation).map_err(backend)?, u64::try_from(write_epoch).map_err(backend)?, expires_at_ms, coordinator_id, physical_epoch);
        cas_project_reserve(&mut txn, &reservation).await?;
        txn.commit().await.map_err(backend)?;
        Ok(reservation)
    }

    async fn append_reserved_artifact_checkpoint(
        &self,
        event: Option<&NewDirectoryEvent>,
        checkpoint: &ArtifactCheckpoint,
        reservation: &ArtifactCasReservation,
        completion: Option<&CheckpointPublicationCompletionV1>,
        current_now_ms: u64,
    ) -> DirectoryResult<Vec<DirectoryEvent>> {
        if checkpoint.parent_checkpoint_id.is_none() || !checkpoint.baseline_frontier.is_edited_for(&checkpoint.scope) {
            return Err(DirectoryError::Conflict("ordinary artifact checkpoint publication requires established edited lineage".into()));
        }
        validate_artifact_cas_publication_v1(&reservation.plan, checkpoint).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        if let Some(event) = event {
            validate_verified_checkpoint_append(event, checkpoint)?;
        }
        let scope_key = checkpoint_key_v1(&reservation.plan.scope, reservation.plan.checkpoint_id);
        let encoded = encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        if cas_lock_space(&mut txn, &reservation.plan.scope.space_id).await? > i64::try_from(current_now_ms).map_err(backend)? {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is active for this space".into()));
        }
        let mut published = txn.execute(query("MATCH (r:ArtifactCasReference {scopeCheckpointKey: $key}) RETURN r.generation AS generation, r.writeEpoch AS writeEpoch, r.plan AS plan").param("key", scope_key.clone())).await.map_err(backend)?;
        if let Some(row) = published.next(txn.handle()).await.map_err(backend)? {
            let stored: neo4rs::BoltBytes = row.get("plan").map_err(backend)?;
            if event.is_some()
                || row.get::<i64>("generation").map_err(backend)? != i64::try_from(reservation.generation).map_err(backend)?
                || row.get::<i64>("writeEpoch").map_err(backend)? != i64::try_from(reservation.write_epoch).map_err(backend)?
                || stored.value.as_ref() != encoded
            {
                return Err(DirectoryError::Conflict("artifact CAS published reservation conflict".into()));
            }
            return Ok(Vec::new());
        }
        drop(published);
        let mut current = txn
            .execute(query("MATCH (r:ArtifactCasReservation {scopeCheckpointKey: $key}) RETURN r.generation AS generation, r.writeEpoch AS writeEpoch, r.expiresAtMs AS expiresAtMs, r.plan AS plan").param("key", scope_key.clone()))
            .await
            .map_err(backend)?;
        let row = current.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Conflict("artifact CAS reservation is missing, expired, or superseded".into()))?;
        let stored: neo4rs::BoltBytes = row.get("plan").map_err(backend)?;
        if row.get::<i64>("generation").map_err(backend)? != i64::try_from(reservation.generation).map_err(backend)?
            || row.get::<i64>("writeEpoch").map_err(backend)? != i64::try_from(reservation.write_epoch).map_err(backend)?
            || row.get::<i64>("expiresAtMs").map_err(backend)? != i64::try_from(reservation.expires_at_ms).map_err(backend)?
            || reservation.expires_at_ms <= current_now_ms
            || stored.value.as_ref() != encoded
        {
            return Err(DirectoryError::Conflict("artifact CAS reservation is missing, expired, or superseded".into()));
        }
        drop(current);
        let event = event.ok_or_else(|| DirectoryError::Conflict("new artifact CAS publication requires one public event".into()))?;
        let id = time_ordered_id();
        let recorded_at_ms = now_ms();
        let payload_value = serde_json::Value::from(&event.body.to_value());
        let kind = payload_value.get("kind").and_then(|value| value.as_str()).unwrap_or_default().to_string();
        let mut counter = txn.execute(query("MERGE (c:DirectoryCounter {id: 'singleton'}) ON CREATE SET c.seq = 0 SET c.seq = c.seq + 1 RETURN c.seq AS seq")).await.map_err(backend)?;
        let seq: i64 = counter.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("directory counter query returned no row".into()))?.get("seq").map_err(backend)?;
        drop(counter);
        let public_seq = u64::try_from(seq).map_err(backend)?;
        if public_seq > DIRECTORY_WIRE_INTEGER_MAX {
            return Err(DirectoryError::Conflict("directory event sequence exceeds the public integer boundary".into()));
        }
        let full = DirectoryEvent { seq: public_seq, id: id.clone(), hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone(), recorded_at_ms };
        validate_directory_event_page_event(&full).map_err(|_| DirectoryError::Conflict("directory event violates the bounded event-page contract".into()))?;
        txn.run(query("CREATE (e:DirectoryEvent {seq: $seq, id: $id, hlcPhysical: $hlc_physical, hlcLogical: $hlc_logical, actorKind: $actor_kind, actorId: $actor_id, spaceId: $space_id, userId: $user_id, kind: $kind, payload: $payload, recordedAt: $recorded_at})")
            .param("seq", seq).param("id", id.clone()).param("hlc_physical", event.hlc.physical_ms).param("hlc_logical", i64::from(event.hlc.logical)).param("actor_kind", actor_kind_to_str(event.actor.kind)).param("actor_id", event.actor.id.clone()).param("space_id", event.space_id.clone()).param("user_id", event.user_id.clone()).param("kind", kind).param("payload", payload_value.to_string()).param("recorded_at", recorded_at_ms)).await.map_err(backend)?;
        txn.run(
            query("CREATE (:ArtifactAuthorityEvent {eventSeq: $event_seq, scopeCheckpointKey: $key, payload: $payload})").param("event_seq", seq).param("key", scope_key.clone()).param("payload", directory::os_pack::json::to_json_string(checkpoint)),
        )
        .await
        .map_err(backend)?;
        self.project(&mut txn, &full).await?;
        self.project_verified_checkpoint(&mut txn, &full, checkpoint).await?;
        let generation = cas_generation(&mut txn).await?;
        txn.run(query("CREATE (:ArtifactCasLedgerEvent {generation: $generation, operation: 'publish', scopeCheckpointKey: $key, spaceId: $space_id, documentId: $document_id, checkpointId: $checkpoint_id, writeEpoch: $write_epoch, expiresAtMs: $expires_at, eventSeq: $event_seq, plan: $plan})")
            .param("generation", generation).param("key", scope_key).param("space_id", reservation.plan.scope.space_id.clone()).param("document_id", reservation.plan.scope.document_id.clone()).param("checkpoint_id", hex_lower(&reservation.plan.checkpoint_id.0)).param("write_epoch", i64::try_from(reservation.write_epoch).map_err(backend)?).param("expires_at", i64::try_from(reservation.expires_at_ms).map_err(backend)?).param("event_seq", seq).param("plan", encoded)).await.map_err(backend)?;
        cas_project_publish(&mut txn, reservation, generation).await?;
        if let Some(completion) = completion {
            validate_checkpoint_publication_completion(completion)?;
            let mut updated = txn
                .execute(
                    query("MATCH (r:CheckpointPublicationReceipt {key: $key, commandSha256: $command_sha256, disposition: 'pending'}) SET r.disposition = 'completed', r.checkpointId = $checkpoint_id, r.completedAt = $completed_at RETURN count(r) AS changed")
                        .param("key", checkpoint_publication_receipt_key(&completion.actor_user_id, &completion.correlation_id))
                        .param("command_sha256", completion.command_sha256.clone())
                        .param("checkpoint_id", hex_lower(&completion.checkpoint_id.0))
                        .param("completed_at", completion.completed_at),
                )
                .await
                .map_err(backend)?;
            let changed = updated.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Conflict("checkpoint publication completion returned no row".into()))?.get::<i64>("changed").map_err(backend)?;
            drop(updated);
            if changed != 1 {
                return Err(DirectoryError::Conflict("checkpoint publication claim is missing, substituted, or already completed".into()));
            }
        }
        txn.commit().await.map_err(backend)?;
        Ok(vec![full])
    }

    async fn artifact_cas_ledger_generation(&self) -> DirectoryResult<u64> {
        let mut head = self.graph.execute(query("MATCH (h:ArtifactCasLedgerHead {id: 'singleton'}) RETURN h.generation AS generation")).await.map_err(backend)?;
        match head.next().await.map_err(backend)? {
            Some(row) => u64::try_from(row.get::<i64>("generation").map_err(backend)?).map_err(backend),
            None => Ok(0),
        }
    }

    async fn artifact_cas_coordinator_id(&self) -> DirectoryResult<[u8; 32]> {
        let mut result = self.graph.execute(query("MATCH (b:ArtifactCasBarrierIdentity {id: 'singleton'}) RETURN b.coordinatorId AS coordinator")).await.map_err(backend)?;
        let row = result.next().await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS barrier coordinator identity is missing".into()))?;
        let bytes: neo4rs::BoltBytes = row.get("coordinator").map_err(backend)?;
        bytes.value.to_vec().try_into().map_err(|_| DirectoryError::Backend("artifact CAS barrier coordinator identity is invalid".into()))
    }

    async fn artifact_cas_sweep_candidates(&self, after_generation: u64, through_generation: u64, limit: usize) -> DirectoryResult<ArtifactCasSweepCandidatePage> {
        if limit == 0 || limit > ARTIFACT_CAS_SWEEP_PAGE_MAX {
            return Err(DirectoryError::Conflict(format!("artifact CAS sweep page requires limit 1..={ARTIFACT_CAS_SWEEP_PAGE_MAX}")));
        }
        let mut head = self.graph.execute(query("MATCH (h:ArtifactCasLedgerHead {id: 'singleton'}) RETURN h.generation AS generation")).await.map_err(backend)?;
        let current = match head.next().await.map_err(backend)? {
            Some(row) => row.get::<i64>("generation").map_err(backend)?,
            None => 0,
        };
        let after = i64::try_from(after_generation).map_err(backend)?;
        let through = i64::try_from(through_generation).map_err(backend)?;
        if through > current || after > through {
            return Err(DirectoryError::Conflict("artifact CAS sweep bounds are outside the ledger".into()));
        }
        let mut result = self
            .graph
            .execute(
                query("MATCH (e:ArtifactCasLedgerEvent) WHERE e.generation > $after AND e.generation <= $through RETURN e.generation AS generation, e.plan AS plan ORDER BY e.generation LIMIT $limit")
                    .param("after", after)
                    .param("through", through)
                    .param("limit", i64::try_from(limit).map_err(backend)?),
            )
            .await
            .map_err(backend)?;
        let mut next = after;
        let mut objects = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            next = row.get("generation").map_err(backend)?;
            if let Ok(plan) = row.get::<neo4rs::BoltBytes>("plan") {
                objects.extend(decode_artifact_cas_ownership_v1(&plan.value).map_err(|error| DirectoryError::Backend(error.to_string()))?.objects);
            }
        }
        objects.sort_by_key(|object| (object.space_id.clone(), object.kind, object.digest.0));
        objects.dedup();
        Ok(ArtifactCasSweepCandidatePage { observed_generation: through_generation, next_generation: u64::try_from(next).map_err(backend)?, objects })
    }

    async fn artifact_cas_delete_preview_protected(&self, key: &ArtifactCasObjectKey, observed_generation: u64, now_ms: u64) -> DirectoryResult<bool> {
        let token = cas_object_token(key);
        let mut result = self.graph.execute(
            query("MATCH (h:ArtifactCasLedgerHead {id: 'singleton'}) RETURN h.generation >= $observed AS observed, EXISTS { MATCH (r:ArtifactCasReference {spaceId: $space_id}) WHERE $token IN r.objects } AS referenced, EXISTS { MATCH (r:ArtifactCasReservation {spaceId: $space_id}) WHERE r.expiresAtMs > $now AND $token IN r.objects } AS reserved")
                .param("observed", i64::try_from(observed_generation).map_err(backend)?)
                .param("space_id", key.space_id.clone())
                .param("token", token)
                .param("now", i64::try_from(now_ms).map_err(backend)?),
        ).await.map_err(backend)?;
        let row = result.next().await.map_err(backend)?.ok_or_else(|| DirectoryError::Conflict("artifact CAS sweep requires an initialized ledger".into()))?;
        if !row.get::<bool>("observed").map_err(backend)? {
            return Err(DirectoryError::Conflict("artifact CAS sweep observation is ahead of the ledger".into()));
        }
        Ok(row.get::<bool>("referenced").map_err(backend)? || row.get::<bool>("reserved").map_err(backend)?)
    }

    async fn acquire_artifact_cas_delete_fence(&self, key: &ArtifactCasObjectKey, observed_generation: u64, lease_token: [u8; 32], now_ms: u64, expires_at_ms: u64) -> DirectoryResult<Option<ArtifactCasDeleteFence>> {
        if observed_generation == 0 {
            return Err(DirectoryError::Conflict("artifact CAS sweep requires a nonzero observed generation".into()));
        }
        if lease_token == [0; 32] || expires_at_ms <= now_ms {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is invalid".into()));
        }
        let token = cas_object_token(key);
        let now = i64::try_from(now_ms).map_err(backend)?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        if cas_lock_space(&mut txn, &key.space_id).await? > now {
            txn.commit().await.map_err(backend)?;
            return Ok(None);
        }
        let mut epoch_result = txn
            .execute(
                query("MATCH (b:ArtifactCasSpaceBarrier {spaceId: $space_id}) SET b.fenceEpoch = coalesce(b.fenceEpoch, 0) + 1, b.leaseToken = $lease_token, b.leaseExpiresAtMs = $expires_at RETURN b.fenceEpoch AS epoch")
                    .param("space_id", key.space_id.clone())
                    .param("lease_token", lease_token.to_vec())
                    .param("expires_at", i64::try_from(expires_at_ms).map_err(backend)?),
            )
            .await
            .map_err(backend)?;
        let physical_epoch: i64 = epoch_result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS barrier epoch returned no row".into()))?.get("epoch").map_err(backend)?;
        drop(epoch_result);
        let mut result = txn.execute(query("MATCH (h:ArtifactCasLedgerHead {id: 'singleton'}) RETURN h.generation AS generation, EXISTS { MATCH (r:ArtifactCasReference {spaceId: $space_id}) WHERE $token IN r.objects } AS referenced, EXISTS { MATCH (r:ArtifactCasReservation {spaceId: $space_id}) WHERE r.expiresAtMs > $now AND $token IN r.objects } AS reserved").param("space_id", key.space_id.clone()).param("token", token).param("now", now)).await.map_err(backend)?;
        let row = result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Conflict("artifact CAS sweep requires an initialized ledger".into()))?;
        let generation: i64 = row.get("generation").map_err(backend)?;
        if generation < i64::try_from(observed_generation).map_err(backend)? {
            return Err(DirectoryError::Conflict("artifact CAS sweep observation is ahead of the ledger".into()));
        }
        let referenced: bool = row.get("referenced").map_err(backend)?;
        let reserved: bool = row.get("reserved").map_err(backend)?;
        drop(result);
        if referenced || reserved {
            txn.run(query("MATCH (b:ArtifactCasSpaceBarrier {spaceId: $space_id}) REMOVE b.leaseToken, b.leaseExpiresAtMs").param("space_id", key.space_id.clone())).await.map_err(backend)?;
            txn.commit().await.map_err(backend)?;
            return Ok(None);
        }
        let mut identity = txn.execute(query("MATCH (b:ArtifactCasBarrierIdentity {id: 'singleton'}) RETURN b.coordinatorId AS coordinator")).await.map_err(backend)?;
        let identity_row = identity.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS barrier coordinator identity is missing".into()))?;
        let coordinator: neo4rs::BoltBytes = identity_row.get("coordinator").map_err(backend)?;
        let coordinator_id = coordinator.value.to_vec().try_into().map_err(|_| DirectoryError::Backend("artifact CAS barrier coordinator identity is invalid".into()))?;
        drop(identity);
        txn.commit().await.map_err(backend)?;
        Ok(Some(ArtifactCasDeleteFence::new(key.clone(), observed_generation, coordinator_id, u64::try_from(physical_epoch).map_err(backend)?, lease_token)))
    }

    async fn validate_artifact_cas_delete_fence(&self, fence: &ArtifactCasDeleteFence, now_ms: u64) -> DirectoryResult<bool> {
        let token = cas_object_token(fence.object());
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        cas_lock_space(&mut txn, &fence.object().space_id).await?;
        let mut result = txn
            .execute(
                query("MATCH (b:ArtifactCasSpaceBarrier {spaceId: $space_id}), (i:ArtifactCasBarrierIdentity {id: 'singleton'}), (h:ArtifactCasLedgerHead {id: 'singleton'}) RETURN b.fenceEpoch = $epoch AND b.leaseToken = $lease_token AND b.leaseExpiresAtMs > $now AND i.coordinatorId = $coordinator AND h.generation >= $observed AS leaseValid, EXISTS { MATCH (r:ArtifactCasReference {spaceId: $space_id}) WHERE $token IN r.objects } AS referenced, EXISTS { MATCH (r:ArtifactCasReservation {spaceId: $space_id}) WHERE r.expiresAtMs > $now AND $token IN r.objects } AS reserved")
                    .param("space_id", fence.object().space_id.clone())
                    .param("epoch", i64::try_from(fence.physical_epoch()).map_err(backend)?)
                    .param("lease_token", fence.lease_token().to_vec())
                    .param("now", i64::try_from(now_ms).map_err(backend)?)
                    .param("coordinator", fence.coordinator_id().to_vec())
                    .param("observed", i64::try_from(fence.ledger_generation()).map_err(backend)?)
                    .param("token", token),
            )
            .await
            .map_err(backend)?;
        let Some(row) = result.next(txn.handle()).await.map_err(backend)? else {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is no longer owned".into()));
        };
        let lease_valid: bool = row.get("leaseValid").map_err(backend)?;
        let referenced: bool = row.get("referenced").map_err(backend)?;
        let reserved: bool = row.get("reserved").map_err(backend)?;
        drop(result);
        txn.commit().await.map_err(backend)?;
        Ok(lease_valid && !referenced && !reserved)
    }

    async fn renew_artifact_cas_delete_fence(&self, fence: &ArtifactCasDeleteFence, now_ms: u64, expires_at_ms: u64) -> DirectoryResult<()> {
        if expires_at_ms <= now_ms {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease renewal is invalid".into()));
        }
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        cas_lock_space(&mut txn, &fence.object().space_id).await?;
        let mut result = txn
            .execute(
                query("MATCH (b:ArtifactCasSpaceBarrier {spaceId: $space_id}) WHERE b.leaseToken = $lease_token AND b.leaseExpiresAtMs > $now SET b.leaseExpiresAtMs = $expires_at RETURN count(b) AS renewed")
                    .param("space_id", fence.object().space_id.clone())
                    .param("lease_token", fence.lease_token().to_vec())
                    .param("now", i64::try_from(now_ms).map_err(backend)?)
                    .param("expires_at", i64::try_from(expires_at_ms).map_err(backend)?),
            )
            .await
            .map_err(backend)?;
        let renewed: i64 = result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS deletion lease renewal returned no row".into()))?.get("renewed").map_err(backend)?;
        drop(result);
        if renewed != 1 {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is no longer owned".into()));
        }
        txn.commit().await.map_err(backend)?;
        Ok(())
    }

    async fn release_artifact_cas_delete_fence(&self, fence: ArtifactCasDeleteFence) -> DirectoryResult<()> {
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        cas_lock_space(&mut txn, &fence.object().space_id).await?;
        let mut result = txn
            .execute(
                query("MATCH (b:ArtifactCasSpaceBarrier {spaceId: $space_id}) WHERE b.leaseToken = $lease_token REMOVE b.leaseToken, b.leaseExpiresAtMs RETURN count(b) AS released")
                    .param("space_id", fence.object().space_id.clone())
                    .param("lease_token", fence.lease_token().to_vec()),
            )
            .await
            .map_err(backend)?;
        let released: i64 = result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS deletion lease release returned no row".into()))?.get("released").map_err(backend)?;
        drop(result);
        if released != 1 {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is no longer owned".into()));
        }
        txn.commit().await.map_err(backend)?;
        Ok(())
    }

    //#region EventLog
    /// @emoji ➕️ Assigns a dense `seq` via a `(:DirectoryCounter {id:'singleton'})` node
    /// incremented in the same transaction as the `(:DirectoryEvent)` node and the projection —
    /// the write's atomicity comes from `Txn`, not from any Neo4j auto-increment primitive (Neo4j
    /// has none).
    async fn append_decided_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<DirectoryAppendOutcomeV1> {
        if events.iter().any(|event| matches!(&event.body, DirectoryEventBody::ArtifactCheckpointPublished { .. } | DirectoryEventBody::InviteRedeemed { .. })) {
            return Err(DirectoryError::Conflict("event requires its verified authority append seam".into()));
        }
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut persisted = Vec::with_capacity(events.len());
        for event in events {
            let id = time_ordered_id();
            let recorded_at_ms = now_ms();
            let payload_value = serde_json::Value::from(&event.body.to_value());
            let kind = payload_value.get("kind").and_then(|value| value.as_str()).unwrap_or_default().to_string();
            let mut counter = txn.execute(query("MERGE (c:DirectoryCounter {id: 'singleton'}) ON CREATE SET c.seq = 0 SET c.seq = c.seq + 1 RETURN c.seq AS seq")).await.map_err(backend)?;
            let seq: i64 = counter.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("directory counter query returned no row".into()))?.get("seq").map_err(backend)?;
            drop(counter);
            let seq = u64::try_from(seq).map_err(backend)?;
            if let Some(reason) = self.projection_rejection(&mut txn, &event.body).await? {
                txn.rollback().await.map_err(backend)?;
                return Ok(DirectoryAppendOutcomeV1::RejectedBeforeCommit(reason));
            }
            if seq > DIRECTORY_WIRE_INTEGER_MAX {
                return Err(DirectoryError::Conflict("directory event sequence exceeds the public integer boundary".into()));
            }
            let full = DirectoryEvent { seq, id: id.clone(), hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone(), recorded_at_ms };
            validate_directory_event_page_event(&full).map_err(|_| DirectoryError::Conflict("directory event violates the bounded event-page contract".into()))?;
            txn.run(
                query(
                    "CREATE (e:DirectoryEvent {seq: $seq, id: $id, hlcPhysical: $hlc_physical, hlcLogical: $hlc_logical, actorKind: $actor_kind, actorId: $actor_id,
                                                spaceId: $space_id, userId: $user_id, kind: $kind, payload: $payload, recordedAt: $recorded_at})",
                )
                .param("seq", i64::try_from(seq).map_err(backend)?)
                .param("id", id.clone())
                .param("hlc_physical", event.hlc.physical_ms)
                .param("hlc_logical", event.hlc.logical as i64)
                .param("actor_kind", actor_kind_to_str(event.actor.kind))
                .param("actor_id", event.actor.id.clone())
                .param("space_id", event.space_id.clone())
                .param("user_id", event.user_id.clone())
                .param("kind", kind)
                .param("payload", payload_value.to_string())
                .param("recorded_at", recorded_at_ms),
            )
            .await
            .map_err(backend)?;
            self.project(&mut txn, &full).await?;
            match &full.body {
                DirectoryEventBody::ArtifactRetentionAdvanced { retention } => {
                    let generation = cas_generation(&mut txn).await?;
                    txn.run(
                        query("CREATE (:ArtifactCasLedgerEvent {generation: $generation, operation: 'retention', spaceId: $space_id, documentId: $document_id, checkpointId: $checkpoint_id, eventSeq: $event_seq})")
                            .param("generation", generation)
                            .param("space_id", retention.scope.space_id.clone())
                            .param("document_id", retention.scope.document_id.clone())
                            .param("checkpoint_id", hex_lower(&retention.retained_checkpoint_id.0))
                            .param("event_seq", i64::try_from(seq).map_err(backend)?),
                    )
                    .await
                    .map_err(backend)?;
                    cas_project_release(&mut txn, "retention", &retention.scope.space_id, Some(&retention.scope), Some(retention.retained_checkpoint_id)).await?;
                }
                DirectoryEventBody::SpaceDeleted { space_id } => {
                    let generation = cas_generation(&mut txn).await?;
                    txn.run(
                        query("CREATE (:ArtifactCasLedgerEvent {generation: $generation, operation: 'space-delete', spaceId: $space_id, eventSeq: $event_seq})")
                            .param("generation", generation)
                            .param("space_id", space_id.clone())
                            .param("event_seq", i64::try_from(seq).map_err(backend)?),
                    )
                    .await
                    .map_err(backend)?;
                    cas_project_release(&mut txn, "space-delete", space_id, None, None).await?;
                }
                _ => {}
            }
            persisted.push(full);
        }
        txn.commit().await.map_err(backend)?;
        Ok(DirectoryAppendOutcomeV1::Appended(persisted))
    }

    async fn append_decided_events_with_admin_effect(&self, events: &[NewDirectoryEvent], effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<Vec<DirectoryEvent>> {
        if events.is_empty() || events.iter().any(|event| matches!(&event.body, DirectoryEventBody::ArtifactCheckpointPublished { .. } | DirectoryEventBody::InviteRedeemed { .. })) {
            return AdminEffectCommitV1::RejectedBeforeCommit;
        }
        let mut txn = match self.graph.start_txn().await {
            Ok(txn) => txn,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let persisted_result: DirectoryResult<Vec<DirectoryEvent>> = async {
            let mut persisted = Vec::with_capacity(events.len());
            for event in events {
            let id = time_ordered_id();
            let recorded_at_ms = now_ms();
            let payload_value = serde_json::Value::from(&event.body.to_value());
            let kind = payload_value.get("kind").and_then(|value| value.as_str()).unwrap_or_default().to_string();
            let mut counter = txn.execute(query("MERGE (c:DirectoryCounter {id: 'singleton'}) ON CREATE SET c.seq = 0 SET c.seq = c.seq + 1 RETURN c.seq AS seq")).await.map_err(backend)?;
            let seq: i64 = counter.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("directory counter query returned no row".into()))?.get("seq").map_err(backend)?;
            drop(counter);
            let seq = u64::try_from(seq).map_err(backend)?;
            if self.projection_rejection(&mut txn, &event.body).await?.is_some() {
                return Err(DirectoryError::Conflict("administrator event projection was rejected".into()));
            }
            if seq > DIRECTORY_WIRE_INTEGER_MAX {
                return Err(DirectoryError::Conflict("directory event sequence exceeds the public integer boundary".into()));
            }
            let full = DirectoryEvent { seq, id: id.clone(), hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone(), recorded_at_ms };
            validate_directory_event_page_event(&full).map_err(|_| DirectoryError::Conflict("directory event violates the bounded event-page contract".into()))?;
            txn.run(
                query("CREATE (e:DirectoryEvent {seq: $seq, id: $id, hlcPhysical: $hlc_physical, hlcLogical: $hlc_logical, actorKind: $actor_kind, actorId: $actor_id, spaceId: $space_id, userId: $user_id, kind: $kind, payload: $payload, recordedAt: $recorded_at})")
                    .param("seq", i64::try_from(seq).map_err(backend)?)
                    .param("id", id)
                    .param("hlc_physical", event.hlc.physical_ms)
                    .param("hlc_logical", i64::from(event.hlc.logical))
                    .param("actor_kind", actor_kind_to_str(event.actor.kind))
                    .param("actor_id", event.actor.id.clone())
                    .param("space_id", event.space_id.clone())
                    .param("user_id", event.user_id.clone())
                    .param("kind", kind)
                    .param("payload", payload_value.to_string())
                    .param("recorded_at", recorded_at_ms),
            )
            .await
            .map_err(backend)?;
            self.project(&mut txn, &full).await?;
            match &full.body {
                DirectoryEventBody::ArtifactRetentionAdvanced { retention } => {
                    let generation = cas_generation(&mut txn).await?;
                    txn.run(
                        query("CREATE (:ArtifactCasLedgerEvent {generation: $generation, operation: 'retention', spaceId: $space_id, documentId: $document_id, checkpointId: $checkpoint_id, eventSeq: $event_seq})")
                            .param("generation", generation)
                            .param("space_id", retention.scope.space_id.clone())
                            .param("document_id", retention.scope.document_id.clone())
                            .param("checkpoint_id", hex_lower(&retention.retained_checkpoint_id.0))
                            .param("event_seq", i64::try_from(seq).map_err(backend)?),
                    )
                    .await
                    .map_err(backend)?;
                    cas_project_release(&mut txn, "retention", &retention.scope.space_id, Some(&retention.scope), Some(retention.retained_checkpoint_id)).await?;
                }
                DirectoryEventBody::SpaceDeleted { space_id } => {
                    let generation = cas_generation(&mut txn).await?;
                    txn.run(
                        query("CREATE (:ArtifactCasLedgerEvent {generation: $generation, operation: 'space-delete', spaceId: $space_id, eventSeq: $event_seq})")
                            .param("generation", generation)
                            .param("space_id", space_id.clone())
                            .param("event_seq", i64::try_from(seq).map_err(backend)?),
                    )
                    .await
                    .map_err(backend)?;
                    cas_project_release(&mut txn, "space-delete", space_id, None, None).await?;
                }
                _ => {}
            }
                persisted.push(full);
            }
            insert_admin_operation_effect_receipt(&mut txn, &admin_operation_effect_receipt_v1(effect, &persisted)?).await?;
            Ok(persisted)
        }
        .await;
        let persisted = match persisted_result {
            Ok(persisted) => persisted,
            Err(_) => return admin_effect_rollback(txn).await,
        };
        match txn.commit().await {
            Ok(()) => AdminEffectCommitV1::Applied(persisted),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }

    async fn events_since(&self, since_seq: u64, limit: usize) -> DirectoryResult<Vec<DirectoryEvent>> {
        let (since_seq, limit) = bounded_event_read(since_seq, limit)?;
        let mut result = self.graph.execute(query("MATCH (e:DirectoryEvent) WHERE e.seq > $since_seq RETURN e AS e ORDER BY e.seq LIMIT $limit").param("since_seq", since_seq).param("limit", limit)).await.map_err(backend)?;
        let mut events = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            events.push(event_from_node(&row)?);
        }
        Ok(events)
    }

    async fn head_seq(&self) -> DirectoryResult<u64> {
        let mut result = self.graph.execute(query("MATCH (c:DirectoryCounter {id: 'singleton'}) RETURN c.seq AS seq")).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => u64::try_from(row.get::<i64>("seq").map_err(backend)?).map_err(backend),
            None => Ok(0),
        }
    }

    async fn rebuild_projections(&self) -> DirectoryResult<u64> {
        self.rebuild_projections_controlled(&UNCONTROLLED_PROJECTION_REBUILD).await
    }

    async fn rebuild_projections_controlled(&self, control: &dyn ProjectionRebuildControl) -> DirectoryResult<u64> {
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut writer = txn.execute(query("MERGE (c:DirectoryCounter {id: 'singleton'}) ON CREATE SET c.seq = 0 SET c.claimNonce = coalesce(c.claimNonce, 0) + 1 RETURN c.seq AS seq")).await.map_err(backend)?;
        writer.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("directory rebuild writer lock returned no row".into()))?;
        drop(writer);
        let mut count_result = txn.execute(query("MATCH (e:DirectoryEvent) RETURN count(e) AS count")).await.map_err(backend)?;
        let count: i64 = count_result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("directory event count returned no row".into()))?.get("count").map_err(backend)?;
        drop(count_result);
        let total = u64::try_from(count).map_err(backend)?;
        checkpoint_projection_rebuild(control, 0, total)?;
        txn.run(query("MATCH (r:ArtifactCasReservation) DETACH DELETE r")).await.map_err(backend)?;
        txn.run(query("MATCH (r:ArtifactCasReference) DETACH DELETE r")).await.map_err(backend)?;
        txn.run(query("MATCH (r:ArtifactRetention) DETACH DELETE r")).await.map_err(backend)?;
        txn.run(query("MATCH (p:ArtifactCheckpointPrivate) DETACH DELETE p")).await.map_err(backend)?;
        txn.run(query("MATCH (c:ArtifactCheckpoint) DETACH DELETE c")).await.map_err(backend)?;
        txn.run(query("MATCH (i:DocumentIndex) DETACH DELETE i")).await.map_err(backend)?;
        txn.run(query("MATCH (d:DocumentDescriptor) DETACH DELETE d")).await.map_err(backend)?;
        txn.run(query("MATCH (s:Space) DETACH DELETE s")).await.map_err(backend)?;
        txn.run(query("MATCH (u:User) DETACH DELETE u")).await.map_err(backend)?;
        let mut replayed = 0u64;
        let mut cursor = 0i64;
        while replayed < total {
            let mut result = txn.execute(query("MATCH (e:DirectoryEvent) WHERE e.seq > $cursor RETURN e AS e ORDER BY e.seq LIMIT 512").param("cursor", cursor)).await.map_err(backend)?;
            let mut events = Vec::new();
            while let Some(row) = result.next(txn.handle()).await.map_err(backend)? {
                events.push(event_from_node(&row)?);
            }
            drop(result);
            if events.is_empty() {
                return Err(DirectoryError::Backend("directory event replay ended before its counted head".into()));
            }
            for event in &events {
                cursor = i64::try_from(event.seq).map_err(backend)?;
                if let Err(error) = self.project(&mut txn, event).await {
                    txn.rollback().await.map_err(backend)?;
                    return Err(DirectoryError::Backend(format!("Neo4j projection rebuild event {}: {error}", event.seq)));
                }
                if matches!(&event.body, DirectoryEventBody::ArtifactCheckpointPublished { .. }) {
                    let mut private = txn.execute(query("MATCH (a:ArtifactAuthorityEvent {eventSeq: $event_seq}) RETURN a.payload AS payload").param("event_seq", cursor)).await.map_err(backend)?;
                    let row = private.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend(format!("missing private authority journal for checkpoint event {}", event.seq)))?;
                    let payload: String = row.get("payload").map_err(backend)?;
                    drop(private);
                    let checkpoint: ArtifactCheckpoint = directory::os_pack::json::from_json_str(&payload).map_err(backend)?;
                    self.project_verified_checkpoint(&mut txn, event, &checkpoint).await?;
                }
                replayed += 1;
                checkpoint_projection_rebuild(control, replayed, total)?;
            }
        }
        let mut ledger_cursor = 0i64;
        loop {
            let mut result = txn.execute(query("MATCH (e:ArtifactCasLedgerEvent) WHERE e.generation > $cursor RETURN e AS e ORDER BY e.generation LIMIT 512").param("cursor", ledger_cursor)).await.map_err(backend)?;
            let mut entries = Vec::new();
            while let Some(row) = result.next(txn.handle()).await.map_err(backend)? {
                entries.push(row.get::<neo4rs::Node>("e").map_err(backend)?);
            }
            drop(result);
            if entries.is_empty() {
                break;
            }
            for entry in entries {
                ledger_cursor = entry.get("generation").map_err(backend)?;
                let operation: String = entry.get("operation").map_err(backend)?;
                match operation.as_str() {
                    "reserve" | "publish" => {
                        let encoded: neo4rs::BoltBytes = entry.get("plan").map_err(backend)?;
                        let reservation = ArtifactCasReservation::unfenced(
                            decode_artifact_cas_ownership_v1(&encoded.value).map_err(|error| DirectoryError::Backend(error.to_string()))?,
                            u64::try_from(ledger_cursor).map_err(backend)?,
                            u64::try_from(entry.get::<i64>("writeEpoch").map_err(backend)?).map_err(backend)?,
                            u64::try_from(entry.get::<i64>("expiresAtMs").map_err(backend)?).map_err(backend)?,
                        );
                        if operation == "reserve" {
                            cas_project_reserve(&mut txn, &reservation).await?;
                        } else {
                            cas_project_publish(&mut txn, &reservation, ledger_cursor).await?;
                        }
                    }
                    "retention" => {
                        let scope = DocumentScope::new(entry.get::<String>("spaceId").map_err(backend)?, entry.get::<String>("documentId").map_err(backend)?);
                        let checkpoint_id = ArtifactHash(decode_auth_digest_hex(&entry.get::<String>("checkpointId").map_err(backend)?)?);
                        cas_project_release(&mut txn, "retention", &scope.space_id, Some(&scope), Some(checkpoint_id)).await?;
                    }
                    "space-delete" => {
                        let space_id: String = entry.get("spaceId").map_err(backend)?;
                        cas_project_release(&mut txn, "space-delete", &space_id, None, None).await?;
                    }
                    _ => return Err(DirectoryError::Backend("artifact CAS ledger operation is invalid".into())),
                }
            }
        }
        txn.commit().await.map_err(backend)?;
        Ok(replayed)
    }
    //#endregion
}

fn user_from_node(row: &neo4rs::Row) -> DirectoryResult<UserRecord> {
    let node: neo4rs::Node = row.get("u").map_err(backend)?;
    Ok(UserRecord {
        id: node.get("id").map_err(backend)?,
        email: node.get("email").map_err(backend)?,
        display_name: node.get("displayName").map_err(backend)?,
        password_hash: node.get::<String>("passwordHash").ok().filter(|s| !s.is_empty()),
        sso_subject: node.get::<String>("ssoSubject").ok().filter(|s| !s.is_empty()),
        sso_provider: node.get::<String>("ssoProvider").ok().filter(|s| !s.is_empty()),
        created_at: node.get("createdAt").map_err(backend)?,
    })
}

fn space_from_node(row: &neo4rs::Row) -> DirectoryResult<SpaceRecord> {
    let node: neo4rs::Node = row.get("s").map_err(backend)?;
    Ok(SpaceRecord {
        id: node.get("id").map_err(backend)?,
        name: node.get("name").map_err(backend)?,
        owner_user_id: node.get("ownerUserId").map_err(backend)?,
        created_at: node.get("createdAt").map_err(backend)?,
        kind: node.get("kind").map_err(backend)?,
        visibility: node.get("visibility").map_err(backend)?,
    })
}

fn invite_from_node(row: &neo4rs::Row) -> DirectoryResult<InviteRecord> {
    let node: neo4rs::Node = row.get("i").map_err(backend)?;
    let role: String = node.get("role").map_err(backend)?;
    Ok(InviteRecord {
        id: node.get("id").map_err(backend)?,
        selector: node.get("selector").map_err(backend)?,
        secret_digest: decode_auth_digest_hex(&node.get::<String>("secretDigest").map_err(backend)?)?,
        space_id: node.get("spaceId").map_err(backend)?,
        role: SpaceRole::parse(&role).unwrap_or(SpaceRole::Spectator),
        created_at: node.get("createdAt").map_err(backend)?,
        expires_at: node.get("expiresAt").map_err(backend)?,
        revoked_at: node.get::<i64>("revokedAt").ok(),
        revoked_reason: node.get::<String>("revokedReason").ok().filter(|value| !value.is_empty()),
        accepted_at: node.get::<i64>("acceptedAt").ok(),
        accepted_event_id: node.get::<String>("acceptedEventId").ok().filter(|value| !value.is_empty()),
    })
}

fn share_from_node(row: &neo4rs::Row) -> DirectoryResult<ShareTokenRecord> {
    let node: neo4rs::Node = row.get("g").map_err(backend)?;
    Ok(ShareTokenRecord {
        id: node.get("id").map_err(backend)?,
        selector: node.get("selector").map_err(backend)?,
        secret_digest: decode_auth_digest_hex(&node.get::<String>("secretDigest").map_err(backend)?)?,
        scope: DocumentScope::new(node.get::<String>("spaceId").map_err(backend)?, node.get::<String>("documentId").map_err(backend)?),
        created_at: node.get("createdAt").map_err(backend)?,
        expires_at: node.get("expiresAt").map_err(backend)?,
        revoked_at: node.get::<i64>("revokedAt").ok(),
        revoked_reason: node.get::<String>("revokedReason").ok().filter(|value| !value.is_empty()),
    })
}

fn auth_session_from_node(row: &neo4rs::Row) -> DirectoryResult<AuthSessionRecord> {
    let node: neo4rs::Node = row.get("a").map_err(backend)?;
    let session_kind: String = node.get("sessionKind").map_err(backend)?;
    Ok(AuthSessionRecord {
        id: node.get("id").map_err(backend)?,
        selector: node.get("selector").map_err(backend)?,
        secret_digest: decode_auth_digest_hex(&node.get::<String>("secretDigest").map_err(backend)?)?,
        user_id: row.get("userId").map_err(backend)?,
        identity_provider: node.get("identityProvider").map_err(backend)?,
        identity_subject_digest: decode_auth_digest_hex(&node.get::<String>("identitySubjectDigest").map_err(backend)?)?,
        issued_at: node.get("issuedAt").map_err(backend)?,
        expires_at: node.get("expiresAt").map_err(backend)?,
        revoked_at: node.get::<i64>("revokedAt").ok(),
        revoked_reason: node.get::<String>("revokedReason").ok().filter(|value| !value.is_empty()),
        authorization_generation: u64::try_from(node.get::<i64>("authorizationGeneration").map_err(backend)?).map_err(backend)?,
        device_instance_id: node.get("deviceInstanceId").map_err(backend)?,
        session_kind: AuthSessionKind::parse(&session_kind).ok_or_else(|| DirectoryError::Backend("stored session kind is invalid".into()))?,
    })
}

fn auth_audit_from_node(row: &neo4rs::Row) -> DirectoryResult<AuthAuditRecord> {
    let node: neo4rs::Node = row.get("a").map_err(backend)?;
    let optional = |name| node.get::<String>(name).ok().filter(|value| !value.is_empty());
    Ok(AuthAuditRecord {
        id: node.get("id").map_err(backend)?,
        occurred_at: node.get("occurredAt").map_err(backend)?,
        event_kind: node.get("eventKind").map_err(backend)?,
        auth_session_id: optional("authSessionId"),
        target_user_id: optional("targetUserId"),
        actor_user_id: optional("actorUserId"),
        provider: optional("provider"),
        outcome_code: node.get("outcomeCode").map_err(backend)?,
        reason_code: optional("reasonCode"),
        correlation_id: node.get("correlationId").map_err(backend)?,
        peer_class: node.get("peerClass").map_err(backend)?,
    })
}

/// 🔑️ The one durable `(actor, request id)` idempotency key this backend indexes receipts by.
fn directory_command_receipt_key(actor_user_id: &str, request_id: &str) -> String {
    format!("{}:{}", hex_lower(actor_user_id.as_bytes()), request_id)
}

fn checkpoint_publication_receipt_key(actor_user_id: &str, correlation_id: &str) -> String {
    format!("{}:{}", hex_lower(actor_user_id.as_bytes()), correlation_id)
}

fn checkpoint_publication_receipt_from_node(row: &neo4rs::Row) -> DirectoryResult<CheckpointPublicationReceiptRecordV1> {
    let node: neo4rs::Node = row.get("r").map_err(backend)?;
    let disposition = match node.get::<String>("disposition").map_err(backend)?.as_str() {
        "pending" => CheckpointPublicationDispositionV1::Pending,
        "completed" => CheckpointPublicationDispositionV1::Completed,
        other => return Err(DirectoryError::Backend(format!("unknown checkpoint publication disposition '{other}'"))),
    };
    let checkpoint_id = node.get::<String>("checkpointId").ok().filter(|value| !value.is_empty()).map(|value| crate::directory::decode_auth_digest_hex(&value).map(ArtifactHash)).transpose()?;
    Ok(CheckpointPublicationReceiptRecordV1 {
        actor_user_id: node.get("actorUserId").map_err(backend)?,
        correlation_id: node.get("correlationId").map_err(backend)?,
        command_sha256: node.get("commandSha256").map_err(backend)?,
        disposition,
        checkpoint_id,
        claimed_at: node.get("claimedAt").map_err(backend)?,
        completed_at: node.get::<i64>("completedAt").ok().filter(|value| *value > 0),
    })
}

fn directory_command_receipt_from_node(row: &neo4rs::Row) -> DirectoryResult<DirectoryCommandReceiptRecord> {
    let node: neo4rs::Node = row.get("r").map_err(backend)?;
    let disposition_text: String = node.get("disposition").map_err(backend)?;
    let disposition = match disposition_text.as_str() {
        "pending" => DirectoryCommandDispositionV1::Pending,
        "completed" => DirectoryCommandDispositionV1::Completed,
        other => return Err(DirectoryError::Backend(format!("unknown command disposition '{other}'"))),
    };
    let result_kind_text: String = node.get("resultKind").map_err(backend)?;
    let completed_at = node.get::<i64>("completedAt").ok().filter(|value| *value > 0);
    Ok(DirectoryCommandReceiptRecord {
        actor_user_id: node.get("actorUserId").map_err(backend)?,
        request_id: node.get("requestId").map_err(backend)?,
        command_sha256: node.get("commandSha256").map_err(backend)?,
        result_kind: directory_command_result_kind_from_str(&result_kind_text)?,
        disposition,
        event_seq_first: node.get::<i64>("eventSeqFirst").ok().filter(|value| *value > 0).map(u64::try_from).transpose().map_err(backend)?,
        event_seq_last: node.get::<i64>("eventSeqLast").ok().filter(|value| *value > 0).map(u64::try_from).transpose().map_err(backend)?,
        receipt_sha256: node.get::<String>("receiptSha256").ok().filter(|value| !value.is_empty()),
        claimed_at: node.get("claimedAt").map_err(backend)?,
        completed_at,
    })
}

fn admin_operation_audit_from_node(row: &neo4rs::Row) -> DirectoryResult<AdminOperationAuditRecord> {
    let node: neo4rs::Node = row.get("a").map_err(backend)?;
    let event_seq_first = node.get::<i64>("eventSeqFirst").ok().filter(|value| *value > 0).map(u64::try_from).transpose().map_err(backend)?;
    let event_seq_last = node.get::<i64>("eventSeqLast").ok().filter(|value| *value > 0).map(u64::try_from).transpose().map_err(backend)?;
    Ok(AdminOperationAuditRecord {
        sequence: u64::try_from(node.get::<i64>("sequence").map_err(backend)?).map_err(backend)?,
        fact: NewAdminOperationAuditRecord {
            request_id: node.get("requestId").map_err(backend)?,
            intent_digest: node.get("intentDigest").map_err(backend)?,
            operation_id: node.get("operationId").map_err(backend)?,
            occurred_at: node.get("occurredAt").map_err(backend)?,
            phase: node.get("phase").map_err(backend)?,
            intent_kind: node.get("intentKind").map_err(backend)?,
            target_kind: node.get("targetKind").map_err(backend)?,
            target_id: node.get("targetId").map_err(backend)?,
            principal_user_id: node.get("principalUserId").map_err(backend)?,
            principal_session_id: node.get("principalSessionId").map_err(backend)?,
            principal_generation: u64::try_from(node.get::<i64>("principalGeneration").map_err(backend)?).map_err(backend)?,
            correlation_id: node.get("correlationId").map_err(backend)?,
            event_seq_first,
            event_seq_last,
            outcome_code: node.get("outcomeCode").map_err(backend)?,
            reason_code: node.get::<String>("reasonCode").ok().filter(|value| !value.is_empty()),
        },
    })
}

/// 🧭️ Shared by `list_sync_sessions_for_document` (caller already knows `document_id`) and
/// `list_active_sync_sessions` (reads `documentId` off the row itself, since it spans documents).
fn sync_session_from_row(row: &neo4rs::Row, document_id: &str) -> DirectoryResult<SyncSessionRecord> {
    let role: String = row.get("role").unwrap_or_default();
    Ok(SyncSessionRecord {
        id: row.get("id").map_err(backend)?,
        auth_session_id: row.get::<String>("authSessionId").ok().filter(|value| !value.is_empty()),
        authorization_generation: u64::try_from(row.get::<i64>("generation").unwrap_or(0)).unwrap_or(0),
        actor_id: row.get("actorId").unwrap_or_default(),
        space_id: row.get("spaceId").map_err(backend)?,
        document_id: document_id.to_string(),
        surface: row.get("surface").unwrap_or_default(),
        user_id: row.get::<String>("userId").ok(),
        authenticated_email: row.get::<String>("authenticatedEmail").ok().filter(|value| !value.is_empty()),
        space_role: SpaceRole::parse(&role),
        client_label: row.get("clientLabel").map_err(backend)?,
        connected_at: row.get("connectedAt").map_err(backend)?,
        disconnected_at: row.get::<i64>("disconnectedAt").ok(),
    })
}

fn event_from_node(row: &neo4rs::Row) -> DirectoryResult<DirectoryEvent> {
    let node: neo4rs::Node = row.get("e").map_err(backend)?;
    let payload: String = node.get("payload").map_err(backend)?;
    let body = DirectoryEventBody::from_value(directory::DslValue::from(serde_json::from_str::<serde_json::Value>(&payload).map_err(backend)?)).map_err(backend)?;
    let actor_kind: String = node.get("actorKind").map_err(backend)?;
    let seq = u64::try_from(node.get::<i64>("seq").map_err(backend)?).map_err(backend)?;
    let logical = u32::try_from(node.get::<i64>("hlcLogical").map_err(backend)?).map_err(backend)?;
    Ok(DirectoryEvent {
        seq,
        id: node.get("id").map_err(backend)?,
        hlc: Hlc { physical_ms: node.get("hlcPhysical").map_err(backend)?, logical },
        actor: DirectoryActor { kind: actor_kind_from_str(&actor_kind), id: node.get("actorId").map_err(backend)? },
        space_id: node.get::<String>("spaceId").ok(),
        user_id: node.get::<String>("userId").ok(),
        body,
        recorded_at_ms: node.get("recordedAt").map_err(backend)?,
    })
}

#[cfg(test)]
#[path = "🌱️creation-v1/🧪️tests/🔬️standalone/🦀️.rs"]
mod creation_tests;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
