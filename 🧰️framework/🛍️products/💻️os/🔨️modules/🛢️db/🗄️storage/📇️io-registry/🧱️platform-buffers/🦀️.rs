//! 🧱️ Platform buffers: the platform buffer arena, its prepare, copy and close futures and the backend kind. See [`super`].

use super::*;

//#region 🔖️Limits
pub const DB_IO_PLATFORM_BUFFERS: usize = 16;

const DB_IO_PLATFORM_RETIREMENT_SLOTS: usize = DB_IO_PLATFORM_BUFFERS * 2;

const DB_IO_PLATFORM_BUFFER_BYTES: usize = DB_IO_MAX_READ_BYTES as usize;

struct DbIoPlatformBacking(std::cell::UnsafeCell<[u8; DB_IO_PLATFORM_BUFFER_BYTES]>);

impl DbIoPlatformBacking {
    pub const fn new() -> Self {
        Self(std::cell::UnsafeCell::new([0; DB_IO_PLATFORM_BUFFER_BYTES]))
    }
}

unsafe impl Sync for DbIoPlatformBacking {}

static DB_IO_PLATFORM_BACKINGS: [DbIoPlatformBacking; DB_IO_PLATFORM_BUFFERS] = [const { DbIoPlatformBacking::new() }; DB_IO_PLATFORM_BUFFERS];

#[derive(Clone, Copy)]
struct DbIoPlatformSlot {
    pub generation: u64,
    pub occupied: bool,
}

struct DbIoPlatformArena {
    pub slots: [DbIoPlatformSlot; DB_IO_PLATFORM_BUFFERS],
    pub retired: [Option<(u8, u64)>; DB_IO_PLATFORM_RETIREMENT_SLOTS],
    pub retired_read: usize,
    pub retired_len: usize,
    pub next_generation: u64,
}

static DB_IO_PLATFORM_ARENA: std::sync::Mutex<DbIoPlatformArena> =
    std::sync::Mutex::new(DbIoPlatformArena { slots: [DbIoPlatformSlot { generation: 0, occupied: false }; DB_IO_PLATFORM_BUFFERS], retired: [None; DB_IO_PLATFORM_RETIREMENT_SLOTS], retired_read: 0, retired_len: 0, next_generation: 1 });

/// 🧩 Explicit contiguous platform-call buffer backed by a fixed process slot.
pub struct DbIoPlatformBuffer {
    pub slot: u8,
    pub generation: u64,
    pub len: usize,
    pub copied: usize,
    pub credit: DbIoPageWriter,
    pub returned: bool,
}

pub struct DbIoPlatformCopy<'a> {
    pub source: &'a DbIoPages,
    pub cursor: u8,
    pub owner: Option<DbIoPlatformBuffer>,
}

/// 🚪 One retained prepared-platform close opportunity per poll.
pub struct DbIoPlatformClose {
    pub owner: Option<DbIoPlatformBuffer>,
}

/// 🪡 One retained prepared-platform slice fragment per poll.
pub(crate) struct DbIoPlatformSlicesCopy<'a> {
    pub first: &'a [u8],
    pub second: &'a [u8],
    pub source: u8,
    pub cursor: usize,
    pub owner: Option<DbIoPlatformBuffer>,
    pub fault: Option<DbError>,
}

pub fn db_io_prepare_platform(source: &DbIoPages) -> Result<DbIoPlatformCopy<'_>, DbError> {
    let _ = db_io_platform_maintenance_step()?;
    if source.len() > DB_IO_PLATFORM_BUFFER_BYTES {
        return Err(DbError::LimitExceeded("db_io prepared platform buffer"));
    }
    let (slot, generation) = {
        let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = arena.slots.iter().position(|slot| !slot.occupied).ok_or_else(|| DbError::Unavailable("DB I/O prepared platform capacity exhausted".to_string()))?;
        let generation = arena.next_generation;
        arena.next_generation = arena.next_generation.checked_add(1).filter(|next| *next != 0).ok_or(DbError::LimitExceeded("db_io platform generation"))?;
        arena.slots[slot] = DbIoPlatformSlot { generation, occupied: true };
        (slot as u8, generation)
    };
    let credit = match DbIoPageWriter::try_reserve_for_operation(source.operation(), source.page_count() as usize) {
        Ok(credit) => credit,
        Err(error) => {
            let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            arena.slots[slot as usize] = DbIoPlatformSlot { generation: 0, occupied: false };
            return Err(error.into_error());
        }
    };
    let owner = DbIoPlatformBuffer { slot, generation, len: source.len(), copied: 0, credit, returned: false };
    Ok(DbIoPlatformCopy { source, cursor: 0, owner: Some(owner) })
}

