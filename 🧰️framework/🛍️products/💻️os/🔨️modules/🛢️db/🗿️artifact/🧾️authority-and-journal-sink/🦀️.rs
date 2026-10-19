//! 🧾️ Authority and journal sink: `ArtifactAuthority`, the durable group journal sink and commit values and the build and turn types. See [`super`].

use super::*;

//#region 🔖️Ids
//#region 🔖️Actor
/// 🎭️ A live handle to one document's authority actor. Each admitted mailbox message wakes
/// one finite `WorkerPool` turn; no job waits for the next message and no authority owns an OS
/// thread. `db_state::PMap` uses `Arc`-shared HAMT nodes, making the engine movable between turns
/// without sharing mutable actor state or weakening its single-consumer mailbox semantics.
#[cfg(not(target_arch = "wasm32"))]
pub struct ArtifactAuthority {
    pub(super) address: db_actor::Address<ArtifactMessage>,
    pub cancel: Arc<dyn Fn() + Send + Sync>,
    pub handoff: Arc<ArtifactRunnerHandoff>,
    pub retirement: Option<ArtifactRunnerRetirementReservation>,
    pub(super) retirement_close: Option<Arc<dyn Fn() -> bool + Send + Sync>>,
    pub(super) _pool_use: Arc<semio_framework_async::WorkerPoolUse>,
    pub(super) done: std::sync::Mutex<Option<db_actor::ReplyReceiver<()>>>,
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) struct ArtifactDurableGroupJournalSinkV1 {
    pub(super) address: db_actor::Address<ArtifactMessage>,
    pub now_ms: u64,
}

#[cfg(not(target_arch = "wasm32"))]
enum ArtifactDurableGroupJournalCommitStateV1 {
    NotSubmitted { decision_pack: Vec<u8>, decision_sha256: String },
    Awaiting(Pin<Box<db_actor::AskFuture<ArtifactMessage, Result<ArtifactDurableGroupJournalAppendV1, DbError>>>>),
    Rejected(String),
    Failed(String),
    Absent,
    Committed(store::durable_group::DurableOwnedGroupJournalReceiptV1),
    Empty,
}

#[cfg(not(target_arch = "wasm32"))]
struct ArtifactDurableGroupJournalCommitV1 {
    pub(super) address: Option<db_actor::Address<ArtifactMessage>>,
    pub now_ms: u64,
    pub cancelled: Option<Arc<std::sync::atomic::AtomicBool>>,
    close_started: bool,
    pub state: ArtifactDurableGroupJournalCommitStateV1,
}

