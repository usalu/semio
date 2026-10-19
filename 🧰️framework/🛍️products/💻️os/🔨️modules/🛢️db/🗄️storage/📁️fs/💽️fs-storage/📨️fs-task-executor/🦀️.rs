//! 📨️ Fs task executor: the typed task executor implementation of the filesystem executor. See [`super`].

use super::*;

impl DbIoTaskExecutor for FsDbIoExecutor {
    fn supports_writer_authority(&self) -> bool {
        true
    }

    fn bind_writer_control(&mut self, control: DbIoBackendControl) -> Result<(), DbError> {
        let mut owner = self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        owner.as_deref_mut().ok_or(DbError::Closed)?.bind(control)
    }

    fn writer_release_step(&self, _context: &mut std::task::Context<'_>) -> Result<DbIoWriterReleaseStep, DbError> {
        self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_deref_mut().map_or(Ok(DbIoWriterReleaseStep::Idle), WalWriterTable::release_requested_step)
    }

    fn pin_writer_operation(&self, operation: u64, task: &DbIoTask) -> Result<(), DbError> {
        let Some((key, backend, document)) = task.writer_stamp() else { return Ok(()) };
        self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_deref_mut().ok_or(DbError::Closed)?.pin_operation(key, backend, document, operation).map(|_| ())
    }

    fn finish_writer_operation(&self, operation: u64, task: &DbIoTask) -> Result<(), DbError> {
        let Some((key, backend, document)) = task.writer_stamp() else { return Ok(()) };
        if let Some(table) = self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_deref_mut() {
            table.finish_operation_if_pinned(key, backend, document, operation)?;
        }
        Ok(())
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn owner_backing_bytes(&self) -> u64 {
        (size_of::<Self>() + size_of::<WalWriterTable<WalFileWriterGuard>>()) as u64
    }

    fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
        Box::pin(async move {
            let executor: Box<dyn DbIoTaskExecutor> = self;
            (executor, task, Err(DbError::Internal("filesystem backend has no async-native driver".to_string())))
        })
    }

