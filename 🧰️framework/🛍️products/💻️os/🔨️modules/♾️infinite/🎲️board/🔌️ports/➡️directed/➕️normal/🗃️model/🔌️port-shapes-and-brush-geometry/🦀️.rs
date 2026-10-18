//! 🔌️ Port shapes and brush geometry of the board model: pair compatibility, port shapes and families, link gesture rules, brush handle alignment, brush template positions and region paint bounds. See [`super`].

use super::*;

pub fn compat_pair_matches(rule: &LinkCompatRule, a: &str, b: &str) -> bool {
    if rule.source == a && rule.target == b {
        return true;
    }
    if rule.bidirectional && rule.source == b && rule.target == a {
        return true;
    }
    false
}

pub fn handle_port_shape(handle_kind: &str) -> Option<&'static str> {
    if handle_kind.contains(" circular ") {
        Some("circular")
    } else if handle_kind.contains(" rectangular ") {
        Some("rectangular")
    } else {
        None
    }
}

pub fn handle_port_shapes_compatible(source_handle_kind: &str, target_handle_kind: &str) -> bool {
    match (handle_port_shape(source_handle_kind), handle_port_shape(target_handle_kind)) {
        (Some(a), Some(b)) => a == b,
        _ => true,
    }
}

fn single_letter_port_family(handle_kind: &str) -> Option<char> {
    let head = handle_kind.split('-').next()?;
    if head.len() == 1 {
        head.chars().next().filter(|c| c.is_ascii_lowercase())
    } else {
        None
    }
}

pub fn single_letter_port_families_compatible(source_handle_kind: &str, target_handle_kind: &str) -> bool {
    match (single_letter_port_family(source_handle_kind), single_letter_port_family(target_handle_kind)) {
        (Some(a), Some(b)) => a == b,
        _ => true,
    }
}

pub fn link_gesture_rule_applies_kind_strings(rule: &LinkCompatRule, sn: &str, sh: &str, w_src: &str, e_src: &str, tn: &str, th: &str, _w_tgt: &str, e_tgt: &str) -> bool {
    match rule.specificity {
        CompatSpecificity::General => compat_pair_matches(rule, sh, th),
        CompatSpecificity::Node => compat_pair_matches(rule, sn, tn),
        CompatSpecificity::Edge => compat_pair_matches(rule, e_src, e_tgt),
        CompatSpecificity::Handle => compat_pair_matches(rule, sh, th),
        CompatSpecificity::Wire => compat_pair_matches(rule, w_src, th),
    }
}

pub fn brush_handle_alignment_delta(source_handle_angle: f64, target_template_angle: f64) -> f64 {
    let desired = source_handle_angle + std::f64::consts::PI;
    let mut d = (target_template_angle - desired).abs();
    if d > std::f64::consts::PI {
        d = std::f64::consts::TAU - d;
    }
    d
}

pub fn brush_template_world_pos(center: Point, shape: NodeShape, radius: f64, width: f64, height: f64, angle: f64) -> Point {
    match shape {
        NodeShape::Circle => handle_position_on_circle(center, radius, angle),
        NodeShape::Rectangle => handle_position_on_rectangle(center, width, height, angle),
    }
}

/// 📐️ The rectangle the live paint would commit: the dragged box, or the configured
/// brush extent anchored at the press point when the gesture never left it.
pub fn region_paint_bounds(paint: &BoardRegionPaint, extent: (f64, f64)) -> [f64; 4] {
    if paint.dragged {
        return region_bounds(paint.anchor.x, paint.anchor.y, paint.corner.x - paint.anchor.x, paint.corner.y - paint.anchor.y);
    }
    region_bounds(paint.anchor.x, paint.anchor.y, extent.0, extent.1)
}
