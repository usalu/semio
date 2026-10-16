//! 📜️ Db artifact history replay.
extern crate semio_framework_os_kernel as pack;

use db_durability::Frontier;
use db_ids::{*, ArtifactId, DbError};
use protocol::MutationDiff as _;
use std::{future::Future, pin::Pin, sync::Arc};
#[cfg(test)]
use db_io_core::WalStorage;

//#region 🔖️Ids
//#region 🔖️HistoryReplay
const HISTORY_REPLAY_PAGE_BYTES: u64 = 16 * 1024;

const HISTORY_REPLAY_SEGMENT_PAGES: u64 = 1_024;

const HISTORY_REPLAY_RESULT_BYTES: u64 = 15 * 1024 * 1024;

const HISTORY_REPLAY_OPERATION_BYTES: u64 = 32 * 1024 * 1024;

const HISTORY_REPLAY_MAX_FRAME_BYTES: u64 = 1024 * 1024;

const HISTORY_REPLAY_MAX_FIELD_BYTES: usize = HISTORY_REPLAY_PAGE_BYTES as usize;

const HISTORY_REPLAY_MAX_ENTRIES: usize = 4_096;

const HISTORY_REPLAY_MAX_OPERATION_IDS: usize = 8_192;

const HISTORY_REPLAY_RESULT_PAGES: usize = (HISTORY_REPLAY_RESULT_BYTES / HISTORY_REPLAY_PAGE_BYTES) as usize;

const HISTORY_REPLAY_CONSTRUCTION_SLOTS: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct HistoryTextRange {
    pub start: u32,
    pub len: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactHistoryEntry {
    pub operation_start: u16,
    pub operation_count: u16,
    pub head_seq: u64,
    pub commit_seq: u64,
    pub chain_hash: [u8; 32],
    pub epoch: u64,
}

pub struct ArtifactHistoryView {
    pub entries: Vec<ArtifactHistoryEntry>,
    pub operation_ids: Vec<HistoryTextRange>,
    result_pages: Vec<Option<Vec<u8>>>,
    pub result_len: u64,
    pub retained_bytes: u64,
    pub retained_items: usize,
}

impl ArtifactHistoryView {
    pub fn operation_id_eq(&self, entry: usize, operation: usize, expected: &str) -> bool {
        let Some(entry) = self.entries.get(entry) else {
            return false;
        };
        if operation >= entry.operation_count as usize {
            return false;
        }
        let Some(index) = (entry.operation_start as usize).checked_add(operation) else {
            return false;
        };
        let Some(range) = self.operation_ids.get(index).copied() else {
            return false;
        };
        let expected = expected.as_bytes();
        if expected.len() != range.len as usize {
            return false;
        }
        let offset = range.start as usize;
        let page = offset / HISTORY_REPLAY_PAGE_BYTES as usize;
        let within = offset % HISTORY_REPLAY_PAGE_BYTES as usize;
        let Some(first) = self.result_pages.get(page).and_then(Option::as_ref) else {
            return false;
        };
        let first_len = expected.len().min(first.len().saturating_sub(within));
        if first_len == 0 || expected.get(..first_len) != first.get(within..within + first_len) {
            return false;
        }
        if first_len == expected.len() {
            return true;
        }
        self.result_pages.get(page + 1).and_then(Option::as_ref).and_then(|second| second.get(..expected.len() - first_len)) == expected.get(first_len..)
    }

    pub fn close_step(&mut self) -> bool {
        if self.operation_ids.pop().is_some() {
            return true;
        }
        if self.entries.pop().is_some() {
            return true;
        }
        if let Some(owner) = self.result_pages.pop() {
            drop(owner);
            return true;
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.operation_ids.is_empty() && self.entries.is_empty() && self.result_pages.is_empty()
    }
}

/// 🔒️ Serializes the tests that saturate or consume the process-global history replay
/// construction registry and admission slots, so one law's capacity never becomes another's refusal.
#[cfg(any(test, feature = "testkit"))]
pub fn history_capacity_test_lock() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

pub struct HistoryReplayReservation {
    source_pages: Vec<Option<Vec<u8>>>,
    source_page_count: usize,
    result_pages: Vec<Option<Vec<u8>>>,
    pub operation_ids: Vec<HistoryTextRange>,
    pub entries: Vec<ArtifactHistoryEntry>,
    scratch: Option<Vec<u8>>,
    retained_operation_bytes: u64,
    retained_result_bytes: u64,
}

impl std::fmt::Debug for HistoryReplayReservation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("HistoryReplayReservation")
            .field("source_pages", &self.source_pages.len())
            .field("source_page_count", &self.source_page_count)
            .field("result_pages", &self.result_pages.len())
            .field("operation_ids", &self.operation_ids.len())
            .field("entries", &self.entries.len())
            .field("scratch", &self.scratch.as_ref().map(Vec::len))
            .field("retained_operation_bytes", &self.retained_operation_bytes)
            .field("retained_result_bytes", &self.retained_result_bytes)
            .finish()
    }
}

impl HistoryReplayReservation {
    pub fn try_new() -> Result<Self, HistoryReplayReservationConstructionFault> {
        Self::try_new_retained(None)
    }

    #[cfg(test)]
    fn try_new_with_result_page_failure(failure_after: usize) -> Result<Self, HistoryReplayReservationConstructionFault> {
        Self::try_new_retained(Some(failure_after))
    }

    fn try_new_retained(failure_after: Option<usize>) -> Result<Self, HistoryReplayReservationConstructionFault> {
        let mut builder = match HistoryReplayReservationConstructionBuilder::new() {
            Ok(builder) => builder,
            Err(fault) => return Err(fault),
        };
        if !builder.edit_cursor(|cursor| cursor.source_pages.as_mut().is_some_and(|owners| owners.try_reserve_exact(HISTORY_REPLAY_SEGMENT_PAGES as usize).is_ok())).unwrap_or(false) {
            return Err(builder.fault(|| DbError::Unavailable("history source owner-slot reservation failed".to_string())));
        }
        if !builder
            .edit_cursor(|cursor| {
                let Some(owners) = cursor.source_pages.as_mut() else {
                    return false;
                };
                owners.resize_with(HISTORY_REPLAY_SEGMENT_PAGES as usize, || None);
                true
            })
            .unwrap_or(false)
        {
            return Err(builder.fault(|| DbError::Unavailable("history source owner-slot publication failed".to_string())));
        }
        if !builder.edit_cursor(|cursor| cursor.result_pages.as_mut().is_some_and(|owners| owners.try_reserve_exact(HISTORY_REPLAY_RESULT_PAGES).is_ok())).unwrap_or(false) {
            return Err(builder.fault(|| DbError::Unavailable("history result page-slot reservation failed".to_string())));
        }
        let mut result_owner_bytes = 0u64;
        for page_index in 0..HISTORY_REPLAY_RESULT_PAGES {
            if failure_after == Some(page_index) {
                return Err(builder.fault(|| DbError::Unavailable("history injected result page reservation failed".to_string())));
            }
            let mut page = Vec::new();
            if page.try_reserve_exact(HISTORY_REPLAY_PAGE_BYTES as usize).is_err() {
                return Err(builder.fault(|| DbError::Unavailable("history result page reservation failed".to_string())));
            }
            page.resize(HISTORY_REPLAY_PAGE_BYTES as usize, 0);
            let Some(next_result_owner_bytes) = result_owner_bytes.checked_add(page.capacity() as u64) else {
                let published = builder.edit_cursor(|cursor| {
                    cursor.result_pages.as_mut().is_some_and(|owners| {
                        owners.push(Some(page));
                        true
                    })
                });
                if !published.unwrap_or(false) {
                    return Err(builder.fault(|| DbError::Unavailable("history result page publication failed".to_string())));
                }
                return Err(builder.fault(|| DbError::LimitExceeded("history result reservation bytes")));
            };
            result_owner_bytes = next_result_owner_bytes;
            if !builder
                .edit_cursor(|cursor| {
                    cursor.result_pages.as_mut().is_some_and(|owners| {
                        owners.push(Some(page));
                        true
                    })
                })
                .unwrap_or(false)
            {
                return Err(builder.fault(|| DbError::Unavailable("history result page publication failed".to_string())));
            }
        }
        if failure_after == Some(HISTORY_REPLAY_RESULT_PAGES) {
            return Err(builder.fault(|| DbError::Unavailable("history injected result page boundary failed".to_string())));
        }
        if !builder.edit_cursor(|cursor| cursor.operation_ids.as_mut().is_some_and(|owners| owners.try_reserve_exact(HISTORY_REPLAY_MAX_OPERATION_IDS).is_ok())).unwrap_or(false) {
            return Err(builder.fault(|| DbError::Unavailable("history operation owner-slot reservation failed".to_string())));
        }
        if !builder.edit_cursor(|cursor| cursor.entries.as_mut().is_some_and(|owners| owners.try_reserve_exact(HISTORY_REPLAY_MAX_ENTRIES).is_ok())).unwrap_or(false) {
            return Err(builder.fault(|| DbError::Unavailable("history entry owner-slot reservation failed".to_string())));
        }
        let Some((result_slot_bytes, operation_slot_bytes, entry_slot_bytes, source_slot_bytes)) = builder
            .read_cursor(|cursor| {
                let result = cursor.result_pages.as_ref().map(Vec::capacity);
                let operations = cursor.operation_ids.as_ref().map(Vec::capacity);
                let entries = cursor.entries.as_ref().map(Vec::capacity);
                let source = cursor.source_pages.as_ref().map(Vec::capacity);
                match (result, operations, entries, source) {
                    (Some(result), Some(operations), Some(entries), Some(source)) => {
                        Some(((result * size_of::<Option<Vec<u8>>>()) as u64, (operations * size_of::<HistoryTextRange>()) as u64, (entries * size_of::<ArtifactHistoryEntry>()) as u64, (source * size_of::<Option<Vec<u8>>>()) as u64))
                    }
                    _ => None,
                }
            })
            .flatten()
        else {
            return Err(builder.fault(|| DbError::Unavailable("history construction owner accounting was unavailable".to_string())));
        };
        let Some(retained_result_bytes) = result_owner_bytes.checked_add(result_slot_bytes).and_then(|bytes| bytes.checked_add(operation_slot_bytes)).and_then(|bytes| bytes.checked_add(entry_slot_bytes)) else {
            return Err(builder.fault(|| DbError::LimitExceeded("history retained result bytes")));
        };
        let Some(retained_operation_bytes) = retained_result_bytes.checked_add(source_slot_bytes).and_then(|bytes| bytes.checked_add(HISTORY_REPLAY_MAX_FIELD_BYTES as u64)) else {
            return Err(builder.fault(|| DbError::LimitExceeded("history retained operation bytes")));
        };
        if !builder.edit_cursor(|cursor| cursor.scratch.as_mut().is_some_and(|scratch| scratch.try_reserve_exact(HISTORY_REPLAY_MAX_FIELD_BYTES).is_ok())).unwrap_or(false) {
            return Err(builder.fault(|| DbError::Unavailable("history scratch page reservation failed".to_string())));
        }
        if !builder
            .edit_cursor(|cursor| {
                cursor.scratch.as_mut().is_some_and(|scratch| {
                    scratch.resize(HISTORY_REPLAY_MAX_FIELD_BYTES, 0);
                    true
                })
            })
            .unwrap_or(false)
        {
            return Err(builder.fault(|| DbError::Unavailable("history scratch page publication failed".to_string())));
        }
        builder.finish(retained_operation_bytes, retained_result_bytes)
    }

