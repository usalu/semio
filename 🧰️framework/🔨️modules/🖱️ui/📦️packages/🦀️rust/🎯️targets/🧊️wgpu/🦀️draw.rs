// #region draw
//! 🖌️ Draw list and GPU pipeline for UI quads, vector geometry, and 3D scene passes.

use super::kernel_3d_scene::{Mat4Math, ScenePass3d};
use crate::wgpu::shaders::{BLUR_DOWNSAMPLE_SHADER, GLASS_SHADER, SCENE_BLIT_SHADER, UI_SHADER, VECTOR_SHADER, WORLD3D_LINES_SHADER, WORLD3D_SHADER};
use crate::wgpu::theme::{GlassStyle, Rgba, Theme};
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

pub const KIND_SOLID: f32 = 3.0;
pub const KIND_ROUNDED: f32 = 1.0;
pub const KIND_GLYPH: f32 = 2.0;
pub const KIND_TEXTURED: f32 = 4.0;
pub const KIND_RASTER: f32 = 5.0;
/// 🌀️ Clockwise spinning + pulsing loading ring (see `UiInstance::loading_border` and `UI_SHADER`'s `kind == 6` branch).
pub const KIND_LOADING_BORDER: f32 = 6.0;
/// 🌀️ Dashed, slow-spinning + gently pulsing waiting ring (see `UiInstance::waiting_border` and `UI_SHADER`'s `kind == 7` branch).
pub const KIND_WAITING_BORDER: f32 = 7.0;
/// ✅️ Solid, static at-bounds ring for `UiStatus::Finished` (see `UiInstance::finished_border` and `UI_SHADER`'s `kind == 8` branch) — no motion, distinguishing it from loading/waiting.
pub const KIND_FINISHED_BORDER: f32 = 8.0;
/// 💫️ Raised-cosine breathing pulse ring for `UiState::Introducing` (see `UiInstance::introducing_border` and `UI_SHADER`'s `kind == 9` branch) — the single shared implementation of the introduction-tour pulse, driven by `globals._pad.x`.
pub const KIND_INTRODUCING_BORDER: f32 = 9.0;
pub const SCENE_MIP_LEVELS: u32 = 5;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct BlurGlobals {
    src_mip: f32,
    _pad: [f32; 7],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct GlassInstance {
    pub rect: [f32; 4],
    pub tint: [f32; 4],
    pub params: [f32; 4],
}

#[derive(Clone, Copy, Debug)]
pub struct GlassRegion {
    pub rect: [f32; 4],
    pub radius: f32,
    pub tint: Rgba,
    pub alpha: f32,
    pub blur_px: f32,
    pub saturate: f32,
}

pub struct SceneColorTarget {
    texture: wgpu::Texture,
    blur_scratch: wgpu::Texture,
    blur_scratch_mip_views: Vec<wgpu::TextureView>,
    sample_view: wgpu::TextureView,
    mip_views: Vec<wgpu::TextureView>,
    sampler: wgpu::Sampler,
    width: u32,
    height: u32,
}

impl SceneColorTarget {
    pub fn ensure(device: &wgpu::Device, target: &mut Option<Self>, width: u32, height: u32, format: wgpu::TextureFormat) {
        let width = width.max(1);
        let height = height.max(1);
        if let Some(existing) = target {
            if existing.width == width && existing.height == height {
                return;
            }
        }
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("scene_color"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: SCENE_MIP_LEVELS,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[format],
        });
        let blur_scratch = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("scene_blur_scratch"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: SCENE_MIP_LEVELS,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[format],
        });
        let blur_scratch_mip_views = (0..SCENE_MIP_LEVELS)
            .map(|level| {
                blur_scratch.create_view(&wgpu::TextureViewDescriptor {
                    label: Some(&format!("scene_blur_scratch_mip_{level}")),
                    format: Some(format),
                    dimension: Some(wgpu::TextureViewDimension::D2),
                    base_mip_level: level,
                    mip_level_count: Some(1),
                    ..Default::default()
                })
            })
            .collect();
        let sample_view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("scene_color_sample"),
            format: Some(format),
            dimension: Some(wgpu::TextureViewDimension::D2),
            base_mip_level: 0,
            mip_level_count: Some(SCENE_MIP_LEVELS),
            ..Default::default()
        });
        let mip_views = (0..SCENE_MIP_LEVELS)
            .map(|level| {
                texture.create_view(&wgpu::TextureViewDescriptor {
                    label: Some(&format!("scene_color_mip_{level}")),
                    format: Some(format),
                    dimension: Some(wgpu::TextureViewDimension::D2),
                    base_mip_level: level,
                    mip_level_count: Some(1),
                    ..Default::default()
                })
            })
            .collect();
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor { label: Some("scene_color_sampler"), mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, mipmap_filter: wgpu::FilterMode::Linear, ..Default::default() });
        *target = Some(Self { texture, blur_scratch, blur_scratch_mip_views, sample_view, mip_views, sampler, width, height });
    }

    pub fn mip_view(&self, level: u32) -> &wgpu::TextureView {
        &self.mip_views[level as usize]
    }

    pub fn sample_view(&self) -> &wgpu::TextureView {
        &self.sample_view
    }

    pub fn sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }

    pub fn blur_scratch_mip_view(&self, level: u32) -> &wgpu::TextureView {
        &self.blur_scratch_mip_views[level as usize]
    }

    fn mip_extent(&self, level: u32) -> wgpu::Extent3d {
        wgpu::Extent3d { width: (self.width >> level).max(1), height: (self.height >> level).max(1), depth_or_array_layers: 1 }
    }

    pub fn copy_mip_to_blur_scratch(&self, encoder: &mut wgpu::CommandEncoder, src_mip: u32) {
        let extent = self.mip_extent(src_mip);
        encoder.copy_texture_to_texture(
            wgpu::TexelCopyTextureInfo { texture: &self.texture, mip_level: src_mip, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyTextureInfo { texture: &self.blur_scratch, mip_level: src_mip, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            extent,
        );
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct UiGlobals {
    pub screen_size: [f32; 2],
    pub _pad: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct UiInstance {
    pub rect: [f32; 4],
    pub color: [f32; 4],
    pub params: [f32; 4],
    pub uv_rect: [f32; 4],
}

impl UiInstance {
    pub fn solid(rect: [f32; 4], color: Rgba) -> Self {
        Self { rect, color: [color.r, color.g, color.b, color.a], params: [0.0, 0.0, KIND_SOLID, 0.0], uv_rect: [0.0, 0.0, 1.0, 1.0] }
    }

    pub fn rounded(rect: [f32; 4], color: Rgba, radius: f32, border: f32, border_color: Rgba) -> Self {
        Self { rect, color: [color.r, color.g, color.b, color.a], params: [radius, border, KIND_ROUNDED, border_color.a], uv_rect: [0.0, 0.0, 1.0, 1.0] }
    }

    /// 🌀️ Clockwise spinning + pulsing loading ring in `color`; the sweep and pulse phase come from `globals._pad.x` (elapsed seconds) in `UI_SHADER`.
    pub fn loading_border(rect: [f32; 4], color: Rgba, radius: f32, border: f32) -> Self {
        Self { rect, color: [color.r, color.g, color.b, color.a], params: [radius, border, KIND_LOADING_BORDER, 0.0], uv_rect: [0.0, 0.0, 1.0, 1.0] }
    }

    /// 🌀️ Dashed, slow-spinning + gently pulsing waiting ring in `color`; the sweep and pulse phase come from `globals._pad.x` (elapsed seconds) in `UI_SHADER`.
    pub fn waiting_border(rect: [f32; 4], color: Rgba, radius: f32, border: f32) -> Self {
        Self { rect, color: [color.r, color.g, color.b, color.a], params: [radius, border, KIND_WAITING_BORDER, 0.0], uv_rect: [0.0, 0.0, 1.0, 1.0] }
    }

    /// ✅️ Solid, static at-bounds ring for `UiStatus::Finished` in `color` — no animation.
    pub fn finished_border(rect: [f32; 4], color: Rgba, radius: f32, border: f32) -> Self {
        Self { rect, color: [color.r, color.g, color.b, color.a], params: [radius, border, KIND_FINISHED_BORDER, 0.0], uv_rect: [0.0, 0.0, 1.0, 1.0] }
    }

    /// 💫️ Raised-cosine breathing pulse ring for `UiState::Introducing` in `color`; phase comes from `globals._pad.x` in `UI_SHADER`.
    pub fn introducing_border(rect: [f32; 4], color: Rgba, radius: f32, border: f32) -> Self {
        Self { rect, color: [color.r, color.g, color.b, color.a], params: [radius, border, KIND_INTRODUCING_BORDER, 0.0], uv_rect: [0.0, 0.0, 1.0, 1.0] }
    }

    pub fn glyph(rect: [f32; 4], color: Rgba, uv_rect: [f32; 4]) -> Self {
        Self { rect, color: [color.r, color.g, color.b, color.a], params: [0.0, 0.0, KIND_GLYPH, 0.0], uv_rect }
    }

    pub fn textured(rect: [f32; 4], uv_rect: [f32; 4], color: Rgba) -> Self {
        Self { rect, color: [color.r, color.g, color.b, color.a], params: [0.0, 0.0, KIND_TEXTURED, 0.0], uv_rect }
    }

    pub fn raster(rect: [f32; 4], uv_rect: [f32; 4], alpha: f32) -> Self {
        Self { rect, color: [1.0, 1.0, 1.0, alpha], params: [0.0, 0.0, KIND_RASTER, 0.0], uv_rect }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct VectorVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

//#region SilhouetteClip

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScissorRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl ScissorRect {
    pub fn from_rect(rect: crate::wgpu::geometry::Rect, _screen_h: f32) -> Self {
        let x = rect.x.max(0.0).floor() as u32;
        let y = rect.y.max(0.0).floor() as u32;
        let x2 = (rect.x + rect.w.max(0.0)).max(0.0).ceil() as u32;
        let y2 = (rect.y + rect.h.max(0.0)).max(0.0).ceil() as u32;
        Self { x, y, w: x2.saturating_sub(x), h: y2.saturating_sub(y) }
    }

    pub fn intersect(&self, other: &Self) -> Self {
        let x0 = self.x.max(other.x);
        let y0 = self.y.max(other.y);
        let x1 = (self.x + self.w).min(other.x + other.w);
        let y1 = (self.y + self.h).min(other.y + other.h);
        Self { x: x0, y: y0, w: x1.saturating_sub(x0), h: y1.saturating_sub(y0) }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClipRegion {
    pub scissors: Vec<ScissorRect>,
}

impl ClipRegion {
    /// 🪟️ Builds a bounded orthogonal clip union; callers provide non-overlapping pieces.
    pub fn from_rects(rects: &[crate::wgpu::geometry::Rect], screen_h: f32) -> Self {
        let scissors = rects.iter().map(|rect| ScissorRect::from_rect(*rect, screen_h)).filter(|rect| rect.w > 0 && rect.h > 0).collect();
        Self { scissors }
    }

    fn intersect(&self, other: &Self) -> Self {
        let scissors = self.scissors.iter().flat_map(|left| other.scissors.iter().map(move |right| left.intersect(right))).filter(|rect| rect.w > 0 && rect.h > 0).collect();
        Self { scissors }
    }

    fn effective_scissors(&self, scissor: Option<ScissorRect>, width: f32, height: f32) -> Vec<ScissorRect> {
        let viewport = ScissorRect { x: 0, y: 0, w: width.max(0.0) as u32, h: height.max(0.0) as u32 };
        self.scissors.iter().map(|clip| clip.intersect(&viewport)).map(|clip| scissor.map_or(clip, |parent| clip.intersect(&parent))).filter(|clip| clip.w > 0 && clip.h > 0).collect()
    }
}

//#endregion SilhouetteClip

#[derive(Clone, Default)]
pub struct DrawLayer {
    pub scissor: Option<ScissorRect>,
    pub clip: Option<ClipRegion>,
    pub foreground_of: Option<usize>,
    pub ui_instances: Vec<UiInstance>,
    pub raster_instances: Vec<(String, UiInstance)>,
    pub vector_vertices: Vec<VectorVertex>,
    pub overlay_ui_instances: Vec<UiInstance>,
    pub overlay_vector_vertices: Vec<VectorVertex>,
}

#[derive(Clone)]
pub struct DrawList {
    pub scene_passes: Vec<ScenePass3d>,
    pub layers: Vec<DrawLayer>,
    pub glass_regions: Vec<GlassRegion>,
    scissor_stack: Vec<ScissorRect>,
    clip_stack: Vec<ClipRegion>,
    glass_content_stack: Vec<usize>,
    screen_h: f32,
}

impl Default for DrawList {
    fn default() -> Self {
        let mut list = Self { scene_passes: Vec::new(), layers: Vec::new(), glass_regions: Vec::new(), scissor_stack: Vec::new(), clip_stack: Vec::new(), glass_content_stack: Vec::new(), screen_h: 720.0 };
        list.layers.push(DrawLayer::default());
        list
    }
}

impl DrawList {
    pub(crate) fn retire_step(&mut self) -> bool {
        if let Some(pass) = self.scene_passes.last_mut() {
            if let Some(draw) = pass.textured_draws.last_mut() {
                if let Some(instance) = draw.instances.last_mut() {
                    if instance.texture_key.pop().is_some() {
                        return false;
                    }
                    draw.instances.pop();
                    return false;
                }
                pass.textured_draws.pop();
                return false;
            }
            if let Some(draw) = pass.translucent_draws.last_mut() {
                if let Some(instance) = draw.instances.last_mut() {
                    if instance.id.pop().is_some() {
                        return false;
                    }
                    draw.instances.pop();
                    return false;
                }
                if draw.mesh_key.pop().is_some() {
                    return false;
                }
                pass.translucent_draws.pop();
                return false;
            }
            if let Some(draw) = pass.line_draws.last_mut() {
                if draw.vertices.pop().is_some() {
                    return false;
                }
                pass.line_draws.pop();
                return false;
            }
            if let Some(draw) = pass.draws.last_mut() {
                if let Some(instance) = draw.instances.last_mut() {
                    if instance.id.pop().is_some() {
                        return false;
                    }
                    draw.instances.pop();
                    return false;
                }
                if draw.mesh_key.pop().is_some() {
                    return false;
                }
                pass.draws.pop();
                return false;
            }
            self.scene_passes.pop();
            return false;
        }
        if let Some(layer) = self.layers.last_mut() {
            if let Some(clip) = layer.clip.as_mut() {
                if clip.scissors.pop().is_some() {
                    return false;
                }
                layer.clip = None;
                return false;
            }
            if let Some((key, _)) = layer.raster_instances.last_mut() {
                if key.pop().is_some() {
                    return false;
                }
                layer.raster_instances.pop();
                return false;
            }
            if layer.overlay_vector_vertices.pop().is_some() || layer.overlay_ui_instances.pop().is_some() || layer.vector_vertices.pop().is_some() || layer.ui_instances.pop().is_some() {
                return false;
            }
            self.layers.pop();
            return false;
        }
        if self.glass_regions.pop().is_some() || self.scissor_stack.pop().is_some() || self.glass_content_stack.pop().is_some() {
            return false;
        }
        if let Some(clip) = self.clip_stack.last_mut() {
            if clip.scissors.pop().is_some() {
                return false;
            }
            self.clip_stack.pop();
            return false;
        }
        true
    }

    pub(crate) fn retirement_is_empty(&self) -> bool {
        self.scene_passes.is_empty() && self.layers.is_empty() && self.glass_regions.is_empty() && self.scissor_stack.is_empty() && self.clip_stack.is_empty() && self.glass_content_stack.is_empty()
    }

    pub fn set_screen_height(&mut self, height: f32) {
        self.screen_h = height;
    }

    fn active_foreground_of(&self) -> Option<usize> {
        self.glass_content_stack.last().copied()
    }

    fn active_layer(&mut self) -> &mut DrawLayer {
        if self.layers.is_empty() {
            self.layers.push(DrawLayer::default());
        }
        self.layers.last_mut().expect("layer")
    }

    pub fn clear(&mut self) {
        self.scene_passes.clear();
        self.layers.clear();
        self.layers.push(DrawLayer::default());
        self.glass_regions.clear();
        self.scissor_stack.clear();
        self.clip_stack.clear();
        self.glass_content_stack.clear();
    }

    pub fn push_scissor(&mut self, rect: crate::wgpu::geometry::Rect) {
        let mut scissor = ScissorRect::from_rect(rect, self.screen_h);
        if let Some(parent) = self.scissor_stack.last() {
            scissor = parent.intersect(&scissor);
        }
        self.scissor_stack.push(scissor);
        self.layers.push(DrawLayer { scissor: Some(scissor), clip: self.clip_stack.last().cloned(), foreground_of: self.active_foreground_of(), ..DrawLayer::default() });
    }

    pub fn pop_scissor(&mut self) {
        self.scissor_stack.pop();
        let parent = self.scissor_stack.last().copied();
        self.layers.push(DrawLayer { scissor: parent, clip: self.clip_stack.last().cloned(), foreground_of: self.active_foreground_of(), ..DrawLayer::default() });
    }

    /// 🪟️ Clips subsequent draw content to an exact union of non-overlapping rectangles.
    pub fn begin_silhouette_clip(&mut self, rects: &[crate::wgpu::geometry::Rect]) {
        let mut clip = ClipRegion::from_rects(rects, self.screen_h);
        if let Some(parent) = self.clip_stack.last() {
            clip = parent.intersect(&clip);
        }
        self.clip_stack.push(clip.clone());
        self.layers.push(DrawLayer { scissor: self.scissor_stack.last().copied(), clip: Some(clip), foreground_of: self.active_foreground_of(), ..DrawLayer::default() });
    }

    pub fn end_silhouette_clip(&mut self) {
        self.clip_stack.pop();
        self.layers.push(DrawLayer { scissor: self.scissor_stack.last().copied(), clip: self.clip_stack.last().cloned(), foreground_of: self.active_foreground_of(), ..DrawLayer::default() });
    }

    pub fn push_scene_pass(&mut self, mut pass: ScenePass3d) {
        if self.layers.is_empty() {
            self.layers.push(DrawLayer::default());
        }
        let layer_index = self.layers.len() - 1;
        let layer = &self.layers[layer_index];
        pass.layer_index = layer_index;
        pass.ui_watermark = layer.ui_instances.len();
        pass.vector_watermark = layer.vector_vertices.len();
        self.scene_passes.push(pass);
    }

    pub fn push_solid(&mut self, rect: [f32; 4], color: Rgba) {
        self.active_layer().ui_instances.push(UiInstance::solid(rect, color));
    }

    pub fn push_rounded(&mut self, rect: [f32; 4], color: Rgba, radius: f32) {
        self.active_layer().ui_instances.push(UiInstance::rounded(rect, color, radius, 0.0, color));
    }

    /// 🌀️ Clockwise spinning + pulsing loading ring around `rect`, in `color` (gray `theme.border_normal` at rest, `theme.selected` when the node is selected/active).
    pub fn push_loading_border(&mut self, rect: [f32; 4], color: Rgba, radius: f32, stroke: f32) {
        self.active_layer().ui_instances.push(UiInstance::loading_border(rect, color, radius, stroke));
    }

    /// 🌀️ Dashed, slow-spinning + gently pulsing waiting ring around `rect`, in `color` (gray `theme.border_normal` at rest, `theme.selected` when the node is selected/active).
    pub fn push_waiting_border(&mut self, rect: [f32; 4], color: Rgba, radius: f32, stroke: f32) {
        self.active_layer().ui_instances.push(UiInstance::waiting_border(rect, color, radius, stroke));
    }

    /// ✅️ Solid, static at-bounds ring around `rect`, in `color` — `UiStatus::Finished`.
    pub fn push_finished_border(&mut self, rect: [f32; 4], color: Rgba, radius: f32, stroke: f32) {
        self.active_layer().ui_instances.push(UiInstance::finished_border(rect, color, radius, stroke));
    }

    /// 💫️ Raised-cosine breathing pulse ring around `rect`, in `color` — `UiState::Introducing`.
    pub fn push_introducing_border(&mut self, rect: [f32; 4], color: Rgba, radius: f32, stroke: f32) {
        self.active_layer().ui_instances.push(UiInstance::introducing_border(rect, color, radius, stroke));
    }

    /// 🧊️ Pushes a glass region rendered with an already-resolved `style` — callers derive `style`
    /// from `Theme::glass(level)` themselves (see
    /// `.🦑️repo/🎫️tickets/26/07/27/UNIFIED-6-LEVEL-UI-SURFACE-SYSTEM/contract.txt`) rather than this method
    /// picking a per-tier lookup.
    pub fn push_glass(&mut self, rect: [f32; 4], radius: f32, style: GlassStyle) -> usize {
        let index = self.glass_regions.len();
        self.glass_regions.push(GlassRegion { rect, radius, tint: style.tint, alpha: style.alpha, blur_px: style.blur_px, saturate: style.saturate });
        index
    }

    pub fn begin_glass_content(&mut self, region: usize) {
        self.glass_content_stack.push(region);
        self.layers.push(DrawLayer { scissor: self.scissor_stack.last().copied(), clip: self.clip_stack.last().cloned(), foreground_of: Some(region), ..DrawLayer::default() });
    }

    pub fn end_glass_content(&mut self) {
        self.glass_content_stack.pop();
        self.layers.push(DrawLayer { scissor: self.scissor_stack.last().copied(), clip: self.clip_stack.last().cloned(), foreground_of: self.active_foreground_of(), ..DrawLayer::default() });
    }

    pub fn push_glyph(&mut self, rect: [f32; 4], color: Rgba, uv_rect: [f32; 4]) {
        self.active_layer().ui_instances.push(UiInstance::glyph(rect, color, uv_rect));
    }

    pub fn push_glyph_overlay(&mut self, rect: [f32; 4], color: Rgba, uv_rect: [f32; 4]) {
        self.active_layer().overlay_ui_instances.push(UiInstance::glyph(rect, color, uv_rect));
    }

    pub fn push_solid_overlay(&mut self, rect: [f32; 4], color: Rgba) {
        self.active_layer().overlay_ui_instances.push(UiInstance::solid(rect, color));
    }

    pub fn push_textured(&mut self, rect: [f32; 4], uv_rect: [f32; 4], color: Rgba) {
        self.active_layer().ui_instances.push(UiInstance::textured(rect, uv_rect, color));
    }

    pub fn push_raster_quad(&mut self, key: &str, rect: [f32; 4], uv_rect: [f32; 4], alpha: f32) {
        self.active_layer().raster_instances.push((key.to_string(), UiInstance::raster(rect, uv_rect, alpha)));
    }

    pub fn push_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgba, width: f32) {
        let dx = x1 - x0;
        let dy = y1 - y0;
        let len = (dx * dx + dy * dy).sqrt().max(0.001);
        let nx = -dy / len * width * 0.5;
        let ny = dx / len * width * 0.5;
        let c = [color.r, color.g, color.b, color.a];
        let layer = self.active_layer();
        layer.vector_vertices.extend_from_slice(&[
            VectorVertex { position: [x0 + nx, y0 + ny], color: c },
            VectorVertex { position: [x1 + nx, y1 + ny], color: c },
            VectorVertex { position: [x0 - nx, y0 - ny], color: c },
            VectorVertex { position: [x1 + nx, y1 + ny], color: c },
            VectorVertex { position: [x1 - nx, y1 - ny], color: c },
            VectorVertex { position: [x0 - nx, y0 - ny], color: c },
        ]);
    }

    pub fn push_line_overlay(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgba, width: f32) {
        let dx = x1 - x0;
        let dy = y1 - y0;
        let len = (dx * dx + dy * dy).sqrt().max(0.001);
        let nx = -dy / len * width * 0.5;
        let ny = dx / len * width * 0.5;
        let c = [color.r, color.g, color.b, color.a];
        let layer = self.active_layer();
        layer.overlay_vector_vertices.extend_from_slice(&[
            VectorVertex { position: [x0 + nx, y0 + ny], color: c },
            VectorVertex { position: [x1 + nx, y1 + ny], color: c },
            VectorVertex { position: [x0 - nx, y0 - ny], color: c },
            VectorVertex { position: [x1 + nx, y1 + ny], color: c },
            VectorVertex { position: [x1 - nx, y1 - ny], color: c },
            VectorVertex { position: [x0 - nx, y0 - ny], color: c },
        ]);
    }

    pub fn push_triangle_fan(&mut self, points: &[[f32; 2]], color: Rgba) {
        if points.len() < 3 {
            return;
        }
        let c = [color.r, color.g, color.b, color.a];
        let layer = self.active_layer();
        for tri in 1..points.len() - 1 {
            layer.vector_vertices.push(VectorVertex { position: points[0], color: c });
            layer.vector_vertices.push(VectorVertex { position: points[tri], color: c });
            layer.vector_vertices.push(VectorVertex { position: points[tri + 1], color: c });
        }
    }

    pub fn push_triangle_fan_overlay(&mut self, points: &[[f32; 2]], color: Rgba) {
        if points.len() < 3 {
            return;
        }
        let c = [color.r, color.g, color.b, color.a];
        let layer = self.active_layer();
        for tri in 1..points.len() - 1 {
            layer.overlay_vector_vertices.push(VectorVertex { position: points[0], color: c });
            layer.overlay_vector_vertices.push(VectorVertex { position: points[tri], color: c });
            layer.overlay_vector_vertices.push(VectorVertex { position: points[tri + 1], color: c });
        }
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per line endpoint/style attribute; grouping into a struct is a T2 restructure, out of scope")]
    pub fn push_dashed_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgba, width: f32, dash: f32, gap: f32) {
        for (sx0, sy0, sx1, sy1) in dashed_line_segments(x0, y0, x1, y1, dash, gap) {
            self.push_line(sx0, sy0, sx1, sy1, color, width);
        }
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per line endpoint/style attribute; grouping into a struct is a T2 restructure, out of scope")]
    pub fn push_dashed_line_overlay(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgba, width: f32, dash: f32, gap: f32) {
        for (sx0, sy0, sx1, sy1) in dashed_line_segments(x0, y0, x1, y1, dash, gap) {
            self.push_line_overlay(sx0, sy0, sx1, sy1, color, width);
        }
    }
}

pub const SELECTION_MARQUEE_STROKE_WIDTH: f32 = 1.5;
pub const SELECTION_MARQUEE_FILL_ALPHA: f32 = 0.12;
pub const SELECTION_MARQUEE_DASH_LEN: f32 = 5.0;
pub const SELECTION_MARQUEE_DASH_GAP: f32 = 4.0;

pub fn selection_marquee_stroke(theme: &Theme) -> Rgba {
    theme.selected
}

pub fn selection_marquee_fill(theme: &Theme) -> Rgba {
    theme.selected.with_alpha(SELECTION_MARQUEE_FILL_ALPHA)
}

fn dashed_line_segments(x0: f32, y0: f32, x1: f32, y1: f32, dash: f32, gap: f32) -> Vec<(f32, f32, f32, f32)> {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let len = (dx * dx + dy * dy).sqrt().max(0.001);
    let ux = dx / len;
    let uy = dy / len;
    let mut traveled = 0.0f32;
    let mut drawing = true;
    let mut segments = Vec::new();
    while traveled < len {
        let segment = if drawing { dash } else { gap };
        let next = (traveled + segment).min(len);
        if drawing {
            segments.push((x0 + ux * traveled, y0 + uy * traveled, x0 + ux * next, y0 + uy * next));
        }
        traveled = next;
        drawing = !drawing;
    }
    segments
}

#[cfg(test)]
mod selection_marquee_tests {
    use super::*;
    use crate::wgpu::theme::Theme;

    #[test]
    fn dashed_line_segments_emit_dashes_along_segment() {
        let segments = dashed_line_segments(0.0, 0.0, 20.0, 0.0, 5.0, 4.0);
        assert!(!segments.is_empty());
        let span: f32 = segments.iter().map(|(x0, _, x1, _)| x1 - x0).sum();
        assert!(span > 0.0 && span <= 20.0);
    }

    #[test]
    fn selection_marquee_colors_use_active_token_only() {
        let theme = Theme::default();
        assert_eq!(selection_marquee_stroke(&theme), theme.selected);
        assert_eq!(selection_marquee_fill(&theme).a, SELECTION_MARQUEE_FILL_ALPHA);
    }
}

#[allow(clippy::too_many_arguments, reason = "one arg per line endpoint/style attribute; grouping into a struct is a T2 restructure, out of scope")]
fn push_marquee_segment(draw: &mut DrawList, overlay: bool, x0: f32, y0: f32, x1: f32, y1: f32, stroke: Rgba, dashed: bool) {
    if dashed {
        if overlay {
            draw.push_dashed_line_overlay(x0, y0, x1, y1, stroke, SELECTION_MARQUEE_STROKE_WIDTH, SELECTION_MARQUEE_DASH_LEN, SELECTION_MARQUEE_DASH_GAP);
        } else {
            draw.push_dashed_line(x0, y0, x1, y1, stroke, SELECTION_MARQUEE_STROKE_WIDTH, SELECTION_MARQUEE_DASH_LEN, SELECTION_MARQUEE_DASH_GAP);
        }
    } else if overlay {
        draw.push_line_overlay(x0, y0, x1, y1, stroke, SELECTION_MARQUEE_STROKE_WIDTH);
    } else {
        draw.push_line(x0, y0, x1, y1, stroke, SELECTION_MARQUEE_STROKE_WIDTH);
    }
}

pub fn paint_selection_marquee(draw: &mut DrawList, theme: &Theme, crossing: bool, lasso: bool, points: &[[f32; 2]], overlay: bool) {
    if points.len() < 2 {
        return;
    }
    let stroke = selection_marquee_stroke(theme);
    let fill = selection_marquee_fill(theme);
    let dashed = crossing;
    if lasso {
        if points.len() >= 3 {
            if overlay {
                draw.push_triangle_fan_overlay(points, fill);
            } else {
                draw.push_triangle_fan(points, fill);
            }
        }
        for window in points.windows(2) {
            push_marquee_segment(draw, overlay, window[0][0], window[0][1], window[1][0], window[1][1], stroke, dashed);
        }
        let first = points[0];
        let last = points[points.len() - 1];
        push_marquee_segment(draw, overlay, last[0], last[1], first[0], first[1], stroke, dashed);
        return;
    }
    let start = points[0];
    let end = points[points.len() - 1];
    let rx = start[0].min(end[0]);
    let ry = start[1].min(end[1]);
    let rw = (end[0] - start[0]).abs();
    let rh = (end[1] - start[1]).abs();
    if overlay {
        draw.push_solid_overlay([rx, ry, rw, rh], fill);
    } else {
        draw.push_solid([rx, ry, rw, rh], fill);
    }
    push_marquee_segment(draw, overlay, start[0], start[1], end[0], start[1], stroke, dashed);
    push_marquee_segment(draw, overlay, end[0], start[1], end[0], end[1], stroke, dashed);
    push_marquee_segment(draw, overlay, end[0], end[1], start[0], end[1], stroke, dashed);
    push_marquee_segment(draw, overlay, start[0], end[1], start[0], start[1], stroke, dashed);
}

pub fn ear_clip_polygon(points: &[[f32; 2]]) -> Vec<[f32; 2]> {
    if points.len() < 3 {
        return Vec::new();
    }
    let mut indices: Vec<usize> = (0..points.len()).collect();
    let mut triangles = Vec::new();
    let mut guard = 0usize;
    while indices.len() > 3 && guard < points.len() * points.len() {
        guard += 1;
        let mut ear_found = false;
        for i in 0..indices.len() {
            let prev = indices[(i + indices.len() - 1) % indices.len()];
            let curr = indices[i];
            let next = indices[(i + 1) % indices.len()];
            let a = points[prev];
            let b = points[curr];
            let c = points[next];
            let cross = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
            if cross <= 0.0 {
                continue;
            }
            let mut contains = false;
            for &idx in &indices {
                if idx == prev || idx == curr || idx == next {
                    continue;
                }
                let p = points[idx];
                if point_in_triangle(p, a, b, c) {
                    contains = true;
                    break;
                }
            }
            if contains {
                continue;
            }
            triangles.push(a);
            triangles.push(b);
            triangles.push(c);
            indices.remove(i);
            ear_found = true;
            break;
        }
        if !ear_found {
            break;
        }
    }
    if indices.len() == 3 {
        triangles.push(points[indices[0]]);
        triangles.push(points[indices[1]]);
        triangles.push(points[indices[2]]);
    }
    triangles
}

fn point_in_triangle(p: [f32; 2], a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> bool {
    let d1 = sign(p, a, b);
    let d2 = sign(p, b, c);
    let d3 = sign(p, c, a);
    let has_neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let has_pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
    !(has_neg && has_pos)
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct World3dVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct World3dGlobals {
    pub view_proj: [f32; 16],
    pub light_dir: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct World3dGpuInstance {
    pub model0: [f32; 4],
    pub model1: [f32; 4],
    pub model2: [f32; 4],
    pub model3: [f32; 4],
    pub color: [f32; 4],
    pub flags: [f32; 4],
}

impl World3dGpuInstance {
    pub fn from_instance(model: [f32; 16], color: [f32; 4], selected: bool, hovered: bool) -> Self {
        Self {
            model0: [model[0], model[1], model[2], model[3]],
            model1: [model[4], model[5], model[6], model[7]],
            model2: [model[8], model[9], model[10], model[11]],
            model3: [model[12], model[13], model[14], model[15]],
            color,
            flags: [if selected { 1.0 } else { 0.0 }, if hovered { 1.0 } else { 0.0 }, 0.0, 0.0],
        }
    }
}

pub struct GpuMeshBuffers {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

#[derive(Default)]
pub struct MeshGpuTable {
    meshes: std::collections::HashMap<String, GpuMeshBuffers>,
}

pub fn mesh_content_version(positions: &[f32], normals: &[f32], indices: &[u32]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for value in positions.iter().chain(normals.iter()) {
        hash ^= value.to_bits() as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for value in indices {
        hash ^= *value as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

impl MeshGpuTable {
    pub fn get(&self, key: &str) -> Option<&GpuMeshBuffers> {
        self.meshes.get(key)
    }

    pub fn lookup_key(mesh_key: &str, version: u64) -> String {
        format!("{mesh_key}:{version}")
    }

    pub fn get_versioned(&self, mesh_key: &str, version: u64) -> Option<&GpuMeshBuffers> {
        self.get(&Self::lookup_key(mesh_key, version))
    }

    pub fn ensure_mesh(&mut self, device: &wgpu::Device, key: &str, version: u64, positions: &[f32], normals: &[f32], indices: &[u32]) {
        let store_key = format!("{key}:{version}");
        if self.meshes.contains_key(&store_key) {
            return;
        }
        let prefix = format!("{key}:");
        self.meshes.retain(|existing, _| !existing.starts_with(&prefix) || existing == &store_key);
        let mut vertices = Vec::with_capacity(positions.len() / 3);
        for index in 0..positions.len() / 3 {
            vertices.push(World3dVertex {
                position: [positions[index * 3], positions[index * 3 + 1], positions[index * 3 + 2]],
                normal: [normals.get(index * 3).copied().unwrap_or(0.0), normals.get(index * 3 + 1).copied().unwrap_or(1.0), normals.get(index * 3 + 2).copied().unwrap_or(0.0)],
            });
        }
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("world3d_vertices"), contents: bytemuck::cast_slice(&vertices), usage: wgpu::BufferUsages::VERTEX });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("world3d_indices"), contents: bytemuck::cast_slice(indices), usage: wgpu::BufferUsages::INDEX });
        self.meshes.insert(store_key, GpuMeshBuffers { vertex_buffer, index_buffer, index_count: indices.len() as u32 });
    }

    pub fn evict_mesh(&mut self, key: &str) {
        let prefix = format!("{key}:");
        self.meshes.retain(|existing, _| !existing.starts_with(&prefix));
    }
}

pub const WORLD_GLOBALS_SLOT_SIZE: u64 = 256;

#[derive(Default)]
pub struct GrowBuffer {
    buffer: Option<wgpu::Buffer>,
    capacity: usize,
}

impl GrowBuffer {
    pub fn slice(&self) -> Option<wgpu::BufferSlice<'_>> {
        self.buffer.as_ref().map(|buffer| buffer.slice(..))
    }

    pub fn upload<T: Pod>(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, data: &[T], usage: wgpu::BufferUsages, label: &str) -> Option<wgpu::BufferSlice<'_>> {
        if data.is_empty() {
            return None;
        }
        let bytes = bytemuck::cast_slice(data);
        let required = bytes.len();
        if self.capacity < required {
            self.capacity = required.next_power_of_two().max(256);
            self.buffer = Some(device.create_buffer(&wgpu::BufferDescriptor { label: Some(label), size: self.capacity as u64, usage, mapped_at_creation: false }));
        }
        let buffer = self.buffer.as_ref()?;
        queue.write_buffer(buffer, 0, bytes);
        Some(buffer.slice(..))
    }
}

#[derive(Default)]
pub struct FrameBuffers {
    pub world_instances: GrowBuffer,
    pub world_lines: GrowBuffer,
    pub ui_instances: GrowBuffer,
    pub mask_instances: GrowBuffer,
    pub vector_vertices: GrowBuffer,
    pub glass_instances: GrowBuffer,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct WorldLineGpuVertex {
    position: [f32; 3],
    color: [f32; 4],
}

struct WorldDrawRange {
    mesh_key: String,
    mesh_version: u64,
    instance_offset: u32,
    instance_count: u32,
}

struct PreparedWorldPass {
    globals: World3dGlobals,
    viewport: [f32; 4],
    draws: Vec<WorldDrawRange>,
    translucent_draws: Vec<WorldDrawRange>,
    line_start: u32,
    line_count: u32,
}

struct WorldGlobalsRing {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    slot_stride: u32,
    capacity_slots: u32,
}

impl WorldGlobalsRing {
    fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, initial_slots: u32) -> Self {
        let slot_stride = WORLD_GLOBALS_SLOT_SIZE as u32;
        let capacity_slots = initial_slots.max(1);
        let buffer =
            device.create_buffer(&wgpu::BufferDescriptor { label: Some("world3d_globals_ring"), size: slot_stride as u64 * capacity_slots as u64, usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("world3d_bind_group"),
            layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding { buffer: &buffer, offset: 0, size: std::num::NonZeroU64::new(size_of::<World3dGlobals>() as u64) }) }],
        });
        Self { buffer, bind_group, slot_stride, capacity_slots }
    }

    fn ensure_slots(&mut self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout, slots: u32) {
        if slots <= self.capacity_slots {
            return;
        }
        self.capacity_slots = slots.next_power_of_two().max(4);
        self.buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("world3d_globals_ring"),
            size: self.slot_stride as u64 * self.capacity_slots as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("world3d_bind_group"),
            layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding { buffer: &self.buffer, offset: 0, size: std::num::NonZeroU64::new(size_of::<World3dGlobals>() as u64) }) }],
        });
    }

    fn write_passes(&self, queue: &wgpu::Queue, passes: &[World3dGlobals]) {
        for (index, globals) in passes.iter().enumerate() {
            let offset = (index as u64) * self.slot_stride as u64;
            queue.write_buffer(&self.buffer, offset, bytemuck::bytes_of(globals));
        }
    }

    fn offset_for_slot(&self, slot: u32) -> u32 {
        slot * self.slot_stride
    }
}

fn sign(p1: [f32; 2], p2: [f32; 2], p3: [f32; 2]) -> f32 {
    (p1[0] - p3[0]) * (p2[1] - p3[1]) - (p2[0] - p3[0]) * (p1[1] - p3[1])
}

pub const ICON_ATLAS_TEXTURE_SIZE: u32 = 2048;

pub struct IconAtlas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
    entries: std::collections::HashMap<String, [f32; 4]>,
}

