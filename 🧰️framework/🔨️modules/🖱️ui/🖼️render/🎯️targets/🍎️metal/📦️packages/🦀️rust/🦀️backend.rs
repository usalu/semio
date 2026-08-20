//! @emoji 🍎️ `MetalBackend`: the concrete `ui_render::GraphicsBackend` implementation for macOS.
//!
//! Milestones reached (see `📓️terra-backend-metal-report.md` for the authoritative statement):
//! device + `CAMetalLayer` + swapchain (resize incl. zero-size park/restore, `nextDrawable`, present,
//! clear-colour frame); UI quads + vector pipelines with glyph/icon atlas + raster texture upload and
//! per-batch scissor; stencil silhouette clip (mask/content pass pair, `mask_range` replayed
//! verbatim); offscreen scene target → mip blur → glass composite → blit; world3d mesh/lines (see
//! `🦀️world3d.rs`'s header for the one documented interleaving limitation — textured mesh skipped, it
//! is unwired dead surface in the reference too); device loss bookkeeping + `backend-testing`
//! (`debug_force_device_loss`/`recover`/`read_back`).
//!
//! `render` replays `RenderPacket` in two device passes, mirroring `GpuContext::render_frame`
//! (`🎯️targets/🧊️wgpu/🦀️gpu.rs`): an offscreen pass into `SceneTarget` (2D backdrop content, then
//! every `SurfacePass`, then 2D backdrop overlay content — see `🦀️world3d.rs` for why world3d is not
//! interleaved layer-by-layer), then a composite pass that blurs the scene's mip chain, blits it to
//! the real swapchain drawable, composites glass regions on top, and finally paints foreground
//! (glass-content) 2D batches directly onto the swapchain view.

#[cfg(not(target_os = "macos"))]
compile_error!("semio-framework-ui-backend-metal builds only on macOS.");

use crate::frame_buffers::FrameBuffers;
use crate::pipelines::{Pipelines, DEPTH_STENCIL_FORMAT};
use crate::resources::GpuResources;
use crate::scene_target::SceneTarget;
use crate::world3d::WorldGlobalsRing;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2_metal::{MTLBlitCommandEncoder, MTLBuffer, MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue, MTLDevice, MTLLoadAction, MTLOrigin, MTLPixelFormat, MTLPrimitiveType, MTLRenderCommandEncoder, MTLRenderPassDescriptor, MTLStoreAction, MTLTexture};
use objc2_quartz_core::{CAMetalDrawable, CAMetalLayer};
use raw_window_handle::{RawWindowHandle, WindowHandle};
use ui_render::{BackendError, DeviceCapabilities, DeviceStatus, DrawBatch, FrameStats, GpuTier, GraphicsBackend, LossReason, MemoryClass, PhysicalSize, PipelineKind, RecoveredResources, RenderPacket, RenderReport, ResourceKind, ResourceOp, SurfaceFormat, TextureId};

#[cfg(feature = "backend-testing")]
use ui_render::ReadbackImage;

// 🔓️ SAFETY: `MTLCreateSystemDefaultDevice` requires the process to link `CoreGraphics.framework`
// (documented on `objc2_metal::MTLCreateSystemDefaultDevice`); this empty `extern` block with a
// `#[link]` attribute is the linker directive that satisfies it without adding an `objc2-core-graphics`
// dependency this crate does not otherwise need.
#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

//#region 🔖️Backend

type Device = ProtocolObject<dyn MTLDevice>;
type Queue = ProtocolObject<dyn MTLCommandQueue>;
type MetalTexture = ProtocolObject<dyn MTLTexture>;
type CommandBuffer = ProtocolObject<dyn MTLCommandBuffer>;
type Encoder = ProtocolObject<dyn MTLRenderCommandEncoder>;
type MetalBuffer = ProtocolObject<dyn MTLBuffer>;

//#region ⚠️MetalGraphicsError

/// ⚠️ This crate's internal failure set — richer than `ui_render::BackendError` needs at the trait
/// boundary, so `MetalBackend`'s trait methods map every variant onto the closest `BackendError` one
/// (see `From` below) rather than growing the contract's own error enum for Metal-specific detail.
#[derive(Debug)]
pub enum MetalGraphicsError {
    AllocationFailed(String),
    UnsupportedAtlasChannels(u32),
    ShaderCompilationFailed(String),
}

impl From<MetalGraphicsError> for BackendError {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(error: MetalGraphicsError) -> Self {
        match error {
            MetalGraphicsError::AllocationFailed(_) => BackendError::OutOfMemory,
            MetalGraphicsError::UnsupportedAtlasChannels(_) => BackendError::UnsupportedFormat("atlas upload byte density must be 1 (R8) or 4 (RGBA8) bytes/pixel"),
            MetalGraphicsError::ShaderCompilationFailed(message) => BackendError::ShaderCompilationFailed(message),
        }
    }
}

//#endregion ⚠️MetalGraphicsError

//#region 🍎️MetalBackend

/// 🍎️ The concrete macOS `GraphicsBackend`. Every Metal/objc2 type stays behind this crate's own
/// interface — nothing here appears in a public signature outside this crate (CLAUDE.md's "external
/// libraries behind an interface" rule).
pub struct MetalBackend {
    device: Retained<Device>,
    queue: Retained<Queue>,
    layer: Retained<CAMetalLayer>,
    size: PhysicalSize,
    dpr: f32,
    surface_format: MTLPixelFormat,
    pipelines: Pipelines,
    resources: GpuResources,
    scene_target: SceneTarget,
    depth_texture: Option<Retained<MetalTexture>>,
    world_ring: WorldGlobalsRing,
    frame_buffers: FrameBuffers,
    status: DeviceStatus,
    /// 📐️ The static 6-corner unit-quad `buffer(0)` every UI/vector/glass pipeline's vertex function
    /// reads — created once, never rewritten.
    quad_vertex_buffer: Retained<MetalBuffer>,
    /// 🌐️ The tiny `{screen_size, _pad}` uniform UI/vector/glass share at `buffer(2)` — rewritten via
    /// its `Shared`-storage `contents()` pointer once per frame by `update_globals`.
    ui_globals_buffer: Retained<MetalBuffer>,
    #[cfg(feature = "backend-testing")]
    readback: Option<Retained<MetalTexture>>,
}

//#region Construction

