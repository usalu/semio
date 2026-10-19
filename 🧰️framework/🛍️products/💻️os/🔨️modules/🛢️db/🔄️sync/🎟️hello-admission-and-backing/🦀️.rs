//! 🎟️ Hello admission and backing: the sync hello limits and progress, the admission slots, state and handle, the ready future, the backing ledger and reservations, the owners with their envelope and frame closes and the credit and turn helpers. See [`super`].

use super::*;

//#region 🔖️Hello
//#region 👋️RetainedHello
pub const DATABASE_SYNC_HELLO_SLOTS: usize = 8;

pub const DATABASE_SYNC_HELLO_MAX_ITEMS: usize = 65_536;

pub const DATABASE_SYNC_HELLO_MAX_BYTES: usize = 256 * 1024 * 1024;

pub const DATABASE_SYNC_HELLO_RETRY_LIMIT: u8 = 8;

pub(super) const DATABASE_SYNC_HELLO_DEADLINE_MS: u64 = 30_000;

pub(super) const DATABASE_SYNC_HELLO_TURN_MS: u64 = 8;

/// ⏱️ How long one worker turn keeps re-polling a greeting that woke itself while it was
/// polled (every decoded field and WAL record yields once): each such wake used to be a whole
/// worker-pool round trip, ~10 per replayed envelope.
pub(super) const DATABASE_SYNC_HELLO_POLL_TURN_MICROS: u64 = 2_000;

pub const DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES: usize = 4 * 1024;

/// 📦️ The most envelopes one hello tail frame carries before it ends on the next commit boundary: the tail a replica
/// lacks streams as a sequence of such frames — each with the exact frontier after its last commit, as a peer's relay
/// carries — so no hello holds more than one frame of it, however long the document's history (ticket 26/09/23 session
/// 14c: a 13 201-edit document could no longer be opened, its whole tail was decoded into one frame first).
pub const DATABASE_SYNC_HELLO_TAIL_FRAME_ENVELOPES: usize = 256;

pub const DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_ITEMS: usize = db_io_core::DB_IO_OPERATION_PAGES;

pub const DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_BYTES: usize = DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_ITEMS * db_io_core::DB_IO_PAGE_BYTES;

const DATABASE_SYNC_HELLO_ADMISSION_WAITERS: usize = 256;

pub(super) const DATABASE_SYNC_HELLO_ADMISSION_SATURATED: &str = "database sync hello admission saturated";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DatabaseSyncHelloProgress {
    Admitted,
    Replay,
    Decode,
    Bootstrap,
    Welcome,
    Streaming,
    Closing,
    Completed,
    Cancelled,
    Fault,
}

impl DatabaseSyncHelloProgress {
    pub(super) fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Admitted,
            1 => Self::Replay,
            2 => Self::Decode,
            3 => Self::Bootstrap,
            4 => Self::Welcome,
            5 => Self::Streaming,
            6 => Self::Closing,
            7 => Self::Completed,
            8 => Self::Cancelled,
            _ => Self::Fault,
        }
    }
}

#[derive(Clone, Copy)]
struct DatabaseSyncHelloAdmissionSlot {
    pub(super) generation: u64,
    pub(super) items: usize,
    pub(super) bytes: usize,
    occupied: bool,
}

const EMPTY_DATABASE_SYNC_HELLO_SLOT: DatabaseSyncHelloAdmissionSlot = DatabaseSyncHelloAdmissionSlot { generation: 0, items: 0, bytes: 0, occupied: false };

struct DatabaseSyncHelloAdmissionState {
    slots: [DatabaseSyncHelloAdmissionSlot; DATABASE_SYNC_HELLO_SLOTS],
    pub(super) items: usize,
    pub(super) bytes: usize,
    next_generation: u64,
    waiters: AdmissionWaiters<DATABASE_SYNC_HELLO_ADMISSION_WAITERS>,
}

static DATABASE_SYNC_HELLO_ADMISSION: std::sync::Mutex<DatabaseSyncHelloAdmissionState> = std::sync::Mutex::new(DatabaseSyncHelloAdmissionState {
    slots: [EMPTY_DATABASE_SYNC_HELLO_SLOT; DATABASE_SYNC_HELLO_SLOTS],
    items: 0,
    bytes: 0,
    next_generation: 1,
    waiters: AdmissionWaiters::new(),
});

