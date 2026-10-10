//! 💡️ `semio.infer` cold-job bridge. Exact ActionBus routes such as `s.wfc.wfc3d.solve`
//! decode through their factory-owned schema and retain one persistent `InteractiveJob` session.
//! Every guest continuation admits exactly one bounded step to the shared WorkerPool; previews
//! coalesce, checkpoints and commits remain lossless under explicit item/byte bounds, and
//! diagnostics use a bounded ring. Inferences without an ActionBus route retain the synchronous
//! two-phase registry path.

use super::{BoundedJob, JobBudget, JobStep, WORK_UNITS_EXECUTE, WORK_UNITS_PUMP, WORK_UNITS_RETIRE};
use semio_framework_job::{Generation, Operation, OperationId, RevisionId, StepOutcome, JobOutcomeSlot, close_step_outcome_slot};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value_derive::ToValue;
use std::collections::VecDeque;

const PREVIEW_MAX_BYTES: usize = 1 << 20;
#[cfg(test)]
const LOSSLESS_MAX_ITEMS: usize = 2;
const LOSSLESS_MAX_BYTES: usize = 2 << 20;
const DIAGNOSTIC_MAX_ITEMS: usize = 32;
const DIAGNOSTIC_MAX_BYTES: usize = 64 << 10;

//#region 🌉️Channels
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, ToValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
enum InferenceBridgeKind {
    Scheduled,
    Preview,
    Diagnostic,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, ToValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct InferenceBridgeItem {
    kind: InferenceBridgeKind,
    operation: u64,
    generation: u64,
    sequence: u64,
    payload: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg(test)]
enum LosslessInferenceItem {
    Checkpoint(semio_framework_job::Checkpoint),
    #[cfg(test)]
    TestBytes(usize),
}

#[cfg(test)]
impl LosslessInferenceItem {
    fn byte_len(&self) -> usize {
        match self {
            Self::Checkpoint(checkpoint) => checkpoint.state.len(),
            #[cfg(test)]
            Self::TestBytes(bytes) => *bytes,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum InferenceBridgeError {
    Oversized {
        channel: &'static str,
        bytes: usize,
        max_bytes: usize,
    },
    #[cfg(test)]
    Saturated {
        channel: &'static str,
        items: usize,
        bytes: usize,
    },
}

impl std::fmt::Display for InferenceBridgeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Oversized { channel, bytes, max_bytes } => write!(formatter, "{channel} item has {bytes} bytes, above {max_bytes}"),
            #[cfg(test)]
            Self::Saturated { channel, items, bytes } => write!(formatter, "{channel} is saturated at {items} items/{bytes} bytes"),
        }
    }
}

struct InferenceBridge {
    operation: Operation,
    sequence: u64,
    preview: Option<InferenceBridgeItem>,
    #[cfg(test)]
    lossless: [Option<LosslessInferenceItem>; LOSSLESS_MAX_ITEMS],
    #[cfg(test)]
    lossless_len: usize,
    #[cfg(test)]
    lossless_bytes: usize,
    diagnostics: VecDeque<InferenceBridgeItem>,
    diagnostic_bytes: usize,
}

impl InferenceBridge {
    fn new(operation: Operation) -> Self {
        Self {
            operation,
            sequence: 0,
            preview: None,
            #[cfg(test)]
            lossless: std::array::from_fn(|_| None),
            #[cfg(test)]
            lossless_len: 0,
            #[cfg(test)]
            lossless_bytes: 0,
            diagnostics: VecDeque::new(),
            diagnostic_bytes: 0,
        }
    }

    fn item(&mut self, kind: InferenceBridgeKind, payload: Vec<u8>) -> InferenceBridgeItem {
        let item = InferenceBridgeItem { kind, operation: self.operation.operation.0, generation: self.operation.generation.0, sequence: self.sequence, payload };
        self.sequence = self.sequence.saturating_add(1);
        item
    }

    fn publish_preview(&mut self, payload: Vec<u8>) -> Result<(), InferenceBridgeError> {
        if payload.len() > PREVIEW_MAX_BYTES {
            return Err(InferenceBridgeError::Oversized { channel: "preview", bytes: payload.len(), max_bytes: PREVIEW_MAX_BYTES });
        }
        let item = self.item(InferenceBridgeKind::Preview, payload);
        self.preview = Some(item);
        Ok(())
    }

    fn take_preview(&mut self) -> Option<InferenceBridgeItem> {
        self.preview.take()
    }

    #[cfg(test)]
    fn publish_lossless(&mut self, item: LosslessInferenceItem) -> Result<(), InferenceBridgeError> {
        let bytes = item.byte_len();
        if bytes > LOSSLESS_MAX_BYTES {
            return Err(InferenceBridgeError::Oversized { channel: "checkpoint-commit", bytes, max_bytes: LOSSLESS_MAX_BYTES });
        }
        let total_bytes = self.lossless_bytes.checked_add(bytes).ok_or(InferenceBridgeError::Saturated { channel: "checkpoint-commit", items: self.lossless_len, bytes: self.lossless_bytes })?;
        if self.lossless_len >= LOSSLESS_MAX_ITEMS || total_bytes > LOSSLESS_MAX_BYTES {
            return Err(InferenceBridgeError::Saturated { channel: "checkpoint-commit", items: self.lossless_len, bytes: self.lossless_bytes });
        }
        self.lossless[self.lossless_len] = Some(item);
        self.lossless_len += 1;
        self.lossless_bytes = total_bytes;
        Ok(())
    }

