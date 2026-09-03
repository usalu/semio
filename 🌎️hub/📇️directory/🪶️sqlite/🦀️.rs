//! 🪶️ `HubDirectory` over SQLite (rusqlite) — the zero-touch default for local dev and single-user
//! self-hosting; no external database service required. `#[cfg(feature = "sqlite")]`-gated as a
//! whole by the parent `directory` module (see `📇️directory/🦀️.rs`'s `//#region 🔖️Backends`).
//!
//! Uses synchronous `rusqlite` behind `Arc<Mutex<Connection>>`, not an async SQLite driver: a
//! Cargo workspace may link only one native `sqlite3` (`links = "sqlite3"`), and `rusqlite` is
//! already the sqlite binding used elsewhere in this workspace (`vcs`, `db_storage_sqlite`,
//! semio_compose_rs's unrelated client lib) — adding `sqlx-sqlite`'s `libsqlite3-sys` alongside it
//! is a hard `cargo` resolution conflict, not a style choice. Trait methods stay `async fn`
//! (satisfying the shared `HubDirectory` interface) but their bodies are synchronous rusqlite
//! calls: queries are short, the mutex guard is never held across an `.await`, so nothing here
//! blocks the executor for longer than a real query takes.

use crate::directory::error::{DirectoryError, DirectoryResult};
use crate::directory::model::*;
use crate::directory::{
    active_capability, auth_audit, bounded_event_read, checkpoint_projection_rebuild, kind_to_str, prepare_auth_session, prepare_invite, prepare_share_token, role_from_wire, validate_bounded_auth_text, validate_verified_checkpoint_append,
    visibility_to_str, HubClock, HubDirectory, InviteCapability, NewDirectoryEvent, ProjectionRebuildControl, SessionCapability, ShareCapability, ARTIFACT_CHECKPOINT_LINEAGE_MAX, AUTH_AUDIT_PAGE_MAX, AUTH_TEXT_MAX_BYTES,
    ArtifactCasSweepCandidatePage, UNCONTROLLED_PROJECTION_REBUILD, ARTIFACT_CAS_RESERVATION_MAX_TTL_MS, ARTIFACT_CAS_SWEEP_PAGE_MAX,
};
use crate::artifact_authority::chunk_cas::{decode_artifact_cas_ownership_v1, encode_artifact_cas_ownership_v1, validate_artifact_cas_publication_v1, ArtifactCasDeleteFence, ArtifactCasObjectKey, ArtifactCasOwnershipPlanV1, ArtifactCasReservation};
use directory::os_directory::{
    ArtifactCheckpoint, ArtifactHash, ArtifactRetention, DirectoryActor, DirectoryActorKind, DirectoryEvent, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, DocumentDescriptor, DocumentFrontier, DocumentOwner,
    Hlc, PublishedArtifactCheckpoint,
};
use directory::os_identity::time_ordered_id;
use directory::{DslValue, FromValue, ToValue};
use rusqlite::{Connection, OptionalExtension, Transaction};
use std::sync::{Arc, Mutex};

