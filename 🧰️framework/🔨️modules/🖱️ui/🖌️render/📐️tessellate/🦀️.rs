//! 📐️ CPU tessellation: ear clipping, dashed-line/thick-line expansion, triangle fans, pixel
//! and pixel snapping. Every function here is pure — same inputs, same outputs,
//! no GPU type in sight — which is what makes [`crate::scene::Scene::finish`]'s `snap`/`batch` steps
//! deterministic and unit-testable without a device.

use crate::scene::ScissorRect;

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

/// 🧲️ Snaps a `[x, y, w, h]` rect's edges (not its width/height) to the physical pixel grid at `dpr`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn snap_rect(rect: [f32; 4], dpr: f32) -> [f32; 4] {
    let x0 = snap_to_device_pixels(rect[0], dpr);
    let y0 = snap_to_device_pixels(rect[1], dpr);
    let x1 = snap_to_device_pixels(rect[0] + rect[2], dpr);
    let y1 = snap_to_device_pixels(rect[1] + rect[3], dpr);
    [x0, y0, (x1 - x0).max(0.0), (y1 - y0).max(0.0)]
}

/// 📍️ Snaps a `[x, y]` point to the physical pixel grid at `dpr` — used for vector vertices, which
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

//#region Scissor

/// ✂️ The rectangular scissor a batch renders under: `scissor` intersected with the viewport, the whole viewport when the batch has none, `None` when nothing of it is visible. A frame clips its content with exactly this rectangle.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn viewport_scissor(scissor: Option<ScissorRect>, width: f32, height: f32) -> Option<ScissorRect> {
    let viewport = ScissorRect { x: 0, y: 0, w: width.max(0.0) as u32, h: height.max(0.0) as u32 };
    let resolved = scissor.map_or(viewport, |value| value.intersect(&viewport));
    (resolved.w > 0 && resolved.h > 0).then_some(resolved)
}

//#endregion Scissor

//#region Tests

#[cfg(test)]
#[path = "../🧪️tests/🔬️tessellate-unit/🦀️.rs"]
mod tests;

//#endregion Tests

//#endregion 🔖️Tessellate
