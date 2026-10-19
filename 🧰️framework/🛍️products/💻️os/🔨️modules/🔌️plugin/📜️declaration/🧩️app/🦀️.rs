//! 🧩️ The app contract: `ArtifactApp`, media artifacts, `PluginApp` and the action registry.
#![feature(associated_type_defaults)]
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[doc(hidden)]
#[path = "../👷️job/🦀️.rs"]
pub mod tool_contract;
#[doc(hidden)]
#[path = "🛬️lifted/♻️retirement/🦀️.rs"]
pub mod completion_retirement;
#[allow(unused_imports)]
pub use completion_retirement::*;
#[doc(hidden)]
#[path = "🛬️lifted/⚠️fault/🦀️.rs"]
pub mod completion_fault;
#[allow(unused_imports)]
pub use completion_fault::*;
#[doc(hidden)]
#[path = "🛬️lifted/🚫️refusal/🦀️.rs"]
pub mod completion_refusal_retirement;
#[allow(unused_imports)]
pub use completion_refusal_retirement::*;
#[doc(hidden)]
#[path = "🛬️lifted/🧬️context/🦀️.rs"]
pub mod owned_context;
#[allow(unused_imports)]
pub use owned_context::*;
#[doc(hidden)]
#[path = "🛬️lifted/🎞️media-turn/🦀️.rs"]
pub mod media_operation_refusal;
#[allow(unused_imports)]
pub use media_operation_refusal::*;
#[doc(hidden)]
#[path = "🛬️lifted/🛫️intrinsic/🦀️.rs"]
pub mod media_intrinsic;
#[allow(unused_imports)]
pub use media_intrinsic::*;
#[doc(hidden)]
#[path = "🛬️lifted/⏳️operation-progress/🦀️.rs"]
pub mod operation_progress;
#[allow(unused_imports)]
pub use operation_progress::*;
#[doc(hidden)]
#[path = "🛬️lifted/📨️emission/🦀️.rs"]
pub mod app_emission;
#[doc(hidden)]
#[path = "🛬️lifted/⏯️tool-run/🦀️.rs"]
pub mod tool_run_contract;
#[doc(hidden)]
#[path = "🛬️lifted/🎬️media-export/🦀️.rs"]
pub mod media_contract;
use semio_framework::history_edit::HISTORY_EDIT_ACTION_IDS;
use semio_framework_plugin_window_kits::AppActionRegistry;
use semio_framework_plugin_mounted_owner::MountedOwnerPhaseV1;
use semio_framework_plugin_ownership::transient_publication::bounded_transient_store_disposer;
use semio_framework_plugin_window_config::WindowConfigOwnerRegistry;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwnerRegistry;
use semio_framework_value::DslValue;
use semio_framework::kernel::UiDirtyScope;
/// 🪪️ Declarative app manifest shared by plugin builders and surface registrations.
use semio_framework::AppDefinition;
use semio_framework::{kernel::{ActorId, ClipboardError, ClipboardFragment, Effect, PastePlacement}, ActionArgDef, ActionDefinition, ActionKind, AppIo, ArgPresentation, CapabilityAudience, ConfigSpec, Fault, FaultCode, FaultOrigin, InteractionVerb, MediaForm, MediaPortSpec, ViewModel, REVERT_TO_COMMAND_ACTION_ID, OPEN_ARTIFACT_FILE_ACTION_ID, SAVE_ARTIFACT_FILE_ACTION_ID};
use semio_framework_2d_compute::EngineHandles;
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
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::HashMap;
use pack::snapshot::ArtifactPack;
use store::{os_store::store_schema::ArtifactEnvelope, ArtifactStore, ConfigStore};
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_layout::layout::ContextMenuItemSpec;
use semio_framework_ui_layout::layout::ContextMenuRequest;
use semio_framework_ui_layout::layout::WindowEngagement;
use semio_framework_ui_layout::layout::WindowMeasure;
/// 🎞️ `MediaWireFormat` backs `MediaArtifactDescriptor::wire` (see `🔖️DocumentContract`
/// below) — the plugin ABI's `consume-media`/`produce-media` payload framing, separate from
/// `MediaPayload` above (which pairs with `Media`'s per-port `MediaType` projection).
/// The legacy format enum was retired in ticket 26/08/11/
/// SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6 — `MediaWireFormat::
/// Binary` now carries a plain format kind id string.
use semio_framework::MediaWireFormat;
/// 🎞️ The `Media`/`MediaPayload`/`MediaFingerprint`/`MediaError` value vocabulary backing
/// `ArtifactApp::{media_ports, export_media, import_media, media_fingerprint}` — re-exported so
/// implementers never need a direct `semio-framework-core` dependency just to satisfy this trait.
use semio_framework::{Media, MediaError, MediaFingerprint, MediaPayload};
/// 🧬️ `MediaClass`/`MediaType` also live in `semio-framework-core` — re-exported so callers can build
/// `ArtifactKindSpec.media_type` and `AppBuilder::media_input(...)`/`media_output(...)` port specs
/// without a direct `semio-framework-core` dependency.
use semio_framework::{MediaClass, MediaType};
/// 🏅️🪆️🎯️ Standards/subsets dialect vocabulary (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase
/// 2). Defined in `semio_framework` so plugins and the OS product
/// share one definition without an inverted dependency; re-exported here verbatim.
use semio_framework_artifact_reference::Dialect;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress}};
use semio_framework::AppRole;
use semio_framework_plugin_decl_emission::Emit;
use crate::app_emission::EphemeralEmit;
use semio_framework_plugin_decl_emission::HostEvent;
use semio_framework_plugin_decl_views::AppRenderOperationContext;
use semio_framework_plugin_decl_views::{ArtifactView,ArtifactPendingView,MountedSnapshotReadPreparation};
use semio_framework_plugin_decl_views::ConfigView;
use semio_framework_plugin_decl_views::DraftView;
use semio_framework_plugin_decl_views::InteractionView;
use semio_framework_plugin_decl_views::PresenceView;
use semio_framework_plugin_window_transient::window_transient::TransientView;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use crate::media_contract::ArtifactMediaExportJobRequest;
use crate::tool_contract::ArtifactReservedToolJob;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactInitializationAdmission;
use semio_framework_plugin_decl_emission::ArtifactMutationOutcome;
use semio_framework_plugin_ownership::store_lifecycle::bounded_config_store_disposer;
use semio_framework_plugin_ownership::store_lifecycle::bounded_document_store_disposer;
use semio_framework_plugin_ownership::store_lifecycle::bounded_document_store_initialization_job;
use crate::tool_contract::ArtifactBoundedFirstStepProof;
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwnerHandle;
use crate::tool_contract::ArtifactOwnedToolJobRequest;
use crate::tool_contract::ArtifactReservedToolJobRequest;
use crate::tool_contract::ArtifactToolCompletionValue;
use crate::tool_contract::ArtifactToolFactoryRegistry;
use semio_framework_plugin_decl_views::tool_contract::EmptyArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunJob;
use crate::tool_run_contract::ToolRunJobRequest;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;

use semio_framework_plugin_window_kits::merge_ui_values;
use semio_framework_plugin_interaction::UiAssemblyResult;

#[path = "🎛️artifact-app-trait/🦀️.rs"]
mod artifact_app_trait;
pub use artifact_app_trait::*;

#[path = "🗂️natural-file-and-media-artifacts/🦀️.rs"]
mod natural_file_and_media_artifacts;
pub use natural_file_and_media_artifacts::*;

#[path = "🛡️viewer-actions-and-transaction-outcomes/🦀️.rs"]
mod viewer_actions_and_transaction_outcomes;
pub use viewer_actions_and_transaction_outcomes::*;

//#endregion 🔖️Limits

#[path="📣️proofs/🦀️.rs"]
mod bounded_first_step_proof_catalog;