//#region 🔖️Schema
const SCHEMA: &str = "\
PRAGMA foreign_keys = ON;
CREATE TABLE IF NOT EXISTS hub_share_grant (
    id TEXT PRIMARY KEY,
    selector TEXT NOT NULL UNIQUE,
    secret_digest BLOB NOT NULL CHECK (length(secret_digest) = 32),
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
    revoked_at INTEGER,
    revoked_reason TEXT
);
CREATE TABLE IF NOT EXISTS hub_user (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    password_hash TEXT,
    sso_subject TEXT,
    sso_provider TEXT,
    created_at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS hub_space (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    owner_user_id TEXT NOT NULL REFERENCES hub_user(id),
    created_at INTEGER NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('atelier', 'studio', 'archive')),
    visibility TEXT NOT NULL CHECK (visibility IN ('private', 'public'))
);
CREATE TABLE IF NOT EXISTS hub_space_membership (
    space_id TEXT NOT NULL REFERENCES hub_space(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES hub_user(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('author', 'spectator')),
    created_at INTEGER NOT NULL,
    PRIMARY KEY (space_id, user_id)
);
CREATE TABLE IF NOT EXISTS hub_document_descriptor (
    space_id TEXT NOT NULL REFERENCES hub_space(id) ON DELETE CASCADE,
    document_id TEXT NOT NULL,
    artifact_kind TEXT NOT NULL,
    artifact_schema TEXT NOT NULL,
    owner_plugin_id TEXT NOT NULL,
    owner_package_id TEXT NOT NULL,
    owner_version TEXT NOT NULL,
    owner_package_hash TEXT NOT NULL,
    pack_schema_hash TEXT NOT NULL,
    bootstrap_version INTEGER NOT NULL,
    bootstrap_head_seq INTEGER NOT NULL,
    bootstrap_commit_seq INTEGER NOT NULL,
    bootstrap_epoch INTEGER NOT NULL,
    bootstrap_snapshot_hash TEXT NOT NULL,
    announced_at INTEGER NOT NULL,
    PRIMARY KEY (space_id, document_id)
);
CREATE TABLE IF NOT EXISTS hub_artifact_checkpoint (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    checkpoint_id BLOB NOT NULL CHECK (length(checkpoint_id) = 32),
    parent_checkpoint_id BLOB CHECK (parent_checkpoint_id IS NULL OR length(parent_checkpoint_id) = 32),
    descriptor_digest BLOB NOT NULL CHECK (length(descriptor_digest) = 32),
    frontier_document_id TEXT NOT NULL,
    head_edit_ordinal INTEGER NOT NULL,
    head_edit_id TEXT NOT NULL,
    last_commit_seq INTEGER NOT NULL,
    chain_hash BLOB NOT NULL CHECK (length(chain_hash) = 32),
    pack_sha256 BLOB NOT NULL CHECK (length(pack_sha256) = 32),
    pack_byte_length INTEGER NOT NULL,
    spr_sha256 BLOB NOT NULL CHECK (length(spr_sha256) = 32),
    spr_byte_length INTEGER NOT NULL,
    aggregate_sha256 BLOB NOT NULL CHECK (length(aggregate_sha256) = 32),
    published_at INTEGER NOT NULL,
    event_seq INTEGER NOT NULL UNIQUE,
    active INTEGER NOT NULL CHECK (active IN (0, 1)),
    payload TEXT NOT NULL,
    PRIMARY KEY (space_id, document_id, checkpoint_id),
    FOREIGN KEY (space_id, document_id) REFERENCES hub_document_descriptor(space_id, document_id) ON DELETE CASCADE
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_artifact_checkpoint_active ON hub_artifact_checkpoint (space_id, document_id) WHERE active = 1;
CREATE INDEX IF NOT EXISTS idx_artifact_checkpoint_lineage ON hub_artifact_checkpoint (space_id, document_id, event_seq);
CREATE TABLE IF NOT EXISTS hub_artifact_retention (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    retained_checkpoint_id BLOB NOT NULL CHECK (length(retained_checkpoint_id) = 32),
    floor_document_id TEXT NOT NULL,
    floor_head_edit_ordinal INTEGER NOT NULL,
    floor_head_edit_id TEXT NOT NULL,
    floor_last_commit_seq INTEGER NOT NULL,
    floor_chain_hash BLOB NOT NULL CHECK (length(floor_chain_hash) = 32),
    checkpoint_lineage_head BLOB NOT NULL CHECK (length(checkpoint_lineage_head) = 32),
    event_seq INTEGER NOT NULL UNIQUE,
    payload TEXT NOT NULL,
    PRIMARY KEY (space_id, document_id),
    FOREIGN KEY (space_id, document_id, retained_checkpoint_id) REFERENCES hub_artifact_checkpoint(space_id, document_id, checkpoint_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS hub_auth_session (
    id TEXT PRIMARY KEY,
    selector TEXT NOT NULL UNIQUE,
    secret_digest BLOB NOT NULL CHECK (length(secret_digest) = 32),
    user_id TEXT NOT NULL REFERENCES hub_user(id) ON DELETE CASCADE,
    identity_provider TEXT NOT NULL,
    identity_subject_digest BLOB NOT NULL CHECK (length(identity_subject_digest) = 32),
    issued_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
    revoked_at INTEGER,
    revoked_reason TEXT,
    authorization_generation INTEGER NOT NULL CHECK (authorization_generation >= 1),
    device_instance_id TEXT NOT NULL,
    session_kind TEXT NOT NULL CHECK (session_kind IN ('external', 'development-local'))
);
CREATE TABLE IF NOT EXISTS hub_sync_session (
    id TEXT PRIMARY KEY,
    auth_session_id TEXT REFERENCES hub_auth_session(id) ON DELETE SET NULL,
    authorization_generation INTEGER NOT NULL,
    actor_id TEXT NOT NULL,
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    surface TEXT NOT NULL,
    user_id TEXT REFERENCES hub_user(id) ON DELETE SET NULL,
    space_role TEXT,
    client_label TEXT NOT NULL,
    connected_at INTEGER NOT NULL,
    disconnected_at INTEGER
);
CREATE TABLE IF NOT EXISTS hub_space_invite (
    id TEXT PRIMARY KEY,
    selector TEXT NOT NULL UNIQUE,
    secret_digest BLOB NOT NULL CHECK (length(secret_digest) = 32),
    space_id TEXT NOT NULL REFERENCES hub_space(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('author', 'spectator')),
    created_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
    revoked_at INTEGER,
    revoked_reason TEXT,
    accepted_at INTEGER
);
CREATE TABLE IF NOT EXISTS hub_auth_audit (
    sequence INTEGER PRIMARY KEY AUTOINCREMENT,
    id TEXT NOT NULL UNIQUE,
    occurred_at INTEGER NOT NULL,
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
CREATE TABLE IF NOT EXISTS hub_directory_event (
    seq INTEGER PRIMARY KEY AUTOINCREMENT CHECK (seq <= 9007199254740991),
    id TEXT NOT NULL UNIQUE,
    hlc_physical INTEGER NOT NULL,
    hlc_logical INTEGER NOT NULL,
    actor_kind TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    space_id TEXT,
    user_id TEXT,
    kind TEXT NOT NULL,
    payload TEXT NOT NULL,
    recorded_at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS hub_artifact_authority_journal (
    event_seq INTEGER PRIMARY KEY REFERENCES hub_directory_event(seq),
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    checkpoint_id BLOB NOT NULL CHECK (length(checkpoint_id) = 32),
    payload TEXT NOT NULL,
    UNIQUE (space_id, document_id, checkpoint_id)
);
CREATE TABLE IF NOT EXISTS hub_artifact_checkpoint_private (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    checkpoint_id BLOB NOT NULL CHECK (length(checkpoint_id) = 32),
    event_seq INTEGER NOT NULL UNIQUE REFERENCES hub_artifact_authority_journal(event_seq),
    payload TEXT NOT NULL,
    PRIMARY KEY (space_id, document_id, checkpoint_id),
    FOREIGN KEY (space_id, document_id, checkpoint_id) REFERENCES hub_artifact_checkpoint(space_id, document_id, checkpoint_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS hub_artifact_cas_ledger_head (
    singleton INTEGER PRIMARY KEY CHECK(singleton = 1),
    generation INTEGER NOT NULL CHECK(generation >= 0)
);
INSERT OR IGNORE INTO hub_artifact_cas_ledger_head(singleton, generation) VALUES (1, 0);
CREATE TABLE IF NOT EXISTS hub_artifact_cas_ledger_journal (
    generation INTEGER PRIMARY KEY CHECK(generation >= 1),
    operation TEXT NOT NULL CHECK(operation IN ('reserve', 'publish', 'retention', 'space-delete')),
    space_id TEXT NOT NULL,
    document_id TEXT,
    checkpoint_id BLOB CHECK(checkpoint_id IS NULL OR length(checkpoint_id) = 32),
    write_epoch INTEGER,
    expires_at_ms INTEGER,
    event_seq INTEGER,
    plan BLOB,
    CHECK((operation IN ('reserve', 'publish') AND document_id IS NOT NULL AND checkpoint_id IS NOT NULL AND write_epoch >= 1 AND expires_at_ms IS NOT NULL AND plan IS NOT NULL) OR operation IN ('retention', 'space-delete'))
);
CREATE TABLE IF NOT EXISTS hub_artifact_cas_reservation (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    checkpoint_id BLOB NOT NULL CHECK(length(checkpoint_id) = 32),
    generation INTEGER NOT NULL UNIQUE,
    write_epoch INTEGER NOT NULL CHECK(write_epoch >= 1),
    expires_at_ms INTEGER NOT NULL,
    plan BLOB NOT NULL,
    PRIMARY KEY(space_id, document_id, checkpoint_id)
);
CREATE TABLE IF NOT EXISTS hub_artifact_cas_reservation_object (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    checkpoint_id BLOB NOT NULL CHECK(length(checkpoint_id) = 32),
    kind TEXT NOT NULL CHECK(kind IN ('chunk', 'manifest')),
    object_digest BLOB NOT NULL CHECK(length(object_digest) = 32),
    PRIMARY KEY(space_id, document_id, checkpoint_id, kind, object_digest),
    FOREIGN KEY(space_id, document_id, checkpoint_id) REFERENCES hub_artifact_cas_reservation(space_id, document_id, checkpoint_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS hub_artifact_cas_reference (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    checkpoint_id BLOB NOT NULL CHECK(length(checkpoint_id) = 32),
    generation INTEGER NOT NULL UNIQUE,
    write_epoch INTEGER NOT NULL CHECK(write_epoch >= 1),
    plan BLOB NOT NULL,
    PRIMARY KEY(space_id, document_id, checkpoint_id)
);
CREATE TABLE IF NOT EXISTS hub_artifact_cas_reference_object (
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    checkpoint_id BLOB NOT NULL CHECK(length(checkpoint_id) = 32),
    kind TEXT NOT NULL CHECK(kind IN ('chunk', 'manifest')),
    object_digest BLOB NOT NULL CHECK(length(object_digest) = 32),
    PRIMARY KEY(space_id, document_id, checkpoint_id, kind, object_digest),
    FOREIGN KEY(space_id, document_id, checkpoint_id) REFERENCES hub_artifact_cas_reference(space_id, document_id, checkpoint_id) ON DELETE CASCADE
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
";
//#endregion 🔖️Schema

fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
}

fn backend<E: std::fmt::Display>(err: E) -> DirectoryError {
    DirectoryError::Backend(err.to_string())
}

fn blob32(row: &rusqlite::Row<'_>, index: usize) -> rusqlite::Result<[u8; 32]> {
    let bytes: Vec<u8> = row.get(index)?;
    bytes.try_into().map_err(|bytes: Vec<u8>| rusqlite::Error::FromSqlConversionFailure(index, rusqlite::types::Type::Blob, format!("expected 32 bytes, got {}", bytes.len()).into()))
}

fn insert_auth_audit(conn: &Connection, event: &AuthAuditRecord) -> DirectoryResult<()> {
    conn.execute(
        "INSERT INTO hub_auth_audit (id, occurred_at, event_kind, auth_session_id, target_user_id, actor_user_id, provider, outcome_code, reason_code, correlation_id, peer_class) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        rusqlite::params![event.id, event.occurred_at, event.event_kind, event.auth_session_id, event.target_user_id, event.actor_user_id, event.provider, event.outcome_code, event.reason_code, event.correlation_id, event.peer_class],
    )
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

/// @emoji 🗄️ SQLite-backed `HubDirectory`. One `rusqlite::Connection` behind a `Mutex` — see this
/// module's own doc for why this isn't an async SQLite driver.
pub struct SqliteDirectory {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteDirectory {
    /// @emoji 🔌️ Opens (creating if absent) the SQLite database at `path` and bootstraps the schema.
    /// `path` may be `:memory:` for tests.
    pub async fn connect(path: &str) -> DirectoryResult<Self> {
        let conn = Connection::open(path).map_err(backend)?;
        conn.execute_batch(SCHEMA).map_err(backend)?;
        Ok(Self { conn: Arc::new(Mutex::new(conn)) })
    }

    fn lock(&self) -> DirectoryResult<std::sync::MutexGuard<'_, Connection>> {
        self.conn.lock().map_err(|_| DirectoryError::Backend("sqlite connection lock poisoned".into()))
    }

    fn revoke_auth_sessions_matching(
        &self,
        predicate: &str,
        key: &str,
        subject_digest: Option<[u8; 32]>,
        reason: &str,
        actor_user_id: Option<&str>,
        correlation_id: &str,
    ) -> DirectoryResult<Vec<RevokedAuthSession>> {
        validate_bounded_auth_text(reason, "session revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        let rows: Vec<(String, String, i64, String)> = {
            let sql = format!("SELECT id, user_id, authorization_generation, identity_provider FROM hub_auth_session WHERE {predicate} AND revoked_at IS NULL ORDER BY id");
            let mut statement = tx.prepare(&sql).map_err(backend)?;
            if let Some(digest) = subject_digest {
                statement
                    .query_map(rusqlite::params![key, digest.as_slice()], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)))
                    .map_err(backend)?
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(backend)?
            } else {
                statement.query_map([key], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))).map_err(backend)?.collect::<Result<Vec<_>, _>>().map_err(backend)?
            }
        };
        let mut revoked = Vec::with_capacity(rows.len());
        for (id, user_id, generation, provider) in rows {
            let next_generation = generation.checked_add(1).ok_or_else(|| DirectoryError::Conflict("authorization generation overflow".into()))?;
            tx.execute("UPDATE hub_auth_session SET revoked_at = ?2, revoked_reason = ?3, authorization_generation = ?4 WHERE id = ?1 AND revoked_at IS NULL", rusqlite::params![id, revoked_at, reason, next_generation]).map_err(backend)?;
            let audit = auth_audit(revoked_at, "session-revoked", Some(&id), Some(&user_id), actor_user_id, Some(&provider), "success", Some(reason), correlation_id, "server")?;
            insert_auth_audit(&tx, &audit)?;
            revoked.push(RevokedAuthSession { id, authorization_generation: u64::try_from(next_generation).map_err(backend)?, revoked_at });
        }
        tx.commit().map_err(backend)?;
        Ok(revoked)
    }

    fn persist_event(&self, tx: &Transaction<'_>, event: &NewDirectoryEvent) -> DirectoryResult<DirectoryEvent> {
        let id = time_ordered_id();
        let recorded_at_ms = now_ms();
        let payload_value = serde_json::Value::from(&event.body.to_value());
        let kind = payload_value.get("kind").and_then(|value| value.as_str()).unwrap_or_default().to_string();
        tx.execute(
            "INSERT INTO hub_directory_event (id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, kind, payload, recorded_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![id, event.hlc.physical_ms, event.hlc.logical, actor_kind_to_str(event.actor.kind), event.actor.id, event.space_id, event.user_id, kind, payload_value.to_string(), recorded_at_ms],
        )
        .map_err(backend)?;
        Ok(DirectoryEvent { seq: u64::try_from(tx.last_insert_rowid()).map_err(backend)?, id, hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone(), recorded_at_ms })
    }

    fn project_verified_checkpoint(&self, tx: &Transaction<'_>, event: &DirectoryEvent, checkpoint: &ArtifactCheckpoint) -> DirectoryResult<()> {
        let new_event = NewDirectoryEvent { hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone() };
        validate_verified_checkpoint_append(&new_event, checkpoint)?;
        let payload = serde_json::Value::from(&checkpoint.to_value()).to_string();
        let event_seq = i64::try_from(event.seq).map_err(backend)?;
        tx.execute(
            "INSERT INTO hub_artifact_checkpoint_private (space_id, document_id, checkpoint_id, event_seq, payload) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![checkpoint.scope.space_id, checkpoint.scope.document_id, checkpoint.checkpoint_id.0.as_slice(), event_seq, payload],
        )
        .map_err(backend)?;
        Ok(())
    }

    fn cas_generation(tx: &Transaction<'_>) -> DirectoryResult<i64> {
        let current: i64 = tx.query_row("SELECT generation FROM hub_artifact_cas_ledger_head WHERE singleton = 1", [], |row| row.get(0)).map_err(backend)?;
        let next = current.checked_add(1).ok_or_else(|| DirectoryError::Conflict("artifact CAS ledger generation overflow".into()))?;
        tx.execute("UPDATE hub_artifact_cas_ledger_head SET generation = ?1 WHERE singleton = 1 AND generation = ?2", rusqlite::params![next, current]).map_err(backend)?;
        Ok(next)
    }

    fn cas_insert_objects(tx: &Transaction<'_>, table: &str, plan: &ArtifactCasOwnershipPlanV1) -> DirectoryResult<()> {
        if !matches!(table, "hub_artifact_cas_reservation_object" | "hub_artifact_cas_reference_object") {
            return Err(DirectoryError::Backend("invalid artifact CAS projection table".into()));
        }
        let sql = format!("INSERT INTO {table}(space_id, document_id, checkpoint_id, kind, object_digest) VALUES (?1, ?2, ?3, ?4, ?5)");
        for object in &plan.objects {
            tx.execute(&sql, rusqlite::params![plan.scope.space_id, plan.scope.document_id, plan.checkpoint_id.0.as_slice(), object.kind.name(), object.digest.0.as_slice()]).map_err(backend)?;
        }
        Ok(())
    }

    fn cas_project_reserve(tx: &Transaction<'_>, reservation: &ArtifactCasReservation) -> DirectoryResult<()> {
        let plan = encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        let generation = i64::try_from(reservation.generation).map_err(backend)?;
        let write_epoch = i64::try_from(reservation.write_epoch).map_err(backend)?;
        let expires_at_ms = i64::try_from(reservation.expires_at_ms).map_err(backend)?;
        tx.execute("DELETE FROM hub_artifact_cas_reservation WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3", rusqlite::params![reservation.plan.scope.space_id, reservation.plan.scope.document_id, reservation.plan.checkpoint_id.0.as_slice()]).map_err(backend)?;
        tx.execute(
            "INSERT INTO hub_artifact_cas_reservation(space_id, document_id, checkpoint_id, generation, write_epoch, expires_at_ms, plan) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![reservation.plan.scope.space_id, reservation.plan.scope.document_id, reservation.plan.checkpoint_id.0.as_slice(), generation, write_epoch, expires_at_ms, plan],
        ).map_err(backend)?;
        Self::cas_insert_objects(tx, "hub_artifact_cas_reservation_object", &reservation.plan)
    }

    fn cas_project_publish(tx: &Transaction<'_>, reservation: &ArtifactCasReservation, generation: i64) -> DirectoryResult<()> {
        let plan = encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        let write_epoch = i64::try_from(reservation.write_epoch).map_err(backend)?;
        tx.execute("DELETE FROM hub_artifact_cas_reservation WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3", rusqlite::params![reservation.plan.scope.space_id, reservation.plan.scope.document_id, reservation.plan.checkpoint_id.0.as_slice()]).map_err(backend)?;
        tx.execute(
            "INSERT INTO hub_artifact_cas_reference(space_id, document_id, checkpoint_id, generation, write_epoch, plan) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![reservation.plan.scope.space_id, reservation.plan.scope.document_id, reservation.plan.checkpoint_id.0.as_slice(), generation, write_epoch, plan],
        ).map_err(backend)?;
        Self::cas_insert_objects(tx, "hub_artifact_cas_reference_object", &reservation.plan)
    }

    fn cas_project_release(tx: &Transaction<'_>, operation: &str, space_id: &str, document_id: Option<&str>, checkpoint_id: Option<ArtifactHash>) -> DirectoryResult<()> {
        match operation {
            "retention" => {
                let document_id = document_id.ok_or_else(|| DirectoryError::Backend("artifact CAS retention document missing".into()))?;
                let checkpoint_id = checkpoint_id.ok_or_else(|| DirectoryError::Backend("artifact CAS retention checkpoint missing".into()))?;
                tx.execute(
                    "DELETE FROM hub_artifact_cas_reference WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id IN (
                        SELECT checkpoint_id FROM hub_artifact_authority_journal WHERE space_id = ?1 AND document_id = ?2 AND event_seq < (
                            SELECT event_seq FROM hub_artifact_authority_journal WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3
                        )
                    )",
                    rusqlite::params![space_id, document_id, checkpoint_id.0.as_slice()],
                ).map_err(backend)?;
                tx.execute(
                    "DELETE FROM hub_artifact_checkpoint_private WHERE space_id = ?1 AND document_id = ?2 AND event_seq < (
                        SELECT event_seq FROM hub_artifact_authority_journal WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3
                    )",
                    rusqlite::params![space_id, document_id, checkpoint_id.0.as_slice()],
                ).map_err(backend)?;
            }
            "space-delete" => {
                tx.execute("DELETE FROM hub_artifact_cas_reservation WHERE space_id = ?1", [space_id]).map_err(backend)?;
                tx.execute("DELETE FROM hub_artifact_cas_reference WHERE space_id = ?1", [space_id]).map_err(backend)?;
            }
            _ => return Err(DirectoryError::Backend("invalid artifact CAS release operation".into())),
        }
        Ok(())
    }

    fn append_cas_release(&self, tx: &Transaction<'_>, event: &DirectoryEvent) -> DirectoryResult<()> {
        let (operation, space_id, document_id, checkpoint_id) = match &event.body {
            DirectoryEventBody::ArtifactRetentionAdvanced { retention } => ("retention", retention.scope.space_id.as_str(), Some(retention.scope.document_id.as_str()), Some(retention.retained_checkpoint_id)),
            DirectoryEventBody::SpaceDeleted { space_id } => ("space-delete", space_id.as_str(), None, None),
            _ => return Ok(()),
        };
        let generation = Self::cas_generation(tx)?;
        tx.execute(
            "INSERT INTO hub_artifact_cas_ledger_journal(generation, operation, space_id, document_id, checkpoint_id, event_seq) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![generation, operation, space_id, document_id, checkpoint_id.map(|value| value.0.to_vec()), i64::try_from(event.seq).map_err(backend)?],
        ).map_err(backend)?;
        Self::cas_project_release(tx, operation, space_id, document_id, checkpoint_id)
    }

    /// @emoji 🌱️ Seeds a placeholder `seed` system user and a default `studio`/`private` space it
    /// owns, through the event log (`user.created` + `space.created` + `member.upserted`) like any
    /// other write — the system user satisfies `hub_space.owner_user_id`'s foreign key until a real
    /// bootstrap admin claims ownership through `/admin` (HP-6). Document existence itself is
    /// `db::Database`'s concern (see `bin.rs`), not seeded here.
    pub async fn seed(&self) -> DirectoryResult<()> {
        let user_exists: i64 = self.lock()?.query_row("SELECT COUNT(*) FROM hub_user WHERE id = 'seed'", [], |row| row.get(0)).map_err(backend)?;
        if user_exists > 0 {
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

    //#region 🔖️Projections
    /// @emoji 🧮️ The only place `hub_user`/`hub_space`/`hub_space_membership` rows are written —
    /// applies one already-persisted `DirectoryEvent`'s effect inside the same transaction
    /// `append_events`/`rebuild_projections` run in. Unconditional: by the time an event exists in
    /// the log, `decide` (`../🦀️.rs`) already enforced every law, so this never rejects.
    fn project(&self, tx: &Transaction<'_>, event: &DirectoryEvent) -> DirectoryResult<()> {
        match &event.body {
            DirectoryEventBody::UserCreated { user_id, email, display_name } => {
                tx.execute("INSERT OR IGNORE INTO hub_user (id, email, display_name, created_at) VALUES (?1, ?2, ?3, ?4)", rusqlite::params![user_id, email, display_name, event.recorded_at_ms]).map_err(backend)?;
            }
            DirectoryEventBody::SpaceCreated { space_id, name, space_kind, visibility, owner_user_id } => {
                tx.execute(
                    "INSERT OR IGNORE INTO hub_space (id, name, owner_user_id, created_at, kind, visibility) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    rusqlite::params![space_id, name, owner_user_id, event.recorded_at_ms, kind_to_str(*space_kind), visibility_to_str(*visibility)],
                )
                .map_err(backend)?;
            }
            DirectoryEventBody::SpaceRenamed { space_id, name } => {
                tx.execute("UPDATE hub_space SET name = ?2 WHERE id = ?1", rusqlite::params![space_id, name]).map_err(backend)?;
            }
            DirectoryEventBody::SpaceVisibilityChanged { space_id, visibility } => {
                tx.execute("UPDATE hub_space SET visibility = ?2 WHERE id = ?1", rusqlite::params![space_id, visibility_to_str(*visibility)]).map_err(backend)?;
            }
            DirectoryEventBody::SpaceArchived { space_id } => {
                tx.execute("UPDATE hub_space SET kind = 'archive' WHERE id = ?1", rusqlite::params![space_id]).map_err(backend)?;
            }
            DirectoryEventBody::SpaceDeleted { space_id } => {
                tx.execute("DELETE FROM hub_share_grant WHERE space_id = ?1", rusqlite::params![space_id]).map_err(backend)?;
                tx.execute("DELETE FROM hub_artifact_retention WHERE space_id = ?1", rusqlite::params![space_id]).map_err(backend)?;
                tx.execute("DELETE FROM hub_artifact_checkpoint_private WHERE space_id = ?1", rusqlite::params![space_id]).map_err(backend)?;
                tx.execute("DELETE FROM hub_artifact_checkpoint WHERE space_id = ?1", rusqlite::params![space_id]).map_err(backend)?;
                tx.execute("DELETE FROM hub_document_descriptor WHERE space_id = ?1", rusqlite::params![space_id]).map_err(backend)?;
                tx.execute("DELETE FROM hub_space_membership WHERE space_id = ?1", rusqlite::params![space_id]).map_err(backend)?;
                tx.execute("DELETE FROM hub_space_invite WHERE space_id = ?1", rusqlite::params![space_id]).map_err(backend)?;
                tx.execute("DELETE FROM hub_space WHERE id = ?1", rusqlite::params![space_id]).map_err(backend)?;
            }
            DirectoryEventBody::MemberUpserted { space_id, user_id, role } => {
                tx.execute(
                    "INSERT INTO hub_space_membership (space_id, user_id, role, created_at) VALUES (?1, ?2, ?3, ?4)
                     ON CONFLICT(space_id, user_id) DO UPDATE SET role = ?3",
                    rusqlite::params![space_id, user_id, role_from_wire(*role).as_str(), event.recorded_at_ms],
                )
                .map_err(backend)?;
            }
            DirectoryEventBody::MemberRemoved { space_id, user_id } => {
                tx.execute("DELETE FROM hub_space_membership WHERE space_id = ?1 AND user_id = ?2", rusqlite::params![space_id, user_id]).map_err(backend)?;
            }
            DirectoryEventBody::InviteRedeemed { space_id, user_id, role, .. } => {
                tx.execute(
                    "INSERT INTO hub_space_membership (space_id, user_id, role, created_at) VALUES (?1, ?2, ?3, ?4)
                     ON CONFLICT(space_id, user_id) DO UPDATE SET role = ?3",
                    rusqlite::params![space_id, user_id, role_from_wire(*role).as_str(), event.recorded_at_ms],
                )
                .map_err(backend)?;
            }
            DirectoryEventBody::DocumentAnnounced { descriptor } => {
                tx.execute(
                    "INSERT OR IGNORE INTO hub_document_descriptor (space_id, document_id, artifact_kind, artifact_schema, owner_plugin_id, owner_package_id, owner_version, owner_package_hash, pack_schema_hash, bootstrap_version, bootstrap_head_seq, bootstrap_commit_seq, bootstrap_epoch, bootstrap_snapshot_hash, announced_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
                    rusqlite::params![descriptor.space_id, descriptor.document_id, descriptor.artifact_kind, descriptor.artifact_schema, descriptor.owner.plugin_id, descriptor.owner.package_id, descriptor.owner.version, descriptor.owner.package_hash, descriptor.pack_schema_hash, descriptor.bootstrap_version as i64, descriptor.bootstrap_frontier.head_seq as i64, descriptor.bootstrap_frontier.commit_seq as i64, descriptor.bootstrap_frontier.epoch as i64, descriptor.bootstrap_snapshot_hash, event.recorded_at_ms],
                )
                .map_err(backend)?;
            }
            DirectoryEventBody::ArtifactCheckpointPublished { checkpoint } => {
                let payload = serde_json::Value::from(&checkpoint.to_value()).to_string();
                tx.execute(
                    "UPDATE hub_artifact_checkpoint SET active = 0 WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id <> ?3",
                    rusqlite::params![checkpoint.scope.space_id, checkpoint.scope.document_id, checkpoint.checkpoint_id.0.as_slice()],
                )
                .map_err(backend)?;
                tx.execute(
                    "INSERT INTO hub_artifact_checkpoint (space_id, document_id, checkpoint_id, parent_checkpoint_id, descriptor_digest, frontier_document_id, head_edit_ordinal, head_edit_id, last_commit_seq, chain_hash, pack_sha256, pack_byte_length, spr_sha256, spr_byte_length, aggregate_sha256, published_at, event_seq, active, payload)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, 1, ?18)
                     ON CONFLICT(space_id, document_id, checkpoint_id) DO UPDATE SET active = 1, payload = excluded.payload",
                    rusqlite::params![checkpoint.scope.space_id, checkpoint.scope.document_id, checkpoint.checkpoint_id.0.as_slice(), checkpoint.parent_checkpoint_id.map(|id| id.0.to_vec()), checkpoint.descriptor_digest_v1.0.as_slice(), checkpoint.baseline_frontier.document_id, i64::try_from(checkpoint.baseline_frontier.head_edit_ordinal).map_err(backend)?, checkpoint.baseline_frontier.head_edit_id, i64::try_from(checkpoint.baseline_frontier.last_commit_seq).map_err(backend)?, checkpoint.baseline_frontier.chain_hash.0.as_slice(), checkpoint.pack.sha256.0.as_slice(), i64::try_from(checkpoint.pack.byte_length).map_err(backend)?, checkpoint.spr.sha256.0.as_slice(), i64::try_from(checkpoint.spr.byte_length).map_err(backend)?, checkpoint.aggregate_sha256.0.as_slice(), i64::try_from(checkpoint.published_at_ms).map_err(backend)?, i64::try_from(event.seq).map_err(backend)?, payload],
                )
                .map_err(backend)?;
            }
            DirectoryEventBody::ArtifactRetentionAdvanced { retention } => {
                let payload = serde_json::Value::from(&retention.to_value()).to_string();
                tx.execute(
                    "INSERT INTO hub_artifact_retention (space_id, document_id, retained_checkpoint_id, floor_document_id, floor_head_edit_ordinal, floor_head_edit_id, floor_last_commit_seq, floor_chain_hash, checkpoint_lineage_head, event_seq, payload)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
                     ON CONFLICT(space_id, document_id) DO UPDATE SET retained_checkpoint_id = excluded.retained_checkpoint_id, floor_document_id = excluded.floor_document_id, floor_head_edit_ordinal = excluded.floor_head_edit_ordinal, floor_head_edit_id = excluded.floor_head_edit_id, floor_last_commit_seq = excluded.floor_last_commit_seq, floor_chain_hash = excluded.floor_chain_hash, checkpoint_lineage_head = excluded.checkpoint_lineage_head, event_seq = excluded.event_seq, payload = excluded.payload",
                    rusqlite::params![retention.scope.space_id, retention.scope.document_id, retention.retained_checkpoint_id.0.as_slice(), retention.retained_floor.document_id, i64::try_from(retention.retained_floor.head_edit_ordinal).map_err(backend)?, retention.retained_floor.head_edit_id, i64::try_from(retention.retained_floor.last_commit_seq).map_err(backend)?, retention.retained_floor.chain_hash.0.as_slice(), retention.checkpoint_lineage_head.0.as_slice(), i64::try_from(event.seq).map_err(backend)?, payload],
                )
                .map_err(backend)?;
            }
        }
        Ok(())
    }
    //#endregion 🔖️Projections
}

impl HubDirectory for SqliteDirectory {
    //#region ShareTokens
    async fn issue_share_token(&self, scope: &DocumentScope, ttl_secs: i64, correlation_id: &str) -> DirectoryResult<IssuedShareToken> {
        let issued = prepare_share_token(scope, ttl_secs, now_ms())?;
        let audit = auth_audit(issued.record.created_at, "share-issued", Some(&issued.record.id), None, None, None, "success", None, correlation_id, "server")?;
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        tx.execute(
            "INSERT INTO hub_share_grant (id, selector, secret_digest, space_id, document_id, created_at, expires_at, revoked_at, revoked_reason) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, NULL, NULL)",
            rusqlite::params![issued.record.id, issued.record.selector, issued.record.secret_digest.as_slice(), scope.space_id, scope.document_id, issued.record.created_at, issued.record.expires_at],
        )
        .map_err(backend)?;
        insert_auth_audit(&tx, &audit)?;
        tx.commit().map_err(backend)?;
        Ok(issued)
    }

    async fn revoke_share_token(&self, scope: &DocumentScope, share_id: &str, reason: &str, correlation_id: &str) -> DirectoryResult<()> {
        validate_bounded_auth_text(reason, "share revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let audit = auth_audit(revoked_at, "share-revoked", Some(share_id), None, None, None, "success", Some(reason), correlation_id, "server")?;
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        let changed = tx.execute(
            "UPDATE hub_share_grant SET revoked_at = ?4, revoked_reason = ?5 WHERE id = ?1 AND space_id = ?2 AND document_id = ?3 AND revoked_at IS NULL",
            rusqlite::params![share_id, scope.space_id, scope.document_id, revoked_at, reason],
        ).map_err(backend)?;
        if changed == 0 {
            Err(DirectoryError::NotFound(format!("share grant {share_id}")))
        } else {
            insert_auth_audit(&tx, &audit)?;
            tx.commit().map_err(backend)?;
            Ok(())
        }
    }

    async fn authenticate_share(&self, scope: &DocumentScope, capability: &ShareCapability) -> DirectoryResult<bool> {
        let conn = self.lock()?;
        let row: Option<(String, Vec<u8>, i64, Option<i64>)> = conn
            .query_row(
                "SELECT selector, secret_digest, expires_at, revoked_at FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2 AND selector = ?3",
                rusqlite::params![scope.space_id, scope.document_id, capability.selector()],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .optional()
            .map_err(backend)?;
        let Some((selector, digest, expires_at, revoked_at)) = row else { return Ok(false) };
        let Ok(digest): Result<[u8; 32], _> = digest.try_into() else { return Err(DirectoryError::Backend("stored share digest width is invalid".into())) };
        Ok(active_capability(&selector, &digest, expires_at, revoked_at, capability.selector(), &capability.secret_digest(), now_ms()))
    }
    //#endregion

    //#region Users
    async fn create_user(&self, email: &str, display_name: &str, password_hash: Option<&str>, sso_subject: Option<&str>, sso_provider: Option<&str>) -> DirectoryResult<UserRecord> {
        let id = time_ordered_id();
        let created_at = now_ms();
        self.lock()?
            .execute(
                "INSERT INTO hub_user (id, email, display_name, password_hash, sso_subject, sso_provider, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                rusqlite::params![id, email, display_name, password_hash, sso_subject, sso_provider, created_at],
            )
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
        self.lock()?.query_row("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE id = ?1", [user_id], user_row).optional().map_err(backend)
    }

    async fn get_user_by_email(&self, email: &str) -> DirectoryResult<Option<UserRecord>> {
        self.lock()?.query_row("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE email = ?1", [email], user_row).optional().map_err(backend)
    }

    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> DirectoryResult<Option<UserRecord>> {
        self.lock()?
            .query_row("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE sso_provider = ?1 AND sso_subject = ?2", rusqlite::params![provider, subject], user_row)
            .optional()
            .map_err(backend)
    }

    async fn list_users(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<UserRecord>> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user ORDER BY created_at LIMIT ?1 OFFSET ?2").map_err(backend)?;
        let rows = stmt.query_map(rusqlite::params![limit, offset], user_row).map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).collect())
    }
    //#endregion

    //#region Spaces
    async fn get_space(&self, space_id: &str) -> DirectoryResult<Option<SpaceRecord>> {
        self.lock()?
            .query_row("SELECT id, name, owner_user_id, created_at, kind, visibility FROM hub_space WHERE id = ?1", [space_id], |row| {
                Ok(SpaceRecord { id: row.get(0)?, name: row.get(1)?, owner_user_id: row.get(2)?, created_at: row.get(3)?, kind: row.get(4)?, visibility: row.get(5)? })
            })
            .optional()
            .map_err(backend)
    }

    async fn list_spaces_for_user(&self, user_id: &str) -> DirectoryResult<Vec<(SpaceRecord, SpaceRole)>> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare(
                "SELECT s.id, s.name, s.owner_user_id, s.created_at, s.kind, s.visibility, m.role FROM hub_space s
                 JOIN hub_space_membership m ON m.space_id = s.id WHERE m.user_id = ?1 ORDER BY s.created_at",
            )
            .map_err(backend)?;
        let rows =
            stmt.query_map([user_id], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?, row.get::<_, i64>(3)?, row.get::<_, String>(4)?, row.get::<_, String>(5)?, row.get::<_, String>(6)?))).map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).filter_map(|(id, name, owner_user_id, created_at, kind, visibility, role)| SpaceRole::parse(&role).map(|role| (SpaceRecord { id, name, owner_user_id, created_at, kind, visibility }, role))).collect())
    }

    async fn list_spaces(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<SpaceRecord>> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare("SELECT id, name, owner_user_id, created_at, kind, visibility FROM hub_space ORDER BY created_at LIMIT ?1 OFFSET ?2").map_err(backend)?;
        let rows = stmt.query_map(rusqlite::params![limit, offset], |row| Ok(SpaceRecord { id: row.get(0)?, name: row.get(1)?, owner_user_id: row.get(2)?, created_at: row.get(3)?, kind: row.get(4)?, visibility: row.get(5)? })).map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).collect())
    }

    async fn list_members(&self, space_id: &str) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare(
                "SELECT u.id, u.email, u.display_name, u.password_hash, u.sso_subject, u.sso_provider, u.created_at, m.role
                 FROM hub_space_membership m JOIN hub_user u ON u.id = m.user_id WHERE m.space_id = ?1 ORDER BY m.created_at",
            )
            .map_err(backend)?;
        let rows = stmt
            .query_map([space_id], |row| {
                let role: String = row.get(7)?;
                Ok((UserRecord { id: row.get(0)?, email: row.get(1)?, display_name: row.get(2)?, password_hash: row.get(3)?, sso_subject: row.get(4)?, sso_provider: row.get(5)?, created_at: row.get(6)? }, role))
            })
            .map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).filter_map(|(user, role)| SpaceRole::parse(&role).map(|role| (user, role))).collect())
    }

    async fn get_role(&self, space_id: &str, user_id: &str) -> DirectoryResult<Option<SpaceRole>> {
        let role: Option<String> = self.lock()?.query_row("SELECT role FROM hub_space_membership WHERE space_id = ?1 AND user_id = ?2", rusqlite::params![space_id, user_id], |row| row.get(0)).optional().map_err(backend)?;
        Ok(role.and_then(|r| SpaceRole::parse(&r)))
    }

    async fn get_document_descriptor(&self, scope: &DocumentScope) -> DirectoryResult<Option<DocumentDescriptor>> {
        self.lock()?
            .query_row(
                "SELECT space_id, document_id, artifact_kind, artifact_schema, owner_plugin_id, owner_package_id, owner_version, owner_package_hash, pack_schema_hash, bootstrap_version, bootstrap_head_seq, bootstrap_commit_seq, bootstrap_epoch, bootstrap_snapshot_hash FROM hub_document_descriptor WHERE space_id = ?1 AND document_id = ?2",
                rusqlite::params![scope.space_id, scope.document_id],
                document_descriptor_row,
            )
            .optional()
            .map_err(backend)
    }

    async fn list_document_descriptors(&self, space_id: &str) -> DirectoryResult<Vec<DocumentDescriptor>> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare("SELECT space_id, document_id, artifact_kind, artifact_schema, owner_plugin_id, owner_package_id, owner_version, owner_package_hash, pack_schema_hash, bootstrap_version, bootstrap_head_seq, bootstrap_commit_seq, bootstrap_epoch, bootstrap_snapshot_hash FROM hub_document_descriptor WHERE space_id = ?1 ORDER BY document_id").map_err(backend)?;
        let rows = stmt.query_map([space_id], document_descriptor_row).map_err(backend)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(backend)
    }

    async fn get_artifact_checkpoint(&self, scope: &DocumentScope, checkpoint_id: ArtifactHash) -> DirectoryResult<Option<PublishedArtifactCheckpoint>> {
        self.lock()?
            .query_row("SELECT payload FROM hub_artifact_checkpoint WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3", rusqlite::params![scope.space_id, scope.document_id, checkpoint_id.0.as_slice()], published_checkpoint_row)
            .optional()
            .map_err(backend)
    }

    async fn get_verified_artifact_checkpoint(&self, scope: &DocumentScope, checkpoint_id: ArtifactHash) -> DirectoryResult<Option<ArtifactCheckpoint>> {
        self.lock()?
            .query_row("SELECT payload FROM hub_artifact_checkpoint_private WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3", rusqlite::params![scope.space_id, scope.document_id, checkpoint_id.0.as_slice()], verified_checkpoint_row)
            .optional()
            .map_err(backend)
    }

    async fn get_active_artifact_checkpoint(&self, scope: &DocumentScope) -> DirectoryResult<Option<PublishedArtifactCheckpoint>> {
        self.lock()?.query_row("SELECT payload FROM hub_artifact_checkpoint WHERE space_id = ?1 AND document_id = ?2 AND active = 1", rusqlite::params![scope.space_id, scope.document_id], published_checkpoint_row).optional().map_err(backend)
    }

    async fn get_artifact_retention(&self, scope: &DocumentScope) -> DirectoryResult<Option<ArtifactRetention>> {
        self.lock()?.query_row("SELECT payload FROM hub_artifact_retention WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![scope.space_id, scope.document_id], artifact_retention_row).optional().map_err(backend)
    }

    async fn artifact_checkpoint_count(&self, scope: &DocumentScope) -> DirectoryResult<u64> {
        let count: i64 = self.lock()?.query_row("SELECT COUNT(*) FROM hub_artifact_checkpoint WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![scope.space_id, scope.document_id], |row| row.get(0)).map_err(backend)?;
        u64::try_from(count).map_err(backend)
    }

    async fn list_artifact_checkpoint_lineage(&self, scope: &DocumentScope, limit: usize) -> DirectoryResult<Vec<PublishedArtifactCheckpoint>> {
        if limit == 0 || limit as u64 > ARTIFACT_CHECKPOINT_LINEAGE_MAX {
            return Err(DirectoryError::Conflict(format!("artifact checkpoint lineage limit must be 1..={ARTIFACT_CHECKPOINT_LINEAGE_MAX}")));
        }
        let conn = self.lock()?;
        let mut stmt = conn.prepare("SELECT payload FROM hub_artifact_checkpoint WHERE space_id = ?1 AND document_id = ?2 ORDER BY event_seq LIMIT ?3").map_err(backend)?;
        let rows = stmt.query_map(rusqlite::params![scope.space_id, scope.document_id, limit as i64], published_checkpoint_row).map_err(backend)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(backend)
    }
    //#endregion

    //#region AuthSessions
    async fn issue_auth_session(&self, issue: &AuthSessionIssue) -> DirectoryResult<IssuedAuthSession> {
        let issued = prepare_auth_session(issue, now_ms())?;
        let audit = auth_audit(
            issued.record.issued_at,
            "session-issued",
            Some(&issued.record.id),
            Some(&issued.record.user_id),
            None,
            Some(&issued.record.identity_provider),
            "success",
            None,
            &issue.correlation_id,
            &issue.peer_class,
        )?;
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        tx.execute(
            "INSERT INTO hub_auth_session (id, selector, secret_digest, user_id, identity_provider, identity_subject_digest, issued_at, expires_at, revoked_at, revoked_reason, authorization_generation, device_instance_id, session_kind) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL, NULL, ?9, ?10, ?11)",
            rusqlite::params![issued.record.id, issued.record.selector, issued.record.secret_digest.as_slice(), issued.record.user_id, issued.record.identity_provider, issued.record.identity_subject_digest.as_slice(), issued.record.issued_at, issued.record.expires_at, i64::try_from(issued.record.authorization_generation).map_err(backend)?, issued.record.device_instance_id, issued.record.session_kind.as_str()],
        )
        .map_err(backend)?;
        insert_auth_audit(&tx, &audit)?;
        tx.commit().map_err(backend)?;
        Ok(issued)
    }

    async fn authenticate_session(&self, capability: &SessionCapability) -> DirectoryResult<Option<AuthSessionRecord>> {
        let record = self
            .lock()?
            .query_row(
                "SELECT id, selector, secret_digest, user_id, identity_provider, identity_subject_digest, issued_at, expires_at, revoked_at, revoked_reason, authorization_generation, device_instance_id, session_kind FROM hub_auth_session WHERE selector = ?1",
                [capability.selector()],
                auth_session_row,
            )
            .optional()
            .map_err(backend)?;
        Ok(record.filter(|record| {
            active_capability(&record.selector, &record.secret_digest, record.expires_at, record.revoked_at, capability.selector(), &capability.secret_digest(), now_ms())
        }))
    }

    async fn revoke_auth_session(&self, id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Option<RevokedAuthSession>> {
        validate_bounded_auth_text(reason, "session revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        let row: Option<(String, String, i64)> = tx
            .query_row("SELECT id, user_id, authorization_generation FROM hub_auth_session WHERE id = ?1 AND revoked_at IS NULL", [id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
            .optional()
            .map_err(backend)?;
        let Some((id, user_id, generation)) = row else { return Ok(None) };
        let next_generation = generation.checked_add(1).ok_or_else(|| DirectoryError::Conflict("authorization generation overflow".into()))?;
        tx.execute("UPDATE hub_auth_session SET revoked_at = ?2, revoked_reason = ?3, authorization_generation = ?4 WHERE id = ?1", rusqlite::params![id, revoked_at, reason, next_generation]).map_err(backend)?;
        let audit = auth_audit(revoked_at, "session-revoked", Some(&id), Some(&user_id), actor_user_id, None, "success", Some(reason), correlation_id, "server")?;
        insert_auth_audit(&tx, &audit)?;
        tx.commit().map_err(backend)?;
        Ok(Some(RevokedAuthSession { id, authorization_generation: u64::try_from(next_generation).map_err(backend)?, revoked_at }))
    }

    async fn revoke_auth_sessions_for_user(&self, user_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>> {
        self.revoke_auth_sessions_matching("user_id = ?1", user_id, None, reason, actor_user_id, correlation_id)
    }

    async fn revoke_auth_sessions_for_identity(&self, provider: &str, subject_digest: [u8; 32], reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>> {
        self.revoke_auth_sessions_matching("identity_provider = ?1 AND identity_subject_digest = ?2", provider, Some(subject_digest), reason, actor_user_id, correlation_id)
    }

    async fn list_auth_audit(&self, limit: usize, offset: usize) -> DirectoryResult<Vec<AuthAuditRecord>> {
        if limit == 0 || limit > AUTH_AUDIT_PAGE_MAX {
            return Err(DirectoryError::Conflict(format!("auth audit limit must be 1..={AUTH_AUDIT_PAGE_MAX}")));
        }
        let conn = self.lock()?;
        let mut statement = conn
            .prepare("SELECT id, occurred_at, event_kind, auth_session_id, target_user_id, actor_user_id, provider, outcome_code, reason_code, correlation_id, peer_class FROM hub_auth_audit ORDER BY sequence LIMIT ?1 OFFSET ?2")
            .map_err(backend)?;
        let rows = statement.query_map(rusqlite::params![i64::try_from(limit).map_err(backend)?, i64::try_from(offset).map_err(backend)?], auth_audit_row).map_err(backend)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(backend)
    }
    //#endregion

    //#region Invites
    async fn issue_invite(&self, space_id: &str, role: SpaceRole, ttl_secs: i64, correlation_id: &str) -> DirectoryResult<IssuedInvite> {
        let issued = prepare_invite(space_id, role, ttl_secs, now_ms())?;
        let audit = auth_audit(issued.record.created_at, "invite-issued", Some(&issued.record.id), None, None, None, "success", None, correlation_id, "server")?;
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        tx.execute(
            "INSERT INTO hub_space_invite (id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, NULL, NULL, NULL)",
            rusqlite::params![issued.record.id, issued.record.selector, issued.record.secret_digest.as_slice(), space_id, role.as_str(), issued.record.created_at, issued.record.expires_at],
        )
        .map_err(backend)?;
        insert_auth_audit(&tx, &audit)?;
        tx.commit().map_err(backend)?;
        Ok(issued)
    }

    async fn authenticate_invite(&self, capability: &InviteCapability) -> DirectoryResult<Option<InviteRecord>> {
        let record = self.lock()?.query_row("SELECT id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at FROM hub_space_invite WHERE selector = ?1", [capability.selector()], invite_row).optional().map_err(backend)?;
        Ok(record.filter(|record| {
            record.accepted_at.is_none()
                && active_capability(&record.selector, &record.secret_digest, record.expires_at, record.revoked_at, capability.selector(), &capability.secret_digest(), now_ms())
        }))
    }

    async fn revoke_invite(&self, invite_id: &str, reason: &str, correlation_id: &str) -> DirectoryResult<()> {
        validate_bounded_auth_text(reason, "invite revoke reason", AUTH_TEXT_MAX_BYTES)?;
        let revoked_at = now_ms();
        let audit = auth_audit(revoked_at, "invite-revoked", Some(invite_id), None, None, None, "success", Some(reason), correlation_id, "server")?;
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        let changed = tx.execute("UPDATE hub_space_invite SET revoked_at = ?2, revoked_reason = ?3 WHERE id = ?1 AND revoked_at IS NULL", rusqlite::params![invite_id, revoked_at, reason]).map_err(backend)?;
        if changed == 0 {
            return Err(DirectoryError::NotFound(format!("invite {invite_id}")));
        }
        insert_auth_audit(&tx, &audit)?;
        tx.commit().map_err(backend)?;
        Ok(())
    }

    async fn list_invites(&self, space_id: &str) -> DirectoryResult<Vec<InviteRecord>> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare("SELECT id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at FROM hub_space_invite WHERE space_id = ?1 ORDER BY created_at DESC").map_err(backend)?;
        let rows = stmt.query_map([space_id], invite_row).map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).collect())
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
        let role_str = space_role.map(|r| r.as_str());
        self.lock()?
            .execute(
                "INSERT INTO hub_sync_session (id, auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, space_role, client_label, connected_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                rusqlite::params![id, auth_session_id, i64::try_from(authorization_generation).map_err(backend)?, actor_id, space_id, document_id, surface, user_id, role_str, client_label, connected_at],
            )
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
            space_role,
            client_label: client_label.to_string(),
            connected_at,
            disconnected_at: None,
        })
    }

    async fn record_sync_session_close(&self, sync_session_id: &str) -> DirectoryResult<()> {
        self.lock()?.execute("UPDATE hub_sync_session SET disconnected_at = ?2 WHERE id = ?1", rusqlite::params![sync_session_id, now_ms()]).map_err(backend)?;
        Ok(())
    }

    async fn list_sync_sessions_for_document(&self, document_id: &str) -> DirectoryResult<Vec<SyncSessionRecord>> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare("SELECT id, auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, space_role, client_label, connected_at, disconnected_at FROM hub_sync_session WHERE document_id = ?1 ORDER BY connected_at DESC").map_err(backend)?;
        let rows = stmt.query_map([document_id], sync_session_row).map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).collect())
    }

    async fn list_active_sync_sessions(&self, space_id: Option<&str>) -> DirectoryResult<Vec<SyncSessionRecord>> {
        let conn = self.lock()?;
        let rows: Vec<SyncSessionRecord> = match space_id {
            Some(space_id) => {
                let mut stmt = conn
                    .prepare("SELECT id, auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, space_role, client_label, connected_at, disconnected_at FROM hub_sync_session WHERE space_id = ?1 AND disconnected_at IS NULL ORDER BY connected_at DESC")
                    .map_err(backend)?;
                let mapped = stmt.query_map([space_id], sync_session_row).map_err(backend)?;
                mapped.filter_map(|row| row.ok()).collect()
            }
            None => {
                let mut stmt =
                    conn.prepare("SELECT id, auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, space_role, client_label, connected_at, disconnected_at FROM hub_sync_session WHERE disconnected_at IS NULL ORDER BY connected_at DESC").map_err(backend)?;
                let mapped = stmt.query_map([], sync_session_row).map_err(backend)?;
                mapped.filter_map(|row| row.ok()).collect()
            }
        };
        Ok(rows)
    }

    async fn close_all_sync_sessions(&self) -> DirectoryResult<()> {
        self.lock()?.execute("UPDATE hub_sync_session SET disconnected_at = ?1 WHERE disconnected_at IS NULL", rusqlite::params![now_ms()]).map_err(backend)?;
        Ok(())
    }
    //#endregion

    async fn reserve_artifact_cas(&self, plan: &ArtifactCasOwnershipPlanV1, expires_at_ms: u64, now_ms: u64) -> DirectoryResult<ArtifactCasReservation> {
        let encoded = encode_artifact_cas_ownership_v1(plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        if expires_at_ms <= now_ms || expires_at_ms.checked_sub(now_ms).is_none_or(|ttl| ttl > ARTIFACT_CAS_RESERVATION_MAX_TTL_MS) {
            return Err(DirectoryError::Conflict(format!("artifact CAS reservation ttl must be 1..={ARTIFACT_CAS_RESERVATION_MAX_TTL_MS} milliseconds")));
        }
        let expires_at = i64::try_from(expires_at_ms).map_err(backend)?;
        let now = i64::try_from(now_ms).map_err(backend)?;
        let mut conn = self.lock()?;
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(backend)?;
        let historical: Option<Vec<u8>> = tx.query_row(
            "SELECT plan FROM hub_artifact_cas_ledger_journal WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3 AND plan IS NOT NULL ORDER BY generation LIMIT 1",
            rusqlite::params![plan.scope.space_id, plan.scope.document_id, plan.checkpoint_id.0.as_slice()],
            |row| row.get(0),
        ).optional().map_err(backend)?;
        if historical.as_ref().is_some_and(|value| value != &encoded) {
            return Err(DirectoryError::Conflict("artifact CAS checkpoint identity names a different ownership plan".into()));
        }
        let published: Option<(i64, i64, Vec<u8>)> = tx.query_row(
            "SELECT generation, write_epoch, plan FROM hub_artifact_cas_reference WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3",
            rusqlite::params![plan.scope.space_id, plan.scope.document_id, plan.checkpoint_id.0.as_slice()],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        ).optional().map_err(backend)?;
        if let Some((generation, write_epoch, stored)) = published {
            if stored != encoded {
                return Err(DirectoryError::Conflict("artifact CAS published ownership conflict".into()));
            }
            return Ok(ArtifactCasReservation { plan: plan.clone(), generation: u64::try_from(generation).map_err(backend)?, write_epoch: u64::try_from(write_epoch).map_err(backend)?, expires_at_ms: i64::MAX as u64 });
        }
        let released: i64 = tx.query_row(
            "SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_ledger_journal WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3 AND operation = 'publish')",
            rusqlite::params![plan.scope.space_id, plan.scope.document_id, plan.checkpoint_id.0.as_slice()],
            |row| row.get(0),
        ).map_err(backend)?;
        if released != 0 {
            return Err(DirectoryError::Conflict("artifact CAS released checkpoint cannot be reserved again".into()));
        }
        let current: Option<(i64, i64, i64, Vec<u8>)> = tx.query_row(
            "SELECT generation, write_epoch, expires_at_ms, plan FROM hub_artifact_cas_reservation WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3",
            rusqlite::params![plan.scope.space_id, plan.scope.document_id, plan.checkpoint_id.0.as_slice()],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        ).optional().map_err(backend)?;
        if let Some((generation, write_epoch, current_expiry, stored)) = current {
            if stored != encoded {
                return Err(DirectoryError::Conflict("artifact CAS reservation identity conflict".into()));
            }
            if current_expiry > now {
                return Ok(ArtifactCasReservation { plan: plan.clone(), generation: u64::try_from(generation).map_err(backend)?, write_epoch: u64::try_from(write_epoch).map_err(backend)?, expires_at_ms: u64::try_from(current_expiry).map_err(backend)? });
            }
        }
        let previous_epoch: i64 = tx.query_row(
            "SELECT COALESCE(MAX(write_epoch), 0) FROM hub_artifact_cas_ledger_journal WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3",
            rusqlite::params![plan.scope.space_id, plan.scope.document_id, plan.checkpoint_id.0.as_slice()],
            |row| row.get(0),
        ).map_err(backend)?;
        let write_epoch = previous_epoch.checked_add(1).ok_or_else(|| DirectoryError::Conflict("artifact CAS write epoch overflow".into()))?;
        let generation = Self::cas_generation(&tx)?;
        tx.execute(
            "INSERT INTO hub_artifact_cas_ledger_journal(generation, operation, space_id, document_id, checkpoint_id, write_epoch, expires_at_ms, plan) VALUES (?1, 'reserve', ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![generation, plan.scope.space_id, plan.scope.document_id, plan.checkpoint_id.0.as_slice(), write_epoch, expires_at, encoded],
        ).map_err(backend)?;
        let reservation = ArtifactCasReservation { plan: plan.clone(), generation: u64::try_from(generation).map_err(backend)?, write_epoch: u64::try_from(write_epoch).map_err(backend)?, expires_at_ms };
        Self::cas_project_reserve(&tx, &reservation)?;
        tx.commit().map_err(backend)?;
        Ok(reservation)
    }

    async fn append_reserved_artifact_checkpoint(&self, event: Option<&NewDirectoryEvent>, checkpoint: &ArtifactCheckpoint, reservation: &ArtifactCasReservation, now_ms: u64) -> DirectoryResult<Vec<DirectoryEvent>> {
        validate_artifact_cas_publication_v1(&reservation.plan, checkpoint).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        if let Some(event) = event {
            validate_verified_checkpoint_append(event, checkpoint)?;
        }
        let encoded = encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        let now = i64::try_from(now_ms).map_err(backend)?;
        let token_generation = i64::try_from(reservation.generation).map_err(backend)?;
        let token_epoch = i64::try_from(reservation.write_epoch).map_err(backend)?;
        let mut conn = self.lock()?;
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(backend)?;
        let published: Option<(i64, i64, Vec<u8>)> = tx.query_row(
            "SELECT generation, write_epoch, plan FROM hub_artifact_cas_reference WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3",
            rusqlite::params![reservation.plan.scope.space_id, reservation.plan.scope.document_id, reservation.plan.checkpoint_id.0.as_slice()],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        ).optional().map_err(backend)?;
        if let Some((generation, write_epoch, stored)) = published {
            if event.is_some() || generation != token_generation || write_epoch != token_epoch || stored != encoded {
                return Err(DirectoryError::Conflict("artifact CAS published reservation conflict".into()));
            }
            return Ok(Vec::new());
        }
        let current: Option<(i64, i64, i64, Vec<u8>)> = tx.query_row(
            "SELECT generation, write_epoch, expires_at_ms, plan FROM hub_artifact_cas_reservation WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3",
            rusqlite::params![reservation.plan.scope.space_id, reservation.plan.scope.document_id, reservation.plan.checkpoint_id.0.as_slice()],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        ).optional().map_err(backend)?;
        if current.as_ref().is_none_or(|(generation, epoch, expiry, stored)| *generation != token_generation || *epoch != token_epoch || *expiry != i64::try_from(reservation.expires_at_ms).unwrap_or(-1) || *expiry <= now || stored != &encoded) {
            return Err(DirectoryError::Conflict("artifact CAS reservation is missing, expired, or superseded".into()));
        }
        let event = event.ok_or_else(|| DirectoryError::Conflict("new artifact CAS publication requires one public event".into()))?;
        let full = self.persist_event(&tx, event)?;
        let checkpoint_payload = serde_json::Value::from(&checkpoint.to_value()).to_string();
        let event_seq = i64::try_from(full.seq).map_err(backend)?;
        tx.execute(
            "INSERT INTO hub_artifact_authority_journal(event_seq, space_id, document_id, checkpoint_id, payload) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![event_seq, checkpoint.scope.space_id, checkpoint.scope.document_id, checkpoint.checkpoint_id.0.as_slice(), checkpoint_payload],
        ).map_err(backend)?;
        self.project(&tx, &full)?;
        self.project_verified_checkpoint(&tx, &full, checkpoint)?;
        let generation = Self::cas_generation(&tx)?;
        tx.execute(
            "INSERT INTO hub_artifact_cas_ledger_journal(generation, operation, space_id, document_id, checkpoint_id, write_epoch, expires_at_ms, event_seq, plan) VALUES (?1, 'publish', ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![generation, reservation.plan.scope.space_id, reservation.plan.scope.document_id, reservation.plan.checkpoint_id.0.as_slice(), token_epoch, i64::try_from(reservation.expires_at_ms).map_err(backend)?, event_seq, encoded],
        ).map_err(backend)?;
        Self::cas_project_publish(&tx, reservation, generation)?;
        tx.commit().map_err(backend)?;
        Ok(vec![full])
    }

    async fn artifact_cas_ledger_generation(&self) -> DirectoryResult<u64> {
        let generation: i64 = self.lock()?.query_row("SELECT generation FROM hub_artifact_cas_ledger_head WHERE singleton = 1", [], |row| row.get(0)).map_err(backend)?;
        u64::try_from(generation).map_err(backend)
    }

    async fn artifact_cas_sweep_candidates(&self, after_generation: u64, through_generation: u64, limit: usize) -> DirectoryResult<ArtifactCasSweepCandidatePage> {
        if limit == 0 || limit > ARTIFACT_CAS_SWEEP_PAGE_MAX {
            return Err(DirectoryError::Conflict(format!("artifact CAS sweep page requires limit 1..={ARTIFACT_CAS_SWEEP_PAGE_MAX}")));
        }
        let after = i64::try_from(after_generation).map_err(backend)?;
        let through = i64::try_from(through_generation).map_err(backend)?;
        let limit = i64::try_from(limit).map_err(backend)?;
        let conn = self.lock()?;
        let current: i64 = conn.query_row("SELECT generation FROM hub_artifact_cas_ledger_head WHERE singleton = 1", [], |row| row.get(0)).map_err(backend)?;
        if through > current || after > through {
            return Err(DirectoryError::Conflict("artifact CAS sweep bounds are outside the ledger".into()));
        }
        let rows: Vec<(i64, Option<Vec<u8>>)> = {
            let mut statement = conn.prepare("SELECT generation, plan FROM hub_artifact_cas_ledger_journal WHERE generation > ?1 AND generation <= ?2 ORDER BY generation LIMIT ?3").map_err(backend)?;
            let mapped = statement.query_map(rusqlite::params![after, through, limit], |row| Ok((row.get(0)?, row.get(1)?))).map_err(backend)?;
            mapped.collect::<Result<Vec<_>, _>>().map_err(backend)?
        };
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

    async fn artifact_cas_delete_fence(&self, key: &ArtifactCasObjectKey, observed_generation: u64, now_ms: u64) -> DirectoryResult<Option<ArtifactCasDeleteFence>> {
        if observed_generation == 0 {
            return Err(DirectoryError::Conflict("artifact CAS sweep requires a nonzero observed generation".into()));
        }
        let observed = i64::try_from(observed_generation).map_err(backend)?;
        let now = i64::try_from(now_ms).map_err(backend)?;
        let conn = self.lock()?;
        let current: i64 = conn.query_row("SELECT generation FROM hub_artifact_cas_ledger_head WHERE singleton = 1", [], |row| row.get(0)).map_err(backend)?;
        if current < observed {
            return Err(DirectoryError::Conflict("artifact CAS sweep observation is ahead of the ledger".into()));
        }
        let referenced: i64 = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reference_object WHERE space_id = ?1 AND kind = ?2 AND object_digest = ?3)",
            rusqlite::params![key.space_id, key.kind.name(), key.digest.0.as_slice()],
            |row| row.get(0),
        ).map_err(backend)?;
        let reserved: i64 = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reservation_object object JOIN hub_artifact_cas_reservation reservation USING(space_id, document_id, checkpoint_id) WHERE object.space_id = ?1 AND object.kind = ?2 AND object.object_digest = ?3 AND reservation.expires_at_ms > ?4)",
            rusqlite::params![key.space_id, key.kind.name(), key.digest.0.as_slice(), now],
            |row| row.get(0),
        ).map_err(backend)?;
        Ok((referenced == 0 && reserved == 0).then(|| ArtifactCasDeleteFence::new(key.clone(), u64::try_from(current).unwrap_or(0))))
    }

    //#region EventLog
    async fn append_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<Vec<DirectoryEvent>> {
        if events.iter().any(|event| matches!(&event.body, DirectoryEventBody::ArtifactCheckpointPublished { .. })) {
            return Err(DirectoryError::Conflict("checkpoint publication requires the verified authority append seam".into()));
        }
        let mut conn = self.lock()?;
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(backend)?;
        let mut persisted = Vec::with_capacity(events.len());
        for event in events {
            let full = self.persist_event(&tx, event)?;
            self.project(&tx, &full)?;
            self.append_cas_release(&tx, &full)?;
            persisted.push(full);
        }
        tx.commit().map_err(backend)?;
        Ok(persisted)
    }

    async fn events_since(&self, since_seq: u64, limit: usize) -> DirectoryResult<Vec<DirectoryEvent>> {
        let (since_seq, limit) = bounded_event_read(since_seq, limit)?;
        let conn = self.lock()?;
        let mut stmt = conn.prepare("SELECT seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, payload, recorded_at FROM hub_directory_event WHERE seq > ?1 ORDER BY seq LIMIT ?2").map_err(backend)?;
        let rows = stmt.query_map(rusqlite::params![since_seq, limit], event_row).map_err(backend)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(backend)
    }

    async fn head_seq(&self) -> DirectoryResult<u64> {
        let seq: Option<i64> = self.lock()?.query_row("SELECT MAX(seq) FROM hub_directory_event", [], |row| row.get(0)).map_err(backend)?;
        u64::try_from(seq.unwrap_or(0)).map_err(backend)
    }

    async fn rebuild_projections(&self) -> DirectoryResult<u64> {
        self.rebuild_projections_controlled(&UNCONTROLLED_PROJECTION_REBUILD).await
    }

    async fn rebuild_projections_controlled(&self, control: &dyn ProjectionRebuildControl) -> DirectoryResult<u64> {
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        let event_count: i64 = tx.query_row("SELECT COUNT(*) FROM hub_directory_event", [], |row| row.get(0)).map_err(backend)?;
        let total = u64::try_from(event_count).map_err(backend)?;
        checkpoint_projection_rebuild(control, 0, total)?;
        tx.execute_batch(
            "DELETE FROM hub_artifact_cas_reservation; DELETE FROM hub_artifact_cas_reference; DELETE FROM hub_artifact_retention; DELETE FROM hub_artifact_checkpoint_private; DELETE FROM hub_artifact_checkpoint; DELETE FROM hub_document_descriptor; DELETE FROM hub_space_membership; DELETE FROM hub_space; DELETE FROM hub_user;",
        )
        .map_err(backend)?;
        let mut replayed = 0u64;
        let mut cursor = 0i64;
        const PAGE: i64 = 512;
        while replayed < total {
            let events: Vec<DirectoryEvent> = {
                let mut stmt = tx.prepare("SELECT seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, payload, recorded_at FROM hub_directory_event WHERE seq > ?1 ORDER BY seq LIMIT ?2").map_err(backend)?;
                let mapped = stmt.query_map(rusqlite::params![cursor, PAGE], event_row).map_err(backend)?;
                mapped.collect::<Result<Vec<_>, _>>().map_err(backend)?
            };
            if events.is_empty() {
                return Err(DirectoryError::Backend("directory event replay ended before its counted head".into()));
            }
            for event in &events {
                cursor = i64::try_from(event.seq).map_err(backend)?;
                self.project(&tx, event)?;
                if matches!(&event.body, DirectoryEventBody::ArtifactCheckpointPublished { .. }) {
                    let checkpoint = tx
                        .query_row("SELECT payload FROM hub_artifact_authority_journal WHERE event_seq = ?1", rusqlite::params![cursor], verified_checkpoint_row)
                        .optional()
                        .map_err(backend)?
                        .ok_or_else(|| DirectoryError::Backend(format!("missing private authority journal for checkpoint event {}", event.seq)))?;
                    self.project_verified_checkpoint(&tx, event, &checkpoint)?;
                }
                replayed += 1;
                checkpoint_projection_rebuild(control, replayed, total)?;
            }
        }
        let ledger_rows: Vec<(i64, String, String, Option<String>, Option<Vec<u8>>, Option<i64>, Option<i64>, Option<Vec<u8>>)> = {
            let mut statement = tx.prepare("SELECT generation, operation, space_id, document_id, checkpoint_id, write_epoch, expires_at_ms, plan FROM hub_artifact_cas_ledger_journal ORDER BY generation").map_err(backend)?;
            let mapped = statement.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?, row.get(7)?))).map_err(backend)?;
            mapped.collect::<Result<Vec<_>, _>>().map_err(backend)?
        };
        for (generation, operation, space_id, document_id, checkpoint_id, write_epoch, expires_at_ms, plan) in ledger_rows {
            match operation.as_str() {
                "reserve" => {
                    let plan = decode_artifact_cas_ownership_v1(plan.as_deref().ok_or_else(|| DirectoryError::Backend("artifact CAS reserve journal plan missing".into()))?).map_err(|error| DirectoryError::Backend(error.to_string()))?;
                    let reservation = ArtifactCasReservation {
                        plan,
                        generation: u64::try_from(generation).map_err(backend)?,
                        write_epoch: u64::try_from(write_epoch.ok_or_else(|| DirectoryError::Backend("artifact CAS reserve journal epoch missing".into()))?).map_err(backend)?,
                        expires_at_ms: u64::try_from(expires_at_ms.ok_or_else(|| DirectoryError::Backend("artifact CAS reserve journal expiry missing".into()))?).map_err(backend)?,
                    };
                    Self::cas_project_reserve(&tx, &reservation)?;
                }
                "publish" => {
                    let plan = decode_artifact_cas_ownership_v1(plan.as_deref().ok_or_else(|| DirectoryError::Backend("artifact CAS publish journal plan missing".into()))?).map_err(|error| DirectoryError::Backend(error.to_string()))?;
                    let reservation = ArtifactCasReservation {
                        plan,
                        generation: u64::try_from(generation).map_err(backend)?,
                        write_epoch: u64::try_from(write_epoch.ok_or_else(|| DirectoryError::Backend("artifact CAS publish journal epoch missing".into()))?).map_err(backend)?,
                        expires_at_ms: u64::try_from(expires_at_ms.ok_or_else(|| DirectoryError::Backend("artifact CAS publish journal expiry missing".into()))?).map_err(backend)?,
                    };
                    Self::cas_project_publish(&tx, &reservation, generation)?;
                }
                "retention" | "space-delete" => {
                    let checkpoint_id = checkpoint_id.map(|bytes| bytes.try_into().map(ArtifactHash).map_err(|_: Vec<u8>| DirectoryError::Backend("artifact CAS release journal checkpoint width".into()))).transpose()?;
                    Self::cas_project_release(&tx, &operation, &space_id, document_id.as_deref(), checkpoint_id)?;
                }
                _ => return Err(DirectoryError::Backend("artifact CAS ledger operation is invalid".into())),
            }
        }
        tx.commit().map_err(backend)?;
        Ok(replayed)
    }
    //#endregion
}