/// 🖼️ `BGRA8Unorm_sRGB` is `CAMetalLayer`'s conventional swapchain format (matches the wgpu target's
/// preference for an sRGB surface format when one is available).
const SURFACE_FORMAT: MTLPixelFormat = MTLPixelFormat::BGRA8Unorm_sRGB;
/// 🌫️ The offscreen scene target's format — plain (non-sRGB) `BGRA8Unorm` because the scene target is
/// read back through `sample()`/blit, not presented directly; sRGB conversion happens once, at the
/// final `scene_blit_pipeline` write into the sRGB swapchain view.
const SCENE_FORMAT: MTLPixelFormat = MTLPixelFormat::BGRA8Unorm;

// 🔓️ SAFETY (module-wide note for every `objc2::msg_send!` call below — only `setWantsLayer:`/
// `setLayer:` in `new` remain; `setDrawableSize:` no longer needs one, see `set_drawable_size`):
// each call targets a plain AppKit `NSView` selector with no return value, on a pointer this crate
// received from a `raw_window_handle::AppKitWindowHandle` (whose contract guarantees a live `NSView`)
// — never a dangling or type-mismatched receiver.
impl MetalBackend {
    /// 🏗️ Builds a device, command queue, and a `CAMetalLayer` attached to `window_handle`'s
    /// `NSView`. Only construction is async per U1 — the body below performs no real `.await` because
    /// Metal's device/queue/layer creation is synchronous, unlike wgpu's adapter/device request.
    // 🚫️async: U1 — the ONE permitted async fn per the `GraphicsBackend` docstring; construction only.
    pub async fn new(window_handle: WindowHandle<'_>, size: PhysicalSize, dpr: f32) -> Result<Self, BackendError> {
        let RawWindowHandle::AppKit(handle) = window_handle.as_raw() else {
            return Err(BackendError::UnsupportedFormat("metal backend requires an AppKit window handle"));
        };
        let (device, queue, layer) = create_device_queue_layer(size, dpr)?;
        // 🔓️ SAFETY: `handle.ns_view` is guaranteed by `raw_window_handle::AppKitWindowHandle`'s own
        // contract to be a valid, live `NSView*` for the lifetime of `window_handle`; `setWantsLayer:`/
        // `setLayer:` are ordinary AppKit calls with no return value, called on the main thread (the
        // same thread constraint `raw_window_handle` documents for `AppKitWindowHandle` itself).
        unsafe {
            let view: &AnyObject = &*handle.ns_view.as_ptr().cast::<AnyObject>();
            let _: () = objc2::msg_send![view, setWantsLayer: true];
            let _: () = objc2::msg_send![view, setLayer: &*layer];
        }
        Ok(Self::from_parts(device, queue, layer, size, dpr))
    }

    /// 🧪️ A `CAMetalLayer` not attached to any view — Metal drawables work independent of window
    /// presence, so this exercises the full render pipeline for `backend-testing` without an AppKit
    /// window. Never used outside tests/the conformance harness.
    #[cfg(feature = "backend-testing")]
    // 🚫️async: U1 — the ONE permitted async fn per the `GraphicsBackend` docstring; construction only.
    pub async fn new_headless(size: PhysicalSize, dpr: f32) -> Result<Self, BackendError> {
        let (device, queue, layer) = create_device_queue_layer(size, dpr)?;
        Ok(Self::from_parts(device, queue, layer, size, dpr))
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from_parts(device: Retained<Device>, queue: Retained<Queue>, layer: Retained<CAMetalLayer>, size: PhysicalSize, dpr: f32) -> Self {
        let pipelines = Pipelines::new(&device, SURFACE_FORMAT, SCENE_FORMAT);
        let resources = GpuResources::new(&device);
        let scene_target = SceneTarget::new(&device, size.width.max(1), size.height.max(1), SCENE_FORMAT);
        let depth_texture = Some(allocate_depth_texture(&device, size.width.max(1), size.height.max(1)));
        let quad_vertex_buffer = crate::resources::new_buffer_with_bytes(&device, bytemuck::cast_slice(&crate::types::UNIT_QUAD_CORNERS), "unit_quad_corners").expect("metal backend: failed to allocate the static unit-quad buffer");
        let ui_globals_buffer = device.newBufferWithLength_options(16, objc2_metal::MTLResourceOptions::StorageModeShared).expect("metal backend: failed to allocate the UI globals buffer");
        Self {
            device,
            queue,
            layer,
            size,
            dpr,
            surface_format: SURFACE_FORMAT,
            pipelines,
            resources,
            scene_target,
            depth_texture,
            world_ring: WorldGlobalsRing::default(),
            frame_buffers: FrameBuffers::default(),
            status: DeviceStatus::Healthy,
            quad_vertex_buffer,
            ui_globals_buffer,
            #[cfg(feature = "backend-testing")]
            readback: None,
        }
    }

    /// ✍️ Rewrites the shared `{screen_size, _pad}` uniform (`_pad.x` carries elapsed seconds, read by
    /// the UI megashader's animated border kinds — mirrors `UiGlobals`/`update_globals` in the wgpu
    /// target).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn update_globals(&self, width: f32, height: f32, time_seconds: f32) {
        let values: [f32; 4] = [width, height, time_seconds, 0.0];
        // 🔓️ SAFETY: `ui_globals_buffer` is a 16-byte `Shared`-storage buffer allocated in
        // `from_parts` and never resized; `values` is exactly 16 bytes, so this copy never writes past
        // the allocation. `Shared` storage needs no explicit flush for the GPU to observe the write.
        unsafe {
            let destination = self.ui_globals_buffer.contents();
            std::ptr::copy_nonoverlapping(values.as_ptr().cast::<u8>(), destination.as_ptr().cast::<u8>(), 16);
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn validate_known_resources(&self, packet: &RenderPacket) -> Result<(), BackendError> {
        for batch in &packet.batches {
            if let Some(texture) = batch.texture {
                if !self.resources.knows_texture(texture) {
                    return Err(BackendError::UnknownResource(ResourceKind::Texture));
                }
            }
        }
        for pass in &packet.surface_passes {
            for draw in pass.draws.iter().chain(pass.translucent_draws.iter()) {
                if !self.resources.knows_mesh(draw.mesh) {
                    return Err(BackendError::UnknownResource(ResourceKind::Mesh));
                }
            }
        }
        Ok(())
    }
}

/// 🏗️ Shared by `new`/`new_headless`: device, queue, and a fully-configured (but not-yet-attached)
/// `CAMetalLayer`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn create_device_queue_layer(size: PhysicalSize, _dpr: f32) -> Result<(Retained<Device>, Retained<Queue>, Retained<CAMetalLayer>), BackendError> {
    let device = objc2_metal::MTLCreateSystemDefaultDevice().ok_or(BackendError::DeviceLost(LossReason::Device))?;
    let queue = device.newCommandQueue().ok_or(BackendError::DeviceLost(LossReason::Device))?;
    let layer = CAMetalLayer::new();
    layer.setDevice(Some(&device));
    layer.setPixelFormat(SURFACE_FORMAT);
    layer.setFramebufferOnly(true);
    set_drawable_size(&layer, size.width.max(1), size.height.max(1));
    Ok((device, queue, layer))
}

/// 📐️ `CAMetalLayer::setDrawableSize` is a plain safe method taking `objc2_core_foundation::CGSize`
/// by value (confirmed in `objc2-quartz-core-0.3.2/src/generated/CAMetalLayer.rs:105-107`:
/// `pub fn setDrawableSize(&self, drawable_size: CGSize);`, no `unsafe` marker) — no `msg_send!` needed
/// at all once the real typed `CGSize` is available (`objc2-core-foundation-0.3.2/src/geometry.rs:112`:
/// `pub struct CGSize { pub width: CGFloat, pub height: CGFloat }`, `CGFloat = f64` on 64-bit targets,
/// `Encode`/`RefEncode` implemented under its default-on `"objc2"` feature).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn set_drawable_size(layer: &CAMetalLayer, width: u32, height: u32) {
    layer.setDrawableSize(objc2_core_foundation::CGSize::new(width as f64, height as f64));
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn allocate_depth_texture(device: &Device, width: u32, height: u32) -> Retained<MetalTexture> {
    let descriptor = objc2_metal::MTLTextureDescriptor::new();
    descriptor.setPixelFormat(DEPTH_STENCIL_FORMAT);
    unsafe {
        descriptor.setWidth(width.max(1) as _);
        descriptor.setHeight(height.max(1) as _);
    }
    descriptor.setUsage(objc2_metal::MTLTextureUsage::RenderTarget);
    descriptor.setResourceOptions(objc2_metal::MTLResourceOptions::StorageModePrivate);
    device.newTextureWithDescriptor(&descriptor).expect("metal backend: failed to allocate depth/stencil texture")
}

//#endregion Construction

//#region Rendering

impl MetalBackend {
    /// 🎬️ Pass 1 — mirrors `render_scene_content`: draws every backdrop 2D batch, then every
    /// `SurfacePass` (see `🦀️world3d.rs`'s header for the interleaving limitation), then every
    /// backdrop-overlay 2D batch, all into `scene_target` (mip 0) + `depth_texture`, cleared once at
    /// pass start. One encoder for the whole pass — see this file's header for why a single encoder
    /// suffices where the wgpu target used several (its per-group buffer re-collection, not a Metal
    /// requirement; `🦀️frame_buffers.rs`'s header has the full reasoning).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn encode_scene_pass(&mut self, command_buffer: &CommandBuffer, packet: &RenderPacket, time_seconds: f32) {
        let width = self.size.width as f32;
        let height = self.size.height as f32;
        self.update_globals(width, height, time_seconds);
        self.frame_buffers.quad_instances.upload(&self.device, bytemuck::cast_slice(&packet.quad_instances));
        self.frame_buffers.vector_vertices.upload(&self.device, bytemuck::cast_slice(&packet.vector_vertices));
        crate::world3d::upload_world_passes(&self.device, &mut self.world_ring, &mut self.frame_buffers, &packet.surface_passes);

