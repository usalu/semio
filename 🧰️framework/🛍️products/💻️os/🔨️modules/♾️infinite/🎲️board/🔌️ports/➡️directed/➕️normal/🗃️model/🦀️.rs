//! 🗃️ Normal board model: events, plans, kinds, styles-neutral records and constants shared by the board host components.
//!
//! Crate `semio-framework-os-infinite-board-normal-model`; a downward producer of the infinite board family.

#![allow(clippy::missing_errors_doc, reason = "Graph board host is internal to directed port normal.")]
#![allow(clippy::too_many_arguments, reason = "Immediate-mode paint helpers take one positional arg per geometry/style input; grouping them into structs would obscure the call sites.")]

use semio_framework_os_infinite_board_schema::{array, board_locked_option, board_visible_option, BoardNodeShape, BoardSnapshot, EdgeTipDef, EdgeTipGeometry, field, NodeDescriptor};
use semio_framework_os_infinite_board_engine::{BoardElementStyleKind, Camera, CanvasPalette, CompatSpecificity, EdgeDescriptor, EdgeStrokePattern, GestureStage, HandleData, HandleDescriptor, LinkCompatRule, NodeKindHandleTemplate, region_bounds, RegionDescriptor, RegionGrip, SceneDescriptor};
use semio_framework_canvas::{Affine, Circle, Color, CubicBez, FillRule, Point, Scene, Stroke, Vec2};
use std::collections::{BTreeMap, BTreeSet};
use serde::Deserialize;
use semio_framework_canvas::lod::{Lod, LodScale};
use graph::{NodeShape, Normal};
use semio_framework_graph_manifest::PropertyValue;
use semio_framework_graph_drawing::routing::{handle_position_on_circle, handle_position_on_rectangle};

//#endregion 🧵️FillJob

pub fn brush_preview_snapshot_from_session_json(node: &serde_json::Value, edge: &serde_json::Value, source_handle_id: &str) -> Option<BrushPreviewSnapshot> {
    let node_kind_id = node.get("nodeKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty())?;
    let x = node.get("x").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
    let y = node.get("y").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
    let shape = match node.get("shape").and_then(|x| x.as_str()).map(str::trim) {
        Some("rectangle") => NodeShape::Rectangle,
        _ => NodeShape::Circle,
    };
    let (radius, width, height) = match shape {
        NodeShape::Circle => (node.get("radius").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?, 0.0, 0.0),
        NodeShape::Rectangle => {
            let w = node.get("width").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
            let h = node.get("height").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
            (0.0, w, h)
        }
    };
    let target_handle_index = edge.get("targetHandleIndex").and_then(|v| v.as_u64()).map(|v| v as usize)?;
    let mut handles: Vec<NodeKindHandleTemplate> = Vec::new();
    if let Some(arr) = node.get("handles").and_then(|x| x.as_array()) {
        for row in arr {
            let ho = row.as_object()?;
            let handle_kind = ho.get("handleKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty())?;
            let angle = ho.get("angle").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
            let radius = ho.get("radius").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0);
            handles.push(NodeKindHandleTemplate { handle_kind: handle_kind.to_string(), angle, radius });
        }
    }
    if handles.is_empty() {
        return None;
    }
    let icon_kind = node.get("iconKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
    Some(BrushPreviewSnapshot { source_handle_id: source_handle_id.to_string(), node_kind_id: node_kind_id.to_string(), x, y, shape, radius, width, height, handles, target_handle_index, icon_kind })
}

/// 🏁️ The rows one released gesture publishes: its staged `select` (tagged with the gesture id when a
/// record follows) and the record itself — `(None, None)` for a release that changed nothing.
pub fn gesture_release_events(gesture: &GestureStage, record: Option<Result<BoardOwnedEvent, BoardEventFault>>) -> [Option<Result<BoardOwnedEvent, BoardEventFault>>; 2] {
    let tag = record.is_some().then_some(gesture.id.as_str());
    [gesture.select.as_ref().map(|(ids, merge)| BoardOwnedEvent::staged_select(ids, merge.as_deref(), tag)), record]
}

/// 🎬️ The drag record of `start_positions` moved by `delta`, `None` for a release that never moved.
pub fn drag_gesture_record(gesture: &GestureStage, start_positions: &BTreeMap<String, (f64, f64)>, delta: Vec2, proximity_pair: Option<&(String, String)>) -> Option<Result<BoardOwnedEvent, BoardEventFault>> {
    if (delta.x == 0.0 && delta.y == 0.0) || start_positions.is_empty() {
        return None;
    }
    Some(BoardOwnedEvent::gesture(&gesture.id, start_positions.keys().map(String::as_str), BoardGestureMotion::Drag { dx: delta.x, dy: delta.y }, proximity_pair.map(std::slice::from_ref).unwrap_or_default()))
}

pub fn sorted_selection_ids(set: &BTreeSet<String>) -> Vec<String> {
    let mut v: Vec<_> = set.iter().cloned().collect();
    v.sort();
    v
}

pub fn push_pick_target(out: &mut Vec<BoardPickTargetJson>, domain: &str, id: String, generality: u32, label: Option<String>) {
    if out.iter().any(|row| row.domain == domain && row.id == id) {
        return;
    }
    out.push(BoardPickTargetJson { domain: domain.to_string(), id, generality, label });
}

pub fn board_draw_lod_label(lod: BoardDrawLod) -> &'static str {
    lod.label()
}

pub fn parse_compat_specificity(raw: &str) -> Result<CompatSpecificity, NormalPortError> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "general" => Ok(CompatSpecificity::General),
        "node" => Ok(CompatSpecificity::Node),
        "edge" => Ok(CompatSpecificity::Edge),
        "handle" | "vortex" => Ok(CompatSpecificity::Handle),
        "wire" => Ok(CompatSpecificity::Wire),
        _ => Err(NormalPortError::InvalidCompatSpecificity(raw.to_string())),
    }
}

pub fn reject_kind_catalog_row_legacy_label(row: &serde_json::Map<String, serde_json::Value>, slice: &'static str) -> Result<(), NormalPortError> {
    if row.contains_key("label") {
        return Err(NormalPortError::LegacyLabelField(slice));
    }
    Ok(())
}

pub fn parse_catalog_tip_slot(value: Option<&str>) -> Option<String> {
    let s = value?.trim();
    if s.is_empty() || s.eq_ignore_ascii_case("none") {
        Some(String::new())
    } else {
        Some(s.to_string())
    }
}

pub fn parse_css_hex_color(s: &str) -> Option<Color> {
    let s = s.trim();
    let hex = s.strip_prefix('#')?;
    match hex.len() {
        3 => {
            let mut full = String::new();
            for ch in hex.chars() {
                full.push(ch);
                full.push(ch);
            }
            let v = u32::from_str_radix(&full, 16).ok()?;
            let r = ((v >> 16) & 0xff) as u8;
            let g = ((v >> 8) & 0xff) as u8;
            let b = (v & 0xff) as u8;
            Some(Color::from_rgba8(r, g, b, 255))
        }
        6 => {
            let v = u32::from_str_radix(hex, 16).ok()?;
            let r = ((v >> 16) & 0xff) as u8;
            let g = ((v >> 8) & 0xff) as u8;
            let b = (v & 0xff) as u8;
            Some(Color::from_rgba8(r, g, b, 255))
        }
        8 => {
            let v = u32::from_str_radix(hex, 16).ok()?;
            let r = ((v >> 24) & 0xff) as u8;
            let g = ((v >> 16) & 0xff) as u8;
            let b = ((v >> 8) & 0xff) as u8;
            let a = (v & 0xff) as u8;
            Some(Color::from_rgba8(r, g, b, a))
        }
        _ => None,
    }
}

/// 🎨️ Accepts `#rgb`/`#rrggbb`/`#rrggbbaa` or CSS `hsl()` / `hsla()` (comma or space syntax, optional `/` alpha).
pub fn parse_css_color(s: &str) -> Option<Color> {
    if let Some(c) = parse_css_hex_color(s) {
        return Some(c);
    }
    parse_css_hsl_color(s)
}

fn parse_css_hsl_color(s: &str) -> Option<Color> {
    let low = s.trim().to_ascii_lowercase();
    let (legacy_alpha_form, inner) = if let Some(inner) = low.strip_prefix("hsla(").and_then(|x| x.strip_suffix(')')) {
        (true, inner)
    } else {
        let inner = low.strip_prefix("hsl(").and_then(|x| x.strip_suffix(')'))?;
        (false, inner)
    };
    let inner = inner.trim();
    let (main, alpha_slash) = inner.split_once('/').map_or((inner, None), |(a, b)| (a.trim(), Some(b.trim())));
    let normalized = main.replace(',', " ");
    let parts: Vec<&str> = normalized.split_whitespace().collect();
    if parts.len() < 3 {
        return None;
    }
    let h = parse_css_hsl_hue(parts[0])?;
    let sat = parse_css_hsl_sl(parts[1])?;
    let light = parse_css_hsl_sl(parts[2])?;
    let alpha = if let Some(a) = alpha_slash {
        parse_css_alpha_channel(a)?
    } else if legacy_alpha_form && parts.len() >= 4 {
        parse_css_alpha_channel(parts[3])?
    } else {
        1.0
    };
    let (r, g, b) = hsl_to_rgb_bytes(h, sat, light);
    let a = (alpha * 255.0).round().clamp(0.0, 255.0) as u8;
    Some(Color::from_rgba8(r, g, b, a))
}

fn parse_css_hsl_hue(tok: &str) -> Option<f64> {
    let t = tok.trim();
    let n = t.strip_suffix("deg").map_or(t, str::trim);
    let v: f64 = n.parse().ok()?;
    v.is_finite().then_some(v)
}

fn parse_css_hsl_sl(tok: &str) -> Option<f64> {
    let t = tok.trim();
    if let Some(p) = t.strip_suffix('%') {
        let v: f64 = p.trim().parse().ok()?;
        Some((v / 100.0).clamp(0.0, 1.0))
    } else {
        let v: f64 = t.parse().ok()?;
        Some(v.clamp(0.0, 1.0))
    }
}

fn parse_css_alpha_channel(tok: &str) -> Option<f64> {
    parse_css_hsl_sl(tok)
}