    #[cfg(test)]
    fn take_lossless(&mut self) -> Option<LosslessInferenceItem> {
        let item = self.lossless[0].take()?;
        for index in 1..self.lossless_len {
            self.lossless[index - 1] = self.lossless[index].take();
        }
        self.lossless_len -= 1;
        self.lossless_bytes = self.lossless_bytes.saturating_sub(item.byte_len());
        Some(item)
    }

    fn publish_diagnostic(&mut self, payload: Vec<u8>) {
        if payload.len() > DIAGNOSTIC_MAX_BYTES {
            return;
        }
        while self.diagnostics.len() >= DIAGNOSTIC_MAX_ITEMS || self.diagnostic_bytes.saturating_add(payload.len()) > DIAGNOSTIC_MAX_BYTES {
            let Some(removed) = self.diagnostics.pop_front() else {
                break;
            };
            self.diagnostic_bytes = self.diagnostic_bytes.saturating_sub(removed.payload.len());
        }
        self.diagnostic_bytes = self.diagnostic_bytes.saturating_add(payload.len());
        let item = self.item(InferenceBridgeKind::Diagnostic, payload);
        self.diagnostics.push_back(item);
    }

    fn scheduled(&mut self) -> InferenceBridgeItem {
        self.item(InferenceBridgeKind::Scheduled, Vec::new())
    }