pub(super) struct DatabaseSyncHelloAdmission {
    pub(super) slot: usize,
    pub(super) generation: u64,
    pub(super) items: usize,
    pub(super) bytes: usize,
}

impl DatabaseSyncHelloAdmission {
    pub(super) fn try_claim(input_items: usize, input_bytes: usize) -> Result<Self, DbError> {
        if input_items == 0 || input_items > DATABASE_SYNC_HELLO_MAX_ITEMS || input_bytes > DATABASE_SYNC_HELLO_MAX_BYTES {
            return Err(DbError::LimitExceeded("database sync hello input credit"));
        }
        let mut state = DATABASE_SYNC_HELLO_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(slot) = state.slots.iter().position(|entry| !entry.occupied) else {
            return Err(DbError::Unavailable(DATABASE_SYNC_HELLO_ADMISSION_SATURATED.to_string()));
        };
        let items = DATABASE_SYNC_HELLO_MAX_ITEMS;
        let bytes = DATABASE_SYNC_HELLO_MAX_BYTES;
        let next_items = state.items.checked_add(items).ok_or(DbError::LimitExceeded("database sync hello total items"))?;
        let next_bytes = state.bytes.checked_add(bytes).ok_or(DbError::LimitExceeded("database sync hello total bytes"))?;
        if next_items > DATABASE_SYNC_HELLO_SLOTS * DATABASE_SYNC_HELLO_MAX_ITEMS || next_bytes > DATABASE_SYNC_HELLO_SLOTS * DATABASE_SYNC_HELLO_MAX_BYTES {
            return Err(DbError::Unavailable("database sync hello global credit saturated".to_string()));
        }
        let generation = state.next_generation;
        state.next_generation = generation.checked_add(1).ok_or(DbError::LimitExceeded("database sync hello generation"))?;
        state.slots[slot] = DatabaseSyncHelloAdmissionSlot { generation, items, bytes, occupied: true };
        state.items = next_items;
        state.bytes = next_bytes;
        Ok(Self { slot, generation, items, bytes })
    }

    pub(super) fn is_current(&self) -> bool {
        let state = DATABASE_SYNC_HELLO_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.slots.get(self.slot).is_some_and(|entry| entry.occupied && entry.generation == self.generation && entry.items == self.items && entry.bytes == self.bytes)
    }
}

impl Drop for DatabaseSyncHelloAdmission {
    fn drop(&mut self) {
        let mut state = DATABASE_SYNC_HELLO_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(entry) = state.slots.get_mut(self.slot) else { return };
        if !entry.occupied || entry.generation != self.generation || entry.items != self.items || entry.bytes != self.bytes {
            return;
        }
        *entry = EMPTY_DATABASE_SYNC_HELLO_SLOT;
        state.items = state.items.saturating_sub(self.items);
        state.bytes = state.bytes.saturating_sub(self.bytes);
        let waiters = state.waiters.wakers();
        drop(state);
        waiters.into_iter().flatten().for_each(std::task::Waker::wake);
    }
}

/// ⏳️ Resolves once a sync-hello admission slot is free (or at `deadline_ms` on the pool's
/// clock), so a hello beyond the process's `DATABASE_SYNC_HELLO_SLOTS` concurrent hellos waits its
/// turn instead of being refused: two dozen document sockets reopening together after a hub
/// restart are all welcomed (ticket 26/09/23 H9 session 12, growth e2e g15). The slot is not
/// reserved; the caller claims it with its next submission and waits again if another hello took it.
pub struct DatabaseSyncHelloAdmissionReady {
    pub pool: std::sync::Arc<semio_framework_async::WorkerPool>,
    pub deadline_ms: u64,
    pub waiter: Option<u64>,
    deadline_armed: bool,
}

impl DatabaseSyncHelloAdmissionReady {
    /// 🎟️ Waits for a free slot until `deadline_ms` on `pool`'s clock.
    pub fn new(pool: std::sync::Arc<semio_framework_async::WorkerPool>, deadline_ms: u64) -> Self {
        Self { pool, deadline_ms, waiter: None, deadline_armed: false }
    }
}

impl std::future::Future for DatabaseSyncHelloAdmissionReady {
    type Output = Result<(), DbError>;

