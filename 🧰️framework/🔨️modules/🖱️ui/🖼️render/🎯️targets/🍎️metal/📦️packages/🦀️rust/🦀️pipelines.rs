//! @emoji 🏗️ Builds every `MTLRenderPipelineState`/`MTLDepthStencilState`/`MTLSamplerState` this
//! backend needs, from the hand-written MSL in `🦀️msl.rs`.
//!
//! **Metal's state split differs from wgpu's in a way that changes the object count.** In wgpu (and
//! the WGSL contract's `PipelineSpec`), blend mode, color-write-mask, depth-compare, depth-write and
//! stencil test/ops are all baked into one `RenderPipeline`. In Metal, only shader functions, the
//! vertex layout and per-attachment format/blend/write-mask live in `MTLRenderPipelineState`; depth
//! compare/write and stencil test/ops live in a *separate* `MTLDepthStencilState` bound at encode
//! time via `setDepthStencilState:`; and cull mode, winding, depth bias and viewport/scissor are pure
//! encoder state with no object at all. Concretely this means the wgpu target's
//! `world_pipeline_translucent` and `world_line_pipeline` — identical depth/stencil behaviour, only
//! their depth *bias* differs, and depth bias is encoder state here — collapse onto **one** shared
//! `MTLDepthStencilState` (`world3d_translucent_ds`); `🦀️backend.rs` applies the bias around the
//! translucent mesh draws specifically via `setDepthBias:slopeScale:clamp:` and resets it before the
//! line draws. Every other pipeline/state pairing below is a direct port of
//! `🎯️targets/🧊️wgpu/🦀️draw.rs`'s `UiPipelines::new` (`content_stencil_state`/`mask_stencil_state`,
//! the two UI pipelines, `vector_pipeline`, `world_pipeline{,_translucent}`, `world_line_pipeline`,
//! `blur_downsample_pipeline`, `scene_blit_pipeline`, `glass_pipeline`).

use crate::msl::{BLUR_DOWNSAMPLE_SHADER_MSL, GLASS_SHADER_MSL, SCENE_BLIT_SHADER_MSL, UI_SHADER_MSL, VECTOR_SHADER_MSL, WORLD3D_LINES_SHADER_MSL, WORLD3D_MESH_SHADER_MSL};
use crate::types::{World3dGpuInstance, World3dGpuVertex, WorldLineGpuVertex};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::ns_string;
use objc2_metal::{
    MTLBlendFactor, MTLBlendOperation, MTLCompareFunction, MTLDevice, MTLLibrary, MTLPixelFormat, MTLRenderPipelineDescriptor, MTLRenderPipelineState, MTLSamplerAddressMode, MTLSamplerDescriptor, MTLSamplerMinMagFilter, MTLSamplerMipFilter,
    MTLSamplerState, MTLStencilDescriptor, MTLStencilOperation, MTLVertexDescriptor, MTLVertexFormat, MTLVertexStepFunction, MTLColorWriteMask, MTLDepthStencilDescriptor, MTLDepthStencilState,
};
use ui_render::{GlassInstance, QuadInstance, VectorVertex};

//#region 🔖️Pipelines

/// 🧊️ The one combined depth+stencil pixel format every pipeline/pass in this backend that touches
/// depth or stencil agrees on. `Depth32Float_Stencil8` (not the wgpu target's `Depth24PlusStencil8`,
/// which has no exact Metal counterpart) because it is guaranteed available on every Metal-capable
/// Mac — `Depth24Unorm_Stencil8` is an Intel/AMD-only optional format and would silently fail
/// `newDepthStencilStateWithDescriptor`-adjacent texture creation on Apple Silicon.
pub const DEPTH_STENCIL_FORMAT: MTLPixelFormat = MTLPixelFormat::Depth32Float_Stencil8;

