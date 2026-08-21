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
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

//#region 🧬️Contract
#[derive(Clone, Debug, PartialEq)]
pub struct ToolOperationSpec {
    pub controller_id: String,
    pub tool_id: String,
    pub args: Option<DslValue>,
    pub operation: Operation,
}

impl ToolOperationSpec {
    pub fn new(controller_id: impl Into<String>, tool_id: impl Into<String>, args: Option<DslValue>, operation: Operation) -> Self {
        Self { controller_id: controller_id.into(), tool_id: tool_id.into(), args, operation }
    }
}

pub struct ToolJobDispatch<J: InteractiveJob> {
    pub spec: ToolOperationSpec,
    pub job: J,
}

pub trait ToolJobFactory: Send {
    type Job: InteractiveJob;

    fn id(&self) -> &str;
    fn classification(&self) -> InteractiveJobClassification;
    fn create_job(&mut self, spec: &ToolOperationSpec) -> Result<Self::Job, ToolJobFactoryError>;
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
    NonInteractiveClassification { controller_id: String, classification: InteractiveJobClassification },
}

impl Display for ToolRegistrationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonInteractiveClassification { controller_id, classification } => {
                write!(formatter, "tool factory '{controller_id}' is not UI-reachable: {classification:?}")
            }
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
pub struct ActionBus<F: ToolJobFactory> {
    factories: HashMap<String, F>,
}

pub enum NoInteractiveJobs {}

impl InteractiveJob for NoInteractiveJobs {
    fn step(&mut self, _cx: &mut StepContext<'_>) -> StepOutcome {
        match *self {}
    }
}

pub enum NoToolJobFactories {}

impl ToolJobFactory for NoToolJobFactories {
    type Job = NoInteractiveJobs;

    fn id(&self) -> &str {
        match *self {}
    }

    fn classification(&self) -> InteractiveJobClassification {
        match *self {}
    }

    fn create_job(&mut self, _spec: &ToolOperationSpec) -> Result<Self::Job, ToolJobFactoryError> {
        match *self {}
    }
}

impl<F: ToolJobFactory> Default for ActionBus<F> {
    fn default() -> Self {
        Self::new()
    }
}

impl<F: ToolJobFactory> ActionBus<F> {
    pub fn new() -> Self {
        Self { factories: HashMap::new() }
    }

    pub fn register(&mut self, factory: F) -> Result<(), ToolRegistrationError> {
        let controller_id = factory.id().to_string();
        let classification = factory.classification();
        if classification != InteractiveJobClassification::Migrated {
            return Err(ToolRegistrationError::NonInteractiveClassification { controller_id, classification });
        }
        self.factories.insert(controller_id, factory);
        Ok(())
    }

    pub fn unregister(&mut self, controller_id: &str) -> bool {
        self.factories.remove(controller_id).is_some()
    }

    pub fn dispatch(&mut self, spec: ToolOperationSpec) -> Result<ToolJobDispatch<F::Job>, ToolDispatchError> {
        let controller_id = spec.controller_id.clone();
        let tool_id = spec.tool_id.clone();
        let factory = self.factories.get_mut(&controller_id).ok_or_else(|| ToolDispatchError::UnknownController { controller_id: controller_id.clone() })?;
        let job = factory.create_job(&spec).map_err(|error| ToolDispatchError::Factory { controller_id, tool_id, detail: error.detail })?;
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
        id: String,
        classification: InteractiveJobClassification,
    }

    impl ToolJobFactory for EchoFactory {
        type Job = ImmediateJob;

        fn id(&self) -> &str {
            &self.id
        }

        fn classification(&self) -> InteractiveJobClassification {
            self.classification
        }

        fn create_job(&mut self, spec: &ToolOperationSpec) -> Result<Self::Job, ToolJobFactoryError> {
            Ok(ImmediateJob { output: format!("{}:ok", spec.tool_id).into_bytes() })
        }
    }

    fn operation_spec(controller_id: &str, tool_id: &str) -> ToolOperationSpec {
        ToolOperationSpec::new(controller_id, tool_id, None, Operation::new(allocate_operation_id(), RevisionId(7), Generation(3), 11))
    }

    #[test]
    fn dispatch_returns_a_resumable_job_and_preserves_operation_identity() {
        let mut bus = ActionBus::new();
        bus.register(EchoFactory { id: "app".into(), classification: InteractiveJobClassification::Migrated }).unwrap();
        let dispatch = bus.dispatch(operation_spec("app", "ping")).unwrap();
        assert_eq!(dispatch.spec.operation.base_revision, RevisionId(7));
        assert_eq!(dispatch.spec.operation.generation, Generation(3));
    }

    #[test]
    fn registration_rejects_every_non_migrated_factory() {
        for classification in [InteractiveJobClassification::Unclassified, InteractiveJobClassification::BatchOnlyPendingRewrite, InteractiveJobClassification::ForbiddenFromUi, InteractiveJobClassification::Deleted] {
            let mut bus = ActionBus::new();
            assert!(matches!(
                bus.register(EchoFactory { id: "app".into(), classification }),
                Err(ToolRegistrationError::NonInteractiveClassification { classification: rejected, .. }) if rejected == classification
            ));
        }
    }

    #[test]
    fn unknown_controller_is_an_explicit_dispatch_error() {
        let mut bus: ActionBus<EchoFactory> = ActionBus::new();
        assert!(matches!(bus.dispatch(operation_spec("missing", "ping")), Err(ToolDispatchError::UnknownController { .. })));
    }
}
//#endregion 🎯️ToolJobBus
