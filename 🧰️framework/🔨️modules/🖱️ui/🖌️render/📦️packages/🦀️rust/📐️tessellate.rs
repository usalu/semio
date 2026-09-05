//! @emoji 📐️ CPU tessellation: ear clipping, dashed-line/thick-line expansion, triangle fans, pixel
//! snapping and silhouette mask geometry. Every function here is pure — same inputs, same outputs,
//! no GPU type in sight — which is what makes [`crate::scene::Scene::finish`]'s `snap`/`batch` steps
//! deterministic and unit-testable without a device.

use crate::scene::{ClipRegion, QuadInstance, ScissorRect};

//#region 🔖️Tessellate

//#region PixelSnap

/// 🎯️ Rounds a logical coordinate to the nearest physical pixel at `dpr`, then converts back to
/// logical units. Applied to both edges of a rect independently (never position + size) so width
/// cannot drift a fraction of a physical pixel from what the two snapped edges actually span — that
/// drift is the shimmer this step exists to kill.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn snap_to_device_pixels(value: f32, dpr: f32) -> f32 {
    if dpr <= 0.0 {
        return value;
    }
    (value * dpr).round() / dpr
}

/// 🎯️ Snaps a `[x, y, w, h]` rect's edges (not its width/height) to the physical pixel grid at `dpr`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn snap_rect(rect: [f32; 4], dpr: f32) -> [f32; 4] {
    let x0 = snap_to_device_pixels(rect[0], dpr);
    let y0 = snap_to_device_pixels(rect[1], dpr);
    let x1 = snap_to_device_pixels(rect[0] + rect[2], dpr);
    let y1 = snap_to_device_pixels(rect[1] + rect[3], dpr);
    [x0, y0, (x1 - x0).max(0.0), (y1 - y0).max(0.0)]
}

/// 🎯️ Snaps a `[x, y]` point to the physical pixel grid at `dpr` — used for vector vertices, which
/// have no width/height to preserve.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn snap_point(point: [f32; 2], dpr: f32) -> [f32; 2] {
    [snap_to_device_pixels(point[0], dpr), snap_to_device_pixels(point[1], dpr)]
}

//#endregion PixelSnap

//#region Lines

/// ➖️ Expands a segment into a thick-line quad (two triangles, six positions) by offsetting
/// perpendicular to the segment direction by half `width`. Ported verbatim from the wgpu target's
/// `DrawList::push_line`, split from its color/layer-append side effects so the geometry is testable
/// on its own.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn thick_line_positions(x0: f32, y0: f32, x1: f32, y1: f32, width: f32) -> [[f32; 2]; 6] {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let len = (dx * dx + dy * dy).sqrt().max(0.001);
    let nx = -dy / len * width * 0.5;
    let ny = dx / len * width * 0.5;
    [[x0 + nx, y0 + ny], [x1 + nx, y1 + ny], [x0 - nx, y0 - ny], [x1 + nx, y1 + ny], [x1 - nx, y1 - ny], [x0 - nx, y0 - ny]]
}

/// 〰️ Splits a segment into alternating `dash`/`gap` sub-segments, ported verbatim from the wgpu
/// target's free function of the same name. The wgpu target's `push_dashed_line` called
/// `self.push_line(...)` — an `async fn` — as a bare statement without `.await`; the resulting
/// `Future` was constructed and immediately dropped, so every dashed line silently drew nothing. That
/// call site does not exist here: `SceneBuilder::push_dashed_line` calls this pure function and then
/// `SceneBuilder::push_line` directly, both plain sync `fn`, so the bug has no shape to recur in.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn dashed_line_segments(x0: f32, y0: f32, x1: f32, y1: f32, dash: f32, gap: f32) -> Vec<(f32, f32, f32, f32)> {
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

//#endregion Lines

//#region TriangleFan

