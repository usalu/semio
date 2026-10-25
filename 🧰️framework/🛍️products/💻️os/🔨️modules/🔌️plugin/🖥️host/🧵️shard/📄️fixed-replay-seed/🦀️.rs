//! 📄️ Fixed replay seed: the replay page and slot limits, the replay test authority, the fixed replay seed page and seed, the replay phases, the mounted replay seed, the spawn refusals and the replay abi buffer. See [`super`].

use super::*;

//#region 📄️FixedReplaySeed
pub const JOB_REPLAY_SEED_PAGE_BYTES: usize = semio_framework_actor::JOB_REPLAY_PAGE_BYTES;

pub const JOB_REPLAY_KIND_PAGE_CAPACITY: usize = 16;

const JOB_REPLAY_INPUT_PAGE_CAPACITY: usize = semio_framework_actor::JOB_REPLAY_RECORD_CAPACITY;

const JOB_REPLAY_CHECKPOINT_PAGE_CAPACITY: usize = semio_framework_actor::JOB_REPLAY_RECORD_CAPACITY;

pub const JOB_REPLAY_SEED_SLOT_CAPACITY: usize = SHARD_DEFERRED_ITEMS;

pub const JOB_REPLAY_REFUSAL_SLOT_CAPACITY: usize = 512;

const JOB_REPLAY_SEED_PROCESS_PAGES: usize = semio_framework_actor::JOB_REPLAY_PROCESS_PAGE_CAPACITY;

pub static JOB_REPLAY_SEED_PAGES: AtomicUsize = AtomicUsize::new(0);

pub static JOB_REPLAY_ABI_BYTES: AtomicUsize = AtomicUsize::new(0);

#[cfg(any(test, feature = "testkit"))]
static REPLAY_TEST_AUTHORITY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[cfg(any(test, feature = "testkit"))]
pub struct ReplayTestAuthority;

#[cfg(any(test, feature = "testkit"))]
pub fn replay_test_authority() -> ReplayTestAuthority {
    while REPLAY_TEST_AUTHORITY.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
        std::thread::yield_now();
    }
    ReplayTestAuthority
}

#[cfg(any(test, feature = "testkit"))]
impl Drop for ReplayTestAuthority {
    fn drop(&mut self) {
        REPLAY_TEST_AUTHORITY.store(false, Ordering::Release);
    }
}

pub(super) struct FixedReplaySeedPage {
    storage: Option<Box<[MaybeUninit<u8>; JOB_REPLAY_SEED_PAGE_BYTES]>>,
    pub(super) length: usize,
}

impl FixedReplaySeedPage {
    fn try_copy(bytes: &[u8]) -> Result<Self, ()> {
        if bytes.len() > JOB_REPLAY_SEED_PAGE_BYTES {
            return Err(());
        }
        JOB_REPLAY_SEED_PAGES.try_update(Ordering::AcqRel, Ordering::Acquire, |pages| pages.checked_add(1).filter(|pages| *pages <= JOB_REPLAY_SEED_PROCESS_PAGES)).map_err(|_| ())?;
        let mut storage = Box::new([MaybeUninit::uninit(); JOB_REPLAY_SEED_PAGE_BYTES]);
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), storage.as_mut_ptr().cast::<u8>(), bytes.len()) };
        Ok(Self { storage: Some(storage), length: bytes.len() })
    }

    pub(super) fn bytes(&self) -> &[u8] {
        let storage = self.storage.as_ref().expect("fixed replay seed page owns backing");
        unsafe { std::slice::from_raw_parts(storage.as_ptr().cast::<u8>(), self.length) }
    }
}

impl Drop for FixedReplaySeedPage {
    fn drop(&mut self) {
        if self.storage.take().is_some() {
            JOB_REPLAY_SEED_PAGES.fetch_sub(1, Ordering::AcqRel);
        }
    }
}

pub struct FixedReplaySeed {
    pub request: JobReplayRequest,
    pub(super) kind: [Option<FixedReplaySeedPage>; JOB_REPLAY_KIND_PAGE_CAPACITY],
    pub(super) input: [Option<FixedReplaySeedPage>; JOB_REPLAY_INPUT_PAGE_CAPACITY],
    pub(super) checkpoint: [Option<FixedReplaySeedPage>; JOB_REPLAY_CHECKPOINT_PAGE_CAPACITY],
    pub kind_pages: usize,
    pub(super) input_pages: usize,
    pub(super) checkpoint_pages: usize,
    pub kind_length: usize,
    pub(super) input_length: usize,
    pub(super) checkpoint_length: usize,
    close_cursor: usize,
}

