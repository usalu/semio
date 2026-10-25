//! 🏗️ Scene builder: the scene layer and the `SceneBuilder` with its lifecycle, scissor, silhouette clip, surface, quad, vector and glass recording. See [`super`].

use super::*;

//#region 🔖️Scene
//#region 🔖️SceneBuilder
/// 🗂️ One draw layer: a scissor/clip/glass-membership state plus the primitives painted while that
/// state was active. `foreground_of` is `Some(region_index)` while inside `begin_glass_content(region)
/// .. end_glass_content()`, `None` for backdrop content. Ported from the wgpu target's `DrawLayer`.
#[derive(Default)]
pub struct SceneLayer {
    pub scissor: Option<ScissorRect>,
    pub clip: Option<ClipRegion>,
    pub foreground_of: Option<usize>,
    pub quad_instances: Vec<QuadInstance>,
    pub raster_instances: Vec<(TextureId, QuadInstance)>,
    pub vector_vertices: Vec<VectorVertex>,
    pub overlay_quad_instances: Vec<QuadInstance>,
    pub overlay_vector_vertices: Vec<VectorVertex>,
}

#[cfg(any(test, feature = "backend-testing"))]
impl SceneLayer {
    pub(super) fn is_empty(&self) -> bool {
        self.quad_instances.is_empty() && self.raster_instances.is_empty() && self.vector_vertices.is_empty() && self.overlay_quad_instances.is_empty() && self.overlay_vector_vertices.is_empty()
    }

    pub(super) fn merge_from(&mut self, mut other: Self) {
        self.quad_instances.append(&mut other.quad_instances);
        self.raster_instances.append(&mut other.raster_instances);
        self.vector_vertices.append(&mut other.vector_vertices);
        self.overlay_quad_instances.append(&mut other.overlay_quad_instances);
        self.overlay_vector_vertices.append(&mut other.overlay_vector_vertices);
    }
}

/// 🖊️ The pure-data display-list builder: stacked scissors, stacked silhouette clips, glass regions
/// and their foreground content, plus an overlay stream per layer. Ported from the wgpu target's
/// `DrawList` — same push/pop semantics, same layer-splitting-on-state-change behavior — with every
/// `wgpu::` type gone and every method a plain sync `fn`.
pub struct SceneBuilder {
    pub scene_passes: Vec<SurfacePass>,
    pub layers: Vec<SceneLayer>,
    pub glass_regions: Vec<GlassRegion>,
    pub(super) scissor_stack: Vec<ScissorRect>,
    pub(super) clip_stack: Vec<ClipRegion>,
    pub(super) glass_content_stack: Vec<usize>,
}

impl Default for SceneBuilder {
    fn default() -> Self {
        Self { scene_passes: Vec::new(), layers: vec![SceneLayer::default()], glass_regions: Vec::new(), scissor_stack: Vec::new(), clip_stack: Vec::new(), glass_content_stack: Vec::new() }
    }
}

impl SceneBuilder {
    fn active_foreground_of(&self) -> Option<usize> {
        self.glass_content_stack.last().copied()
    }

    fn active_layer(&mut self) -> &mut SceneLayer {
        if self.layers.is_empty() {
            self.layers.push(SceneLayer::default());
        }
        self.layers.last_mut().expect("layers is never empty past the guard above")
    }

    //#region Lifecycle

    pub fn clear(&mut self) {
        self.scene_passes.clear();
        self.layers.clear();
        self.layers.push(SceneLayer::default());
        self.glass_regions.clear();
        self.scissor_stack.clear();
        self.clip_stack.clear();
        self.glass_content_stack.clear();
    }

    //#endregion Lifecycle

    //#region Scissor

    pub fn push_scissor(&mut self, rect: LayoutRect) {
        let mut scissor = ScissorRect::from_rect(rect);
        if let Some(parent) = self.scissor_stack.last() {
            scissor = parent.intersect(&scissor);
        }
        self.scissor_stack.push(scissor);
        self.layers.push(SceneLayer { scissor: Some(scissor), clip: self.clip_stack.last().cloned(), foreground_of: self.active_foreground_of(), ..SceneLayer::default() });
    }