    fn poll(self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();
        let mut state = DATABASE_SYNC_HELLO_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let free = state.slots.iter().any(|entry| !entry.occupied);
        if free || this.pool.now_ms() >= this.deadline_ms {
            if let Some(waiter) = this.waiter.take() {
                state.waiters.remove(waiter);
            }
            return std::task::Poll::Ready(if free { Ok(()) } else { Err(DbError::Unavailable(DATABASE_SYNC_HELLO_ADMISSION_SATURATED.to_string())) });
        }
        if !state.waiters.register(&mut this.waiter, context.waker()) {
            return std::task::Poll::Ready(Err(DbError::Unavailable("database sync hello admission waiters saturated".to_string())));
        }
        drop(state);
        if !this.deadline_armed {
            this.deadline_armed = true;
            let waker = context.waker().clone();
            this.pool.callback_at(this.deadline_ms, move || waker.wake());
        }
        std::task::Poll::Pending
    }
}

impl Drop for DatabaseSyncHelloAdmissionReady {
    fn drop(&mut self) {
        if let Some(waiter) = self.waiter.take() {
            DATABASE_SYNC_HELLO_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).waiters.remove(waiter);
        }
    }
}

/// 🔢️ How many tasks wait for a sync-hello admission slot right now.
pub fn database_sync_hello_admission_waiters() -> usize {
    DATABASE_SYNC_HELLO_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).waiters.len()
}

#[derive(Default)]
pub struct DatabaseSyncHelloBackingLedger {
    pub items: usize,
    pub bytes: usize,
}

impl DatabaseSyncHelloBackingLedger {
    pub fn observe(&mut self, items: usize, bytes: usize, label: &'static str) -> Result<(), DbError> {
        let next_items = self.items.checked_add(items).ok_or(DbError::LimitExceeded(label))?;
        let next_bytes = self.bytes.checked_add(bytes).ok_or(DbError::LimitExceeded(label))?;
        if next_items > DATABASE_SYNC_HELLO_MAX_ITEMS || next_bytes > DATABASE_SYNC_HELLO_MAX_BYTES {
            return Err(DbError::LimitExceeded(label));
        }
        self.items = next_items;
        self.bytes = next_bytes;
        Ok(())
    }

    pub fn release(&mut self, items: usize, bytes: usize) -> Result<(), DbError> {
        self.items = self.items.checked_sub(items).ok_or(DbError::Internal("database sync hello item credit underflow".to_string()))?;
        self.bytes = self.bytes.checked_sub(bytes).ok_or(DbError::Internal("database sync hello byte credit underflow".to_string()))?;
        Ok(())
    }

    pub(super) fn reserve_allocation(&mut self, items: usize, requested: usize, label: &'static str) -> Result<usize, DbError> {
        let reserved = DATABASE_SYNC_HELLO_MAX_BYTES.checked_sub(self.bytes).ok_or(DbError::LimitExceeded(label))?;
        if requested > reserved {
            return Err(DbError::LimitExceeded(label));
        }
        self.observe(items, reserved, label)?;
        Ok(reserved)
    }

    pub(super) fn settle_allocation(&mut self, reserved: usize, actual: usize, label: &'static str) -> Result<(), DbError> {
        if actual > reserved {
            return Err(DbError::LimitExceeded(label));
        }
        self.release(0, reserved - actual)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.items == 0 && self.bytes == 0
    }

    pub fn close_one_credit(&mut self) -> bool {
        if self.bytes != 0 {
            self.bytes -= self.bytes.min(db_io_core::DB_IO_PAGE_BYTES);
            return true;
        }
        if self.items != 0 {
            self.items -= 1;
            return true;
        }
        false
    }
}

pub fn database_sync_hello_allocate_vec<T>(ledger: &mut DatabaseSyncHelloBackingLedger, count: usize, label: &'static str) -> Result<Vec<T>, DbError> {
    if count == 0 {
        return Ok(Vec::new());
    }
    let requested = count.checked_mul(size_of::<T>()).ok_or(DbError::LimitExceeded(label))?;
    let reserved = ledger.reserve_allocation(1, requested, label)?;
    let mut owner = Vec::new();
    if owner.try_reserve_exact(count).is_err() {
        ledger.release(1, reserved)?;
        return Err(DbError::LimitExceeded(label));
    }
    let actual = owner.capacity().checked_mul(size_of::<T>()).ok_or(DbError::LimitExceeded(label))?;
    if actual > reserved {
        drop(owner);
        ledger.release(1, reserved)?;
        return Err(DbError::LimitExceeded(label));
    }
    ledger.settle_allocation(reserved, actual, label)?;
    Ok(owner)
}

struct DatabaseSyncHelloSnapshotBackingReservation {
    pub bytes: usize,
}

