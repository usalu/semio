//! 🧮️ Job limits, ledger and admission: the job limits, the backing ledger, admission and the segment and hash owners. See [`super`].

use super::*;

//#region 🧵️RetainedCompactionJob
pub(super) const DATABASE_COMPACTION_SLOTS: usize = 32;

const DATABASE_COMPACTION_MAX_SEGMENTS: usize = 64;

#[cfg(test)]
pub(super) const DATABASE_COMPACTION_MAX_HASHES: usize = 4_096;

pub(super) const DATABASE_COMPACTION_OPERATION_ITEMS: u64 = 16_768;

pub(super) const DATABASE_COMPACTION_OPERATION_BYTES: u64 = 2 * 1024 * 1024;

const DATABASE_COMPACTION_TOTAL_ITEMS: u64 = DATABASE_COMPACTION_OPERATION_ITEMS * DATABASE_COMPACTION_SLOTS as u64;

const DATABASE_COMPACTION_TOTAL_BYTES: u64 = DATABASE_COMPACTION_OPERATION_BYTES * DATABASE_COMPACTION_SLOTS as u64;

pub(super) const DATABASE_COMPACTION_RETRY_LIMIT: u8 = 8;

pub(super) const DATABASE_COMPACTION_DEADLINE_MS: u64 = 30_000;

pub(super) const DATABASE_COMPACTION_TURN_MS: u64 = 8;

pub(super) const DATABASE_COMPACTION_INDEX_FUEL: usize = 256;

#[derive(Default)]
pub(super) struct DatabaseCompactionBackingLedger {
    pub(super) items: u64,
    pub(super) bytes: u64,
}

impl DatabaseCompactionBackingLedger {
    fn observe(&mut self, items: usize, bytes: usize, label: &'static str) -> Result<(), DbError> {
        let items = u64::try_from(items).map_err(|_| DbError::LimitExceeded(label))?;
        let bytes = u64::try_from(bytes).map_err(|_| DbError::LimitExceeded(label))?;
        let next_items = self.items.checked_add(items).ok_or(DbError::LimitExceeded(label))?;
        let next_bytes = self.bytes.checked_add(bytes).ok_or(DbError::LimitExceeded(label))?;
        if next_items > DATABASE_COMPACTION_OPERATION_ITEMS || next_bytes > DATABASE_COMPACTION_OPERATION_BYTES {
            return Err(DbError::LimitExceeded(label));
        }
        self.items = next_items;
        self.bytes = next_bytes;
        Ok(())
    }

    pub(super) fn release(&mut self, items: usize, bytes: usize) -> Result<(), DbError> {
        self.items = self.items.checked_sub(u64::try_from(items).map_err(|_| DbError::LimitExceeded("database compaction backing release items"))?).ok_or_else(|| DbError::Internal("database compaction backing items returned twice".to_string()))?;
        self.bytes = self.bytes.checked_sub(u64::try_from(bytes).map_err(|_| DbError::LimitExceeded("database compaction backing release bytes"))?).ok_or_else(|| DbError::Internal("database compaction backing bytes returned twice".to_string()))?;
        Ok(())
    }
}

pub(super) fn database_compaction_observe_backing(ledger: &mut DatabaseCompactionBackingLedger, items: usize, bytes: usize, label: &'static str) -> Result<(), DbError> {
    ledger.observe(items, bytes, label)
}

pub(super) fn database_compaction_descriptor_backing(descriptor: &db_snapshot::SnapshotDescriptor) -> Result<(usize, usize), DbError> {
    let items = descriptor
        .roots
        .capacity()
        .checked_add(descriptor.new_pages.capacity())
        .and_then(|value| value.checked_add(1))
        .ok_or(DbError::LimitExceeded("database compaction snapshot backing items"))?;
    let hash_bytes = descriptor.roots.capacity().checked_add(descriptor.new_pages.capacity()).and_then(|value| value.checked_mul(size_of::<ContentHash>())).ok_or(DbError::LimitExceeded("database compaction snapshot backing bytes"))?;
    let bytes = hash_bytes.checked_add(descriptor.document.0.capacity()).ok_or(DbError::LimitExceeded("database compaction snapshot backing bytes"))?;
    Ok((items, bytes))
}

async fn retire_compaction_descriptor(mut descriptor: db_snapshot::SnapshotDescriptor) {
    let new_pages = std::mem::take(&mut descriptor.new_pages);
    semio_framework_async::yield_once().await;
    drop(new_pages);
    let roots = std::mem::take(&mut descriptor.roots);
    semio_framework_async::yield_once().await;
    drop(roots);
    let document = std::mem::take(&mut descriptor.document.0);
    semio_framework_async::yield_once().await;
    drop(document);
}

