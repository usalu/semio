//! 📥️ Items of `store` that belong to `os-store-space-member` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🦀️.rs`.

// 🚫️async: R7 — `async fn` in a public trait warns because auto trait bounds (e.g. `Send`) cannot
// be named on the method. Answered structurally per R3: every former `dyn` seam in this crate is a
// concrete enum, so `Send` is derived at each spawn site from the concrete type, never from a bound
// on the trait method's returned future. This crate is guest-reachable, so its futures are
// deliberately `?Send` — do not "fix" this warning by adding `-> impl Future<..> + Send` (contradicts
// R3) or by making a trait method sync (contradicts O1/R1).
#![allow(async_fn_in_trait)]

use semio_framework_composition_admission::{ChildRestoreProjection, ChildRestoreProjectionError};
use semio_framework_os_store_text_io::ArtifactStoreSnapshot as _;
use semio_framework_value::ErasedSnapshotRetirement;
use semio_framework_value::retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use crate::factory::MemberOpenDeclaration;
use semio_framework_os_store_member_open::{MemberOpenDiagnostic, MemberOpenOperation, MemberOpenRequest};
use semio_framework_os_store_member_open::source_admission::MemberOpenSourceAdmission;
use crate::lifted_operation::UnsupportedMemberFactoryOpen;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::{ActorId, HybridLogicalTimestamp};
use semio_framework_os_spr::{ColdMutation, Mutation, OpBinary, OpText};
use std::sync::Arc;
use semio_framework_os_store_read::ErasedSnapshotRead;
use semio_framework_value_snapshot_read::SnapshotReadRegistryAliasRetirement;
use semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use semio_framework_os_vcs::{Author, VcsError};
use semio_framework_os_vcs::{Checkpoint, apply_mutation};
use semio_framework_os_store_space_schema::{ArtifactCommand, HistoryLane};
use semio_framework_os_store_envelope::owner::OwnerRef;
use semio_framework_os_store_link_schema::{ArtifactLink,LinkPin};
use semio_framework_os_store_child::ChildRef;
//#region 🔖️EphemeralLanes

use semio_framework_os_store_ephemeral::*;
use semio_framework_os_store_text_io::{decode_document_pack_bytes, encode_document_pack_bytes};
use pack::snapshot::ArtifactPack;
use semio_framework_os_store_text_io::ArtifactStoreSnapshot;
use crate::lifted_document_materialize::{edit_ids_for_changes, materialize_document_snapshot};
use semio_framework_os_store_engine_core::lifted_document_materialize::uncommitted_edit_ids;
use semio_framework_os_store_text_io::text_format::print_document_pack;
use semio_framework_os_store_ledger::{ArtifactStoreHistoryCommitReservation, retire_replayed_projection};
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::{ArtifactStoreBatchAdmissionRejected, ErasedMemberStoreOneItemPublication};
use semio_framework_os_store_one_item_publication::{ArtifactStoreOneItemAdvance, ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPublicationPhase};
use semio_framework_os_store_one_item_publication::lifted_publication::{MemberStoreOneItemWire, MemberStoreOneItemWireRequest};
use semio_framework_os_store_one_item_publication::{ArtifactStoreBatchSourceOf, ArtifactTypedBatchItemAuthority};
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::MemberStoreOneItemPublication;
use semio_framework_os_store_one_item::{ARTIFACT_STORE_ONE_ITEM_ID_BYTES, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep};
use semio_framework_os_store_one_item_batch::member_owned_batch::MemberStoreOwnedBatchRequest;
use semio_framework_os_store_replay::PairMerge;
//#region 🔖️Backbone
//#region 🔖️Backbone
use semio_framework_os_store_backbone as backbone;
//#endregion 🔖️Backbone

//#region 🔖️BlobStore
//#region 🔖️BlobStore
use semio_framework_os_store_blob_identity::BlobRef;
use semio_framework_os_store_engine_core::lifted_store::CommandReceipt;
use semio_framework_os_store_engine::lifted_store::ArtifactStore;

#[path = "🔗️member-link-resolution-and-blob-store/🦀️.rs"]
mod member_link_resolution_and_blob_store;
pub use member_link_resolution_and_blob_store::{
    content_id, ArtifactRefs, BlobStore, LinkResolver, LinkState, MemberDirectory,
    MemberLinkResolver, NoBlobStore,
};

#[path = "👥️space-member-trait/🦀️.rs"]
mod space_member_trait;
pub use space_member_trait::{SpaceMember,MemberIdentityRef};

#[path = "🏪️space-member-for-artifact-store/🦀️.rs"]
mod space_member_for_artifact_store;

#[path = "🏭️member-factory-visitors-and-no-members/🦀️.rs"]
mod member_factory_visitors_and_no_members;
#[path = "../../../🪪️member-facets/🧊️reference/🦀️.rs"]
mod reference_space_member;
pub use reference_space_member::ReferenceSpaceMember;
pub use member_factory_visitors_and_no_members::{
    MemberFactory, MemberStoreVisitor, MemberStoreVisitorMut, MemberVisit, NoMembers,
};

//#endregion 🔖️BlobStore
//#endregion 🔖️BlobStore
//#endregion 🔖️Backbone
//#endregion 🔖️EphemeralLanes

#[path = "../../../🪪️member-facets/🧊️reference/🦀️.rs"]
mod reference;
pub use reference::ReferenceSpaceMember;

pub use semio_framework_value_native_owned::native_decoding::NativeDecodeControl as MemberNativeDecodeControl;

#[path="🎮️native-vcs/🦀️.rs"]mod native_vcs;
pub use native_vcs::MemberNativeVcsCommand;