        let descriptor = make_render_pass_descriptor(self.scene_target.texture(), 0, MTLLoadAction::Clear, Some((0.05, 0.05, 0.06, 1.0)), self.depth_texture.as_deref(), MTLLoadAction::Clear);
        let Some(encoder) = command_buffer.renderCommandEncoderWithDescriptor(&descriptor) else { return };
        encoder.setScissorRect(objc2_metal::MTLScissorRect { x: 0, y: 0, width: width as usize, height: height as usize });

        let quad_buffer = self.frame_buffers.quad_instances.buffer();
        let vector_buffer = self.frame_buffers.vector_vertices.buffer();

        let backdrop_normal = packet.batches.iter().filter(|batch| batch.pipeline != PipelineKind::Glass && batch.layer_state.foreground_of.is_none() && !batch.layer_state.overlay);
        encode_2d_batches(&encoder, &self.pipelines, &self.resources, &self.quad_vertex_buffer, &self.ui_globals_buffer, quad_buffer, vector_buffer, backdrop_normal, width, height);

        crate::world3d::encode_passes(&encoder, &self.pipelines, &self.resources, &self.world_ring, &self.frame_buffers, &packet.surface_passes, width, height);

        let backdrop_overlay = packet.batches.iter().filter(|batch| batch.pipeline != PipelineKind::Glass && batch.layer_state.foreground_of.is_none() && batch.layer_state.overlay);
        encode_2d_batches(&encoder, &self.pipelines, &self.resources, &self.quad_vertex_buffer, &self.ui_globals_buffer, quad_buffer, vector_buffer, backdrop_overlay, width, height);

        encoder.endEncoding();
    }

    /// 🎬️ Pass 2 — mirrors `composite_to_swapchain`: blur the scene's mip chain, blit it to the real
    /// swapchain drawable, composite every glass region on top, then paint glass-foreground 2D content
    /// (normal then overlay) directly onto the drawable, reusing `depth_texture` with `Load` ops so it
    /// still clips against the same stencil silhouettes the offscreen pass wrote.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn encode_composite_pass(&mut self, command_buffer: &CommandBuffer, drawable_texture: &MetalTexture, packet: &RenderPacket) {
        let width = self.size.width as f32;
        let height = self.size.height as f32;

        self.run_blur_chain(command_buffer);
        self.blit_scene_to_drawable(command_buffer, drawable_texture);
        self.composite_glass(command_buffer, drawable_texture, packet);

        let has_foreground = packet.batches.iter().any(|batch| batch.layer_state.foreground_of.is_some());
        if !has_foreground {
            return;
        }
        let descriptor = make_render_pass_descriptor(drawable_texture, 0, MTLLoadAction::Load, None, self.depth_texture.as_deref(), MTLLoadAction::Load);
        let Some(encoder) = command_buffer.renderCommandEncoderWithDescriptor(&descriptor) else { return };
        encoder.setScissorRect(objc2_metal::MTLScissorRect { x: 0, y: 0, width: width as usize, height: height as usize });

