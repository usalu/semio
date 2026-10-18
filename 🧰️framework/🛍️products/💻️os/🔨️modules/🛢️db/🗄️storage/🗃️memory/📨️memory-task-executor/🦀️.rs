//! 📨️ Memory task executor: the typed task executor implementation of the in-memory executor. See [`super`].

use super::*;

//#region 🔖️Memory
impl DbIoTaskExecutor for MemoryDbIoExecutor {
    fn supports_writer_authority(&self) -> bool {
        true
    }

    fn bind_writer_control(&mut self, control: DbIoBackendControl) -> Result<(), DbError> {
        lock(&self.writers).as_mut().ok_or(DbError::Closed)?.bind(control)
    }

    fn writer_release_step(&self, _context: &mut std::task::Context<'_>) -> Result<DbIoWriterReleaseStep, DbError> {
        match lock(&self.writers).as_mut() {
            Some(table) => table.release_requested_step(),
            None => Ok(DbIoWriterReleaseStep::Idle),
        }
    }

    fn pin_writer_operation(&self, operation: u64, task: &DbIoTask) -> Result<(), DbError> {
        if let Some((key, backend, document)) = task.writer_stamp() {
            lock(&self.writers).as_mut().ok_or(DbError::Closed)?.pin_operation(key, backend, document, operation)?;
        }
        Ok(())
    }

    fn finish_writer_operation(&self, operation: u64, task: &DbIoTask) -> Result<(), DbError> {
        if let Some((key, backend, document)) = task.writer_stamp() {
            if let Some(table) = lock(&self.writers).as_mut() {
                table.finish_operation_if_pinned(key, backend, document, operation)?;
            }
        }
        Ok(())
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn bind_owner_operation(&mut self, operation: u64) -> Result<(), DbError> {
        if operation == 0 || self.backing_operation.swap(operation, std::sync::atomic::Ordering::AcqRel) != 0 {
            return Err(DbError::Internal("memory backend backing operation bound twice".to_string()));
        }
        Ok(())
    }

    fn owner_backing_bytes(&self) -> u64 {
        (size_of_val(self)
            + lock(&self.writers).as_deref().map_or(0, size_of_val)
            + size_of_val(lock(&self.wal).as_ref())
            + size_of_val(lock(&self.snapshots).as_ref())
            + size_of_val(lock(&self.payloads).as_ref())
            + size_of_val(lock(&self.index_runs).as_ref())
            + size_of_val(lock(&self.leases).as_ref())
            + size_of_val(lock(&self.operations).as_ref())
            + size_of_val(lock(&self.retired_pages).as_ref())
            + size_of_val(lock(&self.retired_wal).as_ref())) as u64
    }

    fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
        Box::pin(async move {
            let executor: Box<dyn DbIoTaskExecutor> = self;
            (executor, task, Err(DbError::Internal("memory backend has no async-native driver".to_string())))
        })
    }

