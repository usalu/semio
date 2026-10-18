//! 🔲️ Selection marquee of the board engine: the selection thresholds, merge modes, drag shapes, preselection and containment tests. See [`super`].

use super::*;

pub const SELECTION_CLICK_MAX_DISTANCE_PX: f64 = ui_styling::metrics::board::SELECTION_CLICK_MAX_DISTANCE_PX;

pub const SELECTION_LASSO_MIN_POINT_DISTANCE_PX: f64 = ui_styling::metrics::board::SELECTION_LASSO_MIN_POINT_DISTANCE_PX;

pub const GRID_WORLD_LARGE: f64 = ui_styling::metrics::board::GRID_WORLD_LARGE;

pub const GRID_WORLD_MEDIUM: f64 = ui_styling::metrics::board::GRID_WORLD_MEDIUM;

pub const GRID_WORLD_SMALL: f64 = ui_styling::metrics::board::GRID_WORLD_SMALL;

pub const GRID_WORLD_MICRO: f64 = ui_styling::metrics::board::GRID_WORLD_MICRO;

pub const GRID_FACTOR_DEFAULT: f64 = ui_styling::metrics::board::GRID_FACTOR_DEFAULT;

pub const SELECTION_MARQUEE_DRAG_THRESHOLD_PX: f64 = ui_styling::metrics::board::SELECTION_MARQUEE_MAX_DISTANCE_PX;

/// 🚫️ How near a refused port a wire drag must come before the surface names the refusal. A snap
/// tolerance of zero (proximity connection off) must still explain the port the pointer is on, so
/// this floor is independent of `proximity_distance_world`.
pub const DRAW_EDGE_TYPE_REFUSAL_WORLD_TOLERANCE: f64 = 12.0;

/// 🎯️ Normalizes `default` to `replace` for merge-mode strings.
pub fn normalize_selection_mode(mode: &str) -> String {
    if mode == "default" {
        "replace".into()
    } else {
        mode.to_string()
    }
}

/// 🎯️ Maps shift/ctrl modifiers to marquee selection mode (ctrl+shift → invertive).
pub fn pick_merge_mode_for_modifiers(ctrl_or_meta: bool, shift: bool, option_mode: &str) -> String {
    if ctrl_or_meta && shift {
        return "invertive".into();
    }
    if ctrl_or_meta {
        return "subtractive".into();
    }
    if shift {
        return "additive".into();
    }
    normalize_selection_mode(option_mode)
}

/// 🎯️ Applies pick merge mode for a single id.
pub fn merge_pick_into_selection(initial: &BTreeSet<String>, hit_id: &str, mode: &str) -> BTreeSet<String> {
    let mut next = initial.clone();
    match mode {
        "additive" => {
            next.insert(hit_id.to_string());
        }
        "subtractive" => {
            next.remove(hit_id);
        }
        "replace" => {
            next.clear();
            next.insert(hit_id.to_string());
        }
        _ => {
            if next.contains(hit_id) {
                next.remove(hit_id);
            } else {
                next.insert(hit_id.to_string());
            }
        }
    }
    next
}

/// 🎯️ Applies pick merge mode for a marquee hit set.
pub fn merge_ids_into_selection(initial: &BTreeSet<String>, hits: &BTreeSet<String>, mode: &str) -> BTreeSet<String> {
    if mode == "replace" {
        return hits.clone();
    }
    let mut next = initial.clone();
    for id in hits {
        match mode {
            "additive" => {
                next.insert(id.clone());
            }
            "subtractive" => {
                next.remove(id);
            }
            _ => {
                if next.contains(id) {
                    next.remove(id);
                } else {
                    next.insert(id.clone());
                }
            }
        }
    }
    next
}

pub const SELECTION_DRAG_DIRECTION_THRESHOLD_PX: f64 = ui_styling::metrics::board::SELECTION_DRAG_DIRECTION_PX;

/// 🎯️ Drag left→right = enclosing/full; right→left = crossing/partial (rectangle endpoints).
pub fn selection_drag_enclosing_rectangle(start: Point, end: Point) -> bool {
    end.x >= start.x
}

