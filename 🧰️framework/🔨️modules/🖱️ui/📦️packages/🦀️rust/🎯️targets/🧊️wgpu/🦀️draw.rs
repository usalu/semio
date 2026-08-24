// #region draw
//! 🖌️ Draw list and GPU pipeline for UI quads, vector geometry, and 3D scene passes.

use super::kernel_3d_scene::{Mat4Math, ScenePass3d};
use crate::wgpu::prepared::PreparedRasterPages;
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
    retained_output: Option<RetainedOutputGrant>,
}

#[derive(Clone, Copy)]
struct RetainedOutputGrant {
    item_limit: usize,
    byte_limit: usize,
    items: usize,
    bytes: usize,
    faulted: bool,
}

impl Default for DrawList {
    fn default() -> Self {
        let mut list = Self { scene_passes: Vec::new(), layers: Vec::new(), glass_regions: Vec::new(), scissor_stack: Vec::new(), clip_stack: Vec::new(), glass_content_stack: Vec::new(), screen_h: 720.0, retained_output: None };
        list.layers.push(DrawLayer::default());
        list
    }
}

impl DrawList {
    /// 🎟️ Pre-admits fixed candidate backing before a retained paint child transfers output.
    pub fn try_reserve_retained_items(&mut self, items: usize) -> Result<(), ()> {
        if self.layers.is_empty() {
            self.layers.try_reserve_exact(1).map_err(|_| ())?;
            self.layers.push(DrawLayer::default());
        }
        let vertices = items.checked_mul(6).ok_or(())?;
        let layer = self.layers.last_mut().ok_or(())?;
        layer.ui_instances.try_reserve_exact(items).map_err(|_| ())?;
        layer.overlay_ui_instances.try_reserve_exact(items).map_err(|_| ())?;
        layer.vector_vertices.try_reserve_exact(vertices).map_err(|_| ())?;
        layer.overlay_vector_vertices.try_reserve_exact(vertices).map_err(|_| ())?;
        layer.raster_instances.try_reserve_exact(items).map_err(|_| ())?;
        Ok(())
    }

    /// 🎫️ Starts one exact retained output grant and pre-admits all fixed container backing.
    pub fn begin_retained_output(&mut self, item_limit: usize, byte_limit: usize) -> Result<(), ()> {
        if self.retained_output.is_some() {
            return Err(());
        }
        self.try_reserve_retained_items(item_limit)?;
        self.layers.try_reserve_exact(item_limit).map_err(|_| ())?;
        self.glass_regions.try_reserve_exact(item_limit).map_err(|_| ())?;
        self.scissor_stack.try_reserve_exact(item_limit).map_err(|_| ())?;
        self.glass_content_stack.try_reserve_exact(item_limit).map_err(|_| ())?;
        self.retained_output = Some(RetainedOutputGrant { item_limit, byte_limit, items: 0, bytes: 0, faulted: false });
        Ok(())
    }

    /// 🧾️ Closes the exact retained output grant and reports any attempted overflow.
    pub fn finish_retained_output(&mut self) -> Result<(usize, usize), ()> {
        let Some(grant) = self.retained_output.take() else { return Err(()) };
        if grant.faulted {
            return Err(());
        }
        Ok((grant.items, grant.bytes))
    }

    fn claim_retained_output(&mut self, items: usize, bytes: usize) -> bool {
        let Some(grant) = self.retained_output.as_mut() else { return true };
        let Some(next_items) = grant.items.checked_add(items) else {
            grant.faulted = true;
            return false;
        };
        let Some(next_bytes) = grant.bytes.checked_add(bytes) else {
            grant.faulted = true;
            return false;
        };
        if next_items > grant.item_limit || next_bytes > grant.byte_limit {
            grant.faulted = true;
            return false;
        }
        grant.items = next_items;
        grant.bytes = next_bytes;
        true
    }

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
        self.scene_passes.is_empty() && self.layers.is_empty() && self.glass_regions.is_empty() && self.scissor_stack.is_empty() && self.clip_stack.is_empty() && self.glass_content_stack.is_empty() && self.retained_output.is_none()
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
        self.retained_output = None;
    }

    pub fn push_scissor(&mut self, rect: crate::wgpu::geometry::Rect) {
        if !self.claim_retained_output(1, std::mem::size_of::<DrawLayer>()) {
            return;
        }
        let mut scissor = ScissorRect::from_rect(rect, self.screen_h);
        if let Some(parent) = self.scissor_stack.last() {
            scissor = parent.intersect(&scissor);
        }
        self.scissor_stack.push(scissor);
        self.layers.push(DrawLayer { scissor: Some(scissor), clip: self.clip_stack.last().cloned(), foreground_of: self.active_foreground_of(), ..DrawLayer::default() });
    }

    pub fn pop_scissor(&mut self) {
        if !self.claim_retained_output(1, std::mem::size_of::<DrawLayer>()) {
            return;
        }
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
        if !self.claim_retained_output(1, std::mem::size_of::<UiInstance>()) {
            return;
        }
        self.active_layer().ui_instances.push(UiInstance::solid(rect, color));
    }

    pub fn push_rounded(&mut self, rect: [f32; 4], color: Rgba, radius: f32) {
        if !self.claim_retained_output(1, std::mem::size_of::<UiInstance>()) {
            return;
        }
        self.active_layer().ui_instances.push(UiInstance::rounded(rect, color, radius, 0.0, color));
    }

    /// 🌀️ Clockwise spinning + pulsing loading ring around `rect`, in `color` (gray `theme.border_normal` at rest, `theme.selected` when the node is selected/active).
    pub fn push_loading_border(&mut self, rect: [f32; 4], color: Rgba, radius: f32, stroke: f32) {
        if !self.claim_retained_output(1, std::mem::size_of::<UiInstance>()) {
            return;
        }
        self.active_layer().ui_instances.push(UiInstance::loading_border(rect, color, radius, stroke));
    }

    /// 🌀️ Dashed, slow-spinning + gently pulsing waiting ring around `rect`, in `color` (gray `theme.border_normal` at rest, `theme.selected` when the node is selected/active).
    pub fn push_waiting_border(&mut self, rect: [f32; 4], color: Rgba, radius: f32, stroke: f32) {
        if !self.claim_retained_output(1, std::mem::size_of::<UiInstance>()) {
            return;
        }
        self.active_layer().ui_instances.push(UiInstance::waiting_border(rect, color, radius, stroke));
    }

    /// ✅️ Solid, static at-bounds ring around `rect`, in `color` — `UiStatus::Finished`.
    pub fn push_finished_border(&mut self, rect: [f32; 4], color: Rgba, radius: f32, stroke: f32) {
        if !self.claim_retained_output(1, std::mem::size_of::<UiInstance>()) {
            return;
        }
        self.active_layer().ui_instances.push(UiInstance::finished_border(rect, color, radius, stroke));
    }

    /// 💫️ Raised-cosine breathing pulse ring around `rect`, in `color` — `UiState::Introducing`.
    pub fn push_introducing_border(&mut self, rect: [f32; 4], color: Rgba, radius: f32, stroke: f32) {
        if !self.claim_retained_output(1, std::mem::size_of::<UiInstance>()) {
            return;
        }
        self.active_layer().ui_instances.push(UiInstance::introducing_border(rect, color, radius, stroke));
    }

    /// 🧊️ Pushes a glass region rendered with an already-resolved `style` — callers derive `style`
    /// from `Theme::glass(level)` themselves (see
    /// `.🦑️repo/🎫️tickets/26/07/27/UNIFIED-6-LEVEL-UI-SURFACE-SYSTEM/contract.txt`) rather than this method
    /// picking a per-tier lookup.
    pub fn push_glass(&mut self, rect: [f32; 4], radius: f32, style: GlassStyle) -> usize {
        if !self.claim_retained_output(1, std::mem::size_of::<GlassRegion>()) {
            return usize::MAX;
        }
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
        if !self.claim_retained_output(1, std::mem::size_of::<UiInstance>()) {
            return;
        }
        self.active_layer().ui_instances.push(UiInstance::glyph(rect, color, uv_rect));
    }

    pub fn push_glyph_overlay(&mut self, rect: [f32; 4], color: Rgba, uv_rect: [f32; 4]) {
        if !self.claim_retained_output(1, std::mem::size_of::<UiInstance>()) {
            return;
        }
        self.active_layer().overlay_ui_instances.push(UiInstance::glyph(rect, color, uv_rect));
    }

    pub fn push_solid_overlay(&mut self, rect: [f32; 4], color: Rgba) {
        if !self.claim_retained_output(1, std::mem::size_of::<UiInstance>()) {
            return;
        }
        self.active_layer().overlay_ui_instances.push(UiInstance::solid(rect, color));
    }

    pub fn push_textured(&mut self, rect: [f32; 4], uv_rect: [f32; 4], color: Rgba) {
        if !self.claim_retained_output(1, std::mem::size_of::<UiInstance>()) {
            return;
        }
        self.active_layer().ui_instances.push(UiInstance::textured(rect, uv_rect, color));
    }

    pub fn push_raster_quad(&mut self, key: &str, rect: [f32; 4], uv_rect: [f32; 4], alpha: f32) {
        let Some(bytes) = key.len().checked_add(std::mem::size_of::<UiInstance>()) else {
            let _ = self.claim_retained_output(usize::MAX, usize::MAX);
            return;
        };
        if !self.claim_retained_output(1, bytes) {
            return;
        }
        self.active_layer().raster_instances.push((key.to_string(), UiInstance::raster(rect, uv_rect, alpha)));
    }

    pub fn push_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgba, width: f32) {
        if !self.claim_retained_output(1, 6 * std::mem::size_of::<VectorVertex>()) {
            return;
        }
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
        if !self.claim_retained_output(1, 6 * std::mem::size_of::<VectorVertex>()) {
            return;
        }
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
        let Some(vertices) = points.len().checked_sub(2).and_then(|triangles| triangles.checked_mul(3)) else {
            let _ = self.claim_retained_output(usize::MAX, usize::MAX);
            return;
        };
        if !self.claim_retained_output(1, vertices.saturating_mul(std::mem::size_of::<VectorVertex>())) {
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
        let Some(vertices) = points.len().checked_sub(2).and_then(|triangles| triangles.checked_mul(3)) else {
            let _ = self.claim_retained_output(usize::MAX, usize::MAX);
            return;
        };
        if !self.claim_retained_output(1, vertices.saturating_mul(std::mem::size_of::<VectorVertex>())) {
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

pub const MESH_GPU_TABLE_CAPACITY: usize = 256;
const MESH_GPU_KEY_BYTES: usize = 256;
pub const MESH_GPU_KEEP_VERSION_CAPACITY: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MeshGpuKey {
    bytes: [u8; MESH_GPU_KEY_BYTES],
    len: u16,
}

impl MeshGpuKey {
    fn new(key: &str) -> Result<Self, &'static str> {
        if key.is_empty() || key.len() > MESH_GPU_KEY_BYTES {
            return Err("mesh GPU key exceeded fixed credits");
        }
        let mut bytes = [0; MESH_GPU_KEY_BYTES];
        bytes[..key.len()].copy_from_slice(key.as_bytes());
        Ok(Self { bytes, len: key.len() as u16 })
    }

    fn matches(self, key: &str) -> bool {
        usize::from(self.len) == key.len() && &self.bytes[..usize::from(self.len)] == key.as_bytes()
    }
}

struct MeshGpuEntry<T> {
    key: MeshGpuKey,
    version: u64,
    value: T,
}

struct FixedMeshGpuRegistry<T> {
    slots: [Option<MeshGpuEntry<T>>; MESH_GPU_TABLE_CAPACITY],
    len: usize,
}

impl<T> Default for FixedMeshGpuRegistry<T> {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None), len: 0 }
    }
}