    fn retained_bytes(&self) -> Option<u64> {
        Some(self.retained_operation_bytes)
    }

    fn preflight_result_range(&self, result_len: u64, len: u64) -> Result<u64, DbError> {
        if self.operation_ids.len() >= HISTORY_REPLAY_MAX_OPERATION_IDS {
            return Err(DbError::LimitExceeded("history operation item credit"));
        }
        match result_len.checked_add(len) {
            Some(end) if end <= HISTORY_REPLAY_RESULT_BYTES => Ok(end),
            _ => Err(DbError::LimitExceeded("history result byte credit")),
        }
    }

    #[cfg(test)]
    fn retain_source_page(&mut self, page: Vec<u8>) -> Result<(), Vec<u8>> {
        let Some(slot) = self.source_pages.get_mut(self.source_page_count) else {
            return Err(page);
        };
        if slot.is_some() {
            return Err(page);
        }
        *slot = Some(page);
        self.source_page_count += 1;
        Ok(())
    }
}

pub struct HistoryReplayReservationCloseCursor {
    source_pages: Option<Vec<Option<Vec<u8>>>>,
    source_page_count: usize,
    result_pages: Option<Vec<Option<Vec<u8>>>>,
    pub operation_ids: Option<Vec<HistoryTextRange>>,
    pub entries: Option<Vec<ArtifactHistoryEntry>>,
    scratch: Option<Vec<u8>>,
    retained_operation_bytes: u64,
    retained_result_bytes: u64,
    pub started: bool,
}

impl HistoryReplayReservationCloseCursor {
    pub fn new(reservation: HistoryReplayReservation) -> Self {
        let HistoryReplayReservation { source_pages, source_page_count, result_pages, operation_ids, entries, scratch, retained_operation_bytes, retained_result_bytes } = reservation;
        Self { source_pages: Some(source_pages), source_page_count, result_pages: Some(result_pages), operation_ids: Some(operation_ids), entries: Some(entries), scratch, retained_operation_bytes, retained_result_bytes, started: false }
    }

    pub fn close_step(&mut self) -> bool {
        self.started = true;
        if self.source_page_count != 0 {
            self.source_page_count -= 1;
            if let Some(owner) = self.source_pages.as_mut().and_then(|pages| pages.get_mut(self.source_page_count)).and_then(Option::take) {
                drop(owner);
            }
            return true;
        }
        if self.operation_ids.as_mut().is_some_and(|owners| owners.pop().is_some()) {
            return true;
        }
        if self.entries.as_mut().is_some_and(|owners| owners.pop().is_some()) {
            return true;
        }
        if let Some(owner) = self.result_pages.as_mut().and_then(Vec::pop) {
            drop(owner);
            return true;
        }
        if self.scratch.take().is_some() {
            return true;
        }
        if self.source_pages.take().is_some() {
            return true;
        }
        if self.operation_ids.take().is_some() {
            return true;
        }
        if self.entries.take().is_some() {
            return true;
        }
        if self.result_pages.take().is_some() {
            return true;
        }
        false
    }

