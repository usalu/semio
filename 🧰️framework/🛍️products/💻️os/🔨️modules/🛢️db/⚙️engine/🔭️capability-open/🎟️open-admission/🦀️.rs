//! 🎟️ Capability open admission: the slot, item and byte credits, the fixed admission state and the admission handle that releases its slot on drop. See [`super`].

use super::*;

//#region 🔖️CapabilityOpen
pub(super) const DATABASE_CAPABILITY_OPEN_SLOTS: usize = 64;

pub(super) const DATABASE_CAPABILITY_OPEN_ITEMS: u64 = 8;

pub(super) const DATABASE_CAPABILITY_OPEN_BYTES: u64 = 16 * 1024;

pub(super) const DATABASE_CAPABILITY_OPEN_TOTAL_ITEMS: u64 = DATABASE_CAPABILITY_OPEN_ITEMS * DATABASE_CAPABILITY_OPEN_SLOTS as u64;

pub(super) const DATABASE_CAPABILITY_OPEN_TOTAL_BYTES: u64 = DATABASE_CAPABILITY_OPEN_BYTES * DATABASE_CAPABILITY_OPEN_SLOTS as u64;

pub(super) const DATABASE_CAPABILITY_OPEN_RETRY_MS: u64 = 1;

pub(super) const DATABASE_CAPABILITY_OPEN_RETRY_LIMIT: u8 = 8;

pub(super) const DATABASE_CAPABILITY_OPEN_ACTIVE: u8 = 1;

pub(super) const DATABASE_CAPABILITY_OPEN_QUEUED: u8 = 2;

pub(super) const DATABASE_CAPABILITY_OPEN_CLOSED: u8 = 4;

#[derive(Clone, Copy)]
struct DatabaseCapabilityOpenAdmissionSlot {
    pub generation: u64,
    pub items: u64,
    pub bytes: u64,
    pub occupied: bool,
}

const EMPTY_DATABASE_CAPABILITY_OPEN_SLOT: DatabaseCapabilityOpenAdmissionSlot = DatabaseCapabilityOpenAdmissionSlot { generation: 0, items: 0, bytes: 0, occupied: false };

pub(super) struct DatabaseCapabilityOpenAdmissionState {
    pub slots: [DatabaseCapabilityOpenAdmissionSlot; DATABASE_CAPABILITY_OPEN_SLOTS],
    pub items: u64,
    pub bytes: u64,
    pub next_generation: u64,
}

impl DatabaseCapabilityOpenAdmissionState {
    #[cfg(test)]
    pub fn empty() -> Self {
        Self { slots: [EMPTY_DATABASE_CAPABILITY_OPEN_SLOT; DATABASE_CAPABILITY_OPEN_SLOTS], items: 0, bytes: 0, next_generation: 1 }
    }

    pub fn try_claim(&mut self, items: u64, bytes: u64) -> Result<(usize, u64), DbError> {
        if items == 0 || items > DATABASE_CAPABILITY_OPEN_ITEMS {
            return Err(DbError::LimitExceeded("database capability-open item credit"));
        }
        if bytes == 0 || bytes > DATABASE_CAPABILITY_OPEN_BYTES {
            return Err(DbError::LimitExceeded("database capability-open byte credit"));
        }
        let Some(slot) = self.slots.iter().position(|entry| !entry.occupied) else {
            return Err(DbError::Unavailable("database capability-open item capacity exhausted".to_string()));
        };
        if self.items.checked_add(items).is_none_or(|next| next > DATABASE_CAPABILITY_OPEN_TOTAL_ITEMS) {
            return Err(DbError::Unavailable("database capability-open aggregate item capacity exhausted".to_string()));
        }
        if self.bytes.checked_add(bytes).is_none_or(|next| next > DATABASE_CAPABILITY_OPEN_TOTAL_BYTES) {
            return Err(DbError::Unavailable("database capability-open aggregate byte capacity exhausted".to_string()));
        }
        let generation = self.next_generation;
        let Some(next_generation) = generation.checked_add(1) else {
            return Err(DbError::LimitExceeded("database capability-open generation"));
        };
        self.next_generation = next_generation;
        self.slots[slot] = DatabaseCapabilityOpenAdmissionSlot { generation, items, bytes, occupied: true };
        self.items += items;
        self.bytes += bytes;
        Ok((slot, generation))
    }

    pub fn release(&mut self, slot: usize, generation: u64, items: u64, bytes: u64) -> bool {
        let Some(entry) = self.slots.get_mut(slot) else {
            return false;
        };
        if !entry.occupied || entry.generation != generation || entry.items != items || entry.bytes != bytes {
            return false;
        }
        *entry = EMPTY_DATABASE_CAPABILITY_OPEN_SLOT;
        self.items = self.items.checked_sub(items).expect("database capability-open item credit underflow");
        self.bytes = self.bytes.checked_sub(bytes).expect("database capability-open byte credit underflow");
        true
    }
}

pub(super) static DATABASE_CAPABILITY_OPEN_ADMISSION: Mutex<DatabaseCapabilityOpenAdmissionState> =
    Mutex::new(DatabaseCapabilityOpenAdmissionState { slots: [EMPTY_DATABASE_CAPABILITY_OPEN_SLOT; DATABASE_CAPABILITY_OPEN_SLOTS], items: 0, bytes: 0, next_generation: 1 });

pub(super) struct DatabaseCapabilityOpenAdmission {
    pub slot: usize,
    pub generation: u64,
    pub items: u64,
    pub bytes: u64,
}

impl DatabaseCapabilityOpenAdmission {
    pub fn try_claim(items: u64, bytes: u64) -> Result<Self, DbError> {
        let mut state = DATABASE_CAPABILITY_OPEN_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let (slot, generation) = state.try_claim(items, bytes)?;
        Ok(Self { slot, generation, items, bytes })
    }

    pub fn is_current(&self) -> bool {
        let state = DATABASE_CAPABILITY_OPEN_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.slots.get(self.slot).is_some_and(|entry| entry.occupied && entry.generation == self.generation && entry.items == self.items && entry.bytes == self.bytes)
    }
}

impl Drop for DatabaseCapabilityOpenAdmission {
    fn drop(&mut self) {
        let mut state = DATABASE_CAPABILITY_OPEN_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.release(self.slot, self.generation, self.items, self.bytes);
    }
}
//#endregion 🔖️CapabilityOpen
