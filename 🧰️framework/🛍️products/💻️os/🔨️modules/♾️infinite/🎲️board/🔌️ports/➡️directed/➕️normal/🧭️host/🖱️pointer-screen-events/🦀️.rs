//! 🖱️ Pointer screen events of the normal board host: pointer down, move and up in screen space. See [`super`].

use super::*;

// #region board_host
impl BoardHost {
    /// 🖍️ The area brush owns the whole pointer while it is armed: a press anchors a region
    /// rectangle instead of picking, so the same gesture can never also start a marquee.
    ///
    /// 🎯️ LAST of all the pick paths: a region is the board's backdrop, so it is only grabbed
    /// once the nodes, handles, edges and the rotate ring have all missed.
    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool) {
        self.scene.interaction_revision = self.scene.interaction_revision.wrapping_add(1);
        self.focus.set_selection_screen_preview(None);
        let screen = Point::new(sx, sy);
        let world = self.scene.screen_to_world(screen);
        if self.focus.active_utility == ActiveUtility::AreaBrush {
            if button == 1 {
                self.focus.interaction = Interaction::Pan { origin: self.scene.camera.clone(), start_screen: screen };
            } else if button == 0 {
                begin_region_paint(&mut self.scene, &mut self.focus, &mut self.style, &mut self.outbox, &mut self.gestures, world);
            }
            return;
        }
        if self.focus.active_utility == ActiveUtility::Brush {
            if button == 1 {
                self.focus.interaction = Interaction::Pan { origin: self.scene.camera.clone(), start_screen: screen };
            }
            return;
        }
        let hit = resolve_hit_world(&self.scene, &self.focus, &self.style, world).or_else(|| resolve_overview_free_link_handle_pointer_world(&self.scene, &self.focus, &self.style, world));
        if let Interaction::LinkTargetNode { source_id, target_node_id } = self.focus.interaction.clone() {
            self.focus.interaction = Interaction::None;
            self.outbox.clear_link_gesture_events();
            if button == 0 {
                if let Some(th) = node_sole_free_link_compatible_handle(&self.scene, &self.style, &source_id, &target_node_id) {
                    if hit.as_deref() == Some(target_node_id.as_str()) || hit.as_deref() == Some(th.as_str()) {
                        try_commit_link_edge(&mut self.scene, &mut self.style, &mut self.outbox, &source_id, &th, Some(BoardEventKind::IndirectConnect));
                        self.update_hover_from_world(world);
                        return;
                    }
                }
                if let Some(hid) = hit.as_ref().filter(|id| self.scene.handles.get(*id).is_some_and(|h| h.node_id == target_node_id) && handle_eligible_link_target_ring(&self.scene, &self.style, id.as_str(), source_id.as_str())) {
                    try_commit_link_edge(&mut self.scene, &mut self.style, &mut self.outbox, &source_id, hid, Some(BoardEventKind::IndirectConnect));
                    self.update_hover_from_world(world);
                    return;
                }
            }
            self.update_hover_from_world(world);
            return;
        }
        if let Interaction::ExternalLinkPreview { source_id, ring_node_id, ring_handle_ids, .. } = self.focus.interaction.clone() {
            if button == 0 {
                if let Some(target_node_id) = ring_node_id {
                    if let Some(th) = node_sole_free_link_compatible_handle(&self.scene, &self.style, &source_id, &target_node_id) {
                        if hit.as_deref() == Some(target_node_id.as_str()) || hit.as_deref() == Some(th.as_str()) {
                            self.focus.interaction = Interaction::None;
                            self.outbox.clear_link_gesture_events();
                            try_commit_link_edge(&mut self.scene, &mut self.style, &mut self.outbox, &source_id, &th, Some(BoardEventKind::IndirectConnect));
                            self.update_hover_from_world(world);
                            return;
                        }
                    }
                }
                if let Some(hid) = hit.as_ref().filter(|id| ring_handle_ids.iter().any(|rh| rh == *id)) {
                    self.focus.interaction = Interaction::None;
                    self.outbox.clear_link_gesture_events();
                    try_commit_link_edge(&mut self.scene, &mut self.style, &mut self.outbox, &source_id, hid, Some(BoardEventKind::IndirectConnect));
                    self.update_hover_from_world(world);
                    return;
                }
            }
            self.update_hover_from_world(world);
            return;
        }
        let merge_from_modifiers = ctrl_or_meta || shift;
        let pick_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.focus.selection_options.mode.as_str());
        if button == 0 && !merge_from_modifiers && self.try_begin_transform_drag_at(world) {
            return;
        }
        if button == 0 && !merge_from_modifiers && try_begin_bounded_selection_drag_at(&mut self.scene, &mut self.focus, &mut self.style, &mut self.outbox, world) {
            return;
        }
        if button == 1 {
            self.focus.interaction = Interaction::Pan { origin: self.scene.camera.clone(), start_screen: screen };
            return;
        }
        if let Some(ref hid) = hit {
            if let Some(node) = self.scene.nodes.get(hid) {
                if node.draggable && !node.locked {
                    let nid = hid.clone();
                    let nx = node.x;
                    let ny = node.y;
                    let mut gesture = self.outbox.begin_gesture();
                    let members_before: Vec<String> = self.focus.selection.iter().filter(|id| self.scene.nodes.get(*id).is_some_and(|n| n.draggable)).cloned().collect();
                    let drag_group_before = members_before.contains(&nid) && members_before.len() > 1;
                    let force_pick_merge = (pick_mode == "replace" && !drag_group_before) || pick_mode == "subtractive" || (pick_mode == "invertive" && merge_from_modifiers);
                    if !drag_group_before || force_pick_merge {
                        let next = merge_pick_into_selection(&self.focus.selection, &nid, pick_mode.as_str());
                        let ids: Vec<_> = next.iter().cloned().collect();
                        stage_selection_ids(&mut self.scene, &mut self.focus, &mut self.outbox, &mut gesture, &ids, merge_from_modifiers.then_some(pick_mode.as_str()));
                    }
                    let members: Vec<String> = self.focus.selection.iter().filter(|id| self.scene.nodes.get(*id).is_some_and(|n| n.draggable && !n.locked)).cloned().collect();
                    let drag_group = members.contains(&nid) && members.len() > 1;
                    let mut start_positions = BTreeMap::new();
                    for id in if drag_group { members.as_slice() } else { std::slice::from_ref(&nid) } {
                        if let Some(n) = self.scene.nodes.get(id) {
                            start_positions.insert(id.clone(), (n.x, n.y));
                        }
                    }
                    self.focus.interaction = Interaction::DragNodes { primary_id: nid, offset: world - Point::new(nx, ny), start_positions, proximity_pair: None, gesture, delta: Vec2::ZERO };
                    set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, hit);
                    return;
                }
            }
        }
        if let Some(ref hid) = hit {
            if button == 0 && self.scene.handles.contains_key(hid) && !self.scene.handle_has_incident_edge(hid.as_str()) {
                let next = merge_pick_into_selection(&self.focus.selection, hid, pick_mode.as_str());
                let ids: Vec<_> = next.iter().cloned().collect();
                let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
                set_selection_ids_gestured(&mut self.scene, &mut self.focus, &mut self.outbox, &ids, gesture);
                self.focus.interaction = Interaction::LinkAtSourceHandle { source_id: hid.clone(), start_screen: screen };
                set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, Some(hid.clone()));
                return;
            }
        }
        if hit.is_none() && button == 0 && try_begin_region_drag_at(&mut self.scene, &mut self.focus, &mut self.outbox, &mut self.gestures, world) {
            return;
        }
        if hit.is_none() && button == 0 {
            self.focus.interaction = Interaction::SelectionPending { initial_ids: self.focus.selection.clone(), start: world, start_screen: screen };
            set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, None);
            return;
        }
        self.focus.interaction = Interaction::None;
        if let Some(id) = hit {
            let next = merge_pick_into_selection(&self.focus.selection, &id, pick_mode.as_str());
            let ids: Vec<_> = next.iter().cloned().collect();
            let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
            set_selection_ids_gestured(&mut self.scene, &mut self.focus, &mut self.outbox, &ids, gesture);
            set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, Some(id));
        } else {
            let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
            set_selection_ids_gestured(&mut self.scene, &mut self.focus, &mut self.outbox, &[], gesture);
            set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, None);
        }
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.scene.interaction_revision = self.scene.interaction_revision.wrapping_add(1);
        let screen = Point::new(sx, sy);
        let world = self.scene.screen_to_world(screen);
        if self.focus.active_utility == ActiveUtility::AreaBrush {
            if let Interaction::Pan { origin, start_screen } = self.focus.interaction.clone() {
                let delta = screen - start_screen;
                board_set_camera(&mut self.scene, &mut self.outbox, origin.x - delta.x / origin.zoom, origin.y - delta.y / origin.zoom, origin.zoom);
                self.focus.interaction = Interaction::Pan { origin, start_screen };
                return;
            }
            update_region_paint(&mut self.scene, &mut self.style, &mut self.gestures, world);
            return;
        }
        if self.gestures.region_drag.is_some() {
            update_region_drag(&mut self.scene, &mut self.style, &mut self.gestures, world);
            return;
        }
        if self.focus.active_utility == ActiveUtility::Brush {
            brush_update_alt(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox, alt);
            match std::mem::replace(&mut self.focus.interaction, Interaction::None) {
                Interaction::Pan { origin, start_screen } => {
                    let delta = screen - start_screen;
                    let nx = origin.x - delta.x / origin.zoom;
                    let ny = origin.y - delta.y / origin.zoom;
                    board_set_camera(&mut self.scene, &mut self.outbox, nx, ny, origin.zoom);
                    self.focus.interaction = Interaction::Pan { origin, start_screen };
                }
                _ => {
                    self.focus.interaction = Interaction::None;
                    brush_pointer_move(&mut self.scene, &mut self.focus, &mut self.style, &mut self.brush, &mut self.outbox, world);
                }
            }
            return;
        }
        if self.gestures.transform_drag.is_some() {
            update_transform_drag(&mut self.scene, &mut self.outbox, &mut self.gestures, world, self.style.grid_snap_enabled);
            return;
        }
        match std::mem::replace(&mut self.focus.interaction, Interaction::None) {
            Interaction::DragNodes { primary_id, offset, start_positions, proximity_pair: retained_proximity_pair, gesture, delta: retained_delta } => {
                let Some(delta) = drag_delta(&self.scene, &self.style, world, &primary_id, offset, &start_positions) else {
                    self.focus.interaction = Interaction::DragNodes { primary_id, offset, start_positions, proximity_pair: retained_proximity_pair, gesture, delta: retained_delta };
                    return;
                };
                let mut event_count = 0usize;
                let mut event_bytes = 0usize;
                for (id, (ox0, oy0)) in &start_positions {
                    if self.scene.nodes.contains_key(id) {
                        event_count += 1;
                        let Some(total) = board_node_move_owned_bytes(id, ox0 + delta.x, oy0 + delta.y).and_then(|bytes| event_bytes.checked_add(bytes)) else {
                            self.outbox.event_schema_fault = true;
                            self.focus.interaction = Interaction::DragNodes { primary_id, offset, start_positions, proximity_pair: retained_proximity_pair, gesture, delta: retained_delta };
                            return;
                        };
                        event_bytes = total;
                    }
                }
                if self.outbox.events.reserve(event_count, event_bytes).is_err() {
                    self.focus.interaction = Interaction::DragNodes { primary_id, offset, start_positions, proximity_pair: retained_proximity_pair, gesture, delta: retained_delta };
                    return;
                }
                let mut geometry_changed = false;
                for (id, (ox0, oy0)) in &start_positions {
                    if let Some(n) = self.scene.nodes.get_mut(id) {
                        let mx = ox0 + delta.x;
                        let my = oy0 + delta.y;
                        if (n.x - mx).abs() > 1e-9 || (n.y - my).abs() > 1e-9 {
                            geometry_changed = true;
                        }
                        n.x = mx;
                        n.y = my;
                        let event = BoardOwnedEvent::node_move(id, mx, my).expect("node move event was exactly preflighted");
                        self.outbox.events.push(event).expect("node move event credits were reserved before mutation");
                    }
                }
                if geometry_changed {
                    self.scene.bump_content_scene_generation();
                }
                let proximity_pair = if start_positions.len() == 1 { node_drag_proximity_handle_pair(&self.scene, &self.style, primary_id.as_str()) } else { None };
                self.focus.interaction = Interaction::DragNodes { primary_id, offset, start_positions, proximity_pair, gesture, delta };
            }
            Interaction::Pan { origin, start_screen } => {
                let delta = screen - start_screen;
                let nx = origin.x - delta.x / origin.zoom;
                let ny = origin.y - delta.y / origin.zoom;
                board_set_camera(&mut self.scene, &mut self.outbox, nx, ny, origin.zoom);
                self.focus.interaction = Interaction::Pan { origin, start_screen };
            }
            Interaction::SelectionPending { initial_ids, start, start_screen } => {
                if distance_between(start_screen, screen) < SELECTION_CLICK_MAX_DISTANCE_PX {
                    self.focus.interaction = Interaction::SelectionPending { initial_ids, start, start_screen };
                } else {
                    let points = vec![start, world];
                    let screen_points = vec![start_screen, screen];
                    let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.focus.selection_options.mode.as_str());
                    let next = resolve_area_selection_with_initial(&self.scene, &self.focus, &self.style, &initial_ids, start, &points, merge_mode.as_str());
                    let ids: Vec<_> = next.iter().cloned().collect();
                    let merge_from_modifiers = ctrl_or_meta || shift;
                    let gesture = merge_from_modifiers.then_some(merge_mode.as_str());
                    board_apply_area_preselect(&mut self.scene, &mut self.focus, &mut self.outbox, &initial_ids, &ids, gesture);
                    self.focus.sync_selection_screen_overlay(start_screen, &screen_points);
                    self.focus.interaction = Interaction::Selection { initial_ids, points, screen_points, start, start_screen };
                }
            }
            Interaction::Selection { mut points, mut screen_points, start, initial_ids, start_screen } => {
                let last_screen = screen_points.last().copied().unwrap_or(start_screen);
                let add_point = self.focus.selection_options.method == "lasso" || distance_between(screen, last_screen) >= SELECTION_LASSO_MIN_POINT_DISTANCE_PX;
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
                let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.focus.selection_options.mode.as_str());
                let next = resolve_area_selection_with_initial(&self.scene, &self.focus, &self.style, &initial, start, &pts, merge_mode.as_str());
                let ids: Vec<_> = next.iter().cloned().collect();
                let merge_from_modifiers = ctrl_or_meta || shift;
                let gesture = merge_from_modifiers.then_some(merge_mode.as_str());
                board_apply_area_preselect(&mut self.scene, &mut self.focus, &mut self.outbox, &initial, &ids, gesture);
                self.focus.sync_selection_screen_overlay(start_screen, &screen_points);
                self.focus.interaction = Interaction::Selection { initial_ids, points, screen_points, start, start_screen };
            }
            Interaction::LinkAtSourceHandle { source_id, start_screen } => {
                if distance_between(screen, start_screen) >= LINK_DRAG_MIN_DISTANCE_PX {
                    let optional_target_handle_id = nearest_link_snap_handle_world(&self.scene, &self.style, &source_id, world);
                    self.apply_link_drag_snap_hover(&source_id, world, optional_target_handle_id.as_deref());
                    self.focus.interaction = Interaction::LinkDragSnap { source_id: source_id.clone(), target_id: optional_target_handle_id, end_world: world };
                    sync_link_gesture_events(&mut self.scene, &mut self.focus, &mut self.style, &mut self.outbox);
                } else {
                    self.focus.interaction = Interaction::LinkAtSourceHandle { source_id, start_screen };
                    self.update_hover_from_world(world);
                }
            }
            Interaction::LinkDragSnap { source_id, .. } => {
                let optional_target_handle_id = nearest_link_snap_handle_world(&self.scene, &self.style, &source_id, world);
                self.apply_link_drag_snap_hover(&source_id, world, optional_target_handle_id.as_deref());
                self.focus.interaction = Interaction::LinkDragSnap { source_id: source_id.clone(), target_id: optional_target_handle_id, end_world: world };
                sync_link_gesture_events(&mut self.scene, &mut self.focus, &mut self.style, &mut self.outbox);
            }
            Interaction::LinkTargetNode { source_id, target_node_id } => {
                self.focus.interaction = Interaction::LinkTargetNode { source_id, target_node_id };
                self.update_hover_from_world(world);
            }
            Interaction::ExternalLinkPreview { source_id, end_world, compatible_node_ids, ring_node_id, ring_handle_ids } => {
                self.focus.interaction = Interaction::ExternalLinkPreview { source_id, end_world, compatible_node_ids, ring_node_id, ring_handle_ids };
                self.update_hover_from_world(world);
            }
            Interaction::None => {
                self.focus.interaction = Interaction::None;
                self.update_hover_from_world(world);
            }
        }
    }

    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.scene.interaction_revision = self.scene.interaction_revision.wrapping_add(1);
        let screen = Point::new(sx, sy);
        let world = self.scene.screen_to_world(screen);
        if self.focus.active_utility == ActiveUtility::AreaBrush {
            if matches!(self.focus.interaction, Interaction::Pan { .. }) {
                self.focus.interaction = Interaction::None;
            }
            commit_region_paint(&mut self.scene, &mut self.outbox, &mut self.gestures);
            return;
        }
        if commit_region_drag(&mut self.scene, &mut self.outbox, &mut self.gestures) {
            self.update_hover_from_world(world);
            return;
        }
        if self.focus.active_utility == ActiveUtility::Brush {
            brush_update_alt(&mut self.scene, &mut self.style, &mut self.brush, &mut self.outbox, alt);
            if matches!(self.focus.interaction, Interaction::Pan { .. }) {
                self.focus.interaction = Interaction::None;
            }
            brush_finish_slot(&mut self.scene, &mut self.focus, &mut self.style, &mut self.brush, &mut self.outbox);
            set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, None);
            return;
        }
        if commit_transform_drag(&mut self.outbox, &mut self.gestures) {
            self.update_hover_from_world(world);
            return;
        }
        let grabbed = std::mem::take(&mut self.focus.interaction);
        match grabbed {
            Interaction::LinkDragSnap { source_id, target_id, .. } => {
                if let Some(ref target_handle_id) = target_id {
                    if link_snap_commit_proximity_ok(&self.scene, &self.style, target_handle_id, world) && try_commit_link_edge(&mut self.scene, &mut self.style, &mut self.outbox, &source_id, target_handle_id, Some(BoardEventKind::ProximityConnect)) {
                        self.focus.interaction = Interaction::None;
                        self.outbox.clear_link_gesture_events();
                        self.update_hover_from_world(world);
                        return;
                    }
                }
                if let Some(target_node_id) = resolve_node_hit_world(&self.scene, &self.style, world) {
                    let source_node_id = self.scene.handles.get(&source_id).map(|h| h.node_id.clone());
                    if source_node_id.as_deref() != Some(target_node_id.as_str()) {
                        if let Some(sole_target) = node_sole_free_link_compatible_handle(&self.scene, &self.style, source_id.as_str(), target_node_id.as_str()) {
                            try_commit_link_edge(&mut self.scene, &mut self.style, &mut self.outbox, &source_id, &sole_target, Some(BoardEventKind::IndirectConnect));
                            self.outbox.clear_link_gesture_events();
                        } else {
                            self.focus.interaction = Interaction::LinkTargetNode { source_id, target_node_id: target_node_id.clone() };
                            set_hovered_id(&mut self.scene, &mut self.focus, &mut self.outbox, Some(target_node_id));
                            sync_link_gesture_events(&mut self.scene, &mut self.focus, &mut self.style, &mut self.outbox);
                        }
                        self.update_hover_from_world(world);
                        return;
                    }
                }
                self.focus.interaction = Interaction::None;
                self.outbox.clear_link_gesture_events();
                self.update_hover_from_world(world);
            }
            Interaction::LinkAtSourceHandle { .. } => {
                self.focus.interaction = Interaction::None;
                self.outbox.clear_link_gesture_events();
                self.update_hover_from_world(world);
            }
            Interaction::DragNodes { start_positions, proximity_pair, gesture, delta, .. } => {
                let record = drag_gesture_record(&gesture, &start_positions, delta, proximity_pair.as_ref());
                self.outbox.publish_gesture_release(&gesture, record);
                self.focus.interaction = Interaction::None;
                self.update_hover_from_world(world);
            }
            Interaction::SelectionPending { initial_ids, start, start_screen } => {
                let _ = (start, start_screen);
                let merge_from_modifiers = ctrl_or_meta || shift;
                if !merge_from_modifiers {
                    clear_selection_on_background_click(&mut self.scene, &mut self.focus, &mut self.outbox);
                } else {
                    let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.focus.selection_options.mode.as_str());
                    let gesture = Some(merge_mode.as_str());
                    let next = resolve_area_selection_with_initial(&self.scene, &self.focus, &self.style, &initial_ids, start, &[start], merge_mode.as_str());
                    let ids: Vec<_> = next.iter().cloned().collect();
                    set_selection_ids_gestured(&mut self.scene, &mut self.focus, &mut self.outbox, &ids, gesture);
                }
                self.focus.set_selection_screen_preview(None);
                self.update_hover_from_world(world);
            }
            Interaction::Selection { mut points, mut screen_points, start, initial_ids, start_screen } => {
                points.push(world);
                screen_points.push(screen);
                let end_screen = screen_points.last().copied().unwrap_or(start_screen);
                let click_only = distance_between(start_screen, end_screen) < SELECTION_CLICK_MAX_DISTANCE_PX;
                let merge_from_modifiers = ctrl_or_meta || shift;
                let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.focus.selection_options.mode.as_str());
                let gesture = merge_from_modifiers.then_some(merge_mode.as_str());
                if click_only {
                    commit_area_select_from_initial(&mut self.scene, &mut self.focus, &mut self.outbox, &initial_ids, &[], gesture);
                } else {
                    let next = resolve_area_selection_with_initial(&self.scene, &self.focus, &self.style, &initial_ids, start, &points, merge_mode.as_str());
                    let ids: Vec<_> = next.iter().cloned().collect();
                    commit_area_select_from_initial(&mut self.scene, &mut self.focus, &mut self.outbox, &initial_ids, &ids, gesture);
                }
                self.focus.set_selection_screen_preview(None);
                self.update_hover_from_world(world);
            }
            Interaction::ExternalLinkPreview { .. } => {
                self.focus.interaction = grabbed;
                self.update_hover_from_world(world);
            }
            _ => {
                self.focus.interaction = Interaction::None;
                self.update_hover_from_world(world);
            }
        }
    }
}
