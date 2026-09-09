//#region 🎯️ToolJobBus
//! 🎯️ Typed routing from renderer actions to resumable interactive jobs.
//!
//! Every UI-reachable tool enters through one [`ToolOperationSpec`], is resolved by a
//! [`ToolJobFactory`], and leaves as a worker-drivable [`ToolJobDispatch`]. Factories are accepted
//! only when their manifest classification is [`InteractiveJobClassification::Migrated`], so a
//! callback, batch-only command, forbidden command, or unclassified command cannot become reachable
//! merely by being inserted into the bus.

use crate::manifest::InteractiveJobClassification;
use dsl::DslValue;
use semio_framework_job::{InteractiveJob, Operation, StepContext, StepOutcome};
use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex, OnceLock};

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
    value: Box<dyn Any + Send>,
}

impl ToolPayload {
    pub fn new<T: Send + 'static>(schema_id: impl Into<String>, value: T) -> Self {
        Self { schema_id: schema_id.into(), value: Box::new(value) }
    }

    fn downcast<T: Send + 'static>(self) -> Result<T, ToolJobFactoryError> {
        self.value.downcast::<T>().map(|value| *value).map_err(|_| ToolJobFactoryError::new(format!("tool payload '{}' has the wrong Rust payload type", self.schema_id)))
    }
}

pub struct ToolOperationSpec {
    pub controller_id: String,
    pub tool_id: String,
    pub payload: ToolPayload,
    pub operation: Operation,
}

impl ToolOperationSpec {
    pub fn new<T: Send + 'static>(controller_id: impl Into<String>, tool_id: impl Into<String>, schema_id: impl Into<String>, payload: T, operation: Operation) -> Self {
        Self { controller_id: controller_id.into(), tool_id: tool_id.into(), payload: ToolPayload::new(schema_id, payload), operation }
    }

    pub fn key(&self) -> ToolFactoryKey {
        ToolFactoryKey::new(self.controller_id.clone(), self.tool_id.clone())
    }
}

pub struct ToolJobDispatch {
    pub spec: ToolOperationSpec,
    pub job: ErasedToolJob,
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
    fn try_new(declared_bytes: usize, maximum_bytes: usize) -> Result<Self, ToolJobFactoryError> {
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

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        self.begin_close();
        if self.terminal_is_empty() {
            return semio_framework_job::InteractiveJobCloseStep::Complete;
        }
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if let Some(page) = self.pages.last_mut() {
            if !page.is_empty() && maximum_bytes == 0 {
                return semio_framework_job::InteractiveJobCloseStep::Blocked;
            }
            let released_bytes = maximum_bytes.min(page.len);
            page.len -= released_bytes;
            self.admitted_bytes = self.admitted_bytes.checked_sub(released_bytes).expect("retained wire byte accounting diverged");
            let released_items = usize::from(page.is_empty());
            if released_items != 0 {
                self.pages.truncate(self.pages.len() - 1);
            }
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        drop(std::mem::take(&mut self.pages));
        semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.pages.is_empty() && self.pages.capacity() == 0 && self.admitted_bytes == 0
    }
}

pub struct RetainedToolWireDispatchRejected {
    pub error: ToolDispatchError,
    pub input: RetainedToolWireInput,
    pub checkpoint: Option<RetainedToolWireInput>,
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
    /// 🧵️ `+ Send`: an erased tool job is dispatched onto the pool, so the object type states the
    /// thread-transfer requirement itself now that `InteractiveJob` no longer imposes `Send` on
    /// single-threaded targets (see `semio_framework_job::JobThreadTransfer`).
    inner: Box<dyn InteractiveJob + Send>,
}

impl ErasedToolJob {
    fn new<J: InteractiveJob + Send + 'static>(job: J) -> Self {
        Self { inner: Box::new(job) }
    }
}

