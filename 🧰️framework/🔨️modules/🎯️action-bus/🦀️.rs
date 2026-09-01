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
    inner: Box<dyn InteractiveJob>,
}

impl ErasedToolJob {
    fn new<J: InteractiveJob + 'static>(job: J) -> Self {
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
    type Job: InteractiveJob + 'static;

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
    fn create_job_from_wire_pages(
        &mut self,
        _operation: Operation,
        input: RetainedToolWireInput,
        checkpoint: Option<RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, RetainedToolWireInput, Option<RetainedToolWireInput>)> {
        Err((ToolJobFactoryError::new("tool factory does not own a retained wire page decoder"), input, checkpoint))
    }

    /// 🧬️ Builds the concrete factory payload while transferring the already-admitted raw
    /// pages into the same production job. Factories with a domain decoder override this so their job
    /// consumes one retained page per step; the default preserves the pages only as factory authority.
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
    fn create_job_from_wire_pages(
        &mut self,
        operation: Operation,
        input: RetainedToolWireInput,
        checkpoint: Option<RetainedToolWireInput>,
    ) -> Result<ErasedToolJob, (ToolJobFactoryError, RetainedToolWireInput, Option<RetainedToolWireInput>)>;
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

    fn create_job_from_wire_pages(
        &mut self,
        operation: Operation,
        input: RetainedToolWireInput,
        checkpoint: Option<RetainedToolWireInput>,
    ) -> Result<ErasedToolJob, (ToolJobFactoryError, RetainedToolWireInput, Option<RetainedToolWireInput>)> {
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
    pub fn begin_exact_wire(
        &self,
        controller_id: impl Into<String>,
        tool_id: impl Into<String>,
        schema_id: impl Into<String>,
        declared_bytes: usize,
    ) -> Result<(ToolWireAdmission, RetainedToolWireInput), ToolDispatchError> {
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
        let input = RetainedToolWireInput::try_new(declared_bytes, contract.max_raw_wire_bytes).map_err(|error| ToolDispatchError::Factory {
            controller_id: controller_id.clone(),
            tool_id: tool_id.clone(),
            detail: error.detail,
        })?;
        Ok((ToolWireAdmission { key, factory_type_id: *factory_type_id, factory_type_name, schema_id, contract }, input))
    }

    /// 🛡️ Admits an exact owner/tool/schema wire envelope before any caller-specific decoder runs.
    pub fn admit_exact_wire(&self, controller_id: impl Into<String>, tool_id: impl Into<String>, schema_id: impl Into<String>, payload: &[u8]) -> Result<ToolWireAdmission, ToolDispatchError> {
        self.begin_exact_wire(controller_id, tool_id, schema_id, payload.len()).map(|(admission, _)| admission)
    }

    /// 🧬️ Moves one sealed raw-page owner into the exact registered application factory.
    /// No generic command or serialization value exists before this boundary.
    pub fn dispatch_wire_retained(
        &self,
        admission: ToolWireAdmission,
        input: RetainedToolWireInput,
        checkpoint: Option<RetainedToolWireInput>,
        operation: Operation,
    ) -> Result<ToolJobDispatch, RetainedToolWireDispatchRejected> {
        let reject = |error, input, checkpoint| RetainedToolWireDispatchRejected { error, input, checkpoint };
        let controller_id = admission.key.controller_id.clone();
        let tool_id = admission.key.tool_id.clone();
        if !input.sealed || input.closing || input.declared_bytes != input.admitted_bytes || input.maximum_bytes != admission.contract.max_raw_wire_bytes {
            return Err(reject(
                ToolDispatchError::Factory { controller_id, tool_id, detail: "retained tool wire owner is not exactly sealed to its admission".to_string() },
                input,
                checkpoint,
            ));
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
            return Err(reject(
                ToolDispatchError::Factory { controller_id, tool_id, detail: "tool wire admission became stale before factory transfer".to_string() },
                input,
                checkpoint,
            ));
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
    pub fn dispatch_wire_retained_with_spec(
        &self,
        admission: ToolWireAdmission,
        input: RetainedToolWireInput,
        checkpoint: Option<RetainedToolWireInput>,
        mut spec: ToolOperationSpec,
    ) -> Result<ToolJobDispatch, RetainedToolWireDispatchRejected> {
        let reject = |error, input, checkpoint| RetainedToolWireDispatchRejected { error, input, checkpoint };
        let controller_id = admission.key.controller_id.clone();
        let tool_id = admission.key.tool_id.clone();
        if spec.key() != admission.key || spec.payload.schema_id != admission.schema_id {
            return Err(reject(
                ToolDispatchError::Factory { controller_id, tool_id, detail: "typed payload does not match its retained wire admission".to_string() },
                input,
                checkpoint,
            ));
        }
        if !input.sealed || input.closing || input.declared_bytes != input.admitted_bytes || input.maximum_bytes != admission.contract.max_raw_wire_bytes {
            return Err(reject(
                ToolDispatchError::Factory { controller_id, tool_id, detail: "retained tool wire owner is not exactly sealed to its admission".to_string() },
                input,
                checkpoint,
            ));
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
            return Err(reject(
                ToolDispatchError::Factory { controller_id, tool_id, detail: "tool wire admission became stale before concrete factory transfer".to_string() },
                input,
                checkpoint,
            ));
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
#[path = "🧹️wire-retirement/🦀️.rs"]
mod wire_retirement_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_job::{allocate_operation_id, CommitCandidate, Generation, RevisionId};

    struct ImmediateJob {
        output: Option<Vec<u8>>,
        writer: Option<semio_framework_job::RetainedJobPayloadWriter>,
        cursor: usize,
        closing: bool,
    }

    impl InteractiveJob for ImmediateJob {
        fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
            let writer = self.writer.get_or_insert_with(|| semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CommitOutput));
            if !writer.write_slice_page(cx, self.output.as_deref().unwrap_or_default(), &mut self.cursor).unwrap_or(false) {
                return StepOutcome::Yield;
            }
            self.output = None;
            let output = self.writer.take().expect("immediate output writer").finish().unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput));
            StepOutcome::Complete(CommitCandidate { state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState), output })
        }

        fn begin_close(&mut self) {
            self.closing = true;
            if let Some(writer) = self.writer.as_mut() {
                writer.begin_close();
            }
        }

        fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
            self.begin_close();
            if let Some(writer) = self.writer.as_mut() {
                return match writer.close_step(maximum_items, maximum_bytes) {
                    semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                    semio_framework_job::JobPayloadCloseStep::Complete => {
                        self.writer = None;
                        semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                    }
                };
            }
            if self.output.is_some() {
                if maximum_items == 0 {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                self.output = None;
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            semio_framework_job::InteractiveJobCloseStep::Complete
        }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.output.is_none() && self.writer.is_none()
        }
    }