        let quad_buffer = self.frame_buffers.quad_instances.buffer();
        let vector_buffer = self.frame_buffers.vector_vertices.buffer();
        let foreground_normal = packet.batches.iter().filter(|batch| batch.pipeline != PipelineKind::Glass && batch.layer_state.foreground_of.is_some() && !batch.layer_state.overlay);
        encode_2d_batches(&encoder, &self.pipelines, &self.resources, &self.quad_vertex_buffer, &self.ui_globals_buffer, quad_buffer, vector_buffer, foreground_normal, width, height);
        let foreground_overlay = packet.batches.iter().filter(|batch| batch.pipeline != PipelineKind::Glass && batch.layer_state.foreground_of.is_some() && batch.layer_state.overlay);
        encode_2d_batches(&encoder, &self.pipelines, &self.resources, &self.quad_vertex_buffer, &self.ui_globals_buffer, quad_buffer, vector_buffer, foreground_overlay, width, height);

        encoder.endEncoding();
    }

    /// 🌫️ Ports `run_blur_chain`: for each mip 1..`SCENE_MIP_LEVELS`, blit-copies the previous mip from
    /// `scene_target.texture()` into `scene_target.blur_scratch()` (Metal cannot read and write the
    /// same texture within one pass, same reasoning as the wgpu target), then renders a fullscreen
    /// 5-tap box downsample from the scratch texture into this mip. No per-mip `TextureView`s are
    /// needed on either side — see `🦀️msl.rs`'s header.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn run_blur_chain(&self, command_buffer: &CommandBuffer) {
        for mip in 1..crate::scene_target::SCENE_MIP_LEVELS {
            let src_mip = mip - 1;
            if let Some(blit_encoder) = command_buffer.blitCommandEncoder() {
                let extent = objc2_metal::MTLSize { width: (self.scene_target.width() >> src_mip).max(1) as usize, height: (self.scene_target.height() >> src_mip).max(1) as usize, depth: 1 };
                // 🔓️ SAFETY: `src_mip`/`extent` are always within `SCENE_MIP_LEVELS`/the texture's own
                // dimensions by construction (the loop bound and the `>>` shift above); both textures
                // are the same size and mip count (`SceneTarget::ensure` keeps them in lockstep), and
                // both are kept alive for the whole call by `self.scene_target`.
                unsafe {
                    blit_encoder.copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin(
                        self.scene_target.texture(),
                        0,
                        src_mip as usize,
                        MTLOrigin { x: 0, y: 0, z: 0 },
                        extent,
                        self.scene_target.blur_scratch(),
                        0,
                        src_mip as usize,
                        MTLOrigin { x: 0, y: 0, z: 0 },
                    );
                }
                blit_encoder.endEncoding();
            }
            let descriptor = make_render_pass_descriptor(self.scene_target.texture(), mip as usize, MTLLoadAction::DontCare, None, None, MTLLoadAction::DontCare);
            let Some(encoder) = command_buffer.renderCommandEncoderWithDescriptor(&descriptor) else { continue };
            encoder.setRenderPipelineState(&self.pipelines.blur_downsample);
            let src_mip_value: u32 = src_mip;
            // 🔓️ SAFETY: `src_mip_value` is a local on this frame's stack, valid and initialized for
            // the whole `setFragmentBytes_length_atIndex` call, which copies it immediately (Metal's
            // documented "bytes" fast path never retains the pointer past the call).
            unsafe {
                let pointer = std::ptr::NonNull::from(&src_mip_value).cast::<std::ffi::c_void>();
                encoder.setFragmentBytes_length_atIndex(pointer, 4, 0);
                encoder.setFragmentTexture_atIndex(Some(self.scene_target.blur_scratch()), 0);
                encoder.setFragmentSamplerState_atIndex(Some(&self.pipelines.scene_sampler), 0);
                encoder.drawPrimitives_vertexStart_vertexCount(MTLPrimitiveType::Triangle, 0, 6);
            }
            encoder.endEncoding();
        }
    }

    /// 🪟️ Ports `blit_scene_to_swapchain`: samples mip 0 of the fully-blurred scene chain into the
    /// swapchain drawable, clearing it first (this is the frame's real "clear colour" as seen by the
    /// user — the offscreen scene clear earlier is invisible, fully overwritten here).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn blit_scene_to_drawable(&self, command_buffer: &CommandBuffer, drawable_texture: &MetalTexture) {
        let descriptor = make_render_pass_descriptor(drawable_texture, 0, MTLLoadAction::Clear, Some((0.05, 0.05, 0.06, 1.0)), None, MTLLoadAction::DontCare);
        let Some(encoder) = command_buffer.renderCommandEncoderWithDescriptor(&descriptor) else { return };
        encoder.setRenderPipelineState(&self.pipelines.scene_blit);
        // 🔓️ SAFETY: `scene_target.texture()` outlives this call (owned by `self`); no bounds/lifetime
        // hazard from the raw texture/sampler bind calls themselves.
        unsafe {
            encoder.setFragmentTexture_atIndex(Some(self.scene_target.texture()), 0);
            encoder.setFragmentSamplerState_atIndex(Some(&self.pipelines.scene_sampler), 0);
            encoder.drawPrimitives_vertexStart_vertexCount(MTLPrimitiveType::Triangle, 0, 6);
        }
        encoder.endEncoding();
    }

    /// 🥂️ Ports `composite_glass_regions`: one instanced draw over every glass region in the packet.
    /// **Simplification vs. the reference's one-draw-per-region loop**: since glass has no stencil
    /// mask and every instance shares one pipeline/state, a single `instanceCount = glass_instances.len()`
    /// draw is pixel-identical to sequential per-region draws (GPUs rasterize/blend instances of one
    /// draw call in submission order, same as separate draw calls would) — one API round-trip instead
    /// of N.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn composite_glass(&mut self, command_buffer: &CommandBuffer, drawable_texture: &MetalTexture, packet: &RenderPacket) {
        if packet.glass_instances.is_empty() {
            return;
        }
        self.frame_buffers.glass_instances.upload(&self.device, bytemuck::cast_slice(&packet.glass_instances));
        let Some(glass_buffer) = self.frame_buffers.glass_instances.buffer() else { return };
        let descriptor = make_render_pass_descriptor(drawable_texture, 0, MTLLoadAction::Load, None, None, MTLLoadAction::DontCare);
        let Some(encoder) = command_buffer.renderCommandEncoderWithDescriptor(&descriptor) else { return };
        encoder.setRenderPipelineState(&self.pipelines.glass);
        // 🔓️ SAFETY: `glass_buffer` and `self.quad_vertex_buffer`/`self.ui_globals_buffer` are all
        // owned buffers outliving this call; offsets are all `0` (whole-buffer binds).
        unsafe {
            encoder.setVertexBuffer_offset_atIndex(Some(&self.quad_vertex_buffer), 0, 0);
            encoder.setVertexBuffer_offset_atIndex(Some(glass_buffer), 0, 1);
            encoder.setVertexBuffer_offset_atIndex(Some(&self.ui_globals_buffer), 0, 2);
            encoder.setFragmentTexture_atIndex(Some(self.scene_target.texture()), 1);
            encoder.setFragmentSamplerState_atIndex(Some(&self.pipelines.scene_sampler), 1);
            encoder.drawPrimitives_vertexStart_vertexCount_instanceCount(MTLPrimitiveType::Triangle, 0, 6, packet.glass_instances.len());
        }
        encoder.endEncoding();
    }
}

