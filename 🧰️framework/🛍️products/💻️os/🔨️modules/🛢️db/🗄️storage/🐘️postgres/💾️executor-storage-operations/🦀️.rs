//! 💾️ Postgres executor storage operations: the wal, snapshot, payload, catalog and index storage operations of the executor. See [`super`].

use super::*;

//#region 🔖️WalStorage
impl PostgresDbIoExecutor {
    pub(super) async fn create_segment(connection: &mut PgConnection, document: &str, index: u64) -> Result<(), DbError> {
        let idx = to_i64(index)?;
        sqlx::query("INSERT INTO db_wal_segment (document_id, segment_index) VALUES ($1, $2)")
            .bind(document)
            .bind(idx)
            .execute(&mut *connection)
            .await
            .map_err(|err| map_create_error(err, || format!("wal segment {index} for {document} already exists")))?;
        Ok(())
    }

    pub(super) async fn append(&self, connection: &mut PgConnection, document: &str, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
        let prepared = db_io_prepare_platform(&bytes)?.await?;
        let result = async {
            let idx = to_i64(index)?;
            let doc = document;
            let mut tx = connection.begin().await.map_err(map_session_error)?;
            let row: Option<(bool,)> = sqlx::query_as("SELECT sealed FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2 FOR UPDATE").bind(doc).bind(idx).fetch_optional(&mut *tx).await.map_err(map_session_error)?;
            let sealed = row.ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?.0;
            if sealed {
                return Err(DbError::InvalidArgument(format!("cannot append to sealed wal segment {index}")));
            }
            let (new_len,): (i64,) = sqlx::query_as("UPDATE db_wal_segment SET bytes = bytes || $1 WHERE document_id = $2 AND segment_index = $3 RETURNING octet_length(bytes)::BIGINT")
                .bind(prepared.as_slice())
                .bind(doc)
                .bind(idx)
                .fetch_one(&mut *tx)
                .await
                .map_err(map_session_error)?;
            tx.commit().await.map_err(map_session_error)?;
            Ok(new_len as u64)
        }
        .await;
        db_io_close_platform(prepared).await?;
        result
    }

    /// 💾️ Every write above already ran as a committed statement/transaction on the writer
    /// session, and Postgres fsyncs its own WAL at COMMIT under the default `synchronous_commit =
    /// on`, so `Fsync` is satisfied when `append`/`truncate_tail` return. `Quorum` (replica
    /// acknowledgement) is `db_cluster`'s concern over Postgres's own replication.
    pub(super) fn sync(_class: DurabilityClass) -> Result<(), DbError> {
        Ok(())
    }