impl<T> FixedMeshGpuRegistry<T> {
    fn get(&self, key: &str, version: u64) -> Option<&T> {
        self.slots.iter().flatten().find(|entry| entry.version == version && entry.key.matches(key)).map(|entry| &entry.value)
    }

    fn insert(&mut self, entry: MeshGpuEntry<T>) -> Result<(), MeshGpuEntry<T>> {
        let Some(slot) = self.slots.iter_mut().find(|slot| slot.is_none()) else { return Err(entry) };
        *slot = Some(entry);
        self.len += 1;
        Ok(())
    }

    fn take(&mut self, index: usize) -> Option<MeshGpuEntry<T>> {
        let value = self.slots.get_mut(index)?.take();
        if value.is_some() {
            self.len = self.len.saturating_sub(1);
        }
        value
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == MESH_GPU_TABLE_CAPACITY
    }
}

pub struct MeshGpuTable {
    meshes: FixedMeshGpuRegistry<GpuMeshBuffers>,
    upload: Option<MeshGpuUploadCursor>,
    retirement: Option<MeshGpuRetirementCursor>,
    closing: bool,
}

struct MeshGpuUploadCursor {
    key: MeshGpuKey,
    version: u64,
    lease: crate::wgpu::kernel_3d_scene::Mesh3dLease,
    schema: crate::wgpu::kernel_3d_scene::Mesh3dSchema,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    vertex: u32,
    index: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MeshGpuRetirementSelector {
    Exact { key: MeshGpuKey, version: u64 },
    KeyExcept { key: MeshGpuKey, versions: [u64; MESH_GPU_KEEP_VERSION_CAPACITY], len: u16 },
    All,
}

impl MeshGpuRetirementSelector {
    fn selects<T>(self, entry: &MeshGpuEntry<T>) -> bool {
        match self {
            Self::Exact { key, version } => entry.key == key && entry.version == version,
            Self::KeyExcept { key, versions, len } => entry.key == key && !versions[..usize::from(len)].contains(&entry.version),
            Self::All => true,
        }
    }
}

struct MeshGpuRetirementCursor {
    selector: MeshGpuRetirementSelector,
    scan: usize,
    owner: Option<MeshGpuRetirementOwner>,
}

struct MeshGpuRetirementOwner {
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
}

impl Default for MeshGpuTable {
    fn default() -> Self {
        Self { meshes: FixedMeshGpuRegistry::default(), upload: None, retirement: None, closing: false }
    }
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
        let (key, version) = key.rsplit_once(':')?;
        self.get_versioned(key, version.parse().ok()?)
    }

    pub fn get_versioned(&self, mesh_key: &str, version: u64) -> Option<&GpuMeshBuffers> {
        self.meshes.get(mesh_key, version)
    }

    pub fn ensure_mesh_step(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, key: &str, version: u64, lease: crate::wgpu::kernel_3d_scene::Mesh3dLease) -> Result<bool, &'static str> {
        if self.closing {
            return Err("mesh GPU table is closing");
        }
        if self.meshes.get(key, version).is_some() {
            return Ok(true);
        }
        if self.upload.is_none() {
            if self.meshes.is_full() {
                return Err("mesh GPU table capacity exhausted");
            }
            let key = MeshGpuKey::new(key)?;
            let schema = lease.schema().map_err(|_| "mesh upload lease was stale")?;
            let vertex_bytes = u64::from(schema.vertices).checked_mul(std::mem::size_of::<World3dVertex>() as u64).ok_or("mesh upload vertex byte credits overflowed")?;
            let index_bytes = u64::from(schema.indices).checked_mul(std::mem::size_of::<u32>() as u64).ok_or("mesh upload index byte credits overflowed")?;
            if vertex_bytes == 0 || index_bytes == 0 {
                return Err("mesh upload schema was empty");
            }
            let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor { label: Some("world3d_vertices"), size: vertex_bytes, usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
            let index_buffer = device.create_buffer(&wgpu::BufferDescriptor { label: Some("world3d_indices"), size: index_bytes, usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
            self.upload = Some(MeshGpuUploadCursor { key, version, lease, schema, vertex_buffer: Some(vertex_buffer), index_buffer: Some(index_buffer), vertex: 0, index: 0 });
        }
        let cursor = self.upload.as_mut().expect("mesh upload cursor initialized above");
        if !cursor.key.matches(key) || cursor.version != version || cursor.lease != lease {
            return Err("mesh upload authority is occupied by another generation");
        }
        if cursor.vertex < cursor.schema.vertices {
            let position = cursor.lease.vec3(crate::wgpu::kernel_3d_scene::Mesh3dField::Positions, cursor.vertex).map_err(|_| "mesh upload position lease was stale")?;
            let normal = cursor.lease.vec3(crate::wgpu::kernel_3d_scene::Mesh3dField::Normals, cursor.vertex).unwrap_or([0.0, 1.0, 0.0]);
            let vertex = World3dVertex { position, normal };
            queue.write_buffer(cursor.vertex_buffer.as_ref().ok_or("mesh upload vertex buffer was retired")?, u64::from(cursor.vertex) * std::mem::size_of::<World3dVertex>() as u64, bytemuck::bytes_of(&vertex));
            cursor.vertex += 1;
            return Ok(false);
        }
        if cursor.index < cursor.schema.indices {
            let value = cursor.lease.u32(crate::wgpu::kernel_3d_scene::Mesh3dField::Indices, cursor.index).map_err(|_| "mesh upload index lease was stale")?;
            queue.write_buffer(cursor.index_buffer.as_ref().ok_or("mesh upload index buffer was retired")?, u64::from(cursor.index) * std::mem::size_of::<u32>() as u64, &value.to_le_bytes());
            cursor.index += 1;
            return Ok(false);
        }
        let mut cursor = self.upload.take().expect("completed mesh upload cursor");
        let entry = MeshGpuEntry {
            key: cursor.key,
            version: cursor.version,
            value: GpuMeshBuffers { vertex_buffer: cursor.vertex_buffer.take().expect("completed mesh vertex buffer"), index_buffer: cursor.index_buffer.take().expect("completed mesh index buffer"), index_count: cursor.schema.indices },
        };
        match self.meshes.insert(entry) {
            Ok(()) => Ok(true),
            Err(entry) => {
                cursor.vertex_buffer = Some(entry.value.vertex_buffer);
                cursor.index_buffer = Some(entry.value.index_buffer);
                self.upload = Some(cursor);
                Err("mesh GPU table capacity exhausted")
            }
        }
    }

    pub fn close_upload_step(&mut self) -> bool {
        let Some(cursor) = self.upload.as_mut() else { return true };
        if let Some(buffer) = cursor.vertex_buffer.take() {
            buffer.destroy();
            return false;
        }
        if let Some(buffer) = cursor.index_buffer.take() {
            buffer.destroy();
            return false;
        }
        self.upload = None;
        false
    }

    pub fn upload_terminal_is_empty(&self) -> bool {
        self.upload.is_none()
    }

    fn begin_retirement(&mut self, selector: MeshGpuRetirementSelector) -> Result<(), &'static str> {
        if let Some(retirement) = self.retirement.as_ref() {
            return (retirement.selector == selector).then_some(()).ok_or("mesh GPU retirement authority is occupied");
        }
        self.retirement = Some(MeshGpuRetirementCursor { selector, scan: 0, owner: None });
        Ok(())
    }

    fn retirement_step(&mut self) -> bool {
        let Some(cursor) = self.retirement.as_mut() else { return true };
        if let Some(owner) = cursor.owner.as_mut() {
            if let Some(buffer) = owner.vertex_buffer.take() {
                buffer.destroy();
                return false;
            }
            if let Some(buffer) = owner.index_buffer.take() {
                buffer.destroy();
                return false;
            }
            cursor.owner = None;
            if matches!(cursor.selector, MeshGpuRetirementSelector::Exact { .. }) {
                cursor.scan = MESH_GPU_TABLE_CAPACITY;
            }
            return false;
        }
        if cursor.scan >= MESH_GPU_TABLE_CAPACITY {
            self.retirement = None;
            return true;
        }
        let index = cursor.scan;
        cursor.scan += 1;
        let selected = self.meshes.slots[index].as_ref().is_some_and(|entry| cursor.selector.selects(entry));
        if selected {
            let entry = self.meshes.take(index).expect("selected mesh GPU entry");
            cursor.owner = Some(MeshGpuRetirementOwner { vertex_buffer: Some(entry.value.vertex_buffer), index_buffer: Some(entry.value.index_buffer) });
        }
        false
    }

    pub fn retire_exact_step(&mut self, key: &str, version: u64) -> Result<bool, &'static str> {
        self.begin_retirement(MeshGpuRetirementSelector::Exact { key: MeshGpuKey::new(key)?, version })?;
        Ok(self.retirement_step())
    }

