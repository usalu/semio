//! 🧰️ Memory executor: the direct storage operations of the in-memory executor, the page owner close step, the lock helper and the cursor index. See [`super`].

use super::*;

//#region 🔖️Memory
impl MemoryDbIoExecutor {
    pub(super) fn backing_bytes() -> u64 {
        (size_of::<Self>()
            + size_of::<db_io_driver::WalWriterTable<()>>()
            + DB_IO_MEMORY_OWNERS * (size_of::<Option<MemoryWalOwner>>() + 2 * size_of::<Option<MemoryPageOwner>>() + size_of::<Option<MemoryPayloadOwner>>() + size_of::<Option<MemoryLeaseOwner>>())
            + DB_IO_OPERATION_ITEMS * (size_of::<Option<MemoryDbIoCursor>>() + size_of::<Option<DbIoPages>>() + size_of::<Option<MemWalSegment>>())) as u64
    }

    pub(super) fn backing_operation(&self) -> Result<u64, DbError> {
        match self.backing_operation.load(std::sync::atomic::Ordering::Acquire) {
            0 => Err(DbError::Internal("memory backend lost its backing operation".to_string())),
            operation => Ok(operation),
        }
    }

    fn return_segment_credit(&self) -> Result<(), DbError> {
        db_io_operation_return(self.backing_operation()?, memory_wal_segment_credit())
    }

    pub(super) fn retain_pages(&self, input: &mut DbIoPages) -> Result<DbIoPages, DbError> {
        let backend_operation = self.backing_operation()?;
        let mut pages = input.take_for_async_driver();
        pages.transfer_to_backend(backend_operation)?;
        Ok(pages)
    }

    pub(super) fn retirement_step(&self) -> Result<bool, DbError> {
        let mut retired_pages = lock(&self.retired_pages);
        if let Some(slot) = retired_pages.iter_mut().find(|slot| slot.is_some()) {
            let Some(owner) = slot.as_mut() else {
                return Err(DbError::Internal("memory retired page slot lost its owner".to_string()));
            };
            if owner.close_step()?.is_some() {
                return Ok(true);
            }
            *slot = None;
            return Ok(true);
        }
        drop(retired_pages);
        let mut retired_wal = lock(&self.retired_wal);
        if let Some(slot) = retired_wal.iter_mut().find(|slot| slot.is_some()) {
            let Some(owner) = slot.as_mut() else {
                return Err(DbError::Internal("memory retired WAL slot lost its owner".to_string()));
            };
            for chunk in owner.chunks.iter_mut().flatten() {
                if chunk.close_step()?.is_some() {
                    return Ok(true);
                }
            }
            if let Some(chunk) = owner.chunks.iter_mut().find(|chunk| chunk.as_ref().is_some_and(DbIoPages::terminal_is_empty)) {
                *chunk = None;
                return Ok(true);
            }
            self.return_segment_credit()?;
            *slot = None;
            return Ok(true);
        }
        Ok(false)
    }