impl Default for IconAtlas {
    fn default() -> Self {
        Self { width: 1, height: 1, pixels: vec![0, 0, 0, 0], entries: std::collections::HashMap::new() }
    }
}

impl IconAtlas {
    pub fn from_packed(width: u32, height: u32, pixels: Vec<u8>, entries: Vec<(String, [f32; 4])>) -> Self {
        Self { width, height, pixels, entries: entries.into_iter().collect() }
    }

    pub fn icon_uv(&self, icon_id: &str) -> Option<[f32; 4]> {
        self.entries.get(icon_id).copied()
    }
}

pub struct RasterTexture {
    pub texture: wgpu::Texture,
    pub bind_group: wgpu::BindGroup,
    pub width: u32,
    pub height: u32,
}

pub struct RasterTextureTable {
    textures: std::collections::HashMap<String, RasterTexture>,
    layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
}

impl RasterTextureTable {
    pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor { label: Some("raster_sampler"), mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, ..Default::default() });
        Self { textures: std::collections::HashMap::new(), layout: layout.clone(), sampler }
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    pub fn ensure_raster(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        globals_buffer: &wgpu::Buffer,
        glyph_view: &wgpu::TextureView,
        glyph_sampler: &wgpu::Sampler,
        _icon_view: &wgpu::TextureView,
        _icon_sampler: &wgpu::Sampler,
        key: &str,
        pixels: &[u8],
        width: u32,
        height: u32,
    ) {
        if let Some(existing) = self.textures.get(key) {
            queue.write_texture(
                wgpu::TexelCopyTextureInfo { texture: &existing.texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                pixels,
                wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(width * 4), rows_per_image: Some(height) },
                wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            );
            return;
        }
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("raster_texture"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        queue.write_texture(
            wgpu::TexelCopyTextureInfo { texture: &texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            pixels,
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(width * 4), rows_per_image: Some(height) },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("raster_texture_bind_group"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: globals_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(glyph_view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(glyph_sampler) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&view) },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            ],
        });
        self.textures.insert(key.to_string(), RasterTexture { texture, bind_group, width, height });
    }

    pub fn get(&self, key: &str) -> Option<&RasterTexture> {
        self.textures.get(key)
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    pub fn replace_gpu_bind_group(
        &mut self,
        device: &wgpu::Device,
        globals_buffer: &wgpu::Buffer,
        glyph_view: &wgpu::TextureView,
        glyph_sampler: &wgpu::Sampler,
        key: &str,
        raster_view: &wgpu::TextureView,
        texture: wgpu::Texture,
        width: u32,
        height: u32,
    ) {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("raster_bind_group"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: globals_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(glyph_view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(glyph_sampler) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(raster_view) },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            ],
        });
        self.textures.insert(key.to_string(), RasterTexture { texture, bind_group, width, height });
    }
}