impl FixedReplaySeed {
    pub(super) fn new(request: JobReplayRequest, kind_length: usize, input_length: usize) -> Result<Self, ()> {
        if kind_length.div_ceil(JOB_REPLAY_SEED_PAGE_BYTES) > JOB_REPLAY_KIND_PAGE_CAPACITY || input_length.div_ceil(JOB_REPLAY_SEED_PAGE_BYTES) > JOB_REPLAY_INPUT_PAGE_CAPACITY {
            return Err(());
        }
        Ok(Self {
            request,
            kind: std::array::from_fn(|_| None),
            input: std::array::from_fn(|_| None),
            checkpoint: std::array::from_fn(|_| None),
            kind_pages: 0,
            input_pages: 0,
            checkpoint_pages: 0,
            kind_length,
            input_length,
            checkpoint_length: 0,
            close_cursor: 0,
        })
    }

    pub fn copy_kind_page(&mut self, bytes: &[u8], cursor: &mut usize) -> Result<bool, ()> {
        if *cursor == bytes.len() {
            return Ok(true);
        }
        let end = cursor.checked_add(JOB_REPLAY_SEED_PAGE_BYTES).unwrap_or(bytes.len()).min(bytes.len());
        let page = FixedReplaySeedPage::try_copy(&bytes[*cursor..end])?;
        let slot = self.kind.get_mut(self.kind_pages).ok_or(())?;
        *slot = Some(page);
        self.kind_pages += 1;
        *cursor = end;
        Ok(*cursor == bytes.len())
    }

    pub fn copy_input_page(&mut self, bytes: &[u8], cursor: &mut usize) -> Result<bool, ()> {
        if *cursor == bytes.len() {
            return Ok(true);
        }
        let end = cursor.checked_add(JOB_REPLAY_SEED_PAGE_BYTES).unwrap_or(bytes.len()).min(bytes.len());
        let page = FixedReplaySeedPage::try_copy(&bytes[*cursor..end])?;
        let slot = self.input.get_mut(self.input_pages).ok_or(())?;
        *slot = Some(page);
        self.input_pages += 1;
        *cursor = end;
        Ok(*cursor == bytes.len())
    }

    pub(super) fn copy_checkpoint_page(&mut self, bytes: &[u8], cursor: &mut usize) -> Result<bool, ()> {
        if bytes.len().div_ceil(JOB_REPLAY_SEED_PAGE_BYTES) > JOB_REPLAY_CHECKPOINT_PAGE_CAPACITY {
            return Err(());
        }
        self.checkpoint_length = bytes.len();
        if *cursor == bytes.len() {
            return Ok(true);
        }
        let end = cursor.checked_add(JOB_REPLAY_SEED_PAGE_BYTES).unwrap_or(bytes.len()).min(bytes.len());
        let page = FixedReplaySeedPage::try_copy(&bytes[*cursor..end])?;
        let slot = self.checkpoint.get_mut(self.checkpoint_pages).ok_or(())?;
        *slot = Some(page);
        self.checkpoint_pages += 1;
        *cursor = end;
        Ok(*cursor == bytes.len())
    }

