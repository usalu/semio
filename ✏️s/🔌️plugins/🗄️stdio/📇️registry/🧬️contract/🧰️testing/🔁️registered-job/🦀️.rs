//! 🔁️ Registered retained jobs under an original fixture caller: wire pages, snapshot reads with their issuer, bus admission, outcome stepping and the funded close ladder.
//!
//! @see `semio_framework_action_bus::ActionBus::admit_job`, `semio_framework_job::OriginalJobFixtureCaller`.

use super::original_grant;
use semio_framework_action_bus::{admit_original_tool_construction, ActionBus, ErasedToolJob, RetainedToolWireInput, ToolFactoryKey, ToolJobAdmissionDemand, ToolOperationSpec, ToolWirePage, TOOL_WIRE_PAGE_BYTES};
use semio_framework_job::{drive_step, InteractiveJob, InteractiveStage, JobOutcomeView, Operation, OriginalJobDecodeOwner, OriginalJobFixtureCaller, OriginalJobReturn, OriginalJobReturnOrigin, RetainedCloneProgress};
use semio_framework_value_snapshot_read::{snapshot_registry_alias_close_step, SnapshotRead, SnapshotReadRegistryAliasRetirement, SnapshotReadRegistryHandle};
use std::sync::Arc;

/// 🎟️ One original caller with its decode owner, closed together by [`Session::finish`].
pub struct Session {
    pub caller: OriginalJobFixtureCaller,
    pub decoder: OriginalJobDecodeOwner,
    pub origin: OriginalJobReturnOrigin,
}

impl Session {
    pub fn new(operation: Operation) -> Self {
        Self { caller: OriginalJobFixtureCaller::admit(operation.operation, operation.generation, original_grant(), || Some(0), u64::MAX, 1_000_000), decoder: OriginalJobDecodeOwner::empty(), origin: OriginalJobReturnOrigin::empty() }
    }

    /// 🏁️ Retires the decode owner through funded turns, then the caller.
    pub fn finish(mut self) {
        while !self.decoder.terminal_is_empty() {
            let decoder = &mut self.decoder;
            self.caller
                .turn(original_grant(), |cx| {
                    let turn = decoder.close_step(cx.retained_grant());
                    let step = turn.result?;
                    cx.consume_retained(step.progress())?;
                    cx.consume_fuel(turn.work);
                    Ok(())
                })
                .expect("fixture decode owner closes");
        }
        for _ in 0..4096 {
            if self.origin.terminal_is_empty() {
                break;
            }
            let origin = &mut self.origin;
            self.caller.turn(original_grant(), |cx| origin.close_step(cx).map(|_| ())).expect("fixture return origin closes");
        }
        assert!(self.origin.terminal_is_empty(), "fixture return origin kept custody after 4096 close steps");
        self.caller.retire(original_grant());
    }
}

/// 📄️ Sealed wire pages of `bytes` as one retained tool input.
pub fn wire(bytes: &[u8]) -> RetainedToolWireInput {
    let (mut input, _) = RetainedToolWireInput::admit_original(bytes.len(), bytes.len(), original_grant()).expect("wire extent").expect("wire extent admitted");
    for chunk in bytes.chunks(TOOL_WIRE_PAGE_BYTES) {
        input.admit_page(ToolWirePage::try_copy_from(chunk).expect("wire page")).ok().expect("wire page admitted");
    }
    input.seal().expect("wire sealed");
    input
}

/// 👁️ The exact issuer of one snapshot read, returned by [`SnapshotLease::close`] after the job closed.
pub struct SnapshotLease {
    registry: Option<SnapshotReadRegistryHandle>,
    ticket: Option<(u16, u64)>,
}

/// 📸️ An immutable snapshot read published under `generation` and `revision`, with the issuer that must outlive the job.
pub fn snapshot_read<T: Send + Sync + 'static>(snapshot: Arc<T>, generation: u64, revision: [u8; 32]) -> (SnapshotRead<T>, SnapshotLease) {
    let registry = SnapshotReadRegistryHandle::new();
    assert!(registry.publish_authority(generation, revision));
    let lease = registry.try_issue(snapshot.clone()).unwrap_or_else(|_| panic!("snapshot issuer refused the exact source"));
    let read = SnapshotRead::new(snapshot, lease);
    let (_, index, generation) = read.lease_identity().expect("snapshot lease identity");
    (read, SnapshotLease { registry: Some(registry), ticket: Some((index, generation)) })
}

impl SnapshotLease {
    pub fn close(mut self, caller: &mut OriginalJobFixtureCaller) {
        let registry = self.registry.as_ref().expect("issuer");
        if let Some((index, generation)) = self.ticket.take() {
            if registry.contains(index, generation) {
                caller
                    .turn(original_grant(), |cx| {
                        drop(registry.try_take(index, generation)?);
                        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: std::mem::size_of::<Arc<dyn std::any::Any + Send + Sync>>(), ..Default::default() })?;
                        cx.consume_fuel(1);
                        Ok(())
                    })
                    .expect("snapshot lease returns");
            }
        }
        assert!(registry.terminal_is_empty());
        let mut retirement = Some(SnapshotReadRegistryAliasRetirement::new(self.registry.take().expect("issuer")));
        while retirement.is_some() {
            caller
                .turn(original_grant(), |cx| {
                    let step = snapshot_registry_alias_close_step(&mut retirement, cx.retained_grant())?;
                    cx.consume_retained(step.progress())?;
                    cx.consume_fuel(step.progress().copied_items as u64);
                    Ok(())
                })
                .expect("snapshot issuer closes");
        }
    }
}