    fn latest_diagnostic(&self) -> Option<&InferenceBridgeItem> {
        self.diagnostics.back()
    }
}

fn encode_bridge_item(item: &InferenceBridgeItem) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(1 + 8 * 3 + 4 + item.payload.len());
    bytes.push(match item.kind {
        InferenceBridgeKind::Scheduled => 0,
        InferenceBridgeKind::Preview => 1,
        InferenceBridgeKind::Diagnostic => 2,
    });
    bytes.extend_from_slice(&item.operation.to_le_bytes());
    bytes.extend_from_slice(&item.generation.to_le_bytes());
    bytes.extend_from_slice(&item.sequence.to_le_bytes());
    bytes.extend_from_slice(&(item.payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&item.payload);
    bytes
}
//#endregion 🌉️Channels

type InferenceSession = semio_framework_job::MountedWorkerJobSession<semio_framework::action_bus::ErasedToolJob>;
type InferenceRejected = semio_framework_job::WorkerJobSessionAdmissionRejected<semio_framework::action_bus::ErasedToolJob>;

// 🚫️async: E4 fn-pointer slot — registered into `BoundedJobFactory` (see
// `⚛️reactor/💼️jobs/🦀️.rs`'s `builtin_registry`); the admission body routes and constructs only.
pub(super) fn job_infer(_job:u64,input:&mut Option<Vec<u8>>,restored:&mut Option<Vec<u8>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<Box<dyn BoundedJob>>,semio_framework_value::ValueError>{
    let operation=cx.operation().0;let generation=cx.generation().0;
    super::admit_original_job(input,restored,cx,move|input,restored|OriginalInferenceJob::new(input,restored,operation,generation))
}

/// ♻️ Quotes the actual boxed original inference birth while the caller keeps both source handles.
pub(super) fn job_infer_demands(_job:u64,input:&Option<Vec<u8>>,restored:&Option<Vec<u8>>,_cx:&semio_framework_job::StepContext<'_>)->Result<RetainedCloneGrant,semio_framework_value::ValueError>{super::original_job_admission_demands::<OriginalInferenceJob>(input,restored)}

/// 🎒️ The exact raw input and checkpoint remain owned until the original gateway and source close.
struct OriginalInferenceJob {
    input:Option<Vec<u8>>,restored:Option<Vec<u8>>,gateway:Option<crate::ArtifactInferenceGateway>,output:Option<Vec<u8>>,
    failure:Option<crate::ArtifactInferenceGatewayFailure>,closing_failure:Option<crate::ArtifactInferenceGatewayFailure>,reply:Option<crate::extension_invocation_failure::RetainedExtensionFaultReply>,
    secondary:Option<semio_framework_value::ValueError>,active:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,cancelled:bool,
}
impl OriginalInferenceJob {
    fn new(input:Vec<u8>,restored:Option<Vec<u8>>,operation:u64,generation:u64)->Self{let gateway=crate::ArtifactInferenceGateway::new(operation,generation,&input);Self{input:Some(input),restored,gateway:Some(gateway),output:None,failure:None,closing_failure:None,reply:None,secondary:None,active:None,cancelled:false}}
    fn close_field<T:semio_framework_value::retirement::RetireOwned>(source:&mut Option<T>,active:&mut Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,semio_framework_value::ValueError>{
        use semio_framework_value::{retirement::admit_owned_retirement,close_factory_ticket,factory_ticket_demands};
        if active.is_some(){let demand=factory_ticket_demands(active.as_ref().unwrap(),grant.maximum_copy_bytes)?;if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(Default::default())}return close_factory_ticket(active,grant).map(|step|step.progress());}
        let Some(original)=source.as_ref()else{return Ok(Default::default())};let capacity=original.retirement_birth_bytes().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original inference source has no declared close birth"))?;
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<capacity{return Ok(Default::default())}
        match admit_owned_retirement(source.take().unwrap(),grant){Ok((owner,progress))=>{*active=Some(owner);Ok(progress)},Err((error,original))=>{*source=Some(original);Err(error)}}
    }
    fn close_sources(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_value::ValueError>{
        let grant=cx.retained_grant();let result=if self.closing_failure.is_some(){Self::close_field(&mut self.closing_failure,&mut self.active,grant)}else if self.secondary.is_some(){Self::close_field(&mut self.secondary,&mut self.active,grant)}else if self.input.is_some()||self.active.is_some(){Self::close_field(&mut self.input,&mut self.active,grant)}else if self.restored.is_some(){Self::close_field(&mut self.restored,&mut self.active,grant)}else{return Ok(true)};
        let progress=match result{Ok(progress)=>progress,Err(error)=>{cx.consume_retained(error.retained_progress())?;return Err(error)}};cx.consume_retained(progress)?;cx.consume_fuel(1);Ok(false)
    }
}
impl OriginalInferenceJob {
    fn step_original(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->JobStep{
        if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return JobStep::Running(None)}
        if let Some(gateway)=&mut self.gateway{
            if self.cancelled||self.failure.is_some(){gateway.begin_close();}
            if self.closing_failure.is_some(){match Self::close_field(&mut self.closing_failure,&mut self.active,cx.retained_grant()){Ok(progress)=>{if let Err(error)=cx.consume_retained(progress){self.secondary=Some(error);}cx.consume_fuel(1);},Err(error)=>self.secondary=Some(error)}return JobStep::Running(None)}
            if self.secondary.is_some()||self.active.is_some(){match Self::close_field(&mut self.secondary,&mut self.active,cx.retained_grant()){Ok(progress)=>{if let Err(error)=cx.consume_retained(progress){self.secondary=Some(error);}cx.consume_fuel(1);},Err(error)=>self.secondary=Some(error)}return JobStep::Running(None)}
            let answer=crate::wire_artifact_infer(gateway,self.input.as_ref().unwrap(),cx);
            if let Some(cause)=answer.refusal{if self.failure.is_none(){self.failure=Some(cause);}else{assert!(self.closing_failure.is_none());self.closing_failure=Some(cause);}gateway.begin_close();}
            if let Some(payload)=answer.payload{assert!(self.output.is_none());self.output=Some(payload);}
            if gateway.terminal_is_empty(){self.gateway=None;}
            return JobStep::Running(None);
        }
        match self.close_sources(cx){Ok(false)=>return JobStep::Running(None),Err(error)=>{assert!(self.secondary.is_none());self.secondary=Some(error);return JobStep::Running(None)},Ok(true)=>{}}
        if self.failure.is_some()&&self.reply.is_none(){self.reply=Some(crate::extension_invocation_failure::RetainedExtensionFaultReply::new(crate::extension_invocation_failure::ExtensionInvocationCause::Inference(self.failure.take().unwrap())));if let Err(error)=cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()}){self.secondary=Some(error);}return JobStep::Running(None)}
        if self.cancelled&&self.output.is_some(){match Self::close_field(&mut self.output,&mut self.active,cx.retained_grant()){Ok(progress)=>{if let Err(error)=cx.consume_retained(progress){self.secondary=Some(error);}},Err(error)=>self.secondary=Some(error)}return JobStep::Running(None)}
        if self.cancelled&&self.reply.is_none(){self.reply=Some(crate::extension_invocation_failure::RetainedExtensionFaultReply::new(crate::extension_invocation_failure::ExtensionInvocationCause::Value(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"original inference cancelled"))));return JobStep::Running(None)}
        if let Some(reply)=&mut self.reply{match reply.advance(cx){Ok(Some(bytes))=>{assert!(reply.terminal_is_empty());self.reply=None;return JobStep::Failed(bytes)},Ok(None)=>return JobStep::Running(None),Err(error)=>{self.secondary=Some(error);return JobStep::Running(None)}}}
        if self.output.is_some(){if let Err(error)=cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()}){self.secondary=Some(error);return JobStep::Running(None)}return JobStep::Done(self.output.take().unwrap())}
        JobStep::Running(None)
    }
 }
