//! 🔌️ Shared wasm extension glue for flow modules.

use neural_engine::{inject_channel_defaults, ColdOwner, Dictionary, Registry};

/// 🧮️ Weighs the allocator events that the paid-receipt laws of this crate's own test binary compare with their receipts.
#[cfg(test)]
#[global_allocator]
static EXTENSIONS_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_framework_value::{ErasedSnapshotRetirement,RetirementDemand,ValueError,ValueRefusalKind,retirement::RetireOwned,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

pub use semio_framework_os_flow_operators::extensions::schema::*;

#[path = "🔎️inference-dependency-and-evaluate-json/🦀️.rs"]
mod inference_dependency_and_evaluate_json;
pub use inference_dependency_and_evaluate_json::{
    evaluate_function_json, evaluate_json, flow_inference_dependency_json,
};

#[path = "🎫️evaluate-request-admission-and-drive/🦀️.rs"]
mod evaluate_request_admission_and_drive;
pub use evaluate_request_admission_and_drive::{
    evaluate_step_envelope_cold, evaluation_remaining_grant, EvaluateRequest,
    EvaluationContinuationIdentity, EvaluationExternalResult, EvaluationRequestAdmission,
    EVALUATE_ENVELOPE_SCHEMA, EVALUATE_STEP_BUDGET, EVALUATE_STEP_WALL_MICROS,
};
use evaluate_request_admission_and_drive::{
    evaluation_admit_original_request, evaluation_drive_original, COLD_EVALUATION_IDENTITY_POLICY,
};

#[path = "🧯️reply-failure-and-child-demands/🦀️.rs"]
mod reply_failure_and_child_demands;
pub use reply_failure_and_child_demands::{
    evaluation_admit_original, evaluation_child_close, evaluation_child_demands,
    evaluation_owned_birth, EvaluationFailure, EvaluationReply, EvaluationWire,
};
use reply_failure_and_child_demands::evaluation_value_demands;

#[path = "🧰️input-preparation-and-envelope-source/🦀️.rs"]
mod input_preparation_and_envelope_source;
pub use input_preparation_and_envelope_source::{EvaluationInputPreparation, EvaluationStep};
use input_preparation_and_envelope_source::{EvaluationEnvelopeSource, EVALUATION_JOB_CAPACITY};

#[path = "🗃️evaluation-output-and-retained-evaluation/🦀️.rs"]
mod evaluation_output_and_retained_evaluation;
pub use evaluation_output_and_retained_evaluation::{EvaluationOutput, RetainedEvaluation};

#[path = "⏱️evaluation-jobs-and-cancellation/🦀️.rs"]
mod evaluation_jobs_and_cancellation;
use evaluation_jobs_and_cancellation::EvaluationJobRegistry;
pub use evaluation_jobs_and_cancellation::{
    cancel_all_evaluations, cancel_evaluation, cancel_inference_evaluation,
    evaluation_dependency_identity, evaluation_jobs, evaluation_progress, evaluation_retire_cold,
    evaluation_retirement_demands, evaluation_retirement_pending,
    retire_cancelled_evaluations_close_step,
};

#[path = "🧩️extension-resources-topic-and-command/🦀️.rs"]
mod extension_resources_topic_and_command;
pub use extension_resources_topic_and_command::{
    command_json, flow_extension_topic_contribution, ExtensionEvaluationResources,
};

// #region 🔖️Evaluate

/// 🔀️ Decodes one original extension evaluation request with its mandatory retained policy.
/// ⏱️ Every request key retains the same original Registry frontier across round trips.
/// 🧾️ Each round trip debits actual physical effects once before reporting progress or completion.
#[path="📥️request/🦀️.rs"]
pub mod request;
pub use request::{EvaluationInvokeCursor,EvaluationRequestContextInput,evaluate_invoke_json};

#[path="📨️result/🦀️.rs"] pub mod external_completion;
use external_completion::EvaluationExternalCompletion;

// #endregion 🔖️Evaluate

// #region ⏱️EvaluationJobs

#[path="⚠️failure/🦀️.rs"] mod original_failure;

// #endregion ⏱️EvaluationJobs
