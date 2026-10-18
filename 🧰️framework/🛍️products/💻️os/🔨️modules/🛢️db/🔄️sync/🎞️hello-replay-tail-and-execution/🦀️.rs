//! 🎞️ Hello replay, tail and execution: the retained replay, the tail handoff and slot, the tail and its reader, the follow up with its grant and steps, the prepared execution, the execute turn, the follow up close fault and the quarantine close. See [`super`].

use super::*;

//#region 🔖️Hello
//#region 👋️RetainedHello
/// 🧾️ What a retained hello takes from the WAL before it answers: the server frontier, the id of its last command and
/// the snapshot floor — every command is hashed into the chain and counted, none is materialized; the tail a replica lacks
/// streams afterwards ([`DatabaseSyncHelloTail`]).
struct DatabaseSyncHelloReplay {
    pub frontier: Frontier,
    pub head_edit_id: String,
    pub floor_head_seq: u64,
}

pub async fn replay_sync_state_retained(
    storage: &db_storage::DbBackend,
    document: ArtifactId,
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    expired: std::sync::Arc<std::sync::atomic::AtomicBool>,
    ledger: &mut DatabaseSyncHelloBackingLedger,
    progress: &std::sync::atomic::AtomicU8,
) -> Result<DatabaseSyncHelloReplay, DbError> {
    progress.store(DatabaseSyncHelloProgress::Replay as u8, std::sync::atomic::Ordering::Release);
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_SYNC_HELLO_TURN_MS);
    let control = db_wal_log::WalCursorControl::new(cancelled.clone(), deadline, DATABASE_SYNC_HELLO_MAX_ITEMS)?;
    database_sync_hello_control(&cancelled, &expired)?;
    let wal = storage.wal().await;
    database_sync_hello_control(&cancelled, &expired)?;
    let mut records = db_wal_log::replay_committed_document(&wal, &document, control).await?;
    let mut decode_control = db_wal_log::WalCursorControl::stall_bounded(cancelled.clone(), db_wal_log::WAL_REPLAY_STEP_STALL_BOUND, db_wal_log::WAL_REPLAY_STEP_FUEL)?;
    let mut chain = semio_framework_hash::Hasher::new();
    let mut head_seq = 0u64;
    let mut commit_seq = 0u64;
    let mut floor_head_seq = 0u64;
    let mut head_edit_id = String::new();
    let mut turn = db_wal_log::WalTurn::start();
    let replay = async {
        loop {
            records.replenish(std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_SYNC_HELLO_TURN_MS), DATABASE_SYNC_HELLO_MAX_ITEMS)?;
            database_sync_hello_control(&cancelled, &expired)?;
            let mut transaction = match records.next_transaction_step().await {
                Ok(db_wal_log::WalCommittedStep::Transaction(transaction)) => transaction,
                Ok(db_wal_log::WalCommittedStep::Yield) => {
                    turn.step().await;
                    continue;
                }
                Ok(db_wal_log::WalCommittedStep::Done) => break,
                Err(error) if database_sync_hello_turn_exhausted(&error) => {
                    database_sync_hello_opportunity(&cancelled, &expired).await?;
                    continue;
                }
                Err(error) => return Err(error),
            };
            loop {
                transaction.replenish(std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_SYNC_HELLO_TURN_MS), DATABASE_SYNC_HELLO_MAX_ITEMS)?;
                decode_control.renew_step()?;
                database_sync_hello_control(&cancelled, &expired)?;
                match transaction.next_record_step() {
                    Ok(db_wal_log::WalCommittedRecordStep::Record(db_wal_log::WalRecord::Command(bytes))) => {
                        progress.store(DatabaseSyncHelloProgress::Decode as u8, std::sync::atomic::Ordering::Release);
                        chain.update(&bytes.hash());
                        head_edit_id = wal_command_id(bytes, &mut decode_control)?;
                        head_seq = head_seq.checked_add(1).ok_or(DbError::LimitExceeded("database sync hello head sequence"))?;
                    }
                    Ok(db_wal_log::WalCommittedRecordStep::Record(db_wal_log::WalRecord::SnapshotPub { frontier, .. })) => floor_head_seq = frontier.head_seq,
                    Ok(db_wal_log::WalCommittedRecordStep::Record(_)) => {}
                    Ok(db_wal_log::WalCommittedRecordStep::Yield) => {
                        turn.step().await;
                        continue;
                    }
                    Ok(db_wal_log::WalCommittedRecordStep::Done) => break,
                    Err(error) if database_sync_hello_turn_exhausted(&error) => {
                        database_sync_hello_opportunity(&cancelled, &expired).await?;
                        continue;
                    }
                    Err(error) => return Err(error),
                }
                while transaction.close_record_step()? {
                    turn.step().await;
                }
                turn.step().await;
            }
            transaction.finish()?;
            commit_seq = commit_seq.checked_add(1).ok_or(DbError::LimitExceeded("database sync hello commit sequence"))?;
            turn.step().await;
        }
        database_sync_hello_control(&cancelled, &expired)?;
        Ok::<(), DbError>(())
    }
    .await;
    let close = async {
        while records.close_owner_step()? {
            semio_framework_async::yield_once().await;
        }
        Ok::<(), DbError>(())
    }
    .await;
    replay.and(close)?;
    ledger.observe(usize::from(head_edit_id.capacity() != 0), head_edit_id.capacity(), "database sync hello server frontier head backing")?;
    let chain_hash = if head_seq == 0 { [0; 32] } else { *chain.finalize().as_bytes() };
    Ok(DatabaseSyncHelloReplay { frontier: Frontier { document, head_seq, commit_seq, chain_hash, epoch: 0 }, head_edit_id, floor_head_seq })
}