/// 🚪️ Admits `payload` with its wire and optional checkpoint through the registered factory of `key` and returns the erased job.
pub fn admit<P: Send + 'static>(bus: &ActionBus, session: &mut Session, operation: Operation, key: ToolFactoryKey, schema: &str, payload: P, input: Option<RetainedToolWireInput>, checkpoint: Option<RetainedToolWireInput>) -> ErasedToolJob {
    let capacity = std::mem::size_of::<P>();
    let mut spec = None;
    let born = session
        .caller
        .turn_with_decoder(original_grant(), &mut session.decoder, |cx| {
            admit_original_tool_construction(cx, &mut spec, ToolJobAdmissionDemand { items: 1, copy_bytes: 0, capacity_bytes: capacity, release_bytes: 0, depth: 1 }, |identity| {
                Ok((ToolOperationSpec::from_owned(key, schema.to_string(), Box::new(payload), operation, identity), RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: capacity, ..Default::default() }))
            })
        })
        .expect("fixture specification");
    assert!(born);
    let mut spec = spec.expect("specification born");
    let (mut input, mut checkpoint, mut output) = (input, checkpoint, None);
    let admitted = session.caller.turn_with_decoder(original_grant(), &mut session.decoder, |cx| bus.admit_job(cx, &mut spec, &mut input, &mut checkpoint, &mut output)).expect("registered factory admits");
    assert!(admitted && input.is_none() && checkpoint.is_none() && spec.payload.retained_payload().is_none());
    output.expect("job born")
}

/// 🔭️ What one step of a job yielded, with the checkpoint bytes copied out of the borrowed outcome.
#[derive(Debug, PartialEq, Eq)]
pub enum Seen {
    Idle,
    Yield,
    Preview,
    Checkpoint(Option<Vec<u8>>),
    Complete,
    Cancelled,
    Fault,
}

const STEP_SITE: &str = "stdio-contract-testing registered job";

/// 🚚️ Drives `job` until its outcome is received by the session origin and answers the stage the driving turn reached.
fn drive(job: &mut ErasedToolJob, session: &mut Session) -> String {
    let Session { caller, decoder, origin } = session;
    let mut returned = None;
    let mut continuation: Option<semio_framework_job::OriginalJobContinuation> = None;
    let mut stage = String::new();
    for _ in 0..4096 {
        let received = caller
            .turn_with_decoder(original_grant(), decoder, |cx| {
                cx.with_original_work_loan(1, cx.original_absolute_deadline_us(), |cx| {
                    if returned.is_none() && continuation.is_none() {
                        if !origin.body_ready() {
                            origin.admit(cx)?;
                            return Ok(false);
                        }
                        let (_, receiver) = origin.context_parts()?;
                        let driven = drive_step(&mut *job, cx, receiver, STEP_SITE, InteractiveStage::InteractiveStep, &mut None)?;
                        stage = cx.stage().to_string();
                        match driven {
                            OriginalJobReturn::Preparing | OriginalJobReturn::Pending => return Ok(false),
                            OriginalJobReturn::Outcome(reply) => returned = Some(reply.into_descriptor()),
                            OriginalJobReturn::Continuation(_) => panic!("job step returned an original continuation"),
                        }
                    }
                    if returned.is_some() { origin.receive_outcome(cx, &mut returned) } else { origin.receive_continuation(cx, &mut continuation) }
                })
            })
            .expect("job step");
        if received {
            return stage;
        }
    }
    panic!("job step never reached its receiving origin");
}

/// 🔔️ Acknowledges the outcome the origin holds so the next step rearms the same source.
fn acknowledge(session: &mut Session) {
    let Session { caller, decoder, origin } = session;
    for _ in 0..4096 {
        if origin.outcome().is_none() {
            return;
        }
        caller
            .turn_with_decoder(original_grant(), decoder, |cx| cx.with_original_work_loan(1, cx.original_absolute_deadline_us(), |cx| origin.acknowledge_outcome(cx)))
            .expect("job outcome acknowledged");
    }
    panic!("job outcome never acknowledged");
}

/// 👣️ One funded step of `job`.
pub fn step(job: &mut ErasedToolJob, session: &mut Session) -> Seen {
    drive(job, session);
    let Some(descriptor) = session.origin.outcome() else { return Seen::Idle };
    let seen = match job.borrow_outcome(descriptor).expect("job outcome") {
        JobOutcomeView::Yield { .. } => Seen::Yield,
        JobOutcomeView::PreviewReady { .. } => Seen::Preview,
        JobOutcomeView::CheckpointReady { state, .. } => Seen::Checkpoint(state.single_page().map(<[u8]>::to_vec)),
        JobOutcomeView::Complete { .. } => Seen::Complete,
        JobOutcomeView::Cancelled { .. } => Seen::Cancelled,
        JobOutcomeView::Fault { .. } => Seen::Fault,
    };
    acknowledge(session);
    seen
}

/// 🪜️ One funded step of `job`, dropping its outcome and answering the stage the step context reached.
pub fn step_stage(job: &mut ErasedToolJob, session: &mut Session) -> String {
    let stage = drive(job, session);
    acknowledge(session);
    stage
}

/// 🧯️ Closes `job` through funded turns until nothing is left in its custody.
pub fn close(job: &mut ErasedToolJob, session: &mut Session) {
    job.begin_close();
    for _ in 0..4096 {
        if job.terminal_is_empty() {
            return;
        }
        session.caller.turn_with_decoder(original_grant(), &mut session.decoder, |cx| job.close_step(cx)).expect("job close step");
    }
    panic!("job kept custody after 4096 close steps");
}
