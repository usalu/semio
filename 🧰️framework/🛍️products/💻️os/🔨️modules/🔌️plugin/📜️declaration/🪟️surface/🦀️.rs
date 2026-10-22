//! 🪟️ Editor and viewer surfaces: `ArtifactEditor`, `ArtifactViewer`, their app wrappers, builders, declarations and the artifact codec table.
#![feature(associated_type_defaults)]
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[doc(hidden)]
#[path = "../📥️natural-file/🦀️.rs"]
pub mod natural_file_import;
use semio_framework_plugin_mounted_owner::MountedOwnerPhaseV1;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_mounted_owner::MountedOwnerTurnV1;
use semio_framework_plugin_ownership::transient_publication::bounded_transient_store_disposer;
use semio_framework_plugin_window_config::WindowConfigOwnerRegistry;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_window_config::WindowConfigPack;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwnerRegistry;
use semio_framework_value::DslValue;
use protocol::{OpBinary, OpText};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::kernel::FixedCommandPage;
use semio_framework::kernel::UiDirtyScope;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::manifest::{ActionInvocation as ManifestActionInvocation, CommandInvocation as ManifestCommandInvocation};
/// 🪪️ Declarative app manifest shared by plugin builders and surface registrations.
use semio_framework::AppDefinition;
use semio_framework::{kernel::{ActorId, ClipboardError, ClipboardFragment, Effect, PastePlacement}, ActionArgDef, ActionDefinition, ActionKind, ActionRef, AppIo, CapabilityAudience, CommandDefinition, CommandGrammar, ConfigSpec, DialogDefinition, Fault, FaultCode, FaultFrom, FaultOrigin, IconName, InteractionDefinition, InteractionRef, InteractionVerb, MediaForm, MediaPortSpec, ModeDefinition, PanelGroup, PanelTabDefinition, ToolDefinition, ToolRef, UtilityDefinition, UtilityRef, ViewModel, WindowKindDefinition};
use semio_framework::onboarding::{IntroductionDefinition, TutorialDefinition};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::{kernel::{AppEvent, HistoryPatch, InvocationResult}};
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
use std::collections::HashMap;
/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: `AsyncTask`'s boxed `run` closure/future.
use std::future::Future;
/// 🧵️ Used only by `artifact_app_laws`' spawned-task shape, so a plain `cargo check` reports it
/// unused while `--all-targets` needs it — do not remove on the strength of that warning.
#[allow(unused_imports)]
use std::pin::Pin;
use pack::snapshot::ArtifactPack;
use store::{os_store::document_materialize::create_document_envelope, os_store::store_schema::ArtifactEnvelope, ArtifactStore, ConfigStore, MemberFactory, Mutation, MutationDiff, NoMembers, SpaceMember};
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_layout::layout::ContextMenuItemSpec;
use semio_framework_ui_layout::layout::ContextMenuRequest;
use semio_framework_ui_layout::layout::NamedLayout;
use semio_framework_ui_layout::layout::WindowEngagement;
use semio_framework_ui_layout::layout::WindowLayout;
use semio_framework_ui_layout::layout::WindowMeasure;
/// 🗂️ `OsMediaCapability`/`ArtifactKindSpec` now live in `semio-framework-core` (both this crate and
/// `semio-framework-os` already depend on it) the same way the legacy format enum used to — re-exported
/// here verbatim instead of duplicated, so `AppBuilder::artifact_kind(...)` and
/// `semio_framework_os`'s artifact catalog registry share one definition.
use semio_framework::ArtifactKindSpec;
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
use semio_framework_artifact_reference::{ArtifactDialect, Dialect};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_artifact_reference::SubsetId;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress}};
use semio_framework::{surface_app_id, AppRole};
#[cfg(test)]
use semio_framework_plugin_decl_views::NoConfig;
#[cfg(test)]
use semio_framework_plugin_decl_views::NoConfigMutation;
#[cfg(test)]
use semio_framework_plugin_decl_views::NoPresence;
#[cfg(test)]
use semio_framework_plugin_decl_views::NoPresenceMutation;
#[cfg(test)]
use semio_framework_plugin_window_transient::built_text_to_component_tree;
#[cfg(test)]
use semio_framework_plugin_decl_app::MediaConsumption;
#[cfg(test)]
use semio_framework_plugin_decl_app::MediaArtifactError;
#[cfg(test)]
use semio_framework_plugin_decl_views::media_contract::ArtifactMediaExportResult;
#[cfg(test)]
use semio_framework_plugin_decl_views::media_contract::ArtifactMediaExportPoll;
#[cfg(test)]
use semio_framework_plugin_decl_views::media_contract::ArtifactMediaExportHandle;
#[cfg(test)]
use semio_framework_plugin_decl_app::EphemeralSnapshot;
#[cfg(test)]
use semio_framework_plugin_decl_app::TransactionPrepareOutcome;
#[cfg(test)]
use semio_framework_plugin_decl_emission::ActionMeta;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolPublicContract;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::ToolOwnerWitness;
use semio_framework_plugin_app_builder::AppBuilder;
use semio_framework_plugin_app_builder::ExampleSource;
use semio_framework_plugin_app_builder::PanelTabSpec;
use semio_framework_plugin_window_kits::AppActionRegistry;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_app::NATURAL_FILE_PORT;
use semio_framework_plugin_decl_app::NaturalFileCodec;
use semio_framework_plugin_decl_app::NaturalFileDecodeOwner;
use semio_framework_plugin_decl_app::document_load_emit;
use semio_framework_plugin_decl_app::natural_file_input_bytes;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_decl_emission::surface_contract::ViewEmit;
use semio_framework_plugin_decl_app::app_emission::EphemeralEmit;
use semio_framework_plugin_decl_emission::HostEvent;
use semio_framework_plugin_decl_views::AppRenderOperationContext;
use semio_framework_plugin_decl_views::{ArtifactView,ArtifactPendingView,MountedSnapshotReadPreparation};
use semio_framework_plugin_decl_views::ConfigView;
use semio_framework_plugin_decl_views::DraftView;
use semio_framework_plugin_decl_views::InteractionView;
use semio_framework_plugin_decl_views::NoDraft;
use semio_framework_plugin_decl_views::NoDraftMutation;
use semio_framework_plugin_decl_views::PresenceView;
use semio_framework_plugin_window_transient::window_transient::TransientView;
use semio_framework_plugin_abi::ArtifactBuilder;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_plugin_decl_app::media_contract::ArtifactMediaExportJobRequest;
use semio_framework_plugin_decl_app::tool_contract::ArtifactReservedToolJob;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactInitializationAdmission;
use semio_framework_plugin_decl_emission::ArtifactMutationOutcome;
use semio_framework_plugin_ownership::store_lifecycle::bounded_config_store_disposer;
use semio_framework_plugin_ownership::store_lifecycle::bounded_document_store_disposer;
use semio_framework_plugin_ownership::store_lifecycle::bounded_document_store_initialization_job;
use semio_framework_plugin_decl_app::tool_contract::ArtifactBoundedFirstStepProof;
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwnerHandle;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobRequest;
use semio_framework_plugin_decl_app::tool_contract::ArtifactReservedToolInput;
use semio_framework_plugin_decl_app::tool_contract::ArtifactReservedToolJobRequest;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionValue;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolFactoryRegistry;
use semio_framework_plugin_decl_views::tool_contract::EmptyArtifactInstanceOperationOwner;
use crate::natural_file_import::NaturalFileImportJob;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunJob;
use semio_framework_plugin_decl_app::tool_run_contract::ToolRunJobRequest;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_window_kits::merge_ui_values;
use semio_framework_plugin_interaction::UiAssemblyResult;

#[path = "✏️artifact-editor-trait/🦀️.rs"]
mod artifact_editor_trait;
pub use artifact_editor_trait::*;

#[path = "🧬️artifact-codec-table-and-replay/🦀️.rs"]
mod artifact_codec_table_and_replay;
pub use artifact_codec_table_and_replay::*;

#[path = "👁️artifact-viewer-trait/🦀️.rs"]
mod artifact_viewer_trait;
pub use artifact_viewer_trait::*;

#[path = "🖊️editor-app/🦀️.rs"]
mod editor_app;
pub use editor_app::*;

#[path = "📖️viewer-app/🦀️.rs"]
mod viewer_app;
pub use viewer_app::*;

#[path = "🏗️surface-builders/🦀️.rs"]
mod surface_builders;
pub use surface_builders::*;