fn user_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<UserRecord> {
    Ok(UserRecord { id: row.get(0)?, email: row.get(1)?, display_name: row.get(2)?, password_hash: row.get(3)?, sso_subject: row.get(4)?, sso_provider: row.get(5)?, created_at: row.get(6)? })
}

fn sync_session_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<SyncSessionRecord> {
    let space_role: Option<String> = row.get(8)?;
    Ok(SyncSessionRecord {
        id: row.get(0)?,
        auth_session_id: row.get(1)?,
        authorization_generation: u64::try_from(row.get::<_, i64>(2)?).unwrap_or(0),
        actor_id: row.get(3)?,
        space_id: row.get(4)?,
        document_id: row.get(5)?,
        surface: row.get(6)?,
        user_id: row.get(7)?,
        space_role: space_role.and_then(|r| SpaceRole::parse(&r)),
        client_label: row.get(9)?,
        connected_at: row.get(10)?,
        disconnected_at: row.get(11)?,
    })
}

fn invite_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<InviteRecord> {
    let role: String = row.get(4)?;
    Ok(InviteRecord {
        id: row.get(0)?,
        selector: row.get(1)?,
        secret_digest: blob32(row, 2)?,
        space_id: row.get(3)?,
        role: SpaceRole::parse(&role).unwrap_or(SpaceRole::Spectator),
        created_at: row.get(5)?,
        expires_at: row.get(6)?,
        revoked_at: row.get(7)?,
        revoked_reason: row.get(8)?,
        accepted_at: row.get(9)?,
    })
}

