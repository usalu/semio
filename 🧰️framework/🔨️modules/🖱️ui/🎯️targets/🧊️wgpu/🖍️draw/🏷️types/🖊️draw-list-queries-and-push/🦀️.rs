//! 🖊️ Draw list queries and push: the screen height and clock, the overlay routing, the active counts, the scissor and clip stacks and the push methods for every primitive. See [`super`].

use super::*;

// #region draw_types
impl DrawList {
    pub fn set_screen_height(&mut self, height: f32) {
        self.screen_h = height;
    }

    /// ⏱️ The frame clock this list's time-varying paints read, in monotonic seconds — stamped once
    /// per frame by `Ui::advance_clock`, the same value the tooltip dwell deadlines run on. `0.0`
    /// means unstamped, which freezes every such paint at the start of its cycle rather than
    /// jittering.
    pub fn set_clock_seconds(&mut self, seconds: f32) {
        if seconds.is_finite() {
            self.clock_seconds = seconds;
        }
    }

    /// ⏰️ This list's frame clock in monotonic seconds.
    pub fn clock_seconds(&self) -> f32 {
        self.clock_seconds
    }

    /// 📐️ The measured surface height this list draws into, in logical pixels — what an
    /// immediate-mode widget pass hands `WidgetContext::viewport_height` so a `Select` popup can
    /// flip and clamp the way React's `window.innerHeight` lets `resolveSelectPlacement` do.
    pub fn screen_height(&self) -> f32 {
        self.screen_h
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

    /// 🪟️ Opens an overlay-routed region — see [`DrawList::overlay_route`]. Nests; every `begin`
    /// must be paired with an [`DrawList::end_overlay_route`].
    pub fn begin_overlay_route(&mut self) {
        self.overlay_route = self.overlay_route.saturating_add(1);
    }

    /// ⏹️ Closes the innermost overlay-routed region. Saturating, so an unbalanced `end` cannot
    /// wrap the counter and silently route the rest of the frame into the overlay bucket.
    pub fn end_overlay_route(&mut self) {
        self.overlay_route = self.overlay_route.saturating_sub(1);
    }

    /// 🧭️ Whether pushes are currently routed into the overlay bucket.
    pub fn overlay_routed(&self) -> bool {
        self.overlay_route > 0
    }

    /// 🪣️ The quad bucket an ordinary `push_*` lands in: the overlay one inside an overlay-routed
    /// region, the layer's own one otherwise.
    fn active_ui_instances(&mut self) -> &mut Vec<UiInstance> {
        let overlay = self.overlay_route > 0;
        let layer = self.active_layer();
        if overlay {
            &mut layer.overlay_ui_instances
        } else {
            &mut layer.ui_instances
        }
    }

    /// 🖍️ [`DrawList::active_ui_instances`] for vector geometry.
    fn active_vector_vertices(&mut self) -> &mut Vec<VectorVertex> {
        let overlay = self.overlay_route > 0;
        let layer = self.active_layer();
        if overlay {
            &mut layer.overlay_vector_vertices
        } else {
            &mut layer.vector_vertices
        }
    }

    fn active_raster_instances(&mut self) -> &mut Vec<(String, UiInstance)> {
        let overlay = self.overlay_route > 0;
        let layer = self.active_layer();
        if overlay {
            &mut layer.overlay_raster_instances
        } else {
            &mut layer.raster_instances
        }
    }

    pub fn clear(&mut self) {
        self.overlay_route = 0;
        self.scene_passes.clear();
        self.layers.clear();
        self.layers.push(DrawLayer::default());
        self.glass_regions.clear();
        self.scissor_stack.clear();
        self.clip_stack.clear();
        self.glass_content_stack.clear();
        self.retained_output = None;
    }

    pub fn push_scissor(&mut self, rect: semio_framework_ui_theme::geometry::Rect) {
        if !self.claim_retained_output(1, size_of::<DrawLayer>()) {
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
        if !self.claim_retained_output(1, size_of::<DrawLayer>()) {
            return;
        }
        self.scissor_stack.pop();
        let parent = self.scissor_stack.last().copied();
        self.layers.push(DrawLayer { scissor: parent, clip: self.clip_stack.last().cloned(), foreground_of: self.active_foreground_of(), ..DrawLayer::default() });
    }

    /// ✂️ Clips subsequent draw content to an exact union of non-overlapping rectangles.
    pub fn begin_silhouette_clip(&mut self, rects: &[semio_framework_ui_theme::geometry::Rect]) {
        if ClipRegion::source_pieces_overlap(rects) {
            let _ = self.claim_retained_output(usize::MAX, usize::MAX);
            return;
        }
        let candidate = ClipRegion::from_disjoint_rects(rects, self.screen_h);
        let Some(items) = candidate.scissors.len().checked_add(2) else {
            let _ = self.claim_retained_output(usize::MAX, usize::MAX);
            return;
        };
        let Some(bytes) = candidate.scissors.len().checked_mul(size_of::<ScissorRect>()).and_then(|bytes| bytes.checked_add(size_of::<DrawLayer>())) else {
            let _ = self.claim_retained_output(usize::MAX, usize::MAX);
            return;
        };
        if !self.claim_retained_output(items, bytes) {
            return;
        }
        let mut clip = candidate;
        if let Some(parent) = self.clip_stack.last() {
            clip = parent.intersect(&clip);
        }
        self.clip_stack.push(clip.clone());
        self.layers.push(DrawLayer { scissor: self.scissor_stack.last().copied(), clip: Some(clip), foreground_of: self.active_foreground_of(), ..DrawLayer::default() });
    }

    pub fn end_silhouette_clip(&mut self) {
        if !self.claim_retained_output(1, size_of::<DrawLayer>()) {
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
        let Some(items) = items.checked_add(1) else {
            let _ = self.claim_retained_output(usize::MAX, usize::MAX);
            return;
        };
        let Some(bytes) = bytes.checked_add(size_of::<DrawLayer>()) else {
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
        let following = DrawLayer { scissor: layer.scissor, clip: layer.clip.clone(), foreground_of: layer.foreground_of, ..DrawLayer::default() };
        pass.layer_index = layer_index;
        pass.ui_watermark = layer.ui_instances.len();
        pass.vector_watermark = layer.vector_vertices.len();
        self.scene_passes.push(pass);
        self.layers.push(following);
    }

    pub fn push_solid(&mut self, rect: [f32; 4], color: Rgba) {
        if !self.claim_retained_output(1, size_of::<UiInstance>()) {
            return;
        }
        self.active_ui_instances().push(UiInstance::solid(rect, color));
    }

    pub fn push_rounded(&mut self, rect: [f32; 4], color: Rgba, radius: f32) {
        if !self.claim_retained_output(1, size_of::<UiInstance>()) {
            return;
        }
        self.active_ui_instances().push(UiInstance::rounded(rect, color, radius, 0.0, color));
    }

    /// 🌀️ Clockwise spinning + pulsing loading ring around `rect`, in `color` (gray `theme.border_normal` at rest, `theme.selected` when the node is selected/active).
    pub fn push_loading_border(&mut self, rect: [f32; 4], color: Rgba, radius: f32, stroke: f32) {
        if !self.claim_retained_output(1, size_of::<UiInstance>()) {
            return;
        }
        self.active_ui_instances().push(UiInstance::loading_border(rect, color, radius, stroke));
    }

    /// ⌛️ Dashed, slow-spinning + gently pulsing waiting ring around `rect`, in `color` (gray `theme.border_normal` at rest, `theme.selected` when the node is selected/active).
    pub fn push_waiting_border(&mut self, rect: [f32; 4], color: Rgba, radius: f32, stroke: f32) {
        if !self.claim_retained_output(1, size_of::<UiInstance>()) {
            return;
        }
        self.active_ui_instances().push(UiInstance::waiting_border(rect, color, radius, stroke));
    }

    /// ✅️ Solid, static at-bounds ring around `rect`, in `color` — `UiStatus::Finished`.
    pub fn push_finished_border(&mut self, rect: [f32; 4], color: Rgba, radius: f32, stroke: f32) {
        if !self.claim_retained_output(1, size_of::<UiInstance>()) {
            return;
        }
        self.active_ui_instances().push(UiInstance::finished_border(rect, color, radius, stroke));
    }

    /// 💫️ Raised-cosine breathing pulse ring around `rect`, in `color` — `UiState::Introducing`.
    pub fn push_introducing_border(&mut self, rect: [f32; 4], color: Rgba, radius: f32, stroke: f32) {
        if !self.claim_retained_output(1, size_of::<UiInstance>()) {
            return;
        }
        self.active_ui_instances().push(UiInstance::introducing_border(rect, color, radius, stroke));
    }

    /// 🧊️ Pushes a glass region rendered with an already-resolved `style` — callers derive `style`
    /// from `Theme::glass(level)` themselves (see
    /// `.🧬semio/🦑️repo/🎫️tickets/26/07/27/UNIFIED-6-LEVEL-UI-SURFACE-SYSTEM/contract.txt`) rather than this method
    /// picking a per-tier lookup.
    pub fn push_glass(&mut self, rect: [f32; 4], radius: f32, style: GlassStyle) -> usize {
        if !self.claim_retained_output(2, size_of::<GlassRegion>() + size_of::<DrawLayer>()) {
            return usize::MAX;
        }
        let index = self.glass_regions.len();
        let layer_index = self.layers.len();
        self.layers.push(DrawLayer { scissor: self.scissor_stack.last().copied(), clip: self.clip_stack.last().cloned(), foreground_of: self.active_foreground_of(), ..DrawLayer::default() });
        self.glass_regions.push(GlassRegion { layer_index, rect, radius, tint: style.tint, alpha: style.alpha, blur_px: style.blur_px, saturate: style.saturate });
        index
    }

    pub fn begin_glass_content(&mut self, region: usize) {
        if !self.claim_retained_output(1, size_of::<DrawLayer>()) {
            return;
        }
        self.glass_content_stack.push(region);
        self.layers.push(DrawLayer { scissor: self.scissor_stack.last().copied(), clip: self.clip_stack.last().cloned(), foreground_of: Some(region), ..DrawLayer::default() });
    }

    pub fn end_glass_content(&mut self) {
        if !self.claim_retained_output(1, size_of::<DrawLayer>()) {
            return;
        }
        self.glass_content_stack.pop();
        self.layers.push(DrawLayer { scissor: self.scissor_stack.last().copied(), clip: self.clip_stack.last().cloned(), foreground_of: self.active_foreground_of(), ..DrawLayer::default() });
    }

    pub fn push_glyph(&mut self, rect: [f32; 4], color: Rgba, uv_rect: [f32; 4]) {
        if !self.claim_retained_output(1, size_of::<UiInstance>()) {
            return;
        }
        self.active_ui_instances().push(UiInstance::glyph(rect, color, uv_rect));
    }

    /// ✒️ Emits one transformed glyph under the current draw-layer ownership.
    pub fn push_glyph_affine(&mut self,rect:[f32;4],color:Rgba,uv_rect:[f32;4],matrix:[f32;6]) {
        if self.claim_retained_output(1,size_of::<UiInstance>()) {
            self.active_ui_instances().push(UiInstance::glyph_affine(rect,color,uv_rect,matrix));
        }
    }

    pub fn push_glyph_overlay(&mut self, rect: [f32; 4], color: Rgba, uv_rect: [f32; 4]) {
        if !self.claim_retained_output(1, size_of::<UiInstance>()) {
            return;
        }
        self.active_layer().overlay_ui_instances.push(UiInstance::glyph(rect, color, uv_rect));
    }

    pub fn push_solid_overlay(&mut self, rect: [f32; 4], color: Rgba) {
        if !self.claim_retained_output(1, size_of::<UiInstance>()) {
            return;
        }
        self.active_layer().overlay_ui_instances.push(UiInstance::solid(rect, color));
    }

    /// 🔵️ Paints a rounded overlay with the same extent and radius contract as the main UI lane.
    pub fn push_rounded_overlay(&mut self, rect: [f32; 4], color: Rgba, radius: f32) {
        if !self.claim_retained_output(1, size_of::<UiInstance>()) {
            return;
        }
        self.active_layer().overlay_ui_instances.push(UiInstance::rounded(rect, color, radius, 0.0, color));
    }

    pub fn push_textured(&mut self, rect: [f32; 4], uv_rect: [f32; 4], color: Rgba) {
        if !self.claim_retained_output(1, size_of::<UiInstance>()) {
            return;
        }
        self.active_ui_instances().push(UiInstance::textured(rect, uv_rect, color));
    }

    pub fn push_raster_quad(&mut self, key: &str, rect: [f32; 4], uv_rect: [f32; 4], alpha: f32) {
        self.push_rounded_raster_quad(key, rect, uv_rect, alpha, 0.0);
    }

    /// ↗️ Emits a sampled image quad with its complete authored affine transform.
    pub fn push_raster_quad_affine(&mut self,key:&str,rect:[f32;4],uv_rect:[f32;4],alpha:f32,matrix:[f32;6]) {
        let Some(bytes)=key.len().checked_add(size_of::<UiInstance>()) else {
            let _=self.claim_retained_output(usize::MAX,usize::MAX);
            return;
        };
        if self.claim_retained_output(1,bytes) {
            self.active_raster_instances().push((key.to_owned(),UiInstance::raster_affine(rect,uv_rect,alpha,matrix)));
        }
    }

    /// 📻️ An image quad whose sampled pixels are clipped to its rounded CSS-style boundary.
    pub fn push_rounded_raster_quad(&mut self, key: &str, rect: [f32; 4], uv_rect: [f32; 4], alpha: f32, radius: f32) {
        let Some(bytes) = key.len().checked_add(size_of::<UiInstance>()) else {
            let _ = self.claim_retained_output(usize::MAX, usize::MAX);
            return;
        };
        if !self.claim_retained_output(1, bytes) {
            return;
        }
        let mut instance = UiInstance::raster(rect, uv_rect, alpha);
        instance.params[0] = radius.max(0.0).min(rect[2].min(rect[3]).max(0.0) * 0.5);
        self.active_raster_instances().push((key.to_string(), instance));
    }

    pub fn push_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgba, width: f32) {
        if !self.claim_retained_output(1, 6 * size_of::<VectorVertex>()) {
            return;
        }
        let dx = x1 - x0;
        let dy = y1 - y0;
        let len = (dx * dx + dy * dy).sqrt().max(0.001);
        let nx = -dy / len * width * 0.5;
        let ny = dx / len * width * 0.5;
        let c = [color.r, color.g, color.b, color.a];
        let layer = self.active_vector_vertices();
        layer.extend_from_slice(&[
            VectorVertex { position: [x0 + nx, y0 + ny], color: c },
            VectorVertex { position: [x1 + nx, y1 + ny], color: c },
            VectorVertex { position: [x0 - nx, y0 - ny], color: c },
            VectorVertex { position: [x1 + nx, y1 + ny], color: c },
            VectorVertex { position: [x1 - nx, y1 - ny], color: c },
            VectorVertex { position: [x0 - nx, y0 - ny], color: c },
        ]);
    }

    pub fn push_line_overlay(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgba, width: f32) {
        if !self.claim_retained_output(1, 6 * size_of::<VectorVertex>()) {
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
        let Some(bytes) = vertices.checked_mul(size_of::<VectorVertex>()) else {
            let _ = self.claim_retained_output(usize::MAX, usize::MAX);
            return;
        };
        if !self.claim_retained_output(1, bytes) {
            return;
        }
        let c = [color.r, color.g, color.b, color.a];
        let layer = self.active_vector_vertices();
        for tri in 1..points.len() - 1 {
            layer.push(VectorVertex { position: points[0], color: c });
            layer.push(VectorVertex { position: points[tri], color: c });
            layer.push(VectorVertex { position: points[tri + 1], color: c });
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
        let Some(bytes) = vertices.checked_mul(size_of::<VectorVertex>()) else {
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
// #endregion draw_types