impl BoundedJob for OriginalInferenceJob {
    fn step(&mut self,_budget:JobBudget,_original:&mut super::IoRunControl<'_,'_>,_snapshot:&mut super::SqliteSnapshotControl<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<JobStep,semio_framework_value::ValueError>{let step=self.step_original(cx);match self.secondary.take(){Some(error)=>Err(error),None=>Ok(step)}}
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_value::ValueError>{
        if self.terminal_drop_is_shallow(){return Ok(true)}
        if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}
        self.cancelled=true;
        if self.active.is_some()||self.closing_failure.is_some()||self.secondary.is_some(){
            let result=if self.active.is_some(){Self::close_field(&mut self.input,&mut self.active,cx.retained_grant())}else if self.closing_failure.is_some(){Self::close_field(&mut self.closing_failure,&mut self.active,cx.retained_grant())}else{Self::close_field(&mut self.secondary,&mut self.active,cx.retained_grant())};
            match result{Ok(progress)=>cx.consume_retained(progress)?,Err(error)=>{cx.consume_retained(error.retained_progress())?;return Err(error)}}cx.consume_fuel(1);return Ok(false)
        }
        if let Some(gateway)=&mut self.gateway{
            gateway.begin_close();
            if gateway.terminal_is_empty(){cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;self.gateway=None;return Ok(false)}
            let answer=crate::wire_artifact_infer(gateway,self.input.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original inference close lost its raw input"))?,cx);
            if let Some(payload)=answer.payload{assert!(self.output.is_none());self.output=Some(payload);}
            if let Some(cause)=answer.refusal{if self.failure.is_none(){self.failure=Some(cause)}else{assert!(self.closing_failure.is_none());self.closing_failure=Some(cause)}}
            return Ok(false)
        }
        if let Some(reply)=&mut self.reply{reply.begin_close();if reply.terminal_is_empty(){cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;self.reply=None;return Ok(false)}assert!(reply.advance(cx)?.is_none());return Ok(false)}
        let grant=cx.retained_grant();let result=if self.active.is_some(){Self::close_field(&mut self.input,&mut self.active,grant)}else if self.closing_failure.is_some(){Self::close_field(&mut self.closing_failure,&mut self.active,grant)}else if self.secondary.is_some(){Self::close_field(&mut self.secondary,&mut self.active,grant)}else if self.failure.is_some(){Self::close_field(&mut self.failure,&mut self.active,grant)}else if self.output.is_some(){Self::close_field(&mut self.output,&mut self.active,grant)}else if self.input.is_some(){Self::close_field(&mut self.input,&mut self.active,grant)}else if self.restored.is_some(){Self::close_field(&mut self.restored,&mut self.active,grant)}else{return Ok(true)};
        match result{Ok(progress)=>{cx.consume_retained(progress)?;cx.consume_fuel(1);Ok(false)},Err(error)=>{cx.consume_retained(error.retained_progress())?;Err(error)}}
    }
    fn retirement_demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        use semio_framework_value::{RetirementDemand,retirement::RetireOwned};
        fn source<T:RetireOwned>(source:&T)->Result<RetirementDemand,semio_framework_value::ValueError>{Ok(RetirementDemand{capacity_bytes:source.retirement_birth_bytes().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original inference source has no declared close birth"))?,depth:1,..Default::default()})}
        if let Some(active)=&self.active{return semio_framework_value::factory_ticket_demands(active,copy)}
        if let Some(value)=&self.closing_failure{return source(value)}if let Some(value)=&self.secondary{return source(value)}
        if let Some(gateway)=&self.gateway{return if gateway.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{crate::original_inference_gateway_demands(gateway,copy)}}
        if let Some(reply)=&self.reply{return reply.demands(copy)}
        if let Some(value)=&self.closing_failure{return source(value)}if let Some(value)=&self.secondary{return source(value)}if let Some(value)=&self.failure{return source(value)}if let Some(value)=&self.output{return source(value)}if let Some(value)=&self.input{return source(value)}if let Some(value)=&self.restored{return source(value)}Ok(Default::default())
    }
    fn cancel(&mut self){self.cancelled=true;if let Some(gateway)=&mut self.gateway{gateway.begin_close();}}
    fn checkpoint(&self)->Option<Vec<u8>>{None}
    fn terminal_drop_is_shallow(&self)->bool{self.input.is_none()&&self.restored.is_none()&&self.gateway.is_none()&&self.output.is_none()&&self.failure.is_none()&&self.closing_failure.is_none()&&self.reply.is_none()&&self.secondary.is_none()&&self.active.is_none()}
}
impl Drop for OriginalInferenceJob{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_drop_is_shallow(),"original inference job dropped before full source closure");}}

// 🚫️async: E1 pure parse consumed by the sync factory above and by `decode`'s own body.
fn decode_request(input: &[u8]) -> Result<crate::app::WireArtifactInferenceRequest, semio_framework::Fault> {
    let input_text = std::str::from_utf8(input).map_err(|error| super::fault("job.infer.decode", format!("invalid {} input: {error}", super::JOB_KIND_INFER)))?;
    semio_framework_pack_json::from_json_str(input_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| super::fault("job.infer.decode", format!("invalid {} input: {error}", super::JOB_KIND_INFER)))
}

/// 🚦️ The explicit states of one ActionBus-routed inference. Each is exactly one `step-job`
/// opportunity: `Dispatch` admits the worker session, `Pump` advances it by as many bounded worker
/// steps as the crossing's grant covers, `OutcomeClose` retires one page of the checked-out outcome, `SessionClose` retires one
/// page of the session after a terminal outcome, `RejectedClose` retires an admission rejection,
/// and `Complete` has no action left. The state walk is the literal translation of the former
/// `run_interactive_inference` future: every `ctx.tick().await` in that body is one state boundary
/// here, so the sliceability the async shape had is preserved without an executor to park on.
#[derive(Clone, Copy, PartialEq, Eq)]
enum InteractivePhase {
    Dispatch,
    Pump,
    OutcomeClose,
    SessionClose,
    RejectedClose,
    Complete,
}

/// 🔁️ The exact original worker transition retained for one caller turn.
enum PumpTransition { Settled(JobStep) }

struct InteractiveInferenceJob {
    request: crate::app::WireArtifactInferenceRequest,
    restored: Option<Vec<u8>>,
    operation: Operation,
    bridge: InferenceBridge,
    /// 🛑️ Drop guard: releases this inference's cancellation slot when the machine is dropped,
    /// exactly as the former future's own `let _cancellation = …` binding did for its whole run.
    _cancellation: crate::app::ArtifactInferenceCancellationGuard,
    cancel: semio_framework_job::CancelToken,
    session: Option<InferenceSession>,
    rejected: Option<InferenceRejected>,
    outcome: JobOutcomeSlot,
    retirement_progress: RetainedCloneProgress,
    result: Option<Result<(Vec<u8>,Option<Vec<u8>>), semio_framework::Fault>>,
    terminal: bool,
    checkpoint: Option<Vec<u8>>,
    phase: InteractivePhase,
    cancelled: bool,
}

impl InteractiveInferenceJob {
    /// 🎟️ Admission: validates the request's declared resources and claims its cancellation slot
    /// before any worker capacity is touched, so a refused inference never reaches the pool.
    fn admit(job: u64, request: crate::app::WireArtifactInferenceRequest, restored: Option<&[u8]>) -> Result<Self, Vec<u8>> {
        crate::app::validate_wire_request_resources(&request).map_err(|error| semio_framework_diagnostic::encode_fault_bytes(&super::fault(error.code, error.message)))?;
        let cancellation = crate::app::begin_artifact_inference(&request.cancellation_id).map_err(|error| semio_framework_diagnostic::encode_fault_bytes(&super::fault(error.code, error.message)))?;
        let operation = Operation::new(OperationId(job), RevisionId(request.revision), Generation(request.generation), 0);
        let bridge = InferenceBridge::new(operation);
        Ok(Self {
            request,
            restored: restored.map(<[u8]>::to_vec),
            operation,
            bridge,
            _cancellation: cancellation,
            cancel: semio_framework_job::root_cancel_token(),
            session: None,
            rejected: None,
            outcome: JobOutcomeSlot::empty(),
            retirement_progress: RetainedCloneProgress::default(),
            result: None,
            terminal: false,
            checkpoint: None,
            phase: InteractivePhase::Dispatch,
            cancelled: false,
        })
    }