    pub fn resume(&mut self) -> Option<HistoryReplayReservation> {
        if self.started {
            return None;
        }
        Some(HistoryReplayReservation {
            source_pages: self.source_pages.take()?,
            source_page_count: self.source_page_count,
            result_pages: self.result_pages.take()?,
            operation_ids: self.operation_ids.take()?,
            entries: self.entries.take()?,
            scratch: self.scratch.take(),
            retained_operation_bytes: self.retained_operation_bytes,
            retained_result_bytes: self.retained_result_bytes,
        })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.source_pages.is_none() && self.result_pages.is_none() && self.operation_ids.is_none() && self.entries.is_none() && self.scratch.is_none()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HistoryReplayReservationConstructionToken {
    pub slot: usize,
    pub generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct HistoryReplayReservationConstructionHandbackRejection {
    pub slot: usize,
    pub generation: u64,
}

struct HistoryReplayReservationConstructionSlot {
    pub generation: u64,
    occupied: bool,
    checked_out: bool,
    pub error: Option<DbError>,
    pub cursor: Option<HistoryReplayReservationCloseCursor>,
}

impl Default for HistoryReplayReservationConstructionSlot {
    fn default() -> Self {
        Self { generation: 0, occupied: false, checked_out: false, error: None, cursor: None }
    }
}

struct HistoryReplayReservationConstructionRegistry {
    pub slots: [HistoryReplayReservationConstructionSlot; HISTORY_REPLAY_CONSTRUCTION_SLOTS],
    next_generation: u64,
}

fn history_replay_reservation_construction_registry() -> &'static std::sync::Mutex<HistoryReplayReservationConstructionRegistry> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<HistoryReplayReservationConstructionRegistry>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(HistoryReplayReservationConstructionRegistry { slots: std::array::from_fn(|_| HistoryReplayReservationConstructionSlot::default()), next_generation: 1 }))
}

fn claim_history_replay_reservation_construction() -> Result<HistoryReplayReservationConstructionToken, DbError> {
    let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(slot) = registry.slots.iter().position(|slot| !slot.occupied) else {
        return Err(DbError::Unavailable("history construction registry capacity exhausted".to_string()));
    };
    let generation = registry.next_generation;
    let Some(next_generation) = generation.checked_add(1) else {
        return Err(DbError::LimitExceeded("history construction generation"));
    };
    registry.next_generation = next_generation;
    registry.slots[slot] = HistoryReplayReservationConstructionSlot {
        generation,
        occupied: true,
        checked_out: true,
        error: None,
        cursor: Some(HistoryReplayReservationCloseCursor {
            source_pages: Some(Vec::new()),
            source_page_count: 0,
            result_pages: Some(Vec::new()),
            operation_ids: Some(Vec::new()),
            entries: Some(Vec::new()),
            scratch: Some(Vec::new()),
            retained_operation_bytes: 0,
            retained_result_bytes: 0,
            started: false,
        }),
    };
    Ok(HistoryReplayReservationConstructionToken { slot, generation })
}

fn handback_history_replay_reservation_construction(token: &HistoryReplayReservationConstructionToken) -> Result<(), HistoryReplayReservationConstructionHandbackRejection> {
    let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(slot) = registry.slots.get_mut(token.slot) else {
        return Err(HistoryReplayReservationConstructionHandbackRejection { slot: token.slot, generation: token.generation });
    };
    if !slot.occupied || !slot.checked_out || slot.generation != token.generation || slot.cursor.is_none() {
        return Err(HistoryReplayReservationConstructionHandbackRejection { slot: token.slot, generation: token.generation });
    }
    slot.checked_out = false;
    Ok(())
}

fn release_history_replay_reservation_construction(token: &HistoryReplayReservationConstructionToken) -> bool {
    let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(slot) = registry.slots.get_mut(token.slot) else {
        return false;
    };
    if !slot.occupied || !slot.checked_out || slot.generation != token.generation || slot.error.is_some() || slot.cursor.is_some() {
        return false;
    }
    slot.generation = 0;
    slot.occupied = false;
    slot.checked_out = false;
    true
}

#[cfg(test)]
pub(crate) fn take_history_replay_reservation_construction_fault(generation: u64) -> Option<HistoryReplayReservationConstructionFault> {
    let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let slot_index = registry.slots.iter().position(|slot| slot.occupied && !slot.checked_out && slot.generation == generation)?;
    let slot = &mut registry.slots[slot_index];
    slot.checked_out = true;
    Some(HistoryReplayReservationConstructionFault { token: Some(HistoryReplayReservationConstructionToken { slot: slot_index, generation }), unregistered_error: None })
}

struct HistoryReplayReservationConstructionBuilder {
    pub token: Option<HistoryReplayReservationConstructionToken>,
}

impl HistoryReplayReservationConstructionBuilder {
    pub fn new() -> Result<Self, HistoryReplayReservationConstructionFault> {
        let token = match claim_history_replay_reservation_construction() {
            Ok(token) => token,
            Err(error) => return Err(HistoryReplayReservationConstructionFault::unregistered(error)),
        };
        Ok(Self { token: Some(token) })
    }

    fn edit_cursor<T>(&mut self, edit: impl FnOnce(&mut HistoryReplayReservationCloseCursor) -> T) -> Option<T> {
        let token = self.token.as_ref()?;
        let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = registry.slots.get_mut(token.slot)?;
        if !slot.occupied || !slot.checked_out || slot.generation != token.generation || slot.error.is_some() {
            return None;
        }
        slot.cursor.as_mut().map(edit)
    }

    fn read_cursor<T>(&self, read: impl FnOnce(&HistoryReplayReservationCloseCursor) -> T) -> Option<T> {
        let token = self.token.as_ref()?;
        let registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = registry.slots.get(token.slot)?;
        if !slot.occupied || !slot.checked_out || slot.generation != token.generation || slot.error.is_some() {
            return None;
        }
        slot.cursor.as_ref().map(read)
    }

    pub fn fault(mut self, error: impl FnOnce() -> DbError) -> HistoryReplayReservationConstructionFault {
        let Some(token) = self.token.take() else {
            return HistoryReplayReservationConstructionFault::unregistered(DbError::Unavailable("history construction token was unavailable".to_string()));
        };
        {
            let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(slot) = registry.slots.get_mut(token.slot) else {
                return HistoryReplayReservationConstructionFault { token: Some(token), unregistered_error: None };
            };
            if slot.occupied && slot.checked_out && slot.generation == token.generation && slot.error.is_none() && slot.cursor.is_some() {
                slot.error = Some(error());
            }
        }
        HistoryReplayReservationConstructionFault { token: Some(token), unregistered_error: None }
    }

    pub fn finish(mut self, retained_operation_bytes: u64, retained_result_bytes: u64) -> Result<HistoryReplayReservation, HistoryReplayReservationConstructionFault> {
        let Some(token) = self.token.take() else {
            return Err(HistoryReplayReservationConstructionFault::unregistered(DbError::Unavailable("history construction token was unavailable".to_string())));
        };
        let reservation = {
            let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(slot) = registry.slots.get_mut(token.slot) else {
                return Err(HistoryReplayReservationConstructionFault { token: Some(token), unregistered_error: None });
            };
            if !slot.occupied || !slot.checked_out || slot.generation != token.generation || slot.error.is_some() {
                return Err(HistoryReplayReservationConstructionFault { token: Some(token), unregistered_error: None });
            }
            let Some(mut cursor) = slot.cursor.take() else {
                return Err(HistoryReplayReservationConstructionFault { token: Some(token), unregistered_error: None });
            };
            cursor.retained_operation_bytes = retained_operation_bytes;
            cursor.retained_result_bytes = retained_result_bytes;
            let Some(reservation) = cursor.resume() else {
                slot.cursor = Some(cursor);
                return Err(HistoryReplayReservationConstructionFault { token: Some(token), unregistered_error: None });
            };
            slot.generation = 0;
            slot.occupied = false;
            slot.checked_out = false;
            reservation
        };
        Ok(reservation)
    }
}

impl Drop for HistoryReplayReservationConstructionBuilder {
    fn drop(&mut self) {
        if let Some(token) = self.token.as_ref() {
            let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(slot) = registry.slots.get_mut(token.slot).filter(|slot| slot.occupied && slot.checked_out && slot.generation == token.generation && slot.error.is_none() && slot.cursor.is_some()) {
                slot.error = Some(DbError::Unavailable("history reservation construction unwound".to_string()));
            }
        }
        if let Some(token) = self.token.take() {
            let _ = handback_history_replay_reservation_construction(&token);
        }
    }
}

pub struct HistoryReplayReservationConstructionFault {
    pub token: Option<HistoryReplayReservationConstructionToken>,
    unregistered_error: Option<DbError>,
}

impl HistoryReplayReservationConstructionFault {
    fn unregistered(error: DbError) -> Self {
        Self { token: None, unregistered_error: Some(error) }
    }

    pub fn generation(&self) -> Option<u64> {
        self.token.as_ref().map(|token| token.generation)
    }

    pub fn take_error(&mut self) -> Option<DbError> {
        if self.unregistered_error.is_some() {
            return self.unregistered_error.take();
        }
        let token = self.token.as_ref()?;
        let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = registry.slots.get_mut(token.slot)?;
        if !slot.occupied || !slot.checked_out || slot.generation != token.generation {
            return None;
        }
        slot.error.take()
    }

    pub fn close_step(&mut self) -> bool {
        if self.unregistered_error.take().is_some() {
            return true;
        }
        let Some(token) = self.token.as_ref() else {
            return false;
        };
        {
            let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(slot) = registry.slots.get_mut(token.slot) else {
                return false;
            };
            if !slot.occupied || !slot.checked_out || slot.generation != token.generation {
                return false;
            }
            if let Some(cursor) = slot.cursor.as_mut() {
                if cursor.close_step() {
                    return true;
                }
                if cursor.terminal_is_empty() {
                    slot.cursor = None;
                    return true;
                }
            }
            if slot.error.take().is_some() {
                return true;
            }
        }
        if release_history_replay_reservation_construction(token) {
            self.token = None;
            return true;
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.token.is_none() && self.unregistered_error.is_none()
    }

    #[cfg(test)]
    fn retained_result_page_count(&self) -> usize {
        let Some(token) = self.token.as_ref() else {
            return 0;
        };
        let registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        registry.slots.get(token.slot).filter(|slot| slot.occupied && slot.generation == token.generation).and_then(|slot| slot.cursor.as_ref()).and_then(|cursor| cursor.result_pages.as_ref()).map_or(0, Vec::len)
    }

    #[cfg(test)]
    fn retained_result_page_pointer(&self, index: usize) -> Option<*const u8> {
        let token = self.token.as_ref()?;
        let registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        registry
            .slots
            .get(token.slot)
            .filter(|slot| slot.occupied && slot.generation == token.generation)
            .and_then(|slot| slot.cursor.as_ref())
            .and_then(|cursor| cursor.result_pages.as_ref())
            .and_then(|owners| owners.get(index))
            .and_then(Option::as_ref)
            .map(Vec::as_ptr)
    }

    #[cfg(test)]
    fn retained_error_pointer(&self) -> Option<*const u8> {
        if let Some(DbError::Unavailable(message)) = self.unregistered_error.as_ref() {
            return Some(message.as_ptr());
        }
        let token = self.token.as_ref()?;
        let registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match registry.slots.get(token.slot).filter(|slot| slot.occupied && slot.generation == token.generation)?.error.as_ref()? {
            DbError::Unavailable(message) => Some(message.as_ptr()),
            _ => None,
        }
    }
}

impl std::fmt::Debug for HistoryReplayReservationConstructionFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("HistoryReplayReservationConstructionFault").field("generation", &self.generation()).field("terminal_is_empty", &self.terminal_is_empty()).finish()
    }
}

impl Drop for HistoryReplayReservationConstructionFault {
    fn drop(&mut self) {
        if let Some(token) = self.token.take() {
            let _ = handback_history_replay_reservation_construction(&token);
        }
    }
}

impl Drop for HistoryReplayReservationCloseCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "history replay reservation reached Drop before retained retirement or exact resume");
    }
}