impl InteractiveJob for ErasedToolJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        self.inner.step(cx)
    }

    fn begin_close(&mut self) {
        self.inner.begin_close();
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        self.inner.close_step(maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.inner.terminal_is_empty()
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
    fn create_job(&mut self, operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError>;

    /// 🌉 Decodes the owned cold-job wire payload without exposing a serialization dependency
    /// through the action-bus API. Factories mounted on a wire route override this explicitly; UI-only
    /// factories retain the typed [`ToolOperationSpec`] path and reject wire dispatch.
    fn create_job_from_wire(&mut self, _operation: Operation, _payload: &[u8], _checkpoint: Option<Vec<u8>>) -> Result<Self::Job, ToolJobFactoryError> {
        Err(ToolJobFactoryError::new("tool factory does not own a wire payload decoder"))
    }

    /// 🧬️ Takes the already-admitted raw page owner before application parsing or identity
    /// construction. Rejection returns every retained owner to the caller for bounded retirement.
    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Refusal returns existing wire page and checkpoint owners directly, avoiding an additional heap wrapper."))]
    fn create_job_from_wire_pages(&mut self, _operation: Operation, input: RetainedToolWireInput, checkpoint: Option<RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, RetainedToolWireInput, Option<RetainedToolWireInput>)> {
        Err((ToolJobFactoryError::new("tool factory does not own a retained wire page decoder"), input, checkpoint))
    }

    /// 🧬️ Builds the concrete factory payload while transferring the already-admitted raw
    /// pages into the same production job. Factories with a domain decoder override this so their job
    /// consumes one retained page per step; the default preserves the pages only as factory authority.
    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Refusal returns existing wire page and checkpoint owners directly, avoiding an additional heap wrapper."))]
    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: Operation,
        _payload: Self::Payload,
        input: RetainedToolWireInput,
        checkpoint: Option<RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, RetainedToolWireInput, Option<RetainedToolWireInput>)> {
        Err((ToolJobFactoryError::new("tool factory does not own retained wire pages alongside its typed payload"), input, checkpoint))
    }
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

#[derive(Clone, Debug, PartialEq, Eq)]
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
trait ErasedToolJobFactory: Send {
    fn create_job(&mut self, spec: &mut ToolOperationSpec) -> Result<ErasedToolJob, ToolJobFactoryError>;
    fn create_job_from_wire(&mut self, operation: Operation, payload: &[u8], checkpoint: Option<Vec<u8>>) -> Result<ErasedToolJob, ToolJobFactoryError>;
    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Refusal returns existing wire page and checkpoint owners directly, avoiding an additional heap wrapper."))]
    fn create_job_from_wire_pages(&mut self, operation: Operation, input: RetainedToolWireInput, checkpoint: Option<RetainedToolWireInput>) -> Result<ErasedToolJob, (ToolJobFactoryError, RetainedToolWireInput, Option<RetainedToolWireInput>)>;
    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Refusal returns existing wire page and checkpoint owners directly, avoiding an additional heap wrapper."))]
    fn create_job_from_wire_pages_with_payload(
        &mut self,
        spec: &mut ToolOperationSpec,
        input: RetainedToolWireInput,
        checkpoint: Option<RetainedToolWireInput>,
    ) -> Result<ErasedToolJob, (ToolJobFactoryError, RetainedToolWireInput, Option<RetainedToolWireInput>)>;
}

struct ToolJobFactoryAdapter<F: ToolJobFactory> {
    factory: F,
}

impl<F: ToolJobFactory> ErasedToolJobFactory for ToolJobFactoryAdapter<F> {
    fn create_job(&mut self, spec: &mut ToolOperationSpec) -> Result<ErasedToolJob, ToolJobFactoryError> {
        if spec.payload.schema_id != self.factory.payload_schema_id() {
            return Err(ToolJobFactoryError::new(format!("expected payload schema '{}', got '{}'", self.factory.payload_schema_id(), spec.payload.schema_id)));
        }
        let placeholder = ToolPayload::new(spec.payload.schema_id.clone(), ());
        let payload = std::mem::replace(&mut spec.payload, placeholder).downcast::<F::Payload>()?;
        self.factory.create_job(spec.operation, payload).map(ErasedToolJob::new)
    }

    fn create_job_from_wire(&mut self, operation: Operation, payload: &[u8], checkpoint: Option<Vec<u8>>) -> Result<ErasedToolJob, ToolJobFactoryError> {
        self.factory.create_job_from_wire(operation, payload, checkpoint).map(ErasedToolJob::new)
    }

    fn create_job_from_wire_pages(&mut self, operation: Operation, input: RetainedToolWireInput, checkpoint: Option<RetainedToolWireInput>) -> Result<ErasedToolJob, (ToolJobFactoryError, RetainedToolWireInput, Option<RetainedToolWireInput>)> {
        self.factory.create_job_from_wire_pages(operation, input, checkpoint).map(ErasedToolJob::new)
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        spec: &mut ToolOperationSpec,
        input: RetainedToolWireInput,
        checkpoint: Option<RetainedToolWireInput>,
    ) -> Result<ErasedToolJob, (ToolJobFactoryError, RetainedToolWireInput, Option<RetainedToolWireInput>)> {
        if spec.payload.schema_id != self.factory.payload_schema_id() {
            return Err((ToolJobFactoryError::new(format!("expected payload schema '{}', got '{}'", self.factory.payload_schema_id(), spec.payload.schema_id)), input, checkpoint));
        }
        let placeholder = ToolPayload::new(spec.payload.schema_id.clone(), ());
        let payload = match std::mem::replace(&mut spec.payload, placeholder).downcast::<F::Payload>() {
            Ok(payload) => payload,
            Err(error) => return Err((error, input, checkpoint)),
        };
        self.factory.create_job_from_wire_pages_with_payload(spec.operation, payload, input, checkpoint).map(ErasedToolJob::new)
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

    /// 🛡️ Reserves the exact raw extent before any caller-specific decoder, command identity,
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

    /// 🛡️ Admits an exact owner/tool/schema wire envelope before any caller-specific decoder runs.
    pub fn admit_exact_wire(&self, controller_id: impl Into<String>, tool_id: impl Into<String>, schema_id: impl Into<String>, payload: &[u8]) -> Result<ToolWireAdmission, ToolDispatchError> {
        self.begin_exact_wire(controller_id, tool_id, schema_id, payload.len()).map(|(admission, _)| admission)
    }

    /// 🧬️ Moves one sealed raw-page owner into the exact registered application factory.
    /// No generic command or serialization value exists before this boundary.
    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Refusal returns existing wire page and checkpoint owners directly, avoiding an additional heap wrapper."))]
    pub fn dispatch_wire_retained(&self, admission: ToolWireAdmission, input: RetainedToolWireInput, checkpoint: Option<RetainedToolWireInput>, operation: Operation) -> Result<ToolJobDispatch, RetainedToolWireDispatchRejected> {
        let reject = |error, input, checkpoint| RetainedToolWireDispatchRejected { error, input, checkpoint };
        let controller_id = admission.key.controller_id.clone();
        let tool_id = admission.key.tool_id.clone();
        if !input.sealed || input.closing || input.declared_bytes != input.admitted_bytes || input.maximum_bytes != admission.contract.max_raw_wire_bytes {
            return Err(reject(ToolDispatchError::Factory { controller_id, tool_id, detail: "retained tool wire owner is not exactly sealed to its admission".to_string() }, input, checkpoint));
        }
        let mut inner = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(index) = inner.factory_by_key.get(&admission.key).copied() else {
            return Err(reject(ToolDispatchError::UnknownController { controller_id: format!("{controller_id}/{tool_id}") }, input, checkpoint));
        };
        let Some((factory_type_id, factory_type_name, schema_id)) = inner.factory_identity_by_key.get(&admission.key) else {
            return Err(reject(ToolDispatchError::UnknownController { controller_id: format!("{controller_id}/{tool_id}") }, input, checkpoint));
        };
        let current_contract = *inner.contract_by_key.get(&admission.key).expect("factory contract is registered atomically with its key");
        if *factory_type_id != admission.factory_type_id || *factory_type_name != admission.factory_type_name || schema_id != &admission.schema_id || current_contract != admission.contract {
            return Err(reject(ToolDispatchError::Factory { controller_id, tool_id, detail: "tool wire admission became stale before factory transfer".to_string() }, input, checkpoint));
        }
        let factory = inner.factories.get_mut(index).expect("factory index is registered atomically with its keys");
        let job = match factory.create_job_from_wire_pages(operation, input, checkpoint) {
            Ok(job) => job,
            Err((error, input, checkpoint)) => {
                return Err(reject(ToolDispatchError::Factory { controller_id, tool_id, detail: error.detail }, input, checkpoint));
            }
        };
        inner.dispatch_count = inner.dispatch_count.saturating_add(1);
        let spec = ToolOperationSpec::new(admission.key.controller_id, admission.key.tool_id, admission.schema_id, (), operation);
        Ok(ToolJobDispatch { spec, job })
    }

    /// 🧬️ Transfers a concrete app payload and its exact retained ingress pages through the
    /// same registered factory. This is the production route for factories whose worker performs the
    /// domain decode incrementally before it starts the prepared reducer payload.
    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Refusal returns existing wire page and checkpoint owners directly, avoiding an additional heap wrapper."))]
    pub fn dispatch_wire_retained_with_spec(&self, admission: &ToolWireAdmission, input: RetainedToolWireInput, checkpoint: Option<RetainedToolWireInput>, mut spec: ToolOperationSpec) -> Result<ToolJobDispatch, RetainedToolWireDispatchRejected> {
        let reject = |error, input, checkpoint| RetainedToolWireDispatchRejected { error, input, checkpoint };
        let controller_id = admission.key.controller_id.clone();
        let tool_id = admission.key.tool_id.clone();
        if spec.key() != admission.key || spec.payload.schema_id != admission.schema_id {
            return Err(reject(ToolDispatchError::Factory { controller_id, tool_id, detail: "typed payload does not match its retained wire admission".to_string() }, input, checkpoint));
        }
        if !input.sealed || input.closing || input.declared_bytes != input.admitted_bytes || input.maximum_bytes != admission.contract.max_raw_wire_bytes {
            return Err(reject(ToolDispatchError::Factory { controller_id, tool_id, detail: "retained tool wire owner is not exactly sealed to its admission".to_string() }, input, checkpoint));
        }
        let mut inner = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(index) = inner.factory_by_key.get(&admission.key).copied() else {
            return Err(reject(ToolDispatchError::UnknownController { controller_id: format!("{controller_id}/{tool_id}") }, input, checkpoint));
        };
        let Some((factory_type_id, factory_type_name, schema_id)) = inner.factory_identity_by_key.get(&admission.key) else {
            return Err(reject(ToolDispatchError::UnknownController { controller_id: format!("{controller_id}/{tool_id}") }, input, checkpoint));
        };
        let current_contract = *inner.contract_by_key.get(&admission.key).expect("factory contract is registered atomically with its key");
        if *factory_type_id != admission.factory_type_id || *factory_type_name != admission.factory_type_name || schema_id != &admission.schema_id || current_contract != admission.contract {
            return Err(reject(ToolDispatchError::Factory { controller_id, tool_id, detail: "tool wire admission became stale before concrete factory transfer".to_string() }, input, checkpoint));
        }
        let factory = inner.factories.get_mut(index).expect("factory index is registered atomically with its keys");
        let job = match factory.create_job_from_wire_pages_with_payload(&mut spec, input, checkpoint) {
            Ok(job) => job,
            Err((error, input, checkpoint)) => {
                return Err(reject(ToolDispatchError::Factory { controller_id, tool_id, detail: error.detail }, input, checkpoint));
            }
        };
        inner.dispatch_count = inner.dispatch_count.saturating_add(1);
        Ok(ToolJobDispatch { spec, job })
    }

    pub fn dispatch(&self, mut spec: ToolOperationSpec) -> Result<ToolJobDispatch, ToolDispatchError> {
        let key = spec.key();
        let controller_id = key.controller_id.clone();
        let tool_id = key.tool_id.clone();
        let mut inner = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let index = inner.factory_by_key.get(&key).copied().ok_or_else(|| ToolDispatchError::UnknownController { controller_id: format!("{controller_id}/{tool_id}") })?;
        let factory = inner.factories.get_mut(index).expect("factory index is registered atomically with its keys");
        let job = factory.create_job(&mut spec).map_err(|error| ToolDispatchError::Factory { controller_id, tool_id, detail: error.detail })?;
        inner.dispatch_count = inner.dispatch_count.saturating_add(1);
        Ok(ToolJobDispatch { spec, job })
    }

    /// 🌉 Resolves an owned byte payload through the same exact controller/tool identity as a
    /// typed UI dispatch. The factory alone owns decoding into its schema-first payload type; an
    /// optional lossless checkpoint is supplied separately so a cold-job restart never has to mutate
    /// or reinterpret the authoritative input bytes.
    pub fn dispatch_wire(&self, controller_id: impl Into<String>, tool_id: impl Into<String>, schema_id: impl Into<String>, payload: &[u8], checkpoint: Option<Vec<u8>>, operation: Operation) -> Result<ToolJobDispatch, ToolDispatchError> {
        let controller_id = controller_id.into();
        let tool_id = tool_id.into();
        let schema_id = schema_id.into();
        let key = ToolFactoryKey::new(controller_id.clone(), tool_id.clone());
        let mut inner = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let index = inner.factory_by_key.get(&key).copied().ok_or_else(|| ToolDispatchError::UnknownController { controller_id: format!("{controller_id}/{tool_id}") })?;
        let contract = *inner.contract_by_key.get(&key).expect("execution contract is registered atomically with its key");
        if payload.len() > contract.max_raw_wire_bytes {
            return Err(ToolDispatchError::RawWireLimit { controller_id, tool_id, actual: payload.len(), maximum: contract.max_raw_wire_bytes });
        }
        let expected_schema = inner.factory_identity_by_key.get(&key).map(|(_, _, schema)| schema.clone()).expect("factory identity is registered atomically with its key");
        if expected_schema != schema_id {
            return Err(ToolDispatchError::Factory { controller_id, tool_id, detail: format!("expected payload schema '{expected_schema}', got '{schema_id}'") });
        }
        let factory = inner.factories.get_mut(index).expect("factory index is registered atomically with its keys");
        let job = factory.create_job_from_wire(operation, payload, checkpoint).map_err(|error| ToolDispatchError::Factory { controller_id: controller_id.clone(), tool_id: tool_id.clone(), detail: error.detail })?;
        inner.dispatch_count = inner.dispatch_count.saturating_add(1);
        let spec = ToolOperationSpec::new(controller_id, tool_id, schema_id, (), operation);
        Ok(ToolJobDispatch { spec, job })
    }
}
//#endregion 🚌️Bus

/// 🌉️ Bridges staged JSON action args into the owned DSL boundary.
pub fn optional_json_to_dsl(args: Option<serde_json::Value>) -> Option<DslValue> {
    args.map(DslValue::from)
}

#[cfg(test)]
#[path = "🧹️wire-retirement/🧪️tests/🔬️standalone/🦀️.rs"]
mod wire_retirement_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🎯️ToolJobBus