/// 📄️ One tail frame the reader handed over: its envelopes and the frontier after its last commit (head, head id, commit
/// sequence, chain hash).
type DatabaseSyncHelloTailPage = (Vec<protocol::MutationEnvelope>, (u64, String, u64, [u8; 32]));

/// 🤝️ The one-page slot between the tail reader and the hello driver: the reader parks here while the previous frame is still
/// in the slot, and the driver wakes it when it takes that frame.
#[derive(Default)]
struct DatabaseSyncHelloTailHandoff {
    pub(super) page: Option<DatabaseSyncHelloTailPage>,
    reader: Option<std::task::Waker>,
}

/// ⏳️ Resolves once the tail slot is empty.
struct DatabaseSyncHelloTailSlotFree<'slot>(&'slot std::sync::Mutex<DatabaseSyncHelloTailHandoff>);

impl std::future::Future for DatabaseSyncHelloTailSlotFree<'_> {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        let mut handoff = self.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if handoff.page.is_none() {
            return std::task::Poll::Ready(());
        }
        handoff.reader = Some(context.waker().clone());
        std::task::Poll::Pending
    }
}

type DatabaseSyncHelloTailReaderFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), DbError>> + Send>>;

/// 🚰️ The tail a replica lacks, streamed from the WAL one frame at a time: ONE reader ([`database_sync_hello_tail_reader`])
/// walks the WAL once and hands each frame over through a one-page slot; each frame carries the exact frontier after its last
/// commit and its backing is charged while it is out and released when it closes, so a hello holds at most about two frames of
/// the tail however long it is.
struct DatabaseSyncHelloTail {
    pub(super) document: ArtifactId,
    pub(super) origin: protocol::ActorId,
    handoff: std::sync::Arc<std::sync::Mutex<DatabaseSyncHelloTailHandoff>>,
    reader: Option<DatabaseSyncHelloTailReaderFuture>,
    pub(super) done: bool,
}

impl DatabaseSyncHelloTail {
    pub(super) fn new(storage: std::sync::Arc<db_storage::DbBackend>, document: ArtifactId, origin: protocol::ActorId, replica_head: u64, server: &Frontier, cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>, expired: std::sync::Arc<std::sync::atomic::AtomicBool>) -> Self {
        let handoff = std::sync::Arc::new(std::sync::Mutex::new(DatabaseSyncHelloTailHandoff::default()));
        let reader = Box::pin(database_sync_hello_tail_reader(storage, document.clone(), replica_head, server.head_seq, server.commit_seq, handoff.clone(), cancelled, expired));
        Self { document, origin, handoff, reader: Some(reader), done: false }
    }