    /// 🚫️async: E1 pure price table consumed by `step`'s sync budget gate.
    fn price(&self) -> u64 {
        match self.phase {
            InteractivePhase::Dispatch => WORK_UNITS_EXECUTE,
            InteractivePhase::Pump => WORK_UNITS_PUMP,
            InteractivePhase::OutcomeClose | InteractivePhase::SessionClose | InteractivePhase::RejectedClose => WORK_UNITS_RETIRE,
            InteractivePhase::Complete => 0,
        }
    }

    /// 🧹️ Drives every deep owner this machine still holds through its own bounded close protocol
    /// so the wrapper left behind is shallow — the property `step_job`/`cancel_job` assert on.
    /// 🚫️async: E1 retirement driver consumed by `cancel` (an externally-declared sync trait method)
    /// and by `fail`; every close protocol it drives is itself synchronous.
    fn record_retirement(&mut self,progress:RetainedCloneProgress)->Result<(),semio_framework::Fault>{
        self.retirement_progress=self.retirement_progress.checked_add(progress).map_err(|error|super::fault("job.infer.retirement-receipt",error.to_string()))?;
        if !progress.fits(self.request.retained){return Err(super::fault("job.infer.retirement-receipt","interactive inference exceeded its original full grant"));}
        Ok(())
    }

    /// 🚫️async: E1 pure terminal constructor consumed by every sync state action below.
    fn fail(&mut self, error: semio_framework::Fault) -> JobStep {
        self.result=Some(Err(error.with_retained_progress(self.retirement_progress)));self.terminal=true;
        if !self.outcome.is_empty(){self.phase=InteractivePhase::OutcomeClose;}
        else if let Some(session)=self.session.as_mut(){session.begin_close();self.phase=InteractivePhase::SessionClose;}
        else if self.rejected.is_some(){self.phase=InteractivePhase::RejectedClose;}
        else{self.phase=InteractivePhase::Complete;let Err(error)=self.result.take().unwrap()else{unreachable!()};return JobStep::Failed(semio_framework_diagnostic::encode_fault_bytes(&error));}
        JobStep::Running(Some(self.retirement_progress()))
    }

    /// 🚀️ `Dispatch`: publishes the request's identity preview, resolves the ActionBus factory for
    /// `semio.infer/<schema>`, and mounts one worker session for it.
    /// 🚫️async: E1 state action consumed by the sync `BoundedJob::step` dispatch table.
    fn dispatch(&mut self) -> JobStep {
        if let Err(error) = self.bridge.publish_preview(semio_framework_pack_json::to_json_string(&(self.request.artifact_kind.clone(), self.request.inference_schema.clone())).into_bytes()) {
            return self.fail(bridge_fault(&error));
        }
        let progress = self.bridge.take_preview().map(|item| encode_bridge_item(&item));
        let bus = semio_framework::ActionBus::production();
        let key = semio_framework::ToolFactoryKey::new(super::JOB_KIND_INFER, self.request.inference_schema.clone());
        let Some(schema_id) = bus.payload_schema_id(&key) else {
            return self.fail(super::fault("job.infer.dispatch", "interactive inference factory disappeared before admission"));
        };
        let dispatch = match bus.dispatch_wire(super::JOB_KIND_INFER, self.request.inference_schema.clone(), schema_id, &self.request.canonical_payload, self.restored.clone(), self.operation) {
            Ok(dispatch) => dispatch,
            Err(error) => return self.fail(super::fault("job.infer.dispatch", error.to_string())),
        };
        let params = semio_framework_job::BatchJobParams {
            operation: self.operation.operation,
            generation: self.operation.generation,
            cancel: self.cancel.clone(),
            config: semio_framework_job::BatchDriveConfig {
                retained:self.request.retained,
                site: "semio.infer.action-bus",
                stage: semio_framework_job::InteractiveStage::UserVisibleSimStep,
                fuel_per_step: self.request.budgets.work_units.clamp(1, semio_framework_job::USER_VISIBLE_LANE_FUEL),
                step_budget_us: semio_framework_job::USER_VISIBLE_LANE_WALL_US,
            },
            now_us: semio_framework_job::default_now_us,
        };
        match InferenceSession::try_new(dispatch.job, params) {
            Ok(session) => {
                self.session = Some(session);
                self.phase = InteractivePhase::Pump;
            }
            Err(rejected) => {
                self.rejected = Some(rejected);
                self.phase = InteractivePhase::RejectedClose;
            }
        }
        JobStep::Running(progress)
    }