    pub fn evict_mesh_step(&mut self, key: &str) -> Result<bool, &'static str> {
        self.evict_mesh_except_step(key, &[])
    }

    pub fn evict_mesh_except_step(&mut self, key: &str, keep_versions: &[u64]) -> Result<bool, &'static str> {
        if keep_versions.len() > MESH_GPU_KEEP_VERSION_CAPACITY {
            return Err("mesh GPU eviction keep-version credits exhausted");
        }
        let mut versions = [0; MESH_GPU_KEEP_VERSION_CAPACITY];
        versions[..keep_versions.len()].copy_from_slice(keep_versions);
        self.begin_retirement(MeshGpuRetirementSelector::KeyExcept { key: MeshGpuKey::new(key)?, versions, len: keep_versions.len() as u16 })?;
        Ok(self.retirement_step())
    }

    pub fn close_step(&mut self) -> bool {
        self.closing = true;
        if !self.close_upload_step() {
            return false;
        }
        if self.retirement.is_none() {
            let _ = self.begin_retirement(MeshGpuRetirementSelector::All);
        }
        if !self.retirement_step() {
            return false;
        }
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.upload.is_none() && self.retirement.is_none() && self.meshes.is_empty()
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
    pub view: wgpu::TextureView,
    pub bind_group: wgpu::BindGroup,
    pub width: u32,
    pub height: u32,
}