    fn take_page(&self) -> Option<DatabaseSyncHelloTailPage> {
        let mut handoff = self.handoff.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let page = handoff.page.take()?;
        if let Some(reader) = handoff.reader.take() {
            reader.wake();
        }
        Some(page)
    }

    /// 🚚️ Advances the reader and hands out the next frame it produced.
    fn drive(&mut self, ledger: &mut DatabaseSyncHelloBackingLedger, context: &mut std::task::Context<'_>) -> Result<DatabaseSyncHelloFollowUpStep, DbError> {
        if self.done {
            return Ok(DatabaseSyncHelloFollowUpStep::Done);
        }
        let finished = match self.reader.as_mut() {
            Some(reader) => match std::future::Future::poll(reader.as_mut(), context) {
                std::task::Poll::Pending => false,
                std::task::Poll::Ready(read) => {
                    self.reader = None;
                    read?;
                    true
                }
            },
            None => true,
        };
        let Some((envelopes, (head_edit_ordinal, head_edit_id, last_commit_seq, chain_hash))) = self.take_page() else {
            if finished {
                self.done = true;
                return Ok(DatabaseSyncHelloFollowUpStep::Done);
            }
            return Ok(DatabaseSyncHelloFollowUpStep::Waiting);
        };
        let frontier = protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(self.document.0.clone()), head_edit_ordinal, head_edit_id, last_commit_seq, chain_hash };
        let frame = protocol::ServerFrame::Commands { envelopes, origin: self.origin.clone(), frontier };
        let (items, bytes) = database_sync_hello_returned_frame_credit(&frame)?;
        ledger.observe(items, bytes, "database sync hello tail frame backing")?;
        Ok(DatabaseSyncHelloFollowUpStep::Frame(frame))
    }

    pub(super) fn close_one(&mut self) -> bool {
        if self.reader.take().is_some() || self.handoff.lock().unwrap_or_else(std::sync::PoisonError::into_inner).page.take().is_some() {
            return true;
        }
        database_sync_hello_retire_string(&mut self.origin.0) || database_sync_hello_retire_string(&mut self.document.0)
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.reader.is_none() && self.handoff.lock().unwrap_or_else(std::sync::PoisonError::into_inner).page.is_none() && self.origin.0.capacity() == 0 && self.document.0.capacity() == 0
    }
}