    /// ⚙️ Advances one original worker turn before its checked-out owner enters bounded closure.
    fn pump(&mut self, _budget: JobBudget) -> JobStep {
        let PumpTransition::Settled(step)=self.pump_transition();step
    }

    /// 🔁️ One mounted-session transition of [`Self::pump`]: submits and settles one worker step,
    /// then either absorbs its non-terminal outcome in place or hands the crossing back.
    /// 🚫️async: E1 state action body consumed by the sync `pump` loop above.
    fn pump_transition(&mut self) -> PumpTransition {
        match crate::app::inference_cancelled(&self.request.cancellation_id) {
            Ok(true) => self.cancel.cancel_now(),
            Ok(false) => {}
            Err(error) => return PumpTransition::Settled(self.fail(super::fault(error.code, error.message))),
        }
        let scheduled = self.bridge.scheduled();
        let mut progress = encode_bridge_item(&scheduled);
        let stepped = match self.session.as_mut() {
            Some(session) => session.step_on_caller(),
            None => return PumpTransition::Settled(self.fail(super::fault("job.infer.session-missing", "interactive inference lost its mounted worker session before pumping"))),
        };
        if stepped.is_err() {
            return PumpTransition::Settled(self.fail(super::fault("job.infer.worker-pump", "interactive inference mounted worker transition was rejected")));
        }
        let Some(receipt)=self.session.as_ref().and_then(InferenceSession::checked_out_retained_step_progress)else{return PumpTransition::Settled(self.fail(super::fault("job.infer.receipt-missing","interactive inference lost its same-owner actual worker receipt")));};
        if let Err(error)=self.record_retirement(receipt){return PumpTransition::Settled(self.fail(error));}
        let Some(outcome) = self.session.as_mut().and_then(InferenceSession::take_checked_out_outcome) else {
            return PumpTransition::Settled(self.fail(super::fault("job.infer.outcome-missing", "interactive inference mounted worker checkout lost its exact outcome")));
        };
        self.terminal = outcome.is_terminal();
        let result = match &outcome {
            StepOutcome::Yield => None,
            StepOutcome::PreviewReady(payload) => {
                let bytes = match copy_retained_payload(payload, PREVIEW_MAX_BYTES) {
                    Ok(bytes) => bytes,
                    Err(error) => {
                        self.outcome.retain(outcome).expect("inference retains its original checked-out outcome");
                        return PumpTransition::Settled(self.fail(error));
                    }
                };
                if let Err(error) = self.bridge.publish_preview(bytes) {
                    self.outcome.retain(outcome).expect("inference retains its original checked-out outcome");
                    return PumpTransition::Settled(self.fail(bridge_fault(&error)));
                }
                if let Some(item) = self.bridge.take_preview() {
                    progress = encode_bridge_item(&item);
                }
                None
            }
            StepOutcome::CheckpointReady(checkpoint) => {
                match copy_retained_payload(&checkpoint.state, LOSSLESS_MAX_BYTES) {
                    Ok(bytes) => self.checkpoint = Some(bytes),
                    Err(error) => {
                        self.outcome.retain(outcome).expect("inference retains its original checked-out outcome");
                        return PumpTransition::Settled(self.fail(error));
                    }
                }
                None
            }
            StepOutcome::Complete(candidate) => match copy_retained_payload(&candidate.output, LOSSLESS_MAX_BYTES).and_then(|output| copy_retained_payload(&candidate.state, LOSSLESS_MAX_BYTES).map(|state| (output, state))) {
                Ok((output, state)) => Some(Ok((output, (!state.is_empty()).then_some(state).or_else(|| self.checkpoint.clone())))),
                Err(error) => {
                    self.outcome.retain(outcome).expect("inference retains its original checked-out outcome");
                    return PumpTransition::Settled(self.fail(error));
                }
            },
            StepOutcome::Cancelled => Some(Err(super::fault("job.infer.cancelled", "interactive inference was cancelled"))),
            StepOutcome::Fault(fault) => {
                match copy_retained_payload(&fault.detail, DIAGNOSTIC_MAX_BYTES) {
                    Ok(bytes) => self.bridge.publish_diagnostic(bytes),
                    Err(error) => {
                        self.outcome.retain(outcome).expect("inference retains its original checked-out outcome");
                        return PumpTransition::Settled(self.fail(error));
                    }
                }
                if let Some(item) = self.bridge.latest_diagnostic() {
                    progress = encode_bridge_item(item);
                }
                let detail = self.bridge.latest_diagnostic().map_or_else(|| "interactive inference failed without retained diagnostic bytes".to_string(), |item| String::from_utf8_lossy(&item.payload).into_owned());
                Some(Err(super::fault("job.infer.interactive", detail)))
            }
        };
        self.outcome.retain(outcome).expect("inference retains its original checked-out outcome");
        self.result = result;
        self.phase = InteractivePhase::OutcomeClose;
        PumpTransition::Settled(JobStep::Running(Some(progress)))
    }

