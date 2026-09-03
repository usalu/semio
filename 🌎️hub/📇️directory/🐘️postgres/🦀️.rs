//! 🐘️ `HubDirectory` over PostgreSQL — direct `sqlx-postgres`/`sqlx-core` (not the `sqlx` facade),
//! avoiding the facade's optional SQLite resolver edge. The scale-out backend for multi-node
//! self-hosted deployments. `#[cfg(feature = "postgres")]`-gated as a whole by the parent
//! `directory` module (see `📇️directory/🦀️.rs`'s `//#region 🔖️Backends`).
//!
//! 🌳️ `SCHEMA` is inlined as a `const` string rather than `include_str!`-ed from a sibling
//! `.sql` file: Shape V2 tree purity allows only `component.<ext>` files, `📦️packages`, and plain
//! component folders below an owner root, so a standalone `🗄️.sql` asset has no home in this
//! tree (it is neither example/fixture/generated data for `rootDataDirNames` nor packaging code)
//! — folding it into a string literal is a zero-behavior-change mechanical transform (see
//! `📋️TEMPLATE-FAMILY.md`'s "non-source assets" section for the general rule this establishes).

use crate::artifact_authority::chunk_cas::{decode_artifact_cas_ownership_v1, encode_artifact_cas_ownership_v1, validate_artifact_cas_publication_v1, ArtifactCasDeleteFence, ArtifactCasObjectKey, ArtifactCasOwnershipPlanV1, ArtifactCasReservation};
use crate::directory::error::{DirectoryError, DirectoryResult};
use crate::directory::model::*;
use crate::directory::{
    active_capability, auth_audit, bounded_event_read, checkpoint_projection_rebuild, kind_to_str, prepare_auth_session, prepare_invite, prepare_share_token, role_from_wire, same_admin_operation_request, validate_admin_operation_audit,
    validate_bounded_auth_text, validate_verified_checkpoint_append, visibility_to_str, ArtifactCasSweepCandidatePage, HubClock, HubDirectory, InviteCapability, NewDirectoryEvent, ProjectionRebuildControl, SessionCapability, ShareCapability,
    ADMIN_PAGE_MAX, ARTIFACT_CAS_RESERVATION_MAX_TTL_MS, ARTIFACT_CAS_SWEEP_PAGE_MAX, ARTIFACT_CHECKPOINT_LINEAGE_MAX, AUTH_AUDIT_PAGE_MAX, AUTH_TEXT_MAX_BYTES, UNCONTROLLED_PROJECTION_REBUILD,
};
use directory::os_directory::{
    ArtifactCheckpoint, ArtifactHash, ArtifactRetention, DirectoryActor, DirectoryActorKind, DirectoryEvent, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, DocumentDescriptor, Hlc, PublishedArtifactCheckpoint,
};
use directory::os_identity::time_ordered_id;
use directory::{DslValue, FromValue, ToValue};
use semio_framework_hash::Sha256;
use sqlx_postgres::{PgPool, PgPoolOptions};

//#region 🔖️Schema
// 🛢️ os-hub directory Postgres schema (identity/tenancy only) — idempotent bootstrap
// (CREATE ... IF NOT EXISTS), no migration framework (greenfield: there are no users yet, so
// schema changes are edited in place, not migrated). Document persistence and blobs are
// `db::Database`'s tables, not this schema's.
const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS hub_user (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    password_hash TEXT,
    sso_subject TEXT,
    sso_provider TEXT,
    created_at BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS hub_space (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    owner_user_id TEXT NOT NULL REFERENCES hub_user(id),
    created_at BIGINT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('atelier', 'studio', 'archive')),
    visibility TEXT NOT NULL CHECK (visibility IN ('private', 'public'))
);