fn board_hsl_hue_to_rgb_component(p: f64, q: f64, mut t: f64) -> f64 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 0.5 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}

fn hsl_to_rgb_bytes(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let h_norm = ((h % 360.0 + 360.0) % 360.0) / 360.0;
    let s = s.clamp(0.0, 1.0);
    let l = l.clamp(0.0, 1.0);
    if s <= f64::EPSILON {
        let v = (l * 255.0).round().clamp(0.0, 255.0) as u8;
        return (v, v, v);
    }
    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;
    let r = board_hsl_hue_to_rgb_component(p, q, h_norm + 1.0 / 3.0);
    let g = board_hsl_hue_to_rgb_component(p, q, h_norm);
    let b = board_hsl_hue_to_rgb_component(p, q, h_norm - 1.0 / 3.0);
    ((r * 255.0).round().clamp(0.0, 255.0) as u8, (g * 255.0).round().clamp(0.0, 255.0) as u8, (b * 255.0).round().clamp(0.0, 255.0) as u8)
}

pub fn explicit_style_kind(style: Option<&str>) -> Option<BoardElementStyleKind> {
    match style.map(str::trim).filter(|s| !s.is_empty()) {
        Some("original") => Some(BoardElementStyleKind::Original),
        Some("neutral") => Some(BoardElementStyleKind::Neutral),
        Some("hovered") => Some(BoardElementStyleKind::Hovered),
        Some("selected") => Some(BoardElementStyleKind::Selected),
        Some("highlighted") => Some(BoardElementStyleKind::Highlighted),
        Some("disabled") => Some(BoardElementStyleKind::Disabled),
        _ => None,
    }
}

pub fn locked_style_dim(kind: BoardElementStyleKind, locked: bool) -> BoardElementStyleKind {
    if locked && !matches!(kind, BoardElementStyleKind::Selected | BoardElementStyleKind::Highlighted | BoardElementStyleKind::Hovered) {
        BoardElementStyleKind::Disabled
    } else {
        kind
    }
}

pub fn chrome_pass_for_entity(entity_id: &str, overlay_ids: &BTreeSet<String>) -> StyleChromePass {
    if overlay_ids.contains(entity_id) {
        StyleChromePass::InteractionOverlay
    } else {
        StyleChromePass::CachedBase
    }
}

pub fn node_fill_for_style(theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
    match kind {
        BoardElementStyleKind::Hovered => theme.node_fill_hovered,
        BoardElementStyleKind::Selected => theme.node_fill_selected,
        BoardElementStyleKind::Highlighted => theme.node_fill_selection_exit,
        BoardElementStyleKind::Disabled => theme.node_fill_disabled,
        BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.node_fill,
    }
}

pub fn node_stroke_for_style(theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
    match kind {
        BoardElementStyleKind::Hovered => theme.node_stroke_hovered,
        BoardElementStyleKind::Selected => theme.node_stroke_selected,
        BoardElementStyleKind::Highlighted => theme.node_stroke_selection_exit,
        BoardElementStyleKind::Disabled => theme.node_stroke_disabled,
        BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.node_stroke,
    }
}

pub fn resolve_handle_stroke_color(_h: &HandleData, theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
    match kind {
        BoardElementStyleKind::Hovered => theme.handle_stroke_hovered,
        BoardElementStyleKind::Selected => theme.handle_stroke_selected,
        BoardElementStyleKind::Highlighted => theme.handle_stroke_selection_exit,
        BoardElementStyleKind::Disabled => theme.handle_stroke_disabled,
        BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.handle_stroke,
    }
}

pub fn edge_stroke_for_style(theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
    match kind {
        BoardElementStyleKind::Hovered => theme.edge_stroke_hovered,
        BoardElementStyleKind::Selected => theme.edge_stroke_selected,
        BoardElementStyleKind::Highlighted => theme.edge_stroke_selection_exit,
        BoardElementStyleKind::Disabled => theme.edge_stroke_disabled,
        BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.edge_stroke,
    }
}

pub fn lerp_color(a: Color, b: Color, t: f64) -> Color {
    let t = t.clamp(0.0, 1.0);
    let ac = a.to_rgba8();
    let bc = b.to_rgba8();
    Color::from_rgba8(
        (f64::from(ac.r) * (1.0 - t) + f64::from(bc.r) * t).round() as u8,
        (f64::from(ac.g) * (1.0 - t) + f64::from(bc.g) * t).round() as u8,
        (f64::from(ac.b) * (1.0 - t) + f64::from(bc.b) * t).round() as u8,
        (f64::from(ac.a) * (1.0 - t) + f64::from(bc.a) * t).round() as u8,
    )
}

pub fn edge_stroke_for_kind_pattern(pattern: EdgeStrokePattern, width: f64) -> Stroke {
    use semio_framework_canvas::Cap;
    let mut stroke = Stroke::new(width);
    match pattern {
        EdgeStrokePattern::Solid => {}
        EdgeStrokePattern::Dashed => {
            stroke.set_dash_pattern(vec![width * 3.0, width * 2.0]);
        }
        EdgeStrokePattern::Dotted => {
            stroke.set_dash_pattern(vec![width * 0.35, width * 1.65]);
            stroke.set_start_cap(Cap::Round);
            stroke.set_end_cap(Cap::Round);
        }
    }
    stroke
}

fn append_edge_tip(scene: &mut Scene, tip: Point, dir: Vec2, color: Color, stroke_width: f64, tip_def: &EdgeTipDef) {
    use semio_framework_canvas::BezPath;
    let len = dir.hypot();
    if len < 1e-9 {
        return;
    }
    let d = dir / len;
    let n = Vec2::new(-d.y, d.x);
    let sw = stroke_width.max(1.0) * tip_def.scale.max(0.25);
    match tip_def.geometry {
        EdgeTipGeometry::Arrow => {
            let length = sw * 4.2;
            let half_w = sw * 1.15;
            let base = tip - d * length;
            let mut path = BezPath::new();
            path.move_to(tip);
            path.line_to(base + n * half_w);
            path.line_to(base - n * half_w);
            path.close_path();
            if tip_def.filled {
                scene.fill(FillRule::NonZero, Affine::IDENTITY, color, None, &path);
            } else {
                scene.stroke(&Stroke::new(sw.max(ui_styling::strokes::EDGE_TIP_MIN)), Affine::IDENTITY, color, None, &path);
            }
        }
        EdgeTipGeometry::FineArrow => {
            let length = sw * 3.2;
            let half_w = sw * 0.75;
            let base = tip - d * length;
            let mut path = BezPath::new();
            path.move_to(tip);
            path.line_to(base + n * half_w);
            path.move_to(tip);
            path.line_to(base - n * half_w);
            let outline = Stroke::new((sw * 0.9).max(1.0));
            scene.stroke(&outline, Affine::IDENTITY, color, None, &path);
        }
        EdgeTipGeometry::Diamond => {
            let length = sw * 3.6;
            let half_w = sw * 1.05;
            let back = tip - d * length;
            let mid = tip - d * (length * 0.5);
            let mut path = BezPath::new();
            path.move_to(tip);
            path.line_to(mid + n * half_w);
            path.line_to(back);
            path.line_to(mid - n * half_w);
            path.close_path();
            if tip_def.filled {
                scene.fill(FillRule::NonZero, Affine::IDENTITY, color, None, &path);
            } else {
                scene.stroke(&Stroke::new(sw.max(ui_styling::strokes::EDGE_TIP_MIN)), Affine::IDENTITY, color, None, &path);
            }
        }
        EdgeTipGeometry::Circle => {
            let r = sw * 1.4;
            let center = tip - d * r;
            let circle = Circle::new(center, r);
            if tip_def.filled {
                scene.fill(FillRule::NonZero, Affine::IDENTITY, color, None, &circle);
            } else {
                scene.stroke(&Stroke::new(sw.max(ui_styling::strokes::EDGE_TIP_MIN)), Affine::IDENTITY, color, None, &circle);
            }
        }
        EdgeTipGeometry::Bar => {
            let half = sw * ui_styling::strokes::EDGE_TIP_MIN;
            let center = tip - d * (sw * 0.5);
            let mut path = BezPath::new();
            path.move_to(center + n * half);
            path.line_to(center - n * half);
            scene.stroke(&Stroke::new(sw.max(ui_styling::strokes::EDGE_TIP_MIN)), Affine::IDENTITY, color, None, &path);
        }
    }
}

pub fn append_edge_tips_on_curve(scene: &mut Scene, curve: &CubicBez, color: Color, stroke_w: f64, source: Option<&EdgeTipDef>, target: Option<&EdgeTipDef>) {
    let inset = stroke_w * 0.35;
    if let Some(tip_def) = target {
        let mut tangent = curve.p3() - curve.p2();
        if tangent.hypot() < 1e-9 {
            tangent = curve.p3() - curve.p1();
        }
        if tangent.hypot() >= 1e-9 {
            let dir = tangent / tangent.hypot();
            let tip = curve.p3() - dir * inset;
            append_edge_tip(scene, tip, tangent, color, stroke_w, tip_def);
        }
    }
    if let Some(tip_def) = source {
        let mut tangent = curve.p0() - curve.p1();
        if tangent.hypot() < 1e-9 {
            tangent = curve.p0() - curve.p2();
        }
        if tangent.hypot() >= 1e-9 {
            let dir = tangent / tangent.hypot();
            let tip = curve.p0() - dir * inset;
            append_edge_tip(scene, tip, tangent, color, stroke_w, tip_def);
        }
    }
}

pub fn wire_stroke_for_style(theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
    match kind {
        BoardElementStyleKind::Hovered => theme.wire_stroke_hovered,
        BoardElementStyleKind::Selected => theme.wire_stroke_selected,
        BoardElementStyleKind::Highlighted => theme.wire_stroke_highlighted,
        BoardElementStyleKind::Disabled => theme.wire_stroke_disabled,
        BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.wire_stroke,
    }
}

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

