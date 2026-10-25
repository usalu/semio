//! 🎯️ Action bus and tool job registry, plus the root platform shell that owns one bus.

use semio_framework_job::InteractiveJob;
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_job::Operation;
use semio_framework_job::StepContext;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;
use semio_framework_manifest_actions::InteractiveJobClassification;
use semio_framework_manifest_app_surface::AppDefinition;

//#region 🎯️ToolJobBus

#[path="🚪️admission/🦀️.rs"]
pub mod admission;
pub use admission::{ToolJobAdmissionDemand,OriginalToolSourceBinding,OriginalToolJobIdentity,admit_original_tool_job,admit_original_tool_job_measured,admit_original_tool_job_controlled,admit_original_tool_construction,OriginalToolConstructionScope,admit_original_tool_construction_scoped};
//#region 🧬️Contract
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ToolFactoryKey {
    pub controller_id: String,
    pub tool_id: String,
}

impl ToolFactoryKey {
    pub fn new(controller_id: impl Into<String>, tool_id: impl Into<String>) -> Self {
        Self { controller_id: controller_id.into(), tool_id: tool_id.into() }
    }
}

pub struct ToolPayload {
    pub schema_id: String,
    value: Option<Box<dyn Any + Send>>,
}

impl ToolPayload {
    /// 🎒️ The caller transfers the exact already-born payload allocation without another Box.
    pub fn from_owned<T:Send+'static>(schema_id:String,value:Box<T>)->Self{Self{schema_id,value:Some(value)}}
    /// 🫴️ Returns the exact original typed source for its caller's bounded retirement without another allocation.
    pub fn take_original<T:Send+'static>(&mut self)->Result<Option<Box<T>>,semio_framework_value::ValueError>{match self.value.take(){None=>Ok(None),Some(original)=>match original.downcast::<T>(){Ok(original)=>Ok(Some(original)),Err(original)=>{self.value=Some(original);Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"original payload retirement requested a different defining type"))}}}}
    /// 👁️ A refused admission leaves its original payload allocation in the caller's specification.
    pub fn retained_payload(&self)->Option<&(dyn Any+Send)>{self.value.as_deref()}
}

pub struct ToolOperationSpec {
    pub key: ToolFactoryKey,
    pub payload: ToolPayload,
    pub operation: Operation,
    pub source: OriginalToolJobIdentity,
}

