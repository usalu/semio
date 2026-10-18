//! 🛑️ Cancel and transform gumball of the normal board host: pointer leave and cancel, area select cancel and the transform gumball geometry, JSON, drag and paint. See [`super`].

use super::*;

impl BoardHost {
    pub fn pointer_leave_screen(&mut self, alt: bool) {
        self.scene.interaction_revision = self.scene.interaction_revision.wrapping_add(1);
        if cancel_region_paint(&mut self.scene, &mut self.gestures) || cancel_region_drag(&mut self.scene, &mut self.focus, &mut self.gestures) {
            set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, None);
            return;
        }
        if self.focus.active_utility == ActiveUtility::Brush {
            brush_update_alt(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox, alt);
            brush_finish_slot(&mut self.scene, &mut self.focus, &mut self.style, &mut self.brush, &mut self.outbox);
            set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, None);
            return;
        }
        if cancel_transform_drag(&mut self.scene, &mut self.gestures) {
            return;
        }
        if matches!(self.focus.interaction, Interaction::None) {
            set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, None);
        }
    }

    pub fn pointer_cancel_screen(&mut self) -> bool {
        begin_pointer_cancel(&mut self.scene, &mut self.commit);
        let cancelled_region = cancel_region_paint(&mut self.scene, &mut self.gestures) | cancel_region_drag(&mut self.scene, &mut self.focus, &mut self.gestures) | cancel_transform_drag(&mut self.scene, &mut self.gestures);
        let cancelled_brush = self.brush.brush_preview.is_some() || self.brush.brush_slot_source_id.is_some();
        self.brush.brush_slot_suggestions_active = false;
        self.brush.brush_slot_source_id = None;
        self.brush.brush_candidates.clear();
        self.brush.brush_candidate_index = 0;
        self.brush.brush_preview = None;
        self.brush.brush_preview_emit_key = None;
        self.brush.brush_candidates_emit_key = None;
        let interaction = std::mem::take(&mut self.focus.interaction);
        let cancelled_interaction = !matches!(interaction, Interaction::None);
        match interaction {
            Interaction::DragNodes { start_positions, gesture, .. } => cancel_node_drag(&mut self.scene, &mut self.focus, &mut self.outbox, start_positions, gesture),
            Interaction::SelectionPending { initial_ids, .. } | Interaction::Selection { initial_ids, .. } => {
                self.focus.selection = initial_ids;
                self.focus.preselect.clear();
                self.focus.preselect_removed.clear();
                self.outbox.last_preselect_emit_sig = None;
                self.outbox.last_select_emit_sig = None;
                sync_selection_flags_to_objects(&mut self.scene, &mut self.focus);
                self.focus.set_selection_screen_preview(None);
                self.scene.bump_content_scene_generation();
            }
            Interaction::LinkAtSourceHandle { .. }
            | Interaction::LinkDragSnap { .. }
            | Interaction::LinkTargetNode { .. }
            | Interaction::ExternalLinkPreview { .. } => {
                self.gestures.link_screen_preview = None;
                self.outbox.link_compat_nodes_emit_key = None;
                self.outbox.link_target_ring_emit_key = None;
                self.scene.bump_content_scene_generation();
            }
            Interaction::Pan { .. } | Interaction::None => {}
        }
        cancelled_region || cancelled_brush || cancelled_interaction
    }

    /// ↩️ Aborts an in‑flight rectangle/lasso drag and restores the selection snapshot from when the gesture began.
    pub fn cancel_area_select(&mut self) -> bool {
        if cancel_region_paint(&mut self.scene, &mut self.gestures) || cancel_region_drag(&mut self.scene, &mut self.focus, &mut self.gestures) {
            return true;
        }
        if cancel_transform_drag(&mut self.scene, &mut self.gestures) {
            return true;
        }
        let reservation = match &self.focus.interaction {
            Interaction::Selection { initial_ids, .. } => {
                let event = BoardOwnedEvent::id_list(BoardEventKind::PreselectCancel, "ids", initial_ids.iter().map(String::as_str));
                let Some(reservation) = self.outbox.reserve_owned_event(event) else {
                    return false;
                };
                Some(reservation)
            }
            _ => None,
        };
        let prev = std::mem::replace(&mut self.focus.interaction, Interaction::None);
        match prev {
            Interaction::DragNodes { start_positions, gesture, .. } => {
                cancel_node_drag(&mut self.scene, &mut self.focus, &mut self.outbox, start_positions, gesture);
                true
            }
            Interaction::SelectionPending { .. } => {
                self.focus.set_selection_screen_preview(None);
                true
            }
            Interaction::Selection { initial_ids, .. } => {
                self.focus.set_selection_screen_preview(None);
                self.focus.preselect.clear();
                self.focus.preselect_removed.clear();
                self.outbox.last_preselect_emit_sig = None;
                self.focus.selection = initial_ids;
                sync_selection_flags_to_objects(&mut self.scene, &mut self.focus);
                self.scene.bump_content_scene_generation();
                self.outbox.last_select_emit_sig = None;
                self.outbox.publish_event_reservation(reservation.expect("selection cancellation reserved one event"));
                true
            }
            other => {
                self.focus.interaction = other;
                false
            }
        }
    }

    /// ⭕️ Pivot and ring radius when the rotate gumball is live: the select utility is
    /// active, the rotate flag is on, and at least one selected node exists to turn.
    pub fn transform_gumball_geometry(&self) -> Option<(Point, f64)> {
        if self.focus.active_utility != ActiveUtility::Select || !self.gestures.transform_flags.rotate_enabled {
            return None;
        }
        let ids = transform_selection_node_ids(&self.scene, &self.focus);
        if ids.is_empty() {
            return None;
        }
        let centers: Vec<Point> = ids.iter().filter_map(|id| self.scene.nodes.get(id)).map(|n| Point::new(n.x, n.y)).collect();
        let pivot = transform_pivot_of(&centers)?;
        let mut corners = Vec::with_capacity(centers.len() * 2);
        for id in &ids {
            let Some(node) = self.scene.nodes.get(id) else { continue };
            let bounds = self.style.node_world_bounds(node, 0.0);
            corners.push(Point::new(bounds.min_x, bounds.min_y));
            corners.push(Point::new(bounds.max_x, bounds.max_y));
        }
        Some((pivot, transform_ring_radius_world(pivot, &corners, self.scene.camera.zoom)))
    }

    /// 🩺️ The gumball's whole probe-visible state: which handles are composed, whether the
    /// ring is drawn, and the live drag's pivot/angle. Read as `data-board-transform-json`.
    pub fn transform_gumball_json(&self) -> String {
        let geometry = self.gestures.transform_drag.as_ref().map(|drag| (drag.pivot, drag.radius_world)).or_else(|| self.transform_gumball_geometry());
        let mut out = String::from("{\"move\":");
        out.push_str(if self.gestures.transform_flags.move_enabled { "true" } else { "false" });
        out.push_str(",\"rotate\":");
        out.push_str(if self.gestures.transform_flags.rotate_enabled { "true" } else { "false" });
        out.push_str(",\"ringVisible\":");
        out.push_str(if geometry.is_some() { "true" } else { "false" });
        out.push_str(",\"dragging\":");
        out.push_str(if self.gestures.transform_drag.is_some() { "true" } else { "false" });
        out.push_str(",\"radians\":");
        out.push_str(&self.gestures.transform_drag.as_ref().map_or(0.0, |drag| drag.radians).to_string());
        if let Some((pivot, radius)) = geometry {
            out.push_str(",\"pivot\":{\"x\":");
            out.push_str(&pivot.x.to_string());
            out.push_str(",\"y\":");
            out.push_str(&pivot.y.to_string());
            out.push_str("},\"radius\":");
            out.push_str(&radius.to_string());
        }
        out.push('}');
        out
    }

    /// 🔄️ Grabs the rotate ring. Asked BEFORE any node/handle/edge hit test, so a ring that
    /// crosses a node still rotates instead of starting a drag on whatever sits under it.
    pub(super) fn try_begin_transform_drag_at(&mut self, world: Point) -> bool {
        let Some((pivot, radius_world)) = self.transform_gumball_geometry() else {
            return false;
        };
        if !transform_ring_hit(pivot, radius_world, self.scene.camera.zoom, world) {
            return false;
        }
        let ids = transform_selection_node_ids(&self.scene, &self.focus);
        let mut start_positions = BTreeMap::new();
        for id in &ids {
            if let Some(node) = self.scene.nodes.get(id) {
                if node.draggable && !node.locked {
                    start_positions.insert(id.clone(), (node.x, node.y));
                }
            }
        }
        if start_positions.is_empty() {
            return false;
        }
        let start_handle_angles = self.scene.handles.iter().filter(|(_, handle)| start_positions.contains_key(&handle.node_id)).map(|(id, handle)| (id.clone(), handle.angle)).collect();
        let gesture = self.outbox.begin_gesture();
        self.gestures.transform_drag = Some(BoardTransformDrag { gesture, pivot, grab: world, radius_world, radians: 0.0, start_positions, start_handle_angles });
        set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, None);
        true
    }

    /// ⭕️ Paints the ring, its pivot dot and the live grab spoke.
    ///
    /// ⭕️ While the gesture is live the ring is the one grabbed at press time: re-deriving it from
    /// the turning nodes would make the band breathe under the cursor for a non-circular selection.
    pub(crate) fn append_transform_gumball_paint(&self, scene: &mut Scene, world_space: bool) {
        let Some((pivot, radius_world)) = self.gestures.transform_drag.as_ref().map(|drag| (drag.pivot, drag.radius_world)).or_else(|| self.transform_gumball_geometry()) else {
            return;
        };
        let center = self.scene.draw_space_point(pivot, world_space);
        let radius = self.scene.draw_space_len(radius_world, world_space);
        let color = if self.gestures.transform_drag.is_some() { self.style.canvas_theme.node_stroke_selected } else { self.style.canvas_theme.node_stroke };
        let mut ring_stroke = Stroke::new(ui_styling::strokes::SELECTION_PREVIEW);
        if self.gestures.transform_drag.is_none() {
            ring_stroke.set_dash_pattern(vec![6.0, 5.0]);
        }
        scene.stroke(&ring_stroke, Affine::IDENTITY, color, None, &Circle::new(center, radius));
        scene.fill(FillRule::NonZero, Affine::IDENTITY, color, None, &Circle::new(center, self.scene.draw_space_len(TRANSFORM_RING_HIT_TOLERANCE_PX / self.scene.camera.zoom.max(1e-9) * 0.25, world_space)));
        if let Some(ref drag) = self.gestures.transform_drag {
            let spoke = rotate_point_about(drag.pivot, Point::new(drag.pivot.x + radius_world, drag.pivot.y), drag.radians);
            let mut path = semio_framework_canvas::BezPath::new();
            path.move_to(center);
            path.line_to(self.scene.draw_space_point(spoke, world_space));
            scene.stroke(&Stroke::new(ui_styling::strokes::SELECTION_PREVIEW), Affine::IDENTITY, color, None, &path);
        }
    }
    //#endregion 🕹️TransformGumball
}