    fn execute_step(&self, operation: u64, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        match task {
            DbIoTask::BackendOpen { path, .. } => {
                if path.as_str() != self.root.as_str() {
                    return Err(DbError::InvalidArgument("DB I/O filesystem root authority mismatch".to_string()));
                }
                durable_directory(self.root(), &self.lifecycle)?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::WalWriterAcquire { document, .. } => {
                let mut writers = self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let table = writers.as_deref_mut().ok_or(DbError::Closed)?;
                let permit = table.acquire_with(document, || {
                    let path = self.writer_sidecar(document)?;
                    std::fs::create_dir_all(path.parent().expect("filesystem writer sidecar has a parent")).map_err(io_err)?;
                    WalFileWriterGuard::try_acquire(&path)
                })?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::WalWriter(permit))))
            }
            DbIoTask::WalCreate { document, index, .. } => {
                let dir = self.document_dir("wal", document)?;
                durable_wal_create(&dir, *index, &self.lifecycle)?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::WalAppend { document, index, input, .. } => {
                let dir = self.document_dir("wal", document)?;
                if sealed_marker_path(&dir, *index).exists() {
                    return Err(DbError::InvalidArgument(format!("cannot append to sealed wal segment {index}")));
                }
                let path = segment_path(&dir, *index);
                if let Some(fragment) = input.page(0) {
                    let len = fragment.len();
                    let mut file = std::fs::OpenOptions::new().append(true).open(&path).map_err(|error| open_err(error, || format!("wal segment {index} not found")))?;
                    file.write_all(fragment).map_err(io_err)?;
                    input.advance(len)?;
                    if !input.is_empty() {
                        return Ok((DbIoExecutionStep::Yield, None));
                    }
                }
                let len = std::fs::metadata(path).map_err(io_err)?.len();
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Length(len))))
            }
            DbIoTask::WalSync { document, index, class, .. } => {
                if matches!(class, DurabilityClass::Fsync | DurabilityClass::Quorum(_)) {
                    let dir = self.document_dir("wal", document)?;
                    let path = segment_path(&dir, *index);
                    durable_file_sync(&std::fs::OpenOptions::new().write(true).open(path).map_err(io_err)?, &self.lifecycle)?;
                    self.lifecycle.hit(FsLifecyclePhase::WalFileSynced)?;
                    durable_directory(&dir, &self.lifecycle)?;
                    sync_directory(&dir)?;
                    self.lifecycle.hit(FsLifecyclePhase::WalParentSynced)?;
                }
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::WalSeal { document, index, .. } => {
                let dir = self.document_dir("wal", document)?;
                durable_wal_seal(&dir, *index, &self.lifecycle)?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::WalRead { document, index, range, output, .. } => {
                let path = segment_path(&self.document_dir("wal", document)?, *index);
                self.read_pages_step(operation, &path, range.offset, Some(range.len), output)
            }
            DbIoTask::WalLength { document, index, .. } => {
                let path = segment_path(&self.document_dir("wal", document)?, *index);
                let len = std::fs::metadata(path).map_err(|error| open_err(error, || format!("wal segment {index} not found")))?.len();
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Length(len))))
            }
            DbIoTask::WalState { document, index, .. } => {
                let dir = self.document_dir("wal", document)?;
                std::fs::metadata(segment_path(&dir, *index)).map_err(|error| open_err(error, || format!("wal segment {index} not found")))?;
                let state = match std::fs::metadata(sealed_marker_path(&dir, *index)) {
                    Ok(_) => WalSegmentState::Sealed,
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => WalSegmentState::Active,
                    Err(error) => return Err(io_err(error)),
                };
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::WalSegmentState(state))))
            }
            DbIoTask::WalList { document, output, .. } => {
                if self.list_step(&self.document_dir("wal", document)?, "segment-", ".bin", output)? {
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::List(std::mem::take(output)))))
                } else {
                    Ok((DbIoExecutionStep::Yield, None))
                }
            }
            DbIoTask::WalTruncate { document, index, new_len, .. } => {
                let dir = self.document_dir("wal", document)?;
                if sealed_marker_path(&dir, *index).exists() {
                    return Err(DbError::InvalidArgument("cannot truncate sealed wal segment".to_string()));
                }
                let path = segment_path(&dir, *index);
                let file = std::fs::OpenOptions::new().write(true).open(path).map_err(io_err)?;
                if *new_len > file.metadata().map_err(io_err)?.len() {
                    return Err(DbError::InvalidArgument("truncate_tail new_len exceeds current segment length".to_string()));
                }
                file.set_len(*new_len).map_err(io_err)?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::WalDelete { document, index, .. } => {
                let dir = self.document_dir("wal", document)?;
                durable_wal_delete(&dir, *index, &self.lifecycle)?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::SnapshotWrite { document, generation, input, .. } => self.replace_step(&generation_path(&self.document_dir("snapshot", document)?, *generation), operation, input, &[]),
            DbIoTask::SnapshotRead { document, generation, output, .. } => self.read_pages_step(operation, &generation_path(&self.document_dir("snapshot", document)?, *generation), 0, None, output),
            DbIoTask::SnapshotLatest { document, output, .. } => {
                if self.list_step(&self.document_dir("snapshot", document)?, "gen-", ".pack", output)? {
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::OptionalLength(output.as_slice().last().copied()))))
                } else {
                    Ok((DbIoExecutionStep::Yield, None))
                }
            }
            DbIoTask::SnapshotList { document, output, .. } => {
                if self.list_step(&self.document_dir("snapshot", document)?, "gen-", ".pack", output)? {
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::List(std::mem::take(output)))))
                } else {
                    Ok((DbIoExecutionStep::Yield, None))
                }
            }
            DbIoTask::SnapshotDelete { document, generation, .. } => {
                let path = generation_path(&self.document_dir("snapshot", document)?, *generation);
                if path.exists() {
                    std::fs::remove_file(path).map_err(io_err)?;
                }
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::PayloadPut { input, .. } => {
                let temporary = self.root().join("payload").join(format!(".{operation:016x}.dbio"));
                if !temporary.exists() {
                    if let Some(parent) = temporary.parent() {
                        std::fs::create_dir_all(parent).map_err(io_err)?;
                    }
                    std::fs::File::create(&temporary).map_err(io_err)?;
                }
                if let Some(fragment) = input.page(0) {
                    let mut file = std::fs::OpenOptions::new().create(true).append(true).open(&temporary).map_err(io_err)?;
                    file.write_all(fragment).map_err(io_err)?;
                }
                let Some(hash) = self.payload_hash_step(operation, input)? else { return Ok((DbIoExecutionStep::Yield, None)) };
                let path = payload_path(self.root(), &hash);
                if temporary.exists() {
                    if let Some(parent) = path.parent() {
                        std::fs::create_dir_all(parent).map_err(io_err)?;
                    }
                    if !path.exists() {
                        std::fs::rename(temporary, path).map_err(io_err)?;
                    } else {
                        std::fs::remove_file(temporary).map_err(io_err)?;
                    }
                }
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Hash(hash))))
            }
            DbIoTask::PayloadGet { hash, output, .. } => self.read_pages_step(operation, &payload_path(self.root(), hash), 0, None, output),
            DbIoTask::PayloadExists { hash, .. } => Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Exists(payload_path(self.root(), hash).exists())))),
            DbIoTask::PayloadLength { hash, .. } => {
                let len = std::fs::metadata(payload_path(self.root(), hash)).map_err(|error| open_err(error, || format!("payload {hash} not found")))?.len();
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Length(len))))
            }
            DbIoTask::PayloadDelete { hash, .. } => {
                let path = payload_path(self.root(), hash);
                if path.exists() {
                    std::fs::remove_file(path).map_err(io_err)?;
                }
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::CatalogRead { output, .. } => {
                let path = catalog_path(self.root());
                if !path.exists() {
                    return Ok((DbIoExecutionStep::Complete, Some(DbIoResult::OptionalCatalog(None))));
                }
                let (step, result) = self.read_step(operation, &path, 8, None, true, output)?;
                let result = match result {
                    Some((pages, Some(fence))) => Some(DbIoResult::OptionalCatalog(Some((pages, fence)))),
                    Some((pages, None)) => {
                        drop(pages);
                        return Err(DbError::Internal("catalog read cursor lost its retained fence".to_string()));
                    }
                    None => None,
                };
                Ok((step, result))
            }
            DbIoTask::CatalogCas { expected, input, .. } => {
                let _guard = self.catalog_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let current = read_root_fence(self.root())?.unwrap_or(EpochFence::INITIAL);
                expected.check(current)?;
                let next = expected.next();
                let (step, result) = self.replace_step(&catalog_path(self.root()), operation, input, &next.epoch.to_le_bytes())?;
                Ok((step, result.map(|_| DbIoResult::Fence(next))))
            }
            DbIoTask::IndexWrite { document, run_id, input, .. } => self.replace_step(&run_path(&self.document_dir("index", document)?, *run_id), operation, input, &[]),
            DbIoTask::IndexRead { document, run_id, output, .. } => self.read_pages_step(operation, &run_path(&self.document_dir("index", document)?, *run_id), 0, None, output),
            DbIoTask::IndexList { document, output, .. } => {
                if self.list_step(&self.document_dir("index", document)?, "run-", ".bin", output)? {
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::List(std::mem::take(output)))))
                } else {
                    Ok((DbIoExecutionStep::Yield, None))
                }
            }
            DbIoTask::IndexDelete { document, run_id, .. } => {
                let path = run_path(&self.document_dir("index", document)?, *run_id);
                if path.exists() {
                    std::fs::remove_file(path).map_err(io_err)?;
                }
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::LeaseAcquire { document, holder, now_ms, ttl_ms, .. } => {
                let _guard = self.lease_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let path = lease_path(self.root(), document.as_str())?;
                let fence = match read_lease_file(&path)? {
                    Some((_, expires, existing)) if *now_ms < expires && existing.as_str() != holder.as_str() => return Err(DbError::Conflict("resource is leased by another holder".to_string())),
                    Some((fence, expires, _)) if *now_ms < expires => fence,
                    Some((fence, _, _)) => fence.next(),
                    None => EpochFence::INITIAL,
                };
                let expires_at_ms = now_ms.checked_add(*ttl_ms).ok_or(DbError::LimitExceeded("filesystem lease expiry"))?;
                write_lease_file(&path, operation, fence, expires_at_ms, holder.as_str())?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Fence(fence))))
            }
            DbIoTask::LeaseRenew { document, holder, fence, now_ms, ttl_ms, .. } => {
                let _guard = self.lease_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let path = lease_path(self.root(), document.as_str())?;
                let (current, expires, existing) = read_lease_file(&path)?.ok_or_else(|| DbError::NotFound("lease not found".to_string()))?;
                if *now_ms >= expires || existing.as_str() != holder.as_str() {
                    return Err(DbError::Unauthorized("lease owner or expiry mismatch".to_string()));
                }
                fence.check(current)?;
                let expires_at_ms = now_ms.checked_add(*ttl_ms).ok_or(DbError::LimitExceeded("filesystem lease expiry"))?;
                write_lease_file(&path, operation, *fence, expires_at_ms, holder.as_str())?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::LeaseRelease { document, holder, fence, .. } => {
                let _guard = self.lease_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let path = lease_path(self.root(), document.as_str())?;
                let (current, _, existing) = read_lease_file(&path)?.ok_or_else(|| DbError::NotFound("lease not found".to_string()))?;
                if existing.as_str() != holder.as_str() {
                    return Err(DbError::Unauthorized("lease owner mismatch".to_string()));
                }
                fence.check(current)?;
                std::fs::remove_file(path).map_err(io_err)?;
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
            }
            DbIoTask::LeaseGet { document, now_ms, .. } => {
                let lease = read_lease_file(&lease_path(self.root(), document.as_str())?)?;
                let result = match lease {
                    Some((fence, expires_at_ms, holder)) if *now_ms < expires_at_ms => Some(super::super::DbIoLeaseResult::new(document.clone(), holder, fence, expires_at_ms)),
                    _ => None,
                };
                Ok((DbIoExecutionStep::Complete, Some(DbIoResult::OptionalLease(result))))
            }
            DbIoTask::BackendClose { .. } => Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit))),
        }
    }

    fn close_operation_step(&self, operation: u64, task: &DbIoTask) -> Result<bool, DbError> {
        let owner_slot = super::super::db_io_operation_owner_slot(operation).ok();
        for reader in owner_slot.map_or(&self.readers[..], |slot| std::slice::from_ref(&self.readers[slot])) {
            let mut reader = reader.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if reader.as_ref().is_some_and(|owner| owner.operation == operation) {
                reader.take();
                return Ok(false);
            }
        }
        let temporary = match task {
            DbIoTask::SnapshotWrite { document, generation, .. } => {
                let path = generation_path(&self.document_dir("snapshot", document)?, *generation);
                path.parent().zip(path.file_name().and_then(std::ffi::OsStr::to_str)).map(|(parent, name)| parent.join(format!(".{name}.{operation:016x}.dbio")))
            }
            DbIoTask::IndexWrite { document, run_id, .. } => {
                let path = run_path(&self.document_dir("index", document)?, *run_id);
                path.parent().zip(path.file_name().and_then(std::ffi::OsStr::to_str)).map(|(parent, name)| parent.join(format!(".{name}.{operation:016x}.dbio")))
            }
            DbIoTask::CatalogCas { .. } => {
                let path = catalog_path(self.root());
                path.parent().zip(path.file_name().and_then(std::ffi::OsStr::to_str)).map(|(parent, name)| parent.join(format!(".{name}.{operation:016x}.dbio")))
            }
            DbIoTask::PayloadPut { .. } => Some(self.root().join("payload").join(format!(".{operation:016x}.dbio"))),
            _ => None,
        };
        if let Some(temporary) = temporary {
            if temporary.exists() {
                std::fs::remove_file(temporary).map_err(io_err)?;
                return Ok(false);
            }
        }
        for state in owner_slot.map_or(&self.payload_hashes[..], |slot| std::slice::from_ref(&self.payload_hashes[slot])) {
            let mut state = state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if state.as_ref().is_some_and(|(owner, _)| *owner == operation) {
                state.take();
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
        {
            let mut owner = self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(table) = owner.as_deref_mut() {
                if table.close_step()? {
                    return Ok(false);
                }
                if !table.terminal_is_empty() {
                    return Err(DbError::Internal("filesystem WAL writer table returned a false terminal witness".to_string()));
                }
                owner.take();
                return Ok(false);
            }
        }
        let cursor = self.backend_close_cursor.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        if cursor < self.readers.len() {
            self.readers[cursor].lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            return Ok(false);
        }
        let hash = cursor - self.readers.len();
        if hash < self.payload_hashes.len() {
            self.payload_hashes[hash].lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            return Ok(false);
        }
        self.backend_terminal.store(true, std::sync::atomic::Ordering::Release);
        Ok(true)
    }

    fn backend_terminal_is_empty(&self) -> bool {
        self.backend_terminal.load(std::sync::atomic::Ordering::Acquire)
            && self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.readers.iter().all(|owner| owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none())
            && self.payload_hashes.iter().all(|owner| owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none())
    }
}