//#region 🎨️VertexDescriptors

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn ui_vertex_descriptor() -> Retained<MTLVertexDescriptor> {
    let descriptor = MTLVertexDescriptor::new();
    let layouts = descriptor.layouts();
    let attributes = descriptor.attributes();
    unsafe {
        let corner_layout = layouts.objectAtIndexedSubscript(0);
        corner_layout.setStride(8);
        corner_layout.setStepFunction(MTLVertexStepFunction::PerVertex);
        let instance_layout = layouts.objectAtIndexedSubscript(1);
        instance_layout.setStride(std::mem::size_of::<QuadInstance>() as _);
        instance_layout.setStepFunction(MTLVertexStepFunction::PerInstance);

        let corner_attr = attributes.objectAtIndexedSubscript(0);
        corner_attr.setFormat(MTLVertexFormat::Float2);
        corner_attr.setOffset(0);
        corner_attr.setBufferIndex(0);

        let rect_attr = attributes.objectAtIndexedSubscript(1);
        rect_attr.setFormat(MTLVertexFormat::Float4);
        rect_attr.setOffset(0);
        rect_attr.setBufferIndex(1);
        let color_attr = attributes.objectAtIndexedSubscript(2);
        color_attr.setFormat(MTLVertexFormat::Float4);
        color_attr.setOffset(16);
        color_attr.setBufferIndex(1);
        let params_attr = attributes.objectAtIndexedSubscript(3);
        params_attr.setFormat(MTLVertexFormat::Float4);
        params_attr.setOffset(32);
        params_attr.setBufferIndex(1);
        let uv_attr = attributes.objectAtIndexedSubscript(4);
        uv_attr.setFormat(MTLVertexFormat::Float4);
        uv_attr.setOffset(48);
        uv_attr.setBufferIndex(1);
    }
    descriptor
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn vector_vertex_descriptor() -> Retained<MTLVertexDescriptor> {
    let descriptor = MTLVertexDescriptor::new();
    let layouts = descriptor.layouts();
    let attributes = descriptor.attributes();
    unsafe {
        let layout = layouts.objectAtIndexedSubscript(0);
        layout.setStride(std::mem::size_of::<VectorVertex>() as _);
        layout.setStepFunction(MTLVertexStepFunction::PerVertex);
        let position_attr = attributes.objectAtIndexedSubscript(0);
        position_attr.setFormat(MTLVertexFormat::Float2);
        position_attr.setOffset(0);
        position_attr.setBufferIndex(0);
        let color_attr = attributes.objectAtIndexedSubscript(1);
        color_attr.setFormat(MTLVertexFormat::Float4);
        color_attr.setOffset(8);
        color_attr.setBufferIndex(0);
    }
    descriptor
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn glass_vertex_descriptor() -> Retained<MTLVertexDescriptor> {
    let descriptor = MTLVertexDescriptor::new();
    let layouts = descriptor.layouts();
    let attributes = descriptor.attributes();
    unsafe {
        let corner_layout = layouts.objectAtIndexedSubscript(0);
        corner_layout.setStride(8);
        corner_layout.setStepFunction(MTLVertexStepFunction::PerVertex);
        let instance_layout = layouts.objectAtIndexedSubscript(1);
        instance_layout.setStride(std::mem::size_of::<GlassInstance>() as _);
        instance_layout.setStepFunction(MTLVertexStepFunction::PerInstance);

        let corner_attr = attributes.objectAtIndexedSubscript(0);
        corner_attr.setFormat(MTLVertexFormat::Float2);
        corner_attr.setOffset(0);
        corner_attr.setBufferIndex(0);
        let rect_attr = attributes.objectAtIndexedSubscript(1);
        rect_attr.setFormat(MTLVertexFormat::Float4);
        rect_attr.setOffset(0);
        rect_attr.setBufferIndex(1);
        let tint_attr = attributes.objectAtIndexedSubscript(2);
        tint_attr.setFormat(MTLVertexFormat::Float4);
        tint_attr.setOffset(16);
        tint_attr.setBufferIndex(1);
        let params_attr = attributes.objectAtIndexedSubscript(3);
        params_attr.setFormat(MTLVertexFormat::Float4);
        params_attr.setOffset(32);
        params_attr.setBufferIndex(1);
    }
    descriptor
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn world3d_mesh_vertex_descriptor() -> Retained<MTLVertexDescriptor> {
    let descriptor = MTLVertexDescriptor::new();
    let layouts = descriptor.layouts();
    let attributes = descriptor.attributes();
    unsafe {
        let vertex_layout = layouts.objectAtIndexedSubscript(0);
        vertex_layout.setStride(std::mem::size_of::<World3dGpuVertex>() as _);
        vertex_layout.setStepFunction(MTLVertexStepFunction::PerVertex);
        let instance_layout = layouts.objectAtIndexedSubscript(1);
        instance_layout.setStride(std::mem::size_of::<World3dGpuInstance>() as _);
        instance_layout.setStepFunction(MTLVertexStepFunction::PerInstance);

        let position_attr = attributes.objectAtIndexedSubscript(0);
        position_attr.setFormat(MTLVertexFormat::Float3);
        position_attr.setOffset(0);
        position_attr.setBufferIndex(0);
        let normal_attr = attributes.objectAtIndexedSubscript(1);
        normal_attr.setFormat(MTLVertexFormat::Float3);
        normal_attr.setOffset(12);
        normal_attr.setBufferIndex(0);

        let offsets = [0u64, 16, 32, 48, 64, 80];
        for (slot, offset) in (3..=8).zip(offsets) {
            let attr = attributes.objectAtIndexedSubscript(slot);
            attr.setFormat(MTLVertexFormat::Float4);
            attr.setOffset(offset);
            attr.setBufferIndex(1);
        }
    }
    descriptor
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn world3d_line_vertex_descriptor() -> Retained<MTLVertexDescriptor> {
    let descriptor = MTLVertexDescriptor::new();
    let layouts = descriptor.layouts();
    let attributes = descriptor.attributes();
    unsafe {
        let layout = layouts.objectAtIndexedSubscript(0);
        layout.setStride(std::mem::size_of::<WorldLineGpuVertex>() as _);
        layout.setStepFunction(MTLVertexStepFunction::PerVertex);
        let position_attr = attributes.objectAtIndexedSubscript(0);
        position_attr.setFormat(MTLVertexFormat::Float3);
        position_attr.setOffset(0);
        position_attr.setBufferIndex(0);
        let color_attr = attributes.objectAtIndexedSubscript(1);
        color_attr.setFormat(MTLVertexFormat::Float4);
        color_attr.setOffset(12);
        color_attr.setBufferIndex(0);
    }
    descriptor
}

//#endregion 🎨️VertexDescriptors

//#region 🖇️PipelineBuilder

type Device = ProtocolObject<dyn MTLDevice>;
type Library = ProtocolObject<dyn MTLLibrary>;

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn compile_library(device: &Device, label: &str, source: &str) -> Retained<Library> {
    let source_ns = objc2_foundation::NSString::from_str(source);
    device.newLibraryWithSource_options_error(&source_ns, None).unwrap_or_else(|error| panic!("metal backend: {label} failed to compile: {error:?}"))
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn function(library: &Library, name: &objc2_foundation::NSString) -> Retained<ProtocolObject<dyn objc2_metal::MTLFunction>> {
    library.newFunctionWithName(name).unwrap_or_else(|| panic!("metal backend: MSL library is missing function {name}"))
}

/// 🎨️ `blend: None` disables blending (an opaque overwrite — the wgpu target's `BlendState::REPLACE`
/// has no distinct Metal equivalent since Metal has no "replace" blend factor combination baked in;
/// disabling blending entirely produces the identical `dst = src` result). `Some((src, dst))` enables
/// alpha blending with that factor pair on both RGB and alpha channels (mirrors
/// `wgpu::BlendState::ALPHA_BLENDING`: `SourceAlpha` / `OneMinusSourceAlpha`, op `Add`).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn configure_color_attachment(descriptor: &MTLRenderPipelineDescriptor, pixel_format: MTLPixelFormat, blend: Option<(MTLBlendFactor, MTLBlendFactor)>, write_mask: MTLColorWriteMask) {
    let attachment = unsafe { descriptor.colorAttachments().objectAtIndexedSubscript(0) };
    attachment.setPixelFormat(pixel_format);
    attachment.setWriteMask(write_mask);
    match blend {
        Some((source, destination)) => {
            attachment.setBlendingEnabled(true);
            attachment.setSourceRGBBlendFactor(source);
            attachment.setDestinationRGBBlendFactor(destination);
            attachment.setRgbBlendOperation(MTLBlendOperation::Add);
            attachment.setSourceAlphaBlendFactor(source);
            attachment.setDestinationAlphaBlendFactor(destination);
            attachment.setAlphaBlendOperation(MTLBlendOperation::Add);
        }
        None => attachment.setBlendingEnabled(false),
    }
}

const ALPHA_BLEND: Option<(MTLBlendFactor, MTLBlendFactor)> = Some((MTLBlendFactor::SourceAlpha, MTLBlendFactor::OneMinusSourceAlpha));

//#endregion 🖇️PipelineBuilder

//#region 📦️Pipelines

/// 📦️ Every compiled pipeline/depth-stencil/sampler state this backend replays batches through. Built
/// once in `MetalBackend::new` against the swapchain's pixel format and the offscreen scene target's
/// pixel format (the two color targets in play — see `🦀️scene_target.rs`).
pub struct Pipelines {
    pub ui_mask: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    pub ui_content: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    pub vector: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    pub glass: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    pub blur_downsample: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    pub scene_blit: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    pub world3d_mesh_opaque: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    pub world3d_mesh_translucent: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    pub world3d_line: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,

    /// 🩹️ `Always`/`Replace`/`Replace`, write mask `0xff` — stamps the silhouette mask. Mirrors
    /// `mask_stencil_state()`.
    pub mask_depth_stencil: Retained<ProtocolObject<dyn MTLDepthStencilState>>,
    /// 🔒 `Equal`/`Keep`, read mask `0xff` write mask `0x00` — clips UI/vector content against a
    /// previously written mask. Mirrors `content_stencil_state()`; shared by `ui_content` and
    /// `vector`.
    pub content_depth_stencil: Retained<ProtocolObject<dyn MTLDepthStencilState>>,
    /// 🗻️ Opaque world mesh: depth write on, `Less`.
    pub world3d_opaque_depth_stencil: Retained<ProtocolObject<dyn MTLDepthStencilState>>,
    /// 🫧️ Translucent world mesh *and* lines: depth write off, `LessEqual` — identical depth/stencil
    /// behaviour in Metal (depth bias is encoder state, not part of this object; see this file's
    /// header), so the two wgpu-target states collapse onto one here.
    pub world3d_translucent_depth_stencil: Retained<ProtocolObject<dyn MTLDepthStencilState>>,

    pub glyph_sampler: Retained<ProtocolObject<dyn MTLSamplerState>>,
    pub icon_sampler: Retained<ProtocolObject<dyn MTLSamplerState>>,
    pub scene_sampler: Retained<ProtocolObject<dyn MTLSamplerState>>,
}

impl Pipelines {
    /// 🏗️ `surface_format` is the swapchain's pixel format (`scene_blit`, `glass` target it);
    /// `scene_format` is the offscreen scene color target's format (`blur_downsample` targets it, and
    /// every 2D/3D content pipeline targets it too, mirroring `render_scene_content` rendering into
    /// `scene_view` before `composite_to_swapchain` blits to the real swapchain view).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(device: &Device, surface_format: MTLPixelFormat, scene_format: MTLPixelFormat) -> Self {
        let ui_library = compile_library(device, "ui_shader", UI_SHADER_MSL);
        let vector_library = compile_library(device, "vector_shader", VECTOR_SHADER_MSL);
        let world_mesh_library = compile_library(device, "world3d_mesh_shader", WORLD3D_MESH_SHADER_MSL);
        let world_line_library = compile_library(device, "world3d_lines_shader", WORLD3D_LINES_SHADER_MSL);
        let blur_library = compile_library(device, "blur_downsample_shader", BLUR_DOWNSAMPLE_SHADER_MSL);
        let blit_library = compile_library(device, "scene_blit_shader", SCENE_BLIT_SHADER_MSL);
        let glass_library = compile_library(device, "glass_shader", GLASS_SHADER_MSL);

        let ui_vs = function(&ui_library, ns_string!("ui_vertex_main"));
        let ui_fs = function(&ui_library, ns_string!("ui_fragment_main"));
        let ui_layout = ui_vertex_descriptor();

        let mask_descriptor = MTLRenderPipelineDescriptor::new();
        mask_descriptor.setLabel(Some(ns_string!("silhouette_mask_pipeline")));
        mask_descriptor.setVertexFunction(Some(&ui_vs));
        mask_descriptor.setFragmentFunction(Some(&ui_fs));
        mask_descriptor.setVertexDescriptor(Some(&ui_layout));
        configure_color_attachment(&mask_descriptor, scene_format, None, MTLColorWriteMask::None);
        mask_descriptor.setDepthAttachmentPixelFormat(DEPTH_STENCIL_FORMAT);
        mask_descriptor.setStencilAttachmentPixelFormat(DEPTH_STENCIL_FORMAT);
        let ui_mask = device.newRenderPipelineStateWithDescriptor_error(&mask_descriptor).expect("metal backend: silhouette_mask_pipeline failed to link");

        let content_descriptor = MTLRenderPipelineDescriptor::new();
        content_descriptor.setLabel(Some(ns_string!("ui_pipeline")));
        content_descriptor.setVertexFunction(Some(&ui_vs));
        content_descriptor.setFragmentFunction(Some(&ui_fs));
        content_descriptor.setVertexDescriptor(Some(&ui_layout));
        configure_color_attachment(&content_descriptor, scene_format, ALPHA_BLEND, MTLColorWriteMask::All);
        content_descriptor.setDepthAttachmentPixelFormat(DEPTH_STENCIL_FORMAT);
        content_descriptor.setStencilAttachmentPixelFormat(DEPTH_STENCIL_FORMAT);
        let ui_content = device.newRenderPipelineStateWithDescriptor_error(&content_descriptor).expect("metal backend: ui_pipeline failed to link");

        let vector_vs = function(&vector_library, ns_string!("vector_vertex_main"));
        let vector_fs = function(&vector_library, ns_string!("vector_fragment_main"));
        let vector_descriptor = MTLRenderPipelineDescriptor::new();
        vector_descriptor.setLabel(Some(ns_string!("vector_pipeline")));
        vector_descriptor.setVertexFunction(Some(&vector_vs));
        vector_descriptor.setFragmentFunction(Some(&vector_fs));
        vector_descriptor.setVertexDescriptor(Some(&vector_vertex_descriptor()));
        configure_color_attachment(&vector_descriptor, scene_format, ALPHA_BLEND, MTLColorWriteMask::All);
        vector_descriptor.setDepthAttachmentPixelFormat(DEPTH_STENCIL_FORMAT);
        vector_descriptor.setStencilAttachmentPixelFormat(DEPTH_STENCIL_FORMAT);
        let vector = device.newRenderPipelineStateWithDescriptor_error(&vector_descriptor).expect("metal backend: vector_pipeline failed to link");

        let glass_vs = function(&glass_library, ns_string!("glass_vertex_main"));
        let glass_fs = function(&glass_library, ns_string!("glass_fragment_main"));
        let glass_descriptor = MTLRenderPipelineDescriptor::new();
        glass_descriptor.setLabel(Some(ns_string!("glass_pipeline")));
        glass_descriptor.setVertexFunction(Some(&glass_vs));
        glass_descriptor.setFragmentFunction(Some(&glass_fs));
        glass_descriptor.setVertexDescriptor(Some(&glass_vertex_descriptor()));
        configure_color_attachment(&glass_descriptor, surface_format, ALPHA_BLEND, MTLColorWriteMask::All);
        let glass = device.newRenderPipelineStateWithDescriptor_error(&glass_descriptor).expect("metal backend: glass_pipeline failed to link");

        let blur_vs = function(&blur_library, ns_string!("blur_downsample_vertex_main"));
        let blur_fs = function(&blur_library, ns_string!("blur_downsample_fragment_main"));
        let blur_descriptor = MTLRenderPipelineDescriptor::new();
        blur_descriptor.setLabel(Some(ns_string!("blur_downsample_pipeline")));
        blur_descriptor.setVertexFunction(Some(&blur_vs));
        blur_descriptor.setFragmentFunction(Some(&blur_fs));
        configure_color_attachment(&blur_descriptor, scene_format, None, MTLColorWriteMask::All);
        let blur_downsample = device.newRenderPipelineStateWithDescriptor_error(&blur_descriptor).expect("metal backend: blur_downsample_pipeline failed to link");

        let blit_vs = function(&blit_library, ns_string!("scene_blit_vertex_main"));
        let blit_fs = function(&blit_library, ns_string!("scene_blit_fragment_main"));
        let blit_descriptor = MTLRenderPipelineDescriptor::new();
        blit_descriptor.setLabel(Some(ns_string!("scene_blit_pipeline")));
        blit_descriptor.setVertexFunction(Some(&blit_vs));
        blit_descriptor.setFragmentFunction(Some(&blit_fs));
        configure_color_attachment(&blit_descriptor, surface_format, None, MTLColorWriteMask::All);
        let scene_blit = device.newRenderPipelineStateWithDescriptor_error(&blit_descriptor).expect("metal backend: scene_blit_pipeline failed to link");

        let world_mesh_vs = function(&world_mesh_library, ns_string!("world3d_mesh_vertex_main"));
        let world_mesh_fs = function(&world_mesh_library, ns_string!("world3d_mesh_fragment_main"));
        let world_mesh_layout = world3d_mesh_vertex_descriptor();

        let world_opaque_descriptor = MTLRenderPipelineDescriptor::new();
        world_opaque_descriptor.setLabel(Some(ns_string!("world3d_pipeline")));
        world_opaque_descriptor.setVertexFunction(Some(&world_mesh_vs));
        world_opaque_descriptor.setFragmentFunction(Some(&world_mesh_fs));
        world_opaque_descriptor.setVertexDescriptor(Some(&world_mesh_layout));
        configure_color_attachment(&world_opaque_descriptor, scene_format, None, MTLColorWriteMask::All);
        world_opaque_descriptor.setDepthAttachmentPixelFormat(DEPTH_STENCIL_FORMAT);
        world_opaque_descriptor.setStencilAttachmentPixelFormat(DEPTH_STENCIL_FORMAT);
        let world3d_mesh_opaque = device.newRenderPipelineStateWithDescriptor_error(&world_opaque_descriptor).expect("metal backend: world3d_pipeline failed to link");

        let world_translucent_descriptor = MTLRenderPipelineDescriptor::new();
        world_translucent_descriptor.setLabel(Some(ns_string!("world3d_pipeline_translucent")));
        world_translucent_descriptor.setVertexFunction(Some(&world_mesh_vs));
        world_translucent_descriptor.setFragmentFunction(Some(&world_mesh_fs));
        world_translucent_descriptor.setVertexDescriptor(Some(&world_mesh_layout));
        configure_color_attachment(&world_translucent_descriptor, scene_format, ALPHA_BLEND, MTLColorWriteMask::All);
        world_translucent_descriptor.setDepthAttachmentPixelFormat(DEPTH_STENCIL_FORMAT);
        world_translucent_descriptor.setStencilAttachmentPixelFormat(DEPTH_STENCIL_FORMAT);
        let world3d_mesh_translucent = device.newRenderPipelineStateWithDescriptor_error(&world_translucent_descriptor).expect("metal backend: world3d_pipeline_translucent failed to link");

        let world_line_vs = function(&world_line_library, ns_string!("world3d_line_vertex_main"));
        let world_line_fs = function(&world_line_library, ns_string!("world3d_line_fragment_main"));
        let world_line_descriptor = MTLRenderPipelineDescriptor::new();
        world_line_descriptor.setLabel(Some(ns_string!("world3d_line_pipeline")));
        world_line_descriptor.setVertexFunction(Some(&world_line_vs));
        world_line_descriptor.setFragmentFunction(Some(&world_line_fs));
        world_line_descriptor.setVertexDescriptor(Some(&world3d_line_vertex_descriptor()));
        configure_color_attachment(&world_line_descriptor, scene_format, ALPHA_BLEND, MTLColorWriteMask::All);
        world_line_descriptor.setDepthAttachmentPixelFormat(DEPTH_STENCIL_FORMAT);
        world_line_descriptor.setStencilAttachmentPixelFormat(DEPTH_STENCIL_FORMAT);
        let world3d_line = device.newRenderPipelineStateWithDescriptor_error(&world_line_descriptor).expect("metal backend: world3d_line_pipeline failed to link");

        //#region DepthStencilStates

        let mask_face = MTLStencilDescriptor::new();
        mask_face.setStencilCompareFunction(MTLCompareFunction::Always);
        mask_face.setStencilFailureOperation(MTLStencilOperation::Replace);
        mask_face.setDepthFailureOperation(MTLStencilOperation::Replace);
        mask_face.setDepthStencilPassOperation(MTLStencilOperation::Replace);
        mask_face.setReadMask(0xff);
        mask_face.setWriteMask(0xff);
        let mask_ds_descriptor = MTLDepthStencilDescriptor::new();
        mask_ds_descriptor.setDepthCompareFunction(MTLCompareFunction::Always);
        mask_ds_descriptor.setDepthWriteEnabled(false);
        mask_ds_descriptor.setFrontFaceStencil(Some(&mask_face));
        mask_ds_descriptor.setBackFaceStencil(Some(&mask_face));
        let mask_depth_stencil = device.newDepthStencilStateWithDescriptor(&mask_ds_descriptor).expect("metal backend: mask depth-stencil state failed");

        let content_face = MTLStencilDescriptor::new();
        content_face.setStencilCompareFunction(MTLCompareFunction::Equal);
        content_face.setStencilFailureOperation(MTLStencilOperation::Keep);
        content_face.setDepthFailureOperation(MTLStencilOperation::Keep);
        content_face.setDepthStencilPassOperation(MTLStencilOperation::Keep);
        content_face.setReadMask(0xff);
        content_face.setWriteMask(0x00);
        let content_ds_descriptor = MTLDepthStencilDescriptor::new();
        content_ds_descriptor.setDepthCompareFunction(MTLCompareFunction::Always);
        content_ds_descriptor.setDepthWriteEnabled(false);
        content_ds_descriptor.setFrontFaceStencil(Some(&content_face));
        content_ds_descriptor.setBackFaceStencil(Some(&content_face));
        let content_depth_stencil = device.newDepthStencilStateWithDescriptor(&content_ds_descriptor).expect("metal backend: content depth-stencil state failed");

        let world_content_face = MTLStencilDescriptor::new();
        world_content_face.setStencilCompareFunction(MTLCompareFunction::Equal);
        world_content_face.setStencilFailureOperation(MTLStencilOperation::Keep);
        world_content_face.setDepthFailureOperation(MTLStencilOperation::Keep);
        world_content_face.setDepthStencilPassOperation(MTLStencilOperation::Keep);
        world_content_face.setReadMask(0xff);
        world_content_face.setWriteMask(0x00);

        let world_opaque_ds_descriptor = MTLDepthStencilDescriptor::new();
        world_opaque_ds_descriptor.setDepthCompareFunction(MTLCompareFunction::Less);
        world_opaque_ds_descriptor.setDepthWriteEnabled(true);
        world_opaque_ds_descriptor.setFrontFaceStencil(Some(&world_content_face));
        world_opaque_ds_descriptor.setBackFaceStencil(Some(&world_content_face));
        let world3d_opaque_depth_stencil = device.newDepthStencilStateWithDescriptor(&world_opaque_ds_descriptor).expect("metal backend: world3d opaque depth-stencil state failed");

        let world_translucent_ds_descriptor = MTLDepthStencilDescriptor::new();
        world_translucent_ds_descriptor.setDepthCompareFunction(MTLCompareFunction::LessEqual);
        world_translucent_ds_descriptor.setDepthWriteEnabled(false);
        world_translucent_ds_descriptor.setFrontFaceStencil(Some(&world_content_face));
        world_translucent_ds_descriptor.setBackFaceStencil(Some(&world_content_face));
        let world3d_translucent_depth_stencil = device.newDepthStencilStateWithDescriptor(&world_translucent_ds_descriptor).expect("metal backend: world3d translucent depth-stencil state failed");

        //#endregion DepthStencilStates

        //#region Samplers

        let glyph_sampler_descriptor = MTLSamplerDescriptor::new();
        glyph_sampler_descriptor.setMinFilter(MTLSamplerMinMagFilter::Linear);
        glyph_sampler_descriptor.setMagFilter(MTLSamplerMinMagFilter::Linear);
        glyph_sampler_descriptor.setMipFilter(MTLSamplerMipFilter::NotMipmapped);
        glyph_sampler_descriptor.setSAddressMode(MTLSamplerAddressMode::ClampToEdge);
        glyph_sampler_descriptor.setTAddressMode(MTLSamplerAddressMode::ClampToEdge);
        let glyph_sampler = device.newSamplerStateWithDescriptor(&glyph_sampler_descriptor).expect("metal backend: glyph sampler failed");

        let icon_sampler_descriptor = MTLSamplerDescriptor::new();
        icon_sampler_descriptor.setMinFilter(MTLSamplerMinMagFilter::Linear);
        icon_sampler_descriptor.setMagFilter(MTLSamplerMinMagFilter::Linear);
        icon_sampler_descriptor.setMipFilter(MTLSamplerMipFilter::NotMipmapped);
        icon_sampler_descriptor.setSAddressMode(MTLSamplerAddressMode::ClampToEdge);
        icon_sampler_descriptor.setTAddressMode(MTLSamplerAddressMode::ClampToEdge);
        let icon_sampler = device.newSamplerStateWithDescriptor(&icon_sampler_descriptor).expect("metal backend: icon sampler failed");

        let scene_sampler_descriptor = MTLSamplerDescriptor::new();
        scene_sampler_descriptor.setMinFilter(MTLSamplerMinMagFilter::Linear);
        scene_sampler_descriptor.setMagFilter(MTLSamplerMinMagFilter::Linear);
        scene_sampler_descriptor.setMipFilter(MTLSamplerMipFilter::Linear);
        scene_sampler_descriptor.setSAddressMode(MTLSamplerAddressMode::ClampToEdge);
        scene_sampler_descriptor.setTAddressMode(MTLSamplerAddressMode::ClampToEdge);
        let scene_sampler = device.newSamplerStateWithDescriptor(&scene_sampler_descriptor).expect("metal backend: scene sampler failed");

        //#endregion Samplers

        Self {
            ui_mask,
            ui_content,
            vector,
            glass,
            blur_downsample,
            scene_blit,
            world3d_mesh_opaque,
            world3d_mesh_translucent,
            world3d_line,
            mask_depth_stencil,
            content_depth_stencil,
            world3d_opaque_depth_stencil,
            world3d_translucent_depth_stencil,
            glyph_sampler,
            icon_sampler,
            scene_sampler,
        }
    }
}

//#endregion 📦️Pipelines

//#endregion 🔖️Pipelines
