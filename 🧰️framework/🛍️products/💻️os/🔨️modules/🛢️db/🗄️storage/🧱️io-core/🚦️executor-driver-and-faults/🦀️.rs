//! 🚦️ Executor, driver and faults: the execution step and mode, the async driver types and runtime, the writer release step, the task executor trait, the backend parts and the fault kinds, causes and terminal. See [`super`].

use super::*;

//#region 🔖️Limits
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoExecutionStep {
    Yield,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoExecutorMode {
    BlockingLane,
    AsyncNative,
}

/// 🎁️ What an async-native driver hands back when its task reaches a terminal: the executor the
/// backend registry lends out for the turn, the typed task owner, and the terminal itself.
pub type DbIoAsyncDriverOutput = (Box<dyn DbIoTaskExecutor>, DbIoTask, Result<DbIoResult, DbError>);

pub type DbIoAsyncDriverFuture = Pin<Box<dyn Future<Output = DbIoAsyncDriverOutput> + Send + 'static>>;

/// 🧵️ The one seam where a driver bound to a foreign executor may be polled.
///
/// `Lane::Io` of the process [`WorkerPool`] is plain `std::thread`s with no foreign runtime context
/// of any kind — that is the whole point of `semio_framework_async` ("No `tokio` in this crate").
/// A driver like `sqlx` or `neo4rs` demands ITS runtime's thread-local context at **poll** time, not
/// merely at construction time, so polling such a future on a pool worker aborts the process
/// (`this functionality requires a Tokio context`). A backend whose driver has that requirement
/// therefore owns a bounded runtime of its own and returns it here; [`DbIoAsyncTaskLease::start_on_lane_io`]
/// then never puts the driver's future in the slot at all. What the slot gets instead is the
/// **bridge** this returns: a repo-owned rendezvous (`semio_framework_async_sync::oneshot`) that is safe
/// to poll anywhere, because the only thing it does is read a value and register a waker.
///
/// This is an interface, not an implementation: nothing in `db_storage` names a concrete runtime.
pub trait DbIoAsyncDriverRuntime: Send + Sync {
    /// 🚚 Take ownership of `future`, drive it to completion on a thread this runtime owns, and
    /// return the `Lane::Io`-safe bridge future that resolves to the same output.
    fn detach(&self, future: DbIoAsyncDriverFuture) -> DbIoAsyncDriverFuture;
}

/// 👣️ One writer-guard release opportunity either progressed or waits for an exact future wake.
pub enum DbIoWriterReleaseStep {
    More,
    Faulted,
    Idle,
}

/// 🔌 Platform drivers implement one typed, resumable task step behind repository owners.
pub trait DbIoTaskExecutor: Send + Sync {
    fn supports_writer_authority(&self) -> bool {
        false
    }

    fn bind_writer_control(&mut self, _control: DbIoBackendControl) -> Result<(), DbError> {
        Ok(())
    }

    fn writer_release_step(&self, _context: &mut std::task::Context<'_>) -> Result<DbIoWriterReleaseStep, DbError> {
        Ok(DbIoWriterReleaseStep::Idle)
    }

    fn pin_writer_operation(&self, _operation: u64, task: &DbIoTask) -> Result<(), DbError> {
        if task.writer_stamp().is_some() {
            Err(DbError::Unavailable("backend has no WAL writer fence".to_string()))
        } else {
            Ok(())
        }
    }

    fn finish_writer_operation(&self, _operation: u64, _task: &DbIoTask) -> Result<(), DbError> {
        Ok(())
    }

    fn mode(&self) -> DbIoExecutorMode {
        DbIoExecutorMode::BlockingLane
    }

    /// 🧵️ The runtime this executor's driver must be polled on, when its driver is bound to one.
    /// `None` — the default, and the answer for every backend in this crate — means the future
    /// [`Self::drive_async`] returns is safe to poll on `Lane::Io` directly.
    fn driver_runtime(&self) -> Option<&'static dyn DbIoAsyncDriverRuntime> {
        None
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    fn bind_owner_operation(&mut self, _operation: u64) -> Result<(), DbError> {
        Ok(())
    }

    fn owner_backing_bytes(&self) -> u64 {
        size_of_val(self) as u64
    }

    fn execute_step(&self, operation: u64, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError>;

    fn drive_async(self: Box<Self>, operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture;

    fn close_operation_step(&self, operation: u64, task: &DbIoTask) -> Result<bool, DbError>;

    fn close_backend_step(&mut self, context: &mut std::task::Context<'_>) -> Result<bool, DbError>;

    fn backend_terminal_is_empty(&self) -> bool;
}

pub fn db_io_backend_parts(control: DbIoBackendControl) -> (u16, u64) {
    match control {
        DbIoBackendControl::Memory { slot, generation }
        | DbIoBackendControl::Filesystem { slot, generation }
        | DbIoBackendControl::Sqlite { slot, generation }
        | DbIoBackendControl::Postgres { slot, generation }
        | DbIoBackendControl::Neo4j { slot, generation } => (slot, generation),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoFaultKind {
    Backend,
    Cancelled,
    Panic,
    Saturated,
    Stale,
}

/// 🧩 Bounded storage-error taxonomy retained independently from runner provenance.
///
/// DbError::Rejected is an artifact-engine outcome above this storage executor boundary; a
/// backend returning it violates the layer contract and is retained as Internal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbIoFaultCause {
        Pack(pack::PackCauseKind),
    Protocol,
Io,
    NotFound,
    AlreadyExists,
    InvalidArgument,
    LimitExceeded(&'static str),
    Conflict,
    Fenced { expected: u64, actual: u64 },
    StaleGeneration { expected: u64, actual: u64 },
    Unavailable,
    Timeout,
    Corrupt,
    Closed,
    Unauthorized,
    Unimplemented(&'static str),
    Internal,
}

impl DbIoFaultCause {
    pub fn from_db_error(error: &DbError) -> Self {
        match error {
                        DbError::Pack(error) => Self::Pack(error.cause_kind()),
            DbError::Protocol(_) => Self::Protocol,
DbError::Io(_) => Self::Io,
            DbError::NotFound(_) => Self::NotFound,
            DbError::AlreadyExists(_) => Self::AlreadyExists,
            DbError::InvalidArgument(_) => Self::InvalidArgument,
            DbError::LimitExceeded(detail) => Self::LimitExceeded(detail),
            DbError::Conflict(_) => Self::Conflict,
            DbError::Fenced { expected, actual } => Self::Fenced { expected: *expected, actual: *actual },
            DbError::StaleGeneration { expected, actual } => Self::StaleGeneration { expected: expected.0, actual: actual.0 },
            DbError::Unavailable(_) => Self::Unavailable,
            DbError::Timeout(_) => Self::Timeout,
            DbError::Corrupt(_) => Self::Corrupt,
            DbError::Closed => Self::Closed,
            DbError::Unauthorized(_) => Self::Unauthorized,
            DbError::Unimplemented(detail) => Self::Unimplemented(detail),
            DbError::Internal(_) => Self::Internal,
            DbError::Rejected { .. } => Self::Internal,
        }
    }
}

pub const DB_IO_FAULT_CUSTODY_SLOTS: usize = DB_IO_TOTAL_PAGES + DB_IO_PROCESS_CONTROL_CREDIT + 4 * DB_IO_OPERATION_ITEMS + DB_IO_BACKEND_CONTROLS;

pub struct DbIoFaultCustodyCell { pub generation: u64, pub reserved: bool, pub owner: Option<DbIoFault> }
pub struct DbIoFaultCustodyArena { pub next_generation: u64, pub cells: [DbIoFaultCustodyCell; DB_IO_FAULT_CUSTODY_SLOTS] }
pub static DB_IO_FAULT_CUSTODY: std::sync::Mutex<DbIoFaultCustodyArena> = std::sync::Mutex::new(DbIoFaultCustodyArena { next_generation: 1, cells: [const { DbIoFaultCustodyCell { generation: 0, reserved: false, owner: None } }; DB_IO_FAULT_CUSTODY_SLOTS] });
pub const DB_IO_FAULT_CUSTODY_STATIC_BACKING_BYTES: u64 = size_of::<std::sync::Mutex<DbIoFaultCustodyArena>>() as u64;

#[derive(Debug)]
pub struct DbIoFaultCustody { pub slot: usize, pub generation: u64 }

pub fn db_io_fault_custody_reserve() -> Result<DbIoFaultCustody, DbError> {
    let mut arena = DB_IO_FAULT_CUSTODY.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let slot = arena.cells.iter().position(|cell| !cell.reserved).filter(|_| arena.next_generation != u64::MAX).ok_or(DbError::LimitExceeded("db_io fault custody"))?;
    let generation = arena.next_generation;
    arena.next_generation += 1;
    arena.cells[slot].generation = generation;
    arena.cells[slot].reserved = true;
    Ok(DbIoFaultCustody { slot, generation })
}

impl Drop for DbIoFaultCustody {
    fn drop(&mut self) {
        db_io_fault_custody_release(self.slot, self.generation).expect("original fault custody reservation");
        db_io_admission_released();
    }
}

fn db_io_fault_custody_release(slot: usize, generation: u64) -> Result<(), DbError> {
    let mut arena = DB_IO_FAULT_CUSTODY.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let cell = &mut arena.cells[slot];
    if !cell.reserved || cell.generation != generation { return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(if cell.reserved { cell.generation } else { 0 }) }); }
    if cell.owner.is_some() { return Err(DbError::LimitExceeded("db_io fault custody remains occupied")); }
    cell.reserved = false;
    Ok(())
}

pub fn db_io_fault_custody_take(arena: &mut DbIoFaultCustodyArena, slot: usize, generation: u64) -> Result<Option<DbIoFault>, DbError> {
    let cell = &mut arena.cells[slot];
    if !cell.reserved || cell.generation != generation { return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(if cell.reserved { cell.generation } else { 0 }) }); }
    Ok(cell.owner.take())
}

pub struct DbIoFault {
    pub kind: DbIoFaultKind,
    pub cause: DbIoFaultCause,
    pub detail: DbIoText,
        pub structured: Option<DbError>,
pub result_handback: Option<DbIoResultHandback>,
    pub custody: Option<DbIoFaultCustody>,
}

impl std::fmt::Debug for DbIoFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DbIoFault").field("kind", &self.kind).field("cause", &self.cause).field("detail", &self.detail.as_str()).field("has_result_handback", &self.result_handback.is_some()).finish()
    }
}

impl DbIoFault {
    pub fn into_db_error(mut self) -> DbError {
                if let Some(error) = self.structured.take() {
            while self.close_step() {}
            return error;
        }
let detail = self.detail.as_str().to_string();
        let error = match self.cause {
                        DbIoFaultCause::Pack(_) | DbIoFaultCause::Protocol => panic!("typed DB I/O fault lost its original error owner"),
DbIoFaultCause::Io => DbError::Io(detail),
            DbIoFaultCause::NotFound => DbError::NotFound(detail),
            DbIoFaultCause::AlreadyExists => DbError::AlreadyExists(detail),
            DbIoFaultCause::InvalidArgument => DbError::InvalidArgument(detail),
            DbIoFaultCause::LimitExceeded(detail) => DbError::LimitExceeded(detail),
            DbIoFaultCause::Conflict => DbError::Conflict(detail),
            DbIoFaultCause::Fenced { expected, actual } => DbError::Fenced { expected, actual },
            DbIoFaultCause::StaleGeneration { expected, actual } => DbError::StaleGeneration { expected: GenerationId(expected), actual: GenerationId(actual) },
            DbIoFaultCause::Unavailable => DbError::Unavailable(detail),
            DbIoFaultCause::Timeout => DbError::Timeout(detail),
            DbIoFaultCause::Corrupt => DbError::Corrupt(detail),
            DbIoFaultCause::Closed => DbError::Closed,
            DbIoFaultCause::Unauthorized => DbError::Unauthorized(detail),
            DbIoFaultCause::Unimplemented(detail) => DbError::Unimplemented(detail),
            DbIoFaultCause::Internal => DbError::Internal(detail),
        };
        while self.close_step() {}
        error
    }

    pub fn close_step(&mut self) -> bool {
        if self.detail.close_step() {
            return true;
        }
        if let Some(handle) = self.result_handback {
            if db_io_result_handback(handle).is_ok() {
                self.result_handback = None;
                return true;
            }
        }
        if self.structured.is_some() {
            let progressed = db_io_close_structured_fault(&mut self.structured);
            if self.structured.is_none() && self.result_handback.is_none() { self.custody.take(); }
            return progressed;
        }
        self.result_handback.is_none() && self.custody.take().is_some()
    }
}

impl Drop for DbIoFault {
    fn drop(&mut self) {
        if self.detail.terminal_is_empty() && self.result_handback.is_none() && self.structured.is_none() {
            return;
        }
        let custody = self.custody.take().expect("fault admitted before its original cause");
        let (slot, generation) = (custody.slot, custody.generation);
        let owner = Self { kind: self.kind, cause: self.cause, detail: std::mem::replace(&mut self.detail, DbIoText::new()), structured: self.structured.take(), result_handback: self.result_handback.take(), custody: Some(custody) };
        let mut arena = DB_IO_FAULT_CUSTODY.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let cell = &mut arena.cells[slot];
        assert!(cell.reserved && cell.generation == generation && cell.owner.is_none(), "exact original fault parking cell");
        cell.owner = Some(owner);
        drop(arena);
        db_io_request_lost_owner_maintenance();
    }
}

pub enum DbIoTerminal {
    Result(DbIoResult),
    Fault(DbIoFault),
    Cancelled(Option<DbIoResult>),
}
//#endregion 🔖️Limits

const DB_IO_STRUCTURED_FAULT_CLOSE_GRANT: semio_framework_value::retained_clone::RetainedCloneGrant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: DB_IO_PAGE_BYTES, maximum_depth: 1 };

fn db_io_close_structured_fault(owner: &mut Option<DbError>) -> bool {
    let Some(error) = owner.take() else { return false; };
    let (mut cause, pack_owner) = match error {
        DbError::Pack(error) => (protocol::ProtocolError::Pack(error), true),
        DbError::Protocol(error) => (error, false),
        error => { *owner = Some(error); return false; }
    };
    let policy = DB_IO_STRUCTURED_FAULT_CLOSE_GRANT;
    let step = protocol::protocol_cause_retirement_demand(&cause).ok().filter(|demand| policy.maximum_items != 0 && demand.copy_bytes <= policy.maximum_copy_bytes && demand.capacity_bytes <= policy.maximum_capacity_bytes && demand.release_bytes <= policy.maximum_release_bytes && demand.depth <= policy.maximum_depth).map(|_| protocol::close_protocol_cause_one(&mut cause, policy));
    if matches!(step, Some(Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)))) { return true; }
    *owner = Some(if pack_owner {
        let protocol::ProtocolError::Pack(error) = cause else { unreachable!("Pack retirement preserves its original protocol wrapper"); };
        DbError::Pack(error)
    } else { DbError::Protocol(cause) });
    step.is_some_and(|step| step.is_ok())
}