/// 🏗️ One color attachment (`color`/`level`/`load`/optional clear) plus an optional shared
/// depth+stencil attachment (`depth`/`depth_load`) — every render pass in this backend is one of
/// these two shapes.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn make_render_pass_descriptor(color: &MetalTexture, level: usize, load: MTLLoadAction, clear: Option<(f64, f64, f64, f64)>, depth: Option<&MetalTexture>, depth_load: MTLLoadAction) -> Retained<MTLRenderPassDescriptor> {
    let descriptor = MTLRenderPassDescriptor::renderPassDescriptor();
    // 🔓️ SAFETY: index `0` is always in bounds — every pass in this backend uses exactly one color
    // attachment.
    let color_attachment = unsafe { descriptor.colorAttachments().objectAtIndexedSubscript(0) };
    color_attachment.setTexture(Some(color));
    color_attachment.setLevel(level);
    color_attachment.setLoadAction(load);
    color_attachment.setStoreAction(MTLStoreAction::Store);
    if let Some((red, green, blue, alpha)) = clear {
        color_attachment.setClearColor(objc2_metal::MTLClearColor { red, green, blue, alpha });
    }
    if let Some(depth_texture) = depth {
        let depth_attachment = descriptor.depthAttachment();
        depth_attachment.setTexture(Some(depth_texture));
        depth_attachment.setLoadAction(depth_load);
        depth_attachment.setStoreAction(MTLStoreAction::Store);
        depth_attachment.setClearDepth(1.0);
        let stencil_attachment = descriptor.stencilAttachment();
        stencil_attachment.setTexture(Some(depth_texture));
        stencil_attachment.setLoadAction(depth_load);
        stencil_attachment.setStoreAction(MTLStoreAction::Store);
        stencil_attachment.setClearStencil(0);
    }
    descriptor
}

/// 🎞️ Replays one filtered group of `DrawBatch`es verbatim — the trait's own invariant (see
/// `ui_render::GraphicsBackend`'s docstring): this function makes no ordering/batching/clipping
/// decision of its own, it only picks pipeline/textures per `batch.pipeline`/`batch.texture` and
/// issues the one draw call `Scene::finish` already ranged out. `PipelineKind::Glass` batches never
/// reach here (composited separately by `MetalBackend::composite_glass`); the remaining
/// `BlurMipChain`/`SceneBlit`/`StencilMask`/`World3d*` variants never appear in `RenderPacket::batches`
/// at all (see `ui_render::scene::batch`'s construction — only `UiQuad`/`UiRasterTextured`/`Vector`/
/// `Glass` are ever pushed).
#[allow(clippy::too_many_arguments)]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn encode_2d_batches<'a>(
    encoder: &Encoder,
    pipelines: &Pipelines,
    resources: &GpuResources,
    quad_vertex_buffer: &MetalBuffer,
    ui_globals_buffer: &MetalBuffer,
    quad_buffer: Option<&MetalBuffer>,
    vector_buffer: Option<&MetalBuffer>,
    batches: impl Iterator<Item = &'a DrawBatch>,
    width: f32,
    height: f32,
) {
    let quad_stride = std::mem::size_of::<ui_render::QuadInstance>();
    let vector_stride = std::mem::size_of::<ui_render::VectorVertex>();
    for batch in batches {
        match batch.mask_range {
            Some((start, count)) => draw_silhouette_mask(encoder, pipelines, quad_vertex_buffer, ui_globals_buffer, quad_buffer, start, count, quad_stride, width, height),
            None => encoder.setStencilReferenceValue(1),
        }
        match batch.pipeline {
            PipelineKind::UiQuad | PipelineKind::UiRasterTextured => {
                let Some(quad_buffer) = quad_buffer else { continue };
                encoder.setRenderPipelineState(&pipelines.ui_content);
                encoder.setDepthStencilState(Some(&pipelines.content_depth_stencil));
                // 🔓️ SAFETY: `batch.instance_range` is a range `Scene::finish` produced into the same
                // `quad_instances` array this `quad_buffer` was uploaded from — always in bounds.
                unsafe {
                    encoder.setVertexBuffer_offset_atIndex(Some(quad_vertex_buffer), 0, 0);
                    encoder.setVertexBuffer_offset_atIndex(Some(quad_buffer), (batch.instance_range.0 as usize) * quad_stride, 1);
                    encoder.setVertexBuffer_offset_atIndex(Some(ui_globals_buffer), 0, 2);
                    encoder.setFragmentBuffer_offset_atIndex(Some(ui_globals_buffer), 0, 2);
                }
                bind_ui_textures(encoder, pipelines, resources, batch.texture);
                unsafe {
                    encoder.drawPrimitives_vertexStart_vertexCount_instanceCount(MTLPrimitiveType::Triangle, 0, 6, batch.instance_range.1 as usize);
                }
            }
            PipelineKind::Vector => {
                let Some(vector_buffer) = vector_buffer else { continue };
                encoder.setRenderPipelineState(&pipelines.vector);
                encoder.setDepthStencilState(Some(&pipelines.content_depth_stencil));
                // 🔓️ SAFETY: same reasoning as the `UiQuad` arm above, against `vector_vertices`.
                unsafe {
                    encoder.setVertexBuffer_offset_atIndex(Some(vector_buffer), (batch.instance_range.0 as usize) * vector_stride, 0);
                    encoder.setVertexBuffer_offset_atIndex(Some(ui_globals_buffer), 0, 2);
                    encoder.drawPrimitives_vertexStart_vertexCount(MTLPrimitiveType::Triangle, 0, batch.instance_range.1 as usize);
                }
            }
            PipelineKind::Glass | PipelineKind::BlurMipChain | PipelineKind::SceneBlit | PipelineKind::StencilMask | PipelineKind::World3dMesh | PipelineKind::World3dLines | PipelineKind::World3dTextured => {
                // 🕳️ Never constructed by `Scene::finish` into `RenderPacket::batches` — see this
                // function's doc comment.
            }
        }
    }
    encoder.setScissorRect(objc2_metal::MTLScissorRect { x: 0, y: 0, width: width as usize, height: height as usize });
}