    fn execute_step(&self, operation: u64, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        let complete = |result| Ok((DbIoExecutionStep::Complete, Some(result)));
        let yield_step = || Ok((DbIoExecutionStep::Yield, None));
        match task {
            DbIoTask::WalWriterAcquire { document, .. } => complete(DbIoResult::WalWriter(lock(&self.writers).as_mut().ok_or(DbError::Closed)?.acquire_with(document, || Ok(()))?)),
            DbIoTask::BackendOpen { path, .. } => {
                if path.as_str() != "memory://fixed" {
                    return Err(DbError::InvalidArgument("memory backend authority mismatch".to_string()));
                }
                complete(DbIoResult::Unit)
            }
            DbIoTask::WalCreate { document, index, .. } => {
                let mut wal = lock(&self.wal);
                if wal.iter().flatten().any(|owner| owner.document == *document && owner.index == *index) {
                    return Err(DbError::AlreadyExists("memory WAL segment already exists".to_string()));
                }
                let slot = wal.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory WAL fixed owner capacity exhausted".to_string()))?;
                db_io_backend_owner_add(self.backing_operation()?, memory_wal_segment_credit())?;
                *slot = Some(MemoryWalOwner { document: document.clone(), index: *index, segment: MemWalSegment::new() });
                complete(DbIoResult::Unit)
            }
            DbIoTask::WalAppend { document, index, input, .. } => {
                let mut wal = lock(&self.wal);
                let segment = &mut wal.iter_mut().flatten().find(|owner| owner.document == *document && owner.index == *index).ok_or_else(|| DbError::NotFound("memory WAL segment not found".to_string()))?.segment;
                if segment.sealed {
                    return Err(DbError::InvalidArgument(format!("cannot append to sealed wal segment {index}")));
                }
                let next_len = segment.len.checked_add(input.len() as u64).ok_or(DbError::LimitExceeded("memory WAL retained length"))?;
                check_len(next_len, DB_IO_MAX_READ_BYTES, "memory WAL retained length")?;
                if !input.is_empty() {
                    let slot = segment.chunks.iter_mut().find(|chunk| chunk.is_none()).ok_or(DbError::LimitExceeded("memory WAL retained chunk items"))?;
                    *slot = Some(self.retain_pages(input)?);
                }
                segment.len = next_len;
                complete(DbIoResult::Length(next_len))
            }
            DbIoTask::WalSync { .. } => complete(DbIoResult::Unit),
            DbIoTask::WalSeal { document, index, .. } => {
                let mut wal = lock(&self.wal);
                let segment = &mut wal.iter_mut().flatten().find(|owner| owner.document == *document && owner.index == *index).ok_or_else(|| DbError::NotFound("memory WAL segment not found".to_string()))?.segment;
                segment.sealed = true;
                complete(DbIoResult::Unit)
            }
            DbIoTask::WalRead { document, index, range, output, .. } => {
                let mut cursors = lock(&self.operations);
                let cursor_index = memory_cursor_index(&mut cursors[..], operation, || MemoryDbIoCursor::WalRead { operation, chunk: 0, page: 0, offset: 0, skip: range.offset as usize, remaining: range.len as usize })?;
                let Some(MemoryDbIoCursor::WalRead { chunk, page, offset, skip, remaining, .. }) = cursors[cursor_index].as_mut() else {
                    return Err(DbError::Internal("memory WAL cursor taxonomy mismatch".to_string()));
                };
                if *remaining == 0 {
                    return match output.seal_retained_step()? {
                        Some(pages) => {
                            cursors[cursor_index] = None;
                            complete(DbIoResult::Pages(pages))
                        }
                        None => yield_step(),
                    };
                }
                let wal = lock(&self.wal);
                let segment = &wal.iter().flatten().find(|owner| owner.document == *document && owner.index == *index).ok_or_else(|| DbError::NotFound("memory WAL segment not found".to_string()))?.segment;
                let owner = segment.chunks.get(*chunk as usize).and_then(Option::as_ref).ok_or_else(|| DbError::InvalidArgument("memory WAL range exceeds retained chunks".to_string()))?;
                let fragment = match owner.page(*page) {
                    Some(fragment) => fragment,
                    None => {
                        *chunk += 1;
                        *page = 0;
                        *offset = 0;
                        return yield_step();
                    }
                };
                if *skip >= fragment.len() {
                    *skip -= fragment.len();
                    *page += 1;
                    return yield_step();
                }
                if *skip != 0 {
                    *offset = *skip;
                    *skip = 0;
                }
                let requested = (fragment.len() - *offset).min(*remaining);
                let written = output.write_fragment(&fragment[*offset..*offset + requested])?;
                *offset += written;
                *remaining -= written;
                if *offset == fragment.len() {
                    *page += 1;
                    *offset = 0;
                }
                yield_step()
            }
            DbIoTask::WalLength { document, index, .. } => {
                let wal = lock(&self.wal);
                let length = wal.iter().flatten().find(|owner| owner.document == *document && owner.index == *index).map(|owner| owner.segment.len).ok_or_else(|| DbError::NotFound("memory WAL segment not found".to_string()))?;
                complete(DbIoResult::Length(length))
            }
            DbIoTask::WalState { document, index, .. } => {
                let wal = lock(&self.wal);
                let sealed = wal.iter().flatten().find(|owner| owner.document == *document && owner.index == *index).map(|owner| owner.segment.sealed).ok_or_else(|| DbError::NotFound("memory WAL segment not found".to_string()))?;
                complete(DbIoResult::WalSegmentState(if sealed { WalSegmentState::Sealed } else { WalSegmentState::Active }))
            }
            DbIoTask::WalList { document, output, .. } => {
                let mut cursors = lock(&self.operations);
                let cursor_index = memory_cursor_index(&mut cursors[..], operation, || MemoryDbIoCursor::List { operation, after: None })?;
                let Some(MemoryDbIoCursor::List { after, .. }) = cursors[cursor_index].as_mut() else {
                    return Err(DbError::Internal("memory list cursor taxonomy mismatch".to_string()));
                };
                let next = lock(&self.wal).iter().flatten().filter(|owner| owner.document == *document && after.is_none_or(|after| owner.index > after)).map(|owner| owner.index).min();
                if let Some(value) = next {
                    output.push(value)?;
                    *after = Some(value);
                    return yield_step();
                }
                cursors[cursor_index] = None;
                complete(DbIoResult::List(output.take_for_result()))
            }
            DbIoTask::WalTruncate { document, index, new_len, .. } => {
                let mut retired = lock(&self.retired_pages);
                let mut wal = lock(&self.wal);
                let segment = &mut wal.iter_mut().flatten().find(|owner| owner.document == *document && owner.index == *index).ok_or_else(|| DbError::NotFound("memory WAL segment not found".to_string()))?.segment;
                if segment.sealed || *new_len > segment.len {
                    return Err(DbError::InvalidArgument("memory WAL truncate rejected".to_string()));
                }
                let mut retained = *new_len as usize;
                for chunk in &mut segment.chunks {
                    let Some(owner) = chunk.take() else { continue };
                    if retained >= owner.len() {
                        retained -= owner.len();
                        *chunk = Some(owner);
                    } else if retained > 0 {
                        *chunk = Some(owner.try_prefix(retained).map_err(|_| DbError::Internal("memory WAL prefix cursor rejected".to_string()))?);
                        retained = 0;
                    } else {
                        let slot = retired.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory WAL truncate retirement capacity exhausted".to_string()))?;
                        *slot = Some(owner);
                    }
                }
                segment.len = *new_len;
                complete(DbIoResult::Unit)
            }
            DbIoTask::WalDelete { document, index, .. } => {
                let mut retired = lock(&self.retired_wal);
                let slot = retired.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory WAL delete retirement capacity exhausted".to_string()))?;
                let mut wal = lock(&self.wal);
                if let Some(owner) = wal.iter_mut().find(|owner| owner.as_ref().is_some_and(|owner| owner.document == *document && owner.index == *index)).and_then(Option::take) {
                    *slot = Some(owner.segment);
                }
                complete(DbIoResult::Unit)
            }
            DbIoTask::SnapshotWrite { document, generation, input, .. } => {
                let mut retired = lock(&self.retired_pages);
                let slot = retired.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory snapshot replacement retirement capacity exhausted".to_string()))?;
                let mut snapshots = lock(&self.snapshots);
                if let Some(owner) = snapshots.iter_mut().flatten().find(|owner| owner.document == *document && owner.ordinal == *generation) {
                    *slot = Some(std::mem::replace(&mut owner.pages, self.retain_pages(input)?));
                } else {
                    let target = snapshots.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory snapshot fixed owner capacity exhausted".to_string()))?;
                    *target = Some(MemoryPageOwner { document: document.clone(), ordinal: *generation, pages: self.retain_pages(input)? });
                }
                complete(DbIoResult::Unit)
            }
            DbIoTask::SnapshotRead { document, generation, output, .. } => {
                let mut cursors = lock(&self.operations);
                let cursor_index = memory_cursor_index(&mut cursors[..], operation, || MemoryDbIoCursor::PageCopy { operation, page: 0 })?;
                let Some(MemoryDbIoCursor::PageCopy { page, .. }) = cursors[cursor_index].as_mut() else {
                    return Err(DbError::Internal("memory page cursor taxonomy mismatch".to_string()));
                };
                let snapshots = lock(&self.snapshots);
                let owner = snapshots.iter().flatten().find(|owner| owner.document == *document && owner.ordinal == *generation).ok_or_else(|| DbError::NotFound("memory snapshot generation not found".to_string()))?;
                if let Some(fragment) = owner.pages.page(*page) {
                    if output.write_fragment(fragment)? != fragment.len() {
                        return Err(DbError::LimitExceeded("memory admitted page output"));
                    }
                    *page += 1;
                    return yield_step();
                }
                drop(snapshots);
                match output.seal_retained_step()? {
                    Some(pages) => {
                        cursors[cursor_index] = None;
                        complete(DbIoResult::Pages(pages))
                    }
                    None => yield_step(),
                }
            }
            DbIoTask::SnapshotLatest { document, .. } => complete(DbIoResult::OptionalLength(lock(&self.snapshots).iter().flatten().filter(|owner| owner.document == *document).map(|owner| owner.ordinal).max())),
            DbIoTask::SnapshotList { document, output, .. } => {
                let mut cursors = lock(&self.operations);
                let cursor_index = memory_cursor_index(&mut cursors[..], operation, || MemoryDbIoCursor::List { operation, after: None })?;
                let Some(MemoryDbIoCursor::List { after, .. }) = cursors[cursor_index].as_mut() else {
                    return Err(DbError::Internal("memory snapshot list cursor taxonomy mismatch".to_string()));
                };
                let next = lock(&self.snapshots).iter().flatten().filter(|owner| owner.document == *document && after.is_none_or(|after| owner.ordinal > after)).map(|owner| owner.ordinal).min();
                if let Some(value) = next {
                    output.push(value)?;
                    *after = Some(value);
                    return yield_step();
                }
                cursors[cursor_index] = None;
                complete(DbIoResult::List(output.take_for_result()))
            }
            DbIoTask::SnapshotDelete { document, generation, .. } => {
                let mut retired = lock(&self.retired_pages);
                let slot = retired.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory snapshot delete retirement capacity exhausted".to_string()))?;
                let mut snapshots = lock(&self.snapshots);
                if let Some(owner) = snapshots.iter_mut().find(|owner| owner.as_ref().is_some_and(|owner| owner.document == *document && owner.ordinal == *generation)).and_then(Option::take) {
                    *slot = Some(owner.pages);
                }
                complete(DbIoResult::Unit)
            }
            DbIoTask::PayloadPut { input, .. } => {
                let mut cursors = lock(&self.operations);
                let cursor_index = memory_cursor_index(&mut cursors[..], operation, || MemoryDbIoCursor::PayloadHash { operation, page: 0, hasher: semio_framework_hash::Hasher::new() })?;
                let Some(MemoryDbIoCursor::PayloadHash { page, hasher, .. }) = cursors[cursor_index].as_mut() else {
                    return Err(DbError::Internal("memory hash cursor taxonomy mismatch".to_string()));
                };
                if let Some(fragment) = input.page(*page) {
                    hasher.update(fragment);
                    *page += 1;
                    return yield_step();
                }
                let Some(MemoryDbIoCursor::PayloadHash { hasher, .. }) = cursors[cursor_index].take() else {
                    return Err(DbError::Internal("memory hash cursor was not retained".to_string()));
                };
                let hash = ContentHash(*hasher.finalize().as_bytes());
                let mut payloads = lock(&self.payloads);
                if !payloads.iter().flatten().any(|owner| owner.hash == hash) {
                    let slot = payloads.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory payload fixed owner capacity exhausted".to_string()))?;
                    *slot = Some(MemoryPayloadOwner { hash, pages: self.retain_pages(input)? });
                }
                complete(DbIoResult::Hash(hash))
            }
            DbIoTask::PayloadGet { hash, output, .. } => {
                let mut cursors = lock(&self.operations);
                let cursor_index = memory_cursor_index(&mut cursors[..], operation, || MemoryDbIoCursor::PageCopy { operation, page: 0 })?;
                let Some(MemoryDbIoCursor::PageCopy { page, .. }) = cursors[cursor_index].as_mut() else {
                    return Err(DbError::Internal("memory payload cursor taxonomy mismatch".to_string()));
                };
                let payloads = lock(&self.payloads);
                let owner = payloads.iter().flatten().find(|owner| owner.hash == *hash).ok_or_else(|| DbError::NotFound("memory payload not found".to_string()))?;
                if let Some(fragment) = owner.pages.page(*page) {
                    if output.write_fragment(fragment)? != fragment.len() {
                        return Err(DbError::LimitExceeded("memory payload output"));
                    }
                    *page += 1;
                    return yield_step();
                }
                drop(payloads);
                match output.seal_retained_step()? {
                    Some(pages) => {
                        cursors[cursor_index] = None;
                        complete(DbIoResult::Pages(pages))
                    }
                    None => yield_step(),
                }
            }
            DbIoTask::PayloadExists { hash, .. } => complete(DbIoResult::Exists(lock(&self.payloads).iter().flatten().any(|owner| owner.hash == *hash))),
            DbIoTask::PayloadLength { hash, .. } => {
                complete(DbIoResult::Length(lock(&self.payloads).iter().flatten().find(|owner| owner.hash == *hash).map(|owner| owner.pages.len() as u64).ok_or_else(|| DbError::NotFound("memory payload not found".to_string()))?))
            }
            DbIoTask::PayloadDelete { hash, .. } => {
                let mut retired = lock(&self.retired_pages);
                let slot = retired.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory payload delete retirement capacity exhausted".to_string()))?;
                let mut payloads = lock(&self.payloads);
                if let Some(owner) = payloads.iter_mut().find(|owner| owner.as_ref().is_some_and(|owner| owner.hash == *hash)).and_then(Option::take) {
                    *slot = Some(owner.pages);
                }
                complete(DbIoResult::Unit)
            }
            DbIoTask::CatalogRead { output, .. } => {
                let mut cursors = lock(&self.operations);
                let catalog = lock(&self.catalog);
                let Some((owner, fence)) = catalog.as_ref() else {
                    return complete(DbIoResult::OptionalCatalog(None));
                };
                let cursor_index = memory_cursor_index(&mut cursors[..], operation, || MemoryDbIoCursor::PageCopy { operation, page: 0 })?;
                let Some(MemoryDbIoCursor::PageCopy { page, .. }) = cursors[cursor_index].as_mut() else {
                    return Err(DbError::Internal("memory catalog cursor taxonomy mismatch".to_string()));
                };
                if let Some(fragment) = owner.page(*page) {
                    if output.write_fragment(fragment)? != fragment.len() {
                        return Err(DbError::LimitExceeded("memory catalog output"));
                    }
                    *page += 1;
                    return yield_step();
                }
                let fence = *fence;
                drop(catalog);
                match output.seal_retained_step()? {
                    Some(pages) => {
                        cursors[cursor_index] = None;
                        complete(DbIoResult::OptionalCatalog(Some((pages, fence))))
                    }
                    None => yield_step(),
                }
            }
            DbIoTask::CatalogCas { expected, input, .. } => {
                let mut retired = lock(&self.retired_pages);
                let slot = retired.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory catalog replacement retirement capacity exhausted".to_string()))?;
                let mut catalog = lock(&self.catalog);
                let current = catalog.as_ref().map_or(EpochFence::INITIAL, |(_, fence)| *fence);
                expected.check(current)?;
                let next = expected.next();
                *slot = catalog.replace((self.retain_pages(input)?, next)).map(|(pages, _)| pages);
                complete(DbIoResult::Fence(next))
            }
            DbIoTask::IndexWrite { document, run_id, input, .. } => {
                let mut retired = lock(&self.retired_pages);
                let slot = retired.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory index replacement retirement capacity exhausted".to_string()))?;
                let mut runs = lock(&self.index_runs);
                if let Some(owner) = runs.iter_mut().flatten().find(|owner| owner.document == *document && owner.ordinal == *run_id) {
                    *slot = Some(std::mem::replace(&mut owner.pages, self.retain_pages(input)?));
                } else {
                    let target = runs.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory index fixed owner capacity exhausted".to_string()))?;
                    *target = Some(MemoryPageOwner { document: document.clone(), ordinal: *run_id, pages: self.retain_pages(input)? });
                }
                complete(DbIoResult::Unit)
            }
            DbIoTask::IndexRead { document, run_id, output, .. } => {
                let mut cursors = lock(&self.operations);
                let cursor_index = memory_cursor_index(&mut cursors[..], operation, || MemoryDbIoCursor::PageCopy { operation, page: 0 })?;
                let Some(MemoryDbIoCursor::PageCopy { page, .. }) = cursors[cursor_index].as_mut() else {
                    return Err(DbError::Internal("memory index cursor taxonomy mismatch".to_string()));
                };
                let runs = lock(&self.index_runs);
                let owner = runs.iter().flatten().find(|owner| owner.document == *document && owner.ordinal == *run_id).ok_or_else(|| DbError::NotFound("memory index run not found".to_string()))?;
                if let Some(fragment) = owner.pages.page(*page) {
                    if output.write_fragment(fragment)? != fragment.len() {
                        return Err(DbError::LimitExceeded("memory index output"));
                    }
                    *page += 1;
                    return yield_step();
                }
                drop(runs);
                match output.seal_retained_step()? {
                    Some(pages) => {
                        cursors[cursor_index] = None;
                        complete(DbIoResult::Pages(pages))
                    }
                    None => yield_step(),
                }
            }
            DbIoTask::IndexDelete { document, run_id, .. } => {
                let mut retired = lock(&self.retired_pages);
                let slot = retired.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory index delete retirement capacity exhausted".to_string()))?;
                let mut runs = lock(&self.index_runs);
                if let Some(owner) = runs.iter_mut().find(|owner| owner.as_ref().is_some_and(|owner| owner.document == *document && owner.ordinal == *run_id)).and_then(Option::take) {
                    *slot = Some(owner.pages);
                }
                complete(DbIoResult::Unit)
            }
            DbIoTask::IndexList { document, output, .. } => {
                let mut cursors = lock(&self.operations);
                let cursor_index = memory_cursor_index(&mut cursors[..], operation, || MemoryDbIoCursor::List { operation, after: None })?;
                let Some(MemoryDbIoCursor::List { after, .. }) = cursors[cursor_index].as_mut() else {
                    return Err(DbError::Internal("memory index list cursor taxonomy mismatch".to_string()));
                };
                let next = lock(&self.index_runs).iter().flatten().filter(|owner| owner.document == *document && after.is_none_or(|after| owner.ordinal > after)).map(|owner| owner.ordinal).min();
                if let Some(value) = next {
                    output.push(value)?;
                    *after = Some(value);
                    return yield_step();
                }
                cursors[cursor_index] = None;
                complete(DbIoResult::List(output.take_for_result()))
            }
            DbIoTask::LeaseAcquire { document, holder, ttl_ms, now_ms, .. } => {
                let mut leases = lock(&self.leases);
                let existing = leases.iter_mut().flatten().find(|owner| owner.resource == *document);
                let fence = match existing.as_ref() {
                    Some(info) if *now_ms < info.expires_at_ms && info.holder != *holder => return Err(DbError::Conflict("memory lease is owned by another holder".to_string())),
                    Some(info) if *now_ms < info.expires_at_ms => info.fence,
                    Some(info) => info.fence.next(),
                    None => EpochFence::INITIAL,
                };
                if let Some(info) = existing {
                    info.holder = holder.clone();
                    info.fence = fence;
                    info.expires_at_ms = now_ms.checked_add(*ttl_ms).ok_or(DbError::LimitExceeded("memory lease expiry"))?;
                } else {
                    let slot = leases.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Unavailable("memory lease fixed owner capacity exhausted".to_string()))?;
                    *slot = Some(MemoryLeaseOwner { resource: document.clone(), holder: holder.clone(), fence, expires_at_ms: now_ms.checked_add(*ttl_ms).ok_or(DbError::LimitExceeded("memory lease expiry"))? });
                }
                complete(DbIoResult::Fence(fence))
            }
            DbIoTask::LeaseRenew { document, holder, fence, ttl_ms, now_ms, .. } => {
                let mut leases = lock(&self.leases);
                let info = leases.iter_mut().flatten().find(|owner| owner.resource == *document).ok_or_else(|| DbError::NotFound("memory lease not found".to_string()))?;
                if *now_ms >= info.expires_at_ms || info.holder != *holder {
                    return Err(DbError::Unauthorized("memory lease owner or expiry mismatch".to_string()));
                }
                fence.check(info.fence)?;
                info.expires_at_ms = now_ms.checked_add(*ttl_ms).ok_or(DbError::LimitExceeded("memory lease expiry"))?;
                complete(DbIoResult::Unit)
            }
            DbIoTask::LeaseRelease { document, holder, fence, .. } => {
                let mut cursors = lock(&self.operations);
                let mut leases = lock(&self.leases);
                let cursor_index = if let Some(index) = cursors.iter().position(|cursor| cursor.as_ref().is_some_and(|cursor| cursor.operation() == operation)) {
                    index
                } else {
                    let index = leases.iter().position(|owner| owner.as_ref().is_some_and(|owner| owner.resource == *document)).ok_or_else(|| DbError::NotFound("memory lease not found".to_string()))?;
                    let info = leases[index].as_ref().ok_or_else(|| DbError::Internal("memory lease index lost owner".to_string()))?;
                    if info.holder != *holder {
                        return Err(DbError::Unauthorized("memory lease owner mismatch".to_string()));
                    }
                    fence.check(info.fence)?;
                    memory_cursor_index(&mut cursors[..], operation, || MemoryDbIoCursor::LeaseRelease { operation, index: index as u16 })?
                };
                let Some(MemoryDbIoCursor::LeaseRelease { index, .. }) = cursors[cursor_index].as_ref() else {
                    return Err(DbError::Internal("memory lease release cursor taxonomy mismatch".to_string()));
                };
                let index = usize::from(*index);
                let owner = leases[index].as_mut().ok_or_else(|| DbError::Internal("memory lease release lost owner".to_string()))?;
                if owner.holder.close_step() {
                    return yield_step();
                }
                if owner.resource.close_step() {
                    return yield_step();
                }
                leases[index] = None;
                cursors[cursor_index] = None;
                complete(DbIoResult::Unit)
            }
            DbIoTask::LeaseGet { document, now_ms, .. } => {
                let leases = lock(&self.leases);
                let result = leases.iter().flatten().find(|owner| owner.resource == *document).filter(|info| *now_ms < info.expires_at_ms).map(|info| DbIoLeaseResult::new(info.resource.clone(), info.holder.clone(), info.fence, info.expires_at_ms));
                complete(DbIoResult::OptionalLease(result))
            }
            DbIoTask::BackendClose { .. } => complete(DbIoResult::Unit),
        }
    }

    fn close_operation_step(&self, operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
        let mut operations = lock(&self.operations);
        if let Some(index) = operations.iter().position(|cursor| cursor.as_ref().is_some_and(|cursor| cursor.operation() == operation)) {
            operations[index] = None;
            return Ok(false);
        }
        if self.retirement_step()? {
            return Ok(false);
        }
        Ok(true)
    }

    fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
        {
            let mut writers = lock(&self.writers);
            if let Some(table) = writers.as_mut() {
                if table.close_step()? {
                    return Ok(false);
                }
                if !table.terminal_is_empty() {
                    return Err(DbError::Internal("memory writer table lost terminal witness".to_string()));
                }
                writers.take();
                return Ok(false);
            }
        }
        if let Some(cursor) = lock(&self.operations).iter_mut().find(|cursor| cursor.is_some()) {
            *cursor = None;
            return Ok(false);
        }
        MemoryDbIoExecutor::close_backend_step(self)
    }

    fn backend_terminal_is_empty(&self) -> bool {
        lock(&self.writers).is_none() && lock(&self.operations).iter().all(Option::is_none) && MemoryDbIoExecutor::backend_terminal_is_empty(self)
    }
}
//#endregion 🔖️Memory
