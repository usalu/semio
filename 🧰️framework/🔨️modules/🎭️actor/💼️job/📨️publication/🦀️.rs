//! 📨️ Retained Actor Job publications own replay pages, progress checkouts and their exact acknowledgement.
use std::mem::{ManuallyDrop,MaybeUninit};
use semio_framework_value_derive::{FromValue,ToValue};
use semio_framework_actor_vocabulary::*;
pub use semio_framework_actor_pack as pack;
pub use semio_framework_job as job;
//#region 📜️FixedReplay
pub const JOB_REPLAY_RECORD_CAPACITY: usize = 256;
pub const JOB_REPLAY_RECORD_PAGE_CAPACITY: usize = 32;
pub const JOB_REPLAY_PROCESS_PAGE_CAPACITY: usize = 4_096;
pub const JOB_REPLAY_PAGE_BYTES: usize = job::JOB_PAYLOAD_PAGE_BYTES;
const JOB_REPLAY_SCALAR_TRANSFERS: u8 = 12;

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
    fn from_outcome(outcome: &JobStepOutcome) -> Self {
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
struct JobReplayPage {
    storage: ManuallyDrop<Option<Box<[MaybeUninit<u8>; JOB_REPLAY_PAGE_BYTES]>>>,
    length: usize,
}

impl JobReplayPage {
    fn try_copy(bytes: &[u8]) -> Result<Self, JobReplayFault> {
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

    fn bytes(&self) -> &[u8] {
        let storage = self.storage.as_ref().expect("sealed replay page owns backing");
        unsafe { std::slice::from_raw_parts(storage.as_ptr().cast::<u8>(), self.length) }
    }

    fn close(&mut self) -> usize {
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

struct JobReplayRecord {
    header: JobReplayRecordHeader,
    pages: ManuallyDrop<[Option<JobReplayPage>; JOB_REPLAY_RECORD_PAGE_CAPACITY]>,
    page_count: usize,
    close_cursor: usize,
}

impl JobReplayRecord {
    fn new(header: JobReplayRecordHeader) -> Self {
        Self { header, pages: ManuallyDrop::new(std::array::from_fn(|_| None)), page_count: 0, close_cursor: 0 }
    }

    fn push_page(&mut self, page: JobReplayPage) -> Result<(), JobReplayPage> {
        let Some(slot) = self.pages.iter_mut().find(|slot| slot.is_none()) else { return Err(page) };
        *slot = Some(page);
        self.page_count += 1;
        Ok(())
    }

    fn close_step(&mut self, maximum_bytes: usize) -> JobReplayCloseStep {
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

    fn terminal_is_empty(&self) -> bool {
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

struct JobReplayCapture {
    publication: ManuallyDrop<Option<JobPublication>>,
    retirement: ManuallyDrop<Option<JobPublicationOwner>>,
    record: ManuallyDrop<Option<JobReplayRecord>>,
    scalar_cursor: u8,
    payload_item: u8,
    payload_offset: usize,
    payload_digest: u64,
    expected_pages: usize,
    sealed: bool,
    acknowledged: bool,
}

impl JobReplayCapture {
    fn new(publication: JobPublication, header: JobReplayRecordHeader, expected_pages: usize) -> Self {
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

    fn payload(&self) -> Option<&[u8]> {
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

    fn payload_items(&self) -> u8 {
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
    publication: ManuallyDrop<Option<JobPublication>>,
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

pub struct JobReplayLog {
    route: JobReplayRoute,
    generation: u64,
    records: ManuallyDrop<Box<[Option<JobReplayRecord>; JOB_REPLAY_RECORD_CAPACITY]>>,
    record_count: usize,
    reserved_records: usize,
    reserved_pages: usize,
    prefix_digest: u64,
    next_ordinal: u64,
    last_checkpoint_index: Option<usize>,
    pending: ManuallyDrop<Option<JobReplayCapture>>,
    ready_record: Option<usize>,
    replay_cursor: usize,
    replaying: bool,
    closing: bool,
    close_cursor: usize,
}

impl JobReplayLog {
    pub fn new(route: JobReplayRoute, generation: u64) -> Result<Self, JobReplayFault> {
        if generation == 0 {
            return Err(JobReplayFault::Stale);
        }
        Ok(Self {
            route,
            generation,
            records: ManuallyDrop::new(semio_framework_async::boxed_fixed_slots(|| None)),
            record_count: 0,
            reserved_records: 0,
            reserved_pages: 0,
            prefix_digest: 0xcbf2_9ce4_8422_2325,
            next_ordinal: 0,
            last_checkpoint_index: None,
            pending: ManuallyDrop::new(None),
            ready_record: None,
            replay_cursor: 0,
            replaying: false,
            closing: false,
            close_cursor: 0,
        })
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn sealed_records(&self) -> usize {
        self.record_count
    }

    pub fn route(&self) -> JobReplayRoute {
        self.route
    }

    pub fn prefix_digest(&self) -> u64 {
        self.prefix_digest
    }

    pub fn record_header(&self, index: usize) -> Option<JobReplayRecordHeader> {
        self.records.get(index).and_then(Option::as_ref).map(|record| record.header)
    }

    pub fn last_checkpoint_header(&self) -> Option<JobReplayRecordHeader> {
        self.last_checkpoint_index.and_then(|index| self.record_header(index))
    }

    pub fn begin_capture(&mut self, cx: &mut job::StepContext<'_>, actor: ActorId, schedule: JobReplaySchedule, publication: JobPublication) -> Result<(), JobReplayRejected> {
        let JobReplaySchedule { worker_count, worker_slot, granted_fuel, deadline_class_ms } = schedule;
        if self.closing || cx.is_cancelled() {
            return Err(replay_rejected(JobReplayFault::Cancelled, publication));
        }
        if cx.should_yield() {
            return Err(replay_rejected(JobReplayFault::Deadline, publication));
        }
        if self.pending.is_some() || self.ready_record.is_some() {
            return Err(replay_rejected(JobReplayFault::Busy, publication));
        }
        if cx.generation().0 != publication.turn.operation.generation || cx.operation().0 != publication.turn.operation.operation || publication.turn.operation.generation != self.generation {
            return Err(replay_rejected(JobReplayFault::Stale, publication));
        }
        if self.next_ordinal == u64::MAX {
            return Err(replay_rejected(JobReplayFault::Exhausted, publication));
        }
        let Some(payload_bytes) = replay_payload_bytes(&publication) else { return Err(replay_rejected(JobReplayFault::Capacity, publication)) };
        let payload_items = replay_payload_items(&publication);
        let expected_pages = replay_payload_pages(&publication);
        if expected_pages > JOB_REPLAY_RECORD_PAGE_CAPACITY || self.record_count + self.reserved_records >= JOB_REPLAY_RECORD_CAPACITY {
            return Err(replay_rejected(JobReplayFault::Capacity, publication));
        }
        cx.consume_fuel(1);
        let header = JobReplayRecordHeader {
            actor,
            turn: publication.turn,
            ordinal: self.next_ordinal,
            granted_fuel,
            deadline_class_ms,
            worker_count,
            worker_slot,
            cancellation_observed: matches!(publication.outcome, JobStepOutcome::Cancelled),
            kind: JobReplayPublicationKind::from_outcome(&publication.outcome),
            policy: JobReplayPublicationPolicy::Pending,
            applied_progress: match &publication.outcome {
                JobStepOutcome::CheckpointReady { checkpoint } => checkpoint.applied_progress,
                _ => publication.turn.step_sequence,
            },
            payload_items: payload_items as u8,
            payload_bytes,
            payload_digest: 0,
            prefix_digest: 0,
        };
        self.reserved_records += 1;
        self.reserved_pages += expected_pages;
        *self.pending = Some(JobReplayCapture::new(publication, header, expected_pages));
        Ok(())
    }

    pub fn capture_step(&mut self, cx: &mut job::StepContext<'_>) -> JobReplayStep {
        if self.pending.is_none() {
            return if self.ready_record.is_some() { JobReplayStep::PublicationReady } else { JobReplayStep::Complete };
        }
        if cx.is_cancelled() {
            return JobReplayStep::Refused(JobReplayFault::Cancelled);
        }
        if cx.should_yield() {
            return JobReplayStep::MoreWork;
        }
        let pending = self.pending.as_mut().expect("capture pending");
        if pending.acknowledged {
            return JobReplayStep::MoreWork;
        }
        if pending.scalar_cursor < JOB_REPLAY_SCALAR_TRANSFERS {
            cx.consume_fuel(1);
            pending.scalar_cursor += 1;
            return JobReplayStep::MoreWork;
        }
        if pending.payload_item < pending.payload_items() {
            let Some(payload) = pending.payload() else { return JobReplayStep::Refused(JobReplayFault::Mismatch) };
            if pending.payload_offset == payload.len() {
                cx.consume_fuel(1);
                pending.payload_item += 1;
                pending.payload_offset = 0;
                return JobReplayStep::MoreWork;
            }
            let end = pending.payload_offset.saturating_add(JOB_REPLAY_PAGE_BYTES).min(payload.len());
            cx.consume_fuel(1);
            if cx.is_cancelled() || cx.deadline_exceeded() {
                return JobReplayStep::Refused(if cx.is_cancelled() { JobReplayFault::Cancelled } else { JobReplayFault::Deadline });
            }
            let (page, payload_digest) = {
                let chunk = &payload[pending.payload_offset..end];
                let page = match JobReplayPage::try_copy(chunk) {
                    Ok(page) => page,
                    Err(fault) => return JobReplayStep::Refused(fault),
                };
                let payload_digest = chunk.iter().fold(pending.payload_digest, |digest, byte| replay_mix(digest, u64::from(*byte)));
                (page, payload_digest)
            };
            pending.payload_digest = payload_digest;
            pending.payload_offset = end;
            if let Err(mut page) = pending.record.as_mut().expect("capture owns record").push_page(page) {
                let _ = page.close();
                return JobReplayStep::Refused(JobReplayFault::Capacity);
            }
            return JobReplayStep::MoreWork;
        }
        cx.consume_fuel(1);
        if pending.sealed {
            return JobReplayStep::PublicationReady;
        }
        let record = pending.record.as_mut().expect("capture owns record");
        if record.page_count != pending.expected_pages {
            return JobReplayStep::Refused(JobReplayFault::Mismatch);
        }
        record.header.payload_digest = pending.payload_digest;
        record.header.prefix_digest = replay_header_digest(self.prefix_digest, &self.route, &record.header);
        if self.replaying {
            let Some(expected) = self.records.get(self.replay_cursor).and_then(Option::as_ref) else { return JobReplayStep::Refused(JobReplayFault::Sequence) };
            if !replay_record_matches(expected, record) {
                return JobReplayStep::Refused(JobReplayFault::Mismatch);
            }
            let Some(next_ordinal) = self.next_ordinal.checked_add(1) else { return JobReplayStep::Refused(JobReplayFault::Exhausted) };
            self.ready_record = Some(self.replay_cursor);
            self.prefix_digest = expected.header.prefix_digest;
            self.next_ordinal = next_ordinal;
            self.replay_cursor += 1;
        } else {
            let index = self.record_count;
            self.records[index] = pending.record.take();
            self.record_count += 1;
            if self.records[index].as_ref().is_some_and(|record| record.header.kind == JobReplayPublicationKind::Checkpoint) {
                self.last_checkpoint_index = Some(index);
            }
            self.next_ordinal = self.next_ordinal.checked_add(1).expect("ordinal exhaustion preflighted");
            self.prefix_digest = self.records[index].as_ref().expect("sealed replay record").header.prefix_digest;
            self.reserved_records -= 1;
            self.reserved_pages -= pending.expected_pages;
            self.ready_record = Some(index);
        }
        pending.sealed = true;
        JobReplayStep::PublicationReady
    }

    pub fn take_captured_publication(&mut self) -> Option<JobPublication> {
        self.ready_record?;
        self.pending.as_mut()?.publication.take()
    }

    pub fn publication_is_ready(&self) -> bool {
        self.ready_record.is_some() && self.pending.as_ref().is_some_and(|pending| pending.publication.is_some())
    }

    pub fn acknowledge_publication(&mut self, cx: &mut job::StepContext<'_>, policy: JobReplayPublicationPolicy) -> Result<(), JobReplayFault> {
        if cx.is_cancelled() || cx.should_yield() {
            return Err(if cx.is_cancelled() { JobReplayFault::Cancelled } else { JobReplayFault::Deadline });
        }
        cx.consume_fuel(1);
        let index = self.ready_record.take().ok_or(JobReplayFault::Sequence)?;
        if policy == JobReplayPublicationPolicy::Pending {
            self.ready_record = Some(index);
            return Err(JobReplayFault::Mismatch);
        }
        if self.replaying {
            if self.records[index].as_ref().ok_or(JobReplayFault::Sequence)?.header.policy != policy {
                self.ready_record = Some(index);
                return Err(JobReplayFault::Mismatch);
            }
        } else {
            self.records[index].as_mut().ok_or(JobReplayFault::Sequence)?.header.policy = policy;
        }
        let pending = self.pending.as_mut().ok_or(JobReplayFault::Sequence)?;
        if self.replaying {
            pending.acknowledged = true;
            return Ok(());
        }
        let pending = self.pending.take().ok_or(JobReplayFault::Sequence)?;
        drop(pending);
        Ok(())
    }

    pub fn maintenance_step(&mut self, cx: &mut job::StepContext<'_>) -> JobReplayStep {
        let Some(pending) = self.pending.as_mut() else { return JobReplayStep::Complete };
        if !pending.acknowledged {
            return self.capture_step(cx);
        }
        if cx.should_yield() {
            return JobReplayStep::MoreWork;
        }
        cx.consume_fuel(1);
        if let Some(record) = pending.record.as_mut() {
            if !record.terminal_is_empty() {
                let before = record.page_count;
                let _ = record.close_step(JOB_REPLAY_PAGE_BYTES);
                self.reserved_pages -= before.saturating_sub(record.page_count);
                return JobReplayStep::MoreWork;
            }
            drop(pending.record.take());
            self.reserved_records -= 1;
            return JobReplayStep::MoreWork;
        }
        let pending = self.pending.take().expect("acknowledged replay candidate");
        drop(pending);
        JobReplayStep::Complete
    }

    pub fn has_pending_work(&self) -> bool {
        self.pending.is_some() || self.ready_record.is_some() || (self.closing && !self.terminal_is_empty())
    }

    pub fn close_started(&self) -> bool {
        self.closing
    }

    pub fn begin_replay(&mut self, generation: u64) -> Result<(), JobReplayFault> {
        if generation != self.generation || self.pending.is_some() || self.ready_record.is_some() || self.closing {
            return Err(JobReplayFault::Stale);
        }
        self.replaying = true;
        self.replay_cursor = 0;
        self.prefix_digest = 0xcbf2_9ce4_8422_2325;
        self.next_ordinal = 0;
        Ok(())
    }

    pub fn expected_turn(&self) -> Option<JobTurn> {
        self.replaying.then(|| self.records.get(self.replay_cursor).and_then(Option::as_ref).map(|record| record.header.turn)).flatten()
    }

    pub fn replay_is_complete(&self) -> bool {
        self.replaying && self.replay_cursor == self.record_count && self.pending.is_none() && self.ready_record.is_none()
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn close_step(&mut self, cx: &mut job::StepContext<'_>) -> JobReplayCloseStep {
        self.closing = true;
        if cx.is_cancelled() || cx.should_yield() {
            return JobReplayCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        cx.consume_fuel(1);
        if let Some(pending) = self.pending.as_mut() {
            if let Some(record) = pending.record.as_mut() {
                if !record.terminal_is_empty() {
                    return record.close_step(JOB_REPLAY_PAGE_BYTES);
                }
                drop(pending.record.take());
                return JobReplayCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            if pending.retirement.is_none() {
                if let Some(publication) = pending.publication.take() {
                    let total_phases = replay_payload_items(&publication) as u8 + 1;
                    *pending.retirement = Some(JobPublicationOwner { publication, phase: 0, total_phases, owner_bytes: 0 });
                    return JobReplayCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
            }
            if let Some(owner) = pending.retirement.as_mut() {
                if !owner.terminal_is_empty() {
                    let (complete, released_items, released_bytes) = owner.close_one();
                    if !complete {
                        return JobReplayCloseStep::Pending { released_items, released_bytes };
                    }
                }
                *pending.retirement = None;
                return JobReplayCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            let pending = self.pending.take().expect("terminal replay capture");
            drop(pending);
            self.reserved_records = 0;
            self.reserved_pages = 0;
            return JobReplayCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        let index = self.close_cursor % JOB_REPLAY_RECORD_CAPACITY;
        self.close_cursor = (self.close_cursor + 1) % JOB_REPLAY_RECORD_CAPACITY;
        let Some(record) = self.records[index].as_mut() else {
            return if self.terminal_is_empty() { JobReplayCloseStep::Complete } else { JobReplayCloseStep::Pending { released_items: 0, released_bytes: 0 } };
        };
        if !record.terminal_is_empty() {
            return record.close_step(JOB_REPLAY_PAGE_BYTES);
        }
        self.records[index] = None;
        self.record_count -= 1;
        JobReplayCloseStep::Pending { released_items: 1, released_bytes: 0 }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.record_count == 0 && self.reserved_records == 0 && self.reserved_pages == 0 && self.pending.is_none() && self.ready_record.is_none() && self.records.iter().all(Option::is_none)
    }
}

impl Drop for JobReplayLog {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            unsafe {
                ManuallyDrop::drop(&mut self.records);
                ManuallyDrop::drop(&mut self.pending);
            }
        } else {
            debug_assert!(false, "mounted replay log requires generation-qualified recovery and incremental close");
        }
    }
}

fn replay_rejected(fault: JobReplayFault, publication: JobPublication) -> JobReplayRejected {
    JobReplayRejected { fault, publication: ManuallyDrop::new(Some(publication)) }
}

fn replay_payload_items(publication: &JobPublication) -> usize {
    match &publication.outcome {
        JobStepOutcome::PreviewReady { .. } | JobStepOutcome::CheckpointReady { .. } | JobStepOutcome::Fault { .. } => 1,
        JobStepOutcome::Complete { .. } => 2,
        JobStepOutcome::Yield | JobStepOutcome::Cancelled => 0,
    }
}

fn replay_payload_bytes(publication: &JobPublication) -> Option<usize> {
    match &publication.outcome {
        JobStepOutcome::PreviewReady { preview } => Some(preview.len()),
        JobStepOutcome::CheckpointReady { checkpoint } => Some(checkpoint.state.len()),
        JobStepOutcome::Complete { candidate } => candidate.state.len().checked_add(candidate.output.len()),
        JobStepOutcome::Fault { detail } => Some(detail.len()),
        JobStepOutcome::Yield | JobStepOutcome::Cancelled => Some(0),
    }
}

fn replay_payload_pages(publication: &JobPublication) -> usize {
    match &publication.outcome {
        JobStepOutcome::PreviewReady { preview } => preview.len().div_ceil(JOB_REPLAY_PAGE_BYTES),
        JobStepOutcome::CheckpointReady { checkpoint } => checkpoint.state.len().div_ceil(JOB_REPLAY_PAGE_BYTES),
        JobStepOutcome::Complete { candidate } => candidate.state.len().div_ceil(JOB_REPLAY_PAGE_BYTES) + candidate.output.len().div_ceil(JOB_REPLAY_PAGE_BYTES),
        JobStepOutcome::Fault { detail } => detail.len().div_ceil(JOB_REPLAY_PAGE_BYTES),
        JobStepOutcome::Yield | JobStepOutcome::Cancelled => 0,
    }
}

fn replay_header_digest(mut digest: u64, route: &JobReplayRoute, header: &JobReplayRecordHeader) -> u64 {
    for byte in route.plugin.iter().chain(route.package.iter()).chain(route.controller.iter()).chain(route.tool.iter()).chain(route.artifact.iter()).chain(route.request_schema.iter()).chain(route.request_digest.iter()) {
        digest = replay_mix(digest, u64::from(*byte));
    }
    for value in [
        route.window,
        u64::from(route.request_version),
        header.actor.0,
        header.turn.job,
        header.turn.operation.operation,
        header.turn.operation.base_revision,
        header.turn.operation.generation,
        header.turn.operation.seed,
        header.turn.step_sequence,
        header.ordinal,
        header.granted_fuel,
        u64::from(header.deadline_class_ms),
        u64::from(header.worker_count),
        u64::from(header.worker_slot),
        u64::from(header.cancellation_observed),
        match header.kind {
            JobReplayPublicationKind::Yield => 0,
            JobReplayPublicationKind::Preview => 1,
            JobReplayPublicationKind::Checkpoint => 2,
            JobReplayPublicationKind::Commit => 3,
            JobReplayPublicationKind::Cancelled => 4,
            JobReplayPublicationKind::Fault => 5,
        },
        header.applied_progress,
        u64::from(header.payload_items),
        header.payload_bytes as u64,
        header.payload_digest,
    ] {
        digest = replay_mix(digest, value);
    }
    digest
}

fn replay_record_matches(expected: &JobReplayRecord, actual: &JobReplayRecord) -> bool {
    expected.header.actor == actual.header.actor
        && expected.header.turn == actual.header.turn
        && expected.header.ordinal == actual.header.ordinal
        && expected.header.granted_fuel == actual.header.granted_fuel
        && expected.header.deadline_class_ms == actual.header.deadline_class_ms
        && expected.header.worker_count == actual.header.worker_count
        && expected.header.worker_slot == actual.header.worker_slot
        && expected.header.cancellation_observed == actual.header.cancellation_observed
        && expected.header.kind == actual.header.kind
        && expected.header.applied_progress == actual.header.applied_progress
        && expected.header.payload_items == actual.header.payload_items
        && expected.header.payload_bytes == actual.header.payload_bytes
        && expected.header.payload_digest == actual.header.payload_digest
        && expected.header.prefix_digest == actual.header.prefix_digest
        && expected.page_count == actual.page_count
        && expected.pages.iter().flatten().zip(actual.pages.iter().flatten()).all(|(left, right)| left.bytes() == right.bytes())
}
//#endregion 📜️FixedReplay

//#region 🎨️JobProgressOverlay
pub const JOB_PROGRESS_ACTIVE_CAPACITY: usize = 64;
pub const JOB_PROGRESS_RETIREMENT_CAPACITY: usize = 128;
pub const JOB_PROGRESS_PAGE_MAXIMUM_BYTES: usize = 16 * 1024;
pub const JOB_PROGRESS_TOTAL_MAXIMUM_BYTES: usize = 4 * 1024 * 1024;
pub const JOB_PROGRESS_TOTAL_MAXIMUM_ITEMS: usize = 512;

/// 🪪️ Exact identity of one actor-visible job progress publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JobProgressIdentity {
    pub actor: ActorId,
    pub job: u64,
    pub operation: u64,
    pub base_revision: u64,
    pub generation: u64,
    pub step_sequence: u64,
    pub preview_sequence: u64,
}

impl JobProgressIdentity {
    pub fn from_publication(actor: ActorId, publication: &JobPublication) -> Self {
        Self {
            actor,
            job: publication.turn.job,
            operation: publication.turn.operation.operation,
            base_revision: publication.turn.operation.base_revision,
            generation: publication.turn.operation.generation,
            step_sequence: publication.turn.step_sequence,
            preview_sequence: publication.turn.operation.preview_sequence,
        }
    }

    fn same_job(self, other: Self) -> bool {
        self.actor == other.actor && self.job == other.job
    }

    fn same_operation(self, other: Self) -> bool {
        self.same_job(other) && self.operation == other.operation && self.base_revision == other.base_revision && self.generation == other.generation
    }
}

/// 📡️ Scalar, coalescible progress kind retained separately from committed actor scenes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobProgressKind {
    Yield,
    Preview,
    Checkpoint,
    CommitValidated,
    Cancelled,
    Fault,
}

impl JobProgressKind {
    fn of(outcome: &JobStepOutcome) -> Self {
        match outcome {
            JobStepOutcome::Yield => Self::Yield,
            JobStepOutcome::PreviewReady { .. } => Self::Preview,
            JobStepOutcome::CheckpointReady { .. } => Self::Checkpoint,
            JobStepOutcome::Complete { .. } => Self::CommitValidated,
            JobStepOutcome::Cancelled => Self::Cancelled,
            JobStepOutcome::Fault { .. } => Self::Fault,
        }
    }

    fn terminal(self) -> bool {
        matches!(self, Self::CommitValidated | Self::Cancelled | Self::Fault)
    }
}

/// 🔐️ Independent revision/generation authority supplied immediately before publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JobProgressLiveAuthority {
    pub operation: u64,
    pub base_revision: u64,
    pub generation: u64,
}

impl JobProgressLiveAuthority {
    pub fn new(operation: u64, base_revision: u64, generation: u64) -> Self {
        Self { operation, base_revision, generation }
    }

    fn accepts(self, identity: JobProgressIdentity) -> bool {
        self.operation == identity.operation && self.base_revision == identity.base_revision && self.generation == identity.generation
    }
}

/// 🚫️ Fail-closed reason returned without consuming a rejected publication owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobProgressFault {
    Cancelled,
    Budget,
    Capacity,
    Oversized,
    Stale,
    StepSequence,
    PreviewSequence,
    Busy,
    Token,
    Missing,
    CheckedOut,
    Exhausted,
}

impl std::fmt::Display for JobProgressFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "job progress overlay fault: {self:?}")
    }
}

impl std::error::Error for JobProgressFault {}

/// 📮️ Exact rejected publication owner; callers must return or close it rather than recreate input.
#[must_use = "a rejected job publication retains the exact payload owner"]
#[derive(Debug)]
pub struct JobProgressRejected {
    fault: JobProgressFault,
    publication: Option<JobPublication>,
}

impl JobProgressRejected {
    pub fn new(fault: JobProgressFault, publication: JobPublication) -> Self {
        Self { fault, publication: Some(publication) }
    }

    pub fn fault(&self) -> JobProgressFault {
        self.fault
    }

    pub fn publication(&self) -> &JobPublication {
        self.publication.as_ref().expect("rejected job publication owner already returned")
    }

    pub fn into_publication(mut self) -> JobPublication {
        self.publication.take().expect("rejected job publication owner already returned")
    }
}

#[derive(Debug)]
pub(crate) struct JobPublicationOwner {
    pub(crate) publication: JobPublication,
    pub(crate) phase: u8,
    total_phases: u8,
    owner_bytes: usize,
}

impl JobPublicationOwner {
    pub(crate) fn try_new(publication: JobPublication) -> Result<Self, (JobProgressFault, JobPublication)> {
        match publication_owner_shape(&publication, false) {
            Ok((_, owner_bytes, total_phases)) => Ok(Self { publication, phase: 0, total_phases, owner_bytes }),
            Err(fault) => Err((fault, publication)),
        }
    }

    pub(crate) fn close_one(&mut self) -> (bool, usize, usize) {
        let released_bytes = match (&mut self.publication.outcome, self.phase) {
            (JobStepOutcome::PreviewReady { preview }, 0) => std::mem::take(preview).capacity(),
            (JobStepOutcome::CheckpointReady { checkpoint }, 0) => std::mem::take(&mut checkpoint.state).capacity(),
            (JobStepOutcome::Complete { candidate }, 0) => std::mem::take(&mut candidate.state).capacity(),
            (JobStepOutcome::Complete { candidate }, 1) => std::mem::take(&mut candidate.output).capacity(),
            (JobStepOutcome::Fault { detail }, 0) => std::mem::take(detail).capacity(),
            _ => 0,
        };
        self.phase = self.phase.saturating_add(1);
        self.owner_bytes = self.owner_bytes.saturating_sub(released_bytes);
        (self.phase >= self.total_phases, 1, released_bytes)
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.phase >= self.total_phases
            && self.owner_bytes == 0
            && match &self.publication.outcome {
                JobStepOutcome::PreviewReady { preview } => preview.capacity() == 0,
                JobStepOutcome::CheckpointReady { checkpoint } => checkpoint.state.capacity() == 0,
                JobStepOutcome::Complete { candidate } => candidate.state.capacity() == 0 && candidate.output.capacity() == 0,
                JobStepOutcome::Fault { detail } => detail.capacity() == 0,
                JobStepOutcome::Yield | JobStepOutcome::Cancelled => true,
            }
    }
}

fn publication_owner_shape(publication: &JobPublication, enforce_page_limit: bool) -> Result<(usize, usize, u8), JobProgressFault> {
    let capacities = match &publication.outcome {
        JobStepOutcome::Yield | JobStepOutcome::Cancelled => ([0usize, 0usize], 0usize),
        JobStepOutcome::PreviewReady { preview } => ([preview.capacity(), 0], 1),
        JobStepOutcome::CheckpointReady { checkpoint } => ([checkpoint.state.capacity(), 0], 1),
        JobStepOutcome::Complete { candidate } => ([candidate.state.capacity(), candidate.output.capacity()], 2),
        JobStepOutcome::Fault { detail } => ([detail.capacity(), 0], 1),
    };
    if enforce_page_limit && capacities.0[..capacities.1].iter().any(|capacity| *capacity > JOB_PROGRESS_PAGE_MAXIMUM_BYTES) {
        return Err(JobProgressFault::Oversized);
    }
    let owner_bytes = capacities.0[..capacities.1].iter().try_fold(0usize, |total, capacity| total.checked_add(*capacity)).ok_or(JobProgressFault::Capacity)?;
    let total_phases = u8::try_from(capacities.1.checked_add(1).ok_or(JobProgressFault::Capacity)?).map_err(|_| JobProgressFault::Capacity)?;
    Ok((capacities.1 + 1, owner_bytes, total_phases))
}

#[derive(Debug)]
struct JobProgressSlot {
    epoch: u64,
    occupied: bool,
    published: bool,
    awaiting_ack: bool,
    checked_out: bool,
    terminal: bool,
    reservation: Option<u64>,
    identity: JobProgressIdentity,
    kind: JobProgressKind,
    applied_progress: u64,
    preview: Option<JobPublicationOwner>,
}

impl JobProgressSlot {
    fn vacant() -> Self {
        Self {
            epoch: 0,
            occupied: false,
            published: false,
            awaiting_ack: false,
            checked_out: false,
            terminal: false,
            reservation: None,
            identity: JobProgressIdentity { actor: ActorId(0), job: 0, operation: 0, base_revision: 0, generation: 0, step_sequence: 0, preview_sequence: 0 },
            kind: JobProgressKind::Yield,
            applied_progress: 0,
            preview: None,
        }
    }

    fn clear(&mut self) {
        let epoch = self.epoch;
        *self = Self::vacant();
        self.epoch = epoch;
    }
}

#[derive(Debug)]
struct JobProgressRetirementSlot {
    reservation: Option<u64>,
    owner: Option<JobPublicationOwner>,
}

#[derive(Clone, Copy, Debug)]
struct JobProgressPriorState {
    published: bool,
    terminal: bool,
    identity: JobProgressIdentity,
    kind: JobProgressKind,
    applied_progress: u64,
}

impl JobProgressRetirementSlot {
    fn vacant() -> Self {
        Self { reservation: None, owner: None }
    }
}

/// 🎟️ Linear reservation made before the publication owner crosses into the overlay.
#[must_use = "job progress admission must be published or cancelled"]
#[derive(Debug)]
pub struct JobProgressAdmission {
    token: u64,
    active_index: usize,
    active_epoch: u64,
    identity: JobProgressIdentity,
    live: JobProgressLiveAuthority,
    retirement_indices: [usize; 2],
    abort_retirement: Option<usize>,
    owner_items: usize,
    owner_bytes: usize,
}

/// ✅️ Linear ACK/abort authority for one adopted publication.
#[must_use = "an adopted job publication must be acknowledged or aborted"]
#[derive(Debug)]
pub struct JobProgressReceipt {
    identity: JobProgressIdentity,
    kind: JobProgressKind,
    applied_progress: u64,
    owner_bytes: usize,
    active_index: usize,
    active_epoch: u64,
    prior: JobProgressPriorState,
    displaced_retirement: Option<usize>,
    abort_retirement: Option<(usize, u64)>,
}

impl JobProgressReceipt {
    pub fn identity(&self) -> JobProgressIdentity {
        self.identity
    }

    pub fn kind(&self) -> JobProgressKind {
        self.kind
    }

    pub fn applied_progress(&self) -> u64 {
        self.applied_progress
    }

    pub fn owner_bytes(&self) -> usize {
        self.owner_bytes
    }
}

/// 🧹️ One truthful cleanup opportunity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobProgressCloseStep {
    Pending { released_items: usize, released_bytes: usize },
    Blocked,
    Complete,
}

/// 🎨️ Fixed actor-owned preview/progress authority; committed scenes are never stored here.
pub struct JobProgressOverlayStore {
    active: [JobProgressSlot; JOB_PROGRESS_ACTIVE_CAPACITY],
    retirements: [JobProgressRetirementSlot; JOB_PROGRESS_RETIREMENT_CAPACITY],
    closing_actors: [Option<ActorId>; JOB_PROGRESS_ACTIVE_CAPACITY],
    close_all: bool,
    next_token: u64,
    owned_items: usize,
    owned_bytes: usize,
    reserved_items: usize,
    reserved_bytes: usize,
}

impl Default for JobProgressOverlayStore {
    fn default() -> Self {
        Self::new()
    }
}

impl JobProgressOverlayStore {
    pub fn new() -> Self {
        Self {
            active: std::array::from_fn(|_| JobProgressSlot::vacant()),
            retirements: std::array::from_fn(|_| JobProgressRetirementSlot::vacant()),
            closing_actors: [None; JOB_PROGRESS_ACTIVE_CAPACITY],
            close_all: false,
            next_token: 1,
            owned_items: 0,
            owned_bytes: 0,
            reserved_items: 0,
            reserved_bytes: 0,
        }
    }

    pub fn live_authority(&self, actor: ActorId, job: u64) -> Option<JobProgressLiveAuthority> {
        self.active.iter().find(|slot| slot.occupied && slot.identity.actor == actor && slot.identity.job == job).map(|slot| JobProgressLiveAuthority::new(slot.identity.operation, slot.identity.base_revision, slot.identity.generation))
    }

    pub fn has_visible_preview(&self, actor: ActorId, job: u64) -> bool {
        self.active.iter().any(|slot| slot.occupied && slot.identity.actor == actor && slot.identity.job == job && slot.preview.is_some())
    }

    pub fn owned_items(&self) -> usize {
        self.owned_items
    }

    pub fn owned_bytes(&self) -> usize {
        self.owned_bytes
    }

    pub fn begin_operation(&mut self, actor: ActorId, job: u64, live: JobProgressLiveAuthority) -> Result<(), JobProgressFault> {
        if self.close_all || self.closing_actors.iter().flatten().any(|closing| *closing == actor) {
            return Err(JobProgressFault::Cancelled);
        }
        if self.active.iter().any(|slot| slot.occupied && slot.identity.actor == actor && slot.identity.job == job) {
            return Err(JobProgressFault::Busy);
        }
        let slot = self.active.iter_mut().find(|slot| !slot.occupied && slot.reservation.is_none()).ok_or(JobProgressFault::Capacity)?;
        slot.epoch = slot.epoch.checked_add(1).ok_or(JobProgressFault::Exhausted)?;
        slot.occupied = true;
        slot.published = false;
        slot.identity = JobProgressIdentity { actor, job, operation: live.operation, base_revision: live.base_revision, generation: live.generation, step_sequence: 0, preview_sequence: 0 };
        Ok(())
    }

    pub fn preflight(&mut self, cx: &mut job::StepContext<'_>, actor: ActorId, publication: &JobPublication, live: JobProgressLiveAuthority) -> Result<JobProgressAdmission, JobProgressFault> {
        let identity = JobProgressIdentity::from_publication(actor, publication);
        if cx.is_cancelled() || self.close_all || self.closing_actors.iter().flatten().any(|closing| *closing == actor) {
            return Err(JobProgressFault::Cancelled);
        }
        if cx.should_yield() {
            return Err(JobProgressFault::Budget);
        }
        if cx.operation().0 != identity.operation || cx.generation().0 != identity.generation || !live.accepts(identity) {
            return Err(JobProgressFault::Stale);
        }
        let (owner_items, owner_bytes, _) = publication_owner_shape(publication, true)?;
        let kind = JobProgressKind::of(&publication.outcome);
        let active_index = self.active.iter().position(|slot| slot.occupied && slot.identity.actor == actor && slot.identity.job == publication.turn.job).ok_or(JobProgressFault::Missing)?;
        let active_epoch = self.active[active_index].epoch;
        if self.active[active_index].checked_out {
            return Err(JobProgressFault::CheckedOut);
        }
        if self.active[active_index].awaiting_ack || self.active[active_index].reservation.is_some() {
            return Err(JobProgressFault::Busy);
        }
        if active_epoch == u64::MAX {
            return Err(JobProgressFault::Exhausted);
        }
        if !self.active[active_index].identity.same_operation(identity) {
            return Err(JobProgressFault::Stale);
        }
        let prior_preview_sequence = if self.active[active_index].published {
            let expected_step = self.active[active_index].identity.step_sequence.checked_add(1).ok_or(JobProgressFault::Exhausted)?;
            if identity.step_sequence != expected_step {
                return Err(JobProgressFault::StepSequence);
            }
            self.active[active_index].identity.preview_sequence
        } else {
            if identity.step_sequence != 0 {
                return Err(JobProgressFault::StepSequence);
            }
            0
        };
        let expected_preview = if kind == JobProgressKind::Preview { prior_preview_sequence.checked_add(1).ok_or(JobProgressFault::Exhausted)? } else { prior_preview_sequence };
        if identity.preview_sequence != expected_preview {
            return Err(JobProgressFault::PreviewSequence);
        }
        let has_preview = self.active[active_index].preview.is_some();
        let adoption_retirements = match kind {
            JobProgressKind::Preview => usize::from(has_preview),
            JobProgressKind::Checkpoint => 1,
            JobProgressKind::CommitValidated | JobProgressKind::Fault => 1 + usize::from(has_preview),
            JobProgressKind::Cancelled => usize::from(has_preview),
            JobProgressKind::Yield => 0,
        };
        let staged_preview = kind == JobProgressKind::Preview;
        let retirement_count = adoption_retirements + usize::from(staged_preview);
        if retirement_count > 2 {
            return Err(JobProgressFault::Capacity);
        }
        let mut retirement_indices = [usize::MAX; 2];
        let mut found = 0usize;
        for (index, retirement) in self.retirements.iter().enumerate() {
            if retirement.owner.is_none() && retirement.reservation.is_none() {
                retirement_indices[found] = index;
                found += 1;
                if found == retirement_count {
                    break;
                }
            }
        }
        if found != retirement_count {
            return Err(JobProgressFault::Capacity);
        }
        let next_items = self.owned_items.checked_add(self.reserved_items).and_then(|total| total.checked_add(owner_items)).ok_or(JobProgressFault::Capacity)?;
        let next_bytes = self.owned_bytes.checked_add(self.reserved_bytes).and_then(|total| total.checked_add(owner_bytes)).ok_or(JobProgressFault::Capacity)?;
        if next_items > JOB_PROGRESS_TOTAL_MAXIMUM_ITEMS || next_bytes > JOB_PROGRESS_TOTAL_MAXIMUM_BYTES {
            return Err(JobProgressFault::Capacity);
        }
        let token = self.next_token;
        self.next_token = self.next_token.checked_add(1).ok_or(JobProgressFault::Exhausted)?;
        self.active[active_index].reservation = Some(token);
        for index in retirement_indices.iter().copied().take(retirement_count) {
            self.retirements[index].reservation = Some(token);
        }
        self.reserved_items += owner_items;
        self.reserved_bytes += owner_bytes;
        cx.consume_fuel(1);
        Ok(JobProgressAdmission { token, active_index, active_epoch, identity, live, retirement_indices, abort_retirement: staged_preview.then_some(retirement_indices[adoption_retirements]), owner_items, owner_bytes })
    }

    #[expect(clippy::needless_pass_by_value, reason = "Cancellation consumes the single-use reservation authority even though its accounting fields are copied.")]
    pub fn cancel_admission(&mut self, admission: JobProgressAdmission) -> Result<(), JobProgressFault> {
        let slot = self.active.get(admission.active_index).ok_or(JobProgressFault::Token)?;
        if slot.epoch != admission.active_epoch || slot.reservation != Some(admission.token) {
            return Err(JobProgressFault::Token);
        }
        for index in admission.retirement_indices.iter().copied().filter(|index| *index != usize::MAX) {
            let retirement = self.retirements.get(index).ok_or(JobProgressFault::Token)?;
            if retirement.reservation != Some(admission.token) || retirement.owner.is_some() {
                return Err(JobProgressFault::Token);
            }
        }
        self.active[admission.active_index].reservation = None;
        for index in admission.retirement_indices.iter().copied().filter(|index| *index != usize::MAX) {
            let retirement = &mut self.retirements[index];
            retirement.reservation = None;
        }
        self.reserved_items = self.reserved_items.saturating_sub(admission.owner_items);
        self.reserved_bytes = self.reserved_bytes.saturating_sub(admission.owner_bytes);
        Ok(())
    }

    pub fn publish_reserved(&mut self, cx: &mut job::StepContext<'_>, admission: JobProgressAdmission, publication: JobPublication, live: JobProgressLiveAuthority) -> Result<JobProgressReceipt, JobProgressRejected> {
        let identity = JobProgressIdentity::from_publication(admission.identity.actor, &publication);
        let slot_valid = self.active.get(admission.active_index).is_some_and(|slot| slot.epoch == admission.active_epoch && slot.reservation == Some(admission.token));
        if cx.is_cancelled() || cx.operation().0 != identity.operation || cx.generation().0 != identity.generation {
            let _ = self.cancel_admission(admission);
            return Err(JobProgressRejected::new(JobProgressFault::Cancelled, publication));
        }
        if !slot_valid || identity != admission.identity || live != admission.live || !live.accepts(identity) {
            let _ = self.cancel_admission(admission);
            return Err(JobProgressRejected::new(JobProgressFault::Stale, publication));
        }
        let kind = JobProgressKind::of(&publication.outcome);
        let applied_progress = match &publication.outcome {
            JobStepOutcome::CheckpointReady { checkpoint } => checkpoint.applied_progress,
            _ => 0,
        };
        let retains_incoming = matches!(kind, JobProgressKind::Preview | JobProgressKind::Checkpoint | JobProgressKind::CommitValidated | JobProgressKind::Fault);
        let incoming = if retains_incoming {
            match JobPublicationOwner::try_new(publication) {
                Ok(owner) => Some(owner),
                Err((fault, publication)) => {
                    let _ = self.cancel_admission(admission);
                    return Err(JobProgressRejected::new(fault, publication));
                }
            }
        } else {
            drop(publication);
            None
        };
        let prior = JobProgressPriorState {
            published: self.active[admission.active_index].published,
            terminal: self.active[admission.active_index].terminal,
            identity: self.active[admission.active_index].identity,
            kind: self.active[admission.active_index].kind,
            applied_progress: self.active[admission.active_index].applied_progress,
        };
        let displaced = match kind {
            JobProgressKind::Preview | JobProgressKind::CommitValidated | JobProgressKind::Cancelled | JobProgressKind::Fault => self.active[admission.active_index].preview.take(),
            JobProgressKind::Yield | JobProgressKind::Checkpoint => None,
        };
        let mut retirement_cursor = 0usize;
        let mut displaced_retirement = None;
        if let Some(displaced) = displaced {
            let index = admission.retirement_indices[retirement_cursor];
            self.retirements[index].reservation = None;
            self.retirements[index].owner = Some(displaced);
            displaced_retirement = Some(index);
            retirement_cursor += 1;
        }
        if kind == JobProgressKind::Preview {
            self.active[admission.active_index].preview = incoming;
        } else if let Some(incoming) = incoming {
            let index = admission.retirement_indices[retirement_cursor];
            self.retirements[index].reservation = None;
            self.retirements[index].owner = Some(incoming);
        }
        let next_epoch = admission.active_epoch + 1;
        let slot = &mut self.active[admission.active_index];
        slot.epoch = next_epoch;
        slot.occupied = true;
        slot.published = true;
        slot.awaiting_ack = true;
        slot.checked_out = false;
        slot.terminal = kind.terminal();
        slot.reservation = None;
        slot.identity = identity;
        slot.kind = kind;
        slot.applied_progress = applied_progress;
        self.reserved_items = self.reserved_items.saturating_sub(admission.owner_items);
        self.reserved_bytes = self.reserved_bytes.saturating_sub(admission.owner_bytes);
        if retains_incoming {
            self.owned_items += admission.owner_items;
            self.owned_bytes += admission.owner_bytes;
        }
        let abort_retirement = admission.abort_retirement.map(|index| (index, admission.token));
        Ok(JobProgressReceipt { identity, kind, applied_progress, owner_bytes: admission.owner_bytes, active_index: admission.active_index, active_epoch: next_epoch, prior, displaced_retirement, abort_retirement })
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact retained progress receipt without allocating or losing its prior owner.")]
    pub fn acknowledge(&mut self, receipt: JobProgressReceipt) -> Result<(), (JobProgressFault, JobProgressReceipt)> {
        let Some(slot) = self.active.get_mut(receipt.active_index) else {
            return Err((JobProgressFault::Token, receipt));
        };
        if slot.epoch != receipt.active_epoch || !slot.occupied || !slot.awaiting_ack || slot.identity != receipt.identity {
            return Err((JobProgressFault::Token, receipt));
        }
        if let Some((index, token)) = receipt.abort_retirement {
            let Some(retirement) = self.retirements.get_mut(index) else {
                return Err((JobProgressFault::Token, receipt));
            };
            if retirement.reservation != Some(token) || retirement.owner.is_some() {
                return Err((JobProgressFault::Token, receipt));
            }
            retirement.reservation = None;
        }
        slot.awaiting_ack = false;
        if slot.terminal {
            if slot.preview.is_some() {
                slot.awaiting_ack = true;
                return Err((JobProgressFault::Token, receipt));
            }
            slot.clear();
        }
        Ok(())
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact retained progress receipt without allocating or losing its prior owner.")]
    pub fn abort(&mut self, receipt: JobProgressReceipt) -> Result<(), (JobProgressFault, JobProgressReceipt)> {
        let Some(slot) = self.active.get_mut(receipt.active_index) else {
            return Err((JobProgressFault::Token, receipt));
        };
        if slot.epoch != receipt.active_epoch || !slot.occupied || !slot.awaiting_ack || slot.identity != receipt.identity {
            return Err((JobProgressFault::Token, receipt));
        }
        if receipt.kind == JobProgressKind::Preview {
            let Some(owner) = slot.preview.take() else {
                return Err((JobProgressFault::Token, receipt));
            };
            let Some((index, token)) = receipt.abort_retirement else {
                slot.preview = Some(owner);
                return Err((JobProgressFault::Token, receipt));
            };
            let Some(retirement) = self.retirements.get_mut(index) else {
                slot.preview = Some(owner);
                return Err((JobProgressFault::Token, receipt));
            };
            if retirement.reservation != Some(token) || retirement.owner.is_some() {
                slot.preview = Some(owner);
                return Err((JobProgressFault::Token, receipt));
            }
            retirement.reservation = None;
            retirement.owner = Some(owner);
        }
        let restored_preview = if let Some(index) = receipt.displaced_retirement {
            let Some(retirement) = self.retirements.get_mut(index) else {
                return Err((JobProgressFault::Token, receipt));
            };
            let Some(owner) = retirement.owner.take() else {
                return Err((JobProgressFault::Token, receipt));
            };
            Some(owner)
        } else {
            None
        };
        slot.published = receipt.prior.published;
        slot.awaiting_ack = false;
        slot.checked_out = false;
        slot.terminal = receipt.prior.terminal;
        slot.reservation = None;
        slot.identity = receipt.prior.identity;
        slot.kind = receipt.prior.kind;
        slot.applied_progress = receipt.prior.applied_progress;
        slot.preview = restored_preview.or_else(|| slot.preview.take());
        Ok(())
    }

    pub fn retain_rejected(&mut self, rejected: JobProgressRejected) -> Result<(), JobProgressRejected> {
        let Some(index) = self.retirements.iter().position(|slot| slot.owner.is_none() && slot.reservation.is_none()) else {
            return Err(rejected);
        };
        let Ok((items, bytes, _)) = publication_owner_shape(rejected.publication(), false) else {
            return Err(rejected);
        };
        if self.owned_items.checked_add(self.reserved_items).and_then(|total| total.checked_add(items)).is_none_or(|total| total > JOB_PROGRESS_TOTAL_MAXIMUM_ITEMS)
            || self.owned_bytes.checked_add(self.reserved_bytes).and_then(|total| total.checked_add(bytes)).is_none_or(|total| total > JOB_PROGRESS_TOTAL_MAXIMUM_BYTES)
        {
            return Err(rejected);
        }
        let publication = rejected.into_publication();
        let owner = match JobPublicationOwner::try_new(publication) {
            Ok(owner) => owner,
            Err((fault, publication)) => return Err(JobProgressRejected::new(fault, publication)),
        };
        self.owned_items = self.owned_items.saturating_add(items);
        self.owned_bytes = self.owned_bytes.saturating_add(bytes);
        self.retirements[index].owner = Some(owner);
        Ok(())
    }

    pub fn take(&mut self, actor: ActorId, job: u64) -> Result<JobProgressCheckout<'_>, JobProgressFault> {
        let slot = self.active.iter_mut().find(|slot| slot.occupied && slot.identity.actor == actor && slot.identity.job == job).ok_or(JobProgressFault::Missing)?;
        if slot.awaiting_ack || slot.checked_out {
            return Err(JobProgressFault::Busy);
        }
        let owner = slot.preview.take().ok_or(JobProgressFault::Missing)?;
        slot.checked_out = true;
        let epoch = slot.epoch;
        Ok(JobProgressCheckout { slot, owner: Some(owner), epoch })
    }

    pub fn begin_close_actor(&mut self, actor: ActorId) -> Result<(), JobProgressFault> {
        if self.closing_actors.iter().flatten().any(|current| *current == actor) {
            return Ok(());
        }
        let slot = self.closing_actors.iter_mut().find(|slot| slot.is_none()).ok_or(JobProgressFault::Capacity)?;
        *slot = Some(actor);
        Ok(())
    }

    pub fn begin_close_all(&mut self) {
        self.close_all = true;
    }

    pub fn has_close_work(&self) -> bool {
        self.retirements.iter().any(|slot| slot.owner.is_some()) || self.closing_actors.iter().any(Option::is_some) || self.close_all
    }

    pub fn close_step(&mut self, cx: &mut job::StepContext<'_>) -> JobProgressCloseStep {
        if cx.should_yield() {
            return JobProgressCloseStep::Blocked;
        }
        if let Some(index) = self.retirements.iter().position(|slot| slot.owner.is_some()) {
            cx.consume_fuel(1);
            let (terminal, released_items, released_bytes) = self.retirements[index].owner.as_mut().expect("job progress retirement owner").close_one();
            self.owned_items = self.owned_items.saturating_sub(released_items);
            self.owned_bytes = self.owned_bytes.saturating_sub(released_bytes);
            if terminal {
                let owner = self.retirements[index].owner.take().expect("terminal job progress retirement owner");
                assert!(owner.terminal_is_empty(), "terminal job progress retirement witness changed before removal");
            }
            return JobProgressCloseStep::Pending { released_items, released_bytes };
        }
        let closing_actor = self.closing_actors.iter().flatten().next().copied();
        if let Some(index) = self.active.iter().position(|slot| slot.occupied && (self.close_all || closing_actor == Some(slot.identity.actor))) {
            if self.active[index].checked_out {
                return JobProgressCloseStep::Blocked;
            }
            cx.consume_fuel(1);
            if let Some(owner) = self.active[index].preview.take() {
                let Some(retirement) = self.retirements.iter_mut().find(|slot| slot.owner.is_none() && slot.reservation.is_none()) else {
                    self.active[index].preview = Some(owner);
                    return JobProgressCloseStep::Blocked;
                };
                retirement.owner = Some(owner);
                return JobProgressCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.active[index].clear();
            return JobProgressCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(index) = self.closing_actors.iter().position(Option::is_some) {
            cx.consume_fuel(1);
            self.closing_actors[index] = None;
            return JobProgressCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.close_all {
            cx.consume_fuel(1);
            self.close_all = false;
            return JobProgressCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        JobProgressCloseStep::Complete
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.active.iter().all(|slot| !slot.occupied && slot.reservation.is_none())
            && self.retirements.iter().all(|slot| slot.owner.is_none() && slot.reservation.is_none())
            && self.closing_actors.iter().all(Option::is_none)
            && !self.close_all
            && self.owned_items == 0
            && self.owned_bytes == 0
            && self.reserved_items == 0
            && self.reserved_bytes == 0
    }

    pub fn actor_terminal_is_empty(&self, actor: ActorId) -> bool {
        self.active.iter().all(|slot| !slot.occupied || slot.identity.actor != actor) && self.closing_actors.iter().flatten().all(|closing| *closing != actor) && self.retirements.iter().all(|slot| slot.owner.is_none())
    }
}

/// 🔄️ Borrowed preview owner whose Drop returns the exact owner to its original fixed slot.
pub struct JobProgressCheckout<'a> {
    slot: &'a mut JobProgressSlot,
    owner: Option<JobPublicationOwner>,
    epoch: u64,
}

impl JobProgressCheckout<'_> {
    pub fn identity(&self) -> JobProgressIdentity {
        self.slot.identity
    }

    pub fn preview(&self) -> &[u8] {
        match &self.owner.as_ref().expect("checked-out preview owner").publication.outcome {
            JobStepOutcome::PreviewReady { preview } => preview,
            _ => &[],
        }
    }

    pub fn resume(self) {}
}

impl Drop for JobProgressCheckout<'_> {
    fn drop(&mut self) {
        assert_eq!(self.slot.epoch, self.epoch, "checked-out job preview ABA");
        assert!(self.slot.checked_out && self.slot.preview.is_none(), "checked-out job preview slot changed before handback");
        self.slot.preview = self.owner.take();
        self.slot.checked_out = false;
    }
}
//#endregion 🎨️JobProgressOverlay

/// 🚫️ Publication validation failure detected before actor-visible state can change.
#[derive(Debug)]
pub enum JobPublicationError {
    OperationMismatch { active: u64, published: u64 },
    Stale { live_revision: u64, live_generation: u64 },
    StepSequence { expected: u64, published: u64 },
    PreviewSequence { before: u64, after: u64 },
    Terminal,
    Retained(semio_framework_value::ValueError),
    Original(job::OriginalTurnFailure),
}

impl std::fmt::Display for JobPublicationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OperationMismatch { active, published } => write!(formatter, "job bridge operation mismatch: active={active}, published={published}"),
            Self::Stale { live_revision, live_generation } => write!(formatter, "job bridge stale revision/generation: live revision={live_revision}, generation={live_generation}"),
            Self::StepSequence { expected, published } => write!(formatter, "job bridge step sequence mismatch: expected={expected}, published={published}"),
            Self::PreviewSequence { before, after } => write!(formatter, "job bridge preview cursor mismatch: before={before}, after={after}"),
            Self::Terminal => formatter.write_str("job bridge received a turn after a terminal publication"),
            Self::Retained(error) => std::fmt::Display::fmt(error, formatter),
            Self::Original(error)=>formatter.write_str(error.primary.message()),
        }
    }
}

impl std::error::Error for JobPublicationError {}

#[cfg(test)]
struct JobOutcomeProjection {
    descriptor: job::JobOutcomeDescriptor,
    bytes: [Vec<u8>; 2],
    phase: usize,
    page: usize,
}

#[cfg(test)]
impl JobOutcomeProjection {
    fn new(descriptor: job::JobOutcomeDescriptor) -> Self {
        Self { descriptor, bytes: [Vec::new(), Vec::new()], phase: 0, page: 0 }
    }

    fn step<J: job::InteractiveJob + ?Sized>(&mut self, producer: &J, cx: &mut job::StepContext<'_>) -> Result<Option<JobStepOutcome>, semio_framework_value::ValueError> {
        if !matches!(self.descriptor.kind(), job::JobOutcomeKind::Cancelled) && (cx.is_cancelled() || cx.fuel_remaining() == 0 || cx.latest_us().map(|now| now >= cx.deadline_us()).unwrap_or_else(|| cx.deadline_exceeded())) { return Ok(None); }
        let view = producer.borrow_outcome(&self.descriptor)?;
        let originals = match view {
            job::JobOutcomeView::PreviewReady { payload, .. } => [Some(payload), None],
            job::JobOutcomeView::CheckpointReady { state, .. } => [Some(state), None],
            job::JobOutcomeView::Complete { state, output, .. } => [state, output],
            job::JobOutcomeView::Fault { detail, .. } => [Some(detail), None],
            job::JobOutcomeView::Yield { .. } | job::JobOutcomeView::Cancelled { .. } => [None, None],
        };
        while self.phase < originals.len() {
            if let Some(page) = originals[self.phase].and_then(|original| original.page(self.page)) {
                let destination = &mut self.bytes[self.phase];
                let extent = destination.len().checked_add(page.len()).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original actor projection extent overflow"))?;
                let original_extent = originals[self.phase].unwrap().len();
                if extent > original_extent { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original actor projection exceeded admitted payload extent")); }
                let required = if destination.capacity() == 0 { original_extent } else { 0 };
                let grant = cx.retained_grant();
                if grant.maximum_items == 0 || grant.maximum_copy_bytes < page.len() || grant.maximum_capacity_bytes < required || grant.maximum_depth < 1 { return Ok(None); }
                if required != 0 {
                    let layout = std::alloc::Layout::array::<u8>(required).map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original actor projection layout overflow"))?;
                    let pointer = std::ptr::NonNull::new(unsafe { std::alloc::alloc(layout) }).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed, "original actor projection allocation failed"))?;
                    *destination = unsafe { Vec::from_raw_parts(pointer.as_ptr(), 0, required) };
                }
                let progress = job::RetainedCloneProgress { copied_items: 1, copied_bytes: page.len(), retained_capacity_bytes: required, ..Default::default() };
                cx.consume_retained(progress).map_err(|error| error.with_retained_progress(progress))?;
                destination.extend_from_slice(page);
                self.page += 1;
                return Ok(None);
            }
            self.phase += 1;
            self.page = 0;
        }
        let acknowledged = self.descriptor.acknowledge(cx.retained_grant());
        cx.consume_retained(acknowledged.progress())?;
        if !self.descriptor.is_acknowledged() { return Ok(None); }
        let first = std::mem::take(&mut self.bytes[0]);
        let second = std::mem::take(&mut self.bytes[1]);
        Ok(Some(match self.descriptor.kind() {
            job::JobOutcomeKind::Yield => JobStepOutcome::Yield,
            job::JobOutcomeKind::PreviewReady => JobStepOutcome::PreviewReady { preview: first },
            job::JobOutcomeKind::CheckpointReady { applied_progress } => JobStepOutcome::CheckpointReady { checkpoint: JobCheckpoint { state: first, applied_progress } },
            job::JobOutcomeKind::Complete => JobStepOutcome::Complete { candidate: JobCommitCandidate { state: first, output: second } },
            job::JobOutcomeKind::Cancelled => JobStepOutcome::Cancelled,
            job::JobOutcomeKind::Fault => JobStepOutcome::Fault { detail: first },
        }))
    }
}

/// 🌉️ Stateful actor bridge that advances at most one interactive job step or one retained payload page per turn.
#[cfg(test)]
pub struct JobTurnBridge {
    operation: job::Operation,
    next_step_sequence: u64,
    terminal: bool,
    pending: Option<JobOutcomeProjection>,
    original: job::RetainedCloneGrant,
    progress: job::RetainedCloneProgress,
    authority: Option<job::JobPayloadAuthority>,
}

#[cfg(test)]
impl JobTurnBridge {
    pub fn new(operation: job::Operation, original: job::RetainedCloneGrant) -> Result<Self, semio_framework_value::ValueError> {
        let Some((authority, progress)) = job::JobPayloadAuthority::admit(operation.operation, operation.generation, original)? else {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "actor bridge requires original payload authority admission"));
        };
        Ok(Self { operation, next_step_sequence: 0, terminal: false, pending: None, original, progress, authority: Some(authority) })
    }

    pub fn close_step(&mut self, grant: job::RetainedCloneGrant) -> Result<job::RetainedCloneStep, semio_framework_value::ValueError> {
        self.terminal = true;
        if let Some(pending) = self.pending.as_mut() {
            if !pending.descriptor.is_acknowledged() { return Ok(job::RetainedCloneStep::Progress(pending.descriptor.acknowledge(grant).progress())); }
            if let Some(bytes) = pending.bytes.iter_mut().find(|bytes| bytes.capacity() != 0) {
                let released_bytes = bytes.capacity();
                if grant.maximum_items == 0 || grant.maximum_release_bytes < released_bytes || grant.maximum_depth == 0 { return Ok(job::RetainedCloneStep::Progress(Default::default())); }
                drop(std::mem::take(bytes));
                return Ok(job::RetainedCloneStep::Progress(job::RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() }));
            }
            if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(job::RetainedCloneStep::Progress(Default::default())); }
            self.pending = None;
            return Ok(job::RetainedCloneStep::Progress(job::RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if let Some(authority) = self.authority.as_mut() {
            if !authority.terminal_is_empty() { return authority.close_step(grant).map(|step| job::RetainedCloneStep::Progress(step.progress())); }
            if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(job::RetainedCloneStep::Progress(Default::default())); }
            self.authority = None;
            return Ok(job::RetainedCloneStep::Progress(job::RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        Ok(job::RetainedCloneStep::Complete(Default::default()))
    }

    pub fn terminal_is_empty(&self) -> bool { self.pending.is_none() && self.authority.is_none() }

    #[allow(clippy::too_many_arguments)]
    pub fn step<J: job::InteractiveJob + ?Sized>(
        &mut self,
        job: &mut J,
        turn: JobTurn,
        active_operation: job::OperationId,
        live_revision: job::RevisionId,
        live_generation: job::Generation,
        site: &'static str,
        stage: job::InteractiveStage,
        budget: job::StepBudget,
        cancel: job::CancelToken,
        now_us: fn() -> Option<u64>,
    ) -> Result<JobPublication, JobPublicationError> {
        let next_step_sequence = self.next_step_sequence.checked_add(1).ok_or_else(|| JobPublicationError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original actor bridge sequence exhausted")))?;
        if self.terminal {
            return Err(JobPublicationError::Terminal);
        }
        if turn.operation.operation != active_operation.0 || self.operation.operation != active_operation {
            return Err(JobPublicationError::OperationMismatch { active: active_operation.0, published: turn.operation.operation });
        }
        if turn.step_sequence != self.next_step_sequence {
            return Err(JobPublicationError::StepSequence { expected: self.next_step_sequence, published: turn.step_sequence });
        }
        if !matches!(job::validate_commit(&self.operation, live_revision, live_generation), job::CommitValidation::Accepted)
            || turn.operation.base_revision != live_revision.0
            || turn.operation.generation != live_generation.0
            || turn.operation.base_revision != self.operation.base_revision.0
            || turn.operation.generation != self.operation.generation.0
            || turn.operation.preview_sequence != self.operation.preview_sequence
            || turn.operation.seed != self.operation.seed
        {
            return Err(JobPublicationError::Stale { live_revision: live_revision.0, live_generation: live_generation.0 });
        }
        if budget.retained != self.original {
            return Err(JobPublicationError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "actor bridge requires its original cumulative grant")));
        }
        let authority = self.authority.as_ref().ok_or_else(|| JobPublicationError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "original actor bridge authority has closed")))?;
        let mut context = job::StepContext::with_payload_authority(self.operation.operation, self.operation.generation, budget, &cancel, now_us, &mut self.operation.preview_sequence, &mut self.progress, authority).map_err(JobPublicationError::Retained)?;
        let outcome = if let Some(projection) = self.pending.as_mut() {
            match projection.step(job, &mut context).map_err(JobPublicationError::Retained)? {
                Some(outcome) => { self.pending = None; outcome }
                None => JobStepOutcome::Yield,
            }
        } else {
            let before = context.preview_sequence();
            let outcome = job::drive_step(job, &mut context, site, stage, &mut None).map_err(JobPublicationError::Original)?;
            let after = context.preview_sequence();
            let Some(outcome) = outcome else {
                self.next_step_sequence = next_step_sequence;
                drop(context);
                return Ok(JobPublication { turn: JobTurn { operation: JobOperation::from_job(self.operation), ..turn }, outcome: JobStepOutcome::Yield });
            };
            let mut descriptor = outcome.into_descriptor();
            let preview = matches!(descriptor.kind(), job::JobOutcomeKind::PreviewReady);
            if (preview && after != before.saturating_add(1)) || (!preview && after != before) {
                let acknowledged = descriptor.acknowledge(context.retained_grant());
                context.consume_retained(acknowledged.progress()).map_err(JobPublicationError::Retained)?;
                self.pending = (!descriptor.is_acknowledged()).then(|| JobOutcomeProjection::new(descriptor));
                return Err(JobPublicationError::PreviewSequence { before, after });
            }
            self.pending = Some(JobOutcomeProjection::new(descriptor));
            if matches!(self.pending.as_ref().unwrap().descriptor.kind(), job::JobOutcomeKind::Yield | job::JobOutcomeKind::Cancelled) {
                let projected = self.pending.as_mut().unwrap().step(job, &mut context).map_err(JobPublicationError::Retained)?;
                if let Some(projected) = projected { self.pending = None; projected } else { JobStepOutcome::Yield }
            } else { JobStepOutcome::Yield }
        };
        let terminal = matches!(outcome, JobStepOutcome::Complete { .. } | JobStepOutcome::Cancelled | JobStepOutcome::Fault { .. });
        drop(context);
        let publication = JobPublication { turn: JobTurn { operation: JobOperation::from_job(self.operation), ..turn }, outcome };
        self.next_step_sequence = next_step_sequence;
        self.terminal = terminal;
        Ok(publication)
    }
}
//#endregion 🪪️JobBridge


#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