CREATE TABLE IF NOT EXISTS hub_space_membership (
    space_id TEXT NOT NULL REFERENCES hub_space(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES hub_user(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('author', 'spectator')),
    created_at BIGINT NOT NULL,
    PRIMARY KEY (space_id, user_id)
);

CREATE TABLE IF NOT EXISTS hub_document_descriptor (
    space_id TEXT NOT NULL REFERENCES hub_space(id) ON DELETE CASCADE,
    document_id TEXT NOT NULL,
    descriptor JSONB NOT NULL,
    announced_at BIGINT NOT NULL,
    PRIMARY KEY (space_id, document_id)
);

CREATE TABLE IF NOT EXISTS hub_artifact_checkpoint (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    checkpoint_id BYTEA NOT NULL CHECK (octet_length(checkpoint_id) = 32),
    parent_checkpoint_id BYTEA CHECK (parent_checkpoint_id IS NULL OR octet_length(parent_checkpoint_id) = 32),
    descriptor_digest BYTEA NOT NULL CHECK (octet_length(descriptor_digest) = 32),
    frontier_document_id TEXT NOT NULL,
    head_edit_ordinal BIGINT NOT NULL,
    head_edit_id TEXT NOT NULL,
    last_commit_seq BIGINT NOT NULL,
    chain_hash BYTEA NOT NULL CHECK (octet_length(chain_hash) = 32),
    pack_sha256 BYTEA NOT NULL CHECK (octet_length(pack_sha256) = 32),
    pack_byte_length BIGINT NOT NULL,
    spr_sha256 BYTEA NOT NULL CHECK (octet_length(spr_sha256) = 32),
    spr_byte_length BIGINT NOT NULL,
    aggregate_sha256 BYTEA NOT NULL CHECK (octet_length(aggregate_sha256) = 32),
    published_at BIGINT NOT NULL,
    event_seq BIGINT NOT NULL UNIQUE,
    active BOOLEAN NOT NULL,
    payload JSONB NOT NULL,
    PRIMARY KEY (space_id, document_id, checkpoint_id),
    FOREIGN KEY (space_id, document_id) REFERENCES hub_document_descriptor(space_id, document_id) ON DELETE CASCADE
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_artifact_checkpoint_active ON hub_artifact_checkpoint (space_id, document_id) WHERE active;
CREATE INDEX IF NOT EXISTS idx_artifact_checkpoint_lineage ON hub_artifact_checkpoint (space_id, document_id, event_seq);

CREATE TABLE IF NOT EXISTS hub_artifact_retention (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    retained_checkpoint_id BYTEA NOT NULL CHECK (octet_length(retained_checkpoint_id) = 32),
    floor_document_id TEXT NOT NULL,
    floor_head_edit_ordinal BIGINT NOT NULL,
    floor_head_edit_id TEXT NOT NULL,
    floor_last_commit_seq BIGINT NOT NULL,
    floor_chain_hash BYTEA NOT NULL CHECK (octet_length(floor_chain_hash) = 32),
    checkpoint_lineage_head BYTEA NOT NULL CHECK (octet_length(checkpoint_lineage_head) = 32),
    event_seq BIGINT NOT NULL UNIQUE,
    payload JSONB NOT NULL,
    PRIMARY KEY (space_id, document_id),
    FOREIGN KEY (space_id, document_id, retained_checkpoint_id) REFERENCES hub_artifact_checkpoint(space_id, document_id, checkpoint_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS hub_auth_session (
    id TEXT PRIMARY KEY,
    selector TEXT NOT NULL UNIQUE,
    secret_digest BYTEA NOT NULL CHECK (octet_length(secret_digest) = 32),
    user_id TEXT NOT NULL REFERENCES hub_user(id) ON DELETE CASCADE,
    identity_provider TEXT NOT NULL,
    identity_subject_digest BYTEA NOT NULL CHECK (octet_length(identity_subject_digest) = 32),
    issued_at BIGINT NOT NULL,
    expires_at BIGINT NOT NULL,
    revoked_at BIGINT,
    revoked_reason TEXT,
    authorization_generation BIGINT NOT NULL CHECK (authorization_generation >= 1),
    device_instance_id TEXT NOT NULL,
    session_kind TEXT NOT NULL CHECK (session_kind IN ('external', 'development-local'))
);

CREATE TABLE IF NOT EXISTS hub_sync_session (
    id TEXT PRIMARY KEY,
    auth_session_id TEXT REFERENCES hub_auth_session(id) ON DELETE SET NULL,
    authorization_generation BIGINT NOT NULL,
    actor_id TEXT NOT NULL,
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    surface TEXT NOT NULL,
    user_id TEXT REFERENCES hub_user(id) ON DELETE SET NULL,
    authenticated_email TEXT,
    space_role TEXT,
    client_label TEXT NOT NULL,
    connected_at BIGINT NOT NULL,
    disconnected_at BIGINT
);

CREATE TABLE IF NOT EXISTS hub_share_grant (
    id TEXT PRIMARY KEY,
    selector TEXT NOT NULL UNIQUE,
    secret_digest BYTEA NOT NULL CHECK (octet_length(secret_digest) = 32),
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    created_at BIGINT NOT NULL,
    expires_at BIGINT NOT NULL,
    revoked_at BIGINT,
    revoked_reason TEXT
);

CREATE TABLE IF NOT EXISTS hub_space_invite (
    id TEXT PRIMARY KEY,
    selector TEXT NOT NULL UNIQUE,
    secret_digest BYTEA NOT NULL CHECK (octet_length(secret_digest) = 32),
    space_id TEXT NOT NULL REFERENCES hub_space(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('author', 'spectator')),
    created_at BIGINT NOT NULL,
    expires_at BIGINT NOT NULL,
    revoked_at BIGINT,
    revoked_reason TEXT,
    accepted_at BIGINT
);

CREATE TABLE IF NOT EXISTS hub_auth_audit (
    sequence BIGSERIAL PRIMARY KEY,
    id TEXT NOT NULL UNIQUE,
    occurred_at BIGINT NOT NULL,
    event_kind TEXT NOT NULL,
    auth_session_id TEXT,
    target_user_id TEXT,
    actor_user_id TEXT,
    provider TEXT,
    outcome_code TEXT NOT NULL,
    reason_code TEXT,
    correlation_id TEXT NOT NULL,
    peer_class TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS hub_admin_operation_audit (
    sequence BIGSERIAL PRIMARY KEY,
    request_id TEXT NOT NULL,
    intent_digest TEXT NOT NULL CHECK (length(intent_digest) = 64),
    operation_id TEXT NOT NULL,
    occurred_at BIGINT NOT NULL,
    phase TEXT NOT NULL CHECK (phase IN ('accepted', 'succeeded', 'failed', 'cancelled')),
    terminal BOOLEAN NOT NULL,
    intent_kind TEXT NOT NULL,
    target_kind TEXT NOT NULL,
    target_id TEXT NOT NULL,
    principal_user_id TEXT NOT NULL,
    principal_session_id TEXT NOT NULL,
    principal_generation BIGINT NOT NULL CHECK (principal_generation >= 1),
    correlation_id TEXT NOT NULL,
    event_seq_first BIGINT,
    event_seq_last BIGINT,
    outcome_code TEXT NOT NULL,
    reason_code TEXT,
    UNIQUE (request_id, terminal)
);

CREATE TABLE IF NOT EXISTS hub_directory_event (
    seq BIGINT PRIMARY KEY,
    id TEXT NOT NULL UNIQUE,
    hlc_physical BIGINT NOT NULL,
    hlc_logical BIGINT NOT NULL,
    actor_kind TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    space_id TEXT,
    user_id TEXT,
    kind TEXT NOT NULL,
    payload JSONB NOT NULL,
    recorded_at BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS hub_directory_event_head (
    singleton BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (singleton),
    seq BIGINT NOT NULL CHECK (seq BETWEEN 0 AND 9007199254740991)
);
INSERT INTO hub_directory_event_head (singleton, seq) VALUES (TRUE, 0) ON CONFLICT (singleton) DO NOTHING;

CREATE TABLE IF NOT EXISTS hub_artifact_authority_journal (
    event_seq BIGINT PRIMARY KEY REFERENCES hub_directory_event(seq),
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    checkpoint_id BYTEA NOT NULL CHECK (octet_length(checkpoint_id) = 32),
    payload JSONB NOT NULL,
    UNIQUE (space_id, document_id, checkpoint_id)
);

CREATE TABLE IF NOT EXISTS hub_artifact_checkpoint_private (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    checkpoint_id BYTEA NOT NULL CHECK (octet_length(checkpoint_id) = 32),
    event_seq BIGINT NOT NULL UNIQUE REFERENCES hub_artifact_authority_journal(event_seq),
    payload JSONB NOT NULL,
    PRIMARY KEY (space_id, document_id, checkpoint_id),
    FOREIGN KEY (space_id, document_id, checkpoint_id) REFERENCES hub_artifact_checkpoint(space_id, document_id, checkpoint_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS hub_artifact_cas_ledger_head (
    singleton BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK(singleton),
    generation BIGINT NOT NULL CHECK(generation >= 0)
);
INSERT INTO hub_artifact_cas_ledger_head(singleton, generation) VALUES (TRUE, 0) ON CONFLICT(singleton) DO NOTHING;
CREATE TABLE IF NOT EXISTS hub_artifact_cas_barrier_identity (
    singleton BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK(singleton),
    coordinator_id BYTEA NOT NULL CHECK(octet_length(coordinator_id) = 32)
);

CREATE TABLE IF NOT EXISTS hub_artifact_cas_ledger_journal (
    generation BIGINT PRIMARY KEY CHECK(generation >= 1),
    operation TEXT NOT NULL CHECK(operation IN ('reserve', 'publish', 'retention', 'space-delete')),
    space_id TEXT NOT NULL,
    document_id TEXT,
    checkpoint_id BYTEA CHECK(checkpoint_id IS NULL OR octet_length(checkpoint_id) = 32),
    write_epoch BIGINT,
    expires_at_ms BIGINT,
    event_seq BIGINT,
    plan BYTEA
);
CREATE TABLE IF NOT EXISTS hub_artifact_cas_reservation (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    checkpoint_id BYTEA NOT NULL CHECK(octet_length(checkpoint_id) = 32),
    generation BIGINT NOT NULL UNIQUE,
    write_epoch BIGINT NOT NULL CHECK(write_epoch >= 1),
    expires_at_ms BIGINT NOT NULL,
    plan BYTEA NOT NULL,
    PRIMARY KEY(space_id, document_id, checkpoint_id)
);
CREATE TABLE IF NOT EXISTS hub_artifact_cas_reservation_object (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    checkpoint_id BYTEA NOT NULL CHECK(octet_length(checkpoint_id) = 32),
    kind TEXT NOT NULL CHECK(kind IN ('chunk', 'manifest')),
    object_digest BYTEA NOT NULL CHECK(octet_length(object_digest) = 32),
    PRIMARY KEY(space_id, document_id, checkpoint_id, kind, object_digest),
    FOREIGN KEY(space_id, document_id, checkpoint_id) REFERENCES hub_artifact_cas_reservation(space_id, document_id, checkpoint_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS hub_artifact_cas_reference (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    checkpoint_id BYTEA NOT NULL CHECK(octet_length(checkpoint_id) = 32),
    generation BIGINT NOT NULL UNIQUE,
    write_epoch BIGINT NOT NULL CHECK(write_epoch >= 1),
    plan BYTEA NOT NULL,
    PRIMARY KEY(space_id, document_id, checkpoint_id)
);
CREATE TABLE IF NOT EXISTS hub_artifact_cas_reference_object (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    checkpoint_id BYTEA NOT NULL CHECK(octet_length(checkpoint_id) = 32),
    kind TEXT NOT NULL CHECK(kind IN ('chunk', 'manifest')),
    object_digest BYTEA NOT NULL CHECK(octet_length(object_digest) = 32),
    PRIMARY KEY(space_id, document_id, checkpoint_id, kind, object_digest),
    FOREIGN KEY(space_id, document_id, checkpoint_id) REFERENCES hub_artifact_cas_reference(space_id, document_id, checkpoint_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS hub_artifact_cas_delete_lease (
    space_id TEXT PRIMARY KEY,
    fence_epoch BIGINT NOT NULL CHECK(fence_epoch >= 1),
    lease_token BYTEA CHECK(lease_token IS NULL OR octet_length(lease_token) = 32),
    expires_at_ms BIGINT,
    CHECK((lease_token IS NULL AND expires_at_ms IS NULL) OR (lease_token IS NOT NULL AND expires_at_ms >= 1))
);
CREATE INDEX IF NOT EXISTS idx_artifact_cas_journal_scope ON hub_artifact_cas_ledger_journal(space_id, document_id, checkpoint_id, generation);
CREATE INDEX IF NOT EXISTS idx_artifact_cas_reservation_object_lookup ON hub_artifact_cas_reservation_object(space_id, kind, object_digest);
CREATE INDEX IF NOT EXISTS idx_artifact_cas_reference_object_lookup ON hub_artifact_cas_reference_object(space_id, kind, object_digest);

CREATE INDEX IF NOT EXISTS idx_membership_user ON hub_space_membership (user_id);
CREATE INDEX IF NOT EXISTS idx_sync_session_document ON hub_sync_session (document_id, disconnected_at);
CREATE INDEX IF NOT EXISTS idx_sync_session_space ON hub_sync_session (space_id, disconnected_at);
CREATE INDEX IF NOT EXISTS idx_space_invite_space ON hub_space_invite (space_id);
CREATE INDEX IF NOT EXISTS idx_share_grant_scope ON hub_share_grant (space_id, document_id);
CREATE INDEX IF NOT EXISTS idx_auth_session_user_active ON hub_auth_session (user_id, revoked_at);
CREATE INDEX IF NOT EXISTS idx_auth_session_identity_active ON hub_auth_session (identity_provider, identity_subject_digest, revoked_at);
CREATE INDEX IF NOT EXISTS idx_auth_audit_occurred ON hub_auth_audit (occurred_at, sequence);
CREATE INDEX IF NOT EXISTS idx_admin_operation_audit_sequence ON hub_admin_operation_audit (sequence);
CREATE INDEX IF NOT EXISTS idx_admin_operation_audit_request ON hub_admin_operation_audit (request_id, sequence);
CREATE INDEX IF NOT EXISTS idx_admin_operation_audit_operation ON hub_admin_operation_audit (operation_id, sequence);
";
//#endregion 🔖️Schema

fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
}

fn backend<E: std::fmt::Display>(err: E) -> DirectoryError {
    DirectoryError::Backend(err.to_string())
}

fn array32(bytes: Vec<u8>, field: &str) -> DirectoryResult<[u8; 32]> {
    bytes.try_into().map_err(|bytes: Vec<u8>| DirectoryError::Backend(format!("{field} requires 32 bytes, got {}", bytes.len())))
}

async fn insert_auth_audit(tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, event: &AuthAuditRecord) -> DirectoryResult<()> {
    sqlx_core::query::query(
        "INSERT INTO hub_auth_audit (id, occurred_at, event_kind, auth_session_id, target_user_id, actor_user_id, provider, outcome_code, reason_code, correlation_id, peer_class) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
    )
    .bind(&event.id)
    .bind(event.occurred_at)
    .bind(&event.event_kind)
    .bind(&event.auth_session_id)
    .bind(&event.target_user_id)
    .bind(&event.actor_user_id)
    .bind(&event.provider)
    .bind(&event.outcome_code)
    .bind(&event.reason_code)
    .bind(&event.correlation_id)
    .bind(&event.peer_class)
    .execute(&mut **tx)
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

async fn cas_generation(tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>) -> DirectoryResult<i64> {
    let (generation,): (i64,) = sqlx_core::query_as::query_as("UPDATE hub_artifact_cas_ledger_head SET generation = generation + 1 WHERE singleton RETURNING generation").fetch_one(&mut **tx).await.map_err(backend)?;
    Ok(generation)
}

async fn cas_lock_space(tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, space_id: &str) -> DirectoryResult<()> {
    sqlx_core::query::query("SELECT pg_advisory_xact_lock(hashtextextended($1, 1162037326))").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
    Ok(())
}

async fn cas_reservation_barrier(tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, space_id: &str, now_ms: i64) -> DirectoryResult<([u8; 32], u64)> {
    cas_lock_space(tx, space_id).await?;
    let current: Option<(i64, Option<i64>)> = sqlx_core::query_as::query_as("SELECT fence_epoch, expires_at_ms FROM hub_artifact_cas_delete_lease WHERE space_id = $1 FOR UPDATE").bind(space_id).fetch_optional(&mut **tx).await.map_err(backend)?;
    if current.as_ref().and_then(|(_, expiry)| *expiry).is_some_and(|expiry| expiry > now_ms) {
        return Err(DirectoryError::Conflict("artifact CAS deletion lease is active for this space".into()));
    }
    let epoch = match current {
        Some((epoch, _)) => epoch.checked_add(1).ok_or_else(|| DirectoryError::Conflict("artifact CAS fence epoch overflow".into()))?,
        None => 1,
    };
    sqlx_core::query::query(
        "INSERT INTO hub_artifact_cas_delete_lease(space_id, fence_epoch, lease_token, expires_at_ms) VALUES ($1,$2,NULL,NULL) ON CONFLICT(space_id) DO UPDATE SET fence_epoch = excluded.fence_epoch, lease_token = NULL, expires_at_ms = NULL",
    )
    .bind(space_id)
    .bind(epoch)
    .execute(&mut **tx)
    .await
    .map_err(backend)?;
    let (coordinator,): (Vec<u8>,) = sqlx_core::query_as::query_as("SELECT coordinator_id FROM hub_artifact_cas_barrier_identity WHERE singleton").fetch_one(&mut **tx).await.map_err(backend)?;
    Ok((coordinator.try_into().map_err(|_| DirectoryError::Backend("artifact CAS barrier coordinator identity is invalid".into()))?, u64::try_from(epoch).map_err(backend)?))
}

async fn cas_insert_objects(tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, table: &str, plan: &ArtifactCasOwnershipPlanV1) -> DirectoryResult<()> {
    let sql = match table {
        "reservation" => "INSERT INTO hub_artifact_cas_reservation_object(space_id, document_id, checkpoint_id, kind, object_digest) VALUES ($1, $2, $3, $4, $5)",
        "reference" => "INSERT INTO hub_artifact_cas_reference_object(space_id, document_id, checkpoint_id, kind, object_digest) VALUES ($1, $2, $3, $4, $5)",
        _ => return Err(DirectoryError::Backend("invalid artifact CAS projection table".into())),
    };
    for object in &plan.objects {
        sqlx_core::query::query(sql).bind(&plan.scope.space_id).bind(&plan.scope.document_id).bind(plan.checkpoint_id.0.as_slice()).bind(object.kind.name()).bind(object.digest.0.as_slice()).execute(&mut **tx).await.map_err(backend)?;
    }
    Ok(())
}

async fn cas_project_reserve(tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, reservation: &ArtifactCasReservation) -> DirectoryResult<()> {
    let plan = encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
    sqlx_core::query::query("DELETE FROM hub_artifact_cas_reservation WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3")
        .bind(&reservation.plan.scope.space_id)
        .bind(&reservation.plan.scope.document_id)
        .bind(reservation.plan.checkpoint_id.0.as_slice())
        .execute(&mut **tx)
        .await
        .map_err(backend)?;
    sqlx_core::query::query("INSERT INTO hub_artifact_cas_reservation(space_id, document_id, checkpoint_id, generation, write_epoch, expires_at_ms, plan) VALUES ($1, $2, $3, $4, $5, $6, $7)")
        .bind(&reservation.plan.scope.space_id)
        .bind(&reservation.plan.scope.document_id)
        .bind(reservation.plan.checkpoint_id.0.as_slice())
        .bind(i64::try_from(reservation.generation).map_err(backend)?)
        .bind(i64::try_from(reservation.write_epoch).map_err(backend)?)
        .bind(i64::try_from(reservation.expires_at_ms).map_err(backend)?)
        .bind(plan)
        .execute(&mut **tx)
        .await
        .map_err(backend)?;
    cas_insert_objects(tx, "reservation", &reservation.plan).await
}

async fn cas_project_publish(tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, reservation: &ArtifactCasReservation, generation: i64) -> DirectoryResult<()> {
    let plan = encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
    sqlx_core::query::query("DELETE FROM hub_artifact_cas_reservation WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3")
        .bind(&reservation.plan.scope.space_id)
        .bind(&reservation.plan.scope.document_id)
        .bind(reservation.plan.checkpoint_id.0.as_slice())
        .execute(&mut **tx)
        .await
        .map_err(backend)?;
    sqlx_core::query::query("INSERT INTO hub_artifact_cas_reference(space_id, document_id, checkpoint_id, generation, write_epoch, plan) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(&reservation.plan.scope.space_id)
        .bind(&reservation.plan.scope.document_id)
        .bind(reservation.plan.checkpoint_id.0.as_slice())
        .bind(generation)
        .bind(i64::try_from(reservation.write_epoch).map_err(backend)?)
        .bind(plan)
        .execute(&mut **tx)
        .await
        .map_err(backend)?;
    cas_insert_objects(tx, "reference", &reservation.plan).await
}

async fn cas_project_release(tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, operation: &str, space_id: &str, document_id: Option<&str>, checkpoint_id: Option<ArtifactHash>) -> DirectoryResult<()> {
    match operation {
        "retention" => {
            sqlx_core::query::query("DELETE FROM hub_artifact_cas_reference WHERE space_id = $1 AND document_id = $2 AND checkpoint_id IN (SELECT checkpoint_id FROM hub_artifact_authority_journal WHERE space_id = $1 AND document_id = $2 AND event_seq < (SELECT event_seq FROM hub_artifact_authority_journal WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3))")
                .bind(space_id).bind(document_id.ok_or_else(|| DirectoryError::Backend("artifact CAS retention document missing".into()))?).bind(checkpoint_id.ok_or_else(|| DirectoryError::Backend("artifact CAS retention checkpoint missing".into()))?.0.as_slice()).execute(&mut **tx).await.map_err(backend)?;
            sqlx_core::query::query(
                "DELETE FROM hub_artifact_checkpoint_private WHERE space_id = $1 AND document_id = $2 AND event_seq < (SELECT event_seq FROM hub_artifact_authority_journal WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3)",
            )
            .bind(space_id)
            .bind(document_id.ok_or_else(|| DirectoryError::Backend("artifact CAS retention document missing".into()))?)
            .bind(checkpoint_id.ok_or_else(|| DirectoryError::Backend("artifact CAS retention checkpoint missing".into()))?.0.as_slice())
            .execute(&mut **tx)
            .await
            .map_err(backend)?;
        }
        "space-delete" => {
            sqlx_core::query::query("DELETE FROM hub_artifact_cas_reservation WHERE space_id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
            sqlx_core::query::query("DELETE FROM hub_artifact_cas_reference WHERE space_id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
        }
        _ => return Err(DirectoryError::Backend("invalid artifact CAS release operation".into())),
    }
    Ok(())
}

/// @emoji 🐘️ PostgreSQL-backed `HubDirectory`, pooled via `PgPool`.
pub struct PostgresDirectory {
    pool: PgPool,
}

impl PostgresDirectory {
    /// @emoji 🔌️ Connects to `database_url` and bootstraps the schema (idempotent, no migration framework).
    pub async fn connect(database_url: &str) -> DirectoryResult<Self> {
        let pool = PgPoolOptions::new().max_connections(20).connect(database_url).await.map_err(backend)?;
        for statement in SCHEMA.split(';').map(str::trim).filter(|s| !s.is_empty() && !s.starts_with("--")) {
            sqlx_core::query::query(statement).execute(&pool).await.map_err(backend)?;
        }
        let mut identity = Sha256::new();
        identity.update(b"semio.hub.artifact-cas.barrier-identity.v1\0");
        identity.update(time_ordered_id().as_bytes());
        sqlx_core::query::query("INSERT INTO hub_artifact_cas_barrier_identity(singleton, coordinator_id) VALUES (TRUE, $1) ON CONFLICT(singleton) DO NOTHING").bind(identity.finalize().as_slice()).execute(&pool).await.map_err(backend)?;
        Ok(Self { pool })
    }

    async fn revoke_auth_sessions_matching(&self, key: &str, subject_digest: Option<[u8; 32]>, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>> {
        validate_bounded_auth_text(reason, "session revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let rows: Vec<(String, String, i64, String)> = if let Some(subject_digest) = subject_digest {
            sqlx_core::query_as::query_as(
                "UPDATE hub_auth_session SET revoked_at = $3, revoked_reason = $4, authorization_generation = authorization_generation + 1 WHERE identity_provider = $1 AND identity_subject_digest = $2 AND revoked_at IS NULL RETURNING id, user_id, authorization_generation, identity_provider",
            )
            .bind(key)
            .bind(subject_digest.as_slice())
            .bind(revoked_at)
            .bind(reason)
            .fetch_all(&mut *tx)
            .await
            .map_err(backend)?
        } else {
            sqlx_core::query_as::query_as(
                "UPDATE hub_auth_session SET revoked_at = $2, revoked_reason = $3, authorization_generation = authorization_generation + 1 WHERE user_id = $1 AND revoked_at IS NULL RETURNING id, user_id, authorization_generation, identity_provider",
            )
            .bind(key)
            .bind(revoked_at)
            .bind(reason)
            .fetch_all(&mut *tx)
            .await
            .map_err(backend)?
        };
        let mut revoked = Vec::with_capacity(rows.len());
        for (id, user_id, generation, provider) in rows {
            let audit = auth_audit(revoked_at, "session-revoked", Some(&id), Some(&user_id), actor_user_id, Some(&provider), "success", Some(reason), correlation_id, "server")?;
            insert_auth_audit(&mut tx, &audit).await?;
            revoked.push(RevokedAuthSession { id, authorization_generation: u64::try_from(generation).map_err(backend)?, revoked_at });
        }
        tx.commit().await.map_err(backend)?;
        Ok(revoked)
    }

    /// @emoji 🌱️ Seeds a placeholder `seed` system user and a default `studio`/`private` space it
    /// owns, through the event log (`user.created` + `space.created` + `member.upserted`) like any
    /// other write. The system user satisfies `hub_space.owner_user_id`'s foreign key until a real
    /// bootstrap admin claims ownership through `/admin` (HP-6).
    pub async fn seed(&self) -> DirectoryResult<()> {
        let user_exists: (i64,) = sqlx_core::query_as::query_as("SELECT COUNT(*) FROM hub_user WHERE id = 'seed'").fetch_one(&self.pool).await.map_err(backend)?;
        if user_exists.0 > 0 {
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

    async fn project_verified_checkpoint(&self, tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, event: &DirectoryEvent, checkpoint: &ArtifactCheckpoint) -> DirectoryResult<()> {
        let new_event = NewDirectoryEvent { hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone() };
        validate_verified_checkpoint_append(&new_event, checkpoint)?;
        sqlx_core::query::query("INSERT INTO hub_artifact_checkpoint_private (space_id, document_id, checkpoint_id, event_seq, payload) VALUES ($1, $2, $3, $4, $5)")
            .bind(&checkpoint.scope.space_id)
            .bind(&checkpoint.scope.document_id)
            .bind(checkpoint.checkpoint_id.0.as_slice())
            .bind(i64::try_from(event.seq).map_err(backend)?)
            .bind(serde_json::Value::from(&checkpoint.to_value()))
            .execute(&mut **tx)
            .await
            .map_err(backend)?;
        Ok(())
    }

    //#region 🔖️Projections
    /// @emoji 🧮️ The only place `hub_user`/`hub_space`/`hub_space_membership` rows are written —
    /// see the sqlite backend's twin for the full rationale (unconditional: `decide` already
    /// enforced every law before this event existed).
    async fn project(&self, tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, event: &DirectoryEvent) -> DirectoryResult<()> {
        match &event.body {
            DirectoryEventBody::UserCreated { user_id, email, display_name } => {
                sqlx_core::query::query("INSERT INTO hub_user (id, email, display_name, created_at) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO NOTHING")
                    .bind(user_id)
                    .bind(email)
                    .bind(display_name)
                    .bind(event.recorded_at_ms)
                    .execute(&mut **tx)
                    .await
                    .map_err(backend)?;
            }
            DirectoryEventBody::SpaceCreated { space_id, name, space_kind, visibility, owner_user_id } => {
                sqlx_core::query::query("INSERT INTO hub_space (id, name, owner_user_id, created_at, kind, visibility) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (id) DO NOTHING")
                    .bind(space_id)
                    .bind(name)
                    .bind(owner_user_id)
                    .bind(event.recorded_at_ms)
                    .bind(kind_to_str(*space_kind))
                    .bind(visibility_to_str(*visibility))
                    .execute(&mut **tx)
                    .await
                    .map_err(backend)?;
            }
            DirectoryEventBody::SpaceRenamed { space_id, name } => {
                sqlx_core::query::query("UPDATE hub_space SET name = $2 WHERE id = $1").bind(space_id).bind(name).execute(&mut **tx).await.map_err(backend)?;
            }
            DirectoryEventBody::SpaceVisibilityChanged { space_id, visibility } => {
                sqlx_core::query::query("UPDATE hub_space SET visibility = $2 WHERE id = $1").bind(space_id).bind(visibility_to_str(*visibility)).execute(&mut **tx).await.map_err(backend)?;
            }
            DirectoryEventBody::SpaceArchived { space_id } => {
                sqlx_core::query::query("UPDATE hub_space SET kind = 'archive' WHERE id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
            }
            DirectoryEventBody::SpaceDeleted { space_id } => {
                sqlx_core::query::query("DELETE FROM hub_share_grant WHERE space_id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
                sqlx_core::query::query("DELETE FROM hub_artifact_retention WHERE space_id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
                sqlx_core::query::query("DELETE FROM hub_artifact_checkpoint_private WHERE space_id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
                sqlx_core::query::query("DELETE FROM hub_artifact_checkpoint WHERE space_id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
                sqlx_core::query::query("DELETE FROM hub_document_descriptor WHERE space_id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
                sqlx_core::query::query("DELETE FROM hub_space_membership WHERE space_id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
                sqlx_core::query::query("DELETE FROM hub_space_invite WHERE space_id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
                sqlx_core::query::query("DELETE FROM hub_space WHERE id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
            }
            DirectoryEventBody::MemberUpserted { space_id, user_id, role } => {
                sqlx_core::query::query("INSERT INTO hub_space_membership (space_id, user_id, role, created_at) VALUES ($1, $2, $3, $4) ON CONFLICT (space_id, user_id) DO UPDATE SET role = $3")
                    .bind(space_id)
                    .bind(user_id)
                    .bind(role_from_wire(*role).as_str())
                    .bind(event.recorded_at_ms)
                    .execute(&mut **tx)
                    .await
                    .map_err(backend)?;
            }
            DirectoryEventBody::MemberRemoved { space_id, user_id } => {
                sqlx_core::query::query("DELETE FROM hub_space_membership WHERE space_id = $1 AND user_id = $2").bind(space_id).bind(user_id).execute(&mut **tx).await.map_err(backend)?;
            }
            DirectoryEventBody::InviteRedeemed { space_id, user_id, role, .. } => {
                sqlx_core::query::query("INSERT INTO hub_space_membership (space_id, user_id, role, created_at) VALUES ($1, $2, $3, $4) ON CONFLICT (space_id, user_id) DO UPDATE SET role = $3")
                    .bind(space_id)
                    .bind(user_id)
                    .bind(role_from_wire(*role).as_str())
                    .bind(event.recorded_at_ms)
                    .execute(&mut **tx)
                    .await
                    .map_err(backend)?;
            }
            DirectoryEventBody::DocumentAnnounced { descriptor } => {
                sqlx_core::query::query("INSERT INTO hub_document_descriptor (space_id, document_id, descriptor, announced_at) VALUES ($1, $2, $3, $4) ON CONFLICT (space_id, document_id) DO NOTHING")
                    .bind(&descriptor.space_id)
                    .bind(&descriptor.document_id)
                    .bind(serde_json::Value::from(&descriptor.to_value()))
                    .bind(event.recorded_at_ms)
                    .execute(&mut **tx)
                    .await
                    .map_err(backend)?;
            }
            DirectoryEventBody::ArtifactCheckpointPublished { checkpoint } => {
                let payload = serde_json::Value::from(&checkpoint.to_value());
                sqlx_core::query::query("UPDATE hub_artifact_checkpoint SET active = FALSE WHERE space_id = $1 AND document_id = $2 AND checkpoint_id <> $3")
                    .bind(&checkpoint.scope.space_id)
                    .bind(&checkpoint.scope.document_id)
                    .bind(checkpoint.checkpoint_id.0.as_slice())
                    .execute(&mut **tx)
                    .await
                    .map_err(backend)?;
                sqlx_core::query::query(
                    "INSERT INTO hub_artifact_checkpoint (space_id, document_id, checkpoint_id, parent_checkpoint_id, descriptor_digest, frontier_document_id, head_edit_ordinal, head_edit_id, last_commit_seq, chain_hash, pack_sha256, pack_byte_length, spr_sha256, spr_byte_length, aggregate_sha256, published_at, event_seq, active, payload)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, TRUE, $18)
                     ON CONFLICT (space_id, document_id, checkpoint_id) DO UPDATE SET active = TRUE, payload = EXCLUDED.payload",
                )
                .bind(&checkpoint.scope.space_id)
                .bind(&checkpoint.scope.document_id)
                .bind(checkpoint.checkpoint_id.0.as_slice())
                .bind(checkpoint.parent_checkpoint_id.map(|id| id.0.to_vec()))
                .bind(checkpoint.descriptor_digest_v1.0.as_slice())
                .bind(&checkpoint.baseline_frontier.document_id)
                .bind(i64::try_from(checkpoint.baseline_frontier.head_edit_ordinal).map_err(backend)?)
                .bind(&checkpoint.baseline_frontier.head_edit_id)
                .bind(i64::try_from(checkpoint.baseline_frontier.last_commit_seq).map_err(backend)?)
                .bind(checkpoint.baseline_frontier.chain_hash.0.as_slice())
                .bind(checkpoint.pack.sha256.0.as_slice())
                .bind(i64::try_from(checkpoint.pack.byte_length).map_err(backend)?)
                .bind(checkpoint.spr.sha256.0.as_slice())
                .bind(i64::try_from(checkpoint.spr.byte_length).map_err(backend)?)
                .bind(checkpoint.aggregate_sha256.0.as_slice())
                .bind(i64::try_from(checkpoint.published_at_ms).map_err(backend)?)
                .bind(i64::try_from(event.seq).map_err(backend)?)
                .bind(payload)
                .execute(&mut **tx)
                .await
                .map_err(backend)?;
            }
            DirectoryEventBody::ArtifactRetentionAdvanced { retention } => {
                let payload = serde_json::Value::from(&retention.to_value());
                sqlx_core::query::query(
                    "INSERT INTO hub_artifact_retention (space_id, document_id, retained_checkpoint_id, floor_document_id, floor_head_edit_ordinal, floor_head_edit_id, floor_last_commit_seq, floor_chain_hash, checkpoint_lineage_head, event_seq, payload)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                     ON CONFLICT (space_id, document_id) DO UPDATE SET retained_checkpoint_id = EXCLUDED.retained_checkpoint_id, floor_document_id = EXCLUDED.floor_document_id, floor_head_edit_ordinal = EXCLUDED.floor_head_edit_ordinal, floor_head_edit_id = EXCLUDED.floor_head_edit_id, floor_last_commit_seq = EXCLUDED.floor_last_commit_seq, floor_chain_hash = EXCLUDED.floor_chain_hash, checkpoint_lineage_head = EXCLUDED.checkpoint_lineage_head, event_seq = EXCLUDED.event_seq, payload = EXCLUDED.payload",
                )
                .bind(&retention.scope.space_id)
                .bind(&retention.scope.document_id)
                .bind(retention.retained_checkpoint_id.0.as_slice())
                .bind(&retention.retained_floor.document_id)
                .bind(i64::try_from(retention.retained_floor.head_edit_ordinal).map_err(backend)?)
                .bind(&retention.retained_floor.head_edit_id)
                .bind(i64::try_from(retention.retained_floor.last_commit_seq).map_err(backend)?)
                .bind(retention.retained_floor.chain_hash.0.as_slice())
                .bind(retention.checkpoint_lineage_head.0.as_slice())
                .bind(i64::try_from(event.seq).map_err(backend)?)
                .bind(payload)
                .execute(&mut **tx)
                .await
                .map_err(backend)?;
            }
        }
        Ok(())
    }
    //#endregion 🔖️Projections
}

impl HubDirectory for PostgresDirectory {
    //#region ShareTokens
    async fn issue_share_token_as(&self, scope: &DocumentScope, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<IssuedShareToken> {
        let issued = prepare_share_token(scope, ttl_secs, now_ms())?;
        let audit = auth_audit(issued.record.created_at, "share-issued", Some(&issued.record.id), None, actor_user_id, None, "success", None, correlation_id, "server")?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        sqlx_core::query::query("INSERT INTO hub_share_grant (id, selector, secret_digest, space_id, document_id, created_at, expires_at, revoked_at, revoked_reason) VALUES ($1, $2, $3, $4, $5, $6, $7, NULL, NULL)")
            .bind(&issued.record.id)
            .bind(&issued.record.selector)
            .bind(issued.record.secret_digest.as_slice())
            .bind(&scope.space_id)
            .bind(&scope.document_id)
            .bind(issued.record.created_at)
            .bind(issued.record.expires_at)
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
        insert_auth_audit(&mut tx, &audit).await?;
        tx.commit().await.map_err(backend)?;
        Ok(issued)
    }

    async fn revoke_share_token_as(&self, scope: &DocumentScope, share_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()> {
        validate_bounded_auth_text(reason, "share revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let audit = auth_audit(revoked_at, "share-revoked", Some(share_id), None, actor_user_id, None, "success", Some(reason), correlation_id, "server")?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let result = sqlx_core::query::query("UPDATE hub_share_grant SET revoked_at = $4, revoked_reason = $5 WHERE id = $1 AND space_id = $2 AND document_id = $3 AND revoked_at IS NULL")
            .bind(share_id)
            .bind(&scope.space_id)
            .bind(&scope.document_id)
            .bind(revoked_at)
            .bind(reason)
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
        if result.rows_affected() == 0 {
            Err(DirectoryError::NotFound(format!("share grant {share_id}")))
        } else {
            insert_auth_audit(&mut tx, &audit).await?;
            tx.commit().await.map_err(backend)?;
            Ok(())
        }
    }

    async fn authenticate_share(&self, scope: &DocumentScope, capability: &ShareCapability) -> DirectoryResult<bool> {
        Ok(self.authenticate_share_binding(scope, capability).await?.is_some())
    }

    async fn authenticate_share_binding(&self, scope: &DocumentScope, capability: &ShareCapability) -> DirectoryResult<Option<ShareTokenRecord>> {
        let row: Option<ShareRow> =
            sqlx_core::query_as::query_as("SELECT id, selector, secret_digest, space_id, document_id, created_at, expires_at, revoked_at, revoked_reason FROM hub_share_grant WHERE space_id = $1 AND document_id = $2 AND selector = $3")
                .bind(&scope.space_id)
                .bind(&scope.document_id)
                .bind(capability.selector())
                .fetch_optional(&self.pool)
                .await
                .map_err(backend)?;
        let record = row.map(share_from_row).transpose()?;
        Ok(record.filter(|record| active_capability(&record.selector, &record.secret_digest, record.expires_at, record.revoked_at, capability.selector(), &capability.secret_digest(), now_ms())))
    }

    async fn socket_share_binding(&self, share_id: &str, selector: &str, scope: &DocumentScope, now_ms: i64) -> DirectoryResult<SocketShareBindingStatus> {
        let row: Option<ShareRow> =
            sqlx_core::query_as::query_as("SELECT id, selector, secret_digest, space_id, document_id, created_at, expires_at, revoked_at, revoked_reason FROM hub_share_grant WHERE id = $1 AND selector = $2 AND space_id = $3 AND document_id = $4")
                .bind(share_id)
                .bind(selector)
                .bind(&scope.space_id)
                .bind(&scope.document_id)
                .fetch_optional(&self.pool)
                .await
                .map_err(backend)?;
        Ok(match row.map(share_from_row).transpose()? {
            None => SocketShareBindingStatus::Unavailable,
            Some(record) if record.revoked_at.is_some() => SocketShareBindingStatus::Revoked,
            Some(record) if record.expires_at <= now_ms => SocketShareBindingStatus::Expired,
            Some(record) => SocketShareBindingStatus::Active { expires_at_ms: record.expires_at },
        })
    }
    //#endregion

    //#region Users
    async fn create_user(&self, email: &str, display_name: &str, password_hash: Option<&str>, sso_subject: Option<&str>, sso_provider: Option<&str>) -> DirectoryResult<UserRecord> {
        let id = time_ordered_id();
        let created_at = now_ms();
        sqlx_core::query::query("INSERT INTO hub_user (id, email, display_name, password_hash, sso_subject, sso_provider, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(&id)
            .bind(email)
            .bind(display_name)
            .bind(password_hash)
            .bind(sso_subject)
            .bind(sso_provider)
            .bind(created_at)
            .execute(&self.pool)
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
        let row: Option<(String, String, String, Option<String>, Option<String>, Option<String>, i64)> =
            sqlx_core::query_as::query_as("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE id = $1").bind(user_id).fetch_optional(&self.pool).await.map_err(backend)?;
        Ok(row.map(user_from_row))
    }

    async fn get_user_by_email(&self, email: &str) -> DirectoryResult<Option<UserRecord>> {
        let row: Option<(String, String, String, Option<String>, Option<String>, Option<String>, i64)> =
            sqlx_core::query_as::query_as("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE email = $1").bind(email).fetch_optional(&self.pool).await.map_err(backend)?;
        Ok(row.map(user_from_row))
    }

    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> DirectoryResult<Option<UserRecord>> {
        let row: Option<(String, String, String, Option<String>, Option<String>, Option<String>, i64)> =
            sqlx_core::query_as::query_as("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE sso_provider = $1 AND sso_subject = $2")
                .bind(provider)
                .bind(subject)
                .fetch_optional(&self.pool)
                .await
                .map_err(backend)?;
        Ok(row.map(user_from_row))
    }

    async fn list_users(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<UserRecord>> {
        let rows: Vec<(String, String, String, Option<String>, Option<String>, Option<String>, i64)> =
            sqlx_core::query_as::query_as("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user ORDER BY created_at LIMIT $1 OFFSET $2")
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await
                .map_err(backend)?;
        Ok(rows.into_iter().map(user_from_row).collect())
    }

    async fn admin_overview_counts(&self) -> DirectoryResult<AdminDirectoryOverviewCounts> {
        let (spaces, users, connections): (i64, i64, i64) =
            sqlx_core::query_as::query_as("SELECT (SELECT COUNT(*) FROM hub_space), (SELECT COUNT(*) FROM hub_user), (SELECT COUNT(*) FROM hub_sync_session WHERE disconnected_at IS NULL)").fetch_one(&self.pool).await.map_err(backend)?;
        Ok(AdminDirectoryOverviewCounts { spaces: u64::try_from(spaces).map_err(backend)?, users: u64::try_from(users).map_err(backend)?, connections: u64::try_from(connections).map_err(backend)? })
    }
    //#endregion

    //#region Spaces
    async fn get_space(&self, space_id: &str) -> DirectoryResult<Option<SpaceRecord>> {
        let row: Option<(String, String, String, i64, String, String)> =
            sqlx_core::query_as::query_as("SELECT id, name, owner_user_id, created_at, kind, visibility FROM hub_space WHERE id = $1").bind(space_id).fetch_optional(&self.pool).await.map_err(backend)?;
        Ok(row.map(|(id, name, owner_user_id, created_at, kind, visibility)| SpaceRecord { id, name, owner_user_id, created_at, kind, visibility }))
    }

    async fn list_spaces_for_user(&self, user_id: &str) -> DirectoryResult<Vec<(SpaceRecord, SpaceRole)>> {
        let rows: Vec<(String, String, String, i64, String, String, String)> = sqlx_core::query_as::query_as(
            "SELECT s.id, s.name, s.owner_user_id, s.created_at, s.kind, s.visibility, m.role FROM hub_space s
             JOIN hub_space_membership m ON m.space_id = s.id WHERE m.user_id = $1 ORDER BY s.created_at",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        Ok(rows.into_iter().filter_map(|(id, name, owner_user_id, created_at, kind, visibility, role)| SpaceRole::parse(&role).map(|role| (SpaceRecord { id, name, owner_user_id, created_at, kind, visibility }, role))).collect())
    }

    async fn list_spaces(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<SpaceRecord>> {
        let rows: Vec<(String, String, String, i64, String, String)> =
            sqlx_core::query_as::query_as("SELECT id, name, owner_user_id, created_at, kind, visibility FROM hub_space ORDER BY created_at LIMIT $1 OFFSET $2").bind(limit).bind(offset).fetch_all(&self.pool).await.map_err(backend)?;
        Ok(rows.into_iter().map(|(id, name, owner_user_id, created_at, kind, visibility)| SpaceRecord { id, name, owner_user_id, created_at, kind, visibility }).collect())
    }

    async fn list_admin_space_summaries_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<AdminSpaceSummaryRecord>> {
        if limit == 0 || limit > super::ADMIN_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("administrator space page limit must be 1..={}", super::ADMIN_PAGE_FETCH_MAX)));
        }
        let rows: Vec<(String, String, String, i64, String, String, i64, i64, i64, i64)> = sqlx_core::query_as::query_as(
            "SELECT s.id, s.name, s.owner_user_id, s.created_at, s.kind, s.visibility,
                    (SELECT COUNT(*) FROM hub_space_membership m WHERE m.space_id = s.id),
                    (SELECT COUNT(*) FROM hub_document_descriptor d WHERE d.space_id = s.id),
                    (SELECT COUNT(*) FROM hub_sync_session y WHERE y.space_id = s.id AND y.disconnected_at IS NULL),
                    COALESCE((SELECT MAX(e.recorded_at) FROM hub_directory_event e WHERE e.space_id = s.id), s.created_at)
             FROM hub_space s WHERE ($1::TEXT IS NULL OR s.id = $1) ORDER BY s.id LIMIT $2 OFFSET $3",
        )
        .bind(space_id)
        .bind(i64::try_from(limit).map_err(backend)?)
        .bind(i64::try_from(offset).map_err(backend)?)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        rows.into_iter()
            .map(|(id, name, owner_user_id, created_at, kind, visibility, member_count, document_count, active_connections, updated_at)| {
                Ok(AdminSpaceSummaryRecord {
                    space: SpaceRecord { id, name, owner_user_id, created_at, kind, visibility },
                    member_count: u64::try_from(member_count).map_err(backend)?,
                    document_count: u64::try_from(document_count).map_err(backend)?,
                    active_connections: u64::try_from(active_connections).map_err(backend)?,
                    updated_at,
                })
            })
            .collect()
    }

    async fn list_admin_space_members_page(&self, space_id: &str, offset: usize, limit: usize) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>> {
        if limit == 0 || limit > super::ADMIN_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("administrator member page limit must be 1..={}", super::ADMIN_PAGE_FETCH_MAX)));
        }
        let rows: Vec<(String, String, String, Option<String>, Option<String>, Option<String>, i64, String)> = sqlx_core::query_as::query_as(
            "SELECT u.id, u.email, u.display_name, u.password_hash, u.sso_subject, u.sso_provider, u.created_at, m.role
             FROM hub_space_membership m JOIN hub_user u ON u.id = m.user_id
             WHERE m.space_id = $1 ORDER BY u.id LIMIT $2 OFFSET $3",
        )
        .bind(space_id)
        .bind(i64::try_from(limit).map_err(backend)?)
        .bind(i64::try_from(offset).map_err(backend)?)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        rows.into_iter()
            .map(|(id, email, display_name, password_hash, sso_subject, sso_provider, created_at, role)| {
                SpaceRole::parse(&role).map(|role| (UserRecord { id, email, display_name, password_hash, sso_subject, sso_provider, created_at }, role)).ok_or_else(|| DirectoryError::Backend("stored member role is invalid".into()))
            })
            .collect()
    }

    async fn list_members(&self, space_id: &str) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>> {
        let rows: Vec<(String, String, String, Option<String>, Option<String>, Option<String>, i64, String)> = sqlx_core::query_as::query_as(
            "SELECT u.id, u.email, u.display_name, u.password_hash, u.sso_subject, u.sso_provider, u.created_at, m.role
             FROM hub_space_membership m JOIN hub_user u ON u.id = m.user_id WHERE m.space_id = $1 ORDER BY m.created_at",
        )
        .bind(space_id)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        Ok(rows
            .into_iter()
            .filter_map(|(id, email, display_name, password_hash, sso_subject, sso_provider, created_at, role)| SpaceRole::parse(&role).map(|role| (UserRecord { id, email, display_name, password_hash, sso_subject, sso_provider, created_at }, role)))
            .collect())
    }

    async fn get_role(&self, space_id: &str, user_id: &str) -> DirectoryResult<Option<SpaceRole>> {
        let row: Option<(String,)> = sqlx_core::query_as::query_as("SELECT role FROM hub_space_membership WHERE space_id = $1 AND user_id = $2").bind(space_id).bind(user_id).fetch_optional(&self.pool).await.map_err(backend)?;
        Ok(row.and_then(|(role,)| SpaceRole::parse(&role)))
    }

    async fn get_document_descriptor(&self, scope: &DocumentScope) -> DirectoryResult<Option<DocumentDescriptor>> {
        let row: Option<(serde_json::Value,)> =
            sqlx_core::query_as::query_as("SELECT descriptor FROM hub_document_descriptor WHERE space_id = $1 AND document_id = $2").bind(&scope.space_id).bind(&scope.document_id).fetch_optional(&self.pool).await.map_err(backend)?;
        row.map(|(value,)| DocumentDescriptor::from_value(DslValue::from(value)).map_err(backend)).transpose()
    }

    async fn list_document_descriptors(&self, space_id: &str) -> DirectoryResult<Vec<DocumentDescriptor>> {
        let rows: Vec<(serde_json::Value,)> = sqlx_core::query_as::query_as("SELECT descriptor FROM hub_document_descriptor WHERE space_id = $1 ORDER BY document_id").bind(space_id).fetch_all(&self.pool).await.map_err(backend)?;
        rows.into_iter().map(|(value,)| DocumentDescriptor::from_value(DslValue::from(value)).map_err(backend)).collect()
    }

    async fn list_document_descriptors_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<DocumentDescriptor>> {
        if limit == 0 || limit > super::ADMIN_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("administrator document page limit must be 1..={}", super::ADMIN_PAGE_FETCH_MAX)));
        }
        let limit = i64::try_from(limit).map_err(backend)?;
        let offset = i64::try_from(offset).map_err(backend)?;
        let rows: Vec<(serde_json::Value,)> = match space_id {
            Some(space_id) => sqlx_core::query_as::query_as("SELECT descriptor FROM hub_document_descriptor WHERE space_id = $1 ORDER BY space_id, document_id LIMIT $2 OFFSET $3")
                .bind(space_id)
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await
                .map_err(backend)?,
            None => sqlx_core::query_as::query_as("SELECT descriptor FROM hub_document_descriptor ORDER BY space_id, document_id LIMIT $1 OFFSET $2").bind(limit).bind(offset).fetch_all(&self.pool).await.map_err(backend)?,
        };
        rows.into_iter().map(|(value,)| DocumentDescriptor::from_value(DslValue::from(value)).map_err(backend)).collect()
    }

    async fn get_artifact_checkpoint(&self, scope: &DocumentScope, checkpoint_id: ArtifactHash) -> DirectoryResult<Option<PublishedArtifactCheckpoint>> {
        let row: Option<(serde_json::Value,)> = sqlx_core::query_as::query_as("SELECT payload FROM hub_artifact_checkpoint WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3")
            .bind(&scope.space_id)
            .bind(&scope.document_id)
            .bind(checkpoint_id.0.as_slice())
            .fetch_optional(&self.pool)
            .await
            .map_err(backend)?;
        row.map(|(value,)| PublishedArtifactCheckpoint::from_value(DslValue::from(value)).map_err(backend)).transpose()
    }

    async fn get_verified_artifact_checkpoint(&self, scope: &DocumentScope, checkpoint_id: ArtifactHash) -> DirectoryResult<Option<ArtifactCheckpoint>> {
        let row: Option<(serde_json::Value,)> = sqlx_core::query_as::query_as("SELECT payload FROM hub_artifact_checkpoint_private WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3")
            .bind(&scope.space_id)
            .bind(&scope.document_id)
            .bind(checkpoint_id.0.as_slice())
            .fetch_optional(&self.pool)
            .await
            .map_err(backend)?;
        row.map(|(value,)| ArtifactCheckpoint::from_value(DslValue::from(value)).map_err(backend)).transpose()
    }

    async fn get_active_artifact_checkpoint(&self, scope: &DocumentScope) -> DirectoryResult<Option<PublishedArtifactCheckpoint>> {
        let row: Option<(serde_json::Value,)> =
            sqlx_core::query_as::query_as("SELECT payload FROM hub_artifact_checkpoint WHERE space_id = $1 AND document_id = $2 AND active").bind(&scope.space_id).bind(&scope.document_id).fetch_optional(&self.pool).await.map_err(backend)?;
        row.map(|(value,)| PublishedArtifactCheckpoint::from_value(DslValue::from(value)).map_err(backend)).transpose()
    }

    async fn get_artifact_retention(&self, scope: &DocumentScope) -> DirectoryResult<Option<ArtifactRetention>> {
        let row: Option<(serde_json::Value,)> =
            sqlx_core::query_as::query_as("SELECT payload FROM hub_artifact_retention WHERE space_id = $1 AND document_id = $2").bind(&scope.space_id).bind(&scope.document_id).fetch_optional(&self.pool).await.map_err(backend)?;
        row.map(|(value,)| ArtifactRetention::from_value(DslValue::from(value)).map_err(backend)).transpose()
    }

    async fn artifact_checkpoint_count(&self, scope: &DocumentScope) -> DirectoryResult<u64> {
        let row: (i64,) = sqlx_core::query_as::query_as("SELECT COUNT(*) FROM hub_artifact_checkpoint WHERE space_id = $1 AND document_id = $2").bind(&scope.space_id).bind(&scope.document_id).fetch_one(&self.pool).await.map_err(backend)?;
        u64::try_from(row.0).map_err(backend)
    }

    async fn list_artifact_checkpoint_lineage(&self, scope: &DocumentScope, limit: usize) -> DirectoryResult<Vec<PublishedArtifactCheckpoint>> {
        if limit == 0 || limit as u64 > ARTIFACT_CHECKPOINT_LINEAGE_MAX {
            return Err(DirectoryError::Conflict(format!("artifact checkpoint lineage limit must be 1..={ARTIFACT_CHECKPOINT_LINEAGE_MAX}")));
        }
        let rows: Vec<(serde_json::Value,)> = sqlx_core::query_as::query_as("SELECT payload FROM hub_artifact_checkpoint WHERE space_id = $1 AND document_id = $2 ORDER BY event_seq LIMIT $3")
            .bind(&scope.space_id)
            .bind(&scope.document_id)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(backend)?;
        rows.into_iter().map(|(value,)| PublishedArtifactCheckpoint::from_value(DslValue::from(value)).map_err(backend)).collect()
    }
    //#endregion

    //#region AuthSessions
    async fn issue_auth_session(&self, issue: &AuthSessionIssue) -> DirectoryResult<IssuedAuthSession> {
        let issued = prepare_auth_session(issue, now_ms())?;
        let audit = auth_audit(issued.record.issued_at, "session-issued", Some(&issued.record.id), Some(&issued.record.user_id), None, Some(&issued.record.identity_provider), "success", None, &issue.correlation_id, &issue.peer_class)?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        sqlx_core::query::query("INSERT INTO hub_auth_session (id, selector, secret_digest, user_id, identity_provider, identity_subject_digest, issued_at, expires_at, revoked_at, revoked_reason, authorization_generation, device_instance_id, session_kind) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NULL, NULL, $9, $10, $11)")
            .bind(&issued.record.id)
            .bind(&issued.record.selector)
            .bind(issued.record.secret_digest.as_slice())
            .bind(&issued.record.user_id)
            .bind(&issued.record.identity_provider)
            .bind(issued.record.identity_subject_digest.as_slice())
            .bind(issued.record.issued_at)
            .bind(issued.record.expires_at)
            .bind(i64::try_from(issued.record.authorization_generation).map_err(backend)?)
            .bind(&issued.record.device_instance_id)
            .bind(issued.record.session_kind.as_str())
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
        insert_auth_audit(&mut tx, &audit).await?;
        tx.commit().await.map_err(backend)?;
        Ok(issued)
    }

    async fn authenticate_session(&self, capability: &SessionCapability) -> DirectoryResult<Option<AuthSessionRecord>> {
        let row: Option<AuthSessionRow> = sqlx_core::query_as::query_as(
            "SELECT id, selector, secret_digest, user_id, identity_provider, identity_subject_digest, issued_at, expires_at, revoked_at, revoked_reason, authorization_generation, device_instance_id, session_kind FROM hub_auth_session WHERE selector = $1",
        )
        .bind(capability.selector())
        .fetch_optional(&self.pool)
        .await
        .map_err(backend)?;
        let record = row.map(auth_session_from_row).transpose()?;
        Ok(record.filter(|record| active_capability(&record.selector, &record.secret_digest, record.expires_at, record.revoked_at, capability.selector(), &capability.secret_digest(), now_ms())))
    }

    async fn socket_session_binding(&self, session_id: &str, user_id: &str, authorization_generation: u64, space_id: Option<&str>, now_ms: i64) -> DirectoryResult<SocketSessionBindingStatus> {
        let row: Option<AuthSessionRow> = sqlx_core::query_as::query_as(
            "SELECT id, selector, secret_digest, user_id, identity_provider, identity_subject_digest, issued_at, expires_at, revoked_at, revoked_reason, authorization_generation, device_instance_id, session_kind FROM hub_auth_session WHERE id = $1",
        )
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(backend)?;
        let Some(record) = row.map(auth_session_from_row).transpose()? else { return Ok(SocketSessionBindingStatus::Unavailable) };
        if record.user_id != user_id {
            return Ok(SocketSessionBindingStatus::Unavailable);
        }
        if record.revoked_at.is_some() || record.authorization_generation != authorization_generation {
            return Ok(SocketSessionBindingStatus::Revoked);
        }
        if record.expires_at <= now_ms {
            return Ok(SocketSessionBindingStatus::Expired);
        }
        let role = match space_id {
            Some(space_id) => {
                let row: Option<(String,)> = sqlx_core::query_as::query_as("SELECT role FROM hub_space_membership WHERE space_id = $1 AND user_id = $2").bind(space_id).bind(user_id).fetch_optional(&self.pool).await.map_err(backend)?;
                row.and_then(|(role,)| SpaceRole::parse(&role))
            }
            None => None,
        };
        if space_id.is_some() && role.is_none() {
            return Ok(SocketSessionBindingStatus::MembershipLost);
        }
        Ok(SocketSessionBindingStatus::Active { role, expires_at_ms: record.expires_at })
    }

    async fn revoke_auth_session(&self, id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Option<RevokedAuthSession>> {
        validate_bounded_auth_text(reason, "session revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let row: Option<(String, String, i64, String)> = sqlx_core::query_as::query_as(
            "UPDATE hub_auth_session SET revoked_at = $2, revoked_reason = $3, authorization_generation = authorization_generation + 1 WHERE id = $1 AND revoked_at IS NULL RETURNING id, user_id, authorization_generation, identity_provider",
        )
        .bind(id)
        .bind(revoked_at)
        .bind(reason)
        .fetch_optional(&mut *tx)
        .await
        .map_err(backend)?;
        let Some((id, user_id, generation, provider)) = row else { return Ok(None) };
        let audit = auth_audit(revoked_at, "session-revoked", Some(&id), Some(&user_id), actor_user_id, Some(&provider), "success", Some(reason), correlation_id, "server")?;
        insert_auth_audit(&mut tx, &audit).await?;
        tx.commit().await.map_err(backend)?;
        Ok(Some(RevokedAuthSession { id, authorization_generation: u64::try_from(generation).map_err(backend)?, revoked_at }))
    }

    async fn revoke_auth_sessions_for_user(&self, user_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>> {
        self.revoke_auth_sessions_matching(user_id, None, reason, actor_user_id, correlation_id).await
    }

    async fn revoke_auth_sessions_for_identity(&self, provider: &str, subject_digest: [u8; 32], reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>> {
        self.revoke_auth_sessions_matching(provider, Some(subject_digest), reason, actor_user_id, correlation_id).await
    }

    async fn list_auth_audit(&self, limit: usize, offset: usize) -> DirectoryResult<Vec<AuthAuditRecord>> {
        if limit == 0 || limit > AUTH_AUDIT_PAGE_MAX {
            return Err(DirectoryError::Conflict(format!("auth audit limit must be 1..={AUTH_AUDIT_PAGE_MAX}")));
        }
        let rows: Vec<AuthAuditRow> =
            sqlx_core::query_as::query_as("SELECT id, occurred_at, event_kind, auth_session_id, target_user_id, actor_user_id, provider, outcome_code, reason_code, correlation_id, peer_class FROM hub_auth_audit ORDER BY sequence LIMIT $1 OFFSET $2")
                .bind(i64::try_from(limit).map_err(backend)?)
                .bind(i64::try_from(offset).map_err(backend)?)
                .fetch_all(&self.pool)
                .await
                .map_err(backend)?;
        Ok(rows.into_iter().map(auth_audit_from_row).collect())
    }
    //#endregion

    //#region AdminOperations
    async fn append_admin_operation_audit(&self, fact: &NewAdminOperationAuditRecord) -> DirectoryResult<AdminOperationAuditRecord> {
        validate_admin_operation_audit(fact)?;
        let terminal = fact.phase != "accepted";
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let existing: Vec<AdminOperationAuditRow> = sqlx_core::query_as::query_as(
            "SELECT sequence, request_id, intent_digest, operation_id, occurred_at, phase, intent_kind, target_kind, target_id, principal_user_id, principal_session_id, principal_generation, correlation_id, event_seq_first, event_seq_last, outcome_code, reason_code FROM hub_admin_operation_audit WHERE request_id = $1 ORDER BY sequence LIMIT 2 FOR UPDATE",
        )
        .bind(&fact.request_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(backend)?;
        let existing = existing.into_iter().map(admin_operation_audit_from_row).collect::<DirectoryResult<Vec<_>>>()?;
        if !terminal {
            if let Some(established) = existing.first() {
                let outcome = if same_admin_operation_request(&established.fact, fact) { Ok(established.clone()) } else { Err(DirectoryError::Conflict("admin request id was reused for a different intent".into())) };
                tx.rollback().await.map_err(backend)?;
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
                tx.rollback().await.map_err(backend)?;
                return outcome;
            }
        }
        let inserted: Option<(i64,)> = sqlx_core::query_as::query_as(
            "INSERT INTO hub_admin_operation_audit (request_id, intent_digest, operation_id, occurred_at, phase, terminal, intent_kind, target_kind, target_id, principal_user_id, principal_session_id, principal_generation, correlation_id, event_seq_first, event_seq_last, outcome_code, reason_code) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17) ON CONFLICT (request_id, terminal) DO NOTHING RETURNING sequence",
        )
        .bind(&fact.request_id)
        .bind(&fact.intent_digest)
        .bind(&fact.operation_id)
        .bind(fact.occurred_at)
        .bind(&fact.phase)
        .bind(terminal)
        .bind(&fact.intent_kind)
        .bind(&fact.target_kind)
        .bind(&fact.target_id)
        .bind(&fact.principal_user_id)
        .bind(&fact.principal_session_id)
        .bind(i64::try_from(fact.principal_generation).map_err(backend)?)
        .bind(&fact.correlation_id)
        .bind(fact.event_seq_first.map(i64::try_from).transpose().map_err(backend)?)
        .bind(fact.event_seq_last.map(i64::try_from).transpose().map_err(backend)?)
        .bind(&fact.outcome_code)
        .bind(&fact.reason_code)
        .fetch_optional(&mut *tx)
        .await
        .map_err(backend)?;
        match inserted {
            Some((sequence,)) => {
                tx.commit().await.map_err(backend)?;
                Ok(AdminOperationAuditRecord { sequence: u64::try_from(sequence).map_err(backend)?, fact: fact.clone() })
            }
            None => {
                tx.rollback().await.map_err(backend)?;
                let established =
                    self.admin_operation_audit_for_request(&fact.request_id).await?.into_iter().find(|row| (row.fact.phase != "accepted") == terminal).ok_or_else(|| DirectoryError::Conflict("admin request race has no established receipt".into()))?;
                if same_admin_operation_request(&established.fact, fact) {
                    Ok(established)
                } else {
                    Err(DirectoryError::Conflict("admin request id race changed intent".into()))
                }
            }
        }
    }

    async fn admin_operation_audit_for_request(&self, request_id: &str) -> DirectoryResult<Vec<AdminOperationAuditRecord>> {
        validate_bounded_auth_text(request_id, "admin request id", AUTH_TEXT_MAX_BYTES)?;
        let rows: Vec<AdminOperationAuditRow> = sqlx_core::query_as::query_as(
            "SELECT sequence, request_id, intent_digest, operation_id, occurred_at, phase, intent_kind, target_kind, target_id, principal_user_id, principal_session_id, principal_generation, correlation_id, event_seq_first, event_seq_last, outcome_code, reason_code FROM hub_admin_operation_audit WHERE request_id = $1 ORDER BY sequence LIMIT 2",
        )
        .bind(request_id)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        rows.into_iter().map(admin_operation_audit_from_row).collect()
    }

    async fn admin_operation_audit_for_operation(&self, operation_id: &str) -> DirectoryResult<Vec<AdminOperationAuditRecord>> {
        validate_bounded_auth_text(operation_id, "admin operation id", AUTH_TEXT_MAX_BYTES)?;
        let rows: Vec<AdminOperationAuditRow> = sqlx_core::query_as::query_as(
            "SELECT sequence, request_id, intent_digest, operation_id, occurred_at, phase, intent_kind, target_kind, target_id, principal_user_id, principal_session_id, principal_generation, correlation_id, event_seq_first, event_seq_last, outcome_code, reason_code FROM hub_admin_operation_audit WHERE operation_id = $1 ORDER BY sequence LIMIT 2",
        )
        .bind(operation_id)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        rows.into_iter().map(admin_operation_audit_from_row).collect()
    }

    async fn list_admin_operation_audit(&self, after_sequence: u64, limit: usize) -> DirectoryResult<Vec<AdminOperationAuditRecord>> {
        if limit == 0 || limit > ADMIN_PAGE_MAX {
            return Err(DirectoryError::Conflict(format!("admin audit limit must be 1..={ADMIN_PAGE_MAX}")));
        }
        let rows: Vec<AdminOperationAuditRow> = sqlx_core::query_as::query_as(
            "SELECT sequence, request_id, intent_digest, operation_id, occurred_at, phase, intent_kind, target_kind, target_id, principal_user_id, principal_session_id, principal_generation, correlation_id, event_seq_first, event_seq_last, outcome_code, reason_code FROM hub_admin_operation_audit WHERE sequence > $1 ORDER BY sequence LIMIT $2",
        )
        .bind(i64::try_from(after_sequence).map_err(backend)?)
        .bind(i64::try_from(limit).map_err(backend)?)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        rows.into_iter().map(admin_operation_audit_from_row).collect()
    }
    //#endregion

    //#region Invites
    async fn issue_invite_as(&self, space_id: &str, role: SpaceRole, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<IssuedInvite> {
        let issued = prepare_invite(space_id, role, ttl_secs, now_ms())?;
        let audit = auth_audit(issued.record.created_at, "invite-issued", Some(&issued.record.id), None, actor_user_id, None, "success", None, correlation_id, "server")?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        sqlx_core::query::query("INSERT INTO hub_space_invite (id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at) VALUES ($1, $2, $3, $4, $5, $6, $7, NULL, NULL, NULL)")
            .bind(&issued.record.id)
            .bind(&issued.record.selector)
            .bind(issued.record.secret_digest.as_slice())
            .bind(space_id)
            .bind(role.as_str())
            .bind(issued.record.created_at)
            .bind(issued.record.expires_at)
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
        insert_auth_audit(&mut tx, &audit).await?;
        tx.commit().await.map_err(backend)?;
        Ok(issued)
    }

    async fn authenticate_invite(&self, capability: &InviteCapability) -> DirectoryResult<Option<InviteRecord>> {
        let row: Option<InviteRow> = sqlx_core::query_as::query_as("SELECT id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at FROM hub_space_invite WHERE selector = $1")
            .bind(capability.selector())
            .fetch_optional(&self.pool)
            .await
            .map_err(backend)?;
        let record = row.map(invite_from_row).transpose()?;
        Ok(record.filter(|record| record.accepted_at.is_none() && active_capability(&record.selector, &record.secret_digest, record.expires_at, record.revoked_at, capability.selector(), &capability.secret_digest(), now_ms())))
    }

    async fn revoke_invite_as(&self, invite_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()> {
        validate_bounded_auth_text(reason, "invite revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let audit = auth_audit(revoked_at, "invite-revoked", Some(invite_id), None, actor_user_id, None, "success", Some(reason), correlation_id, "server")?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let result = sqlx_core::query::query("UPDATE hub_space_invite SET revoked_at = $2, revoked_reason = $3 WHERE id = $1 AND revoked_at IS NULL").bind(invite_id).bind(revoked_at).bind(reason).execute(&mut *tx).await.map_err(backend)?;
        if result.rows_affected() == 0 {
            return Err(DirectoryError::NotFound(format!("invite {invite_id}")));
        }
        insert_auth_audit(&mut tx, &audit).await?;
        tx.commit().await.map_err(backend)?;
        Ok(())
    }

    async fn list_invites(&self, space_id: &str) -> DirectoryResult<Vec<InviteRecord>> {
        let rows: Vec<InviteRow> =
            sqlx_core::query_as::query_as("SELECT id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at FROM hub_space_invite WHERE space_id = $1 ORDER BY created_at DESC")
                .bind(space_id)
                .fetch_all(&self.pool)
                .await
                .map_err(backend)?;
        rows.into_iter().map(invite_from_row).collect()
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
        let role_str = space_role.map(|r| r.as_str());
        sqlx_core::query::query(
            "INSERT INTO hub_sync_session (id, auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, authenticated_email, space_role, client_label, connected_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
        )
        .bind(&id)
        .bind(auth_session_id)
        .bind(i64::try_from(authorization_generation).map_err(backend)?)
        .bind(actor_id)
        .bind(space_id)
        .bind(document_id)
        .bind(surface)
        .bind(user_id)
        .bind(authenticated_email)
        .bind(role_str)
        .bind(client_label)
        .bind(connected_at)
        .execute(&self.pool)
        .await
        .map_err(backend)?;
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
        sqlx_core::query::query("UPDATE hub_sync_session SET disconnected_at = $2 WHERE id = $1").bind(sync_session_id).bind(now_ms()).execute(&self.pool).await.map_err(backend)?;
        Ok(())
    }

    async fn list_sync_sessions_for_document(&self, document_id: &str) -> DirectoryResult<Vec<SyncSessionRecord>> {
        let rows: Vec<SyncSessionRow> = sqlx_core::query_as::query_as(
            "SELECT id, auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, authenticated_email, space_role, client_label, connected_at, disconnected_at FROM hub_sync_session
             WHERE document_id = $1 ORDER BY connected_at DESC",
        )
        .bind(document_id)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        Ok(rows.into_iter().map(sync_session_from_row).collect())
    }

    async fn list_active_sync_sessions_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<SyncSessionRecord>> {
        if limit == 0 || limit > super::ACTIVE_SYNC_SESSION_READ_MAX {
            return Err(DirectoryError::Conflict(format!("active sync-session limit must be 1..={}", super::ACTIVE_SYNC_SESSION_READ_MAX)));
        }
        let limit = i64::try_from(limit).map_err(backend)?;
        let offset = i64::try_from(offset).map_err(backend)?;
        let rows: Vec<SyncSessionRow> = match space_id {
            Some(space_id) => sqlx_core::query_as::query_as(
                "SELECT id, auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, authenticated_email, space_role, client_label, connected_at, disconnected_at FROM hub_sync_session
                 WHERE space_id = $1 AND disconnected_at IS NULL ORDER BY connected_at DESC, id ASC LIMIT $2 OFFSET $3",
            )
            .bind(space_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(backend)?,
            None => sqlx_core::query_as::query_as(
                "SELECT id, auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, authenticated_email, space_role, client_label, connected_at, disconnected_at FROM hub_sync_session
                 WHERE disconnected_at IS NULL ORDER BY connected_at DESC, id ASC LIMIT $1 OFFSET $2",
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(backend)?,
        };
        Ok(rows.into_iter().map(sync_session_from_row).collect())
    }

    async fn close_all_sync_sessions(&self) -> DirectoryResult<()> {
        sqlx_core::query::query("UPDATE hub_sync_session SET disconnected_at = $1 WHERE disconnected_at IS NULL").bind(now_ms()).execute(&self.pool).await.map_err(backend)?;
        Ok(())
    }
    //#endregion

    async fn reserve_artifact_cas(&self, plan: &ArtifactCasOwnershipPlanV1, expires_at_ms: u64, now_ms: u64) -> DirectoryResult<ArtifactCasReservation> {
        let encoded = encode_artifact_cas_ownership_v1(plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        if expires_at_ms <= now_ms || expires_at_ms.checked_sub(now_ms).is_none_or(|ttl| ttl > ARTIFACT_CAS_RESERVATION_MAX_TTL_MS) {
            return Err(DirectoryError::Conflict(format!("artifact CAS reservation ttl must be 1..={ARTIFACT_CAS_RESERVATION_MAX_TTL_MS} milliseconds")));
        }
        let now = i64::try_from(now_ms).map_err(backend)?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let (coordinator_id, physical_epoch) = cas_reservation_barrier(&mut tx, &plan.scope.space_id, now).await?;
        let historical: Option<(Vec<u8>,)> = sqlx_core::query_as::query_as("SELECT plan FROM hub_artifact_cas_ledger_journal WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3 AND plan IS NOT NULL ORDER BY generation LIMIT 1")
            .bind(&plan.scope.space_id)
            .bind(&plan.scope.document_id)
            .bind(plan.checkpoint_id.0.as_slice())
            .fetch_optional(&mut *tx)
            .await
            .map_err(backend)?;
        if historical.as_ref().is_some_and(|(value,)| value != &encoded) {
            return Err(DirectoryError::Conflict("artifact CAS checkpoint identity names a different ownership plan".into()));
        }
        let published: Option<(i64, i64, Vec<u8>)> = sqlx_core::query_as::query_as("SELECT generation, write_epoch, plan FROM hub_artifact_cas_reference WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3")
            .bind(&plan.scope.space_id)
            .bind(&plan.scope.document_id)
            .bind(plan.checkpoint_id.0.as_slice())
            .fetch_optional(&mut *tx)
            .await
            .map_err(backend)?;
        if let Some((generation, write_epoch, stored)) = published {
            if stored != encoded {
                return Err(DirectoryError::Conflict("artifact CAS published ownership conflict".into()));
            }
            let reservation = ArtifactCasReservation::fenced(plan.clone(), u64::try_from(generation).map_err(backend)?, u64::try_from(write_epoch).map_err(backend)?, i64::MAX as u64, coordinator_id, physical_epoch);
            tx.commit().await.map_err(backend)?;
            return Ok(reservation);
        }
        let (released,): (bool,) = sqlx_core::query_as::query_as("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_ledger_journal WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3 AND operation = 'publish')")
            .bind(&plan.scope.space_id)
            .bind(&plan.scope.document_id)
            .bind(plan.checkpoint_id.0.as_slice())
            .fetch_one(&mut *tx)
            .await
            .map_err(backend)?;
        if released {
            return Err(DirectoryError::Conflict("artifact CAS released checkpoint cannot be reserved again".into()));
        }
        let current: Option<(i64, i64, i64, Vec<u8>)> = sqlx_core::query_as::query_as("SELECT generation, write_epoch, expires_at_ms, plan FROM hub_artifact_cas_reservation WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3")
            .bind(&plan.scope.space_id)
            .bind(&plan.scope.document_id)
            .bind(plan.checkpoint_id.0.as_slice())
            .fetch_optional(&mut *tx)
            .await
            .map_err(backend)?;
        if let Some((generation, write_epoch, expiry, stored)) = current {
            if stored != encoded {
                return Err(DirectoryError::Conflict("artifact CAS reservation identity conflict".into()));
            }
            if expiry > now {
                let reservation = ArtifactCasReservation::fenced(plan.clone(), u64::try_from(generation).map_err(backend)?, u64::try_from(write_epoch).map_err(backend)?, u64::try_from(expiry).map_err(backend)?, coordinator_id, physical_epoch);
                tx.commit().await.map_err(backend)?;
                return Ok(reservation);
            }
        }
        let (previous_epoch,): (i64,) = sqlx_core::query_as::query_as("SELECT COALESCE(MAX(write_epoch), 0) FROM hub_artifact_cas_ledger_journal WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3")
            .bind(&plan.scope.space_id)
            .bind(&plan.scope.document_id)
            .bind(plan.checkpoint_id.0.as_slice())
            .fetch_one(&mut *tx)
            .await
            .map_err(backend)?;
        let write_epoch = previous_epoch.checked_add(1).ok_or_else(|| DirectoryError::Conflict("artifact CAS write epoch overflow".into()))?;
        let generation = cas_generation(&mut tx).await?;
        sqlx_core::query::query("INSERT INTO hub_artifact_cas_ledger_journal(generation, operation, space_id, document_id, checkpoint_id, write_epoch, expires_at_ms, plan) VALUES ($1, 'reserve', $2, $3, $4, $5, $6, $7)")
            .bind(generation)
            .bind(&plan.scope.space_id)
            .bind(&plan.scope.document_id)
            .bind(plan.checkpoint_id.0.as_slice())
            .bind(write_epoch)
            .bind(i64::try_from(expires_at_ms).map_err(backend)?)
            .bind(encoded)
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
        let reservation = ArtifactCasReservation::fenced(plan.clone(), u64::try_from(generation).map_err(backend)?, u64::try_from(write_epoch).map_err(backend)?, expires_at_ms, coordinator_id, physical_epoch);
        cas_project_reserve(&mut tx, &reservation).await?;
        tx.commit().await.map_err(backend)?;
        Ok(reservation)
    }

    async fn append_reserved_artifact_checkpoint(&self, event: Option<&NewDirectoryEvent>, checkpoint: &ArtifactCheckpoint, reservation: &ArtifactCasReservation, current_now_ms: u64) -> DirectoryResult<Vec<DirectoryEvent>> {
        validate_artifact_cas_publication_v1(&reservation.plan, checkpoint).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        if let Some(event) = event {
            validate_verified_checkpoint_append(event, checkpoint)?;
        }
        let encoded = encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        let token_generation = i64::try_from(reservation.generation).map_err(backend)?;
        let token_epoch = i64::try_from(reservation.write_epoch).map_err(backend)?;
        let token_expiry = i64::try_from(reservation.expires_at_ms).map_err(backend)?;
        let now = i64::try_from(current_now_ms).map_err(backend)?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        cas_lock_space(&mut tx, &reservation.plan.scope.space_id).await?;
        let (leased,): (bool,) =
            sqlx_core::query_as::query_as("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_delete_lease WHERE space_id = $1 AND expires_at_ms > $2)").bind(&reservation.plan.scope.space_id).bind(now).fetch_one(&mut *tx).await.map_err(backend)?;
        if leased {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is active for this space".into()));
        }
        let published: Option<(i64, i64, Vec<u8>)> = sqlx_core::query_as::query_as("SELECT generation, write_epoch, plan FROM hub_artifact_cas_reference WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3")
            .bind(&reservation.plan.scope.space_id)
            .bind(&reservation.plan.scope.document_id)
            .bind(reservation.plan.checkpoint_id.0.as_slice())
            .fetch_optional(&mut *tx)
            .await
            .map_err(backend)?;
        if let Some((generation, epoch, stored)) = published {
            if event.is_some() || generation != token_generation || epoch != token_epoch || stored != encoded {
                return Err(DirectoryError::Conflict("artifact CAS published reservation conflict".into()));
            }
            return Ok(Vec::new());
        }
        let current: Option<(i64, i64, i64, Vec<u8>)> = sqlx_core::query_as::query_as("SELECT generation, write_epoch, expires_at_ms, plan FROM hub_artifact_cas_reservation WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3 FOR UPDATE")
            .bind(&reservation.plan.scope.space_id)
            .bind(&reservation.plan.scope.document_id)
            .bind(reservation.plan.checkpoint_id.0.as_slice())
            .fetch_optional(&mut *tx)
            .await
            .map_err(backend)?;
        if current.as_ref().is_none_or(|(generation, epoch, expiry, stored)| *generation != token_generation || *epoch != token_epoch || *expiry != token_expiry || *expiry <= now || stored != &encoded) {
            return Err(DirectoryError::Conflict("artifact CAS reservation is missing, expired, or superseded".into()));
        }
        let event = event.ok_or_else(|| DirectoryError::Conflict("new artifact CAS publication requires one public event".into()))?;
        let id = time_ordered_id();
        let recorded_at_ms = now_ms();
        let payload_value = serde_json::Value::from(&event.body.to_value());
        let kind = payload_value.get("kind").and_then(|value| value.as_str()).unwrap_or_default();
        let (event_seq,): (i64,) = sqlx_core::query_as::query_as("UPDATE hub_directory_event_head SET seq = seq + 1 WHERE singleton RETURNING seq").fetch_one(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query("INSERT INTO hub_directory_event(seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, kind, payload, recorded_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)")
            .bind(event_seq)
            .bind(&id)
            .bind(event.hlc.physical_ms)
            .bind(i64::from(event.hlc.logical))
            .bind(actor_kind_to_str(event.actor.kind))
            .bind(&event.actor.id)
            .bind(&event.space_id)
            .bind(&event.user_id)
            .bind(kind)
            .bind(&payload_value)
            .bind(recorded_at_ms)
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
        let full = DirectoryEvent { seq: u64::try_from(event_seq).map_err(backend)?, id, hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone(), recorded_at_ms };
        sqlx_core::query::query("INSERT INTO hub_artifact_authority_journal(event_seq, space_id, document_id, checkpoint_id, payload) VALUES ($1,$2,$3,$4,$5)")
            .bind(event_seq)
            .bind(&checkpoint.scope.space_id)
            .bind(&checkpoint.scope.document_id)
            .bind(checkpoint.checkpoint_id.0.as_slice())
            .bind(serde_json::Value::from(&checkpoint.to_value()))
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
        self.project(&mut tx, &full).await?;
        self.project_verified_checkpoint(&mut tx, &full, checkpoint).await?;
        let generation = cas_generation(&mut tx).await?;
        sqlx_core::query::query("INSERT INTO hub_artifact_cas_ledger_journal(generation, operation, space_id, document_id, checkpoint_id, write_epoch, expires_at_ms, event_seq, plan) VALUES ($1,'publish',$2,$3,$4,$5,$6,$7,$8)")
            .bind(generation)
            .bind(&reservation.plan.scope.space_id)
            .bind(&reservation.plan.scope.document_id)
            .bind(reservation.plan.checkpoint_id.0.as_slice())
            .bind(token_epoch)
            .bind(token_expiry)
            .bind(event_seq)
            .bind(encoded)
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
        cas_project_publish(&mut tx, reservation, generation).await?;
        tx.commit().await.map_err(backend)?;
        Ok(vec![full])
    }

    async fn artifact_cas_ledger_generation(&self) -> DirectoryResult<u64> {
        let (generation,): (i64,) = sqlx_core::query_as::query_as("SELECT generation FROM hub_artifact_cas_ledger_head WHERE singleton").fetch_one(&self.pool).await.map_err(backend)?;
        u64::try_from(generation).map_err(backend)
    }

    async fn artifact_cas_coordinator_id(&self) -> DirectoryResult<[u8; 32]> {
        let (bytes,): (Vec<u8>,) = sqlx_core::query_as::query_as("SELECT coordinator_id FROM hub_artifact_cas_barrier_identity WHERE singleton").fetch_one(&self.pool).await.map_err(backend)?;
        bytes.try_into().map_err(|_| DirectoryError::Backend("artifact CAS barrier coordinator identity is invalid".into()))
    }

    async fn artifact_cas_sweep_candidates(&self, after_generation: u64, through_generation: u64, limit: usize) -> DirectoryResult<ArtifactCasSweepCandidatePage> {
        if limit == 0 || limit > ARTIFACT_CAS_SWEEP_PAGE_MAX {
            return Err(DirectoryError::Conflict(format!("artifact CAS sweep page requires limit 1..={ARTIFACT_CAS_SWEEP_PAGE_MAX}")));
        }
        let (current,): (i64,) = sqlx_core::query_as::query_as("SELECT generation FROM hub_artifact_cas_ledger_head WHERE singleton").fetch_one(&self.pool).await.map_err(backend)?;
        let after = i64::try_from(after_generation).map_err(backend)?;
        let through = i64::try_from(through_generation).map_err(backend)?;
        if through > current || after > through {
            return Err(DirectoryError::Conflict("artifact CAS sweep bounds are outside the ledger".into()));
        }
        let rows: Vec<(i64, Option<Vec<u8>>)> = sqlx_core::query_as::query_as("SELECT generation, plan FROM hub_artifact_cas_ledger_journal WHERE generation > $1 AND generation <= $2 ORDER BY generation LIMIT $3")
            .bind(after)
            .bind(through)
            .bind(i64::try_from(limit).map_err(backend)?)
            .fetch_all(&self.pool)
            .await
            .map_err(backend)?;
        let mut objects = Vec::new();
        let mut next = after;
        for (generation, plan) in rows {
            next = generation;
            if let Some(plan) = plan {
                objects.extend(decode_artifact_cas_ownership_v1(&plan).map_err(|error| DirectoryError::Backend(error.to_string()))?.objects);
            }
        }
        objects.sort_by_key(|object| (object.space_id.clone(), object.kind, object.digest.0));
        objects.dedup();
        Ok(ArtifactCasSweepCandidatePage { observed_generation: through_generation, next_generation: u64::try_from(next).map_err(backend)?, objects })
    }

    async fn artifact_cas_delete_preview_protected(&self, key: &ArtifactCasObjectKey, observed_generation: u64, now_ms: u64) -> DirectoryResult<bool> {
        let (current,): (i64,) = sqlx_core::query_as::query_as("SELECT generation FROM hub_artifact_cas_ledger_head WHERE singleton").fetch_one(&self.pool).await.map_err(backend)?;
        if current < i64::try_from(observed_generation).map_err(backend)? {
            return Err(DirectoryError::Conflict("artifact CAS sweep observation is ahead of the ledger".into()));
        }
        let (referenced,): (bool,) = sqlx_core::query_as::query_as("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reference_object WHERE space_id = $1 AND kind = $2 AND object_digest = $3)")
            .bind(&key.space_id)
            .bind(key.kind.name())
            .bind(key.digest.0.as_slice())
            .fetch_one(&self.pool)
            .await
            .map_err(backend)?;
        let (reserved,): (bool,) = sqlx_core::query_as::query_as("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reservation_object object JOIN hub_artifact_cas_reservation reservation USING(space_id, document_id, checkpoint_id) WHERE object.space_id = $1 AND object.kind = $2 AND object.object_digest = $3 AND reservation.expires_at_ms > $4)")
            .bind(&key.space_id).bind(key.kind.name()).bind(key.digest.0.as_slice()).bind(i64::try_from(now_ms).map_err(backend)?).fetch_one(&self.pool).await.map_err(backend)?;
        Ok(referenced || reserved)
    }

    async fn acquire_artifact_cas_delete_fence(&self, key: &ArtifactCasObjectKey, observed_generation: u64, lease_token: [u8; 32], now_ms: u64, expires_at_ms: u64) -> DirectoryResult<Option<ArtifactCasDeleteFence>> {
        if observed_generation == 0 {
            return Err(DirectoryError::Conflict("artifact CAS sweep requires a nonzero observed generation".into()));
        }
        if lease_token == [0; 32] || expires_at_ms <= now_ms {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is invalid".into()));
        }
        let now = i64::try_from(now_ms).map_err(backend)?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        cas_lock_space(&mut tx, &key.space_id).await?;
        let current_lease: Option<(i64, Option<i64>)> =
            sqlx_core::query_as::query_as("SELECT fence_epoch, expires_at_ms FROM hub_artifact_cas_delete_lease WHERE space_id = $1 FOR UPDATE").bind(&key.space_id).fetch_optional(&mut *tx).await.map_err(backend)?;
        if current_lease.as_ref().and_then(|(_, expiry)| *expiry).is_some_and(|expiry| expiry > now) {
            tx.commit().await.map_err(backend)?;
            return Ok(None);
        }
        let physical_epoch = match current_lease {
            Some((epoch, _)) => epoch.checked_add(1).ok_or_else(|| DirectoryError::Conflict("artifact CAS fence epoch overflow".into()))?,
            None => 1,
        };
        sqlx_core::query::query("INSERT INTO hub_artifact_cas_delete_lease(space_id, fence_epoch, lease_token, expires_at_ms) VALUES ($1,$2,$3,$4) ON CONFLICT(space_id) DO UPDATE SET fence_epoch = excluded.fence_epoch, lease_token = excluded.lease_token, expires_at_ms = excluded.expires_at_ms").bind(&key.space_id).bind(physical_epoch).bind(lease_token.as_slice()).bind(i64::try_from(expires_at_ms).map_err(backend)?).execute(&mut *tx).await.map_err(backend)?;
        let (current,): (i64,) = sqlx_core::query_as::query_as("SELECT generation FROM hub_artifact_cas_ledger_head WHERE singleton").fetch_one(&mut *tx).await.map_err(backend)?;
        if current < i64::try_from(observed_generation).map_err(backend)? {
            return Err(DirectoryError::Conflict("artifact CAS sweep observation is ahead of the ledger".into()));
        }
        let (referenced,): (bool,) = sqlx_core::query_as::query_as("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reference_object WHERE space_id = $1 AND kind = $2 AND object_digest = $3)")
            .bind(&key.space_id)
            .bind(key.kind.name())
            .bind(key.digest.0.as_slice())
            .fetch_one(&mut *tx)
            .await
            .map_err(backend)?;
        let (reserved,): (bool,) = sqlx_core::query_as::query_as("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reservation_object object JOIN hub_artifact_cas_reservation reservation USING(space_id, document_id, checkpoint_id) WHERE object.space_id = $1 AND object.kind = $2 AND object.object_digest = $3 AND reservation.expires_at_ms > $4)").bind(&key.space_id).bind(key.kind.name()).bind(key.digest.0.as_slice()).bind(now).fetch_one(&mut *tx).await.map_err(backend)?;
        if referenced || reserved {
            sqlx_core::query::query("UPDATE hub_artifact_cas_delete_lease SET lease_token = NULL, expires_at_ms = NULL WHERE space_id = $1 AND lease_token = $2")
                .bind(&key.space_id)
                .bind(lease_token.as_slice())
                .execute(&mut *tx)
                .await
                .map_err(backend)?;
            tx.commit().await.map_err(backend)?;
            return Ok(None);
        }
        let (coordinator,): (Vec<u8>,) = sqlx_core::query_as::query_as("SELECT coordinator_id FROM hub_artifact_cas_barrier_identity WHERE singleton").fetch_one(&mut *tx).await.map_err(backend)?;
        let coordinator_id = coordinator.try_into().map_err(|_| DirectoryError::Backend("artifact CAS barrier coordinator identity is invalid".into()))?;
        tx.commit().await.map_err(backend)?;
        Ok(Some(ArtifactCasDeleteFence::new(key.clone(), observed_generation, coordinator_id, u64::try_from(physical_epoch).map_err(backend)?, lease_token)))
    }

    async fn validate_artifact_cas_delete_fence(&self, fence: &ArtifactCasDeleteFence, now_ms: u64) -> DirectoryResult<bool> {
        let now = i64::try_from(now_ms).map_err(backend)?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        cas_lock_space(&mut tx, &fence.object().space_id).await?;
        let lease: Option<(i64, Option<Vec<u8>>, Option<i64>)> =
            sqlx_core::query_as::query_as("SELECT fence_epoch, lease_token, expires_at_ms FROM hub_artifact_cas_delete_lease WHERE space_id = $1 FOR UPDATE").bind(&fence.object().space_id).fetch_optional(&mut *tx).await.map_err(backend)?;
        let lease_valid = lease.is_some_and(|(epoch, token, expiry)| u64::try_from(epoch).ok() == Some(fence.physical_epoch()) && token.as_deref() == Some(fence.lease_token().as_slice()) && expiry.is_some_and(|expiry| expiry > now));
        let (current,): (i64,) = sqlx_core::query_as::query_as("SELECT generation FROM hub_artifact_cas_ledger_head WHERE singleton").fetch_one(&mut *tx).await.map_err(backend)?;
        let (referenced,): (bool,) = sqlx_core::query_as::query_as("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reference_object WHERE space_id = $1 AND kind = $2 AND object_digest = $3)")
            .bind(&fence.object().space_id)
            .bind(fence.object().kind.name())
            .bind(fence.object().digest.0.as_slice())
            .fetch_one(&mut *tx)
            .await
            .map_err(backend)?;
        let (reserved,): (bool,) = sqlx_core::query_as::query_as("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reservation_object object JOIN hub_artifact_cas_reservation reservation USING(space_id, document_id, checkpoint_id) WHERE object.space_id = $1 AND object.kind = $2 AND object.object_digest = $3 AND reservation.expires_at_ms > $4)")
            .bind(&fence.object().space_id).bind(fence.object().kind.name()).bind(fence.object().digest.0.as_slice()).bind(now).fetch_one(&mut *tx).await.map_err(backend)?;
        let (coordinator,): (Vec<u8>,) = sqlx_core::query_as::query_as("SELECT coordinator_id FROM hub_artifact_cas_barrier_identity WHERE singleton").fetch_one(&mut *tx).await.map_err(backend)?;
        tx.commit().await.map_err(backend)?;
        Ok(lease_valid && coordinator.as_slice() == fence.coordinator_id().as_slice() && u64::try_from(current).map_err(backend)? >= fence.ledger_generation() && !referenced && !reserved)
    }

    async fn renew_artifact_cas_delete_fence(&self, fence: &ArtifactCasDeleteFence, now_ms: u64, expires_at_ms: u64) -> DirectoryResult<()> {
        if expires_at_ms <= now_ms {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease renewal is invalid".into()));
        }
        let result = sqlx_core::query::query("UPDATE hub_artifact_cas_delete_lease SET expires_at_ms = $3 WHERE space_id = $1 AND lease_token = $2 AND expires_at_ms > $4")
            .bind(&fence.object().space_id)
            .bind(fence.lease_token().as_slice())
            .bind(i64::try_from(expires_at_ms).map_err(backend)?)
            .bind(i64::try_from(now_ms).map_err(backend)?)
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        if result.rows_affected() != 1 {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is no longer owned".into()));
        }
        Ok(())
    }

    async fn release_artifact_cas_delete_fence(&self, fence: ArtifactCasDeleteFence) -> DirectoryResult<()> {
        let result = sqlx_core::query::query("UPDATE hub_artifact_cas_delete_lease SET lease_token = NULL, expires_at_ms = NULL WHERE space_id = $1 AND lease_token = $2")
            .bind(&fence.object().space_id)
            .bind(fence.lease_token().as_slice())
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        if result.rows_affected() != 1 {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is no longer owned".into()));
        }
        Ok(())
    }

    //#region EventLog
    async fn append_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<Vec<DirectoryEvent>> {
        if events.iter().any(|event| matches!(&event.body, DirectoryEventBody::ArtifactCheckpointPublished { .. })) {
            return Err(DirectoryError::Conflict("checkpoint publication requires the verified authority append seam".into()));
        }
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let mut persisted = Vec::with_capacity(events.len());
        for event in events {
            let id = time_ordered_id();
            let recorded_at_ms = now_ms();
            let payload_value = serde_json::Value::from(&event.body.to_value());
            let kind = payload_value.get("kind").and_then(|value| value.as_str()).unwrap_or_default().to_string();
            let row: (i64,) = sqlx_core::query_as::query_as("UPDATE hub_directory_event_head SET seq = seq + 1 WHERE singleton RETURNING seq").fetch_one(&mut *tx).await.map_err(backend)?;
            sqlx_core::query::query(
                "INSERT INTO hub_directory_event (seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, kind, payload, recorded_at)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
            )
            .bind(row.0)
            .bind(&id)
            .bind(event.hlc.physical_ms)
            .bind(event.hlc.logical as i64)
            .bind(actor_kind_to_str(event.actor.kind))
            .bind(&event.actor.id)
            .bind(&event.space_id)
            .bind(&event.user_id)
            .bind(&kind)
            .bind(&payload_value)
            .bind(recorded_at_ms)
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
            let full = DirectoryEvent { seq: u64::try_from(row.0).map_err(backend)?, id, hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone(), recorded_at_ms };
            self.project(&mut tx, &full).await?;
            let release = match &full.body {
                DirectoryEventBody::ArtifactRetentionAdvanced { retention } => Some(("retention", retention.scope.space_id.as_str(), Some(retention.scope.document_id.as_str()), Some(retention.retained_checkpoint_id))),
                DirectoryEventBody::SpaceDeleted { space_id } => Some(("space-delete", space_id.as_str(), None, None)),
                _ => None,
            };
            if let Some((operation, space_id, document_id, checkpoint_id)) = release {
                let generation = cas_generation(&mut tx).await?;
                sqlx_core::query::query("INSERT INTO hub_artifact_cas_ledger_journal(generation, operation, space_id, document_id, checkpoint_id, event_seq) VALUES ($1,$2,$3,$4,$5,$6)")
                    .bind(generation)
                    .bind(operation)
                    .bind(space_id)
                    .bind(document_id)
                    .bind(checkpoint_id.map(|value| value.0.to_vec()))
                    .bind(row.0)
                    .execute(&mut *tx)
                    .await
                    .map_err(backend)?;
                cas_project_release(&mut tx, operation, space_id, document_id, checkpoint_id).await?;
            }
            persisted.push(full);
        }
        tx.commit().await.map_err(backend)?;
        Ok(persisted)
    }

    async fn events_since(&self, since_seq: u64, limit: usize) -> DirectoryResult<Vec<DirectoryEvent>> {
        let (since_seq, limit) = bounded_event_read(since_seq, limit)?;
        let rows: Vec<(i64, String, i64, i64, String, String, Option<String>, Option<String>, serde_json::Value, i64)> = sqlx_core::query_as::query_as(
            "SELECT seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, payload, recorded_at
             FROM hub_directory_event WHERE seq > $1 ORDER BY seq LIMIT $2",
        )
        .bind(since_seq)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        rows.into_iter().map(event_from_row).collect()
    }

    async fn head_seq(&self) -> DirectoryResult<u64> {
        let row: (Option<i64>,) = sqlx_core::query_as::query_as("SELECT MAX(seq) FROM hub_directory_event").fetch_one(&self.pool).await.map_err(backend)?;
        u64::try_from(row.0.unwrap_or(0)).map_err(backend)
    }

    async fn rebuild_projections(&self) -> DirectoryResult<u64> {
        self.rebuild_projections_controlled(&UNCONTROLLED_PROJECTION_REBUILD).await
    }

    async fn rebuild_projections_controlled(&self, control: &dyn ProjectionRebuildControl) -> DirectoryResult<u64> {
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let (event_count,): (i64,) = sqlx_core::query_as::query_as("SELECT COUNT(*) FROM hub_directory_event").fetch_one(&mut *tx).await.map_err(backend)?;
        let total = u64::try_from(event_count).map_err(backend)?;
        checkpoint_projection_rebuild(control, 0, total)?;
        sqlx_core::query::query("DELETE FROM hub_artifact_cas_reservation").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query("DELETE FROM hub_artifact_cas_reference").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query("DELETE FROM hub_artifact_retention").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query("DELETE FROM hub_artifact_checkpoint_private").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query("DELETE FROM hub_artifact_checkpoint").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query("DELETE FROM hub_document_descriptor").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query("DELETE FROM hub_space_membership").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query("DELETE FROM hub_space").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query("DELETE FROM hub_user").execute(&mut *tx).await.map_err(backend)?;
        let mut replayed = 0u64;
        let mut cursor = 0i64;
        while replayed < total {
            let rows: Vec<(i64, String, i64, i64, String, String, Option<String>, Option<String>, serde_json::Value, i64)> =
                sqlx_core::query_as::query_as("SELECT seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, payload, recorded_at FROM hub_directory_event WHERE seq > $1 ORDER BY seq LIMIT 512")
                    .bind(cursor)
                    .fetch_all(&mut *tx)
                    .await
                    .map_err(backend)?;
            if rows.is_empty() {
                return Err(DirectoryError::Backend("directory event replay ended before its counted head".into()));
            }
            for row in rows {
                cursor = row.0;
                let event = event_from_row(row)?;
                self.project(&mut tx, &event).await?;
                if matches!(&event.body, DirectoryEventBody::ArtifactCheckpointPublished { .. }) {
                    let private: Option<(serde_json::Value,)> = sqlx_core::query_as::query_as("SELECT payload FROM hub_artifact_authority_journal WHERE event_seq = $1").bind(cursor).fetch_optional(&mut *tx).await.map_err(backend)?;
                    let checkpoint =
                        private.ok_or_else(|| DirectoryError::Backend(format!("missing private authority journal for checkpoint event {}", event.seq))).and_then(|(value,)| ArtifactCheckpoint::from_value(DslValue::from(value)).map_err(backend))?;
                    self.project_verified_checkpoint(&mut tx, &event, &checkpoint).await?;
                }
                replayed += 1;
                checkpoint_projection_rebuild(control, replayed, total)?;
            }
        }
        type CasLedgerRow = (i64, String, String, Option<String>, Option<Vec<u8>>, Option<i64>, Option<i64>, Option<Vec<u8>>);
        let ledger_rows: Vec<CasLedgerRow> =
            sqlx_core::query_as::query_as("SELECT generation, operation, space_id, document_id, checkpoint_id, write_epoch, expires_at_ms, plan FROM hub_artifact_cas_ledger_journal ORDER BY generation").fetch_all(&mut *tx).await.map_err(backend)?;
        for (generation, operation, space_id, document_id, checkpoint_id, write_epoch, expires_at_ms, plan) in ledger_rows {
            match operation.as_str() {
                "reserve" => {
                    let reservation = ArtifactCasReservation::unfenced(
                        decode_artifact_cas_ownership_v1(plan.as_deref().ok_or_else(|| DirectoryError::Backend("artifact CAS reserve journal plan missing".into()))?).map_err(|error| DirectoryError::Backend(error.to_string()))?,
                        u64::try_from(generation).map_err(backend)?,
                        u64::try_from(write_epoch.ok_or_else(|| DirectoryError::Backend("artifact CAS reserve journal epoch missing".into()))?).map_err(backend)?,
                        u64::try_from(expires_at_ms.ok_or_else(|| DirectoryError::Backend("artifact CAS reserve journal expiry missing".into()))?).map_err(backend)?,
                    );
                    cas_project_reserve(&mut tx, &reservation).await?;
                }
                "publish" => {
                    let reservation = ArtifactCasReservation::unfenced(
                        decode_artifact_cas_ownership_v1(plan.as_deref().ok_or_else(|| DirectoryError::Backend("artifact CAS publish journal plan missing".into()))?).map_err(|error| DirectoryError::Backend(error.to_string()))?,
                        u64::try_from(generation).map_err(backend)?,
                        u64::try_from(write_epoch.ok_or_else(|| DirectoryError::Backend("artifact CAS publish journal epoch missing".into()))?).map_err(backend)?,
                        u64::try_from(expires_at_ms.ok_or_else(|| DirectoryError::Backend("artifact CAS publish journal expiry missing".into()))?).map_err(backend)?,
                    );
                    cas_project_publish(&mut tx, &reservation, generation).await?;
                }
                "retention" | "space-delete" => {
                    let checkpoint_id = checkpoint_id.map(|bytes| array32(bytes, "artifact CAS release checkpoint").map(ArtifactHash)).transpose()?;
                    cas_project_release(&mut tx, &operation, &space_id, document_id.as_deref(), checkpoint_id).await?;
                }
                _ => return Err(DirectoryError::Backend("artifact CAS ledger operation is invalid".into())),
            }
        }
        tx.commit().await.map_err(backend)?;
        Ok(replayed)
    }
    //#endregion
}

fn user_from_row(row: (String, String, String, Option<String>, Option<String>, Option<String>, i64)) -> UserRecord {
    let (id, email, display_name, password_hash, sso_subject, sso_provider, created_at) = row;
    UserRecord { id, email, display_name, password_hash, sso_subject, sso_provider, created_at }
}

type InviteRow = (String, String, Vec<u8>, String, String, i64, i64, Option<i64>, Option<String>, Option<i64>);
type ShareRow = (String, String, Vec<u8>, String, String, i64, i64, Option<i64>, Option<String>);
type AuthSessionRow = (String, String, Vec<u8>, String, String, Vec<u8>, i64, i64, Option<i64>, Option<String>, i64, String, String);
type AuthAuditRow = (String, i64, String, Option<String>, Option<String>, Option<String>, Option<String>, String, Option<String>, String, String);
struct AdminOperationAuditRow {
    sequence: i64,
    request_id: String,
    intent_digest: String,
    operation_id: String,
    occurred_at: i64,
    phase: String,
    intent_kind: String,
    target_kind: String,
    target_id: String,
    principal_user_id: String,
    principal_session_id: String,
    principal_generation: i64,
    correlation_id: String,
    event_seq_first: Option<i64>,
    event_seq_last: Option<i64>,
    outcome_code: String,
    reason_code: Option<String>,
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
type SyncSessionRow = (String, Option<String>, i64, String, String, String, String, Option<String>, Option<String>, Option<String>, String, i64, Option<i64>);

fn invite_from_row(row: InviteRow) -> DirectoryResult<InviteRecord> {
    let (id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at) = row;
    Ok(InviteRecord { id, selector, secret_digest: array32(secret_digest, "invite digest")?, space_id, role: SpaceRole::parse(&role).unwrap_or(SpaceRole::Spectator), created_at, expires_at, revoked_at, revoked_reason, accepted_at })
}

fn share_from_row(row: ShareRow) -> DirectoryResult<ShareTokenRecord> {
    let (id, selector, secret_digest, space_id, document_id, created_at, expires_at, revoked_at, revoked_reason) = row;
    Ok(ShareTokenRecord { id, selector, secret_digest: array32(secret_digest, "share digest")?, scope: DocumentScope::new(space_id, document_id), created_at, expires_at, revoked_at, revoked_reason })
}

fn auth_session_from_row(row: AuthSessionRow) -> DirectoryResult<AuthSessionRecord> {
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

fn auth_audit_from_row(row: AuthAuditRow) -> AuthAuditRecord {
    let (id, occurred_at, event_kind, auth_session_id, target_user_id, actor_user_id, provider, outcome_code, reason_code, correlation_id, peer_class) = row;
    AuthAuditRecord { id, occurred_at, event_kind, auth_session_id, target_user_id, actor_user_id, provider, outcome_code, reason_code, correlation_id, peer_class }
}

fn admin_operation_audit_from_row(row: AdminOperationAuditRow) -> DirectoryResult<AdminOperationAuditRecord> {
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

fn sync_session_from_row(row: SyncSessionRow) -> SyncSessionRecord {
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

fn event_from_row(row: (i64, String, i64, i64, String, String, Option<String>, Option<String>, serde_json::Value, i64)) -> DirectoryResult<DirectoryEvent> {
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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;
    use std::process::Command;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    //#region 🔖️PostgresFixture
    static NEXT_CONTAINER: AtomicU64 = AtomicU64::new(1);

    struct PostgresContainer {
        name: String,
    }

    impl Drop for PostgresContainer {
        fn drop(&mut self) {
            let _ = Command::new("docker").args(["rm", "--force", &self.name]).output();
        }
    }

    /// 🐘️ Starts a disposable real Postgres behind a private fixture boundary, without a Rust
    /// container-orchestration dependency.
    async fn test_directory() -> (PostgresDirectory, PostgresContainer) {
        let port = TcpListener::bind(("127.0.0.1", 0)).expect("reserve postgres fixture port").local_addr().expect("postgres fixture address").port();
        let sequence = NEXT_CONTAINER.fetch_add(1, Ordering::Relaxed);
        let name = format!("semio-hub-postgres-{}-{sequence}", std::process::id());
        let mapping = format!("127.0.0.1:{port}:5432");
        let output = Command::new("docker").args(["run", "--detach", "--rm", "--name", &name, "--env", "POSTGRES_PASSWORD=postgres", "--publish", &mapping, "postgres:16-alpine"]).output().expect("start docker for postgres fixture");
        assert!(output.status.success(), "start postgres fixture: {}", String::from_utf8_lossy(&output.stderr));
        let container = PostgresContainer { name };
        let url = format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");
        let mut last_error = None;
        for _ in 0..300 {
            match PostgresDirectory::connect(&url).await {
                Ok(directory) => return (directory, container),
                Err(error) => last_error = Some(error),
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        panic!("connect to postgres fixture: {}", last_error.expect("postgres fixture must report a connection error"));
    }
    //#endregion 🔖️PostgresFixture

    fn actor(id: &str) -> DirectoryActor {
        DirectoryActor { kind: DirectoryActorKind::User, id: id.to_string() }
    }

    /// 🌱️ `create_space`/`upsert_membership` were removed (writes now go through
    /// `append_events` — see the module root's `//#region 🔖️Decider`); rebuilds just enough of a
    /// `create-space` decision by hand so these backend tests do not need a full `DirectoryService`.
    async fn seed_space(dir: &PostgresDirectory, clock: &mut HubClock, owner_user_id: &str, kind: DirectorySpaceKind) -> String {
        let space_id = time_ordered_id();
        let owner_role = if kind == DirectorySpaceKind::Archive { DirectorySpaceRole::Spectator } else { DirectorySpaceRole::Author };
        let events = vec![
            NewDirectoryEvent {
                hlc: clock.tick(),
                actor: actor(owner_user_id),
                space_id: Some(space_id.clone()),
                user_id: Some(owner_user_id.to_string()),
                body: DirectoryEventBody::SpaceCreated { space_id: space_id.clone(), name: "Space".into(), space_kind: kind, visibility: DirectorySpaceVisibility::Private, owner_user_id: owner_user_id.to_string() },
            },
            NewDirectoryEvent {
                hlc: clock.tick(),
                actor: actor(owner_user_id),
                space_id: Some(space_id.clone()),
                user_id: Some(owner_user_id.to_string()),
                body: DirectoryEventBody::MemberUpserted { space_id: space_id.clone(), user_id: owner_user_id.to_string(), role: owner_role },
            },
        ];
        dir.append_events(&events).await.expect("seed space");
        space_id
    }

    fn admin_audit_fact(phase: &str, outcome_code: &str) -> NewAdminOperationAuditRecord {
        NewAdminOperationAuditRecord {
            request_id: "request:postgres-race".into(),
            intent_digest: "1".repeat(64),
            operation_id: "operation:postgres-race".into(),
            occurred_at: now_ms(),
            phase: phase.into(),
            intent_kind: "delete-space".into(),
            target_kind: "space".into(),
            target_id: "space:one".into(),
            principal_user_id: "user:admin".into(),
            principal_session_id: "session:admin".into(),
            principal_generation: 7,
            correlation_id: "correlation:postgres-race".into(),
            event_seq_first: None,
            event_seq_last: None,
            outcome_code: outcome_code.into(),
            reason_code: None,
        }
    }

    #[tokio::test]
    async fn admin_operation_audit_concurrent_absent_request_rereads_established_receipt() {
        let (directory, _container) = test_directory().await;
        let directory = Arc::new(directory);
        let barrier = Arc::new(tokio::sync::Barrier::new(17));
        let accepted = admin_audit_fact("accepted", "accepted");
        let mut writers = Vec::new();
        for _ in 0..16 {
            let directory = directory.clone();
            let barrier = barrier.clone();
            let accepted = accepted.clone();
            writers.push(tokio::spawn(async move {
                barrier.wait().await;
                directory.append_admin_operation_audit(&accepted).await
            }));
        }
        barrier.wait().await;
        let mut sequence = None;
        for writer in writers {
            let established = writer.await.expect("postgres audit writer").expect("race loser rereads receipt");
            if let Some(first) = sequence {
                assert_eq!(first, established.sequence);
            } else {
                sequence = Some(established.sequence);
            }
        }
        assert_eq!(directory.admin_operation_audit_for_request(&accepted.request_id).await.expect("postgres audit").len(), 1);
        let terminal = admin_audit_fact("succeeded", "space-deleted");
        assert_eq!(directory.append_admin_operation_audit(&terminal).await.expect("postgres terminal").fact.intent_digest, accepted.intent_digest);
    }

    // 🔬️ Users, spaces, and role-based membership round-trip against a real Postgres.
    #[tokio::test]
    async fn user_space_membership_round_trip() {
        let (directory, _container) = test_directory().await;
        let mut clock = HubClock::new();
        let user = directory.create_user("a@example.com", "Ada", None, None, None).await.expect("create user");
        let space_id = seed_space(&directory, &mut clock, &user.id, DirectorySpaceKind::Studio).await;
        assert_eq!(directory.get_role(&space_id, &user.id).await.unwrap(), Some(SpaceRole::Author));
    }

    // 🔬️ Schema bootstrap + seed grow a default space the seed user authors — proves the DDL
    // (`hub_space`/`hub_space_membership`, `author`/`spectator` role CHECK) matches this crate's
    // own queries against a real Postgres instance.
    #[tokio::test]
    async fn seed_creates_default_space_and_membership() {
        let (directory, _container) = test_directory().await;
        directory.seed().await.expect("seed");
        let space = directory.get_space("default").await.unwrap().expect("default space");
        assert_eq!(space.kind, "studio");
        assert_eq!(space.visibility, "private");
        assert_eq!(directory.get_role("default", "seed").await.unwrap(), Some(SpaceRole::Author));
    }

    // 🔬️ Event log replay reproduces the same projections against a real Postgres, mirroring the
    // sqlite backend's `event_log_replay_matches_projections`.
    #[tokio::test]
    async fn event_log_replay_matches_projections() {
        let (directory, _container) = test_directory().await;
        directory.seed().await.expect("seed");
        let head = directory.head_seq().await.expect("head seq");
        let before = directory.get_space("default").await.unwrap();
        let replayed = directory.rebuild_projections().await.expect("rebuild");
        assert_eq!(replayed, head);
        assert_eq!(directory.get_space("default").await.unwrap(), before);
    }
}
//#endregion 🧪️Tests
