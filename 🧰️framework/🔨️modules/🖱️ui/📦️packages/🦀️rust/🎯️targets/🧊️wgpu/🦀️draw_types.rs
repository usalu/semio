// #region draw_types
//! 🧩 Target-neutral draw-list and value types split out of `draw.rs` — CPU-side accumulation
//! buffers, geometry math, and fault/id types with zero reference to the `wgpu` GPU crate, so
//! they compile under the light `wgpu` feature (no `wgpu-engine`) for `wasm32-wasip2` program
//! components. See `.🦑️repo/🎫️tickets/26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/🔍️research/📓️wgpu-tier-split.md`.
//! `draw.rs` (the real `wgpu`-crate-touching GPU pipeline, still `wgpu-engine`-gated) re-exports
//! this module wholesale so every pre-existing `crate::wgpu::draw::{DrawList, ...}` import path
//! keeps resolving unchanged.

use super::kernel_3d_scene::ScenePass3d;
use crate::wgpu::theme::{GlassStyle, Rgba, Theme};
use bytemuck::{Pod, Zeroable};

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

#[derive(Clone, Copy, Debug)]
pub struct GlassRegion {
    pub rect: [f32; 4],
    pub radius: f32,
    pub tint: Rgba,
    pub alpha: f32,
    pub blur_px: f32,
    pub saturate: f32,
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

    // 🧩️ `pub(crate)`, not private: `draw.rs`'s retained GPU pipeline (a sibling module now that
    // `ClipRegion` moved here — ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS)
    // calls this directly to resolve a node's effective scissor rect against its GPU render pass.
    pub(crate) fn effective_scissors(&self, scissor: Option<ScissorRect>, width: f32, height: f32) -> Vec<ScissorRect> {
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
    prepared_items: usize,
    prepared_bytes: usize,
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
        let mut list = Self {
            scene_passes: Vec::new(),
            layers: Vec::new(),
            glass_regions: Vec::new(),
            scissor_stack: Vec::new(),
            clip_stack: Vec::new(),
            glass_content_stack: Vec::new(),
            screen_h: 720.0,
            retained_output: None,
            prepared_items: 0,
            prepared_bytes: 0,
        };
        list.layers.push(DrawLayer::default());
        list
    }
}

fn prepared_scene_pass_usage(pass: &ScenePass3d) -> Option<(usize, usize)> {
    let mut items = 1usize;
    let mut bytes = std::mem::size_of::<ScenePass3d>();
    let mut include = |next_items: usize, next_bytes: usize| {
        items = items.checked_add(next_items)?;
        bytes = bytes.checked_add(next_bytes)?;
        Some(())
    };
    include(pass.draws.len(), pass.draws.capacity().checked_mul(std::mem::size_of::<crate::wgpu::kernel_3d_scene::SceneDraw3d>())?)?;
    include(pass.translucent_draws.len(), pass.translucent_draws.capacity().checked_mul(std::mem::size_of::<crate::wgpu::kernel_3d_scene::SceneDraw3d>())?)?;
    for draw in pass.draws.iter().chain(pass.translucent_draws.iter()) {
        include(draw.mesh_key.len(), draw.mesh_key.capacity())?;
        include(draw.instances.len(), draw.instances.capacity().checked_mul(std::mem::size_of::<crate::wgpu::kernel_3d_scene::Instance3d>())?)?;
        for instance in &draw.instances {
            include(instance.id.len(), instance.id.capacity())?;
        }
    }
    include(pass.line_draws.len(), pass.line_draws.capacity().checked_mul(std::mem::size_of::<crate::wgpu::kernel_3d_scene::LineDraw3d>())?)?;
    for draw in &pass.line_draws {
        include(draw.vertices.len(), draw.vertices.capacity().checked_mul(std::mem::size_of::<crate::wgpu::kernel_3d_scene::LineVertex3d>())?)?;
    }
    include(pass.textured_draws.len(), pass.textured_draws.capacity().checked_mul(std::mem::size_of::<crate::wgpu::kernel_3d_scene::TexturedDraw3d>())?)?;
    for draw in &pass.textured_draws {
        include(draw.instances.len(), draw.instances.capacity().checked_mul(std::mem::size_of::<crate::wgpu::kernel_3d_scene::TexturedInstance3d>())?)?;
        for instance in &draw.instances {
            include(instance.texture_key.len(), instance.texture_key.capacity())?;
        }
    }
    Some((items, bytes))
}