pub fn database_sync_hello_reserve_snapshot_chunk(ledger: &mut DatabaseSyncHelloBackingLedger, logical_len: usize) -> Result<DatabaseSyncHelloSnapshotBackingReservation, DbError> {
    if logical_len == 0 || logical_len > DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES {
        return Err(DbError::LimitExceeded("database sync hello fixed snapshot chunk length"));
    }
    ledger.observe(1, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES, "database sync hello fixed snapshot chunk backing")?;
    Ok(DatabaseSyncHelloSnapshotBackingReservation { bytes: DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES })
}

impl DatabaseSyncHelloSnapshotBackingReservation {
    pub fn allocate(self, ledger: &mut DatabaseSyncHelloBackingLedger) -> Result<protocol::SnapshotChunkBytes, DbError> {
        let mut owner = protocol::SnapshotChunkBytes::allocate_fixed();
        let actual = owner.backing_bytes();
        if actual != self.bytes {
            owner.close_one();
            ledger.release(1, self.bytes)?;
            return Err(DbError::LimitExceeded("database sync hello fixed snapshot chunk observed backing"));
        }
        Ok(owner)
    }
}

#[derive(Clone, Copy)]
struct DatabaseSyncHelloSnapshotPageReservation {
    pub(super) items: usize,
    pub(super) bytes: usize,
}

pub fn database_sync_hello_reserve_snapshot_pages(ledger: &mut DatabaseSyncHelloBackingLedger) -> Result<DatabaseSyncHelloSnapshotPageReservation, DbError> {
    ledger.observe(DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_ITEMS, DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_BYTES, "database sync hello fixed snapshot page backing")?;
    Ok(DatabaseSyncHelloSnapshotPageReservation { items: DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_ITEMS, bytes: DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_BYTES })
}

impl DatabaseSyncHelloSnapshotPageReservation {
    pub fn observed(self, pages: &db_io_core::DbIoPages) -> Result<(usize, usize), DbError> {
        let items = usize::from(pages.page_count());
        let bytes = items.checked_mul(db_io_core::DB_IO_PAGE_BYTES).ok_or(DbError::LimitExceeded("database sync hello snapshot observed page backing"))?;
        if items > self.items || bytes > self.bytes || pages.len() > bytes {
            return Err(DbError::LimitExceeded("database sync hello snapshot observed page backing"));
        }
        Ok((items, bytes))
    }

    pub fn settle(self, ledger: &mut DatabaseSyncHelloBackingLedger, items: usize, bytes: usize) -> Result<(), DbError> {
        ledger.release(self.items - items, self.bytes - bytes)
    }

    pub(super) fn release(self, ledger: &mut DatabaseSyncHelloBackingLedger) -> Result<(), DbError> {
        ledger.release(self.items, self.bytes)
    }
}

pub fn database_sync_hello_clone_string(source: &str, ledger: &mut DatabaseSyncHelloBackingLedger, label: &'static str) -> Result<String, DbError> {
    let mut owner = database_sync_hello_allocate_vec::<u8>(ledger, source.len(), label)?;
    owner.extend_from_slice(source.as_bytes());
    String::from_utf8(owner).map_err(|error| {
        let mut owner = error.into_bytes();
        let capacity = owner.capacity();
        drop(std::mem::take(&mut owner));
        let _ = ledger.release(1, capacity);
        DbError::Corrupt("database sync hello admitted text lost utf-8 validity".to_string())
    })
}

pub struct DatabaseSyncHelloOwners {
    pub storage: Option<std::sync::Arc<db_storage::DbBackend>>,
    pub document: ArtifactId,
    pub(super) hello_frontier: Option<protocol::RuntimeFrontierSummary>,
    pub session_id: String,
    pub origin: protocol::ActorId,
    pub(super) snapshot_chunk_bytes: usize,
}

pub(super) fn database_sync_hello_retire_string(owner: &mut String) -> bool {
    if owner.capacity() == 0 {
        return false;
    }
    drop(std::mem::take(owner));
    true
}

const DATABASE_SYNC_HELLO_SHARED_TEXT_CLOSE_GRANT: semio_framework_value::retained_clone::RetainedCloneGrant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: DATABASE_SYNC_HELLO_MAX_BYTES, maximum_depth: 1 };