//#region ⚠️ Errors
/// ⚠️ Errors from board host theme/catalog/layout JSON mutators and manifest validation.
#[derive(Debug)]
pub enum NormalPortError {
    Json(serde_json::Error),
    ExternalLinkPreviewJson(serde_json::Error),
    BrushSessionJson(serde_json::Error),
    DropPreviewJson(serde_json::Error),
    Theme(String),
    GridFactorOutOfRange,
    CompatNotArray,
    RowNotObject(&'static str),
    CompatSourceMissing,
    CompatTargetMissing,
    InvalidCompatSpecificity(String),
    LegacyLabelField(&'static str),
    KindCatalogsRootNotObject,
    IdMissing(&'static str),
    HandleKindColorMissing,
    InvalidHandleKindColor(String),
    NodeKindHandleKindMissing,
    NodeKindHandleAngleMissing,
    EdgeTipRowInvalid(String),
    CatalogMissingKind(&'static str, String),
    DropPreviewInvalid,
    InvalidHandleColor(String, String),
    EventCredits,
    PlacementNamingJson(serde_json::Error),
    PlacementNamingEmpty,
}

impl std::fmt::Display for NormalPortError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "{error}"),
            Self::ExternalLinkPreviewJson(error) => write!(formatter, "setLinkSessionJson: {error}"),
            Self::BrushSessionJson(error) => write!(formatter, "setBrushSessionJson: {error}"),
            Self::DropPreviewJson(error) => write!(formatter, "setDropPreviewJson: {error}"),
            Self::Theme(message) => formatter.write_str(message),
            Self::GridFactorOutOfRange => formatter.write_str("gridFactor must be finite and in (0, 1e6]"),
            Self::CompatNotArray => formatter.write_str("expected JSON array of compatibility objects"),
            Self::RowNotObject(row) => write!(formatter, "{row} row must be object"),
            Self::CompatSourceMissing => formatter.write_str("compat row missing string source"),
            Self::CompatTargetMissing => formatter.write_str("compat row missing string target"),
            Self::InvalidCompatSpecificity(value) => write!(formatter, "compat specificity must be general|node|edge|handle|wire|vortex, got {value:?}"),
            Self::LegacyLabelField(row) => write!(formatter, "{row} kind row must use name, not legacy label"),
            Self::KindCatalogsRootNotObject => formatter.write_str("kind catalogs root must be object"),
            Self::IdMissing(row) => write!(formatter, "{row} id missing"),
            Self::HandleKindColorMissing => formatter.write_str("handle kind color missing"),
            Self::InvalidHandleKindColor(color) => write!(formatter, "invalid handle kind color {color:?}"),
            Self::NodeKindHandleKindMissing => formatter.write_str("node kind handle handleKind missing"),
            Self::NodeKindHandleAngleMissing => formatter.write_str("node kind handle angle missing"),
            Self::EdgeTipRowInvalid(row) => write!(formatter, "edge tip row {row:?} invalid"),
            Self::CatalogMissingKind(catalog, kind) => write!(formatter, "catalog missing {catalog} kind {kind:?}"),
            Self::DropPreviewInvalid => formatter.write_str("setDropPreviewJson: preview payload missing nodeKind, screen/world point, or size"),
            Self::InvalidHandleColor(handle, color) => write!(formatter, "invalid color on handle {handle}: {color:?}"),
            Self::EventCredits => formatter.write_str("board event credits exhausted before descriptor publication"),
            Self::PlacementNamingJson(error) => write!(formatter, "setPlacementNamingJson: {error}"),
            Self::PlacementNamingEmpty => formatter.write_str("setPlacementNamingJson: brushNodePrefix and brushEdgePrefix must be non-empty"),
        }
    }
}

impl std::error::Error for NormalPortError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => std::error::Error::source(error),
            Self::ExternalLinkPreviewJson(error) | Self::BrushSessionJson(error) | Self::DropPreviewJson(error) | Self::PlacementNamingJson(error) => Some(error),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for NormalPortError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}
//#endregion ⚠️ Errors

pub const EDGE_HIT_TOLERANCE_PX: f64 = ui_styling::metrics::board::EDGE_HIT_TOLERANCE_PX;

pub const HANDLE_HIT_TOLERANCE_PX: f64 = ui_styling::metrics::board::HANDLE_HIT_TOLERANCE_PX;

pub const INDIRECT_HANDLE_MARKER_NODE_SCALE: f64 = ui_styling::metrics::board::INDIRECT_HANDLE_MARKER_SCALE;

/// 🧱️ Hard ceiling on the handle rows [`BoardHost::handle_positions_json`] publishes into the DOM.
/// Nakagin carries 358 handles; a DOM attribute is not a place to stream a document.
pub const BOARD_HANDLE_VITALS_CAP: usize = 128;

/// Radial offset from node rim to indirect-handle center, as a fraction of node half-extent (circle radius or half the shorter rectangle side).
pub const INDIRECT_HANDLE_RING_GAP_NODE_SCALE: f64 = ui_styling::metrics::board::INDIRECT_HANDLE_RING_GAP_SCALE;

pub const LINK_DRAG_MIN_DISTANCE_PX: f64 = ui_styling::metrics::board::LINK_DRAG_MIN_DISTANCE_PX;

pub const LINK_HANDLE_SNAP_EXTRA_PX: f64 = ui_styling::metrics::board::LINK_HANDLE_SNAP_EXTRA_PX;

pub const LINK_COMMIT_SNAP_TIGHT_PX: f64 = ui_styling::metrics::board::LINK_COMMIT_SNAP_TIGHT_PX;

pub const DEFAULT_SUGGESTION_OFFSET: f64 = ui_styling::metrics::board::SUGGESTION_OFFSET;

pub const DEFAULT_BRUSH_NODE_SIZE: f64 = ui_styling::metrics::board::BRUSH_NODE_SIZE;

pub const BOUNDED_DRAG_HIT_PAD_PX: f64 = ui_styling::metrics::board::BOUNDED_DRAG_HIT_PAD_PX;

/// 🖍️ World extent an area-brush click paints before the utility's own steppers state one — one
/// large grid cell, so the very first region a user drops is visible without configuring anything.
pub const REGION_DEFAULT_BRUSH_EXTENT_WORLD: f64 = ui_styling::metrics::board::GRID_WORLD_LARGE;

pub const DEFAULT_WIRE_KIND_ID: &str = "wire.link";

const BOARD_LODS: &[Lod; 6] = &[
    Lod { id: "minimap", name: "Minimap", description: "Whole-board silhouette; group selection and bounded drag only.", max_zoom: 0.15 },
    Lod { id: "overview", name: "Overview", description: "Topology and indirect handle rings; no per-node picks.", max_zoom: 0.35 },
    Lod { id: "compact", name: "Compact", description: "Dense graph layout with simplified chrome.", max_zoom: 0.55 },
    Lod { id: "normal", name: "Normal", description: "Standard editing: nodes, edges, and handle rings.", max_zoom: 1.25 },
    Lod { id: "detail", name: "Detail", description: "Node icons and richer strokes.", max_zoom: 2.5 },
    Lod { id: "micro", name: "Micro", description: "Maximum fidelity including handle icons.", max_zoom: f64::INFINITY },
];

pub const BOARD_LOD_SCALE: LodScale = LodScale { lods: BOARD_LODS };

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardDrawLod {
    Minimap,
    Overview,
    Compact,
    Normal,
    Detail,
    Micro,
}

impl BoardDrawLod {
    pub fn label(self) -> &'static str {
        match self {
            Self::Minimap => "minimap",
            Self::Overview => "overview",
            Self::Compact => "compact",
            Self::Normal => "normal",
            Self::Detail => "detail",
            Self::Micro => "micro",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        Some(match id {
            "minimap" => Self::Minimap,
            "overview" => Self::Overview,
            "compact" => Self::Compact,
            "normal" => Self::Normal,
            "detail" => Self::Detail,
            "micro" => Self::Micro,
            _ => return None,
        })
    }

    pub fn from_scale_index(index: usize) -> Self {
        match index {
            0 => Self::Minimap,
            1 => Self::Overview,
            2 => Self::Compact,
            3 => Self::Normal,
            4 => Self::Detail,
            _ => Self::Micro,
        }
    }
}

/// 🔭️ The board's six draw LOD bands ([`BoardDrawLod`]) as `[{id, name, description, maxZoom}]` JSON for LOD pickers.
pub fn board_lod_scale_json() -> String {
    let rows: Vec<serde_json::Value> = BOARD_LODS
        .iter()
        .map(|lod| {
            serde_json::json!({
                "id": lod.id,
                "name": lod.name,
                "description": lod.description,
                "maxZoom": lod.max_zoom,
            })
        })
        .collect();
    serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
}

/// 🏷️ Domain naming a board consumer installs through [`BoardHost::set_naming`] and [`BoardFillSnapshotIngress::new`]:
/// the id prefixes of brush and fill placements, and the fill job's progress stage and stale-operation labels.
#[derive(Clone, Copy, Debug)]
pub struct BoardNaming {
    pub brush_node_prefix: &'static str,
    pub brush_edge_prefix: &'static str,
    pub fill_node_prefix: &'static str,
    pub fill_edge_prefix: &'static str,
    pub fill_stage_label: fn(BoardFillStage) -> &'static str,
    pub fill_stale_operation_fault: &'static str,
}

/// 📮️ The wire form of a domain's brush placement prefixes, JSON `{"brushNodePrefix":…,"brushEdgePrefix":…}`: what a scene
/// carries (`Board2dScene::placement_naming`) to hosts no domain can hand a `&'static` [`BoardNaming`], such as the os renderer's
/// wgpu `Board2d` surface. Installed with [`BoardHost::set_placement_naming_json`], it overrides the installed naming's brush prefixes.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BoardPlacementNaming {
    pub brush_node_prefix: String,
    pub brush_edge_prefix: String,
}

impl BoardPlacementNaming {
    /// 🧾️ Parses the scene wire form; both prefixes must be non-empty.
    pub fn from_json(json: &str) -> Result<Self, NormalPortError> {
        let naming: Self = serde_json::from_str(json).map_err(NormalPortError::PlacementNamingJson)?;
        if naming.brush_node_prefix.is_empty() || naming.brush_edge_prefix.is_empty() {
            return Err(NormalPortError::PlacementNamingEmpty);
        }
        Ok(naming)
    }