    /// 🧾️ `OutcomeClose`: one page of the checked-out outcome's retained payload authority. On
    /// completion the machine either closes a terminal session or resumes the worker for the next
    /// pump, exactly as the former future's inner close loop did.
    /// 🚫️async: E1 state action consumed by the sync `BoundedJob::step` dispatch table.
    fn close_outcome(&mut self) -> JobStep {
        if self.outcome.is_empty(){return self.fail(super::fault("job.infer.outcome-missing","interactive inference lost the outcome it was retiring"));}
        let step=match close_step_outcome_slot(&mut self.outcome,self.request.retained){Ok(step)=>step,Err(error)=>{if let Err(receipt_error)=self.record_retirement(error.retained_progress()){return self.fail(receipt_error);}return self.fail(super::fault("job.infer.retirement",error.to_string()));}};
        if let Err(error)=self.record_retirement(step.progress()){return self.fail(error);}
        match step {
            RetainedCloneStep::Progress(_)=>JobStep::Running(Some(self.retirement_progress())),
            RetainedCloneStep::Complete(_) if self.outcome.is_empty()=>{
                if self.terminal{if let Some(session)=self.session.as_mut(){session.begin_close();}self.phase=InteractivePhase::SessionClose;return JobStep::Running(Some(self.retirement_progress()));}
                match self.session.as_mut().map(InferenceSession::resume){Some(Ok(()))=>{self.phase=InteractivePhase::Pump;JobStep::Running(Some(self.retirement_progress()))},_=>self.fail(super::fault("job.infer.resume","interactive inference outcome lost its exact resume authority"))}
            },
            RetainedCloneStep::Complete(_)=>self.fail(super::fault("job.infer.outcome-false-terminal","interactive inference outcome retained its original slot")),
        }
    }

    /// 🧾️ `SessionClose`: one page of the mounted session's retirement after a terminal outcome.
    /// 🚫️async: E1 state action consumed by the sync `BoundedJob::step` dispatch table.
    fn close_session(&mut self) -> JobStep {
        let Some(session) = self.session.as_mut() else {
            return self.fail(super::fault("job.infer.session-missing", "interactive inference lost the session it was retiring"));
        };
        let step=session.close_step(self.request.retained);let empty=session.terminal_is_empty();
        if let Err(error)=self.record_retirement(step.progress()){return self.fail(error);}
        match step {
            semio_framework_job::WorkerJobCloseStep::Pending { .. } | semio_framework_job::WorkerJobCloseStep::Blocked => JobStep::Running(Some(self.retirement_progress())),
            semio_framework_job::WorkerJobCloseStep::Complete { .. } if empty => {
                self.session = None;
                self.phase = InteractivePhase::Complete;
                match self.result.take() {
                    Some(Ok((bytes,state))) => match encode_result(self.request.clone(),bytes,state,self.retirement_progress){Ok(bytes)=>JobStep::Done(bytes),Err(error)=>JobStep::Failed(semio_framework_diagnostic::encode_fault_bytes(&error))},
                    Some(Err(error)) => JobStep::Failed(semio_framework_diagnostic::encode_fault_bytes(&error.with_retained_progress(self.retirement_progress))),
                    None => JobStep::Failed(semio_framework_diagnostic::encode_fault_bytes(&super::fault("job.infer.terminal-result", "terminal interactive inference produced no result"))),
                }
            }
            semio_framework_job::WorkerJobCloseStep::Refused{kind,..}=>self.fail(super::fault("job.infer.retirement",format!("interactive worker close refused {kind:?}"))),
            semio_framework_job::WorkerJobCloseStep::Complete { .. } => self.fail(super::fault("job.infer.session-false-terminal", "interactive inference session did not reach terminal-empty authority")),
        }
    }

    /// 🧾️ `RejectedClose`: one page of an admission rejection's retirement, after which the
    /// capacity refusal is reported — the former future's own rejection loop, state by state.
    /// 🚫️async: E1 state action consumed by the sync `BoundedJob::step` dispatch table.
    fn close_rejected(&mut self) -> JobStep {
        let Some(rejected) = self.rejected.as_mut() else {
            return self.fail(super::fault("job.infer.admission", "interactive inference lost the rejection it was retiring"));
        };
        let step=rejected.close_step(self.request.retained);let empty=rejected.terminal_is_empty();
        if let Err(error)=self.record_retirement(step.progress()){return self.fail(error);}
        match step {
            semio_framework_job::InteractiveJobCloseStep::Pending { .. } | semio_framework_job::InteractiveJobCloseStep::Blocked => JobStep::Running(Some(self.retirement_progress())),
            semio_framework_job::InteractiveJobCloseStep::Complete { .. } if empty => {
                self.rejected = None;
                self.phase = InteractivePhase::Complete;
                JobStep::Failed(semio_framework_diagnostic::encode_fault_bytes(&super::fault("job.infer.admission", "interactive inference worker session capacity is exhausted")))
            }
            semio_framework_job::InteractiveJobCloseStep::Refused{kind,..}=>self.fail(super::fault("job.infer.retirement",format!("interactive rejection close refused {kind:?}"))),
            semio_framework_job::InteractiveJobCloseStep::Complete { .. } => self.fail(super::fault("job.infer.admission-false-terminal", "interactive inference admission rejection did not reach terminal-empty authority")),
        }
    }