#[derive(Default)]
struct HistoryPageSet {
    pub pages: Vec<Option<Vec<u8>>>,
    pub len: u64,
}

impl db_wal_log::WalImmutableByteSource for HistoryPageSet {
    fn byte_len(&self) -> usize {
        self.len as usize
    }

    fn fragment_at(&self, offset: usize, limit: usize) -> Result<&[u8], DbError> {
        if offset >= limit || limit as u64 > self.len {
            return Err(DbError::Corrupt("history immutable source range".to_string()));
        }
        let bytes = self.page_slice(offset as u64, (limit - offset) as u64)?;
        if bytes.is_empty() {
            return Err(DbError::Corrupt("history immutable source lost bytes".to_string()));
        }
        Ok(bytes)
    }
}

impl HistoryPageSet {
    pub fn byte(&self, offset: u64) -> Result<u8, DbError> {
        if offset >= self.len {
            return Err(DbError::Corrupt("history page cursor exceeded the segment".to_string()));
        }
        let page = usize::try_from(offset / HISTORY_REPLAY_PAGE_BYTES).map_err(|_| DbError::LimitExceeded("history page index"))?;
        let within = usize::try_from(offset % HISTORY_REPLAY_PAGE_BYTES).map_err(|_| DbError::LimitExceeded("history page offset"))?;
        self.pages.get(page).and_then(Option::as_ref).and_then(|bytes| bytes.get(within)).copied().ok_or_else(|| DbError::Corrupt("history page owner is missing admitted bytes".to_string()))
    }

    fn page_slice(&self, offset: u64, maximum: u64) -> Result<&[u8], DbError> {
        if offset >= self.len {
            return Ok(&[]);
        }
        let page = usize::try_from(offset / HISTORY_REPLAY_PAGE_BYTES).map_err(|_| DbError::LimitExceeded("history page index"))?;
        let within = usize::try_from(offset % HISTORY_REPLAY_PAGE_BYTES).map_err(|_| DbError::LimitExceeded("history page offset"))?;
        let bytes = self.pages.get(page).and_then(Option::as_ref).ok_or_else(|| DbError::Corrupt("history page owner is missing".to_string()))?;
        let available = bytes.len().checked_sub(within).ok_or_else(|| DbError::Corrupt("history page offset exceeds owner".to_string()))?;
        let take = available.min(usize::try_from(maximum).unwrap_or(usize::MAX));
        Ok(&bytes[within..within + take])
    }

    fn read_varint(&self, pos: &mut u64, end: u64) -> Result<u64, DbError> {
        let mut next = *pos;
        let value = db_wal_log::wal_read_canonical_varint(|| {
            if next >= end {
                return Err(DbError::Corrupt("history varint exceeds its admitted range".to_string()));
            }
            let byte = self.byte(next)?;
            next = next.checked_add(1).ok_or(DbError::LimitExceeded("history varint offset"))?;
            Ok(byte)
        })?;
        *pos = next;
        Ok(value)
    }

    fn read_range(&self, pos: &mut u64, end: u64, maximum: u64) -> Result<std::ops::Range<u64>, DbError> {
        let len = self.read_varint(pos, end)?;
        if len > maximum {
            return Err(DbError::LimitExceeded("history envelope field bytes"));
        }
        let start = *pos;
        let next = start.checked_add(len).ok_or(DbError::LimitExceeded("history envelope field range"))?;
        if next > end {
            return Err(DbError::Corrupt("history envelope field exceeds its frame payload".to_string()));
        }
        *pos = next;
        Ok(start..next)
    }

    fn copy_small<'a>(&self, range: std::ops::Range<u64>, scratch: &'a mut [u8]) -> Result<&'a [u8], DbError> {
        let len = range.end.checked_sub(range.start).ok_or(DbError::LimitExceeded("history copied range"))?;
        if len > HISTORY_REPLAY_PAGE_BYTES {
            return Err(DbError::LimitExceeded("history scratch page bytes"));
        }
        let first = self.page_slice(range.start, len)?;
        scratch[..first.len()].copy_from_slice(first);
        let remaining = usize::try_from(len).map_err(|_| DbError::LimitExceeded("history scratch page bytes"))?.checked_sub(first.len()).ok_or(DbError::LimitExceeded("history scratch page bytes"))?;
        if remaining != 0 {
            let offset = range.start.checked_add(first.len() as u64).ok_or(DbError::LimitExceeded("history scratch page offset"))?;
            let second = self.page_slice(offset, remaining as u64)?;
            if second.len() != remaining {
                return Err(DbError::Corrupt("history scratch page is truncated".to_string()));
            }
            scratch[first.len()..first.len() + remaining].copy_from_slice(second);
        }
        Ok(&scratch[..len as usize])
    }

    fn read_array<const N: usize>(&self, pos: &mut u64, end: u64, scratch: &mut [u8]) -> Result<[u8; N], DbError> {
        let next = pos.checked_add(N as u64).ok_or(DbError::LimitExceeded("history scalar range"))?;
        if next > end {
            return Err(DbError::Corrupt("history scalar exceeds its frame payload".to_string()));
        }
        let owner = self.copy_small(*pos..next, scratch)?;
        *pos = next;
        owner.try_into().map_err(|_| DbError::Corrupt("history scalar length mismatch".to_string()))
    }
}

pub enum HistoryEnvelopeField {
    MutationId,
    Document,
    Actor,
    DependencyCount,
    Dependency,
    ObservedFlag,
    Observed,
    TargetCount,
    TargetSegment,
    DiffSchema,
    DiffPayload,
    InverseSchema,
    InversePayload,
    ClockActor,
    ClockPhysical,
    ClockLogical,
    TrailingFlags,
    TransactionId,
    TransactionTool,
    Verb,
    Line,
    Done,
}

pub struct HistoryEnvelopeCursor {
    pub pos: u64,
    pub end: u64,
    pub field: HistoryEnvelopeField,
    pub dependencies: u64,
    pub target_segments: u64,
    pub verb: bool,
    pub line: bool,
    pub mutation_id: Option<std::ops::Range<u64>>,
}

impl HistoryEnvelopeCursor {
    pub fn new(pages: &HistoryPageSet, offset: u64, len: u64) -> Result<Self, DbError> {
        let end = offset.checked_add(len).ok_or(DbError::LimitExceeded("history command range"))?;
        if end > pages.len {
            return Err(DbError::Corrupt("history command range exceeds retained pages".to_string()));
        }
        Ok(Self { pos: offset, end, field: HistoryEnvelopeField::MutationId, dependencies: 0, target_segments: 0, verb: false, line: false, mutation_id: None })
    }

    pub fn finish(&self) -> Result<HistoryEnvelopeField, DbError> {
        if self.pos != self.end {
            return Err(DbError::Corrupt("history envelope has trailing bytes".to_string()));
        }
        Ok(HistoryEnvelopeField::Done)
    }

    fn skip_text(&mut self, pages: &HistoryPageSet, scratch: &mut [u8]) -> Result<(), DbError> {
        let range = pages.read_range(&mut self.pos, self.end, HISTORY_REPLAY_MAX_FIELD_BYTES as u64)?;
        let scratch = pages.copy_small(range, scratch)?;
        std::str::from_utf8(&scratch).map_err(|_| DbError::Corrupt("history envelope text is not valid utf-8".to_string()))?;
        Ok(())
    }

