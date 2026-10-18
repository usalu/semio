//! 💧️ Retained typed hydration of one persisted Pack and SPR history into an exact document owner.

use semio_framework_value::ErasedSnapshotRetirement;
use semio_framework_os_store_member_open::MemberOpenDiagnostic;
use semio_framework_os_store_text_io::text_format::{conflict_from_history_conflict, mutation_meta_from_history_op_meta};
use {semio_framework_artifact_reference::ArtifactRef};
use pack::snapshot::ArtifactPack;
use semio_framework_os_store_space_schema::ArtifactEnvelope;
use semio_framework_os_store_engine::lifted_store::ArtifactStore;
use semio_framework_os_store_initialization_runtime::lifted_store::ArtifactStoreInitializationRuntime;
use semio_framework_os_store_engine_owners::DocumentStoreOwners;
use semio_framework_os_store_replay::{EditReplay, ReplayStep};
use semio_framework_os_store_envelope::owner::OwnerRef;
use semio_framework_os_vcs::CompositionPin;
use semio_framework_os_spr::{Edit, Mutation};
use semio_framework_value::{FromValue, ToValue};
use semio_framework_os_pack::{OpBinary, OpText};
use semio_framework_job::{Generation, OperationId, StepContext};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::mem::ManuallyDrop;

#[path = "🧬️hydration-types-constructors-and-drive/🦀️.rs"]
mod hydration_types_constructors_and_drive;
pub use hydration_types_constructors_and_drive::{
    drive_hydration_retirement, drive_hydration_runtime, hydration_fold_byte_grant,
    PersistedDocumentHydrationOutput, PersistedDocumentHydrationProgress,
    PersistedDocumentHydrationStep, PersistedDocumentHydrationTarget,
    PersistedDocumentStoreHydrationStep, Phase, RetainedPersistedDocumentHydration,
};
use hydration_types_constructors_and_drive::{
    hydration_child_grant, owned_frame_funded, HydratedReplayEdits,
};

#[path = "🚶️hydration-step/🦀️.rs"]
mod hydration_step;

#[path = "🧹️hydration-retirement-drop-and-completion/🦀️.rs"]
mod hydration_retirement_drop_and_completion;