/// 🚰️ Walks the document's WAL once from its first retained segment: hashes every command into the chain, decodes those past
/// `replica_head` and, on the first commit boundary after [`DATABASE_SYNC_HELLO_TAIL_FRAME_ENVELOPES`] envelopes (or at the
/// tail's end, `end_head_seq`), hands the frame over — waiting while the previous one is still in the slot. The frame that
/// emits the last command carries the server's commit sequence `end_commit_seq`: transactions without a command after it
/// count in the frontier the welcome announced.
#[allow(clippy::too_many_arguments)]
async fn database_sync_hello_tail_reader(
    storage: std::sync::Arc<db_storage::DbBackend>,
    document: ArtifactId,
    replica_head: u64,
    end_head_seq: u64,
    end_commit_seq: u64,
    handoff: std::sync::Arc<std::sync::Mutex<DatabaseSyncHelloTailHandoff>>,
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    expired: std::sync::Arc<std::sync::atomic::AtomicBool>,
) -> Result<(), DbError> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_SYNC_HELLO_TURN_MS);
    let control = db_wal_log::WalCursorControl::new(cancelled.clone(), deadline, DATABASE_SYNC_HELLO_MAX_ITEMS)?;
    database_sync_hello_control(&cancelled, &expired)?;
    let wal = storage.wal().await;
    database_sync_hello_control(&cancelled, &expired)?;
    let mut records = db_wal_log::replay_committed_document(&wal, &document, control).await?;
    let mut decode_control = db_wal_log::WalCursorControl::stall_bounded(cancelled.clone(), db_wal_log::WAL_REPLAY_STEP_STALL_BOUND, db_wal_log::WAL_REPLAY_STEP_FUEL)?;
    let mut chain = semio_framework_hash::Hasher::new();
    let mut envelopes: Vec<protocol::MutationEnvelope> = Vec::new();
    let mut head_seq = 0u64;
    let mut commit_seq = 0u64;
    let mut head_edit_id = String::new();
    let mut turn = db_wal_log::WalTurn::start();
    let read = async {
        while head_seq < end_head_seq {
            records.replenish(std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_SYNC_HELLO_TURN_MS), DATABASE_SYNC_HELLO_MAX_ITEMS)?;
            database_sync_hello_control(&cancelled, &expired)?;
            let mut transaction = match records.next_transaction_step().await {
                Ok(db_wal_log::WalCommittedStep::Transaction(transaction)) => transaction,
                Ok(db_wal_log::WalCommittedStep::Yield) => {
                    turn.step().await;
                    continue;
                }
                Ok(db_wal_log::WalCommittedStep::Done) => break,
                Err(error) if database_sync_hello_turn_exhausted(&error) => {
                    database_sync_hello_opportunity(&cancelled, &expired).await?;
                    continue;
                }
                Err(error) => return Err(error),
            };
            loop {
                transaction.replenish(std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_SYNC_HELLO_TURN_MS), DATABASE_SYNC_HELLO_MAX_ITEMS)?;
                decode_control.renew_step()?;
                database_sync_hello_control(&cancelled, &expired)?;
                match transaction.next_record_step() {
                    Ok(db_wal_log::WalCommittedRecordStep::Record(db_wal_log::WalRecord::Command(bytes))) => {
                        head_seq = head_seq.checked_add(1).ok_or(DbError::LimitExceeded("database sync hello tail head sequence"))?;
                        chain.update(&bytes.hash());
                        if head_seq > replica_head {
                            let envelope = decode_wal_command(bytes, &mut decode_control)?;
                            head_edit_id.clone_from(&envelope.mutation_id.0);
                            envelopes.push(envelope);
                        }
                    }
                    Ok(db_wal_log::WalCommittedRecordStep::Record(_)) => {}
                    Ok(db_wal_log::WalCommittedRecordStep::Yield) => {
                        turn.step().await;
                        continue;
                    }
                    Ok(db_wal_log::WalCommittedRecordStep::Done) => break,
                    Err(error) if database_sync_hello_turn_exhausted(&error) => {
                        database_sync_hello_opportunity(&cancelled, &expired).await?;
                        continue;
                    }
                    Err(error) => return Err(error),
                }
                while transaction.close_record_step()? {
                    turn.step().await;
                }
                turn.step().await;
            }
            transaction.finish()?;
            commit_seq = commit_seq.checked_add(1).ok_or(DbError::LimitExceeded("database sync hello tail commit sequence"))?;
            if envelopes.len() >= DATABASE_SYNC_HELLO_TAIL_FRAME_ENVELOPES || (head_seq >= end_head_seq && !envelopes.is_empty()) {
                let frame_commit_seq = if head_seq >= end_head_seq { end_commit_seq } else { commit_seq };
                let frontier = (head_seq, std::mem::take(&mut head_edit_id), frame_commit_seq, *chain.finalize().as_bytes());
                DatabaseSyncHelloTailSlotFree(&handoff).await;
                database_sync_hello_control(&cancelled, &expired)?;
                handoff.lock().unwrap_or_else(std::sync::PoisonError::into_inner).page = Some((std::mem::take(&mut envelopes), frontier));
            }
            turn.step().await;
        }
        database_sync_hello_control(&cancelled, &expired)?;
        Ok::<(), DbError>(())
    }
    .await;
    let close = async {
        while records.close_owner_step()? {
            semio_framework_async::yield_once().await;
        }
        Ok::<(), DbError>(())
    }
    .await;
    read.and(close)
}

pub enum DatabaseSyncHelloFollowUp {
    None,
    Tail(Box<DatabaseSyncHelloTail>),
    Snapshot { pages: db_io_core::DbIoPages, chunk_bytes: usize, offset: usize, page: u8, page_offset: usize, seq: u32, chunk: Option<protocol::SnapshotChunkBytes>, done: bool },
}

