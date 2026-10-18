//! 📜️ Replay log: the replay log with its capture, replay and release operations, its drop and the payload accounting and header digest helpers. See [`super`].

use super::*;

//#region 📜️FixedReplay
pub struct JobReplayLog {
    pub(super) route: JobReplayRoute,
    pub(super) generation: u64,
    records: ManuallyDrop<Box<[Option<JobReplayRecord>; JOB_REPLAY_RECORD_CAPACITY]>>,
    record_count: usize,
    reserved_records: usize,
    reserved_pages: usize,
    pub(super) prefix_digest: u64,
    next_ordinal: u64,
    last_checkpoint_index: Option<usize>,
    pub(super) pending: ManuallyDrop<Option<JobReplayCapture>>,
    ready_record: Option<usize>,
    replay_cursor: usize,
    pub(super) replaying: bool,
    pub(super) closing: bool,
    pub(super) close_cursor: usize,
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

    pub fn begin_capture(&mut self, cx: &mut semio_framework_job::StepContext<'_>, actor: ActorId, schedule: JobReplaySchedule, publication: JobPublication) -> Result<(), JobReplayRejected> {
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

    pub fn capture_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> JobReplayStep {
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

    pub fn acknowledge_publication(&mut self, cx: &mut semio_framework_job::StepContext<'_>, policy: JobReplayPublicationPolicy) -> Result<(), JobReplayFault> {
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

    pub fn maintenance_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> JobReplayStep {
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

    pub fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> JobReplayCloseStep {
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

pub(super) fn replay_payload_items(publication: &JobPublication) -> usize {
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