    pub(super) fn close_one(&mut self) -> bool {
        let total = JOB_REPLAY_KIND_PAGE_CAPACITY + JOB_REPLAY_INPUT_PAGE_CAPACITY + JOB_REPLAY_CHECKPOINT_PAGE_CAPACITY;
        for offset in 0..total {
            let index = (self.close_cursor + offset) % total;
            let page = if index < JOB_REPLAY_KIND_PAGE_CAPACITY {
                self.kind[index].take()
            } else if index < JOB_REPLAY_KIND_PAGE_CAPACITY + JOB_REPLAY_INPUT_PAGE_CAPACITY {
                self.input[index - JOB_REPLAY_KIND_PAGE_CAPACITY].take()
            } else {
                self.checkpoint[index - JOB_REPLAY_KIND_PAGE_CAPACITY - JOB_REPLAY_INPUT_PAGE_CAPACITY].take()
            };
            let Some(page) = page else { continue };
            drop(page);
            if index < JOB_REPLAY_KIND_PAGE_CAPACITY {
                self.kind_pages -= 1;
            } else if index < JOB_REPLAY_KIND_PAGE_CAPACITY + JOB_REPLAY_INPUT_PAGE_CAPACITY {
                self.input_pages -= 1;
            } else {
                self.checkpoint_pages -= 1;
            }
            self.close_cursor = (index + 1) % total;
            return false;
        }
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReplaySeedPhase {
    CaptureKind,
    CaptureInput,
    Checkpoint,
    CaptureCheckpoint,
    RetireCheckpointOwner,
    Start,
    RetireKindOwner,
    ActivateAuthority,
    ActivateTurn,
    ActivateRunning,
    ActivatePlacement,
    ActivateReady,
    Retained,
    MaterializeKind,
    MaterializeInput,
    MaterializeCheckpoint,
    Restore,
    RetireMaterializedCheckpoint,
    PrepareReplayKind,
    Restart,
    RetireReplayKind,
    Closing,
    RetireSeedShell,
    RetireMountedShell,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReplaySeedCloseReason {
    Completed,
    Cancelled,
    ActorLost,
    Fault { stage: &'static str, detail: String },
}

/// 🌱️ One admitted `Effect::SpawnJob` from capture to its end. A `replayable` seed captures the
/// spawning instance's checkpoint before it starts the job, so a host can later replay it on another
/// worker; a framework reserved tool job (`semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND`) is
/// live-only and starts right after its kind and input pages — its hosts never replay it, and a whole
/// guest checkpoint outgrows the fixed checkpoint pages (block2d's undo, ticket 26/09/23 slice WG8 §1.4).
pub struct MountedReplaySeed {
    pub actor: u64,
    pub job: u64,
    pub authority: JobTurn,
    pub placement: JobPlacement,
    pub(super) replayable: bool,
    pub(super) worker_count: u16,
    pub(super) worker_slot: u16,
    pub phase: ReplaySeedPhase,
    pub close_reason: Option<ReplaySeedCloseReason>,
    pub seed: Option<FixedReplaySeed>,
    pub kind_owner: Option<String>,
    pub input_owner: Option<Vec<u8>>,
    pub(super) checkpoint_owner: Option<Vec<u8>>,
    pub kind_cursor: usize,
    pub input_cursor: usize,
    pub(super) checkpoint_cursor: usize,
    pub materialized_kind: Option<Vec<u8>>,
    pub(super) materialized_input: Option<Vec<u8>>,
    pub(super) materialized_checkpoint: Option<Vec<u8>>,
    pub(super) replay_kind_owner: Option<String>,
    pub(super) materialize_page: usize,
    pub abi_reserved: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ReplaySpawnRefusalPhase {
    RetireInput,
    RetireKind,
    Publish,
    RetireShell,
}

pub struct ReplaySpawnRefusal {
    pub actor: u64,
    pub job: u64,
    pub reason: &'static [u8],
    pub input: Option<Vec<u8>>,
    pub kind: Option<String>,
    pub phase: ReplaySpawnRefusalPhase,
    pub publish: bool,
}

impl ReplaySpawnRefusal {
    pub fn new(actor: u64, job: u64, reason: &'static [u8], kind: String, input: Vec<u8>) -> Self {
        Self { actor, job, reason, input: Some(input), kind: Some(kind), phase: ReplaySpawnRefusalPhase::RetireInput, publish: true }
    }
}

impl MountedReplaySeed {
    pub fn new(actor: u64, job: u64, authority: JobTurn, request: JobReplayRequest, placement: JobPlacement, kind: String, input: Vec<u8>) -> Result<Self, (String, Vec<u8>)> {
        let seed = match FixedReplaySeed::new(request, kind.len(), input.len()) {
            Ok(seed) => seed,
            Err(()) => return Err((kind, input)),
        };
        Ok(Self {
            actor,
            job,
            authority,
            placement,
            replayable: kind != semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND,
            worker_count: 0,
            worker_slot: 0,
            phase: ReplaySeedPhase::CaptureKind,
            close_reason: None,
            seed: Some(seed),
            kind_owner: Some(kind),
            input_owner: Some(input),
            checkpoint_owner: None,
            kind_cursor: 0,
            input_cursor: 0,
            checkpoint_cursor: 0,
            materialized_kind: None,
            materialized_input: None,
            materialized_checkpoint: None,
            replay_kind_owner: None,
            materialize_page: 0,
            abi_reserved: 0,
        })
    }

    pub(super) fn release_abi(&mut self, bytes: usize) -> Result<(), PluginHostError> {
        let remaining = self.abi_reserved.checked_sub(bytes).ok_or_else(|| PluginHostError::Plugin("ShardLoop::replay: local ABI accounting underflow".into()))?;
        JOB_REPLAY_ABI_BYTES.try_update(Ordering::AcqRel, Ordering::Acquire, |owned| owned.checked_sub(bytes)).map_err(|_| PluginHostError::Plugin("ShardLoop::replay: process ABI accounting underflow".into()))?;
        self.abi_reserved = remaining;
        Ok(())
    }
}

impl Drop for MountedReplaySeed {
    fn drop(&mut self) {
        let bytes = std::mem::take(&mut self.abi_reserved);
        if bytes != 0 {
            JOB_REPLAY_ABI_BYTES.fetch_sub(bytes, Ordering::AcqRel);
        }
    }
}

pub fn try_replay_abi_buffer(bytes: usize) -> Result<Vec<u8>, ()> {
    JOB_REPLAY_ABI_BYTES.try_update(Ordering::AcqRel, Ordering::Acquire, |owned| owned.checked_add(bytes).filter(|owned| *owned <= SHARD_DEFERRED_BYTES)).map_err(|_| ())?;
    let mut buffer = Vec::new();
    if buffer.try_reserve_exact(bytes).is_err() {
        JOB_REPLAY_ABI_BYTES.fetch_sub(bytes, Ordering::AcqRel);
        return Err(());
    }
    Ok(buffer)
}
//#endregion 📄️FixedReplaySeed
