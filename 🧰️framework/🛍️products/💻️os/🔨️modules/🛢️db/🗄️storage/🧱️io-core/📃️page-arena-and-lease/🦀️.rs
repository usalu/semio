//! 📃️ Page arena and lease: the page phase, the page backing and slots, the page arena, the page lease and the page checkout. See [`super`].

use super::*;

//#region 🔖️Limits
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoPagePhase {
    Free,
    CheckedOutInput,
    CheckedOutWriter,
    Queued,
    Executing,
    TerminalResult,
    Closing,
}

pub struct DbIoPageBacking(std::cell::UnsafeCell<[u8; DB_IO_PAGE_BYTES]>);

impl DbIoPageBacking {
    pub const fn new() -> Self {
        Self(std::cell::UnsafeCell::new([0; DB_IO_PAGE_BYTES]))
    }
}

unsafe impl Sync for DbIoPageBacking {}

static DB_IO_PAGE_BACKINGS: [DbIoPageBacking; DB_IO_TOTAL_PAGES] = [const { DbIoPageBacking::new() }; DB_IO_TOTAL_PAGES];

#[derive(Clone, Copy)]
pub struct DbIoPageSlot {
    pub generation: u64,
    pub operation: u64,
    pub phase: DbIoPagePhase,
}

pub const EMPTY_DB_IO_PAGE_SLOT: DbIoPageSlot = DbIoPageSlot { generation: 0, operation: 0, phase: DbIoPagePhase::Free };

pub struct DbIoPageArenaState {
    pub slots: [DbIoPageSlot; DB_IO_TOTAL_PAGES],
    pub free: [u16; DB_IO_TOTAL_PAGES],
    pub free_read: usize,
    pub free_len: usize,
    pub retired: [Option<(u16, u64)>; DB_IO_PAGE_RETIREMENT_SLOTS],
    pub retired_read: usize,
    pub retired_len: usize,
    pub next_generation: u64,
}

impl DbIoPageArenaState {
    pub fn new() -> Self {
        Self {
            slots: [EMPTY_DB_IO_PAGE_SLOT; DB_IO_TOTAL_PAGES],
            free: std::array::from_fn(|index| index as u16),
            free_read: 0,
            free_len: DB_IO_TOTAL_PAGES,
            retired: [None; DB_IO_PAGE_RETIREMENT_SLOTS],
            retired_read: 0,
            retired_len: 0,
            next_generation: 1,
        }
    }
}

static DB_IO_PAGE_ARENA: std::sync::OnceLock<std::sync::Mutex<DbIoPageArenaState>> = std::sync::OnceLock::new();

pub fn db_io_page_arena() -> &'static std::sync::Mutex<DbIoPageArenaState> {
    DB_IO_PAGE_ARENA.get_or_init(|| std::sync::Mutex::new(DbIoPageArenaState::new()))
}

pub fn db_io_page_credit(count: usize) -> DbIoCredit {
    DbIoCredit { pages: count, bytes: (count * DB_IO_PAGE_BYTES) as u64, items: count, controls: 0 }
}

pub(super) fn db_io_page_shell_credit() -> DbIoCredit {
    DbIoCredit { pages: 0, bytes: 0, items: 1, controls: 1 }
}

#[derive(Debug)]
pub(super) struct DbIoPageLease {
    pub slot: u16,
    pub generation: u64,
    pub operation: u64,
    pub(super) used: u16,
    pub returned: bool,
}

impl DbIoPageLease {
    pub(super) fn validate_identity(&self, slot: DbIoPageSlot) -> Result<(), DbError> {
        if slot.generation != self.generation {
            return Err(DbError::StaleGeneration { expected: GenerationId(self.generation), actual: GenerationId(slot.generation) });
        }
        if slot.operation != self.operation {
            return Err(DbError::Internal(format!("DB I/O page operation mismatch: expected {}, got {} at generation {}", self.operation, slot.operation, self.generation)));
        }
        Ok(())
    }

    pub(super) fn validate_phase(&self, slot: DbIoPageSlot, expected: DbIoPagePhase) -> Result<(), DbError> {
        self.validate_identity(slot)?;
        if slot.phase != expected {
            return Err(DbError::Internal(format!("DB I/O page phase mismatch: expected {expected:?}, got {:?} for operation {} generation {}", slot.phase, self.operation, self.generation)));
        }
        Ok(())
    }