impl ToolOperationSpec {
    /// 🧳️ Original owned metadata and payload enter without another allocation or identity copy.
    pub fn from_owned<T:Send+'static>(key:ToolFactoryKey,schema_id:String,payload:Box<T>,operation:Operation,source:OriginalToolJobIdentity)->Self{Self{key,payload:ToolPayload::from_owned(schema_id,payload),operation,source}}
    /// 🔑️ Borrows the exact owned lookup key without allocating on a denied factory turn.
    pub fn key(&self)->&ToolFactoryKey{&self.key}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToolWireAdmission {
    pub key: ToolFactoryKey,
    pub factory_type_id: TypeId,
    pub factory_type_name: &'static str,
    pub schema_id: String,
    pub contract: ToolExecutionContract,
}

pub const TOOL_WIRE_PAGE_BYTES: usize = 4_096;

#[derive(Debug)]
pub struct ToolWirePage {
    bytes: [u8; TOOL_WIRE_PAGE_BYTES],
    len: usize,
}

impl ToolWirePage {
    pub fn try_copy_from(bytes: &[u8]) -> Result<Self, ToolJobFactoryError> {
        if bytes.len() > TOOL_WIRE_PAGE_BYTES {
            return Err(ToolJobFactoryError::new("tool wire page exceeds its fixed byte capacity"));
        }
        let mut page = Self { bytes: [0; TOOL_WIRE_PAGE_BYTES], len: bytes.len() };
        page.bytes[..bytes.len()].copy_from_slice(bytes);
        Ok(page)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

pub struct RetainedToolWireInput {
    pages: Vec<ToolWirePage>,
    declared_bytes: usize,
    admitted_bytes: usize,
    maximum_bytes: usize,
    sealed: bool,
    closing: bool,
}

impl RetainedToolWireInput {
    /// 📥️ Opens the empty page owner of one declared wire extent; refused when the extent exceeds its admitted maximum.
    pub fn try_new(declared_bytes: usize, maximum_bytes: usize) -> Result<Self, ToolJobFactoryError> {
        if declared_bytes > maximum_bytes {
            return Err(ToolJobFactoryError::new("declared tool wire extent exceeds its admitted contract"));
        }
        let page_capacity = declared_bytes.saturating_add(TOOL_WIRE_PAGE_BYTES - 1) / TOOL_WIRE_PAGE_BYTES;
        let mut pages = Vec::new();
        pages.try_reserve_exact(page_capacity).map_err(|_| ToolJobFactoryError::new("tool wire page owner capacity could not be retained"))?;
        Ok(Self { pages, declared_bytes, admitted_bytes: 0, maximum_bytes, sealed: false, closing: false })
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact fixed wire page without allocating or dropping its bytes.")]
    pub fn admit_page(&mut self, page: ToolWirePage) -> Result<(), (ToolJobFactoryError, ToolWirePage)> {
        if self.sealed || self.closing {
            return Err((ToolJobFactoryError::new("tool wire input is sealed or closing"), page));
        }
        let Some(next) = self.admitted_bytes.checked_add(page.len()) else { return Err((ToolJobFactoryError::new("tool wire byte extent overflowed"), page)) };
        if next > self.declared_bytes || next > self.maximum_bytes || self.pages.len() == self.pages.capacity() {
            return Err((ToolJobFactoryError::new("tool wire page exceeds its pre-admitted extent"), page));
        }
        self.admitted_bytes = next;
        self.pages.push(page);
        Ok(())
    }

    pub fn seal(&mut self) -> Result<(), ToolJobFactoryError> {
        if self.closing || self.admitted_bytes != self.declared_bytes {
            return Err(ToolJobFactoryError::new("tool wire input cannot seal before its exact declared extent is present"));
        }
        self.sealed = true;
        Ok(())
    }

    /// 🛡️ Seals the admitted prefix after the maximum extent was reserved before an
    /// incremental encoder ran. The retained page capacity is not released or widened here; only the
    /// truthful logical extent is narrowed to the bytes already owned by this input.
    pub fn seal_admitted_prefix(&mut self) -> Result<(), ToolJobFactoryError> {
        if self.closing || self.sealed {
            return Err(ToolJobFactoryError::new("tool wire input is already sealed or closing"));
        }
        self.declared_bytes = self.admitted_bytes;
        self.sealed = true;
        Ok(())
    }

    pub fn page(&self, index: usize) -> Option<&[u8]> {
        (self.sealed && !self.closing).then(|| self.pages.get(index).map(ToolWirePage::as_slice)).flatten()
    }

    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    pub fn declared_bytes(&self) -> usize {
        self.declared_bytes
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    pub fn next_close_capacity_byte_demand(&self,_copy:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    pub fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.pages.capacity().checked_mul(size_of::<ToolWirePage>()).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"wire page backing extent overflow"))}
    pub fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(usize::from(self.pages.capacity()!=0))}
    pub fn close_step(&mut self,grant:semio_framework_job::RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>{
        use semio_framework_job::{InteractiveJobCloseStep,RetainedCloneProgress};
        self.begin_close();
        if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:RetainedCloneProgress::default()})}
        let bytes=self.next_close_release_byte_demand()?;
        if grant.maximum_items==0||bytes>grant.maximum_release_bytes{return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress::default()})}
        if self.pages.capacity()!=0&&grant.maximum_depth==0{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original wire input close exceeds caller depth"))}
        drop(std::mem::take(&mut self.pages));self.admitted_bytes=0;
        Ok(InteractiveJobCloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,copied_bytes:0,retained_capacity_bytes:0,released_bytes:bytes}})
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.pages.is_empty() && self.pages.capacity() == 0 && self.admitted_bytes == 0
    }
}