fn auth_session_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<AuthSessionRecord> {
    let generation = row.get::<_, i64>(10)?;
    let session_kind: String = row.get(12)?;
    Ok(AuthSessionRecord {
        id: row.get(0)?,
        selector: row.get(1)?,
        secret_digest: blob32(row, 2)?,
        user_id: row.get(3)?,
        identity_provider: row.get(4)?,
        identity_subject_digest: blob32(row, 5)?,
        issued_at: row.get(6)?,
        expires_at: row.get(7)?,
        revoked_at: row.get(8)?,
        revoked_reason: row.get(9)?,
        authorization_generation: u64::try_from(generation).unwrap_or(0),
        device_instance_id: row.get(11)?,
        session_kind: AuthSessionKind::parse(&session_kind).unwrap_or(AuthSessionKind::External),
    })
}

fn auth_audit_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<AuthAuditRecord> {
    Ok(AuthAuditRecord {
        id: row.get(0)?,
        occurred_at: row.get(1)?,
        event_kind: row.get(2)?,
        auth_session_id: row.get(3)?,
        target_user_id: row.get(4)?,
        actor_user_id: row.get(5)?,
        provider: row.get(6)?,
        outcome_code: row.get(7)?,
        reason_code: row.get(8)?,
        correlation_id: row.get(9)?,
        peer_class: row.get(10)?,
    })
}