#[cfg(not(target_arch = "wasm32"))]
impl store::durable_group::DurableOwnedGroupJournalSinkV1 for ArtifactDurableGroupJournalSinkV1 {
    fn begin_commit(&mut self, decision_pack: Vec<u8>, decision_sha256: String) -> Box<dyn store::durable_group::DurableOwnedGroupJournalCommitV1> {
        Box::new(ArtifactDurableGroupJournalCommitV1 {
            address: Some(self.address.clone()),
            now_ms: self.now_ms,
            cancelled: Some(Arc::new(std::sync::atomic::AtomicBool::new(false))),
            close_started: false,
            state: ArtifactDurableGroupJournalCommitStateV1::NotSubmitted { decision_pack, decision_sha256 },
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl store::durable_group::DurableOwnedGroupJournalCommitV1 for ArtifactDurableGroupJournalCommitV1 {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::durable_group::DurableOwnedGroupJournalAdvanceV1, String> {
        use store::durable_group::DurableOwnedGroupJournalAdvanceV1;
        if !grant.permits_one() {
            return Ok(DurableOwnedGroupJournalAdvanceV1::Pending);
        }
        if matches!(self.state, ArtifactDurableGroupJournalCommitStateV1::NotSubmitted { .. }) {
            let state = std::mem::replace(&mut self.state, ArtifactDurableGroupJournalCommitStateV1::Empty);
            let ArtifactDurableGroupJournalCommitStateV1::NotSubmitted { decision_pack, decision_sha256 } = state else { unreachable!() };
            if self.cancelled.as_ref().expect("live journal cancellation authority").load(std::sync::atomic::Ordering::Acquire) {
                self.state = ArtifactDurableGroupJournalCommitStateV1::Absent;
            } else if grant.maximum_copy_bytes < decision_pack.len() {
                self.state = ArtifactDurableGroupJournalCommitStateV1::Rejected("durable group journal grant is smaller than the canonical decision".to_string());
            } else {
                match store::durable_group::DurableOwnedGroupJournalRecordV1::admit(decision_pack, &decision_sha256) {
                    Ok(record) => {
                        let cancelled = Arc::clone(self.cancelled.as_ref().expect("live journal cancellation authority"));
                        let future = self.address.as_ref().expect("live journal actor authority").ask(Priority::Command, |reply| ArtifactMessage::AppendDurableGroupDecision { record, cancelled, now_ms: self.now_ms, reply });
                        self.state = ArtifactDurableGroupJournalCommitStateV1::Awaiting(Box::pin(future));
                    }
                    Err(error) => self.state = ArtifactDurableGroupJournalCommitStateV1::Rejected(error.to_string()),
                }
            }
        }
        if let ArtifactDurableGroupJournalCommitStateV1::Awaiting(future) = &mut self.state {
            let polled = future.as_mut().poll(&mut std::task::Context::from_waker(std::task::Waker::noop()));
            if let std::task::Poll::Ready(result) = polled {
                self.state = match result {
                    Ok(Ok(ArtifactDurableGroupJournalAppendV1::Absent)) => ArtifactDurableGroupJournalCommitStateV1::Absent,
                    Ok(Ok(ArtifactDurableGroupJournalAppendV1::Rejected(error))) => ArtifactDurableGroupJournalCommitStateV1::Rejected(error.to_string()),
                    Ok(Ok(ArtifactDurableGroupJournalAppendV1::Committed(receipt))) => ArtifactDurableGroupJournalCommitStateV1::Committed(receipt),
                    Ok(Err(error)) | Err(error) => ArtifactDurableGroupJournalCommitStateV1::Failed(error.to_string()),
                };
            }
        }
        match &self.state {
            ArtifactDurableGroupJournalCommitStateV1::NotSubmitted { .. } | ArtifactDurableGroupJournalCommitStateV1::Awaiting(_) => Ok(DurableOwnedGroupJournalAdvanceV1::Pending),
            ArtifactDurableGroupJournalCommitStateV1::Rejected(reason) => Ok(DurableOwnedGroupJournalAdvanceV1::Rejected(reason.clone())),
            ArtifactDurableGroupJournalCommitStateV1::Failed(reason) => Err(reason.clone()),
            ArtifactDurableGroupJournalCommitStateV1::Absent => Ok(DurableOwnedGroupJournalAdvanceV1::Absent),
            ArtifactDurableGroupJournalCommitStateV1::Committed(receipt) => Ok(DurableOwnedGroupJournalAdvanceV1::Committed(receipt.clone())),
            ArtifactDurableGroupJournalCommitStateV1::Empty => Err("durable group journal owner is closed".to_string()),
        }
    }

    fn cancel(&mut self) {
        self.cancelled.as_ref().expect("live journal cancellation authority").store(true, std::sync::atomic::Ordering::Release);
    }

    fn begin_close(&mut self) {
        self.close_started = true;
    }

    fn retirement_demands(&self, _body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
        let release_bytes = match &self.state {
            ArtifactDurableGroupJournalCommitStateV1::NotSubmitted { decision_pack, decision_sha256 } => if decision_pack.capacity() != 0 { decision_pack.capacity() } else { decision_sha256.capacity() },
            ArtifactDurableGroupJournalCommitStateV1::Rejected(message) => message.capacity(),
            ArtifactDurableGroupJournalCommitStateV1::Committed(receipt) => if receipt.anchor_sha256.capacity() != 0 { receipt.anchor_sha256.capacity() } else { receipt.decision_sha256.capacity() },
            ArtifactDurableGroupJournalCommitStateV1::Awaiting(_) | ArtifactDurableGroupJournalCommitStateV1::Failed(_) => return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "uncertain durable journal retains its original resolution owner")),
            ArtifactDurableGroupJournalCommitStateV1::Absent => 0,
            ArtifactDurableGroupJournalCommitStateV1::Empty if self.address.is_some() || self.cancelled.is_some() => return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "durable journal retains its original actor and cancellation capability")),
            ArtifactDurableGroupJournalCommitStateV1::Empty => 0,
        };
        Ok(RetirementDemand { release_bytes, depth: usize::from(!semio_framework_value::ErasedSnapshotRetirement::terminal_is_empty(self)), ..Default::default() })
    }

 }