/// 🔒️ Exact, reviewable admission bounds owned by one UI-reachable command factory.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToolExecutionContract {
    pub max_raw_wire_bytes: usize,
    pub max_decoded_items: usize,
    pub max_work_units_per_step: u64,
    pub max_output_bytes: usize,
    pub max_step_micros: u32,
    pub checkpoint_every_steps: u32,
    pub progress_every_steps: u32,
    pub cancellation: ToolCancellationPolicy,
    pub freshness: ToolFreshnessPolicy,
    pub shape: ToolExecutionShape,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolExecutionShape {
    Resumable,
    BoundedFirstStep,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolCancellationPolicy {
    PerOperation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolFreshnessPolicy {
    ValidateImmediatelyBeforeExposure,
}

impl ToolExecutionContract {
    pub const INTERACTIVE_MAX_STEP_MICROS: u32 = 8_000;

    pub const fn resumable(max_raw_wire_bytes: usize, max_decoded_items: usize, max_work_units_per_step: u64, max_output_bytes: usize, max_step_micros: u32, checkpoint_every_steps: u32, progress_every_steps: u32) -> Self {
        Self {
            max_raw_wire_bytes,
            max_decoded_items,
            max_work_units_per_step,
            max_output_bytes,
            max_step_micros,
            checkpoint_every_steps,
            progress_every_steps,
            cancellation: ToolCancellationPolicy::PerOperation,
            freshness: ToolFreshnessPolicy::ValidateImmediatelyBeforeExposure,
            shape: ToolExecutionShape::Resumable,
        }
    }

    pub const fn bounded_first_step(max_raw_wire_bytes: usize, max_decoded_items: usize, max_work_units: u64, max_output_bytes: usize, max_step_micros: u32) -> Self {
        Self {
            max_raw_wire_bytes,
            max_decoded_items,
            max_work_units_per_step: max_work_units,
            max_output_bytes,
            max_step_micros,
            checkpoint_every_steps: 1,
            progress_every_steps: 1,
            cancellation: ToolCancellationPolicy::PerOperation,
            freshness: ToolFreshnessPolicy::ValidateImmediatelyBeforeExposure,
            shape: ToolExecutionShape::BoundedFirstStep,
        }
    }

    fn validate(self) -> Result<(), &'static str> {
        if self.max_raw_wire_bytes == 0 {
            return Err("max_raw_wire_bytes must be non-zero");
        }
        if self.max_decoded_items == 0 {
            return Err("max_decoded_items must be non-zero");
        }
        if self.max_work_units_per_step == 0 {
            return Err("max_work_units_per_step must be non-zero");
        }
        if self.max_output_bytes == 0 {
            return Err("max_output_bytes must be non-zero");
        }
        if self.max_step_micros == 0 || self.max_step_micros >= Self::INTERACTIVE_MAX_STEP_MICROS {
            return Err("max_step_micros must be strictly below 8000");
        }
        if self.checkpoint_every_steps == 0 || self.progress_every_steps == 0 {
            return Err("checkpoint and progress cadence must be non-zero");
        }
        Ok(())
    }
}

pub struct ErasedToolJob {
    inner: std::mem::ManuallyDrop<Option<Box<dyn InteractiveJob + Send>>>,
}

impl ErasedToolJob {
/// 📩️ Receives a preborn typed job allocation without cloning or allocating another wrapper.
    pub fn from_owned(inner: Box<dyn InteractiveJob + Send>) -> Self { Self { inner: std::mem::ManuallyDrop::new(Some(inner)) } }

    /// 💼️ Borrows the exact original erased allocation while its physical owner remains retained.
    pub fn retained_job(&self) -> Option<&(dyn InteractiveJob + Send)> { self.inner.as_deref() }

    fn inner(&self) -> Result<&(dyn InteractiveJob + Send), semio_framework_value::ValueError> {
        self.inner.as_deref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original erased job is physically retired"))
    }

    fn shell_copy_bytes() -> usize { std::mem::size_of::<Option<Box<dyn InteractiveJob + Send>>>() }
}

impl InteractiveJob for ErasedToolJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        self.inner.as_mut().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original erased job is physically retired"))?.step(cx)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.inner()?.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) { if let Some(inner) = self.inner.as_mut() { inner.begin_close(); } }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<semio_framework_job::InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> {
        use semio_framework_job::InteractiveJobCloseStep;
        use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::RetainedCloneProgress};
        let Some(inner) = self.inner.as_mut() else { return Ok(InteractiveJobCloseStep::Complete { progress: Default::default() }) };
        let grant = cx.retained_grant();
        if cx.should_yield() || grant.maximum_items == 0 { return Ok(InteractiveJobCloseStep::Pending { progress: Default::default() }) }
        if inner.terminal_is_empty() {
            if grant.maximum_depth == 0 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "original erased job box close exceeds caller depth").into()) }
            let released_bytes = std::mem::size_of_val(inner.as_ref());
            let copied_bytes = Self::shell_copy_bytes();
            if grant.maximum_copy_bytes < copied_bytes || grant.maximum_release_bytes < released_bytes { return Ok(InteractiveJobCloseStep::Pending { progress: Default::default() }) }
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes, released_bytes, ..Default::default() };
            drop(self.inner.take());
            cx.consume_fuel(1);
            cx.consume_retained(progress).map_err(|error| semio_framework_job::OriginalTurnFailure::from(error.with_retained_progress(progress)))?;
            return Ok(InteractiveJobCloseStep::Complete { progress });
        }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "original erased job child close exceeds caller depth").into()) }
        let child = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        let step = cx.with_retained_loan(child, |same| inner.close_step(same))?;
        let step = step.admit(child, inner.terminal_is_empty()).map_err(semio_framework_job::OriginalTurnFailure::from)?;
        Ok(match step { InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress }, step => step })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        match self.inner.as_ref() { None => Ok(0), Some(inner) if inner.terminal_is_empty() => Ok(Self::shell_copy_bytes()), Some(inner) => inner.next_close_copy_byte_demand() }
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        match self.inner.as_ref() { None => Ok(0), Some(inner) if inner.terminal_is_empty() => Ok(0), Some(inner) => inner.next_close_capacity_byte_demand(maximum_copy_bytes) }
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        match self.inner.as_ref() { None => Ok(0), Some(inner) if inner.terminal_is_empty() => Ok(std::mem::size_of_val(inner.as_ref())), Some(inner) => inner.next_close_release_byte_demand() }
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        match self.inner.as_ref() { None => Ok(0), Some(inner) if inner.terminal_is_empty() => Ok(1), Some(inner) => inner.next_close_depth_demand()?.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "original erased job child depth overflow")) }
    }

    fn register_close_wake(&self, waker: &std::task::Waker) -> bool { self.inner.as_ref().is_some_and(|inner| inner.register_close_wake(waker)) }

    fn terminal_is_empty(&self) -> bool { self.inner.is_none() }
}