    pub fn phase(&self) -> Result<DbIoPagePhase, DbError> {
        let state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = state.slots[self.slot as usize];
        self.validate_identity(slot)?;
        Ok(slot.phase)
    }

    pub fn transition(&self, expected: DbIoPagePhase, next: DbIoPagePhase) -> Result<(), DbError> {
        let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = &mut state.slots[self.slot as usize];
        self.validate_phase(*slot, expected)?;
        slot.phase = next;
        Ok(())
    }

    pub fn bytes(&self) -> &[u8] {
        let used = usize::from(self.used);
        unsafe { &(&*DB_IO_PAGE_BACKINGS[self.slot as usize].0.get())[..used] }
    }

    pub fn write(&mut self, offset: usize, source: &[u8]) -> Result<(), DbError> {
        let end = offset.checked_add(source.len()).ok_or(DbError::LimitExceeded("db_io page write"))?;
        if end > DB_IO_PAGE_BYTES || offset > usize::from(self.used) {
            return Err(DbError::LimitExceeded("db_io page writer reservation"));
        }
        let state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = state.slots[self.slot as usize];
        self.validate_identity(slot)?;
        if !matches!(slot.phase, DbIoPagePhase::CheckedOutWriter | DbIoPagePhase::Executing) {
            return Err(DbError::Internal(format!("DB I/O page phase mismatch: expected CheckedOutWriter or Executing, got {:?} for operation {} generation {}", slot.phase, self.operation, self.generation)));
        }
        unsafe { (&mut *DB_IO_PAGE_BACKINGS[self.slot as usize].0.get())[offset..end].copy_from_slice(source) };
        self.used = self.used.max(end as u16);
        Ok(())
    }

    pub(super) fn return_to_arena(mut self) -> Result<usize, DbError> {
        let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = &mut state.slots[self.slot as usize];
        self.validate_phase(*slot, DbIoPagePhase::Closing)?;
        *slot = EMPTY_DB_IO_PAGE_SLOT;
        let write = (state.free_read + state.free_len) % DB_IO_TOTAL_PAGES;
        state.free[write] = self.slot;
        state.free_len += 1;
        self.returned = true;
        drop(state);
        db_io_operation_return(self.operation, db_io_page_credit(1))?;
        Ok(DB_IO_PAGE_BYTES)
    }

    fn install_lost_handle(&mut self) {
        let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = &mut state.slots[self.slot as usize];
        if slot.generation != self.generation || slot.operation != self.operation || slot.phase == DbIoPagePhase::Free {
            return;
        }
        slot.phase = DbIoPagePhase::Closing;
        if state.retired_len >= DB_IO_TOTAL_PAGES {
            DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
        }
        let write = (state.retired_read + state.retired_len) % DB_IO_PAGE_RETIREMENT_SLOTS;
        state.retired[write] = Some((self.slot, self.generation));
        state.retired_len += 1;
        self.returned = true;
    }
}

impl Drop for DbIoPageLease {
    fn drop(&mut self) {
        if !self.returned {
            self.install_lost_handle();
        }
    }
}

pub(super) fn db_io_checkout_pages(operation: u64, count: usize, phase: DbIoPagePhase) -> Result<[Option<DbIoPageLease>; DB_IO_OPERATION_PAGES], DbError> {
    if operation == 0 || count > DB_IO_OPERATION_PAGES {
        return Err(DbError::LimitExceeded("db_io operation page credit"));
    }
    let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    db_io_preflight_page_checkout(&state, count)?;
    let mut leases: [Option<DbIoPageLease>; DB_IO_OPERATION_PAGES] = std::array::from_fn(|_| None);
    for lease in leases.iter_mut().take(count) {
        let slot = state.free[state.free_read];
        state.free_read = (state.free_read + 1) % DB_IO_TOTAL_PAGES;
        state.free_len -= 1;
        let generation = state.next_generation;
        state.next_generation += 1;
        state.slots[slot as usize] = DbIoPageSlot { generation, operation, phase };
        *lease = Some(DbIoPageLease { slot, generation, operation, used: 0, returned: false });
    }
    Ok(leases)
}

pub fn db_io_preflight_page_checkout(state: &DbIoPageArenaState, count: usize) -> Result<(), DbError> {
    if count > state.free_len || state.next_generation.checked_add(count as u64).is_none() {
        return Err(DbError::Unavailable(DB_IO_PROCESS_PAGES_EXHAUSTED.to_string()));
    }
    Ok(())
}
//#endregion 🔖️Limits