    /// 🖨️ The scene wire form of these prefixes.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl BoardNaming {
    /// 📤️ This naming's brush prefixes in wire form, for a domain to publish on its scene.
    pub fn placement_naming(&self) -> BoardPlacementNaming {
        BoardPlacementNaming { brush_node_prefix: self.brush_node_prefix.into(), brush_edge_prefix: self.brush_edge_prefix.into() }
    }
}

/// 🎨️ Whether drawable style resolves committed selection chrome or neutral cached geometry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StyleChromePass {
    CachedBase,
    InteractionOverlay,
}

/// 🎨️ Which node/handle primitives to paint in a layered draw pass (fills behind icons/text).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeHandlePaintLayer {
    Full,
    Fill,
    Stroke,
    Icons,
}

#[derive(Clone, Copy, Debug)]
struct BrushCandidateEntry {
    pub node_kind_id: BoardPointerSpan,
    pub target_handle_index: u16,
    sort_delta: f64,
}

#[derive(Clone, Copy)]
pub struct BrushCandidateRef<'a> {
    pub node_kind_id: &'a str,
    pub target_handle_index: usize,
}

#[derive(Clone)]
pub struct BrushCandidatePage {
    pub bytes: Box<[u8; BOARD_POINTER_BYTE_CAPACITY]>,
    pub byte_len: u16,
    pub entries: Box<[Option<BrushCandidateEntry>; BOARD_POINTER_ITEM_CAPACITY]>,
    pub len: u16,
}

impl Default for BrushCandidatePage {
    fn default() -> Self {
        Self { bytes: Box::new([0; BOARD_POINTER_BYTE_CAPACITY]), byte_len: 0, entries: Box::new([None; BOARD_POINTER_ITEM_CAPACITY]), len: 0 }
    }
}

impl BrushCandidatePage {
    pub fn push(&mut self, node_kind_id: &str, target_handle_index: usize, sort_delta: f64) -> Result<(), BoardEventFault> {
        let index = usize::from(self.len);
        if index == BOARD_POINTER_ITEM_CAPACITY || target_handle_index > u16::MAX as usize {
            return Err(BoardEventFault::ItemCredits);
        }
        let start = usize::from(self.byte_len);
        let end = start.checked_add(node_kind_id.len()).ok_or(BoardEventFault::ByteCredits)?;
        if end > BOARD_POINTER_BYTE_CAPACITY {
            return Err(BoardEventFault::ByteCredits);
        }
        self.bytes[start..end].copy_from_slice(node_kind_id.as_bytes());
        self.byte_len = end as u16;
        self.entries[index] = Some(BrushCandidateEntry { node_kind_id: BoardPointerSpan { start: start as u16, len: node_kind_id.len() as u16 }, target_handle_index: target_handle_index as u16, sort_delta });
        self.len += 1;
        Ok(())
    }

    fn id_from(bytes: &[u8], span: BoardPointerSpan) -> &str {
        let start = usize::from(span.start);
        let end = start + usize::from(span.len);
        std::str::from_utf8(&bytes[start..end]).expect("brush candidate ids originate from UTF-8 strings")
    }

    pub fn sort(&mut self) {
        let bytes = &self.bytes[..usize::from(self.byte_len)];
        self.entries[..usize::from(self.len)].sort_by(|left, right| {
            let left = left.expect("candidate entry");
            let right = right.expect("candidate entry");
            left.sort_delta
                .partial_cmp(&right.sort_delta)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| Self::id_from(bytes, left.node_kind_id).cmp(Self::id_from(bytes, right.node_kind_id)))
                .then_with(|| left.target_handle_index.cmp(&right.target_handle_index))
        });
    }

    pub fn get(&self, index: usize) -> Option<BrushCandidateRef<'_>> {
        let entry = self.entries.get(index)?.as_ref()?;
        Some(BrushCandidateRef { node_kind_id: Self::id_from(self.bytes.as_ref(), entry.node_kind_id), target_handle_index: usize::from(entry.target_handle_index) })
    }

    pub fn first(&self) -> Option<BrushCandidateRef<'_>> {
        self.get(0)
    }

    pub fn iter(&self) -> impl Iterator<Item = BrushCandidateRef<'_>> {
        self.entries[..usize::from(self.len)].iter().flatten().map(|entry| BrushCandidateRef { node_kind_id: Self::id_from(self.bytes.as_ref(), entry.node_kind_id), target_handle_index: usize::from(entry.target_handle_index) })
    }

    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        self.byte_len = 0;
        self.len = 0;
    }
}

#[derive(Clone, Debug)]
pub struct BrushPreviewSnapshot {
    pub source_handle_id: String,
    pub node_kind_id: String,
    pub x: f64,
    pub y: f64,
    pub shape: NodeShape,
    pub radius: f64,
    pub width: f64,
    pub height: f64,
    pub handles: Vec<NodeKindHandleTemplate>,
    pub target_handle_index: usize,
    pub icon_kind: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillStage {
    ResetSources,
    PrepareSources,
    SelectTarget,
    ResetCandidates,
    PrepareCandidates,
    ScanCompatibility,
    SelectCandidate,
    ConstructPreview,
    ScanHostCollision,
    ScanVirtualCollision,
    AcceptCandidate,
    AcceptNodeId,
    AcceptEdgeId,
    AcceptEdgeKind,
    AcceptNodeKind,
    AcceptSourceHandle,
    AcceptTargetHandle,
    AcceptIcon,
    AcceptVirtualNode,
    AcceptSourceConnection,
    AcceptHandles,
    AcceptHandleId,
    AcceptHandleVirtual,
    AcceptHandlePublish,
    PublishPlanPrefix,
    Complete,
}

impl BoardFillStage {
    pub fn id(self) -> &'static str {
        match self {
            Self::ResetSources => "reset-sources",
            Self::PrepareSources => "prepare-sources",
            Self::SelectTarget => "select-target",
            Self::ResetCandidates => "reset-candidates",
            Self::PrepareCandidates => "prepare-candidates",
            Self::ScanCompatibility => "scan-compatibility",
            Self::SelectCandidate => "select-candidate",
            Self::ConstructPreview => "construct-preview",
            Self::ScanHostCollision => "scan-host-collision",
            Self::ScanVirtualCollision => "scan-virtual-collision",
            Self::AcceptCandidate => "accept-candidate",
            Self::AcceptNodeId => "accept-node-id",
            Self::AcceptEdgeId => "accept-edge-id",
            Self::AcceptEdgeKind => "accept-edge-kind",
            Self::AcceptNodeKind => "accept-node-kind",
            Self::AcceptSourceHandle => "accept-source-handle",
            Self::AcceptTargetHandle => "accept-target-handle",
            Self::AcceptIcon => "accept-icon",
            Self::AcceptVirtualNode => "accept-virtual-node",
            Self::AcceptSourceConnection => "accept-source-connection",
            Self::AcceptHandles => "accept-handles",
            Self::AcceptHandleId => "accept-handle-id",
            Self::AcceptHandleVirtual => "accept-handle-virtual",
            Self::AcceptHandlePublish => "accept-handle-publish",
            Self::PublishPlanPrefix => "publish-plan-prefix",
            Self::Complete => "complete",
        }
    }
}

/// 🔄️ One live rotate-ring gesture. The node centres and handle angles captured at grab time are
/// the base every frame re-derives from, so the preview never accumulates float drift and a
/// cancel restores the exact pre-gesture geometry.
#[derive(Clone, Debug)]
pub struct BoardTransformDrag {
    pub gesture: GestureStage,
    pub pivot: Point,
    pub grab: Point,
    pub radius_world: f64,
    pub radians: f64,
    pub start_positions: BTreeMap<String, (f64, f64)>,
    pub start_handle_angles: BTreeMap<String, f64>,
}

/// 🚚️ A live target-region gesture: which region, which grip, and the bounds it was grabbed at.
/// Every frame re-derives from `start_bounds`, so the preview never accumulates float drift and a
/// cancel restores the exact pre-gesture rectangle.
#[derive(Clone, Debug)]
pub struct BoardRegionDrag {
    pub gesture: GestureStage,
    pub id: String,
    pub grip: RegionGrip,
    pub grab: Point,
    pub start_bounds: [f64; 4],
    pub bounds: [f64; 4],
    pub delta: Vec2,
}

/// 🖍️ A live area-brush paint: the grid-snapped anchor and the corner the pointer is at. A release
/// that never left the anchor commits the configured brush extent instead of a zero-area sliver.
#[derive(Clone, Debug)]
pub struct BoardRegionPaint {
    pub anchor: Point,
    pub corner: Point,
    pub dragged: bool,
}

#[derive(Clone, Debug)]
pub struct DropPreviewSnapshot {
    pub node_kind_id: String,
    pub x: f64,
    pub y: f64,
    pub shape: NodeShape,
    pub radius: f64,
    pub width: f64,
    pub height: f64,
    pub icon_kind: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct BoardPickTargetJson {
    pub domain: String,
    pub id: String,
    pub generality: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

pub const BOARD_EVENT_ITEM_CAPACITY: usize = 256;

pub const BOARD_EVENT_BYTE_CAPACITY: usize = 256 * 1024;

pub const BOARD_EVENT_PAYLOAD_BYTE_CAPACITY: usize = 16 * 1024;

pub const BOARD_EVENT_KEY_BYTE_CAPACITY: usize = 256;

/// 🏷️ Every row kind the board engine publishes. `NodeMove` and `TransformPreview` are live-preview
/// frames hosts mirror into sibling panes and never dispatch; a finished pointer gesture is ONE `Gesture`
/// record (drag, rotate), riding one batch with the `select` row its press staged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardEventKind {
    Camera,
    NodeMove,
    TransformPreview,
    Gesture,
    Select,
    Preselect,
    PreselectCancel,
    Hover,
    BrushPreview,
    BrushCandidates,
    BrushPlace,
    LinkCompatibleNodes,
    LinkTargetRing,
    EdgeCreate,
    EdgeDelete,
    NodeDelete,
    IndirectConnect,
    ProximityConnect,
    RegionCreate,
    RegionResize,
}

impl BoardEventKind {
    pub const ALL: [Self; 20] = [
        Self::Camera,
        Self::NodeMove,
        Self::TransformPreview,
        Self::Gesture,
        Self::Select,
        Self::Preselect,
        Self::PreselectCancel,
        Self::Hover,
        Self::BrushPreview,
        Self::BrushCandidates,
        Self::BrushPlace,
        Self::LinkCompatibleNodes,
        Self::LinkTargetRing,
        Self::EdgeCreate,
        Self::EdgeDelete,
        Self::NodeDelete,
        Self::IndirectConnect,
        Self::ProximityConnect,
        Self::RegionCreate,
        Self::RegionResize,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Camera => "camera",
            Self::NodeMove => "nodeMove",
            Self::TransformPreview => "transformPreview",
            Self::Gesture => "gesture",
            Self::Select => "select",
            Self::Preselect => "preselect",
            Self::PreselectCancel => "preselectCancel",
            Self::Hover => "hover",
            Self::BrushPreview => "brushPreview",
            Self::BrushCandidates => "brushCandidates",
            Self::BrushPlace => "brushPlace",
            Self::LinkCompatibleNodes => "linkCompatibleNodes",
            Self::LinkTargetRing => "linkTargetRing",
            Self::EdgeCreate => "edgeCreate",
            Self::EdgeDelete => "edgeDelete",
            Self::NodeDelete => "nodeDelete",
            Self::IndirectConnect => "indirectConnect",
            Self::ProximityConnect => "proximityConnect",
            Self::RegionCreate => "regionCreate",
            Self::RegionResize => "regionResize",
        }
    }

