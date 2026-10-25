//! 🎟️ Catalog bootstrap admission: the slot, item, page and byte credits, the fixed admission state and the admission handle that releases its slot on drop. See [`super`].

use super::*;

//#region 🔖️CatalogBootstrapCas
pub(super) const DATABASE_CATALOG_BOOTSTRAP_SLOTS: usize = 64;

const DATABASE_CATALOG_BOOTSTRAP_ITEMS: u64 = 8;

pub(super) const DATABASE_CATALOG_BOOTSTRAP_PAGES: u8 = 8;

const DATABASE_CATALOG_BOOTSTRAP_RESULT_BYTES: u64 = 16 * 1024;

const DATABASE_CATALOG_BOOTSTRAP_BYTES: u64 = DATABASE_CATALOG_BOOTSTRAP_PAGES as u64 * db_io_core::DB_IO_PAGE_BYTES as u64 + DATABASE_CATALOG_BOOTSTRAP_RESULT_BYTES;

pub(super) const DATABASE_CATALOG_BOOTSTRAP_TOTAL_ITEMS: u64 = DATABASE_CATALOG_BOOTSTRAP_ITEMS * DATABASE_CATALOG_BOOTSTRAP_SLOTS as u64;

pub(super) const DATABASE_CATALOG_BOOTSTRAP_TOTAL_BYTES: u64 = DATABASE_CATALOG_BOOTSTRAP_BYTES * DATABASE_CATALOG_BOOTSTRAP_SLOTS as u64;

pub(super) const DATABASE_CATALOG_BOOTSTRAP_RETRY_LIMIT: u8 = 8;

#[derive(Clone, Copy)]
pub(super) struct DatabaseCatalogBootstrapAdmissionSlot {
    pub generation: u64,
    pub items: u64,
    pub bytes: u64,
    pub occupied: bool,
}

const EMPTY_DATABASE_CATALOG_BOOTSTRAP_SLOT: DatabaseCatalogBootstrapAdmissionSlot = DatabaseCatalogBootstrapAdmissionSlot { generation: 0, items: 0, bytes: 0, occupied: false };

pub(super) struct DatabaseCatalogBootstrapAdmissionState {
    pub slots: [DatabaseCatalogBootstrapAdmissionSlot; DATABASE_CATALOG_BOOTSTRAP_SLOTS],
    pub items: u64,
    pub bytes: u64,
    pub next_generation: u64,
}

impl DatabaseCatalogBootstrapAdmissionState {
    #[cfg(test)]
    pub fn empty() -> Self {
        Self { slots: [EMPTY_DATABASE_CATALOG_BOOTSTRAP_SLOT; DATABASE_CATALOG_BOOTSTRAP_SLOTS], items: 0, bytes: 0, next_generation: 1 }
    }

    pub fn try_claim(&mut self, pages: u8) -> Result<(usize, u64, u64), DbError> {
        if pages == 0 || pages > DATABASE_CATALOG_BOOTSTRAP_PAGES {
            return Err(DbError::LimitExceeded("database catalog-bootstrap page credit"));
        }
        let bytes = u64::from(pages).checked_mul(db_io_core::DB_IO_PAGE_BYTES as u64).and_then(|bytes| bytes.checked_add(DATABASE_CATALOG_BOOTSTRAP_RESULT_BYTES)).ok_or(DbError::LimitExceeded("database catalog-bootstrap byte credit"))?;
        let Some(slot) = self.slots.iter().position(|entry| !entry.occupied) else {
            return Err(DbError::LimitExceeded("database catalog-bootstrap slot capacity"));
        };
        let items = self.items.checked_add(DATABASE_CATALOG_BOOTSTRAP_ITEMS).ok_or(DbError::LimitExceeded("database catalog-bootstrap aggregate items"))?;
        let total_bytes = self.bytes.checked_add(bytes).ok_or(DbError::LimitExceeded("database catalog-bootstrap aggregate bytes"))?;
        if items > DATABASE_CATALOG_BOOTSTRAP_TOTAL_ITEMS || total_bytes > DATABASE_CATALOG_BOOTSTRAP_TOTAL_BYTES {
            return Err(DbError::LimitExceeded("database catalog-bootstrap aggregate capacity"));
        }
        let generation = self.next_generation;
        let Some(next_generation) = generation.checked_add(1) else {
            return Err(DbError::LimitExceeded("database catalog-bootstrap generation"));
        };
        self.next_generation = next_generation;
        self.slots[slot] = DatabaseCatalogBootstrapAdmissionSlot { generation, items: DATABASE_CATALOG_BOOTSTRAP_ITEMS, bytes, occupied: true };
        self.items = items;
        self.bytes = total_bytes;
        Ok((slot, generation, bytes))
    }

    pub fn is_current(&self, slot: usize, generation: u64, bytes: u64) -> bool {
        self.slots.get(slot).is_some_and(|entry| entry.occupied && entry.generation == generation && entry.items == DATABASE_CATALOG_BOOTSTRAP_ITEMS && entry.bytes == bytes)
    }

    pub fn release(&mut self, slot: usize, generation: u64, bytes: u64) -> bool {
        if !self.is_current(slot, generation, bytes) {
            return false;
        }
        let Some(items) = self.items.checked_sub(DATABASE_CATALOG_BOOTSTRAP_ITEMS) else { return false };
        let Some(total_bytes) = self.bytes.checked_sub(bytes) else { return false };
        self.slots[slot] = EMPTY_DATABASE_CATALOG_BOOTSTRAP_SLOT;
        self.items = items;
        self.bytes = total_bytes;
        true
    }
}

pub(super) static DATABASE_CATALOG_BOOTSTRAP_ADMISSION: Mutex<DatabaseCatalogBootstrapAdmissionState> =
    Mutex::new(DatabaseCatalogBootstrapAdmissionState { slots: [EMPTY_DATABASE_CATALOG_BOOTSTRAP_SLOT; DATABASE_CATALOG_BOOTSTRAP_SLOTS], items: 0, bytes: 0, next_generation: 1 });

pub(super) struct DatabaseCatalogBootstrapAdmission {
    pub slot: usize,
    pub generation: u64,
    pub bytes: u64,
}

impl DatabaseCatalogBootstrapAdmission {
    pub fn try_claim(pages: u8) -> Result<Self, DbError> {
        let (slot, generation, bytes) = DATABASE_CATALOG_BOOTSTRAP_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).try_claim(pages)?;
        Ok(Self { slot, generation, bytes })
    }

    pub fn is_current(&self) -> bool {
        DATABASE_CATALOG_BOOTSTRAP_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_current(self.slot, self.generation, self.bytes)
    }
}

impl Drop for DatabaseCatalogBootstrapAdmission {
    fn drop(&mut self) {
        DATABASE_CATALOG_BOOTSTRAP_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).release(self.slot, self.generation, self.bytes);
    }
}
//#endregion 🔖️CatalogBootstrapCas