impl Drop for ErasedToolJob {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "original erased job dropped before paid physical Box retirement");
        if self.terminal_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.inner); } }
    }
}

pub trait ToolJobFactory: Send + 'static {
    type Payload: Send + 'static;
    /// 🧵️ `+ Send`: a factory-produced job is erased into [`ErasedToolJob`] and dispatched onto the
    /// worker pool, so the bound is stated where the transfer happens rather than on every
    /// `InteractiveJob` (see `semio_framework_job::JobThreadTransfer`).
    type Job: InteractiveJob + Send + 'static;

    fn keys(&self) -> &[ToolFactoryKey];
    fn payload_schema_id(&self) -> &str;
    fn classification(&self) -> InteractiveJobClassification;
    fn execution_contract(&self) -> ToolExecutionContract;
    /// 📏️ Quotes actual construction from borrowed original sources before any transfer.
    fn admission_demands(&self,operation:&Operation,payload:&Self::Payload,input:Option<&RetainedToolWireInput>,checkpoint:Option<&RetainedToolWireInput>)->Result<ToolJobAdmissionDemand,semio_framework_value::ValueError>;

    /// ⛔️ The caller retains all denied sources and any born unpublished candidate on refusal.
    fn admit_job(&mut self,context:&mut StepContext<'_>,operation:&Operation,payload:&mut Option<Box<Self::Payload>>,input:&mut Option<RetainedToolWireInput>,checkpoint:&mut Option<RetainedToolWireInput>,output:&mut Option<Box<Self::Job>>)->Result<bool,semio_framework_job::OriginalTurnFailure>;

}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToolJobFactoryError {
    pub detail: String,
}