    /// 🔎️ The kind a row `name` spells, `None` for a name the engine never publishes.
    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.name() == name)
    }
}

/// ✂️ A drag offset measured from pointer input, as the shortest decimal of its f32 value. Pointer coordinates
/// arrive with f32 precision, so every f64 digit below it is representation noise — an 80 px drag reading
/// `80.00003051757813` — that must never become a stored mutation input. A rotation keeps its f64 angle: a snapped
/// angle is an exact multiple whose narrowing would move every orbit off its exact pose.
pub fn board_pointer_offset(value: f64) -> f64 {
    let narrowed = value as f32;
    if !narrowed.is_finite() {
        return value;
    }
    narrowed.to_string().parse().unwrap_or(value)
}

/// 🎬️ What one released pointer gesture did, in its record's own terms: a drag by one offset, or a
/// rotation by `angle` radians (counter-clockwise) about `pivot`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoardGestureMotion {
    Drag { dx: f64, dy: f64 },
    Rotate { pivot: Point, angle: f64 },
}

#[derive(Debug, PartialEq, Eq)]
pub enum BoardEventFault {
    ItemCredits,
    ByteCredits,
    KeyCredits,
    Schema,
}

#[derive(Debug)]
pub struct BoardOwnedEvent {
    pub kind: BoardEventKind,
    pub payload: Box<[u8; BOARD_EVENT_PAYLOAD_BYTE_CAPACITY]>,
    payload_len: u16,
    pub key: [u8; BOARD_EVENT_KEY_BYTE_CAPACITY],
    key_len: u16,
}

pub struct BoardPayloadBuilder {
    pub bytes: Box<[u8; BOARD_EVENT_PAYLOAD_BYTE_CAPACITY]>,
    pub len: u16,
}

impl BoardPayloadBuilder {
    pub fn new() -> Self {
        Self { bytes: Box::new([0; BOARD_EVENT_PAYLOAD_BYTE_CAPACITY]), len: 0 }
    }

    pub fn raw(&mut self, value: &str) -> Result<(), BoardEventFault> {
        let start = usize::from(self.len);
        let end = start.checked_add(value.len()).ok_or(BoardEventFault::ByteCredits)?;
        if end > BOARD_EVENT_PAYLOAD_BYTE_CAPACITY {
            return Err(BoardEventFault::ByteCredits);
        }
        self.bytes[start..end].copy_from_slice(value.as_bytes());
        self.len = end as u16;
        Ok(())
    }

    pub fn string(&mut self, value: &str) -> Result<(), BoardEventFault> {
        self.raw("\"")?;
        for character in value.chars() {
            match character {
                '"' => self.raw("\\\"")?,
                '\\' => self.raw("\\\\")?,
                '\n' => self.raw("\\n")?,
                '\r' => self.raw("\\r")?,
                '\t' => self.raw("\\t")?,
                character if character <= '\u{1f}' => {
                    let escaped = format!("\\u{:04x}", character as u32);
                    self.raw(&escaped)?;
                }
                character => {
                    let mut encoded = [0; 4];
                    self.raw(character.encode_utf8(&mut encoded))?;
                }
            }
        }
        self.raw("\"")
    }

    fn string_array<'a>(&mut self, values: impl IntoIterator<Item = &'a str>) -> Result<(), BoardEventFault> {
        self.raw("[")?;
        let mut first = true;
        for value in values {
            if !first {
                self.raw(",")?;
            }
            first = false;
            self.string(value)?;
        }
        self.raw("]")
    }

    pub fn number(&mut self, value: f64) -> Result<(), BoardEventFault> {
        if !value.is_finite() {
            return Err(BoardEventFault::Schema);
        }
        self.raw(&value.to_string())
    }

    pub fn usize(&mut self, value: usize) -> Result<(), BoardEventFault> {
        self.raw(&value.to_string())
    }

    fn boolean(&mut self, value: bool) -> Result<(), BoardEventFault> {
        self.raw(if value { "true" } else { "false" })
    }

    pub fn finish(self, kind: BoardEventKind, key: Option<&str>) -> Result<BoardOwnedEvent, BoardEventFault> {
        let payload = std::str::from_utf8(&self.bytes[..usize::from(self.len)]).map_err(|_| BoardEventFault::Schema)?;
        BoardOwnedEvent::from_payload(kind, payload, key)
    }

    pub fn finish_owned(self, kind: BoardEventKind) -> BoardOwnedEvent {
        BoardOwnedEvent { kind, payload: self.bytes, payload_len: self.len, key: [0; BOARD_EVENT_KEY_BYTE_CAPACITY], key_len: 0 }
    }
}

pub struct BoardEventReservation {
    expected_len: u16,
    expected_bytes: usize,
    pub event: Option<BoardOwnedEvent>,
}

const BOARD_EVENT_BATCH_CAPACITY: usize = 4;

pub struct BoardEventBatchReservation {
    expected_len: u16,
    expected_bytes: usize,
    pub events: [Option<BoardOwnedEvent>; BOARD_EVENT_BATCH_CAPACITY],
    pub len: u8,
    pub cursor: u8,
}

impl BoardEventBatchReservation {
    pub fn one(event: BoardOwnedEvent) -> Self {
        let mut events = std::array::from_fn(|_| None);
        events[0] = Some(event);
        Self { expected_len: 0, expected_bytes: 0, events, len: 1, cursor: 0 }
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn push(&mut self, event: BoardOwnedEvent) -> Result<(), BoardOwnedEvent> {
        let index = usize::from(self.len);
        if index == BOARD_EVENT_BATCH_CAPACITY {
            return Err(event);
        }
        self.events[index] = Some(event);
        self.len += 1;
        Ok(())
    }

    pub fn owned_bytes(&self) -> Option<usize> {
        self.events[usize::from(self.cursor)..usize::from(self.len)].iter().try_fold(0usize, |bytes, event| bytes.checked_add(event.as_ref()?.owned_bytes()))
    }

    pub fn peek(&self) -> Option<&BoardOwnedEvent> {
        self.events.get(usize::from(self.cursor))?.as_ref()
    }

    pub fn pop(&mut self) -> Option<BoardOwnedEvent> {
        if self.cursor == self.len {
            return None;
        }
        let event = self.events[usize::from(self.cursor)].take();
        self.cursor += 1;
        event
    }

    pub fn is_empty(&self) -> bool {
        self.cursor == self.len
    }
}

impl BoardOwnedEvent {
    pub fn from_payload(kind: BoardEventKind, payload: &str, key: Option<&str>) -> Result<Self, BoardEventFault> {
        if payload.len() > BOARD_EVENT_PAYLOAD_BYTE_CAPACITY {
            return Err(BoardEventFault::ByteCredits);
        }
        let key = key.unwrap_or_default();
        if key.len() > BOARD_EVENT_KEY_BYTE_CAPACITY {
            return Err(BoardEventFault::KeyCredits);
        }
        let mut owned = Self { kind, payload: Box::new([0; BOARD_EVENT_PAYLOAD_BYTE_CAPACITY]), payload_len: payload.len() as u16, key: [0; BOARD_EVENT_KEY_BYTE_CAPACITY], key_len: key.len() as u16 };
        owned.payload[..payload.len()].copy_from_slice(payload.as_bytes());
        owned.key[..key.len()].copy_from_slice(key.as_bytes());
        Ok(owned)
    }

    pub fn kind(&self) -> BoardEventKind {
        self.kind
    }

    pub fn payload_json(&self) -> &str {
        std::str::from_utf8(&self.payload[..usize::from(self.payload_len)]).expect("board event payload is UTF-8")
    }

    pub fn key(&self) -> Option<&str> {
        (self.key_len > 0).then(|| std::str::from_utf8(&self.key[..usize::from(self.key_len)]).expect("board event key is UTF-8"))
    }

    pub fn owned_bytes(&self) -> usize {
        usize::from(self.payload_len) + usize::from(self.key_len)
    }

    pub fn write_json(&self, output: &mut String) {
        output.push_str("{\"name\":\"");
        output.push_str(self.kind.name());
        output.push_str("\",\"payload\":");
        output.push_str(self.payload_json());
        output.push('}');
    }