#[cfg(not(target_arch = "wasm32"))]
impl semio_framework_value::ErasedSnapshotRetirement for ArtifactDurableGroupJournalCommitV1 {
        fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { store::durable_group::DurableOwnedGroupJournalCommitV1::retirement_demands(self, body) }
fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(store::durable_group::DurableOwnedGroupJournalCommitV1::retirement_demands(self, 0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(store::durable_group::DurableOwnedGroupJournalCommitV1::retirement_demands(self, body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(store::durable_group::DurableOwnedGroupJournalCommitV1::retirement_demands(self, 0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(store::durable_group::DurableOwnedGroupJournalCommitV1::retirement_demands(self, 0)?.depth) }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if !self.close_started || grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let demand = store::durable_group::DurableOwnedGroupJournalCommitV1::retirement_demands(self, grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        match &mut self.state {
            ArtifactDurableGroupJournalCommitStateV1::NotSubmitted { decision_pack, decision_sha256 } if decision_pack.capacity() != 0 => drop(std::mem::take(decision_pack)),
            ArtifactDurableGroupJournalCommitStateV1::NotSubmitted { decision_sha256, .. } if decision_sha256.capacity() != 0 => drop(std::mem::take(decision_sha256)),
            ArtifactDurableGroupJournalCommitStateV1::Rejected(message) if message.capacity() != 0 => drop(std::mem::take(message)),
            ArtifactDurableGroupJournalCommitStateV1::Committed(receipt) if receipt.anchor_sha256.capacity() != 0 => drop(std::mem::take(&mut receipt.anchor_sha256)),
            ArtifactDurableGroupJournalCommitStateV1::Committed(receipt) if receipt.decision_sha256.capacity() != 0 => drop(std::mem::take(&mut receipt.decision_sha256)),
            _ => self.state = ArtifactDurableGroupJournalCommitStateV1::Empty,
        }
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() }))
    }

    fn terminal_is_empty(&self) -> bool { matches!(self.state, ArtifactDurableGroupJournalCommitStateV1::Empty) && self.address.is_none() && self.cancelled.is_none() }

}

/// 🧳️ The runner owns its engine **boxed**, and every future it drives yields the box, never the
/// engine itself. `ArtifactEngine` measured ~529 KB by value (read off the
/// two `catch_unwind` shim frames of ticket 26/09/18 slice HS1's crash reports), and `run_turn`
/// carries each turn's result out through `std::panic::catch_unwind` → `do_call` → its own frame. A
/// debug build gives every one of those moves its own stack slot, so a by-value engine priced
/// `run_turn` at 3,598,608 B and the whole document-mount chain at 8,506,832 B — on pool workers
/// that own 2 MiB. Moving a pointer instead is what keeps the hub's first document socket from
/// aborting the process; see `📓️hs1-hub-pool-worker-stack-overflow.md`.
#[cfg(not(target_arch = "wasm32"))]
pub(super) type ArtifactBuildFuture = Pin<Box<dyn Future<Output = Result<Box<ArtifactEngine>, ArtifactEngineOpenRejected>> + Send + 'static>>;

#[cfg(not(target_arch = "wasm32"))]
type ArtifactTurnFuture = Pin<Box<dyn Future<Output = Box<ArtifactEngine>> + Send + 'static>>;

#[cfg(not(target_arch = "wasm32"))]
pub(super) enum ArtifactTurn {
    Future(ArtifactTurnFuture),
    CloseFlush(ArtifactTurnFuture),
    History { engine: Option<Box<ArtifactEngine>>, replay: HistoryReplayFuture, reply: Option<db_actor::ReplySender<Result<ArtifactHistoryView, DbError>>> },
}
//#endregion 🔖️Actor
//#endregion 🔖️Ids
