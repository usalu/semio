//! 🪆️ Child members and child content: member keys and paths, the member registry, owned document ingress and child content views.
use semio_framework_value::ErasedSnapshotRetirement as _;
extern crate semio_framework_os_kernel as store;
use std::mem::ManuallyDrop;
use semio_framework_plugin_decl_views::child_view::CHILD_CONTENT_SLOTS;
use semio_framework_plugin_decl_views::child_view::MemberKey;
use semio_framework_plugin_decl_views::child_view::MemberKeyRef;
use semio_framework_plugin_decl_views::child_view::ChildContentEntry;
use semio_framework_plugin_decl_views::child_view::ChildContentPage;
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_plugin_decl_views::child_view::ChildContentTake;
use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
use semio_framework_plugin_decl_views::child_view::ChildContentOwners;
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin};
/// 🚪️ `io`'s `ArtifactRef`/`ArtifactKindId` vocabulary is not glob-re-exported at the
/// `semio-framework-os-kernel` crate root (deliberate — see that crate's own glue.rs comment on
/// the `io` mount), so it is named through the `store::io::` path everywhere in this file,
/// exactly like the sibling `🎞️gif` migration leaf (`semio_framework_artifact_reference::ArtifactDialect`) already does.
use semio_framework_artifact_reference::ArtifactRef;
use store::{MemberFactory, SpaceMember};
/// 🏅️🪆️🎯️ Standards/subsets dialect vocabulary (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase
/// 2). Defined in `semio_framework` so plugins and the OS product
/// share one definition without an inverted dependency; re-exported here verbatim.
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::controlled::ControlledRetirement;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_app::ArtifactApp;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use semio_framework_plugin_ownership::bounded_ownership::ArtifactFixedRegistry;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;

#[path = "🧭️member-keys-and-paths/🦀️.rs"]
mod member_keys_and_paths;
pub use member_keys_and_paths::{MemberPath, MemberStep, OWNED_DOCUMENT_MEMBER_PREPARATION_BYTES};

#[path = "🗂️member-registry/🦀️.rs"]
mod member_registry;
pub use member_registry::{ChildMemberAdmission, ChildMemberEntry, ChildMemberRegistrationError, ChildMemberRegistry};

#[path = "📥️document-member-ingress/🦀️.rs"]
mod document_member_ingress;
pub use document_member_ingress::{OWNED_DOCUMENT_INGRESS_PAGE_SLOTS, OwnedDocumentMemberIngress, OwnedDocumentMemberIngressRegistry};

#[path = "🔭️closure-source-and-content-owners/🦀️.rs"]
mod closure_source_and_content_owners;
pub use closure_source_and_content_owners::{ChildContentBorrowedOwners, DocumentClosureSourceView, member_is_retiring, original_content_step_owners};
use closure_source_and_content_owners::{child_snapshot_owner, child_snapshot_owner_ref};

#[path = "♻️child-retirement-and-return-pump/🦀️.rs"]
mod child_retirement_and_return_pump;
pub use child_retirement_and_return_pump::{ChildContentMemberMetadata, ChildContentRetirement, ChildMemberMetadata, ChildMemberRetirement, ChildStoreAdmission, SnapshotReadReturnPump};
#[cfg(any(test, feature = "artifact-app-testing"))]
pub use child_retirement_and_return_pump::ChildAdmissionTestState;
