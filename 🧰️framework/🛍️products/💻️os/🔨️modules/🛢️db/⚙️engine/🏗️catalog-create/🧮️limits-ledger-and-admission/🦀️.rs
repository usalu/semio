//! 🧮️ Limits, ledger and admission: the create-catalog limits, the backing ledger and the admission slots. See [`super`].

use super::*;

//#region 🔖️CreateDocumentCatalogCas
pub(super) const DATABASE_CREATE_CATALOG_SLOTS: usize = 32;

pub(super) const DATABASE_CREATE_CATALOG_MAX_ENTRIES: usize = 4_096;

pub(super) const DATABASE_CREATE_CATALOG_MAX_ID_BYTES: usize = db_io_core::DbIoText::maximum_capacity();

pub(super) const DATABASE_CREATE_CATALOG_MAX_PAGES: usize = db_io_core::DB_IO_OPERATION_PAGES;

pub(super) const DATABASE_CREATE_CATALOG_COPY_BYTES: usize = 256;

pub(super) const DATABASE_CREATE_CATALOG_ITEMS: u64 = (DATABASE_CREATE_CATALOG_MAX_ENTRIES * 4 + DATABASE_CREATE_CATALOG_MAX_PAGES + 64) as u64;

const DATABASE_CREATE_CATALOG_BYTES: u64 = (DATABASE_CREATE_CATALOG_MAX_ENTRIES * 2 * (DATABASE_CREATE_CATALOG_MAX_ID_BYTES + size_of::<CatalogEntry>()) + DATABASE_CREATE_CATALOG_MAX_PAGES * db_io_core::DB_IO_PAGE_BYTES + 128 * 1024) as u64;

const DATABASE_CREATE_CATALOG_TOTAL_ITEMS: u64 = DATABASE_CREATE_CATALOG_ITEMS * DATABASE_CREATE_CATALOG_SLOTS as u64;

const DATABASE_CREATE_CATALOG_TOTAL_BYTES: u64 = DATABASE_CREATE_CATALOG_BYTES * DATABASE_CREATE_CATALOG_SLOTS as u64;

pub(super) const DATABASE_CREATE_CATALOG_RETRY_LIMIT: u8 = 8;

pub(super) const DATABASE_CREATE_CATALOG_DEADLINE_MS: u64 = 30_000;

pub(super) const DATABASE_CREATE_CATALOG_ARC_CONTROL_BYTES: usize = size_of::<Vec<CatalogEntry>>() + size_of::<usize>() * 2;

#[derive(Clone, Copy)]
pub(super) struct DatabaseCreateCatalogBackingLedger {
    pub items: u64,
    pub bytes: u64,
}

impl DatabaseCreateCatalogBackingLedger {
    pub fn new(document_capacity: usize, base_capacity: usize) -> Result<Self, DbError> {
        let base_bytes = base_capacity.checked_mul(size_of::<CatalogEntry>()).ok_or(DbError::LimitExceeded("database create-catalog base backing bytes"))?;
        let bytes = document_capacity.checked_add(base_bytes).and_then(|bytes| bytes.checked_add(DATABASE_CREATE_CATALOG_ARC_CONTROL_BYTES)).ok_or(DbError::LimitExceeded("database create-catalog initial backing bytes"))?;
        let items = u64::try_from(base_capacity).ok().and_then(|items| items.checked_add(3)).ok_or(DbError::LimitExceeded("database create-catalog initial backing items"))?;
        let bytes = u64::try_from(bytes).map_err(|_| DbError::LimitExceeded("database create-catalog initial backing bytes"))?;
        let ledger = Self { items, bytes };
        ledger.validate()?;
        Ok(ledger)
    }

    pub fn observe(&mut self, items: u64, bytes: usize) -> Result<(), DbError> {
        let next_items = self.items.checked_add(items).ok_or(DbError::LimitExceeded("database create-catalog observed backing items"))?;
        let bytes = u64::try_from(bytes).map_err(|_| DbError::LimitExceeded("database create-catalog observed backing bytes"))?;
        let next_bytes = self.bytes.checked_add(bytes).ok_or(DbError::LimitExceeded("database create-catalog observed backing bytes"))?;
        let next = Self { items: next_items, bytes: next_bytes };
        next.validate()?;
        *self = next;
        Ok(())
    }