/// 🔺️ Fans `points` into `[a, points[i], points[i + 1]]` triangles for `i` in `1..len - 1`. Ported
/// verbatim from the wgpu target's `DrawList::push_triangle_fan`, split from its color/layer-append
/// side effects.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn triangle_fan_positions(points: &[[f32; 2]]) -> Vec<[[f32; 2]; 3]> {
    if points.len() < 3 {
        return Vec::new();
    }
    (1..points.len() - 1).map(|tri| [points[0], points[tri], points[tri + 1]]).collect()
}

//#endregion TriangleFan

//#region EarClip

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn sign(p1: [f32; 2], p2: [f32; 2], p3: [f32; 2]) -> f32 {
    (p1[0] - p3[0]) * (p2[1] - p3[1]) - (p2[0] - p3[0]) * (p1[1] - p3[1])
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn point_in_triangle(p: [f32; 2], a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> bool {
    let d1 = sign(p, a, b);
    let d2 = sign(p, b, c);
    let d3 = sign(p, c, a);
    let has_neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let has_pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
    !(has_neg && has_pos)
}

/// ✂️ Ear-clipping triangulation for a simple (non-self-intersecting) polygon. Ported verbatim from
/// the wgpu target.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
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
                if point_in_triangle(points[idx], a, b, c) {
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

//#endregion EarClip

//#region SilhouetteMask

/// ⋃️ The smallest rect covering every piece, or `None` for an empty slice. Ported verbatim.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn union_scissors(scissors: &[ScissorRect]) -> Option<ScissorRect> {
    let first = *scissors.first()?;
    let (mut x0, mut y0, mut x1, mut y1) = (first.x, first.y, first.x + first.w, first.y + first.h);
    for scissor in &scissors[1..] {
        x0 = x0.min(scissor.x);
        y0 = y0.min(scissor.y);
        x1 = x1.max(scissor.x + scissor.w);
        y1 = y1.max(scissor.y + scissor.h);
    }
    Some(ScissorRect { x: x0, y: y0, w: x1 - x0, h: y1 - y0 })
}

/// ⋃️ Unions two optional bounds, passing either through unchanged if the other is absent.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn merge_scissor_bounds(a: Option<ScissorRect>, b: Option<ScissorRect>) -> Option<ScissorRect> {
    match (a, b) {
        (Some(a), Some(b)) => union_scissors(&[a, b]),
        (Some(value), None) | (None, Some(value)) => Some(value),
        (None, None) => None,
    }
}

/// 🪟️ The concrete pieces a batch must scissor/stencil-mask against: a silhouette clip's rectangles
/// intersected with the ambient scissor and the viewport, or the scissor alone, or the whole viewport.
/// Ported verbatim from the wgpu target's `layer_scissors`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn layer_scissors(scissor: Option<ScissorRect>, clip: Option<&ClipRegion>, width: f32, height: f32) -> Vec<Option<ScissorRect>> {
    if let Some(clip) = clip {
        return clip.effective_scissors(scissor, width, height).into_iter().map(Some).collect();
    }
    let viewport = ScissorRect { x: 0, y: 0, w: width.max(0.0) as u32, h: height.max(0.0) as u32 };
    match scissor.map(|value| value.intersect(&viewport)) {
        Some(value) if value.w > 0 && value.h > 0 => vec![Some(value)],
        Some(_) => Vec::new(),
        None => vec![None],
    }
}

