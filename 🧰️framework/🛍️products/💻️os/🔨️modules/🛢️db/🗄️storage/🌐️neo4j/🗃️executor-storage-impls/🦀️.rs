//! 🗃️ Executor storage impls: the snapshot, payload, catalog, index and lease storage of the executor. See [`super`].

use super::*;

//#region 🔖️SnapshotStorage
impl SnapshotStorage for Neo4jDbIoExecutor {
    fn publication_scope(&self) -> usize {
        std::ptr::from_ref(self).addr()
    }

    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
        check_len(bytes.len() as u64, DB_IO_MAX_READ_BYTES, "snapshot_storage::write_generation")?;
        let generation_param = u64_to_i64(generation, "snapshot generation")?;
        let prepared = db_io_prepare_platform(&bytes)?.await?;
        let result = self
            .run(
                query(CYPHER_SNAPSHOT_WRITE)
                    .param("document", document.0.clone())
                    .param("generation", generation_param)
                    .param("bytes", prepared.as_static_driver_slice().to_vec())
                    .param("len", u64_to_i64(bytes.len() as u64, "snapshot generation length")?),
            )
            .await;
        db_io_close_platform(prepared).await?;
        result
    }

    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
        let generation_param = u64_to_i64(generation, "snapshot generation")?;
        let reservation = self.reserve_driver_read(DB_IO_MAX_READ_BYTES)?;
        let row = self.fetch_one(query(CYPHER_SNAPSHOT_READ).param("document", document.0.clone()).param("generation", generation_param)).await?;
        let row = row.ok_or_else(|| DbError::NotFound(format!("snapshot generation {generation} for {document} not found")))?;
        let len = i64_to_u64(row.get("len").map_err(map_de_error)?, "snapshot generation length")?;
        check_len(len, DB_IO_MAX_READ_BYTES, "snapshot_storage::read_generation")?;
        let bytes: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, (len as usize).div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
        write_driver_bytes(reservation, bytes, 0, len as usize, &mut output).await
    }

    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        let row = self.fetch_one(query(CYPHER_SNAPSHOT_LATEST).param("document", document.0.clone())).await?;
        match row.and_then(|row| row.get::<i64>("maxGeneration").ok()) {
            Some(max) => Ok(Some(i64_to_u64(max, "snapshot generation")?)),
            None => Ok(None),
        }
    }

    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        let mut stream = self.graph()?.execute(query(CYPHER_SNAPSHOT_LIST).param("document", document.0.clone())).await.map_err(map_neo4rs_error)?;
        let mut out = DbIoU64List::new();
        while let Some(row) = stream.next().await.map_err(map_neo4rs_error)? {
            out.push(i64_to_u64(row.get("generation").map_err(map_de_error)?, "snapshot generation")?)?;
        }
        Ok(out)
    }

    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        let generation_param = u64_to_i64(generation, "snapshot generation")?;
        self.run(query(CYPHER_SNAPSHOT_DELETE).param("document", document.0.clone()).param("generation", generation_param)).await
    }
}
//#endregion 🔖️SnapshotStorage

//#region 🔖️PayloadStorage
impl PayloadStorage for Neo4jDbIoExecutor {
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
        check_len(bytes.len() as u64, DB_IO_MAX_READ_BYTES, "payload_storage::put")?;
        let hash = db_io_hash_pages(&bytes).await;
        let prepared = db_io_prepare_platform(&bytes)?.await?;
        let result = self.run(query(CYPHER_PAYLOAD_PUT).param("hash", hash.to_string()).param("bytes", prepared.as_static_driver_slice().to_vec()).param("len", u64_to_i64(bytes.len() as u64, "payload length")?)).await;
        db_io_close_platform(prepared).await?;
        result?;
        Ok(hash)
    }

    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
        let reservation = self.reserve_driver_read(DB_IO_MAX_READ_BYTES)?;
        let row = self.fetch_one(query(CYPHER_PAYLOAD_GET).param("hash", hash.to_string())).await?;
        let row = row.ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))?;
        let len = i64_to_u64(row.get("len").map_err(map_de_error)?, "payload length")?;
        check_len(len, DB_IO_MAX_READ_BYTES, "payload_storage::get")?;
        let bytes: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, (len as usize).div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
        write_driver_bytes(reservation, bytes, 0, len as usize, &mut output).await
    }

    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        let row = self.fetch_one(query(CYPHER_PAYLOAD_CONTAINS).param("hash", hash.to_string())).await?;
        let count: i64 = row.ok_or_else(|| DbError::Internal("payload_storage::contains returned no row".to_string()))?.get("c").map_err(map_de_error)?;
        Ok(count > 0)
    }

    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        self.run(query(CYPHER_PAYLOAD_DELETE).param("hash", hash.to_string())).await
    }

    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        let row = self.fetch_one(query(CYPHER_PAYLOAD_LEN).param("hash", hash.to_string())).await?;
        let row = row.ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))?;
        i64_to_u64(row.get("len").map_err(map_de_error)?, "payload length")
    }
}
//#endregion 🔖️PayloadStorage

