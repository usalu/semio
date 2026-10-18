//! 🖱️ Engine pointer of the board engine: pointer down, move and up in world and screen space with selection, drag and draw edge handling. See [`super`].

use super::*;

// #region 🔖️Engine
impl<P: GraphPortModel, D: Directedness> GraphEngine<P, D> {
    pub fn pointer_down(&mut self, x: f64, y: f64, extend_selection: bool) {
        self.pointer_down_screen(x, y, x, y, 0, extend_selection, false, false);
    }

    /// 📦️ Starts a group drag when `point` lies inside the padded union bounds of draggable selected nodes.
    pub fn try_begin_selection_union_drag_at(&mut self, point: Point, pad_world: f64) -> bool {
        let members: Vec<NodeId> = self.selection.node_ids.iter().copied().filter(|id| self.nodes.get(id).is_some_and(|n| n.draggable)).collect();
        if members.is_empty() {
            return false;
        }
        let mut corners = Vec::new();
        for id in &self.selection.node_ids {
            let Some(node) = self.nodes.get(id) else {
                continue;
            };
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            corners.push(Point::new(node.center.x - hw, node.center.y - hh));
            corners.push(Point::new(node.center.x + hw, node.center.y + hh));
        }
        let Some(bounds) = world_box_from_points(&corners) else {
            return false;
        };
        if !world_box_contains_point(inflate_world_box(bounds, pad_world), point) {
            return false;
        }
        let primary_id = members
            .iter()
            .min_by(|a, b| {
                let da = self.nodes.get(a).map_or(f64::INFINITY, |n| distance_between(point, n.center));
                let db = self.nodes.get(b).map_or(f64::INFINITY, |n| distance_between(point, n.center));
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
            .copied()
            .unwrap_or(members[0]);
        let Some(primary) = self.nodes.get(&primary_id) else {
            return false;
        };
        self.drag_start_positions.clear();
        for id in &members {
            if let Some(node) = self.nodes.get(id) {
                self.drag_start_positions.insert(*id, RetainedPoint(node.center));
            }
        }
        self.interaction = InteractionMode::DragNodes { primary_id, offset: point - primary.center };
        self.hover = None;
        true
    }

    /// 🫳️ Starts dragging a selected draggable node (or its multi-selection group) from `point`.
    pub fn try_begin_selected_node_drag_at(&mut self, node_id: NodeId, point: Point) -> bool {
        if !self.selection.node_ids.contains(&node_id) {
            return false;
        }
        let Some(node) = self.nodes.get(&node_id) else {
            return false;
        };
        if !node.draggable {
            return false;
        }
        let members: Vec<NodeId> = self.selection.node_ids.iter().copied().filter(|id| self.nodes.get(id).is_some_and(|n| n.draggable)).collect();
        let drag_group = members.contains(&node_id) && members.len() > 1;
        self.drag_start_positions.clear();
        for id in if drag_group { members.as_slice() } else { std::slice::from_ref(&node_id) } {
            if let Some(n) = self.nodes.get(id) {
                self.drag_start_positions.insert(*id, RetainedPoint(n.center));
            }
        }
        if drag_group {
            self.interaction = InteractionMode::DragNodes { primary_id: node_id, offset: point - node.center };
        } else {
            self.interaction = InteractionMode::DragNode { node_id, offset: point - node.center };
        }
        self.hover = None;
        true
    }

    /// 🫳️ Selects a draggable node and starts moving it from `point`.
    pub fn pointer_down_on_draggable_node_at(&mut self, node_id: NodeId, point: Point, shift: bool, ctrl_or_meta: bool) {
        let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
        let merge_from_modifiers = ctrl_or_meta || shift;
        let members_before: Vec<NodeId> = self.selection.node_ids.iter().copied().filter(|id| self.nodes.get(id).is_some_and(|n| n.draggable)).collect();
        let drag_group_before = members_before.contains(&node_id) && members_before.len() > 1;
        let force_pick_merge = (merge_mode == "replace" && !drag_group_before) || merge_mode == "subtractive" || (merge_mode == "invertive" && merge_from_modifiers);
        if !drag_group_before || force_pick_merge {
            self.apply_pick_with_mode(HitObject::Node(node_id), merge_mode.as_str());
        }
        if let Some(node) = self.nodes.get(&node_id) {
            if node.draggable {
                let members: Vec<NodeId> = self.selection.node_ids.iter().copied().filter(|id| self.nodes.get(id).is_some_and(|n| n.draggable)).collect();
                let drag_group = members.contains(&node_id) && members.len() > 1;
                self.drag_start_positions.clear();
                for id in if drag_group { members.as_slice() } else { std::slice::from_ref(&node_id) } {
                    if let Some(n) = self.nodes.get(id) {
                        self.drag_start_positions.insert(*id, RetainedPoint(n.center));
                    }
                }
                if drag_group {
                    self.interaction = InteractionMode::DragNodes { primary_id: node_id, offset: point - node.center };
                } else {
                    self.interaction = InteractionMode::DragNode { node_id, offset: point - node.center };
                }
            }
        }
        self.update_hover(Some(node_id));
    }

    /// 🪝️ Merges a handle into the engine selection without starting edge draw.
    pub fn select_handle_with_mode(&mut self, handle_id: HandleId, mode: &str) {
        if !self.selection_options.select_handles || !P::HAS_PORTS {
            return;
        }
        let current: BTreeSet<String> = self.selection.handle_ids.iter().map(|id| id.to_string()).collect();
        let next = merge_pick_into_selection(&current, &handle_id.to_string(), mode);
        self.selection.handle_ids = next.iter().filter_map(|s| s.parse().ok()).collect();
        self.clear_preselect();
        self.push_selection_event();
    }

    /// 🖱️ Flat pointer-event fields (screen/world coords, button, modifiers) mirror this repo's shared WASM host-bridge pointer contract, matched by every sibling board/editor/layout crate's `pointer_*_screen` — not bundled into a struct, to keep the JS-callable surface a plain positional arg list.
    /// Primary (`button == 0`) selects and may start drag / marquee / edge-draw. Secondary (`button == 2`) only updates selection/hover for context menus — never moves nodes.
    ///
    /// 🖱️ Secondary/middle empty presses must not clear selection — context menus and pan gestures own those buttons.
    #[allow(clippy::too_many_arguments, reason = "flat args mirror the shared WASM host-bridge pointer-event contract used across all `pointer_*_screen` methods in this repo")]
    pub fn pointer_down_screen(&mut self, screen_x: f64, screen_y: f64, world_x: f64, world_y: f64, button: u8, shift: bool, ctrl_or_meta: bool, _alt: bool) {
        self.proximity_connection = None;
        self.selection_preview_points.clear();
        self.selection_preview_crossing = false;
        let point = Point::new(world_x, world_y);
        let screen = Point::new(screen_x, screen_y);
        let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
        let merge_from_modifiers = ctrl_or_meta || shift;
        let primary = button == 0;
        match self.hit_test(point) {
            Some(HitObject::Node(node_id)) => {
                let members_before: Vec<NodeId> = self.selection.node_ids.iter().copied().filter(|id| self.nodes.get(id).is_some_and(|n| n.draggable)).collect();
                let drag_group_before = members_before.contains(&node_id) && members_before.len() > 1;
                let force_pick_merge = (merge_mode == "replace" && !drag_group_before) || merge_mode == "subtractive" || (merge_mode == "invertive" && merge_from_modifiers);
                if !drag_group_before || force_pick_merge {
                    self.apply_pick_with_mode(HitObject::Node(node_id), merge_mode.as_str());
                }
                if primary {
                    if let Some(node) = self.nodes.get(&node_id) {
                        if node.draggable {
                            let members: Vec<NodeId> = self.selection.node_ids.iter().copied().filter(|id| self.nodes.get(id).is_some_and(|n| n.draggable)).collect();
                            let drag_group = members.contains(&node_id) && members.len() > 1;
                            self.drag_start_positions.clear();
                            for id in if drag_group { members.as_slice() } else { std::slice::from_ref(&node_id) } {
                                if let Some(n) = self.nodes.get(id) {
                                    self.drag_start_positions.insert(*id, RetainedPoint(n.center));
                                }
                            }
                            if drag_group {
                                self.interaction = InteractionMode::DragNodes { primary_id: node_id, offset: point - node.center };
                            } else {
                                self.interaction = InteractionMode::DragNode { node_id, offset: point - node.center };
                            }
                        }
                    }
                } else {
                    self.interaction = InteractionMode::Idle;
                }
                self.update_hover(Some(node_id));
            }
            Some(HitObject::Endpoint(ep)) => {
                self.apply_pick_with_mode(HitObject::Endpoint(ep), merge_mode.as_str());
                let hid = P::endpoint_as_u64(ep);
                self.update_hover(Some(hid));
                if primary && P::HAS_PORTS {
                    self.begin_draw_edge_from_handle(hid, point);
                } else {
                    self.interaction = InteractionMode::Idle;
                }
            }
            Some(HitObject::Edge(edge_id)) => {
                self.apply_pick_with_mode(HitObject::Edge(edge_id), merge_mode.as_str());
                self.update_hover(Some(edge_id));
                self.interaction = InteractionMode::Idle;
            }
            None if primary => {
                self.area_initial = self.selection.clone();
                self.interaction = InteractionMode::SelectionPending { start: point, start_screen: screen };
                self.update_hover(None);
            }
            None => {
                self.update_hover(None);
                self.interaction = InteractionMode::Idle;
            }
        }
    }

    pub fn pointer_move(&mut self, x: f64, y: f64) {
        self.pointer_move_screen(x, y, x, y, false, false, false);
    }

    #[allow(clippy::too_many_arguments, reason = "flat args mirror the shared WASM host-bridge pointer-event contract used across all `pointer_*_screen` methods in this repo")]
    pub fn pointer_move_screen(&mut self, screen_x: f64, screen_y: f64, world_x: f64, world_y: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        let point = Point::new(world_x, world_y);
        let screen = Point::new(screen_x, screen_y);
        match std::mem::replace(&mut self.interaction, InteractionMode::Idle) {
            InteractionMode::Pan { start_screen, cam_x, cam_y, zoom } => {
                let dx = (screen.x - start_screen.x) / zoom;
                let dy = (screen.y - start_screen.y) / zoom;
                self.set_camera(cam_x - dx, cam_y - dy, zoom);
                self.interaction = InteractionMode::Pan { start_screen, cam_x, cam_y, zoom };
            }
            InteractionMode::DragNode { node_id, offset } => {
                if alt {
                    self.proximity_distance_override = Some(0.0);
                }
                if let Some(node) = self.nodes.get_mut(&node_id) {
                    node.center = point - offset;
                    self.events.push(BoardEvent::NodeMoved { id: node_id, x: node.center.x(), y: node.center.y() });
                }
                self.update_node_drag_proximity(&[node_id]);
                if alt {
                    self.proximity_distance_override = None;
                    self.proximity_connection = None;
                }
                self.interaction = InteractionMode::DragNode { node_id, offset };
            }
            InteractionMode::DragNodes { primary_id, offset } => {
                let Some((px0, py0)) = self.drag_start_positions.get(&primary_id).map(|p| (p.x, p.y)) else {
                    self.interaction = InteractionMode::Idle;
                    return;
                };
                let nx = point.x - offset.x;
                let ny = point.y - offset.y;
                let dx = nx - px0;
                let dy = ny - py0;
                let dragged_ids: Vec<NodeId> = self.drag_start_positions.keys().copied().collect();
                if alt {
                    self.proximity_distance_override = Some(0.0);
                }
                for (id, start) in &self.drag_start_positions {
                    if let Some(node) = self.nodes.get_mut(id) {
                        node.center = Point::new(start.x + dx, start.y + dy);
                        self.events.push(BoardEvent::NodeMoved { id: *id, x: node.center.x, y: node.center.y });
                    }
                }
                self.update_node_drag_proximity(&dragged_ids);
                if alt {
                    self.proximity_distance_override = None;
                    self.proximity_connection = None;
                }
                self.interaction = InteractionMode::DragNodes { primary_id, offset };
            }
            InteractionMode::DrawEdge { anchor_handle, anchor_is_source, fixed_target, reconnecting, .. } => {
                if alt {
                    self.proximity_distance_override = Some(0.0);
                }
                let snap_target = if alt { None } else { self.nearest_wire_snap_handle(anchor_handle, anchor_is_source, fixed_target, reconnecting, point) };
                if alt {
                    self.proximity_distance_override = None;
                }
                self.interaction = InteractionMode::DrawEdge { anchor_handle, anchor_is_source, fixed_target, cursor: point, reconnecting, snap_target };
                let hover = snap_target.or_else(|| {
                    self.hit_test(point).map(|hit| match hit {
                        HitObject::Edge(id) => id,
                        HitObject::Endpoint(ep) => P::endpoint_as_u64(ep),
                        HitObject::Node(id) => id,
                    })
                });
                self.update_hover(hover);
            }
            InteractionMode::SelectionPending { start, start_screen } => {
                if distance_between(screen, start_screen) < SELECTION_MARQUEE_DRAG_THRESHOLD_PX {
                    self.interaction = InteractionMode::SelectionPending { start, start_screen };
                } else {
                    let area_points = vec![start, point];
                    let area_screen_points = vec![start_screen, screen];
                    self.area_points = area_points.clone();
                    self.area_screen_points = area_screen_points.clone();
                    self.apply_area_preselect(start, &area_points, shift, ctrl_or_meta);
                    self.sync_selection_screen_overlay(start_screen, &area_screen_points);
                    self.interaction = InteractionMode::AreaSelect { start, start_screen };
                }
            }
            InteractionMode::AreaSelect { start, start_screen } => {
                let mut points = self.area_points.clone();
                let mut screen_points = self.area_screen_points.clone();
                let last_screen = screen_points.last().copied().unwrap_or(start_screen);
                let add_point = self.selection_options.method == "lasso" || distance_between(screen, last_screen) >= SELECTION_LASSO_MIN_POINT_DISTANCE_PX;
                if add_point {
                    points.push(point);
                    screen_points.push(screen);
                } else if !points.is_empty() {
                    let last = points.len() - 1;
                    points[last] = point;
                    let ls = screen_points.len() - 1;
                    screen_points[ls] = screen;
                }
                let points_for_preselect = points.clone();
                let screen_for_overlay = screen_points.clone();
                self.apply_area_preselect(start, &points_for_preselect, shift, ctrl_or_meta);
                self.sync_selection_screen_overlay(start_screen, &screen_for_overlay);
                self.area_points = points;
                self.area_screen_points = screen_points;
                self.interaction = InteractionMode::AreaSelect { start, start_screen };
            }
            InteractionMode::Idle => {
                self.interaction = InteractionMode::Idle;
                self.update_hover(self.hit_test(point).map(|hit| match hit {
                    HitObject::Edge(id) => id,
                    HitObject::Endpoint(ep) => P::endpoint_as_u64(ep),
                    HitObject::Node(id) => id,
                }));
            }
        }
    }

    pub fn pointer_up(&mut self, x: f64, y: f64) {
        self.pointer_up_screen(x, y, x, y, false, false, false);
    }

    /// 🔌️ Dragging a wired endpoint off its port and releasing over nothing is the
    /// reconnect gesture's other outcome: the wire is CUT. Leaving it attached made
    /// wire deletion unreachable from the pointer altogether.
    #[allow(clippy::too_many_arguments, reason = "flat args mirror the shared WASM host-bridge pointer-event contract used across all `pointer_*_screen` methods in this repo")]
    pub fn pointer_up_screen(&mut self, screen_x: f64, screen_y: f64, world_x: f64, world_y: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        let point = Point::new(world_x, world_y);
        let screen = Point::new(screen_x, screen_y);
        let grabbed = std::mem::replace(&mut self.interaction, InteractionMode::Idle);
        let node_drag_proximity = self.proximity_connection.take();
        match grabbed {
            InteractionMode::DrawEdge { anchor_handle, anchor_is_source, fixed_target, reconnecting, snap_target, .. } => {
                let endpoint = snap_target.filter(|hid| self.wire_snap_still_active(*hid, point)).or_else(|| {
                    self.hit_test(point).and_then(|hit| match hit {
                        HitObject::Endpoint(ep) => Some(P::endpoint_as_u64(ep)),
                        _ => None,
                    })
                });
                if !alt {
                    if let Some(hit_hid) = endpoint {
                        let (source_hid, target_handle) = if let Some(tgt) = fixed_target {
                            (hit_hid, tgt)
                        } else if anchor_is_source {
                            (anchor_handle, hit_hid)
                        } else {
                            (hit_hid, anchor_handle)
                        };
                        self.try_connect_handles(source_hid, target_handle, reconnecting);
                    } else if let Some(edge_id) = reconnecting {
                        self.remove_edge(edge_id);
                    }
                }
            }
            InteractionMode::DragNodes { .. } | InteractionMode::DragNode { .. } => {
                if !alt {
                    if let Some(conn) = node_drag_proximity {
                        self.try_connect_handles(conn.source, conn.target, None);
                    }
                }
            }
            InteractionMode::SelectionPending { start, start_screen } => {
                let merge_from_modifiers = ctrl_or_meta || shift;
                if !merge_from_modifiers {
                    self.selection = Selection::default();
                    self.push_selection_event();
                } else {
                    let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                    let next = self.resolve_area_hits(&self.area_initial_string_set(), start, &[start], merge_mode.as_str());
                    self.commit_selection_from_hits(&next);
                }
                let _ = (start_screen, start);
                self.clear_preselect();
                self.selection_preview_points.clear();
                self.selection_preview_crossing = false;
            }
            InteractionMode::AreaSelect { start, start_screen } => {
                let mut points = self.area_points.clone();
                let mut screen_points = self.area_screen_points.clone();
                points.push(point);
                screen_points.push(screen);
                let end_screen = screen_points.last().copied().unwrap_or(start_screen);
                let click_only = distance_between(start_screen, end_screen) < SELECTION_CLICK_MAX_DISTANCE_PX;
                let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                let next = if click_only { BTreeSet::new() } else { self.resolve_area_hits(&self.area_initial_string_set(), start, &points, merge_mode.as_str()) };
                self.commit_selection_from_hits(&next);
                self.clear_preselect();
                self.selection_preview_points.clear();
                self.selection_preview_crossing = false;
            }
            InteractionMode::Pan { .. } => {}
            other => {
                self.interaction = other;
            }
        }
    }
}
// #endregion 🔖️Engine
