extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
use semio_framework_job_publication::{RetainedJobPublication, JobPublicationKind};
use semio_framework_plugin_decl_views::{AppOperationContext, InteractionHoverState};
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobContext;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletion;
use semio_framework_plugin_decl_views::media_contract::ArtifactDownloadOutput;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_decl_app::app_emission::EphemeralEmit;
use semio_framework_plugin_decl_views::history_view::HistoryView;
use semio_framework::action_bus::RetainedToolWireInput;
use semio_framework::Fault;
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, StepContext};
use semio_framework_job_fault_publication::RetainedFaultPublication;
use std::sync::Arc;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{ValueError, ValueRefusalKind, RetirementDemand};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_plugin_decl_app::ArtifactOwnedContextHandle;
use std::mem::MaybeUninit;

#[doc(hidden)]
#[path="♻️metadata/🦀️.rs"]
pub mod metadata_retirement;
#[doc(hidden)]
#[path="🎟️admission/🦀️.rs"]
pub mod admission;
pub use crate::admission::ArtifactRetainedAdmissionRefusal;
#[doc(hidden)]
#[path="📸️checkpoint/👣️cursor/🦀️.rs"]
pub mod checkpoint_cursor;
pub use crate::checkpoint_cursor::ArtifactCommandCheckpointCursor;
#[path="🔗️read/🦀️.rs"]
mod snapshot_read_donor;
pub use crate::snapshot_read_donor::ArtifactCommandSnapshotRead;

#[path = "🔖️checkpoint-and-work-contract/🦀️.rs"]
mod checkpoint_and_work_contract;
pub use checkpoint_and_work_contract::{ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES, ArtifactCommandExtent, ArtifactCommandInputs, ArtifactCommandReducer, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedWorkCapacity, BoundedArtifactCommandWork};
use checkpoint_and_work_contract::{ARTIFACT_COMMAND_FAULT_DETAIL_MAXIMUM_BYTES, decode_artifact_command_checkpoint};
#[cfg(test)]
use checkpoint_and_work_contract::{ARTIFACT_COMMAND_CHECKPOINT_HEADER_BYTES, ArtifactCommandCheckpoint, encode_artifact_command_checkpoint};

#[path = "🧵️retained-command-job/🦀️.rs"]
mod retained_command_job;
pub use retained_command_job::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, reducer_fault_detail};
#[cfg(any(test, feature = "artifact-app-testing"))]
use retained_command_job::ArtifactRetainedCommandPhase;

#[cfg(any(test, feature = "artifact-app-testing"))]
#[path = "🧯️raw-allocation-close-oracle/🦀️.rs"]
mod raw_allocation_close_oracle;
#[cfg(any(test, feature = "artifact-app-testing"))]
pub use raw_allocation_close_oracle::test_raw_allocation_close;

#[doc(hidden)]
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub mod tests;

#[path = "🛬️lifted/🏛️reserved/🦀️.rs"]
pub mod reserved_route_capacity_lift;
#[path = "🔄️full-operation/🦀️.rs"]
pub mod typed_operation;
