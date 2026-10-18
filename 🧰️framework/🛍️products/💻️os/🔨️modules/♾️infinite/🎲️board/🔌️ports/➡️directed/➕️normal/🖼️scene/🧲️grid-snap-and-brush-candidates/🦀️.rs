//! 🧲️ Grid snap and brush candidates of the board scene: the current draw level of detail, grid snap steps, link rule gates, brush anchors and candidates and the region snap. See [`super`].

use super::*;

impl BoardSceneState {
    pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
        self.width = width.max(1);
        self.height = height.max(1);
        self.dpr = dpr.max(1.0);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
    }

    /// 📷️ Reads the accepted viewport pose used by touch and pointer gesture planning.
    pub fn camera(&self) -> [f64; 3] {
        [self.camera.x, self.camera.y, self.camera.zoom]
    }

    pub fn resolve_element_kind_hover(&self, id: &str) -> Option<(String, String)> {
        if let Some(node) = self.nodes.get(id) {
            return Some(("node".to_string(), node.node_kind.clone()));
        }
        if let Some(handle) = self.handles.get(id) {
            return Some(("handle".to_string(), handle.handle_kind.clone()));
        }
        if let Some(edge) = self.edges.get(id) {
            return Some(("edge".to_string(), edge.edge_kind.clone()));
        }
        if let Some(wire) = self.wires.get(id) {
            return Some(("wire".to_string(), wire.wire_kind.clone()));
        }
        None
    }

    pub fn handle_slot_center_world(&self, node_id: &str, hw: Point, offset: f64) -> Option<Point> {
        let n = self.nodes.get(node_id)?;
        let nc = Point::new(n.x, n.y);
        let normal = normalize_or_zero(hw - nc);
        Some(hw + normal * offset)
    }
}

pub fn current_draw_lod(scene: &BoardSceneState, style: &BoardStyleState) -> BoardDrawLod {
    if !style.automatic_lod {
        if let Some(lod) = style.forced_draw_lod {
            return lod;
        }
    }
    BoardDrawLod::from_scale_index(BOARD_LOD_SCALE.resolve_index(scene.camera.zoom))
}

pub(crate) fn board_lod_visible_grid_snap_step_world(scene: &BoardSceneState, style: &BoardStyleState) -> Option<f64> {
    match current_draw_lod(scene, style) {
        BoardDrawLod::Minimap => None,
        BoardDrawLod::Overview | BoardDrawLod::Compact => Some(style.grid_step_large_world()),
        BoardDrawLod::Normal => Some(style.grid_step_medium_world()),
        BoardDrawLod::Detail => Some(style.grid_step_small_world()),
        BoardDrawLod::Micro => Some(style.grid_step_micro_world()),
    }
}

fn board_snap_world_scalar(scene: &BoardSceneState, style: &BoardStyleState, v: f64) -> f64 {
    if !style.grid_snap_enabled {
        return v;
    }
    let Some(step) = board_lod_visible_grid_snap_step_world(scene, style) else {
        return v;
    };
    (v / step).round() * step
}

pub(crate) fn board_snap_world_pair(scene: &BoardSceneState, style: &BoardStyleState, x: f64, y: f64) -> (f64, f64) {
    (board_snap_world_scalar(scene, style, x), board_snap_world_scalar(scene, style, y))
}

/// 👁️ Shows or hides the world grid. Hiding it never changes snapping, which reads `grid_snap_enabled`.
pub fn board_set_grid_visible(scene: &mut BoardSceneState, style: &mut BoardStyleState, visible: bool) {
    if style.grid_visible != visible {
        style.grid_visible = visible;
        scene.content_scene_generation = scene.content_scene_generation.wrapping_add(1);
    }
}

pub fn handles_link_compatible_for_drag(scene: &BoardSceneState, style: &BoardStyleState, source: &HandleData, target: &HandleData) -> bool {
    if !handle_port_shapes_compatible(source.handle_kind.as_str(), target.handle_kind.as_str()) {
        return false;
    }
    if style.link_compat_rules.is_empty() {
        return true;
    }
    let mut matched: Vec<&LinkCompatRule> = style.link_compat_rules.iter().filter(|rule| link_gesture_rule_applies(scene, style, rule, source, target)).collect();
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

fn link_gesture_rule_applies(scene: &BoardSceneState, style: &BoardStyleState, rule: &LinkCompatRule, source: &HandleData, target: &HandleData) -> bool {
    let w_src = style.resolve_default_wire_kind_for_handle(source);
    let w_tgt = style.resolve_default_wire_kind_for_handle(target);
    let e_src = style.resolve_default_edge_kind_for_wire_kind(&w_src);
    let e_tgt = style.resolve_default_edge_kind_for_wire_kind(&w_tgt);
    let sn = scene.nodes.get(&source.node_id).map_or("", |n| n.node_kind.as_str());
    let tn = scene.nodes.get(&target.node_id).map_or("", |n| n.node_kind.as_str());
    let sh = source.handle_kind.as_str();
    let th = target.handle_kind.as_str();
    match rule.specificity {
        CompatSpecificity::General => compat_pair_matches(rule, sh, th),
        CompatSpecificity::Node => compat_pair_matches(rule, sn, tn),
        CompatSpecificity::Edge => compat_pair_matches(rule, e_src.as_str(), e_tgt.as_str()),
        CompatSpecificity::Handle => compat_pair_matches(rule, sh, th),
        CompatSpecificity::Wire => compat_pair_matches(rule, w_src.as_str(), th),
    }
}

/// 🖌️ Brush slot anchor follows indirect-handle layout at overview/normal LOD so hit targets match painted rings.
pub fn brush_handle_anchor_world(scene: &BoardSceneState, style: &BoardStyleState, h: &HandleData) -> Option<Point> {
    if matches!(current_draw_lod(scene, style), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
        indirect_handle_world_pos(scene, style, h).or_else(|| handle_world_pos(scene, style, h))
    } else {
        handle_world_pos(scene, style, h)
    }
}

pub fn brush_compatible_candidates(scene: &BoardSceneState, style: &BoardStyleState, source: &HandleData) -> Option<BrushCandidatePage> {
    let sn = scene.nodes.get(&source.node_id).map_or("", |n| n.node_kind.as_str());
    let sh = source.handle_kind.as_str();
    let mut out = BrushCandidatePage::default();
    for (kind_id, kind) in &style.node_kinds {
        if kind.handles.is_empty() {
            continue;
        }
        let tn = kind_id.as_str();
        for (i, tmpl) in kind.handles.iter().enumerate() {
            if style.link_kinds_compatible_for_brush(sn, sh, tn, tmpl.handle_kind.as_str()) {
                let delta = brush_handle_alignment_delta(source.angle, tmpl.angle);
                out.push(kind_id, i, delta).ok()?;
            }
        }
    }
    out.sort();
    Some(out)
}

impl BoardSceneState {
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
}

impl BoardStyleState {
    pub fn node_world_bounds(&self, n: &NodeData, pad: f64) -> WorldBox {
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
}

/// 🧲️ The world step every region gesture quantizes to, or `None` while the grid-snap
/// modifier is off — one definition shared by paint, move and resize.
pub fn region_snap_step(scene: &BoardSceneState, style: &BoardStyleState) -> Option<f64> {
    if !style.grid_snap_enabled {
        return None;
    }
    board_lod_visible_grid_snap_step_world(scene, style)
}

pub fn snap_region_point(scene: &BoardSceneState, style: &BoardStyleState, point: Point) -> Point {
    let step = region_snap_step(scene, style);
    Point::new(snap_region_scalar(point.x, step), snap_region_scalar(point.y, step))
}
