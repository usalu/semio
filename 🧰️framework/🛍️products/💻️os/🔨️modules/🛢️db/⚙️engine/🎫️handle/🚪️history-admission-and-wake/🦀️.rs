//! 🚪️ History admission and wake: the history limits, admission slots, outcome types, registry and wake. See [`super`].

use super::*;

//#region 🔖️ArtifactHandle
pub const ARTIFACT_HISTORY_OPERATION_SLOTS: usize = 8;

const ARTIFACT_HISTORY_PAGE_BYTES: u64 = 16 * 1024;

const ARTIFACT_HISTORY_OPERATION_PAGES: u64 = 2_048;

const ARTIFACT_HISTORY_OPERATION_BYTES: u64 = ARTIFACT_HISTORY_PAGE_BYTES * ARTIFACT_HISTORY_OPERATION_PAGES;

const ARTIFACT_HISTORY_TOTAL_BYTES: u64 = ARTIFACT_HISTORY_OPERATION_BYTES * ARTIFACT_HISTORY_OPERATION_SLOTS as u64;

const ARTIFACT_HISTORY_OPERATION_ITEMS: usize = 20_481;

#[derive(Clone, Copy)]
struct ArtifactHistoryAdmissionSlot {
    pub generation: u64,
    pub bytes: u64,
    pub items: usize,
    pub occupied: bool,
}

const EMPTY_ARTIFACT_HISTORY_SLOT: ArtifactHistoryAdmissionSlot = ArtifactHistoryAdmissionSlot { generation: 0, bytes: 0, items: 0, occupied: false };

struct ArtifactHistoryAdmissionState {
    pub slots: [ArtifactHistoryAdmissionSlot; ARTIFACT_HISTORY_OPERATION_SLOTS],
    pub bytes: u64,
    pub next_generation: u64,
}

pub static ARTIFACT_HISTORY_ADMISSION: Mutex<ArtifactHistoryAdmissionState> = Mutex::new(ArtifactHistoryAdmissionState { slots: [EMPTY_ARTIFACT_HISTORY_SLOT; ARTIFACT_HISTORY_OPERATION_SLOTS], bytes: 0, next_generation: 1 });

pub struct ArtifactHistoryAdmission {
    pub slot: usize,
    pub generation: u64,
    pub reservation: Option<db_artifact_history::HistoryReplayReservation>,
}

pub enum ArtifactHistoryAdmissionError {
    Rejected(DbError),
    Construction { admission: ArtifactHistoryAdmission, fault: db_artifact_history::HistoryReplayReservationConstructionFault },
}

impl std::fmt::Debug for ArtifactHistoryAdmissionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rejected(error) => formatter.debug_tuple("Rejected").field(error).finish(),
            Self::Construction { fault, .. } => formatter.debug_struct("Construction").field("fault", fault).finish(),
        }
    }
}

impl ArtifactHistoryAdmission {
    pub fn try_claim() -> Result<Self, ArtifactHistoryAdmissionError> {
        let (slot, generation) = {
            let mut state = ARTIFACT_HISTORY_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(slot) = state.slots.iter().position(|entry| !entry.occupied) else {
                return Err(ArtifactHistoryAdmissionError::Rejected(DbError::Unavailable("artifact history item capacity exhausted".to_string())));
            };
            let Some(next_bytes) = state.bytes.checked_add(ARTIFACT_HISTORY_OPERATION_BYTES) else {
                return Err(ArtifactHistoryAdmissionError::Rejected(DbError::LimitExceeded("artifact history aggregate bytes")));
            };
            if next_bytes > ARTIFACT_HISTORY_TOTAL_BYTES {
                return Err(ArtifactHistoryAdmissionError::Rejected(DbError::Unavailable("artifact history byte capacity exhausted".to_string())));
            }
            let generation = state.next_generation;
            let Some(next_generation) = state.next_generation.checked_add(1) else {
                return Err(ArtifactHistoryAdmissionError::Rejected(DbError::LimitExceeded("artifact history generation")));
            };
            state.next_generation = next_generation;
            state.slots[slot] = ArtifactHistoryAdmissionSlot { generation, bytes: ARTIFACT_HISTORY_OPERATION_BYTES, items: ARTIFACT_HISTORY_OPERATION_ITEMS, occupied: true };
            state.bytes = next_bytes;
            (slot, generation)
        };
        let mut claim = Self { slot, generation, reservation: None };
        match db_artifact_history::HistoryReplayReservation::try_new() {
            Ok(reservation) => {
                claim.reservation = Some(reservation);
                Ok(claim)
            }
            Err(fault) => Err(ArtifactHistoryAdmissionError::Construction { admission: claim, fault }),
        }
    }

    pub(super) fn take_reservation(&mut self) -> Option<db_artifact_history::HistoryReplayReservation> {
        self.reservation.take()
    }

    pub fn begin_reservation_close(&mut self) -> Option<db_artifact_history::HistoryReplayReservationCloseCursor> {
        self.reservation.take().map(db_artifact_history::HistoryReplayReservationCloseCursor::new)
    }

    pub(super) fn restore_reservation(&mut self, reservation: db_artifact_history::HistoryReplayReservation) {
        assert!(self.reservation.is_none(), "artifact history reservation restore slot was occupied");
        self.reservation = Some(reservation);
    }
}