pub(crate) struct UiPipelines {
    mask_pipeline: wgpu::RenderPipeline,
    ui_pipeline: wgpu::RenderPipeline,
    vector_pipeline: wgpu::RenderPipeline,
    world_pipeline: wgpu::RenderPipeline,
    world_pipeline_translucent: wgpu::RenderPipeline,
    world_line_pipeline: wgpu::RenderPipeline,
    blur_downsample_pipeline: wgpu::RenderPipeline,
    scene_blit_pipeline: wgpu::RenderPipeline,
    glass_pipeline: wgpu::RenderPipeline,
    quad_vertex_buffer: wgpu::Buffer,
    globals_buffer: wgpu::Buffer,
    blur_globals_buffer: wgpu::Buffer,
    world_globals_ring: WorldGlobalsRing,
    world_bind_group_layout: wgpu::BindGroupLayout,
    blur_bind_group_layout: wgpu::BindGroupLayout,
    scene_bind_group_layout: wgpu::BindGroupLayout,
    glyph_texture: wgpu::Texture,
    glyph_sampler: wgpu::Sampler,
    icon_texture: wgpu::Texture,
    icon_sampler: wgpu::Sampler,
    glyph_bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
}

struct LayerBatch {
    layer_index: usize,
    scissor: Option<ScissorRect>,
    clip: Option<ClipRegion>,
    ui_start: u32,
    ui_count: u32,
    vec_start: u32,
    vec_count: u32,
}

