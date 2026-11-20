//! 🔁️ Registered retained jobs under an original fixture caller: wire pages, snapshot reads with their issuer, bus admission, outcome stepping and the funded close ladder.
//!
//! @see `semio_framework_action_bus::ActionBus::admit_job`, `semio_framework_job::OriginalJobFixtureCaller`.

use super::original_grant;
use semio_framework_action_bus::{admit_original_tool_construction_settled, ActionBus, ErasedToolJob, RetainedToolWireInput, ToolFactoryKey, ToolJobAdmissionDemand, ToolOperationSpec, ToolWirePage, TOOL_WIRE_PAGE_BYTES};
use semio_framework_job::{close_original_effect_step, drive_step, InteractiveJob, InteractiveStage, JobOutcomeView, Operation, OriginalConstructionAdmission, OriginalConstructionStatus, OriginalJobDecodeOwner, OriginalJobFixtureCaller, OriginalJobReturn, OriginalJobReturnOrigin, RetainedCloneProgress};
use semio_framework_value_snapshot_read::{snapshot_registry_alias_close_step, snapshot_registry_alias_demands, SnapshotRead, SnapshotReadRegistryAliasRetirement, SnapshotReadRegistryHandle};
use semio_framework_value::{retirement::controlled::ControlledRetirement, RetirementDemand};
use std::sync::Arc;

/// 🎟️ One original caller with its decode owner, closed together by [`Session::finish`].
pub struct Session {
    pub caller: OriginalJobFixtureCaller,
    pub decoder: OriginalJobDecodeOwner,
    pub origin: OriginalJobReturnOrigin,
}

impl Session {
    pub fn new(operation: Operation) -> Self {
        let mut caller = OriginalJobFixtureCaller::admit(operation.operation, operation.generation, original_grant(), || Some(0), u64::MAX, 1_000_000);
        let decoder = caller.admitted_decoder();
        Self { caller, decoder, origin: OriginalJobReturnOrigin::empty() }
    }

    /// 🏁️ Retires the decode owner through funded turns, then the caller.
    pub fn finish(mut self) {
        for _ in 0..4096 {
            if self.origin.terminal_is_empty() {
                break;
            }
            let origin = &mut self.origin;
            self.caller.turn_with_decoder(original_grant(), &mut self.decoder, |cx| origin.close_step(cx).map(|_| ())).expect("fixture return origin closes");
        }
        assert!(self.origin.terminal_is_empty(), "fixture return origin kept custody after 4096 close steps");
        for _ in 0..4096 {
            if self.decoder.terminal_is_empty() {
                break;
            }
            self.caller.turn(original_grant(), |cx| self.decoder.close_on_source(cx).map(|_| ())).expect("fixture decode owner closes");
        }
        assert!(self.decoder.terminal_is_empty(), "fixture decode owner kept custody after 4096 close steps");
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
                    let demand = snapshot_registry_alias_demands(&retirement)?;
                    close_original_effect_step(cx, demand, |effect| snapshot_registry_alias_close_step(&mut retirement, effect))?;
                    Ok(())
                })
                .expect("snapshot issuer closes");
        }
    }
}

/// 🚪️ Admits `payload` with its wire and optional checkpoint through the registered factory of `key` and returns the erased job.
pub fn admit<P: Send + 'static>(bus: &ActionBus, session: &mut Session, operation: Operation, key: ToolFactoryKey, schema: &str, payload: P, input: Option<RetainedToolWireInput>, checkpoint: Option<RetainedToolWireInput>) -> ErasedToolJob {
    let capacity = size_of::<P>();
    let demand = ToolJobAdmissionDemand { items: 1, copy_bytes: 0, capacity_bytes: capacity, release_bytes: 0, depth: 1 };
    let mut spec = None;
    let mut construction = OriginalConstructionAdmission::default();
    let schema = schema.to_owned();
    let born = session.caller.turn_with_decoder(original_grant(), &mut session.decoder, |cx| {
        admit_original_tool_construction_settled(cx, &mut construction, &mut spec, demand, |scope| {
            scope.admit_owner(RetirementDemand { capacity_bytes: capacity, depth: 1, ..Default::default() })?;
            let spec = ToolOperationSpec::from_owned(key, schema, Box::new(payload), operation, scope.identity());
            Ok((spec, RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: capacity, ..Default::default() }))
        })
    }).expect("fixture specification");
    assert!(born && construction.terminal_is_empty());

    let mut spec = spec.expect("specification born");
    let (mut input, mut checkpoint, mut output) = (input, checkpoint, None);
    let mut ready = false;
    for _ in 0..64 {
        let status = session.caller.turn_with_decoder(original_grant(), &mut session.decoder, |cx| {
            bus.admit_job(cx, &mut construction, &mut spec, &mut input, &mut checkpoint, &mut output)
        }).expect("registered factory admits");
        match status {
            OriginalConstructionStatus::Pending => continue,
            OriginalConstructionStatus::Ready => ready = true,
            OriginalConstructionStatus::Failed => panic!("registered fixture construction refused"),
        }
        break;
    }
    assert!(ready && input.is_none() && checkpoint.is_none() && spec.payload.retained_payload().is_none());
    for _ in 0..64 {
        if construction.terminal_is_empty() {
            break;
        }
        session.caller.turn_with_decoder(original_grant(), &mut session.decoder, |cx| construction.close_step(cx)).expect("registered construction closes");
    }
    assert!(construction.terminal_is_empty());

    let mut fields = ControlledRetirement::retain((spec.key.controller_id, spec.key.tool_id, spec.payload.schema_id));
    for _ in 0..4096 {
        if fields.terminal_is_empty() {
            break;
        }
        session.caller.turn(original_grant(), |cx| {
            let body = cx.retained_grant().maximum_copy_bytes;
            let demand = fields.selected_demand(body)?;
            close_original_effect_step(cx, demand, |effect| fields.step(body, effect))?;
            Ok(())
        }).expect("fixture specification fields close");
    }
    assert!(fields.terminal_is_empty());
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