    struct EchoFactory {
        keys: Vec<ToolFactoryKey>,
        classification: InteractiveJobClassification,
    }

    impl ToolJobFactory for EchoFactory {
        type Payload = String;
        type Job = ImmediateJob;

        fn keys(&self) -> &[ToolFactoryKey] {
            &self.keys
        }

        fn payload_schema_id(&self) -> &str {
            "test.echo.v1"
        }

        fn classification(&self) -> InteractiveJobClassification {
            self.classification
        }

        fn execution_contract(&self) -> ToolExecutionContract {
            ToolExecutionContract::bounded_first_step(64, 1, 1, 64, 100)
        }

        fn create_job(&mut self, _operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
            Ok(ImmediateJob { output: Some(format!("{payload}:ok").into_bytes()), writer: None, cursor: 0, closing: false })
        }
    }

    fn operation_spec(controller_id: &str, tool_id: &str) -> ToolOperationSpec {
        ToolOperationSpec::new(controller_id, tool_id, "test.echo.v1", tool_id.to_string(), Operation::new(allocate_operation_id(), RevisionId(7), Generation(3), 11))
    }

    fn echo_factory(controller_id: &str, tool_ids: &[&str], classification: InteractiveJobClassification) -> EchoFactory {
        EchoFactory { keys: tool_ids.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect(), classification }
    }

