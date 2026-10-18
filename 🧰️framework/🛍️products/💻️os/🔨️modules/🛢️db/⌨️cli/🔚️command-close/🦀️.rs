//! 🔚️ Command close: the mounted WAL, batch, replay and snapshot command close witnesses with their future and drop impls. See [`super`].

//#region 🔖️CommandClose
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CliCommandCloseExit {
    Closed,
    Fault,
    Interrupted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct CliCommandCloseWitness {
    pub(super) exit: CliCommandCloseExit,
    pub(super) opportunities: u64,
}

pub(super) static CLI_COMMAND_CLOSE_INTERRUPTED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

static CLI_COMMAND_CLOSE_FAULTED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub(super) fn cli_command_close_witness(exit: CliCommandCloseExit, opportunities: u64) -> CliCommandCloseWitness {
    CliCommandCloseWitness { exit, opportunities }
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum CliCommandCloseTerminal {
    Witness(CliCommandCloseWitness),
    Fault(db_ids::DbError),
}

fn cli_command_close_terminal(terminal: &CliCommandCloseTerminal) -> Result<CliCommandCloseWitness, db_ids::DbError> {
    match terminal {
        CliCommandCloseTerminal::Witness(witness) => Ok(*witness),
        CliCommandCloseTerminal::Fault(error) => Err(error.clone()),
    }
}

/// 🧹️ Mounted WAL-record close state; one poll owns one close opportunity.
pub(super) struct MountedWalRecordCommandClose {
    pub(super) owner: Option<db_wal_log::WalRecord>,
    pub(super) opportunities: u64,
    pub(super) terminal: Option<CliCommandCloseTerminal>,
}

impl MountedWalRecordCommandClose {
    pub(super) fn new(owner: db_wal_log::WalRecord) -> Self {
        Self { owner: Some(owner), opportunities: 0, terminal: None }
    }
}

impl std::future::Future for MountedWalRecordCommandClose {
    type Output = Result<CliCommandCloseWitness, db_ids::DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        self.opportunities += 1;
        if self.owner.is_none() {
            let interrupted = cli_command_close_witness(CliCommandCloseExit::Interrupted, self.opportunities);
            let terminal = self.terminal.get_or_insert(CliCommandCloseTerminal::Witness(interrupted));
            return std::task::Poll::Ready(cli_command_close_terminal(terminal));
        }
        let owner = self.owner.as_mut().ok_or_else(|| db_ids::DbError::Internal("WAL record close lost its retained owner".to_string()))?;
        match owner.close_step() {
            Ok(true) => {
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            Ok(false) => {
                self.owner = None;
                let witness = cli_command_close_witness(CliCommandCloseExit::Closed, self.opportunities);
                self.terminal = Some(CliCommandCloseTerminal::Witness(witness));
                std::task::Poll::Ready(Ok(witness))
            }
            Err(error) => {
                self.owner = None;
                self.terminal = Some(CliCommandCloseTerminal::Fault(error.clone()));
                CLI_COMMAND_CLOSE_FAULTED.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                std::task::Poll::Ready(Err(error))
            }
        }
    }
}

impl Drop for MountedWalRecordCommandClose {
    fn drop(&mut self) {
        if self.owner.take().is_some() {
            let _ = cli_command_close_witness(CliCommandCloseExit::Interrupted, self.opportunities);
            CLI_COMMAND_CLOSE_INTERRUPTED.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        }
    }
}

/// 🧰️ Mounted WAL-batch close state; the batch cursor retires one record owner per poll.
pub(super) struct MountedWalBatchCommandClose {
    pub(super) owner: Option<db_wal_log::WalRecordBatch>,
    pub(super) opportunities: u64,
    pub(super) terminal: Option<CliCommandCloseTerminal>,
}

impl MountedWalBatchCommandClose {
    pub(super) fn new(owner: db_wal_log::WalRecordBatch) -> Self {
        Self { owner: Some(owner), opportunities: 0, terminal: None }
    }
}

impl std::future::Future for MountedWalBatchCommandClose {
    type Output = Result<CliCommandCloseWitness, db_ids::DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        self.opportunities += 1;
        if self.owner.is_none() {
            let interrupted = cli_command_close_witness(CliCommandCloseExit::Interrupted, self.opportunities);
            let terminal = self.terminal.get_or_insert(CliCommandCloseTerminal::Witness(interrupted));
            return std::task::Poll::Ready(cli_command_close_terminal(terminal));
        }
        let owner = self.owner.as_mut().ok_or_else(|| db_ids::DbError::Internal("WAL batch close lost its retained owner".to_string()))?;
        match owner.close_step() {
            Ok(true) => {
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            Ok(false) => {
                self.owner = None;
                let witness = cli_command_close_witness(CliCommandCloseExit::Closed, self.opportunities);
                self.terminal = Some(CliCommandCloseTerminal::Witness(witness));
                std::task::Poll::Ready(Ok(witness))
            }
            Err(error) => {
                self.owner = None;
                self.terminal = Some(CliCommandCloseTerminal::Fault(error.clone()));
                CLI_COMMAND_CLOSE_FAULTED.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                std::task::Poll::Ready(Err(error))
            }
        }
    }
}

impl Drop for MountedWalBatchCommandClose {
    fn drop(&mut self) {
        if self.owner.take().is_some() {
            let _ = cli_command_close_witness(CliCommandCloseExit::Interrupted, self.opportunities);
            CLI_COMMAND_CLOSE_INTERRUPTED.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        }
    }
}

/// 📼️ Mounted replay-cursor close state with one synchronous owner step per poll.
pub(super) struct MountedWalReplayCommandClose<'storage> {
    pub(super) owner: Option<MountedWalReplayCommandOwner<'storage>>,
    pub(super) opportunities: u64,
    pub(super) terminal: Option<CliCommandCloseTerminal>,
}

enum MountedWalReplayCommandOwner<'storage> {
    Raw(db_wal_log::WalReplayCursor<'storage, db_storage_fs::FsStorage>),
    Committed(db_wal_log::WalCommittedCursor<'storage, db_storage_fs::FsStorage>),
}

impl MountedWalReplayCommandOwner<'_> {
    fn close_owner_step(&mut self) -> Result<bool, db_ids::DbError> {
        match self {
            Self::Raw(owner) => owner.close_owner_step(),
            Self::Committed(owner) => owner.close_owner_step(),
        }
    }
}

impl<'storage> MountedWalReplayCommandClose<'storage> {
    pub(super) fn new(owner: db_wal_log::WalReplayCursor<'storage, db_storage_fs::FsStorage>) -> Self {
        Self { owner: Some(MountedWalReplayCommandOwner::Raw(owner)), opportunities: 0, terminal: None }
    }

    pub(super) fn committed(owner: db_wal_log::WalCommittedCursor<'storage, db_storage_fs::FsStorage>) -> Self {
        Self { owner: Some(MountedWalReplayCommandOwner::Committed(owner)), opportunities: 0, terminal: None }
    }
}

impl std::future::Future for MountedWalReplayCommandClose<'_> {
    type Output = Result<CliCommandCloseWitness, db_ids::DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        self.opportunities += 1;
        if self.owner.is_none() {
            let interrupted = cli_command_close_witness(CliCommandCloseExit::Interrupted, self.opportunities);
            let terminal = self.terminal.get_or_insert(CliCommandCloseTerminal::Witness(interrupted));
            return std::task::Poll::Ready(cli_command_close_terminal(terminal));
        }
        let owner = self.owner.as_mut().ok_or_else(|| db_ids::DbError::Internal("WAL replay close lost its retained owner".to_string()))?;
        match owner.close_owner_step() {
            Ok(true) => {
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            Ok(false) => {
                self.owner = None;
                let witness = cli_command_close_witness(CliCommandCloseExit::Closed, self.opportunities);
                self.terminal = Some(CliCommandCloseTerminal::Witness(witness));
                std::task::Poll::Ready(Ok(witness))
            }
            Err(error) => {
                self.owner = None;
                self.terminal = Some(CliCommandCloseTerminal::Fault(error.clone()));
                CLI_COMMAND_CLOSE_FAULTED.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                std::task::Poll::Ready(Err(error))
            }
        }
    }
}