    pub fn selection(kind: BoardEventKind, ids: &[String], anchor_ids: Option<&[String]>, removed_ids: Option<&[String]>, gesture: Option<&str>) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"ids\":")?;
        payload.string_array(ids.iter().map(String::as_str))?;
        if let Some(anchor_ids) = anchor_ids {
            payload.raw(",\"anchorIds\":")?;
            payload.string_array(anchor_ids.iter().map(String::as_str))?;
        }
        if let Some(removed_ids) = removed_ids {
            payload.raw(",\"removedIds\":")?;
            payload.string_array(removed_ids.iter().map(String::as_str))?;
        }
        if kind == BoardEventKind::Select {
            payload.raw(",\"exitHighlightIds\":[]")?;
        }
        if let Some(gesture) = gesture {
            payload.raw(",\"gestureMergeMode\":")?;
            payload.string(gesture)?;
        }
        payload.raw("}")?;
        payload.finish(kind, None)
    }

    pub fn camera(x: f64, y: f64, zoom: f64) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"x\":")?;
        payload.number(x)?;
        payload.raw(",\"y\":")?;
        payload.number(y)?;
        payload.raw(",\"zoom\":")?;
        payload.number(zoom)?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::Camera, None)
    }

    pub fn id(kind: BoardEventKind, id: &str) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"id\":")?;
        payload.string(id)?;
        payload.raw("}")?;
        payload.finish(kind, None)
    }

    pub fn id_list<'a>(kind: BoardEventKind, field: &str, ids: impl IntoIterator<Item = &'a str>) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{")?;
        payload.string(field)?;
        payload.raw(":[")?;
        for (count, id) in ids.into_iter().enumerate() {
            if count == BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardEventFault::ItemCredits);
            }
            if count > 0 {
                payload.raw(",")?;
            }
            payload.string(id)?;
        }
        payload.raw("]}")?;
        payload.finish(kind, None)
    }

    pub fn preselect_sets(ids: &BTreeSet<String>, removed_ids: &BTreeSet<String>, gesture: Option<&str>) -> Result<Self, BoardEventFault> {
        if ids.len() > BOARD_POINTER_ITEM_CAPACITY || removed_ids.len() > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardEventFault::ItemCredits);
        }
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"ids\":")?;
        payload.string_array(ids.iter().map(String::as_str))?;
        payload.raw(",\"removedIds\":")?;
        payload.string_array(removed_ids.iter().map(String::as_str))?;
        if let Some(gesture) = gesture {
            payload.raw(",\"gestureMergeMode\":")?;
            payload.string(gesture)?;
        }
        payload.raw("}")?;
        payload.finish(BoardEventKind::Preselect, None)
    }

    pub fn select_set(ids: &BTreeSet<String>, anchor_ids: Option<&BTreeSet<String>>, gesture: Option<&str>) -> Result<Self, BoardEventFault> {
        if ids.len() > BOARD_POINTER_ITEM_CAPACITY || anchor_ids.is_some_and(|anchor_ids| anchor_ids.len() > BOARD_POINTER_ITEM_CAPACITY) {
            return Err(BoardEventFault::ItemCredits);
        }
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"ids\":")?;
        payload.string_array(ids.iter().map(String::as_str))?;
        if let Some(anchor_ids) = anchor_ids {
            payload.raw(",\"anchorIds\":")?;
            payload.string_array(anchor_ids.iter().map(String::as_str))?;
        }
        payload.raw(",\"exitHighlightIds\":[]")?;
        if let Some(gesture) = gesture {
            payload.raw(",\"gestureMergeMode\":")?;
            payload.string(gesture)?;
        }
        payload.raw("}")?;
        payload.finish(BoardEventKind::Select, None)
    }

    pub fn edge(kind: BoardEventKind, id: &str, source: &str, target: &str) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"id\":")?;
        payload.string(id)?;
        payload.raw(",\"source\":")?;
        payload.string(source)?;
        payload.raw(",\"target\":")?;
        payload.string(target)?;
        payload.raw("}")?;
        payload.finish(kind, None)
    }

    pub fn node_move(id: &str, x: f64, y: f64) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"id\":")?;
        payload.string(id)?;
        payload.raw(",\"x\":")?;
        payload.number(x)?;
        payload.raw(",\"y\":")?;
        payload.number(y)?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::NodeMove, Some(id))
    }

    /// 🔄️ The live rotate-ring frame as resolved positions — a TRANSIENT row hosts mirror into
    /// sibling panes and drop before dispatch, so a drag previews everywhere and still commits once.
    pub fn transform_preview<'a>(moves: impl IntoIterator<Item = (&'a str, f64, f64)>) -> Result<Self, BoardEventFault> {
        Self::moves_row(BoardEventKind::TransformPreview, moves)
    }

    /// 🕹️ The selection a gesture's press staged, published at its release. `gesture_id` tags it when the
    /// release also carries the gesture record, so both leave as one dispatch.
    fn staged_select(ids: &[String], merge: Option<&str>, gesture_id: Option<&str>) -> Result<Self, BoardEventFault> {
        if ids.len() > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardEventFault::ItemCredits);
        }
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"ids\":")?;
        payload.string_array(ids.iter().map(String::as_str))?;
        payload.raw(",\"exitHighlightIds\":[]")?;
        if let Some(merge) = merge {
            payload.raw(",\"gestureMergeMode\":")?;
            payload.string(merge)?;
        }
        if let Some(gesture_id) = gesture_id {
            payload.raw(",\"gestureId\":")?;
            payload.string(gesture_id)?;
        }
        payload.raw("}")?;
        payload.finish(BoardEventKind::Select, None)
    }

    /// 🎬️ ONE finished pointer gesture: its id, kind, the ids it acted on, its motion and the proximity
    /// handle pairs it recorded — never positions, so the record is a parametric input a guest can yield
    /// as one editable mutation.
    pub fn gesture<'a>(gesture_id: &str, targets: impl IntoIterator<Item = &'a str>, motion: BoardGestureMotion, proximity: &[(String, String)]) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"gestureId\":")?;
        payload.string(gesture_id)?;
        payload.raw(",\"kind\":")?;
        payload.string(match motion {
            BoardGestureMotion::Drag { .. } => "drag",
            BoardGestureMotion::Rotate { .. } => "rotate",
        })?;
        payload.raw(",\"targets\":[")?;
        let mut count = 0usize;
        for id in targets {
            if count == BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardEventFault::ItemCredits);
            }
            if count > 0 {
                payload.raw(",")?;
            }
            payload.string(id)?;
            count += 1;
        }
        if count == 0 {
            return Err(BoardEventFault::Schema);
        }
        payload.raw("]")?;
        match motion {
            BoardGestureMotion::Drag { dx, dy } => {
                payload.raw(",\"dx\":")?;
                payload.number(board_pointer_offset(dx))?;
                payload.raw(",\"dy\":")?;
                payload.number(board_pointer_offset(dy))?;
            }
            BoardGestureMotion::Rotate { pivot, angle } => {
                payload.raw(",\"pivotX\":")?;
                payload.number(pivot.x)?;
                payload.raw(",\"pivotY\":")?;
                payload.number(pivot.y)?;
                payload.raw(",\"angle\":")?;
                payload.number(angle)?;
            }
        }
        payload.raw(",\"proximity\":[")?;
        for (index, (source, target)) in proximity.iter().enumerate() {
            if index > 0 {
                payload.raw(",")?;
            }
            payload.raw("{\"source\":")?;
            payload.string(source)?;
            payload.raw(",\"target\":")?;
            payload.string(target)?;
            payload.raw("}")?;
        }
        payload.raw("]}")?;
        payload.finish(BoardEventKind::Gesture, None)
    }

    fn moves_row<'a>(kind: BoardEventKind, moves: impl IntoIterator<Item = (&'a str, f64, f64)>) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"moves\":[")?;
        let mut count = 0usize;
        for (id, x, y) in moves {
            if count == BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardEventFault::ItemCredits);
            }
            if count > 0 {
                payload.raw(",")?;
            }
            payload.raw("{\"id\":")?;
            payload.string(id)?;
            payload.raw(",\"x\":")?;
            payload.number(x)?;
            payload.raw(",\"y\":")?;
            payload.number(y)?;
            payload.raw("}")?;
            count += 1;
        }
        if count == 0 {
            return Err(BoardEventFault::Schema);
        }
        payload.raw("]}")?;
        payload.finish(kind, None)
    }

    /// 🎯️ One painted target region, as the normalized rectangle the brush released on. The id is
    /// minted by the document, never here — the engine paints geometry, the guest owns identity.
    pub fn region_create(bounds: [f64; 4]) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"x\":")?;
        payload.number(bounds[0])?;
        payload.raw(",\"y\":")?;
        payload.number(bounds[1])?;
        payload.raw(",\"width\":")?;
        payload.number(bounds[2] - bounds[0])?;
        payload.raw(",\"height\":")?;
        payload.number(bounds[3] - bounds[1])?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::RegionCreate, None)
    }

    /// 📐️ A region grip drag committed on release: corner AND extent, because a west/north grip
    /// moves the minimum corner as well as the size.
    pub fn region_resize(id: &str, bounds: [f64; 4]) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"id\":")?;
        payload.string(id)?;
        payload.raw(",\"x\":")?;
        payload.number(bounds[0])?;
        payload.raw(",\"y\":")?;
        payload.number(bounds[1])?;
        payload.raw(",\"width\":")?;
        payload.number(bounds[2] - bounds[0])?;
        payload.raw(",\"height\":")?;
        payload.number(bounds[3] - bounds[1])?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::RegionResize, Some(id))
    }

    pub fn link_compatible(source: &str, node_ids: &[String]) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"source\":")?;
        payload.string(source)?;
        payload.raw(",\"nodeIds\":")?;
        payload.string_array(node_ids.iter().map(String::as_str))?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::LinkCompatibleNodes, None)
    }

    pub fn link_ring(source: &str, node_id: Option<&str>, handle_ids: &[String]) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"source\":")?;
        payload.string(source)?;
        payload.raw(",\"nodeId\":")?;
        if let Some(node_id) = node_id {
            payload.string(node_id)?;
        } else {
            payload.raw("null")?;
        }
        payload.raw(",\"handleIds\":")?;
        payload.string_array(handle_ids.iter().map(String::as_str))?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::LinkTargetRing, None)
    }

    pub fn hover(id: Option<&str>, kind: Option<(&str, &str)>) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"id\":")?;
        if let Some(id) = id {
            payload.string(id)?;
        } else {
            payload.raw("null")?;
        }
        payload.raw(",\"kind\":")?;
        if let Some((domain, kind_id)) = kind {
            payload.raw("{\"domain\":")?;
            payload.string(domain)?;
            payload.raw(",\"kindId\":")?;
            payload.string(kind_id)?;
            payload.raw("}")?;
        } else {
            payload.raw("null")?;
        }
        payload.raw("}")?;
        payload.finish(BoardEventKind::Hover, None)
    }

    fn brush_handles(payload: &mut BoardPayloadBuilder, handles: &[NodeKindHandleTemplate]) -> Result<(), BoardEventFault> {
        if handles.len() > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardEventFault::ItemCredits);
        }
        payload.raw("[")?;
        for (index, handle) in handles.iter().enumerate() {
            if index > 0 {
                payload.raw(",")?;
            }
            payload.raw("{\"angle\":")?;
            payload.number(handle.angle)?;
            payload.raw(",\"handleKind\":")?;
            payload.string(&handle.handle_kind)?;
            if let Some(radius) = handle.radius {
                payload.raw(",\"radius\":")?;
                payload.number(radius)?;
            }
            payload.raw("}")?;
        }
        payload.raw("]")
    }

    pub fn brush_preview(preview: Option<&BrushPreviewSnapshot>) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        let Some(preview) = preview else {
            payload.raw("{\"node\":null,\"edge\":null}")?;
            return payload.finish(BoardEventKind::BrushPreview, None);
        };
        payload.raw("{\"node\":{\"nodeKind\":")?;
        payload.string(&preview.node_kind_id)?;
        payload.raw(",\"x\":")?;
        payload.number(preview.x)?;
        payload.raw(",\"y\":")?;
        payload.number(preview.y)?;
        payload.raw(",\"shape\":")?;
        payload.string(if preview.shape == NodeShape::Rectangle { "rectangle" } else { "circle" })?;
        if preview.shape == NodeShape::Rectangle {
            payload.raw(",\"width\":")?;
            payload.number(preview.width)?;
            payload.raw(",\"height\":")?;
            payload.number(preview.height)?;
        } else {
            payload.raw(",\"radius\":")?;
            payload.number(preview.radius)?;
        }
        if let Some(icon) = &preview.icon_kind {
            payload.raw(",\"iconKind\":")?;
            payload.string(icon)?;
        }
        payload.raw(",\"handles\":")?;
        Self::brush_handles(&mut payload, &preview.handles)?;
        payload.raw("},\"edge\":{\"sourceHandleId\":")?;
        payload.string(&preview.source_handle_id)?;
        payload.raw(",\"targetHandleIndex\":")?;
        payload.usize(preview.target_handle_index)?;
        payload.raw("}}")?;
        payload.finish(BoardEventKind::BrushPreview, None)
    }

    pub fn brush_candidates(source: &str, candidates: &BrushCandidatePage, index: usize, suggestions_active: bool) -> Result<Self, BoardEventFault> {
        if candidates.len() > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardEventFault::ItemCredits);
        }
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"sourceHandleId\":")?;
        payload.string(source)?;
        payload.raw(",\"candidates\":[")?;
        for (candidate_index, candidate) in candidates.iter().enumerate() {
            if candidate_index > 0 {
                payload.raw(",")?;
            }
            payload.raw("{\"nodeKind\":")?;
            payload.string(candidate.node_kind_id)?;
            payload.raw(",\"targetHandleIndex\":")?;
            payload.usize(candidate.target_handle_index)?;
            payload.raw("}")?;
        }
        payload.raw("],\"index\":")?;
        payload.usize(index)?;
        payload.raw(",\"suggestionsActive\":")?;
        payload.boolean(suggestions_active)?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::BrushCandidates, None)
    }

    pub fn brush_place(preview: &BrushPreviewSnapshot, node_id: &str, edge_id: &str) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"nodeId\":")?;
        payload.string(node_id)?;
        payload.raw(",\"edgeId\":")?;
        payload.string(edge_id)?;
        payload.raw(",\"nodeKind\":")?;
        payload.string(&preview.node_kind_id)?;
        payload.raw(",\"sourceHandleId\":")?;
        payload.string(&preview.source_handle_id)?;
        payload.raw(",\"targetHandleIndex\":")?;
        payload.usize(preview.target_handle_index)?;
        payload.raw(",\"x\":")?;
        payload.number(preview.x)?;
        payload.raw(",\"y\":")?;
        payload.number(preview.y)?;
        payload.raw(",\"shape\":")?;
        payload.string(if preview.shape == NodeShape::Rectangle { "rectangle" } else { "circle" })?;
        if preview.shape == NodeShape::Rectangle {
            payload.raw(",\"width\":")?;
            payload.number(preview.width)?;
            payload.raw(",\"height\":")?;
            payload.number(preview.height)?;
        } else {
            payload.raw(",\"radius\":")?;
            payload.number(preview.radius)?;
        }
        if let Some(icon) = &preview.icon_kind {
            payload.raw(",\"iconKind\":")?;
            payload.string(icon)?;
        }
        payload.raw(",\"handles\":")?;
        Self::brush_handles(&mut payload, &preview.handles)?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::BrushPlace, None)
    }
}

