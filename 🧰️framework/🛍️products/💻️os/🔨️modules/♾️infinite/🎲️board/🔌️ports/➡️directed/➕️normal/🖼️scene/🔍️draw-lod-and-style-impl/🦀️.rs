//! 🔍️ Draw level of detail and style impl of the board scene: the scene state helpers, the draw level of detail for a frame, the overlay paint state and the style state methods. See [`super`].

use super::*;

impl BoardSceneState {
    pub fn bump_content_scene_generation(&mut self) {
        self.content_scene_generation = self.content_scene_generation.wrapping_add(1);
    }

    #[doc(hidden)]
    pub fn test_content_scene_generation(&self) -> u64 {
        self.content_scene_generation
    }

    pub fn viewport(&self) -> semio_framework_canvas::camera::Viewport {
        semio_framework_canvas::camera::Viewport { width: self.width, height: self.height, dpr: self.dpr }
    }

    pub fn camera_content_affine(&self) -> Affine {
        semio_framework_canvas::camera::camera_content_affine(&self.camera.into(), &self.viewport())
    }
}

/// 📶️ Draw LOD used while building the vector scene (pins during wheel zoom).
pub fn board_draw_lod_for_frame(scene: &BoardSceneState, style: &BoardStyleState) -> BoardDrawLod {
    if style.wheel_zoom_active {
        if let Some(pinned) = style.wheel_zoom_render_lod {
            return pinned;
        }
    }
    current_draw_lod(scene, style)
}

/// 🏷️ Camera, draw LOD, and visible node centers from the WASM host for the JS text overlay (must match the last GPU frame).
pub fn overlay_paint_state_json(scene: &BoardSceneState, style: &BoardStyleState) -> String {
    let nodes: Vec<serde_json::Value> = scene.nodes.values().filter(|n| n.visible).map(|n| serde_json::json!({ "id": n.id, "x": n.x, "y": n.y })).collect();
    serde_json::json!({
        "camera": { "x": scene.camera.x, "y": scene.camera.y, "zoom": scene.camera.zoom },
        "lod": board_draw_lod_label(board_draw_lod_for_frame(scene, style)),
        "nodes": nodes,
    })
    .to_string()
}

impl BoardStyleState {
    pub fn grid_step_large_world(&self) -> f64 {
        GRID_WORLD_LARGE * self.grid_factor
    }
    pub fn grid_step_medium_world(&self) -> f64 {
        GRID_WORLD_MEDIUM * self.grid_factor
    }
    pub fn grid_step_small_world(&self) -> f64 {
        GRID_WORLD_SMALL * self.grid_factor
    }
    pub fn grid_step_micro_world(&self) -> f64 {
        GRID_WORLD_MICRO * self.grid_factor
    }

    /// 🪪️ Installs the owning domain's placement naming: brush placements minted and fill snapshots captured
    /// afterwards carry its id prefixes and stage labels.
    pub fn set_naming(&mut self, naming: &'static BoardNaming) {
        self.naming = naming;
        self.placement_naming = None;
    }

