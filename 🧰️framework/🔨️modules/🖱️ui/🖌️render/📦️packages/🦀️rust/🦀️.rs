//! 🖼️ The custom GPU renderer's backend-neutral core.
//!
//! This crate lowers a semantic [`ui_contract::UiSnapshot`] into pixels' worth of *description* — a
//! [`RenderPacket`] — and stops there. It contains no device, no swapchain, no shader compilation and
//! no window: `wgpu`, `winit` and every graphics binding live in the backend and host crates, and a CI
//! `cargo tree` assertion fails the build if one of them appears here.
//!
//! The pipeline is one synchronous run-to-completion transaction (ruling U1, ticket
//! `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`):
//!
//! ```text
//! UiSnapshot/UiPatch → elements → request_layout → taffy → prepaint → paint
//!                    → Scene::finish → FrameSnapshot → RenderPacket → GraphicsBackend
//! ```
//!
//! Nothing suspends inside it. A dependency that is not ready yet (an unshaped font, an undecoded
//! image) is represented as `Measurement::Pending` and drawn as a placeholder; when it lands, the
//! resource registry invalidates the windows that referenced it and a *later* frame shows it. That is
//! why input dispatch can be guaranteed to run against the same generation the user actually saw:
//! `FrameSnapshot` carries the scene, hitboxes, dispatch tree, focus, IME and accessibility together,
//! and is swapped in atomically.

#[path = "../../🧱️element/🦀️.rs"]
mod element;
#[path = "../../🖼️frame/🦀️.rs"]
mod frame;
#[path = "../../📏️layout/🦀️.rs"]
mod layout;
#[path = "../../🗺️surface/🦀️.rs"]
mod surface;

pub use element::{AnyElement, Element, FrameArena, FrameArenaIndex, PaintCx, PrepaintCx, RetainedStore, SharedFrameCx};
pub use frame::{AccessibilityNode, AccessibilitySnapshot, FocusSnapshot, FrameEngine, FrameGeneration, FrameInputs, FrameSnapshot, ImeSnapshot};
pub use layout::{AvailableSpace, LayoutCx, LayoutNodeId, MeasureFn};
pub use surface::{
    dispatch_pointer_moved, render_placed_surface, route_pointer_event, AnySurface, ClipId, Surface, SurfaceError, SurfaceInput, SurfacePlacement, SurfacePrepare, SurfaceRegistry, SurfaceRenderCx, SurfaceRenderTarget, SurfaceResourceNeeds,
    Transform2D,
};

pub use semio_framework_ui_render_interaction::{
    hit_test, node_bounds, AccessibilityEvent, AccessibilityTarget, Bounds, CursorRequest, Deadline, DeadlineKey, DispatchEvent, DispatchFlags, DispatchOutcome, DispatchTree, Dispatcher, ElementId, EventModifiers, FrameNodeId, FrameScheduler,
    Hitbox, ImeDirective, ImeEvent, InvalidationReason, ListenerSet, PointerButton, PointerId, PointerInfo, PointerKind, ReconciliationKey, TextEditTarget,
};
#[cfg(feature = "backend-testing")]
pub use semio_framework_ui_render_packet::ReadbackImage;
pub use semio_framework_ui_render_packet::{
    AtlasId, BackendError, DeviceCapabilities, DeviceStatus, DrawBatch, FinishParams, FrameStats, GlassInstance, GlassStyle, GpuTier, GraphicsBackend, LayerState, LayoutRect, LineVertex3, LossReason, MemoryClass, MeshId, MeshInstance, PhysicalSize,
    PipelineKind, QuadInstance, RecoveredResources, RenderPacket, RenderReport, ResourceKind, ResourceOp, ResourceRegistry, Scene, SceneBuilder, SceneError, ScissorRect, SurfaceFormat, SurfaceLighting, SurfaceMaterial, SurfacePass, SurfaceShadow,
    TextureId, VectorVertex,
};
pub use semio_framework_ui_render_shader_contract::{
    BindGroupEntrySpec, BindGroupSpec, BindingKind, BlendMode, ColorWriteMask, CompareFunction, CullMode, DepthBiasSpec, DepthStencilFormat, DepthStencilSpec, PipelineSpec, PrimitiveTopology, ShaderStageVisibility, StencilOperation,
    StencilStateSpec, VertexBufferSpec, VertexFormat, VertexStepMode, ALL_SHADERS, BLUR_DOWNSAMPLE_SHADER, BLUR_FAMILY, GLASS_FAMILY, GLASS_SHADER, SCENE_BLIT_SHADER, UI_CONTENT_PIPELINE, UI_FAMILY, UI_SHADER, VECTOR_FAMILY, VECTOR_PIPELINE,
    VECTOR_SHADER, WORLD3D_FAMILY, WORLD3D_LINES_SHADER, WORLD3D_SHADER, WORLD3D_TEXTURED_PIPELINE,
};
pub use semio_framework_ui_render_text::{FontDependencyId, FontFamilyChoice, Measurement, ShapedText, TextAlignment, TextFontError, TextFontRole, TextFontSet, TextRunStyle, TextStyle, TextSystem};
#[cfg(not(target_arch = "wasm32"))]
pub use semio_framework_ui_render_text::{TextFontFiles, TextFontReadError};