pub struct DatabaseSyncHelloGrant {
    pub(super) deadline: std::time::Instant,
    checks: u8,
    forced_expiry_check: u8,
}

impl DatabaseSyncHelloGrant {
    fn fresh() -> Result<Self, DbError> {
        let deadline = std::time::Instant::now().checked_add(std::time::Duration::from_millis(DATABASE_SYNC_HELLO_TURN_MS)).ok_or(DbError::LimitExceeded("database sync hello grant deadline"))?;
        Ok(Self { deadline, checks: 0, forced_expiry_check: 0 })
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn expiring_at(check: u8) -> Self {
        Self { deadline: std::time::Instant::now() + std::time::Duration::from_secs(1), checks: 0, forced_expiry_check: check }
    }

    fn check(&mut self, cancelled: &std::sync::atomic::AtomicBool, expired: &std::sync::atomic::AtomicBool) -> Result<(), DbError> {
        database_sync_hello_control(cancelled, expired)?;
        self.checks = self.checks.saturating_add(1);
        if self.forced_expiry_check != 0 && self.checks >= self.forced_expiry_check || std::time::Instant::now() >= self.deadline {
            return Err(DbError::Timeout("database sync hello 8 ms grant".to_string()));
        }
        Ok(())
    }
}

/// 🪜️ What one drive of a hello follow-up produced: a frame, the end of the stream, progress toward a frame (drive
/// again), or a read in flight whose completion wakes the hello.
pub(super) enum DatabaseSyncHelloFollowUpStep {
    Frame(protocol::ServerFrame),
    Done,
    Progress,
    Waiting,
}

impl DatabaseSyncHelloFollowUp {
    pub(super) fn drive_one(&mut self, ledger: &mut DatabaseSyncHelloBackingLedger, cancelled: &std::sync::atomic::AtomicBool, expired: &std::sync::atomic::AtomicBool, context: &mut std::task::Context<'_>) -> Result<DatabaseSyncHelloFollowUpStep, DbError> {
        let mut grant = DatabaseSyncHelloGrant::fresh()?;
        self.drive_one_with_grant(ledger, cancelled, expired, &mut grant, context)
    }

    pub fn drive_one_with_grant(
        &mut self,
        ledger: &mut DatabaseSyncHelloBackingLedger,
        cancelled: &std::sync::atomic::AtomicBool,
        expired: &std::sync::atomic::AtomicBool,
        grant: &mut DatabaseSyncHelloGrant,
        context: &mut std::task::Context<'_>,
    ) -> Result<DatabaseSyncHelloFollowUpStep, DbError> {
        grant.check(cancelled, expired)?;
        match self {
            Self::None => Ok(DatabaseSyncHelloFollowUpStep::Done),
            Self::Tail(tail) => tail.drive(ledger, context),
            Self::Snapshot { pages, chunk_bytes, offset, page, page_offset, seq, chunk, done } => {
                if *done {
                    return Ok(DatabaseSyncHelloFollowUpStep::Done);
                }
                if *offset == pages.len() {
                    grant.check(cancelled, expired)?;
                    *done = true;
                    return Ok(DatabaseSyncHelloFollowUpStep::Frame(protocol::ServerFrame::SnapshotDone { seq_count: *seq }));
                }
                let unit_bytes = (*chunk_bytes).min(DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES);
                if chunk.is_none() {
                    grant.check(cancelled, expired)?;
                    let len = unit_bytes.min(pages.len() - *offset);
                    let reservation = database_sync_hello_reserve_snapshot_chunk(ledger, len)?;
                    let owner = reservation.allocate(ledger)?;
                    *chunk = Some(owner);
                }
                grant.check(cancelled, expired)?;
                let fragment = pages.page(*page).ok_or_else(|| DbError::Corrupt("database sync hello snapshot page missing".to_string()))?;
                let target = chunk.as_mut().ok_or_else(|| DbError::Internal("database sync hello snapshot chunk owner missing".to_string()))?;
                let remaining = unit_bytes.saturating_sub(target.len()).min(pages.len() - *offset);
                let copied = remaining.min(fragment.len().saturating_sub(*page_offset));
                if copied == 0 {
                    return Err(DbError::Corrupt("database sync hello snapshot cursor stalled".to_string()));
                }
                grant.check(cancelled, expired)?;
                if !target.try_extend_from_slice(&fragment[*page_offset..*page_offset + copied]) {
                    return Err(DbError::LimitExceeded("database sync hello fixed snapshot chunk copy"));
                }
                *offset += copied;
                *page_offset += copied;
                if *page_offset == fragment.len() {
                    *page = page.checked_add(1).ok_or(DbError::LimitExceeded("database sync hello snapshot page cursor"))?;
                    *page_offset = 0;
                }
                if target.len() == unit_bytes || *offset == pages.len() {
                    grant.check(cancelled, expired)?;
                    let next_seq = seq.checked_add(1).ok_or(DbError::LimitExceeded("database sync hello snapshot sequence"))?;
                    let bytes = chunk.take().ok_or_else(|| DbError::Internal("database sync hello completed chunk missing".to_string()))?;
                    let frame = protocol::ServerFrame::SnapshotChunk { seq: *seq, bytes };
                    *seq = next_seq;
                    return Ok(DatabaseSyncHelloFollowUpStep::Frame(frame));
                }
                Ok(DatabaseSyncHelloFollowUpStep::Progress)
            }
        }
    }

