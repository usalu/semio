//! 🫧️ Exact transient and interaction publication owners and the ephemeral one-item publication they share.

use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory, ValueError, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value_snapshot_read::*;
use semio_framework_io_control::NativeSnapshotDecodeOwner;
use semio_framework_os_store_read::ArtifactStoreDisplacedSnapshotRetirement;
use semio_framework_value::factory::FactorySharedRetirement as ReturnedSnapshotReadRetirement;
use semio_framework_value::retained_clone::RetainedCloneBirthDemand;
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_os_store_one_item::*;
use protocol::{Mutation, MutationDiff, apply_operation};
use semio_framework_value::{FromValue, ToValue};
use std::{collections::BTreeMap, marker::PhantomData, sync::Arc, mem::{size_of, size_of_val}};

//#region 🔖️EphemeralLanes
#[path = "📢️publication/🧩️preparation/🦀️.rs"]
pub mod ephemeral_preparation;
#[path = "📢️publication/🔁️transfer/🦀️.rs"]
pub mod ephemeral_transfer;
#[path = "📤️ephemeral-one-item-publication/🦀️.rs"]
mod ephemeral_one_item_publication;
pub use ephemeral_one_item_publication::{
    ArtifactEphemeralBaseOwner, ArtifactEphemeralBaseRead,
    ArtifactEphemeralOneItemAdmissionRejected, ArtifactEphemeralOneItemPreparation,
    ArtifactEphemeralOneItemPreparationFactory, ArtifactEphemeralOneItemPreparationRequest,
    ArtifactEphemeralOneItemPrepared, ArtifactEphemeralOneItemPublication,
};

#[path = "🌫️transient-and-interaction-stores/🦀️.rs"]
mod transient_and_interaction_stores;
pub use transient_and_interaction_stores::{
    InteractionStore, TransientStore, TransientStoreRetirement,
};

//#endregion 🔖️EphemeralLanes

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER:semio_framework_trace_memory_testing::RequestedAllocator=semio_framework_trace_memory_testing::RequestedAllocator;