impl DbIoPlatformBuffer {
    pub fn as_slice(&self) -> &[u8] {
        unsafe { &(&*DB_IO_PLATFORM_BACKINGS[self.slot as usize].0.get())[..self.len] }
    }

    pub fn as_static_driver_slice(&self) -> &'static [u8] {
        unsafe { &(&*DB_IO_PLATFORM_BACKINGS[self.slot as usize].0.get())[..self.len] }
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if self.credit.close_step()?.is_some() {
            return Ok(true);
        }
        if self.returned {
            return Ok(false);
        }
        let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = &mut arena.slots[self.slot as usize];
        if !slot.occupied || slot.generation != self.generation {
            return Err(DbError::Internal("DB I/O prepared platform slot lost ABA authority".to_string()));
        }
        *slot = DbIoPlatformSlot { generation: 0, occupied: false };
        self.returned = true;
        Ok(true)
    }
}

pub fn db_io_close_platform(owner: DbIoPlatformBuffer) -> DbIoPlatformClose {
    DbIoPlatformClose { owner: Some(owner) }
}

impl Future for DbIoPlatformClose {
    type Output = Result<(), DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        let Some(platform) = owner.owner.as_mut() else { return std::task::Poll::Ready(Ok(())) };
        if platform.close_step()? {
            context.waker().wake_by_ref();
            return std::task::Poll::Pending;
        }
        owner.owner.take();
        std::task::Poll::Ready(Ok(()))
    }
}

fn db_io_reserve_platform_buffer(operation: u64, total: usize) -> Result<DbIoPlatformBuffer, DbError> {
    if total > DB_IO_PLATFORM_BUFFER_BYTES {
        return Err(DbError::LimitExceeded("db_io prepared platform slices"));
    }
    let (slot, generation) = {
        let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = arena.slots.iter().position(|slot| !slot.occupied).ok_or_else(|| DbError::Unavailable("DB I/O prepared platform capacity exhausted".to_string()))?;
        let generation = arena.next_generation;
        arena.next_generation = arena.next_generation.checked_add(1).filter(|next| *next != 0).ok_or(DbError::LimitExceeded("db_io platform generation"))?;
        arena.slots[slot] = DbIoPlatformSlot { generation, occupied: true };
        (slot as u8, generation)
    };
    let credit = match DbIoPageWriter::try_reserve_for_operation(operation, total.div_ceil(DB_IO_PAGE_BYTES)) {
        Ok(credit) => credit,
        Err(error) => {
            let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            arena.slots[slot as usize] = DbIoPlatformSlot { generation: 0, occupied: false };
            return Err(error.into_error());
        }
    };
    Ok(DbIoPlatformBuffer { slot, generation, len: total, copied: 0, credit, returned: false })
}

pub fn db_io_prepare_platform_slices<'a>(operation: u64, first: &'a [u8], second: &'a [u8]) -> DbIoPlatformSlicesCopy<'a> {
    let reserved = first.len().checked_add(second.len()).ok_or(DbError::LimitExceeded("db_io prepared platform slices")).and_then(|total| db_io_reserve_platform_buffer(operation, total));
    match reserved {
        Ok(owner) => DbIoPlatformSlicesCopy { first, second, source: 0, cursor: 0, owner: Some(owner), fault: None },
        Err(error) => DbIoPlatformSlicesCopy { first, second, source: 0, cursor: 0, owner: None, fault: Some(error) },
    }
}

impl Future for DbIoPlatformSlicesCopy<'_> {
    type Output = Result<DbIoPlatformBuffer, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        if let Some(error) = owner.fault.take() {
            return std::task::Poll::Ready(Err(error));
        }
        if owner.source > 1 {
            return std::task::Poll::Ready(owner.owner.take().ok_or_else(|| DbError::Internal("DB I/O prepared-platform slice cursor lost its exact owner".to_string())));
        }
        let source = if owner.source == 0 { owner.first } else { owner.second };
        if owner.cursor == source.len() {
            owner.source += 1;
            owner.cursor = 0;
        } else {
            let written = (source.len() - owner.cursor).min(DB_IO_PAGE_BYTES);
            let platform = owner.owner.as_mut().ok_or_else(|| DbError::Internal("DB I/O prepared-platform slice cursor lost its owner during copy".to_string()))?;
            let end = platform.copied.checked_add(written).ok_or(DbError::LimitExceeded("db_io prepared platform slices cursor"))?;
            unsafe { (&mut *DB_IO_PLATFORM_BACKINGS[platform.slot as usize].0.get())[platform.copied..end].copy_from_slice(&source[owner.cursor..owner.cursor + written]) };
            platform.copied = end;
            owner.cursor += written;
        }
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

