//! @emoji 🔌️ Instance → adapter → device → surface construction, and the pure
//! `wgpu` → `ui_render` capability/format translations `crate::backend::WebGpuBackend::new`/
//! `capabilities` need. Ported from `🎯️targets/🧊️wgpu/🦀️gpu.rs`'s `GpuContext::from_window`, adapted
//! from a `winit::window::Window` target to a directly-supplied `web_sys::HtmlCanvasElement` (this
//! crate has no window of its own — the embedder owns the canvas).

use ui_render::{BackendError, DeviceCapabilities, GpuTier, MemoryClass, SurfaceFormat};

//#region 🔖️GpuContext

//#region 🎨️FormatTranslation

/// 🎨️ `None` for a surface format this contract has no marker for (`Rgba8Unorm` non-srgb, etc.) — a
/// backend still configures the surface with it, it just can't be named in `DeviceCapabilities`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn surface_format_marker(format: wgpu::TextureFormat) -> Option<SurfaceFormat> {
    match format {
        wgpu::TextureFormat::Bgra8UnormSrgb => Some(SurfaceFormat::Bgra8UnormSrgb),
        wgpu::TextureFormat::Rgba8UnormSrgb => Some(SurfaceFormat::Rgba8UnormSrgb),
        wgpu::TextureFormat::Rgba16Float => Some(SurfaceFormat::Rgba16Float),
        _ => None,
    }
}

/// 🎨️ Picks the base (non-srgb) surface format the same way `gpu.rs` did — the swapchain texture
/// itself stays in this format; a per-frame view reinterprets it as [`srgb_view_format`] for
/// hardware-correct blending. Falls back to whatever the surface offers first if every format is srgb.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn pick_surface_format(formats: &[wgpu::TextureFormat]) -> wgpu::TextureFormat {
    formats.iter().copied().find(|format| !format.is_srgb()).unwrap_or_else(|| formats.first().copied().unwrap_or(wgpu::TextureFormat::Bgra8Unorm))
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn srgb_view_format(surface_format: wgpu::TextureFormat) -> wgpu::TextureFormat {
    if surface_format.is_srgb() {
        surface_format
    } else {
        surface_format.add_srgb_suffix()
    }
}

//#endregion 🎨️FormatTranslation

//#region 🧭️Capabilities

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn memory_class(limits: &wgpu::Limits) -> MemoryClass {
    match limits.max_buffer_size {
        0..=268_435_456 => MemoryClass::Constrained,
        268_435_457..=2_147_483_648 => MemoryClass::Standard,
        _ => MemoryClass::Abundant,
    }
}

/// 🏎️ webgpu exposes no direct "tier" concept; `Integrated` is the honest default for a browser
/// context (a discrete-GPU signal is not reliably available from `AdapterInfo` on the web backend).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn gpu_tier(info: &wgpu::AdapterInfo) -> GpuTier {
    match info.device_type {
        wgpu::DeviceType::Cpu => GpuTier::Software,
        wgpu::DeviceType::DiscreteGpu => GpuTier::Discrete,
        _ => GpuTier::Integrated,
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn device_capabilities(info: &wgpu::AdapterInfo, limits: &wgpu::Limits, surface_format: wgpu::TextureFormat) -> DeviceCapabilities {
    DeviceCapabilities {
        max_texture_dimension: limits.max_texture_dimension_2d,
        max_bind_groups: limits.max_bind_groups,
        supports_msaa: true,
        supports_timestamp_queries: false,
        supports_storage_buffers: limits.max_storage_buffers_per_shader_stage > 0,
        preferred_surface_format: surface_format_marker(surface_format).unwrap_or(SurfaceFormat::Rgba8UnormSrgb),
        memory_class: memory_class(limits),
        gpu_tier: gpu_tier(info),
    }
}

//#endregion 🧭️Capabilities

//#region 🏗️Construct

/// 🏗️ Everything [`crate::backend::WebGpuBackend::new`] needs after the two truly async steps
/// (`request_adapter`/`request_device`) resolve. Kept a plain struct so construction stays a flat
/// sequence of `?`-checked steps rather than one giant nested closure.
pub(crate) struct GpuContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'static>,
    pub surface_format: wgpu::TextureFormat,
    pub view_format: wgpu::TextureFormat,
    pub alpha_mode: wgpu::CompositeAlphaMode,
    pub capabilities: DeviceCapabilities,
}