impl Drop for ArtifactHistoryAdmission {
    fn drop(&mut self) {
        let live = self.reservation.take().is_some();
        {
            let mut state = ARTIFACT_HISTORY_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let entry = &mut state.slots[self.slot];
            if entry.occupied && entry.generation == self.generation && entry.bytes == ARTIFACT_HISTORY_OPERATION_BYTES && entry.items == ARTIFACT_HISTORY_OPERATION_ITEMS {
                *entry = EMPTY_ARTIFACT_HISTORY_SLOT;
                state.bytes = state.bytes.checked_sub(ARTIFACT_HISTORY_OPERATION_BYTES).expect("artifact history byte credit underflow");
            }
        }
        assert!(!live || std::thread::panicking(), "artifact history admission dropped a live replay reservation");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HistoryProgress {
    Admitted,
    Scheduled,
    Waiting,
    Mapping,
    Completed,
    Cancelled,
    Fault,
}

type ArtifactActorHistoryFuture = db_actor::AskFuture<db_artifact_contract::ArtifactMessage, Result<db_artifact_history::ArtifactHistoryView, DbError>>;

pub type ArtifactHistoryOutcome = Result<HistoryView, DbError>;

pub(super) enum ArtifactHistoryWorkOwner {
    Request,
    Actor(ArtifactActorHistoryFuture),
}

pub struct ArtifactHistoryState {
    pub pool: WorkerPool,
    pub authority: Arc<db_artifact::ArtifactAuthority>,
    pub generation: u64,
    pub authority_generation: GenerationId,
    pub admission: Mutex<Option<ArtifactHistoryAdmission>>,
    pub work: Mutex<Option<ArtifactHistoryWorkOwner>>,
    pub completion: Mutex<Option<ArtifactHistoryOutcome>>,
    pub terminal_work: Mutex<Option<ArtifactHistoryWorkOwner>>,
    pub terminal_result: Mutex<Option<ArtifactHistoryOutcome>>,
    pub(super) terminal_reservation: Mutex<Option<db_artifact_history::HistoryReplayReservationCloseCursor>>,
    pub(super) reservation_checked_out: std::sync::atomic::AtomicBool,
    pub(super) terminal_construction: Mutex<Option<db_artifact_history::HistoryReplayReservationConstructionFault>>,
    pub(super) construction_checked_out: std::sync::atomic::AtomicBool,
    pub retry_job: Mutex<Option<(semio_framework_async::Job, u8)>>,
    pub terminal_job: Mutex<Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>>,
    pub waker: Mutex<Option<std::task::Waker>>,
    pub retry_armed: std::sync::atomic::AtomicBool,
    pub retry_generation: std::sync::atomic::AtomicU64,
    pub scheduled: std::sync::atomic::AtomicBool,
    pub cancelled: Arc<std::sync::atomic::AtomicBool>,
    pub abandoned: std::sync::atomic::AtomicBool,
    pub finished: std::sync::atomic::AtomicBool,
    pub progress: std::sync::atomic::AtomicU8,
    #[cfg(test)]
    pub controlled_completion_hook: Mutex<Option<Box<dyn FnOnce(ArtifactHistoryOutcome, HistoryProgress) + Send>>>,
    #[cfg(any(test, feature = "testkit"))]
    pub controlled_publication_before_waker_hook: Mutex<Option<semio_framework_async::Job>>,
}

pub struct HistoryFuture {
    pub state: Arc<ArtifactHistoryState>,
    pub resolved: bool,
}

pub struct ArtifactHistoryTerminalHandle {
    pub state: Arc<ArtifactHistoryState>,
}

pub struct ArtifactHistoryTerminalJob {
    pub state: Arc<ArtifactHistoryState>,
    pub owner: Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>,
}

pub struct ArtifactHistoryTerminalWork {
    pub state: Arc<ArtifactHistoryState>,
    pub owner: Option<ArtifactHistoryWorkOwner>,
}

pub struct ArtifactHistoryTerminalReservation {
    pub state: Arc<ArtifactHistoryState>,
    pub owner: Option<db_artifact_history::HistoryReplayReservationCloseCursor>,
}

pub struct ArtifactHistoryTerminalConstructionFault {
    pub state: Arc<ArtifactHistoryState>,
    pub owner: Option<db_artifact_history::HistoryReplayReservationConstructionFault>,
}

pub(super) struct ArtifactHistoryWake {
    pub state: std::sync::Weak<ArtifactHistoryState>,
    pub generation: u64,
}

pub(super) fn artifact_history_registry() -> &'static Mutex<[Option<Arc<ArtifactHistoryState>>; ARTIFACT_HISTORY_OPERATION_SLOTS]> {
    static REGISTRY: std::sync::OnceLock<Mutex<[Option<Arc<ArtifactHistoryState>>; ARTIFACT_HISTORY_OPERATION_SLOTS]>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(std::array::from_fn(|_| None)))
}

pub(super) fn register_artifact_history(state: &Arc<ArtifactHistoryState>) {
    if state.generation == 0 {
        return;
    }
    let mut registry = artifact_history_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if registry.iter().any(|slot| slot.as_ref().is_some_and(|registered| registered.generation == state.generation)) {
        return;
    }
    if let Some(slot) = registry.iter_mut().find(|slot| slot.is_none()) {
        *slot = Some(state.clone());
    }
}

pub(super) fn unregister_artifact_history(generation: u64) {
    let mut registry = artifact_history_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(slot) = registry.iter_mut().find(|slot| slot.as_ref().is_some_and(|state| state.generation == generation)) {
        *slot = None;
    }
}

impl std::task::Wake for ArtifactHistoryWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if let Some(state) = self.state.upgrade() {
            if self.generation == state.generation {
                state.schedule();
            }
        }
    }
}
//#endregion 🔖️ArtifactHandle