enum LayerBatchFilter {
    Backdrop,
    Foreground,
}

impl Copy for LayerBatchFilter {}

impl Clone for LayerBatchFilter {
    fn clone(&self) -> Self {
        *self
    }
}

fn layer_matches_filter(layer: &DrawLayer, filter: LayerBatchFilter) -> bool {
    match filter {
        LayerBatchFilter::Backdrop => layer.foreground_of.is_none(),
        LayerBatchFilter::Foreground => layer.foreground_of.is_some(),
    }
}

fn build_layer_batches(draw: &DrawList, filter: LayerBatchFilter) -> (Vec<UiInstance>, Vec<VectorVertex>, Vec<LayerBatch>) {
    let mut all_ui = Vec::new();
    let mut all_vec = Vec::new();
    let mut batches = Vec::new();
    let scene_layers: std::collections::HashSet<usize> = draw.scene_passes.iter().filter(|pass| layer_matches_filter(&draw.layers[pass.layer_index], filter)).map(|pass| pass.layer_index).collect();
    for (layer_index, layer) in draw.layers.iter().enumerate() {
        if !layer_matches_filter(layer, filter) {
            continue;
        }
        if layer.ui_instances.is_empty() && layer.vector_vertices.is_empty() && !scene_layers.contains(&layer_index) {
            continue;
        }
        let ui_start = all_ui.len() as u32;
        all_ui.extend_from_slice(&layer.ui_instances);
        let vec_start = all_vec.len() as u32;
        all_vec.extend_from_slice(&layer.vector_vertices);
        batches.push(LayerBatch { layer_index, scissor: layer.scissor, clip: layer.clip.clone(), ui_start, ui_count: layer.ui_instances.len() as u32, vec_start, vec_count: layer.vector_vertices.len() as u32 });
    }
    (all_ui, all_vec, batches)
}

fn build_overlay_layer_batches(draw: &DrawList, filter: LayerBatchFilter) -> (Vec<UiInstance>, Vec<VectorVertex>, Vec<LayerBatch>) {
    let mut all_ui = Vec::new();
    let mut all_vec = Vec::new();
    let mut batches = Vec::new();
    for (layer_index, layer) in draw.layers.iter().enumerate() {
        if !layer_matches_filter(layer, filter) {
            continue;
        }
        if layer.overlay_ui_instances.is_empty() && layer.overlay_vector_vertices.is_empty() {
            continue;
        }
        let ui_start = all_ui.len() as u32;
        all_ui.extend_from_slice(&layer.overlay_ui_instances);
        let vec_start = all_vec.len() as u32;
        all_vec.extend_from_slice(&layer.overlay_vector_vertices);
        batches.push(LayerBatch { layer_index, scissor: layer.scissor, clip: layer.clip.clone(), ui_start, ui_count: layer.overlay_ui_instances.len() as u32, vec_start, vec_count: layer.overlay_vector_vertices.len() as u32 });
    }
    (all_ui, all_vec, batches)
}

fn set_pass_scissor(pass: &mut wgpu::RenderPass<'_>, scissor: Option<ScissorRect>, width: f32, height: f32) {
    if let Some(scissor) = scissor {
        pass.set_scissor_rect(scissor.x, scissor.y, scissor.w, scissor.h);
    } else {
        pass.set_scissor_rect(0, 0, width as u32, height as u32);
    }
}

fn layer_scissors(scissor: Option<ScissorRect>, clip: Option<&ClipRegion>, width: f32, height: f32) -> Vec<Option<ScissorRect>> {
    if let Some(clip) = clip {
        return clip.effective_scissors(scissor, width, height).into_iter().map(Some).collect();
    }
    let viewport = ScissorRect { x: 0, y: 0, w: width.max(0.0) as u32, h: height.max(0.0) as u32 };
    match scissor.map(|value| value.intersect(&viewport)) {
        Some(value) if value.w > 0 && value.h > 0 => vec![Some(value)],
        Some(_) => Vec::new(),
        None => vec![None],
    }
}

fn content_stencil_state() -> wgpu::StencilState {
    let face = wgpu::StencilFaceState { compare: wgpu::CompareFunction::Equal, fail_op: wgpu::StencilOperation::Keep, depth_fail_op: wgpu::StencilOperation::Keep, pass_op: wgpu::StencilOperation::Keep };
    wgpu::StencilState { front: face, back: face, read_mask: 0xff, write_mask: 0x00 }
}

fn mask_stencil_state() -> wgpu::StencilState {
    let face = wgpu::StencilFaceState { compare: wgpu::CompareFunction::Always, fail_op: wgpu::StencilOperation::Replace, depth_fail_op: wgpu::StencilOperation::Replace, pass_op: wgpu::StencilOperation::Replace };
    wgpu::StencilState { front: face, back: face, read_mask: 0xff, write_mask: 0xff }
}

fn stencil_attachment<'a>(view: &'a wgpu::TextureView, depth_load: wgpu::LoadOp<f32>, stencil_load: wgpu::LoadOp<u32>) -> wgpu::RenderPassDepthStencilAttachment<'a> {
    wgpu::RenderPassDepthStencilAttachment { view, depth_ops: Some(wgpu::Operations { load: depth_load, store: wgpu::StoreOp::Store }), stencil_ops: Some(wgpu::Operations { load: stencil_load, store: wgpu::StoreOp::Store }) }
}

fn union_scissors(scissors: &[ScissorRect]) -> Option<ScissorRect> {
    let first = *scissors.first()?;
    let (mut x0, mut y0, mut x1, mut y1) = (first.x, first.y, first.x + first.w, first.y + first.h);
    for scissor in &scissors[1..] {
        x0 = x0.min(scissor.x);
        y0 = y0.min(scissor.y);
        x1 = x1.max(scissor.x + scissor.w);
        y1 = y1.max(scissor.y + scissor.h);
    }
    Some(ScissorRect { x: x0, y: y0, w: x1 - x0, h: y1 - y0 })
}

fn merge_scissor_bounds(a: Option<ScissorRect>, b: Option<ScissorRect>) -> Option<ScissorRect> {
    match (a, b) {
        (Some(a), Some(b)) => union_scissors(&[a, b]),
        (Some(value), None) | (None, Some(value)) => Some(value),
        (None, None) => None,
    }
}

fn mask_instances(scissor: Option<ScissorRect>, clip: Option<&ClipRegion>, previous_bounds: Option<ScissorRect>, width: f32, height: f32) -> (Vec<UiInstance>, Option<ScissorRect>) {
    let white = Rgba::new(1.0, 1.0, 1.0, 1.0);
    let viewport = ScissorRect { x: 0, y: 0, w: width.max(0.0) as u32, h: height.max(0.0) as u32 };
    let pieces: Vec<ScissorRect> = layer_scissors(scissor, clip, width, height).into_iter().map(|piece| piece.unwrap_or(viewport)).collect();
    let current_bounds = union_scissors(&pieces);
    let Some(reset_bounds) = merge_scissor_bounds(previous_bounds, current_bounds) else {
        return (Vec::new(), None);
    };
    let mut instances = vec![UiInstance::solid([reset_bounds.x as f32, reset_bounds.y as f32, reset_bounds.w as f32, reset_bounds.h as f32], white)];
    instances.extend(pieces.into_iter().map(|piece| UiInstance::solid([piece.x as f32, piece.y as f32, piece.w as f32, piece.h as f32], white)));
    (instances, current_bounds)
}

fn build_batch_masks(batches: &[LayerBatch], width: f32, height: f32) -> (Vec<UiInstance>, Vec<(u32, u32)>) {
    let mut instances = Vec::new();
    let mut ranges = Vec::with_capacity(batches.len());
    let mut previous_bounds = None;
    for batch in batches {
        let start = instances.len() as u32;
        let (batch_instances, current_bounds) = mask_instances(batch.scissor, batch.clip.as_ref(), previous_bounds, width, height);
        let count = batch_instances.len() as u32;
        instances.extend(batch_instances);
        ranges.push((start, count));
        previous_bounds = current_bounds;
    }
    (instances, ranges)
}

