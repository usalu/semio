//! 🗄️ The dedicated SQLite artifact CAS table and its storage port.

use super::*;

/// 🪶️ Dedicated SQLite artifact CAS table, separate from generic database payloads.
pub struct SqliteArtifactChunkCasStorage {
    connection: Arc<Mutex<rusqlite::Connection>>,
}

impl SqliteArtifactChunkCasStorage {
    const SCHEMA: &'static str = "CREATE TABLE IF NOT EXISTS hub_artifact_cas_object (
        space_digest BLOB NOT NULL CHECK(length(space_digest) = 32),
        kind TEXT NOT NULL CHECK(kind IN ('chunk', 'manifest')),
        object_digest BLOB NOT NULL CHECK(length(object_digest) = 32),
        bytes BLOB NOT NULL,
        byte_length INTEGER NOT NULL,
        PRIMARY KEY(space_digest, kind, object_digest),
        CHECK(byte_length = length(bytes)),
        CHECK((kind = 'chunk' AND byte_length BETWEEN 1 AND 262144) OR (kind = 'manifest' AND byte_length BETWEEN 1 AND 65536))
    );
    CREATE TABLE IF NOT EXISTS hub_artifact_cas_coordinator (
        singleton INTEGER PRIMARY KEY CHECK(singleton = 1),
        coordinator_id BLOB NOT NULL CHECK(length(coordinator_id) = 32)
    );
    CREATE TABLE IF NOT EXISTS hub_artifact_cas_space_fence (
        coordinator_id BLOB NOT NULL CHECK(length(coordinator_id) = 32),
        space_digest BLOB NOT NULL CHECK(length(space_digest) = 32),
        physical_epoch INTEGER NOT NULL CHECK(physical_epoch >= 1),
        updated_at_ms INTEGER NOT NULL,
        PRIMARY KEY(coordinator_id, space_digest)
    )";

    /// 🔌️ Opens and bootstraps one durable dedicated CAS table.
    pub async fn open(path: &Path) -> Result<Self, AuthorityError> {
        let path = path.to_path_buf();
        let connection = tokio::task::spawn_blocking(move || -> Result<rusqlite::Connection, AuthorityError> {
            let connection = rusqlite::Connection::open(path).map_err(|_| AuthorityError::Store("artifact CAS SQLite open failed".to_string()))?;
            connection.pragma_update(None, "journal_mode", "WAL").map_err(|_| AuthorityError::Store("artifact CAS SQLite WAL setup failed".to_string()))?;
            connection.pragma_update(None, "synchronous", "FULL").map_err(|_| AuthorityError::Store("artifact CAS SQLite durability setup failed".to_string()))?;
            connection.execute_batch(Self::SCHEMA).map_err(|_| AuthorityError::Store("artifact CAS SQLite schema failed".to_string()))?;
            Ok(connection)
        })
        .await
        .map_err(|_| AuthorityError::Store("artifact CAS SQLite worker failed".to_string()))??;
        Ok(Self { connection: Arc::new(Mutex::new(connection)) })
    }

    #[cfg(test)]
    pub(super) async fn memory() -> Result<Self, AuthorityError> {
        let connection = rusqlite::Connection::open_in_memory().map_err(|_| AuthorityError::Store("artifact CAS SQLite memory open failed".to_string()))?;
        connection.execute_batch(Self::SCHEMA).map_err(|_| AuthorityError::Store("artifact CAS SQLite memory schema failed".to_string()))?;
        Ok(Self { connection: Arc::new(Mutex::new(connection)) })
    }
}