impl ToolJobFactoryError {
    pub fn new(detail: impl Into<String>) -> Self {
        Self { detail: detail.into() }
    }
}

impl Display for ToolJobFactoryError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl Error for ToolJobFactoryError {}

//#endregion 🧬️Contract
//#region 🚫️Rejection

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToolRegistrationError {
    EmptyFactory,
    DuplicateKey { key: ToolFactoryKey },
    InvalidExecutionContract { key: ToolFactoryKey, detail: &'static str },
    NonInteractiveClassification { key: ToolFactoryKey, classification: InteractiveJobClassification },
    UnknownAliasTarget { alias: ToolFactoryKey, target: ToolFactoryKey },
}

impl Display for ToolRegistrationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyFactory => formatter.write_str("tool factory must own at least one controller/tool key"),
            Self::DuplicateKey { key } => write!(formatter, "tool factory key '{}/{}' is already registered", key.controller_id, key.tool_id),
            Self::InvalidExecutionContract { key, detail } => write!(formatter, "tool factory '{}/{}' has an invalid execution contract: {detail}", key.controller_id, key.tool_id),
            Self::NonInteractiveClassification { key, classification } => write!(formatter, "tool factory '{}/{}' is not UI-reachable: {classification:?}", key.controller_id, key.tool_id),
            Self::UnknownAliasTarget { alias, target } => write!(formatter, "tool alias '{}/{}' targets unknown exact factory '{}/{}'", alias.controller_id, alias.tool_id, target.controller_id, target.tool_id),
        }
    }
}

impl Error for ToolRegistrationError {}

#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub enum ToolDispatchError {
    UnknownController { controller_id: String },
    Factory { controller_id: String, tool_id: String, detail: String },
    RawWireLimit { controller_id: String, tool_id: String, actual: usize, maximum: usize },
}

impl Display for ToolDispatchError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownController { controller_id } => write!(formatter, "unknown tool controller '{controller_id}'"),
            Self::Factory { controller_id, tool_id, detail } => write!(formatter, "tool factory '{controller_id}' rejected '{tool_id}': {detail}"),
            Self::RawWireLimit { controller_id, tool_id, actual, maximum } => write!(formatter, "tool factory '{controller_id}/{tool_id}' rejected {actual} raw bytes before decoding; maximum is {maximum}"),
        }
    }
}

impl Error for ToolDispatchError {}

//#endregion 🚫️Rejection
//#region 🚌️Bus