    pub(super) async fn seal(connection: &mut PgConnection, document: &str, index: u64) -> Result<(), DbError> {
        let idx = to_i64(index)?;
        let result: Option<(bool,)> =
            sqlx::query_as("UPDATE db_wal_segment SET sealed = TRUE WHERE document_id = $1 AND segment_index = $2 RETURNING sealed").bind(document).bind(idx).fetch_optional(&mut *connection).await.map_err(map_session_error)?;
        result.map(|_| ()).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))
    }

    pub(super) async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
        check_len(range.len, DB_IO_MAX_READ_BYTES, "wal_storage::read")?;
        let idx = to_i64(index)?;
        let doc = document.0.as_str();
        let len_row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes)::BIGINT FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2").bind(doc).bind(idx).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let current_len = len_row.ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?.0 as u64;
        let (offset, len) = validate_read_range(current_len, range)?;
        let reservation = self.reserve_driver_output(DB_IO_MAX_READ_BYTES)?;
        let (bytes,): (Vec<u8>,) =
            sqlx::query_as("SELECT substring(bytes FROM $1::integer FOR $2::integer) FROM db_wal_segment WHERE document_id = $3 AND segment_index = $4").bind(offset + 1).bind(len).bind(doc).bind(idx).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, bytes.len().div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
        db_io_write_observed_bytes(reservation, bytes, &mut output).await
    }

    pub(super) async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        let idx = to_i64(index)?;
        let row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes)::BIGINT FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2").bind(document.0.as_str()).bind(idx).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        row.map(|(len,)| len as u64).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))
    }

    pub(super) async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<WalSegmentState, DbError> {
        let idx = to_i64(index)?;
        let row: Option<(bool,)> = sqlx::query_as(POSTGRES_WAL_STATE_QUERY).bind(document.0.as_str()).bind(idx).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        row.map(|(sealed,)| postgres_wal_segment_state(sealed)).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))
    }

    pub(super) async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        postgres_ascending_ids(&self.pool, "SELECT segment_index FROM db_wal_segment WHERE document_id = $1 ORDER BY segment_index ASC LIMIT $2", document).await
    }

    pub(super) async fn truncate_tail(connection: &mut PgConnection, document: &str, index: u64, new_len: u64) -> Result<(), DbError> {
        let idx = to_i64(index)?;
        let mut tx = connection.begin().await.map_err(map_session_error)?;
        let row: Option<(bool, i64)> =
            sqlx::query_as("SELECT sealed, octet_length(bytes)::BIGINT FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2 FOR UPDATE").bind(document).bind(idx).fetch_optional(&mut *tx).await.map_err(map_session_error)?;
        let (sealed, current_len) = row.ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
        validate_truncate(sealed, current_len as u64, new_len)?;
        sqlx::query("UPDATE db_wal_segment SET bytes = substring(bytes FROM 1 FOR $1::integer) WHERE document_id = $2 AND segment_index = $3").bind(to_i64(new_len)?).bind(document).bind(idx).execute(&mut *tx).await.map_err(map_session_error)?;
        tx.commit().await.map_err(map_session_error)?;
        Ok(())
    }

    pub(super) async fn delete_segment(connection: &mut PgConnection, document: &str, index: u64) -> Result<(), DbError> {
        let idx = to_i64(index)?;
        sqlx::query("DELETE FROM db_wal_segment WHERE document_id = $1 AND segment_index = $2").bind(document).bind(idx).execute(&mut *connection).await.map_err(map_session_error)?;
        Ok(())
    }

    pub(super) fn writer_table(&mut self) -> Result<&mut WalWriterTable<PostgresWalWriterGuard>, DbError> {
        self.writers.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner).as_deref_mut().ok_or(DbError::Closed)
    }

    /// 🔐️ Runs one pinned WAL mutation on its permit's lock session. A session the server
    /// ended is never lent again: the permit is fenced from then on and its release is immediate.
    pub(super) async fn fenced_wal_mutation(&mut self, operation: u64, task: &mut DbIoTask) -> Result<DbIoResult, DbError> {
        let (key, backend, document) = task.writer_stamp().map(|(key, backend, document)| (key, backend, document.clone())).ok_or_else(|| DbError::Internal("PostgreSQL WAL mutation lost its writer stamp".to_string()))?;
        let mut session = self.writer_table()?.pinned_guard_mut(key, backend, &document, operation)?.lend(key.generation())?;
        let connection = &mut session.connection;
        let result = match task {
            DbIoTask::WalCreate { index, .. } => Self::create_segment(connection, document.as_str(), *index).await.map(|()| DbIoResult::Unit),
            DbIoTask::WalAppend { index, input, .. } => {
                let input = input.take_for_async_driver();
                self.append(connection, document.as_str(), *index, input).await.map(DbIoResult::Length)
            }
            DbIoTask::WalSync { class, .. } => Self::sync(*class).map(|()| DbIoResult::Unit),
            DbIoTask::WalSeal { index, .. } => Self::seal(connection, document.as_str(), *index).await.map(|()| DbIoResult::Unit),
            DbIoTask::WalTruncate { index, new_len, .. } => Self::truncate_tail(connection, document.as_str(), *index, *new_len).await.map(|()| DbIoResult::Unit),
            DbIoTask::WalDelete { index, .. } => Self::delete_segment(connection, document.as_str(), *index).await.map(|()| DbIoResult::Unit),
            _ => Err(DbError::Internal("PostgreSQL fenced WAL mutation taxonomy".to_string())),
        };
        if let Err(DbError::Fenced { .. }) = result {
            drop(session);
            return Err(DbError::Fenced { expected: 0, actual: key.generation() });
        }
        self.writer_table()?.pinned_guard_mut(key, backend, &document, operation)?.restore(session);
        result
    }
}
//#endregion 🔖️WalStorage