impl DrawList {
    /// 🪣 Creates an allocation-free transfer slot for a later exact draw admission.
    pub fn empty() -> Self {
        Self { scene_passes: Vec::new(), layers: Vec::new(), glass_regions: Vec::new(), scissor_stack: Vec::new(), clip_stack: Vec::new(), glass_content_stack: Vec::new(), screen_h: 0.0, retained_output: None, prepared_items: 0, prepared_bytes: 0 }
    }

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
        let Some(prepared_items) = self.prepared_items.checked_add(items) else { return false };
        let Some(prepared_bytes) = self.prepared_bytes.checked_add(bytes) else { return false };
        if let Some(grant) = self.retained_output.as_mut() {
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
        }
        self.prepared_items = prepared_items;
        self.prepared_bytes = prepared_bytes;
        true
    }

    /// 📊 Returns the cumulative pre-transfer draw claim maintained at every producer push.
    pub(crate) fn prepared_output_usage(&self) -> (usize, usize) {
        (self.prepared_items, self.prepared_bytes)
    }

    pub(crate) fn retire_step(&mut self) -> bool {
        if let Some(pass) = self.scene_passes.last_mut() {
            if let Some(draw) = pass.textured_draws.last_mut() {
                if let Some(instance) = draw.instances.last_mut() {
                    if instance.texture_key.pop().is_some() {
                        return false;
                    }
                    if instance.texture_key.capacity() > 0 {
                        instance.texture_key = String::new();
                        return false;
                    }
                    draw.instances.pop();
                    return false;
                }
                if draw.instances.capacity() > 0 {
                    draw.instances = Vec::new();
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
                    if instance.id.capacity() > 0 {
                        instance.id = String::new();
                        return false;
                    }
                    draw.instances.pop();
                    return false;
                }
                if draw.instances.capacity() > 0 {
                    draw.instances = Vec::new();
                    return false;
                }
                if draw.mesh_key.pop().is_some() {
                    return false;
                }
                if draw.mesh_key.capacity() > 0 {
                    draw.mesh_key = String::new();
                    return false;
                }
                pass.translucent_draws.pop();
                return false;
            }
            if let Some(draw) = pass.line_draws.last_mut() {
                if draw.vertices.pop().is_some() {
                    return false;
                }
                if draw.vertices.capacity() > 0 {
                    draw.vertices = Vec::new();
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
                    if instance.id.capacity() > 0 {
                        instance.id = String::new();
                        return false;
                    }
                    draw.instances.pop();
                    return false;
                }
                if draw.instances.capacity() > 0 {
                    draw.instances = Vec::new();
                    return false;
                }
                if draw.mesh_key.pop().is_some() {
                    return false;
                }
                if draw.mesh_key.capacity() > 0 {
                    draw.mesh_key = String::new();
                    return false;
                }
                pass.draws.pop();
                return false;
            }
            if pass.textured_draws.capacity() > 0 {
                pass.textured_draws = Vec::new();
                return false;
            }
            if pass.translucent_draws.capacity() > 0 {
                pass.translucent_draws = Vec::new();
                return false;
            }
            if pass.line_draws.capacity() > 0 {
                pass.line_draws = Vec::new();
                return false;
            }
            if pass.draws.capacity() > 0 {
                pass.draws = Vec::new();
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
                if clip.scissors.capacity() > 0 {
                    clip.scissors = Vec::new();
                    return false;
                }
                layer.clip = None;
                return false;
            }
            if let Some((key, _)) = layer.raster_instances.last_mut() {
                if key.pop().is_some() {
                    return false;
                }
                if key.capacity() > 0 {
                    *key = String::new();
                    return false;
                }
                layer.raster_instances.pop();
                return false;
            }
            if layer.overlay_vector_vertices.pop().is_some() || layer.overlay_ui_instances.pop().is_some() || layer.vector_vertices.pop().is_some() || layer.ui_instances.pop().is_some() {
                return false;
            }
            if layer.overlay_vector_vertices.capacity() > 0 {
                layer.overlay_vector_vertices = Vec::new();
                return false;
            }
            if layer.overlay_ui_instances.capacity() > 0 {
                layer.overlay_ui_instances = Vec::new();
                return false;
            }
            if layer.vector_vertices.capacity() > 0 {
                layer.vector_vertices = Vec::new();
                return false;
            }
            if layer.ui_instances.capacity() > 0 {
                layer.ui_instances = Vec::new();
                return false;
            }
            if layer.raster_instances.capacity() > 0 {
                layer.raster_instances = Vec::new();
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
            if clip.scissors.capacity() > 0 {
                clip.scissors = Vec::new();
                return false;
            }
            self.clip_stack.pop();
            return false;
        }
        if self.scene_passes.capacity() > 0 {
            self.scene_passes = Vec::new();
            return false;
        }
        if self.layers.capacity() > 0 {
            self.layers = Vec::new();
            return false;
        }
        if self.glass_regions.capacity() > 0 {
            self.glass_regions = Vec::new();
            return false;
        }
        if self.scissor_stack.capacity() > 0 {
            self.scissor_stack = Vec::new();
            return false;
        }
        if self.clip_stack.capacity() > 0 {
            self.clip_stack = Vec::new();
            return false;
        }
        if self.glass_content_stack.capacity() > 0 {
            self.glass_content_stack = Vec::new();
            return false;
        }
        if self.retained_output.take().is_some() {
            return false;
        }
        if self.prepared_items != 0 {
            self.prepared_items = 0;
            return false;
        }
        if self.prepared_bytes != 0 {
            self.prepared_bytes = 0;
            return false;
        }
        true
    }

    pub(crate) fn retirement_is_empty(&self) -> bool {
        self.scene_passes.is_empty()
            && self.scene_passes.capacity() == 0
            && self.layers.is_empty()
            && self.layers.capacity() == 0
            && self.glass_regions.is_empty()
            && self.glass_regions.capacity() == 0
            && self.scissor_stack.is_empty()
            && self.scissor_stack.capacity() == 0
            && self.clip_stack.is_empty()
            && self.clip_stack.capacity() == 0
            && self.glass_content_stack.is_empty()
            && self.glass_content_stack.capacity() == 0
            && self.retained_output.is_none()
            && self.prepared_items == 0
            && self.prepared_bytes == 0
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
        let Some(items) = rects.len().checked_add(2) else {
            let _ = self.claim_retained_output(usize::MAX, usize::MAX);
            return;
        };
        let Some(bytes) = rects.len().checked_mul(std::mem::size_of::<ScissorRect>()).and_then(|bytes| bytes.checked_add(std::mem::size_of::<DrawLayer>())) else {
            let _ = self.claim_retained_output(usize::MAX, usize::MAX);
            return;
        };
        if !self.claim_retained_output(items, bytes) {
            return;
        }
        let mut clip = ClipRegion::from_rects(rects, self.screen_h);
        if let Some(parent) = self.clip_stack.last() {
            clip = parent.intersect(&clip);
        }
        self.clip_stack.push(clip.clone());
        self.layers.push(DrawLayer { scissor: self.scissor_stack.last().copied(), clip: Some(clip), foreground_of: self.active_foreground_of(), ..DrawLayer::default() });
    }

    pub fn end_silhouette_clip(&mut self) {
        if !self.claim_retained_output(1, std::mem::size_of::<DrawLayer>()) {
            return;
        }
        self.clip_stack.pop();
        self.layers.push(DrawLayer { scissor: self.scissor_stack.last().copied(), clip: self.clip_stack.last().cloned(), foreground_of: self.active_foreground_of(), ..DrawLayer::default() });
    }

    pub fn push_scene_pass(&mut self, mut pass: ScenePass3d) {
        let Some((items, bytes)) = prepared_scene_pass_usage(&pass) else {
            let _ = self.claim_retained_output(usize::MAX, usize::MAX);
            return;
        };
        if !self.claim_retained_output(items, bytes) {
            return;
        }
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
        if !self.claim_retained_output(1, std::mem::size_of::<DrawLayer>()) {
            return;
        }
        self.glass_content_stack.push(region);
        self.layers.push(DrawLayer { scissor: self.scissor_stack.last().copied(), clip: self.clip_stack.last().cloned(), foreground_of: Some(region), ..DrawLayer::default() });
    }

    pub fn end_glass_content(&mut self) {
        if !self.claim_retained_output(1, std::mem::size_of::<DrawLayer>()) {
            return;
        }
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
        let Some(bytes) = vertices.checked_mul(std::mem::size_of::<VectorVertex>()) else {
            let _ = self.claim_retained_output(usize::MAX, usize::MAX);
            return;
        };
        if !self.claim_retained_output(1, bytes) {
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
        let Some(bytes) = vertices.checked_mul(std::mem::size_of::<VectorVertex>()) else {
            let _ = self.claim_retained_output(usize::MAX, usize::MAX);
            return;
        };
        if !self.claim_retained_output(1, bytes) {
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

//#region gizmo
/// 🧭️ Orbit-view gizmo placement/tip-geometry/hit-test math split out of `widgets.rs`'s gizmo
/// module (ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS) — pure
/// screen-space math with zero `WidgetContext`/`DrawList` touch, so it stays available for
/// `wasm32-wasip2` pointer-hover hit-testing without pulling `wgpu-engine`. The actual paint call
/// (`paint_orbit_view_gizmo`, which needs the GPU-adjacent `WidgetContext`) stays in
/// `widgets.rs`'s own `gizmo` module, still `wgpu-engine`-gated.
pub mod gizmo {
    use crate::wgpu::{Camera3d, Rect, Rgba, Vec3, Vec3Math};

    /// 🧭️ Permanent X/Y/Z paints — primary / secondary / tertiary (semio tokens), not muted chrome.
    pub fn spatial_axis_rgba(axis: u8, alpha: f32) -> Rgba {
        match axis {
            0 => Rgba::new(1.0, 0.204, 0.310, alpha),   // primary #ff344f
            1 => Rgba::new(0.204, 0.820, 0.749, alpha), // secondary #34d1bf
            _ => Rgba::new(0.980, 0.584, 0.0, alpha),   // tertiary #fa9500
        }
    }

    /// 🧭️ Mirrors `resolveSceneGizmoViewportPlacement` — bottom-right corner inset matching pane `--spacing-single` chrome.
    pub fn orbit_view_gizmo_placement(viewport: Rect) -> (f32, f32) {
        let chrome_inset = 4.0_f32;
        let gizmo_half_extent = 28.0_f32;
        let preferred = chrome_inset + gizmo_half_extent;
        let max_fit = (viewport.w.min(viewport.h) / 3.0).floor().max(22.0);
        let margin = preferred.min(max_fit);
        (margin, margin)
    }

    /// 🧭️ Screen-space tip used for orbit-view gizmo hover hit-testing and paint.
    pub struct OrbitViewGizmoTip {
        pub screen_x: f32,
        pub screen_y: f32,
        pub depth: f32,
        pub pick_radius: f32,
        pub color: Rgba,
        pub is_corner: bool,
        pub prominent: bool,
    }

    pub fn orbit_view_gizmo_tips(camera: &Camera3d, viewport: Rect) -> Vec<OrbitViewGizmoTip> {
        let (margin_x, margin_y) = orbit_view_gizmo_placement(viewport);
        let origin_x = viewport.x + viewport.w - margin_x;
        let origin_y = viewport.y + viewport.h - margin_y;
        let axis_len = (viewport.w.min(viewport.h) * 0.04).clamp(14.0, 24.0);
        let forward = camera.position.sub_m(camera.target);
        let forward_len = forward.length_m();
        if forward_len < 1e-5 {
            return Vec::new();
        }
        let forward = forward.scale_m(1.0 / forward_len);
        let right = forward.cross_m(camera.up);
        let right_len = right.length_m();
        if right_len < 1e-5 {
            return Vec::new();
        }
        let right = right.scale_m(1.0 / right_len);
        let up = right.cross_m(forward).normalize_m();
        let neutral = Rgba::new(0.62, 0.62, 0.66, 0.9);
        let axes = [
            (Vec3 { x: 1.0, y: 0.0, z: 0.0 }, spatial_axis_rgba(0, 1.0), true),
            (Vec3 { x: -1.0, y: 0.0, z: 0.0 }, spatial_axis_rgba(0, 0.75), false),
            (Vec3 { x: 0.0, y: 1.0, z: 0.0 }, spatial_axis_rgba(1, 1.0), true),
            (Vec3 { x: 0.0, y: -1.0, z: 0.0 }, spatial_axis_rgba(1, 0.75), false),
            (Vec3 { x: 0.0, y: 0.0, z: 1.0 }, spatial_axis_rgba(2, 1.0), true),
            (Vec3 { x: 0.0, y: 0.0, z: -1.0 }, spatial_axis_rgba(2, 0.75), false),
        ];
        let corners = [
            (Vec3 { x: 0.72, y: 0.72, z: 0.72 }, true),
            (Vec3 { x: -0.72, y: 0.72, z: 0.72 }, true),
            (Vec3 { x: 0.72, y: -0.72, z: 0.72 }, true),
            (Vec3 { x: -0.72, y: -0.72, z: 0.72 }, true),
            (Vec3 { x: 0.72, y: 0.72, z: -0.72 }, false),
            (Vec3 { x: -0.72, y: 0.72, z: -0.72 }, false),
            (Vec3 { x: 0.72, y: -0.72, z: -0.72 }, false),
            (Vec3 { x: -0.72, y: -0.72, z: -0.72 }, false),
        ];
        let mut tips: Vec<OrbitViewGizmoTip> = axes
            .into_iter()
            .map(|(axis, color, prominent)| {
                let sx = axis.dot_m(right);
                let sy = -axis.dot_m(up);
                let depth = axis.dot_m(forward);
                let tip_x = origin_x + sx * axis_len;
                let tip_y = origin_y + sy * axis_len;
                let pick_radius = if prominent { 10.0 } else { 7.0 };
                OrbitViewGizmoTip { screen_x: tip_x, screen_y: tip_y, depth, pick_radius, color, is_corner: false, prominent }
            })
            .chain(corners.into_iter().map(|(axis, prominent)| {
                let sx = axis.dot_m(right);
                let sy = -axis.dot_m(up);
                let depth = axis.dot_m(forward);
                let tip_x = origin_x + sx * axis_len;
                let tip_y = origin_y + sy * axis_len;
                let pick_radius = if prominent { 10.0 } else { 7.0 };
                OrbitViewGizmoTip { screen_x: tip_x, screen_y: tip_y, depth, pick_radius, color: neutral, is_corner: true, prominent }
            }))
            .collect();
        tips.push(OrbitViewGizmoTip { screen_x: origin_x, screen_y: origin_y, depth: 0.0, pick_radius: 9.0, color: neutral, is_corner: false, prominent: true });
        tips
    }

    pub fn orbit_view_gizmo_hit_test(x: f32, y: f32, tips: &[OrbitViewGizmoTip]) -> Option<usize> {
        tips.iter()
            .enumerate()
            .filter_map(|(index, tip)| {
                let distance = ((x - tip.screen_x).powi(2) + (y - tip.screen_y).powi(2)).sqrt();
                if distance <= tip.pick_radius + 3.0 {
                    Some((index, distance))
                } else {
                    None
                }
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(index, _)| index)
    }

    
}
//#endregion gizmo
// #endregion draw_types
