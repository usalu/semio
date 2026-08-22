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
use std::any::Any;
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
}

pub trait ToolJobFactory: Send + 'static {
    type Payload: Send + 'static;
    type Job: InteractiveJob + 'static;

    fn keys(&self) -> &[ToolFactoryKey];
    fn payload_schema_id(&self) -> &str;
    fn classification(&self) -> InteractiveJobClassification;
    fn create_job(&mut self, operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError>;
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
    NonInteractiveClassification { key: ToolFactoryKey, classification: InteractiveJobClassification },
}

impl Display for ToolRegistrationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyFactory => formatter.write_str("tool factory must own at least one controller/tool key"),
            Self::DuplicateKey { key } => write!(formatter, "tool factory key '{}/{}' is already registered", key.controller_id, key.tool_id),
            Self::NonInteractiveClassification { key, classification } => write!(formatter, "tool factory '{}/{}' is not UI-reachable: {classification:?}", key.controller_id, key.tool_id),
        }
    }
}

impl Error for ToolRegistrationError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToolDispatchError {
    UnknownController { controller_id: String },
    Factory { controller_id: String, tool_id: String, detail: String },
}

impl Display for ToolDispatchError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownController { controller_id } => write!(formatter, "unknown tool controller '{controller_id}'"),
            Self::Factory { controller_id, tool_id, detail } => write!(formatter, "tool factory '{controller_id}' rejected '{tool_id}': {detail}"),
        }
    }
}

impl Error for ToolDispatchError {}
//#endregion 🚫️Rejection

//#region 🚌️Bus
trait ErasedToolJobFactory: Send {
    fn create_job(&mut self, spec: &mut ToolOperationSpec) -> Result<ErasedToolJob, ToolJobFactoryError>;
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
}

struct ActionBusInner {
    factory_by_key: HashMap<ToolFactoryKey, usize>,
    factory_identity_by_key: HashMap<ToolFactoryKey, (&'static str, String)>,
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
        Self { inner: Arc::new(Mutex::new(ActionBusInner { factory_by_key: HashMap::new(), factory_identity_by_key: HashMap::new(), factories: Vec::new(), dispatch_count: 0 })) }
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
            inner.factory_identity_by_key.insert(key.clone(), (std::any::type_name::<F>(), factory.payload_schema_id().to_string()));
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
        let identity = (std::any::type_name::<F>(), factory.payload_schema_id());
        let mut inner = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for key in keys {
            if let Some((registered_type, registered_schema)) = inner.factory_identity_by_key.get(key) {
                if (*registered_type, registered_schema.as_str()) != identity {
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
            inner.factory_identity_by_key.insert(key, (identity.0, identity.1.to_string()));
        }
        inner.factories.push(Box::new(ToolJobFactoryAdapter { factory }));
        Ok(())
    }

    pub fn contains(&self, key: &ToolFactoryKey) -> bool {
        self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).factory_by_key.contains_key(key)
    }

    pub fn keys(&self) -> Vec<ToolFactoryKey> {
        self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).factory_by_key.keys().cloned().collect()
    }

    pub fn dispatch_count(&self) -> u64 {
        self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).dispatch_count
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
        output: Vec<u8>,
    }

    impl InteractiveJob for ImmediateJob {
        fn step(&mut self, _cx: &mut StepContext<'_>) -> StepOutcome {
            StepOutcome::Complete(CommitCandidate { state: Vec::new(), output: self.output.clone() })
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

        fn create_job(&mut self, _operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
            Ok(ImmediateJob { output: format!("{payload}:ok").into_bytes() })
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

        fn create_job(&mut self, _operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
            Ok(ImmediateJob { output: payload.to_le_bytes().to_vec() })
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
}
//#endregion 🎯️ToolJobBus
