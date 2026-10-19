//! 🎯️ Marquee, gizmo and icon atlas: the selection marquee, the mesh content version, the gizmo module, the ear clipping and the icon atlas. See [`super`].

use super::*;

// #region draw_types
pub const SELECTION_MARQUEE_STROKE_WIDTH: f32 = 1.5;

pub const SELECTION_MARQUEE_FILL_ALPHA: f32 = 0.12;

pub const SELECTION_MARQUEE_DASH_LEN: f32 = 5.0;

pub const SELECTION_MARQUEE_DASH_GAP: f32 = 4.0;

pub fn selection_marquee_stroke(theme: &Theme) -> Rgba {
    theme.selected
}

pub fn selection_marquee_fill(theme: &Theme) -> Rgba {
    theme.selected.with_alpha(SELECTION_MARQUEE_FILL_ALPHA)
}

pub(super) fn dashed_line_segments(x0: f32, y0: f32, x1: f32, y1: f32, dash: f32, gap: f32) -> Vec<(f32, f32, f32, f32)> {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let len = (dx * dx + dy * dy).sqrt().max(0.001);
    let ux = dx / len;
    let uy = dy / len;
    let mut traveled = 0.0f32;
    let mut drawing = true;
    let mut segments = Vec::new();
    while traveled < len {
        let segment = if drawing { dash } else { gap };
        let next = (traveled + segment).min(len);
        if drawing {
            segments.push((x0 + ux * traveled, y0 + uy * traveled, x0 + ux * next, y0 + uy * next));
        }
        traveled = next;
        drawing = !drawing;
    }
    segments
}

#[allow(clippy::too_many_arguments, reason = "one arg per line endpoint/style attribute; grouping into a struct is a T2 restructure, out of scope")]
fn push_marquee_segment(draw: &mut DrawList, overlay: bool, x0: f32, y0: f32, x1: f32, y1: f32, stroke: Rgba, dashed: bool) {
    if dashed {
        if overlay {
            draw.push_dashed_line_overlay(x0, y0, x1, y1, stroke, SELECTION_MARQUEE_STROKE_WIDTH, SELECTION_MARQUEE_DASH_LEN, SELECTION_MARQUEE_DASH_GAP);
        } else {
            draw.push_dashed_line(x0, y0, x1, y1, stroke, SELECTION_MARQUEE_STROKE_WIDTH, SELECTION_MARQUEE_DASH_LEN, SELECTION_MARQUEE_DASH_GAP);
        }
    } else if overlay {
        draw.push_line_overlay(x0, y0, x1, y1, stroke, SELECTION_MARQUEE_STROKE_WIDTH);
    } else {
        draw.push_line(x0, y0, x1, y1, stroke, SELECTION_MARQUEE_STROKE_WIDTH);
    }
}

pub fn paint_selection_marquee(draw: &mut DrawList, theme: &Theme, crossing: bool, lasso: bool, points: &[[f32; 2]], overlay: bool) {
    if points.len() < 2 {
        return;
    }
    let stroke = selection_marquee_stroke(theme);
    let fill = selection_marquee_fill(theme);
    let dashed = crossing;
    if lasso {
        if points.len() >= 3 {
            if overlay {
                draw.push_triangle_fan_overlay(points, fill);
            } else {
                draw.push_triangle_fan(points, fill);
            }
        }
        for window in points.windows(2) {
            push_marquee_segment(draw, overlay, window[0][0], window[0][1], window[1][0], window[1][1], stroke, dashed);
        }
        let first = points[0];
        let last = points[points.len() - 1];
        push_marquee_segment(draw, overlay, last[0], last[1], first[0], first[1], stroke, dashed);
        return;
    }
    let start = points[0];
    let end = points[points.len() - 1];
    let rx = start[0].min(end[0]);
    let ry = start[1].min(end[1]);
    let rw = (end[0] - start[0]).abs();
    let rh = (end[1] - start[1]).abs();
    if overlay {
        draw.push_solid_overlay([rx, ry, rw, rh], fill);
    } else {
        draw.push_solid([rx, ry, rw, rh], fill);
    }
    push_marquee_segment(draw, overlay, start[0], start[1], end[0], start[1], stroke, dashed);
    push_marquee_segment(draw, overlay, end[0], start[1], end[0], end[1], stroke, dashed);
    push_marquee_segment(draw, overlay, end[0], end[1], start[0], end[1], stroke, dashed);
    push_marquee_segment(draw, overlay, start[0], end[1], start[0], start[1], stroke, dashed);
}

