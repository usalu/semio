//! 💡️ `semio.infer` cold-job bridge. Exact ActionBus routes such as `s.wfc.wfc3d.solve`
//! decode through their factory-owned schema and retain one persistent `InteractiveJob` session.
//! Every guest continuation admits exactly one bounded step to the shared WorkerPool; previews
//! coalesce, checkpoints and commits remain lossless under explicit item/byte bounds, and
//! diagnostics use a bounded ring. Inferences without an ActionBus route retain the synchronous
//! two-phase registry path.

use super::{BoundedJob, JobStep, WORK_UNITS_EXECUTE, WORK_UNITS_PUMP, WORK_UNITS_RETIRE};
use semio_framework_job::{Generation, JobOutcomeView, Operation, OperationId, RevisionId};
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
    pub(crate) operation: u64,
    pub(crate) generation: u64,
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

type InferenceSession = semio_framework_job_worker::MountedWorkerJobSession<semio_framework::action_bus::ErasedToolJob>;
type InferenceRejected = semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource<semio_framework::action_bus::ErasedToolJob>;

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
    failure:Option<crate::ArtifactInferenceGatewayRefusal>,closing_failure:Option<crate::ArtifactInferenceGatewayRefusal>,reply:Option<crate::extension_invocation_failure::RetainedExtensionFaultReply>,
    secondary:Option<semio_framework_job::OriginalTurnFailure>,active:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,cancelled:bool,
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
    fn close_sources(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let grant=cx.retained_grant();let result=if self.closing_failure.is_some(){Self::close_field(&mut self.closing_failure,&mut self.active,grant)}else if self.secondary.is_some(){Self::close_field(&mut self.secondary,&mut self.active,grant)}else if self.input.is_some()||self.active.is_some(){Self::close_field(&mut self.input,&mut self.active,grant)}else if self.restored.is_some(){Self::close_field(&mut self.restored,&mut self.active,grant)}else{return Ok(true)};
        let progress=match result{Ok(progress)=>progress,Err(error)=>{return Err(semio_framework_job::OriginalTurnFailure::debit_original(error,cx))}};cx.consume_retained(progress)?;cx.consume_fuel(1);Ok(false)
    }
}
impl OriginalInferenceJob {
    fn step_original(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->JobStep{
        if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return JobStep::Running(None)}
        if let Some(gateway)=&mut self.gateway{
            if self.cancelled||self.failure.is_some(){gateway.begin_close();}
            if self.closing_failure.is_some(){match Self::close_field(&mut self.closing_failure,&mut self.active,cx.retained_grant()){Ok(progress)=>{if let Err(error)=cx.consume_retained(progress){self.secondary=Some(error.into());}cx.consume_fuel(1);},Err(error)=>self.secondary=Some(semio_framework_job::OriginalTurnFailure::debit_original(error,cx))}return JobStep::Running(None)}
            if self.secondary.is_some()||self.active.is_some(){match Self::close_field(&mut self.secondary,&mut self.active,cx.retained_grant()){Ok(progress)=>{if let Err(error)=cx.consume_retained(progress){self.secondary=Some(error.into());}cx.consume_fuel(1);},Err(error)=>self.secondary=Some(semio_framework_job::OriginalTurnFailure::debit_original(error,cx))}return JobStep::Running(None)}
            let answer=crate::wire_artifact_infer(gateway,self.input.as_ref().unwrap(),cx);
            if let Some(cause)=answer.refusal{if self.failure.is_none(){self.failure=Some(cause);}else{assert!(self.closing_failure.is_none());self.closing_failure=Some(cause);}gateway.begin_close();}
            if let Some(payload)=answer.payload{assert!(self.output.is_none());self.output=Some(payload);}
            if gateway.terminal_is_empty(){self.gateway=None;}
            return JobStep::Running(None);
        }
        match self.close_sources(cx){Ok(false)=>return JobStep::Running(None),Err(error)=>{assert!(self.secondary.is_none());self.secondary=Some(error.into());return JobStep::Running(None)},Ok(true)=>{}}
        if self.failure.is_some()&&self.reply.is_none(){self.reply=Some(crate::extension_invocation_failure::RetainedExtensionFaultReply::new(crate::extension_invocation_failure::ExtensionInvocationCause::Inference(self.failure.take().unwrap())));if let Err(error)=cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()}){self.secondary=Some(error.into());}return JobStep::Running(None)}
        if self.cancelled&&self.output.is_some(){match Self::close_field(&mut self.output,&mut self.active,cx.retained_grant()){Ok(progress)=>{if let Err(error)=cx.consume_retained(progress){self.secondary=Some(error.into());}},Err(error)=>self.secondary=Some(semio_framework_job::OriginalTurnFailure::debit_original(error,cx))}return JobStep::Running(None)}
        if self.cancelled&&self.reply.is_none(){self.reply=Some(crate::extension_invocation_failure::RetainedExtensionFaultReply::new(crate::extension_invocation_failure::ExtensionInvocationCause::Value(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"original inference cancelled"))));return JobStep::Running(None)}
        if let Some(reply)=&mut self.reply{match reply.advance(cx){Ok(Some(bytes))=>{assert!(reply.terminal_is_empty());self.reply=None;return JobStep::Failed(bytes)},Ok(None)=>return JobStep::Running(None),Err(error)=>{self.secondary=Some(error.into());return JobStep::Running(None)}}}
        if self.output.is_some(){if let Err(error)=cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()}){self.secondary=Some(error.into());return JobStep::Running(None)}return JobStep::Done(self.output.take().unwrap())}
        JobStep::Running(None)
    }
 }
