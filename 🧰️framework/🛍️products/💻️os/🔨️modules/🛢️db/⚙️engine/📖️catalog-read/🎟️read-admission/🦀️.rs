//! 🎟️ Catalog read admission: the slot, item and byte credits, the fixed admission state and the admission handle that releases its slot on drop. See [`super`].

use super::*;

//#region 🔖️CatalogRootRead
pub(super) const DATABASE_CATALOG_READ_SLOTS: usize = 64;

pub(super) const DATABASE_CATALOG_READ_ITEMS: u64 = 8;

pub(super) const DATABASE_CATALOG_READ_BYTES: u64 = 64 * 1024;

const DATABASE_CATALOG_READ_TOTAL_ITEMS: u64 = DATABASE_CATALOG_READ_ITEMS * DATABASE_CATALOG_READ_SLOTS as u64;

const DATABASE_CATALOG_READ_TOTAL_BYTES: u64 = DATABASE_CATALOG_READ_BYTES * DATABASE_CATALOG_READ_SLOTS as u64;

pub(super) const DATABASE_CATALOG_READ_RETRY_LIMIT: u8 = 8;

pub(super) const DATABASE_CATALOG_READ_ACTIVE: u8 = 1;

pub(super) const DATABASE_CATALOG_READ_QUEUED: u8 = 2;

pub(super) const DATABASE_CATALOG_READ_CLOSED: u8 = 4;

#[derive(Clone, Copy)]
struct DatabaseCatalogReadAdmissionSlot {
    pub generation: u64,
    pub occupied: bool,
}

const EMPTY_DATABASE_CATALOG_READ_SLOT: DatabaseCatalogReadAdmissionSlot = DatabaseCatalogReadAdmissionSlot { generation: 0, occupied: false };

pub(super) struct DatabaseCatalogReadAdmissionState {
    pub slots: [DatabaseCatalogReadAdmissionSlot; DATABASE_CATALOG_READ_SLOTS],
    pub items: u64,
    pub bytes: u64,
    pub next_generation: u64,
}

impl DatabaseCatalogReadAdmissionState {
    #[cfg(test)]
    pub fn empty() -> Self {
        Self { slots: [EMPTY_DATABASE_CATALOG_READ_SLOT; DATABASE_CATALOG_READ_SLOTS], items: 0, bytes: 0, next_generation: 1 }
    }

    pub fn try_claim(&mut self, items: u64, bytes: u64) -> Result<(usize, u64), DbError> {
        if items == 0 || items > DATABASE_CATALOG_READ_ITEMS {
            return Err(DbError::LimitExceeded("database catalog-read item credit"));
        }
        if bytes == 0 || bytes > DATABASE_CATALOG_READ_BYTES {
            return Err(DbError::LimitExceeded("database catalog-read byte credit"));
        }
        let slot = self.slots.iter().position(|entry| !entry.occupied).ok_or_else(|| DbError::Unavailable("database catalog-read slot capacity exhausted".to_string()))?;
        let next_items = self.items.checked_add(items).ok_or(DbError::LimitExceeded("database catalog-read aggregate items"))?;
        let next_bytes = self.bytes.checked_add(bytes).ok_or(DbError::LimitExceeded("database catalog-read aggregate bytes"))?;
        if next_items > DATABASE_CATALOG_READ_TOTAL_ITEMS || next_bytes > DATABASE_CATALOG_READ_TOTAL_BYTES {
            return Err(DbError::Unavailable("database catalog-read aggregate capacity exhausted".to_string()));
        }
        let generation = self.next_generation;
        self.next_generation = generation.checked_add(1).ok_or(DbError::LimitExceeded("database catalog-read generation"))?;
        self.slots[slot] = DatabaseCatalogReadAdmissionSlot { generation, occupied: true };
        self.items = next_items;
        self.bytes = next_bytes;
        Ok((slot, generation))
    }

    pub fn is_current(&self, slot: usize, generation: u64) -> bool {
        self.slots.get(slot).is_some_and(|entry| entry.occupied && entry.generation == generation)
    }

    pub fn release(&mut self, slot: usize, generation: u64) -> bool {
        if !self.is_current(slot, generation) {
            return false;
        }
        self.slots[slot] = EMPTY_DATABASE_CATALOG_READ_SLOT;
        self.items = self.items.checked_sub(DATABASE_CATALOG_READ_ITEMS).expect("database catalog-read item credit underflow");
        self.bytes = self.bytes.checked_sub(DATABASE_CATALOG_READ_BYTES).expect("database catalog-read byte credit underflow");
        true
    }
}

pub(super) static DATABASE_CATALOG_READ_ADMISSION: Mutex<DatabaseCatalogReadAdmissionState> = Mutex::new(DatabaseCatalogReadAdmissionState { slots: [EMPTY_DATABASE_CATALOG_READ_SLOT; DATABASE_CATALOG_READ_SLOTS], items: 0, bytes: 0, next_generation: 1 });

pub(super) struct DatabaseCatalogReadAdmission {
    pub slot: usize,
    pub generation: u64,
}

impl DatabaseCatalogReadAdmission {
    pub fn try_claim() -> Result<Self, DbError> {
        let (slot, generation) = DATABASE_CATALOG_READ_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).try_claim(DATABASE_CATALOG_READ_ITEMS, DATABASE_CATALOG_READ_BYTES)?;
        Ok(Self { slot, generation })
    }

    pub fn is_current(&self) -> bool {
        DATABASE_CATALOG_READ_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_current(self.slot, self.generation)
    }
}

impl Drop for DatabaseCatalogReadAdmission {
    fn drop(&mut self) {
        DATABASE_CATALOG_READ_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).release(self.slot, self.generation);
    }
}
//#endregion 🔖️CatalogRootRead
