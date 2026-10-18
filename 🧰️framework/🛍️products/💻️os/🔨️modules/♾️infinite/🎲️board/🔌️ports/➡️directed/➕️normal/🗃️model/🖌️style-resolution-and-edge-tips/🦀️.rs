//! 🖌️ Style resolution and edge tips of the board model: explicit style kinds, locked dimming, chrome passes, node, handle and edge strokes, colour interpolation, edge tips on curves and wire strokes. See [`super`].

use super::*;

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
