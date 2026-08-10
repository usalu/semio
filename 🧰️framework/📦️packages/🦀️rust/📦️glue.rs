//! 🥅️ Render-independent framework kernel: declarative {@link UiNode}, {@link Platform}, {@link ActionBus}.

extern crate semio_framework_os_kernel as protocol_core;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as dsl;

pub use ui_wgpu::wgpu::IconName;
pub use ui_wgpu::wgpu::{Locale, Terminology};

#[path = "../../🔨️modules/🎯️action-bus/🦀️component.rs"]
pub mod action_bus;

#[path = "../../🔨️modules/🔺️mesh/🦀️component.rs"]
pub mod mesh;

#[path = "../../🔨️modules/🖥️platform/🦀️component.rs"]
pub mod platform;

#[path = "../../🔨️modules/🛂️manifest/🦀️component.rs"]
pub mod manifest;


pub use action_bus::{ActionBus, ActionHandler, optional_json_to_dsl};
pub use dsl::{from_dsl_value, to_dsl_value, DslValue};
pub use dsl::{Diagnostic, Fault, FaultCause, FaultCode, FaultFrom, FaultOrigin, FaultScope, Severity, TextError, TextSpan};

// 🛂️ The declarative component model (layout/utilities/UiNode) lives in `ui_wgpu` now — re-import
// honestly (not a re-export) wherever this crate's manifest/kernel types need it; see `pub mod manifest`.
pub use mesh::{
    mesh_box, mesh_cone, mesh_cylinder, mesh_from_glb, mesh_from_indexed, mesh_from_indexed_with_face_groups, mesh_from_kind, mesh_ico_sphere,
    mesh_plane, mesh_to_glb, mesh_to_obj, mesh_from_obj, mesh_to_stl, mesh_from_stl, mesh_torus, mesh_uv_sphere, MeshData,
    dwg_drawing_to_mesh, dwg_drawing_to_paths, dwg_from_bytes, dwg_to_bytes, mesh_to_dwg_drawing, paths_to_dwg_drawing,
    DwgColor, DwgDrawing, DwgEntity, DwgGeometry, DwgLayer, DwgPathSegment,
    MeshExporter, MeshImporter, ObjExporter, ObjImporter, GlbExporter, GlbImporter, StlExporter, StlImporter,
    MediaFormat, IoError, DocumentCodec, RasterImage, PageDoc, PageDocPage, TableDoc, TextDoc, Archive, ArchiveEntry,
    TxtCodec, MdCodec, JsonCodec, CsvCodec, BmpCodec, PngCodec, JpgCodec, GifCodec, TiffCodec,
    PdfCodec, DocxCodec, PptxCodec, XlsxCodec, ZipCodec, BcfCodec, PlyCodec, LasCodec, GltfCodec, DxfCodec, IfcCodec,
    OsMediaCapability, ArtifactKindSpec,
    MediaClass, MediaForm, MediaType, MediaWireFormat, MediaPortDirection, PortMultiplicity, MediaPortSpec,
    MediaCompat, media_types_compatible,
    Media, MediaPayload, MediaFingerprint, MediaError, MediaConverter,
    AppIo, ArtifactPresentation,
    ConfigFieldShape, ConfigFieldSpec, ConfigSpec,
    CommandFieldSpec, CommandVariantSpec, CommandGrammar,
};
pub use platform::{PanelVisibility, Platform, PlatformSpec};
pub use manifest::*;
pub use manifest as ui;
pub use manifest::kernel::{
    ActorId, AppEvent, AppInstanceId, AssetHandle, Capability, CapabilityGrant, CapabilityRequirement,
    CapabilityToken, ActionContext, ActionDef, ActionId, ActionInvocation, CommandContext, CommandId, CommandInvocation,
    ActionRequest, InvocationId, InvocationResult, HostEffect, HybridLogicalTimestamp, IconRenderExportItem, InverseMutation,
    KernelMutation, MergeStrategyKind, DocumentDiff, DocumentHandle, DocumentId, DocumentKind,
    DocumentVersion, MutationId, PhysicalSize, PluginInstanceId, PresencePeer,
    PresencePoint, PresenceViewport, decode_presence_peer, encode_presence_peer,
    ArtifactId, ArtifactKind, Appearance, Rights, SchemaId, SchemaVersion, Scope, UndoGroup, UndoPolicy,
    WindowEvent, WindowHandle, WindowInput, WindowKindDef, WindowKindId, WindowOutput,
};