impl UiPipelines {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let globals_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ui_globals_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry { binding: 4, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
            ],
        });

        let glyph_bind_group_layout = globals_bind_group_layout.clone();
        let _ = glyph_bind_group_layout;

        let ui_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("ui_shader"), source: wgpu::ShaderSource::Wgsl(UI_SHADER.into()) });
        let vector_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("vector_shader"), source: wgpu::ShaderSource::Wgsl(VECTOR_SHADER.into()) });
        let world_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("world3d_shader"), source: wgpu::ShaderSource::Wgsl(WORLD3D_SHADER.into()) });
        let world_lines_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("world3d_lines_shader"), source: wgpu::ShaderSource::Wgsl(WORLD3D_LINES_SHADER.into()) });

        let depth_state =
            Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth24PlusStencil8, depth_write_enabled: true, depth_compare: wgpu::CompareFunction::Less, stencil: content_stencil_state(), bias: wgpu::DepthBiasState::default() });
        let overlay_depth_state =
            Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth24PlusStencil8, depth_write_enabled: false, depth_compare: wgpu::CompareFunction::Always, stencil: content_stencil_state(), bias: wgpu::DepthBiasState::default() });

        let quad_vertices: &[f32] = &[0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0];
        let quad_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("ui_quad_vertices"), contents: bytemuck::cast_slice(quad_vertices), usage: wgpu::BufferUsages::VERTEX });

        let globals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("ui_globals"),
            contents: bytemuck::bytes_of(&UiGlobals { screen_size: [1.0, 1.0], _pad: [0.0, 0.0] }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let glyph_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("glyph_atlas"),
            size: wgpu::Extent3d { width: ICON_ATLAS_TEXTURE_SIZE, height: ICON_ATLAS_TEXTURE_SIZE, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let glyph_view = glyph_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let glyph_sampler = device.create_sampler(&wgpu::SamplerDescriptor { label: Some("glyph_sampler"), mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, ..Default::default() });
        let icon_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("icon_atlas"),
            size: wgpu::Extent3d { width: ICON_ATLAS_TEXTURE_SIZE, height: ICON_ATLAS_TEXTURE_SIZE, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let icon_view = icon_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let icon_sampler = device.create_sampler(&wgpu::SamplerDescriptor { label: Some("icon_sampler"), mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, ..Default::default() });
        let glyph_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ui_bind_group"),
            layout: &globals_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: globals_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&glyph_view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&glyph_sampler) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&icon_view) },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::Sampler(&icon_sampler) },
            ],
        });
        let ui_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("ui_pipeline_layout"), bind_group_layouts: &[&globals_bind_group_layout], push_constant_ranges: &[] });
        let mask_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("silhouette_mask_pipeline"),
            layout: Some(&ui_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &ui_shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    wgpu::VertexBufferLayout { array_stride: 8, step_mode: wgpu::VertexStepMode::Vertex, attributes: &[wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x2 }] },
                    wgpu::VertexBufferLayout {
                        array_stride: size_of::<UiInstance>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute { offset: 0, shader_location: 1, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 16, shader_location: 2, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 32, shader_location: 3, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 48, shader_location: 4, format: wgpu::VertexFormat::Float32x4 },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState { module: &ui_shader, entry_point: Some("fs_main"), targets: &[Some(wgpu::ColorTargetState { format, blend: None, write_mask: wgpu::ColorWrites::empty() })], compilation_options: Default::default() }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Always,
                stencil: mask_stencil_state(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        let ui_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("ui_pipeline"),
            layout: Some(&ui_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &ui_shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    wgpu::VertexBufferLayout { array_stride: 8, step_mode: wgpu::VertexStepMode::Vertex, attributes: &[wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x2 }] },
                    wgpu::VertexBufferLayout {
                        array_stride: size_of::<UiInstance>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute { offset: 0, shader_location: 1, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 16, shader_location: 2, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 32, shader_location: 3, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 48, shader_location: 4, format: wgpu::VertexFormat::Float32x4 },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &ui_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: overlay_depth_state.clone(),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let vector_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("vector_pipeline_layout"), bind_group_layouts: &[&globals_bind_group_layout], push_constant_ranges: &[] });
        let vector_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("vector_pipeline"),
            layout: Some(&vector_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vector_shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: size_of::<VectorVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x2 }, wgpu::VertexAttribute { offset: 8, shader_location: 1, format: wgpu::VertexFormat::Float32x4 }],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &vector_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: overlay_depth_state,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let world_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("world3d_globals_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: true, min_binding_size: std::num::NonZeroU64::new(size_of::<World3dGlobals>() as u64) },
                count: None,
            }],
        });

        let world_globals_ring = WorldGlobalsRing::new(device, &world_bind_group_layout, 8);

        let world_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("world3d_pipeline_layout"), bind_group_layouts: &[&world_bind_group_layout], push_constant_ranges: &[] });
        let world_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("world3d_pipeline"),
            layout: Some(&world_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &world_shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: size_of::<World3dVertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x3 }, wgpu::VertexAttribute { offset: 12, shader_location: 1, format: wgpu::VertexFormat::Float32x3 }],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: size_of::<World3dGpuInstance>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute { offset: 0, shader_location: 3, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 16, shader_location: 4, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 32, shader_location: 5, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 48, shader_location: 6, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 64, shader_location: 7, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 80, shader_location: 8, format: wgpu::VertexFormat::Float32x4 },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &world_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::REPLACE), write_mask: wgpu::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: depth_state,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        let translucent_depth_state = Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth24PlusStencil8,
            depth_write_enabled: false,
            depth_compare: wgpu::CompareFunction::LessEqual,
            stencil: content_stencil_state(),
            bias: wgpu::DepthBiasState { constant: -2, slope_scale: -1.0, clamp: 0.0 },
        });
        let world_line_depth_state =
            Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth24PlusStencil8, depth_write_enabled: false, depth_compare: wgpu::CompareFunction::LessEqual, stencil: content_stencil_state(), bias: wgpu::DepthBiasState::default() });
        let world_pipeline_translucent = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("world3d_pipeline_translucent"),
            layout: Some(&world_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &world_shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: size_of::<World3dVertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x3 }, wgpu::VertexAttribute { offset: 12, shader_location: 1, format: wgpu::VertexFormat::Float32x3 }],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: size_of::<World3dGpuInstance>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute { offset: 0, shader_location: 3, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 16, shader_location: 4, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 32, shader_location: 5, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 48, shader_location: 6, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 64, shader_location: 7, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 80, shader_location: 8, format: wgpu::VertexFormat::Float32x4 },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &world_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState { cull_mode: Some(wgpu::Face::Back), ..Default::default() },
            depth_stencil: translucent_depth_state,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        let world_line_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("world3d_line_pipeline"),
            layout: Some(&world_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &world_lines_shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: size_of::<WorldLineGpuVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x3 }, wgpu::VertexAttribute { offset: 12, shader_location: 1, format: wgpu::VertexFormat::Float32x4 }],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &world_lines_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::LineList, ..Default::default() },
            depth_stencil: world_line_depth_state,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let blur_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("blur_downsample_shader"), source: wgpu::ShaderSource::Wgsl(BLUR_DOWNSAMPLE_SHADER.into()) });
        let scene_blit_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("scene_blit_shader"), source: wgpu::ShaderSource::Wgsl(SCENE_BLIT_SHADER.into()) });
        let glass_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("glass_shader"), source: wgpu::ShaderSource::Wgsl(GLASS_SHADER.into()) });

        let blur_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("blur_downsample_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: std::num::NonZeroU64::new(size_of::<BlurGlobals>() as u64) },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
            ],
        });

        let scene_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("scene_sample_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
            ],
        });

        let blur_globals_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("blur_globals"), contents: bytemuck::bytes_of(&BlurGlobals { src_mip: 0.0, _pad: [0.0; 7] }), usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST });

        let blur_downsample_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("blur_downsample_pipeline_layout"), bind_group_layouts: &[&blur_bind_group_layout], push_constant_ranges: &[] });
        let blur_downsample_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("blur_downsample_pipeline"),
            layout: Some(&blur_downsample_pipeline_layout),
            vertex: wgpu::VertexState { module: &blur_shader, entry_point: Some("vs_main"), buffers: &[], compilation_options: Default::default() },
            fragment: Some(wgpu::FragmentState {
                module: &blur_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::REPLACE), write_mask: wgpu::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let scene_blit_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("scene_blit_pipeline_layout"), bind_group_layouts: &[&scene_bind_group_layout], push_constant_ranges: &[] });
        let scene_blit_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("scene_blit_pipeline"),
            layout: Some(&scene_blit_pipeline_layout),
            vertex: wgpu::VertexState { module: &scene_blit_shader, entry_point: Some("vs_main"), buffers: &[], compilation_options: Default::default() },
            fragment: Some(wgpu::FragmentState {
                module: &scene_blit_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::REPLACE), write_mask: wgpu::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let glass_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("glass_pipeline_layout"), bind_group_layouts: &[&globals_bind_group_layout, &scene_bind_group_layout], push_constant_ranges: &[] });
        let glass_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("glass_pipeline"),
            layout: Some(&glass_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &glass_shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    wgpu::VertexBufferLayout { array_stride: 8, step_mode: wgpu::VertexStepMode::Vertex, attributes: &[wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x2 }] },
                    wgpu::VertexBufferLayout {
                        array_stride: size_of::<GlassInstance>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute { offset: 0, shader_location: 1, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 16, shader_location: 2, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 32, shader_location: 3, format: wgpu::VertexFormat::Float32x4 },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &glass_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let _ = queue;
        Self {
            mask_pipeline,
            ui_pipeline,
            vector_pipeline,
            world_pipeline,
            world_pipeline_translucent,
            world_line_pipeline,
            blur_downsample_pipeline,
            scene_blit_pipeline,
            glass_pipeline,
            quad_vertex_buffer,
            globals_buffer,
            blur_globals_buffer,
            world_globals_ring,
            world_bind_group_layout,
            blur_bind_group_layout,
            scene_bind_group_layout,
            glyph_texture,
            glyph_sampler,
            icon_texture,
            icon_sampler,
            glyph_bind_group,
            bind_group_layout: globals_bind_group_layout,
        }
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn globals_buffer(&self) -> &wgpu::Buffer {
        &self.globals_buffer
    }

    pub fn glyph_view(&self) -> wgpu::TextureView {
        self.glyph_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub fn glyph_sampler(&self) -> &wgpu::Sampler {
        &self.glyph_sampler
    }

    pub fn icon_view(&self) -> wgpu::TextureView {
        self.icon_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub fn icon_sampler(&self) -> &wgpu::Sampler {
        &self.icon_sampler
    }

    pub fn depth_format(&self) -> wgpu::TextureFormat {
        wgpu::TextureFormat::Depth24PlusStencil8
    }

    fn prepare_world_passes(draw: &DrawList, filter: LayerBatchFilter) -> (Vec<PreparedWorldPass>, Vec<World3dGpuInstance>, Vec<WorldLineGpuVertex>, Vec<Option<usize>>) {
        let mut prepared = Vec::new();
        let mut all_instances = Vec::new();
        let mut all_lines = Vec::new();
        let mut pass_index_map = vec![None; draw.scene_passes.len()];
        for (source_index, scene) in draw.scene_passes.iter().enumerate() {
            if !layer_matches_filter(&draw.layers[scene.layer_index], filter) {
                continue;
            }
            let mut pass_draws = Vec::new();
            for draw_call in &scene.draws {
                if draw_call.instances.is_empty() {
                    continue;
                }
                let instance_offset = all_instances.len() as u32;
                let instance_count = draw_call.instances.len() as u32;
                for instance in &draw_call.instances {
                    all_instances.push(World3dGpuInstance::from_instance(instance.model.to_cols_array_m(), instance.color, instance.selected, instance.hovered));
                }
                pass_draws.push(WorldDrawRange { mesh_key: draw_call.mesh_key.clone(), mesh_version: draw_call.mesh_version, instance_offset, instance_count });
            }
            let mut translucent_draws = Vec::new();
            for draw_call in &scene.translucent_draws {
                if draw_call.instances.is_empty() {
                    continue;
                }
                let instance_offset = all_instances.len() as u32;
                let instance_count = draw_call.instances.len() as u32;
                for instance in &draw_call.instances {
                    all_instances.push(World3dGpuInstance::from_instance(instance.model.to_cols_array_m(), instance.color, instance.selected, instance.hovered));
                }
                translucent_draws.push(WorldDrawRange { mesh_key: draw_call.mesh_key.clone(), mesh_version: draw_call.mesh_version, instance_offset, instance_count });
            }
            let line_start = all_lines.len() as u32;
            for line_draw in &scene.line_draws {
                for vertex in &line_draw.vertices {
                    all_lines.push(WorldLineGpuVertex { position: vertex.position, color: vertex.color });
                }
            }
            let line_count = all_lines.len() as u32 - line_start;
            pass_index_map[source_index] = Some(prepared.len());
            prepared.push(PreparedWorldPass {
                globals: World3dGlobals { view_proj: scene.view_proj, light_dir: [scene.light_dir[0], scene.light_dir[1], scene.light_dir[2], 0.0] },
                viewport: scene.viewport,
                draws: pass_draws,
                translucent_draws,
                line_start,
                line_count,
            });
        }
        (prepared, all_instances, all_lines, pass_index_map)
    }

    fn upload_world_passes(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, draw: &DrawList, frame_buffers: &mut FrameBuffers, filter: LayerBatchFilter) -> Option<(Vec<PreparedWorldPass>, Vec<Option<usize>>)> {
        if draw.scene_passes.is_empty() {
            return None;
        }
        let (prepared, all_instances, all_lines, pass_index_map) = Self::prepare_world_passes(draw, filter);
        if prepared.is_empty() {
            return None;
        }
        if all_instances.is_empty() && all_lines.is_empty() {
            return Some((prepared, pass_index_map));
        }
        self.world_globals_ring.ensure_slots(device, &self.world_bind_group_layout, prepared.len() as u32);
        let globals: Vec<World3dGlobals> = prepared.iter().map(|pass| pass.globals).collect();
        self.world_globals_ring.write_passes(queue, &globals);
        if !all_instances.is_empty() {
            frame_buffers.world_instances.upload(device, queue, &all_instances, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "world3d_instances");
        }
        if !all_lines.is_empty() {
            frame_buffers.world_lines.upload(device, queue, &all_lines, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "world3d_lines");
        }
        Some((prepared, pass_index_map))
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    fn draw_world_pass_at<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        mesh_store: &MeshGpuTable,
        prepared: &PreparedWorldPass,
        slot: u32,
        instance_buffer: wgpu::BufferSlice<'a>,
        line_buffer: Option<wgpu::BufferSlice<'a>>,
        screen_w: f32,
        screen_h: f32,
        clip: Option<ScissorRect>,
    ) {
        let instance_stride = size_of::<World3dGpuInstance>() as u64;
        pass.set_pipeline(&self.world_pipeline);
        let viewport = prepared.viewport;
        pass.set_viewport(viewport[0], viewport[1], viewport[2], viewport[3], 0.0, 1.0);
        let scene_scissor = ScissorRect { x: viewport[0] as u32, y: viewport[1] as u32, w: viewport[2] as u32, h: viewport[3] as u32 };
        let scene_scissor = clip.map_or(scene_scissor, |clip| scene_scissor.intersect(&clip));
        if scene_scissor.w == 0 || scene_scissor.h == 0 {
            pass.set_viewport(0.0, 0.0, screen_w, screen_h, 0.0, 1.0);
            set_pass_scissor(pass, clip, screen_w, screen_h);
            pass.set_pipeline(&self.ui_pipeline);
            pass.set_bind_group(0, &self.glyph_bind_group, &[]);
            return;
        }
        pass.set_scissor_rect(scene_scissor.x, scene_scissor.y, scene_scissor.w, scene_scissor.h);
        pass.set_bind_group(0, &self.world_globals_ring.bind_group, &[self.world_globals_ring.offset_for_slot(slot)]);
        for draw_call in &prepared.draws {
            Self::draw_world_range(pass, mesh_store, draw_call, instance_buffer, instance_stride);
        }
        if prepared.line_count > 0 {
            if let Some(line_buffer) = line_buffer {
                pass.set_pipeline(&self.world_line_pipeline);
                pass.set_bind_group(0, &self.world_globals_ring.bind_group, &[self.world_globals_ring.offset_for_slot(slot)]);
                let line_stride = size_of::<WorldLineGpuVertex>() as u64;
                let byte_offset = prepared.line_start as u64 * line_stride;
                pass.set_vertex_buffer(0, line_buffer.slice(byte_offset..byte_offset + prepared.line_count as u64 * line_stride));
                pass.draw(0..prepared.line_count, 0..1);
            }
        }
        if !prepared.translucent_draws.is_empty() {
            pass.set_pipeline(&self.world_pipeline_translucent);
            pass.set_bind_group(0, &self.world_globals_ring.bind_group, &[self.world_globals_ring.offset_for_slot(slot)]);
            for draw_call in &prepared.translucent_draws {
                Self::draw_world_range(pass, mesh_store, draw_call, instance_buffer, instance_stride);
            }
        }
        pass.set_viewport(0.0, 0.0, screen_w, screen_h, 0.0, 1.0);
        set_pass_scissor(pass, clip, screen_w, screen_h);
        pass.set_pipeline(&self.ui_pipeline);
        pass.set_bind_group(0, &self.glyph_bind_group, &[]);
    }

    fn draw_world_range<'a>(pass: &mut wgpu::RenderPass<'a>, mesh_store: &MeshGpuTable, draw_call: &WorldDrawRange, instance_buffer: wgpu::BufferSlice<'a>, instance_stride: u64) {
        let store_key = MeshGpuTable::lookup_key(&draw_call.mesh_key, draw_call.mesh_version);
        let Some(mesh) = mesh_store.get(&store_key) else {
            return;
        };
        let byte_offset = draw_call.instance_offset as u64 * instance_stride;
        pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer.slice(byte_offset..byte_offset + draw_call.instance_count as u64 * instance_stride));
        pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..mesh.index_count, 0, 0..draw_call.instance_count);
    }

    fn draw_ui_instances<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, instance_buffer: &wgpu::BufferSlice<'a>, start: u32, count: u32) {
        if count == 0 {
            return;
        }
        pass.set_pipeline(&self.ui_pipeline);
        pass.set_bind_group(0, &self.glyph_bind_group, &[]);
        pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, *instance_buffer);
        pass.draw(0..6, start..start + count);
    }

    fn draw_silhouette_mask<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, mask_buffer: &wgpu::BufferSlice<'a>, start: u32, count: u32, width: f32, height: f32) {
        if count == 0 {
            pass.set_stencil_reference(1);
            return;
        }
        set_pass_scissor(pass, None, width, height);
        pass.set_pipeline(&self.mask_pipeline);
        pass.set_bind_group(0, &self.glyph_bind_group, &[]);
        pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, *mask_buffer);
        pass.set_stencil_reference(0);
        pass.draw(0..6, start..start + 1);
        if count > 1 {
            pass.set_stencil_reference(1);
            pass.draw(0..6, start + 1..start + count);
        }
        pass.set_stencil_reference(1);
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    fn draw_raster_layers<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        raster_store: &RasterTextureTable,
        draw: &DrawList,
        frame_buffers: &'a mut FrameBuffers,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        width: f32,
        height: f32,
        filter: LayerBatchFilter,
    ) {
        let raster_layers: Vec<&DrawLayer> = draw.layers.iter().filter(|layer| layer_matches_filter(layer, filter) && !layer.raster_instances.is_empty()).collect();
        let mut mask_data = Vec::new();
        let mut mask_ranges = Vec::with_capacity(raster_layers.len());
        let mut previous_bounds = None;
        for layer in &raster_layers {
            let start = mask_data.len() as u32;
            let (instances, current_bounds) = mask_instances(layer.scissor, layer.clip.as_ref(), previous_bounds, width, height);
            let count = instances.len() as u32;
            mask_data.extend(instances);
            mask_ranges.push((start, count));
            previous_bounds = current_bounds;
        }
        let Some(mask_buffer) = frame_buffers.mask_instances.upload(device, queue, &mask_data, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "raster_silhouette_masks") else {
            return;
        };
        for (layer_index, layer) in raster_layers.into_iter().enumerate() {
            let (mask_start, mask_count) = mask_ranges[layer_index];
            self.draw_silhouette_mask(pass, &mask_buffer, mask_start, mask_count, width, height);
            let mut batch_key: Option<String> = None;
            let mut batch_instances: Vec<UiInstance> = Vec::new();
            let mut flush = |key: &str, instances: &[UiInstance]| {
                if instances.is_empty() {
                    return;
                }
                let Some(rt) = raster_store.get(key) else {
                    return;
                };
                pass.set_pipeline(&self.ui_pipeline);
                pass.set_bind_group(0, &rt.bind_group, &[]);
                let Some(buffer) = frame_buffers.ui_instances.upload(device, queue, instances, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "raster_instances") else {
                    return;
                };
                pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
                pass.set_vertex_buffer(1, buffer);
                pass.draw(0..6, 0..instances.len() as u32);
            };
            for (key, instance) in &layer.raster_instances {
                if batch_key.as_deref() != Some(key.as_str()) {
                    if let Some(ref prior) = batch_key {
                        flush(prior, &batch_instances);
                    }
                    batch_key = Some(key.clone());
                    batch_instances.clear();
                }
                batch_instances.push(*instance);
            }
            if let Some(ref key) = batch_key {
                flush(key, &batch_instances);
            }
        }
        pass.set_scissor_rect(0, 0, width as u32, height as u32);
    }

    fn draw_vector_vertices<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, vector_buffer: &wgpu::BufferSlice<'a>, start: u32, count: u32) {
        if count == 0 {
            return;
        }
        pass.set_pipeline(&self.vector_pipeline);
        pass.set_bind_group(0, &self.glyph_bind_group, &[]);
        pass.set_vertex_buffer(0, *vector_buffer);
        pass.draw(start..start + count, 0..1);
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    fn render_interleaved_layers<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        draw: &DrawList,
        batches: &[LayerBatch],
        mask_buffer: Option<&wgpu::BufferSlice<'a>>,
        mask_ranges: &[(u32, u32)],
        ui_buffer: Option<&wgpu::BufferSlice<'a>>,
        vector_buffer: Option<&wgpu::BufferSlice<'a>>,
        world_prepared: Option<&[PreparedWorldPass]>,
        pass_index_map: &[Option<usize>],
        instance_buffer: Option<wgpu::BufferSlice<'a>>,
        line_buffer: Option<wgpu::BufferSlice<'a>>,
        mesh_store: &MeshGpuTable,
        width: f32,
        height: f32,
        depth_enabled: bool,
    ) {
        for (batch_index, batch) in batches.iter().enumerate() {
            if let Some((start, count)) = mask_ranges.get(batch_index).copied() {
                if let Some(mask_buffer) = mask_buffer {
                    self.draw_silhouette_mask(pass, mask_buffer, start, count, width, height);
                } else {
                    pass.set_stencil_reference(1);
                }
            }
            let mut layer_passes: Vec<(usize, usize, usize)> = draw.scene_passes.iter().enumerate().filter(|(_, scene)| scene.layer_index == batch.layer_index).map(|(index, scene)| (index, scene.ui_watermark, scene.vector_watermark)).collect();
            layer_passes.sort_by_key(|(_, ui, vec)| (*ui, *vec));
            if layer_passes.is_empty() {
                if let Some(instance_buffer) = ui_buffer {
                    self.draw_ui_instances(pass, instance_buffer, batch.ui_start, batch.ui_count);
                }
                if let Some(vector_buffer) = vector_buffer {
                    self.draw_vector_vertices(pass, vector_buffer, batch.vec_start, batch.vec_count);
                }
                continue;
            }
            let mut ui_local = 0u32;
            let mut vec_local = 0u32;
            for (pass_index, ui_mark, vec_mark) in layer_passes {
                let ui_mark = ui_mark as u32;
                let vec_mark = vec_mark as u32;
                if ui_mark > ui_local {
                    if let Some(instance_buffer) = ui_buffer {
                        self.draw_ui_instances(pass, instance_buffer, batch.ui_start + ui_local, ui_mark - ui_local);
                    }
                    ui_local = ui_mark;
                }
                if vec_mark > vec_local {
                    if let Some(vector_buffer) = vector_buffer {
                        self.draw_vector_vertices(pass, vector_buffer, batch.vec_start + vec_local, vec_mark - vec_local);
                    }
                    vec_local = vec_mark;
                }
                if depth_enabled {
                    if let (Some(prepared), Some(instance_buffer)) = (world_prepared, instance_buffer.as_ref()) {
                        if let Some(prepared_slot) = pass_index_map.get(pass_index).and_then(|slot| *slot) {
                            if let Some(scene) = prepared.get(prepared_slot) {
                                self.draw_world_pass_at(pass, mesh_store, scene, prepared_slot as u32, *instance_buffer, line_buffer, width, height, None);
                            }
                        }
                    }
                }
            }
            if ui_local < batch.ui_count {
                if let Some(instance_buffer) = ui_buffer {
                    self.draw_ui_instances(pass, instance_buffer, batch.ui_start + ui_local, batch.ui_count - ui_local);
                }
            }
            if vec_local < batch.vec_count {
                if let Some(vector_buffer) = vector_buffer {
                    self.draw_vector_vertices(pass, vector_buffer, batch.vec_start + vec_local, batch.vec_count - vec_local);
                }
            }
        }
        pass.set_scissor_rect(0, 0, width as u32, height as u32);
    }

    pub fn update_globals(&self, queue: &wgpu::Queue, width: f32, height: f32, time_seconds: f32) {
        queue.write_buffer(&self.globals_buffer, 0, bytemuck::bytes_of(&UiGlobals { screen_size: [width, height], _pad: [time_seconds, 0.0] }));
    }

    pub fn upload_glyph_atlas(&self, queue: &wgpu::Queue, pixels: &[u8], width: u32, height: u32) {
        queue.write_texture(
            wgpu::TexelCopyTextureInfo { texture: &self.glyph_texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            pixels,
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(width), rows_per_image: Some(height) },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );
    }

    pub fn upload_icon_atlas(&self, queue: &wgpu::Queue, pixels: &[u8], width: u32, height: u32) {
        queue.write_texture(
            wgpu::TexelCopyTextureInfo { texture: &self.icon_texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            pixels,
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(width * 4), rows_per_image: Some(height) },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    pub fn render_scene_content<'a>(
        &'a mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        scene: &'a SceneColorTarget,
        depth_view: Option<&'a wgpu::TextureView>,
        draw: &DrawList,
        mesh_store: &MeshGpuTable,
        raster_store: &RasterTextureTable,
        frame_buffers: &mut FrameBuffers,
        width: f32,
        height: f32,
        time_seconds: f32,
    ) {
        self.update_globals(queue, width, height, time_seconds);
        let scene_view = scene.mip_view(0);
        let world_upload = if depth_view.is_some() { self.upload_world_passes(device, queue, draw, frame_buffers, LayerBatchFilter::Backdrop) } else { None };
        let (prepared_holder, pass_index_map) = match world_upload {
            Some((prepared, map)) => (Some(prepared), map),
            None => (None, vec![None; draw.scene_passes.len()]),
        };
        let world_prepared = prepared_holder.as_deref();
        let (all_ui, all_vec, batches) = build_layer_batches(draw, LayerBatchFilter::Backdrop);
        let (mask_data, mask_ranges) = build_batch_masks(&batches, width, height);
        let mask_buffer = frame_buffers.mask_instances.upload(device, queue, &mask_data, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "silhouette_masks");
        let ui_buffer = if all_ui.is_empty() { None } else { frame_buffers.ui_instances.upload(device, queue, &all_ui, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "ui_instances") };
        let vector_buffer = if all_vec.is_empty() { None } else { frame_buffers.vector_vertices.upload(device, queue, &all_vec, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "vector_vertices") };
        let instance_buffer = frame_buffers.world_instances.slice();
        let line_buffer = frame_buffers.world_lines.slice();
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("ui_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: scene_view,
                resolve_target: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.05, g: 0.05, b: 0.06, a: 1.0 }), store: wgpu::StoreOp::Store },
                depth_slice: None,
            })],
            depth_stencil_attachment: depth_view.map(|depth| stencil_attachment(depth, wgpu::LoadOp::Clear(1.0), wgpu::LoadOp::Clear(0))),
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        self.render_interleaved_layers(
            &mut pass,
            draw,
            &batches,
            mask_buffer.as_ref(),
            &mask_ranges,
            ui_buffer.as_ref(),
            vector_buffer.as_ref(),
            world_prepared,
            &pass_index_map,
            instance_buffer,
            line_buffer,
            mesh_store,
            width,
            height,
            depth_view.is_some(),
        );
        drop(pass);
        if draw.layers.iter().any(|layer| layer_matches_filter(layer, LayerBatchFilter::Backdrop) && !layer.raster_instances.is_empty()) {
            let mut raster_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ui_raster_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: scene_view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }, depth_slice: None })],
                depth_stencil_attachment: depth_view.map(|depth| stencil_attachment(depth, wgpu::LoadOp::Load, wgpu::LoadOp::Clear(0))),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            self.draw_raster_layers(&mut raster_pass, raster_store, draw, frame_buffers, device, queue, width, height, LayerBatchFilter::Backdrop);
        }
        let (overlay_ui, overlay_vec, overlay_batches) = build_overlay_layer_batches(draw, LayerBatchFilter::Backdrop);
        if !overlay_ui.is_empty() || !overlay_vec.is_empty() {
            let (overlay_mask_data, overlay_mask_ranges) = build_batch_masks(&overlay_batches, width, height);
            let overlay_mask_buffer = frame_buffers.mask_instances.upload(device, queue, &overlay_mask_data, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "overlay_silhouette_masks");
            let overlay_ui_buffer = if overlay_ui.is_empty() { None } else { frame_buffers.ui_instances.upload(device, queue, &overlay_ui, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "overlay_ui_instances") };
            let overlay_vector_buffer = if overlay_vec.is_empty() { None } else { frame_buffers.vector_vertices.upload(device, queue, &overlay_vec, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "overlay_vector_vertices") };
            let mut overlay_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ui_overlay_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: scene_view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }, depth_slice: None })],
                depth_stencil_attachment: depth_view.map(|depth| stencil_attachment(depth, wgpu::LoadOp::Load, wgpu::LoadOp::Clear(0))),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            self.render_interleaved_layers(
                &mut overlay_pass,
                draw,
                &overlay_batches,
                overlay_mask_buffer.as_ref(),
                &overlay_mask_ranges,
                overlay_ui_buffer.as_ref(),
                overlay_vector_buffer.as_ref(),
                None,
                &[],
                None,
                None,
                mesh_store,
                width,
                height,
                depth_view.is_some(),
            );
        }
    }

    fn has_glass_foreground(draw: &DrawList) -> bool {
        let layer_content = draw.layers.iter().any(|layer| layer.foreground_of.is_some() && (!layer.ui_instances.is_empty() || !layer.vector_vertices.is_empty() || !layer.raster_instances.is_empty()));
        let scene_content = draw.scene_passes.iter().any(|pass| layer_matches_filter(&draw.layers[pass.layer_index], LayerBatchFilter::Foreground));
        layer_content || scene_content
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    fn render_glass_foreground<'a>(
        &'a mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
        draw: &DrawList,
        depth_view: Option<&'a wgpu::TextureView>,
        mesh_store: &MeshGpuTable,
        raster_store: &RasterTextureTable,
        frame_buffers: &mut FrameBuffers,
        width: f32,
        height: f32,
    ) {
        if !Self::has_glass_foreground(draw) {
            return;
        }
        let world_upload = if depth_view.is_some() { self.upload_world_passes(device, queue, draw, frame_buffers, LayerBatchFilter::Foreground) } else { None };
        let (prepared_holder, pass_index_map) = match world_upload {
            Some((prepared, map)) => (Some(prepared), map),
            None => (None, vec![None; draw.scene_passes.len()]),
        };
        let world_prepared = prepared_holder.as_deref();
        let (all_ui, all_vec, batches) = build_layer_batches(draw, LayerBatchFilter::Foreground);
        if all_ui.is_empty() && all_vec.is_empty() && batches.is_empty() && world_prepared.is_none() {
            return;
        }
        let (mask_data, mask_ranges) = build_batch_masks(&batches, width, height);
        let mask_buffer = frame_buffers.mask_instances.upload(device, queue, &mask_data, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "glass_foreground_silhouette_masks");
        let ui_buffer = if all_ui.is_empty() { None } else { frame_buffers.ui_instances.upload(device, queue, &all_ui, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "glass_foreground_ui_instances") };
        let vector_buffer = if all_vec.is_empty() { None } else { frame_buffers.vector_vertices.upload(device, queue, &all_vec, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "glass_foreground_vector_vertices") };
        let instance_buffer = frame_buffers.world_instances.slice();
        let line_buffer = frame_buffers.world_lines.slice();
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("glass_foreground_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment { view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }, depth_slice: None })],
            depth_stencil_attachment: depth_view.map(|depth| stencil_attachment(depth, wgpu::LoadOp::Load, wgpu::LoadOp::Clear(0))),
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        self.render_interleaved_layers(
            &mut pass,
            draw,
            &batches,
            mask_buffer.as_ref(),
            &mask_ranges,
            ui_buffer.as_ref(),
            vector_buffer.as_ref(),
            world_prepared,
            &pass_index_map,
            instance_buffer,
            line_buffer,
            mesh_store,
            width,
            height,
            depth_view.is_some(),
        );
        drop(pass);
        if draw.layers.iter().any(|layer| layer_matches_filter(layer, LayerBatchFilter::Foreground) && !layer.raster_instances.is_empty()) {
            let mut raster_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("glass_foreground_raster_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }, depth_slice: None })],
                depth_stencil_attachment: depth_view.map(|depth| stencil_attachment(depth, wgpu::LoadOp::Load, wgpu::LoadOp::Clear(0))),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            self.draw_raster_layers(&mut raster_pass, raster_store, draw, frame_buffers, device, queue, width, height, LayerBatchFilter::Foreground);
        }
        let (overlay_ui, overlay_vec, overlay_batches) = build_overlay_layer_batches(draw, LayerBatchFilter::Foreground);
        if !overlay_ui.is_empty() || !overlay_vec.is_empty() {
            let (overlay_mask_data, overlay_mask_ranges) = build_batch_masks(&overlay_batches, width, height);
            let overlay_mask_buffer = frame_buffers.mask_instances.upload(device, queue, &overlay_mask_data, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "glass_foreground_overlay_silhouette_masks");
            let overlay_ui_buffer = if overlay_ui.is_empty() { None } else { frame_buffers.ui_instances.upload(device, queue, &overlay_ui, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "glass_foreground_overlay_ui_instances") };
            let overlay_vector_buffer =
                if overlay_vec.is_empty() { None } else { frame_buffers.vector_vertices.upload(device, queue, &overlay_vec, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "glass_foreground_overlay_vector_vertices") };
            let mut overlay_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("glass_foreground_overlay_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }, depth_slice: None })],
                depth_stencil_attachment: depth_view.map(|depth| stencil_attachment(depth, wgpu::LoadOp::Load, wgpu::LoadOp::Clear(0))),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            self.render_interleaved_layers(
                &mut overlay_pass,
                draw,
                &overlay_batches,
                overlay_mask_buffer.as_ref(),
                &overlay_mask_ranges,
                overlay_ui_buffer.as_ref(),
                overlay_vector_buffer.as_ref(),
                None,
                &[],
                None,
                None,
                mesh_store,
                width,
                height,
                depth_view.is_some(),
            );
        }
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    pub fn composite_to_swapchain<'a>(
        &'a mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
        scene: &'a SceneColorTarget,
        depth_view: Option<&'a wgpu::TextureView>,
        draw: &DrawList,
        overlay: Option<&DrawList>,
        mesh_store: &MeshGpuTable,
        raster_store: &RasterTextureTable,
        frame_buffers: &mut FrameBuffers,
        width: f32,
        height: f32,
    ) {
        self.run_blur_chain(device, queue, scene);
        self.blit_scene_to_swapchain(device, encoder, view, scene);
        let max_mip = SCENE_MIP_LEVELS - 1;
        self.composite_glass_regions(device, queue, encoder, view, scene, frame_buffers, &draw.glass_regions, max_mip, width, height);
        self.render_glass_foreground(device, queue, encoder, view, draw, depth_view, mesh_store, raster_store, frame_buffers, width, height);
        if let Some(overlay) = overlay {
            if !overlay.glass_regions.is_empty() {
                self.composite_glass_regions(device, queue, encoder, view, scene, frame_buffers, &overlay.glass_regions, max_mip, width, height);
            }
            self.render_glass_foreground(device, queue, encoder, view, overlay, depth_view, mesh_store, raster_store, frame_buffers, width, height);
            let mut overlay_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ui_overlay_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }, depth_slice: None })],
                depth_stencil_attachment: depth_view.map(|depth| stencil_attachment(depth, wgpu::LoadOp::Load, wgpu::LoadOp::Clear(0))),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            self.render_overlay(device, queue, &mut overlay_pass, overlay, frame_buffers, width, height);
        }
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    #[allow(dead_code, reason = "top-level UiPipelines render entrypoint; not yet called internally, likely wired externally by framework/renderer/wgpu")]
    pub fn render<'a>(
        &'a mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
        scene: &'a SceneColorTarget,
        depth_view: Option<&'a wgpu::TextureView>,
        draw: &DrawList,
        overlay: Option<&DrawList>,
        mesh_store: &MeshGpuTable,
        raster_store: &RasterTextureTable,
        frame_buffers: &mut FrameBuffers,
        width: f32,
        height: f32,
        time_seconds: f32,
    ) {
        self.render_scene_content(device, queue, encoder, scene, depth_view, draw, mesh_store, raster_store, frame_buffers, width, height, time_seconds);
        self.composite_to_swapchain(device, queue, encoder, view, scene, depth_view, draw, overlay, mesh_store, raster_store, frame_buffers, width, height);
    }

    fn run_blur_chain(&self, device: &wgpu::Device, queue: &wgpu::Queue, scene: &SceneColorTarget) {
        for mip in 1..SCENE_MIP_LEVELS {
            let src_mip = mip - 1;
            queue.write_buffer(&self.blur_globals_buffer, 0, bytemuck::bytes_of(&BlurGlobals { src_mip: 0.0, _pad: [0.0; 7] }));
            let blur_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("blur_downsample_bind_group"),
                layout: &self.blur_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: self.blur_globals_buffer.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(scene.blur_scratch_mip_view(src_mip)) },
                    wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(scene.sampler()) },
                ],
            });
            let mut copy_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("blur_copy_encoder") });
            scene.copy_mip_to_blur_scratch(&mut copy_encoder, src_mip);
            queue.submit(Some(copy_encoder.finish()));
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("blur_downsample_encoder") });
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("blur_downsample_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: scene.mip_view(mip),
                    resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&self.blur_downsample_pipeline);
            pass.set_bind_group(0, &blur_bind_group, &[]);
            pass.draw(0..6, 0..1);
            drop(pass);
            queue.submit(Some(encoder.finish()));
        }
    }

    fn blit_scene_to_swapchain(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, scene: &SceneColorTarget) {
        let scene_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("scene_blit_bind_group"),
            layout: &self.scene_bind_group_layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(scene.sample_view()) }, wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(scene.sampler()) }],
        });
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("scene_blit_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.05, g: 0.05, b: 0.06, a: 1.0 }), store: wgpu::StoreOp::Store },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        pass.set_pipeline(&self.scene_blit_pipeline);
        pass.set_bind_group(0, &scene_bind_group, &[]);
        pass.draw(0..6, 0..1);
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    fn composite_glass_regions(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        scene: &SceneColorTarget,
        frame_buffers: &mut FrameBuffers,
        regions: &[GlassRegion],
        max_mip: u32,
        width: f32,
        height: f32,
    ) {
        if regions.is_empty() {
            return;
        }
        let instances: Vec<GlassInstance> = regions
            .iter()
            .map(|region| GlassInstance { rect: region.rect, tint: [region.tint.r, region.tint.g, region.tint.b, region.tint.a], params: [region.radius, region.alpha, Theme::glass_mip_level(region.blur_px, max_mip), region.saturate] })
            .collect();
        let glass_buffer = frame_buffers.glass_instances.upload(device, queue, &instances, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "glass_instances");
        let Some(glass_buffer) = glass_buffer else {
            return;
        };
        let scene_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("glass_scene_bind_group"),
            layout: &self.scene_bind_group_layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(scene.sample_view()) }, wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(scene.sampler()) }],
        });
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("glass_composite_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment { view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }, depth_slice: None })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        pass.set_pipeline(&self.glass_pipeline);
        pass.set_bind_group(0, &self.glyph_bind_group, &[]);
        pass.set_bind_group(1, &scene_bind_group, &[]);
        pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, glass_buffer.slice(..));
        pass.draw(0..6, 0..instances.len() as u32);
        let _ = (width, height);
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    pub fn render_overlay<'a>(&'a self, device: &wgpu::Device, queue: &wgpu::Queue, pass: &mut wgpu::RenderPass<'a>, overlay: &DrawList, frame_buffers: &'a mut FrameBuffers, width: f32, height: f32) {
        pass.set_pipeline(&self.ui_pipeline);
        pass.set_bind_group(0, &self.glyph_bind_group, &[]);

        let (all_ui, all_vec, batches) = build_layer_batches(overlay, LayerBatchFilter::Backdrop);
        let (mask_data, mask_ranges) = build_batch_masks(&batches, width, height);
        let mask_buffer = frame_buffers.mask_instances.upload(device, queue, &mask_data, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "top_overlay_silhouette_masks");
        let ui_buffer = if all_ui.is_empty() { None } else { frame_buffers.ui_instances.upload(device, queue, &all_ui, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "overlay_ui_instances") };
        let vector_buffer = if all_vec.is_empty() { None } else { frame_buffers.vector_vertices.upload(device, queue, &all_vec, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "overlay_vector_vertices") };

        for (batch_index, batch) in batches.iter().enumerate() {
            if let Some((start, count)) = mask_ranges.get(batch_index).copied() {
                if let Some(mask_buffer) = mask_buffer.as_ref() {
                    self.draw_silhouette_mask(pass, mask_buffer, start, count, width, height);
                } else {
                    pass.set_stencil_reference(1);
                }
            }
            if batch.ui_count > 0 {
                if let Some(instance_buffer) = &ui_buffer {
                    pass.set_pipeline(&self.ui_pipeline);
                    pass.set_bind_group(0, &self.glyph_bind_group, &[]);
                    pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
                    pass.set_vertex_buffer(1, *instance_buffer);
                    pass.draw(0..6, batch.ui_start..batch.ui_start + batch.ui_count);
                }
            }
            if batch.vec_count > 0 {
                if let Some(vector_buffer) = &vector_buffer {
                    pass.set_pipeline(&self.vector_pipeline);
                    pass.set_vertex_buffer(0, *vector_buffer);
                    pass.draw(batch.vec_start..batch.vec_start + batch.vec_count, 0..1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{content_stencil_state, ear_clip_polygon, mask_instances, mask_stencil_state, mesh_content_version, ClipRegion, DrawList, ScissorRect, WORLD_GLOBALS_SLOT_SIZE};
    use crate::wgpu::geometry::Rect;
    use crate::wgpu::kernel_3d_scene::ScenePass3d;
    use crate::wgpu::theme::Rgba;

    #[test]
    fn scissor_intersects_child() {
        let a = ScissorRect { x: 0, y: 0, w: 100, h: 100 };
        let b = ScissorRect { x: 50, y: 50, w: 100, h: 100 };
        let c = a.intersect(&b);
        assert_eq!(c.w, 50);
        assert_eq!(c.h, 50);
    }

    #[test]
    fn scissor_covers_fractional_rect_edges_without_pixel_seams() {
        assert_eq!(ScissorRect::from_rect(Rect::new(10.25, 20.75, 30.5, 40.5), 100.0), ScissorRect { x: 10, y: 20, w: 31, h: 42 });
    }

    //#region SilhouetteClipTests

    #[test]
    fn clip_region_preserves_disjoint_cutouts_and_intersects_scissor() {
        let clip = ClipRegion::from_rects(&[Rect::new(0.0, 0.0, 40.0, 20.0), Rect::new(80.0, 0.0, 20.0, 20.0), Rect::new(0.0, 20.0, 100.0, 80.0)], 100.0);
        let scissors = clip.effective_scissors(Some(ScissorRect { x: 10, y: 0, w: 80, h: 100 }), 100.0, 100.0);
        assert_eq!(scissors, vec![ScissorRect { x: 10, y: 0, w: 30, h: 20 }, ScissorRect { x: 80, y: 0, w: 10, h: 20 }, ScissorRect { x: 10, y: 20, w: 80, h: 80 }]);
    }

    #[test]
    fn draw_list_nests_and_restores_clip_regions() {
        let mut draw = DrawList::default();
        draw.begin_silhouette_clip(&[Rect::new(0.0, 0.0, 100.0, 100.0)]);
        draw.begin_silhouette_clip(&[Rect::new(25.0, 25.0, 100.0, 100.0)]);
        assert_eq!(draw.layers.last().and_then(|layer| layer.clip.as_ref()).map(|clip| clip.scissors.as_slice()), Some([ScissorRect { x: 25, y: 25, w: 75, h: 75 }].as_slice()));
        draw.end_silhouette_clip();
        assert_eq!(draw.layers.last().and_then(|layer| layer.clip.as_ref()).map(|clip| clip.scissors.as_slice()), Some([ScissorRect { x: 0, y: 0, w: 100, h: 100 }].as_slice()));
        draw.end_silhouette_clip();
        assert!(draw.layers.last().is_some_and(|layer| layer.clip.is_none()));
    }

    #[test]
    fn glass_foreground_inherits_active_silhouette_clip() {
        let mut draw = DrawList::default();
        draw.begin_silhouette_clip(&[Rect::new(0.0, 0.0, 40.0, 20.0), Rect::new(0.0, 20.0, 100.0, 80.0)]);
        let glass = draw.push_glass([0.0, 0.0, 40.0, 20.0], 0.0, crate::wgpu::theme::Theme::default().glass(crate::wgpu::theme::Level::Window));
        draw.begin_glass_content(glass);
        assert_eq!(draw.layers.last().and_then(|layer| layer.clip.as_ref()).map(|clip| clip.scissors.len()), Some(2));
    }

    #[test]
    fn silhouette_stencil_states_write_masks_then_require_equality() {
        let mask = mask_stencil_state();
        assert_eq!(mask.front.compare, wgpu::CompareFunction::Always);
        assert_eq!(mask.front.pass_op, wgpu::StencilOperation::Replace);
        assert_eq!(mask.write_mask, 0xff);
        let content = content_stencil_state();
        assert_eq!(content.front.compare, wgpu::CompareFunction::Equal);
        assert_eq!(content.front.pass_op, wgpu::StencilOperation::Keep);
        assert_eq!(content.write_mask, 0x00);
    }

    #[test]
    fn silhouette_mask_reset_is_bounded_to_previous_and_current_unions() {
        let previous = Some(ScissorRect { x: 10, y: 10, w: 30, h: 20 });
        let clip = ClipRegion { scissors: vec![ScissorRect { x: 80, y: 15, w: 20, h: 25 }] };
        let (instances, current) = mask_instances(None, Some(&clip), previous, 500.0, 400.0);
        assert_eq!(instances[0].rect, [10.0, 10.0, 90.0, 30.0]);
        assert_eq!(instances[1].rect, [80.0, 15.0, 20.0, 25.0]);
        assert_eq!(current, Some(ScissorRect { x: 80, y: 15, w: 20, h: 25 }));
    }

    #[test]
    fn empty_silhouette_clip_writes_no_visible_stencil_region() {
        let empty = ClipRegion { scissors: Vec::new() };
        let (instances, current) = mask_instances(None, Some(&empty), None, 500.0, 400.0);
        assert!(instances.is_empty(), "a cleared pass needs neither a reset nor a reference-one mask draw");
        assert_eq!(current, None);
    }

    //#endregion SilhouetteClipTests

    #[test]
    fn scissor_from_rect_uses_top_left_origin() {
        let scissor = ScissorRect::from_rect(Rect::new(10.0, 20.0, 80.0, 60.0), 720.0);
        assert_eq!(scissor.x, 10);
        assert_eq!(scissor.y, 20);
        assert_eq!(scissor.w, 80);
        assert_eq!(scissor.h, 60);
    }

    #[test]
    fn draw_list_push_scissor_splits_layers() {
        let mut draw = DrawList::default();
        draw.set_screen_height(200.0);
        draw.push_solid([0.0, 0.0, 200.0, 200.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
        draw.push_scissor(Rect::new(10.0, 10.0, 80.0, 80.0));
        draw.push_solid([10.0, 10.0, 80.0, 80.0], Rgba::new(0.0, 1.0, 0.0, 1.0));
        draw.pop_scissor();
        assert!(draw.layers.len() >= 3);
    }

    #[test]
    fn ear_clip_produces_triangles() {
        let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
        let tris = ear_clip_polygon(&square);
        assert!(tris.len() >= 3);
    }

    #[test]
    fn world_globals_slot_size_is_aligned() {
        const { assert!(WORLD_GLOBALS_SLOT_SIZE >= 80) };
        assert_eq!(WORLD_GLOBALS_SLOT_SIZE % 256, 0);
    }

    #[test]
    fn scene_pass_records_layer_watermarks() {
        let mut draw = DrawList::default();
        draw.push_solid([0.0, 0.0, 10.0, 10.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
        draw.push_solid([1.0, 1.0, 8.0, 8.0], Rgba::new(0.0, 1.0, 0.0, 1.0));
        draw.push_scene_pass(ScenePass3d { viewport: [0.0, 0.0, 100.0, 100.0], view_proj: [0.0; 16], light_dir: [0.0, 0.0, 1.0], ..Default::default() });
        draw.push_line(0.0, 0.0, 1.0, 1.0, Rgba::new(0.0, 0.0, 1.0, 1.0), 1.0);
        let pass = &draw.scene_passes[0];
        assert_eq!(pass.layer_index, 0);
        assert_eq!(pass.ui_watermark, 2);
        assert_eq!(pass.vector_watermark, 0);
        assert_eq!(draw.layers[0].ui_instances.len(), 2);
        assert_eq!(draw.layers[0].vector_vertices.len(), 6);
    }

    #[test]
    fn mesh_instances_without_lines_are_valid_world_pass() {
        use crate::wgpu::kernel_3d_scene::{Instance3d, SceneDraw3d, ScenePass3d};

        let pass = ScenePass3d {
            viewport: [0.0, 0.0, 320.0, 240.0],
            view_proj: [0.0; 16],
            light_dir: [0.4, 0.6, 0.8],
            draws: vec![SceneDraw3d {
                mesh_key: "box".into(),
                mesh_version: 1,
                instances: vec![Instance3d { id: "preview".into(), model: Instance3d::model_from_trs([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]), color: [0.7, 0.7, 0.75, 1.0], selected: false, hovered: false }],
            }],
            ..Default::default()
        };
        assert!(!pass.draws[0].instances.is_empty());
        assert!(pass.line_draws.is_empty());
    }

    #[test]
    fn mesh_content_version_changes_with_indices() {
        let v0 = mesh_content_version(&[0.0, 0.0, 0.0], &[0.0, 1.0, 0.0], &[0, 1, 2]);
        let v1 = mesh_content_version(&[0.0, 0.0, 0.0], &[0.0, 1.0, 0.0], &[0, 2, 1]);
        assert_ne!(v0, v1);
    }

    #[test]
    fn overlay_layers_collected_separately_from_backdrop_ui() {
        use super::{build_layer_batches, build_overlay_layer_batches, LayerBatchFilter};
        let mut draw = DrawList::default();
        draw.push_solid([0.0, 0.0, 100.0, 100.0], Rgba::new(0.1, 0.1, 0.1, 1.0));
        draw.push_glyph_overlay([10.0, 10.0, 20.0, 12.0], Rgba::new(1.0, 1.0, 1.0, 1.0), [0.0, 0.0, 0.1, 0.1]);
        draw.push_line_overlay(0.0, 0.0, 50.0, 50.0, Rgba::new(1.0, 0.0, 0.0, 1.0), 1.0);
        let (backdrop_ui, _, _) = build_layer_batches(&draw, LayerBatchFilter::Backdrop);
        let (overlay_ui, overlay_vec, overlay_batches) = build_overlay_layer_batches(&draw, LayerBatchFilter::Backdrop);
        assert_eq!(backdrop_ui.len(), 1);
        assert_eq!(overlay_ui.len(), 1);
        assert_eq!(overlay_vec.len(), 6);
        assert_eq!(overlay_batches.len(), 1);
        assert_eq!(draw.layers[overlay_batches[0].layer_index].overlay_ui_instances.len(), 1);
    }

    #[test]
    fn glass_content_layers_tagged_with_foreground_of() {
        use super::Theme;
        use crate::wgpu::theme::Level;
        let theme = Theme::default();
        let mut draw = DrawList::default();
        draw.push_solid([0.0, 0.0, 100.0, 100.0], Rgba::new(0.2, 0.2, 0.2, 1.0));
        let glass = draw.push_glass([10.0, 10.0, 80.0, 80.0], 8.0, theme.glass(Level::Panel));
        assert_eq!(glass, 0);
        draw.begin_glass_content(glass);
        draw.push_solid([10.0, 10.0, 80.0, 80.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
        draw.end_glass_content();
        let backdrop = draw.layers.iter().filter(|layer| layer.foreground_of.is_none()).count();
        let foreground = draw.layers.iter().filter(|layer| layer.foreground_of == Some(glass)).count();
        assert_eq!(backdrop, 2);
        assert_eq!(foreground, 1);
        assert_eq!(draw.layers[1].ui_instances.len(), 1);
    }

    #[test]
    fn glass_foreground_layers_excluded_from_backdrop_batches() {
        use super::{build_layer_batches, LayerBatchFilter, Theme};
        use crate::wgpu::theme::Level;
        let theme = Theme::default();
        let mut draw = DrawList::default();
        draw.push_solid([0.0, 0.0, 200.0, 200.0], Rgba::new(0.1, 0.1, 0.1, 1.0));
        let glass = draw.push_glass([20.0, 20.0, 160.0, 160.0], 8.0, theme.glass(Level::Panel));
        draw.begin_glass_content(glass);
        draw.push_solid([20.0, 20.0, 160.0, 160.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
        draw.end_glass_content();
        let (backdrop_ui, _, backdrop_batches) = build_layer_batches(&draw, LayerBatchFilter::Backdrop);
        let (foreground_ui, _, foreground_batches) = build_layer_batches(&draw, LayerBatchFilter::Foreground);
        assert_eq!(backdrop_ui.len(), 1);
        assert_eq!(foreground_ui.len(), 1);
        assert_eq!(backdrop_batches.len(), 1);
        assert_eq!(foreground_batches.len(), 1);
        assert!(draw.layers[backdrop_batches[0].layer_index].foreground_of.is_none());
        assert_eq!(draw.layers[foreground_batches[0].layer_index].foreground_of, Some(glass));
    }

    #[test]
    fn glass_scissor_inherits_foreground_tag() {
        use super::Theme;
        use crate::wgpu::theme::Level;
        let theme = Theme::default();
        let mut draw = DrawList::default();
        let glass = draw.push_glass([0.0, 0.0, 100.0, 100.0], 8.0, theme.glass(Level::Panel));
        draw.begin_glass_content(glass);
        draw.push_scissor(Rect::new(10.0, 10.0, 80.0, 80.0));
        draw.push_solid([10.0, 10.0, 80.0, 80.0], Rgba::new(0.0, 1.0, 0.0, 1.0));
        draw.pop_scissor();
        draw.end_glass_content();
        let scissor_layer = draw.layers.iter().find(|layer| layer.scissor.is_some()).expect("scissor layer");
        assert_eq!(scissor_layer.foreground_of, Some(glass));
    }

    /// 🪜️ `Theme::glass` must be formula-derived off `Level::index` (never a per-tier lookup
    /// table): alpha/blur both monotone across all 6 levels. There is deliberately no separate
    /// "chrome" variant — a level's attached chrome (title caps, ribbons, tab bars, rails) always
    /// renders the exact same `glass(level)` as its body, so one level never shows two appearances.
    #[test]
    fn glass_alpha_and_blur_are_formula_derived_per_level() {
        use super::Theme;
        use crate::wgpu::theme::Level;
        let theme = Theme::default();
        let ordered = [Level::Base, Level::Window, Level::Pane, Level::Panel, Level::Dialog, Level::Menu];
        for (k, level) in ordered.iter().enumerate() {
            assert_eq!(level.index(), k);
            let style = theme.glass(*level);
            assert!((style.alpha - (1.0 - k as f32 * ui_styling::levels::GLASS_ALPHA_STEP as f32)).abs() < 1e-6);
            assert!((style.blur_px - k as f32 * ui_styling::levels::GLASS_BLUR_STEP_PX as f32).abs() < 1e-6);
            assert_eq!(style.tint, theme.level_bg[k]);
        }
        assert!(theme.glass(Level::Base).alpha > theme.glass(Level::Menu).alpha);
        assert!(theme.glass(Level::Base).blur_px < theme.glass(Level::Menu).blur_px);
        assert_eq!(theme.surface(Level::Panel), theme.level_bg[Level::Panel.index()]);
    }
}
// #endregion draw