    fn validate(&self) -> Result<(), DbError> {
        if self.items > DATABASE_CREATE_CATALOG_ITEMS || self.bytes > DATABASE_CREATE_CATALOG_BYTES {
            return Err(DbError::LimitExceeded("database create-catalog observed backing capacity"));
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct DatabaseCreateCatalogAdmissionSlot {
    pub generation: u64,
    pub occupied: bool,
}

const EMPTY_DATABASE_CREATE_CATALOG_SLOT: DatabaseCreateCatalogAdmissionSlot = DatabaseCreateCatalogAdmissionSlot { generation: 0, occupied: false };

struct DatabaseCreateCatalogAdmissionState {
    pub slots: [DatabaseCreateCatalogAdmissionSlot; DATABASE_CREATE_CATALOG_SLOTS],
    pub items: u64,
    pub bytes: u64,
    pub next_generation: u64,
}

impl DatabaseCreateCatalogAdmissionState {
    pub fn try_claim(&mut self, document: &protocol::ArtifactId) -> Result<(usize, u64), DbError> {
        if document.0.capacity() > DATABASE_CREATE_CATALOG_MAX_ID_BYTES || document.0.len() > DATABASE_CREATE_CATALOG_MAX_ID_BYTES {
            return Err(DbError::LimitExceeded("database create-catalog document bytes"));
        }
        let Some(slot) = self.slots.iter().position(|entry| !entry.occupied) else {
            return Err(DbError::LimitExceeded("database create-catalog slot capacity"));
        };
        let Some(items) = self.items.checked_add(DATABASE_CREATE_CATALOG_ITEMS) else {
            return Err(DbError::LimitExceeded("database create-catalog aggregate items"));
        };
        let Some(bytes) = self.bytes.checked_add(DATABASE_CREATE_CATALOG_BYTES) else {
            return Err(DbError::LimitExceeded("database create-catalog aggregate bytes"));
        };
        if items > DATABASE_CREATE_CATALOG_TOTAL_ITEMS || bytes > DATABASE_CREATE_CATALOG_TOTAL_BYTES {
            return Err(DbError::LimitExceeded("database create-catalog aggregate capacity"));
        }
        let generation = self.next_generation;
        let Some(next_generation) = generation.checked_add(1) else {
            return Err(DbError::LimitExceeded("database create-catalog generation"));
        };
        self.next_generation = next_generation;
        self.slots[slot] = DatabaseCreateCatalogAdmissionSlot { generation, occupied: true };
        self.items = items;
        self.bytes = bytes;
        Ok((slot, generation))
    }

    pub fn is_current(&self, slot: usize, generation: u64) -> bool {
        self.slots.get(slot).is_some_and(|entry| entry.occupied && entry.generation == generation)
    }

    pub fn release(&mut self, slot: usize, generation: u64) -> bool {
        if !self.is_current(slot, generation) {
            return false;
        }
        let Some(items) = self.items.checked_sub(DATABASE_CREATE_CATALOG_ITEMS) else { return false };
        let Some(bytes) = self.bytes.checked_sub(DATABASE_CREATE_CATALOG_BYTES) else { return false };
        self.slots[slot] = EMPTY_DATABASE_CREATE_CATALOG_SLOT;
        self.items = items;
        self.bytes = bytes;
        true
    }
}

pub(super) static DATABASE_CREATE_CATALOG_ADMISSION: Mutex<DatabaseCreateCatalogAdmissionState> =
    Mutex::new(DatabaseCreateCatalogAdmissionState { slots: [EMPTY_DATABASE_CREATE_CATALOG_SLOT; DATABASE_CREATE_CATALOG_SLOTS], items: 0, bytes: 0, next_generation: 1 });

pub(super) struct DatabaseCreateCatalogAdmission {
    pub slot: usize,
    pub generation: u64,
}

impl DatabaseCreateCatalogAdmission {
    pub fn try_claim(document: &protocol::ArtifactId) -> Result<Self, DbError> {
        let (slot, generation) = DATABASE_CREATE_CATALOG_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).try_claim(document)?;
        Ok(Self { slot, generation })
    }

    pub fn is_current(&self) -> bool {
        DATABASE_CREATE_CATALOG_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_current(self.slot, self.generation)
    }
}

impl Drop for DatabaseCreateCatalogAdmission {
    fn drop(&mut self) {
        DATABASE_CREATE_CATALOG_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).release(self.slot, self.generation);
    }
}
//#endregion 🔖️CreateDocumentCatalogCas