    pub fn step(&mut self, pages: &HistoryPageSet, scratch: &mut [u8], document: &ArtifactId) -> Result<Option<std::ops::Range<u64>>, DbError> {
        match self.field {
            HistoryEnvelopeField::MutationId => {
                let range = pages.read_range(&mut self.pos, self.end, HISTORY_REPLAY_MAX_FIELD_BYTES as u64)?;
                let value = pages.copy_small(range.clone(), scratch)?;
                std::str::from_utf8(value).map_err(|_| DbError::Corrupt("history operation id is not valid utf-8".to_string()))?;
                self.mutation_id = Some(range);
                self.field = HistoryEnvelopeField::Document;
            }
            HistoryEnvelopeField::Document => {
                let range = pages.read_range(&mut self.pos, self.end, HISTORY_REPLAY_MAX_FIELD_BYTES as u64)?;
                if pages.copy_small(range, scratch)? != document.0.as_bytes() {
                    return Err(DbError::Corrupt("history envelope document differs".to_string()));
                }
                self.field = HistoryEnvelopeField::Actor;
            }
            HistoryEnvelopeField::Actor => {
                self.skip_text(pages, scratch)?;
                self.field = HistoryEnvelopeField::DependencyCount;
            }
            HistoryEnvelopeField::DependencyCount => {
                self.dependencies = pages.read_varint(&mut self.pos, self.end)?;
                if self.dependencies > HISTORY_REPLAY_MAX_OPERATION_IDS as u64 {
                    return Err(DbError::LimitExceeded("history dependency item credit"));
                }
                self.field = if self.dependencies == 0 { HistoryEnvelopeField::ObservedFlag } else { HistoryEnvelopeField::Dependency };
            }
            HistoryEnvelopeField::Dependency => {
                self.skip_text(pages, scratch)?;
                self.dependencies -= 1;
                if self.dependencies == 0 {
                    self.field = HistoryEnvelopeField::ObservedFlag;
                }
            }
            HistoryEnvelopeField::ObservedFlag => {
                self.field = match pages.read_varint(&mut self.pos, self.end)? {
                    0 => HistoryEnvelopeField::TargetCount,
                    1 => HistoryEnvelopeField::Observed,
                    _ => return Err(DbError::Corrupt("history envelope observed flag is invalid".to_string())),
                };
            }
            HistoryEnvelopeField::Observed => {
                self.skip_text(pages, scratch)?;
                self.field = HistoryEnvelopeField::TargetCount;
            }
            HistoryEnvelopeField::TargetCount => {
                self.target_segments = pages.read_varint(&mut self.pos, self.end)?;
                if self.target_segments > HISTORY_REPLAY_MAX_OPERATION_IDS as u64 {
                    return Err(DbError::LimitExceeded("history target segment credit"));
                }
                self.field = if self.target_segments == 0 { HistoryEnvelopeField::DiffSchema } else { HistoryEnvelopeField::TargetSegment };
            }
            HistoryEnvelopeField::TargetSegment => {
                self.skip_text(pages, scratch)?;
                self.target_segments -= 1;
                if self.target_segments == 0 {
                    self.field = HistoryEnvelopeField::DiffSchema;
                }
            }
            HistoryEnvelopeField::DiffSchema => {
                self.skip_text(pages, scratch)?;
                self.field = HistoryEnvelopeField::DiffPayload;
            }
            HistoryEnvelopeField::DiffPayload => {
                pages.read_range(&mut self.pos, self.end, HISTORY_REPLAY_MAX_FRAME_BYTES)?;
                self.field = HistoryEnvelopeField::InverseSchema;
            }
            HistoryEnvelopeField::InverseSchema => {
                self.skip_text(pages, scratch)?;
                self.field = HistoryEnvelopeField::InversePayload;
            }
            HistoryEnvelopeField::InversePayload => {
                pages.read_range(&mut self.pos, self.end, HISTORY_REPLAY_MAX_FRAME_BYTES)?;
                self.field = HistoryEnvelopeField::ClockActor;
            }
            HistoryEnvelopeField::ClockActor => {
                pages.read_varint(&mut self.pos, self.end)?;
                self.field = HistoryEnvelopeField::ClockPhysical;
            }
            HistoryEnvelopeField::ClockPhysical => {
                pages.read_varint(&mut self.pos, self.end)?;
                self.field = HistoryEnvelopeField::ClockLogical;
            }
            HistoryEnvelopeField::ClockLogical => {
                pages.read_varint(&mut self.pos, self.end)?;
                self.field = HistoryEnvelopeField::TrailingFlags;
            }
            HistoryEnvelopeField::TrailingFlags => {
                let flags = pages.read_varint(&mut self.pos, self.end)?;
                if flags > 0b111 {
                    return Err(DbError::Corrupt("history envelope trailing flags are invalid".to_string()));
                }
                self.verb = flags & 0b10 != 0;
                self.line = flags & 0b100 != 0;
                self.field = match (flags & 0b01 != 0, self.verb, self.line) {
                    (true, _, _) => HistoryEnvelopeField::TransactionId,
                    (false, true, _) => HistoryEnvelopeField::Verb,
                    (false, false, true) => HistoryEnvelopeField::Line,
                    (false, false, false) => self.finish()?,
                };
            }
            HistoryEnvelopeField::TransactionId => {
                self.skip_text(pages, scratch)?;
                self.field = HistoryEnvelopeField::TransactionTool;
            }
            HistoryEnvelopeField::TransactionTool => {
                self.skip_text(pages, scratch)?;
                self.field = if self.verb { HistoryEnvelopeField::Verb } else if self.line { HistoryEnvelopeField::Line } else { self.finish()? };
            }
            HistoryEnvelopeField::Verb => {
                self.skip_text(pages, scratch)?;
                self.field = if self.line { HistoryEnvelopeField::Line } else { self.finish()? };
            }
            HistoryEnvelopeField::Line => {
                self.skip_text(pages, scratch)?;
                self.field = self.finish()?;
            }
            HistoryEnvelopeField::Done => return Ok(self.mutation_id.take()),
        }
        Ok(None)
    }
}

pub enum HistoryFrontierField {
    Document,
    Head,
    Commit,
    Chain,
    Epoch,
}

pub struct HistoryFrontierCursor {
    pub pos: u64,
    pub end: u64,
    pub field: HistoryFrontierField,
    pub head_seq: u64,
    pub commit_seq: u64,
    pub chain_hash: [u8; 32],
}

impl HistoryFrontierCursor {
    pub fn new(pages: &HistoryPageSet, offset: u64, len: u64) -> Result<Self, DbError> {
        let end = offset.checked_add(len).ok_or(DbError::LimitExceeded("history frontier range"))?;
        if end > pages.len {
            return Err(DbError::Corrupt("history frontier range exceeds retained pages".to_string()));
        }
        Ok(Self { pos: offset, end, field: HistoryFrontierField::Document, head_seq: 0, commit_seq: 0, chain_hash: [0; 32] })
    }

    pub fn step(&mut self, pages: &HistoryPageSet, scratch: &mut [u8], document: &ArtifactId) -> Result<Option<(u64, u64, [u8; 32], u64)>, DbError> {
        match self.field {
            HistoryFrontierField::Document => {
                let range = pages.read_range(&mut self.pos, self.end, HISTORY_REPLAY_MAX_FIELD_BYTES as u64)?;
                if pages.copy_small(range, scratch)? != document.0.as_bytes() {
                    return Err(DbError::Corrupt("history frontier document differs".to_string()));
                }
                self.field = HistoryFrontierField::Head;
            }
            HistoryFrontierField::Head => {
                self.head_seq = u64::from_le_bytes(pages.read_array(&mut self.pos, self.end, scratch)?);
                self.field = HistoryFrontierField::Commit;
            }
            HistoryFrontierField::Commit => {
                self.commit_seq = u64::from_le_bytes(pages.read_array(&mut self.pos, self.end, scratch)?);
                self.field = HistoryFrontierField::Chain;
            }
            HistoryFrontierField::Chain => {
                self.chain_hash = pages.read_array(&mut self.pos, self.end, scratch)?;
                self.field = HistoryFrontierField::Epoch;
            }
            HistoryFrontierField::Epoch => {
                let epoch = u64::from_le_bytes(pages.read_array(&mut self.pos, self.end, scratch)?);
                if self.pos != self.end {
                    return Err(DbError::Corrupt("history frontier has trailing bytes".to_string()));
                }
                return Ok(Some((self.head_seq, self.commit_seq, self.chain_hash, epoch)));
            }
        }
        Ok(None)
    }
}