    #[test]
    fn dispatch_returns_a_resumable_job_and_preserves_operation_identity() {
        let bus = ActionBus::new();
        bus.register(echo_factory("app", &["ping"], InteractiveJobClassification::Migrated)).unwrap();
        let dispatch = bus.dispatch(operation_spec("app", "ping")).unwrap();
        assert_eq!(dispatch.spec.operation.base_revision, RevisionId(7));
        assert_eq!(dispatch.spec.operation.generation, Generation(3));
    }

    #[test]
    fn registration_rejects_every_non_migrated_factory() {
        for classification in [InteractiveJobClassification::Unclassified, InteractiveJobClassification::BatchOnlyPendingRewrite, InteractiveJobClassification::ForbiddenFromUi, InteractiveJobClassification::Deleted] {
            let bus = ActionBus::new();
            assert!(matches!(
                bus.register(echo_factory("app", &["ping"], classification)),
                Err(ToolRegistrationError::NonInteractiveClassification { classification: rejected, .. }) if rejected == classification
            ));
        }
    }

    #[test]
    fn unknown_controller_is_an_explicit_dispatch_error() {
        let bus = ActionBus::new();
        assert!(matches!(bus.dispatch(operation_spec("missing", "ping")), Err(ToolDispatchError::UnknownController { .. })));
    }

    #[test]
    fn aliases_require_an_existing_exact_factory_and_never_fallback() {
        let bus = ActionBus::new();
        let alias = ToolFactoryKey::new("app", "alias");
        let target = ToolFactoryKey::new("app", "exact");
        assert!(matches!(bus.register_alias(alias.clone(), target.clone()), Err(ToolRegistrationError::UnknownAliasTarget { .. })));
        bus.register(echo_factory("app", &["exact"], InteractiveJobClassification::Migrated)).unwrap();
        bus.register_alias(alias, target).unwrap();
        assert!(matches!(bus.dispatch(operation_spec("app", "alias")), Err(ToolDispatchError::UnknownController { .. })));
        assert!(matches!(bus.dispatch(operation_spec("app", "missing")), Err(ToolDispatchError::UnknownController { .. })));
    }

    #[test]
    fn exact_wire_admission_rejects_alias_schema_and_raw_limit_before_decode() {
        let bus = ActionBus::new();
        let exact = ToolFactoryKey::new("app", "exact");
        let alias = ToolFactoryKey::new("app", "alias");
        bus.register(echo_factory("app", &["exact"], InteractiveJobClassification::Migrated)).unwrap();
        bus.register_alias(alias.clone(), exact.clone()).unwrap();
        assert!(matches!(bus.admit_exact_wire("app", "alias", "test.echo.v1", b"ok"), Err(ToolDispatchError::UnknownController { .. })));
        let operation = Operation::new(allocate_operation_id(), RevisionId(0), Generation(0), 1);
        assert!(matches!(bus.dispatch_wire("app", "alias", "test.echo.v1", b"ok", None, operation), Err(ToolDispatchError::UnknownController { .. })));
        assert!(matches!(bus.admit_exact_wire("app", "exact", "wrong.schema", b"ok"), Err(ToolDispatchError::Factory { .. })));
        assert!(matches!(bus.admit_exact_wire("app", "exact", "test.echo.v1", &[0; 65]), Err(ToolDispatchError::RawWireLimit { actual: 65, maximum: 64, .. })));
        let admission = bus.admit_exact_wire("app", "exact", "test.echo.v1", b"ok").unwrap();
        assert_eq!(admission.key, exact);
        assert_eq!(admission.factory_type_id, TypeId::of::<EchoFactory>());
        assert_eq!(admission.contract.max_raw_wire_bytes, 64);
    }

