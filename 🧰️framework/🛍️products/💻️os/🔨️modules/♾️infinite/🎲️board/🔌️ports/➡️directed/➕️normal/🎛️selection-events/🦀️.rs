//! 🎛️ Board host brush sessions, event authority, selection commits and hit and pick resolution.

use super::*;

    // #region board_host
    //#region 🧵️FillJob
impl BoardHost {
    //#endregion 🧵️FillJob

    fn brush_preview_snapshot_from_session_json(node: &serde_json::Value, edge: &serde_json::Value, source_handle_id: &str) -> Option<BrushPreviewSnapshot> {
        let node_kind_id = node.get("nodeKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty())?;
        let x = node.get("x").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
        let y = node.get("y").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
        let shape = match node.get("shape").and_then(|x| x.as_str()).map(str::trim) {
            Some("rectangle") => NodeShape::Rectangle,
            _ => NodeShape::Circle,
        };
        let (radius, width, height) = match shape {
            NodeShape::Circle => (node.get("radius").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?, 0.0, 0.0),
            NodeShape::Rectangle => {
                let w = node.get("width").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
                let h = node.get("height").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
                (0.0, w, h)
            }
        };
        let target_handle_index = edge.get("targetHandleIndex").and_then(|v| v.as_u64()).map(|v| v as usize)?;
        let mut handles: Vec<NodeKindHandleTemplate> = Vec::new();
        if let Some(arr) = node.get("handles").and_then(|x| x.as_array()) {
            for row in arr {
                let ho = row.as_object()?;
                let handle_kind = ho.get("handleKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty())?;
                let angle = ho.get("angle").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
                let radius = ho.get("radius").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0);
                handles.push(NodeKindHandleTemplate { handle_kind: handle_kind.to_string(), angle, radius });
            }
        }
        if handles.is_empty() {
            return None;
        }
        let icon_kind = node.get("iconKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
        Some(BrushPreviewSnapshot { source_handle_id: source_handle_id.to_string(), node_kind_id: node_kind_id.to_string(), x, y, shape, radius, width, height, handles, target_handle_index, icon_kind })
    }

    /// 🖌️ Mirrors brush slot + preview from another authoring pane (no pointer input on this host).
    pub fn set_brush_session_mirror_json(&mut self, json: &str) -> Result<(), NormalPortError> {
        if json.trim().is_empty() {
            self.brush_slot_suggestions_active = false;
            self.brush_slot_source_id = None;
            self.brush_candidates.clear();
            self.brush_candidate_index = 0;
            self.brush_preview = None;
            self.brush_preview_emit_key = None;
            self.brush_candidates_emit_key = None;
            self.bump_content_scene_generation();
            return Ok(());
        }
        if json.len() > BOARD_EVENT_BYTE_CAPACITY {
            return Err(NormalPortError::EventCredits);
        }
        let v: serde_json::Value = serde_json::from_str(json).map_err(NormalPortError::BrushSessionJson)?;
        let source = v.get("sourceHandleId").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
        self.brush_slot_source_id = source.clone();
        let mut candidates = BrushCandidatePage::default();
        if let Some(rows) = v.get("candidates").and_then(|x| x.as_array()) {
            if rows.len() > BOARD_POINTER_ITEM_CAPACITY {
                return Err(NormalPortError::EventCredits);
            }
            for row in rows {
                let (node_kind_id, target_handle_index) = if let Some(node_kind_id) = row.as_str().map(str::trim).filter(|value| !value.is_empty()) {
                    (node_kind_id, 0)
                } else {
                    let node_kind_id = row.get("nodeKind").or_else(|| row.get("nodeKindId")).and_then(|value| value.as_str()).map(str::trim).filter(|value| !value.is_empty()).ok_or(NormalPortError::EventCredits)?;
                    let target_handle_index = row.get("targetHandleIndex").and_then(|value| value.as_u64()).unwrap_or(0);
                    let target_handle_index = usize::try_from(target_handle_index).map_err(|_| NormalPortError::EventCredits)?;
                    (node_kind_id, target_handle_index)
                };
                candidates.push(node_kind_id, target_handle_index, 0.0).map_err(|_| NormalPortError::EventCredits)?;
            }
        }
        self.brush_candidates = candidates;
        self.brush_candidate_index = v.get("index").and_then(|x| x.as_u64()).map_or(0, |i| i as usize);
        if self.brush_candidates.is_empty() {
            self.brush_candidate_index = 0;
        } else {
            self.brush_candidate_index %= self.brush_candidates.len();
        }
        self.brush_preview = match (source.as_deref(), v.get("preview")) {
            (Some(source_id), Some(preview)) if !preview.is_null() => {
                let node = preview.get("node").filter(|n| !n.is_null());
                let edge = preview.get("edge").filter(|e| !e.is_null());
                match (node, edge) {
                    (Some(node), Some(edge)) => Self::brush_preview_snapshot_from_session_json(node, edge, source_id),
                    _ => None,
                }
            }
            _ => None,
        };
        self.brush_preview_emit_key = None;
        self.brush_candidates_emit_key = None;
        self.brush_slot_suggestions_active = v.get("suggestionsActive").and_then(|x| x.as_bool()).unwrap_or(false);
        if self.brush_preview.is_none() && !self.brush_candidates.is_empty() {
            self.brush_rebuild_preview();
        } else {
            self.brush_sync_preview_events();
        }
        self.bump_content_scene_generation();
        Ok(())
    }

    /// 🔗️ A handle that already carries an edge is not open, so it grows nothing. The pointer
    /// path filters those out before it ever enters a slot (`brush_nearest_slot_source`); the
    /// direct entry points (`brush_open_slot`, `brush_target_slot`) reach this instead, and the
    /// slot must then resolve an EMPTY page rather than a wrong one — that empty page is what a
    /// fully fastened document's suggestions popup reads as "no placement available".
    fn brush_enter_slot(&mut self, source_handle_id: &str) {
        if self.brush_slot_source_id.as_deref() == Some(source_handle_id) {
            return;
        }
        if self.brush_slot_source_id.is_some() {
            self.brush_finish_slot();
        }
        self.brush_slot_source_id = Some(source_handle_id.to_string());
        let fastened = self.handle_has_incident_edge(source_handle_id);
        let Some(source) = self.handles.get(source_handle_id).filter(|_| !fastened).cloned() else {
            self.brush_candidates.clear();
            self.brush_candidate_index = 0;
            self.brush_rebuild_preview();
            return;
        };
        let Some(candidates) = self.brush_compatible_candidates(&source) else {
            self.event_schema_fault = true;
            return;
        };
        self.brush_candidates = candidates;
        self.brush_candidate_index = 0;
        self.brush_rebuild_preview();
    }

    pub(super) fn brush_rebuild_preview(&mut self) {
        let Some(ref source_id) = self.brush_slot_source_id else {
            self.brush_preview = None;
            self.brush_sync_preview_events();
            return;
        };
        let preview = self.brush_candidates.get(self.brush_candidate_index).and_then(|candidate| self.brush_build_preview(source_id, candidate));
        self.brush_preview = preview;
        if self.brush_preview.is_some() {
            self.bump_content_scene_generation();
        }
        self.brush_preview_emit_key = None;
        self.brush_candidates_emit_key = None;
        self.brush_sync_preview_events();
    }

    pub(super) fn brush_pointer_move(&mut self, world: Point) {
        if let Some(slot) = self.brush_nearest_slot_source(world) {
            self.brush_enter_slot(&slot);
            self.set_hovered_id(self.brush_slot_source_id.clone());
        } else if self.brush_slot_source_id.is_some() {
            self.brush_finish_slot();
            self.set_hovered_id(None);
        }
    }

    pub fn set_active_utility(&mut self, label: &str) {
        let next = match label {
            "brush" => ActiveUtility::Brush,
            "areaBrush" => ActiveUtility::AreaBrush,
            _ => ActiveUtility::Select,
        };
        if self.active_utility == next {
            return;
        }
        if self.active_utility == ActiveUtility::Brush {
            self.brush_finish_slot();
        }
        self.cancel_region_paint();
        self.cancel_region_drag();
        self.active_utility = next;
        self.interaction = Interaction::None;
        self.bump_content_scene_generation();
    }

    pub fn set_suggestion_offset(&mut self, distance: f64) {
        let d = if distance.is_finite() && distance >= 0.0 { distance } else { DEFAULT_SUGGESTION_OFFSET };
        if (self.suggestion_offset - d).abs() < 1e-9 {
            return;
        }
        self.suggestion_offset = d;
        if self.active_utility == ActiveUtility::Brush {
            self.brush_preview_emit_key = None;
            self.brush_rebuild_preview();
        }
    }

    pub fn set_brush_kind_weights(&mut self, json: &str) {
        if json.is_empty() {
            return;
        }
        let Ok(v) = serde_json::from_str::<serde_json::Value>(json) else {
            return;
        };
        self.brush_node_kind_weights.clear();
        self.brush_handle_kind_weights.clear();
        if let Some(obj) = v.get("nodeWeights").and_then(|x| x.as_object()) {
            for (k, val) in obj {
                if let Some(w) = val.as_f64() {
                    if w.is_finite() && w >= 0.0 {
                        self.brush_node_kind_weights.insert(k.clone(), w);
                    }
                }
            }
        }
        if let Some(obj) = v.get("handleWeights").and_then(|x| x.as_object()) {
            for (k, val) in obj {
                if let Some(w) = val.as_f64() {
                    if w.is_finite() && w >= 0.0 {
                        self.brush_handle_kind_weights.insert(k.clone(), w);
                    }
                }
            }
        }
        if self.active_utility != ActiveUtility::Brush {
            return;
        }
        if let Some(source) = self.brush_slot_source_id.clone() {
            self.brush_slot_source_id = None;
            self.brush_enter_slot(&source);
        } else {
            self.brush_preview_emit_key = None;
            self.brush_rebuild_preview();
        }
    }

    pub fn set_brush_node_size(&mut self, size: f64) {
        let s = if size.is_finite() && size > 0.0 { size } else { DEFAULT_BRUSH_NODE_SIZE };
        if (self.brush_node_size - s).abs() < 1e-9 {
            return;
        }
        self.brush_node_size = s;
        if self.active_utility == ActiveUtility::Brush {
            self.brush_preview_emit_key = None;
            self.brush_rebuild_preview();
        }
    }

    pub fn brush_cycle_candidate(&mut self, forward: bool) {
        if self.brush_candidates.len() < 2 {
            return;
        }
        let len = self.brush_candidates.len();
        self.brush_candidate_index = if forward { (self.brush_candidate_index + 1) % len } else { (self.brush_candidate_index + len - 1) % len };
        self.brush_rebuild_preview();
    }

    pub fn brush_set_candidate_index(&mut self, index: usize) {
        if self.brush_candidates.is_empty() {
            return;
        }
        self.brush_candidate_index = index % self.brush_candidates.len();
        self.brush_rebuild_preview();
    }

    /// 🖌️ Opens a brush slot on a free handle (suggestions menu; works outside brush utility).
    pub fn brush_open_slot(&mut self, handle_id: &str) {
        if !self.handles.contains_key(handle_id) {
            return;
        }
        self.brush_enter_slot(handle_id);
        self.brush_slot_suggestions_active = true;
        self.brush_rebuild_preview();
        self.set_hovered_id(Some(handle_id.to_string()));
    }

    /// 🎣️ Points the brush slot at a handle, or at nothing, WITHOUT opening the suggestions
    /// popup — the armed brush's own targeting channel, so a program driving the brush headlessly
    /// resolves the same candidate page a pointer hover would without claiming the popup's hover.
    pub fn brush_target_slot(&mut self, handle_id: Option<&str>) {
        match handle_id.filter(|handle_id| self.handles.contains_key(*handle_id)) {
            Some(handle_id) => {
                self.brush_enter_slot(handle_id);
                self.brush_rebuild_preview();
            }
            None => self.brush_clear_slot(),
        }
    }

    /// 🖌️ Commits the active brush preview and clears the slot.
    pub fn brush_commit_slot(&mut self) {
        self.brush_commit_preview();
        self.brush_clear_slot();
    }

    /// 🖌️ Discards the active brush slot without placing.
    pub fn brush_cancel_slot(&mut self) {
        self.brush_clear_slot();
    }

    fn append_brush_node_icon_paint(&self, scene: &mut Scene, lod: BoardDrawLod, center: Point, shape: NodeShape, radius: f64, width: f64, height: f64, icon_kind: &str, world_space: bool) {
        if !matches!(lod, BoardDrawLod::Detail | BoardDrawLod::Micro) {
            return;
        }
        let preserve_original_style = false;
        let (icon_fg, icon_bg) = IconPaintCache::board_icon_paint_colors(&self.canvas_theme);
        let Some(paint) = self.get_or_build_icon_paint(icon_kind, icon_fg, icon_bg, preserve_original_style) else {
            return;
        };
        let (bx, by, bw, bh) = paint.bounds();
        let clip_inset = ui_styling::metrics::icon::CLIP_INSET;
        let fit_inset = ui_styling::metrics::icon::FIT_INSET;
        let (sx_half, sy_half) = match shape {
            NodeShape::Circle => {
                let s = self.draw_space_len(radius, world_space) * fit_inset;
                (s, s)
            }
            NodeShape::Rectangle => (self.draw_space_len(width, world_space) * fit_inset * 0.5, self.draw_space_len(height, world_space) * fit_inset * 0.5),
        };
        let center_ds = self.draw_space_point(center, world_space);
        let cx = bx + bw * 0.5;
        let cy = by + bh * 0.5;
        let avail_w = 2.0 * sx_half;
        let avail_h = 2.0 * sy_half;
        let scale = (avail_w / bw).min(avail_h / bh);
        let aff = Affine::IDENTITY.translate((center_ds.x - scale * cx, center_ds.y - scale * cy)) * Affine::IDENTITY.scale(scale);
        match shape {
            NodeShape::Circle => {
                let r_clip = self.draw_space_len(radius, world_space) * clip_inset;
                let disc = Circle::new(center_ds, r_clip);
                scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &disc);
                match paint.body() {
                    CachedIconBody::Vector(icon_scene) => {
                        scene.append(icon_scene, Some(aff));
                    }
                    CachedIconBody::Raster(img) => {
                        scene.draw_image(img, aff);
                    }
                }
                scene.pop_layer();
            }
            NodeShape::Rectangle => {
                let hw = self.draw_space_len(width, world_space) * clip_inset * 0.5;
                let hh = self.draw_space_len(height, world_space) * clip_inset * 0.5;
                let clip_r = Rect::from_points(Point::new(center_ds.x - hw, center_ds.y - hh), Point::new(center_ds.x + hw, center_ds.y + hh));
                scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &clip_r);
                match paint.body() {
                    CachedIconBody::Vector(icon_scene) => {
                        scene.append(icon_scene, Some(aff));
                    }
                    CachedIconBody::Raster(img) => {
                        scene.draw_image(img, aff);
                    }
                }
                scene.pop_layer();
            }
        }
    }