type HistorySegmentLenFuture = Pin<Box<dyn Future<Output = Result<u64, DbError>> + Send + 'static>>;

type HistoryPageReadFuture = Pin<Box<dyn Future<Output = Result<db_io_core::DbIoPages, DbError>> + Send + 'static>>;

type HistorySegmentListFuture = Pin<Box<dyn Future<Output = Result<db_io_core::DbIoU64List, DbError>> + Send + 'static>>;

pub enum HistoryReplayPhase {
    Inventory { future: HistorySegmentListFuture },
    Probe,
    SegmentLen { index: u64, future: HistorySegmentLenFuture },
    PageStart { index: u64, len: u64, offset: u64 },
    PageRead { index: u64, len: u64, offset: u64, requested: u64, future: HistoryPageReadFuture },
    PageClose { index: u64, len: u64, offset: u64, retained: db_io_core::DbIoPages },
    Verify { index: u64 },
    Frame { index: u64 },
    CommittedBody { index: u64 },
    Envelope { index: u64, cursor: HistoryEnvelopeCursor },
    CopyMutation { index: u64, range: std::ops::Range<u64>, copied: u64, result_start: u64 },
    Frontier { index: u64, cursor: HistoryFrontierCursor },
    Publish { index: u64, head_seq: u64, commit_seq: u64, chain_hash: [u8; 32], epoch: u64 },
    Retire,
    InventoryClose,
    FinalizeSuccess,
}

pub enum HistoryReplayTransition {
    InProgress,
    FaultRetire,
    Complete,
}

pub struct HistoryReplayFuture {
    pub storage: Arc<db_storage::DbBackend>,
    pub document: Arc<ArtifactId>,
    pub phase: Option<HistoryReplayPhase>,
    pub transition: HistoryReplayTransition,
    pub pages: HistoryPageSet,
    pub authenticated: Option<db_wal_log::WalAuthenticatedSource<HistoryPageSet>>,
    gate: Option<db_wal_log::WalTransactionGate>,
    previous_tip: db_wal_log::WalPriorChainTip,
    segments: Option<db_io_core::DbIoU64List>,
    segment_ordinal: usize,
    body_index: usize,
    pending_frontier: Option<(u64, u64, [u8; 32], u64)>,
    page_count: usize,
    pub reservation: Option<HistoryReplayReservation>,
    reservation_close: Option<HistoryReplayReservationCloseCursor>,
    terminal_page: Option<Vec<u8>>,
    terminal_error: Option<DbError>,
    pub result_len: u64,
    pending_start: usize,
    pub operation_generation: u64,
    pub cancelled: Arc<std::sync::atomic::AtomicBool>,
    pub panic_before_transition_commit: bool,
}

impl HistoryReplayFuture {
    pub fn new(storage: Arc<db_storage::DbBackend>, document: ArtifactId, operation_generation: u64, cancelled: Arc<std::sync::atomic::AtomicBool>, mut reservation: HistoryReplayReservation) -> Self {
        let source_pages = std::mem::take(&mut reservation.source_pages);
        reservation.source_page_count = 0;
        let inventory_storage = storage.clone();
        let inventory_document = document.clone();
        Self {
            storage,
            document: Arc::new(document),
            phase: Some(HistoryReplayPhase::Inventory { future: Box::pin(async move { inventory_storage.wal().await.list_segments(&inventory_document).await }) }),
            transition: HistoryReplayTransition::InProgress,
            pages: HistoryPageSet { pages: source_pages, len: 0 },
            authenticated: None,
            gate: Some(db_wal_log::WalTransactionGate::new()),
            previous_tip: db_wal_log::WalPriorChainTip::Genesis,
            segments: None,
            segment_ordinal: 0,
            body_index: 0,
            pending_frontier: None,
            page_count: 0,
            reservation: Some(reservation),
            reservation_close: None,
            terminal_page: None,
            terminal_error: None,
            result_len: 0,
            pending_start: 0,
            operation_generation,
            cancelled,
            panic_before_transition_commit: false,
        }
    }

    pub fn request_close(&mut self, error: DbError) {
        if self.terminal_error.is_none() {
            self.terminal_error = Some(error);
        }
        if !matches!(self.transition, HistoryReplayTransition::Complete) {
            self.transition = HistoryReplayTransition::FaultRetire;
        }
    }