    struct NumberFactory {
        keys: Vec<ToolFactoryKey>,
    }

    impl ToolJobFactory for NumberFactory {
        type Payload = u64;
        type Job = ImmediateJob;

        fn keys(&self) -> &[ToolFactoryKey] {
            &self.keys
        }

        fn payload_schema_id(&self) -> &str {
            "test.number.v1"
        }

        fn classification(&self) -> InteractiveJobClassification {
            InteractiveJobClassification::Migrated
        }

        fn execution_contract(&self) -> ToolExecutionContract {
            ToolExecutionContract::bounded_first_step(8, 1, 1, 8, 100)
        }

        fn create_job(&mut self, _operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
            Ok(ImmediateJob { output: Some(payload.to_le_bytes().to_vec()), writer: None, cursor: 0, closing: false })
        }

        fn create_job_from_wire(&mut self, _operation: Operation, payload: &[u8], checkpoint: Option<Vec<u8>>) -> Result<Self::Job, ToolJobFactoryError> {
            let value = u64::from_le_bytes(payload.try_into().map_err(|_| ToolJobFactoryError::new("number wire payload must contain exactly eight bytes"))?);
            let mut output = value.to_le_bytes().to_vec();
            output.extend(checkpoint.unwrap_or_default());
            Ok(ImmediateJob { output: Some(output), writer: None, cursor: 0, closing: false })
        }
    }

    #[test]
    fn heterogeneous_factories_share_one_bus_with_exact_key_ownership() {
        let bus = ActionBus::new();
        bus.register(echo_factory("text", &["one", "two"], InteractiveJobClassification::Migrated)).unwrap();
        bus.register(NumberFactory { keys: vec![ToolFactoryKey::new("number", "encode")] }).unwrap();
        assert_eq!(bus.keys().len(), 3);
        assert!(bus.contains(&ToolFactoryKey::new("text", "two")));
        let spec = ToolOperationSpec::new("number", "encode", "test.number.v1", 42u64, Operation::new(allocate_operation_id(), RevisionId(0), Generation(0), 9));
        assert!(bus.dispatch(spec).is_ok());
    }

    #[test]
    fn duplicate_factory_key_is_rejected_without_partial_registration() {
        let bus = ActionBus::new();
        bus.register(echo_factory("app", &["one"], InteractiveJobClassification::Migrated)).unwrap();
        let result = bus.register(echo_factory("app", &["two", "one"], InteractiveJobClassification::Migrated));
        assert!(matches!(result, Err(ToolRegistrationError::DuplicateKey { key }) if key == ToolFactoryKey::new("app", "one")));
        assert_eq!(bus.keys().len(), 1);
    }

    #[test]
    fn duplicate_key_inside_one_factory_is_rejected_atomically() {
        let bus = ActionBus::new();
        let result = bus.register(echo_factory("app", &["same", "same"], InteractiveJobClassification::Migrated));
        assert!(matches!(result, Err(ToolRegistrationError::DuplicateKey { key }) if key == ToolFactoryKey::new("app", "same")));
        assert_eq!(bus.keys().len(), 0);
    }