fn document_descriptor_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<DocumentDescriptor> {
    Ok(DocumentDescriptor {
        space_id: row.get(0)?,
        document_id: row.get(1)?,
        artifact_kind: row.get(2)?,
        artifact_schema: row.get(3)?,
        owner: DocumentOwner { plugin_id: row.get(4)?, package_id: row.get(5)?, version: row.get(6)?, package_hash: row.get(7)? },
        pack_schema_hash: row.get(8)?,
        bootstrap_version: row.get::<_, i64>(9)? as u32,
        bootstrap_frontier: DocumentFrontier { head_seq: row.get::<_, i64>(10)? as u64, commit_seq: row.get::<_, i64>(11)? as u64, epoch: row.get::<_, i64>(12)? as u64 },
        bootstrap_snapshot_hash: row.get(13)?,
    })
}

fn published_checkpoint_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<PublishedArtifactCheckpoint> {
    let payload: String = row.get(0)?;
    let value: serde_json::Value = serde_json::from_str(&payload).map_err(|error| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(error)))?;
    PublishedArtifactCheckpoint::from_value(DslValue::from(value)).map_err(|error| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(error)))
}

fn verified_checkpoint_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ArtifactCheckpoint> {
    let payload: String = row.get(0)?;
    let value: serde_json::Value = serde_json::from_str(&payload).map_err(|error| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(error)))?;
    ArtifactCheckpoint::from_value(DslValue::from(value)).map_err(|error| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(error)))
}