    /// 📈️ A fresh scheduled bridge item for a retirement state, so every `Running` this machine
    /// reports carries monotonic progress bytes and the stall guard never mistakes a bounded close
    /// walk for a wedged job.
    /// 🚫️async: E1 pure bridge read consumed by the sync state actions above.
    fn retirement_progress(&mut self) -> Vec<u8> {
        let item = self.bridge.scheduled();
        encode_bridge_item(&item)
    }
}

impl InteractiveInferenceJob {
    fn step_cold(&mut self, budget: JobBudget, _original: &mut super::IoRunControl<'_, '_>, _snapshot: &mut super::SqliteSnapshotControl<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> JobStep {
        let price = self.price();
        if budget.fuel < price {
            return self.fail(super::fault("job.infer.budget-exhausted", format!("interactive inference needs {price} work units for its next state action and was granted {}", budget.fuel)));
        }
        match self.phase {
            InteractivePhase::Dispatch => self.dispatch(),
            InteractivePhase::Pump => self.pump(budget),
            InteractivePhase::OutcomeClose => self.close_outcome(),
            InteractivePhase::SessionClose => self.close_session(),
            InteractivePhase::RejectedClose => self.close_rejected(),
            InteractivePhase::Complete => JobStep::Failed(semio_framework_diagnostic::encode_fault_bytes(&super::fault("job.infer.terminal", "interactive inference has no state action left to advance"))),
        }
    }

    fn cancel(&mut self) {
        self.cancelled = true;
        self.cancel.cancel_now();
        let _=self.fail(super::fault("job.infer.cancelled","interactive inference was cancelled"));
    }

    fn checkpoint(&self) -> Option<Vec<u8>> {
        self.checkpoint.clone()
    }

    fn terminal_drop_is_shallow(&self) -> bool {
        self.session.is_none() && self.rejected.is_none() && self.outcome.is_empty()
    }
}

fn copy_retained_payload(payload: &semio_framework_job::RetainedJobPayload, maximum_bytes: usize) -> Result<Vec<u8>, semio_framework::Fault> {
    if payload.len() > maximum_bytes {
        return Err(super::fault("job.infer.payload-limit", format!("interactive inference retained payload has {} bytes, above {maximum_bytes}", payload.len())));
    }
    let mut bytes = Vec::with_capacity(payload.len());
    let mut reader = payload.reader();
    while let Some(page) = reader.read_page(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
        bytes.extend_from_slice(page);
    }
    if !reader.terminal_is_empty() {
        return Err(super::fault("job.infer.payload-page", "interactive inference retained payload page exceeded the fixed codec grant"));
    }
    Ok(bytes)
}

fn bridge_fault(error: &InferenceBridgeError) -> semio_framework::Fault {
    super::fault("job.infer.bridge", error.to_string())
}

/// 🧾️ The result of a completed interactive inference, reporting only what the host observed: the job completed (`complete`), faulted nowhere (`valid`),
/// its final persisted state or last checkpoint is the resume state (`previous_state`), and it published no diagnostics on the success path. A fidelity the
/// job never declared is `unreported`, never `exact`.
fn encode_result(request: crate::app::WireArtifactInferenceRequest, canonical_payload: Vec<u8>, resume_state: Option<Vec<u8>>,retirement_progress:RetainedCloneProgress) -> Result<Vec<u8>, semio_framework::Fault> {
    let allocation = usize::try_from(request.budgets.allocation_bytes).map_err(|_| super::fault("job.infer.result", "allocation budget exceeds this runtime's address space"))?;
    if canonical_payload.len() > allocation {
        return Err(super::fault("job.infer.result", format!("interactive inference result has {} bytes, above allocation budget {allocation}", canonical_payload.len())));
    }
    let provenance = crate::app::WireArtifactInferenceProvenance {
        owner: request.owner.clone(),
        inference_schema: request.inference_schema.clone(),
        algorithm_version: request.algorithm_version,
        policy_version: request.policy_version,
        source_dialect: request.source_dialect.clone(),
    };
    let result = crate::app::WireArtifactInferenceResult {
        retirement_progress,
        wire_version: crate::app::ARTIFACT_INFERENCE_WIRE_VERSION,
        owner: request.owner,
        artifact_kind: request.artifact_kind,
        artifact_schema: request.artifact_schema,
        artifact_schema_version: request.artifact_schema_version,
        inference_schema: request.inference_schema,
        inference_schema_version: request.inference_schema_version,
        algorithm_version: request.algorithm_version,
        policy_version: request.policy_version,
        revision: request.revision,
        generation: request.generation,
        source_dialect: request.source_dialect,
        policy: request.policy,
        budgets: request.budgets,
        retained: request.retained,
        previous_state: resume_state,
        requested_cache_mode: request.requested_cache_mode.clone(),
        canonical_payload:Some(canonical_payload),
        dependencies: request.dependencies,
        diagnostics: Vec::new(),
        provenance,
        validity: "valid".into(),
        quality: "unreported".into(),
        complete: true,
        actual_cache_mode: request.requested_cache_mode,
        cancellation_id: request.cancellation_id,
    };
    Ok(semio_framework_pack_json::to_json_string(&result).into_bytes())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