/// 🩹️ Ports `draw_silhouette_mask` verbatim: stamps `reset_bounds` (the first instance in the range)
/// with stencil ref `0`, then every remaining "piece" instance with ref `1`, leaving ref `1` set for
/// the content draw that follows.
#[allow(clippy::too_many_arguments)]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn draw_silhouette_mask(encoder: &Encoder, pipelines: &Pipelines, quad_vertex_buffer: &MetalBuffer, ui_globals_buffer: &MetalBuffer, quad_buffer: Option<&MetalBuffer>, start: u32, count: u32, quad_stride: usize, width: f32, height: f32) {
    let Some(quad_buffer) = quad_buffer else {
        encoder.setStencilReferenceValue(1);
        return;
    };
    encoder.setScissorRect(objc2_metal::MTLScissorRect { x: 0, y: 0, width: width as usize, height: height as usize });
    encoder.setRenderPipelineState(&pipelines.ui_mask);
    encoder.setDepthStencilState(Some(&pipelines.mask_depth_stencil));
    // 🔓️ SAFETY: `start`/`count` are a `mask_range` `Scene::finish` produced into the same
    // `quad_instances` array `quad_buffer` was uploaded from — always in bounds.
    unsafe {
        encoder.setVertexBuffer_offset_atIndex(Some(quad_vertex_buffer), 0, 0);
        encoder.setVertexBuffer_offset_atIndex(Some(ui_globals_buffer), 0, 2);
        encoder.setFragmentBuffer_offset_atIndex(Some(ui_globals_buffer), 0, 2);
        encoder.setStencilReferenceValue(0);
        encoder.setVertexBuffer_offset_atIndex(Some(quad_buffer), (start as usize) * quad_stride, 1);
        encoder.drawPrimitives_vertexStart_vertexCount_instanceCount(MTLPrimitiveType::Triangle, 0, 6, 1);
    }
    if count > 1 {
        unsafe {
            encoder.setStencilReferenceValue(1);
            encoder.setVertexBuffer_offset_atIndex(Some(quad_buffer), ((start + 1) as usize) * quad_stride, 1);
            encoder.drawPrimitives_vertexStart_vertexCount_instanceCount(MTLPrimitiveType::Triangle, 0, 6, (count - 1) as usize);
        }
    } else {
        encoder.setStencilReferenceValue(1);
    }
}

/// 🖼️ `texture(0)`/`sampler(0)` are always the shared glyph atlas; `texture(1)`/`sampler(1)` are the
/// shared icon atlas for `UiQuad` batches, or the batch's specific raster texture for
/// `UiRasterTextured` batches (`batch.texture`) — mirrors `RasterTextureTable::ensure_raster`'s
/// per-texture bind group swap.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn bind_ui_textures(encoder: &Encoder, pipelines: &Pipelines, resources: &GpuResources, texture: Option<TextureId>) {
    // 🔓️ SAFETY: texture/sampler bind calls with no bounds/lifetime hazard — every argument is either
    // `None` (unbinds the slot) or borrowed from `resources`/`pipelines`, both outliving this call.
    unsafe {
        encoder.setFragmentTexture_atIndex(resources.glyph_atlas(), 0);
        encoder.setFragmentSamplerState_atIndex(Some(&pipelines.glyph_sampler), 0);
        match texture {
            Some(id) => {
                encoder.setFragmentTexture_atIndex(resources.raster_texture(id), 1);
                encoder.setFragmentSamplerState_atIndex(Some(&pipelines.icon_sampler), 1);
            }
            None => {
                encoder.setFragmentTexture_atIndex(resources.icon_atlas(), 1);
                encoder.setFragmentSamplerState_atIndex(Some(&pipelines.icon_sampler), 1);
            }
        }
    }
}

//#endregion Rendering

//#region 🔌️GraphicsBackendImpl

impl GraphicsBackend for MetalBackend {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn name(&self) -> &'static str {
        "metal"
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn capabilities(&self) -> DeviceCapabilities {
        DeviceCapabilities {
            max_texture_dimension: 16384,
            max_bind_groups: 4,
            supports_msaa: true,
            supports_timestamp_queries: false,
            supports_storage_buffers: true,
            preferred_surface_format: SurfaceFormat::Bgra8UnormSrgb,
            memory_class: if self.device.hasUnifiedMemory() { MemoryClass::Abundant } else { MemoryClass::Standard },
            gpu_tier: if self.device.isLowPower() { GpuTier::Integrated } else { GpuTier::Discrete },
        }
    }