//#region 🔖️CatalogStorage
impl CatalogStorage for Neo4jDbIoExecutor {
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        let mut reservation = self.reserve_driver_read(DB_IO_MAX_READ_BYTES)?;
        let row = self.fetch_one(query(CYPHER_CATALOG_READ)).await?;
        let Some(row) = row else {
            reservation.close_step()?;
            return Ok(None);
        };
        let epoch = i64_to_u64(row.get("epoch").map_err(map_de_error)?, "catalog epoch")?;
        let len = i64_to_u64(row.get("len").map_err(map_de_error)?, "catalog root length")?;
        check_len(len, DB_IO_MAX_READ_BYTES, "catalog_storage::read_root")?;
        let bytes: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, (len as usize).div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
        Ok(Some((write_driver_bytes(reservation, bytes, 0, len as usize, &mut output).await?, EpochFence { epoch })))
    }

    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
        check_len(new_bytes.len() as u64, DB_IO_MAX_READ_BYTES, "catalog_storage::cas_root")?;
        let new_fence = expected.next();
        let prepared = db_io_prepare_platform(&new_bytes)?.await?;
        let row = self
            .fetch_one(
                query(CYPHER_CATALOG_CAS)
                    .param("expected", u64_to_i64(expected.epoch, "catalog epoch")?)
                    .param("newEpoch", u64_to_i64(new_fence.epoch, "catalog epoch")?)
                    .param("bytes", prepared.as_static_driver_slice().to_vec())
                    .param("len", u64_to_i64(new_bytes.len() as u64, "catalog root length")?),
            )
            .await;
        db_io_close_platform(prepared).await?;
        let row = row?;
        if row.is_some() {
            return Ok(new_fence);
        }
        let current = self.read_root().await?.map_or(EpochFence::INITIAL, |(_, fence)| fence);
        Err(DbError::Fenced { expected: current.epoch, actual: expected.epoch })
    }
}
//#endregion 🔖️CatalogStorage

//#region 🔖️IndexStorage
impl IndexStorage for Neo4jDbIoExecutor {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
        check_len(bytes.len() as u64, DB_IO_MAX_READ_BYTES, "index_storage::write_run")?;
        let run_id_param = u64_to_i64(run_id, "index run id")?;
        let prepared = db_io_prepare_platform(&bytes)?.await?;
        let result =
            self.run(query(CYPHER_INDEX_WRITE).param("document", document.0.clone()).param("runId", run_id_param).param("bytes", prepared.as_static_driver_slice().to_vec()).param("len", u64_to_i64(bytes.len() as u64, "index run length")?)).await;
        db_io_close_platform(prepared).await?;
        result
    }

    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
        let run_id_param = u64_to_i64(run_id, "index run id")?;
        let reservation = self.reserve_driver_read(DB_IO_MAX_READ_BYTES)?;
        let row = self.fetch_one(query(CYPHER_INDEX_READ).param("document", document.0.clone()).param("runId", run_id_param)).await?;
        let row = row.ok_or_else(|| DbError::NotFound(format!("index run {run_id} for {document} not found")))?;
        let len = i64_to_u64(row.get("len").map_err(map_de_error)?, "index run length")?;
        check_len(len, DB_IO_MAX_READ_BYTES, "index_storage::read_run")?;
        let bytes: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, (len as usize).div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
        write_driver_bytes(reservation, bytes, 0, len as usize, &mut output).await
    }

    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        let mut stream = self.graph()?.execute(query(CYPHER_INDEX_LIST).param("document", document.0.clone())).await.map_err(map_neo4rs_error)?;
        let mut out = DbIoU64List::new();
        while let Some(row) = stream.next().await.map_err(map_neo4rs_error)? {
            out.push(i64_to_u64(row.get("runId").map_err(map_de_error)?, "index run id")?)?;
        }
        Ok(out)
    }

    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        let run_id_param = u64_to_i64(run_id, "index run id")?;
        self.run(query(CYPHER_INDEX_DELETE).param("document", document.0.clone()).param("runId", run_id_param)).await
    }
}
//#endregion 🔖️IndexStorage