pub(super) async fn close_compaction_descriptor(descriptor: db_snapshot::SnapshotDescriptor, ledger: &mut DatabaseCompactionBackingLedger) -> Result<(), DbError> {
    let (items, bytes) = database_compaction_descriptor_backing(&descriptor)?;
    retire_compaction_descriptor(descriptor).await;
    ledger.release(items, bytes)
}

pub(super) async fn retire_compaction_snapshot_body(mut body: db_snapshot::SnapshotBody) {
    let roots = std::mem::take(&mut body.roots);
    semio_framework_async::yield_once().await;
    drop(roots);
}

pub(super) async fn close_compaction_snapshot_body(body: db_snapshot::SnapshotBody, ledger: &mut DatabaseCompactionBackingLedger, charge: (usize, usize)) -> Result<(), DbError> {
    retire_compaction_snapshot_body(body).await;
    ledger.release(charge.0, charge.1)
}

pub(super) async fn database_compaction_admit_descriptor(descriptor: db_snapshot::SnapshotDescriptor, ledger: &mut DatabaseCompactionBackingLedger) -> Result<db_snapshot::SnapshotDescriptor, DbError> {
    let observed = database_compaction_descriptor_backing(&descriptor).and_then(|(items, bytes)| database_compaction_observe_backing(ledger, items, bytes, "database compaction snapshot backing"));
    if let Err(error) = observed {
        retire_compaction_descriptor(descriptor).await;
        return Err(error);
    }
    Ok(descriptor)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatabaseCompactionProgress {
    Admitted,
    SnapshotFloor,
    LeaseAcquire,
    WalHorizon,
    PayloadTrace,
    WalDelete,
    PayloadDelete,
    IndexMerge,
    SnapshotCollect,
    SnapshotPublish,
    SnapshotRetain,
    LeaseRelease,
    Completed,
    Cancelled,
    Fault,
}

impl DatabaseCompactionProgress {
    pub(super) fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Admitted,
            1 => Self::SnapshotFloor,
            2 => Self::LeaseAcquire,
            3 => Self::WalHorizon,
            4 => Self::PayloadTrace,
            5 => Self::WalDelete,
            6 => Self::PayloadDelete,
            7 => Self::IndexMerge,
            8 => Self::SnapshotCollect,
            9 => Self::SnapshotPublish,
            10 => Self::SnapshotRetain,
            11 => Self::LeaseRelease,
            12 => Self::Completed,
            13 => Self::Cancelled,
            _ => Self::Fault,
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct DatabaseCompactionAdmissionSlot {
    pub(super) generation: u64,
    pub(super) occupied: bool,
}

const EMPTY_DATABASE_COMPACTION_SLOT: DatabaseCompactionAdmissionSlot = DatabaseCompactionAdmissionSlot { generation: 0, occupied: false };

pub(super) struct DatabaseCompactionAdmissionState {
    pub(super) slots: [DatabaseCompactionAdmissionSlot; DATABASE_COMPACTION_SLOTS],
    pub(super) items: u64,
    pub(super) bytes: u64,
    pub(super) next_generation: u64,
}

impl DatabaseCompactionAdmissionState {
    pub(super) fn try_claim(&mut self, document: &ArtifactId) -> Result<(usize, u64), DbError> {
        if document.0.len() > db_io_core::DbIoText::maximum_capacity() || document.0.capacity() > db_io_core::DbIoText::maximum_capacity() {
            return Err(DbError::LimitExceeded("database compaction document backing"));
        }
        let slot = self.slots.iter().position(|entry| !entry.occupied).ok_or(DbError::LimitExceeded("database compaction admission slots"))?;
        let items = self.items.checked_add(DATABASE_COMPACTION_OPERATION_ITEMS).ok_or(DbError::LimitExceeded("database compaction aggregate items"))?;
        let bytes = self.bytes.checked_add(DATABASE_COMPACTION_OPERATION_BYTES).ok_or(DbError::LimitExceeded("database compaction aggregate bytes"))?;
        if items > DATABASE_COMPACTION_TOTAL_ITEMS || bytes > DATABASE_COMPACTION_TOTAL_BYTES {
            return Err(DbError::LimitExceeded("database compaction aggregate capacity"));
        }
        let generation = self.next_generation;
        self.next_generation = generation.checked_add(1).ok_or(DbError::LimitExceeded("database compaction generation"))?;
        self.slots[slot] = DatabaseCompactionAdmissionSlot { generation, occupied: true };
        self.items = items;
        self.bytes = bytes;
        Ok((slot, generation))
    }

    pub(super) fn is_current(&self, slot: usize, generation: u64) -> bool {
        self.slots.get(slot).is_some_and(|entry| entry.occupied && entry.generation == generation)
    }

    pub(super) fn release(&mut self, slot: usize, generation: u64) -> bool {
        if !self.is_current(slot, generation) {
            return false;
        }
        let Some(items) = self.items.checked_sub(DATABASE_COMPACTION_OPERATION_ITEMS) else { return false };
        let Some(bytes) = self.bytes.checked_sub(DATABASE_COMPACTION_OPERATION_BYTES) else { return false };
        self.slots[slot] = EMPTY_DATABASE_COMPACTION_SLOT;
        self.items = items;
        self.bytes = bytes;
        true
    }
}

pub(super) static DATABASE_COMPACTION_ADMISSION: std::sync::Mutex<DatabaseCompactionAdmissionState> =
    std::sync::Mutex::new(DatabaseCompactionAdmissionState { slots: [EMPTY_DATABASE_COMPACTION_SLOT; DATABASE_COMPACTION_SLOTS], items: 0, bytes: 0, next_generation: 1 });

pub(super) struct DatabaseCompactionAdmission {
    pub(super) slot: usize,
    pub(super) generation: u64,
}

impl DatabaseCompactionAdmission {
    pub(super) fn try_claim(document: &ArtifactId) -> Result<Self, DbError> {
        let (slot, generation) = DATABASE_COMPACTION_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).try_claim(document)?;
        Ok(Self { slot, generation })
    }

    pub(super) fn is_current(&self) -> bool {
        DATABASE_COMPACTION_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_current(self.slot, self.generation)
    }
}

impl Drop for DatabaseCompactionAdmission {
    fn drop(&mut self) {
        DATABASE_COMPACTION_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).release(self.slot, self.generation);
    }
}