    pub fn close_one(&mut self) -> Result<bool, DbError> {
        match self {
            Self::None => Ok(false),
            Self::Tail(tail) => Ok(tail.close_one()),
            Self::Snapshot { pages, chunk, .. } => {
                if let Some(owner) = chunk.as_mut() {
                    if owner.close_one() {
                        return Ok(true);
                    }
                    *chunk = None;
                    return Ok(true);
                }
                Ok(pages.close_step()?.is_some())
            }
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        match self {
            Self::None => true,
            Self::Tail(tail) => tail.terminal_is_empty(),
            Self::Snapshot { pages, chunk, .. } => pages.terminal_is_empty() && chunk.is_none(),
        }
    }
}

struct DatabaseSyncHelloPrepared {
    pub(super) welcome: Option<protocol::ServerFrame>,
    pub(super) welcome_close: Option<DatabaseSyncHelloFrameClose>,
    pub(super) follow_up: DatabaseSyncHelloFollowUp,
}

pub(super) struct DatabaseSyncHelloExecution {
    pub owners: Option<DatabaseSyncHelloOwners>,
    pub prepared: Result<DatabaseSyncHelloPrepared, DbError>,
    pub ledger: Option<DatabaseSyncHelloBackingLedger>,
    pub(super) close_fault: Option<DatabaseSyncHelloFollowUpCloseFault>,
}

pub(super) async fn database_sync_hello_execute(
    mut owners: DatabaseSyncHelloOwners,
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    expired: std::sync::Arc<std::sync::atomic::AtomicBool>,
    progress: std::sync::Arc<std::sync::atomic::AtomicU8>,
) -> DatabaseSyncHelloExecution {
    let mut ledger = DatabaseSyncHelloBackingLedger::default();
    let run = async {
        if owners.snapshot_chunk_bytes == 0 {
            return Err(DbError::InvalidArgument("snapshot_chunk_bytes must be non-zero".to_string()));
        }
        let (input_items, input_bytes) = database_sync_hello_input_credit(&owners)?;
        ledger.observe(input_items, input_bytes, "database sync hello cumulative input backing")?;
        database_sync_hello_opportunity(&cancelled, &expired).await?;
        let document = ArtifactId(std::mem::take(&mut owners.document.0));
        database_sync_hello_control(&cancelled, &expired)?;
        let replica_head = owners.hello_frontier.as_ref().map_or(0, |frontier| frontier.head_edit_ordinal);
        let DatabaseSyncHelloReplay { frontier: mut server, head_edit_id, floor_head_seq } = replay_sync_state_retained(owners.storage()?, document, cancelled.clone(), expired.clone(), &mut ledger, &progress).await?;
        database_sync_hello_opportunity(&cancelled, &expired).await?;
        let replica = owners.hello_frontier.as_ref();
        if let Some(replica) = replica.as_ref() {
            if replica.document_id.0 != server.document.0 {
                return Err(DbError::InvalidArgument("database sync hello frontier document mismatch".to_string()));
            }
            if replica.head_edit_ordinal > server.head_seq {
                return Err(DbError::InvalidArgument("database sync hello frontier ahead of server".to_string()));
            }
        }
        progress.store(DatabaseSyncHelloProgress::Bootstrap as u8, std::sync::atomic::Ordering::Release);
        let (bootstrap, follow_up) = if replica_head >= floor_head_seq {
            if replica_head == server.head_seq {
                (protocol::Bootstrap::None, DatabaseSyncHelloFollowUp::None)
            } else {
                let storage = owners.storage.clone().ok_or_else(|| DbError::Internal("database sync hello storage owner missing".to_string()))?;
                let document = ArtifactId(database_sync_hello_clone_string(&server.document.0, &mut ledger, "database sync hello tail document backing")?);
                let origin = protocol::ActorId(std::mem::take(&mut owners.origin.0));
                let tail = DatabaseSyncHelloTail::new(storage, document, origin, replica_head, &server, cancelled.clone(), expired.clone());
                (protocol::Bootstrap::Tail, DatabaseSyncHelloFollowUp::Tail(Box::new(tail)))
            }
        } else {
            let snapshots = owners.storage()?.snapshot().await;
            database_sync_hello_control(&cancelled, &expired)?;
            let generation = snapshots.latest_generation(&server.document).await?.ok_or_else(|| DbError::Unavailable("database sync hello snapshot generation unavailable".to_string()))?;
            database_sync_hello_control(&cancelled, &expired)?;
            let page_reservation = database_sync_hello_reserve_snapshot_pages(&mut ledger)?;
            let mut pages = match snapshots.read_generation(&server.document, generation).await {
                Ok(pages) => pages,
                Err(error) => {
                    page_reservation.release(&mut ledger)?;
                    return Err(error);
                }
            };
            if let Err(error) = database_sync_hello_control(&cancelled, &expired) {
                database_sync_hello_close_pages(&mut pages, &cancelled, &expired).await?;
                page_reservation.release(&mut ledger)?;
                return Err(error);
            }
            let (page_items, page_bytes) = match page_reservation.observed(&pages) {
                Ok(observed) => observed,
                Err(error) => {
                    database_sync_hello_close_pages(&mut pages, &cancelled, &expired).await?;
                    page_reservation.release(&mut ledger)?;
                    return Err(error);
                }
            };
            page_reservation.settle(&mut ledger, page_items, page_bytes)?;
            let mut hash = semio_framework_hash::Hasher::new();
            for page in 0..pages.page_count() {
                let Some(fragment) = pages.page(page) else {
                    database_sync_hello_close_pages(&mut pages, &cancelled, &expired).await?;
                    ledger.release(page_items, page_bytes)?;
                    return Err(DbError::Corrupt("database sync hello snapshot hash page missing".to_string()));
                };
                hash.update(fragment);
                if let Err(error) = database_sync_hello_opportunity(&cancelled, &expired).await {
                    database_sync_hello_close_pages(&mut pages, &cancelled, &expired).await?;
                    ledger.release(page_items, page_bytes)?;
                    return Err(error);
                }
            }
            database_sync_hello_control(&cancelled, &expired)?;
            let observed_generation = match snapshots.latest_generation(&server.document).await {
                Ok(generation) => generation,
                Err(error) => {
                    database_sync_hello_close_pages(&mut pages, &cancelled, &expired).await?;
                    ledger.release(page_items, page_bytes)?;
                    return Err(error);
                }
            };
            if observed_generation != Some(generation) {
                database_sync_hello_close_pages(&mut pages, &cancelled, &expired).await?;
                ledger.release(page_items, page_bytes)?;
                return Err(DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(observed_generation.unwrap_or(0)) });
            }
            let pack_hash = *hash.finalize().as_bytes();
            (protocol::Bootstrap::Snapshot { pack_hash, inline: None }, DatabaseSyncHelloFollowUp::Snapshot { pages, chunk_bytes: owners.snapshot_chunk_bytes, offset: 0, page: 0, page_offset: 0, seq: 0, chunk: None, done: false })
        };
        progress.store(DatabaseSyncHelloProgress::Welcome as u8, std::sync::atomic::Ordering::Release);
        database_sync_hello_control(&cancelled, &expired)?;
        let resume_reservation = ledger.reserve_allocation(1, 0, "database sync hello resume token backing")?;
        let mut resume_token = match issue_resume_token(&server).await {
            Ok(owner) => owner,
            Err(error) => {
                ledger.release(1, resume_reservation)?;
                return Err(error);
            }
        };
        if resume_token.capacity() > resume_reservation {
            drop(std::mem::take(&mut resume_token));
            ledger.release(1, resume_reservation)?;
            return Err(DbError::LimitExceeded("database sync hello resume token observed backing"));
        }
        ledger.settle_allocation(resume_reservation, resume_token.capacity(), "database sync hello resume token backing")?;
        let server_frontier = protocol::RuntimeFrontierSummary {
            document_id: protocol::ArtifactId(std::mem::take(&mut server.document.0)),
            head_edit_ordinal: server.head_seq,
            head_edit_id,
            last_commit_seq: server.commit_seq,
            chain_hash: server.chain_hash,
        };
        let session_id = std::mem::take(&mut owners.session_id);
        let welcome = protocol::ServerFrame::Welcome { session_id, resume_token, server_frontier, bootstrap };
        Ok(DatabaseSyncHelloPrepared { welcome: Some(welcome), welcome_close: None, follow_up })
    }
    .await;
    let terminal = match &run {
        Ok(_) => DatabaseSyncHelloProgress::Streaming,
        Err(DbError::Closed) | Err(DbError::Timeout(_)) => DatabaseSyncHelloProgress::Cancelled,
        Err(_) => DatabaseSyncHelloProgress::Fault,
    };
    progress.store(terminal as u8, std::sync::atomic::Ordering::Release);
    DatabaseSyncHelloExecution { owners: Some(owners), prepared: run, ledger: Some(ledger), close_fault: None }
}

pub type DatabaseSyncHelloExecutionFuture = std::pin::Pin<Box<dyn std::future::Future<Output = DatabaseSyncHelloExecution> + Send + 'static>>;

struct DatabaseSyncHelloFollowUpCloseFault {
    pub error: Option<DbError>,
    pub attempts: u8,
}

impl DatabaseSyncHelloFollowUpCloseFault {
    fn retain(&mut self, error: DbError) {
        if self.error.is_none() {
            self.error = Some(error);
        }
        self.attempts = self.attempts.saturating_add(1);
    }

