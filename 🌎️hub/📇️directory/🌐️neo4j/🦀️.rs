//! 🌐️ `HubDirectory` over Neo4j (neo4rs). Users/Spaces/Memberships are real nodes+relationships —
//! where graph traversal earns its keep (role lookups, VFS tree walks). Document persistence and
//! blobs are no longer this crate's concern — `db::Database` and `db_storage_neo4j` own that now
//! (see `bin.rs`). `#[cfg(feature = "neo4j")]`-gated as a whole by the parent `directory` module
//! (see `📇️directory/🦀️.rs`'s `//#region 🔖️Backends`).

use crate::directory::error::{DirectoryError, DirectoryResult};
use crate::directory::model::*;
use crate::directory::{
    active_capability, auth_audit, bounded_event_read, checkpoint_projection_rebuild, decode_auth_digest_hex, encode_capability_bytes, kind_to_str, prepare_auth_session, prepare_invite, prepare_share_token, role_from_wire,
    validate_bounded_auth_text, validate_verified_checkpoint_append, visibility_to_str, HubClock, HubDirectory, InviteCapability, NewDirectoryEvent, ProjectionRebuildControl, SessionCapability, ShareCapability,
    ArtifactCasSweepCandidatePage, ARTIFACT_CAS_RESERVATION_MAX_TTL_MS, ARTIFACT_CAS_SWEEP_PAGE_MAX, ARTIFACT_CHECKPOINT_LINEAGE_MAX, AUTH_AUDIT_PAGE_MAX, AUTH_TEXT_MAX_BYTES, DIRECTORY_WIRE_INTEGER_MAX,
    UNCONTROLLED_PROJECTION_REBUILD,
};
use crate::artifact_authority::chunk_cas::{decode_artifact_cas_ownership_v1, encode_artifact_cas_ownership_v1, validate_artifact_cas_publication_v1, ArtifactCasDeleteFence, ArtifactCasObjectKey, ArtifactCasOwnershipPlanV1, ArtifactCasReservation};
use directory::os_directory::{
    hex_lower, ArtifactCheckpoint, ArtifactHash, ArtifactRetention, DirectoryActor, DirectoryActorKind, DirectoryEvent, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, DocumentDescriptor, Hlc,
    PublishedArtifactCheckpoint,
};
use directory::os_identity::time_ordered_id;
use directory::{FromValue, ToValue};
use semio_framework_hash::Sha256;
use neo4rs::{query, Graph, Txn};

fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
}

fn backend<E: std::fmt::Display>(err: E) -> DirectoryError {
    DirectoryError::Backend(err.to_string())
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
    let mut result = txn.execute(query("MERGE (b:ArtifactCasSpaceBarrier {spaceId: $space_id}) ON CREATE SET b.lockNonce = 0 SET b.lockNonce = b.lockNonce + 1 RETURN coalesce(b.leaseExpiresAtMs, 0) AS leaseExpiresAtMs").param("space_id", space_id)).await.map_err(backend)?;
    let expires_at_ms = result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS space barrier returned no row".into()))?.get("leaseExpiresAtMs").map_err(backend)?;
    drop(result);
    Ok(expires_at_ms)
}

async fn cas_reservation_barrier(txn: &mut Txn, space_id: &str, now_ms: i64) -> DirectoryResult<([u8; 32], u64)> {
    let lease_expires_at_ms = cas_lock_space(txn, space_id).await?;
    if lease_expires_at_ms > now_ms { return Err(DirectoryError::Conflict("artifact CAS deletion lease is active for this space".into())); }
    let mut epoch_result = txn.execute(query("MATCH (b:ArtifactCasSpaceBarrier {spaceId: $space_id}) SET b.fenceEpoch = coalesce(b.fenceEpoch, 0) + 1 REMOVE b.leaseToken, b.leaseExpiresAtMs RETURN b.fenceEpoch AS epoch").param("space_id", space_id)).await.map_err(backend)?;
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
    txn.run(query("CREATE (r:ArtifactCasReference {scopeCheckpointKey: $key, spaceId: $space_id, documentId: $document_id, checkpointId: $checkpoint_id, generation: $generation, writeEpoch: $write_epoch, plan: $plan, objects: $objects})")
        .param("key", scope_key).param("space_id", reservation.plan.scope.space_id.clone()).param("document_id", reservation.plan.scope.document_id.clone()).param("checkpoint_id", hex_lower(&reservation.plan.checkpoint_id.0)).param("generation", generation).param("write_epoch", i64::try_from(reservation.write_epoch).map_err(backend)?).param("plan", encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?).param("objects", reservation.plan.objects.iter().map(cas_object_token).collect::<Vec<_>>())).await.map_err(backend)?;
    Ok(())
}