impl BoundedJob for OriginalInferenceJob {
    fn step(&mut self,_original:&mut super::IoRunControl<'_,'_>,_snapshot:&mut super::SqliteSnapshotControl<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<JobStep,semio_framework_job::OriginalTurnFailure>{let step=self.step_original(cx);match self.secondary.take(){Some(error)=>Err(error),None=>Ok(step)}}
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.terminal_drop_is_shallow(){return Ok(true)}
        if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}
        self.cancelled=true;
        if self.active.is_some()||self.closing_failure.is_some()||self.secondary.is_some(){
            let result=if self.active.is_some(){Self::close_field(&mut self.input,&mut self.active,cx.retained_grant())}else if self.closing_failure.is_some(){Self::close_field(&mut self.closing_failure,&mut self.active,cx.retained_grant())}else{Self::close_field(&mut self.secondary,&mut self.active,cx.retained_grant())};
            match result{Ok(progress)=>cx.consume_retained(progress)?,Err(error)=>{return Err(semio_framework_job::OriginalTurnFailure::debit_original(error,cx))}}cx.consume_fuel(1);return Ok(false)
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
        match result{Ok(progress)=>{cx.consume_retained(progress)?;cx.consume_fuel(1);Ok(false)},Err(error)=>{Err(semio_framework_job::OriginalTurnFailure::debit_original(error,cx))}}
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
fn decode_request(input: &[u8]) -> Result<crate::artifact_inference::WireArtifactInferenceRequest, semio_framework::Fault> {
    let input_text = std::str::from_utf8(input).map_err(|error| super::fault("job.infer.decode", format!("invalid {} input: {error}", super::JOB_KIND_INFER)))?;
    semio_framework_pack_json::from_json_str(input_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| super::fault("job.infer.decode", format!("invalid {} input: {error}", super::JOB_KIND_INFER)))
}

enum SeenInference { Yield, Preview(Vec<u8>), Checkpoint(Vec<u8>), Complete(Vec<u8>, Vec<u8>), Cancelled, Fault(Vec<u8>) }

fn copy_optional_payload(payload: Option<&semio_framework_job::RetainedJobPayload>, maximum_bytes: usize) -> Result<Vec<u8>, semio_framework::Fault> {
    payload.map_or_else(|| Ok(Vec::new()), |payload| copy_retained_payload(payload, maximum_bytes))
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
fn encode_result(request: crate::artifact_inference::WireArtifactInferenceRequest, canonical_payload: Vec<u8>, resume_state: Option<Vec<u8>>,retirement_progress:RetainedCloneProgress) -> Result<Vec<u8>, semio_framework::Fault> {
    let allocation = usize::try_from(request.budgets.allocation_bytes).map_err(|_| super::fault("job.infer.result", "allocation budget exceeds this runtime's address space"))?;
    if canonical_payload.len() > allocation {
        return Err(super::fault("job.infer.result", format!("interactive inference result has {} bytes, above allocation budget {allocation}", canonical_payload.len())));
    }
    let provenance = crate::artifact_inference::WireArtifactInferenceProvenance {
        owner: request.owner.clone(),
        inference_schema: request.inference_schema.clone(),
        algorithm_version: request.algorithm_version,
        policy_version: request.policy_version,
        source_dialect: request.source_dialect.clone(),
    };
    let result = crate::artifact_inference::WireArtifactInferenceResult {
        retirement_progress,
        wire_version: crate::artifact_inference::ARTIFACT_INFERENCE_WIRE_VERSION,
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
