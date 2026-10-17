//! 🐘️ The dedicated Postgres artifact CAS and its storage port.

use super::*;

/// 🐘️ Dedicated PostgreSQL artifact CAS table.
pub struct PostgresArtifactChunkCasStorage {
    pool: sqlx_postgres::PgPool,
}

impl PostgresArtifactChunkCasStorage {
    /// 🔌️ Connects and bootstraps the dedicated CAS table.
    pub async fn connect(database_url: &str) -> Result<Self, AuthorityError> {
        let pool = sqlx_postgres::PgPoolOptions::new().max_connections(16).connect(database_url).await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL connect failed".to_string()))?;
        sqlx_core::query::query(
            "CREATE TABLE IF NOT EXISTS hub_artifact_cas_object (
            space_digest BYTEA NOT NULL CHECK(octet_length(space_digest) = 32),
            kind TEXT NOT NULL CHECK(kind IN ('chunk', 'manifest')),
            object_digest BYTEA NOT NULL CHECK(octet_length(object_digest) = 32),
            bytes BYTEA NOT NULL,
            byte_length BIGINT NOT NULL,
            PRIMARY KEY(space_digest, kind, object_digest),
            CHECK(byte_length = octet_length(bytes)),
            CHECK((kind = 'chunk' AND byte_length BETWEEN 1 AND 262144) OR (kind = 'manifest' AND byte_length BETWEEN 1 AND 65536))
        )",
        )
        .execute(&pool)
        .await
        .map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL schema failed".to_string()))?;
        sqlx_core::query::query("CREATE TABLE IF NOT EXISTS hub_artifact_cas_coordinator (singleton BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK(singleton), coordinator_id BYTEA NOT NULL CHECK(octet_length(coordinator_id) = 32))")
            .execute(&pool)
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL coordinator schema failed".into()))?;
        sqlx_core::query::query("CREATE TABLE IF NOT EXISTS hub_artifact_cas_space_fence (coordinator_id BYTEA NOT NULL CHECK(octet_length(coordinator_id) = 32), space_digest BYTEA NOT NULL CHECK(octet_length(space_digest) = 32), physical_epoch BIGINT NOT NULL CHECK(physical_epoch >= 1), updated_at_ms BIGINT NOT NULL, PRIMARY KEY(coordinator_id, space_digest))").execute(&pool).await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL fence schema failed".into()))?;
        Ok(Self { pool })
    }
}