pub(super) struct DatabaseCompactionSegmentOwners {
    pub(super) slots: [Option<SegmentHorizon>; DATABASE_COMPACTION_MAX_SEGMENTS],
    pub(super) len: u8,
}

impl DatabaseCompactionSegmentOwners {
    pub(super) fn new() -> Self {
        Self { slots: [None; DATABASE_COMPACTION_MAX_SEGMENTS], len: 0 }
    }

    pub(super) fn push(&mut self, value: SegmentHorizon) -> Result<(), DbError> {
        let index = usize::from(self.len);
        *self.slots.get_mut(index).ok_or(DbError::LimitExceeded("database compaction WAL segment owners"))? = Some(value);
        self.len = self.len.checked_add(1).ok_or(DbError::LimitExceeded("database compaction WAL segment cursor"))?;
        Ok(())
    }

    pub(super) fn get(&self, index: usize) -> Option<SegmentHorizon> {
        self.slots.get(index).copied().flatten()
    }

    pub(super) fn len(&self) -> usize {
        usize::from(self.len)
    }

    pub(super) fn observe_head(&mut self, segment_index: u64, head_seq: u64) -> Result<(), DbError> {
        let horizon = self.slots.iter_mut().take(usize::from(self.len)).flatten().find(|horizon| horizon.segment_index == segment_index).ok_or_else(|| DbError::Corrupt("committed WAL transaction names an unknown segment".to_string()))?;
        horizon.max_head_seq = Some(horizon.max_head_seq.map_or(head_seq, |head| head.max(head_seq)));
        Ok(())
    }
}

#[cfg(test)]
pub(super) struct DatabaseCompactionHashOwners {
    pub(super) slots: [Option<ContentHash>; DATABASE_COMPACTION_MAX_HASHES],
    pub(super) len: u16,
}

#[cfg(test)]
impl DatabaseCompactionHashOwners {
    pub(super) async fn contains(&self, hash: ContentHash, cancelled: &std::sync::atomic::AtomicBool) -> Result<bool, DbError> {
        for slot in self.slots.iter().take(usize::from(self.len)) {
            compaction_opportunity(cancelled).await?;
            if *slot == Some(hash) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub(super) async fn insert(&mut self, hash: ContentHash, cancelled: &std::sync::atomic::AtomicBool) -> Result<(), DbError> {
        if self.contains(hash, cancelled).await? {
            return Ok(());
        }
        let index = usize::from(self.len);
        *self.slots.get_mut(index).ok_or(DbError::LimitExceeded("database compaction payload hash owners"))? = Some(hash);
        self.len = self.len.checked_add(1).ok_or(DbError::LimitExceeded("database compaction payload hash cursor"))?;
        Ok(())
    }
}
//#endregion 🧵️RetainedCompactionJob