    /// 🕳️ A zero-size request parks: `self.size` is still recorded, but `scene_target`/`depth_texture`
    /// are left untouched (never resized to zero) so `render` has a valid — if stale — target the
    /// instant a nonzero `resize` restores it. `render` itself refuses to draw while parked (`size.is_zero()`
    /// → `SkippedZeroSize`), so the staleness is never observed.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn resize(&mut self, size: PhysicalSize, dpr: f32) -> Result<(), BackendError> {
        self.size = size;
        self.dpr = dpr;
        set_drawable_size(&self.layer, size.width.max(1), size.height.max(1));
        if !size.is_zero() {
            self.scene_target.ensure(&self.device, size.width, size.height);
            self.depth_texture = Some(allocate_depth_texture(&self.device, size.width, size.height));
        }
        Ok(())
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn apply_resources(&mut self, ops: &[ResourceOp]) -> Result<(), BackendError> {
        self.resources.apply(&self.device, ops).map_err(Into::into)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn render(&mut self, packet: &RenderPacket, time_seconds: f32) -> Result<RenderReport, BackendError> {
        if let DeviceStatus::Lost(reason) = self.status {
            return Err(BackendError::DeviceLost(reason));
        }
        if self.size.is_zero() {
            return Ok(RenderReport::SkippedZeroSize);
        }
        self.validate_known_resources(packet)?;
        let Some(drawable) = self.layer.nextDrawable() else {
            return Ok(RenderReport::SkippedOutOfDate);
        };
        let drawable_texture = drawable.texture();
        let Some(command_buffer) = self.queue.commandBuffer() else {
            return Err(BackendError::Timeout);
        };
        self.encode_scene_pass(&command_buffer, packet, time_seconds);
        self.encode_composite_pass(&command_buffer, &drawable_texture, packet);
        #[cfg(feature = "backend-testing")]
        self.capture_readback(&command_buffer, &drawable_texture);
        command_buffer.presentDrawable(drawable.as_ref());
        command_buffer.commit();
        let stats = FrameStats {
            encode_duration_seconds: 0.0,
            submit_duration_seconds: 0.0,
            present_duration_seconds: 0.0,
            draw_call_count: packet.batches.len() as u32,
            instance_count: (packet.quad_instances.len() + packet.vector_vertices.len()) as u32,
        };
        Ok(RenderReport::Presented { stats })
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn device_status(&self) -> DeviceStatus {
        self.status
    }

    /// ♻️ A *simulated* recovery: Metal exposes no programmatic "recreate the device" API the way a
    /// forced loss needs to be undone, so this drops every resource table (mirroring what a real
    /// device loss would actually invalidate) and rebuilds `GpuResources` fresh — the caller's
    /// `ResourceRegistry::report_device_loss` re-marks the returned ids `Requested`, and the next
    /// frame's `apply_resources` repopulates them for real.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn recover(&mut self) -> Result<RecoveredResources, BackendError> {
        let (lost_textures, lost_meshes, lost_atlases) = self.resources.drain_known();
        self.resources = GpuResources::new(&self.device);
        self.status = DeviceStatus::Healthy;
        Ok(RecoveredResources { lost_textures, lost_meshes, lost_atlases })
    }

    /// 🧪️ Metal has no real "lose the device on demand" API — this sets the same `DeviceStatus::Lost`
    /// state a real fault would, so `render`/`recover` behave identically to the real-fault path from
    /// this point forward.
    #[cfg(feature = "backend-testing")]
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn debug_force_device_loss(&mut self) {
        self.status = DeviceStatus::Lost(LossReason::Device);
    }

    /// 🧪️ Reads back the readback texture `render` populated via a same-frame blit from the presented
    /// drawable (see `capture_readback`) — `BGRA8Unorm_sRGB` bytes swizzled to the tightly packed RGBA8
    /// `ReadbackImage` the conformance suite expects (an sRGB-vs-linear byte reinterpretation, not a
    /// full colour-managed conversion — acceptable for the suite's `|Δ| ≤ 3/255` tolerance, called out
    /// here rather than silently assumed correct).
    #[cfg(feature = "backend-testing")]
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn read_back(&mut self) -> Result<ReadbackImage, BackendError> {
        if self.size.is_zero() {
            return Err(BackendError::ZeroSizeSurface);
        }
        let Some(texture) = self.readback.as_deref() else {
            return Err(BackendError::Timeout);
        };
        let width = self.size.width;
        let height = self.size.height;
        let bytes_per_row = width as usize * 4;
        let mut bgra = vec![0u8; bytes_per_row * height as usize];
        let region = objc2_metal::MTLRegion { origin: MTLOrigin { x: 0, y: 0, z: 0 }, size: objc2_metal::MTLSize { width: width as usize, height: height as usize, depth: 1 } };
        let Some(pointer) = std::ptr::NonNull::new(bgra.as_mut_ptr().cast::<std::ffi::c_void>()) else {
            return Err(BackendError::OutOfMemory);
        };
        // 🔓️ SAFETY: `bgra` is exactly `bytes_per_row * height` bytes, matching `region`/`bytes_per_row`
        // passed to Metal — the same invariant `getBytes` requires of its destination buffer.
        unsafe {
            texture.getBytes_bytesPerRow_fromRegion_mipmapLevel(pointer, bytes_per_row, region, 0);
        }
        let mut pixels = vec![0u8; bgra.len()];
        for chunk in 0..(bgra.len() / 4) {
            let base = chunk * 4;
            pixels[base] = bgra[base + 2];
            pixels[base + 1] = bgra[base + 1];
            pixels[base + 2] = bgra[base];
            pixels[base + 3] = bgra[base + 3];
        }
        Ok(ReadbackImage { width, height, pixels })
    }
}

#[cfg(feature = "backend-testing")]
impl MetalBackend {
    /// 🧪️ Blits the just-composited drawable into a fresh `Shared`-storage readback texture — always
    /// reallocated at the current size rather than cached, since this path only ever runs inside the
    /// conformance harness (never a hot path worth optimizing).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn capture_readback(&mut self, command_buffer: &CommandBuffer, drawable_texture: &MetalTexture) {
        let width = self.size.width;
        let height = self.size.height;
        let texture = allocate_readback_texture(&self.device, width, height, self.surface_format);
        let Some(blit_encoder) = command_buffer.blitCommandEncoder() else {
            self.readback = Some(texture);
            return;
        };
        let extent = objc2_metal::MTLSize { width: width as usize, height: height as usize, depth: 1 };
        // 🔓️ SAFETY: `texture` was just allocated at exactly `width`x`height`; `drawable_texture` is
        // the backend's own current-frame drawable at the same size.
        unsafe {
            blit_encoder.copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin(
                drawable_texture,
                0,
                0,
                MTLOrigin { x: 0, y: 0, z: 0 },
                extent,
                &texture,
                0,
                0,
                MTLOrigin { x: 0, y: 0, z: 0 },
            );
        }
        blit_encoder.endEncoding();
        self.readback = Some(texture);
    }
}

#[cfg(feature = "backend-testing")]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn allocate_readback_texture(device: &Device, width: u32, height: u32, format: MTLPixelFormat) -> Retained<MetalTexture> {
    let descriptor = objc2_metal::MTLTextureDescriptor::new();
    descriptor.setPixelFormat(format);
    unsafe {
        descriptor.setWidth(width.max(1) as _);
        descriptor.setHeight(height.max(1) as _);
    }
    descriptor.setUsage(objc2_metal::MTLTextureUsage::ShaderRead);
    descriptor.setResourceOptions(objc2_metal::MTLResourceOptions::StorageModeShared);
    device.newTextureWithDescriptor(&descriptor).expect("metal backend: failed to allocate the readback texture")
}

//#endregion 🔌️GraphicsBackendImpl

//#endregion 🍎️MetalBackend

//#region Tests

/// 🧪️ Every test here needs a live Metal device — `MetalBackend::new_headless` itself reports
/// `BackendError::DeviceLost` when `MTLCreateSystemDefaultDevice` finds none (a genuinely possible
/// state on a headless CI runner), so failing construction *is* this module's "skip cleanly" signal;
/// no separate availability probe is needed. `backend-testing` gates the whole module (`new_headless`/
/// `debug_force_device_loss`/`recover`/`read_back` are all feature-gated on the trait itself).
#[cfg(all(test, feature = "backend-testing"))]
mod tests {
    use super::*;
    use ui_render::{FinishParams, ResourceRegistry, Scene, SceneBuilder};