pub fn mesh_content_version(positions: &[f32], normals: &[f32], indices: &[u32]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for value in positions.iter().chain(normals.iter()) {
        hash ^= value.to_bits() as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for value in indices {
        hash ^= *value as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

//#region gizmo
/// 🧭️ Orbit-view gizmo placement/tip-geometry/hit-test math split out of `widgets.rs`'s gizmo
/// module (ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS) — pure
/// screen-space math with zero `WidgetContext`/`DrawList` touch, so it stays available for
/// `wasm32-wasip2` pointer-hover hit-testing without pulling `wgpu-engine`. The actual paint call
/// (`paint_orbit_view_gizmo`, which needs the GPU-adjacent `WidgetContext`) stays in
/// `widgets.rs`'s own `gizmo` module, still `wgpu-engine`-gated.
pub mod gizmo {
    use semio_framework_ui_scene_camera::Camera3d;
    use semio_framework_ui_scene_algebra::{Vec3, Vec3Math};
    use semio_framework_ui_theme::{Rgba, geometry::Rect};

    /// 🧭️ Permanent X/Y/Z paints — the generated `primary`/`secondary`/`tertiary` palette tokens,
    /// the exact triple React's `SPATIAL_AXIS_COLOR_REFS`/`resolveSpatialAxisColors` resolves. The
    /// previous hand-written channels were sRGB fractions stored in a linear-float `Rgba`, so every
    /// axis painted a different color than React's.
    pub fn spatial_axis_rgba(axis: u8, alpha: f32) -> Rgba {
        match axis {
            0 => Rgba::from_token(&ui_styling::colors::PRIMARY).with_alpha(alpha),
            1 => Rgba::from_token(&ui_styling::colors::SECONDARY).with_alpha(alpha),
            _ => Rgba::from_token(&ui_styling::colors::TERTIARY).with_alpha(alpha),
        }
    }

    /// 🧭️ Mirrors `resolveSceneGizmoViewportPlacement` (`🖱️ui/🧱️elements/🎬️Scene/🟦️.tsx:390`) — the
    /// navigation cube's CENTRE offset from the viewport's bottom-right corner. The two axes are NOT
    /// the same: the inline margin clears the pane's `--spacing-single` chrome inset, the block margin
    /// additionally clears the FOLDED projection pane (`h-medium`, 7 × ui-spacing) plus one more
    /// inset, so the cube sits directly above that chip instead of under it. Each axis is clamped
    /// against its OWN third of the viewport, never the shorter side's.
    ///
    /// 🩸️ This priced both axes at the inline margin, which put the cube on the pane's bottom edge —
    /// exactly where the `Utilities`/`Projection` row lands (React: `[32, 58]` at 1280×720, this:
    /// `(32, 32)`).
    pub fn orbit_view_gizmo_placement(viewport: Rect) -> (f32, f32) {
        let chrome_inset = 4.0_f32.max(ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX.round() as f32);
        let folded_pane_chrome = (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::chrome::CONTROL_HEIGHT_UI_SPACING).round() as f32;
        let gizmo_half_extent = 28.0_f32;
        let preferred_x = chrome_inset + gizmo_half_extent;
        let preferred_y = chrome_inset + folded_pane_chrome + chrome_inset + gizmo_half_extent;
        let max_fit = |extent: f32| (extent / 3.0).floor().max(22.0);
        (preferred_x.min(max_fit(viewport.w)), preferred_y.min(max_fit(viewport.h)))
    }

    /// 🧭️ Screen-space tip used for orbit-view gizmo hover hit-testing and paint.
    pub struct OrbitViewGizmoTip {
        pub screen_x: f32,
        pub screen_y: f32,
        pub depth: f32,
        pub pick_radius: f32,
        pub color: Rgba,
        pub is_corner: bool,
        pub prominent: bool,
    }

    /// 🔘️ Matches the visible circle inside React's 64px sprite at its 22px group scale.
    pub fn orbit_view_gizmo_head_radius(prominent: bool, hovered: bool) -> f32 {
        let sprite_scale = if prominent { 1.0 } else { 0.65 };
        let texture_radius = if prominent { 16.0 } else { 12.0 };
        22.0 * 0.62 * sprite_scale * texture_radius / 64.0 * if hovered { 1.1 } else { 1.0 }
    }

    pub fn orbit_view_gizmo_tips(camera: &Camera3d, viewport: Rect) -> Vec<OrbitViewGizmoTip> {
        let (margin_x, margin_y) = orbit_view_gizmo_placement(viewport);
        let origin_x = viewport.x + viewport.w - margin_x;
        let origin_y = viewport.y + viewport.h - margin_y;
        let axis_len = 22.0;
        let forward = camera.position.sub_m(camera.target);
        let forward_len = forward.length_m();
        if forward_len < 1e-5 {
            return Vec::new();
        }
        let forward = forward.scale_m(1.0 / forward_len);
        let right = camera.up.cross_m(forward);
        let right_len = right.length_m();
        if right_len < 1e-5 {
            return Vec::new();
        }
        let right = right.scale_m(1.0 / right_len);
        let up = forward.cross_m(right).normalize_m();
        let neutral = Rgba::from_token(&ui_styling::colors::GRAY).with_alpha(0.9);
        let axes = [
            (Vec3 { x: 1.0, y: 0.0, z: 0.0 }, spatial_axis_rgba(0, 1.0), true),
            (Vec3 { x: -1.0, y: 0.0, z: 0.0 }, spatial_axis_rgba(0, 0.75), false),
            (Vec3 { x: 0.0, y: 1.0, z: 0.0 }, spatial_axis_rgba(1, 1.0), true),
            (Vec3 { x: 0.0, y: -1.0, z: 0.0 }, spatial_axis_rgba(1, 0.75), false),
            (Vec3 { x: 0.0, y: 0.0, z: 1.0 }, spatial_axis_rgba(2, 1.0), true),
            (Vec3 { x: 0.0, y: 0.0, z: -1.0 }, spatial_axis_rgba(2, 0.75), false),
        ];
        let corners = [
            (Vec3 { x: 0.72, y: 0.72, z: 0.72 }, true),
            (Vec3 { x: -0.72, y: 0.72, z: 0.72 }, true),
            (Vec3 { x: 0.72, y: -0.72, z: 0.72 }, true),
            (Vec3 { x: -0.72, y: -0.72, z: 0.72 }, true),
            (Vec3 { x: 0.72, y: 0.72, z: -0.72 }, false),
            (Vec3 { x: -0.72, y: 0.72, z: -0.72 }, false),
            (Vec3 { x: 0.72, y: -0.72, z: -0.72 }, false),
            (Vec3 { x: -0.72, y: -0.72, z: -0.72 }, false),
        ];
        let mut tips: Vec<OrbitViewGizmoTip> = axes
            .into_iter()
            .map(|(axis, color, prominent)| {
                let sx = axis.dot_m(right);
                let sy = -axis.dot_m(up);
                let depth = axis.dot_m(forward);
                let tip_x = origin_x + sx * axis_len;
                let tip_y = origin_y + sy * axis_len;
                let pick_radius = if prominent { 10.0 } else { 7.0 };
                OrbitViewGizmoTip { screen_x: tip_x, screen_y: tip_y, depth, pick_radius, color, is_corner: false, prominent }
            })
            .chain(corners.into_iter().map(|(axis, prominent)| {
                let sx = axis.dot_m(right);
                let sy = -axis.dot_m(up);
                let depth = axis.dot_m(forward);
                let tip_x = origin_x + sx * axis_len;
                let tip_y = origin_y + sy * axis_len;
                let pick_radius = if prominent { 10.0 } else { 7.0 };
                OrbitViewGizmoTip { screen_x: tip_x, screen_y: tip_y, depth, pick_radius, color: neutral, is_corner: true, prominent }
            }))
            .collect();
        tips.push(OrbitViewGizmoTip { screen_x: origin_x, screen_y: origin_y, depth: 0.0, pick_radius: 9.0, color: neutral, is_corner: false, prominent: true });
        tips
    }

    pub fn orbit_view_gizmo_hit_test(x: f32, y: f32, tips: &[OrbitViewGizmoTip]) -> Option<usize> {
        tips.iter()
            .enumerate()
            .filter_map(|(index, tip)| {
                let distance = ((x - tip.screen_x).powi(2) + (y - tip.screen_y).powi(2)).sqrt();
                if distance <= tip.pick_radius + 3.0 {
                    Some((index, distance))
                } else {
                    None
                }
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(index, _)| index)
    }
}
//#endregion gizmo
// #endregion draw_types

pub fn ear_clip_polygon(points: &[[f32; 2]]) -> Vec<[f32; 2]> {
    if points.len() < 3 {
        return Vec::new();
    }
    let mut indices: Vec<usize> = (0..points.len()).collect();
    let mut triangles = Vec::new();
    let mut guard = 0usize;
    while indices.len() > 3 && guard < points.len() * points.len() {
        guard += 1;
        let mut ear_found = false;
        for i in 0..indices.len() {
            let prev = indices[(i + indices.len() - 1) % indices.len()];
            let curr = indices[i];
            let next = indices[(i + 1) % indices.len()];
            let a = points[prev];
            let b = points[curr];
            let c = points[next];
            let cross = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
            if cross <= 0.0 {
                continue;
            }
            let mut contains = false;
            for &idx in &indices {
                if idx == prev || idx == curr || idx == next {
                    continue;
                }
                let p = points[idx];
                if point_in_triangle(p, a, b, c) {
                    contains = true;
                    break;
                }
            }
            if contains {
                continue;
            }
            triangles.push(a);
            triangles.push(b);
            triangles.push(c);
            indices.remove(i);
            ear_found = true;
            break;
        }
        if !ear_found {
            break;
        }
    }
    if indices.len() == 3 {
        triangles.push(points[indices[0]]);
        triangles.push(points[indices[1]]);
        triangles.push(points[indices[2]]);
    }
    triangles
}

fn point_in_triangle(p: [f32; 2], a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> bool {
    let d1 = sign(p, a, b);
    let d2 = sign(p, b, c);
    let d3 = sign(p, c, a);
    let has_neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let has_pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
    !(has_neg && has_pos)
}

pub const ICON_ATLAS_TEXTURE_SIZE: u32 = 2048;

pub struct IconAtlas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
    entries: std::collections::HashMap<String, [f32; 4]>,
}

impl Default for IconAtlas {
    fn default() -> Self {
        Self { width: 1, height: 1, pixels: vec![0, 0, 0, 0], entries: std::collections::HashMap::new() }
    }
}

impl IconAtlas {
    pub fn from_packed(width: u32, height: u32, pixels: Vec<u8>, entries: Vec<(String, [f32; 4])>) -> Self {
        Self { width, height, pixels, entries: entries.into_iter().collect() }
    }

    pub fn icon_uv(&self, icon_id: &str) -> Option<[f32; 4]> {
        self.entries.get(icon_id).copied()
    }
}

fn sign(p1: [f32; 2], p2: [f32; 2], p3: [f32; 2]) -> f32 {
    (p1[0] - p3[0]) * (p2[1] - p3[1]) - (p2[0] - p3[0]) * (p1[1] - p3[1])
}