fn database_sync_hello_retire_shared_text(owner: &mut semio_framework_value::SharedUtf8) -> bool {
    if !owner.has_owner() { return false; }
    let grant = DATABASE_SYNC_HELLO_SHARED_TEXT_CLOSE_GRANT;
    if owner.original_allocation_bytes() > grant.maximum_release_bytes { return true; }
    let (original, _) = std::mem::take(owner).close_original_lease(grant).unwrap_or_else(|_| panic!("database sync actor lease exceeded its original allocation credit"));
    drop(original);
    true
}

fn database_sync_hello_retire_bytes(owner: &mut Vec<u8>) -> bool {
    if owner.capacity() == 0 {
        return false;
    }
    drop(std::mem::take(owner));
    true
}

struct DatabaseSyncHelloEnvelopeClose {
    pub(super) owner: Option<protocol::MutationEnvelope>,
}

impl DatabaseSyncHelloEnvelopeClose {
    pub(super) fn close_one(&mut self) -> bool {
        let Some(owner) = self.owner.as_mut() else { return false };
        if let Some(dependency) = owner.dependencies.pop() {
            drop(dependency);
            return true;
        }
        if owner.dependencies.capacity() != 0 {
            drop(std::mem::take(&mut owner.dependencies));
            return true;
        }
        if database_sync_hello_retire_string(&mut owner.mutation_id.0)
            || database_sync_hello_retire_string(&mut owner.document_id.0)
            || database_sync_hello_retire_shared_text(&mut owner.actor.0)
            || database_sync_hello_retire_string(&mut owner.diff.schema.0)
            || database_sync_hello_retire_bytes(&mut owner.diff.payload)
            || database_sync_hello_retire_string(&mut owner.inverse.schema.0)
            || database_sync_hello_retire_bytes(&mut owner.inverse.payload)
        {
            return true;
        }
        self.owner = None;
        true
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.owner.is_none()
    }
}

pub struct DatabaseSyncHelloFrameClose {
    pub(super) owner: Option<protocol::ServerFrame>,
    pub(super) envelope: Option<DatabaseSyncHelloEnvelopeClose>,
}

impl DatabaseSyncHelloFrameClose {
    pub fn close_one(&mut self) -> bool {
        if let Some(cursor) = self.envelope.as_mut() {
            let pending = cursor.close_one();
            if cursor.terminal_is_empty() {
                self.envelope = None;
            }
            return pending;
        }
        let Some(owner) = self.owner.as_mut() else { return false };
        match owner {
            protocol::ServerFrame::Welcome { session_id, resume_token, server_frontier, bootstrap } => {
                if database_sync_hello_retire_string(session_id)
                    || database_sync_hello_retire_string(resume_token)
                    || database_sync_hello_retire_string(&mut server_frontier.document_id.0)
                    || database_sync_hello_retire_string(&mut server_frontier.head_edit_id)
                {
                    return true;
                }
                if let protocol::Bootstrap::Snapshot { inline, .. } = bootstrap {
                    if let Some(bytes) = inline.as_mut() {
                        if database_sync_hello_retire_bytes(bytes) {
                            return true;
                        }
                        *inline = None;
                        return true;
                    }
                }
            }
            protocol::ServerFrame::Commands { envelopes, origin, frontier } => {
                if let Some(envelope) = envelopes.pop() {
                    self.envelope = Some(DatabaseSyncHelloEnvelopeClose { owner: Some(envelope) });
                    return true;
                }
                if envelopes.capacity() != 0 {
                    drop(std::mem::take(envelopes));
                    return true;
                }
                if database_sync_hello_retire_shared_text(&mut origin.0) || database_sync_hello_retire_string(&mut frontier.document_id.0) || database_sync_hello_retire_string(&mut frontier.head_edit_id) {
                    return true;
                }
            }
            protocol::ServerFrame::SnapshotChunk { bytes, .. } => {
                if bytes.close_one() {
                    return true;
                }
            }
            _ => {}
        }
        self.owner = None;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.owner.is_none() && self.envelope.is_none()
    }
}

impl DatabaseSyncHelloOwners {
    pub(super) fn storage(&self) -> Result<&db_storage::DbBackend, DbError> {
        self.storage.as_deref().ok_or_else(|| DbError::Internal("database sync hello storage owner missing".to_string()))
    }