impl Drop for DbIoPlatformBuffer {
    fn drop(&mut self) {
        if !self.returned {
            let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let slot = arena.slots[self.slot as usize];
            if slot.occupied && slot.generation == self.generation {
                if arena.retired_len >= DB_IO_PLATFORM_BUFFERS {
                    DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
                }
                let write = (arena.retired_read + arena.retired_len) % DB_IO_PLATFORM_RETIREMENT_SLOTS;
                arena.retired[write] = Some((self.slot, self.generation));
                arena.retired_len += 1;
                self.returned = true;
            }
        }
    }
}

pub fn db_io_platform_maintenance_step() -> Result<bool, DbError> {
    let mut arena = DB_IO_PLATFORM_ARENA.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if arena.retired_len == 0 {
        return Ok(false);
    }
    let read = arena.retired_read;
    let Some((slot, generation)) = arena.retired[read].take() else {
        DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
        return Err(DbError::Internal("DB I/O platform retired length lost exact owner".to_string()));
    };
    let owner = arena.slots[slot as usize];
    if !owner.occupied || owner.generation != generation {
        return Err(DbError::Internal("DB I/O platform retirement lost ABA authority".to_string()));
    }
    arena.slots[slot as usize] = DbIoPlatformSlot { generation: 0, occupied: false };
    arena.retired_read = (read + 1) % DB_IO_PLATFORM_RETIREMENT_SLOTS;
    arena.retired_len -= 1;
    Ok(true)
}

impl Future for DbIoPlatformCopy<'_> {
    type Output = Result<DbIoPlatformBuffer, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        let Some(fragment) = owner.source.page(owner.cursor) else {
            return std::task::Poll::Ready(owner.owner.take().ok_or_else(|| DbError::Internal("DB I/O platform cursor lost its retained buffer".to_string())));
        };
        let Some(platform) = owner.owner.as_mut() else {
            return std::task::Poll::Ready(Err(DbError::Internal("DB I/O platform cursor lost its retained buffer".to_string())));
        };
        let end = platform.copied.checked_add(fragment.len()).ok_or(DbError::LimitExceeded("db_io prepared platform cursor"))?;
        unsafe { (&mut *DB_IO_PLATFORM_BACKINGS[platform.slot as usize].0.get())[platform.copied..end].copy_from_slice(fragment) };
        platform.copied = end;
        owner.cursor += 1;
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

pub fn db_io_page_maintenance_step() -> Result<Option<usize>, DbError> {
    let mut state = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let retired_read = state.retired_read;
    let Some((slot, generation)) = state.retired[retired_read].take() else { return Ok(None) };
    state.retired_read = (state.retired_read + 1) % DB_IO_PAGE_RETIREMENT_SLOTS;
    state.retired_len -= 1;
    let owner = state.slots[slot as usize];
    if owner.generation != generation || owner.phase != DbIoPagePhase::Closing {
        return Err(DbError::Internal("db I/O page retirement lost ABA authority".to_string()));
    }
    state.slots[slot as usize] = EMPTY_DB_IO_PAGE_SLOT;
    let write = (state.free_read + state.free_len) % DB_IO_TOTAL_PAGES;
    state.free[write] = slot;
    state.free_len += 1;
    drop(state);
    db_io_operation_return(owner.operation, db_io_page_credit(1))?;
    Ok(Some(DB_IO_PAGE_BYTES))
}

/// 🔤 Converts one pre-admitted external-driver string into fixed repository text.
pub async fn db_io_copy_observed_text(mut reservation: DbIoDriverReservation, source: String) -> Result<DbIoText, DbError> {
    let mut source = DbIoExternalBytes::new(source.into_bytes());
    reservation.observe_capacity(source.capacity()?)?;
    let value = std::str::from_utf8(source.as_slice()?).map_err(|_| DbError::Corrupt("DB I/O observed text is not UTF-8".to_string()))?;
    let text = DbIoText::try_from_str(value)?;
    while !source.terminal_is_empty() {
        let _ = source.close_step();
        semio_framework_async::yield_once().await;
    }
    reservation.close_step()?;
    Ok(text)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoBackendKind {
    Memory,
    Filesystem,
    Sqlite,
    Postgres,
    Neo4j,
}
//#endregion 🔖️Limits
