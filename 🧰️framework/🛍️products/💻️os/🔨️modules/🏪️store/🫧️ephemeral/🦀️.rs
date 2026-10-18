//! 🫧️ Exact presence, transient and interaction publication owners.

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

#[path = "../👥️presence/🦀️.rs"]
pub mod presence;
use presence::{LaneItemReceipt, PRESENCE_PEER_ID_BYTES, PRESENCE_PEER_SLOTS, PresencePeersCapacityError, PresencePeersRetiredEntries, PresencePeersRetirement, PresencePeersRoot};
use presence::{PresencePeerEntry, PresencePeerRootCustody, presence_arc_bytes};

#[path = "📤️ephemeral-one-item-publication/🦀️.rs"]
mod ephemeral_one_item_publication;
pub use ephemeral_one_item_publication::{
    ArtifactEphemeralBaseOwner, ArtifactEphemeralBaseRead,
    ArtifactEphemeralOneItemAdmissionRejected, ArtifactEphemeralOneItemPreparation,
    ArtifactEphemeralOneItemPreparationFactory, ArtifactEphemeralOneItemPreparationRequest,
    ArtifactEphemeralOneItemPrepared, ArtifactEphemeralOneItemPublication,
};

#[path = "👥️presence-store-and-peers-publication/🦀️.rs"]
mod presence_store_and_peers_publication;
pub use presence_store_and_peers_publication::{
    PresencePeersCommit, PresencePeersPublication, PresenceStore,
};

#[path = "🌫️transient-and-interaction-stores/🦀️.rs"]
mod transient_and_interaction_stores;
pub use transient_and_interaction_stores::{
    InteractionStore, TransientStore, TransientStoreRetirement,
};

#[path = "../👥️presence/🚪️io/📬️publication/🦀️.rs"]
mod presence_publication_io;

#[path="../👥️presence/🔗️read/🦀️.rs"]
mod presence_peer_read;
pub use presence_peer_read::PresencePeersRead;

#[path = "../👥️presence/🚫️rejection/🦀️.rs"]
mod presence_peer_rejection;
pub use presence_peer_rejection::PresencePeerAdmissionRejected;

pub use semio_framework_os_presence_native_snapshot::ArtifactPresenceSnapshot;

#[cfg(test)]
use semio_framework_value_snapshot_read::presence_test_retirement;

#[path = "../👥️presence/♻️retirement/🦀️.rs"]
mod presence_retirement;
pub use presence_retirement::PresenceStoreRetirement;

//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER:semio_framework_trace_memory_testing::RequestedAllocator=semio_framework_trace_memory_testing::RequestedAllocator;
