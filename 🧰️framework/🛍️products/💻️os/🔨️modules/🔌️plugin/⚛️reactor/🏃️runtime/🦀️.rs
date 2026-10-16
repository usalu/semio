//! 📤️ WASM component export glue for plugin bundles.
#[cfg(test)] use semio_framework_plugin_ownership::bounded_ownership::runtime_lifecycle_step;
use crate::app_emission::ActionMeta;
use crate::plugin_program::{AppInstance, Plugin, PluginProgram};
use crate::media_contract::ArtifactMediaExportHandle;
use crate::media_contract::ArtifactMediaExportPoll;
use crate::media_contract::ArtifactMediaExportResult;
use crate::tool_jobs::{EmitWire, TransactionProposalDraft, TypedOperationLeftover, TypedOperationResultPage, TypedOperationResultToken};
use crate::app_contract::{EphemeralSnapshot, MediaArtifact, MediaArtifactDescriptor, MediaArtifactError, MediaConsumption};
use crate::plugin_app::PluginApp;
use semio_framework::MediaError;
use semio_framework_plugin_ownership::sdk_fault::PluginAssemblyError;
use crate::peer_roster::PresenceRosterAdmission;
use crate::{ArtifactApp, WindowConfigPack, MountedOwnerPolicyV1, MountedOwnerTurnV1, MountedOwnerIdentityV1, MountedOwnerPhaseV1};
use semio_framework_value::{DslValue,RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework::manifest::{ActionInvocation as ManifestActionInvocation, CommandAddress as ManifestCommandAddress, CommandInvocation as ManifestCommandInvocation, CommandOwnerAddress as ManifestCommandOwnerAddress};
use semio_framework::{
    kernel::{ActivationEvent, CapabilityRequest, CapabilityRequirement, Effect, InvocationResult, MessageEndpoint, QuotaSchema},
    AssetDeclaration, ExecutionMode, ExtensionPointDeclaration, Fault, FaultCode, FaultFrom, FaultOrigin, PluginManifest, TopicContribution, ViewModel,
};
/// 🎯️ M1/M2 (ticket 26/08/17 `design-unified.md`): `UiIntent`/`PresenceUpdate` for
/// `plugin_dispatch_intents`/`plugin_render_surface` — this module (a sibling of `pub mod app`, not
/// nested inside it) has no glob import of the contract crate the way `mod app` does.
use semio_framework_ui_contract as ui_contract;
/// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-helpers`): `sdk-flip` flipped `plugin_render`/
/// `plugin_render_with_document` below to return `ComponentTree` but this module (a sibling of
/// `pub mod app`, not nested inside it) never gained an import for the name — added here.
use semio_framework_ui_runtime::ComponentTree;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::{Cell, RefCell};
use std::mem::{size_of_val};
#[cfg(test)] use std::mem::{size_of};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicU8, Ordering};
use semio_framework_ui_layout::layout::{ContextMenuPoint, ContextMenuRequest, ContextMenuResponse, ContextMenuSurfaceTarget, UiMenuRef};
#[path = "📥️staging/♻️reactor-lifecycle/🛎️app-cell/🦀️.rs"]
mod life_app_cell;
pub use life_app_cell::*;
#[path = "📥️staging/♻️reactor-lifecycle/🧹️cleanup-vocabulary/🦀️.rs"]
mod life_cleanup_vocabulary;
pub use life_cleanup_vocabulary::*;
#[path = "📥️staging/♻️reactor-lifecycle/🗃️instance-registry/🦀️.rs"]
mod life_instance_registry;
pub use life_instance_registry::*;
#[path = "📥️staging/♻️reactor-lifecycle/🧯️cleanup-pumps/🦀️.rs"]
mod life_cleanup_pumps;
pub use life_cleanup_pumps::*;
#[path = "📦️instances/🖼️wire-frames/🦀️.rs"]
mod inst_wire_frames;
pub use inst_wire_frames::*;
#[path = "📦️instances/🪪️descriptor/🦀️.rs"]
mod inst_descriptor;
pub use inst_descriptor::*;
#[path = "📦️instances/🚪️opening/🦀️.rs"]
mod inst_opening;
pub use inst_opening::*;
#[path = "📦️instances/🔁️cleanup-turns/🦀️.rs"]
mod inst_cleanup_turns;
pub use inst_cleanup_turns::*;
#[path = "🧰️ops/🛡️wire-limits/🦀️.rs"]
mod ops_wire_limits;
pub use ops_wire_limits::*;
#[path = "🧰️ops/📄️document/🦀️.rs"]
mod ops_document;
pub use ops_document::*;
#[path = "🧰️ops/🎞️media/🦀️.rs"]
mod ops_media;
pub use ops_media::*;
#[path = "🧰️ops/🖥️surface-render/🦀️.rs"]
mod ops_surface_render;
pub use ops_surface_render::*;
#[path = "🧰️ops/📥️ingress/🦀️.rs"]
mod ops_ingress;
pub use ops_ingress::*;
#[path = "🧰️ops/🧾️typed-operations/🦀️.rs"]
mod ops_typed_operations;
pub use ops_typed_operations::*;
#[path = "🔄️exchange/🔀️plugin-exchange/🦀️.rs"]
mod exch_plugin_exchange;
pub use exch_plugin_exchange::*;
#[path = "📥️staging/🔌️reactor-app/🛠️runtime-helpers/🦀️.rs"]
mod app_runtime_helpers;
pub use app_runtime_helpers::*;
#[path = "../../🧩️extension/🛬️lifted/🏃️runtime/🦀️.rs"]
mod extension_runtime_lift;
pub use extension_runtime_lift::*;
pub use semio_framework_plugin_plugin_program::plugin_runtime::*;
use crate::reserved_job_outcomes::CleanupTurn;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
/// 🏗️ Plugin registration lives in each owner's root `🦀️.rs` via
/// [`Plugin::builder`](crate::Plugin::builder) + [`plugin_exports!`](crate::plugin_exports).
/// The retired `semio_plugin!` macro is gone — typestate on the builder makes missing identity fields a compile error.
#[cfg(test)]
include!("../../🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs");