async fn cas_project_release(txn: &mut Txn, operation: &str, space_id: &str, scope: Option<&DocumentScope>, checkpoint_id: Option<ArtifactHash>) -> DirectoryResult<()> {
    match operation {
        "retention" => {
            let scope = scope.ok_or_else(|| DirectoryError::Backend("artifact CAS retention scope missing".into()))?;
            let floor_key = checkpoint_key_v1(scope, checkpoint_id.ok_or_else(|| DirectoryError::Backend("artifact CAS retention checkpoint missing".into()))?);
            txn.run(query("MATCH (floor:ArtifactAuthorityEvent {scopeCheckpointKey: $floor_key}) MATCH (r:ArtifactCasReference {spaceId: $space_id, documentId: $document_id}) MATCH (published:ArtifactAuthorityEvent {scopeCheckpointKey: r.scopeCheckpointKey}) WHERE published.eventSeq < floor.eventSeq DETACH DELETE r")
                .param("floor_key", floor_key.clone()).param("space_id", space_id).param("document_id", scope.document_id.clone())).await.map_err(backend)?;
            txn.run(query("MATCH (floor:ArtifactAuthorityEvent {scopeCheckpointKey: $floor_key}) MATCH (p:ArtifactCheckpointPrivate {spaceId: $space_id, documentId: $document_id}) WHERE p.eventSeq < floor.eventSeq DETACH DELETE p")
                .param("floor_key", floor_key).param("space_id", space_id).param("document_id", scope.document_id.clone())).await.map_err(backend)?;
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
    "CREATE CONSTRAINT IF NOT EXISTS FOR (i:SpaceInvite) REQUIRE i.selector IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (a:AuthAudit) REQUIRE a.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (e:DirectoryEvent) REQUIRE e.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (e:DirectoryEvent) REQUIRE e.seq IS UNIQUE",
];

/// @emoji 🕸️ Neo4j-backed `HubDirectory`.
pub struct Neo4jDirectory {
    graph: Graph,
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
        Ok(Self { graph })
    }

    async fn revoke_auth_sessions_by(
        &self,
        field: &str,
        key: &str,
        subject_digest: Option<[u8; 32]>,
        reason: &str,
        actor_user_id: Option<&str>,
        correlation_id: &str,
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
        txn.commit().await.map_err(backend)?;
        Ok(revoked)
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
    /// @emoji 🧮️ The only place `:User`/`:Space`/`:MEMBER_OF` graph state is written — see the
    /// sqlite backend's twin for the full rationale (unconditional: `decide` already enforced every
    /// law before this event existed).
    async fn project(&self, txn: &mut Txn, event: &DirectoryEvent) -> DirectoryResult<()> {
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
            }
            DirectoryEventBody::SpaceDeleted { space_id } => {
                txn.run(query("MATCH (g:ShareGrant {spaceId: $id}) DETACH DELETE g").param("id", space_id.clone())).await.map_err(backend)?;
                txn.run(query("MATCH (r:ArtifactRetention {spaceId: $id}) DETACH DELETE r").param("id", space_id.clone())).await.map_err(backend)?;
                txn.run(query("MATCH (p:ArtifactCheckpointPrivate {spaceId: $id}) DETACH DELETE p").param("id", space_id.clone())).await.map_err(backend)?;
                txn.run(query("MATCH (c:ArtifactCheckpoint {spaceId: $id}) DETACH DELETE c").param("id", space_id.clone())).await.map_err(backend)?;
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
            DirectoryEventBody::ArtifactCheckpointPublished { checkpoint } => {
                let scope_key = document_scope_key_v1(&checkpoint.scope);
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
    //#region ShareTokens
    async fn issue_share_token(&self, scope: &DocumentScope, ttl_secs: i64, correlation_id: &str) -> DirectoryResult<IssuedShareToken> {
        let issued = prepare_share_token(scope, ttl_secs, now_ms())?;
        let audit = auth_audit(issued.record.created_at, "share-issued", Some(&issued.record.id), None, None, None, "success", None, correlation_id, "server")?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        txn.run(
                query("CREATE (g:ShareGrant {id: $id, selector: $selector, secretDigest: $secret_digest, spaceId: $space_id, documentId: $document_id, createdAt: $created_at, expiresAt: $expires_at})")
                    .param("id", issued.record.id.clone())
                    .param("selector", issued.record.selector.clone())
                    .param("secret_digest", encode_capability_bytes(&issued.record.secret_digest))
                    .param("space_id", scope.space_id.clone())
                    .param("document_id", scope.document_id.clone())
                    .param("created_at", issued.record.created_at)
                    .param("expires_at", issued.record.expires_at),
            )
            .await
            .map_err(backend)?;
        insert_auth_audit(&mut txn, &audit).await?;
        txn.commit().await.map_err(backend)?;
        Ok(issued)
    }

    async fn revoke_share_token(&self, scope: &DocumentScope, share_id: &str, reason: &str, correlation_id: &str) -> DirectoryResult<()> {
        validate_bounded_auth_text(reason, "share revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let audit = auth_audit(revoked_at, "share-revoked", Some(share_id), None, None, None, "success", Some(reason), correlation_id, "server")?;
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

    async fn authenticate_share(&self, scope: &DocumentScope, capability: &ShareCapability) -> DirectoryResult<bool> {
        Ok(self.authenticate_share_binding(scope, capability).await?.is_some())
    }

    async fn authenticate_share_binding(&self, scope: &DocumentScope, capability: &ShareCapability) -> DirectoryResult<Option<ShareTokenRecord>> {
        let mut result = self
            .graph
            .execute(
                query("MATCH (g:ShareGrant {selector: $selector, spaceId: $space_id, documentId: $document_id}) RETURN g AS g")
                    .param("selector", capability.selector())
                    .param("space_id", scope.space_id.clone())
                    .param("document_id", scope.document_id.clone()),
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
                query("MATCH (g:ShareGrant {id: $id, selector: $selector, spaceId: $space_id, documentId: $document_id}) RETURN g AS g")
                    .param("id", share_id)
                    .param("selector", selector)
                    .param("space_id", scope.space_id.clone())
                    .param("document_id", scope.document_id.clone()),
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
        let mut result = self
            .graph
            .execute(query("MATCH (a:AuthSession {selector: $selector})-[:BELONGS_TO]->(u:User) RETURN a AS a, u.id AS userId").param("selector", capability.selector()))
            .await
            .map_err(backend)?;
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
        let mut revoked = self.revoke_auth_sessions_by("id", id, None, reason, actor_user_id, correlation_id).await?;
        Ok(revoked.pop())
    }

    async fn revoke_auth_sessions_for_user(&self, user_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>> {
        self.revoke_auth_sessions_by("user", user_id, None, reason, actor_user_id, correlation_id).await
    }

    async fn revoke_auth_sessions_for_identity(&self, provider: &str, subject_digest: [u8; 32], reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>> {
        self.revoke_auth_sessions_by("identity", provider, Some(subject_digest), reason, actor_user_id, correlation_id).await
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

    //#region Invites
    async fn issue_invite(&self, space_id: &str, role: SpaceRole, ttl_secs: i64, correlation_id: &str) -> DirectoryResult<IssuedInvite> {
        let issued = prepare_invite(space_id, role, ttl_secs, now_ms())?;
        let audit = auth_audit(issued.record.created_at, "invite-issued", Some(&issued.record.id), None, None, None, "success", None, correlation_id, "server")?;
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

    async fn authenticate_invite(&self, capability: &InviteCapability) -> DirectoryResult<Option<InviteRecord>> {
        let mut result = self.graph.execute(query("MATCH (i:SpaceInvite {selector: $selector}) RETURN i AS i").param("selector", capability.selector())).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => {
                let record = invite_from_node(&row)?;
                Ok((record.accepted_at.is_none() && active_capability(&record.selector, &record.secret_digest, record.expires_at, record.revoked_at, capability.selector(), &capability.secret_digest(), now_ms())).then_some(record))
            }
            None => Ok(None),
        }
    }

    async fn revoke_invite(&self, invite_id: &str, reason: &str, correlation_id: &str) -> DirectoryResult<()> {
        validate_bounded_auth_text(reason, "invite revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let audit = auth_audit(revoked_at, "invite-revoked", Some(invite_id), None, None, None, "success", Some(reason), correlation_id, "server")?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut result = txn.execute(query("MATCH (i:SpaceInvite {id: $id}) WHERE i.revokedAt IS NULL SET i.revokedAt = $revoked_at, i.revokedReason = $reason RETURN count(i) AS c").param("id", invite_id).param("revoked_at", revoked_at).param("reason", reason)).await.map_err(backend)?;
        let changed: i64 = result.next(txn.handle()).await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
        if changed == 0 {
            return Err(DirectoryError::NotFound(format!("invite {invite_id}")));
        }
        insert_auth_audit(&mut txn, &audit).await?;
        txn.commit().await.map_err(backend)?;
        Ok(())
    }

    async fn list_invites(&self, space_id: &str) -> DirectoryResult<Vec<InviteRecord>> {
        let mut result = self.graph.execute(query("MATCH (i:SpaceInvite {spaceId: $space_id}) RETURN i AS i ORDER BY i.createdAt DESC").param("space_id", space_id)).await.map_err(backend)?;
        let mut invites = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            invites.push(invite_from_node(&row)?);
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
                             CREATE (s:SyncSession {id: $id, authSessionId: $auth_session_id, authorizationGeneration: $generation, actorId: $actor_id, spaceId: $space_id, documentId: $document_id, surface: $surface, clientLabel: $client_label, spaceRole: $role, connectedAt: $connected_at})
                             CREATE (s)-[:AS_USER]->(u)",
                        )
                        .param("space_id", space_id)
                        .param("document_id", document_id)
                        .param("surface", surface)
                        .param("user_id", user_id)
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
                        query("CREATE (s:SyncSession {id: $id, authSessionId: $auth_session_id, authorizationGeneration: $generation, actorId: $actor_id, spaceId: $space_id, documentId: $document_id, surface: $surface, clientLabel: $client_label, spaceRole: $role, connectedAt: $connected_at})")
                            .param("space_id", space_id)
                            .param("document_id", document_id)
                            .param("surface", surface)
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
                            s.spaceId AS spaceId, s.surface AS surface, u.id AS userId, s.spaceRole AS role, s.clientLabel AS clientLabel,
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

    async fn list_active_sync_sessions(&self, space_id: Option<&str>) -> DirectoryResult<Vec<SyncSessionRecord>> {
        let cypher = "MATCH (s:SyncSession)
                       WHERE s.disconnectedAt IS NULL AND ($space_id IS NULL OR s.spaceId = $space_id)
                       OPTIONAL MATCH (s)-[:AS_USER]->(u:User)
                       RETURN s.id AS id, s.authSessionId AS authSessionId, s.authorizationGeneration AS generation, s.actorId AS actorId,
                              s.spaceId AS spaceId, s.documentId AS documentId, s.surface AS surface, u.id AS userId,
                              s.spaceRole AS role, s.clientLabel AS clientLabel, s.connectedAt AS connectedAt, s.disconnectedAt AS disconnectedAt
                       ORDER BY s.connectedAt DESC";
        let mut result = self.graph.execute(query(cypher).param("space_id", space_id.map(str::to_string))).await.map_err(backend)?;
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
            if stored.value.as_ref() != encoded { return Err(DirectoryError::Conflict("artifact CAS checkpoint identity names a different ownership plan".into())); }
        }
        drop(historical);
        let mut published = txn.execute(query("MATCH (r:ArtifactCasReference {scopeCheckpointKey: $key}) RETURN r.generation AS generation, r.writeEpoch AS writeEpoch, r.plan AS plan").param("key", scope_key.clone())).await.map_err(backend)?;
        if let Some(row) = published.next(txn.handle()).await.map_err(backend)? {
            let stored: neo4rs::BoltBytes = row.get("plan").map_err(backend)?;
            if stored.value.as_ref() != encoded { return Err(DirectoryError::Conflict("artifact CAS published ownership conflict".into())); }
            let reservation = ArtifactCasReservation::fenced(plan.clone(), u64::try_from(row.get::<i64>("generation").map_err(backend)?).map_err(backend)?, u64::try_from(row.get::<i64>("writeEpoch").map_err(backend)?).map_err(backend)?, i64::MAX as u64, coordinator_id, physical_epoch);
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
        let mut current = txn.execute(query("MATCH (r:ArtifactCasReservation {scopeCheckpointKey: $key}) RETURN r.generation AS generation, r.writeEpoch AS writeEpoch, r.expiresAtMs AS expiresAtMs, r.plan AS plan").param("key", scope_key.clone())).await.map_err(backend)?;
        if let Some(row) = current.next(txn.handle()).await.map_err(backend)? {
            let stored: neo4rs::BoltBytes = row.get("plan").map_err(backend)?;
            if stored.value.as_ref() != encoded { return Err(DirectoryError::Conflict("artifact CAS reservation identity conflict".into())); }
            let expiry: i64 = row.get("expiresAtMs").map_err(backend)?;
            if expiry > i64::try_from(now_ms).map_err(backend)? {
                let reservation = ArtifactCasReservation::fenced(plan.clone(), u64::try_from(row.get::<i64>("generation").map_err(backend)?).map_err(backend)?, u64::try_from(row.get::<i64>("writeEpoch").map_err(backend)?).map_err(backend)?, u64::try_from(expiry).map_err(backend)?, coordinator_id, physical_epoch);
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

    async fn append_reserved_artifact_checkpoint(&self, event: Option<&NewDirectoryEvent>, checkpoint: &ArtifactCheckpoint, reservation: &ArtifactCasReservation, current_now_ms: u64) -> DirectoryResult<Vec<DirectoryEvent>> {
        validate_artifact_cas_publication_v1(&reservation.plan, checkpoint).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        if let Some(event) = event { validate_verified_checkpoint_append(event, checkpoint)?; }
        let scope_key = checkpoint_key_v1(&reservation.plan.scope, reservation.plan.checkpoint_id);
        let encoded = encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        if cas_lock_space(&mut txn, &reservation.plan.scope.space_id).await? > i64::try_from(current_now_ms).map_err(backend)? { return Err(DirectoryError::Conflict("artifact CAS deletion lease is active for this space".into())); }
        let mut published = txn.execute(query("MATCH (r:ArtifactCasReference {scopeCheckpointKey: $key}) RETURN r.generation AS generation, r.writeEpoch AS writeEpoch, r.plan AS plan").param("key", scope_key.clone())).await.map_err(backend)?;
        if let Some(row) = published.next(txn.handle()).await.map_err(backend)? {
            let stored: neo4rs::BoltBytes = row.get("plan").map_err(backend)?;
            if event.is_some() || row.get::<i64>("generation").map_err(backend)? != i64::try_from(reservation.generation).map_err(backend)? || row.get::<i64>("writeEpoch").map_err(backend)? != i64::try_from(reservation.write_epoch).map_err(backend)? || stored.value.as_ref() != encoded {
                return Err(DirectoryError::Conflict("artifact CAS published reservation conflict".into()));
            }
            return Ok(Vec::new());
        }
        drop(published);
        let mut current = txn.execute(query("MATCH (r:ArtifactCasReservation {scopeCheckpointKey: $key}) RETURN r.generation AS generation, r.writeEpoch AS writeEpoch, r.expiresAtMs AS expiresAtMs, r.plan AS plan").param("key", scope_key.clone())).await.map_err(backend)?;
        let row = current.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Conflict("artifact CAS reservation is missing, expired, or superseded".into()))?;
        let stored: neo4rs::BoltBytes = row.get("plan").map_err(backend)?;
        if row.get::<i64>("generation").map_err(backend)? != i64::try_from(reservation.generation).map_err(backend)? || row.get::<i64>("writeEpoch").map_err(backend)? != i64::try_from(reservation.write_epoch).map_err(backend)? || row.get::<i64>("expiresAtMs").map_err(backend)? != i64::try_from(reservation.expires_at_ms).map_err(backend)? || reservation.expires_at_ms <= current_now_ms || stored.value.as_ref() != encoded {
            return Err(DirectoryError::Conflict("artifact CAS reservation is missing, expired, or superseded".into()));
        }
        drop(current);
        let event = event.ok_or_else(|| DirectoryError::Conflict("new artifact CAS publication requires one public event".into()))?;
        let id = time_ordered_id(); let recorded_at_ms = now_ms(); let payload_value = serde_json::Value::from(&event.body.to_value()); let kind = payload_value.get("kind").and_then(|value| value.as_str()).unwrap_or_default().to_string();
        let mut counter = txn.execute(query("MERGE (c:DirectoryCounter {id: 'singleton'}) ON CREATE SET c.seq = 0 SET c.seq = c.seq + 1 RETURN c.seq AS seq")).await.map_err(backend)?;
        let seq: i64 = counter.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("directory counter query returned no row".into()))?.get("seq").map_err(backend)?; drop(counter);
        let public_seq = u64::try_from(seq).map_err(backend)?; if public_seq > DIRECTORY_WIRE_INTEGER_MAX { return Err(DirectoryError::Conflict("directory event sequence exceeds the public integer boundary".into())); }
        txn.run(query("CREATE (e:DirectoryEvent {seq: $seq, id: $id, hlcPhysical: $hlc_physical, hlcLogical: $hlc_logical, actorKind: $actor_kind, actorId: $actor_id, spaceId: $space_id, userId: $user_id, kind: $kind, payload: $payload, recordedAt: $recorded_at})")
            .param("seq", seq).param("id", id.clone()).param("hlc_physical", event.hlc.physical_ms).param("hlc_logical", i64::from(event.hlc.logical)).param("actor_kind", actor_kind_to_str(event.actor.kind)).param("actor_id", event.actor.id.clone()).param("space_id", event.space_id.clone()).param("user_id", event.user_id.clone()).param("kind", kind).param("payload", payload_value.to_string()).param("recorded_at", recorded_at_ms)).await.map_err(backend)?;
        let full = DirectoryEvent { seq: public_seq, id, hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone(), recorded_at_ms };
        txn.run(query("CREATE (:ArtifactAuthorityEvent {eventSeq: $event_seq, scopeCheckpointKey: $key, payload: $payload})").param("event_seq", seq).param("key", scope_key.clone()).param("payload", directory::os_pack::json::to_json_string(checkpoint))).await.map_err(backend)?;
        self.project(&mut txn, &full).await?; self.project_verified_checkpoint(&mut txn, &full, checkpoint).await?;
        let generation = cas_generation(&mut txn).await?;
        txn.run(query("CREATE (:ArtifactCasLedgerEvent {generation: $generation, operation: 'publish', scopeCheckpointKey: $key, spaceId: $space_id, documentId: $document_id, checkpointId: $checkpoint_id, writeEpoch: $write_epoch, expiresAtMs: $expires_at, eventSeq: $event_seq, plan: $plan})")
            .param("generation", generation).param("key", scope_key).param("space_id", reservation.plan.scope.space_id.clone()).param("document_id", reservation.plan.scope.document_id.clone()).param("checkpoint_id", hex_lower(&reservation.plan.checkpoint_id.0)).param("write_epoch", i64::try_from(reservation.write_epoch).map_err(backend)?).param("expires_at", i64::try_from(reservation.expires_at_ms).map_err(backend)?).param("event_seq", seq).param("plan", encoded)).await.map_err(backend)?;
        cas_project_publish(&mut txn, reservation, generation).await?;
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
        if limit == 0 || limit > ARTIFACT_CAS_SWEEP_PAGE_MAX { return Err(DirectoryError::Conflict(format!("artifact CAS sweep page requires limit 1..={ARTIFACT_CAS_SWEEP_PAGE_MAX}"))); }
        let mut head = self.graph.execute(query("MATCH (h:ArtifactCasLedgerHead {id: 'singleton'}) RETURN h.generation AS generation")).await.map_err(backend)?;
        let current = match head.next().await.map_err(backend)? { Some(row) => row.get::<i64>("generation").map_err(backend)?, None => 0 }; let after = i64::try_from(after_generation).map_err(backend)?; let through = i64::try_from(through_generation).map_err(backend)?;
        if through > current || after > through { return Err(DirectoryError::Conflict("artifact CAS sweep bounds are outside the ledger".into())); }
        let mut result = self.graph.execute(query("MATCH (e:ArtifactCasLedgerEvent) WHERE e.generation > $after AND e.generation <= $through RETURN e.generation AS generation, e.plan AS plan ORDER BY e.generation LIMIT $limit").param("after", after).param("through", through).param("limit", i64::try_from(limit).map_err(backend)?)).await.map_err(backend)?;
        let mut next = after; let mut objects = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? { next = row.get("generation").map_err(backend)?; if let Ok(plan) = row.get::<neo4rs::BoltBytes>("plan") { objects.extend(decode_artifact_cas_ownership_v1(&plan.value).map_err(|error| DirectoryError::Backend(error.to_string()))?.objects); } }
        objects.sort_by_key(|object| (object.space_id.clone(), object.kind, object.digest.0)); objects.dedup();
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
        if !row.get::<bool>("observed").map_err(backend)? { return Err(DirectoryError::Conflict("artifact CAS sweep observation is ahead of the ledger".into())); }
        Ok(row.get::<bool>("referenced").map_err(backend)? || row.get::<bool>("reserved").map_err(backend)?)
    }

    async fn acquire_artifact_cas_delete_fence(&self, key: &ArtifactCasObjectKey, observed_generation: u64, lease_token: [u8; 32], now_ms: u64, expires_at_ms: u64) -> DirectoryResult<Option<ArtifactCasDeleteFence>> {
        if observed_generation == 0 { return Err(DirectoryError::Conflict("artifact CAS sweep requires a nonzero observed generation".into())); }
        if lease_token == [0; 32] || expires_at_ms <= now_ms { return Err(DirectoryError::Conflict("artifact CAS deletion lease is invalid".into())); }
        let token = cas_object_token(key);
        let now = i64::try_from(now_ms).map_err(backend)?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        if cas_lock_space(&mut txn, &key.space_id).await? > now {
            txn.commit().await.map_err(backend)?;
            return Ok(None);
        }
        let mut epoch_result = txn.execute(query("MATCH (b:ArtifactCasSpaceBarrier {spaceId: $space_id}) SET b.fenceEpoch = coalesce(b.fenceEpoch, 0) + 1, b.leaseToken = $lease_token, b.leaseExpiresAtMs = $expires_at RETURN b.fenceEpoch AS epoch").param("space_id", key.space_id.clone()).param("lease_token", lease_token.to_vec()).param("expires_at", i64::try_from(expires_at_ms).map_err(backend)?)).await.map_err(backend)?;
        let physical_epoch: i64 = epoch_result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS barrier epoch returned no row".into()))?.get("epoch").map_err(backend)?;
        drop(epoch_result);
        let mut result = txn.execute(query("MATCH (h:ArtifactCasLedgerHead {id: 'singleton'}) RETURN h.generation AS generation, EXISTS { MATCH (r:ArtifactCasReference {spaceId: $space_id}) WHERE $token IN r.objects } AS referenced, EXISTS { MATCH (r:ArtifactCasReservation {spaceId: $space_id}) WHERE r.expiresAtMs > $now AND $token IN r.objects } AS reserved").param("space_id", key.space_id.clone()).param("token", token).param("now", now)).await.map_err(backend)?;
        let row = result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Conflict("artifact CAS sweep requires an initialized ledger".into()))?; let generation: i64 = row.get("generation").map_err(backend)?;
        if generation < i64::try_from(observed_generation).map_err(backend)? { return Err(DirectoryError::Conflict("artifact CAS sweep observation is ahead of the ledger".into())); }
        let referenced: bool = row.get("referenced").map_err(backend)?; let reserved: bool = row.get("reserved").map_err(backend)?;
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
        if expires_at_ms <= now_ms { return Err(DirectoryError::Conflict("artifact CAS deletion lease renewal is invalid".into())); }
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        cas_lock_space(&mut txn, &fence.object().space_id).await?;
        let mut result = txn.execute(query("MATCH (b:ArtifactCasSpaceBarrier {spaceId: $space_id}) WHERE b.leaseToken = $lease_token AND b.leaseExpiresAtMs > $now SET b.leaseExpiresAtMs = $expires_at RETURN count(b) AS renewed").param("space_id", fence.object().space_id.clone()).param("lease_token", fence.lease_token().to_vec()).param("now", i64::try_from(now_ms).map_err(backend)?).param("expires_at", i64::try_from(expires_at_ms).map_err(backend)?)).await.map_err(backend)?;
        let renewed: i64 = result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS deletion lease renewal returned no row".into()))?.get("renewed").map_err(backend)?;
        drop(result);
        if renewed != 1 { return Err(DirectoryError::Conflict("artifact CAS deletion lease is no longer owned".into())); }
        txn.commit().await.map_err(backend)?;
        Ok(())
    }

    async fn release_artifact_cas_delete_fence(&self, fence: ArtifactCasDeleteFence) -> DirectoryResult<()> {
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        cas_lock_space(&mut txn, &fence.object().space_id).await?;
        let mut result = txn.execute(query("MATCH (b:ArtifactCasSpaceBarrier {spaceId: $space_id}) WHERE b.leaseToken = $lease_token REMOVE b.leaseToken, b.leaseExpiresAtMs RETURN count(b) AS released").param("space_id", fence.object().space_id.clone()).param("lease_token", fence.lease_token().to_vec())).await.map_err(backend)?;
        let released: i64 = result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS deletion lease release returned no row".into()))?.get("released").map_err(backend)?;
        drop(result);
        if released != 1 { return Err(DirectoryError::Conflict("artifact CAS deletion lease is no longer owned".into())); }
        txn.commit().await.map_err(backend)?;
        Ok(())
    }

    //#region EventLog
    /// @emoji ➕️ Assigns a dense `seq` via a `(:DirectoryCounter {id:'singleton'})` node
    /// incremented in the same transaction as the `(:DirectoryEvent)` node and the projection —
    /// the write's atomicity comes from `Txn`, not from any Neo4j auto-increment primitive (Neo4j
    /// has none).
    async fn append_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<Vec<DirectoryEvent>> {
        if events.iter().any(|event| matches!(&event.body, DirectoryEventBody::ArtifactCheckpointPublished { .. })) {
            return Err(DirectoryError::Conflict("checkpoint publication requires the verified authority append seam".into()));
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
            if seq > DIRECTORY_WIRE_INTEGER_MAX {
                return Err(DirectoryError::Conflict("directory event sequence exceeds the public integer boundary".into()));
            }
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
            let full = DirectoryEvent { seq, id, hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone(), recorded_at_ms };
            self.project(&mut txn, &full).await?;
            match &full.body {
                DirectoryEventBody::ArtifactRetentionAdvanced { retention } => {
                    let generation = cas_generation(&mut txn).await?;
                    txn.run(query("CREATE (:ArtifactCasLedgerEvent {generation: $generation, operation: 'retention', spaceId: $space_id, documentId: $document_id, checkpointId: $checkpoint_id, eventSeq: $event_seq})").param("generation", generation).param("space_id", retention.scope.space_id.clone()).param("document_id", retention.scope.document_id.clone()).param("checkpoint_id", hex_lower(&retention.retained_checkpoint_id.0)).param("event_seq", i64::try_from(seq).map_err(backend)?)).await.map_err(backend)?;
                    cas_project_release(&mut txn, "retention", &retention.scope.space_id, Some(&retention.scope), Some(retention.retained_checkpoint_id)).await?;
                }
                DirectoryEventBody::SpaceDeleted { space_id } => {
                    let generation = cas_generation(&mut txn).await?;
                    txn.run(query("CREATE (:ArtifactCasLedgerEvent {generation: $generation, operation: 'space-delete', spaceId: $space_id, eventSeq: $event_seq})").param("generation", generation).param("space_id", space_id.clone()).param("event_seq", i64::try_from(seq).map_err(backend)?)).await.map_err(backend)?;
                    cas_project_release(&mut txn, "space-delete", space_id, None, None).await?;
                }
                _ => {}
            }
            persisted.push(full);
        }
        txn.commit().await.map_err(backend)?;
        Ok(persisted)
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
                self.project(&mut txn, event).await?;
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
            while let Some(row) = result.next(txn.handle()).await.map_err(backend)? { entries.push(row.get::<neo4rs::Node>("e").map_err(backend)?); }
            drop(result);
            if entries.is_empty() { break; }
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
                        if operation == "reserve" { cas_project_reserve(&mut txn, &reservation).await?; } else { cas_project_publish(&mut txn, &reservation, ledger_cursor).await?; }
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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    // 🔬️ Neo4j has no in-memory test mode; integration tests run against a live/testcontainers
    // instance per the verification plan (HP-3) — not exercised in unit-test CI without a running
    // Neo4j, unlike the sqlite backend's `:memory:` tests.
}
//#endregion 🧪️Tests