    #[test]
    fn wire_dispatch_uses_the_factory_decoder_and_preserves_the_restart_checkpoint() {
        let bus = ActionBus::new();
        bus.register(NumberFactory { keys: vec![ToolFactoryKey::new("number", "decode-wire")] }).unwrap();
        let operation = Operation::new(allocate_operation_id(), RevisionId(19), Generation(5), 13);
        let mut dispatch = bus.dispatch_wire("number", "decode-wire", "test.number.v1", &42u64.to_le_bytes(), Some(vec![7, 8]), operation).expect("wire dispatch");
        let mut sequence = 0;
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        let mut expected = 42u64.to_le_bytes().to_vec();
        expected.extend([7, 8]);
        let StepOutcome::Complete(mut candidate) = dispatch.job.step(&mut context) else { panic!("wire job did not complete") };
        assert_eq!(candidate.output.page(0), Some(expected.as_slice()));
        assert_eq!(candidate.output.page_count(), 1);
        assert_eq!(
            candidate.output.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES),
            semio_framework_job::JobPayloadCloseStep::Pending { released_items: 1, released_bytes: expected.len() },
        );
        assert_eq!(candidate.output.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete);
        dispatch.job.begin_close();
        while !dispatch.job.terminal_is_empty() {
            let _ = dispatch.job.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
        assert!(matches!(bus.dispatch_wire("number", "decode-wire", "wrong.schema", &42u64.to_le_bytes(), None, operation), Err(ToolDispatchError::Factory { .. })));
        assert!(matches!(bus.dispatch_wire("number", "decode-wire", "test.number.v1", &[0; 9], None, operation), Err(ToolDispatchError::RawWireLimit { actual: 9, maximum: 8, .. })));
    }

    struct RetainedNumberJob {
        input: Option<RetainedToolWireInput>,
        bytes: [u8; 8],
        cursor: usize,
        output: Option<ImmediateJob>,
        closing: bool,
    }