/// 🎯️ Lasso uses the first horizontal step; rectangle compares start vs end.
pub fn selection_drag_enclosing(method: &str, start: Point, points: &[Point]) -> bool {
    if method == "lasso" {
        for point in points.iter().skip(1) {
            let dx = point.x - start.x;
            if dx.abs() < SELECTION_DRAG_DIRECTION_THRESHOLD_PX {
                continue;
            }
            return dx > 0.0;
        }
    }
    let end = points.last().copied().unwrap_or(start);
    selection_drag_enclosing_rectangle(start, end)
}

/// 🧿️ Builds the world-space marquee shape for rectangle or lasso drags.
pub fn selection_drag_shape(method: &str, start: Point, points: &[Point]) -> Option<(WorldBox, bool, Vec<Point>)> {
    let last = points.last().copied().unwrap_or(start);
    let enclosing = selection_drag_enclosing(method, start, points);
    if method == "lasso" && points.len() >= 3 {
        let poly = points.to_vec();
        let b = world_box_from_points(&poly)?;
        return Some((b, enclosing, poly));
    }
    let b = world_box_from_points(&[start, last])?;
    let poly = vec![Point::new(b.min_x, b.min_y), Point::new(b.max_x, b.min_y), Point::new(b.max_x, b.max_y), Point::new(b.min_x, b.max_y)];
    Some((b, enclosing, poly))
}

/// 🧿️ Screen-space overlay points for the shared `SelectionMarquee` overlay.
pub fn selection_screen_overlay_points(method: &str, start_screen: Point, screen_points: &[Point]) -> Option<Vec<Point>> {
    if screen_points.len() < 2 {
        return None;
    }
    let last = *screen_points.last().unwrap_or(&start_screen);
    Some(if method == "lasso" { screen_points.to_vec() } else { vec![start_screen, Point::new(last.x, start_screen.y), last, Point::new(start_screen.x, last.y)] })
}

/// 🧿️ Returns sorted ids for the next preselect set and removed anchor ids.
pub fn area_preselect_ids(anchor: &BTreeSet<String>, ids: &[String]) -> (Vec<String>, Vec<String>) {
    let next: BTreeSet<String> = ids.iter().cloned().collect();
    let mut sorted: Vec<_> = next.iter().cloned().collect();
    sorted.sort();
    let mut removed: Vec<_> = anchor.difference(&next).cloned().collect();
    removed.sort();
    (sorted, removed)
}

fn node_rect_bounds(center: Point, width: f64, height: f64) -> WorldBox {
    let hw = width * 0.5;
    let hh = height * 0.5;
    WorldBox { min_x: center.x - hw, min_y: center.y - hh, max_x: center.x + hw, max_y: center.y + hh }
}

fn node_circle_bounds(center: Point, radius: f64) -> WorldBox {
    WorldBox { min_x: center.x - radius, min_y: center.y - radius, max_x: center.x + radius, max_y: center.y + radius }
}

/// 🎯️ Tests whether a graph node body intersects or is contained by the marquee shape.
pub fn selection_contains_node_bounds(node: &Node, box_: WorldBox, enclosing: bool, polygon: &[Point], lasso: bool) -> bool {
    let bounds = match node.shape {
        NodeShape::Rectangle => node_rect_bounds(node.center, node.width, node.height),
        NodeShape::Circle => node_circle_bounds(node.center, node.radius),
    };
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

/// 🎯️ Tests whether a port handle intersects or is contained by the marquee shape.
pub fn selection_contains_handle_point(pos: Point, pad: f64, box_: WorldBox, enclosing: bool, polygon: &[Point], lasso: bool) -> bool {
    let bounds = WorldBox { min_x: pos.x - pad, min_y: pos.y - pad, max_x: pos.x + pad, max_y: pos.y + pad };
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

/// 🎯️ Tests whether a cubic edge intersects or is contained by the marquee shape.
pub fn selection_contains_edge_curve(curve: CubicBez, box_: WorldBox, enclosing: bool, polygon: &[Point], lasso: bool) -> bool {
    const STEPS: usize = 24;
    let mut samples = Vec::with_capacity(STEPS + 1);
    for i in 0..=STEPS {
        let t = i as f64 / STEPS as f64;
        samples.push(curve.eval(t));
    }
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
