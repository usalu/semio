//! 🥅️ Render-independent framework kernel: declarative {@link UiNode}, {@link Platform}, {@link ActionBus}.

extern crate semio_framework_os_kernel as protocol_core;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as dsl;
// 🔁️ ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W1: `store`
// alias (same pattern every plugin's own glue.rs already uses) so `🔁️workflow/🦀️component.rs`'s
// `store::ArtifactPack`/`store::SpaceConflict`/etc. references resolve once mounted below — this
// crate never referenced `store::` directly before, only through re-exported item names.
extern crate semio_framework_os_kernel as store;
// 🔁️ self-alias so `🔁️workflow/🦀️component.rs`'s own `use semio_framework::{...}` lines resolve
// once mounted below — this crate never needed to refer to itself by its external name before.
extern crate self as semio_framework;

pub use ui_wgpu::wgpu::IconName;
pub use ui_wgpu::wgpu::{Locale, Terminology};

#[path = "../../🔨️modules/🎯️action-bus/🦀️component.rs"]
pub mod action_bus;

#[path = "../../🔨️modules/🚪️io/🦀️component.rs"]
pub mod io;

#[path = "../../🔨️modules/🖥️platform/🦀️component.rs"]
pub mod platform;

#[path = "../../🔨️modules/🛂️manifest/🦀️component.rs"]
pub mod manifest;

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W0: pure hover/selection state
// machine + declaration types, mirroring `manifest`'s own facet-nesting convention (`schema` sits
// alongside the root `component` rather than under it, same as `writer`'s `config { component; schema; }`).
#[path = "."]
pub mod interaction {
    #[path = "../../🔨️modules/🕹️interaction/🦀️component.rs"]
    mod component;
    pub use component::*;

    #[path = "../../🔨️modules/🕹️interaction/🧬️schema/🦀️component.rs"]
    pub mod schema;
}

// 🔁️ ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W1: mounted
// HERE, not in the os-kernel crate — its `semio_framework::{AppDefinition, MediaClass, MediaType,
// ConfigSpec, Terminology, Locale, …}` references need this crate's full assembled surface (mesh's
// media vocabulary, manifest's kernel types, ui_wgpu's Locale/Terminology — all re-exported below),
// which the wasm-safe os-kernel crate cannot depend on without a real dependency cycle (see the
// os-kernel glue.rs's own comment at the site this used to be attempted). The run crate's own
// `extern crate ... as workflow;` alias points here now, not at the kernel.
#[path = "../../🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs"]
pub mod workflow;


pub use action_bus::{ActionBus, ActionHandler, optional_json_to_dsl};
pub use dsl::{from_dsl_value, to_dsl_value, DslValue};
pub use dsl::{Diagnostic, Fault, FaultCause, FaultCode, FaultFrom, FaultOrigin, FaultScope, Severity, TextError, TextSpan};

// 🛂️ The declarative component model (layout/utilities/UiNode) lives in `ui_wgpu` now — re-import
// honestly (not a re-export) wherever this crate's manifest/kernel types need it; see `pub mod manifest`.
// 🔺️ Mesh geometry data, primitive construction, and Obj/Glb/Stl codecs are dissolved into a
// dedicated engine crate (consumed only from artifact facet code / engine-to-engine callers such
// as brep tessellation) — no longer part of this framework module's own re-export surface.
pub use semio_framework_mesh_engine::{
    mesh_box, mesh_cone, mesh_cylinder, mesh_from_glb, mesh_from_indexed, mesh_from_indexed_with_face_groups, mesh_from_kind, mesh_ico_sphere,
    mesh_plane, mesh_to_glb, mesh_to_obj, mesh_from_obj, mesh_to_stl, mesh_from_stl, mesh_torus, mesh_uv_sphere, MeshData,
    MeshExporter, MeshImporter, ObjExporter, ObjImporter, GlbExporter, GlbImporter, StlExporter, StlImporter,
    IoError,
};
// 🚪️ DWG codec (`dwg_to_bytes`/`dwg_from_bytes`/`mesh_to_dwg_drawing`/…) DELETED (ticket 26/08/12/
// DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave DEDUP): `🔺️mesh/🦀️component.rs`
// was a misplaced, fully-duplicated copy of stdio's real DWG artifact
// (`semio_s_plugin_stdio::artifacts::dwg::{dwg_to_bytes, dwg_from_bytes, mesh_to_dwg_drawing, …}`,
// `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/…`). Its sole framework-tier caller
// (`🧊️3d/📐️brep/📦️mesh-io`) moved into stdio's own brep engine this same wave, so this re-export
// has zero remaining callers. `🔺️mesh/🟦️component.ts` (unrelated scene-protocol payload types,
// still imported by `🟦️glue.ts`) was NOT touched — only the Rust DWG codec shared that directory.
// 🔀️ OsMediaCapability/ArtifactKindSpec/MediaClass/MediaForm/MediaType/MediaWireFormat/MediaPortDirection/
// PortMultiplicity/MediaPortSpec/MediaCompat/media_types_compatible/Media/MediaPayload/MediaFingerprint/
// MediaError/MediaConverter/AppIo/ArtifactPresentation/ConfigFieldShape/ConfigFieldSpec/ConfigSpec/
// CommandFieldSpec/CommandVariantSpec/CommandGrammar relocated from `mesh` into `manifest` (ticket
// 26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT wave 4a) — reachable below via `pub use manifest::*;`
// instead, so no external call site needs to change.
pub use io::{
    StandardId, SubsetId, Dialect, ArtifactDialect,
    AnalyzeSource, Confidence as IoConfidence, Analysis, ComposeSource, Composition, ComposeError,
    IoPayload, ErasedComposeSource, ComposedArtifact, ComposerEntry,
    IoDirection, IoKey, IoResolveError,
    register_composer_entries, resolve as io_resolve, dialects_for as io_dialects_for,
    io_keys_for, list_composer_entries, io_dispatch, set_io_fallback_dispatcher,
    WireComposeSource, WireComposedArtifact, wire_list_composer_entries, wire_artifact_compose, wire_decode_composed_artifact,
    SubsetValidator, SubsetValidatorEntry, subset_validator_entry_of, register_subset_validator,
    FormatDescriptor, register_format_descriptors, format_descriptor, normalize_format_kind, format_accept_filter, formats_csv,
};
pub use platform::{PanelVisibility, Platform, PlatformSpec};
pub use workflow::*;
pub use manifest::*;
pub use manifest as ui;
pub use interaction::*;
pub use manifest::kernel::{
    ActorId, AppEvent, AppInstanceId, AssetHandle, Capability, CapabilityGrant, CapabilityRequirement,
    CapabilityToken, ActionContext, ActionDef, ActionId, ActionInvocation, CommandContext, CommandId, CommandInvocation,
    ActionRequest, InvocationId, InvocationResult, HostEffect, HybridLogicalTimestamp, IconRenderExportItem, InverseMutation,
    KernelMutation, MergeStrategyKind, ArtifactDiff, ArtifactHandle, ArtifactId, ArtifactKind,
    ArtifactVersion, MutationId, PhysicalSize, PluginInstanceId, PresencePeer,
    PresencePoint, PresenceViewport, decode_presence_peer, encode_presence_peer,
    Appearance, Rights, SchemaId, SchemaVersion, Scope, UndoGroup, UndoPolicy,
    WindowEvent, WindowHandle, WindowInput, WindowKindDef, WindowKindId, WindowOutput,
};
