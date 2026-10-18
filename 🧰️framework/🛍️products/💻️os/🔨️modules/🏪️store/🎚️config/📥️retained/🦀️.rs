//! 📥️ Bounded hydration of an exact persisted config history into a fresh config store.

use semio_framework_os_store_space_schema::ArtifactEnvelope;
use semio_framework_os_store_engine::lifted_store::ArtifactStore;
use semio_framework_os_store_initialization_runtime::lifted_store::ArtifactStoreInitializationRuntime;
use semio_framework_os_store_engine_owners::DocumentStoreOwners;
use semio_framework_os_spr::{Edit, Mutation};
use semio_framework_value::{ErasedSnapshotRetirement, FromValue, ToValue};
use semio_framework_os_store_text_io::text_format::mutation_meta_from_history_op_meta;
use semio_framework_os_spr::io::binary::OpBinary;
use semio_framework_os_spr::io::text::OpText;
use std::mem::ManuallyDrop;
use semio_framework_value::{ValueError, ValueRefusalKind, RetirementDemand, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};

#[path = "🧬️hydration-types-constructor-and-drive/🦀️.rs"]
mod hydration_types_constructor_and_drive;
pub use hydration_types_constructor_and_drive::{
    ConfigStoreHydrationDiagnostic, ConfigStoreHydrationProgress, ConfigStoreHydrationStep,
    RetainedConfigStoreHydration,
};
use hydration_types_constructor_and_drive::Phase;

#[path = "🚶️hydration-advance/🦀️.rs"]
mod hydration_advance;

#[path = "🧹️hydration-retirement-drop-and-completion/🦀️.rs"]
mod hydration_retirement_drop_and_completion;