trait ErasedToolJobFactory:Send{
    fn admission_demands(&self,spec:&ToolOperationSpec,input:Option<&RetainedToolWireInput>,checkpoint:Option<&RetainedToolWireInput>)->Result<ToolJobAdmissionDemand,semio_framework_value::ValueError>;
    fn admit_job(&mut self,context:&mut StepContext<'_>,spec:&mut ToolOperationSpec,input:&mut Option<RetainedToolWireInput>,checkpoint:&mut Option<RetainedToolWireInput>,output:&mut Option<ErasedToolJob>)->Result<bool,semio_framework_job::OriginalTurnFailure>;
}

struct ToolJobFactoryAdapter<F:ToolJobFactory>{factory:F}
impl<F:ToolJobFactory> ToolJobFactoryAdapter<F>{
    fn original_payload<'a>(&self,spec:&'a ToolOperationSpec)->Result<&'a F::Payload,semio_framework_value::ValueError>{
        use semio_framework_value::{ValueError,ValueRefusalKind};
        if spec.payload.schema_id!=self.factory.payload_schema_id(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original tool payload schema does not match its registered factory"));}
        spec.payload.value.as_ref().and_then(|value|value.downcast_ref::<F::Payload>()).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original tool payload type does not match its registered factory"))
    }
}
impl<F:ToolJobFactory> ErasedToolJobFactory for ToolJobFactoryAdapter<F>{
    fn admission_demands(&self,spec:&ToolOperationSpec,input:Option<&RetainedToolWireInput>,checkpoint:Option<&RetainedToolWireInput>)->Result<ToolJobAdmissionDemand,semio_framework_value::ValueError>{self.factory.admission_demands(&spec.operation,self.original_payload(spec)?,input,checkpoint)}
    fn admit_job(&mut self,context:&mut StepContext<'_>,spec:&mut ToolOperationSpec,input:&mut Option<RetainedToolWireInput>,checkpoint:&mut Option<RetainedToolWireInput>,output:&mut Option<ErasedToolJob>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_value::{ValueError,ValueRefusalKind};
        self.original_payload(spec)?;
        if !spec.source.matches(context){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original tool specification belongs to different source roots or clock").into());}
        if output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original erased tool candidate slot is occupied").into());}
        let original=spec.payload.value.take().expect("validated original payload");
        let typed=match original.downcast::<F::Payload>(){Ok(typed)=>typed,Err(original)=>{spec.payload.value=Some(original);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original tool payload changed after validation").into());}};
        let mut payload=Some(typed);
        let mut candidate=None;
        let result=self.factory.admit_job(context,&spec.operation,&mut payload,input,checkpoint,&mut candidate);
        spec.payload.value=payload.map(|value|value as Box<dyn Any+Send>);
        *output=candidate.map(|job|ErasedToolJob::from_owned(job as Box<dyn InteractiveJob+Send>));
        match result{Ok(true) if output.is_none()=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original factory omitted its admitted job candidate").into()),result=>result}
    }
}

struct ActionBusInner {
    factory_by_key: HashMap<ToolFactoryKey, usize>,
    factory_identity_by_key: HashMap<ToolFactoryKey, (TypeId, &'static str, String)>,
    contract_by_key: HashMap<ToolFactoryKey, ToolExecutionContract>,
    aliases: HashMap<ToolFactoryKey, ToolFactoryKey>,
    factories: Vec<Box<dyn ErasedToolJobFactory>>,
    dispatch_count: u64,
}

#[derive(Clone)]
pub struct ActionBus {
    inner: Arc<Mutex<ActionBusInner>>,
}

impl Default for ActionBus {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionBus {
    pub fn new() -> Self {
        Self { inner: Arc::new(Mutex::new(ActionBusInner { factory_by_key: HashMap::new(), factory_identity_by_key: HashMap::new(), contract_by_key: HashMap::new(), aliases: HashMap::new(), factories: Vec::new(), dispatch_count: 0 })) }
    }

    /// 🌐️ Process-wide production registry shared by Platform and activated app controllers.
    pub fn production() -> Self {
        static PRODUCTION: OnceLock<ActionBus> = OnceLock::new();
        PRODUCTION.get_or_init(Self::new).clone()
    }

    pub fn register<F: ToolJobFactory>(&self, factory: F) -> Result<(), ToolRegistrationError> {
        let keys = factory.keys();
        if keys.is_empty() {
            return Err(ToolRegistrationError::EmptyFactory);
        }
        let classification = factory.classification();
        if classification != InteractiveJobClassification::Migrated {
            return Err(ToolRegistrationError::NonInteractiveClassification { key: keys[0].clone(), classification });
        }
        let contract = factory.execution_contract();
        if let Err(detail) = contract.validate() {
            return Err(ToolRegistrationError::InvalidExecutionContract { key: keys[0].clone(), detail });
        }
        let mut inner = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut factory_keys = HashSet::with_capacity(keys.len());
        for key in keys {
            if !factory_keys.insert(key) || inner.factory_by_key.contains_key(key) {
                return Err(ToolRegistrationError::DuplicateKey { key: key.clone() });
            }
        }
        let index = inner.factories.len();
        for key in keys {
            inner.factory_by_key.insert(key.clone(), index);
            inner.factory_identity_by_key.insert(key.clone(), (TypeId::of::<F>(), std::any::type_name::<F>(), factory.payload_schema_id().to_string()));
            inner.contract_by_key.insert(key.clone(), contract);
        }
        inner.factories.push(Box::new(ToolJobFactoryAdapter { factory }));
        Ok(())
    }

    /// 🫂️ Idempotent activation for a controller whose generated key set may be mounted more than once.
    pub fn register_once<F: ToolJobFactory>(&self, factory: F) -> Result<(), ToolRegistrationError> {
        let keys = factory.keys();
        if keys.is_empty() {
            return Err(ToolRegistrationError::EmptyFactory);
        }
        let classification = factory.classification();
        if classification != InteractiveJobClassification::Migrated {
            return Err(ToolRegistrationError::NonInteractiveClassification { key: keys[0].clone(), classification });
        }
        let contract = factory.execution_contract();
        if let Err(detail) = contract.validate() {
            return Err(ToolRegistrationError::InvalidExecutionContract { key: keys[0].clone(), detail });
        }
        let identity = (TypeId::of::<F>(), std::any::type_name::<F>(), factory.payload_schema_id());
        let mut inner = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for key in keys {
            if let Some((registered_type_id, registered_type_name, registered_schema)) = inner.factory_identity_by_key.get(key) {
                if (*registered_type_id, *registered_type_name, registered_schema.as_str()) != identity {
                    return Err(ToolRegistrationError::DuplicateKey { key: key.clone() });
                }
            }
        }
        let missing = keys.iter().filter(|key| !inner.factory_by_key.contains_key(*key)).cloned().collect::<Vec<_>>();
        if missing.is_empty() {
            return Ok(());
        }
        let index = inner.factories.len();
        for key in missing {
            inner.factory_by_key.insert(key.clone(), index);
            inner.factory_identity_by_key.insert(key.clone(), (identity.0, identity.1, identity.2.to_string()));
            inner.contract_by_key.insert(key, contract);
        }
        inner.factories.push(Box::new(ToolJobFactoryAdapter { factory }));
        Ok(())
    }

    pub fn contains(&self, key: &ToolFactoryKey) -> bool {
        let inner = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        inner.factory_by_key.contains_key(key) || inner.aliases.contains_key(key)
    }

    /// 🏷️ Registers one explicit alias only after its exact target factory and contract exist.
    pub fn register_alias(&self, alias: ToolFactoryKey, target: ToolFactoryKey) -> Result<(), ToolRegistrationError> {
        let mut inner = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if inner.factory_by_key.contains_key(&alias) || inner.aliases.contains_key(&alias) {
            return Err(ToolRegistrationError::DuplicateKey { key: alias });
        }
        if !inner.factory_by_key.contains_key(&target) {
            return Err(ToolRegistrationError::UnknownAliasTarget { alias, target });
        }
        inner.aliases.insert(alias, target);
        Ok(())
    }

    /// 🧬️ Returns the exact schema id owned by one registered controller/tool key.
    pub fn payload_schema_id(&self, key: &ToolFactoryKey) -> Option<String> {
        self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).factory_identity_by_key.get(key).map(|(_, _, schema)| schema.clone())
    }

    pub fn keys(&self) -> Vec<ToolFactoryKey> {
        let inner = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        inner.factory_by_key.keys().chain(inner.aliases.keys()).cloned().collect()
    }

    pub fn dispatch_count(&self) -> u64 {
        self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).dispatch_count
    }

    /// 📡️ Reserves the exact raw extent before any caller-specific decoder, command identity,
    /// or application allocation runs.
    pub fn begin_exact_wire(&self, controller_id: impl Into<String>, tool_id: impl Into<String>, schema_id: impl Into<String>, declared_bytes: usize) -> Result<(ToolWireAdmission, RetainedToolWireInput), ToolDispatchError> {
        let controller_id = controller_id.into();
        let tool_id = tool_id.into();
        let schema_id = schema_id.into();
        let key = ToolFactoryKey::new(controller_id.clone(), tool_id.clone());
        let inner = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let contract = *inner.contract_by_key.get(&key).ok_or_else(|| ToolDispatchError::UnknownController { controller_id: format!("{controller_id}/{tool_id}") })?;
        if declared_bytes > contract.max_raw_wire_bytes {
            return Err(ToolDispatchError::RawWireLimit { controller_id, tool_id, actual: declared_bytes, maximum: contract.max_raw_wire_bytes });
        }
        let (factory_type_id, factory_type_name, expected_schema) = inner.factory_identity_by_key.get(&key).expect("factory identity is registered atomically with its key");
        if expected_schema != &schema_id {
            return Err(ToolDispatchError::Factory { controller_id, tool_id, detail: format!("expected payload schema '{expected_schema}', got '{schema_id}'") });
        }
        let input = RetainedToolWireInput::try_new(declared_bytes, contract.max_raw_wire_bytes).map_err(|error| ToolDispatchError::Factory { controller_id: controller_id.clone(), tool_id: tool_id.clone(), detail: error.detail })?;
        Ok((ToolWireAdmission { key, factory_type_id: *factory_type_id, factory_type_name, schema_id, contract }, input))
    }

    /// 🛠️ Admits an exact owner/tool/schema wire envelope before any caller-specific decoder runs.
    pub fn admit_exact_wire(&self, controller_id: impl Into<String>, tool_id: impl Into<String>, schema_id: impl Into<String>, payload: &[u8]) -> Result<ToolWireAdmission, ToolDispatchError> {
        self.begin_exact_wire(controller_id, tool_id, schema_id, payload.len()).map(|(admission, _)| admission)
    }

    /// 📃️ Moves one sealed raw-page owner into the exact registered application factory.
    /// No generic command or serialization value exists before this boundary.
    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Refusal returns existing wire page and checkpoint owners directly, avoiding an additional heap wrapper."))]
    /// 🏭️ Borrows the exact registered factory and original input owners without allocating a lookup key.
    pub fn admission_demands(&self,spec:&ToolOperationSpec,input:Option<&RetainedToolWireInput>,checkpoint:Option<&RetainedToolWireInput>)->Result<ToolJobAdmissionDemand,semio_framework_value::ValueError>{
        use semio_framework_value::{ValueError,ValueRefusalKind};
        let inner=self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let index=inner.factory_by_key.get(spec.key()).copied().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original tool factory key is not registered"))?;
        inner.factories[index].admission_demands(spec,input,checkpoint)
    }

    /// 🚪️ Pending and refused turns retain caller slots; born candidates remain owned before receipt acceptance.
    pub fn admit_job(&self,context:&mut StepContext<'_>,spec:&mut ToolOperationSpec,input:&mut Option<RetainedToolWireInput>,checkpoint:&mut Option<RetainedToolWireInput>,output:&mut Option<ErasedToolJob>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_value::{ValueError,ValueRefusalKind};
        let mut inner=self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let index=inner.factory_by_key.get(spec.key()).copied().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original tool factory key is not registered"))?;
        let admitted=inner.factories[index].admit_job(context,spec,input,checkpoint,output)?;
        if admitted{inner.dispatch_count=inner.dispatch_count.saturating_add(1);}
        Ok(admitted)
    }

}

//#endregion 🚌️Bus
//#endregion 🎯️ToolJobBus

#[path = "../🖥️platform/🦀️.rs"]
pub mod platform;

#[cfg(test)]
#[global_allocator]
static TEST_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

//#region 🎯️ToolJobBus
#[cfg(test)]
#[path = "🧹️wire-retirement/🧪️tests/🔬️standalone/🦀️.rs"]
mod wire_retirement_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🎯️ToolJobBus