impl Drop for MountedWalReplayCommandClose<'_> {
    fn drop(&mut self) {
        if self.owner.take().is_some() {
            let _ = cli_command_close_witness(CliCommandCloseExit::Interrupted, self.opportunities);
            CLI_COMMAND_CLOSE_INTERRUPTED.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        }
    }
}

/// 📸️ Mounted snapshot-cursor close state with an exact terminal exit witness.
pub(super) struct MountedSnapshotCommandClose<'manager, 'storage> {
    pub(super) owner: Option<db_snapshot::SnapshotChainCursor<'manager, 'storage, db_storage_fs::FsStorage>>,
    pub(super) opportunities: u64,
    pub(super) terminal: Option<CliCommandCloseTerminal>,
}

impl<'manager, 'storage> MountedSnapshotCommandClose<'manager, 'storage> {
    pub(super) fn new(owner: db_snapshot::SnapshotChainCursor<'manager, 'storage, db_storage_fs::FsStorage>) -> Self {
        Self { owner: Some(owner), opportunities: 0, terminal: None }
    }
}

impl std::future::Future for MountedSnapshotCommandClose<'_, '_> {
    type Output = Result<CliCommandCloseWitness, db_ids::DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        self.opportunities += 1;
        if self.owner.is_none() {
            let interrupted = cli_command_close_witness(CliCommandCloseExit::Interrupted, self.opportunities);
            let terminal = self.terminal.get_or_insert(CliCommandCloseTerminal::Witness(interrupted));
            return std::task::Poll::Ready(cli_command_close_terminal(terminal));
        }
        let owner = self.owner.as_mut().ok_or_else(|| db_ids::DbError::Internal("snapshot close lost its retained owner".to_string()))?;
        match owner.close_step() {
            Ok(true) => {
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            Ok(false) => {
                self.owner = None;
                let witness = cli_command_close_witness(CliCommandCloseExit::Closed, self.opportunities);
                self.terminal = Some(CliCommandCloseTerminal::Witness(witness));
                std::task::Poll::Ready(Ok(witness))
            }
            Err(error) => {
                self.owner = None;
                self.terminal = Some(CliCommandCloseTerminal::Fault(error.clone()));
                CLI_COMMAND_CLOSE_FAULTED.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                std::task::Poll::Ready(Err(error))
            }
        }
    }
}

impl Drop for MountedSnapshotCommandClose<'_, '_> {
    fn drop(&mut self) {
        if self.owner.take().is_some() {
            let _ = cli_command_close_witness(CliCommandCloseExit::Interrupted, self.opportunities);
            CLI_COMMAND_CLOSE_INTERRUPTED.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        }
    }
}
//#endregion 🔖️CommandClose