fn artifact_retention_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ArtifactRetention> {
    let payload: String = row.get(0)?;
    let value: serde_json::Value = serde_json::from_str(&payload).map_err(|error| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(error)))?;
    ArtifactRetention::from_value(DslValue::from(value)).map_err(|error| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(error)))
}

fn event_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<DirectoryEvent> {
    let seq: i64 = row.get(0)?;
    let id: String = row.get(1)?;
    let hlc_physical: i64 = row.get(2)?;
    let hlc_logical: i64 = row.get(3)?;
    let actor_kind: String = row.get(4)?;
    let actor_id: String = row.get(5)?;
    let space_id: Option<String> = row.get(6)?;
    let user_id: Option<String> = row.get(7)?;
    let payload: String = row.get(8)?;
    let recorded_at_ms: i64 = row.get(9)?;
    let value: serde_json::Value = serde_json::from_str(&payload).map_err(|error| rusqlite::Error::FromSqlConversionFailure(8, rusqlite::types::Type::Text, Box::new(error)))?;
    let body = DirectoryEventBody::from_value(DslValue::from(value)).map_err(|error| rusqlite::Error::FromSqlConversionFailure(8, rusqlite::types::Type::Text, Box::new(error)))?;
    let seq = u64::try_from(seq).map_err(|error| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Integer, Box::new(error)))?;
    let logical = u32::try_from(hlc_logical).map_err(|error| rusqlite::Error::FromSqlConversionFailure(3, rusqlite::types::Type::Integer, Box::new(error)))?;
    Ok(DirectoryEvent { seq, id, hlc: Hlc { physical_ms: hlc_physical, logical }, actor: DirectoryActor { kind: actor_kind_from_str(&actor_kind), id: actor_id }, space_id, user_id, body, recorded_at_ms })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(serde::Deserialize)]
    struct ShareTokenVectors {
        encoding: Vec<ShareTokenEncodingVector>,
        scope: ShareTokenScopeVector,
    }

    #[derive(serde::Deserialize)]
    struct ShareTokenEncodingVector {
        bytes: Vec<u8>,
        hex: String,
    }

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ShareTokenScope {
        space_id: String,
        document_id: String,
    }

    #[derive(serde::Deserialize)]
    struct ShareTokenScopeVector {
        grant: ShareTokenScope,
        allowed: ShareTokenScope,
        denied: ShareTokenScope,
    }

    fn actor(id: &str) -> DirectoryActor {
        DirectoryActor { kind: DirectoryActorKind::User, id: id.to_string() }
    }

    /// 🌱️ `create_space`/`upsert_membership` were removed (writes now go through
    /// `append_events` — see the module root's `//#region 🔖️Decider`); this recreates just enough
    /// of a `create-space` decision by hand so backend tests do not need a full `DirectoryService`.
    async fn seed_space(dir: &SqliteDirectory, clock: &mut HubClock, owner_user_id: &str, kind: DirectorySpaceKind) -> String {
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

    // 🔬️ Users, spaces, and role-based membership round-trip over the event log.
    #[tokio::test]
    async fn user_space_membership_round_trip() {
        let directory = SqliteDirectory::connect(":memory:").await.expect("connect");
        let mut clock = HubClock::new();
        let user = directory.create_user("a@example.com", "Ada", None, None, None).await.expect("create user");
        let space_id = seed_space(&directory, &mut clock, &user.id, DirectorySpaceKind::Studio).await;
        assert_eq!(directory.get_role(&space_id, &user.id).await.unwrap(), Some(SpaceRole::Author));

        let member = directory.create_user("b@example.com", "Bob", None, None, None).await.expect("create user 2");
        directory
            .append_events(&[NewDirectoryEvent {
                hlc: clock.tick(),
                actor: actor(&user.id),
                space_id: Some(space_id.clone()),
                user_id: Some(member.id.clone()),
                body: DirectoryEventBody::MemberUpserted { space_id: space_id.clone(), user_id: member.id.clone(), role: DirectorySpaceRole::Spectator },
            }])
            .await
            .expect("add member");
        assert_eq!(directory.get_role(&space_id, &member.id).await.unwrap(), Some(SpaceRole::Spectator));
        let spaces = directory.list_spaces_for_user(&member.id).await.unwrap();
        assert_eq!(spaces.len(), 1);

        directory
            .append_events(&[NewDirectoryEvent {
                hlc: clock.tick(),
                actor: actor(&user.id),
                space_id: Some(space_id.clone()),
                user_id: Some(member.id.clone()),
                body: DirectoryEventBody::MemberRemoved { space_id: space_id.clone(), user_id: member.id.clone() },
            }])
            .await
            .expect("remove member");
        assert_eq!(directory.get_role(&space_id, &member.id).await.unwrap(), None);
    }

    // 🔬️ SyncSession open/close is durable, listable, filterable by space, and boot-time cleanup
    // closes every still-open session at once.
    #[tokio::test]
    async fn sync_session_lifecycle() {
        let directory = SqliteDirectory::connect(":memory:").await.expect("connect");
        directory.seed().await.expect("seed");
        let session = directory.record_sync_session_open(None, 0, "s.space@1/*#editor", "default", "default", "default", None, None, "test-client").await.expect("open");
        assert!(session.disconnected_at.is_none());
        assert_eq!(directory.list_active_sync_sessions(Some("default")).await.unwrap().len(), 1);

        directory.record_sync_session_close(&session.id).await.expect("close");
        let sessions = directory.list_sync_sessions_for_document("default").await.unwrap();
        assert_eq!(sessions.len(), 1);
        assert!(sessions[0].disconnected_at.is_some());
        assert!(directory.list_active_sync_sessions(Some("default")).await.unwrap().is_empty());

        directory.record_sync_session_open(None, 0, "s.space@1/*#viewer", "default", "default", "default", None, None, "test-client-2").await.expect("open 2");
        directory.close_all_sync_sessions().await.expect("close all");
        assert!(directory.list_active_sync_sessions(None).await.unwrap().is_empty());
    }

    // 🔬️ Language-neutral vectors validate owned hex encoding against SQLite's independent
    // `hex()` oracle and describe the cross-space authorization boundary shared by every backend.
    #[test]
    fn share_token_vectors_match_sqlite_hex_oracle() {
        let vectors: ShareTokenVectors = serde_json::from_str(include_str!("../🧪️tests/🔣️share-token-vectors.json")).expect("share-token vectors");
        let oracle = Connection::open_in_memory().expect("sqlite oracle");
        for vector in vectors.encoding {
            let actual = crate::directory::encode_capability_bytes(&vector.bytes);
            let sqlite_hex: String = oracle.query_row("SELECT lower(hex(?1))", rusqlite::params![vector.bytes], |row| row.get(0)).expect("sqlite hex");
            assert_eq!(actual, vector.hex);
            assert_eq!(actual, sqlite_hex);
        }
    }

    // 🔬️ Share grants are private by default, space/document scoped, revocable, and expiring.
    #[tokio::test]
    async fn share_token_lifecycle_and_scope() {
        let directory = SqliteDirectory::connect(":memory:").await.expect("connect");
        directory.seed().await.expect("seed");
        let vectors: ShareTokenVectors = serde_json::from_str(include_str!("../🧪️tests/🔣️share-token-vectors.json")).expect("share-token vectors");
        let grant_scope = DocumentScope::new(vectors.scope.grant.space_id, vectors.scope.grant.document_id);
        let allowed_scope = DocumentScope::new(vectors.scope.allowed.space_id, vectors.scope.allowed.document_id);
        let denied_scope = DocumentScope::new(vectors.scope.denied.space_id, vectors.scope.denied.document_id);

        let grant = directory.issue_share_token(&grant_scope, 60, "share-lifecycle").await.expect("mint token");
        assert!(grant.capability.expose_once().starts_with("share.v1."));
        assert!(directory.authenticate_share(&allowed_scope, &grant.capability).await.unwrap());
        assert!(!directory.authenticate_share(&denied_scope, &grant.capability).await.unwrap());

        directory.revoke_share_token(&grant_scope, &grant.record.id, "test-revoke", "share-lifecycle").await.expect("revoke token");
        assert!(!directory.authenticate_share(&allowed_scope, &grant.capability).await.unwrap());
        assert!(matches!(directory.revoke_share_token(&grant_scope, &grant.record.id, "test-revoke", "share-lifecycle").await, Err(DirectoryError::NotFound(_))));

        let expiring_scope = DocumentScope::new("space-a", "expiring");
        let expiring = directory.issue_share_token(&expiring_scope, 60, "share-expiry").await.expect("mint expiring token");
        directory.lock().unwrap().execute("UPDATE hub_share_grant SET expires_at = ?2 WHERE id = ?1", rusqlite::params![expiring.record.id, now_ms() - 1]).unwrap();
        assert!(!directory.authenticate_share(&expiring_scope, &expiring.capability).await.unwrap());
        assert!(matches!(directory.issue_share_token(&expiring_scope, 0, "share-invalid").await, Err(DirectoryError::Conflict(_))));
    }

    #[tokio::test]
    async fn auth_session_storage_is_digest_only_and_revoke_returns_generation() {
        let directory = SqliteDirectory::connect(":memory:").await.expect("connect");
        directory.seed().await.expect("seed");
        let issue = AuthSessionIssue {
            user_id: "seed".into(),
            identity_provider: "oidc.example".into(),
            identity_subject_digest: crate::directory::identity_subject_digest("oidc.example", "sub-123").expect("subject digest"),
            ttl_secs: 60,
            device_instance_id: "device-a".into(),
            session_kind: AuthSessionKind::External,
            correlation_id: "issue-correlation".into(),
            peer_class: "loopback-test".into(),
        };
        let issued = directory.issue_auth_session(&issue).await.expect("issue session");
        let share = directory.issue_share_token(&DocumentScope::new("default", "auth-storage-test"), 60, "share-storage").await.expect("issue share");
        let invite = directory.issue_invite("default", SpaceRole::Spectator, 60, "invite-storage").await.expect("issue invite");
        let raw = issued.capability.expose_once();
        let secret_hex = raw.rsplit('.').next().expect("raw secret");
        let (selector, digest_hex): (String, String) = directory
            .lock()
            .expect("sqlite lock")
            .query_row("SELECT selector, lower(hex(secret_digest)) FROM hub_auth_session WHERE id = ?1", [&issued.record.id], |row| Ok((row.get(0)?, row.get(1)?)))
            .expect("stored session authority");
        assert_eq!(selector, issued.capability.selector());
        assert_eq!(digest_hex, crate::directory::encode_capability_bytes(&issued.capability.secret_digest()));
        assert_ne!(digest_hex, secret_hex);
        assert!(!format!("{:?}", issued.capability).contains(secret_hex));
        let share_raw = share.capability.expose_once();
        let invite_raw = invite.capability.expose_once();
        let conn = directory.lock().expect("sqlite storage inspection");
        for (table, id, raw_capability) in [("hub_share_grant", &share.record.id, &share_raw), ("hub_space_invite", &invite.record.id, &invite_raw)] {
            let sql = format!("SELECT selector, lower(hex(secret_digest)) FROM {table} WHERE id = ?1");
            let (stored_selector, stored_digest): (String, String) = conn.query_row(&sql, [id], |row| Ok((row.get(0)?, row.get(1)?))).expect("stored scoped capability");
            assert!(raw_capability.contains(&stored_selector));
            assert!(!raw_capability.ends_with(&stored_digest));
        }
        drop(conn);
        assert_eq!(directory.authenticate_session(&issued.capability).await.expect("authenticate").expect("active").authorization_generation, 1);

        let revoked = directory
            .revoke_auth_sessions_for_user("seed", "security-test", Some("seed"), "revoke-correlation")
            .await
            .expect("revoke user sessions");
        assert_eq!(revoked.len(), 1);
        assert_eq!(revoked[0].id, issued.record.id);
        assert_eq!(revoked[0].authorization_generation, 2);
        assert!(directory.authenticate_session(&issued.capability).await.expect("authenticate revoked").is_none());
        let audit = directory.list_auth_audit(AUTH_AUDIT_PAGE_MAX, 0).await.expect("auth audit");
        let audit_text = format!("{audit:?}");
        assert!(!audit_text.contains(&raw));
        assert!(!audit_text.contains(secret_hex));
        assert!(audit.iter().any(|entry| entry.event_kind == "session-issued"));
        assert!(audit.iter().any(|entry| entry.event_kind == "session-revoked" && entry.reason_code.as_deref() == Some("security-test")));
    }

    // 🔬️ `seed()` (now event-sourced) still leaves a dense, replayable log.
    #[tokio::test]
    async fn seed_is_replayable() {
        let directory = SqliteDirectory::connect(":memory:").await.expect("connect");
        directory.seed().await.expect("seed");
        assert_eq!(directory.head_seq().await.unwrap(), 3);
        let before = directory.get_space("default").await.unwrap();
        let replayed = directory.rebuild_projections().await.expect("rebuild");
        assert_eq!(replayed, 3);
        assert_eq!(directory.get_space("default").await.unwrap(), before);
    }
}
//#endregion 🧪️Tests