    pub fn close_one(&mut self) -> bool {
        self.error.take().is_some()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.error.is_none()
    }
}

pub fn database_sync_hello_apply_follow_up_close_result(fault: &mut Option<DatabaseSyncHelloFollowUpCloseFault>, terminal_is_empty: bool, result: Result<bool, DbError>) -> bool {
    match result {
        Ok(pending) => pending || !terminal_is_empty,
        Err(error) => {
            let mut retained = DatabaseSyncHelloFollowUpCloseFault { error: None, attempts: 0 };
            retained.retain(error);
            *fault = Some(retained);
            true
        }
    }
}

pub struct DatabaseSyncHelloQuarantineClose {
    pub(super) future: Option<DatabaseSyncHelloExecutionFuture>,
    pub(super) items: usize,
    pub(super) bytes: usize,
}

pub struct DatabaseSyncHelloReturnedFrameLease {
    pub generation: u64,
    pub items: usize,
    pub bytes: usize,
    pub close: Option<DatabaseSyncHelloFrameClose>,
}

impl DatabaseSyncHelloQuarantineClose {
    pub fn close_one(&mut self) -> bool {
        if let Some(future) = self.future.take() {
            drop(future);
            self.items = self.items.saturating_sub(1);
            self.bytes = 0;
            return true;
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.future.is_none() && self.items == 0 && self.bytes == 0
    }
}
//#endregion 👋️RetainedHello
//#endregion 🔖️Hello