    pub fn close_step(&mut self) -> bool {
        if let Some(source) = self.authenticated.take() {
            self.pages = source.abort_into_source();
            return true;
        }
        if self.terminal_page.take().is_some() {
            return true;
        }
        if self.page_count != 0 {
            self.page_count -= 1;
            drop(self.pages.pages[self.page_count].take());
            return true;
        }
        if !self.pages.pages.is_empty() {
            drop(std::mem::take(&mut self.pages.pages));
            return true;
        }
        if let Some(segments) = self.segments.as_mut() {
            if segments.close_step() {
                return true;
            }
            if !segments.terminal_is_empty() {
                return true;
            }
            self.segments = None;
            return true;
        }
        if self.reservation_close.is_none() {
            if let Some(reservation) = self.reservation.take() {
                self.reservation_close = Some(HistoryReplayReservationCloseCursor::new(reservation));
                return true;
            }
        }
        if self.reservation_close.as_mut().is_some_and(HistoryReplayReservationCloseCursor::close_step) {
            return true;
        }
        if self.reservation_close.as_ref().is_some_and(HistoryReplayReservationCloseCursor::terminal_is_empty) {
            self.reservation_close = None;
            return true;
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.authenticated.is_none()
            && self.segments.is_none()
            && self.terminal_page.is_none()
            && self.page_count == 0
            && self.pages.pages.is_empty()
            && self.reservation.is_none()
            && self.reservation_close.is_none()
            && self.phase.is_none()
            && matches!(self.transition, HistoryReplayTransition::Complete)
    }

    fn begin_fault(&mut self, error: DbError) {
        self.request_close(error);
    }

    fn fail_pending(&mut self, error: DbError) {
        self.begin_fault(error);
    }

    fn copy_mutation_fragment(pages: &HistoryPageSet, reservation: &mut Option<HistoryReplayReservation>, range: &std::ops::Range<u64>, copied: u64, result_start: u64) -> Result<u64, DbError> {
        let remaining = range.end.checked_sub(range.start).and_then(|len| len.checked_sub(copied)).ok_or(DbError::LimitExceeded("history result copy range"))?;
        if remaining == 0 {
            return Ok(0);
        }
        let source_offset = range.start.checked_add(copied).ok_or(DbError::LimitExceeded("history result source offset"))?;
        let destination_offset = result_start.checked_add(copied).ok_or(DbError::LimitExceeded("history result destination offset"))?;
        let source = pages.page_slice(source_offset, remaining.min(HISTORY_REPLAY_PAGE_BYTES))?;
        let page_index = usize::try_from(destination_offset / HISTORY_REPLAY_PAGE_BYTES).map_err(|_| DbError::LimitExceeded("history result page index"))?;
        let within = usize::try_from(destination_offset % HISTORY_REPLAY_PAGE_BYTES).map_err(|_| DbError::LimitExceeded("history result page offset"))?;
        let reservation = reservation.as_mut().ok_or(DbError::Closed)?;
        let destination = reservation.result_pages.get_mut(page_index).and_then(Option::as_mut).ok_or_else(|| DbError::Corrupt("history result page owner is missing".to_string()))?;
        let take = source.len().min(destination.len().saturating_sub(within));
        if take == 0 {
            return Err(DbError::Corrupt("history result copy made no progress".to_string()));
        }
        destination[within..within + take].copy_from_slice(&source[..take]);
        Ok(take as u64)
    }

    fn finish_view(&mut self) -> Result<ArtifactHistoryView, DbError> {
        let Some(reservation) = self.reservation.as_ref() else {
            return Err(DbError::Closed);
        };
        let retained_bytes = reservation.retained_result_bytes;
        let retained_items = reservation
            .result_pages
            .len()
            .checked_add(reservation.operation_ids.len())
            .and_then(|items| items.checked_add(reservation.entries.len()))
            .and_then(|items| items.checked_add(1))
            .ok_or(DbError::LimitExceeded("history retained result items"));
        let retained_items = retained_items?;
        let Some(reservation) = self.reservation.take() else {
            return Err(DbError::Closed);
        };
        if reservation.scratch.is_some() || !reservation.source_pages.is_empty() {
            self.reservation = Some(reservation);
            return Err(DbError::Internal("history final view retained non-result owners".to_string()));
        }
        Ok(ArtifactHistoryView { entries: reservation.entries, operation_ids: reservation.operation_ids, result_pages: reservation.result_pages, result_len: self.result_len, retained_bytes, retained_items })
    }

    pub fn next(self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Result<ArtifactHistoryView, DbError>> {
        let this = self.get_mut();
        if this.operation_generation == 0 {
            this.request_close(DbError::Closed);
        }
        if this.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            this.request_close(DbError::Closed);
        }
        if matches!(this.transition, HistoryReplayTransition::Complete) {
            return std::task::Poll::Ready(Err(this.terminal_error.take().unwrap_or(DbError::Closed)));
        }
        if matches!(this.transition, HistoryReplayTransition::FaultRetire) {
            if this.phase.take().is_some() {
                context.waker().wake_by_ref();
                return std::task::Poll::Pending;
            }
            if this.close_step() {
                context.waker().wake_by_ref();
                return std::task::Poll::Pending;
            }
            this.transition = HistoryReplayTransition::Complete;
            return std::task::Poll::Ready(Err(this.terminal_error.take().unwrap_or(DbError::Closed)));
        }
        let mut next = None;
        let mut fault = None;
        let mut finalize = false;
        let Some(active) = this.phase.as_mut() else {
            this.request_close(DbError::Internal("history replay transition authority was missing".to_string()));
            context.waker().wake_by_ref();
            return std::task::Poll::Pending;
        };
        match active {
            HistoryReplayPhase::Inventory { future } => match future.as_mut().poll(context) {
                std::task::Poll::Pending => {}
                std::task::Poll::Ready(Err(error)) => fault = Some(error),
                std::task::Poll::Ready(Ok(segments)) => {
                    this.segments = Some(segments);
                    next = Some(HistoryReplayPhase::Probe);
                }
            },
            HistoryReplayPhase::Probe => {
                if let Some(error) = this.terminal_error.take() {
                    fault = Some(error);
                } else {
                    let segments = this.segments.as_ref().map(db_io_core::DbIoU64List::as_slice).unwrap_or(&[]);
                    match segments.get(this.segment_ordinal).copied() {
                        None => next = Some(HistoryReplayPhase::InventoryClose),
                        Some(segment) if segments.first().and_then(|first| first.checked_add(this.segment_ordinal as u64)) != Some(segment) => fault = Some(DbError::Corrupt("history WAL inventory is not a dense retained suffix".to_string())),
                        Some(segment) => {
                            if this.segment_ordinal == 0 && segment != 0 {
                                this.previous_tip = db_wal_log::WalPriorChainTip::RetainedBoundary;
                            }
                            let storage = this.storage.clone();
                            let document = this.document.clone();
                            next = Some(HistoryReplayPhase::SegmentLen { index: segment, future: Box::pin(async move { storage.wal().await.segment_len(&document, segment).await }) });
                        }
                    }
                }
            }
            HistoryReplayPhase::SegmentLen { index, future } => match future.as_mut().poll(context) {
                std::task::Poll::Pending => {}
                std::task::Poll::Ready(Err(error)) => fault = Some(this.terminal_error.take().unwrap_or(error)),
                std::task::Poll::Ready(Ok(len)) => {
                    if let Some(error) = this.terminal_error.take() {
                        fault = Some(error);
                    } else if let Some(page_count) = len.checked_add(HISTORY_REPLAY_PAGE_BYTES - 1).map(|bytes| bytes / HISTORY_REPLAY_PAGE_BYTES) {
                        let inventory_bytes = this.segments.as_ref().map_or(0, db_io_core::DbIoU64List::retained_bytes);
                        let simultaneous = this
                            .reservation
                            .as_ref()
                            .and_then(HistoryReplayReservation::retained_bytes)
                            .and_then(|bytes| bytes.checked_add(len))
                            .and_then(|bytes| bytes.checked_add(inventory_bytes))
                            .and_then(|bytes| bytes.checked_add(size_of::<Self>() as u64));
                        if len < protocol::format::HEADER_SIZE as u64 || page_count > HISTORY_REPLAY_SEGMENT_PAGES || simultaneous.is_none_or(|bytes| bytes > HISTORY_REPLAY_OPERATION_BYTES) {
                            fault = Some(DbError::LimitExceeded("history replay page/source byte credit"));
                        } else {
                            this.pages.len = len;
                            next = Some(HistoryReplayPhase::PageStart { index: *index, len, offset: 0 });
                        }
                    } else {
                        fault = Some(DbError::LimitExceeded("history segment page rounding"));
                    }
                }
            },
            HistoryReplayPhase::PageStart { index, len, offset } => {
                if offset == len {
                    if let Some(gate) = this.gate.take() {
                        this.authenticated = Some(db_wal_log::WalAuthenticatedSource::new(std::mem::take(&mut this.pages), gate, *index, this.previous_tip));
                        next = Some(HistoryReplayPhase::Verify { index: *index });
                    } else {
                        fault = Some(DbError::Closed);
                    }
                } else {
                    match len.checked_sub(*offset) {
                        Some(remaining) => {
                            let requested = remaining.min(HISTORY_REPLAY_PAGE_BYTES);
                            let storage = this.storage.clone();
                            let document = this.document.clone();
                            let (index, len, offset) = (*index, *len, *offset);
                            next = Some(HistoryReplayPhase::PageRead {
                                index,
                                len,
                                offset,
                                requested,
                                future: Box::pin(async move { storage.wal().await.read(&document, index, pack::ByteRange { offset, len: requested }).await }),
                            });
                        }
                        None => fault = Some(DbError::LimitExceeded("history page remaining bytes")),
                    }
                }
            }
            HistoryReplayPhase::PageRead { index, len, offset, requested, future } => match future.as_mut().poll(context) {
                std::task::Poll::Pending => {}
                std::task::Poll::Ready(Err(error)) => fault = Some(this.terminal_error.take().unwrap_or(error)),
                std::task::Poll::Ready(Ok(retained)) => {
                    let mut page = Vec::with_capacity(retained.len());
                    for fragment in retained.fragments() {
                        page.extend_from_slice(fragment);
                    }
                    let page = page.into_boxed_slice().into_vec();
                    if let Some(error) = this.terminal_error.take() {
                        this.terminal_page = Some(page);
                        fault = Some(error);
                    } else if page.len() as u64 != *requested || page.capacity() as u64 > HISTORY_REPLAY_PAGE_BYTES || this.page_count >= HISTORY_REPLAY_SEGMENT_PAGES as usize {
                        this.terminal_page = Some(page);
                        fault = Some(DbError::LimitExceeded("history backend page ownership"));
                    } else if let Some(next_offset) = offset.checked_add(*requested) {
                        this.pages.pages[this.page_count] = Some(page);
                        this.page_count += 1;
                        next = Some(HistoryReplayPhase::PageClose { index: *index, len: *len, offset: next_offset, retained });
                    } else {
                        this.terminal_page = Some(page);
                        fault = Some(DbError::LimitExceeded("history page offset"));
                    }
                }
            },
            HistoryReplayPhase::PageClose { index, len, offset, retained } => {
                if retained.terminal_is_empty() {
                    next = Some(HistoryReplayPhase::PageStart { index: *index, len: *len, offset: *offset });
                } else if let Err(error) = retained.close_step() {
                    fault = Some(error);
                }
            }
            HistoryReplayPhase::Verify { index } => {
                let result = db_wal_log::WalCursorControl::new(this.cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_millis(8), 1)
                    .and_then(|mut control| this.authenticated.as_mut().ok_or(DbError::Closed)?.verify_step(&this.document, &mut control));
                match result {
                    Ok(true) => next = Some(HistoryReplayPhase::Frame { index: *index }),
                    Ok(false) => {}
                    Err(DbError::Unavailable(message)) if message == "wal cursor deadline reached" => {}
                    Err(error) => fault = Some(error),
                }
            }
            HistoryReplayPhase::Frame { index } => {
                let result =
                    db_wal_log::WalCursorControl::new(this.cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_millis(8), 1).and_then(|mut control| this.authenticated.as_mut().ok_or(DbError::Closed)?.next_step(&mut control));
                match result {
                    Ok(db_wal_log::WalAuthenticatedStep::Yield) => {}
                    Ok(db_wal_log::WalAuthenticatedStep::Committed) => {
                        this.body_index = 0;
                        this.pending_frontier = None;
                        next = Some(HistoryReplayPhase::CommittedBody { index: *index });
                    }
                    Ok(db_wal_log::WalAuthenticatedStep::Done) => {
                        if let Some(source) = this.authenticated.take() {
                            match source.finish() {
                                Ok((pages, gate, tip)) => {
                                    this.pages = pages;
                                    this.gate = Some(gate);
                                    this.previous_tip = db_wal_log::WalPriorChainTip::Verified(tip);
                                    next = Some(HistoryReplayPhase::Retire);
                                }
                                Err(source) => {
                                    this.authenticated = Some(source);
                                    fault = Some(DbError::Corrupt("history source finished without authentication".to_string()));
                                }
                            }
                        } else {
                            fault = Some(DbError::Closed);
                        }
                    }
                    Err(DbError::Unavailable(message)) if message == "wal cursor deadline reached" => {}
                    Err(error) => fault = Some(error),
                }
            }
            HistoryReplayPhase::CommittedBody { index } => {
                if let Some(source) = this.authenticated.as_ref() {
                    if let Some(frame) = source.committed_frame(this.body_index) {
                        this.body_index += 1;
                        let (offset, len) = (frame.payload_start as u64, (frame.payload_end - frame.payload_start) as u64);
                        match frame.kind {
                            _ if this.pending_frontier.is_some() => fault = Some(DbError::Corrupt("history committed frontier is not terminal".to_string())),
                            db_wal_log::WAL_COMMAND => match HistoryEnvelopeCursor::new(source.source(), offset, len) {
                                Ok(cursor) => next = Some(HistoryReplayPhase::Envelope { index: *index, cursor }),
                                Err(error) => fault = Some(error),
                            },
                            db_wal_log::WAL_FRONTIER => match HistoryFrontierCursor::new(source.source(), offset, len) {
                                Ok(cursor) => next = Some(HistoryReplayPhase::Frontier { index: *index, cursor }),
                                Err(error) => fault = Some(error),
                            },
                            _ => {}
                        }
                    } else if let Some((head_seq, commit_seq, chain_hash, epoch)) = this.pending_frontier.take() {
                        next = Some(HistoryReplayPhase::Publish { index: *index, head_seq, commit_seq, chain_hash, epoch });
                    } else if this.reservation.as_ref().is_some_and(|owner| owner.operation_ids.len() != this.pending_start) {
                        fault = Some(DbError::Corrupt("history committed commands have no frontier".to_string()));
                    } else {
                        match this.authenticated.as_mut().ok_or(DbError::Closed).and_then(|source| source.finish_transaction()) {
                            Ok(()) => next = Some(HistoryReplayPhase::Frame { index: *index }),
                            Err(error) => fault = Some(error),
                        }
                    }
                } else {
                    fault = Some(DbError::Closed);
                }
            }
            HistoryReplayPhase::Envelope { index, cursor } => {
                let step = match this.reservation.as_mut().and_then(|owner| owner.scratch.as_mut()) {
                    Some(scratch) => this.authenticated.as_ref().ok_or(DbError::Closed).and_then(|source| cursor.step(source.source(), scratch, &this.document)),
                    None => Err(DbError::Closed),
                };
                match step {
                    Err(error) => fault = Some(error),
                    Ok(Some(range)) => {
                        let len = range.end.saturating_sub(range.start);
                        match this.reservation.as_ref().map_or(Err(DbError::Closed), |owner| owner.preflight_result_range(this.result_len, len)) {
                            Ok(_) => next = Some(HistoryReplayPhase::CopyMutation { index: *index, range, copied: 0, result_start: this.result_len }),
                            Err(error) => fault = Some(error),
                        }
                    }
                    Ok(None) => {}
                }
            }
            HistoryReplayPhase::CopyMutation { index, range, copied, result_start } => {
                match this.authenticated.as_ref().ok_or(DbError::Closed).and_then(|source| Self::copy_mutation_fragment(source.source(), &mut this.reservation, range, *copied, *result_start)) {
                    Err(error) => fault = Some(error),
                    Ok(0) => {
                        if let Some(len) = range.end.checked_sub(range.start).and_then(|value| u16::try_from(value).ok()) {
                            if let Some(reservation) = this.reservation.as_mut() {
                                reservation.operation_ids.push(HistoryTextRange { start: *result_start as u32, len });
                                this.result_len = *result_start + u64::from(len);
                                next = Some(HistoryReplayPhase::CommittedBody { index: *index });
                            } else {
                                fault = Some(DbError::Closed);
                            }
                        } else {
                            fault = Some(DbError::LimitExceeded("history operation id range"));
                        }
                    }
                    Ok(count) => *copied += count,
                }
            }
            HistoryReplayPhase::Frontier { index, cursor } => {
                let step = match this.reservation.as_mut().and_then(|owner| owner.scratch.as_mut()) {
                    Some(scratch) => this.authenticated.as_ref().ok_or(DbError::Closed).and_then(|source| cursor.step(source.source(), scratch, &this.document)),
                    None => Err(DbError::Closed),
                };
                match step {
                    Err(error) => fault = Some(error),
                    Ok(Some(frontier)) => {
                        this.pending_frontier = Some(frontier);
                        next = Some(HistoryReplayPhase::CommittedBody { index: *index });
                    }
                    Ok(None) => {}
                }
            }
            HistoryReplayPhase::Publish { index, head_seq, commit_seq, chain_hash, epoch } => {
                let operation_end = this.reservation.as_ref().map_or(this.pending_start, |owner| owner.operation_ids.len());
                if operation_end == this.pending_start {
                    next = Some(HistoryReplayPhase::Frame { index: *index });
                } else if this.reservation.as_ref().is_none_or(|owner| owner.entries.len() >= HISTORY_REPLAY_MAX_ENTRIES) {
                    fault = Some(DbError::LimitExceeded("history entry item credit"));
                } else {
                    let count = operation_end - this.pending_start;
                    let start = u16::try_from(this.pending_start);
                    let count = u16::try_from(count);
                    match (start, count, this.reservation.as_mut()) {
                        (Ok(operation_start), Ok(operation_count), Some(reservation)) => {
                            reservation.entries.push(ArtifactHistoryEntry { operation_start, operation_count, head_seq: *head_seq, commit_seq: *commit_seq, chain_hash: *chain_hash, epoch: *epoch });
                            this.pending_start = operation_end;
                            next = Some(HistoryReplayPhase::Frame { index: *index });
                        }
                        _ => fault = Some(DbError::LimitExceeded("history entry range")),
                    }
                }
                if fault.is_none() {
                    if let Err(error) = this.authenticated.as_mut().ok_or(DbError::Closed).and_then(|source| source.finish_transaction()) {
                        fault = Some(error);
                    }
                }
            }
            HistoryReplayPhase::Retire => {
                if this.page_count != 0 {
                    this.page_count -= 1;
                    drop(this.pages.pages[this.page_count].take());
                } else {
                    this.pages.len = 0;
                    this.segment_ordinal += 1;
                    next = Some(HistoryReplayPhase::Probe);
                }
            }
            HistoryReplayPhase::InventoryClose => {
                if let Some(segments) = this.segments.as_mut() {
                    segments.close_step();
                    if segments.terminal_is_empty() {
                        this.segments = None;
                    }
                } else {
                    next = Some(HistoryReplayPhase::FinalizeSuccess);
                }
            }
            HistoryReplayPhase::FinalizeSuccess => {
                if this.reservation.as_mut().and_then(|owner| owner.scratch.take()).is_some() {
                } else if !this.pages.pages.is_empty() {
                    drop(std::mem::take(&mut this.pages.pages));
                } else {
                    finalize = true;
                }
            }
        }
        if std::mem::take(&mut this.panic_before_transition_commit) {
            panic!("history replay transition panic fixture");
        }
        if let Some(error) = fault {
            if this.terminal_error.is_none() {
                this.terminal_error = Some(error);
            }
            this.transition = HistoryReplayTransition::FaultRetire;
        } else if finalize {
            match this.finish_view() {
                Ok(view) => {
                    this.phase = None;
                    this.transition = HistoryReplayTransition::Complete;
                    return std::task::Poll::Ready(Ok(view));
                }
                Err(error) => {
                    this.terminal_error.get_or_insert(error);
                    this.transition = HistoryReplayTransition::FaultRetire;
                }
            }
        } else if let Some(next) = next {
            this.phase = Some(next);
        }
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

impl Future for HistoryReplayFuture {
    type Output = Result<ArtifactHistoryView, DbError>;

    fn poll(self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        self.next(context)
    }
}

impl Drop for HistoryReplayFuture {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "history replay reached Drop before retained source/result/page retirement");
    }
}
//#endregion 🔖️HistoryReplay
//#endregion 🔖️Ids

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