impl ArtifactChunkCasStorage for SqliteArtifactChunkCasStorage {
    async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        if coordinator_id == [0; 32] {
            return Err(AuthorityError::Store("artifact CAS coordinator identity is invalid".into()));
        }
        let connection = self.connection.clone();
        tokio::task::spawn_blocking(move || -> Result<(), AuthorityError> {
            let mut connection = connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let tx = connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(|_| AuthorityError::Store("artifact CAS SQLite transaction failed".into()))?;
            tx.execute("INSERT OR IGNORE INTO hub_artifact_cas_coordinator(singleton, coordinator_id) VALUES (1, ?1)", [coordinator_id.as_slice()]).map_err(|_| AuthorityError::Store("artifact CAS SQLite coordinator write failed".into()))?;
            let current: Vec<u8> = tx.query_row("SELECT coordinator_id FROM hub_artifact_cas_coordinator WHERE singleton = 1", [], |row| row.get(0)).map_err(|_| AuthorityError::Store("artifact CAS SQLite coordinator read failed".into()))?;
            if current != coordinator_id {
                return Err(AuthorityError::Store("artifact CAS coordinator identity mismatch".into()));
            }
            tx.commit().map_err(|_| AuthorityError::Store("artifact CAS SQLite commit failed".into()))?;
            Ok(())
        })
        .await
        .map_err(|_| AuthorityError::Store("artifact CAS SQLite worker failed".into()))??;
        context.checkpoint()
    }

    async fn advance_physical_epoch(&self, coordinator_id: [u8; 32], space_id: &str, epoch: u64, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        if epoch == 0 {
            return Err(AuthorityError::Store("artifact CAS physical epoch is invalid".into()));
        }
        let connection = self.connection.clone();
        let space = space_digest(space_id)?.0.to_vec();
        let epoch = i64::try_from(epoch).map_err(|_| AuthorityError::Store("artifact CAS physical epoch overflow".into()))?;
        let now = i64::try_from(context.now_ms()).unwrap_or(i64::MAX);
        tokio::task::spawn_blocking(move || -> Result<(), AuthorityError> {
            use rusqlite::OptionalExtension as _;
            let mut connection = connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let tx = connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(|_| AuthorityError::Store("artifact CAS SQLite transaction failed".into()))?;
            let current: Option<Vec<u8>> = tx.query_row("SELECT coordinator_id FROM hub_artifact_cas_coordinator WHERE singleton = 1", [], |row| row.get(0)).optional().map_err(|_| AuthorityError::Store("artifact CAS SQLite coordinator read failed".into()))?;
            if current.as_deref() != Some(coordinator_id.as_slice()) { return Err(AuthorityError::Store("artifact CAS coordinator identity mismatch".into())); }
            tx.execute("INSERT INTO hub_artifact_cas_space_fence(coordinator_id, space_digest, physical_epoch, updated_at_ms) VALUES (?1, ?2, ?3, ?4) ON CONFLICT(coordinator_id, space_digest) DO UPDATE SET physical_epoch = MAX(physical_epoch, excluded.physical_epoch), updated_at_ms = excluded.updated_at_ms", rusqlite::params![coordinator_id.as_slice(), space, epoch, now]).map_err(|_| AuthorityError::Store("artifact CAS SQLite physical epoch advance failed".into()))?;
            tx.commit().map_err(|_| AuthorityError::Store("artifact CAS SQLite commit failed".into()))?;
            Ok(())
        }).await.map_err(|_| AuthorityError::Store("artifact CAS SQLite worker failed".into()))??;
        context.checkpoint()
    }

    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactCasPutOutcome, AuthorityError> {
        context.checkpoint()?;
        validate_object(key, bytes)?;
        let connection = self.connection.clone();
        let tenant = space_digest(&key.space_id)?.0.to_vec();
        let kind = key.kind.name().to_string();
        let digest = key.digest.0.to_vec();
        let expected = bytes.to_vec();
        let length = i64::try_from(expected.len()).map_err(|_| AuthorityError::ResourceLimit("artifact CAS object byte"))?;
        let outcome = tokio::task::spawn_blocking(move || -> Result<ArtifactCasPutOutcome, AuthorityError> {
            use rusqlite::OptionalExtension as _;
            let mut connection = connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let transaction = connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(|_| AuthorityError::Store("artifact CAS SQLite transaction failed".to_string()))?;
            let inserted = transaction
                .execute("INSERT OR IGNORE INTO hub_artifact_cas_object(space_digest, kind, object_digest, bytes, byte_length) VALUES (?1, ?2, ?3, ?4, ?5)", rusqlite::params![tenant, kind, digest, expected, length])
                .map_err(|_| AuthorityError::Store("artifact CAS SQLite insert failed".to_string()))?
                == 1;
            let stored_length: Option<i64> = transaction
                .query_row("SELECT byte_length FROM hub_artifact_cas_object WHERE space_digest = ?1 AND kind = ?2 AND object_digest = ?3", rusqlite::params![tenant, kind, digest], |row| row.get(0))
                .optional()
                .map_err(|_| AuthorityError::Store("artifact CAS SQLite length read failed".to_string()))?;
            if stored_length != Some(length) {
                return Err(AuthorityError::Store("artifact CAS immutable key collision".to_string()));
            }
            let stored: Vec<u8> = transaction
                .query_row("SELECT bytes FROM hub_artifact_cas_object WHERE space_digest = ?1 AND kind = ?2 AND object_digest = ?3", rusqlite::params![tenant, kind, digest], |row| row.get(0))
                .map_err(|_| AuthorityError::Store("artifact CAS SQLite object read failed".to_string()))?;
            if stored != expected {
                return Err(AuthorityError::Store("artifact CAS immutable key collision".to_string()));
            }
            transaction.commit().map_err(|_| AuthorityError::Store("artifact CAS SQLite commit failed".to_string()))?;
            Ok(if inserted { ArtifactCasPutOutcome::Inserted } else { ArtifactCasPutOutcome::AlreadyPresent })
        })
        .await
        .map_err(|_| AuthorityError::Store("artifact CAS SQLite worker failed".to_string()))??;
        context.checkpoint()?;
        Ok(outcome)
    }

    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
        context.checkpoint()?;
        let connection = self.connection.clone();
        let tenant = space_digest(&key.space_id)?.0.to_vec();
        let kind = key.kind.name().to_string();
        let digest = key.digest.0.to_vec();
        let maximum = key.kind.maximum_bytes();
        let bytes = tokio::task::spawn_blocking(move || -> Result<Vec<u8>, AuthorityError> {
            use rusqlite::OptionalExtension as _;
            let connection = connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let length: i64 = connection
                .query_row("SELECT byte_length FROM hub_artifact_cas_object WHERE space_digest = ?1 AND kind = ?2 AND object_digest = ?3", rusqlite::params![tenant, kind, digest], |row| row.get(0))
                .optional()
                .map_err(|_| AuthorityError::Store("artifact CAS SQLite length read failed".to_string()))?
                .ok_or_else(|| AuthorityError::Store("artifact CAS object not found".to_string()))?;
            if length < 0 || usize::try_from(length).ok().is_none_or(|length| length > maximum) {
                return Err(AuthorityError::ResourceLimit("artifact CAS object byte"));
            }
            connection
                .query_row("SELECT bytes FROM hub_artifact_cas_object WHERE space_digest = ?1 AND kind = ?2 AND object_digest = ?3", rusqlite::params![tenant, kind, digest], |row| row.get(0))
                .map_err(|_| AuthorityError::Store("artifact CAS SQLite object read failed".to_string()))
        })
        .await
        .map_err(|_| AuthorityError::Store("artifact CAS SQLite worker failed".to_string()))??;
        validate_object(key, &bytes)?;
        context.checkpoint()?;
        Ok(bytes)
    }

    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, AuthorityError> {
        context.checkpoint()?;
        if !fence.permits(key) {
            return Err(AuthorityError::Store("artifact CAS deletion fence mismatch".to_string()));
        }
        let connection = self.connection.clone();
        let tenant = space_digest(&key.space_id)?.0.to_vec();
        let kind = key.kind.name().to_string();
        let digest = key.digest.0.to_vec();
        let coordinator_id = *fence.coordinator_id();
        let physical_epoch = i64::try_from(fence.physical_epoch()).map_err(|_| AuthorityError::Store("artifact CAS physical epoch overflow".into()))?;
        let deleted = tokio::task::spawn_blocking(move || -> Result<bool, AuthorityError> {
            use rusqlite::OptionalExtension as _;
            let mut connection = connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let transaction = connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(|_| AuthorityError::Store("artifact CAS SQLite transaction failed".into()))?;
            let current: Option<i64> = transaction
                .query_row("SELECT physical_epoch FROM hub_artifact_cas_space_fence WHERE coordinator_id = ?1 AND space_digest = ?2", rusqlite::params![coordinator_id.as_slice(), tenant], |row| row.get(0))
                .optional()
                .map_err(|_| AuthorityError::Store("artifact CAS SQLite physical epoch read failed".into()))?;
            if current != Some(physical_epoch) {
                return Err(AuthorityError::Store("artifact CAS deletion fence is stale".into()));
            }
            let deleted = transaction
                .execute("DELETE FROM hub_artifact_cas_object WHERE space_digest = ?1 AND kind = ?2 AND object_digest = ?3", rusqlite::params![tenant, kind, digest])
                .map_err(|_| AuthorityError::Store("artifact CAS SQLite delete failed".to_string()))?
                == 1;
            transaction.commit().map_err(|_| AuthorityError::Store("artifact CAS SQLite commit failed".into()))?;
            Ok(deleted)
        })
        .await
        .map_err(|_| AuthorityError::Store("artifact CAS SQLite worker failed".to_string()))??;
        Ok(if deleted { ArtifactCasDeleteOutcome::Deleted } else { ArtifactCasDeleteOutcome::Missing })
    }
}