    pub(super) fn close_one(&mut self) -> bool {
        if let Some(frontier) = self.hello_frontier.as_mut() {
            if database_sync_hello_retire_string(&mut frontier.document_id.0) || database_sync_hello_retire_string(&mut frontier.head_edit_id) {
                return true;
            }
            self.hello_frontier = None;
            return true;
        }
        if database_sync_hello_retire_string(&mut self.session_id) || database_sync_hello_retire_shared_text(&mut self.origin.0) || database_sync_hello_retire_string(&mut self.document.0) {
            return true;
        }
        self.storage.take().is_some()
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.storage.is_none() && self.hello_frontier.is_none() && self.session_id.capacity() == 0 && !self.origin.0.has_owner() && self.document.0.capacity() == 0
    }
}

pub(super) fn database_sync_hello_input_credit(owners: &DatabaseSyncHelloOwners) -> Result<(usize, usize), DbError> {
    let frontier_items = owners.hello_frontier.as_ref().map_or(0, |_| 3);
    let items = 4usize.checked_add(frontier_items).ok_or(DbError::LimitExceeded("database sync hello input items"))?;
    let frontier_bytes = owners.hello_frontier.as_ref().map_or(0, |frontier| frontier.document_id.0.capacity().saturating_add(frontier.head_edit_id.capacity()).saturating_add(size_of::<protocol::RuntimeFrontierSummary>()));
    let bytes = owners
        .document
        .0
        .capacity()
        .checked_add(owners.session_id.capacity())
        .and_then(|value| value.checked_add(owners.origin.0.original_allocation_bytes()))
        .and_then(|value| value.checked_add(frontier_bytes))
        .ok_or(DbError::LimitExceeded("database sync hello input bytes"))?;
    Ok((items, bytes))
}

pub async fn database_sync_hello_opportunity(cancelled: &std::sync::atomic::AtomicBool, expired: &std::sync::atomic::AtomicBool) -> Result<(), DbError> {
    database_sync_hello_control(cancelled, expired)?;
    semio_framework_async::yield_once().await;
    database_sync_hello_control(cancelled, expired)
}

pub fn database_sync_hello_control(cancelled: &std::sync::atomic::AtomicBool, expired: &std::sync::atomic::AtomicBool) -> Result<(), DbError> {
    if expired.load(std::sync::atomic::Ordering::Acquire) {
        return Err(DbError::Timeout("database sync hello deadline".to_string()));
    }
    if cancelled.load(std::sync::atomic::Ordering::Acquire) {
        return Err(DbError::Closed);
    }
    Ok(())
}

pub(super) fn database_sync_hello_turn_exhausted(error: &DbError) -> bool {
    matches!(error, DbError::LimitExceeded("wal cursor fuel")) || matches!(error, DbError::Unavailable(message) if message == "wal cursor deadline reached")
}

/// 🧮️ The heap backing one decoded command holds, in the ledger's units: one item per non-empty
/// owner, its capacity in bytes.
pub(super) fn database_sync_hello_envelope_credit(envelope: &protocol::MutationEnvelope) -> Result<(usize, usize), DbError> {
    let label = "database sync hello envelope backing";
    let mut items = 0usize;
    let mut bytes = 0usize;
    let dependencies = envelope.dependencies.capacity().checked_mul(size_of::<protocol::MutationId>()).ok_or(DbError::LimitExceeded(label))?;
    let owners = [envelope.mutation_id.0.capacity(), envelope.document_id.0.capacity(), envelope.actor.0.original_allocation_bytes(), dependencies, envelope.diff.schema.0.capacity(), envelope.diff.payload.capacity(), envelope.inverse.schema.0.capacity(), envelope.inverse.payload.capacity()];
    for capacity in owners.into_iter().chain(envelope.dependencies.iter().map(|dependency| dependency.0.capacity())) {
        if capacity != 0 {
            items = items.checked_add(1).ok_or(DbError::LimitExceeded(label))?;
            bytes = bytes.checked_add(capacity).ok_or(DbError::LimitExceeded(label))?;
        }
    }
    Ok((items, bytes))
}

pub(super) async fn database_sync_hello_close_pages(pages: &mut db_io_core::DbIoPages, cancelled: &std::sync::atomic::AtomicBool, expired: &std::sync::atomic::AtomicBool) -> Result<(), DbError> {
    let mut control_error = None;
    while pages.close_step()?.is_some() {
        semio_framework_async::yield_once().await;
        if control_error.is_none() {
            control_error = database_sync_hello_control(cancelled, expired).err();
        }
    }
    control_error.map_or(Ok(()), Err)
}
//#endregion 👋️RetainedHello
//#endregion 🔖️Hello
