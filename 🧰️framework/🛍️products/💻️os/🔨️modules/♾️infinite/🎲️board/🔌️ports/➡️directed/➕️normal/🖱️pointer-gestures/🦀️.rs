//! 🖱️ Board host screen pointer gestures, transform gumball, target regions and bounded selection.

use super::*;

// #region board_host
impl BoardHost {
    pub fn commit_pointer(&mut self, plan: &BoardPointerPlan) -> bool {
        if self.interaction_revision != plan.revision {
            return false;
        }
        if !matches!(plan.kind, BoardPointerPlanKind::Idle) {
            return false;
        }
        self.interaction_revision = plan.revision.wrapping_add(1);
        true
    }

    #[cfg(test)]
    fn commit_pointer_legacy_differential(&mut self, plan: &BoardPointerPlan) -> bool {
        match plan.kind {
            BoardPointerPlanKind::Idle => {}
            BoardPointerPlanKind::Pan { camera } => {
                self.camera = Camera { x: camera[0], y: camera[1], zoom: semio_framework_canvas::camera::clamp_zoom(camera[2]) };
            }
            BoardPointerPlanKind::DragMove | BoardPointerPlanKind::FinishDrag => unreachable!("drag plans are committed through the retained pointer authority"),
            BoardPointerPlanKind::FinishPan { camera } => {
                self.camera = Camera { x: camera[0], y: camera[1], zoom: semio_framework_canvas::camera::clamp_zoom(camera[2]) };
                self.interaction = Interaction::None;
            }
            BoardPointerPlanKind::SelectionPreview { start, start_screen } => {
                let grabbed = std::mem::take(&mut self.interaction);
                let initial_ids = match grabbed {
                    Interaction::SelectionPending { initial_ids, .. } | Interaction::Selection { initial_ids, .. } => initial_ids,
                    other => {
                        self.interaction = other;
                        return false;
                    }
                };
                let next: BTreeSet<String> = plan.selection_ids().map(str::to_string).collect();
                self.preselect_removed = initial_ids.difference(&next).cloned().collect();
                self.preselect = next;
                self.last_preselect_emit_sig = None;
                self.sync_selection_flags_to_objects();
                let points = plan.points[..usize::from(plan.point_len)].to_vec();
                let screen_points = plan.screen_points[..usize::from(plan.point_len)].to_vec();
                self.sync_selection_screen_overlay(start_screen, &screen_points);
                self.interaction = Interaction::Selection { initial_ids, points, screen_points, start, start_screen };
            }
            BoardPointerPlanKind::SelectionCommit => {
                if !matches!(self.interaction, Interaction::SelectionPending { .. } | Interaction::Selection { .. }) {
                    return false;
                }
                self.interaction = Interaction::None;
                self.selection = plan.selection_ids().map(str::to_string).collect();
                self.preselect.clear();
                self.preselect_removed.clear();
                self.selection_exit_highlight.clear();
                self.last_select_emit_sig = None;
                self.last_preselect_emit_sig = None;
                self.selection_screen_preview = None;
                self.selection_preview_crossing = false;
                self.sync_selection_flags_to_objects();
                self.bump_content_scene_generation();
            }
            BoardPointerPlanKind::LinkMove { source, target, hover, end_world, activated, start_screen, .. } => {
                if !matches!(self.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. }) {
                    return false;
                }
                let source_id = plan.id(source).to_string();
                let target_id = target.map(|target| plan.id(target).to_string());
                let hover_id = hover.map(|hover| plan.id(hover).to_string());
                let node_ids = self.link_drag_compatible_target_node_ids(&source_id);
                let ring_node_id = activated.then(|| self.link_drag_ring_target_node_id(&source_id, end_world)).flatten().filter(|node_id| self.link_compatible_handle_count_on_node(&source_id, node_id) > 1);
                let ring_handle_ids = ring_node_id.as_deref().map(|node_id| self.link_compatible_handle_ids_on_node(&source_id, node_id)).unwrap_or_default();
                self.link_compat_nodes_emit_key = Some(format!("{}|{}", source_id, node_ids.join(",")));
                self.link_target_ring_emit_key = Some(format!("{}|{}|{}", source_id, ring_node_id.as_deref().unwrap_or(""), ring_handle_ids.join(",")));
                self.hovered_id = hover_id;
                self.hovered_kind = None;
                self.interaction = if activated { Interaction::LinkDragSnap { source_id, target_id, end_world } } else { Interaction::LinkAtSourceHandle { source_id, start_screen } };
                self.bump_content_scene_generation();
            }
            BoardPointerPlanKind::LinkFinish { source, target, edge, target_node, hover, .. } => {
                if !matches!(self.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. } | Interaction::LinkTargetNode { .. }) {
                    return false;
                }
                let source_id = plan.id(source).to_string();
                let target_id = target.map(|target| plan.id(target).to_string());
                let edge_id = edge.map(|edge| plan.id(edge).to_string());
                let target_node_id = target_node.map(|target_node| plan.id(target_node).to_string());
                let hover_id = hover.map(|hover| plan.id(hover).to_string());
                if let (Some(target_id), Some(edge_id)) = (target_id, edge_id) {
                    let Some(source_row) = self.handles.get(&source_id) else {
                        return false;
                    };
                    let Some(target_row) = self.handles.get(&target_id) else {
                        return false;
                    };
                    let edge_kind = self.default_edge_kind_for_created_link(source_row, target_row);
                    self.edges.insert(
                        edge_id.clone(),
                        EdgeData { id: edge_id, source: source_id.clone(), target: target_id, selected: false, visible: true, locked: false, style: None, edge_kind, source_tip: None, target_tip: None, properties: semio_framework_graph_manifest::PropertyBag::new() },
                    );
                }
                if let Some(target_node_id) = target_node_id {
                    let node_ids = self.link_drag_compatible_target_node_ids(&source_id);
                    let ring_handle_ids = self.link_compatible_handle_ids_on_node(&source_id, &target_node_id);
                    self.link_compat_nodes_emit_key = Some(format!("{}|{}", source_id, node_ids.join(",")));
                    self.link_target_ring_emit_key = Some(format!("{}|{}|{}", source_id, target_node_id, ring_handle_ids.join(",")));
                    self.interaction = Interaction::LinkTargetNode { source_id, target_node_id };
                } else {
                    self.link_compat_nodes_emit_key = None;
                    self.link_target_ring_emit_key = None;
                    self.interaction = Interaction::None;
                }
                self.hovered_id = hover_id;
                self.hovered_kind = None;
                self.bump_content_scene_generation();
            }
            BoardPointerPlanKind::LinkRetain { hover } => {
                if !matches!(self.interaction, Interaction::LinkTargetNode { .. } | Interaction::ExternalLinkPreview { .. }) {
                    return false;
                }
                self.hovered_id = hover.map(|hover| plan.id(hover).to_string());
                self.hovered_kind = None;
            }
            BoardPointerPlanKind::Hover { hover } => {
                if !matches!(self.interaction, Interaction::None) {
                    return false;
                }
                self.hovered_id = hover.map(|hover| plan.id(hover).to_string());
                self.hovered_kind = None;
            }
            BoardPointerPlanKind::Brush { source, hover, alt, commit_old } => {
                if self.active_utility != ActiveUtility::Brush || matches!(self.interaction, Interaction::Pan { .. }) {
                    return false;
                }
                let source_id = source.map(|source| plan.id(source).to_string());
                let mut candidates = BrushCandidatePage::default();
                for candidate in plan.deltas[..usize::from(plan.delta_len)].iter().flatten() {
                    if candidates.push(plan.id(candidate.id), candidate.x as usize, 0.0).is_err() {
                        return false;
                    }
                }
                if commit_old {
                    self.brush_placement_serial = self.brush_placement_serial.wrapping_add(1);
                }
                self.brush_alt_pressed = alt;
                self.brush_slot_suggestions_active = false;
                self.brush_slot_source_id = source_id.clone();
                self.brush_candidates = candidates;
                self.brush_candidate_index = 0;
                let offset = if alt { self.suggestion_offset } else { 0.0 };
                self.brush_preview = source_id.as_deref().and_then(|source_id| self.brush_candidates.first().and_then(|candidate| self.brush_build_preview_with_offset(source_id, candidate, offset)));
                self.brush_preview_emit_key = None;
                self.brush_candidates_emit_key = None;
                self.hovered_id = hover.map(|hover| plan.id(hover).to_string());
                self.hovered_kind = None;
                self.interaction = Interaction::None;
                self.bump_content_scene_generation();
            }
        }
        self.interaction_revision = plan.revision.wrapping_add(1);
        true
    }

    /// 🖍️ The area brush owns the whole pointer while it is armed: a press anchors a region
    /// rectangle instead of picking, so the same gesture can never also start a marquee.
    ///
    /// 🎯️ LAST of all the pick paths: a region is the board's backdrop, so it is only grabbed
    /// once the nodes, handles, edges and the rotate ring have all missed.
    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.set_selection_screen_preview(None);
        let screen = Point::new(sx, sy);
        let world = self.screen_to_world(screen);
        if self.active_utility == ActiveUtility::AreaBrush {
            if button == 1 {
                self.interaction = Interaction::Pan { origin: self.camera.clone(), start_screen: screen };
            } else if button == 0 {
                self.begin_region_paint(world);
            }
            return;
        }
        if self.active_utility == ActiveUtility::Brush {
            if button == 1 {
                self.interaction = Interaction::Pan { origin: self.camera.clone(), start_screen: screen };
            }
            return;
        }
        let hit = self.resolve_hit_world(world).or_else(|| self.resolve_overview_free_link_handle_pointer_world(world));
        if let Interaction::LinkTargetNode { source_id, target_node_id } = self.interaction.clone() {
            self.interaction = Interaction::None;
            self.clear_link_gesture_events();
            if button == 0 {
                if let Some(th) = self.node_sole_free_link_compatible_handle(&source_id, &target_node_id) {
                    if hit.as_deref() == Some(target_node_id.as_str()) || hit.as_deref() == Some(th.as_str()) {
                        self.try_commit_link_edge(&source_id, &th, Some(BoardEventKind::IndirectConnect));
                        self.update_hover_from_world(world);
                        return;
                    }
                }
                if let Some(hid) = hit.as_ref().filter(|id| self.handles.get(*id).is_some_and(|h| h.node_id == target_node_id) && self.handle_eligible_link_target_ring(id.as_str(), source_id.as_str())) {
                    self.try_commit_link_edge(&source_id, hid, Some(BoardEventKind::IndirectConnect));
                    self.update_hover_from_world(world);
                    return;
                }
            }
            self.update_hover_from_world(world);
            return;
        }
        if let Interaction::ExternalLinkPreview { source_id, ring_node_id, ring_handle_ids, .. } = self.interaction.clone() {
            if button == 0 {
                if let Some(target_node_id) = ring_node_id {
                    if let Some(th) = self.node_sole_free_link_compatible_handle(&source_id, &target_node_id) {
                        if hit.as_deref() == Some(target_node_id.as_str()) || hit.as_deref() == Some(th.as_str()) {
                            self.interaction = Interaction::None;
                            self.clear_link_gesture_events();
                            self.try_commit_link_edge(&source_id, &th, Some(BoardEventKind::IndirectConnect));
                            self.update_hover_from_world(world);
                            return;
                        }
                    }
                }
                if let Some(hid) = hit.as_ref().filter(|id| ring_handle_ids.iter().any(|rh| rh == *id)) {
                    self.interaction = Interaction::None;
                    self.clear_link_gesture_events();
                    self.try_commit_link_edge(&source_id, hid, Some(BoardEventKind::IndirectConnect));
                    self.update_hover_from_world(world);
                    return;
                }
            }
            self.update_hover_from_world(world);
            return;
        }
        let merge_from_modifiers = ctrl_or_meta || shift;
        let pick_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
        if button == 0 && !merge_from_modifiers && self.try_begin_transform_drag_at(world) {
            return;
        }
        if button == 0 && !merge_from_modifiers && self.try_begin_bounded_selection_drag_at(world) {
            return;
        }
        if button == 1 {
            self.interaction = Interaction::Pan { origin: self.camera.clone(), start_screen: screen };
            return;
        }
        if let Some(ref hid) = hit {
            if let Some(node) = self.nodes.get(hid) {
                if node.draggable && !node.locked {
                    let nid = hid.clone();
                    let nx = node.x;
                    let ny = node.y;
                    let mut gesture = self.begin_gesture();
                    let members_before: Vec<String> = self.selection.iter().filter(|id| self.nodes.get(*id).is_some_and(|n| n.draggable)).cloned().collect();
                    let drag_group_before = members_before.contains(&nid) && members_before.len() > 1;
                    let force_pick_merge = (pick_mode == "replace" && !drag_group_before) || pick_mode == "subtractive" || (pick_mode == "invertive" && merge_from_modifiers);
                    if !drag_group_before || force_pick_merge {
                        let next = merge_pick_into_selection(&self.selection, &nid, pick_mode.as_str());
                        let ids: Vec<_> = next.iter().cloned().collect();
                        self.stage_selection_ids(&mut gesture, &ids, merge_from_modifiers.then_some(pick_mode.as_str()));
                    }
                    let members: Vec<String> = self.selection.iter().filter(|id| self.nodes.get(*id).is_some_and(|n| n.draggable && !n.locked)).cloned().collect();
                    let drag_group = members.contains(&nid) && members.len() > 1;
                    let mut start_positions = BTreeMap::new();
                    for id in if drag_group { members.as_slice() } else { std::slice::from_ref(&nid) } {
                        if let Some(n) = self.nodes.get(id) {
                            start_positions.insert(id.clone(), (n.x, n.y));
                        }
                    }
                    self.interaction = Interaction::DragNodes { primary_id: nid, offset: world - Point::new(nx, ny), start_positions, proximity_pair: None, gesture, delta: Vec2::ZERO };
                    self.set_hovered_id(hit);
                    return;
                }
            }
        }
        if let Some(ref hid) = hit {
            if button == 0 && self.handles.contains_key(hid) && !self.handle_has_incident_edge(hid.as_str()) {
                let next = merge_pick_into_selection(&self.selection, hid, pick_mode.as_str());
                let ids: Vec<_> = next.iter().cloned().collect();
                let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
                self.set_selection_ids_gestured(&ids, gesture);
                self.interaction = Interaction::LinkAtSourceHandle { source_id: hid.clone(), start_screen: screen };
                self.set_hovered_id(Some(hid.clone()));
                return;
            }
        }
        if hit.is_none() && button == 0 && self.try_begin_region_drag_at(world) {
            return;
        }
        if hit.is_none() && button == 0 {
            self.interaction = Interaction::SelectionPending { initial_ids: self.selection.clone(), start: world, start_screen: screen };
            self.set_hovered_id(None);
            return;
        }
        self.interaction = Interaction::None;
        if let Some(id) = hit {
            let next = merge_pick_into_selection(&self.selection, &id, pick_mode.as_str());
            let ids: Vec<_> = next.iter().cloned().collect();
            let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
            self.set_selection_ids_gestured(&ids, gesture);
            self.set_hovered_id(Some(id));
        } else {
            let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
            self.set_selection_ids_gestured(&[], gesture);
            self.set_hovered_id(None);
        }
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        let screen = Point::new(sx, sy);
        let world = self.screen_to_world(screen);
        if self.active_utility == ActiveUtility::AreaBrush {
            if let Interaction::Pan { origin, start_screen } = self.interaction.clone() {
                let delta = screen - start_screen;
                self.set_camera(origin.x - delta.x / origin.zoom, origin.y - delta.y / origin.zoom, origin.zoom);
                self.interaction = Interaction::Pan { origin, start_screen };
                return;
            }
            self.update_region_paint(world);
            return;
        }
        if self.region_drag.is_some() {
            self.update_region_drag(world);
            return;
        }
        if self.active_utility == ActiveUtility::Brush {
            self.brush_update_alt(alt);
            match std::mem::replace(&mut self.interaction, Interaction::None) {
                Interaction::Pan { origin, start_screen } => {
                    let delta = screen - start_screen;
                    let nx = origin.x - delta.x / origin.zoom;
                    let ny = origin.y - delta.y / origin.zoom;
                    self.set_camera(nx, ny, origin.zoom);
                    self.interaction = Interaction::Pan { origin, start_screen };
                }
                _ => {
                    self.interaction = Interaction::None;
                    self.brush_pointer_move(world);
                }
            }
            return;
        }
        if self.transform_drag.is_some() {
            self.update_transform_drag(world, self.grid_snap_enabled);
            return;
        }
        match std::mem::replace(&mut self.interaction, Interaction::None) {
            Interaction::DragNodes { primary_id, offset, start_positions, proximity_pair: retained_proximity_pair, gesture, delta: retained_delta } => {
                let Some(delta) = self.drag_delta(world, &primary_id, offset, &start_positions) else {
                    self.interaction = Interaction::DragNodes { primary_id, offset, start_positions, proximity_pair: retained_proximity_pair, gesture, delta: retained_delta };
                    return;
                };
                let mut event_count = 0usize;
                let mut event_bytes = 0usize;
                for (id, (ox0, oy0)) in &start_positions {
                    if self.nodes.contains_key(id) {
                        event_count += 1;
                        let Some(total) = board_node_move_owned_bytes(id, ox0 + delta.x, oy0 + delta.y).and_then(|bytes| event_bytes.checked_add(bytes)) else {
                            self.event_schema_fault = true;
                            self.interaction = Interaction::DragNodes { primary_id, offset, start_positions, proximity_pair: retained_proximity_pair, gesture, delta: retained_delta };
                            return;
                        };
                        event_bytes = total;
                    }
                }
                if self.events.reserve(event_count, event_bytes).is_err() {
                    self.interaction = Interaction::DragNodes { primary_id, offset, start_positions, proximity_pair: retained_proximity_pair, gesture, delta: retained_delta };
                    return;
                }
                let mut geometry_changed = false;
                for (id, (ox0, oy0)) in &start_positions {
                    if let Some(n) = self.nodes.get_mut(id) {
                        let mx = ox0 + delta.x;
                        let my = oy0 + delta.y;
                        if (n.x - mx).abs() > 1e-9 || (n.y - my).abs() > 1e-9 {
                            geometry_changed = true;
                        }
                        n.x = mx;
                        n.y = my;
                        let event = BoardOwnedEvent::node_move(id, mx, my).expect("node move event was exactly preflighted");
                        self.events.push(event).expect("node move event credits were reserved before mutation");
                    }
                }
                if geometry_changed {
                    self.bump_content_scene_generation();
                }
                let proximity_pair = if start_positions.len() == 1 { self.node_drag_proximity_handle_pair(primary_id.as_str()) } else { None };
                self.interaction = Interaction::DragNodes { primary_id, offset, start_positions, proximity_pair, gesture, delta };
            }
            Interaction::Pan { origin, start_screen } => {
                let delta = screen - start_screen;
                let nx = origin.x - delta.x / origin.zoom;
                let ny = origin.y - delta.y / origin.zoom;
                self.set_camera(nx, ny, origin.zoom);
                self.interaction = Interaction::Pan { origin, start_screen };
            }
            Interaction::SelectionPending { initial_ids, start, start_screen } => {
                if distance_between(start_screen, screen) < SELECTION_CLICK_MAX_DISTANCE_PX {
                    self.interaction = Interaction::SelectionPending { initial_ids, start, start_screen };
                } else {
                    let points = vec![start, world];
                    let screen_points = vec![start_screen, screen];
                    let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                    let next = self.resolve_area_selection_with_initial(&initial_ids, start, &points, merge_mode.as_str());
                    let ids: Vec<_> = next.iter().cloned().collect();
                    let merge_from_modifiers = ctrl_or_meta || shift;
                    let gesture = merge_from_modifiers.then_some(merge_mode.as_str());
                    self.apply_area_preselect(&initial_ids, &ids, gesture);
                    self.sync_selection_screen_overlay(start_screen, &screen_points);
                    self.interaction = Interaction::Selection { initial_ids, points, screen_points, start, start_screen };
                }
            }
            Interaction::Selection { mut points, mut screen_points, start, initial_ids, start_screen } => {
                let last_screen = screen_points.last().copied().unwrap_or(start_screen);
                let add_point = self.selection_options.method == "lasso" || distance_between(screen, last_screen) >= SELECTION_LASSO_MIN_POINT_DISTANCE_PX;
                if add_point {
                    points.push(world);
                    screen_points.push(screen);
                } else if !points.is_empty() {
                    let last = points.len() - 1;
                    points[last] = world;
                    let ls = screen_points.len() - 1;
                    screen_points[ls] = screen;
                }
                let initial = initial_ids.clone();
                let pts = points.clone();
                let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                let next = self.resolve_area_selection_with_initial(&initial, start, &pts, merge_mode.as_str());
                let ids: Vec<_> = next.iter().cloned().collect();
                let merge_from_modifiers = ctrl_or_meta || shift;
                let gesture = merge_from_modifiers.then_some(merge_mode.as_str());
                self.apply_area_preselect(&initial, &ids, gesture);
                self.sync_selection_screen_overlay(start_screen, &screen_points);
                self.interaction = Interaction::Selection { initial_ids, points, screen_points, start, start_screen };
            }
            Interaction::LinkAtSourceHandle { source_id, start_screen } => {
                if distance_between(screen, start_screen) >= LINK_DRAG_MIN_DISTANCE_PX {
                    let optional_target_handle_id = self.nearest_link_snap_handle_world(&source_id, world);
                    self.apply_link_drag_snap_hover(&source_id, world, optional_target_handle_id.as_deref());
                    self.interaction = Interaction::LinkDragSnap { source_id: source_id.clone(), target_id: optional_target_handle_id, end_world: world };
                    self.sync_link_gesture_events();
                } else {
                    self.interaction = Interaction::LinkAtSourceHandle { source_id, start_screen };
                    self.update_hover_from_world(world);
                }
            }
            Interaction::LinkDragSnap { source_id, .. } => {
                let optional_target_handle_id = self.nearest_link_snap_handle_world(&source_id, world);
                self.apply_link_drag_snap_hover(&source_id, world, optional_target_handle_id.as_deref());
                self.interaction = Interaction::LinkDragSnap { source_id: source_id.clone(), target_id: optional_target_handle_id, end_world: world };
                self.sync_link_gesture_events();
            }
            Interaction::LinkTargetNode { source_id, target_node_id } => {
                self.interaction = Interaction::LinkTargetNode { source_id, target_node_id };
                self.update_hover_from_world(world);
            }
            Interaction::ExternalLinkPreview { source_id, end_world, compatible_node_ids, ring_node_id, ring_handle_ids } => {
                self.interaction = Interaction::ExternalLinkPreview { source_id, end_world, compatible_node_ids, ring_node_id, ring_handle_ids };
                self.update_hover_from_world(world);
            }
            Interaction::None => {
                self.interaction = Interaction::None;
                self.update_hover_from_world(world);
            }
        }
    }

    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        let screen = Point::new(sx, sy);
        let world = self.screen_to_world(screen);
        if self.active_utility == ActiveUtility::AreaBrush {
            if matches!(self.interaction, Interaction::Pan { .. }) {
                self.interaction = Interaction::None;
            }
            self.commit_region_paint();
            return;
        }
        if self.commit_region_drag() {
            self.update_hover_from_world(world);
            return;
        }
        if self.active_utility == ActiveUtility::Brush {
            self.brush_update_alt(alt);
            if matches!(self.interaction, Interaction::Pan { .. }) {
                self.interaction = Interaction::None;
            }
            self.brush_finish_slot();
            self.set_hovered_id(None);
            return;
        }
        if self.commit_transform_drag() {
            self.update_hover_from_world(world);
            return;
        }
        let grabbed = std::mem::take(&mut self.interaction);
        match grabbed {
            Interaction::LinkDragSnap { source_id, target_id, .. } => {
                if let Some(ref target_handle_id) = target_id {
                    if self.link_snap_commit_proximity_ok(target_handle_id, world) && self.try_commit_link_edge(&source_id, target_handle_id, Some(BoardEventKind::ProximityConnect)) {
                        self.interaction = Interaction::None;
                        self.clear_link_gesture_events();
                        self.update_hover_from_world(world);
                        return;
                    }
                }
                if let Some(target_node_id) = self.resolve_node_hit_world(world) {
                    let source_node_id = self.handles.get(&source_id).map(|h| h.node_id.clone());
                    if source_node_id.as_deref() != Some(target_node_id.as_str()) {
                        if let Some(sole_target) = self.node_sole_free_link_compatible_handle(source_id.as_str(), target_node_id.as_str()) {
                            self.try_commit_link_edge(&source_id, &sole_target, Some(BoardEventKind::IndirectConnect));
                            self.clear_link_gesture_events();
                        } else {
                            self.interaction = Interaction::LinkTargetNode { source_id, target_node_id: target_node_id.clone() };
                            self.set_hovered_id(Some(target_node_id));
                            self.sync_link_gesture_events();
                        }
                        self.update_hover_from_world(world);
                        return;
                    }
                }
                self.interaction = Interaction::None;
                self.clear_link_gesture_events();
                self.update_hover_from_world(world);
            }
            Interaction::LinkAtSourceHandle { .. } => {
                self.interaction = Interaction::None;
                self.clear_link_gesture_events();
                self.update_hover_from_world(world);
            }
            Interaction::DragNodes { start_positions, proximity_pair, gesture, delta, .. } => {
                let record = Self::drag_gesture_record(&gesture, &start_positions, delta, proximity_pair.as_ref());
                self.publish_gesture_release(&gesture, record);
                self.interaction = Interaction::None;
                self.update_hover_from_world(world);
            }
            Interaction::SelectionPending { initial_ids, start, start_screen } => {
                let _ = (start, start_screen);
                let merge_from_modifiers = ctrl_or_meta || shift;
                if !merge_from_modifiers {
                    self.clear_selection_on_background_click();
                } else {
                    let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                    let gesture = Some(merge_mode.as_str());
                    let next = self.resolve_area_selection_with_initial(&initial_ids, start, &[start], merge_mode.as_str());
                    let ids: Vec<_> = next.iter().cloned().collect();
                    self.set_selection_ids_gestured(&ids, gesture);
                }
                self.set_selection_screen_preview(None);
                self.update_hover_from_world(world);
            }
            Interaction::Selection { mut points, mut screen_points, start, initial_ids, start_screen } => {
                points.push(world);
                screen_points.push(screen);
                let end_screen = screen_points.last().copied().unwrap_or(start_screen);
                let click_only = distance_between(start_screen, end_screen) < SELECTION_CLICK_MAX_DISTANCE_PX;
                let merge_from_modifiers = ctrl_or_meta || shift;
                let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                let gesture = merge_from_modifiers.then_some(merge_mode.as_str());
                if click_only {
                    self.commit_area_select_from_initial(&initial_ids, &[], gesture);
                } else {
                    let next = self.resolve_area_selection_with_initial(&initial_ids, start, &points, merge_mode.as_str());
                    let ids: Vec<_> = next.iter().cloned().collect();
                    self.commit_area_select_from_initial(&initial_ids, &ids, gesture);
                }
                self.set_selection_screen_preview(None);
                self.update_hover_from_world(world);
            }
            Interaction::ExternalLinkPreview { .. } => {
                self.interaction = grabbed;
                self.update_hover_from_world(world);
            }
            _ => {
                self.interaction = Interaction::None;
                self.update_hover_from_world(world);
            }
        }
    }

    pub fn pointer_leave_screen(&mut self, alt: bool) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        if self.cancel_region_paint() || self.cancel_region_drag() {
            self.set_hovered_id(None);
            return;
        }
        if self.active_utility == ActiveUtility::Brush {
            self.brush_update_alt(alt);
            self.brush_finish_slot();
            self.set_hovered_id(None);
            return;
        }
        if self.cancel_transform_drag() {
            return;
        }
        if matches!(self.interaction, Interaction::None) {
            self.set_hovered_id(None);
        }
    }

    pub fn pointer_cancel_screen(&mut self) -> bool {
        self.begin_pointer_cancel();
        let cancelled_region = self.cancel_region_paint() | self.cancel_region_drag() | self.cancel_transform_drag();
        let cancelled_brush = self.brush_preview.is_some() || self.brush_slot_source_id.is_some();
        self.brush_slot_suggestions_active = false;
        self.brush_slot_source_id = None;
        self.brush_candidates.clear();
        self.brush_candidate_index = 0;
        self.brush_preview = None;
        self.brush_preview_emit_key = None;
        self.brush_candidates_emit_key = None;
        let interaction = std::mem::take(&mut self.interaction);
        let cancelled_interaction = !matches!(interaction, Interaction::None);
        match interaction {
            Interaction::DragNodes { start_positions, gesture, .. } => self.cancel_node_drag(start_positions, gesture),
            Interaction::SelectionPending { initial_ids, .. } | Interaction::Selection { initial_ids, .. } => {
                self.selection = initial_ids;
                self.preselect.clear();
                self.preselect_removed.clear();
                self.last_preselect_emit_sig = None;
                self.last_select_emit_sig = None;
                self.sync_selection_flags_to_objects();
                self.set_selection_screen_preview(None);
                self.bump_content_scene_generation();
            }
            Interaction::LinkAtSourceHandle { .. }
            | Interaction::LinkDragSnap { .. }
            | Interaction::LinkTargetNode { .. }
            | Interaction::ExternalLinkPreview { .. } => {
                self.link_screen_preview = None;
                self.link_compat_nodes_emit_key = None;
                self.link_target_ring_emit_key = None;
                self.bump_content_scene_generation();
            }
            Interaction::Pan { .. } | Interaction::None => {}
        }
        cancelled_region || cancelled_brush || cancelled_interaction
    }

    /// ↩️ Aborts an in‑flight rectangle/lasso drag and restores the selection snapshot from when the gesture began.
    pub fn cancel_area_select(&mut self) -> bool {
        if self.cancel_region_paint() || self.cancel_region_drag() {
            return true;
        }
        if self.cancel_transform_drag() {
            return true;
        }
        let reservation = match &self.interaction {
            Interaction::Selection { initial_ids, .. } => {
                let event = BoardOwnedEvent::id_list(BoardEventKind::PreselectCancel, "ids", initial_ids.iter().map(String::as_str));
                let Some(reservation) = self.reserve_owned_event(event) else {
                    return false;
                };
                Some(reservation)
            }
            _ => None,
        };
        let prev = std::mem::replace(&mut self.interaction, Interaction::None);
        match prev {
            Interaction::DragNodes { start_positions, gesture, .. } => {
                self.cancel_node_drag(start_positions, gesture);
                true
            }
            Interaction::SelectionPending { .. } => {
                self.set_selection_screen_preview(None);
                true
            }
            Interaction::Selection { initial_ids, .. } => {
                self.set_selection_screen_preview(None);
                self.preselect.clear();
                self.preselect_removed.clear();
                self.last_preselect_emit_sig = None;
                self.selection = initial_ids;
                self.sync_selection_flags_to_objects();
                self.bump_content_scene_generation();
                self.last_select_emit_sig = None;
                self.publish_event_reservation(reservation.expect("selection cancellation reserved one event"));
                true
            }
            other => {
                self.interaction = other;
                false
            }
        }
    }

    pub(super) fn node_world_bounds(&self, n: &NodeData, pad: f64) -> WorldBox {
        let raw = match n.shape {
            NodeShape::Rectangle => {
                let hw = self.scaled_node_width(n) / 2.0;
                let hh = self.scaled_node_height(n) / 2.0;
                WorldBox { min_x: n.x - hw, min_y: n.y - hh, max_x: n.x + hw, max_y: n.y + hh }
            }
            NodeShape::Circle => WorldBox { min_x: n.x - self.scaled_node_radius(n), min_y: n.y - self.scaled_node_radius(n), max_x: n.x + self.scaled_node_radius(n), max_y: n.y + self.scaled_node_radius(n) },
        };
        inflate_world_box(raw, pad)
    }

    fn selection_draggable_node_members(&self) -> Vec<String> {
        self.selection.iter().filter(|id| self.nodes.get(*id).is_some_and(|n| n.draggable)).cloned().collect()
    }

    fn selection_union_bounds_world(&self) -> Option<WorldBox> {
        let mut corners: Vec<Point> = Vec::new();
        for id in &self.selection {
            let Some(n) = self.nodes.get(id) else {
                continue;
            };
            let b = self.node_world_bounds(n, 0.0);
            corners.push(Point::new(b.min_x, b.min_y));
            corners.push(Point::new(b.max_x, b.max_y));
        }
        world_box_from_points(&corners)
    }

    //#region 🕹️TransformGumball
    /// 🕹️ Composes which gumball handles the select utility offers. `move` is the native
    /// node drag (always available while it is on), `rotate` draws and arms the ring.
    pub fn set_transform_flags(&mut self, move_enabled: bool, rotate_enabled: bool) {
        let next = TransformGumballFlags { move_enabled, rotate_enabled };
        if self.transform_flags == next {
            return;
        }
        if !rotate_enabled {
            self.cancel_transform_drag();
        }
        self.transform_flags = next;
        self.bump_content_scene_generation();
    }

    pub fn transform_flags(&self) -> TransformGumballFlags {
        self.transform_flags
    }

    /// 📐️ Every selected node id in engine order — the rotate gesture's members, pivot
    /// contributors and event ids all come from this one list.
    fn transform_selection_node_ids(&self) -> Vec<String> {
        self.selection.iter().filter(|id| self.nodes.contains_key(*id)).cloned().collect()
    }

    /// ⭕️ Pivot and ring radius when the rotate gumball is live: the select utility is
    /// active, the rotate flag is on, and at least one selected node exists to turn.
    pub fn transform_gumball_geometry(&self) -> Option<(Point, f64)> {
        if self.active_utility != ActiveUtility::Select || !self.transform_flags.rotate_enabled {
            return None;
        }
        let ids = self.transform_selection_node_ids();
        if ids.is_empty() {
            return None;
        }
        let centers: Vec<Point> = ids.iter().filter_map(|id| self.nodes.get(id)).map(|n| Point::new(n.x, n.y)).collect();
        let pivot = transform_pivot_of(&centers)?;
        let mut corners = Vec::with_capacity(centers.len() * 2);
        for id in &ids {
            let Some(node) = self.nodes.get(id) else { continue };
            let bounds = self.node_world_bounds(node, 0.0);
            corners.push(Point::new(bounds.min_x, bounds.min_y));
            corners.push(Point::new(bounds.max_x, bounds.max_y));
        }
        Some((pivot, transform_ring_radius_world(pivot, &corners, self.camera.zoom)))
    }

    /// 🩺️ The gumball's whole probe-visible state: which handles are composed, whether the
    /// ring is drawn, and the live drag's pivot/angle. Read as `data-board-transform-json`.
    pub fn transform_gumball_json(&self) -> String {
        let geometry = self.transform_drag.as_ref().map(|drag| (drag.pivot, drag.radius_world)).or_else(|| self.transform_gumball_geometry());
        let mut out = String::from("{\"move\":");
        out.push_str(if self.transform_flags.move_enabled { "true" } else { "false" });
        out.push_str(",\"rotate\":");
        out.push_str(if self.transform_flags.rotate_enabled { "true" } else { "false" });
        out.push_str(",\"ringVisible\":");
        out.push_str(if geometry.is_some() { "true" } else { "false" });
        out.push_str(",\"dragging\":");
        out.push_str(if self.transform_drag.is_some() { "true" } else { "false" });
        out.push_str(",\"radians\":");
        out.push_str(&self.transform_drag.as_ref().map_or(0.0, |drag| drag.radians).to_string());
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
    fn try_begin_transform_drag_at(&mut self, world: Point) -> bool {
        let Some((pivot, radius_world)) = self.transform_gumball_geometry() else {
            return false;
        };
        if !transform_ring_hit(pivot, radius_world, self.camera.zoom, world) {
            return false;
        }
        let ids = self.transform_selection_node_ids();
        let mut start_positions = BTreeMap::new();
        for id in &ids {
            if let Some(node) = self.nodes.get(id) {
                if node.draggable && !node.locked {
                    start_positions.insert(id.clone(), (node.x, node.y));
                }
            }
        }
        if start_positions.is_empty() {
            return false;
        }
        let start_handle_angles = self.handles.iter().filter(|(_, handle)| start_positions.contains_key(&handle.node_id)).map(|(id, handle)| (id.clone(), handle.angle)).collect();
        let gesture = self.begin_gesture();
        self.transform_drag = Some(BoardTransformDrag { gesture, pivot, grab: world, radius_world, radians: 0.0, start_positions, start_handle_angles });
        self.set_hovered_id(None);
        true
    }

    /// 👁️ Re-derives the whole preview from the grab-time snapshot: positions orbit the
    /// pivot and every handle angle turns with its node, exactly like the guest reducer the
    /// release commits to. Emits nothing — a rotate is ONE document edit, on release.
    ///
    /// 🪞️ A preview frame is expendable: a refused reservation drops the row instead of
    /// claiming overflow credit, so a fast drag can never fault the event terminal.
    fn update_transform_drag(&mut self, world: Point, snap: bool) {
        let Some(mut drag) = self.transform_drag.take() else {
            return;
        };
        drag.radians = snap_transform_angle(transform_ring_angle_delta(drag.pivot, drag.grab, world), snap);
        for (id, (x0, y0)) in &drag.start_positions {
            let rotated = rotate_point_about(drag.pivot, Point::new(*x0, *y0), drag.radians);
            if let Some(node) = self.nodes.get_mut(id) {
                node.x = rotated.x;
                node.y = rotated.y;
            }
        }
        for (id, angle0) in &drag.start_handle_angles {
            if let Some(handle) = self.handles.get_mut(id) {
                handle.angle = angle0 + drag.radians;
            }
        }
        let preview = BoardOwnedEvent::transform_preview(drag.start_positions.keys().filter_map(|id| self.nodes.get(id).map(|node| (id.as_str(), node.x, node.y))));
        self.transform_drag = Some(drag);
        self.bump_content_scene_generation();
        if let Ok(event) = preview {
            if let Ok(reservation) = self.events.reserve_event(event) {
                self.publish_event_reservation(reservation);
            }
        }
    }

    /// 🏁️ Ends the gesture with ONE `rotate` gesture record carrying the turned ids, the pivot and the
    /// angle — however many preview frames the drag painted. A zero-angle release (a click on the ring)
    /// commits nothing.
    fn commit_transform_drag(&mut self) -> bool {
        let Some(drag) = self.transform_drag.take() else {
            return false;
        };
        let record = (drag.radians != 0.0).then(|| BoardOwnedEvent::gesture(&drag.gesture.id, drag.start_positions.keys().map(String::as_str), BoardGestureMotion::Rotate { pivot: drag.pivot, angle: drag.radians }, &[]));
        self.publish_gesture_release(&drag.gesture, record);
        true
    }

    /// ↩️ Restores the exact pre-gesture geometry — Escape, pointer-leave and switching the
    /// rotate flag off all abandon a live ring drag without touching the document.
    fn cancel_transform_drag(&mut self) -> bool {
        let Some(drag) = self.transform_drag.take() else {
            return false;
        };
        for (id, (x0, y0)) in &drag.start_positions {
            if let Some(node) = self.nodes.get_mut(id) {
                node.x = *x0;
                node.y = *y0;
            }
        }
        for (id, angle0) in &drag.start_handle_angles {
            if let Some(handle) = self.handles.get_mut(id) {
                handle.angle = *angle0;
            }
        }
        self.bump_content_scene_generation();
        true
    }

    /// ⭕️ Paints the ring, its pivot dot and the live grab spoke.
    ///
    /// ⭕️ While the gesture is live the ring is the one grabbed at press time: re-deriving it from
    /// the turning nodes would make the band breathe under the cursor for a non-circular selection.
    pub(super) fn append_transform_gumball_paint(&self, scene: &mut Scene, world_space: bool) {
        let Some((pivot, radius_world)) = self.transform_drag.as_ref().map(|drag| (drag.pivot, drag.radius_world)).or_else(|| self.transform_gumball_geometry()) else {
            return;
        };
        let center = self.draw_space_point(pivot, world_space);
        let radius = self.draw_space_len(radius_world, world_space);
        let color = if self.transform_drag.is_some() { self.canvas_theme.node_stroke_selected } else { self.canvas_theme.node_stroke };
        let mut ring_stroke = Stroke::new(ui_styling::strokes::SELECTION_PREVIEW);
        if self.transform_drag.is_none() {
            ring_stroke.set_dash_pattern(vec![6.0, 5.0]);
        }
        scene.stroke(&ring_stroke, Affine::IDENTITY, color, None, &Circle::new(center, radius));
        scene.fill(FillRule::NonZero, Affine::IDENTITY, color, None, &Circle::new(center, self.draw_space_len(TRANSFORM_RING_HIT_TOLERANCE_PX / self.camera.zoom.max(1e-9) * 0.25, world_space)));
        if let Some(ref drag) = self.transform_drag {
            let spoke = rotate_point_about(drag.pivot, Point::new(drag.pivot.x + radius_world, drag.pivot.y), drag.radians);
            let mut path = semio_framework_canvas::BezPath::new();
            path.move_to(center);
            path.line_to(self.draw_space_point(spoke, world_space));
            scene.stroke(&Stroke::new(ui_styling::strokes::SELECTION_PREVIEW), Affine::IDENTITY, color, None, &path);
        }
    }
    //#endregion 🕹️TransformGumball

    //#region 🎯️TargetRegions
    /// 🖍️ World width/height an area-brush CLICK paints; a non-finite or non-positive axis
    /// keeps the previous extent rather than minting a zero-area region no fill could ever satisfy.
    pub fn set_area_brush_extent(&mut self, width: f64, height: f64) {
        let (mut w, mut h) = self.area_brush_extent;
        if width.is_finite() && width > 0.0 {
            w = width;
        }
        if height.is_finite() && height > 0.0 {
            h = height;
        }
        if (w - self.area_brush_extent.0).abs() < 1e-9 && (h - self.area_brush_extent.1).abs() < 1e-9 {
            return;
        }
        self.area_brush_extent = (w, h);
    }

    pub fn area_brush_extent(&self) -> (f64, f64) {
        self.area_brush_extent
    }

    /// 🧲️ The world step every region gesture quantizes to, or `None` while the grid-snap
    /// modifier is off — one definition shared by paint, move and resize.
    pub(super) fn region_snap_step(&self) -> Option<f64> {
        if !self.grid_snap_enabled {
            return None;
        }
        self.lod_visible_grid_snap_step_world()
    }

    fn snap_region_point(&self, point: Point) -> Point {
        let step = self.region_snap_step();
        Point::new(snap_region_scalar(point.x, step), snap_region_scalar(point.y, step))
    }

    /// 📐️ Live bounds of one region: the gesture's preview while it is the dragged one, the
    /// document's rectangle otherwise.
    pub(super) fn region_live_bounds(&self, region: &RegionData) -> [f64; 4] {
        match self.region_drag.as_ref().filter(|drag| drag.id == region.id) {
            Some(drag) => drag.bounds,
            None => region_bounds(region.x, region.y, region.width, region.height),
        }
    }

    /// 🩺️ Every region this board holds, in id order, as the probe reads them out of
    /// `data-board-target-regions-json`. Bounds are normalized and reflect a live drag, so the
    /// attribute and the pixels never disagree mid-gesture.
    pub fn target_regions_json(&self) -> String {
        let mut out = String::from("[");
        for (index, region) in self.regions.values().enumerate() {
            if index > 0 {
                out.push(',');
            }
            let [min_x, min_y, max_x, max_y] = self.region_live_bounds(region);
            out.push_str("{\"id\":\"");
            out.push_str(&region.id.replace('\\', "\\\\").replace('"', "\\\""));
            out.push_str("\",\"x\":");
            out.push_str(&min_x.to_string());
            out.push_str(",\"y\":");
            out.push_str(&min_y.to_string());
            out.push_str(",\"width\":");
            out.push_str(&(max_x - min_x).to_string());
            out.push_str(",\"height\":");
            out.push_str(&(max_y - min_y).to_string());
            if let Some(label) = region.label.as_deref() {
                out.push_str(",\"label\":\"");
                out.push_str(&label.replace('\\', "\\\\").replace('"', "\\\""));
                out.push('"');
            }
            out.push_str(",\"hidden\":");
            out.push_str(if region.hidden { "true" } else { "false" });
            out.push_str(",\"locked\":");
            out.push_str(if region.locked { "true" } else { "false" });
            out.push_str(",\"selected\":");
            out.push_str(if region.selected { "true" } else { "false" });
            out.push('}');
        }
        out.push(']');
        out
    }

    /// 🎯️ The region grip under `world`, asked ONLY after the nodes, handles, edges and the
    /// rotate ring have all missed — a region is the board's backdrop, never a lid over its graph.
    /// A hidden region is not paint and therefore not a pick target either.
    ///
    /// 🤏️ A real grip always outranks a body: overlapping rectangles must still be resizable
    /// at the edge of the one on top, otherwise a region enclosing another would swallow it.
    fn region_grip_hit_world(&self, world: Point) -> Option<(String, RegionGrip)> {
        let zoom = self.camera.zoom;
        let mut best: Option<(String, RegionGrip)> = None;
        for region in self.regions.values() {
            if region.hidden {
                continue;
            }
            let Some(grip) = region_grip_at(self.region_live_bounds(region), zoom, world) else { continue };
            let replaces = match best.as_ref() {
                None => true,
                Some((_, previous)) => *previous == RegionGrip::Body || grip != RegionGrip::Body,
            };
            if replaces {
                best = Some((region.id.clone(), grip));
            }
        }
        best
    }

    /// 🚚️ Grabs a region: the body moves it, a corner/edge grip resizes it. A LOCKED region
    /// refuses the grab outright — it still paints and still selects, it simply cannot be dragged.
    fn try_begin_region_drag_at(&mut self, world: Point) -> bool {
        let Some((id, grip)) = self.region_grip_hit_world(world) else {
            return false;
        };
        let Some((start_bounds, locked)) = self.regions.get(&id).map(|region| (region_bounds(region.x, region.y, region.width, region.height), region.locked)) else {
            return false;
        };
        let ids: Vec<String> = merge_pick_into_selection(&self.selection, &id, "replace").into_iter().collect();
        if locked {
            self.set_selection_ids_gestured(&ids, None);
            self.interaction = Interaction::None;
            self.set_hovered_id(None);
            return true;
        }
        let mut gesture = self.begin_gesture();
        self.stage_selection_ids(&mut gesture, &ids, None);
        self.region_drag = Some(BoardRegionDrag { gesture, id, grip, grab: world, start_bounds, bounds: start_bounds, delta: Vec2::ZERO });
        self.set_hovered_id(None);
        true
    }

    /// 👁️ Re-derives the whole preview from the grab-time rectangle, so the drag never
    /// accumulates float drift. Emits nothing: a region gesture is ONE document edit, on release.
    fn update_region_drag(&mut self, world: Point) {
        let Some(mut drag) = self.region_drag.take() else {
            return;
        };
        let step = self.region_snap_step();
        let dx = snap_region_scalar(world.x - drag.grab.x, step);
        let dy = snap_region_scalar(world.y - drag.grab.y, step);
        drag.bounds = region_grip_drag(drag.start_bounds, drag.grip, dx, dy);
        drag.delta = Vec2::new(dx, dy);
        self.region_drag = Some(drag);
        self.bump_content_scene_generation();
    }

    /// 🏁️ Ends a region gesture as ONE batch with the `select` its press staged: a body drag is a `drag`
    /// gesture record by the snapped offset, a grip drag one `regionResize`. A release that moved nothing
    /// publishes only the selection.
    fn commit_region_drag(&mut self) -> bool {
        let Some(drag) = self.region_drag.take() else {
            return false;
        };
        let moved = drag.bounds != drag.start_bounds;
        if moved {
            if let Some(region) = self.regions.get_mut(&drag.id) {
                region.x = drag.bounds[0];
                region.y = drag.bounds[1];
                region.width = drag.bounds[2] - drag.bounds[0];
                region.height = drag.bounds[3] - drag.bounds[1];
            }
            self.bump_content_scene_generation();
        }
        match (moved, drag.grip) {
            (false, _) => self.publish_gesture_release(&drag.gesture, None),
            (true, RegionGrip::Body) => {
                let motion = BoardGestureMotion::Drag { dx: drag.delta.x, dy: drag.delta.y };
                self.publish_gesture_release(&drag.gesture, Some(BoardOwnedEvent::gesture(&drag.gesture.id, [drag.id.as_str()], motion, &[])));
            }
            (true, _) => {
                let [select, _] = Self::gesture_release_events(&drag.gesture, None);
                let resize = BoardOwnedEvent::region_resize(&drag.id, drag.bounds);
                let reservation = match select {
                    Some(select) => self.reserve_owned_batch(select, Some(resize)),
                    None => self.reserve_owned_batch(resize, None),
                };
                if let Some(reservation) = reservation {
                    self.publish_event_batch(reservation);
                }
            }
        }
        true
    }

    /// ↩️ Abandons a live region gesture with zero trace: the rectangle and the selection its press staged
    /// return, and the document is never touched.
    pub(super) fn cancel_region_drag(&mut self) -> bool {
        let Some(drag) = self.region_drag.take() else {
            return false;
        };
        self.restore_gesture_selection(drag.gesture);
        self.bump_content_scene_generation();
        true
    }

    fn begin_region_paint(&mut self, world: Point) {
        let anchor = self.snap_region_point(world);
        self.region_paint = Some(BoardRegionPaint { anchor, corner: anchor, dragged: false });
        self.set_hovered_id(None);
        self.bump_content_scene_generation();
    }

    fn update_region_paint(&mut self, world: Point) {
        let Some(mut paint) = self.region_paint.take() else {
            return;
        };
        paint.corner = self.snap_region_point(world);
        paint.dragged = (paint.corner.x - paint.anchor.x).abs() > 1e-9 || (paint.corner.y - paint.anchor.y).abs() > 1e-9;
        self.region_paint = Some(paint);
        self.bump_content_scene_generation();
    }

    /// 📐️ The rectangle the live paint would commit: the dragged box, or the configured
    /// brush extent anchored at the press point when the gesture never left it.
    fn region_paint_bounds(paint: &BoardRegionPaint, extent: (f64, f64)) -> [f64; 4] {
        if paint.dragged {
            return region_bounds(paint.anchor.x, paint.anchor.y, paint.corner.x - paint.anchor.x, paint.corner.y - paint.anchor.y);
        }
        region_bounds(paint.anchor.x, paint.anchor.y, extent.0, extent.1)
    }

    /// 🏁️ Ends an area-brush gesture with exactly ONE `regionCreate` row. A rectangle that
    /// collapsed on either axis is refused rather than published — the same floor a resize honours.
    fn commit_region_paint(&mut self) -> bool {
        let Some(paint) = self.region_paint.take() else {
            return false;
        };
        self.bump_content_scene_generation();
        let bounds = Self::region_paint_bounds(&paint, self.area_brush_extent);
        if bounds[2] - bounds[0] < REGION_MIN_EXTENT_WORLD || bounds[3] - bounds[1] < REGION_MIN_EXTENT_WORLD {
            return true;
        }
        let Some(reservation) = self.reserve_owned_event(BoardOwnedEvent::region_create(bounds)) else {
            return true;
        };
        self.publish_event_reservation(reservation);
        true
    }

    pub(super) fn cancel_region_paint(&mut self) -> bool {
        if self.region_paint.take().is_none() {
            return false;
        }
        self.bump_content_scene_generation();
        true
    }

    /// 🎯️ Paints every visible region — translucent fill, outline, corner grips on the
    /// selected one, and a corner tag for a labelled one. Appended BEFORE the node fill layer, so
    /// a region is always the backdrop of the graph and never covers an entity.
    ///
    /// The board engine owns no text primitive, so the `label` is a corner tag here and the string
    /// itself reaches the reader through [`Self::target_regions_json`] and the inspector.
    pub(super) fn append_target_region_paint(&self, scene: &mut Scene, world_space: bool) {
        let zoom = self.camera.zoom.max(1e-9);
        let grip = REGION_GRIP_PX / zoom;
        let inset = REGION_LABEL_INSET_PX / zoom;
        for region in self.regions.values() {
            if region.hidden {
                continue;
            }
            let [min_x, min_y, max_x, max_y] = self.region_live_bounds(region);
            let hovered = self.hovered_id.as_deref() == Some(region.id.as_str());
            let dragging = self.region_drag.as_ref().is_some_and(|drag| drag.id == region.id);
            let stroke_color = if region.selected || dragging {
                self.canvas_theme.node_stroke_selected
            } else if hovered {
                self.canvas_theme.node_stroke_hovered
            } else {
                self.canvas_theme.selection_preview_stroke
            };
            let top_left = self.draw_space_point(Point::new(min_x, min_y), world_space);
            let bottom_right = self.draw_space_point(Point::new(max_x, max_y), world_space);
            let rect = Rect::new(top_left.x, top_left.y, bottom_right.x, bottom_right.y);
            scene.fill(FillRule::NonZero, Affine::IDENTITY, self.canvas_theme.selection_preview_fill, None, &rect);
            let mut outline = Stroke::new(ui_styling::strokes::SELECTION_PREVIEW);
            if region.locked {
                outline.set_dash_pattern(vec![4.0, 4.0]);
            }
            scene.stroke(&outline, Affine::IDENTITY, stroke_color, None, &rect);
            if region.label.is_some() {
                let tag_x = self.draw_space_point(Point::new(min_x + inset, min_y + inset), world_space);
                let tag_far = self.draw_space_point(Point::new(min_x + inset + grip * 2.0, min_y + inset + grip * 0.6), world_space);
                scene.fill(FillRule::NonZero, Affine::IDENTITY, stroke_color, None, &Rect::new(tag_x.x, tag_x.y, tag_far.x, tag_far.y));
            }
            if (region.selected || dragging) && !region.locked {
                for (cx, cy) in [(min_x, min_y), (max_x, min_y), (min_x, max_y), (max_x, max_y)] {
                    let near = self.draw_space_point(Point::new(cx - grip / 2.0, cy - grip / 2.0), world_space);
                    let far = self.draw_space_point(Point::new(cx + grip / 2.0, cy + grip / 2.0), world_space);
                    scene.fill(FillRule::NonZero, Affine::IDENTITY, stroke_color, None, &Rect::new(near.x, near.y, far.x, far.y));
                }
            }
        }
        if let Some(paint) = self.region_paint.as_ref() {
            let [min_x, min_y, max_x, max_y] = Self::region_paint_bounds(paint, self.area_brush_extent);
            let near = self.draw_space_point(Point::new(min_x, min_y), world_space);
            let far = self.draw_space_point(Point::new(max_x, max_y), world_space);
            let rect = Rect::new(near.x, near.y, far.x, far.y);
            scene.fill(FillRule::NonZero, Affine::IDENTITY, self.canvas_theme.selection_preview_fill, None, &rect);
            let mut preview = Stroke::new(ui_styling::strokes::SELECTION_PREVIEW);
            preview.set_dash_pattern(vec![6.0, 5.0]);
            scene.stroke(&preview, Affine::IDENTITY, self.canvas_theme.node_stroke_selected, None, &rect);
        }
    }
    //#endregion 🎯️TargetRegions

    /// 📦️ Starts a group drag when `world` lies inside the padded union bounds of the current selection (minimap/overview LOD).
    fn try_begin_bounded_selection_drag_at(&mut self, world: Point) -> bool {
        if !self.lod_uses_bounded_drag() {
            return false;
        }
        let members: Vec<String> = self.selection_draggable_node_members().into_iter().filter(|id| self.nodes.get(id).is_some_and(|n| !n.locked)).collect();
        if members.is_empty() {
            return false;
        }
        let Some(bounds) = self.selection_union_bounds_world() else {
            return false;
        };
        let pad = BOUNDED_DRAG_HIT_PAD_PX / self.camera.zoom.max(1e-9);
        if !world_box_contains_point(inflate_world_box(bounds, pad), world) {
            return false;
        }
        let primary_id = members
            .iter()
            .min_by(|a, b| {
                let da = self.nodes.get(*a).map_or(f64::INFINITY, |n| distance_between(world, Point::new(n.x, n.y)));
                let db = self.nodes.get(*b).map_or(f64::INFINITY, |n| distance_between(world, Point::new(n.x, n.y)));
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned()
            .unwrap_or_else(|| members[0].clone());
        let (px0, py0) = self.nodes.get(&primary_id).map_or((0.0, 0.0), |n| (n.x, n.y));
        let mut start_positions = BTreeMap::new();
        for id in &members {
            if let Some(n) = self.nodes.get(id) {
                start_positions.insert(id.clone(), (n.x, n.y));
            }
        }
        let gesture = self.begin_gesture();
        self.interaction = Interaction::DragNodes { primary_id, offset: world - Point::new(px0, py0), start_positions, proximity_pair: None, gesture, delta: Vec2::ZERO };
        self.set_hovered_id(None);
        true
    }

    fn selection_drag_shape_world(&self, start: Point, points: &[Point]) -> Option<(WorldBox, bool, Vec<Point>)> {
        selection_drag_shape(self.selection_options.method.as_str(), start, points)
    }

    fn selection_contains_node(&self, n: &NodeData, box_: WorldBox, enclosing: bool, polygon: &[Point]) -> bool {
        let bounds = self.node_world_bounds(n, 0.0);
        let lasso = self.selection_options.method == "lasso";
        if enclosing {
            if lasso {
                polygon_contains_world_box(polygon, bounds)
            } else {
                world_box_contains_box(box_, bounds)
            }
        } else if lasso {
            polygon_intersects_world_box(polygon, bounds)
        } else {
            world_boxes_overlap(box_, bounds)
        }
    }

    fn selection_contains_handle(&self, h: &HandleData, box_: WorldBox, enclosing: bool, polygon: &[Point]) -> bool {
        let Some(pos) = self.handle_world_pos(h) else {
            return false;
        };
        let pad = self.effective_handle_radius(h).max(1.0);
        let bounds = WorldBox { min_x: pos.x - pad, min_y: pos.y - pad, max_x: pos.x + pad, max_y: pos.y + pad };
        let lasso = self.selection_options.method == "lasso";
        if enclosing {
            if lasso {
                polygon_contains_world_box(polygon, bounds)
            } else {
                world_box_contains_box(box_, bounds)
            }
        } else if lasso {
            polygon_intersects_world_box(polygon, bounds)
        } else {
            world_boxes_overlap(box_, bounds)
        }
    }

    fn selection_contains_edge(&self, c: CubicBez, box_: WorldBox, enclosing: bool, polygon: &[Point]) -> bool {
        const STEPS: usize = 24;
        let mut samples = Vec::with_capacity(STEPS + 1);
        for i in 0..=STEPS {
            let t = i as f64 / STEPS as f64;
            samples.push(cubic_bezier_point(c, t));
        }
        let lasso = self.selection_options.method == "lasso";
        if enclosing {
            if lasso {
                samples.iter().all(|&p| point_in_polygon(p, polygon))
            } else {
                samples.iter().all(|&p| world_box_contains_point(box_, p))
            }
        } else if lasso {
            (1..samples.len()).any(|i| segment_intersects_polygon(samples[i - 1], samples[i], polygon))
        } else {
            (1..samples.len()).any(|i| segment_intersects_world_box(samples[i - 1], samples[i], box_))
        }
    }

    pub(super) fn resolve_area_selection_with_initial(&self, initial: &BTreeSet<String>, start: Point, points: &[Point], merge_mode: &str) -> BTreeSet<String> {
        let Some((box_, enclosing, ref polygon)) = self.selection_drag_shape_world(start, points) else {
            return initial.clone();
        };
        let mut hits = BTreeSet::new();
        let o = &self.selection_options;
        if o.select_nodes {
            for n in self.nodes.values() {
                if self.node_selectable(n.id.as_str()) && self.selection_contains_node(n, box_, enclosing, polygon) {
                    hits.insert(n.id.clone());
                }
            }
        }
        if o.select_handles {
            for h in self.handles.values() {
                if self.handle_selectable(h.id.as_str()) && self.selection_contains_handle(h, box_, enclosing, polygon) {
                    hits.insert(h.id.clone());
                }
            }
        }
        if o.select_edges {
            for e in self.edges.values() {
                if !self.edge_selectable(e) {
                    continue;
                }
                if let Some(c) = self.edge_curve(e) {
                    if self.selection_contains_edge(c, box_, enclosing, polygon) {
                        hits.insert(e.id.clone());
                    }
                }
            }
        }
        merge_ids_into_selection(initial, &hits, merge_mode)
    }
}

#[doc(hidden)]
impl BoardHost {
    pub fn test_resolve_node_style_kind(&self, node_id: &str) -> Option<BoardElementStyleKind> {
        self.nodes.get(node_id).map(|n| self.resolve_node_style_kind(n, StyleChromePass::InteractionOverlay))
    }
}

impl semio_framework_canvas::canvas_content::CanvasContent for BoardHost {
    fn build_scene(&self) -> Scene {
        self.build_vector_scene()
    }

    fn clear_color(&self) -> Color {
        self.canvas_theme.raster_clear
    }
}
// #endregion board_host
