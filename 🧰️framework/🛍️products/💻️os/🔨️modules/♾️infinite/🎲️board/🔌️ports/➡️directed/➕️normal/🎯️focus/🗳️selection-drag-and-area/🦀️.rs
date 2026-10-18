//! 🗳️ Selection drag and area of the board focus: draggable members, union bounds, bounded selection drag and area selection with an initial set. See [`super`].

use super::*;

impl BoardFocusState {
    fn selection_drag_shape_world(&self, start: Point, points: &[Point]) -> Option<(WorldBox, bool, Vec<Point>)> {
        selection_drag_shape(self.selection_options.method.as_str(), start, points)
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
}

fn selection_draggable_node_members(scene: &BoardSceneState, focus: &BoardFocusState) -> Vec<String> {
    focus.selection.iter().filter(|id| scene.nodes.get(*id).is_some_and(|n| n.draggable)).cloned().collect()
}

fn selection_union_bounds_world(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState) -> Option<WorldBox> {
    let mut corners: Vec<Point> = Vec::new();
    for id in &focus.selection {
        let Some(n) = scene.nodes.get(id) else {
            continue;
        };
        let b = style.node_world_bounds(n, 0.0);
        corners.push(Point::new(b.min_x, b.min_y));
        corners.push(Point::new(b.max_x, b.max_y));
    }
    world_box_from_points(&corners)
}

/// 📐️ Every selected node id in engine order — the rotate gesture's members, pivot
/// contributors and event ids all come from this one list.
pub fn transform_selection_node_ids(scene: &BoardSceneState, focus: &BoardFocusState) -> Vec<String> {
    focus.selection.iter().filter(|id| scene.nodes.contains_key(*id)).cloned().collect()
}

/// 📦️ Starts a group drag when `world` lies inside the padded union bounds of the current selection (minimap/overview LOD).
pub fn try_begin_bounded_selection_drag_at(scene: &mut BoardSceneState, focus: &mut BoardFocusState, style: &mut BoardStyleState, outbox: &mut BoardOutboxState, world: Point) -> bool {
    if !board_lod_uses_bounded_drag(scene, style) {
        return false;
    }
    let members: Vec<String> = selection_draggable_node_members(scene, focus).into_iter().filter(|id| scene.nodes.get(id).is_some_and(|n| !n.locked)).collect();
    if members.is_empty() {
        return false;
    }
    let Some(bounds) = selection_union_bounds_world(scene, focus, style) else {
        return false;
    };
    let pad = BOUNDED_DRAG_HIT_PAD_PX / scene.camera.zoom.max(1e-9);
    if !world_box_contains_point(inflate_world_box(bounds, pad), world) {
        return false;
    }
    let primary_id = members
        .iter()
        .min_by(|a, b| {
            let da = scene.nodes.get(*a).map_or(f64::INFINITY, |n| distance_between(world, Point::new(n.x, n.y)));
            let db = scene.nodes.get(*b).map_or(f64::INFINITY, |n| distance_between(world, Point::new(n.x, n.y)));
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        })
        .cloned()
        .unwrap_or_else(|| members[0].clone());
    let (px0, py0) = scene.nodes.get(&primary_id).map_or((0.0, 0.0), |n| (n.x, n.y));
    let mut start_positions = BTreeMap::new();
    for id in &members {
        if let Some(n) = scene.nodes.get(id) {
            start_positions.insert(id.clone(), (n.x, n.y));
        }
    }
    let gesture = outbox.begin_gesture();
    focus.interaction = Interaction::DragNodes { primary_id, offset: world - Point::new(px0, py0), start_positions, proximity_pair: None, gesture, delta: Vec2::ZERO };
    set_hovered_id(scene, focus, outbox, None);
    true
}

fn selection_contains_node(focus: &BoardFocusState, style: &BoardStyleState, n: &NodeData, box_: WorldBox, enclosing: bool, polygon: &[Point]) -> bool {
    let bounds = style.node_world_bounds(n, 0.0);
    let lasso = focus.selection_options.method == "lasso";
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

fn selection_contains_handle(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, h: &HandleData, box_: WorldBox, enclosing: bool, polygon: &[Point]) -> bool {
    let Some(pos) = handle_world_pos(scene, style, h) else {
        return false;
    };
    let pad = effective_handle_radius(scene, style, h).max(1.0);
    let bounds = WorldBox { min_x: pos.x - pad, min_y: pos.y - pad, max_x: pos.x + pad, max_y: pos.y + pad };
    let lasso = focus.selection_options.method == "lasso";
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

pub fn resolve_area_selection_with_initial(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, initial: &BTreeSet<String>, start: Point, points: &[Point], merge_mode: &str) -> BTreeSet<String> {
    let Some((box_, enclosing, ref polygon)) = focus.selection_drag_shape_world(start, points) else {
        return initial.clone();
    };
    let mut hits = BTreeSet::new();
    let o = &focus.selection_options;
    if o.select_nodes {
        for n in scene.nodes.values() {
            if scene.node_selectable(n.id.as_str()) && selection_contains_node(focus, style, n, box_, enclosing, polygon) {
                hits.insert(n.id.clone());
            }
        }
    }
    if o.select_handles {
        for h in scene.handles.values() {
            if scene.handle_selectable(h.id.as_str()) && selection_contains_handle(scene, focus, style, h, box_, enclosing, polygon) {
                hits.insert(h.id.clone());
            }
        }
    }
    if o.select_edges {
        for e in scene.edges.values() {
            if !edge_selectable(scene, style, e) {
                continue;
            }
            if let Some(c) = board_edge_curve(scene, style, e) {
                if focus.selection_contains_edge(c, box_, enclosing, polygon) {
                    hits.insert(e.id.clone());
                }
            }
        }
    }
    merge_ids_into_selection(initial, &hits, merge_mode)
}

pub fn test_resolve_node_style_kind(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, node_id: &str) -> Option<BoardElementStyleKind> {
    scene.nodes.get(node_id).map(|n| resolve_node_style_kind(focus, style, n, StyleChromePass::InteractionOverlay))
}