pub const RASTER_TEXTURE_TABLE_CAPACITY: usize = 256;
pub const RASTER_TEXTURE_KEY_BYTES: usize = 256;
pub const RASTER_TEXTURE_ITEM_BYTE_CAPACITY: usize = 16 * 1024 * 1024;
pub const RASTER_TEXTURE_TABLE_BYTE_CAPACITY: usize = 256 * 1024 * 1024;
const RASTER_TEXTURE_PROBE_CAPACITY: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RasterTextureWitness {
    pub scene_revision: u64,
    pub preview_generation: u64,
    pub operation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RasterTextureCleanupStep {
    Pending { released_roots: u8, released_scalars: u8 },
    Blocked(&'static str),
    Complete,
}

impl RasterTextureCleanupStep {
    fn root() -> Self {
        Self::Pending { released_roots: 1, released_scalars: 0 }
    }

    fn scalar() -> Self {
        Self::Pending { released_roots: 0, released_scalars: 1 }
    }

    fn retained() -> Self {
        Self::Pending { released_roots: 0, released_scalars: 0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RasterTextureKey {
    bytes: [u8; RASTER_TEXTURE_KEY_BYTES],
    len: u16,
    hash: u64,
}

impl RasterTextureKey {
    fn new(key: &str) -> Result<Self, &'static str> {
        if key.is_empty() || key.len() > RASTER_TEXTURE_KEY_BYTES {
            return Err("raster texture key exceeded fixed credits");
        }
        let mut bytes = [0; RASTER_TEXTURE_KEY_BYTES];
        bytes[..key.len()].copy_from_slice(key.as_bytes());
        let mut hash = 0xcbf29ce484222325u64;
        for byte in key.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        Ok(Self { bytes, len: key.len() as u16, hash })
    }

    fn matches(self, key: &str) -> bool {
        usize::from(self.len) == key.len() && &self.bytes[..usize::from(self.len)] == key.as_bytes()
    }

    fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).expect("raster key was admitted from UTF-8")
    }

    fn start(self) -> usize {
        self.hash as usize % RASTER_TEXTURE_TABLE_CAPACITY
    }
}

fn raster_texture_bytes(width: u32, height: u32) -> Option<usize> {
    usize::try_from(width).ok()?.checked_mul(usize::try_from(height).ok()?)?.checked_mul(4)
}

fn raster_witness_is_stale(current: RasterTextureWitness, candidate: RasterTextureWitness) -> bool {
    current.scene_revision > candidate.scene_revision
        || (current.scene_revision == candidate.scene_revision && current.preview_generation > candidate.preview_generation)
        || (current.scene_revision == candidate.scene_revision && current.preview_generation == candidate.preview_generation && current.operation >= candidate.operation)
}

struct RasterTextureEntry<T> {
    key: RasterTextureKey,
    witness: RasterTextureWitness,
    bytes: usize,
    value: T,
}

struct FixedRasterTextureRegistry<T> {
    slots: Box<[Option<RasterTextureEntry<T>>; RASTER_TEXTURE_TABLE_CAPACITY]>,
    len: usize,
    bytes: usize,
}

impl<T> Default for FixedRasterTextureRegistry<T> {
    fn default() -> Self {
        Self { slots: Box::new(std::array::from_fn(|_| None)), len: 0, bytes: 0 }
    }
}

impl<T> FixedRasterTextureRegistry<T> {
    fn locate(&self, key: RasterTextureKey) -> Result<Result<usize, usize>, &'static str> {
        let start = key.start();
        let mut vacant = None;
        for offset in 0..RASTER_TEXTURE_PROBE_CAPACITY {
            let index = (start + offset) % RASTER_TEXTURE_TABLE_CAPACITY;
            match self.slots[index].as_ref() {
                Some(entry) if entry.key == key => return Ok(Ok(index)),
                None if vacant.is_none() => vacant = Some(index),
                _ => {}
            }
        }
        vacant.map(Err).ok_or("raster texture probe credits exhausted")
    }

    fn get(&self, key: &str) -> Option<&RasterTextureEntry<T>> {
        let key = RasterTextureKey::new(key).ok()?;
        let Ok(index) = self.locate(key).ok()? else { return None };
        self.slots[index].as_ref()
    }

    fn insert(&mut self, entry: RasterTextureEntry<T>) -> Result<Option<RasterTextureEntry<T>>, RasterTextureEntry<T>> {
        let index = match self.locate(entry.key) {
            Ok(Ok(index)) | Ok(Err(index)) => index,
            Err(_) => return Err(entry),
        };
        let previous = self.slots[index].replace(entry);
        match previous.as_ref() {
            Some(previous) => {
                self.bytes = self.bytes.saturating_sub(previous.bytes);
            }
            None => self.len += 1,
        }
        self.bytes = self.bytes.saturating_add(self.slots[index].as_ref().expect("inserted raster entry").bytes);
        Ok(previous)
    }

    fn insert_vacant(&mut self, index: usize, entry: RasterTextureEntry<T>) -> Result<(), RasterTextureEntry<T>> {
        let Some(slot) = self.slots.get_mut(index) else { return Err(entry) };
        if slot.is_some() {
            return Err(entry);
        }
        *slot = Some(entry);
        self.len += 1;
        self.bytes = self.bytes.saturating_add(slot.as_ref().expect("inserted raster entry").bytes);
        Ok(())
    }

    fn take(&mut self, index: usize) -> Option<RasterTextureEntry<T>> {
        let value = self.slots.get_mut(index)?.take();
        if let Some(value) = value.as_ref() {
            self.len = self.len.saturating_sub(1);
            self.bytes = self.bytes.saturating_sub(value.bytes);
        }
        value
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

pub struct RasterTextureAdmission {
    key: RasterTextureKey,
    witness: RasterTextureWitness,
    width: u32,
    height: u32,
    bytes: usize,
    staged_index: usize,
    nonce: u64,
}

pub enum RasterTextureStageFault {
    Returned { fault: &'static str, admission: RasterTextureAdmission, texture: wgpu::Texture, view: wgpu::TextureView },
    Retained(&'static str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RasterTextureReservation {
    key: RasterTextureKey,
    witness: RasterTextureWitness,
    width: u32,
    height: u32,
    bytes: usize,
    staged_index: usize,
    nonce: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RasterTextureStageClaim {
    reservation: RasterTextureReservation,
    candidate: RasterTextureWitness,
    staged_index: usize,
    staged_nonce: u64,
}

impl RasterTextureReservation {
    fn matches(&self, admission: &RasterTextureAdmission) -> bool {
        self.key == admission.key
            && self.witness == admission.witness
            && self.width == admission.width
            && self.height == admission.height
            && self.bytes == admission.bytes
            && self.staged_index == admission.staged_index
            && self.nonce == admission.nonce
    }
}

fn claim_raster_stage_tuple(
    reservation: Option<RasterTextureReservation>,
    candidate: Option<RasterTextureWitness>,
    staged_occupied: bool,
    admission: &RasterTextureAdmission,
    expected: RasterTextureWitness,
) -> Result<RasterTextureStageClaim, &'static str> {
    if admission.witness != expected {
        return Err("raster operation authority was stale before GPU allocation");
    }
    let Some(reservation) = reservation else {
        return Err("raster texture reservation was missing before GPU allocation");
    };
    if !reservation.matches(admission) {
        return Err("raster texture reservation was stale before GPU allocation");
    }
    if candidate != Some(expected) {
        return Err("raster candidate witness changed before GPU allocation");
    }
    if staged_occupied {
        return Err("raster staged slot changed before GPU allocation");
    }
    Ok(RasterTextureStageClaim { reservation, candidate: expected, staged_index: admission.staged_index, staged_nonce: admission.nonce })
}

struct RasterTextureReservationRetirement {
    key: Option<RasterTextureKey>,
    scene_revision: Option<u64>,
    preview_generation: Option<u64>,
    operation: Option<u64>,
    width: Option<u32>,
    height: Option<u32>,
    bytes: Option<usize>,
    staged_index: Option<usize>,
    nonce: Option<u64>,
}

impl RasterTextureReservationRetirement {
    fn new(reservation: RasterTextureReservation) -> Self {
        Self {
            key: Some(reservation.key),
            scene_revision: Some(reservation.witness.scene_revision),
            preview_generation: Some(reservation.witness.preview_generation),
            operation: Some(reservation.witness.operation),
            width: Some(reservation.width),
            height: Some(reservation.height),
            bytes: Some(reservation.bytes),
            staged_index: Some(reservation.staged_index),
            nonce: Some(reservation.nonce),
        }
    }

    fn step(&mut self) -> RasterTextureCleanupStep {
        if self.key.take().is_some() {
            return RasterTextureCleanupStep::root();
        }
        if self.scene_revision.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.preview_generation.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.operation.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.width.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.height.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.bytes.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.staged_index.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.nonce.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        RasterTextureCleanupStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.key.is_none()
            && self.scene_revision.is_none()
            && self.preview_generation.is_none()
            && self.operation.is_none()
            && self.width.is_none()
            && self.height.is_none()
            && self.bytes.is_none()
            && self.staged_index.is_none()
            && self.nonce.is_none()
    }
}

struct RasterTextureReservationCloseCursor {
    reservation_retirement: Option<RasterTextureReservationRetirement>,
    admission_retirement: Option<RasterTextureReservationRetirement>,
}

impl RasterTextureReservationCloseCursor {
    fn reservation(reservation: RasterTextureReservation) -> Self {
        Self { reservation_retirement: Some(RasterTextureReservationRetirement::new(reservation)), admission_retirement: None }
    }

    fn cancelled(reservation: RasterTextureReservation, admission: RasterTextureAdmission) -> Self {
        Self {
            reservation_retirement: Some(RasterTextureReservationRetirement::new(reservation)),
            admission_retirement: Some(RasterTextureReservationRetirement::new(RasterTextureReservation {
                key: admission.key,
                witness: admission.witness,
                width: admission.width,
                height: admission.height,
                bytes: admission.bytes,
                staged_index: admission.staged_index,
                nonce: admission.nonce,
            })),
        }
    }

    fn rejected(admission: RasterTextureAdmission) -> Self {
        Self {
            reservation_retirement: None,
            admission_retirement: Some(RasterTextureReservationRetirement::new(RasterTextureReservation {
                key: admission.key,
                witness: admission.witness,
                width: admission.width,
                height: admission.height,
                bytes: admission.bytes,
                staged_index: admission.staged_index,
                nonce: admission.nonce,
            })),
        }
    }

    fn step(&mut self) -> RasterTextureCleanupStep {
        if let Some(retirement) = self.reservation_retirement.as_mut() {
            let step = retirement.step();
            if matches!(step, RasterTextureCleanupStep::Complete) {
                assert!(retirement.terminal_is_empty(), "completed raster reservation must be terminal-empty");
                self.reservation_retirement = None;
                return RasterTextureCleanupStep::retained();
            }
            return step;
        }
        if let Some(retirement) = self.admission_retirement.as_mut() {
            let step = retirement.step();
            if matches!(step, RasterTextureCleanupStep::Complete) {
                assert!(retirement.terminal_is_empty(), "completed raster admission must be terminal-empty");
                self.admission_retirement = None;
                return RasterTextureCleanupStep::retained();
            }
            return step;
        }
        RasterTextureCleanupStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.reservation_retirement.is_none() && self.admission_retirement.is_none()
    }

    fn retained_bytes(&self) -> usize {
        self.reservation_retirement.as_ref().and_then(|retirement| retirement.bytes).unwrap_or(0) + self.admission_retirement.as_ref().and_then(|retirement| retirement.bytes).unwrap_or(0)
    }

    fn retained_items(&self) -> usize {
        usize::from(self.reservation_retirement.is_some()) + usize::from(self.admission_retirement.is_some())
    }
}

struct RasterTextureStageClaimRetirement {
    reservation: RasterTextureReservationRetirement,
    candidate: RasterTextureWitnessSlot,
    staged_index: Option<usize>,
    staged_nonce: Option<u64>,
}

impl RasterTextureStageClaimRetirement {
    fn new(claim: RasterTextureStageClaim) -> Self {
        let mut candidate = RasterTextureWitnessSlot::default();
        candidate.set(claim.candidate);
        Self { reservation: RasterTextureReservationRetirement::new(claim.reservation), candidate, staged_index: Some(claim.staged_index), staged_nonce: Some(claim.staged_nonce) }
    }

    fn step(&mut self) -> RasterTextureCleanupStep {
        let reservation = self.reservation.step();
        if !matches!(reservation, RasterTextureCleanupStep::Complete) {
            return reservation;
        }
        if !self.candidate.is_empty() {
            self.candidate.retire_one();
            return RasterTextureCleanupStep::scalar();
        }
        if self.staged_index.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.staged_nonce.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        RasterTextureCleanupStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.reservation.terminal_is_empty() && self.candidate.is_empty() && self.staged_index.is_none() && self.staged_nonce.is_none()
    }
}

#[derive(Default)]
struct RasterTextureWitnessSlot {
    scene_revision: Option<u64>,
    preview_generation: Option<u64>,
    operation: Option<u64>,
}

impl RasterTextureWitnessSlot {
    fn get(&self) -> Option<RasterTextureWitness> {
        Some(RasterTextureWitness { scene_revision: self.scene_revision?, preview_generation: self.preview_generation?, operation: self.operation? })
    }

    fn set(&mut self, witness: RasterTextureWitness) {
        self.scene_revision = Some(witness.scene_revision);
        self.preview_generation = Some(witness.preview_generation);
        self.operation = Some(witness.operation);
    }

    fn retire_one(&mut self) -> bool {
        if self.scene_revision.take().is_some() {
            return false;
        }
        if self.preview_generation.take().is_some() {
            return false;
        }
        if self.operation.take().is_some() {
            return false;
        }
        true
    }

    fn is_empty(&self) -> bool {
        self.scene_revision.is_none() && self.preview_generation.is_none() && self.operation.is_none()
    }
}

struct RasterTextureUploadCursor {
    admission: Option<RasterTextureAdmission>,
    row: u32,
    texture: Option<wgpu::Texture>,
    view: Option<wgpu::TextureView>,
    bind_group: Option<wgpu::BindGroup>,
    allocation_claim: Option<RasterTextureStageClaim>,
}

pub(crate) enum RasterUploadPixels<'a> {
    Contiguous(&'a [u8]),
    Pages(&'a PreparedRasterPages),
}

impl RasterUploadPixels<'_> {
    fn len(&self) -> usize {
        match self {
            Self::Contiguous(pixels) => pixels.len(),
            Self::Pages(pixels) => pixels.byte_len(),
        }
    }

    fn dimensions_match(&self, width: u32, height: u32) -> bool {
        match self {
            Self::Contiguous(_) => true,
            Self::Pages(pixels) => pixels.width() == width && pixels.height() == height,
        }
    }

    fn rows(&self, row: u32, start: usize, end: usize, rows: u32) -> Option<(&[u8], u32)> {
        match self {
            Self::Contiguous(pixels) => pixels.get(start..end).map(|page| (page, rows)),
            Self::Pages(pixels) => pixels.page_for_row(row),
        }
    }
}

struct RasterTextureRetirementOwner {
    key: Option<RasterTextureKey>,
    scene_revision: Option<u64>,
    preview_generation: Option<u64>,
    operation: Option<u64>,
    width: Option<u32>,
    height: Option<u32>,
    bytes: Option<usize>,
    bind_group: Option<wgpu::BindGroup>,
    view: Option<wgpu::TextureView>,
    texture: Option<wgpu::Texture>,
}

impl RasterTextureRetirementOwner {
    fn new(entry: RasterTextureEntry<RasterTexture>) -> Self {
        Self {
            key: Some(entry.key),
            scene_revision: Some(entry.witness.scene_revision),
            preview_generation: Some(entry.witness.preview_generation),
            operation: Some(entry.witness.operation),
            width: Some(entry.value.width),
            height: Some(entry.value.height),
            bytes: Some(entry.bytes),
            bind_group: Some(entry.value.bind_group),
            view: Some(entry.value.view),
            texture: Some(entry.value.texture),
        }
    }

    fn step(&mut self) -> RasterTextureCleanupStep {
        if self.bind_group.take().is_some() {
            return RasterTextureCleanupStep::root();
        }
        if self.view.take().is_some() {
            return RasterTextureCleanupStep::root();
        }
        if let Some(texture) = self.texture.take() {
            texture.destroy();
            return RasterTextureCleanupStep::root();
        }
        if self.key.take().is_some() {
            return RasterTextureCleanupStep::root();
        }
        if self.scene_revision.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.preview_generation.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.operation.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.width.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.height.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.bytes.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        RasterTextureCleanupStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.key.is_none()
            && self.scene_revision.is_none()
            && self.preview_generation.is_none()
            && self.operation.is_none()
            && self.width.is_none()
            && self.height.is_none()
            && self.bytes.is_none()
            && self.bind_group.is_none()
            && self.view.is_none()
            && self.texture.is_none()
    }

    fn retained_bytes(&self) -> usize {
        self.bytes.unwrap_or(0)
    }
}

struct RasterTextureUploadCloseCursor {
    source: Option<RasterTextureUploadCursor>,
    admission: Option<RasterTextureAdmission>,
    admission_retirement: Option<RasterTextureReservationRetirement>,
    allocation_claim_retirement: Option<RasterTextureStageClaimRetirement>,
    owner: Option<RasterTextureRetirementOwner>,
    row_retired: bool,
}

impl RasterTextureUploadCloseCursor {
    fn new(source: RasterTextureUploadCursor) -> Self {
        Self { source: Some(source), admission: None, admission_retirement: None, allocation_claim_retirement: None, owner: None, row_retired: false }
    }

    fn step(&mut self) -> RasterTextureCleanupStep {
        if let Some(owner) = self.owner.as_mut() {
            let step = owner.step();
            if matches!(step, RasterTextureCleanupStep::Complete) {
                assert!(owner.terminal_is_empty(), "completed raster GPU owner must be terminal-empty");
                self.owner = None;
                return RasterTextureCleanupStep::retained();
            }
            return step;
        }
        if let Some(retirement) = self.allocation_claim_retirement.as_mut() {
            let step = retirement.step();
            if matches!(step, RasterTextureCleanupStep::Complete) {
                assert!(retirement.terminal_is_empty(), "completed raster allocation claim must be terminal-empty");
                self.allocation_claim_retirement = None;
                return RasterTextureCleanupStep::retained();
            }
            return step;
        }
        if let Some(retirement) = self.admission_retirement.as_mut() {
            let step = retirement.step();
            if matches!(step, RasterTextureCleanupStep::Complete) {
                assert!(retirement.terminal_is_empty(), "completed raster admission must be terminal-empty");
                self.admission_retirement = None;
                return RasterTextureCleanupStep::retained();
            }
            return step;
        }
        if let Some(source) = self.source.as_mut() {
            if self.admission.is_none() {
                if let Some(admission) = source.admission.take() {
                    self.admission = Some(admission);
                    return RasterTextureCleanupStep::retained();
                }
            }
            if let Some(claim) = source.allocation_claim.take() {
                self.allocation_claim_retirement = Some(RasterTextureStageClaimRetirement::new(claim));
                return RasterTextureCleanupStep::retained();
            }
            if source.texture.is_some() || source.view.is_some() || source.bind_group.is_some() {
                let admission = self.admission.as_ref().expect("retained upload close admission");
                self.owner = Some(RasterTextureRetirementOwner {
                    key: Some(admission.key),
                    scene_revision: Some(admission.witness.scene_revision),
                    preview_generation: Some(admission.witness.preview_generation),
                    operation: Some(admission.witness.operation),
                    width: Some(admission.width),
                    height: Some(admission.height),
                    bytes: Some(admission.bytes),
                    bind_group: source.bind_group.take(),
                    view: source.view.take(),
                    texture: source.texture.take(),
                });
                return RasterTextureCleanupStep::retained();
            }
            if !self.row_retired {
                source.row = 0;
                self.row_retired = true;
                return RasterTextureCleanupStep::scalar();
            }
            self.source = None;
            return RasterTextureCleanupStep::retained();
        }
        if let Some(admission) = self.admission.take() {
            self.admission_retirement = Some(RasterTextureReservationRetirement::new(RasterTextureReservation {
                key: admission.key,
                witness: admission.witness,
                width: admission.width,
                height: admission.height,
                bytes: admission.bytes,
                staged_index: admission.staged_index,
                nonce: admission.nonce,
            }));
            return RasterTextureCleanupStep::retained();
        }
        RasterTextureCleanupStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.source.is_none() && self.admission.is_none() && self.admission_retirement.is_none() && self.allocation_claim_retirement.is_none() && self.owner.is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RasterTextureRetirementMode {
    Abort(RasterTextureWitness),
    Commit(RasterTextureWitness),
    Close,
}

struct RasterTextureRetirementCursor {
    mode: RasterTextureRetirementMode,
    scan: usize,
    owner: Option<RasterTextureRetirementOwner>,
    candidate_retired: bool,
    presenting_retired: bool,
}

pub struct RasterTextureTable {
    live: FixedRasterTextureRegistry<RasterTexture>,
    staged: FixedRasterTextureRegistry<RasterTexture>,
    upload: Option<RasterTextureUploadCursor>,
    upload_close: Option<RasterTextureUploadCloseCursor>,
    retirement: Option<RasterTextureRetirementCursor>,
    reservation: Option<RasterTextureReservation>,
    reservation_retirement: Option<RasterTextureReservationCloseCursor>,
    candidate: RasterTextureWitnessSlot,
    presenting: RasterTextureWitnessSlot,
    next_reservation_nonce: u64,
    layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    closing: bool,
}

impl RasterTextureTable {
    pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor { label: Some("raster_sampler"), mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, ..Default::default() });
        Self {
            live: FixedRasterTextureRegistry::default(),
            staged: FixedRasterTextureRegistry::default(),
            upload: None,
            upload_close: None,
            retirement: None,
            reservation: None,
            reservation_retirement: None,
            candidate: RasterTextureWitnessSlot::default(),
            presenting: RasterTextureWitnessSlot::default(),
            next_reservation_nonce: 1,
            layout: layout.clone(),
            sampler,
            closing: false,
        }
    }

    fn retained_bytes(&self) -> usize {
        self.live.bytes
            + self.staged.bytes
            + self.reservation.map_or(0, |reservation| reservation.bytes)
            + self.reservation_retirement.as_ref().map_or(0, RasterTextureReservationCloseCursor::retained_bytes)
            + self.retirement.as_ref().and_then(|cursor| cursor.owner.as_ref()).map_or(0, RasterTextureRetirementOwner::retained_bytes)
    }

    fn retained_items(&self) -> usize {
        self.live.len
            + self.staged.len
            + usize::from(self.reservation.is_some())
            + self.reservation_retirement.as_ref().map_or(0, RasterTextureReservationCloseCursor::retained_items)
            + usize::from(self.retirement.as_ref().is_some_and(|cursor| cursor.owner.is_some()))
    }

    fn validate_freshness(&self, key: RasterTextureKey, witness: RasterTextureWitness) -> Result<(), &'static str> {
        if let Ok(Ok(index)) = self.live.locate(key) {
            let current = self.live.slots[index].as_ref().expect("located live raster");
            if raster_witness_is_stale(current.witness, witness) {
                return Err("raster operation witness was stale or duplicated");
            }
        }
        Ok(())
    }

    pub fn reserve_engine_texture(&mut self, key: &str, width: u32, height: u32, candidate: RasterTextureWitness, expected: RasterTextureWitness) -> Result<RasterTextureAdmission, &'static str> {
        if let Some(retirement) = self.reservation_retirement.as_mut() {
            if matches!(retirement.step(), RasterTextureCleanupStep::Complete) {
                assert!(retirement.terminal_is_empty(), "completed raster reservation must be terminal-empty");
                self.reservation_retirement = None;
            }
            return Err("raster cancelled reservation retirement is pending");
        }
        if candidate != expected {
            return Err("raster operation authority was stale before admission");
        }
        if self.closing || self.upload.is_some() || self.upload_close.is_some() || self.retirement.is_some() || self.reservation.is_some() {
            return Err("raster texture table is occupied");
        }
        if self.candidate.get().is_some_and(|witness| witness != candidate) {
            return Err("raster candidate generation was occupied");
        }
        let key = RasterTextureKey::new(key)?;
        let bytes = raster_texture_bytes(width, height).ok_or("raster texture byte credits overflowed")?;
        if width == 0 || height == 0 || bytes > RASTER_TEXTURE_ITEM_BYTE_CAPACITY {
            return Err("raster texture exceeded fixed item byte credits");
        }
        if self.retained_items() >= RASTER_TEXTURE_TABLE_CAPACITY {
            return Err("raster texture table item credits exhausted");
        }
        if self.retained_bytes().checked_add(bytes).is_none_or(|total| total > RASTER_TEXTURE_TABLE_BYTE_CAPACITY) {
            return Err("raster texture table byte credits exhausted");
        }
        self.validate_freshness(key, candidate)?;
        if self.staged.get(key.as_str()).is_some() {
            return Err("raster staged generation was duplicated");
        }
        self.live.locate(key)?;
        let staged_index = match self.staged.locate(key)? {
            Ok(index) | Err(index) => index,
        };
        let nonce = self.next_reservation_nonce;
        self.next_reservation_nonce = self.next_reservation_nonce.checked_add(1).ok_or("raster reservation generation exhausted")?;
        let reservation = RasterTextureReservation { key, witness: candidate, width, height, bytes, staged_index, nonce };
        self.reservation = Some(reservation);
        if self.candidate.get().is_none() {
            self.candidate.set(candidate);
        }
        Ok(RasterTextureAdmission { key, witness: candidate, width, height, bytes, staged_index, nonce })
    }

    pub fn cancel_engine_texture_admission(&mut self, admission: RasterTextureAdmission) -> Result<(), &'static str> {
        if self.reservation.as_ref().is_some_and(|reservation| reservation.matches(&admission)) {
            if self.reservation_retirement.is_some() {
                return Err("raster reservation retirement capacity exhausted");
            }
            let reservation = self.reservation.take().expect("matching raster reservation");
            self.reservation_retirement = Some(RasterTextureReservationCloseCursor::cancelled(reservation, admission));
            return Ok(());
        }
        if self.reservation_retirement.is_some() {
            return Err("raster rejected admission retirement capacity exhausted");
        }
        self.reservation_retirement = Some(RasterTextureReservationCloseCursor::rejected(admission));
        Err("raster admission was stale and retained for bounded retirement")
    }

    fn claim_stage_before_gpu_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<RasterTextureStageClaim, &'static str> {
        let staged_occupied = self.staged.slots.get(admission.staged_index).is_none_or(Option::is_some);
        claim_raster_stage_tuple(self.reservation, self.candidate.get(), staged_occupied, admission, expected)
    }

    fn claim_texture_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<RasterTextureStageClaim, &'static str> {
        self.claim_stage_before_gpu_allocation(admission, expected)
    }

    fn claim_view_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<RasterTextureStageClaim, &'static str> {
        self.claim_stage_before_gpu_allocation(admission, expected)
    }

    fn claim_bind_group_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<RasterTextureStageClaim, &'static str> {
        self.claim_stage_before_gpu_allocation(admission, expected)
    }

    pub(super) fn validate_engine_renderer_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<(), &'static str> {
        self.claim_stage_before_gpu_allocation(admission, expected).map(|_| ())
    }

    pub(super) fn validate_engine_target_texture_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<(), &'static str> {
        self.claim_stage_before_gpu_allocation(admission, expected).map(|_| ())
    }

    pub(super) fn validate_engine_target_view_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<(), &'static str> {
        self.claim_stage_before_gpu_allocation(admission, expected).map(|_| ())
    }

    pub(super) fn validate_engine_replacement_texture_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<(), &'static str> {
        self.claim_stage_before_gpu_allocation(admission, expected).map(|_| ())
    }

    pub(super) fn validate_engine_replacement_view_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<(), &'static str> {
        self.claim_stage_before_gpu_allocation(admission, expected).map(|_| ())
    }

    pub(super) fn retain_engine_allocation_fault(&mut self, admission: RasterTextureAdmission, texture: Option<wgpu::Texture>, view: Option<wgpu::TextureView>) {
        assert!(self.upload.is_none() && self.upload_close.is_none(), "matching raster allocation fault must own its reserved close slot");
        self.upload_close = Some(RasterTextureUploadCloseCursor::new(RasterTextureUploadCursor { admission: Some(admission), row: 0, texture, view, bind_group: None, allocation_claim: None }));
    }

    fn stage_claimed_texture(&mut self, admission: RasterTextureAdmission, value: RasterTexture, claim: RasterTextureStageClaim) -> Result<(), (&'static str, RasterTextureAdmission, RasterTexture)> {
        if self.reservation != Some(claim.reservation) || !claim.reservation.matches(&admission) || claim.candidate != admission.witness || claim.staged_index != admission.staged_index || claim.staged_nonce != admission.nonce {
            return Err(("raster allocation claim changed before publication", admission, value));
        }
        if self.candidate.get() != Some(claim.candidate) {
            return Err(("raster candidate witness changed after GPU allocation", admission, value));
        }
        if self.staged.slots[claim.staged_index].is_some() {
            return Err(("raster staged slot changed after GPU allocation", admission, value));
        }
        let entry = RasterTextureEntry { key: admission.key, witness: admission.witness, bytes: admission.bytes, value };
        if let Err(entry) = self.staged.insert_vacant(claim.staged_index, entry) {
            return Err(("raster staged slot changed after preflight", admission, entry.value));
        }
        let _completed_reservation = self.reservation.take().expect("published raster reservation");
        Ok(())
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    pub fn ensure_raster_step(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        globals_buffer: &wgpu::Buffer,
        glyph_view: &wgpu::TextureView,
        glyph_sampler: &wgpu::Sampler,
        _icon_view: &wgpu::TextureView,
        _icon_sampler: &wgpu::Sampler,
        key: &str,
        pixels: RasterUploadPixels<'_>,
        width: u32,
        height: u32,
        candidate: RasterTextureWitness,
        expected: RasterTextureWitness,
    ) -> Result<bool, &'static str> {
        const PAGE_BYTES: usize = 16 * 1024;
        if self.closing || self.retirement.is_some() {
            return Err("raster texture table is closing or retiring");
        }
        let row_bytes = usize::try_from(width).ok().and_then(|value| value.checked_mul(4)).ok_or("raster row byte credits overflowed")?;
        let expected_bytes = row_bytes.checked_mul(usize::try_from(height).map_err(|_| "raster height exceeded fixed credits")?).ok_or("raster byte credits overflowed")?;
        if width == 0 || height == 0 || row_bytes > PAGE_BYTES || pixels.len() != expected_bytes || !pixels.dimensions_match(width, height) {
            return Err("raster upload exceeded fixed page or byte credits");
        }
        if self.upload.is_none() {
            let admission = self.reserve_engine_texture(key, width, height, candidate, expected)?;
            self.upload = Some(RasterTextureUploadCursor { admission: Some(admission), row: 0, texture: None, view: None, bind_group: None, allocation_claim: None });
            let allocation_claim = {
                let admission = self.upload.as_ref().and_then(|cursor| cursor.admission.as_ref()).expect("retained raster texture admission");
                match self.claim_texture_allocation(admission, expected) {
                    Ok(claim) => claim,
                    Err(fault) => return Err(fault),
                }
            };
            self.upload.as_mut().expect("retained raster texture claim").allocation_claim = Some(allocation_claim);
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
            self.upload.as_mut().expect("retained raster texture upload").texture = Some(texture);
            let allocation_claim = {
                let admission = self.upload.as_ref().and_then(|cursor| cursor.admission.as_ref()).expect("retained raster view admission");
                match self.claim_view_allocation(admission, expected) {
                    Ok(claim) => claim,
                    Err(fault) => return Err(fault),
                }
            };
            self.upload.as_mut().expect("retained raster view claim").allocation_claim = Some(allocation_claim);
            let view = self.upload.as_ref().and_then(|cursor| cursor.texture.as_ref()).expect("retained raster texture owner").create_view(&wgpu::TextureViewDescriptor::default());
            self.upload.as_mut().expect("retained raster view upload").view = Some(view);
            let allocation_claim = {
                let admission = self.upload.as_ref().and_then(|cursor| cursor.admission.as_ref()).expect("retained raster bind-group admission");
                match self.claim_bind_group_allocation(admission, expected) {
                    Ok(claim) => claim,
                    Err(fault) => return Err(fault),
                }
            };
            self.upload.as_mut().expect("retained raster bind-group claim").allocation_claim = Some(allocation_claim);
            let view = self.upload.as_ref().and_then(|cursor| cursor.view.as_ref()).expect("retained raster view owner");
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
            let upload = self.upload.as_mut().expect("retained raster bind-group upload");
            upload.bind_group = Some(bind_group);
        }
        let cursor = self.upload.as_mut().expect("raster upload cursor initialized above");
        let admission = cursor.admission.as_ref().ok_or("raster upload admission was retired")?;
        if admission.key.as_str() != key || admission.witness != candidate || admission.width != width || admission.height != height || candidate != expected {
            return Err("raster upload authority was occupied by another generation");
        }
        if cursor.row < height {
            let rows = (PAGE_BYTES / row_bytes).max(1).min(usize::try_from(height - cursor.row).unwrap_or(usize::MAX));
            let start = usize::try_from(cursor.row).unwrap_or(usize::MAX).saturating_mul(row_bytes);
            let end = start.saturating_add(rows.saturating_mul(row_bytes));
            let texture = cursor.texture.as_ref().ok_or("raster upload texture was retired")?;
            let (page, page_rows) = pixels.rows(cursor.row, start, end, rows as u32).ok_or("raster prepared page ownership was incomplete")?;
            queue.write_texture(
                wgpu::TexelCopyTextureInfo { texture, mip_level: 0, origin: wgpu::Origin3d { x: 0, y: cursor.row, z: 0 }, aspect: wgpu::TextureAspect::All },
                page,
                wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(width * 4), rows_per_image: Some(page_rows) },
                wgpu::Extent3d { width, height: page_rows, depth_or_array_layers: 1 },
            );
            cursor.row += page_rows;
            return Ok(false);
        }
        let mut cursor = self.upload.take().expect("completed raster upload cursor");
        let admission = cursor.admission.take().expect("completed raster upload admission");
        let allocation_claim = cursor.allocation_claim.take().expect("completed raster allocation claim");
        let value = RasterTexture {
            texture: cursor.texture.take().expect("completed raster upload texture"),
            view: cursor.view.take().expect("completed raster upload view"),
            bind_group: cursor.bind_group.take().expect("completed raster upload bind group"),
            width,
            height,
        };
        if let Err((fault, admission, value)) = self.stage_claimed_texture(admission, value, allocation_claim) {
            self.upload_close = Some(RasterTextureUploadCloseCursor::new(RasterTextureUploadCursor {
                admission: Some(admission),
                row: height,
                texture: Some(value.texture),
                view: Some(value.view),
                bind_group: Some(value.bind_group),
                allocation_claim: Some(allocation_claim),
            }));
            return Err(fault);
        }
        Ok(true)
    }

    pub fn get(&self, key: &str) -> Option<&RasterTexture> {
        if let Some(witness) = self.presenting.get() {
            if let Some(entry) = self.staged.get(key).filter(|entry| entry.witness == witness) {
                return Some(&entry.value);
            }
        }
        self.live.get(key).map(|entry| &entry.value)
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    pub fn stage_gpu_bind_group(
        &mut self,
        device: &wgpu::Device,
        globals_buffer: &wgpu::Buffer,
        glyph_view: &wgpu::TextureView,
        glyph_sampler: &wgpu::Sampler,
        admission: RasterTextureAdmission,
        raster_view: wgpu::TextureView,
        texture: wgpu::Texture,
        expected: RasterTextureWitness,
    ) -> Result<(), RasterTextureStageFault> {
        let allocation_claim = match self.claim_bind_group_allocation(&admission, expected) {
            Ok(claim) => claim,
            Err(fault) => return Err(RasterTextureStageFault::Returned { fault, admission, texture, view: raster_view }),
        };
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("raster_bind_group"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: globals_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(glyph_view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(glyph_sampler) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&raster_view) },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            ],
        });
        let width = admission.width;
        let height = admission.height;
        let value = RasterTexture { texture, view: raster_view, bind_group, width, height };
        if let Err((fault, admission, value)) = self.stage_claimed_texture(admission, value, allocation_claim) {
            assert!(self.upload.is_none() && self.upload_close.is_none(), "matching raster publication fault must own its reserved close slot");
            self.upload_close = Some(RasterTextureUploadCloseCursor::new(RasterTextureUploadCursor {
                admission: Some(admission),
                row: height,
                texture: Some(value.texture),
                view: Some(value.view),
                bind_group: Some(value.bind_group),
                allocation_claim: Some(allocation_claim),
            }));
            return Err(RasterTextureStageFault::Retained(fault));
        }
        Ok(())
    }

    pub fn begin_presenting(&mut self, witness: RasterTextureWitness) -> Result<bool, &'static str> {
        if self.closing || self.retirement.is_some() || self.upload.is_some() {
            return Err("raster candidate was not terminal before presentation");
        }
        if self.staged.is_empty() {
            return Ok(false);
        }
        if self.candidate.get() != Some(witness) {
            return Err("raster candidate witness was stale before presentation");
        }
        if !self.presenting.is_empty() {
            return Err("raster presentation witness was already occupied");
        }
        self.presenting.set(witness);
        Ok(true)
    }

    fn begin_retirement(&mut self, mode: RasterTextureRetirementMode) -> Result<(), &'static str> {
        if let Some(cursor) = self.retirement.as_ref() {
            return (cursor.mode == mode).then_some(()).ok_or("raster retirement authority was occupied");
        }
        self.retirement = Some(RasterTextureRetirementCursor { mode, scan: 0, owner: None, candidate_retired: false, presenting_retired: false });
        Ok(())
    }

    fn retirement_step(&mut self) -> Result<bool, &'static str> {
        let Some(cursor) = self.retirement.as_mut() else { return Ok(true) };
        if let Some(owner) = cursor.owner.as_mut() {
            return match owner.step() {
                RasterTextureCleanupStep::Pending { .. } => Ok(false),
                RasterTextureCleanupStep::Blocked(fault) => Err(fault),
                RasterTextureCleanupStep::Complete => {
                    assert!(owner.terminal_is_empty(), "completed raster retirement owner must be terminal-empty");
                    cursor.owner = None;
                    Ok(false)
                }
            };
        }
        if cursor.scan >= RASTER_TEXTURE_TABLE_CAPACITY {
            match cursor.mode {
                RasterTextureRetirementMode::Abort(witness) | RasterTextureRetirementMode::Commit(witness) => {
                    if self.presenting.get() != Some(witness) {
                        return Err("raster retirement presentation witness was stale");
                    }
                }
                RasterTextureRetirementMode::Close => {}
            }
            if !cursor.candidate_retired {
                cursor.candidate_retired = self.candidate.retire_one();
                return Ok(false);
            }
            if !cursor.presenting_retired {
                cursor.presenting_retired = self.presenting.retire_one();
                return Ok(false);
            }
            self.retirement = None;
            return Ok(true);
        }
        let index = cursor.scan;
        cursor.scan += 1;
        match cursor.mode {
            RasterTextureRetirementMode::Abort(witness) => {
                let selected = self.staged.slots[index].as_ref().is_some_and(|entry| entry.witness == witness);
                if selected {
                    let entry = self.staged.take(index).expect("selected staged raster");
                    cursor.owner = Some(RasterTextureRetirementOwner::new(entry));
                }
            }
            RasterTextureRetirementMode::Commit(witness) => {
                let selected = self.staged.slots[index].as_ref().is_some_and(|entry| entry.witness == witness);
                if selected {
                    let entry = self.staged.take(index).expect("selected staged raster");
                    match self.live.insert(entry) {
                        Ok(Some(previous)) => cursor.owner = Some(RasterTextureRetirementOwner::new(previous)),
                        Ok(None) => {}
                        Err(entry) => {
                            if let Err(entry) = self.staged.insert_vacant(index, entry) {
                                cursor.owner = Some(RasterTextureRetirementOwner::new(entry));
                            }
                            return Err("raster live table capacity exhausted");
                        }
                    }
                }
            }
            RasterTextureRetirementMode::Close => {
                let staged = self.staged.take(index);
                if staged.is_some() && self.live.slots[index].is_some() {
                    cursor.scan = index;
                }
                if let Some(entry) = staged.or_else(|| self.live.take(index)) {
                    cursor.owner = Some(RasterTextureRetirementOwner::new(entry));
                }
            }
        }
        Ok(false)
    }

    pub fn commit_presented_step(&mut self, witness: RasterTextureWitness) -> Result<bool, &'static str> {
        if self.presenting.is_empty() && self.staged.is_empty() {
            return Ok(true);
        }
        if self.candidate.get() != Some(witness) || self.presenting.get() != Some(witness) {
            return Err("raster commit witness was stale");
        }
        self.begin_retirement(RasterTextureRetirementMode::Commit(witness))?;
        self.retirement_step()
    }

    pub fn abort_presented_step(&mut self, witness: RasterTextureWitness) -> Result<bool, &'static str> {
        if self.presenting.is_empty() && self.candidate.is_empty() && self.staged.is_empty() && self.upload.is_none() {
            return Ok(true);
        }
        if self.presenting.is_empty() {
            self.presenting.set(witness);
        }
        if self.candidate.get() != Some(witness) || self.presenting.get() != Some(witness) {
            return Err("raster abort witness was stale");
        }
        self.begin_retirement(RasterTextureRetirementMode::Abort(witness))?;
        self.retirement_step()
    }

    pub fn close_upload_step(&mut self) -> RasterTextureCleanupStep {
        if let Some(retirement) = self.reservation_retirement.as_mut() {
            let step = retirement.step();
            if matches!(step, RasterTextureCleanupStep::Complete) {
                assert!(retirement.terminal_is_empty(), "completed raster reservation must be terminal-empty");
                self.reservation_retirement = None;
                return RasterTextureCleanupStep::retained();
            }
            return step;
        }
        if let Some(reservation) = self.reservation.take() {
            self.reservation_retirement = Some(RasterTextureReservationCloseCursor::reservation(reservation));
            return RasterTextureCleanupStep::retained();
        }
        if self.upload_close.is_none() {
            if let Some(upload) = self.upload.take() {
                self.upload_close = Some(RasterTextureUploadCloseCursor::new(upload));
                return RasterTextureCleanupStep::retained();
            }
            return RasterTextureCleanupStep::Complete;
        }
        let cursor = self.upload_close.as_mut().expect("retained raster upload close cursor");
        let step = cursor.step();
        if matches!(step, RasterTextureCleanupStep::Complete) {
            assert!(cursor.terminal_is_empty(), "completed raster upload close must be terminal-empty");
            self.upload_close = None;
            return RasterTextureCleanupStep::retained();
        }
        step
    }

    pub fn close_step(&mut self) -> Result<bool, &'static str> {
        self.closing = true;
        match self.close_upload_step() {
            RasterTextureCleanupStep::Pending { .. } => return Ok(false),
            RasterTextureCleanupStep::Blocked(fault) => return Err(fault),
            RasterTextureCleanupStep::Complete => {}
        }
        if self.retirement.is_some() && !self.retirement_step()? {
            return Ok(false);
        }
        if self.retirement.is_none() && (!self.live.is_empty() || !self.staged.is_empty() || !self.candidate.is_empty() || !self.presenting.is_empty()) {
            self.begin_retirement(RasterTextureRetirementMode::Close)?;
            return Ok(false);
        }
        Ok(self.terminal_is_empty())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.live.is_empty()
            && self.staged.is_empty()
            && self.upload.is_none()
            && self.upload_close.is_none()
            && self.retirement.is_none()
            && self.reservation.is_none()
            && self.reservation_retirement.is_none()
            && self.candidate.is_empty()
            && self.presenting.is_empty()
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
        let Some(mesh) = mesh_store.get_versioned(&draw_call.mesh_key, draw_call.mesh_version) else {
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

    pub fn upload_glyph_atlas_page(&self, queue: &wgpu::Queue, pixels: &[u8], width: u32, start_row: u32, rows: u32) {
        queue.write_texture(
            wgpu::TexelCopyTextureInfo { texture: &self.glyph_texture, mip_level: 0, origin: wgpu::Origin3d { x: 0, y: start_row, z: 0 }, aspect: wgpu::TextureAspect::All },
            pixels,
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(width), rows_per_image: Some(rows) },
            wgpu::Extent3d { width, height: rows, depth_or_array_layers: 1 },
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

    pub fn upload_icon_atlas_page(&self, queue: &wgpu::Queue, pixels: &[u8], width: u32, start_row: u32, rows: u32) {
        queue.write_texture(
            wgpu::TexelCopyTextureInfo { texture: &self.icon_texture, mip_level: 0, origin: wgpu::Origin3d { x: 0, y: start_row, z: 0 }, aspect: wgpu::TextureAspect::All },
            pixels,
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(width * 4), rows_per_image: Some(rows) },
            wgpu::Extent3d { width, height: rows, depth_or_array_layers: 1 },
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
    use super::{
        ClipRegion, DrawList, FixedMeshGpuRegistry, FixedRasterTextureRegistry, MESH_GPU_KEEP_VERSION_CAPACITY, MESH_GPU_TABLE_CAPACITY, MeshGpuEntry, MeshGpuKey, RASTER_TEXTURE_ITEM_BYTE_CAPACITY, RASTER_TEXTURE_KEY_BYTES,
        RASTER_TEXTURE_PROBE_CAPACITY, RASTER_TEXTURE_TABLE_CAPACITY, RasterTextureAdmission, RasterTextureCleanupStep, RasterTextureEntry, RasterTextureKey, RasterTextureReservation, RasterTextureReservationCloseCursor,
        RasterTextureReservationRetirement, RasterTextureStageClaim, RasterTextureUploadCloseCursor, RasterTextureUploadCursor, RasterTextureWitness, RasterTextureWitnessSlot, ScissorRect, WORLD_GLOBALS_SLOT_SIZE, claim_raster_stage_tuple,
        content_stencil_state, ear_clip_polygon, mask_instances, mask_stencil_state, mesh_content_version, raster_texture_bytes, raster_witness_is_stale,
    };
    use crate::wgpu::geometry::Rect;
    use crate::wgpu::kernel_3d_scene::ScenePass3d;
    use crate::wgpu::theme::Rgba;

    #[test]
    fn fixed_mesh_gpu_registry_rejects_capacity_plus_one_and_returns_exact_owner() {
        let mut registry = FixedMeshGpuRegistry::default();
        for index in 0..MESH_GPU_TABLE_CAPACITY {
            let key = MeshGpuKey::new(&format!("mesh-{index}")).expect("bounded key");
            registry.insert(MeshGpuEntry { key, version: index as u64, value: Box::new(index) }).ok().expect("fixed mesh slot");
        }
        let rejected = Box::new(MESH_GPU_TABLE_CAPACITY);
        let rejected_pointer = (&*rejected) as *const usize;
        let rejected = registry.insert(MeshGpuEntry { key: MeshGpuKey::new("overflow").expect("bounded key"), version: 1, value: rejected }).expect_err("capacity plus one");
        assert_eq!((&*rejected.value) as *const usize, rejected_pointer);
        assert_eq!(*rejected.value, MESH_GPU_TABLE_CAPACITY);
        let first = registry.take(0).expect("first exact owner");
        assert_eq!(*first.value, 0);
        registry.insert(rejected).ok().expect("returned owner retries after one slot retires");
        assert_eq!(registry.len, MESH_GPU_TABLE_CAPACITY);
    }

    #[test]
    fn fixed_mesh_gpu_registry_version_identity_rejects_stale_aba_lookup() {
        let mut registry = FixedMeshGpuRegistry::default();
        registry.insert(MeshGpuEntry { key: MeshGpuKey::new("terrain").expect("bounded key"), version: 7, value: 70u64 }).ok().expect("first generation");
        assert_eq!(registry.get("terrain", 7), Some(&70));
        let old = registry.take(0).expect("old generation owner");
        assert_eq!(old.version, 7);
        registry.insert(MeshGpuEntry { key: MeshGpuKey::new("terrain").expect("bounded key"), version: 8, value: 80u64 }).ok().expect("replacement generation");
        assert!(registry.get("terrain", 7).is_none());
        assert_eq!(registry.get("terrain", 8), Some(&80));
    }

    fn raster_key_for_start(start: usize, ordinal: usize) -> RasterTextureKey {
        (0usize..1_000_000).map(|candidate| RasterTextureKey::new(&format!("raster-{start}-{ordinal}-{candidate}")).expect("bounded raster key")).find(|key| key.start() == start).expect("deterministic raster key for slot")
    }

    #[test]
    fn fixed_raster_registry_rejects_capacity_plus_one_with_exact_handback() {
        let mut registry = FixedRasterTextureRegistry::default();
        for index in 0..RASTER_TEXTURE_TABLE_CAPACITY {
            let key = raster_key_for_start(index, 0);
            registry.insert(RasterTextureEntry { key, witness: RasterTextureWitness { scene_revision: 1, preview_generation: 1, operation: index as u64 }, bytes: 1, value: Box::new(index) }).ok().expect("fixed raster slot");
        }
        let rejected = Box::new(RASTER_TEXTURE_TABLE_CAPACITY);
        let rejected_pointer = (&*rejected) as *const usize;
        let rejected =
            match registry.insert(RasterTextureEntry { key: RasterTextureKey::new("overflow").expect("bounded raster key"), witness: RasterTextureWitness { scene_revision: 2, preview_generation: 2, operation: 1 }, bytes: 1, value: rejected }) {
                Err(rejected) => rejected,
                Ok(_) => panic!("raster capacity plus one was accepted"),
            };
        assert_eq!((&*rejected.value) as *const usize, rejected_pointer);
        assert_eq!(registry.len, RASTER_TEXTURE_TABLE_CAPACITY);
    }

    #[test]
    fn fixed_raster_registry_probe_saturation_and_replacement_preserve_owners() {
        let start = 17;
        let mut registry = FixedRasterTextureRegistry::default();
        for ordinal in 0..RASTER_TEXTURE_PROBE_CAPACITY {
            let key = raster_key_for_start(start, ordinal);
            registry.insert(RasterTextureEntry { key, witness: RasterTextureWitness { scene_revision: 7, preview_generation: ordinal as u64, operation: ordinal as u64 + 1 }, bytes: 1, value: Box::new(ordinal) }).ok().expect("probe slot");
        }
        let rejected_owner = Box::new(99usize);
        let rejected_pointer = (&*rejected_owner) as *const usize;
        let rejected =
            match registry.insert(RasterTextureEntry { key: raster_key_for_start(start, RASTER_TEXTURE_PROBE_CAPACITY), witness: RasterTextureWitness { scene_revision: 8, preview_generation: 0, operation: 1 }, bytes: 1, value: rejected_owner }) {
                Err(rejected) => rejected,
                Ok(_) => panic!("probe capacity plus one was accepted"),
            };
        assert_eq!((&*rejected.value) as *const usize, rejected_pointer);
        let replacement_key = raster_key_for_start(start, 0);
        let previous =
            registry.insert(RasterTextureEntry { key: replacement_key, witness: RasterTextureWitness { scene_revision: 9, preview_generation: 3, operation: 77 }, bytes: 1, value: Box::new(777usize) }).ok().flatten().expect("exact replaced owner");
        assert_eq!(*previous.value, 0);
        let current = registry.get(replacement_key.as_str()).expect("replacement generation");
        assert_eq!((current.witness.scene_revision, current.witness.preview_generation, current.witness.operation, *current.value), (9, 3, 77, 777));
    }

    #[test]
    fn raster_key_and_byte_credits_reject_exact_plus_one() {
        let key = "k".repeat(RASTER_TEXTURE_KEY_BYTES);
        assert!(RasterTextureKey::new(&key).is_ok());
        assert!(RasterTextureKey::new(&(key + "x")).is_err());
        assert_eq!(raster_texture_bytes(2048, 2048), Some(RASTER_TEXTURE_ITEM_BYTE_CAPACITY));
        assert!(raster_texture_bytes(2048, 2049).is_some_and(|bytes| bytes > RASTER_TEXTURE_ITEM_BYTE_CAPACITY));
    }

    #[test]
    fn raster_operation_freshness_is_independent_and_aba_safe() {
        let current = RasterTextureWitness { scene_revision: 9, preview_generation: 4, operation: 12 };
        assert!(raster_witness_is_stale(current, RasterTextureWitness { operation: 11, ..current }));
        assert!(raster_witness_is_stale(current, current));
        assert!(!raster_witness_is_stale(current, RasterTextureWitness { operation: 13, ..current }));
    }

    #[test]
    fn raster_witness_close_retires_exactly_one_scalar_per_grant() {
        let mut slot = RasterTextureWitnessSlot::default();
        slot.set(RasterTextureWitness { scene_revision: 3, preview_generation: 5, operation: 7 });
        assert!(!slot.retire_one());
        assert!(slot.scene_revision.is_none());
        assert!(slot.preview_generation.is_some());
        assert!(!slot.retire_one());
        assert!(slot.preview_generation.is_none());
        assert!(slot.operation.is_some());
        assert!(!slot.retire_one());
        assert!(slot.operation.is_none());
        assert!(slot.retire_one());
    }

    #[test]
    fn raster_reservation_cancel_retires_one_exact_root_or_scalar_per_grant() {
        let reservation = RasterTextureReservation {
            key: RasterTextureKey::new("cancelled").expect("bounded key"),
            witness: RasterTextureWitness { scene_revision: 3, preview_generation: 5, operation: 7 },
            width: 16,
            height: 8,
            bytes: 512,
            staged_index: 11,
            nonce: 13,
        };
        let mut retirement = RasterTextureReservationRetirement::new(reservation);
        assert_eq!(retirement.step(), RasterTextureCleanupStep::Pending { released_roots: 1, released_scalars: 0 });
        for _ in 0..8 {
            assert_eq!(retirement.step(), RasterTextureCleanupStep::Pending { released_roots: 0, released_scalars: 1 });
        }
        assert_eq!(retirement.step(), RasterTextureCleanupStep::Complete);
    }

    #[test]
    fn raster_matching_cancel_retains_both_reservation_and_admission_to_terminal() {
        let witness = RasterTextureWitness { scene_revision: 61, preview_generation: 67, operation: 71 };
        let key = RasterTextureKey::new("matching-cancel").expect("bounded key");
        let reservation = RasterTextureReservation { key, witness, width: 32, height: 8, bytes: 1024, staged_index: 73, nonce: 79 };
        let admission = RasterTextureAdmission { key, witness, width: 32, height: 8, bytes: 1024, staged_index: 73, nonce: 79 };
        let mut close = RasterTextureReservationCloseCursor::cancelled(reservation, admission);
        let mut released_roots = 0;
        let mut released_scalars = 0;
        let mut steps = 0;
        loop {
            steps += 1;
            assert!(steps < 32, "matching cancellation must terminate");
            match close.step() {
                RasterTextureCleanupStep::Pending { released_roots: roots, released_scalars: scalars } => {
                    assert!(roots + scalars <= 1);
                    released_roots += roots;
                    released_scalars += scalars;
                }
                RasterTextureCleanupStep::Blocked(fault) => panic!("unexpected matching cancellation block: {fault}"),
                RasterTextureCleanupStep::Complete => break,
            }
        }
        assert_eq!((released_roots, released_scalars), (2, 16));
        assert!(close.terminal_is_empty());
    }

    #[test]
    fn raster_gpu_allocation_claim_rejects_missing_aba_candidate_and_occupied_slot() {
        fn authorities(nonce: u64) -> (RasterTextureReservation, RasterTextureAdmission, RasterTextureWitness) {
            let witness = RasterTextureWitness { scene_revision: 17, preview_generation: 19, operation: 23 };
            let key = RasterTextureKey::new("claimed").expect("bounded key");
            let reservation = RasterTextureReservation { key, witness, width: 32, height: 16, bytes: 2048, staged_index: 29, nonce };
            let admission = RasterTextureAdmission { key, witness, width: 32, height: 16, bytes: 2048, staged_index: 29, nonce };
            (reservation, admission, witness)
        }

        let (reservation, admission, witness) = authorities(31);
        let claim = claim_raster_stage_tuple(Some(reservation), Some(witness), false, &admission, witness).expect("full tuple");
        assert_eq!((claim.staged_index, claim.staged_nonce), (29, 31));
        assert!(claim_raster_stage_tuple(None, Some(witness), false, &admission, witness).is_err());

        let (reservation, stale_admission, witness) = authorities(37);
        assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), false, &stale_admission, witness).is_ok());
        let (_, aba_admission, _) = authorities(38);
        assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), false, &aba_admission, witness).is_err());
        let (_, mut mismatched, _) = authorities(37);
        mismatched.key = RasterTextureKey::new("other-key").expect("bounded key");
        assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), false, &mismatched, witness).is_err());
        let (_, mut mismatched, _) = authorities(37);
        mismatched.witness.scene_revision += 1;
        assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), false, &mismatched, mismatched.witness).is_err());
        let (_, mut mismatched, _) = authorities(37);
        mismatched.witness.preview_generation += 1;
        assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), false, &mismatched, mismatched.witness).is_err());
        let (_, mut mismatched, _) = authorities(37);
        mismatched.witness.operation += 1;
        assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), false, &mismatched, mismatched.witness).is_err());
        let (_, mut mismatched, _) = authorities(37);
        mismatched.width += 1;
        assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), false, &mismatched, witness).is_err());
        let (_, mut mismatched, _) = authorities(37);
        mismatched.height += 1;
        assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), false, &mismatched, witness).is_err());
        let (_, mut mismatched, _) = authorities(37);
        mismatched.bytes += 1;
        assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), false, &mismatched, witness).is_err());
        let (_, mut mismatched, _) = authorities(37);
        mismatched.staged_index += 1;
        assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), false, &mismatched, witness).is_err());
        assert!(claim_raster_stage_tuple(Some(reservation), Some(RasterTextureWitness { operation: 24, ..witness }), false, &stale_admission, witness).is_err());
        assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), true, &stale_admission, witness).is_err());
    }

    #[test]
    fn raster_interrupted_upload_close_is_truthful_before_first_and_mid_page() {
        for row in [0, 7] {
            let witness = RasterTextureWitness { scene_revision: 41, preview_generation: 43, operation: 47 };
            let key = RasterTextureKey::new("interrupted").expect("bounded key");
            let reservation = RasterTextureReservation { key, witness, width: 64, height: 64, bytes: 16 * 1024, staged_index: 53, nonce: 59 };
            let admission = RasterTextureAdmission { key, witness, width: 64, height: 64, bytes: 16 * 1024, staged_index: 53, nonce: 59 };
            let claim = RasterTextureStageClaim { reservation, candidate: witness, staged_index: 53, staged_nonce: 59 };
            let mut close = RasterTextureUploadCloseCursor::new(RasterTextureUploadCursor { admission: Some(admission), row, texture: None, view: None, bind_group: None, allocation_claim: Some(claim) });
            let mut steps = 0;
            loop {
                steps += 1;
                assert!(steps < 64, "retained upload close must terminate");
                match close.step() {
                    RasterTextureCleanupStep::Pending { released_roots, released_scalars } => assert!(released_roots + released_scalars <= 1),
                    RasterTextureCleanupStep::Blocked(fault) => panic!("unexpected retained close block: {fault}"),
                    RasterTextureCleanupStep::Complete => break,
                }
            }
            assert!(close.terminal_is_empty());
        }
    }

    #[test]
    fn mesh_gpu_retirement_preserves_acknowledged_versions() {
        let key = MeshGpuKey::new("terrain").expect("bounded key");
        let other = MeshGpuKey::new("other").expect("bounded key");
        let mut versions = [0; MESH_GPU_KEEP_VERSION_CAPACITY];
        versions[..2].copy_from_slice(&[8, 9]);
        let selector = super::MeshGpuRetirementSelector::KeyExcept { key, versions, len: 2 };
        assert!(selector.selects(&MeshGpuEntry { key, version: 7, value: () }));
        assert!(!selector.selects(&MeshGpuEntry { key, version: 8, value: () }));
        assert!(!selector.selects(&MeshGpuEntry { key, version: 9, value: () }));
        assert!(!selector.selects(&MeshGpuEntry { key: other, version: 7, value: () }));
    }

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
        use super::{LayerBatchFilter, build_layer_batches, build_overlay_layer_batches};
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
        use super::{LayerBatchFilter, Theme, build_layer_batches};
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
