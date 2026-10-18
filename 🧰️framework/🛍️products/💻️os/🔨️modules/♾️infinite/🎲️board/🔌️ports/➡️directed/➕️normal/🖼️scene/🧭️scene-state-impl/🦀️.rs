//! 🧭️ Scene state impl of the board scene: the large scene state method block. See [`super`].

use super::*;

impl BoardSceneState {
    /// 📍️ Applies peer-pane node drags without a full descriptor re-sync.
    pub fn set_node_positions(&mut self, moves: &[(String, f64, f64)]) {
        let mut geometry_changed = false;
        for (id, x, y) in moves {
            if !x.is_finite() || !y.is_finite() {
                continue;
            }
            if let Some(node) = self.nodes.get_mut(id.as_str()) {
                if (node.x - *x).abs() > 1e-9 || (node.y - *y).abs() > 1e-9 {
                    node.x = *x;
                    node.y = *y;
                    geometry_changed = true;
                }
            }
        }
        if geometry_changed {
            self.bump_content_scene_generation();
        }
    }

    /// 📍️ Parses `[{"id","x","y"},…]` and updates existing host nodes in place.
    pub fn set_node_positions_json(&mut self, json: &str) -> Result<(), NormalPortError> {
        #[derive(Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
        struct NodePositionMoveJson {
            id: String,
            x: f64,
            y: f64,
        }
        let rows: Vec<NodePositionMoveJson> = serde_json::from_str(json)?;
        let moves: Vec<(String, f64, f64)> = rows.into_iter().map(|row| (row.id, row.x, row.y)).collect();
        self.set_node_positions(&moves);
        Ok(())
    }

    pub fn drawable_cull_pad_world(&self) -> f64 {
        16.0 / self.camera.zoom.max(1e-9)
    }

    pub fn stroke_world_step_grid(&self, scene: &mut Scene, color: Color, stroke_px: f64, world_step: f64, min_step_screen: f64) {
        let step = world_step * self.camera.zoom;
        if step < min_step_screen {
            return;
        }
        let stroke = Stroke::new(stroke_px);
        let w = self.width as f64;
        let h = self.height as f64;
        let origin = self.world_to_screen(Point::new(0.0, 0.0));
        let x_off = ((origin.x % step) + step) % step;
        let y_off = ((origin.y % step) + step) % step;
        let mut p = semio_framework_canvas::BezPath::new();
        let mut x = x_off;
        while x <= w {
            p.move_to(Point::new(x, 0.0));
            p.line_to(Point::new(x, h));
            x += step;
        }
        let mut y = y_off;
        while y <= h {
            p.move_to(Point::new(0.0, y));
            p.line_to(Point::new(w, y));
            y += step;
        }
        scene.stroke(&stroke, Affine::IDENTITY, color, None, &p);
    }

    pub fn draw_space_point(&self, world: Point, world_space: bool) -> Point {
        if world_space {
            world
        } else {
            self.world_to_screen(world)
        }
    }

    pub fn draw_space_len(&self, len_world: f64, world_space: bool) -> f64 {
        if world_space {
            len_world.max(1e-9)
        } else {
            (len_world * self.camera.zoom).max(1.0)
        }
    }

    /// 📏️ Screen-pixel edge stroke width (world-clip tiles and post-cache overlay).
    pub fn edge_screen_stroke_width_px(&self, lod: BoardDrawLod) -> f64 {
        match lod {
            BoardDrawLod::Minimap => ui_styling::strokes::EDGE_MINIMAP,
            BoardDrawLod::Overview | BoardDrawLod::Compact => (ui_styling::strokes::EDGE_OVERVIEW).max(ui_styling::strokes::EDGE_BASE * self.camera.zoom),
            _ => 2.0 * self.camera.zoom.max(0.75),
        }
    }

    /// 📏️ Edge stroke in world units so {@link BoardHost.camera_content_affine} yields ~{@link Self::edge_screen_stroke_width_px}.
    pub fn edge_world_stroke_width(&self, lod: BoardDrawLod) -> f64 {
        let screen_px = self.edge_screen_stroke_width_px(lod);
        let z = self.camera.zoom.max(1e-9);
        (screen_px / z).max(1e-3)
    }

    /// 🗺️ Draws baked icon spans at each node's live center. A pending cache retirement keeps the
    /// previous bake; the translation is the whole correction a drag needs.
    pub fn append_baked_node_icons(&self, scene: &mut Scene, cache: &WorldContentCache, cam_aff: Affine) {
        if cache.spans.is_empty() {
            scene.append(&cache.scene, Some(cam_aff));
            return;
        }
        for span in &cache.spans {
            let Some(node) = self.nodes.get(&span.id) else { continue };
            if !node.visible {
                continue;
            }
            let dx = node.x - span.x;
            let dy = node.y - span.y;
            let placed = if dx.abs() <= 1e-9 && dy.abs() <= 1e-9 { cam_aff } else { cam_aff * Affine::IDENTITY.translate((dx, dy)) };
            scene.append_range(&cache.scene, span.start, span.end, Some(placed));
        }
    }

    pub fn plan_wheel(&self, sx: f64, sy: f64, delta_y: f64) -> BoardWheelPlan {
        let mut next = self.camera.clone();
        semio_framework_canvas::camera::wheel_screen(&mut next, &self.viewport(), sx, sy, delta_y);
        BoardWheelPlan { revision: self.interaction_revision, expected: self.camera.clone(), next }
    }

