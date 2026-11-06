extern crate semio_os_kernel as store;
extern crate semio_os_kernel as protocol;
use semio_framework_job_publication::{RetainedJobPublication, JobPublicationKind};
use semio_plugin_decl_views::{AppOperationContext, InteractionHoverState};
use semio_plugin_decl_app::ArtifactApp;
use semio_plugin_decl_app::tool_contract::ArtifactOwnedToolJobContext;
use semio_plugin_decl_app::tool_contract::ArtifactToolCompletion;
use semio_plugin_decl_media::ArtifactDownloadOutput;
use semio_plugin_decl_emission::Emit;
use semio_plugin_decl_app::app_emission::EphemeralEmit;
use semio_plugin_decl_views::history_view::HistoryView;
use semio_framework::action_bus::RetainedToolWireInput;
use semio_framework::Fault;
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, StepContext};
use semio_framework_job_fault_publication::RetainedFaultPublication;
use std::sync::Arc;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{ValueError, ValueRefusalKind, RetirementDemand};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_plugin_decl_app::ArtifactOwnedContextHandle;
use std::mem::MaybeUninit;

#[doc(hidden)]
pub use semio_framework::action_bus as __action_bus;
#[doc(hidden)]
pub use semio_framework_job as __job;
#[doc(hidden)]
pub use semio_framework_value as __value;

#[doc(hidden)]
#[path="🎟️admission/🦀️.rs"]
pub mod admission;
pub use crate::admission::ArtifactRetainedAdmissionRefusal;
#[path="🏭️factory/🦀️.rs"]
pub mod factory;
#[path="🏭️producer/🦀️.rs"]
pub mod producer;
pub use producer::custody::ArtifactRetainedCommandCustody;
#[doc(hidden)]

#[path="📸️checkpoint/👣️cursor/🦀️.rs"]
pub mod checkpoint_cursor;
pub use crate::checkpoint_cursor::ArtifactCommandCheckpointCursor;
pub use semio_plugin_retained_work::{ArtifactCommandSnapshotRead,ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES,ArtifactCommandExtent,ArtifactCommandInputs,ArtifactCommandReducer,ArtifactCommandWork,ArtifactCommandWorkStep,ArtifactRetainedWorkCapacity,BoundedArtifactCommandWork};
use semio_plugin_retained_work::{ARTIFACT_COMMAND_FAULT_DETAIL_MAXIMUM_BYTES,decode_artifact_command_checkpoint};

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