    /// 🧹️ One explicit memory-backend owner/page retirement opportunity.
    pub fn close_backend_step(&self) -> Result<bool, DbError> {
        if let Some(cursor) = lock(&self.operations).iter_mut().find(|cursor| cursor.is_some()) {
            *cursor = None;
            return Ok(false);
        }
        if self.retirement_step()? {
            return Ok(false);
        }
        match self.close_phase.load(std::sync::atomic::Ordering::Acquire) {
            0 => {
                let mut wal = lock(&self.wal);
                let Some(owner) = wal.iter_mut().find_map(Option::as_mut) else {
                    self.close_phase.store(1, std::sync::atomic::Ordering::Release);
                    return Ok(false);
                };
                if let Some(chunk) = owner.segment.chunks.iter_mut().flatten().find(|chunk| !chunk.terminal_is_empty()) {
                    let _ = chunk.close_step()?;
                    return Ok(false);
                }
                if let Some(chunk) = owner.segment.chunks.iter_mut().find(|chunk| chunk.is_some()) {
                    *chunk = None;
                    return Ok(false);
                }
                if owner.document.close_step() {
                    return Ok(false);
                }
                let slot = wal
                    .iter_mut()
                    .find(|slot| slot.as_ref().is_some_and(|candidate| candidate.document.terminal_is_empty() && candidate.segment.chunks.iter().all(Option::is_none)))
                    .ok_or_else(|| DbError::Internal("memory WAL close cursor lost exact owner".to_string()))?;
                self.return_segment_credit()?;
                *slot = None;
                Ok(false)
            }
            1 => {
                let mut snapshots = lock(&self.snapshots);
                if memory_page_owner_close_step(&mut snapshots[..])? {
                    return Ok(false);
                }
                self.close_phase.store(2, std::sync::atomic::Ordering::Release);
                Ok(false)
            }
            2 => {
                let mut payloads = lock(&self.payloads);
                let Some(owner) = payloads.iter_mut().find_map(Option::as_mut) else {
                    self.close_phase.store(3, std::sync::atomic::Ordering::Release);
                    return Ok(false);
                };
                if owner.pages.close_step()?.is_some() {
                    return Ok(false);
                }
                let slot = payloads.iter_mut().find(|slot| slot.as_ref().is_some_and(|candidate| candidate.pages.terminal_is_empty())).ok_or_else(|| DbError::Internal("memory payload close cursor lost exact owner".to_string()))?;
                *slot = None;
                Ok(false)
            }
            3 => {
                let mut catalog = lock(&self.catalog);
                if let Some((pages, _)) = catalog.as_mut() {
                    if pages.close_step()?.is_some() {
                        return Ok(false);
                    }
                    *catalog = None;
                    return Ok(false);
                }
                self.close_phase.store(4, std::sync::atomic::Ordering::Release);
                Ok(false)
            }
            4 => {
                let mut runs = lock(&self.index_runs);
                if memory_page_owner_close_step(&mut runs[..])? {
                    return Ok(false);
                }
                self.close_phase.store(5, std::sync::atomic::Ordering::Release);
                Ok(false)
            }
            5 => {
                let mut leases = lock(&self.leases);
                let Some(owner) = leases.iter_mut().find_map(Option::as_mut) else {
                    self.close_phase.store(6, std::sync::atomic::Ordering::Release);
                    return Ok(false);
                };
                if owner.holder.close_step() || owner.resource.close_step() {
                    return Ok(false);
                }
                let slot = leases
                    .iter_mut()
                    .find(|slot| slot.as_ref().is_some_and(|candidate| candidate.holder.terminal_is_empty() && candidate.resource.terminal_is_empty()))
                    .ok_or_else(|| DbError::Internal("memory lease close cursor lost exact owner".to_string()))?;
                *slot = None;
                Ok(false)
            }
            _ => Ok(true),
        }
    }

    pub fn backend_terminal_is_empty(&self) -> bool {
        self.close_phase.load(std::sync::atomic::Ordering::Acquire) == 6
            && lock(&self.operations).iter().all(Option::is_none)
            && lock(&self.retired_pages).iter().all(Option::is_none)
            && lock(&self.retired_wal).iter().all(Option::is_none)
            && lock(&self.wal).iter().all(Option::is_none)
            && lock(&self.snapshots).iter().all(Option::is_none)
            && lock(&self.payloads).iter().all(Option::is_none)
            && lock(&self.catalog).is_none()
            && lock(&self.index_runs).iter().all(Option::is_none)
            && lock(&self.leases).iter().all(Option::is_none)
    }
}

fn memory_page_owner_close_step(owners: &mut [Option<MemoryPageOwner>]) -> Result<bool, DbError> {
    let Some(owner) = owners.iter_mut().find_map(Option::as_mut) else { return Ok(false) };
    if owner.pages.close_step()?.is_some() || owner.document.close_step() {
        return Ok(true);
    }
    let slot =
        owners.iter_mut().find(|slot| slot.as_ref().is_some_and(|candidate| candidate.pages.terminal_is_empty() && candidate.document.terminal_is_empty())).ok_or_else(|| DbError::Internal("memory page close cursor lost exact owner".to_string()))?;
    *slot = None;
    Ok(true)
}

/// 🩹️ Recovers a `Mutex` guard from a poisoned lock instead of panicking — a single
/// panicking mailbox/actor elsewhere in the family must not turn every other document's storage
/// access into a cascading panic.
// 🚫️async: E1 pure accessor (no suspension) — see R9
pub fn lock<'a, T>(mutex: &'a std::sync::Mutex<T>) -> std::sync::MutexGuard<'a, T> {
    mutex.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

pub(super) fn memory_cursor_index(cursors: &mut [Option<MemoryDbIoCursor>], operation: u64, initial: impl FnOnce() -> MemoryDbIoCursor) -> Result<usize, DbError> {
    if let Some(index) = cursors.iter().position(|cursor| cursor.as_ref().is_some_and(|cursor| cursor.operation() == operation)) {
        return Ok(index);
    }
    let index = cursors.iter().position(Option::is_none).ok_or_else(|| DbError::Unavailable("memory DB I/O cursor capacity exhausted".to_string()))?;
    cursors[index] = Some(initial());
    Ok(index)
}
//#endregion 🔖️Memory