pub struct BoardEventQueue {
    pub slots: Box<[Option<BoardOwnedEvent>; BOARD_EVENT_ITEM_CAPACITY]>,
    head: u16,
    pub len: u16,
    pub bytes: usize,
    pub claimed_items: u16,
    pub claimed_bytes: usize,
    pub closing: bool,
}

impl Default for BoardEventQueue {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), head: 0, len: 0, bytes: 0, claimed_items: 0, claimed_bytes: 0, closing: false }
    }
}

impl BoardEventQueue {
    pub fn reserve(&self, items: usize, bytes: usize) -> Result<(), BoardEventFault> {
        if self.closing || usize::from(self.len).checked_add(usize::from(self.claimed_items)).and_then(|value| value.checked_add(items)).is_none_or(|value| value > BOARD_EVENT_ITEM_CAPACITY) {
            return Err(BoardEventFault::ItemCredits);
        }
        if self.bytes.checked_add(self.claimed_bytes).and_then(|value| value.checked_add(bytes)).is_none_or(|value| value > BOARD_EVENT_BYTE_CAPACITY) {
            return Err(BoardEventFault::ByteCredits);
        }
        Ok(())
    }

    pub fn claim(&mut self, items: usize, bytes: usize) -> Result<(), BoardEventFault> {
        self.reserve(items, bytes)?;
        let items = u16::try_from(items).map_err(|_| BoardEventFault::ItemCredits)?;
        self.claimed_items += items;
        self.claimed_bytes += bytes;
        Ok(())
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn push_claimed(&mut self, event: BoardOwnedEvent) -> Result<(), BoardOwnedEvent> {
        let bytes = event.owned_bytes();
        if self.closing || self.claimed_items == 0 || self.claimed_bytes < bytes || usize::from(self.len) == BOARD_EVENT_ITEM_CAPACITY {
            return Err(event);
        }
        self.claimed_items -= 1;
        self.claimed_bytes -= bytes;
        let index = (usize::from(self.head) + usize::from(self.len)) % BOARD_EVENT_ITEM_CAPACITY;
        self.bytes += bytes;
        self.slots[index] = Some(event);
        self.len += 1;
        Ok(())
    }

    pub fn release_claim(&mut self, items: usize, bytes: usize) -> bool {
        let Ok(items) = u16::try_from(items) else {
            return false;
        };
        if self.claimed_items < items || self.claimed_bytes < bytes {
            return false;
        }
        self.claimed_items -= items;
        self.claimed_bytes -= bytes;
        true
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn push(&mut self, event: BoardOwnedEvent) -> Result<(), BoardOwnedEvent> {
        if self.reserve(1, event.owned_bytes()).is_err() {
            return Err(event);
        }
        let index = (usize::from(self.head) + usize::from(self.len)) % BOARD_EVENT_ITEM_CAPACITY;
        self.bytes += event.owned_bytes();
        self.slots[index] = Some(event);
        self.len += 1;
        Ok(())
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn reserve_event(&self, event: BoardOwnedEvent) -> Result<BoardEventReservation, BoardOwnedEvent> {
        if self.reserve(1, event.owned_bytes()).is_err() {
            return Err(event);
        }
        Ok(BoardEventReservation { expected_len: self.len, expected_bytes: self.bytes, event: Some(event) })
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn publish_reserved(&mut self, mut reservation: BoardEventReservation) -> Result<(), BoardEventReservation> {
        if self.closing || self.len != reservation.expected_len || self.bytes != reservation.expected_bytes {
            return Err(reservation);
        }
        let event = reservation.event.take().expect("board reservation owns one event");
        let index = (usize::from(self.head) + usize::from(self.len)) % BOARD_EVENT_ITEM_CAPACITY;
        self.bytes += event.owned_bytes();
        self.slots[index] = Some(event);
        self.len += 1;
        Ok(())
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn reserve_batch(&self, mut reservation: BoardEventBatchReservation) -> Result<BoardEventBatchReservation, BoardEventBatchReservation> {
        let Some(bytes) = reservation.owned_bytes() else {
            return Err(reservation);
        };
        if self.reserve(usize::from(reservation.len), bytes).is_err() {
            return Err(reservation);
        }
        reservation.expected_len = self.len;
        reservation.expected_bytes = self.bytes;
        Ok(reservation)
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn publish_batch(&mut self, mut reservation: BoardEventBatchReservation) -> Result<(), BoardEventBatchReservation> {
        if self.closing || self.len != reservation.expected_len || self.bytes != reservation.expected_bytes {
            return Err(reservation);
        }
        for index in 0..usize::from(reservation.len) {
            let event = reservation.events[index].take().expect("board batch slot is occupied");
            let slot = (usize::from(self.head) + usize::from(self.len)) % BOARD_EVENT_ITEM_CAPACITY;
            self.bytes += event.owned_bytes();
            self.slots[slot] = Some(event);
            self.len += 1;
        }
        Ok(())
    }

    pub fn pop(&mut self) -> Option<BoardOwnedEvent> {
        if self.len == 0 {
            return None;
        }
        let index = usize::from(self.head);
        let event = self.slots[index].take()?;
        self.bytes -= event.owned_bytes();
        self.head = ((index + 1) % BOARD_EVENT_ITEM_CAPACITY) as u16;
        self.len -= 1;
        Some(event)
    }

    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &BoardOwnedEvent> {
        (0..usize::from(self.len)).filter_map(|offset| self.slots[(usize::from(self.head) + offset) % BOARD_EVENT_ITEM_CAPACITY].as_ref())
    }

    pub fn owned_bytes(&self) -> usize {
        self.bytes
    }

    pub fn close_step(&mut self) -> bool {
        self.closing = true;
        if self.pop().is_some() {
            return false;
        }
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.bytes == 0 && self.claimed_items == 0 && self.claimed_bytes == 0
    }
}

/// 🎨️ One node's icon commands inside [`WorldContentCache::scene`], baked at `(x, y)`.
pub struct NodeIconSpan {
    pub id: String,
    pub start: usize,
    pub end: usize,
    pub x: f64,
    pub y: f64,
}

/// 🎨️ World-space node icons reused across frames. Spans stay valid under translation: a drag
/// draws each span at `current - baked` until retirement frees a slot for a fresh bake.
pub struct WorldContentCache {
    pub generation: u64,
    pub lod: BoardDrawLod,
    pub scene: Scene,
    pub spans: Vec<NodeIconSpan>,
}

#[cfg(any(test, feature = "testkit"))]
    pub fn board_property_retirement_credits(properties: &semio_framework_graph_manifest::PropertyBag) -> Result<(usize, usize), BoardEventFault> {
        let mut stack: Box<[Option<(&semio_framework_graph_manifest::PropertyValue, u16)>; BOARD_POINTER_ITEM_CAPACITY]> = Box::new(std::array::from_fn(|_| None));
        let mut stack_len = 0usize;
        let mut nodes = 0usize;
        let mut bytes = 0usize;
        for (key, value) in properties {
            bytes = bytes.checked_add(key.len()).ok_or(BoardEventFault::ByteCredits)?;
            if stack_len == BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardEventFault::ItemCredits);
            }
            stack[stack_len] = Some((value, 1));
            stack_len += 1;
        }
        while stack_len > 0 {
            stack_len -= 1;
            let (value, depth) = stack[stack_len].take().expect("property pre-admission stack owner");
            nodes += 1;
            if nodes > BOARD_POINTER_ITEM_CAPACITY || usize::from(depth) > BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardEventFault::ItemCredits);
            }
            match value {
                semio_framework_graph_manifest::PropertyValue::String(value) => {
                    bytes = bytes.checked_add(value.len()).ok_or(BoardEventFault::ByteCredits)?;
                }
                semio_framework_graph_manifest::PropertyValue::Array(values) => {
                    for value in values {
                        if stack_len == BOARD_POINTER_ITEM_CAPACITY {
                            return Err(BoardEventFault::ItemCredits);
                        }
                        stack[stack_len] = Some((value, depth + 1));
                        stack_len += 1;
                    }
                }
                semio_framework_graph_manifest::PropertyValue::Object(values) => {
                    for (key, value) in values {
                        bytes = bytes.checked_add(key.len()).ok_or(BoardEventFault::ByteCredits)?;
                        if stack_len == BOARD_POINTER_ITEM_CAPACITY {
                            return Err(BoardEventFault::ItemCredits);
                        }
                        stack[stack_len] = Some((value, depth + 1));
                        stack_len += 1;
                    }
                }
                semio_framework_graph_manifest::PropertyValue::Null | semio_framework_graph_manifest::PropertyValue::Bool(_) | semio_framework_graph_manifest::PropertyValue::Number(_) => {}
            }
            if bytes > BOARD_POINTER_BYTE_CAPACITY {
                return Err(BoardEventFault::ByteCredits);
            }
        }
        Ok((nodes, bytes))
    }

/// 🧮️ Entities (nodes + handles + edges + wires) one board descriptor, selection or gesture payload
/// may carry. Sized for the largest shipped board example: Nakagin Capsule Tower is 180 nodes
/// + 358 handles + 179 edges = 717 entities — the old 256 refused it silently (`load_board_snapshot_json`
/// → `false`, three empty panes, 2026-09-16).
pub const BOARD_POINTER_ITEM_CAPACITY: usize = 1_024;

/// 🧮️ Entity-id bytes the same payloads may carry — Nakagin's UUID ids total ~29 KiB; kept under the
/// guest's 64 KiB contiguous-request ceiling since every owner here is one boxed array.
pub const BOARD_POINTER_BYTE_CAPACITY: usize = 48 * 1024;

/// 🧮️ Entities one board DESCRIPTOR may carry — the render/hit-test census, decoupled from the
/// gesture payload credits above: a 100-placement fill of an 11-handle kind is 1 200
/// entities, and a board must keep painting past what one pointer payload may name (a whole-board
/// selection above the pointer credits is refused by `select_set`'s own `ItemCredits` law instead).
pub const BOARD_DESCRIPTOR_ITEM_CAPACITY: usize = 8_192;

/// 🧮️ Entity-id bytes one board descriptor may carry (UUID ids: 8 192 × ~45 B).
pub const BOARD_DESCRIPTOR_BYTE_CAPACITY: usize = 384 * 1024;

#[derive(Clone, Copy, Debug)]
pub struct BoardPointerSpan {
    pub start: u16,
    pub len: u16,
}

/// 🛂️ Every refusal a descriptor sync can raise BEFORE it touches the scene: the two fixed
/// descriptor ceilings and the handle colours it would have to parse. A sync that fails halfway
/// leaves a half-built board, and a snapshot parse that clears first leaves an EMPTY one (three
/// blank panes, 2026-09-17), so both preflight through here and mutate only once it passes.
pub fn descriptor_admission(desc: &SceneDescriptor) -> Result<(), NormalPortError> {
    let entity_count = desc
        .nodes
        .len()
        .checked_add(desc.handles.len())
        .and_then(|count| count.checked_add(desc.edges.len()))
        .and_then(|count| count.checked_add(desc.wires.len()))
        .and_then(|count| count.checked_add(desc.regions.len()))
        .ok_or(NormalPortError::EventCredits)?;
    if entity_count > BOARD_DESCRIPTOR_ITEM_CAPACITY {
        return Err(NormalPortError::EventCredits);
    }
    let entity_id_bytes = desc
        .nodes
        .iter()
        .map(|node| node.id.len())
        .chain(desc.handles.iter().map(|handle| handle.id.len()))
        .chain(desc.edges.iter().map(|edge| edge.id.len()))
        .chain(desc.wires.iter().map(|wire| wire.id.len()))
        .chain(desc.regions.iter().map(|region| region.id.len()))
        .try_fold(0usize, usize::checked_add)
        .ok_or(NormalPortError::EventCredits)?;
    if entity_id_bytes > BOARD_DESCRIPTOR_BYTE_CAPACITY {
        return Err(NormalPortError::EventCredits);
    }
    for handle in &desc.handles {
        if let Some(color) = handle.color.as_ref().map(|value| value.trim()).filter(|value| !value.is_empty()) {
            parse_css_color(color).ok_or_else(|| NormalPortError::InvalidHandleColor(handle.id.clone(), color.to_string()))?;
        }
    }
    Ok(())
}

/// 🧾️ The pure half of [`Self::load_board_snapshot_json`]: one snapshot document into one scene
/// descriptor, or `None` for a document this port refuses. Touches no board state.
///
/// 🎯️ Target regions are optional by construction (the artifact omits an empty collection),
/// so a malformed row is skipped rather than refusing the whole document: a board that
/// cannot paint its constraint rectangles must still paint its graph.
pub fn board_snapshot_scene_descriptor(f:BoardSnapshot,has_ports:bool)->Option<SceneDescriptor>{
    let text=|value:Option<String>|value.map(|v|v.trim().to_owned()).filter(|v|!v.is_empty());
    let positive=|value:Option<f64>|value.filter(|v|v.is_finite()&&*v>0.0);
    let mut desc=SceneDescriptor::default();
    for node in f.nodes {
        let(x,y)=(node.x?,node.y?);if !x.is_finite()||!y.is_finite(){return None;}
        let visible=board_visible_option(&node.visibility());let locked=board_locked_option(&node.visibility());
        if has_ports {for h in node.handles? {let angle=h.angle?;if !angle.is_finite(){return None;}let hv=board_visible_option(&h.visibility());let hl=board_locked_option(&h.visibility());desc.handles.push(HandleDescriptor{id:h.id,node_id:node.id.clone(),angle,radius:positive(h.radius),scale:positive(h.scale),selected:None,style:None,handle_kind:Some(text(h.handle_kind).unwrap_or_else(||"port".into())),color:text(h.color),icon_kind:text(h.icon_kind),user_data:None,visible:hv,locked:hl});}}
        else if node.handles.is_some(){return None;}
        let (shape,radius,width,height)=if node.shape==Some(BoardNodeShape::Rectangle){("rectangle",None,Some(positive(node.width)?),Some(positive(node.height)?))}else{("circle",Some(positive(node.radius)?),None,None)};
        desc.nodes.push(NodeDescriptor{id:node.id,x,y,draggable:None,selected:None,style:None,text:text(node.text),icon_kind:text(node.icon_kind),node_kind:text(node.node_kind),user_data:None,visible,locked,root:node.root,shape:Some(shape.into()),radius,width,height,scale:positive(node.scale)});
    }
    for edge in f.edges {let visible=board_visible_option(&edge.visibility());let locked=board_locked_option(&edge.visibility());let(source,target)=(edge.source?,edge.target?);if !has_ports{let ids:BTreeSet<&str>=desc.nodes.iter().map(|n|n.id.as_str()).collect();if !ids.contains(source.as_str())||!ids.contains(target.as_str()){return None;}}
        desc.edges.push(EdgeDescriptor{id:edge.id,source,target,edge_kind:text(edge.edge_kind),source_tip:text(edge.source_tip),target_tip:text(edge.target_tip),selected:None,style:None,user_data:None,visible,locked});}
    for region in f.target_regions {let Some(id)=text(Some(region.id))else{continue;};let finite=|value:Option<f64>|value.filter(|v|v.is_finite()).unwrap_or(0.0);desc.regions.push(RegionDescriptor{id,x:finite(region.x),y:finite(region.y),width:finite(region.width),height:finite(region.height),label:text(region.label),hidden:region.hidden,locked:region.locked,selected:region.selected});}Some(desc)
}