    impl InteractiveJob for RetainedNumberJob {
        fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
            if cx.is_cancelled() {
                return StepOutcome::Cancelled;
            }
            if cx.should_yield() || cx.fuel_remaining() == 0 {
                return StepOutcome::Yield;
            }
            if self.cursor < self.bytes.len() {
                self.bytes[self.cursor] = self.input.as_ref().and_then(|input| input.page(0)).and_then(|page| page.get(self.cursor)).copied().unwrap_or_default();
                self.cursor += 1;
                cx.consume_fuel(1);
                return StepOutcome::CheckpointReady(semio_framework_job::Checkpoint {
                    state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CheckpointState),
                    applied_progress: self.cursor as u64,
                });
            }
            self.output
                .get_or_insert_with(|| ImmediateJob { output: Some(self.bytes.to_vec()), writer: None, cursor: 0, closing: false })
                .step(cx)
        }

        fn begin_close(&mut self) {
            self.closing = true;
            if let Some(input) = self.input.as_mut() {
                input.begin_close();
            }
            if let Some(output) = self.output.as_mut() {
                output.begin_close();
            }
        }

        fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
            self.begin_close();
            if let Some(input) = self.input.as_mut() {
                let step = input.close_step(maximum_items, maximum_bytes);
                if input.terminal_is_empty() {
                    self.input = None;
                }
                return match step {
                    semio_framework_job::InteractiveJobCloseStep::Complete => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
                    other => other,
                };
            }
            if let Some(output) = self.output.as_mut() {
                let step = output.close_step(maximum_items, maximum_bytes);
                if output.terminal_is_empty() {
                    self.output = None;
                }
                return match step {
                    semio_framework_job::InteractiveJobCloseStep::Complete => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
                    other => other,
                };
            }
            semio_framework_job::InteractiveJobCloseStep::Complete
        }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.input.is_none() && self.output.is_none()
        }
    }

    struct RetainedNumberFactory {
        keys: Vec<ToolFactoryKey>,
    }

    impl ToolJobFactory for RetainedNumberFactory {
        type Payload = RetainedNumberJob;
        type Job = RetainedNumberJob;

        fn keys(&self) -> &[ToolFactoryKey] {
            &self.keys
        }

        fn payload_schema_id(&self) -> &str {
            "test.retained-number.v1"
        }

        fn classification(&self) -> InteractiveJobClassification {
            InteractiveJobClassification::Migrated
        }

        fn execution_contract(&self) -> ToolExecutionContract {
            ToolExecutionContract::resumable(8, 1, 1, 8, 100, 1, 1)
        }

        fn create_job(&mut self, _operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
            Ok(payload)
        }

        fn create_job_from_wire_pages(
            &mut self,
            _operation: Operation,
            input: RetainedToolWireInput,
            checkpoint: Option<RetainedToolWireInput>,
        ) -> Result<Self::Job, (ToolJobFactoryError, RetainedToolWireInput, Option<RetainedToolWireInput>)> {
            if input.declared_bytes() != 8 || checkpoint.is_some() {
                return Err((ToolJobFactoryError::new("retained number requires eight bytes and no checkpoint"), input, checkpoint));
            }
            Ok(RetainedNumberJob { input: Some(input), bytes: [0; 8], cursor: 0, output: None, closing: false })
        }

        fn create_job_from_wire_pages_with_payload(
            &mut self,
            _operation: Operation,
            mut payload: Self::Payload,
            input: RetainedToolWireInput,
            checkpoint: Option<RetainedToolWireInput>,
        ) -> Result<Self::Job, (ToolJobFactoryError, RetainedToolWireInput, Option<RetainedToolWireInput>)> {
            if input.declared_bytes() != 8 || checkpoint.is_some() || payload.input.is_some() {
                return Err((ToolJobFactoryError::new("retained number payload requires one eight-byte raw owner and no checkpoint"), input, checkpoint));
            }
            payload.input = Some(input);
            Ok(payload)
        }
    }

    #[test]
    fn retained_wire_pages_are_admitted_sealed_transferred_and_closed_by_logical_bytes() {
        let bus = ActionBus::new();
        bus.register(RetainedNumberFactory { keys: vec![ToolFactoryKey::new("number", "retained")] }).unwrap();
        let (admission, mut input) = bus.begin_exact_wire("number", "retained", "test.retained-number.v1", 8).unwrap();
        input.admit_page(ToolWirePage::try_copy_from(&42u64.to_le_bytes()).unwrap()).unwrap();
        assert!(input.seal().is_ok());
        let operation = Operation::new(allocate_operation_id(), RevisionId(1), Generation(2), 3);
        let mut dispatch = match bus.dispatch_wire_retained(admission, input, None, operation) {
            Ok(dispatch) => dispatch,
            Err(_) => panic!("retained dispatch was rejected"),
        };
        let mut sequence = 0;
        for _ in 0..8 {
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
            assert!(matches!(dispatch.job.step(&mut context), StepOutcome::CheckpointReady(_)));
        }
        dispatch.job.begin_close();
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧹️wire-retirement/🧪️fixture/🔣️.json")).unwrap();
        let law = &fixture["shortClose"];
        let mut released = 0;
        for row in law["steps"].as_array().unwrap() {
            let items = usize::try_from(row["items"].as_u64().unwrap()).unwrap();
            let bytes = usize::try_from(row["bytes"].as_u64().unwrap()).unwrap();
            let released_bytes = usize::try_from(row["releasedBytes"].as_u64().unwrap()).unwrap();
            let expected = if row["blocked"].as_bool().unwrap() { semio_framework_job::InteractiveJobCloseStep::Blocked }
                else { semio_framework_job::InteractiveJobCloseStep::Pending { released_items: usize::try_from(row["releasedItems"].as_u64().unwrap()).unwrap(), released_bytes } };
            assert_eq!(dispatch.job.close_step(items, bytes), expected);
            released += released_bytes;
            assert_eq!(released + usize::try_from(row["remaining"].as_u64().unwrap()).unwrap(), 8);
        }
        assert_eq!(released, 8);
        assert_eq!(dispatch.job.close_step(1, 8), semio_framework_job::InteractiveJobCloseStep::Pending {
            released_items: usize::try_from(law["backingReleaseItems"].as_u64().unwrap()).unwrap(),
            released_bytes: usize::try_from(law["backingReleaseLogicalBytes"].as_u64().unwrap()).unwrap(),
        });
        assert_eq!(dispatch.job.close_step(1, 8), semio_framework_job::InteractiveJobCloseStep::Complete);
        assert!(dispatch.job.terminal_is_empty());
        eprintln!("[DEBUG] retained-number-close zero-items=blocked zero-bytes=blocked logical=7+1 backing-logical=0 terminal=true");
    }

    #[test]
    fn production_typed_payload_and_retained_pages_enter_the_same_registered_factory_job() {
        let bus = ActionBus::new();
        bus.register(RetainedNumberFactory { keys: vec![ToolFactoryKey::new("number", "retained")] }).unwrap();
        let (admission, mut input) = bus.begin_exact_wire("number", "retained", "test.retained-number.v1", 8).unwrap();
        input.admit_page(ToolWirePage::try_copy_from(&42u64.to_le_bytes()).unwrap()).unwrap();
        input.seal().unwrap();
        let operation = Operation::new(allocate_operation_id(), RevisionId(1), Generation(2), 3);
        let payload = RetainedNumberJob { input: None, bytes: [0; 8], cursor: 0, output: None, closing: false };
        let spec = ToolOperationSpec::new("number", "retained", "test.retained-number.v1", payload, operation);
        let mut dispatch = match bus.dispatch_wire_retained_with_spec(admission, input, None, spec) {
            Ok(dispatch) => dispatch,
            Err(_) => panic!("production retained payload dispatch was rejected"),
        };
        let mut sequence = 0;
        for _ in 0..8 {
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
            assert!(matches!(dispatch.job.step(&mut context), StepOutcome::CheckpointReady(_)));
        }
        dispatch.job.begin_close();
        while !dispatch.job.terminal_is_empty() {
            let _ = dispatch.job.close_step(1, 8);
        }
    }

    #[test]
    fn retained_wire_admission_rejects_plus_one_and_returns_the_page_owner_on_saturation() {
        let bus = ActionBus::new();
        bus.register(RetainedNumberFactory { keys: vec![ToolFactoryKey::new("number", "retained")] }).unwrap();
        assert!(matches!(bus.begin_exact_wire("number", "retained", "test.retained-number.v1", 9), Err(ToolDispatchError::RawWireLimit { actual: 9, maximum: 8, .. })));
        let (_, mut input) = bus.begin_exact_wire("number", "retained", "test.retained-number.v1", 8).unwrap();
        input.admit_page(ToolWirePage::try_copy_from(&[0; 8]).unwrap()).unwrap();
        let rejected = input.admit_page(ToolWirePage::try_copy_from(&[1]).unwrap()).expect_err("plus-one page owner must be returned");
        assert_eq!(rejected.1.as_slice(), &[1]);
    }

    #[test]
    fn maximum_extent_owner_exists_before_incremental_encoding_and_seals_to_its_exact_prefix() {
        let bus = ActionBus::new();
        bus.register(RetainedNumberFactory { keys: vec![ToolFactoryKey::new("number", "retained")] }).unwrap();
        let (admission, mut input) = bus.begin_exact_wire("number", "retained", "test.retained-number.v1", 8).unwrap();
        input.admit_page(ToolWirePage::try_copy_from(&42u32.to_le_bytes()).unwrap()).unwrap();
        input.seal_admitted_prefix().unwrap();
        assert_eq!(input.declared_bytes(), 4);
        assert_eq!(input.page(0), Some(42u32.to_le_bytes().as_slice()));
        let operation = Operation::new(allocate_operation_id(), RevisionId(1), Generation(2), 3);
        let rejected = match bus.dispatch_wire_retained(admission, input, None, operation) {
            Ok(_) => panic!("factory owns the exact eight-byte decoder and must reject a truthful four-byte prefix"),
            Err(rejected) => rejected,
        };
        assert_eq!(rejected.input.declared_bytes(), 4);
    }
}
//#endregion 🎯️ToolJobBus
