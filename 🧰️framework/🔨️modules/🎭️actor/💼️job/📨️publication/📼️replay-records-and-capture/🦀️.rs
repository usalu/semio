//! 📼️ Replay records and capture: the record header, the replay record, the capture with their drops, the rejected capture and the replay schedule. See [`super`].

use super::*;

//#region 📜️FixedReplay
/// 📇️ Scalar identity and digest fields for one deterministic replay turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JobReplayRecordHeader {
    pub actor: ActorId,
    pub turn: JobTurn,
    pub ordinal: u64,
    pub granted_fuel: u64,
    pub deadline_class_ms: u32,
    pub worker_count: u16,
    pub worker_slot: u16,
    pub cancellation_observed: bool,
    pub kind: JobReplayPublicationKind,
    pub policy: JobReplayPublicationPolicy,
    pub applied_progress: u64,
    pub payload_items: u8,
    pub payload_bytes: usize,
    pub payload_digest: u64,
    pub prefix_digest: u64,
}

pub(super) struct JobReplayRecord {
    pub(super) header: JobReplayRecordHeader,
    pub(super) pages: ManuallyDrop<[Option<JobReplayPage>; JOB_REPLAY_RECORD_PAGE_CAPACITY]>,
    pub(super) page_count: usize,
    pub(super) close_cursor: usize,
}

impl JobReplayRecord {
    pub(super) fn new(header: JobReplayRecordHeader) -> Self {
        Self { header, pages: ManuallyDrop::new(std::array::from_fn(|_| None)), page_count: 0, close_cursor: 0 }
    }

    pub(super) fn push_page(&mut self, page: JobReplayPage) -> Result<(), JobReplayPage> {
        let Some(slot) = self.pages.iter_mut().find(|slot| slot.is_none()) else { return Err(page) };
        *slot = Some(page);
        self.page_count += 1;
        Ok(())
    }

    pub(super) fn close_step(&mut self, maximum_bytes: usize) -> JobReplayCloseStep {
        if self.page_count == 0 {
            return JobReplayCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        let index = self.close_cursor % JOB_REPLAY_RECORD_PAGE_CAPACITY;
        self.close_cursor = (self.close_cursor + 1) % JOB_REPLAY_RECORD_PAGE_CAPACITY;
        let Some(page) = self.pages[index].as_mut() else { return JobReplayCloseStep::Pending { released_items: 0, released_bytes: 0 } };
        if page.length > maximum_bytes {
            return JobReplayCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        let released_bytes = page.close();
        self.pages[index] = None;
        self.page_count -= 1;
        JobReplayCloseStep::Pending { released_items: 1, released_bytes }
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.page_count == 0 && self.pages.iter().all(Option::is_none)
    }
}

impl Drop for JobReplayRecord {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            unsafe { ManuallyDrop::drop(&mut self.pages) };
        } else {
            debug_assert!(false, "replay record requires one-page admitted close");
        }
    }
}

pub(super) struct JobReplayCapture {
    pub(super) publication: ManuallyDrop<Option<JobPublication>>,
    pub(super) retirement: ManuallyDrop<Option<JobPublicationOwner>>,
    pub(super) record: ManuallyDrop<Option<JobReplayRecord>>,
    pub(super) scalar_cursor: u8,
    pub(super) payload_item: u8,
    pub(super) payload_offset: usize,
    pub(super) payload_digest: u64,
    pub(super) expected_pages: usize,
    pub(super) sealed: bool,
    pub(super) acknowledged: bool,
}

impl JobReplayCapture {
    pub(super) fn new(publication: JobPublication, header: JobReplayRecordHeader, expected_pages: usize) -> Self {
        Self {
            publication: ManuallyDrop::new(Some(publication)),
            retirement: ManuallyDrop::new(None),
            record: ManuallyDrop::new(Some(JobReplayRecord::new(header))),
            scalar_cursor: 0,
            payload_item: 0,
            payload_offset: 0,
            payload_digest: 0xcbf2_9ce4_8422_2325,
            expected_pages,
            sealed: false,
            acknowledged: false,
        }
    }

    pub(super) fn payload(&self) -> Option<&[u8]> {
        let publication = self.publication.as_ref().expect("pending replay capture owns publication");
        match (&publication.outcome, self.payload_item) {
            (JobStepOutcome::PreviewReady { preview }, 0) => Some(preview),
            (JobStepOutcome::CheckpointReady { checkpoint }, 0) => Some(&checkpoint.state),
            (JobStepOutcome::Complete { candidate }, 0) => Some(&candidate.state),
            (JobStepOutcome::Complete { candidate }, 1) => Some(&candidate.output),
            (JobStepOutcome::Fault { detail }, 0) => Some(detail),
            _ => None,
        }
    }

    pub(super) fn payload_items(&self) -> u8 {
        replay_payload_items(self.publication.as_ref().expect("pending replay capture owns publication")) as u8
    }
}

impl Drop for JobReplayCapture {
    fn drop(&mut self) {
        if self.publication.is_none() && self.retirement.is_none() && self.record.is_none() {
            unsafe {
                ManuallyDrop::drop(&mut self.publication);
                ManuallyDrop::drop(&mut self.retirement);
                ManuallyDrop::drop(&mut self.record);
            }
        } else {
            debug_assert!(false, "replay capture requires exact publication handback and record transfer");
        }
    }
}

/// 📮️ Exact live publication retained when replay capture admission refuses.
#[must_use = "rejected replay capture retains the exact live publication"]
#[derive(Debug)]
pub struct JobReplayRejected {
    pub fault: JobReplayFault,
    pub(super) publication: ManuallyDrop<Option<JobPublication>>,
}

impl JobReplayRejected {
    pub fn into_publication(mut self) -> JobPublication {
        self.publication.take().expect("rejected replay publication returned once")
    }
}

impl Drop for JobReplayRejected {
    fn drop(&mut self) {
        if self.publication.is_none() {
            unsafe { ManuallyDrop::drop(&mut self.publication) };
        } else {
            debug_assert!(false, "rejected replay capture requires exact live-owner handback");
        }
    }
}

/// 📜️ Fixed generation-qualified capture authority for the mounted shard/P2d stream.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JobReplaySchedule {
    pub worker_count: u16,
    pub worker_slot: u16,
    pub granted_fuel: u64,
    pub deadline_class_ms: u32,
}
//#endregion 📜️FixedReplay