    pub fn begin_delete_property_audit(&mut self, planning: &mut BoardDeletePlanningOperation, kind: BoardDeleteKind, id: &str) {
        if planning.plan.contains(kind, id) {
            return;
        }
        if id.len() > BOARD_POINTER_BYTE_CAPACITY.saturating_sub(usize::from(planning.plan.byte_len)) {
            planning.fault = Some(BoardEventFault::ByteCredits);
            return;
        }
        let properties = match kind {
            BoardDeleteKind::Edge => self.edges.get_mut(id).map(|entity| std::mem::take(&mut entity.properties)),
            BoardDeleteKind::Node => self.nodes.get_mut(id).map(|entity| std::mem::take(&mut entity.properties)),
            BoardDeleteKind::Handle => self.handles.get_mut(id).map(|entity| std::mem::take(&mut entity.properties)),
            BoardDeleteKind::Wire => self.wires.get_mut(id).map(|entity| std::mem::take(&mut entity.properties)),
        };
        let Some(properties) = properties else {
            planning.fault = Some(BoardEventFault::Schema);
            return;
        };
        planning.property_audit = Some(BoardPropertyAudit::new(kind, id.to_owned(), properties));
    }

    pub fn restore_delete_properties(&mut self, kind: BoardDeleteKind, id: &str, properties: semio_framework_graph_manifest::PropertyBag) -> bool {
        match kind {
            BoardDeleteKind::Edge => self.edges.get_mut(id).map(|entity| entity.properties = properties),
            BoardDeleteKind::Node => self.nodes.get_mut(id).map(|entity| entity.properties = properties),
            BoardDeleteKind::Handle => self.handles.get_mut(id).map(|entity| entity.properties = properties),
            BoardDeleteKind::Wire => self.wires.get_mut(id).map(|entity| entity.properties = properties),
        }
        .is_some()
    }

    /// 🔗️ True when any edge uses this handle as `source` or `target` (handle already participates in a link).
    pub fn handle_has_incident_edge(&self, handle_id: &str) -> bool {
        self.edges.values().any(|e| e.source == handle_id || e.target == handle_id)
    }

    pub(super) fn node_has_any_incident_edge(&self, node_id: &str) -> bool {
        self.handles.values().filter(|h| h.node_id == node_id).any(|h| self.handle_has_incident_edge(h.id.as_str()))
    }

    pub(super) fn node_effectively_visible(&self, node_id: &str) -> bool {
        self.nodes.get(node_id).is_some_and(|n| n.visible)
    }

    pub fn node_selectable(&self, node_id: &str) -> bool {
        self.nodes.get(node_id).is_some_and(|n| n.visible && !n.locked)
    }

    pub fn handle_selectable(&self, handle_id: &str) -> bool {
        self.handles.get(handle_id).is_some_and(|h| h.visible && !h.locked && self.node_selectable(h.node_id.as_str()))
    }

    pub fn wire_selectable(&self, wire: &WireData) -> bool {
        if !wire.visible || wire.locked {
            return false;
        }
        if !self.handle_selectable(wire.source.as_str()) {
            return false;
        }
        wire.target.as_ref().is_none_or(|id| self.handle_selectable(id.as_str()))
    }

    pub fn handle_effectively_visible(&self, handle_id: &str) -> bool {
        self.handles.get(handle_id).is_some_and(|h| h.visible && self.node_effectively_visible(h.node_id.as_str()))
    }

    pub fn wire_effectively_visible(&self, wire: &WireData) -> bool {
        wire.visible && self.handle_effectively_visible(wire.source.as_str()) && wire.target.as_ref().is_none_or(|id| self.handle_effectively_visible(id.as_str()))
    }

    /// 💫️ True when the handle may be drawn or hit-tested on the indirect-connect ghost ring (`overview`/`normal` LOD).
    pub fn handle_eligible_indirect_connect_ring(&self, handle_id: &str) -> bool {
        self.handle_selectable(handle_id) && !self.handle_has_incident_edge(handle_id)
    }

    pub fn build_selection_pointer_plan(&self, kind: BoardPointerPlanKind, next: &BTreeSet<String>, anchor: &BTreeSet<String>, gesture: Option<&str>, points: &[Point], screen_points: &[Point]) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
        if next.len() > BOARD_POINTER_ITEM_CAPACITY || anchor.len() > BOARD_POINTER_ITEM_CAPACITY || points.len() != screen_points.len() || points.len() > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardPointerPlanFault::ItemCredits);
        }
        if anchor.iter().map(String::len).try_fold(0usize, usize::checked_add).is_none_or(|bytes| bytes > BOARD_POINTER_BYTE_CAPACITY) {
            return Err(BoardPointerPlanFault::ByteCredits);
        }
        let mut plan = BoardPointerPlan::empty(self.interaction_revision, kind);
        for id in next {
            plan.push_delta(id, 0.0, 0.0)?;
        }
        for (world, screen) in points.iter().copied().zip(screen_points.iter().copied()) {
            plan.push_point(world, screen)?;
        }
        let event = match kind {
            BoardPointerPlanKind::SelectionPreview { .. } => {
                let removed: BTreeSet<String> = anchor.difference(next).cloned().collect();
                BoardOwnedEvent::preselect_sets(next, &removed, gesture)
            }
            BoardPointerPlanKind::SelectionCommit => BoardOwnedEvent::select_set(next, Some(anchor), gesture),
            _ => return Err(BoardPointerPlanFault::Unsupported),
        }
        .map_err(|fault| match fault {
            BoardEventFault::ItemCredits => BoardPointerPlanFault::ItemCredits,
            BoardEventFault::ByteCredits | BoardEventFault::KeyCredits | BoardEventFault::Schema => BoardPointerPlanFault::ByteCredits,
        })?;
        plan.seal_owned_event(&event)?;
        Ok(plan)
    }
}
