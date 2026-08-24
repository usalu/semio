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

    /// 🛡️ Admits an exact owner/tool/schema wire envelope before any caller-specific decoder runs.
    pub fn admit_exact_wire(&self, controller_id: impl Into<String>, tool_id: impl Into<String>, schema_id: impl Into<String>, payload: &[u8]) -> Result<ToolWireAdmission, ToolDispatchError> {
        let controller_id = controller_id.into();
        let tool_id = tool_id.into();
        let schema_id = schema_id.into();
        let key = ToolFactoryKey::new(controller_id.clone(), tool_id.clone());
        let inner = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let contract = *inner.contract_by_key.get(&key).ok_or_else(|| ToolDispatchError::UnknownController { controller_id: format!("{controller_id}/{tool_id}") })?;
        if payload.len() > contract.max_raw_wire_bytes {
            return Err(ToolDispatchError::RawWireLimit { controller_id, tool_id, actual: payload.len(), maximum: contract.max_raw_wire_bytes });
        }
        let (factory_type_id, factory_type_name, expected_schema) = inner.factory_identity_by_key.get(&key).expect("factory identity is registered atomically with its key");
        if expected_schema != &schema_id {
            return Err(ToolDispatchError::Factory { controller_id, tool_id, detail: format!("expected payload schema '{expected_schema}', got '{schema_id}'") });
        }
        Ok(ToolWireAdmission { key, factory_type_id: *factory_type_id, factory_type_name, schema_id, contract })
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
    args.map(|value| dsl::to_dsl_value(&value).unwrap_or(DslValue::Null))
}

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
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
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
}
//#endregion 🎯️ToolJobBus
