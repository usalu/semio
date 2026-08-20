//! @emoji 🌫️ The offscreen scene-color target + its mip-chain blur scratch texture — the Metal
//! counterpart of the wgpu target's `SceneColorTarget`, mirroring `GpuContext::render_frame`'s
//! two-pass structure (`🎯️targets/🧊️wgpu/🦀️gpu.rs`): render 2D/3D content into this target, then blur
//! its mip chain and composite glass regions on top before blitting to the real swapchain view.
//!
//! **Fewer objects than the wgpu target**, deliberately: Metal's `sample(sampler, uv, level(lod))`
//! takes an explicit LOD directly against the whole mip chain, and
//! `MTLRenderPassColorAttachmentDescriptor.level` picks a render-target mip directly on the original
//! texture — so unlike `SceneColorTarget`, this struct never allocates a `Vec` of per-mip
//! `TextureView`s for either sampling or rendering. Same pixels, see `🦀️msl.rs`'s header for the full
//! reasoning.

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::{MTLDevice, MTLPixelFormat, MTLTexture, MTLTextureDescriptor, MTLTextureUsage};

//#region 🔖️SceneTarget

/// 🌫️ `SCENE_MIP_LEVELS` in the wgpu target's `draw.rs` — five mip levels of box-downsample give the
/// glass backdrop's blur radius range (`Theme::glass_mip_level` maps a blur-px request onto
/// `0..=max_mip`).
pub const SCENE_MIP_LEVELS: u32 = 5;

type Device = ProtocolObject<dyn MTLDevice>;
type MetalTexture = ProtocolObject<dyn MTLTexture>;

/// 🌫️ Owns the two textures `composite_to_swapchain`'s blur/glass pass needs: the scene's own
/// full-mip-chain color target, and a same-shaped scratch texture the blur downsample copies into
/// before reading from it (Metal, like wgpu, cannot bind a texture as both a render-target attachment
/// and a shader-read source within the same texture at once — hence the scratch copy, ported from
/// `SceneColorTarget::copy_mip_to_blur_scratch`).
pub struct SceneTarget {
    texture: Retained<MetalTexture>,
    blur_scratch: Retained<MetalTexture>,
    width: u32,
    height: u32,
    format: MTLPixelFormat,
}

impl SceneTarget {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(device: &Device, width: u32, height: u32, format: MTLPixelFormat) -> Self {
        let width = width.max(1);
        let height = height.max(1);
        let texture = allocate(device, format, width, height, "scene_color");
        let blur_scratch = allocate(device, format, width, height, "scene_blur_scratch");
        Self { texture, blur_scratch, width, height, format }
    }

    /// 🔁️ Recreates both textures only when the requested size actually changed — mirrors
    /// `SceneColorTarget::ensure`'s early return.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn ensure(&mut self, device: &Device, width: u32, height: u32) {
        let width = width.max(1);
        let height = height.max(1);
        if self.width == width && self.height == height {
            return;
        }
        self.texture = allocate(device, self.format, width, height, "scene_color");
        self.blur_scratch = allocate(device, self.format, width, height, "scene_blur_scratch");
        self.width = width;
        self.height = height;
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn texture(&self) -> &MetalTexture {
        &self.texture
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn blur_scratch(&self) -> &MetalTexture {
        &self.blur_scratch
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn width(&self) -> u32 {
        self.width
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn height(&self) -> u32 {
        self.height
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn format(&self) -> MTLPixelFormat {
        self.format
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
/// 🌫️ How many mip levels these dimensions can actually carry. Metal rejects a descriptor asking for
/// more levels than `floor(log2(max(w, h))) + 1` — a hard validation abort, not a soft clamp — so a
/// window enough during a resize (or a 1×1 surface) would take the process down. The blur chain
/// therefore asks for as many levels as it wants *or* as many as exist, whichever is fewer.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn supported_mip_levels(width: u32, height: u32) -> u32 {
    let largest = width.max(height).max(1);
    let available = 32 - largest.leading_zeros();
    available.min(SCENE_MIP_LEVELS).max(1)
}

fn allocate(device: &Device, format: MTLPixelFormat, width: u32, height: u32, label: &str) -> Retained<MetalTexture> {
    let descriptor = MTLTextureDescriptor::new();
    descriptor.setPixelFormat(format);
    // 🔓️ SAFETY: plain dimension/mip-count setters; Metal validates rather than reading OOB, and
    // `width`/`height` are already `.max(1)`-clamped by every caller.
    unsafe {
        descriptor.setWidth(width as _);
        descriptor.setHeight(height as _);
        descriptor.setMipmapLevelCount(supported_mip_levels(width, height) as _);
    }
    descriptor.setUsage(MTLTextureUsage::RenderTarget | MTLTextureUsage::ShaderRead);
    let texture = device.newTextureWithDescriptor(&descriptor).unwrap_or_else(|| panic!("metal backend: failed to allocate {label} ({width}x{height})"));
    let _ = label;
    texture
}

//#endregion 🔖️SceneTarget