    /// 🧵️ Drives an `async fn` that structurally never suspends (Metal's device/queue/layer creation
    /// is synchronous — see `MetalBackend::new`'s docstring) to completion without pulling in an
    /// executor crate. Panics if the future ever actually returns `Pending`, which would mean this
    /// backend grew a real suspension point somewhere and this helper is no longer valid.
    fn block_on<F: std::future::Future>(future: F) -> F::Output {
        use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
        const VTABLE: RawWakerVTable = RawWakerVTable::new(|_| RAW_WAKER, |_| {}, |_| {}, |_| {});
        const RAW_WAKER: RawWaker = RawWaker::new(std::ptr::null(), &VTABLE);
        let waker = unsafe { Waker::from_raw(RAW_WAKER) };
        let mut context = Context::from_waker(&waker);
        let mut future = Box::pin(future);
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => value,
            Poll::Pending => panic!("metal backend: a construction future that should never suspend returned Pending"),
        }
    }

    fn finish_params(viewport: [f32; 2]) -> FinishParams {
        FinishParams { viewport, dpr: 1.0, time_seconds_origin: 0.0, resource_ops: Vec::new() }
    }

    #[test]
    fn constructing_a_headless_backend_succeeds_or_skips_cleanly() {
        let Ok(mut backend) = block_on(MetalBackend::new_headless(PhysicalSize::new(64, 64), 1.0)) else {
            eprintln!("skipping: no Metal device available on this machine");
            return;
        };
        assert_eq!(GraphicsBackend::name(&backend), "metal");
        assert!(GraphicsBackend::capabilities(&backend).max_texture_dimension > 0);
        assert_eq!(GraphicsBackend::device_status(&backend), DeviceStatus::Healthy);
        let _ = GraphicsBackend::resize(&mut backend, PhysicalSize::new(64, 64), 1.0);
    }

    #[test]
    fn zero_size_resize_parks_and_restores() {
        let Ok(mut backend) = block_on(MetalBackend::new_headless(PhysicalSize::new(64, 64), 1.0)) else {
            eprintln!("skipping: no Metal device available on this machine");
            return;
        };
        GraphicsBackend::resize(&mut backend, PhysicalSize::ZERO, 1.0).expect("resize to zero");
        let packet = Scene::finish(SceneBuilder::default(), finish_params([0.0, 0.0])).expect("finish");
        assert!(matches!(GraphicsBackend::render(&mut backend, &packet, 0.0), Ok(RenderReport::SkippedZeroSize)));

        GraphicsBackend::resize(&mut backend, PhysicalSize::new(64, 64), 1.0).expect("resize back");
        let mut builder = SceneBuilder::default();
        builder.push_solid([0.0, 0.0, 10.0, 10.0], [1.0, 0.0, 0.0, 1.0]);
        let packet = Scene::finish(builder, finish_params([64.0, 64.0])).expect("finish");
        let report = GraphicsBackend::render(&mut backend, &packet, 0.0).expect("render after restoring a nonzero size");
        assert!(matches!(report, RenderReport::Presented { .. } | RenderReport::SkippedOutOfDate));
    }

    #[test]
    fn apply_resources_before_render_succeeds_and_an_unapplied_id_errors_cleanly() {
        let Ok(mut backend) = block_on(MetalBackend::new_headless(PhysicalSize::new(64, 64), 1.0)) else {
            eprintln!("skipping: no Metal device available on this machine");
            return;
        };
        let mut registry = ResourceRegistry::default();
        registry.request_texture_upload("known", 4, 4, vec![0; 64]);
        let mut applied_builder = SceneBuilder::default();
        applied_builder.push_raster_quad(&mut registry, "known", [0.0, 0.0, 10.0, 10.0], [0.0, 0.0, 1.0, 1.0], 1.0);
        GraphicsBackend::apply_resources(&mut backend, &registry.drain_ops()).expect("apply_resources");
        let applied_packet = Scene::finish(applied_builder, finish_params([64.0, 64.0])).expect("finish");
        let report = GraphicsBackend::render(&mut backend, &applied_packet, 0.0).expect("render with an applied texture");
        assert!(matches!(report, RenderReport::Presented { .. } | RenderReport::SkippedOutOfDate));

        let mut unapplied_builder = SceneBuilder::default();
        unapplied_builder.push_raster_quad(&mut registry, "unknown", [0.0, 0.0, 10.0, 10.0], [0.0, 0.0, 1.0, 1.0], 1.0);
        let unapplied_packet = Scene::finish(unapplied_builder, finish_params([64.0, 64.0])).expect("finish");
        let result = GraphicsBackend::render(&mut backend, &unapplied_packet, 0.0);
        assert!(matches!(result, Err(BackendError::UnknownResource(ResourceKind::Texture))));
    }

    #[test]
    fn forced_device_loss_reports_lost_and_recover_names_the_dead_generation() {
        let Ok(mut backend) = block_on(MetalBackend::new_headless(PhysicalSize::new(64, 64), 1.0)) else {
            eprintln!("skipping: no Metal device available on this machine");
            return;
        };
        let mut registry = ResourceRegistry::default();
        let texture = registry.request_texture_upload("icon", 4, 4, vec![0; 64]);
        GraphicsBackend::apply_resources(&mut backend, &registry.drain_ops()).expect("apply_resources");

        GraphicsBackend::debug_force_device_loss(&mut backend);
        assert!(matches!(GraphicsBackend::device_status(&backend), DeviceStatus::Lost(_)));
        let packet = Scene::finish(SceneBuilder::default(), finish_params([64.0, 64.0])).expect("finish");
        assert!(matches!(GraphicsBackend::render(&mut backend, &packet, 0.0), Err(BackendError::DeviceLost(_))));

        let recovered = GraphicsBackend::recover(&mut backend).expect("recover");
        assert_eq!(recovered.lost_textures, vec![texture]);
        assert_eq!(GraphicsBackend::device_status(&backend), DeviceStatus::Healthy);
    }

    #[test]
    fn read_back_reports_zero_size_cleanly_before_any_frame_is_presented() {
        let Ok(mut backend) = block_on(MetalBackend::new_headless(PhysicalSize::ZERO, 1.0)) else {
            eprintln!("skipping: no Metal device available on this machine");
            return;
        };
        assert!(matches!(GraphicsBackend::read_back(&mut backend), Err(BackendError::ZeroSizeSurface)));
    }
}

//#endregion Tests

//#endregion 🔖️Backend
