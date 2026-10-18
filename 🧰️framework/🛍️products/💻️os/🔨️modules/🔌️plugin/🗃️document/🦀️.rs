//! 🗃️ Document ingress: envelope decode and ingress, store replacement, composition pins and document archive loads.
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_os_kernel as pack;
use protocol::{OpBinary, OpText};
use semio_framework::kernel::ActorId;
use semio_framework::{Fault, FaultCode, FaultOrigin};
/// 🚪️ `io`'s `ArtifactRef`/`ArtifactKindId` vocabulary is not glob-re-exported at the
/// `semio-framework-os-kernel` crate root (deliberate — see that crate's own glue.rs comment on
/// the `io` mount), so it is named through the `store::io::` path everywhere in this file,
/// exactly like the sibling `🎞️gif` migration leaf (`semio_framework_artifact_reference::ArtifactDialect`) already does.
use semio_framework_artifact_reference::ArtifactRef;
use pack::snapshot::ArtifactPack;
use store::os_store::store_schema::ArtifactEnvelope;
use store::{ArtifactStore, MemberFactory, Mutation, SpaceMember};
use store::os_store::composition_coordinator::CompositionCoordinator;
/// 🏅️🪆️🎯️ Standards/subsets dialect vocabulary (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase
/// 2). Defined in `semio_framework` so plugins and the OS product
/// share one definition without an inverted dependency; re-exported here verbatim.
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use std::mem::ManuallyDrop;
use semio_framework_plugin_composition_emission::root::advance_original_child_root_retirement;
use semio_framework_plugin_composition_emission::root::original_child_root_retirement_demands;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_app::DOCUMENT_LOAD_FAILED_CODE;
use semio_framework_plugin_decl_app::document_load_fault;
use semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use semio_framework_plugin_ownership::bounded_ownership::ArtifactFixedRegistry;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_plugin_ownership::bounded_ownership::plugin_bytes_close;
use semio_framework_plugin_ownership::bounded_ownership::plugin_bytes_retirement_demand;
use semio_framework_plugin_ownership::bounded_ownership::plugin_turn_admitted;
use semio_framework_plugin_child::ChildContentBorrowedOwners;
use semio_framework_plugin_decl_views::child_view::ChildContentEntry;
use semio_framework_plugin_child::ChildContentRetirement;
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_plugin_child::ChildMemberEntry;
use semio_framework_plugin_child::ChildMemberRegistry;
use semio_framework_plugin_child::ChildMemberRetirement;
use semio_framework_plugin_child::DocumentClosureSourceView;
use semio_framework_plugin_decl_views::child_view::MemberKeyRef;
use semio_framework_plugin_child::OWNED_DOCUMENT_MEMBER_PREPARATION_BYTES;
use semio_framework_plugin_child::OwnedDocumentMemberIngress;
use semio_framework_plugin_child::OwnedDocumentMemberIngressRegistry;
use semio_framework_plugin_decl_views::history_view::HistoryCommandFilter;
use semio_framework_plugin_decl_views::history_view::HistoryView;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactStoreInitializationJob;

#[path = "📨️envelope-decode-and-ingress/🦀️.rs"]
mod envelope_decode_and_ingress;
pub use envelope_decode_and_ingress::{ActiveArtifactEnvelopeDecode, ActiveArtifactEnvelopeDecodeState, ActiveArtifactEnvelopeIngress, ArtifactEnvelopeDecodeOperationHandle, ArtifactEnvelopeDecodeOperationPoll, original_rejection_lifecycle};
use envelope_decode_and_ingress::original_worker_lifecycle;

#[path = "🏷️candidate-publication-and-composition-pins/🦀️.rs"]
mod candidate_publication_and_composition_pins;
pub use candidate_publication_and_composition_pins::{CompositionPinsRetirement, publish_boxed_document_store_candidate_if_authoritative, publish_document_store_candidate_if_authoritative};

#[path = "🧾️store-replacement-vocabulary/🦀️.rs"]
mod store_replacement_vocabulary;
pub use store_replacement_vocabulary::{ActiveArtifactStoreReplacement, ActiveArtifactStoreReplacementState, ArtifactStoreReplacementPublicationGuard, ArtifactStoreReplacementRefusal};
use store_replacement_vocabulary::{ArtifactStoreRefusalCode, ArtifactStoreReplacementLeg};

#[path = "🔁️store-replacement-admission-and-member-drive/🦀️.rs"]
mod store_replacement_admission_and_member_drive;

#[path = "🧰️store-replacement-initializer-and-retirement/🦀️.rs"]
mod store_replacement_initializer_and_retirement;
pub use store_replacement_initializer_and_retirement::{AppProjectionCache, ArtifactStoreReplacementAdmissionTarget};

#[path = "🗄️document-archive-load/🦀️.rs"]
mod document_archive_load;
pub use document_archive_load::{ActiveDocumentArchiveLoad, ActiveDocumentArchiveLoadPhase, ActiveDocumentArchiveLoadState, PendingDocumentArchiveMember};

pub use semio_framework_plugin_operation::original_catalog_constructor::{ArtifactStoreConstructorKind, ArtifactStoreConstructorPhase};

pub use semio_framework_plugin_operation::original_catalog_constructor::OriginalCatalogConstructor;

#[path = "../🎞️media/🦀️.rs"]
pub mod media_export;
#[path = "../🎞️media/⚠️turn/🦀️.rs"]
pub mod media_operation_refusal;
#[path = "🛬️lifted/🧩️app/🦀️.rs"]
pub mod lifted_app;
pub use lifted_app::*;