impl ArtifactChunkCasStorage for PostgresArtifactChunkCasStorage {
    async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        if coordinator_id == [0; 32] {
            return Err(AuthorityError::Store("artifact CAS coordinator identity is invalid".into()));
        }
        let mut transaction = self.pool.begin().await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL transaction failed".into()))?;
        sqlx_core::query::query("INSERT INTO hub_artifact_cas_coordinator(singleton, coordinator_id) VALUES (TRUE, $1) ON CONFLICT(singleton) DO NOTHING")
            .bind(coordinator_id.as_slice())
            .execute(&mut *transaction)
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL coordinator write failed".into()))?;
        let (current,): (Vec<u8>,) = sqlx_core::query_as::query_as("SELECT coordinator_id FROM hub_artifact_cas_coordinator WHERE singleton FOR UPDATE")
            .fetch_one(&mut *transaction)
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL coordinator read failed".into()))?;
        if current != coordinator_id {
            return Err(AuthorityError::Store("artifact CAS coordinator identity mismatch".into()));
        }
        transaction.commit().await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL commit failed".into()))?;
        Ok(())
    }

    async fn advance_physical_epoch(&self, coordinator_id: [u8; 32], space_id: &str, epoch: u64, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        if epoch == 0 {
            return Err(AuthorityError::Store("artifact CAS physical epoch is invalid".into()));
        }
        let tenant = space_digest(space_id)?.0.to_vec();
        let epoch = i64::try_from(epoch).map_err(|_| AuthorityError::Store("artifact CAS physical epoch overflow".into()))?;
        let mut transaction = self.pool.begin().await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL transaction failed".into()))?;
        let (current,): (Vec<u8>,) = sqlx_core::query_as::query_as("SELECT coordinator_id FROM hub_artifact_cas_coordinator WHERE singleton FOR UPDATE")
            .fetch_one(&mut *transaction)
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL coordinator read failed".into()))?;
        if current != coordinator_id {
            return Err(AuthorityError::Store("artifact CAS coordinator identity mismatch".into()));
        }
        sqlx_core::query::query("INSERT INTO hub_artifact_cas_space_fence(coordinator_id, space_digest, physical_epoch, updated_at_ms) VALUES ($1,$2,$3,$4) ON CONFLICT(coordinator_id, space_digest) DO UPDATE SET physical_epoch = GREATEST(hub_artifact_cas_space_fence.physical_epoch, excluded.physical_epoch), updated_at_ms = excluded.updated_at_ms").bind(coordinator_id.as_slice()).bind(tenant).bind(epoch).bind(i64::try_from(context.now_ms()).unwrap_or(i64::MAX)).execute(&mut *transaction).await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL physical epoch advance failed".into()))?;
        transaction.commit().await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL commit failed".into()))?;
        context.checkpoint()
    }

    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactCasPutOutcome, AuthorityError> {
        context.checkpoint()?;
        validate_object(key, bytes)?;
        let tenant = space_digest(&key.space_id)?.0.to_vec();
        let digest = key.digest.0.to_vec();
        let length = i64::try_from(bytes.len()).map_err(|_| AuthorityError::ResourceLimit("artifact CAS object byte"))?;
        let mut transaction = self.pool.begin().await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL transaction failed".to_string()))?;
        let inserted = sqlx_core::query::query("INSERT INTO hub_artifact_cas_object(space_digest, kind, object_digest, bytes, byte_length) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
            .bind(&tenant)
            .bind(key.kind.name())
            .bind(&digest)
            .bind(bytes)
            .bind(length)
            .execute(&mut *transaction)
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL insert failed".to_string()))?
            .rows_affected()
            == 1;
        let row: (i64, Vec<u8>) = sqlx_core::query_as::query_as("SELECT byte_length, bytes FROM hub_artifact_cas_object WHERE space_digest = $1 AND kind = $2 AND object_digest = $3")
            .bind(&tenant)
            .bind(key.kind.name())
            .bind(&digest)
            .fetch_one(&mut *transaction)
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL object read failed".to_string()))?;
        if row.0 != length || row.1 != bytes {
            return Err(AuthorityError::Store("artifact CAS immutable key collision".to_string()));
        }
        transaction.commit().await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL commit failed".to_string()))?;
        context.checkpoint()?;
        Ok(if inserted { ArtifactCasPutOutcome::Inserted } else { ArtifactCasPutOutcome::AlreadyPresent })
    }

    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
        context.checkpoint()?;
        let tenant = space_digest(&key.space_id)?.0.to_vec();
        let digest = key.digest.0.to_vec();
        let length: (i64,) = sqlx_core::query_as::query_as("SELECT byte_length FROM hub_artifact_cas_object WHERE space_digest = $1 AND kind = $2 AND object_digest = $3")
            .bind(&tenant)
            .bind(key.kind.name())
            .bind(&digest)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS object not found".to_string()))?;
        if length.0 < 0 || usize::try_from(length.0).ok().is_none_or(|length| length > key.kind.maximum_bytes()) {
            return Err(AuthorityError::ResourceLimit("artifact CAS object byte"));
        }
        let row: (Vec<u8>,) = sqlx_core::query_as::query_as("SELECT bytes FROM hub_artifact_cas_object WHERE space_digest = $1 AND kind = $2 AND object_digest = $3")
            .bind(&tenant)
            .bind(key.kind.name())
            .bind(&digest)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS object not found".to_string()))?;
        validate_object(key, &row.0)?;
        context.checkpoint()?;
        Ok(row.0)
    }

    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, AuthorityError> {
        context.checkpoint()?;
        if !fence.permits(key) {
            return Err(AuthorityError::Store("artifact CAS deletion fence mismatch".to_string()));
        }
        let tenant = space_digest(&key.space_id)?.0.to_vec();
        let digest = key.digest.0.to_vec();
        let mut transaction = self.pool.begin().await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL transaction failed".into()))?;
        let current: Option<(i64,)> = sqlx_core::query_as::query_as("SELECT physical_epoch FROM hub_artifact_cas_space_fence WHERE coordinator_id = $1 AND space_digest = $2 FOR UPDATE")
            .bind(fence.coordinator_id().as_slice())
            .bind(&tenant)
            .fetch_optional(&mut *transaction)
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL physical epoch read failed".into()))?;
        if current != Some((i64::try_from(fence.physical_epoch()).map_err(|_| AuthorityError::Store("artifact CAS physical epoch overflow".into()))?,)) {
            return Err(AuthorityError::Store("artifact CAS deletion fence is stale".into()));
        }
        let deleted = sqlx_core::query::query("DELETE FROM hub_artifact_cas_object WHERE space_digest = $1 AND kind = $2 AND object_digest = $3")
            .bind(&tenant)
            .bind(key.kind.name())
            .bind(&digest)
            .execute(&mut *transaction)
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL delete failed".to_string()))?
            .rows_affected()
            == 1;
        transaction.commit().await.map_err(|_| AuthorityError::Store("artifact CAS PostgreSQL commit failed".into()))?;
        Ok(if deleted { ArtifactCasDeleteOutcome::Deleted } else { ArtifactCasDeleteOutcome::Missing })
    }
}
