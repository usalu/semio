//! 🥅️ Render-independent framework kernel: declarative {@link UiNode}, {@link Platform}, {@link ActionBus}.

#[cfg(test)]
#[global_allocator]
static FRAMEWORK_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[cfg(test)]
#[path="../../🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]
pub(crate) mod test_allocation;
#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER:test_allocation::RequestedAllocator=test_allocation::RequestedAllocator;
pub use ui_wgpu::wgpu::IconName;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;

//#region 🧬️SchemaMetadata
#[cfg(feature = "typegen")]
#[path = "../../🔨️modules/🧬️schema/📽️projection/🦀️.rs"]
pub mod schema_metadata;
//#endregion 🧬️SchemaMetadata

#[path = "../../🔨️modules/📁️filesystem/📷️snapshot/🦀️.rs"]
pub mod source_projection;

#[path = "../../🔨️modules/🎯️action-bus/🦀️.rs"]
pub mod action_bus;


#[path = "../../🔨️modules/🌉️abi/🦀️.rs"]
pub mod abi;

pub use semio_framework_io_schema as io_schema;
pub use semio_framework_io_sqlite_snapshot as sqlite_snapshot;

#[path = "../../🔨️modules/🖥️platform/🦀️.rs"]
pub mod platform;

#[path = "../../🔨️modules/🛂️manifest/🦀️.rs"]
pub mod manifest;

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W0: pure hover/selection state
// machine + declaration types, mirroring `manifest`'s own facet-nesting convention (`schema` sits
// alongside the root `component` rather than under it, same as `writer`'s `config { component; schema; }`).
#[path = "."]
pub mod interaction {
    #[path = "../../🔨️modules/🕹️interaction/🦀️.rs"]
    mod component;
    pub use component::*;

    #[path = "../../🔨️modules/🕹️interaction/🧬️schema/🦀️.rs"]
    pub mod schema;
}



pub use action_bus::{
    ActionBus, ErasedToolJob, ToolCancellationPolicy, ToolDispatchError, ToolExecutionContract, ToolExecutionShape, ToolFactoryKey, ToolFreshnessPolicy, ToolJobDispatch, ToolJobFactory, ToolJobFactoryError, ToolOperationSpec,
    ToolPayload, ToolRegistrationError, ToolWireAdmission,
};
pub use semio_framework_value::{dsl_value,DslValue};
pub use semio_framework_diagnostic::Diagnostic;
pub use semio_framework_diagnostic::Fault;
pub use semio_framework_diagnostic::FaultCause;
pub use semio_framework_diagnostic::FaultCode;
pub use semio_framework_diagnostic::FaultFrom;
pub use semio_framework_diagnostic::FaultOrigin;
pub use semio_framework_diagnostic::FaultParams;
pub use semio_framework_diagnostic::is_fault_param_name;
pub use semio_framework_diagnostic::FaultScope;
pub use semio_framework_diagnostic::Severity;
pub use semio_framework_diagnostic::TextError;
pub use semio_framework_diagnostic::TextSpan;

// 🛂️ The declarative component model (layout/utilities/UiNode) lives in `ui_wgpu` now — re-import
// honestly (not a re-export) wherever this crate's manifest/kernel types need it; see `pub mod manifest`.
// 🔺️ Mesh geometry data, primitive construction, and Obj/Glb/Stl codecs are dissolved into a
// dedicated engine crate (consumed only from artifact facet code / engine-to-engine callers such
// as brep tessellation) — no longer part of this framework module's own re-export surface.
pub use semio_framework_mesh_engine::io as mesh_io;
pub use semio_framework_mesh_engine::{mesh_box, mesh_cone, mesh_cylinder, mesh_from_indexed, mesh_from_indexed_with_face_groups, mesh_from_kind, mesh_ico_sphere, mesh_plane, mesh_torus, mesh_uv_sphere, MeshAttribute, MeshAttributeDomain, MeshAttributeSemantic, MeshAttributeInterpolation, MeshTexture, PolygonMeshSource, validate_polygon_mesh_attributes, validate_mesh_surface_assets, validate_mesh_attribute, MeshData};
// 🚪️ DWG codec (`dwg_to_bytes`/`dwg_from_bytes`/`mesh_to_dwg_drawing`/…) DELETED (ticket 26/08/12/
// DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave DEDUP): `🔺️mesh/🦀️.rs`
// was a misplaced, fully-duplicated copy of stdio's real DWG artifact
// (`semio_s_artifact_stdio_dwg::{dwg_to_bytes, dwg_from_bytes, mesh_to_dwg_drawing, …}`,
// `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/…`). Its sole framework-tier caller
// (`🧊️3d/📐️brep/📦️mesh-io`) moved into stdio's own brep engine this same wave, so this re-export
// has zero remaining callers. `🔺️mesh/🟦️.ts` (unrelated scene-protocol payload types,
// still imported by `🟦️.ts`) was NOT touched — only the Rust DWG codec shared that directory.
// 🔀️ OsMediaCapability/ArtifactKindSpec/MediaClass/MediaForm/MediaType/MediaWireFormat/MediaPortDirection/
// PortMultiplicity/MediaPortSpec/MediaCompat/media_types_compatible/Media/MediaPayload/MediaFingerprint/
// MediaError/MediaConverter/AppIo/ArtifactPresentation/ConfigSpec/
// CommandVariantSpec/CommandGrammar relocated from `mesh` into `manifest` (ticket
// 26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT wave 4a) — reachable below via `pub use manifest::*;`
// instead, so no external call site needs to change.
pub use abi::*;
pub use interaction::*;
pub use semio_framework_artifact_reference::{ArtifactDialect, Dialect, StandardId, SubsetId};
pub use manifest as ui;
pub use manifest::kernel::{
    decode_presence_peer,
    encode_presence_peer,
    ActionContext,
    ActionDef,
    ActionId,
    ActionInvocation,
    ActionRequest,
    ActorId,
    AppEvent,
    AppInstanceId,
    Appearance,
    ArtifactDiff,
    ArtifactHandle,
    ArtifactId,
    ArtifactKind,
    ArtifactVersion,
    AssetHandle,
    Capability,
    CapabilityGrant,
    CapabilityRequirement,
    CapabilityToken,
    CommandContext,
    CommandId,
    CommandInvocation,
    Effect,
    HybridLogicalTimestamp,
    IconRenderExportItem,
    InverseMutation,
    InvocationId,
    InvocationResult,
    KernelMutation,
    MutationId,
    PhysicalSize,
    PluginInstanceId,
    PresencePeer,
    PresenceToolRun,
    PresenceToolRunState,
    PresenceUi,
    PresenceViewKind,
    PresenceWindowView,
    // 🎫️ ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME packet A3-kernel-types: `RequestId`
    // is the completion-correlation id every `req`-carrying `Effect` variant now needs at its call
    // site — re-exported here so plugin call sites can name it as `semio_framework::RequestId` /
    // `semio_framework_plugin::RequestId` without a separate import.
    RequestId,
    Rights,
    SchemaId,
    SchemaVersion,
    Scope,
    UndoGroup,
    UndoPolicy,
    WindowEvent,
    WindowHandle,
    WindowInput,
    WindowKindDef,
    WindowKindId,
    WindowOutput,
};
pub use manifest::*;
pub use platform::{PanelVisibility, Platform, PlatformSpec};