    pub fn pop_scissor(&mut self) {
        self.scissor_stack.pop();
        let parent = self.scissor_stack.last().copied();
        self.layers.push(SceneLayer { scissor: parent, clip: self.clip_stack.last().cloned(), foreground_of: self.active_foreground_of(), ..SceneLayer::default() });
    }

    //#endregion Scissor

    //#region SilhouetteClip

    /// 🪟️ Clips subsequent draw content to an exact union of non-overlapping rectangles.
    pub fn begin_silhouette_clip(&mut self, rects: &[LayoutRect]) {
        let mut clip = ClipRegion::from_rects(rects);
        if let Some(parent) = self.clip_stack.last() {
            clip = parent.intersect(&clip);
        }
        self.clip_stack.push(clip.clone());
        self.layers.push(SceneLayer { scissor: self.scissor_stack.last().copied(), clip: Some(clip), foreground_of: self.active_foreground_of(), ..SceneLayer::default() });
    }

    pub fn end_silhouette_clip(&mut self) {
        self.clip_stack.pop();
        self.layers.push(SceneLayer { scissor: self.scissor_stack.last().copied(), clip: self.clip_stack.last().cloned(), foreground_of: self.active_foreground_of(), ..SceneLayer::default() });
    }

    //#endregion SilhouetteClip

    //#region Surface

    pub fn push_scene_pass(&mut self, mut pass: SurfacePass) {
        if self.layers.is_empty() {
            self.layers.push(SceneLayer::default());
        }
        let layer_index = self.layers.len() - 1;
        let layer = &self.layers[layer_index];
        pass.layer_index = layer_index;
        pass.quad_watermark = layer.quad_instances.len();
        pass.vector_watermark = layer.vector_vertices.len();
        self.scene_passes.push(pass);
    }

    //#endregion Surface

    //#region Quad

    pub fn push_solid(&mut self, rect: [f32; 4], color: [f32; 4]) {
        self.active_layer().quad_instances.push(QuadInstance::solid(rect, color));
    }

    pub fn push_rounded(&mut self, rect: [f32; 4], color: [f32; 4], radius: f32) {
        self.active_layer().quad_instances.push(QuadInstance::rounded(rect, color, radius, 0.0, color));
    }

    pub fn push_loading_border(&mut self, rect: [f32; 4], color: [f32; 4], radius: f32, stroke: f32) {
        self.active_layer().quad_instances.push(QuadInstance::loading_border(rect, color, radius, stroke));
    }

    pub fn push_waiting_border(&mut self, rect: [f32; 4], color: [f32; 4], radius: f32, stroke: f32) {
        self.active_layer().quad_instances.push(QuadInstance::waiting_border(rect, color, radius, stroke));
    }

    pub fn push_finished_border(&mut self, rect: [f32; 4], color: [f32; 4], radius: f32, stroke: f32) {
        self.active_layer().quad_instances.push(QuadInstance::finished_border(rect, color, radius, stroke));
    }

    pub fn push_introducing_border(&mut self, rect: [f32; 4], color: [f32; 4], radius: f32, stroke: f32) {
        self.active_layer().quad_instances.push(QuadInstance::introducing_border(rect, color, radius, stroke));
    }

    pub fn push_glyph(&mut self, rect: [f32; 4], color: [f32; 4], uv_rect: [f32; 4]) {
        self.active_layer().quad_instances.push(QuadInstance::glyph(rect, color, uv_rect));
    }

    pub fn push_glyph_overlay(&mut self, rect: [f32; 4], color: [f32; 4], uv_rect: [f32; 4]) {
        self.active_layer().overlay_quad_instances.push(QuadInstance::glyph(rect, color, uv_rect));
    }

    pub fn push_solid_overlay(&mut self, rect: [f32; 4], color: [f32; 4]) {
        self.active_layer().overlay_quad_instances.push(QuadInstance::solid(rect, color));
    }

    pub fn push_textured(&mut self, rect: [f32; 4], uv_rect: [f32; 4], color: [f32; 4]) {
        self.active_layer().quad_instances.push(QuadInstance::textured(rect, uv_rect, color));
    }