//#region 🔖️LeaseStorage
impl Neo4jDbIoExecutor {
    /// 📖️ Reads `resource`'s current lease row (regardless of expiry — callers decide what
    /// an expired row means) within `txn`.
    async fn lease_row(&self, txn: &mut Txn, resource: &str) -> Result<Option<LeaseRow>, DbError> {
        let mut reservation = DbIoDriverReservation::try_reserve(self.active_operation, DbIoText::maximum_capacity())?;
        let mut stream = txn.execute(query(CYPHER_LEASE_READ).param("resource", resource)).await.map_err(map_neo4rs_error)?;
        let row = stream.next(txn.handle()).await.map_err(map_neo4rs_error)?;
        let Some(row) = row else {
            reservation.close_step()?;
            return Ok(None);
        };
        let fence = EpochFence { epoch: i64_to_u64(row.get("epoch").map_err(map_de_error)?, "lease epoch")? };
        let expires_at_ms = i64_to_u64(row.get("expiresAtMs").map_err(map_de_error)?, "lease expiry")?;
        let holder: String = row.get("holder").map_err(map_de_error)?;
        Ok(Some((fence, expires_at_ms, db_io_copy_observed_text(reservation, holder).await?)))
    }
}

impl LeaseStorage for Neo4jDbIoExecutor {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        let mut txn = self.graph()?.start_txn().await.map_err(map_neo4rs_error)?;
        let existing = self.lease_row(&mut txn, resource).await?;
        let fence = decide_acquire_fence(resource, existing, holder, now_ms)?;
        let expires_at_ms = now_ms.checked_add(ttl_ms).ok_or_else(|| DbError::InvalidArgument("lease ttl_ms overflows now_ms + ttl_ms".to_string()))?;
        txn.run(query(CYPHER_LEASE_WRITE).param("resource", resource).param("holder", holder).param("epoch", u64_to_i64(fence.epoch, "lease epoch")?).param("expiresAtMs", u64_to_i64(expires_at_ms, "lease expiry")?))
            .await
            .map_err(map_neo4rs_error)?;
        txn.commit().await.map_err(map_neo4rs_error)?;
        Ok(fence)
    }

    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        let mut txn = self.graph()?.start_txn().await.map_err(map_neo4rs_error)?;
        let existing = self.lease_row(&mut txn, resource).await?;
        validate_renew(resource, existing, holder, fence, now_ms)?;
        let expires_at_ms = now_ms.checked_add(ttl_ms).ok_or_else(|| DbError::InvalidArgument("lease ttl_ms overflows now_ms + ttl_ms".to_string()))?;
        txn.run(query(CYPHER_LEASE_WRITE).param("resource", resource).param("holder", holder).param("epoch", u64_to_i64(fence.epoch, "lease epoch")?).param("expiresAtMs", u64_to_i64(expires_at_ms, "lease expiry")?))
            .await
            .map_err(map_neo4rs_error)?;
        txn.commit().await.map_err(map_neo4rs_error)?;
        Ok(())
    }

    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        let mut txn = self.graph()?.start_txn().await.map_err(map_neo4rs_error)?;
        let existing = self.lease_row(&mut txn, resource).await?;
        validate_release(resource, existing, holder, fence)?;
        txn.run(query(CYPHER_LEASE_DELETE).param("resource", resource)).await.map_err(map_neo4rs_error)?;
        txn.commit().await.map_err(map_neo4rs_error)?;
        Ok(())
    }

    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        let mut reservation = DbIoDriverReservation::try_reserve(self.active_operation, DbIoText::maximum_capacity())?;
        let row = self.fetch_one(query(CYPHER_LEASE_READ).param("resource", resource)).await?;
        let Some(row) = row else {
            reservation.close_step()?;
            return Ok(None);
        };
        let epoch = i64_to_u64(row.get("epoch").map_err(map_de_error)?, "lease epoch")?;
        let expires_at_ms = i64_to_u64(row.get("expiresAtMs").map_err(map_de_error)?, "lease expiry")?;
        if now_ms >= expires_at_ms {
            reservation.close_step()?;
            return Ok(None);
        }
        let holder: String = row.get("holder").map_err(map_de_error)?;
        let holder = db_io_copy_observed_text(reservation, holder).await?;
        Ok(Some(DbIoLeaseResult::new(DbIoText::try_from_str(resource)?, holder, EpochFence { epoch }, expires_at_ms)))
    }
}
//#endregion 🔖️LeaseStorage
