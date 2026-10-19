//! 🧭️ Replay routes and policy: the fixed replay capacities, the mounted route identity, the publication policy and kind, the replay faults, steps and close steps and the replay page. See [`super`].

use super::*;

//#region 📜️FixedReplay
pub const JOB_REPLAY_RECORD_CAPACITY: usize = 256;

pub const JOB_REPLAY_RECORD_PAGE_CAPACITY: usize = 32;

pub const JOB_REPLAY_PROCESS_PAGE_CAPACITY: usize = 4_096;

pub const JOB_REPLAY_PAGE_BYTES: usize = semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;

pub(super) const JOB_REPLAY_SCALAR_TRANSFERS: u8 = 12;

static JOB_REPLAY_PROCESS_PAGES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// 🧭️ Versioned mounted route identity shared by capture and replay.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct JobReplayRoute {
    pub plugin: [u8; 32],
    pub package: [u8; 32],
    pub controller: [u8; 32],
    pub tool: [u8; 32],
    pub window: u64,
    pub artifact: [u8; 32],
    pub request_schema: [u8; 32],
    pub request_version: u16,
    pub request_digest: [u8; 32],
}

impl JobReplayRoute {
    pub const fn zeroed() -> Self {
        Self { plugin: [0; 32], package: [0; 32], controller: [0; 32], tool: [0; 32], window: 0, artifact: [0; 32], request_schema: [0; 32], request_version: 0, request_digest: [0; 32] }
    }
}

/// 🎯️ Observable P2d disposition recorded after the exact owner crosses the overlay boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub enum JobReplayPublicationPolicy {
    Pending,
    Accepted,
    Displaced,
    Rejected,
}

/// 📡️ Typed publication classification retained without cloning its payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub enum JobReplayPublicationKind {
    Yield,
    Preview,
    Checkpoint,
    Commit,
    Cancelled,
    Fault,
}

impl JobReplayPublicationKind {
    pub(super) fn from_outcome(outcome: &JobStepOutcome) -> Self {
        match outcome {
            JobStepOutcome::Yield => Self::Yield,
            JobStepOutcome::PreviewReady { .. } => Self::Preview,
            JobStepOutcome::CheckpointReady { .. } => Self::Checkpoint,
            JobStepOutcome::Complete { .. } => Self::Commit,
            JobStepOutcome::Cancelled => Self::Cancelled,
            JobStepOutcome::Fault { .. } => Self::Fault,
        }
    }
}

/// 🚫️ Non-consuming mounted replay refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobReplayFault {
    Busy,
    Cancelled,
    Deadline,
    Capacity,
    PageBytes,
    ProcessPages,
    Stale,
    Sequence,
    Mismatch,
    Exhausted,
}

/// 🔁️ One bounded capture/replay opportunity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobReplayStep {
    MoreWork,
    PublicationReady,
    Refused(JobReplayFault),
    Complete,
}

/// 🧹️ One exact replay close opportunity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobReplayCloseStep {
    Pending { released_items: usize, released_bytes: usize },
    Complete,
}

/// 🧩️ One admitted immutable replay payload page.
pub(super) struct JobReplayPage {
    storage: ManuallyDrop<Option<Box<[MaybeUninit<u8>; JOB_REPLAY_PAGE_BYTES]>>>,
    pub(super) length: usize,
}

impl JobReplayPage {
    pub(super) fn try_copy(bytes: &[u8]) -> Result<Self, JobReplayFault> {
        if bytes.len() > JOB_REPLAY_PAGE_BYTES {
            return Err(JobReplayFault::PageBytes);
        }
        JOB_REPLAY_PROCESS_PAGES
            .try_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |pages| pages.checked_add(1).filter(|next| *next <= JOB_REPLAY_PROCESS_PAGE_CAPACITY))
            .map_err(|_| JobReplayFault::ProcessPages)?;
        let mut storage = Box::new([MaybeUninit::uninit(); JOB_REPLAY_PAGE_BYTES]);
        for (target, byte) in storage[..bytes.len()].iter_mut().zip(bytes.iter().copied()) {
            target.write(byte);
        }
        Ok(Self { storage: ManuallyDrop::new(Some(storage)), length: bytes.len() })
    }

    pub(super) fn bytes(&self) -> &[u8] {
        let storage = self.storage.as_ref().expect("sealed replay page owns backing");
        unsafe { std::slice::from_raw_parts(storage.as_ptr().cast::<u8>(), self.length) }
    }

    pub(super) fn close(&mut self) -> usize {
        let storage = self.storage.take().expect("replay page closes once");
        drop(storage);
        JOB_REPLAY_PROCESS_PAGES.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        self.length
    }
}

impl Drop for JobReplayPage {
    fn drop(&mut self) {
        if self.storage.is_none() {
            unsafe { ManuallyDrop::drop(&mut self.storage) };
        } else {
            debug_assert!(false, "replay page requires one-page admitted close");
        }
    }
}
//#endregion 📜️FixedReplay