//#region 🔖️SnapshotStorage
impl SnapshotStorage for PostgresDbIoExecutor {
    fn publication_scope(&self) -> usize {
        std::ptr::from_ref(self).addr()
    }

    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
        let gen = to_i64(generation)?;
        let prepared = db_io_prepare_platform(&bytes)?.await?;
        let result = sqlx::query(
            "INSERT INTO db_snapshot_generation (document_id, generation, bytes) VALUES ($1, $2, $3)
                 ON CONFLICT (document_id, generation) DO UPDATE SET bytes = EXCLUDED.bytes",
        )
        .bind(document.0.as_str())
        .bind(gen)
        .bind(prepared.as_slice())
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error);
        db_io_close_platform(prepared).await?;
        result.map(|_| ())
    }

    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
        let gen = to_i64(generation)?;
        let doc = document.0.as_str();
        let len_row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes)::BIGINT FROM db_snapshot_generation WHERE document_id = $1 AND generation = $2").bind(doc).bind(gen).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let len = len_row.ok_or_else(|| DbError::NotFound(format!("snapshot generation {generation} for {document} not found")))?.0;
        check_len(len as u64, DB_IO_MAX_READ_BYTES, "snapshot_storage::read_generation")?;
        let reservation = self.reserve_driver_output(DB_IO_MAX_READ_BYTES)?;
        let (bytes,): (Vec<u8>,) = sqlx::query_as("SELECT bytes FROM db_snapshot_generation WHERE document_id = $1 AND generation = $2").bind(doc).bind(gen).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, bytes.len().div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
        db_io_write_observed_bytes(reservation, bytes, &mut output).await
    }

    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        let row: (Option<i64>,) = sqlx::query_as("SELECT MAX(generation) FROM db_snapshot_generation WHERE document_id = $1").bind(document.0.as_str()).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(row.0.map(|generation| generation as u64))
    }

    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        postgres_ascending_ids(&self.pool, "SELECT generation FROM db_snapshot_generation WHERE document_id = $1 ORDER BY generation ASC LIMIT $2", document).await
    }

    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        let gen = to_i64(generation)?;
        sqlx::query("DELETE FROM db_snapshot_generation WHERE document_id = $1 AND generation = $2").bind(document.0.as_str()).bind(gen).execute(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(())
    }
}
//#endregion 🔖️SnapshotStorage

//#region 🔖️PayloadStorage
impl PayloadStorage for PostgresDbIoExecutor {
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
        check_len(bytes.len() as u64, DB_IO_MAX_READ_BYTES, "payload_storage::put")?;
        let hash = db_io_hash_pages(&bytes).await;
        let prepared = db_io_prepare_platform(&bytes)?.await?;
        let result = sqlx::query("INSERT INTO db_payload (hash, bytes) VALUES ($1, $2) ON CONFLICT (hash) DO NOTHING").bind(&hash.0[..]).bind(prepared.as_slice()).execute(&self.pool).await.map_err(map_sqlx_error);
        db_io_close_platform(prepared).await?;
        result.map(|_| hash)
    }

    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
        let len_row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes)::BIGINT FROM db_payload WHERE hash = $1").bind(&hash.0[..]).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let len = len_row.ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))?.0;
        check_len(len as u64, DB_IO_MAX_READ_BYTES, "payload_storage::get")?;
        let reservation = self.reserve_driver_output(DB_IO_MAX_READ_BYTES)?;
        let (bytes,): (Vec<u8>,) = sqlx::query_as("SELECT bytes FROM db_payload WHERE hash = $1").bind(&hash.0[..]).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, bytes.len().div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
        db_io_write_observed_bytes(reservation, bytes, &mut output).await
    }

    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM db_payload WHERE hash = $1").bind(&hash.0[..]).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(row.0 > 0)
    }

    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        sqlx::query("DELETE FROM db_payload WHERE hash = $1").bind(&hash.0[..]).execute(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(())
    }

    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        let row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes)::BIGINT FROM db_payload WHERE hash = $1").bind(&hash.0[..]).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        row.map(|(len,)| len as u64).ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))
    }
}
//#endregion 🔖️PayloadStorage

