//! ⚖️ App laws and the test modules of the app vocabulary.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;
/// 🪪️ Runs one test body with a local native authoring identity: `with_authoring_identity!(|identity| body)` binds `identity` as a mutable
/// [`EntityIdentityAuthority`](semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority) for `body`, an expression that may await.
#[cfg(any(test, feature = "artifact-app-testing"))]
#[macro_export]
macro_rules! with_authoring_identity {
    (|$identity:ident| $body:expr) => {{
        let mut __observer = |_: $crate::__value::native_encoding::NativeEncodeProgress| true;
        let __observer: &mut $crate::__kernel::os_vcs::io::binary::entity_identity::control::Observer<'_> = &mut __observer;
        let mut $identity = $crate::__kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1usize << 24, __observer).expect("test authoring identity admission");
        $body
    }};
}
#[doc(hidden)]
pub use semio_framework_os_kernel as __kernel;
#[doc(hidden)]
pub use semio_framework_value as __value;

pub use semio_framework_plugin_testing_caller::{OriginalFixtureCaller, OriginalFixtureScope};
use semio_framework_plugin_window_config::{WindowConfigOwner, WindowConfigOwnerRegistry};
use semio_framework_plugin_window_transient::window_transient::{WindowTransientOwner, WindowTransientOwnerRegistry};
use semio_framework_value::DslValue;
use protocol::{OpBinary, OpText};
use semio_framework::kernel::UiDirtyScope;
use semio_framework::manifest::ActionInvocation as ManifestActionInvocation;
use semio_framework::kernel::{ActorId, Effect, InvocationResult};
use semio_framework::{ActionKind, AppIo, ConfigSpec, Fault, FaultCode, FaultOrigin, MediaPortSpec, ViewModel, ViewWindowInstance, SET_ACTIVE_TOOL_ACTION_ID, SET_ACTIVE_UTILITY_ACTION_ID};
use semio_framework_ui_contract::BuiltNode;
/// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-flip`, 26/08/20): the choke point. Every
/// UI-bearing plugin in the fleet imports its UI vocabulary through THIS block, so flipping it
/// here redirects all 33 at once — `ui_wgpu` leaves the guest dependency graph entirely
/// (registrar-request: drop it from this crate's `Cargo.toml`, add `semio-framework-ui-contract`
/// and `semio-framework-ui-runtime` in its place). `Ui*Node`/`UiPresence`/`UiControlNode`/
/// `ActionDescriptor` have no drop-in replacement of the same name — see
/// `📓️recipe-plugin.md` in this ticket's folder for the variant-by-variant mechanical
/// translation (`UiNode::Text{..}` → `ui::text(..)`, `ActionDescriptor` → `ActionBinding`, …).
/// `ui::*` is the contract crate's own builder DSL (`ui::stack()`, `ui::button()`, …); the bare
/// glob brings every contract type (`Component`, `UiPatch`, `Label`, `ActionId`, …) into scope
/// too, since this module already used every one of the old wgpu-target types unqualified.
use semio_framework_ui_contract::*;
use semio_framework_ui_runtime::ComponentTree;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashMap};
/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: `AsyncTask`'s boxed `run` closure/future.
use std::future::Future;
use pack::snapshot::ArtifactPack;
use store::os_store::document_materialize::create_document_envelope;
use store::os_store::store_schema::ArtifactCommand;
use store::{ArtifactStore, ConfigStore, MemberFactory, Mutation, NoMembers, SpaceMember};
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;
use semio_framework_ui_layout::layout::ContextMenuItemSpec;
use semio_framework_ui_layout::layout::ContextMenuRequest;
use semio_framework_ui_layout::layout::WindowEngagement;
use semio_framework_ui_layout::layout::WindowMeasure;
/// 🎞️ The `Media`/`MediaPayload`/`MediaFingerprint`/`MediaError` value vocabulary backing
/// `ArtifactApp::{media_ports, export_media, import_media, media_fingerprint}` — re-exported so
/// implementers never need a direct `semio-framework-core` dependency just to satisfy this trait.
use semio_framework::{Media, MediaError, MediaFingerprint};
/// 🏅️🪆️🎯️ Standards/subsets dialect vocabulary (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase
/// 2). Defined in `semio_framework` so plugins and the OS product
/// share one definition without an inverted dependency; re-exported here verbatim.
use semio_framework_artifact_reference::{ArtifactDialect, Dialect};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
/// 🙋️ Explicit actor identity for standalone local births and fixtures.
use protocol::LOCAL_ACTOR_ID;
use store::ArtifactStoreInitializationAuthority;
use semio_framework::AppRole;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_app_builder::App;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_window_kits::AppActionRegistry;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_app::ArtifactApp;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_reactor_app::PluginApp;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_emission::ActionMeta;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_emission::Emit;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::AppRenderOperationContext;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::ArtifactView;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::ConfigView;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::DraftView;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::InteractionHoverState;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::InteractionView;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::NoDraft;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::NoPresence;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::NoPresenceMutation;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::NoTransient;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::NoTransientMutation;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::PresenceView;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_window_transient::window_transient::TransientView;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_instance_ladders::ladders_plugin_app_impl::MAINTENANCE_STAGES;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_instance_app::VcsArtifactApp;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_instance_children::children_instance_root::genesis_member_schema;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_job::reserved_jobs::is_framework_reserved_action_id;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_registry::artifact_registration::surface_dependency_breaches;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::child_view::ChildContentView;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::history_view::HistoryView;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_interaction::interaction_topology::empty_draft_references;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::PeerPresenceRoot;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_plugin_program::Plugin;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_job::reserved_jobs::register_framework_reserved_factories;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_ownership::store_lifecycle::bounded_config_store_disposer;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_ownership::store_lifecycle::bounded_document_store_disposer;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_surface::ArtifactEditor;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_surface::ArtifactViewer;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_emission::surface_contract::ViewEmit;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_surface::ViewerApp;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwnerHandle;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolFactoryRegistry;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_job::TransactionProposalDraft;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_interaction::UiAssemblyResult;

#[cfg(any(test, feature = "artifact-app-testing"))]
#[path = "../⚖️authoring-law/🦀️.rs"]
pub mod authoring_law;

#[cfg(any(test, feature = "artifact-app-testing"))]
#[path = "🧪️artifact-app-laws/🦀️.rs"]
pub mod artifact_app_laws;