    fn paint_highlighted_node_preview(&self, scene: &mut Scene, _lod: BoardDrawLod, x: f64, y: f64, shape: NodeShape, radius: f64, width: f64, height: f64, icon_kind: Option<&str>, world_space: bool) {
        let center = Point::new(x, y);
        let style = BoardElementStyleKind::Highlighted;
        let fill = Self::node_fill_for_style(&self.canvas_theme, style);
        let stroke_c = Self::node_stroke_for_style(&self.canvas_theme, style);
        let stroke = Stroke::new(ui_styling::strokes::NODE_BODY);
        match shape {
            NodeShape::Circle => {
                let c = self.draw_space_point(center, world_space);
                let r = self.draw_space_len(radius, world_space);
                let circle = Circle::new(c, r);
                scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &circle);
                scene.stroke(&stroke, Affine::IDENTITY, stroke_c, None, &circle);
            }
            NodeShape::Rectangle => {
                let hw = width * 0.5;
                let hh = height * 0.5;
                let p0 = self.draw_space_point(Point::new(x - hw, y - hh), world_space);
                let p1 = self.draw_space_point(Point::new(x + hw, y + hh), world_space);
                let rect = Rect::from_points(p0, p1);
                scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &rect);
                scene.stroke(&stroke, Affine::IDENTITY, stroke_c, None, &rect);
            }
        }
        if let Some(icon) = icon_kind.map(str::trim).filter(|s| !s.is_empty()) {
            self.append_brush_node_icon_paint(scene, BoardDrawLod::Detail, center, shape, radius, width, height, icon, world_space);
        }
    }

    fn drop_preview_effective_dims(&self, preview: &DropPreviewSnapshot) -> (NodeShape, f64, f64, f64) {
        if let Some(kind) = self.node_kinds.get(preview.node_kind_id.as_str()) {
            let radius = self.brush_node_size * 0.5 * kind.scale;
            let (width, height) = if kind.shape == NodeShape::Rectangle { (self.brush_node_size * kind.scale, self.brush_node_size * kind.scale) } else { (radius * 2.0, radius * 2.0) };
            return (kind.shape, radius, width, height);
        }
        (preview.shape, preview.radius, preview.width, preview.height)
    }

    fn drop_preview_from_json(&self, node: &serde_json::Value) -> Option<DropPreviewSnapshot> {
        let node_kind_id = node.get("nodeKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty())?;
        let (x, y) = match (node.get("screenX").and_then(|v| v.as_f64()).filter(|v| v.is_finite()), node.get("screenY").and_then(|v| v.as_f64()).filter(|v| v.is_finite())) {
            (Some(sx), Some(sy)) => {
                let world = self.screen_to_world(Point::new(sx, sy));
                (world.x, world.y)
            }
            _ => {
                let x = node.get("x").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
                let y = node.get("y").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
                (x, y)
            }
        };
        let shape = match node.get("shape").and_then(|x| x.as_str()).map(str::trim) {
            Some("rectangle") => NodeShape::Rectangle,
            _ => NodeShape::Circle,
        };
        let (radius, width, height) = match shape {
            NodeShape::Circle => (node.get("radius").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?, 0.0, 0.0),
            NodeShape::Rectangle => {
                let w = node.get("width").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
                let h = node.get("height").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
                (0.0, w, h)
            }
        };
        let icon_kind = node.get("iconKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
        Some(DropPreviewSnapshot { node_kind_id: node_kind_id.to_string(), x, y, shape, radius, width, height, icon_kind })
    }

    /// 👻️ Sets or clears the workbench palette snapshot drop ghost node (independent of brush utility).
    pub fn set_drop_preview_json(&mut self, json: &str) -> Result<(), NormalPortError> {
        if json.trim().is_empty() {
            self.drop_preview = None;
            self.bump_content_scene_generation();
            return Ok(());
        }
        let v: serde_json::Value = serde_json::from_str(json).map_err(NormalPortError::DropPreviewJson)?;
        self.drop_preview = self.drop_preview_from_json(&v);
        if self.drop_preview.is_none() {
            return Err(NormalPortError::DropPreviewInvalid);
        }
        self.bump_content_scene_generation();
        Ok(())
    }

    pub(super) fn append_drop_preview_paint(&self, scene: &mut Scene, lod: BoardDrawLod, world_space: bool) {
        let Some(ref preview) = self.drop_preview else {
            return;
        };
        let (shape, radius, width, height) = self.drop_preview_effective_dims(preview);
        let icon_kind = preview.icon_kind.as_deref().filter(|s| !s.is_empty()).or_else(|| self.node_kinds.get(preview.node_kind_id.as_str()).and_then(|k| k.icon.as_deref()));
        self.paint_highlighted_node_preview(scene, lod, preview.x, preview.y, shape, radius, width, height, icon_kind, world_space);
    }

    pub(super) fn append_brush_preview_paint(&self, scene: &mut Scene, lod: BoardDrawLod, world_space: bool) {
        let Some(ref preview) = self.brush_preview else {
            return;
        };
        let _ = lod;
        self.paint_highlighted_node_preview(scene, lod, preview.x, preview.y, preview.shape, preview.radius, preview.width, preview.height, preview.icon_kind.as_deref(), world_space);
        let center = Point::new(preview.x, preview.y);
        let source = match self.handles.get(preview.source_handle_id.as_str()) {
            Some(h) => h,
            None => return,
        };
        let (src_pos, src_out, _) = match self.handle_cap_attachment(source) {
            Some(attachment) => attachment,
            None => return,
        };
        let tmpl = match preview.handles.get(preview.target_handle_index) {
            Some(t) => t,
            None => return,
        };
        let tgt_rim = self.brush_template_world_pos(center, preview.shape, preview.radius, preview.width, preview.height, tmpl.angle);
        let tgt_radius = tmpl.radius.filter(|radius| radius.is_finite() && *radius > 0.0).unwrap_or(ui_styling::radii::HANDLE_DEFAULT);
        let Some(tgt_out) = handle_outward_at_node_rim(tgt_rim, center, preview.shape, preview.radius, preview.width, preview.height) else {
            return;
        };
        let tgt_pos = handle_exterior_cap_peak(tgt_rim, tgt_out, tgt_radius);
        let curve = compute_edge_bezier_outward(src_pos, tgt_pos, src_out, tgt_out);
        let p0 = self.draw_space_point(curve.p0(), world_space);
        let p1 = self.draw_space_point(curve.p1(), world_space);
        let p2 = self.draw_space_point(curve.p2(), world_space);
        let p3 = self.draw_space_point(curve.p3(), world_space);
        let bez = CubicBez::new(p0, p1, p2, p3);
        scene.stroke(&Stroke::new(ui_styling::strokes::WIRE_HIGHLIGHT), Affine::IDENTITY, self.canvas_theme.wire_stroke_highlighted, None, &bez);
    }

    /// 🧩️ Selects world-space clip tiling for Vello scene construction (`none` | `world-clip`).
    pub fn set_world_raster_tiling(&mut self, mode: &str) {
        let next = if mode == "world-clip" { "world-clip".into() } else { "none".into() };
        if self.world_raster_tiling == next {
            return;
        }
        self.world_raster_tiling = next;
    }

    pub fn set_original_element_style(&mut self, enabled: bool) {
        if self.preserve_original_element_style == enabled {
            return;
        }
        self.preserve_original_element_style = enabled;
        self.icon_paint_cache.clear();
    }

    pub fn set_selection_screen_preview(&mut self, points: Option<Vec<Point>>) {
        if points.is_none() {
            self.selection_preview_crossing = false;
        }
        self.selection_screen_preview = points;
    }

    pub fn set_canvas_palette(&mut self, overlay: &infinite::board::BoardPaletteOverlay) {
        self.canvas_theme.apply_overlay(overlay);
        self.icon_paint_cache.clear();
    }

    pub(super) fn sync_selection_screen_overlay(&mut self, start_screen: Point, screen_points: &[Point]) {
        if screen_points.len() < 2 {
            self.selection_screen_preview = None;
            self.selection_preview_crossing = false;
            return;
        }
        let last = *screen_points.last().unwrap_or(&start_screen);
        self.selection_preview_crossing = !selection_drag_enclosing(self.selection_options.method.as_str(), start_screen, screen_points);
        self.selection_screen_preview = Some(if self.selection_options.method == "lasso" { screen_points.to_vec() } else { vec![start_screen, Point::new(last.x, start_screen.y), last, Point::new(start_screen.x, last.y)] });
    }

    fn reserve_selection_event(&mut self, kind: BoardEventKind, ids: &[String], anchor_ids: Option<&[String]>, removed_ids: Option<&[String]>, gesture: Option<&str>) -> Option<BoardEventReservation> {
        self.reserve_owned_event(BoardOwnedEvent::selection(kind, ids, anchor_ids, removed_ids, gesture))
    }

    pub(super) fn reserve_owned_event(&mut self, event: Result<BoardOwnedEvent, BoardEventFault>) -> Option<BoardEventReservation> {
        let event = match event {
            Ok(event) => event,
            Err(_) => {
                self.event_schema_fault = true;
                return None;
            }
        };
        match self.events.reserve_event(event) {
            Ok(reservation) => Some(reservation),
            Err(event) => {
                if self.event_overflow.is_none() {
                    self.event_overflow = Some(event);
                } else {
                    self.event_schema_fault = true;
                }
                None
            }
        }
    }

    pub(super) fn reserve_owned_batch(&mut self, first: Result<BoardOwnedEvent, BoardEventFault>, second: Option<Result<BoardOwnedEvent, BoardEventFault>>) -> Option<BoardEventBatchReservation> {
        let first = match first {
            Ok(event) => event,
            Err(_) => {
                self.event_schema_fault = true;
                return None;
            }
        };
        let mut batch = BoardEventBatchReservation::one(first);
        if let Some(second) = second {
            let second = match second {
                Ok(event) => event,
                Err(_) => {
                    self.event_schema_fault = true;
                    return None;
                }
            };
            if batch.push(second).is_err() {
                self.event_schema_fault = true;
                return None;
            }
        }
        match self.events.reserve_batch(batch) {
            Ok(batch) => Some(batch),
            Err(batch) => {
                if self.event_batch_overflow.is_none() {
                    self.event_batch_overflow = Some(batch);
                } else {
                    self.event_schema_fault = true;
                }
                None
            }
        }
    }

    pub(super) fn publish_event_reservation(&mut self, reservation: BoardEventReservation) {
        if self.events.publish_reserved(reservation).is_err() {
            self.event_schema_fault = true;
        }
    }

    pub(super) fn publish_event_batch(&mut self, reservation: BoardEventBatchReservation) {
        if self.events.publish_batch(reservation).is_err() {
            self.event_schema_fault = true;
        }
    }

    //#region 🎬️Gestures
    /// 🪪️ Opens the next pointer gesture: a fresh id, nothing staged yet.
    pub(super) fn begin_gesture(&mut self) -> GestureStage {
        self.gesture_serial = self.gesture_serial.wrapping_add(1);
        GestureStage { id: format!("gesture-{}", self.gesture_serial), select: None, restore: None }
    }

    /// 🕹️ Applies a press's selection change to the engine at once and stages its `select` row on
    /// `gesture`, keeping the pre-press selection for a cancel. An unchanged selection stages nothing.
    pub(super) fn stage_selection_ids(&mut self, gesture: &mut GestureStage, ids: &[String], merge: Option<&str>) {
        let next: BTreeSet<String> = ids.iter().cloned().collect();
        if next == self.selection {
            return;
        }
        let previous = std::mem::replace(&mut self.selection, next);
        gesture.restore.get_or_insert(previous);
        gesture.select = Some((Self::sorted_selection_ids(&self.selection), merge.map(str::to_string)));
        self.preselect.clear();
        self.preselect_removed.clear();
        self.last_preselect_emit_sig = None;
        self.last_select_emit_sig = None;
        self.selection_exit_highlight.clear();
        self.sync_selection_flags_to_objects();
    }

    /// ↩️ Restores the selection a cancelled gesture's press staged; nothing is published.
    pub(super) fn restore_gesture_selection(&mut self, gesture: GestureStage) {
        let Some(restore) = gesture.restore else {
            return;
        };
        self.selection = restore;
        self.selection_exit_highlight.clear();
        self.sync_selection_flags_to_objects();
        self.bump_content_scene_generation();
    }

    /// 🏁️ The rows one released gesture publishes: its staged `select` (tagged with the gesture id when a
    /// record follows) and the record itself — `(None, None)` for a release that changed nothing.
    pub(super) fn gesture_release_events(gesture: &GestureStage, record: Option<Result<BoardOwnedEvent, BoardEventFault>>) -> [Option<Result<BoardOwnedEvent, BoardEventFault>>; 2] {
        let tag = record.is_some().then_some(gesture.id.as_str());
        [gesture.select.as_ref().map(|(ids, merge)| BoardOwnedEvent::staged_select(ids, merge.as_deref(), tag)), record]
    }

    /// 🏁️ Publishes one released gesture as ONE batch, so a host flushes `select` and the record together.
    pub(super) fn publish_gesture_release(&mut self, gesture: &GestureStage, record: Option<Result<BoardOwnedEvent, BoardEventFault>>) {
        let (first, second) = match Self::gesture_release_events(gesture, record) {
            [Some(select), record] => (select, record),
            [None, Some(record)] => (record, None),
            [None, None] => return,
        };
        if let Some(reservation) = self.reserve_owned_batch(first, second) {
            self.publish_event_batch(reservation);
        }
    }

    /// 🎬️ The drag record of `start_positions` moved by `delta`, `None` for a release that never moved.
    pub(super) fn drag_gesture_record(gesture: &GestureStage, start_positions: &BTreeMap<String, (f64, f64)>, delta: Vec2, proximity_pair: Option<&(String, String)>) -> Option<Result<BoardOwnedEvent, BoardEventFault>> {
        if (delta.x == 0.0 && delta.y == 0.0) || start_positions.is_empty() {
            return None;
        }
        Some(BoardOwnedEvent::gesture(&gesture.id, start_positions.keys().map(String::as_str), BoardGestureMotion::Drag { dx: delta.x, dy: delta.y }, proximity_pair.map(std::slice::from_ref).unwrap_or_default()))
    }

    /// 🧲️ The common offset a drag of `start_positions` grabbed at `offset` reaches at `world`, snapped to the grid
    /// when snapping is on — the one rule the live preview, the release record and the wgpu plan all share.
    pub(super) fn drag_delta(&self, world: Point, primary_id: &str, offset: Vec2, start_positions: &BTreeMap<String, (f64, f64)>) -> Option<Vec2> {
        let (px0, py0) = start_positions.get(primary_id).copied()?;
        let (nx, ny) = (world.x - offset.x, world.y - offset.y);
        let (tx, ty) = if self.grid_snap_enabled { self.snap_world_pair(nx, ny) } else { (nx, ny) };
        Some(Vec2::new(tx - px0, ty - py0))
    }
    /// ↩️ Abandons a live node drag with zero trace: every member returns to its grab-time position, the
    /// press's staged selection is undone, and nothing reaches the document. The restored positions go out as
    /// transient `nodeMove` preview rows only, so sibling panes that mirrored the drag snap back too.
    pub(super) fn cancel_node_drag(&mut self, start_positions: BTreeMap<String, (f64, f64)>, gesture: GestureStage) {
        for (id, (x, y)) in &start_positions {
            if let Some(node) = self.nodes.get_mut(id) {
                node.x = *x;
                node.y = *y;
                if let Ok(reservation) = BoardOwnedEvent::node_move(id, *x, *y).map_err(|_| ()).and_then(|event| self.events.reserve_event(event).map_err(|_| ())) {
                    self.publish_event_reservation(reservation);
                }
            }
        }
        self.restore_gesture_selection(gesture);
        self.bump_content_scene_generation();
    }
    //#endregion 🎬️Gestures

    #[cfg(test)]
    pub fn drain_events_json(&mut self) -> String {
        let mut out = String::from("[");
        let mut first = true;
        while let Some(event) = self.events.pop() {
            if !first {
                out.push(',');
            }
            first = false;
            event.write_json(&mut out);
        }
        if let Some(event) = self.event_overflow.take() {
            if !first {
                out.push(',');
            }
            event.write_json(&mut out);
        }
        out.push(']');
        out
    }

    pub fn pop_owned_event(&mut self) -> Option<BoardOwnedEvent> {
        if let Some(event) = self.events.pop().or_else(|| self.event_overflow.take()) {
            return Some(event);
        }
        let batch = self.event_batch_overflow.as_mut()?;
        let event = batch.pop();
        if batch.is_empty() {
            self.event_batch_overflow = None;
        }
        event
    }

    pub fn peek_owned_event(&self) -> Option<&BoardOwnedEvent> {
        self.events.iter().next().or(self.event_overflow.as_ref()).or_else(|| self.event_batch_overflow.as_ref().and_then(BoardEventBatchReservation::peek))
    }

    pub fn event_terminal_faulted(&self) -> bool {
        self.event_schema_fault
    }

    pub fn step_event_authority(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> BoardAuthorityStep {
        if context.should_yield() {
            return BoardAuthorityStep::Pending;
        }
        let faulted_before = self.event_schema_fault;
        if self.pending_delete_planning.is_some() {
            if context.is_cancelled() {
                if let Some(planning) = self.pending_delete_planning.as_mut() {
                    planning.cancelling = true;
                }
            }
            let result = self.step_delete_planning();
            context.consume_fuel(1);
            return result;
        }
        if self.pending_delete_operation.is_some() {
            if context.is_cancelled() {
                if let Some(operation) = self.pending_delete_operation.as_mut() {
                    if !operation.claimed {
                        operation.cancelling = true;
                    }
                }
            }
            if self.pending_delete_operation.as_ref().is_some_and(|operation| operation.cancelling && !operation.claimed) {
                self.pending_delete_operation = None;
                context.consume_fuel(1);
                return BoardAuthorityStep::Cancelled;
            }
            self.step_pending_delete_operation();
            context.consume_fuel(1);
            if self.event_schema_fault && !faulted_before {
                return BoardAuthorityStep::Fault;
            }
            return if self.pending_delete_operation.is_some() { BoardAuthorityStep::Pending } else { BoardAuthorityStep::Complete };
        }
        BoardAuthorityStep::Complete
    }

    fn set_pointer_selection_flag(&mut self, id: &str, selected: bool) {
        if let Some(node) = self.nodes.get_mut(id) {
            node.selected = selected;
        }
        if let Some(handle) = self.handles.get_mut(id) {
            handle.selected = selected;
        }
        if let Some(edge) = self.edges.get_mut(id) {
            edge.selected = selected;
        }
        if let Some(wire) = self.wires.get_mut(id) {
            wire.selected = selected;
        }
    }

    pub(super) fn step_pointer_commit_operation(&mut self, operation: &mut BoardPointerCommitOperation) -> bool {
        match operation.plan.kind {
            BoardPointerPlanKind::Idle => true,
            BoardPointerPlanKind::Pan { camera } => {
                self.camera = Camera { x: camera[0], y: camera[1], zoom: semio_framework_canvas::camera::clamp_zoom(camera[2]) };
                true
            }
            BoardPointerPlanKind::FinishPan { camera } => match operation.phase {
                0 => {
                    self.camera = Camera { x: camera[0], y: camera[1], zoom: semio_framework_canvas::camera::clamp_zoom(camera[2]) };
                    operation.phase = 1;
                    false
                }
                _ => {
                    self.interaction = Interaction::None;
                    true
                }
            },
            BoardPointerPlanKind::DragMove | BoardPointerPlanKind::FinishDrag => {
                if operation.cursor < operation.plan.delta_len {
                    let delta = operation.plan.deltas[usize::from(operation.cursor)].expect("pointer commit delta");
                    if let Some(node) = self.nodes.get_mut(operation.plan.id(delta.id)) {
                        operation.changed |= (node.x - delta.x).abs() > 1e-9 || (node.y - delta.y).abs() > 1e-9;
                        node.x = delta.x;
                        node.y = delta.y;
                    }
                    operation.cursor += 1;
                    return false;
                }
                if operation.phase == 0 {
                    if operation.changed {
                        self.bump_content_scene_generation();
                    }
                    operation.phase = 1;
                    return false;
                }
                if matches!(operation.plan.kind, BoardPointerPlanKind::FinishDrag) {
                    self.interaction = Interaction::None;
                }
                true
            }
            BoardPointerPlanKind::SelectionPreview { start, start_screen } => self.step_selection_preview_commit(operation, start, start_screen),
            BoardPointerPlanKind::SelectionCommit => self.step_selection_commit(operation),
            BoardPointerPlanKind::LinkMove { source, target, hover, compat_key, ring_key, end_world, activated, start_screen } => match operation.phase {
                0 => {
                    if !matches!(self.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. }) {
                        operation.faulted = true;
                        return true;
                    }
                    operation.phase = 1;
                    false
                }
                1 => {
                    self.link_compat_nodes_emit_key = Some(operation.plan.id(compat_key).to_owned());
                    operation.phase = 2;
                    false
                }
                2 => {
                    self.link_target_ring_emit_key = Some(operation.plan.id(ring_key).to_owned());
                    operation.phase = 3;
                    false
                }
                3 => {
                    self.hovered_id = hover.map(|span| operation.plan.id(span).to_owned());
                    self.hovered_kind = None;
                    operation.phase = 4;
                    false
                }
                4 => {
                    let source_id = operation.plan.id(source).to_owned();
                    let target_id = target.map(|span| operation.plan.id(span).to_owned());
                    self.interaction = if activated { Interaction::LinkDragSnap { source_id, target_id, end_world } } else { Interaction::LinkAtSourceHandle { source_id, start_screen } };
                    operation.phase = 5;
                    false
                }
                _ => {
                    self.bump_content_scene_generation();
                    true
                }
            },
            BoardPointerPlanKind::LinkFinish { source, target, edge, target_node, hover, compat_key, ring_key } => match operation.phase {
                0 => {
                    if !matches!(self.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. } | Interaction::LinkTargetNode { .. }) {
                        operation.faulted = true;
                        return true;
                    }
                    operation.phase = 1;
                    false
                }
                1 => {
                    if let (Some(target), Some(edge)) = (target, edge) {
                        let source_id = operation.plan.id(source).to_owned();
                        let target_id = operation.plan.id(target).to_owned();
                        let edge_id = operation.plan.id(edge).to_owned();
                        let Some(source_row) = self.handles.get(&source_id) else {
                            operation.faulted = true;
                            return true;
                        };
                        let Some(target_row) = self.handles.get(&target_id) else {
                            operation.faulted = true;
                            return true;
                        };
                        let edge_kind = self.default_edge_kind_for_created_link(source_row, target_row);
                        self.edges.insert(
                            edge_id.clone(),
                            EdgeData { id: edge_id, source: source_id, target: target_id, selected: false, visible: true, locked: false, style: None, edge_kind, source_tip: None, target_tip: None, properties: semio_framework_graph_manifest::PropertyBag::new() },
                        );
                    }
                    operation.phase = 2;
                    false
                }
                2 => {
                    self.link_compat_nodes_emit_key = compat_key.map(|span| operation.plan.id(span).to_owned());
                    operation.phase = 3;
                    false
                }
                3 => {
                    self.link_target_ring_emit_key = ring_key.map(|span| operation.plan.id(span).to_owned());
                    operation.phase = 4;
                    false
                }
                4 => {
                    self.interaction =
                        if let Some(target_node) = target_node { Interaction::LinkTargetNode { source_id: operation.plan.id(source).to_owned(), target_node_id: operation.plan.id(target_node).to_owned() } } else { Interaction::None };
                    operation.phase = 5;
                    false
                }
                5 => {
                    self.hovered_id = hover.map(|span| operation.plan.id(span).to_owned());
                    self.hovered_kind = None;
                    operation.phase = 6;
                    false
                }
                _ => {
                    self.bump_content_scene_generation();
                    true
                }
            },
            BoardPointerPlanKind::LinkRetain { hover } | BoardPointerPlanKind::Hover { hover } => {
                self.hovered_id = hover.map(|span| operation.plan.id(span).to_owned());
                self.hovered_kind = None;
                true
            }
            BoardPointerPlanKind::Brush { source, hover, alt, commit_old } => self.step_brush_pointer_commit(operation, source, hover, alt, commit_old),
        }
    }

    fn step_selection_preview_commit(&mut self, operation: &mut BoardPointerCommitOperation, start: Point, start_screen: Point) -> bool {
        match operation.phase {
            0 => {
                if !matches!(self.interaction, Interaction::SelectionPending { .. } | Interaction::Selection { .. }) {
                    operation.faulted = true;
                    return true;
                }
                operation.phase = 1;
                false
            }
            1 => {
                if let Some(id) = self.preselect.pop_first() {
                    self.set_pointer_selection_flag(&id, false);
                    return false;
                }
                operation.phase = 2;
                false
            }
            2 => {
                if self.preselect_removed.pop_first().is_some() {
                    return false;
                }
                operation.phase = 3;
                false
            }
            3 => {
                if operation.cursor < operation.plan.delta_len {
                    let delta = operation.plan.deltas[usize::from(operation.cursor)].expect("selection preview id");
                    let id = operation.plan.id(delta.id).to_owned();
                    self.preselect.insert(id.clone());
                    self.set_pointer_selection_flag(&id, true);
                    operation.cursor += 1;
                    return false;
                }
                operation.cursor = 0;
                operation.phase = 4;
                false
            }
            4 => {
                let next = match &self.interaction {
                    Interaction::SelectionPending { initial_ids, .. } | Interaction::Selection { initial_ids, .. } => match operation.scan_after.as_ref() {
                        Some(after) => initial_ids.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next().cloned(),
                        None => initial_ids.first().cloned(),
                    },
                    _ => None,
                };
                if let Some(id) = next {
                    operation.scan_after = Some(id.clone());
                    if !self.preselect.contains(&id) {
                        self.preselect_removed.insert(id);
                    }
                    return false;
                }
                operation.scan_after = None;
                operation.phase = 5;
                false
            }
            5 => {
                if operation.cursor < operation.plan.point_len {
                    let index = usize::from(operation.cursor);
                    operation.points.push(operation.plan.points[index]);
                    operation.screen_points.push(operation.plan.screen_points[index]);
                    operation.cursor += 1;
                    return false;
                }
                operation.cursor = 0;
                operation.phase = 6;
                false
            }
            6 => {
                if operation.screen_points.len() < 2 {
                    operation.phase = 7;
                    return false;
                }
                if self.selection_options.method == "lasso" {
                    if usize::from(operation.cursor) < operation.screen_points.len() {
                        operation.overlay_points.push(operation.screen_points[usize::from(operation.cursor)]);
                        operation.cursor += 1;
                        return false;
                    }
                } else if operation.cursor < 4 {
                    let last = *operation.screen_points.last().unwrap_or(&start_screen);
                    let point = match operation.cursor {
                        0 => start_screen,
                        1 => Point::new(last.x, start_screen.y),
                        2 => last,
                        _ => Point::new(start_screen.x, last.y),
                    };
                    operation.overlay_points.push(point);
                    operation.cursor += 1;
                    return false;
                }
                operation.phase = 7;
                false
            }
            7 => {
                let previous = std::mem::replace(&mut self.interaction, Interaction::None);
                let initial_ids = match previous {
                    Interaction::SelectionPending { initial_ids, .. } => initial_ids,
                    Interaction::Selection { initial_ids, points, screen_points, .. } => {
                        operation.retiring_points = points;
                        operation.retiring_screen_points = screen_points;
                        initial_ids
                    }
                    _ => {
                        operation.faulted = true;
                        return true;
                    }
                };
                self.selection_preview_crossing = !selection_drag_enclosing(self.selection_options.method.as_str(), start_screen, &operation.screen_points);
                operation.retiring_overlay_points = self.selection_screen_preview.take().unwrap_or_default();
                self.selection_screen_preview = (!operation.overlay_points.is_empty()).then(|| std::mem::take(&mut operation.overlay_points));
                self.interaction = Interaction::Selection { initial_ids, points: std::mem::take(&mut operation.points), screen_points: std::mem::take(&mut operation.screen_points), start, start_screen };
                if let Some((ids, removed, gesture)) = self.last_preselect_emit_sig.take() {
                    operation.retiring_signature_b = ids;
                    operation.retiring_signature_c = removed;
                    operation.retiring_gestures[1] = gesture;
                }
                operation.phase = 8;
                false
            }
            8 => {
                if operation.retire_one() {
                    return false;
                }
                true
            }
            _ => true,
        }
    }

    fn step_selection_commit(&mut self, operation: &mut BoardPointerCommitOperation) -> bool {
        match operation.phase {
            0 => {
                let previous = std::mem::replace(&mut self.interaction, Interaction::None);
                match previous {
                    Interaction::SelectionPending { initial_ids, .. } => operation.retiring_ids = initial_ids,
                    Interaction::Selection { initial_ids, points, screen_points, .. } => {
                        operation.retiring_ids = initial_ids;
                        operation.retiring_points = points;
                        operation.retiring_screen_points = screen_points;
                    }
                    other => {
                        self.interaction = other;
                        operation.faulted = true;
                        return true;
                    }
                }
                operation.phase = 1;
                false
            }
            1 => {
                if let Some(id) = self.selection.pop_first() {
                    self.set_pointer_selection_flag(&id, false);
                    return false;
                }
                operation.phase = 2;
                false
            }
            2 => {
                if let Some(id) = self.preselect.pop_first() {
                    self.set_pointer_selection_flag(&id, false);
                    return false;
                }
                operation.phase = 3;
                false
            }
            3 => {
                if self.preselect_removed.pop_first().is_some() {
                    return false;
                }
                operation.phase = 4;
                false
            }
            4 => {
                if self.selection_exit_highlight.pop_first().is_some() {
                    return false;
                }
                operation.phase = 5;
                false
            }
            5 => {
                if operation.retire_one() {
                    return false;
                }
                operation.phase = 6;
                false
            }
            6 => {
                if operation.cursor < operation.plan.delta_len {
                    let delta = operation.plan.deltas[usize::from(operation.cursor)].expect("selection commit id");
                    let id = operation.plan.id(delta.id).to_owned();
                    self.selection.insert(id.clone());
                    self.set_pointer_selection_flag(&id, true);
                    operation.cursor += 1;
                    return false;
                }
                operation.phase = 7;
                false
            }
            7 => {
                if let Some((ids, gesture)) = self.last_select_emit_sig.take() {
                    operation.retiring_signature_a = ids;
                    operation.retiring_gestures[0] = gesture;
                }
                if let Some((ids, removed, gesture)) = self.last_preselect_emit_sig.take() {
                    operation.retiring_signature_b = ids;
                    operation.retiring_signature_c = removed;
                    operation.retiring_gestures[1] = gesture;
                }
                operation.retiring_overlay_points = self.selection_screen_preview.take().unwrap_or_default();
                self.selection_preview_crossing = false;
                operation.phase = 8;
                false
            }
            8 => {
                if operation.retire_one() {
                    return false;
                }
                operation.phase = 9;
                false
            }
            _ => {
                self.bump_content_scene_generation();
                true
            }
        }
    }

    fn step_brush_pointer_commit(&mut self, operation: &mut BoardPointerCommitOperation, source: Option<BoardPointerSpan>, hover: Option<BoardPointerSpan>, alt: bool, commit_old: bool) -> bool {
        match operation.phase {
            0 => {
                if self.active_utility != ActiveUtility::Brush || matches!(self.interaction, Interaction::Pan { .. }) {
                    operation.faulted = true;
                    return true;
                }
                operation.phase = 1;
                false
            }
            1 => {
                if operation.cursor < operation.plan.delta_len {
                    let candidate = operation.plan.deltas[usize::from(operation.cursor)].expect("brush candidate");
                    if operation.brush_candidates.push(operation.plan.id(candidate.id), candidate.x as usize, 0.0).is_err() {
                        operation.faulted = true;
                        return true;
                    }
                    operation.cursor += 1;
                    return false;
                }
                operation.phase = 2;
                false
            }
            2 => {
                if commit_old {
                    self.brush_placement_serial = self.brush_placement_serial.wrapping_add(1);
                }
                operation.phase = 3;
                false
            }
            3 => {
                self.brush_alt_pressed = alt;
                self.brush_slot_suggestions_active = false;
                operation.phase = 4;
                false
            }
            4 => {
                self.brush_slot_source_id = source.map(|span| operation.plan.id(span).to_owned());
                operation.phase = 5;
                false
            }
            5 => {
                self.brush_candidates = std::mem::take(&mut operation.brush_candidates);
                self.brush_candidate_index = 0;
                operation.phase = 6;
                false
            }
            6 => {
                let offset = if alt { self.suggestion_offset } else { 0.0 };
                self.brush_preview = self.brush_slot_source_id.as_deref().and_then(|source_id| self.brush_candidates.first().and_then(|candidate| self.brush_build_preview_with_offset(source_id, candidate, offset)));
                operation.phase = 7;
                false
            }
            7 => {
                self.brush_preview_emit_key = None;
                operation.phase = 8;
                false
            }
            8 => {
                self.brush_candidates_emit_key = None;
                operation.phase = 9;
                false
            }
            9 => {
                self.hovered_id = hover.map(|span| operation.plan.id(span).to_owned());
                self.hovered_kind = None;
                operation.phase = 10;
                false
            }
            10 => {
                self.interaction = Interaction::None;
                operation.phase = 11;
                false
            }
            _ => {
                self.bump_content_scene_generation();
                true
            }
        }
    }

    pub fn close_event_authority_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        if context.should_yield() {
            return false;
        }
        if let Some(planning) = self.pending_delete_planning.as_mut() {
            planning.cancelling = true;
        }
        if self.pending_delete_planning.is_some() {
            let _ = self.step_delete_planning();
            context.consume_fuel(1);
            return false;
        }
        if let Some(operation) = self.pending_delete_operation.as_mut() {
            if !operation.claimed {
                operation.cancelling = true;
            }
        }
        if self.pending_delete_operation.is_some() {
            let _ = self.step_event_authority(context);
            return false;
        }
        if self.event_overflow.take().is_some() {
            context.consume_fuel(1);
            return false;
        }
        if let Some(batch) = self.event_batch_overflow.as_mut() {
            let _ = batch.pop();
            if batch.is_empty() {
                self.event_batch_overflow = None;
            }
            context.consume_fuel(1);
            return false;
        }
        let closed = self.events.close_step();
        context.consume_fuel(1);
        closed
    }

    pub fn event_authority_terminal_is_empty(&self) -> bool {
        self.pending_delete_planning.is_none() && self.pending_delete_operation.is_none() && self.event_overflow.is_none() && self.event_batch_overflow.is_none() && self.events.terminal_is_empty()
    }

    pub(super) fn is_preselecting(&self) -> bool {
        matches!(&self.interaction, Interaction::Selection { .. })
    }

    /// 💠️ Live area-select preview ids, or committed selection when not preselecting.
    fn selection_chrome_ids(&self) -> BTreeSet<String> {
        if self.is_preselecting() || !self.preselect.is_empty() {
            self.preselect.clone()
        } else {
            self.selection.clone()
        }
    }

    /// 🖱️ Empty selection on background click without exit/highlight chrome or preselect.
    pub(super) fn clear_selection_on_background_click(&mut self) {
        if self.selection.is_empty() {
            return;
        }
        let Some(reservation) = self.reserve_selection_event(BoardEventKind::Select, &[], None, None, None) else {
            return;
        };
        self.preselect.clear();
        self.preselect_removed.clear();
        self.last_preselect_emit_sig = None;
        self.last_select_emit_sig = None;
        self.selection_exit_highlight.clear();
        self.selection.clear();
        self.sync_selection_flags_to_objects();
        self.publish_event_reservation(reservation);
    }

    pub(super) fn sync_selection_flags_to_objects(&mut self) {
        let chrome = self.selection_chrome_ids();
        for n in self.nodes.values_mut() {
            n.selected = chrome.contains(&n.id);
        }
        for h in self.handles.values_mut() {
            h.selected = chrome.contains(&h.id);
        }
        for e in self.edges.values_mut() {
            e.selected = chrome.contains(&e.id);
        }
        for w in self.wires.values_mut() {
            w.selected = chrome.contains(&w.id);
        }
        for r in self.regions.values_mut() {
            r.selected = chrome.contains(&r.id);
        }
    }

    pub(super) fn push_select_event(&mut self) {
        self.last_select_emit_sig = None;
        let mut sorted: Vec<_> = self.selection.iter().cloned().collect();
        sorted.sort();
        let Some(reservation) = self.reserve_selection_event(BoardEventKind::Select, &sorted, None, None, None) else {
            return;
        };
        self.publish_event_reservation(reservation);
    }

    pub fn set_selection_ids(&mut self, ids: &[String]) {
        let next: BTreeSet<String> = ids.iter().cloned().collect();
        if next == self.selection {
            return;
        }
        let mut sorted: Vec<_> = next.iter().cloned().collect();
        sorted.sort();
        let Some(reservation) = self.reserve_selection_event(BoardEventKind::Select, &sorted, None, None, None) else {
            return;
        };
        self.preselect.clear();
        self.preselect_removed.clear();
        self.last_preselect_emit_sig = None;
        self.selection_exit_highlight.clear();
        self.selection = next;
        self.sync_selection_flags_to_objects();
        self.last_select_emit_sig = None;
        self.publish_event_reservation(reservation);
    }

    /// 🔇️ Updates committed selection without emitting `select` (controlled React sync).
    pub fn set_selection_ids_silent(&mut self, ids: &[String]) {
        let next: BTreeSet<String> = ids.iter().cloned().collect();
        if next == self.selection {
            return;
        }
        self.preselect.clear();
        self.preselect_removed.clear();
        self.last_preselect_emit_sig = None;
        self.selection_exit_highlight.clear();
        self.selection = next;
        self.sync_selection_flags_to_objects();
    }

    /// 🔇️ Mirrors area-select preview chrome without emitting `preselect` (shared multi-view sync).
    pub fn set_preselect_state_silent(&mut self, ids: &[String], removed_ids: &[String]) {
        let next: BTreeSet<String> = ids.iter().cloned().collect();
        let removed: BTreeSet<String> = removed_ids.iter().cloned().collect();
        if self.preselect == next && self.preselect_removed == removed {
            return;
        }
        self.preselect = next;
        self.preselect_removed = removed;
        self.sync_selection_flags_to_objects();
    }

    pub(super) fn set_selection_ids_gestured(&mut self, ids: &[String], gesture: Option<&str>) {
        let next: BTreeSet<String> = ids.iter().cloned().collect();
        let mut sorted: Vec<_> = next.iter().cloned().collect();
        sorted.sort();
        let gesture_owned = gesture.map(ToOwned::to_owned);
        let sig = (sorted.clone(), gesture_owned);
        if next == self.selection && self.last_select_emit_sig.as_ref() == Some(&sig) {
            return;
        }
        let Some(reservation) = self.reserve_selection_event(BoardEventKind::Select, &sorted, None, None, gesture) else {
            return;
        };
        self.last_select_emit_sig = Some(sig);
        self.preselect.clear();
        self.preselect_removed.clear();
        self.last_preselect_emit_sig = None;
        if next != self.selection {
            self.selection_exit_highlight.clear();
            self.selection = next;
            self.sync_selection_flags_to_objects();
        }
        self.publish_event_reservation(reservation);
    }

    /// 👁️ Rectangle/lasso drag preview: `preselect` + `preselect_removed` (anchor \\ preselect); emits `preselect` only.
    pub(super) fn apply_area_preselect(&mut self, anchor_ids: &BTreeSet<String>, ids: &[String], gesture: Option<&str>) {
        let next: BTreeSet<String> = ids.iter().cloned().collect();
        let sorted = Self::sorted_selection_ids(&next);
        let removed = Self::sorted_selection_ids(&anchor_ids.difference(&next).cloned().collect());
        let gesture_owned = gesture.map(ToOwned::to_owned);
        let sig = (sorted.clone(), removed.clone(), gesture_owned);
        if self.preselect == next && self.last_preselect_emit_sig.as_ref() == Some(&sig) {
            return;
        }
        let Some(reservation) = self.reserve_selection_event(BoardEventKind::Preselect, &sorted, None, Some(&removed), gesture) else {
            return;
        };
        self.last_preselect_emit_sig = Some(sig);
        self.preselect = next;
        self.preselect_removed = anchor_ids.difference(&self.preselect).cloned().collect();
        self.set_hovered_id_silent(None);
        self.sync_selection_flags_to_objects();
        self.publish_event_reservation(reservation);
    }

    fn sorted_selection_ids(set: &BTreeSet<String>) -> Vec<String> {
        let mut v: Vec<_> = set.iter().cloned().collect();
        v.sort();
        v
    }

    /// 🧿️ Ends a rectangle/lasso cycle: commits `selection`, clears preselect (highlight only lives in preselect).
    pub(super) fn commit_area_select_from_initial(&mut self, initial_ids: &BTreeSet<String>, ids: &[String], gesture: Option<&str>) {
        let next: BTreeSet<String> = ids.iter().cloned().collect();
        let sorted = Self::sorted_selection_ids(&next);
        let anchor = Self::sorted_selection_ids(initial_ids);
        let gesture_owned = gesture.map(ToOwned::to_owned);
        let Some(reservation) = self.reserve_selection_event(BoardEventKind::Select, &sorted, Some(&anchor), None, gesture) else {
            return;
        };
        self.last_select_emit_sig = None;
        self.last_preselect_emit_sig = None;
        self.preselect.clear();
        self.preselect_removed.clear();
        self.selection_exit_highlight.clear();
        self.selection = next;
        self.sync_selection_flags_to_objects();
        let _ = gesture_owned;
        self.publish_event_reservation(reservation);
    }

    /// 🧿️ True during left‑button rectangle/lasso drag so callers can avoid descriptor round‑trips that fight the live marquee state.
    pub fn is_dragging_area_select(&self) -> bool {
        matches!(&self.interaction, Interaction::Selection { .. })
    }

    /// 🖐️ True from the press that opens a pointer lane — a pending or live area select, or any gesture
    /// [`Self::defers_descriptor_sync_from_js`] names — until its release, so a host keeps routing that release to the board
    /// even when it lands outside the surface.
    pub fn pointer_lane_in_flight(&self) -> bool {
        self.defers_descriptor_sync_from_js() || matches!(&self.interaction, Interaction::SelectionPending { .. } | Interaction::Selection { .. })
    }

    /// 🎛️ True while the pointer drives a lane [`Self::plan_pointer`] does not model — a gumball transform drag (the
    /// rotate ring's `rotate` record), a target-region body drag or grip resize, or anything under the area brush. A host
    /// routes those moves and releases through [`Self::pointer_move_screen`] / [`Self::pointer_up_screen`], exactly as
    /// React's Board2dHost routes every pointer, and publishes the events they queue.
    pub fn pointer_lane_is_direct(&self) -> bool {
        self.active_utility == ActiveUtility::AreaBrush || self.region_drag.is_some() || self.transform_drag.is_some()
    }

    /// 🧿️ True during area select, link gestures, node drag, the rotate-ring gesture, or camera pan so JS can defer full `syncDescriptorJson` round-trips.
    pub fn defers_descriptor_sync_from_js(&self) -> bool {
        self.transform_drag.is_some()
            || self.region_drag.is_some()
            || self.region_paint.is_some()
            || matches!(self.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. } | Interaction::LinkTargetNode { .. } | Interaction::ExternalLinkPreview { .. } | Interaction::DragNodes { .. } | Interaction::Pan { .. })
    }

    /// 🩺️ What the engine is doing right now, as one probe-readable row: the live gesture,
    /// the armed utility, the hovered id and the selection/preselect sizes. Published by the React
    /// host as `data-board-interaction-json`, the board twin of `World3dHost`'s `data-interaction-json`.
    pub fn interaction_json(&self) -> String {
        let mode = if self.region_paint.is_some() {
            "regionPaint"
        } else if self.region_drag.is_some() {
            "regionDrag"
        } else if self.transform_drag.is_some() {
            "transformRotate"
        } else {
            match self.interaction {
                Interaction::None => "none",
                Interaction::Pan { .. } => "pan",
                Interaction::DragNodes { .. } => "dragNodes",
                Interaction::SelectionPending { .. } => "selectionPending",
                Interaction::Selection { .. } => "selection",
                Interaction::LinkAtSourceHandle { .. } => "linkAtSourceHandle",
                Interaction::LinkDragSnap { .. } => "linkDragSnap",
                Interaction::LinkTargetNode { .. } => "linkTargetNode",
                Interaction::ExternalLinkPreview { .. } => "externalLinkPreview",
            }
        };
        let utility = match self.active_utility {
            ActiveUtility::Select => "select",
            ActiveUtility::Brush => "brush",
            ActiveUtility::AreaBrush => "areaBrush",
        };
        let mut out = String::from("{\"mode\":\"");
        out.push_str(mode);
        out.push_str("\",\"utility\":\"");
        out.push_str(utility);
        out.push_str("\",\"hoveredId\":");
        match self.hovered_id.as_deref() {
            Some(id) => {
                out.push('"');
                out.push_str(&id.replace('\\', "\\\\").replace('"', "\\\""));
                out.push('"');
            }
            None => out.push_str("null"),
        }
        out.push_str(",\"selectionCount\":");
        out.push_str(&self.selection.len().to_string());
        out.push_str(",\"preselectCount\":");
        out.push_str(&self.preselect.len().to_string());
        out.push_str(",\"revision\":");
        out.push_str(&self.interaction_revision.to_string());
        out.push_str(",\"deferringDescriptorSync\":");
        out.push_str(if self.defers_descriptor_sync_from_js() { "true" } else { "false" });
        out.push('}');
        out
    }

    /// 🩺️ Every handle the pane can actually be POINTED AT, as one bounded probe row:
    /// world position, owning node, handle kind, and whether the handle is still free (no incident
    /// edge — the precondition `openHandleSuggestions` and `createEdge` both need). `data-board-
    /// positions-json` carries NODES only, so nothing in the DOM ever named a handle and no
    /// headless caller could aim at one.
    ///
    /// 🧱️ Bounded twice over, because Nakagin carries 358 handles: the row set is the camera's own
    /// viewport (a handle off-screen cannot be clicked, so publishing it buys nothing) and then a
    /// hard [`BOARD_HANDLE_VITALS_CAP`]. `total`/`onScreen`/`capped` say exactly what was left out.
    pub fn handle_positions_json(&self) -> String {
        let (w, h) = (f64::from(self.width), f64::from(self.height));
        let mut rows: Vec<(&str, Point, &str, &str, bool)> = Vec::new();
        let mut on_screen = 0usize;
        for (id, handle) in &self.handles {
            let Some(world) = self.handle_world_pos(handle) else { continue };
            let screen = self.world_to_screen(world);
            if screen.x < 0.0 || screen.y < 0.0 || screen.x > w || screen.y > h || !self.handle_effectively_visible(id.as_str()) {
                continue;
            }
            on_screen += 1;
            if rows.len() < BOARD_HANDLE_VITALS_CAP {
                rows.push((id.as_str(), world, handle.node_id.as_str(), handle.handle_kind.as_str(), !self.handle_has_incident_edge(id.as_str())));
            }
        }
        let quote = |value: &str, out: &mut String| {
            out.push('"');
            out.push_str(&value.replace('\\', "\\\\").replace('"', "\\\""));
            out.push('"');
        };
        let mut out = String::from("{\"total\":");
        out.push_str(&self.handles.len().to_string());
        out.push_str(",\"onScreen\":");
        out.push_str(&on_screen.to_string());
        out.push_str(",\"published\":");
        out.push_str(&rows.len().to_string());
        out.push_str(",\"capped\":");
        out.push_str(if on_screen > rows.len() { "true" } else { "false" });
        out.push_str(",\"rows\":[");
        for (index, (id, world, node_id, handle_kind, open)) in rows.iter().copied().enumerate() {
            if index > 0 {
                out.push(',');
            }
            out.push('[');
            quote(id, &mut out);
            out.push(',');
            out.push_str(&world.x.to_string());
            out.push(',');
            out.push_str(&world.y.to_string());
            out.push(',');
            quote(node_id, &mut out);
            out.push(',');
            quote(handle_kind, &mut out);
            out.push(',');
            out.push_str(if open { "true" } else { "false" });
            out.push(']');
        }
        out.push_str("]}");
        out
    }

    pub fn world_to_screen(&self, p: Point) -> Point {
        semio_framework_canvas::camera::world_to_screen(&self.camera, &self.viewport(), p)
    }

    pub fn screen_to_world(&self, p: Point) -> Point {
        semio_framework_canvas::camera::screen_to_world(&self.camera, &self.viewport(), p)
    }

    fn node_kind_scale(&self, node_kind: &str) -> f64 {
        self.node_kinds.get(node_kind).map_or(1.0, |k| k.scale)
    }

    fn handle_kind_scale(&self, handle_kind: &str) -> f64 {
        self.handle_kinds.get(handle_kind).map_or(1.0, |k| k.scale)
    }

    fn effective_node_scale(&self, n: &NodeData) -> f64 {
        (n.scale * self.node_kind_scale(n.node_kind.as_str())).max(1e-9)
    }

    pub(super) fn scaled_node_radius(&self, n: &NodeData) -> f64 {
        n.radius * self.effective_node_scale(n)
    }

    pub(super) fn scaled_node_width(&self, n: &NodeData) -> f64 {
        n.width * self.effective_node_scale(n)
    }

    pub(super) fn scaled_node_height(&self, n: &NodeData) -> f64 {
        n.height * self.effective_node_scale(n)
    }

    fn effective_handle_scale(&self, h: &HandleData) -> f64 {
        let node_scale = self.nodes.get(h.node_id.as_str()).map_or(1.0, |n| self.effective_node_scale(n));
        (node_scale * h.scale * self.handle_kind_scale(h.handle_kind.as_str())).max(1e-9)
    }

    pub(crate) fn effective_handle_radius(&self, h: &HandleData) -> f64 {
        h.radius * self.effective_handle_scale(h)
    }

    pub(crate) fn handle_world_pos(&self, h: &HandleData) -> Option<Point> {
        let n = self.nodes.get(&h.node_id)?;
        Some(match n.shape {
            NodeShape::Circle => handle_position_on_circle(Point::new(n.x, n.y), self.scaled_node_radius(n), h.angle),
            NodeShape::Rectangle => handle_position_on_rectangle(Point::new(n.x, n.y), self.scaled_node_width(n), self.scaled_node_height(n), h.angle),
        })
    }

    /// 🎯️ Where an edge meets a handle: the cap's outer peak, the outward normal, and the node center.
    fn handle_cap_attachment(&self, h: &HandleData) -> Option<(Point, Vec2, Point)> {
        let rim = self.handle_world_pos(h)?;
        let n = self.nodes.get(&h.node_id)?;
        let node_center = Point::new(n.x, n.y);
        let outward = handle_outward_at_node_rim(rim, node_center, n.shape, self.scaled_node_radius(n), self.scaled_node_width(n), self.scaled_node_height(n))?;
        let peak = handle_exterior_cap_peak(rim, outward, self.effective_handle_radius(h));
        Some((peak, outward, node_center))
    }

    /// 📐️ Node half-extent for indirect ring layout: circle radius or half the shorter rectangle side.
    fn indirect_node_half_extent(&self, n: &NodeData) -> f64 {
        match n.shape {
            NodeShape::Circle => self.scaled_node_radius(n),
            NodeShape::Rectangle => self.scaled_node_width(n).min(self.scaled_node_height(n)) * 0.5,
        }
    }

    /// 📐️ Radial world offset from node rim to indirect-handle center (`INDIRECT_HANDLE_RING_GAP_NODE_SCALE`× half-extent) so ring–node proportions stay fixed when zooming.
    fn indirect_handle_ring_offset_world(&self, n: &NodeData) -> f64 {
        (self.indirect_node_half_extent(n) * INDIRECT_HANDLE_RING_GAP_NODE_SCALE).max(1e-9)
    }

    /// 📐️ Ghost link handles sit on a rim offset by `INDIRECT_HANDLE_RING_GAP_NODE_SCALE`× node half-extent from the node body so ring spacing scales with the node at every zoom.
    pub fn indirect_handle_world_pos(&self, h: &HandleData) -> Option<Point> {
        let n = self.nodes.get(&h.node_id)?;
        let offset = self.indirect_handle_ring_offset_world(n);
        Some(match n.shape {
            NodeShape::Circle => handle_position_on_circle(Point::new(n.x, n.y), self.scaled_node_radius(n) + offset, h.angle),
            NodeShape::Rectangle => handle_position_on_rectangle(Point::new(n.x, n.y), self.scaled_node_width(n) + 2.0 * offset, self.scaled_node_height(n) + 2.0 * offset, h.angle),
        })
    }

    /// 📐️ Indirect-connect marker radius in world units: `INDIRECT_HANDLE_MARKER_NODE_SCALE`× circle radius or × half the shorter rectangle side.
    pub fn indirect_handle_marker_radius_world(&self, h: &HandleData) -> f64 {
        let Some(n) = self.nodes.get(&h.node_id) else {
            return (self.effective_handle_radius(h) * INDIRECT_HANDLE_MARKER_NODE_SCALE).max(1e-9);
        };
        let handle_local_scale = (h.scale * self.handle_kind_scale(h.handle_kind.as_str())).max(1e-9);
        (self.indirect_node_half_extent(n) * INDIRECT_HANDLE_MARKER_NODE_SCALE * handle_local_scale).max(1e-9)
    }

    /// 🧭️ Source handle id while a link wire is drawn (`LinkDragSnap` / `LinkTargetNode`).
    pub(super) fn active_link_source_handle_id(&self) -> Option<&str> {
        match &self.interaction {
            Interaction::LinkDragSnap { source_id, .. } | Interaction::LinkTargetNode { source_id, .. } | Interaction::ExternalLinkPreview { source_id, .. } => Some(source_id.as_str()),
            _ => None,
        }
    }

    /// 🧭️ Visible target node ids that expose at least one free handle compatible with `source_handle_id`.
    pub(super) fn link_drag_compatible_target_node_ids(&self, source_handle_id: &str) -> Vec<String> {
        let Some(source) = self.handles.get(source_handle_id) else {
            return Vec::new();
        };
        let source_node_id = source.node_id.as_str();
        let mut out = Vec::new();
        let mut seen = BTreeSet::new();
        for (hid, h) in &self.handles {
            if h.node_id == source_node_id || !self.handle_effectively_visible(hid.as_str()) {
                continue;
            }
            if self.handle_has_incident_edge(hid.as_str()) {
                continue;
            }
            if !self.handles_link_compatible_for_drag(source, h) {
                continue;
            }
            if !self.nodes.get(&h.node_id).is_some_and(|n| n.visible) {
                continue;
            }
            if seen.insert(h.node_id.clone()) {
                out.push(h.node_id.clone());
            }
        }
        out.sort();
        out
    }

    /// 🧭️ Count of visible free handles on `node_id` compatible with `source_handle_id`.
    pub(super) fn link_compatible_handle_count_on_node(&self, source_handle_id: &str, node_id: &str) -> usize {
        let Some(source) = self.handles.get(source_handle_id) else {
            return 0;
        };
        if source.node_id == node_id {
            return 0;
        }
        self.handles.iter().filter(|(id, h)| h.node_id == node_id && self.handle_eligible_link_target_ring(id.as_str(), source_handle_id) && self.handles_link_compatible_for_drag(source, h)).count()
    }

    /// 🧭️ Free compatible handle ids on `node_id` for an active link from `source_handle_id`.
    pub(super) fn link_compatible_handle_ids_on_node(&self, source_handle_id: &str, node_id: &str) -> Vec<String> {
        let Some(source) = self.handles.get(source_handle_id) else {
            return Vec::new();
        };
        let mut out: Vec<String> = self
            .handles
            .iter()
            .filter_map(|(id, h)| {
                if h.node_id != node_id {
                    return None;
                }
                if !self.handle_eligible_link_target_ring(id.as_str(), source_handle_id) {
                    return None;
                }
                self.handles_link_compatible_for_drag(source, h).then(|| id.clone())
            })
            .collect();
        out.sort();
        out
    }

    /// 🧭️ Compatible target node under `world` while a link wire is active (node body hit).
    pub(super) fn link_drag_ring_target_node_id(&self, source_handle_id: &str, world: Point) -> Option<String> {
        let nid = self.resolve_node_hit_world(world)?;
        if self.handles.get(source_handle_id)?.node_id == nid {
            return None;
        }
        self.node_has_any_free_link_compatible_handle(source_handle_id, nid.as_str()).then_some(nid)
    }

    /// 🧭️ Resolves which single node draws the overview/normal indirect handle ring when that node has **more than one** eligible free handles (otherwise the sole handle is implicit).
    pub(super) fn indirect_ring_node_id(&self, lod: BoardDrawLod) -> Option<String> {
        if !matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
            return None;
        }
        if let Interaction::LinkTargetNode { source_id, target_node_id } = &self.interaction {
            if self.link_compatible_handle_count_on_node(source_id, target_node_id) > 1 {
                return self.nodes.get(target_node_id).filter(|n| n.visible).map(|n| n.id.clone());
            }
            return None;
        }
        if let Interaction::ExternalLinkPreview { ring_node_id: Some(target_node_id), ring_handle_ids, .. } = &self.interaction {
            if ring_handle_ids.len() > 1 {
                return self.nodes.get(target_node_id).filter(|n| n.visible).map(|n| n.id.clone());
            }
            return None;
        }
        if let Interaction::LinkDragSnap { source_id, end_world, .. } = &self.interaction {
            let ring_nid = self.link_drag_ring_target_node_id(source_id, *end_world)?;
            if self.link_compatible_handle_count_on_node(source_id, ring_nid.as_str()) > 1 {
                return Some(ring_nid);
            }
            return None;
        }
        if self.active_link_source_handle_id().is_some() {
            return None;
        }
        let ring_nid = if self.selection.len() == 1 {
            self.selection.iter().next()?.clone()
        } else {
            return None;
        };
        let n = self.nodes.get(&ring_nid).filter(|n| n.visible)?;
        if self.eligible_indirect_handle_count_on_node(n.id.as_str()) > 1 {
            Some(ring_nid)
        } else {
            None
        }
    }

    fn eligible_indirect_handle_count_on_node(&self, node_id: &str) -> usize {
        self.handles.iter().filter(|(id, h)| h.node_id == node_id && self.handle_effectively_visible(id.as_str()) && self.handle_eligible_indirect_connect_ring(id.as_str())).count()
    }

    /// 🧭️ Returns the handle id when `node_id` has exactly one visible free indirect-eligible handle.
    pub(super) fn sole_eligible_indirect_handle_on_node(&self, node_id: &str) -> Option<String> {
        let mut found: Option<String> = None;
        for (id, h) in &self.handles {
            if h.node_id != node_id || !self.handle_effectively_visible(id.as_str()) || !self.handle_eligible_indirect_connect_ring(id.as_str()) {
                continue;
            }
            if found.is_some() {
                return None;
            }
            found = Some(id.clone());
        }
        found
    }

    /// 🧭️ When the drop target has exactly one free handle compatible with `source_handle_id`, returns that handle id (otherwise `None`).
    pub(super) fn node_sole_free_link_compatible_handle(&self, source_handle_id: &str, target_node_id: &str) -> Option<String> {
        let source = self.handles.get(source_handle_id)?;
        if source.node_id == target_node_id {
            return None;
        }
        let mut found: Option<String> = None;
        for (id, h) in &self.handles {
            if h.node_id != target_node_id || !self.handle_effectively_visible(id.as_str()) {
                continue;
            }
            if self.handle_has_incident_edge(id.as_str()) {
                continue;
            }
            if !self.handles_link_compatible_for_drag(source, h) {
                continue;
            }
            if found.is_some() {
                return None;
            }
            found = Some(id.clone());
        }
        found
    }

    pub(super) fn point_in_node_world(&self, n: &NodeData, point: Point) -> bool {
        match n.shape {
            NodeShape::Rectangle => {
                let hw = self.scaled_node_width(n) / 2.0;
                let hh = self.scaled_node_height(n) / 2.0;
                (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh
            }
            NodeShape::Circle => distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n),
        }
    }

    fn sole_indirect_handle_hit_link_target(&self, point: Point) -> Option<String> {
        let Interaction::LinkTargetNode { source_id, target_node_id } = &self.interaction else {
            return None;
        };
        let th = self.node_sole_free_link_compatible_handle(source_id, target_node_id)?;
        let n = self.nodes.get(target_node_id)?;
        if !n.visible {
            return None;
        }
        if !self.point_in_node_world(n, point) {
            return None;
        }
        Some(th)
    }

    fn sole_indirect_handle_hit_idle_selected_node(&self, point: Point) -> Option<String> {
        if !matches!(self.interaction, Interaction::None) {
            return None;
        }
        if self.selection.len() != 1 {
            return None;
        }
        let nid = self.selection.iter().next()?;
        if !self.nodes.contains_key(nid) {
            return None;
        }
        let sole = self.sole_eligible_indirect_handle_on_node(nid)?;
        let n = self.nodes.get(nid)?;
        if !n.visible {
            return None;
        }
        if !self.point_in_node_world(n, point) {
            return None;
        }
        Some(sole)
    }

    /// 🧭️ True when `target_node_id` hosts at least one visible free handle that can pair with `source_handle_id` under link-compat rules.
    pub(super) fn node_has_any_free_link_compatible_handle(&self, source_handle_id: &str, target_node_id: &str) -> bool {
        let Some(source) = self.handles.get(source_handle_id) else {
            return false;
        };
        if source.node_id == target_node_id {
            return false;
        }
        for (hid, h) in &self.handles {
            if h.node_id != target_node_id || !self.handle_effectively_visible(hid.as_str()) {
                continue;
            }
            if self.handle_has_incident_edge(hid.as_str()) {
                continue;
            }
            if self.handles_link_compatible_for_drag(source, h) {
                return true;
            }
        }
        false
    }

    /// 💫️ True when the handle may appear on a link-target ghost ring (`overview`/`normal` LOD).
    pub(super) fn handle_eligible_link_target_ring(&self, handle_id: &str, source_handle_id: &str) -> bool {
        if !self.handle_effectively_visible(handle_id) || self.handle_has_incident_edge(handle_id) {
            return false;
        }
        let Some(source) = self.handles.get(source_handle_id) else {
            return false;
        };
        let Some(target) = self.handles.get(handle_id) else {
            return false;
        };
        if source.node_id == target.node_id {
            return false;
        }
        self.handles_link_compatible_for_drag(source, target)
    }

    pub(super) fn indirect_ring_handle_eligible(&self, handle_id: &str, ring_node_id: &str) -> bool {
        if self.handles.get(handle_id).is_none_or(|h| h.node_id != ring_node_id) {
            return false;
        }
        if let Some(source_id) = self.active_link_source_handle_id() {
            self.handle_eligible_link_target_ring(handle_id, source_id)
        } else {
            self.handle_eligible_indirect_connect_ring(handle_id)
        }
    }

    fn link_drag_target_ring_hit(&self, source_id: &str, point: Point) -> Option<String> {
        if !matches!(self.current_draw_lod(), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
            return None;
        }
        if let Interaction::ExternalLinkPreview { source_id: active_source, ring_handle_ids, .. } = &self.interaction {
            if active_source != source_id {
                return None;
            }
            let zoom = self.camera.zoom;
            for hid in ring_handle_ids {
                let Some(h) = self.handles.get(hid) else {
                    continue;
                };
                if !self.handle_eligible_link_target_ring(h.id.as_str(), source_id) {
                    continue;
                }
                let Some(pos) = self.indirect_handle_world_pos(h) else {
                    continue;
                };
                let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h);
                if distance_between(point, pos) <= tol {
                    return Some(h.id.clone());
                }
            }
            return None;
        }
        let node_id = self.link_drag_ring_target_node_id(source_id, point)?;
        if self.link_compatible_handle_count_on_node(source_id, node_id.as_str()) <= 1 {
            return None;
        }
        let zoom = self.camera.zoom;
        for h in self.handles.values().rev() {
            if h.node_id != node_id || !self.handle_eligible_link_target_ring(h.id.as_str(), source_id) {
                continue;
            }
            let Some(pos) = self.indirect_handle_world_pos(h) else { continue };
            let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h);
            if distance_between(point, pos) <= tol {
                return Some(h.id.clone());
            }
        }
        None
    }

    fn link_target_ring_snapshot(&self, source_handle_id: &str) -> (Option<String>, Vec<String>) {
        if let Interaction::ExternalLinkPreview { source_id, ring_node_id, ring_handle_ids, .. } = &self.interaction {
            if source_id == source_handle_id {
                return (ring_node_id.clone(), ring_handle_ids.clone());
            }
        }
        let node_id = match &self.interaction {
            Interaction::LinkTargetNode { target_node_id, .. } => Some(target_node_id.clone()),
            Interaction::LinkDragSnap { end_world, .. } => self.link_drag_ring_target_node_id(source_handle_id, *end_world),
            _ => None,
        };
        let Some(nid) = node_id else {
            return (None, Vec::new());
        };
        if self.link_compatible_handle_count_on_node(source_handle_id, nid.as_str()) <= 1 {
            return (None, Vec::new());
        }
        (Some(nid.clone()), self.link_compatible_handle_ids_on_node(source_handle_id, nid.as_str()))
    }

    pub(super) fn sync_link_gesture_events(&mut self) {
        if let Interaction::ExternalLinkPreview { source_id, compatible_node_ids, ring_node_id, ring_handle_ids, .. } = &self.interaction {
            if compatible_node_ids.len().saturating_add(ring_handle_ids.len()) > BOARD_POINTER_ITEM_CAPACITY {
                self.event_schema_fault = true;
                return;
            }
            let compat_key = format!("{}|{}", source_id, compatible_node_ids.join(","));
            let ring_key = format!("{}|{}|{}", source_id, ring_node_id.as_deref().unwrap_or(""), ring_handle_ids.join(","));
            let compat_changed = self.link_compat_nodes_emit_key.as_deref() != Some(compat_key.as_str());
            let ring_changed = self.link_target_ring_emit_key.as_deref() != Some(ring_key.as_str());
            let compat_event = compat_changed.then(|| BoardOwnedEvent::link_compatible(source_id, compatible_node_ids));
            let ring_event = ring_changed.then(|| BoardOwnedEvent::link_ring(source_id, ring_node_id.as_deref(), ring_handle_ids));
            let (first, second) = match (compat_event, ring_event) {
                (Some(first), second) => (first, second),
                (None, Some(first)) => (first, None),
                (None, None) => return,
            };
            let Some(reservation) = self.reserve_owned_batch(first, second) else {
                return;
            };
            if compat_changed {
                self.link_compat_nodes_emit_key = Some(compat_key);
            }
            if ring_changed {
                self.link_target_ring_emit_key = Some(ring_key);
            }
            self.publish_event_batch(reservation);
            return;
        }
        let Some(source) = self.active_link_source_handle_id().map(str::to_string) else {
            self.clear_link_gesture_events();
            return;
        };
        let node_ids = self.link_drag_compatible_target_node_ids(&source);
        if node_ids.len() > BOARD_POINTER_ITEM_CAPACITY {
            self.event_schema_fault = true;
            return;
        }
        let compat_key = format!("{}|{}", source, node_ids.join(","));
        let (ring_node_id, ring_handle_ids) = self.link_target_ring_snapshot(&source);
        if node_ids.len().saturating_add(ring_handle_ids.len()) > BOARD_POINTER_ITEM_CAPACITY {
            self.event_schema_fault = true;
            return;
        }
        let ring_key = format!("{}|{}|{}", source, ring_node_id.as_deref().unwrap_or(""), ring_handle_ids.join(","));
        let compat_changed = self.link_compat_nodes_emit_key.as_deref() != Some(compat_key.as_str());
        let ring_changed = self.link_target_ring_emit_key.as_deref() != Some(ring_key.as_str());
        let compat_event = compat_changed.then(|| BoardOwnedEvent::link_compatible(&source, &node_ids));
        let ring_event = ring_changed.then(|| BoardOwnedEvent::link_ring(&source, ring_node_id.as_deref(), &ring_handle_ids));
        let (first, second) = match (compat_event, ring_event) {
            (Some(first), second) => (first, second),
            (None, Some(first)) => (first, None),
            (None, None) => return,
        };
        let Some(reservation) = self.reserve_owned_batch(first, second) else {
            return;
        };
        if compat_changed {
            self.link_compat_nodes_emit_key = Some(compat_key);
        }
        if ring_changed {
            self.link_target_ring_emit_key = Some(ring_key);
        }
        self.publish_event_batch(reservation);
    }

    pub(super) fn clear_link_gesture_events(&mut self) {
        let compat = self.link_compat_nodes_emit_key.is_some();
        let ring = self.link_target_ring_emit_key.is_some();
        let empty: &[String] = &[];
        let compat_event = compat.then(|| BoardOwnedEvent::link_compatible("", empty));
        let ring_event = ring.then(|| BoardOwnedEvent::link_ring("", None, empty));
        let (first, second) = match (compat_event, ring_event) {
            (Some(first), second) => (first, second),
            (None, Some(first)) => (first, None),
            (None, None) => return,
        };
        let Some(reservation) = self.reserve_owned_batch(first, second) else {
            return;
        };
        if compat {
            self.link_compat_nodes_emit_key = None;
        }
        if ring {
            self.link_target_ring_emit_key = None;
        }
        self.publish_event_batch(reservation);
    }

    fn node_center_world(&self, node_id: &str) -> Option<Point> {
        let n = self.nodes.get(node_id)?;
        Some(Point::new(n.x, n.y))
    }

    pub(super) fn edge_curve(&self, e: &EdgeData) -> Option<CubicBez> {
        if !self.has_ports() {
            let source_node = self.nodes.get(&e.source)?;
            let target_node = self.nodes.get(&e.target)?;
            let source_center = Point::new(source_node.x, source_node.y);
            let target_center = Point::new(target_node.x, target_node.y);
            let source_pos = self.node_rim_point_toward(source_node, target_center);
            let target_pos = self.node_rim_point_toward(target_node, source_center);
            return Some(compute_edge_bezier_points(source_pos, target_pos, source_center, target_center));
        }
        let source_handle = self.handles.get(&e.source)?;
        let target_handle = self.handles.get(&e.target)?;
        let (source_pos, source_out, _) = self.handle_cap_attachment(source_handle)?;
        let (target_pos, target_out, _) = self.handle_cap_attachment(target_handle)?;
        Some(compute_edge_bezier_outward(source_pos, target_pos, source_out, target_out))
    }

    fn link_drag_wire_curve_world(&self, source_id: &str, target_id: Option<&str>, end_world: Point) -> Option<CubicBez> {
        let source_handle = self.handles.get(source_id)?;
        let (source_pos, source_out, _) = self.handle_cap_attachment(source_handle)?;
        let (target_pos, target_out) = if let Some(tid) = target_id {
            let (peak, outward, _) = self.handle_cap_attachment(self.handles.get(tid)?)?;
            (peak, outward)
        } else {
            (end_world, Vec2::ZERO)
        };
        Some(compute_edge_bezier_outward(source_pos, target_pos, source_out, target_out))
    }

    pub(super) fn active_link_wire_curve(&self) -> Option<CubicBez> {
        match &self.interaction {
            Interaction::LinkDragSnap { source_id, target_id, end_world } => self.link_drag_wire_curve_world(source_id.as_str(), target_id.as_deref(), *end_world),
            Interaction::LinkTargetNode { source_id, target_node_id } => self.link_drag_wire_curve_world(source_id.as_str(), None, self.node_center_world(target_node_id)?),
            Interaction::ExternalLinkPreview { source_id, end_world, .. } => self.link_drag_wire_curve_world(source_id.as_str(), None, *end_world),
            Interaction::DragNodes { proximity_pair: Some((src, tgt)), .. } => self.link_drag_wire_curve_world(src.as_str(), Some(tgt.as_str()), Point::ZERO),
            _ => None,
        }
    }

    pub(super) fn wire_curve(&self, w: &WireData) -> Option<CubicBez> {
        let end_world = match (&w.target, w.end_x, w.end_y) {
            (None, Some(x), Some(y)) if x.is_finite() && y.is_finite() => Point::new(x, y),
            (Some(tid), _, _) => {
                self.handles.get(tid)?;
                return self.edge_curve(&EdgeData {
                    id: w.id.clone(),
                    source: w.source.clone(),
                    target: tid.clone(),
                    selected: w.selected,
                    visible: w.visible,
                    locked: w.locked,
                    style: w.style.clone(),
                    edge_kind: String::new(),
                    source_tip: None,
                    target_tip: None,
                    properties: w.properties.clone(),
                });
            }
            _ => return None,
        };
        self.link_drag_wire_curve_world(w.source.as_str(), None, end_world)
    }

    pub(super) fn apply_link_drag_snap_hover(&mut self, _source_handle_id: &str, world: Point, target_handle_id: Option<&str>) {
        if let Some(tid) = target_handle_id {
            self.set_hovered_id(Some(tid.to_string()));
        } else {
            self.update_hover_from_world(world);
        }
    }

    /// 🧭️ Minimap/overview LOD: group selection and bounded drag only — no per-node/edge/handle picks.
    fn lod_disables_discrete_pick(&self) -> bool {
        matches!(self.current_draw_lod(), BoardDrawLod::Minimap | BoardDrawLod::Overview)
    }

    /// 🔗️ Overview LOD: tight world-radius hit on a free handle so link drag can start without enabling broad `resolve_hit_world` handle picks.
    pub(super) fn resolve_overview_free_link_handle_pointer_world(&self, point: Point) -> Option<String> {
        if !matches!(self.current_draw_lod(), BoardDrawLod::Overview) {
            return None;
        }
        if !self.selection_options.select_handles {
            return None;
        }
        const MAX_D_WORLD: f64 = 2.25;
        let mut best: Option<(f64, String)> = None;
        for h in self.handles.values() {
            if !self.handle_effectively_visible(h.id.as_str()) || self.handle_has_incident_edge(h.id.as_str()) {
                continue;
            }
            let Some(pos) = self.handle_world_pos(h) else {
                continue;
            };
            let d = distance_between(point, pos);
            if d <= MAX_D_WORLD && best.as_ref().is_none_or(|(bd, _)| d < *bd) {
                best = Some((d, h.id.clone()));
            }
        }
        best.map(|(_, id)| id)
    }

    /// 🧭️ Minimap/overview LOD: pointer-down inside the selection AABB moves the group without a discrete hit.
    pub(super) fn lod_uses_bounded_drag(&self) -> bool {
        matches!(self.current_draw_lod(), BoardDrawLod::Minimap | BoardDrawLod::Overview)
    }

    pub(super) fn resolve_hover_world(&self, point: Point) -> Option<String> {
        let lod = self.current_draw_lod();
        let zoom = self.camera.zoom;
        if self.has_ports() && !matches!(lod, BoardDrawLod::Minimap) {
            if matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                if let Some(hid) = self.sole_indirect_handle_hit_link_target(point) {
                    return Some(hid);
                }
                if let Interaction::LinkDragSnap { source_id, .. } = &self.interaction {
                    if let Some(hid) = self.link_drag_target_ring_hit(source_id, point) {
                        return Some(hid);
                    }
                }
            }
            if let Some(ring_node_id) = self.indirect_ring_node_id(lod) {
                for h in self.handles.values().rev() {
                    if h.node_id != ring_node_id || !self.handle_selectable(h.id.as_str()) {
                        continue;
                    }
                    if !self.indirect_ring_handle_eligible(h.id.as_str(), ring_node_id.as_str()) {
                        continue;
                    }
                    let Some(pos) = self.indirect_handle_world_pos(h) else { continue };
                    let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h);
                    if distance_between(point, pos) <= tol {
                        return Some(h.id.clone());
                    }
                }
            }
            if matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro) {
                for h in self.handles.values().rev() {
                    if !self.handle_selectable(h.id.as_str()) {
                        continue;
                    }
                    let Some(pos) = self.handle_world_pos(h) else { continue };
                    let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.effective_handle_radius(h);
                    if distance_between(point, pos) <= tol {
                        return Some(h.id.clone());
                    }
                }
            }
            if matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                if let Some(hid) = self.sole_indirect_handle_hit_idle_selected_node(point) {
                    if self.handle_selectable(hid.as_str()) {
                        return Some(hid);
                    }
                }
            }
        }
        for n in self.nodes.values().rev() {
            if !self.node_selectable(n.id.as_str()) {
                continue;
            }
            match n.shape {
                NodeShape::Rectangle => {
                    let hw = self.scaled_node_width(n) / 2.0;
                    let hh = self.scaled_node_height(n) / 2.0;
                    if (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh {
                        return Some(n.id.clone());
                    }
                }
                NodeShape::Circle => {
                    if distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n) {
                        return Some(n.id.clone());
                    }
                }
            }
        }
        for w in self.wires.values().rev() {
            if !self.wire_selectable(w) {
                continue;
            }
            if let Some(c) = self.wire_curve(w) {
                if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                    return Some(w.id.clone());
                }
            }
        }
        for e in self.edges.values().rev() {
            if !self.edge_selectable(e) {
                continue;
            }
            if let Some(c) = self.edge_curve(e) {
                if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                    return Some(e.id.clone());
                }
            }
        }
        None
    }

    fn push_pick_target(out: &mut Vec<BoardPickTargetJson>, domain: &str, id: String, generality: u32, label: Option<String>) {
        if out.iter().any(|row| row.domain == domain && row.id == id) {
            return;
        }
        out.push(BoardPickTargetJson { domain: domain.to_string(), id, generality, label });
    }

    /// 🎯️ The handle the pointer's own hit test resolves leads — including the indirect-ring handles
    /// Overview/Compact draw, which the per-LOD loop below never visits — so a right-click names exactly
    /// the handle the hover paints, not the node underneath it.
    ///
    /// 🎯️ Regions are the least specific target on the board — last in the list, so a context
    /// menu offers the graph first and the backdrop only when nothing else is under the point.
    fn resolve_pick_targets_world(&self, point: Point) -> Vec<BoardPickTargetJson> {
        let mut out = Vec::new();
        let lod = self.current_draw_lod();
        let zoom = self.camera.zoom;
        if let Some(id) = self.resolve_hit_world(point).filter(|id| self.handles.contains_key(id.as_str())) {
            Self::push_pick_target(&mut out, "handle", id.clone(), 2, Some(id));
        }
        if self.has_ports() && !matches!(lod, BoardDrawLod::Minimap) && matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro) {
            for h in self.handles.values().rev() {
                if !self.handle_selectable(h.id.as_str()) {
                    continue;
                }
                let Some(pos) = self.handle_world_pos(h) else { continue };
                let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.effective_handle_radius(h);
                if distance_between(point, pos) <= tol {
                    Self::push_pick_target(&mut out, "handle", h.id.clone(), 2, Some(h.id.clone()));
                }
            }
        }
        for n in self.nodes.values().rev() {
            if !self.node_selectable(n.id.as_str()) {
                continue;
            }
            let hit = match n.shape {
                NodeShape::Rectangle => {
                    let hw = self.scaled_node_width(n) / 2.0;
                    let hh = self.scaled_node_height(n) / 2.0;
                    (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh
                }
                NodeShape::Circle => distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n),
            };
            if hit {
                Self::push_pick_target(&mut out, "node", n.id.clone(), 0, n.text.clone());
            }
        }
        for w in self.wires.values().rev() {
            if !self.wire_selectable(w) {
                continue;
            }
            if let Some(c) = self.wire_curve(w) {
                if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                    Self::push_pick_target(&mut out, "wire", w.id.clone(), 1, Some(w.id.clone()));
                }
            }
        }
        for e in self.edges.values().rev() {
            if !self.edge_selectable(e) {
                continue;
            }
            if let Some(c) = self.edge_curve(e) {
                if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                    Self::push_pick_target(&mut out, "edge", e.id.clone(), 1, Some(e.id.clone()));
                }
            }
        }
        for region in self.regions.values() {
            if region.hidden {
                continue;
            }
            if region_grip_at(self.region_live_bounds(region), zoom, point).is_some() {
                Self::push_pick_target(&mut out, "region", region.id.clone(), 3, region.label.clone().or_else(|| Some(region.id.clone())));
            }
        }
        out
    }

    /// 🎯️ All pick targets under a screen point as JSON (`domain`, `id`, `generality`).
    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        let world = self.screen_to_world(Point::new(sx, sy));
        serde_json::to_string(&self.resolve_pick_targets_world(world)).unwrap_or_else(|_| "[]".into())
    }

    pub fn resolve_hit_world(&self, point: Point) -> Option<String> {
        if self.lod_disables_discrete_pick() {
            return None;
        }
        let zoom = self.camera.zoom;
        let o = &self.selection_options;
        if self.has_ports() && o.select_handles {
            if matches!(self.current_draw_lod(), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                if let Some(hid) = self.sole_indirect_handle_hit_link_target(point) {
                    return Some(hid);
                }
                if let Interaction::LinkDragSnap { source_id, .. } = &self.interaction {
                    if let Some(hid) = self.link_drag_target_ring_hit(source_id, point) {
                        return Some(hid);
                    }
                }
            }
            if let Some(ring_node_id) = self.indirect_ring_node_id(self.current_draw_lod()) {
                for h in self.handles.values().rev() {
                    if h.node_id != ring_node_id || !self.handle_selectable(h.id.as_str()) {
                        continue;
                    }
                    if !self.indirect_ring_handle_eligible(h.id.as_str(), ring_node_id.as_str()) {
                        continue;
                    }
                    let Some(pos) = self.indirect_handle_world_pos(h) else { continue };
                    let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h);
                    if distance_between(point, pos) <= tol {
                        return Some(h.id.clone());
                    }
                }
            }
            if matches!(self.current_draw_lod(), BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro) {
                for h in self.handles.values().rev() {
                    if !self.handle_selectable(h.id.as_str()) {
                        continue;
                    }
                    let Some(pos) = self.handle_world_pos(h) else { continue };
                    let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.effective_handle_radius(h);
                    if distance_between(point, pos) <= tol {
                        return Some(h.id.clone());
                    }
                }
            }
            if matches!(self.current_draw_lod(), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                if let Some(hid) = self.sole_indirect_handle_hit_idle_selected_node(point) {
                    return Some(hid);
                }
            }
        }
        if o.select_nodes {
            for n in self.nodes.values().rev() {
                if !self.node_selectable(n.id.as_str()) {
                    continue;
                }
                match n.shape {
                    NodeShape::Rectangle => {
                        let hw = self.scaled_node_width(n) / 2.0;
                        let hh = self.scaled_node_height(n) / 2.0;
                        if (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh {
                            return Some(n.id.clone());
                        }
                    }
                    NodeShape::Circle => {
                        if distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n) {
                            return Some(n.id.clone());
                        }
                    }
                }
            }
        }
        if o.select_edges {
            for e in self.edges.values().rev() {
                if !self.edge_selectable(e) {
                    continue;
                }
                if let Some(c) = self.edge_curve(e) {
                    if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                        return Some(e.id.clone());
                    }
                }
            }
        }
        None
    }

    pub(super) fn resolve_node_hit_world(&self, point: Point) -> Option<String> {
        for n in self.nodes.values().rev() {
            if !n.visible {
                continue;
            }
            match n.shape {
                NodeShape::Rectangle => {
                    let hw = self.scaled_node_width(n) / 2.0;
                    let hh = self.scaled_node_height(n) / 2.0;
                    if (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh {
                        return Some(n.id.clone());
                    }
                }
                NodeShape::Circle => {
                    if distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n) {
                        return Some(n.id.clone());
                    }
                }
            }
        }
        None
    }
}
    // #endregion board_host