//#region 🔖️CatalogStorage
impl CatalogStorage for PostgresDbIoExecutor {
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        let reservation = self.reserve_driver_output(DB_IO_MAX_READ_BYTES)?;
        let (epoch, bytes): (i64, Option<Vec<u8>>) = sqlx::query_as("SELECT epoch, bytes FROM db_catalog_root WHERE id = 1").fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        match bytes {
            Some(bytes) => {
                let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, bytes.len().div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
                Ok(Some((db_io_write_observed_bytes(reservation, bytes, &mut output).await?, EpochFence { epoch: epoch as u64 })))
            }
            None => Ok(None),
        }
    }

    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
        check_len(new_bytes.len() as u64, DB_IO_MAX_READ_BYTES, "catalog_storage::cas_root")?;
        let prepared = db_io_prepare_platform(&new_bytes)?.await?;
        let result = async {
            let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
            // 🎯️ The bootstrap-seeded singleton row (`id = 1`) always exists, so `SELECT ... FOR
            // UPDATE` here is a real, always-present row lock — unlike `FsStorage::cas_root`'s
            // documented in-process-only mutex, this fences concurrent writers across connections
            // and processes for free via Postgres's own lock manager.
            let (current_epoch,): (i64,) = sqlx::query_as("SELECT epoch FROM db_catalog_root WHERE id = 1 FOR UPDATE").fetch_one(&mut *tx).await.map_err(map_sqlx_error)?;
            let current_fence = EpochFence { epoch: current_epoch as u64 };
            expected.check(current_fence)?;
            let new_fence = expected.next();
            sqlx::query("UPDATE db_catalog_root SET epoch = $1, bytes = $2 WHERE id = 1").bind(to_i64(new_fence.epoch)?).bind(prepared.as_slice()).execute(&mut *tx).await.map_err(map_sqlx_error)?;
            tx.commit().await.map_err(map_sqlx_error)?;
            Ok(new_fence)
        }
        .await;
        db_io_close_platform(prepared).await?;
        result
    }
}
//#endregion 🔖️CatalogStorage

//#region 🔖️IndexStorage
impl IndexStorage for PostgresDbIoExecutor {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
        let run = to_i64(run_id)?;
        let prepared = db_io_prepare_platform(&bytes)?.await?;
        let result = sqlx::query(
            "INSERT INTO db_index_run (document_id, run_id, bytes) VALUES ($1, $2, $3)
                 ON CONFLICT (document_id, run_id) DO UPDATE SET bytes = EXCLUDED.bytes",
        )
        .bind(document.0.as_str())
        .bind(run)
        .bind(prepared.as_slice())
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error);
        db_io_close_platform(prepared).await?;
        result.map(|_| ())
    }

    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
        let run = to_i64(run_id)?;
        let doc = document.0.as_str();
        let len_row: Option<(i64,)> = sqlx::query_as("SELECT octet_length(bytes)::BIGINT FROM db_index_run WHERE document_id = $1 AND run_id = $2").bind(doc).bind(run).fetch_optional(&self.pool).await.map_err(map_sqlx_error)?;
        let len = len_row.ok_or_else(|| DbError::NotFound(format!("index run {run_id} for {document} not found")))?.0;
        check_len(len as u64, DB_IO_MAX_READ_BYTES, "index_storage::read_run")?;
        let reservation = self.reserve_driver_output(DB_IO_MAX_READ_BYTES)?;
        let (bytes,): (Vec<u8>,) = sqlx::query_as("SELECT bytes FROM db_index_run WHERE document_id = $1 AND run_id = $2").bind(doc).bind(run).fetch_one(&self.pool).await.map_err(map_sqlx_error)?;
        let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, bytes.len().div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
        db_io_write_observed_bytes(reservation, bytes, &mut output).await
    }

    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        postgres_ascending_ids(&self.pool, "SELECT run_id FROM db_index_run WHERE document_id = $1 ORDER BY run_id ASC LIMIT $2", document).await
    }

    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        let run = to_i64(run_id)?;
        sqlx::query("DELETE FROM db_index_run WHERE document_id = $1 AND run_id = $2").bind(document.0.as_str()).bind(run).execute(&self.pool).await.map_err(map_sqlx_error)?;
        Ok(())
    }
}
//#endregion 🔖️IndexStorage