/// 🎭️ The stencil-mask quads one batch must paint before its content: a reset quad covering the union
/// of the previous batch's bounds and this batch's bounds (so switching *out* of a smaller mask
/// clears the stencil bits the previous mask set), followed by one quad per silhouette piece. Ported
/// verbatim from the wgpu target's `mask_instances`; `UiInstance::solid` becomes `QuadInstance::solid`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn mask_instances(scissor: Option<ScissorRect>, clip: Option<&ClipRegion>, previous_bounds: Option<ScissorRect>, width: f32, height: f32) -> (Vec<QuadInstance>, Option<ScissorRect>) {
    let white = [1.0, 1.0, 1.0, 1.0];
    let viewport = ScissorRect { x: 0, y: 0, w: width.max(0.0) as u32, h: height.max(0.0) as u32 };
    let pieces: Vec<ScissorRect> = layer_scissors(scissor, clip, width, height).into_iter().map(|piece| piece.unwrap_or(viewport)).collect();
    let current_bounds = union_scissors(&pieces);
    let Some(reset_bounds) = merge_scissor_bounds(previous_bounds, current_bounds) else {
        return (Vec::new(), None);
    };
    let mut instances = vec![QuadInstance::solid([reset_bounds.x as f32, reset_bounds.y as f32, reset_bounds.w as f32, reset_bounds.h as f32], white)];
    instances.extend(pieces.into_iter().map(|piece| QuadInstance::solid([piece.x as f32, piece.y as f32, piece.w as f32, piece.h as f32], white)));
    (instances, current_bounds)
}

//#endregion SilhouetteMask

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::LayoutRect;

    #[test]
    fn dashed_line_segments_emit_dashes_along_segment() {
        let segments = dashed_line_segments(0.0, 0.0, 20.0, 0.0, 5.0, 4.0);
        assert!(!segments.is_empty());
        let span: f32 = segments.iter().map(|(x0, _, x1, _)| x1 - x0).sum();
        assert!(span > 0.0 && span <= 20.0);
    }

    #[test]
    fn thick_line_positions_are_symmetric_about_the_segment() {
        let positions = thick_line_positions(0.0, 0.0, 10.0, 0.0, 2.0);
        assert_eq!(positions[0], [0.0, 1.0]);
        assert_eq!(positions[2], [0.0, -1.0]);
    }

    #[test]
    fn ear_clip_produces_triangles() {
        let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
        let tris = ear_clip_polygon(&square);
        assert!(tris.len() >= 3);
        assert_eq!(tris.len() % 3, 0);
    }

    #[test]
    fn ear_clip_below_three_points_is_empty() {
        assert!(ear_clip_polygon(&[[0.0, 0.0], [1.0, 1.0]]).is_empty());
    }

    #[test]
    fn triangle_fan_covers_every_interior_triangle() {
        let points = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        assert_eq!(triangle_fan_positions(&points).len(), 2);
    }

    #[test]
    fn snap_rect_snaps_both_edges_independently() {
        let rect = [10.4, 20.6, 30.5, 40.5];
        assert_eq!(snap_rect(rect, 1.0), [10.0, 21.0, 31.0, 40.0]);
    }

    #[test]
    fn snap_to_device_pixels_is_deterministic_across_common_dprs() {
        for dpr in [1.0, 1.5, 2.0] {
            let once = snap_to_device_pixels(12.34, dpr);
            let twice = snap_to_device_pixels(12.34, dpr);
            assert_eq!(once, twice);
        }
    }

    #[test]
    fn silhouette_mask_reset_is_bounded_to_previous_and_current_unions() {
        let previous = Some(ScissorRect { x: 10, y: 10, w: 30, h: 20 });
        let clip = ClipRegion::from_rects(&[LayoutRect::new(80.0, 15.0, 20.0, 25.0)]);
        let (instances, current) = mask_instances(None, Some(&clip), previous, 500.0, 400.0);
        assert_eq!(instances[0].rect, [10.0, 10.0, 90.0, 30.0]);
        assert_eq!(instances[1].rect, [80.0, 15.0, 20.0, 25.0]);
        assert_eq!(current, Some(ScissorRect { x: 80, y: 15, w: 20, h: 25 }));
    }

    #[test]
    fn empty_silhouette_clip_writes_no_visible_stencil_region() {
        let empty = ClipRegion { scissors: Vec::new() };
        let (instances, current) = mask_instances(None, Some(&empty), None, 500.0, 400.0);
        assert!(instances.is_empty(), "a cleared pass needs neither a reset nor a reference-one mask draw");
        assert_eq!(current, None);
    }
}

//#endregion Tests

//#endregion 🔖️Tessellate