impl GpuContext {
    /// 🌐️ The one genuinely async part of this whole backend (ticket brief) — instance → surface →
    /// adapter → device, all real round-trips to the browser's GPU process.
    // 🌐️async: genuinely async device/adapter construction — the one exception U1 itself carves out.
    pub(crate) async fn new(canvas: web_sys::HtmlCanvasElement) -> Result<Self, BackendError> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor { backends: wgpu::Backends::BROWSER_WEBGPU, ..Default::default() });
        let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas)).map_err(|_| BackendError::CanvasReplaced)?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, compatible_surface: Some(&surface), force_fallback_adapter: false })
            .await
            .map_err(|_| BackendError::UnsupportedFormat("no compatible WebGPU adapter"))?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("semio_webgpu_backend"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default().using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::Off,
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
            })
            .await
            .map_err(|_| BackendError::OutOfMemory)?;
        let caps = surface.get_capabilities(&adapter);
        let surface_format = pick_surface_format(&caps.formats);
        let view_format = srgb_view_format(surface_format);
        let alpha_mode = caps.alpha_modes.first().copied().unwrap_or(wgpu::CompositeAlphaMode::Auto);
        let capabilities = device_capabilities(&adapter.get_info(), &device.limits(), view_format);
        Ok(Self { device, queue, surface, surface_format, view_format, alpha_mode, capabilities })
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn configure(&self, width: u32, height: u32) {
        configure_surface(&self.device, &self.surface, self.surface_format, self.view_format, self.alpha_mode, width, height);
    }
}

/// 🖼️ Shared by [`GpuContext::new`]'s initial configure and `crate::backend::WebGpuBackend::resize`'s
/// reconfigure — both need the exact same `SurfaceConfiguration` shape.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn configure_surface(device: &wgpu::Device, surface: &wgpu::Surface<'static>, surface_format: wgpu::TextureFormat, view_format: wgpu::TextureFormat, alpha_mode: wgpu::CompositeAlphaMode, width: u32, height: u32) {
    let mut usage = wgpu::TextureUsages::RENDER_ATTACHMENT;
    #[cfg(feature = "backend-testing")]
    {
        usage |= wgpu::TextureUsages::COPY_SRC;
    }
    let config = wgpu::SurfaceConfiguration {
        usage,
        format: surface_format,
        width: width.max(1),
        height: height.max(1),
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode,
        view_formats: vec![view_format],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(device, &config);
}

/// 🕳️ The shared depth/stencil texture every content/mask/vector/world3d pipeline attaches — matches
/// `gpu.rs`'s `ensure_depth`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn create_depth_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("webgpu_backend_depth"),
        size: wgpu::Extent3d { width: width.max(1), height: height.max(1), depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: crate::gpu_types::DEPTH_STENCIL_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    })
}

//#endregion 🏗️Construct

//#endregion 🔖️GpuContext

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_formats_map_to_their_marker() {
        assert_eq!(surface_format_marker(wgpu::TextureFormat::Bgra8UnormSrgb), Some(SurfaceFormat::Bgra8UnormSrgb));
        assert_eq!(surface_format_marker(wgpu::TextureFormat::Rgba8UnormSrgb), Some(SurfaceFormat::Rgba8UnormSrgb));
        assert_eq!(surface_format_marker(wgpu::TextureFormat::Rgba16Float), Some(SurfaceFormat::Rgba16Float));
    }

    #[test]
    fn unmarked_format_is_none() {
        assert_eq!(surface_format_marker(wgpu::TextureFormat::Rgba8Unorm), None);
    }

    #[test]
    fn picks_first_non_srgb_format() {
        let formats = [wgpu::TextureFormat::Bgra8UnormSrgb, wgpu::TextureFormat::Bgra8Unorm, wgpu::TextureFormat::Rgba8Unorm];
        assert_eq!(pick_surface_format(&formats), wgpu::TextureFormat::Bgra8Unorm);
    }

    #[test]
    fn falls_back_to_first_format_when_all_are_srgb() {
        let formats = [wgpu::TextureFormat::Bgra8UnormSrgb];
        assert_eq!(pick_surface_format(&formats), wgpu::TextureFormat::Bgra8UnormSrgb);
    }

    #[test]
    fn srgb_view_format_adds_suffix_to_a_non_srgb_base() {
        assert_eq!(srgb_view_format(wgpu::TextureFormat::Bgra8Unorm), wgpu::TextureFormat::Bgra8UnormSrgb);
        assert_eq!(srgb_view_format(wgpu::TextureFormat::Bgra8UnormSrgb), wgpu::TextureFormat::Bgra8UnormSrgb);
    }
}

//#endregion Tests
