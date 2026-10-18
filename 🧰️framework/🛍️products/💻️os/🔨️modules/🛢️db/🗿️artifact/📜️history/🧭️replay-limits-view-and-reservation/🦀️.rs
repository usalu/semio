//! 🧭️ Replay limits, view and reservation: the replay limits, the history view and the replay reservation with its close cursor. See [`super`].

use super::*;

//#region 🔖️Ids
//#region 🔖️HistoryReplay
pub(super) const HISTORY_REPLAY_PAGE_BYTES: u64 = 16 * 1024;

pub(super) const HISTORY_REPLAY_SEGMENT_PAGES: u64 = 1_024;

pub(super) const HISTORY_REPLAY_RESULT_BYTES: u64 = 15 * 1024 * 1024;

pub(super) const HISTORY_REPLAY_OPERATION_BYTES: u64 = 32 * 1024 * 1024;

pub(super) const HISTORY_REPLAY_MAX_FRAME_BYTES: u64 = 1024 * 1024;

pub(super) const HISTORY_REPLAY_MAX_FIELD_BYTES: usize = HISTORY_REPLAY_PAGE_BYTES as usize;

pub(super) const HISTORY_REPLAY_MAX_ENTRIES: usize = 4_096;

pub(super) const HISTORY_REPLAY_MAX_OPERATION_IDS: usize = 8_192;

pub(super) const HISTORY_REPLAY_RESULT_PAGES: usize = (HISTORY_REPLAY_RESULT_BYTES / HISTORY_REPLAY_PAGE_BYTES) as usize;

pub(super) const HISTORY_REPLAY_CONSTRUCTION_SLOTS: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct HistoryTextRange {
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
    pub(super) result_pages: Vec<Option<Vec<u8>>>,
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
    pub(super) source_pages: Vec<Option<Vec<u8>>>,
    pub(super) source_page_count: usize,
    pub(super) result_pages: Vec<Option<Vec<u8>>>,
    pub operation_ids: Vec<HistoryTextRange>,
    pub entries: Vec<ArtifactHistoryEntry>,
    pub(super) scratch: Option<Vec<u8>>,
    pub(super) retained_operation_bytes: u64,
    pub(super) retained_result_bytes: u64,
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
    pub(super) fn try_new_with_result_page_failure(failure_after: usize) -> Result<Self, HistoryReplayReservationConstructionFault> {
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

    pub(super) fn retained_bytes(&self) -> Option<u64> {
        Some(self.retained_operation_bytes)
    }

    pub(super) fn preflight_result_range(&self, result_len: u64, len: u64) -> Result<u64, DbError> {
        if self.operation_ids.len() >= HISTORY_REPLAY_MAX_OPERATION_IDS {
            return Err(DbError::LimitExceeded("history operation item credit"));
        }
        match result_len.checked_add(len) {
            Some(end) if end <= HISTORY_REPLAY_RESULT_BYTES => Ok(end),
            _ => Err(DbError::LimitExceeded("history result byte credit")),
        }
    }

    #[cfg(test)]
    pub(super) fn retain_source_page(&mut self, page: Vec<u8>) -> Result<(), Vec<u8>> {
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
    pub(super) source_pages: Option<Vec<Option<Vec<u8>>>>,
    pub(super) source_page_count: usize,
    pub(super) result_pages: Option<Vec<Option<Vec<u8>>>>,
    pub operation_ids: Option<Vec<HistoryTextRange>>,
    pub entries: Option<Vec<ArtifactHistoryEntry>>,
    pub(super) scratch: Option<Vec<u8>>,
    pub(super) retained_operation_bytes: u64,
    pub(super) retained_result_bytes: u64,
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
//#endregion 🔖️HistoryReplay
//#endregion 🔖️Ids