    /// 🖼️ Interns `key` into the resource registry (never re-allocating an id for a key seen before)
    /// and appends the instance tagged with that [`TextureId`] — replacing the wgpu target's
    /// `Vec<(String, UiInstance)>`, which cloned a `String` per instance per frame.
    pub fn push_raster_quad(&mut self, resources: &mut ResourceRegistry, key: &str, rect: [f32; 4], uv_rect: [f32; 4], alpha: f32) {
        let texture = resources.intern_texture(key);
        self.active_layer().raster_instances.push((texture, QuadInstance::raster(rect, uv_rect, alpha)));
    }

    //#endregion Quad

    //#region Vector

    pub fn push_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: [f32; 4], width: f32) {
        let layer = self.active_layer();
        for position in tessellate::thick_line_positions(x0, y0, x1, y1, width) {
            layer.vector_vertices.push(VectorVertex { position, color });
        }
    }

    pub fn push_line_overlay(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: [f32; 4], width: f32) {
        let layer = self.active_layer();
        for position in tessellate::thick_line_positions(x0, y0, x1, y1, width) {
            layer.overlay_vector_vertices.push(VectorVertex { position, color });
        }
    }

    pub fn push_triangle_fan(&mut self, points: &[[f32; 2]], color: [f32; 4]) {
        let layer = self.active_layer();
        for triangle in tessellate::triangle_fan_positions(points) {
            layer.vector_vertices.extend(triangle.map(|position| VectorVertex { position, color }));
        }
    }

    pub fn push_triangle_fan_overlay(&mut self, points: &[[f32; 2]], color: [f32; 4]) {
        let layer = self.active_layer();
        for triangle in tessellate::triangle_fan_positions(points) {
            layer.overlay_vector_vertices.extend(triangle.map(|position| VectorVertex { position, color }));
        }
    }

    /// 〰️ The wgpu target's `push_dashed_line` called `self.push_line(...)` — an `async fn` — as a
    /// bare statement with no `.await`; the constructed `Future` was immediately dropped and the dash
    /// silently drew nothing. Both `push_line` and this function are plain sync `fn` here, so that
    /// call site cannot exist — see `tessellate::dashed_line_segments`'s doc comment.
    pub fn push_dashed_line(&mut self, start: [f32; 2], end: [f32; 2], color: [f32; 4], width: f32, dash: f32, gap: f32) {
        for (sx0, sy0, sx1, sy1) in tessellate::dashed_line_segments(start[0], start[1], end[0], end[1], dash, gap) {
            self.push_line(sx0, sy0, sx1, sy1, color, width);
        }
    }

    /// ➰️ Overlay counterpart of [`Self::push_dashed_line`]; the wgpu target's overlay variant had the
    /// same dropped-future bug against `push_line_overlay`, fixed the same way.
    pub fn push_dashed_line_overlay(&mut self, start: [f32; 2], end: [f32; 2], color: [f32; 4], width: f32, dash: f32, gap: f32) {
        for (sx0, sy0, sx1, sy1) in tessellate::dashed_line_segments(start[0], start[1], end[0], end[1], dash, gap) {
            self.push_line_overlay(sx0, sy0, sx1, sy1, color, width);
        }
    }

    //#endregion Vector

    //#region Glass

    /// 🧊️ Appends a glass region rendered with an already-resolved `style` — callers derive `style`
    /// from their own theme/level lookup rather than this method picking one.
    pub fn push_glass(&mut self, rect: [f32; 4], radius: f32, style: GlassStyle) -> usize {
        let index = self.glass_regions.len();
        self.glass_regions.push(GlassRegion { rect, radius, tint: style.tint, alpha: style.alpha, blur_px: style.blur_px, saturate: style.saturate });
        index
    }

    pub fn begin_glass_content(&mut self, region: usize) {
        self.glass_content_stack.push(region);
        self.layers.push(SceneLayer { scissor: self.scissor_stack.last().copied(), clip: self.clip_stack.last().cloned(), foreground_of: Some(region), ..SceneLayer::default() });
    }

    pub fn end_glass_content(&mut self) {
        self.glass_content_stack.pop();
        self.layers.push(SceneLayer { scissor: self.scissor_stack.last().copied(), clip: self.clip_stack.last().cloned(), foreground_of: self.active_foreground_of(), ..SceneLayer::default() });
    }

    //#endregion Glass
}
//#endregion 🔖️SceneBuilder
//#endregion 🔖️Scene