    /// 🆔️ The placement naming this host mints ids with.
    pub fn naming(&self) -> &'static BoardNaming {
        self.naming
    }

    /// 📮️ Installs (`Some`) or clears (`None`) the brush prefixes a scene carries in wire form ([`BoardPlacementNaming`]);
    /// brush placements minted afterwards use them instead of the installed naming's. A refused form changes nothing.
    pub fn set_placement_naming_json(&mut self, json: Option<&str>) -> Result<(), NormalPortError> {
        self.placement_naming = json.map(BoardPlacementNaming::from_json).transpose()?;
        Ok(())
    }

    /// 🔍️ The brush prefixes this host mints with, the wire override first, in wire form.
    pub fn brush_placement_naming(&self) -> BoardPlacementNaming {
        BoardPlacementNaming { brush_node_prefix: self.brush_node_prefix().into(), brush_edge_prefix: self.brush_edge_prefix().into() }
    }

    /// 🔤️ The node id prefix the next brush placement is minted with.
    pub fn brush_node_prefix(&self) -> &str {
        self.placement_naming.as_ref().map_or(self.naming.brush_node_prefix, |naming| naming.brush_node_prefix.as_str())
    }

    /// 🔡️ The edge id prefix the next brush placement is minted with.
    pub fn brush_edge_prefix(&self) -> &str {
        self.placement_naming.as_ref().map_or(self.naming.brush_edge_prefix, |naming| naming.brush_edge_prefix.as_str())
    }

    pub fn has_ports(&self) -> bool {
        self.port_mode.has_ports()
    }

    pub(crate) fn node_rim_point_toward(&self, node: &NodeData, toward: Point) -> Point {
        let center = Point::new(node.x, node.y);
        match node.shape {
            NodeShape::Circle => {
                let radius = self.scaled_node_radius(node);
                let angle = circle_handle_angle_toward(center, toward);
                handle_position_on_circle(center, radius, angle)
            }
            NodeShape::Rectangle => {
                let width = self.scaled_node_width(node);
                let height = self.scaled_node_height(node);
                let angle = rectangle_handle_angle_toward(center, width, height, toward);
                handle_position_on_rectangle(center, width, height, angle)
            }
        }
    }

    pub fn set_grid_snap_enabled(&mut self, enabled: bool) {
        self.grid_snap_enabled = enabled;
    }

    pub fn set_automatic_lod(&mut self, enabled: bool) {
        self.automatic_lod = enabled;
        if enabled {
            self.forced_draw_lod = None;
        }
    }

    pub fn set_forced_draw_lod_label(&mut self, label: &str) {
        let t = label.trim();
        if t.is_empty() {
            self.forced_draw_lod = None;
            return;
        }
        self.forced_draw_lod = match BoardDrawLod::from_id(t) {
            Some(lod) => Some(lod),
            None => {
                self.forced_draw_lod = None;
                return;
            }
        };
    }

    pub fn set_grid_factor(&mut self, v: f64) -> Result<(), NormalPortError> {
        if !v.is_finite() || v <= 0.0 || v > 1_000_000.0 {
            return Err(NormalPortError::GridFactorOutOfRange);
        }
        self.grid_factor = v;
        Ok(())
    }

    pub(crate) fn get_or_build_icon_paint(&self, encoded: &str, fg: Color, bg: Color, preserve_original_style: bool) -> Option<CachedIconPaintLease<'_>> {
        self.icon_paint_cache.get_or_build(encoded, fg, bg, preserve_original_style)
    }

    pub fn clear_icon_vector_cache(&mut self) {
        self.icon_paint_cache.clear();
    }

    /// 🔗️ JSON `[{ "source","target","bidirectional"?,"important"?,"specificity"? },…]` gates link gestures; empty clears restrictions.
    pub fn set_handle_link_compat_from_json(&mut self, json: &str) -> Result<(), NormalPortError> {
        let v: serde_json::Value = serde_json::from_str(json)?;
        let arr = v.as_array().ok_or(NormalPortError::CompatNotArray)?;
        let mut next = Vec::new();
        for row in arr {
            let o = row.as_object().ok_or(NormalPortError::RowNotObject("compat"))?;
            let source = o.get("source").and_then(|x| x.as_str()).ok_or(NormalPortError::CompatSourceMissing)?.trim().to_string();
            let target = o.get("target").and_then(|x| x.as_str()).ok_or(NormalPortError::CompatTargetMissing)?.trim().to_string();
            let bidirectional = o.get("bidirectional").and_then(|x| x.as_bool()).unwrap_or(false);
            let important = o.get("important").and_then(|x| x.as_bool()).unwrap_or(false);
            let spec_s = o.get("specificity").and_then(|x| x.as_str()).unwrap_or("handle");
            let specificity = parse_compat_specificity(spec_s)?;
            next.push(LinkCompatRule { source, target, bidirectional, important, specificity });
        }
        self.link_compat_rules = next;
        Ok(())
    }

    /// 🧩️ JSON object `{ handleKinds?, wireKinds?, nodeKinds?, edgeKinds? }` replacing prior catalogs (omit arrays to clear that slice).
    pub fn set_board_kind_catalogs_from_json(&mut self, json: &str) -> Result<(), NormalPortError> {
        if json.len() > BOARD_EVENT_BYTE_CAPACITY {
            return Err(NormalPortError::EventCredits);
        }
        let v: serde_json::Value = serde_json::from_str(json)?;
        let o = v.as_object().ok_or(NormalPortError::KindCatalogsRootNotObject)?;
        for key in ["handleKinds", "wireKinds", "nodeKinds", "edgeTips", "edgeKinds"] {
            if o.get(key).and_then(serde_json::Value::as_array).is_some_and(|rows| rows.len() > BOARD_POINTER_ITEM_CAPACITY) {
                return Err(NormalPortError::EventCredits);
            }
        }
        let template_count = o
            .get("nodeKinds")
            .and_then(serde_json::Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(serde_json::Value::as_object)
            .filter_map(|row| row.get("handles"))
            .filter_map(serde_json::Value::as_array)
            .try_fold(0usize, |count, handles| count.checked_add(handles.len()))
            .ok_or(NormalPortError::EventCredits)?;
        if template_count > BOARD_POINTER_ITEM_CAPACITY {
            return Err(NormalPortError::EventCredits);
        }
        if let Some(arr) = o.get("handleKinds").and_then(|x| x.as_array()) {
            let mut next = BTreeMap::new();
            for row in arr {
                let ho = row.as_object().ok_or(NormalPortError::RowNotObject("handle kind"))?;
                reject_kind_catalog_row_legacy_label(ho, "handle")?;
                let id = ho.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("handle kind"))?;
                let name = ho.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                let color_s = ho.get("color").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::HandleKindColorMissing)?;
                let color = parse_css_color(color_s).ok_or_else(|| NormalPortError::InvalidHandleKindColor(color_s.to_string()))?;
                let default_wire_kind = ho.get("defaultWireKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                let scale = ho.get("scale").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0).unwrap_or(1.0);
                next.insert(id.to_string(), HandleKindDef { name, color, default_wire_kind, scale });
            }
            self.handle_kinds = next;
        }
        if let Some(arr) = o.get("wireKinds").and_then(|x| x.as_array()) {
            let mut next = BTreeMap::new();
            for row in arr {
                let wo = row.as_object().ok_or(NormalPortError::RowNotObject("wire kind"))?;
                reject_kind_catalog_row_legacy_label(wo, "wire")?;
                let id = wo.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("wire kind"))?;
                let name = wo.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                let default_edge_kind = wo.get("defaultEdgeKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                next.insert(id.to_string(), WireKindDef { name, default_edge_kind });
            }
            self.wire_kinds = next;
        }
        if let Some(arr) = o.get("nodeKinds").and_then(|x| x.as_array()) {
            let mut next = BTreeMap::new();
            for row in arr {
                let no = row.as_object().ok_or(NormalPortError::RowNotObject("node kind"))?;
                reject_kind_catalog_row_legacy_label(no, "node")?;
                let id = no.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("node kind"))?;
                let name = no.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                let scale = no.get("scale").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0).unwrap_or(1.0);
                let shape = match no.get("shape").and_then(|x| x.as_str()).map(str::trim) {
                    Some("rectangle") => NodeShape::Rectangle,
                    _ => NodeShape::Circle,
                };
                let mut handles: Vec<NodeKindHandleTemplate> = Vec::new();
                if let Some(arr) = no.get("handles").and_then(|x| x.as_array()) {
                    for row in arr {
                        let ho = row.as_object().ok_or(NormalPortError::RowNotObject("node kind handle"))?;
                        let handle_kind = ho.get("handleKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::NodeKindHandleKindMissing)?;
                        let angle = ho.get("angle").and_then(|x| x.as_f64()).filter(|x| x.is_finite()).ok_or(NormalPortError::NodeKindHandleAngleMissing)?;
                        let radius = ho.get("radius").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0);
                        handles.push(NodeKindHandleTemplate { handle_kind: handle_kind.to_string(), angle, radius });
                    }
                }
                let icon = no.get("icon").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                let color_fill = no.get("color").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).and_then(parse_css_hex_color);
                next.insert(id.to_string(), NodeKindDef { name, scale, shape, handles, icon, color_fill });
            }
            self.node_kinds = next;
        }
        if let Some(arr) = o.get("edgeTips").and_then(|x| x.as_array()) {
            let mut tips = builtin_edge_tips();
            for row in arr {
                let eo = row.as_object().ok_or(NormalPortError::RowNotObject("edge tip"))?;
                let id = eo.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("edge tip"))?;
                let geometry=match eo.get("geometry") {
                    None=>None,
                    Some(value)=>Some(match value.as_str().map(str::trim){Some("arrow")=>EdgeTipGeometry::Arrow,Some("fine-arrow")=>EdgeTipGeometry::FineArrow,Some("diamond")=>EdgeTipGeometry::Diamond,Some("circle")=>EdgeTipGeometry::Circle,Some("bar")=>EdgeTipGeometry::Bar,_=>return Err(NormalPortError::EdgeTipRowInvalid(id.to_string()))})
                };
                let entry=EdgeTipCatalogEntry{id:id.to_string(),geometry,filled:eo.get("filled").and_then(|v|v.as_bool()),scale:eo.get("scale").and_then(|v|v.as_f64())};
                let def = EdgeTipDef::from_catalog_entry(&entry).ok_or_else(|| NormalPortError::EdgeTipRowInvalid(id.to_string()))?;
                tips.insert(id.to_string(), def);
            }
            self.edge_tips = tips;
        }
        if let Some(arr) = o.get("edgeKinds").and_then(|x| x.as_array()) {
            let mut next = BTreeMap::new();
            for row in arr {
                let eo = row.as_object().ok_or(NormalPortError::RowNotObject("edge kind"))?;
                reject_kind_catalog_row_legacy_label(eo, "edge")?;
                let id = eo.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("edge kind"))?;
                let name = eo.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                let color = eo.get("color").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).and_then(parse_css_hex_color);
                let stroke_width = eo
                    .get("stroke")
                    .and_then(|x| x.as_f64())
                    .filter(|v| v.is_finite() && *v > 0.0)
                    .or_else(|| eo.get("stroke").and_then(|x| x.as_str()).and_then(|s| s.trim().parse::<f64>().ok()).filter(|v| v.is_finite() && *v > 0.0))
                    .unwrap_or(2.0);
                let pattern = match eo.get("pattern").and_then(|x| x.as_str()).map(str::trim) {
                    Some("dashed") => EdgeStrokePattern::Dashed,
                    Some("dotted") => EdgeStrokePattern::Dotted,
                    _ => EdgeStrokePattern::Solid,
                };
                let source_tip = parse_catalog_tip_slot(eo.get("sourceTip").or_else(|| eo.get("source_tip")).and_then(|x| x.as_str()));
                let target_tip = parse_catalog_tip_slot(eo.get("targetTip").or_else(|| eo.get("target_tip")).and_then(|x| x.as_str()).or_else(|| eo.get("marker").and_then(|x| x.as_str())));
                let directed = eo.get("directed").and_then(|x| x.as_bool()).unwrap_or(true);
                next.insert(id.to_string(), EdgeKindDef { name, color, stroke_width, pattern, source_tip, target_tip, directed });
            }
            self.edge_kinds = next;
        }
        Ok(())
    }

    /// 🛡️ Ensures runtime catalogs declare every kind from a compile-time manifest.
    pub fn validate_against_manifest(&self, gm: &GraphManifest) -> Result<(), NormalPortError> {
        for row in &gm.port_kinds {
            let visual = row.presentation.as_ref().is_some_and(|p| p.get("color").is_some());
            if visual && !self.handle_kinds.contains_key(&row.id) {
                return Err(NormalPortError::CatalogMissingKind("handle", row.id.clone()));
            }
        }
        for row in &gm.wire_kinds {
            if !self.wire_kinds.contains_key(&row.id) {
                return Err(NormalPortError::CatalogMissingKind("wire", row.id.clone()));
            }
        }
        for row in &gm.edge_kinds {
            if row.presentation.is_some() && !self.edge_kinds.contains_key(&row.id) {
                return Err(NormalPortError::CatalogMissingKind("edge", row.id.clone()));
            }
        }
        for row in &gm.node_kinds {
            if row.presentation.is_some() && !self.node_kinds.contains_key(&row.id) {
                return Err(NormalPortError::CatalogMissingKind("node", row.id.clone()));
            }
        }
        Ok(())
    }

    fn lookup_edge_tip<'a>(&'a self, id: &str) -> Option<&'a EdgeTipDef> {
        if id.is_empty() {
            return None;
        }
        self.edge_tips.get(id)
    }

    fn resolve_tip_slot<'a>(&'a self, slot: Option<&str>) -> Option<&'a EdgeTipDef> {
        match slot {
            Some("") => None,
            Some(id) => self.lookup_edge_tip(id),
            None => None,
        }
    }

    pub(crate) fn resolve_handle_fill_color(&self, h: &HandleData, theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
        match kind {
            BoardElementStyleKind::Hovered => theme.handle_fill_hovered,
            BoardElementStyleKind::Selected => theme.handle_fill_selected,
            BoardElementStyleKind::Highlighted => theme.handle_fill_selection_exit,
            BoardElementStyleKind::Disabled => theme.handle_fill_disabled,
            BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => {
                if let Some(c) = h.color_fill {
                    return c;
                }
                if let Some(def) = self.handle_kinds.get(&h.handle_kind) {
                    return def.color;
                }
                theme.handle_fill
            }
        }
    }

    pub fn resolve_node_fill_color(&self, n: &NodeData, theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
        let theme_fill = node_fill_for_style(theme, kind);
        match kind {
            BoardElementStyleKind::Hovered | BoardElementStyleKind::Selected | BoardElementStyleKind::Highlighted | BoardElementStyleKind::Disabled => theme_fill,
            BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => {
                let kind_id = n.node_kind.trim();
                if kind_id.is_empty() {
                    return theme_fill;
                }
                self.node_kinds.get(kind_id).and_then(|def| def.color_fill).unwrap_or(theme_fill)
            }
        }
    }

    pub fn resolve_edge_tips<'a>(&'a self, e: &EdgeData) -> (Option<&'a EdgeTipDef>, Option<&'a EdgeTipDef>) {
        let kind_def = self.edge_kinds.get(e.edge_kind.as_str());
        let source_slot = e.source_tip.as_deref().or_else(|| kind_def.and_then(|d| d.source_tip.as_deref()));
        let target_slot = e.target_tip.as_deref().or_else(|| kind_def.and_then(|d| d.target_tip.as_deref()));
        let source = self.resolve_tip_slot(source_slot);
        let mut target = self.resolve_tip_slot(target_slot);
        if target.is_none() && target_slot.is_none() {
            let directed = kind_def.is_none_or(|d| d.directed);
            if directed {
                target = self.lookup_edge_tip("arrow");
            }
        }
        (source, target)
    }

    pub fn resolve_default_wire_kind_for_handle(&self, h: &HandleData) -> String {
        self.handle_kinds.get(&h.handle_kind).and_then(|d| d.default_wire_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| DEFAULT_WIRE_KIND_ID.to_string())
    }

    pub(super) fn resolve_default_edge_kind_for_wire_kind(&self, wire_kind: &str) -> String {
        self.wire_kinds.get(wire_kind).and_then(|d| d.default_edge_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_default()
    }

    pub fn default_edge_kind_for_created_link(&self, source: &HandleData, _target: &HandleData) -> String {
        let wk = self.resolve_default_wire_kind_for_handle(source);
        self.resolve_default_edge_kind_for_wire_kind(&wk)
    }

    fn resolve_default_wire_kind_for_handle_kind(&self, handle_kind: &str) -> String {
        self.handle_kinds.get(handle_kind).and_then(|d| d.default_wire_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| DEFAULT_WIRE_KIND_ID.to_string())
    }

    pub fn board_fill_default_wire_kind<'a>(&'a self, handle_kind: &str) -> Result<&'a str, BoardFillCaptureFault> {
        let Some(value) = self.handle_kinds.get(handle_kind).and_then(|kind| kind.default_wire_kind.as_deref()) else {
            return Ok(DEFAULT_WIRE_KIND_ID);
        };
        if value.len() > BOARD_FILL_TEXT_BYTES {
            return Err(BoardFillCaptureFault::TextCapacity);
        }
        let value = value.trim();
        Ok(if value.is_empty() { DEFAULT_WIRE_KIND_ID } else { value })
    }

    pub fn board_fill_default_edge_kind<'a>(&'a self, wire_kind: &str) -> Result<&'a str, BoardFillCaptureFault> {
        let Some(value) = self.wire_kinds.get(wire_kind).and_then(|kind| kind.default_edge_kind.as_deref()) else {
            return Ok("");
        };
        if value.len() > BOARD_FILL_TEXT_BYTES {
            return Err(BoardFillCaptureFault::TextCapacity);
        }
        Ok(value.trim())
    }

    pub(super) fn link_kinds_compatible_for_brush(&self, sn: &str, sh: &str, tn: &str, th: &str) -> bool {
        if !handle_port_shapes_compatible(sh, th) {
            return false;
        }
        if !single_letter_port_families_compatible(sh, th) {
            return false;
        }
        if self.link_compat_rules.is_empty() {
            return true;
        }
        let w_src = self.resolve_default_wire_kind_for_handle_kind(sh);
        let w_tgt = self.resolve_default_wire_kind_for_handle_kind(th);
        let e_src = self.resolve_default_edge_kind_for_wire_kind(&w_src);
        let e_tgt = self.resolve_default_edge_kind_for_wire_kind(&w_tgt);
        let mut matched: Vec<&LinkCompatRule> = self.link_compat_rules.iter().filter(|rule| link_gesture_rule_applies_kind_strings(rule, sn, sh, &w_src, &e_src, tn, th, &w_tgt, &e_tgt)).collect();
        if matched.is_empty() {
            return false;
        }
        if matched.iter().any(|r| r.important) {
            matched.retain(|r| r.important);
        } else {
            let max_rank = matched.iter().map(|r| r.specificity as i32).max().unwrap_or(0);
            matched.retain(|r| (r.specificity as i32) == max_rank);
        }
        !matched.is_empty()
    }
}
