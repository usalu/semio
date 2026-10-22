//! 👷️ The tool job contract an app declares: bounded proofs, factories and registrations, instance operation owners, owned and reserved job requests, completions and faults.
use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
use semio_framework_diagnostic::FaultFrom;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_plugin_decl_views::gesture_contract::GestureSlot;
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwnerHandle;
use semio_framework_plugin_decl_views::gesture_contract::GestureCapture;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunView;
use semio_framework_job::JobOutcomeBorrow;
use semio_framework_job::JobOutcomeDescriptor;
use semio_framework_job::JobOutcomeKind;
use semio_framework_job::JobOutcomeView;
use semio_framework_job_payload::JobPayloadStream;
use semio_framework_job_payload::RetainedPayloadBuilder;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use std::mem::ManuallyDrop;
use semio_framework_job::StepContext;
use semio_framework_plugin_window_config::WindowConfigSnapshot;
use semio_framework_plugin_window_transient::window_transient::WindowTransientSnapshot;
use semio_framework_value::DslValue;
use semio_framework::Fault;
use semio_framework::FaultCode;
use semio_framework::FaultOrigin;
use semio_framework::ViewModel;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::Terminology;
use semio_framework::Media;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneStep;
use crate::completion_fault::ArtifactCompletionFault;
use store::SnapshotReadProjection as ArtifactContextRoot;
use crate::owned_context::ArtifactOwnedContextHandle;
use semio_framework_async::PublicationClaimHandle;
use crate::ArtifactApp;
use crate::app_emission::EphemeralEmit;
use semio_framework_plugin_decl_views::InteractionHoverState;
use semio_framework_plugin_decl_views::PresencePeersView;
use semio_framework_plugin_decl_views::PresenceView;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure;
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_plugin_decl_views::history_view::HistoryView;
use semio_framework_plugin_decl_views::media_contract::ArtifactDownloadOutput;
pub use semio_framework_plugin_decl_views::media_contract::ArtifactOutputChunks;

use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal;
use semio_framework_plugin_decl_emission::ArtifactMutationOutcome;
use semio_framework_plugin_operation::ToolCancellationLease;
use semio_framework_job::InteractiveJobCloseStep;

#[path = "🧾️bounded-proof-and-factory-registry/🦀️.rs"]
mod bounded_proof_and_factory_registry;
pub use bounded_proof_and_factory_registry::*;

#[path = "🩺️typed-operation-faults-and-raw-input/🦀️.rs"]
mod typed_operation_faults_and_raw_input;
pub use typed_operation_faults_and_raw_input::*;

#[path = "🎯️completion-and-owned-context/🦀️.rs"]
mod completion_and_owned_context;
pub use completion_and_owned_context::*;

#[path = "🧵️reserved-tool-job/🦀️.rs"]
mod reserved_tool_job;
pub use reserved_tool_job::*;

#[path = "🔎️reserved-outcome-witnesses/🦀️.rs"]
mod reserved_outcome_witnesses;
pub use reserved_outcome_witnesses::*;

include!("🎟️birth/🦀️.rs");
#[cfg(feature="reserved-producer-testing")]
pub mod original_reserved_producer_laws{include!("🎟️birth/🧪️tests/🦀️.rs");}
