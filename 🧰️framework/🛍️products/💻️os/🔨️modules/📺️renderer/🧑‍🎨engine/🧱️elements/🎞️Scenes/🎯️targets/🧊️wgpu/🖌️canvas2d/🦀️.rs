//! 🖌️ Canvas2d paint, fill blend and shapes.

use super::*;

//#region Canvas2d
/** 🪣️ A `CanvasLayerRecord["fill"]` mirror — solid color or linear/radial gradient stops, matches
 * `fillStyleToPaint` in `canvas-2d-host.tsx`. Coordinates (`x1/y1/x2/y2`, `cx/cy/r`) are in the same
 * local space as the owning layer's `x`/`y` (this renderer has no per-layer transform matrix yet, so
 * they are treated as offsets from the layer's own `x`/`y` origin). */
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CanvasFillJson {
    #[serde(default)]
    pub(crate) kind: Option<String>,
    #[serde(default)]
    pub(crate) color: Option<Vec<f64>>,
    #[serde(default)]
    pub(crate) x1: f64,
    #[serde(default)]
    pub(crate) y1: f64,
    #[serde(default)]
    pub(crate) x2: f64,
    #[serde(default)]
    pub(crate) y2: f64,
    #[serde(default)]
    pub(crate) cx: f64,
    #[serde(default)]
    pub(crate) cy: f64,
    #[serde(default)]
    pub(crate) r: f64,
    #[serde(default)]
    pub(crate) stops: Vec<CanvasGradientStopJson>,
}

/** 🎨️ One `CanvasGradientStop` — `offset` in `[0,1]`, `color` an `[r,g,b,a?]` channel array in `0..1`. */
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CanvasGradientStopJson {
    #[serde(default)]
    pub(crate) offset: f64,
    #[serde(default)]
    pub(crate) color: Option<Vec<f64>>,
}

/** 🖊️ A `CanvasLayerRecord["stroke"]` mirror. */
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct CanvasStrokeJson {
    #[serde(default)]
    pub(crate) color: Option<Vec<f64>>,
    #[serde(default)]
    pub(crate) width: Option<f64>,
    #[serde(default)]
    dash: Option<Vec<f64>>,
    #[serde(default)]
    cap: Option<String>,
    #[serde(default)]
    pub(crate) join: Option<String>,
}

/** 🖼️ A `CanvasLayerRecord["image"]` mirror — the nested per-node image field (as opposed to the
 * legacy top-level `dataUrl` used by `kind === "image"` records). */
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct CanvasImageFieldJson {
    #[serde(default)]
    src: Option<String>,
    #[serde(default)]
    pub(crate) width: Option<f64>,
    #[serde(default)]
    pub(crate) height: Option<f64>,
}

/** 📝️ A `CanvasLayerRecord["text"]` mirror. */
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct CanvasTextFieldJson {
    #[serde(default)]
    pub(crate) content: Option<String>,
    #[serde(default)]
    pub(crate) size: Option<f64>,
    font_family: TextFace,
}

#[derive(Deserialize)]
pub(crate) struct CanvasLayer {
    #[serde(default)]
    pub(crate) kind: String,
    #[serde(default)]
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) x: f64,
    #[serde(default)]
    pub(crate) y: f64,
    #[serde(default)]
    pub(crate) width: f64,
    #[serde(default)]
    pub(crate) height: f64,
    #[serde(default)]
    pub(crate) x0: Option<f64>,
    #[serde(default)]
    pub(crate) y0: Option<f64>,
    #[serde(default)]
    pub(crate) x1: Option<f64>,
    #[serde(default)]
    pub(crate) y1: Option<f64>,
    #[serde(default, rename = "dataUrl")]
    data_url: Option<String>,
    #[serde(default)]
    pub(crate) points: Option<Vec<[f64; 2]>>,
    #[serde(default)]
    seams: Option<Vec<u8>>,
    /** 🗒️ `role === "meta"` marks a non-visual bookkeeping record (e.g. carries the host's active
     * pointer-tool id) — filtered from rendering, matches `JsonLayersCanvasSession.renderFrame`'s
     * `records.find(role === "meta")` / `layers.filter(role !== "meta")` split. */
    #[serde(default)]
    pub(crate) role: Option<String>,
    #[serde(default)]
    pub(crate) utility: Option<String>,
    #[serde(default)]
    pub(crate) visible: Option<bool>,
    #[serde(default)]
    pub(crate) opacity: Option<f32>,
    #[serde(default, rename = "blendMode")]
    blend_mode: Option<String>,
    #[serde(default)]
    pub(crate) selected: Option<bool>,
    #[serde(default)]
    pub(crate) fill: Option<CanvasFillJson>,
    #[serde(default)]
    pub(crate) stroke: Option<CanvasStrokeJson>,
    #[serde(default)]
    image: Option<CanvasImageFieldJson>,
    #[serde(default)]
    pub(crate) text: Option<CanvasTextFieldJson>,
    #[serde(default)]
    segments: Option<Vec<crate::canvas2d_paint::ScenePathSegment>>,
    #[serde(default)]
    transform: Option<Vec<f64>>,
    #[serde(default, rename = "fillRule")]
    fill_rule: Option<String>,
}

#[derive(Deserialize)]
struct Canvas2dPacketText<'a> {
    #[serde(borrow)]
    pub(crate) content: &'a str,
    #[serde(default = "canvas2d_packet_text_size")]
    pub(crate) size: f64,
}

#[derive(Deserialize)]
struct Canvas2dPacketItem<'a> {
    #[serde(borrow)]
    pub(crate) kind: &'a str,
    #[serde(borrow)]
    pub(crate) id: &'a str,
    #[serde(default)]
    pub(crate) x: f64,
    #[serde(default)]
    pub(crate) y: f64,
    #[serde(default)]
    pub(crate) width: f64,
    #[serde(default)]
    pub(crate) height: f64,
    #[serde(default)]
    pub(crate) x0: f64,
    #[serde(default)]
    pub(crate) y0: f64,
    #[serde(default)]
    pub(crate) x1: f64,
    #[serde(default)]
    pub(crate) y1: f64,
    #[serde(default, borrow)]
    pub(crate) text: Option<Canvas2dPacketText<'a>>,
}

fn canvas2d_packet_text_size() -> f64 {
    11.0
}

pub(crate) fn canvas_layer_should_render(layer: &CanvasLayer) -> bool {
    layer.role.as_deref() != Some("meta") && layer.visible.unwrap_or(true)
}

fn decode_canvas_image_source(data_url: &str) -> Option<Vec<u8>> {
    let payload = data_url.strip_prefix("data:image/png;base64,").or_else(|| data_url.strip_prefix("data:image/jpeg;base64,")).unwrap_or(data_url);
    base64::engine::general_purpose::STANDARD.decode(payload).ok()
}

fn decode_canvas_image_bytes(bytes: &[u8]) -> Option<(Vec<u8>, u32, u32)> {
    let image = image::load_from_memory(&bytes).ok()?;
    let rgba = image.to_rgba8();
    let (width, height) = rgba.dimensions();
    Some((rgba.into_raw(), width, height))
}

pub(crate) fn queue_canvas_image_upload_sized(surface_id: &str, layer_id: &str, data_url: &str) -> (Result<String, RasterUploadRefusal>, Option<(u32, u32)>) {
    let dimensions = std::cell::Cell::new(None);
    let key = queue_canvas_image_upload_with(
        surface_id,
        layer_id,
        data_url.as_bytes(),
        || {
            let bytes = decode_canvas_image_source(data_url).ok_or_else(Vec::new)?;
            let measured = match image::ImageReader::new(std::io::Cursor::new(bytes.as_slice())).with_guessed_format().ok().and_then(|reader| reader.into_dimensions().ok()) {
                Some(measured) => measured,
                None => return Err(bytes),
            };
            dimensions.set(Some(measured));
            Ok((measured.0, measured.1, bytes))
        },
        |bytes| decode_canvas_image_bytes(bytes).map(|(pixels, _, _)| pixels),
    );
    (key, dimensions.get())
}

pub(crate) fn queue_canvas_image_upload(surface_id: &str, layer_id: &str, data_url: &str) -> Option<String> {
    queue_canvas_image_upload_sized(surface_id, layer_id, data_url).0.ok()
}

pub(crate) fn draw_checkerboard(draw: &mut ui_wgpu::wgpu::DrawList, viewport: &Viewport, inner: Rect, theme: &Theme, extent: f32) {
    let cell = 16.0;
    let half = extent * 0.5;
    let light = theme.checker_light;
    let dark = theme.checker_dark;
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (-half, half, -half, half);
    if viewport.zoom > 0.0 {
        let (wx0, wy0) = viewport.screen_to_world(inner.x, inner.y, inner);
        let (wx1, wy1) = viewport.screen_to_world(inner.x + inner.w, inner.y + inner.h, inner);
        min_x = min_x.max(wx0.min(wx1) - cell);
        max_x = max_x.min(wx0.max(wx1) + cell);
        min_y = min_y.max(wy0.min(wy1) - cell);
        max_y = max_y.min(wy0.max(wy1) + cell);
    }
    let start_row = ((min_y - (-half)) / cell).floor().max(0.0) as i64;
    let start_col = ((min_x - (-half)) / cell).floor().max(0.0) as i64;
    let mut row = start_row;
    let mut wy = -half + start_row as f32 * cell;
    while wy < max_y {
        let mut col = start_col;
        let mut wx = -half + start_col as f32 * cell;
        while wx < max_x {
            let color = if (row + col) % 2 == 0 { light } else { dark };
            let (sx, sy) = viewport.world_to_screen(wx, wy, inner);
            let (sx1, sy1) = viewport.world_to_screen(wx + cell, wy + cell, inner);
            let w = (sx1 - sx).abs().max(1.0);
            let h = (sy1 - sy).abs().max(1.0);
            draw.push_solid([sx.min(sx1), sy.min(sy1), w, h], color);
            wx += cell;
            col += 1;
        }
        wy += cell;
        row += 1;
    }
}

fn draw_canvas_infinite_grid(draw: &mut ui_wgpu::wgpu::DrawList, viewport: &Viewport, inner: Rect, theme: &Theme) {
    if viewport.zoom <= 0.0 {
        return;
    }
    let (wx0, wy0) = viewport.screen_to_world(inner.x, inner.y, inner);
    let (wx1, wy1) = viewport.screen_to_world(inner.x + inner.w, inner.y + inner.h, inner);
    let min_x = wx0.min(wx1);
    let max_x = wx0.max(wx1);
    let min_y = wy0.min(wy1);
    let max_y = wy0.max(wy1);
    let color = theme.separator.with_alpha((theme.separator.a * 0.35).max(0.08));
    let steps: [(f32, f32, f32); 4] = [(10.0, 1.0, 0.0), (2.5, 0.72, 8.0), (0.5, 0.48, 10.0), (0.1, 0.32, 12.0)];
    for (world_step, stroke_px, min_screen) in steps {
        let screen = world_step * viewport.zoom;
        if screen < min_screen {
            continue;
        }
        let width = (stroke_px).max(0.5);
        let start_x = (min_x / world_step).floor() * world_step;
        let start_y = (min_y / world_step).floor() * world_step;
        let mut x = start_x;
        while x <= max_x {
            let (sx0, sy0) = viewport.world_to_screen(x, min_y, inner);
            let (sx1, sy1) = viewport.world_to_screen(x, max_y, inner);
            draw.push_line(sx0, sy0, sx1, sy1, color, width);
            x += world_step;
        }
        let mut y = start_y;
        while y <= max_y {
            let (sx0, sy0) = viewport.world_to_screen(min_x, y, inner);
            let (sx1, sy1) = viewport.world_to_screen(max_x, y, inner);
            draw.push_line(sx0, sy0, sx1, sy1, color, width);
            y += world_step;
        }
    }
}

fn draw_dashed_line(draw: &mut ui_wgpu::wgpu::DrawList, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgba, width: f32) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let len = (dx * dx + dy * dy).sqrt().max(0.001);
    let ux = dx / len;
    let uy = dy / len;
    let dash = 4.0f32;
    let gap = 4.0f32;
    let mut traveled = 0.0f32;
    let mut drawing = true;
    while traveled < len {
        let segment = if drawing { dash } else { gap };
        let next = (traveled + segment).min(len);
        if drawing {
            let sx0 = x0 + ux * traveled;
            let sy0 = y0 + uy * traveled;
            let sx1 = x0 + ux * next;
            let sy1 = y0 + uy * next;
            draw.push_line(sx0, sy0, sx1, sy1, color, width);
        }
        traveled = next;
        drawing = !drawing;
    }
}

fn canvas_color_channels(v: &[f64], opacity: f32) -> Rgba {
    let r = v.first().copied().unwrap_or(0.58) as f32;
    let g = v.get(1).copied().unwrap_or(0.64) as f32;
    let b = v.get(2).copied().unwrap_or(0.72) as f32;
    let a = v.get(3).copied().unwrap_or(1.0) as f32;
    Rgba::new(r, g, b, (a * opacity).clamp(0.0, 1.0))
}

fn canvas_mix_rgba(a: Rgba, b: Rgba, t: f32) -> Rgba {
    let t = t.clamp(0.0, 1.0);
    Rgba::new(a.r + (b.r - a.r) * t, a.g + (b.g - a.g) * t, a.b + (b.b - a.b) * t, a.a + (b.a - a.a) * t)
}

pub(crate) fn canvas_gradient_color_at(stops: &[CanvasGradientStopJson], t: f32, opacity: f32) -> Rgba {
    if stops.is_empty() {
        return Rgba::new(0.58, 0.64, 0.72, 0.95 * opacity);
    }
    let t = t.clamp(0.0, 1.0);
    if stops.len() == 1 {
        return canvas_color_channels(stops[0].color.as_deref().unwrap_or(&[]), opacity);
    }
    let last = stops.len() - 1;
    for i in 0..last {
        let a_off = stops[i].offset.clamp(0.0, 1.0) as f32;
        let b_off = stops[i + 1].offset.clamp(0.0, 1.0) as f32;
        if t <= b_off || i == last - 1 {
            let span = (b_off - a_off).max(0.0001);
            let local = ((t - a_off) / span).clamp(0.0, 1.0);
            let ca = canvas_color_channels(stops[i].color.as_deref().unwrap_or(&[]), opacity);
            let cb = canvas_color_channels(stops[i + 1].color.as_deref().unwrap_or(&[]), opacity);
            return canvas_mix_rgba(ca, cb, local);
        }
    }
    canvas_color_channels(stops[last].color.as_deref().unwrap_or(&[]), opacity)
}

fn canvas_blend_channel(mode: &str, cb: f32, cs: f32) -> f32 {
    match mode {
        "multiply" => cb * cs,
        "screen" => cb + cs - cb * cs,
        "overlay" => canvas_blend_channel("hardLight", cs, cb),
        "darken" => cb.min(cs),
        "lighten" => cb.max(cs),
        "colorDodge" => {
            if cb <= 0.0 {
                0.0
            } else if cs >= 1.0 {
                1.0
            } else {
                (cb / (1.0 - cs)).min(1.0)
            }
        }
        "colorBurn" => {
            if cb >= 1.0 {
                1.0
            } else if cs <= 0.0 {
                0.0
            } else {
                1.0 - ((1.0 - cb) / cs).min(1.0)
            }
        }
        "hardLight" => {
            if cs <= 0.5 {
                2.0 * cb * cs
            } else {
                1.0 - 2.0 * (1.0 - cb) * (1.0 - cs)
            }
        }
        "softLight" => {
            let d = if cb <= 0.25 { ((16.0 * cb - 12.0) * cb + 4.0) * cb } else { cb.sqrt() };
            if cs <= 0.5 {
                cb - (1.0 - 2.0 * cs) * cb * (1.0 - cb)
            } else {
                cb + (2.0 * cs - 1.0) * (d - cb)
            }
        }
        "difference" => (cb - cs).abs(),
        "exclusion" => cb + cs - 2.0 * cb * cs,
        _ => cs,
    }
}

pub(crate) fn canvas_apply_blend_mode(mode: Option<&str>, backdrop: Rgba, source: Rgba) -> Rgba {
    let mode = match mode {
        None | Some("") | Some("normal") => return source,
        Some(m) => m,
    };
    let cb = [backdrop.r, backdrop.g, backdrop.b];
    let cs = [source.r, source.g, source.b];
    let blended = match mode {
        "hue" => canvas_set_lum(canvas_set_sat(cs, canvas_sat(cb)), canvas_lum(cb)),
        "saturation" => canvas_set_lum(canvas_set_sat(cb, canvas_sat(cs)), canvas_lum(cb)),
        "color" => canvas_set_lum(cs, canvas_lum(cb)),
        "luminosity" => canvas_set_lum(cb, canvas_lum(cs)),
        _ => [canvas_blend_channel(mode, cb[0], cs[0]), canvas_blend_channel(mode, cb[1], cs[1]), canvas_blend_channel(mode, cb[2], cs[2])],
    };
    Rgba::new(blended[0].clamp(0.0, 1.0), blended[1].clamp(0.0, 1.0), blended[2].clamp(0.0, 1.0), source.a)
}

fn push_shape_fill(draw: &mut ui_wgpu::wgpu::DrawList, rect: Rect, color: Rgba, is_circle: bool) {
    if is_circle {
        let cx = rect.x + rect.w * 0.5;
        let cy = rect.y + rect.h * 0.5;
        let radius = rect.w.min(rect.h) * 0.5;
        draw.push_triangle_fan(&canvas_circle_points(cx, cy, radius.max(0.5), CANVAS_CIRCLE_SEGMENTS), color);
    } else {
        draw.push_rounded([rect.x, rect.y, rect.w, rect.h], color, 4.0);
    }
}

fn push_circle_outline(draw: &mut ui_wgpu::wgpu::DrawList, cx: f32, cy: f32, radius: f32, color: Rgba, width: f32) {
    let points = canvas_circle_points(cx, cy, radius.max(0.5), CANVAS_CIRCLE_SEGMENTS);
    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i + 1) % points.len()];
        draw.push_line(a[0], a[1], b[0], b[1], color, width);
    }
}

fn push_shape_outline(draw: &mut ui_wgpu::wgpu::DrawList, rect: Rect, color: Rgba, width: f32, is_circle: bool, dash: Option<&[f64]>) {
    if is_circle {
        let cx = rect.x + rect.w * 0.5;
        let cy = rect.y + rect.h * 0.5;
        let radius = rect.w.min(rect.h) * 0.5;
        push_circle_outline(draw, cx, cy, radius, color, width);
        return;
    }
    if dash.is_some_and(|d| !d.is_empty()) {
        draw_dashed_line(draw, rect.x, rect.y, rect.x + rect.w, rect.y, color, width);
        draw_dashed_line(draw, rect.x + rect.w, rect.y, rect.x + rect.w, rect.y + rect.h, color, width);
        draw_dashed_line(draw, rect.x + rect.w, rect.y + rect.h, rect.x, rect.y + rect.h, color, width);
        draw_dashed_line(draw, rect.x, rect.y + rect.h, rect.x, rect.y, color, width);
    } else {
        draw_ink_rect_outline(draw, rect.x, rect.y, rect.w, rect.h, color, width);
    }
}

pub(crate) fn push_linear_gradient_fill(draw: &mut ui_wgpu::wgpu::DrawList, viewport: &Viewport, inner: Rect, clip: Rect, origin_x: f64, origin_y: f64, fill: &CanvasFillJson, opacity: f32, blend: Option<&str>, backdrop: Rgba) {
    let (sx1, sy1) = viewport.world_to_screen((origin_x + fill.x1) as f32, (origin_y + fill.y1) as f32, inner);
    let (sx2, sy2) = viewport.world_to_screen((origin_x + fill.x2) as f32, (origin_y + fill.y2) as f32, inner);
    let dx = sx2 - sx1;
    let dy = sy2 - sy1;
    let len = (dx * dx + dy * dy).sqrt();
    draw.push_scissor(clip);
    if len < 0.5 {
        let color = canvas_apply_blend_mode(blend, backdrop, canvas_gradient_color_at(&fill.stops, 1.0, opacity));
        draw.push_rounded([clip.x, clip.y, clip.w, clip.h], color, 0.0);
        draw.pop_scissor();
        return;
    }
    let ux = dx / len;
    let uy = dy / len;
    let px = -uy;
    let py = ux;
    let overhang = (clip.w + clip.h).max(32.0);
    for i in 0..CANVAS_GRADIENT_BANDS {
        let t0 = i as f32 / CANVAS_GRADIENT_BANDS as f32;
        let t1 = (i + 1) as f32 / CANVAS_GRADIENT_BANDS as f32;
        let color = canvas_apply_blend_mode(blend, backdrop, canvas_gradient_color_at(&fill.stops, (t0 + t1) * 0.5, opacity));
        let a0 = if i == 0 { -overhang } else { t0 * len };
        let a1 = if i == CANVAS_GRADIENT_BANDS - 1 { len + overhang } else { t1 * len };
        let bx = sx1 + ux * a0;
        let by = sy1 + uy * a0;
        let ex = sx1 + ux * a1;
        let ey = sy1 + uy * a1;
        let points = [[bx + px * overhang, by + py * overhang], [ex + px * overhang, ey + py * overhang], [ex - px * overhang, ey - py * overhang], [bx - px * overhang, by - py * overhang]];
        draw.push_triangle_fan(&points, color);
    }
    draw.pop_scissor();
}

pub(crate) fn push_radial_gradient_fill(draw: &mut ui_wgpu::wgpu::DrawList, viewport: &Viewport, inner: Rect, clip: Rect, origin_x: f64, origin_y: f64, fill: &CanvasFillJson, opacity: f32, blend: Option<&str>, backdrop: Rgba) {
    let (scx, scy) = viewport.world_to_screen((origin_x + fill.cx) as f32, (origin_y + fill.cy) as f32, inner);
    let sr = (fill.r as f32 * viewport.zoom).max(0.5);
    draw.push_scissor(clip);
    let outer_radius = sr.max(clip.w.max(clip.h));
    for i in (0..CANVAS_GRADIENT_BANDS).rev() {
        let t = (i + 1) as f32 / CANVAS_GRADIENT_BANDS as f32;
        let radius = if i == CANVAS_GRADIENT_BANDS - 1 { outer_radius } else { sr * t };
        let color = canvas_apply_blend_mode(blend, backdrop, canvas_gradient_color_at(&fill.stops, t, opacity));
        draw.push_triangle_fan(&canvas_circle_points(scx, scy, radius.max(0.5), CANVAS_CIRCLE_SEGMENTS), color);
    }
    draw.pop_scissor();
}

#[allow(clippy::too_many_arguments, reason = "one arg per resolved paint/geometry input; grouping into a struct is a T2 restructure, out of scope")]
pub(crate) fn render_canvas_shape_fill(draw: &mut ui_wgpu::wgpu::DrawList, viewport: &Viewport, inner: Rect, shape_rect: Rect, layer: &CanvasLayer, opacity: f32, fallback_fill: Rgba, fallback_stroke: Rgba, backdrop: Rgba, is_circle: bool) {
    let blend = layer.blend_mode.as_deref();
    match &layer.fill {
        Some(fill) if fill.kind.as_deref() == Some("linearGradient") && !fill.stops.is_empty() => {
            push_linear_gradient_fill(draw, viewport, inner, shape_rect, layer.x, layer.y, fill, opacity, blend, backdrop);
        }
        Some(fill) if fill.kind.as_deref() == Some("radialGradient") && !fill.stops.is_empty() => {
            push_radial_gradient_fill(draw, viewport, inner, shape_rect, layer.x, layer.y, fill, opacity, blend, backdrop);
        }
        Some(fill) if fill.color.is_some() => {
            let solid = canvas_apply_blend_mode(blend, backdrop, canvas_color_channels(fill.color.as_deref().unwrap_or(&[]), opacity));
            push_shape_fill(draw, shape_rect, solid, is_circle);
        }
        _ => {
            let solid = canvas_apply_blend_mode(blend, backdrop, fallback_fill);
            push_shape_fill(draw, shape_rect, solid, is_circle);
        }
    }
    if let Some(stroke) = &layer.stroke {
        let color = stroke.color.as_deref().map(|c| canvas_apply_blend_mode(blend, backdrop, canvas_color_channels(c, opacity))).unwrap_or_else(|| fallback_stroke.with_alpha(fallback_stroke.a * opacity));
        let width = (stroke.width.unwrap_or(1.0) as f32).max(1.0);
        push_shape_outline(draw, shape_rect, color, width, is_circle, stroke.dash.as_deref());
    }
}

fn render_canvas2d_packet_item(item: &Canvas2dPacketItem<'_>, viewport: &Viewport, inner: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let color = if item.id.starts_with("residual-field-") {
        theme.diagram_field_residual
    } else if item.id.starts_with("reaction-field-") || item.id.starts_with("load-") {
        theme.diagram_field_reaction
    } else if item.id.starts_with("displacement-field-") || item.id.starts_with("mode-field-") {
        theme.diagram_field_displacement
    } else {
        theme.diagram_accent
    };
    if item.kind == "line" {
        let (x0, y0) = viewport.world_to_screen(item.x0 as f32, item.y0 as f32, inner);
        let (x1, y1) = viewport.world_to_screen(item.x1 as f32, item.y1 as f32, inner);
        ctx.draw.push_line(x0, y0, x1, y1, color, (2.0 * viewport.zoom).max(1.0));
    } else if item.kind == "circle" {
        let (x, y) = viewport.world_to_screen(item.x as f32, item.y as f32, inner);
        let width = (item.width as f32 * viewport.zoom).max(4.0);
        let height = (item.height as f32 * viewport.zoom).max(4.0);
        ctx.draw.push_solid([x, y, width, height], color);
    } else if item.kind == "text" {
        let Some(text) = item.text.as_ref() else { return };
        let (x, y) = viewport.world_to_screen(item.x as f32, item.y as f32, inner);
        draw_text(ctx, text.content, x, y + text.size as f32, (text.size as f32).max(8.0), theme.text);
    }
}

/// 📷️ Resolves one framing revision after layout and publishes its resulting local camera.
fn apply_canvas_framing(scene: &UiComponentSceneNode, bounds: Rect) -> bool {
    let Some(request) = scene.canvas_2d.as_ref().and_then(|canvas| canvas.framing.as_ref()) else { return false };
    if scene_state(&scene.host_id).canvas_framing_revision == Some(request.revision) {
        return false;
    }
    let Some(camera) = request.fit(bounds.w as f64, bounds.h as f64) else { return false };
    let viewport = Viewport { x: camera.x as f32, y: camera.y as f32, zoom: camera.zoom as f32 };
    if ![viewport.x, viewport.y, viewport.zoom].iter().all(|value| value.is_finite()) {
        return false;
    }
    mutate_scene_state(&scene.host_id, |state| {
        state.viewport = viewport;
        state.canvas_framing_revision = Some(request.revision);
        state.camera_dispatch_controller_id = Some(scene.controller_id.clone());
    });
    schedule_scene_camera_dispatch(&scene.host_id, &scene.surface_id);
    true
}

/// 🗒️ `role === "meta"` (activeUtility bookkeeping) and `visible === false` records are
/// non-visual — skip rendering entirely, matches `layers.filter(role !== "meta")` in
/// `canvas-2d-host.tsx`'s `JsonLayersCanvasSession.renderFrame`.
/// 🖼️ Generic bounds-rect (or `kind === "circle"`) draw record — resolves solid/gradient
/// fill, blend-mode approximation, and stroke, matching `drawSceneNode`'s bounds-layer path.
/// 🖊️ Overlay annotation: a two-pass selection highlight (soft outer glow + crisp amber ring)
/// drawn on top of the shape, matches `drawBoundsLayer`'s `isSelected` glow+ring pair in
/// `canvas-2d-host.tsx` (glow at +4px/width 5, ring at +0px/width 2.5, both amber).
/// 🫙️ `layers.length === 0 → ctx.fillText("Empty canvas", -36, 0)` in
/// `🧱️elements/📐️Canvas2dHost/🟦️.tsx`'s `renderFrame` — React counts the same `role !== "meta"`
/// records this loop iterates, so the emptiness verdict matches without a second parse.
pub(crate) fn render_canvas_2d(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(canvas) = &scene.canvas_2d else {
        return render_placeholder("canvas-2d", bounds, ctx);
    };
    let inner = bounds;
    let mut order = CanvasPaintOrder::open(ctx.draw, inner);
    ctx.draw.push_solid([inner.x, inner.y, inner.w, inner.h], theme.canvas_clear);
    let mut viewport = Viewport { x: canvas.camera_x as f32, y: canvas.camera_y as f32, zoom: canvas.zoom as f32 };
    apply_canvas_framing(scene, inner);
    let local = scene_state(&scene.host_id);
    if local.viewport.zoom > 0.0 && scene.component_kind == SurfaceKind::Canvas2d {
        viewport = local.viewport;
    }
    if let Some(snapshot) = canvas.snapshot {
        draw_canvas_infinite_grid(ctx.draw, &viewport, inner, theme);
        order.vector();
        for page_index in 0..snapshot.page_count {
            let _ = ui_wgpu::wgpu::canvas2d_snapshot_with_page(snapshot, page_index, |page| {
                for item in serde_json::Deserializer::from_slice(page.bytes()).into_iter::<Canvas2dPacketItem<'_>>().flatten() {
                    if item.kind == "line" {
                        render_canvas2d_packet_item(&item, &viewport, inner, ctx);
                        order.vector();
                    } else {
                        order.quad(ctx.draw);
                        render_canvas2d_packet_item(&item, &viewport, inner, ctx);
                    }
                }
            });
        }
        crate::canvas2d_gumball::paint(scene, inner, (f64::from(viewport.x), f64::from(viewport.y), f64::from(viewport.zoom)), ctx.theme, ctx.draw);
        order.close(ctx.draw);
        ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(scene.host_id.clone()), kind: HitKind::Generic, drag_axis: Some(DragAxis::Both), drag_data: None });
        return;
    }
    let layers: Vec<CanvasLayer> = serde_json::from_str(&canvas.layers_json).unwrap_or_default();
    if layers.iter().any(|layer| layer.kind == "polyline") {
        draw_checkerboard(ctx.draw, &viewport, inner, ctx.theme, 1024.0);
    }
    draw_canvas_infinite_grid(ctx.draw, &viewport, inner, theme);
    order.vector();
    for (index, layer) in layers.iter().enumerate() {
        if !canvas_layer_should_render(layer) {
            continue;
        }
        if canvas_layer_is_scene_node(layer) {
            render_canvas_scene_node(scene, layer, &viewport, inner, &mut order, ctx);
            continue;
        }
        let opacity = layer.opacity.unwrap_or(1.0).clamp(0.0, 1.0);
        let blend = layer.blend_mode.as_deref();
        if layer.kind == "image" && layer.data_url.is_some() {
            if let Some(data_url) = layer.data_url.as_deref().filter(|src| src.starts_with("data:")) {
                if let Some(key) = queue_canvas_image_upload(&scene.host_id, &layer.id, data_url) {
                    let (sx, sy) = viewport.world_to_screen(layer.x as f32, layer.y as f32, inner);
                    let w = layer.width as f32 * viewport.zoom;
                    let h = layer.height as f32 * viewport.zoom;
                    ctx.draw.push_raster_quad(&key, [sx, sy, w.max(1.0), h.max(1.0)], [0.0, 0.0, 1.0, 1.0], opacity);
                }
            }
            continue;
        }
        if let Some(points) = layer.points.as_ref().filter(|points| layer.kind == "polyline" && !points.is_empty()) {
            let stroke = theme.diagram_stroke.with_alpha(theme.diagram_stroke.a * opacity);
            let seam_stroke = theme.diagram_seam.with_alpha(theme.diagram_seam.a * opacity);
            let width = (1.5 * viewport.zoom).max(1.0);
            for (edge_index, chunk) in points.chunks(2).enumerate() {
                if chunk.len() < 2 {
                    continue;
                }
                let (x0, y0) = viewport.world_to_screen(chunk[0][0] as f32, chunk[0][1] as f32, inner);
                let (x1, y1) = viewport.world_to_screen(chunk[1][0] as f32, chunk[1][1] as f32, inner);
                let is_seam = layer.seams.as_ref().and_then(|seams| seams.get(edge_index)).copied().unwrap_or(0) != 0;
                if is_seam {
                    draw_dashed_line(ctx.draw, x0, y0, x1, y1, seam_stroke, width);
                } else {
                    ctx.draw.push_line(x0, y0, x1, y1, stroke, width);
                }
            }
            order.vector();
            continue;
        }
        let hue = (index * 47 % 360) as f32;
        if layer.kind == "line" || layer.x0.is_some() {
            let x0 = layer.x0.unwrap_or(layer.x) as f32;
            let y0 = layer.y0.unwrap_or(layer.y) as f32;
            let x1 = layer.x1.unwrap_or(layer.x + layer.width) as f32;
            let y1 = layer.y1.unwrap_or(layer.y + layer.height) as f32;
            let (sx0, sy0) = viewport.world_to_screen(x0, y0, inner);
            let (sx1, sy1) = viewport.world_to_screen(x1, y1, inner);
            let base_stroke = layer
                .stroke
                .as_ref()
                .and_then(|stroke| stroke.color.as_deref())
                .map(|c| canvas_color_channels(c, opacity))
                .unwrap_or_else(|| Rgba::new(theme.diagram_accent.r + hue / 720.0, theme.diagram_accent.g, theme.diagram_accent.b, theme.diagram_accent.a * opacity));
            let stroke = canvas_apply_blend_mode(blend, theme.canvas_clear, base_stroke);
            ctx.draw.push_line(sx0, sy0, sx1, sy1, stroke, (2.0 * viewport.zoom).max(1.0));
            order.vector();
            continue;
        }
        let (sx, sy) = viewport.world_to_screen(layer.x as f32, layer.y as f32, inner);
        let w = (layer.width as f32 * viewport.zoom).max(8.0);
        let h = (layer.height as f32 * viewport.zoom).max(8.0);
        let shape_rect = Rect::new(sx, sy, w, h);
        let is_circle = layer.kind == "circle";
        let fallback_fill = Rgba::new(theme.diagram_accent_fill.r + hue / 720.0, theme.diagram_accent_fill.g, theme.diagram_accent_fill.b, theme.diagram_accent_fill.a * opacity);
        order.quad(ctx.draw);
        render_canvas_shape_fill(ctx.draw, &viewport, inner, shape_rect, layer, opacity, fallback_fill, theme.diagram_shape_outline, theme.canvas_clear, is_circle);
        if layer.selected.unwrap_or(false) {
            if is_circle {
                let cx = sx + w * 0.5;
                let cy = sy + h * 0.5;
                let r = w.min(h) * 0.5;
                push_circle_outline(ctx.draw, cx, cy, r + 4.0, CANVAS2D_SELECTION_GLOW, 5.0);
                push_circle_outline(ctx.draw, cx, cy, r, CANVAS2D_SELECTION_RING, 2.5);
            } else {
                draw_ink_rect_outline(ctx.draw, sx - 4.0, sy - 4.0, w + 8.0, h + 8.0, CANVAS2D_SELECTION_GLOW, 5.0);
                draw_ink_rect_outline(ctx.draw, sx, sy, w, h, CANVAS2D_SELECTION_RING, 2.5);
            }
        }
        order.vector();
        let label = if layer.name.is_empty() { layer.id.as_str() } else { layer.name.as_str() };
        if !label.is_empty() {
            order.quad(ctx.draw);
            draw_text(ctx, label, sx + 4.0, sy + 14.0, theme.font_size_small, theme.text);
        }
    }
    if !layers.iter().any(|layer| layer.role.as_deref() != Some("meta")) {
        order.quad(ctx.draw);
        draw_text(ctx, CANVAS_2D_EMPTY_LABEL, inner.x + inner.w * 0.5 - 36.0, inner.y + inner.h * 0.5, theme.font_size_small, theme.text_muted);
    }
    crate::canvas2d_gumball::paint(scene, inner, (f64::from(viewport.x), f64::from(viewport.y), f64::from(viewport.zoom)), ctx.theme, ctx.draw);
    order.close(ctx.draw);
    ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(scene.host_id.clone()), kind: HitKind::Generic, drag_axis: Some(DragAxis::Both), drag_data: None });
}

/// 🧅️ Keeps the Canvas2d paint in record order across the draw list's buckets: one draw layer paints its quads (solid
/// fills, glyphs) before its vector triangles, so a quad that follows vector paint opens a fresh draw layer. Every layer
/// sits under the canvas scissor, which clips the records to the canvas like React's `<canvas>` element.
struct CanvasPaintOrder {
    pub(crate) inner: Rect,
    vectors: bool,
}

impl CanvasPaintOrder {
    fn open(draw: &mut ui_wgpu::wgpu::DrawList, inner: Rect) -> Self {
        draw.push_scissor(inner);
        Self { inner, vectors: false }
    }

    /// 🔺️ Records that vector triangles were pushed into the current draw layer.
    fn vector(&mut self) {
        self.vectors = true;
    }

    /// 🟦️ Readies the draw list for a quad: a fresh draw layer when the current one already holds vector paint.
    fn quad(&mut self, draw: &mut ui_wgpu::wgpu::DrawList) {
        if self.vectors {
            draw.pop_scissor();
            draw.push_scissor(self.inner);
            self.vectors = false;
        }
    }

    fn close(self, draw: &mut ui_wgpu::wgpu::DrawList) {
        draw.pop_scissor();
    }
}

/// 🧩️ Whether React paints `layer` through `drawSceneNode`: it carries `segments`, a `text` or an `image.src`.
fn canvas_layer_is_scene_node(layer: &CanvasLayer) -> bool {
    layer.segments.as_ref().is_some_and(|segments| !segments.is_empty()) || layer.text.is_some() || layer.image.as_ref().and_then(|image| image.src.as_deref()).is_some_and(|src| !src.is_empty())
}

/// 🎨️ `rgbaToCss`: an `[r, g, b, a?]` channel list with its alpha times `opacity`; the slate fallback below three channels.
fn scene_rgba(color: Option<&[f64]>, opacity: f32) -> Rgba {
    canvas_color_channels(color.filter(|channels| channels.len() >= 3).unwrap_or(&[]), opacity)
}

pub(crate) fn screen_points(points: &[[f64; 2]]) -> Vec<[f32; 2]> {
    points.iter().map(|point| [point[0] as f32, point[1] as f32]).collect()
}

/// 🖌️ The wgpu twin of `drawSceneNode` (`📐️Canvas2dHost/🎨️paint`): under the node `transform` and the camera, the path's
/// fill (`evenodd` unless `nonzero`) and stroke (width, caps, joins, dash in layer units) from `canvas2d_paint`, then the
/// text lines (`◻️2d` line law, sized with the map) and the `image.src` raster at the node origin. Approximations of this
/// renderer: group opacity is applied per piece and a stroke is at least one pixel wide.
fn render_canvas_scene_node(scene: &UiComponentSceneNode, layer: &CanvasLayer, viewport: &Viewport, inner: Rect, order: &mut CanvasPaintOrder, ctx: &mut FrameworkWidgetContext<'_>) {
    use crate::canvas2d_paint::{flatten, stroke_polygons, Affine, FillRule, LineCap, LineJoin, StrokeGeometry};
    let backdrop = ctx.theme.canvas_clear;
    let opacity = layer.opacity.unwrap_or(1.0).clamp(0.0, 1.0);
    let blend = layer.blend_mode.as_deref();
    let camera = Affine::camera(f64::from(viewport.x), f64::from(viewport.y), f64::from(viewport.zoom), f64::from(inner.x), f64::from(inner.y), f64::from(inner.w), f64::from(inner.h));
    let map = Affine::node(layer.transform.as_deref()).then(camera);
    let contours = flatten(layer.segments.as_deref().unwrap_or_default(), &map);
    let stroke = layer.stroke.as_ref().filter(|stroke| stroke.width.unwrap_or(1.0).is_finite() && stroke.width.unwrap_or(1.0) > 0.0);
    if let Some(fill) = &layer.fill {
        push_scene_path_fill(ctx.draw, &contours, FillRule::of(layer.fill_rule.as_deref()), (f64::from(inner.y), f64::from(inner.y + inner.h)), fill, &map, opacity, blend, backdrop);
        order.vector();
    }
    if let Some(stroke) = stroke {
        let scale = map.line_scale();
        let geometry = StrokeGeometry { width: (stroke.width.unwrap_or(1.0) * scale).max(1.0), cap: LineCap::of(stroke.cap.as_deref()), join: LineJoin::of(stroke.join.as_deref()), dash: stroke.dash.iter().flatten().map(|value| value * scale).collect() };
        let color = canvas_apply_blend_mode(blend, backdrop, scene_rgba(stroke.color.as_deref(), opacity));
        for polygon in stroke_polygons(&contours, &geometry) {
            ctx.draw.push_triangle_fan(&screen_points(&polygon), color);
        }
        order.vector();
    }
    if let Some(content) = layer.text.as_ref().and_then(|text| text.content.as_deref()).filter(|content| !content.is_empty()) {
        let face=layer.text.as_ref().expect("text content retains its explicit font face").font_family;
        let size = layer.text.as_ref().and_then(|text| text.size).unwrap_or(14.0);
        let text_scale=map.line_scale();
        let pixels = (size * text_scale) as f32;
        let paint = match &layer.fill {
            Some(fill) if matches!(fill.kind.as_deref(), Some("linearGradient" | "radialGradient")) && !fill.stops.is_empty() => Some(canvas_gradient_color_at(&fill.stops, 0.5, opacity)),
            Some(fill) if fill.color.is_some() => Some(scene_rgba(fill.color.as_deref(), opacity)),
            _ => stroke.map(|stroke| scene_rgba(stroke.color.as_deref(), opacity)),
        };
        if let Some(color) = paint.filter(|_| pixels.is_finite() && pixels >= CANVAS_TEXT_MIN_PIXELS && text_scale.is_finite() && text_scale > 0.0) {
            order.quad(ctx.draw);
            let color = canvas_apply_blend_mode(blend, backdrop, color);
            let [a,b,c,d,e,f]=map.0;
            let matrix=[(a/text_scale) as f32,(b/text_scale) as f32,(c/text_scale) as f32,(d/text_scale) as f32,e as f32,f as f32];
            for (index, line) in semio_framework_2d::text::drawing_text_lines(content).enumerate() {
                if line.is_empty() {
                    continue;
                }
                let origin=((layer.x*text_scale) as f32,((layer.y+size+index as f64*size*semio_framework_2d::text::DRAWING_TEXT_LINE_HEIGHT)*text_scale) as f32);
                ui_wgpu::wgpu::widgets::draw_text_face_affine_on(ctx.draw,ctx.atlas,face,line,origin,pixels,color,matrix);
            }
        }
    }
    if let Some(src) = layer.image.as_ref().and_then(|image| image.src.as_deref()).filter(|src| src.starts_with("data:")) {
        if let Some(key) = queue_canvas_image_upload(&scene.host_id, &layer.id, src) {
            let image = layer.image.as_ref();
            let width = image.and_then(|image| image.width).or((layer.width != 0.0).then_some(layer.width)).unwrap_or(CANVAS_SCENE_IMAGE_EXTENT);
            let height = image.and_then(|image| image.height).or((layer.height != 0.0).then_some(layer.height)).unwrap_or(CANVAS_SCENE_IMAGE_EXTENT);
            order.quad(ctx.draw);
            ctx.draw.push_raster_quad_affine(&key,[0.0,0.0,width as f32,height as f32],[0.0,0.0,1.0,1.0],opacity,map.0.map(|value|value as f32));
        }
    }
}

/// 🪣️ Pushes a scene node's path fill (`fillStyleToPaint`): a solid colour as one fan per trapezoid; a linear or radial
/// gradient (layer units) as pieces of at most a tenth of its screen extent, each coloured at its centre. A gradient whose
/// ends coincide paints nothing, as on the platform canvas.
#[allow(clippy::too_many_arguments, reason = "one argument per resolved paint/geometry input of the scene node")]
fn push_scene_path_fill(draw: &mut ui_wgpu::wgpu::DrawList, contours: &[crate::canvas2d_paint::Contour], rule: crate::canvas2d_paint::FillRule, clip: (f64, f64), fill: &CanvasFillJson, map: &crate::canvas2d_paint::Affine, opacity: f32, blend: Option<&str>, backdrop: Rgba) {
    let linear = match fill.kind.as_deref() {
        Some("linearGradient") if !fill.stops.is_empty() => true,
        Some("radialGradient") if !fill.stops.is_empty() => false,
        _ => {
            if fill.color.is_some() {
                let color = canvas_apply_blend_mode(blend, backdrop, scene_rgba(fill.color.as_deref(), opacity));
                for trapezoid in crate::canvas2d_paint::fill_trapezoids(contours, rule, clip) {
                    draw.push_triangle_fan(&screen_points(&trapezoid.corners()), color);
                }
            }
            return;
        }
    };
    let Some(inverse) = map.inverse() else { return };
    let (start, end, center) = ([fill.x1, fill.y1], [fill.x2, fill.y2], [fill.cx, fill.cy]);
    let span = [end[0] - start[0], end[1] - start[1]];
    let span_squared = span[0] * span[0] + span[1] * span[1];
    if (linear && span_squared == 0.0) || (!linear && fill.r <= 0.0) {
        return;
    }
    let extent = if linear { (map.apply(end)[0] - map.apply(start)[0]).hypot(map.apply(end)[1] - map.apply(start)[1]) } else { fill.r * map.line_scale() };
    let trapezoids = crate::canvas2d_paint::fill_trapezoids(contours, rule, clip);
    let area: f64 = trapezoids.iter().map(|t| ((t.top_right - t.top_left) + (t.bottom_right - t.bottom_left)).abs() * 0.5 * (t.bottom - t.top)).sum();
    let step = (extent / CANVAS_GRADIENT_BANDS as f64).max(CANVAS_GRADIENT_MIN_PIECE).max((area / CANVAS_GRADIENT_PIECE_LIMIT as f64).sqrt());
    for piece in trapezoids.iter().flat_map(|trapezoid| trapezoid.pieces(step)) {
        let local = inverse.apply(piece.center());
        let t = if linear { ((local[0] - start[0]) * span[0] + (local[1] - start[1]) * span[1]) / span_squared } else { (local[0] - center[0]).hypot(local[1] - center[1]) / fill.r };
        let color = canvas_apply_blend_mode(blend, backdrop, canvas_gradient_color_at(&fill.stops, t as f32, opacity));
        draw.push_triangle_fan(&screen_points(&piece.corners()), color);
    }
}

/// 🔡️ Scene-node text smaller than this many pixels is not painted (illegible, and it would churn the glyph atlas).
const CANVAS_TEXT_MIN_PIXELS: f32 = 2.0;
/// 🖼️ `drawSceneNode`'s image extent when neither the image nor the node names one.
const CANVAS_SCENE_IMAGE_EXTENT: f64 = 64.0;
/// 🌈️ The smallest gradient piece in pixels and the most pieces one gradient fill may push.
const CANVAS_GRADIENT_MIN_PIECE: f64 = 2.0;
const CANVAS_GRADIENT_PIECE_LIMIT: usize = 4096;

/// 🫙️ The literal `📐️Canvas2dHost` paints in an empty canvas (`🟦️.tsx:634`).
const CANVAS_2D_EMPTY_LABEL: &str = "Empty canvas";

/** 🧾️ Reserves the fixed raster ledger before source preparation or the one-backing decoder. */
/// 🖼️📥️ Queues ONE already-decoded RGBA8 bitmap under its own verbatim `key` — everything
/// [`queue_canvas_image_upload_with`] does for an encoded Canvas layer minus the decode and minus the
/// `canvas-image:<surface>:<layer>` key convention. Its caller is the renderer frame's drain of
/// `ui_wgpu::wgpu::take_ui_image_upload`: `admit_ui_image` already decoded that `data:` PNG in
/// first-party code (`semio-framework-pixels`) and the retained paint keys its `KIND_RASTER` quad by
/// the image `src` itself, so the key must survive unchanged or the quad finds no texture.
pub(crate) fn queue_decoded_raster_upload(surface_id: &str, key: String, width: u32, height: u32, mut pixels: Vec<u8>) -> Option<String> {
    if scene_host_retiring(surface_id) {
        return None;
    }
    let expected = usize::try_from(width).ok()?.checked_mul(usize::try_from(height).ok()?)?.checked_mul(4)?;
    if expected == 0 || expected > RASTER_UPLOAD_BYTE_CAPACITY || pixels.len() != expected || key.len() > RASTER_UPLOAD_KEY_BYTE_CAPACITY {
        return None;
    }
    pixels.shrink_to_fit();
    let ready = PENDING_RASTER_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        surfaces.get_or_insert_with(surface_id.to_string(), PendingRasterSurface::default).is_some_and(|surface| surface.admits_upload())
    });
    if !ready {
        return None;
    }
    let reserved = PENDING_RASTER_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let surface = surfaces.get_mut(surface_id)?;
        match PreparedRasterReservation::try_reserve(key) {
            Ok(reservation) => {
                surface.admission = Some(reservation);
                Some(())
            }
            Err(rejected) => {
                surface.rejected = Some(rejected);
                None
            }
        }
    });
    reserved?;
    let claimed = PENDING_RASTER_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let surface = surfaces.get_mut(surface_id)?;
        let reservation = surface.admission.take()?;
        match reservation.claim(width, height) {
            Ok(reservation) => {
                surface.admission = Some(reservation);
                Some(())
            }
            Err(rejected) => {
                surface.rejected = Some(rejected);
                None
            }
        }
    });
    claimed?;
    let admitted = PENDING_RASTER_STATE.with(|cell| cell.borrow_mut().get_mut(surface_id).and_then(|surface| surface.admission.take()).map(|reservation| reservation.finalize(pixels, Vec::new(), width, height)));
    let (producer, published_key) = match admitted {
        Some(Ok(admitted)) => admitted,
        Some(Err(rejected)) => {
            PENDING_RASTER_STATE.with(|cell| {
                if let Some(surface) = cell.borrow_mut().get_mut(surface_id) {
                    surface.rejected = Some(rejected);
                }
            });
            return None;
        }
        None => return None,
    };
    let accepted = PENDING_RASTER_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let Some(surface) = surfaces.get_mut(surface_id) else { return false };
        match surface.queue.push_back(producer) {
            Ok(()) => true,
            Err(mut producer) => {
                producer.begin_close();
                surface.closing = Some(producer);
                false
            }
        }
    });
    accepted.then_some(published_key)
}

/// 🖼️ Reserves first, then measures, decodes and queues ONE encoded image backing under `canvas-image:<surface>:<layer>`. The
/// answer is typed ([`RasterUploadRefusal`]): `Busy` is back-pressure (no free slot, the process ledger at capacity, an owner still
/// retiring) and the caller offers the source again later; `Invalid` is the SOURCE's own refusal (undecodable, oversized) — a
/// per-image outcome the caller keeps instead of re-offering. Every refused owner is parked in the surface's refusal ring and retired
/// by the upload cursor without faulting the frame, so one bad image never blocks the next image on the same surface.
pub(crate) fn queue_canvas_image_upload_with(
    surface_id: &str,
    layer_id: &str,
    source_identity: &[u8],
    dimensions: impl FnOnce() -> Result<(u32, u32, Vec<u8>), Vec<u8>>,
    decode: impl FnOnce(&[u8]) -> Option<Vec<u8>>,
) -> Result<String, RasterUploadRefusal> {
    if scene_host_retiring(surface_id) {
        return Err(RasterUploadRefusal::Busy);
    }
    if surface_id.len().saturating_add(layer_id.len()).saturating_add(32) > RASTER_UPLOAD_KEY_BYTE_CAPACITY {
        return Err(RasterUploadRefusal::Invalid("raster key exceeded its fixed credits"));
    }
    if source_identity.len() > RASTER_UPLOAD_BYTE_CAPACITY.saturating_mul(2) {
        return Err(RasterUploadRefusal::Invalid("raster source exceeded Canvas upload credits"));
    }
    let key = format!("canvas-image:{surface_id}:{layer_id}");
    let ready = PENDING_RASTER_STATE.with(|cell| cell.borrow_mut().get_or_insert_with(surface_id.to_string(), PendingRasterSurface::default).is_some_and(|surface| surface.admits_upload()));
    if !ready {
        return Err(RasterUploadRefusal::Busy);
    }
    PENDING_RASTER_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let surface = surfaces.get_mut(surface_id).ok_or(RasterUploadRefusal::Busy)?;
        match PreparedRasterReservation::try_reserve_source(key, source_identity.len()) {
            Ok(reservation) => {
                surface.admission = Some(reservation);
                Ok(())
            }
            Err(rejected) => Err(surface.park_refusal(rejected)),
        }
    })?;
    let (width, height, retained_source) = match dimensions() {
        Ok(dimensions) => dimensions,
        Err(retained_source) => return Err(refuse_raster_admission(surface_id, |reservation| reservation.reject_with_retained("raster source dimensions failed", Vec::new(), retained_source).into_content_refusal())),
    };
    let retained_source = PENDING_RASTER_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let surface = surfaces.get_mut(surface_id).ok_or(RasterUploadRefusal::Busy)?;
        let reservation = surface.admission.take().ok_or(RasterUploadRefusal::Busy)?;
        match reservation.claim_with_retained(width, height, retained_source) {
            Ok((reservation, retained_source)) => {
                surface.admission = Some(reservation);
                Ok(retained_source)
            }
            Err(rejected) => Err(surface.park_refusal(rejected.into_content_refusal())),
        }
    })?;
    let Some(pixels) = decode(&retained_source) else {
        return Err(refuse_raster_admission(surface_id, |reservation| reservation.reject_with_retained("raster source decode failed", Vec::new(), retained_source).into_content_refusal()));
    };
    let expected = (width as usize).saturating_mul(height as usize).saturating_mul(4);
    if expected > RASTER_UPLOAD_BYTE_CAPACITY || pixels.len() != expected {
        return Err(refuse_raster_admission(surface_id, |reservation| reservation.reject_with_retained("decoded raster exceeded Canvas upload credits", pixels, retained_source).into_content_refusal()));
    }
    let admitted = PENDING_RASTER_STATE.with(|cell| cell.borrow_mut().get_mut(surface_id).and_then(|surface| surface.admission.take()).map(|reservation| reservation.finalize(pixels, retained_source, width, height)));
    let (producer, published_key) = match admitted {
        Some(Ok(admitted)) => admitted,
        Some(Err(rejected)) => {
            PENDING_RASTER_STATE.with(|cell| {
                if let Some(surface) = cell.borrow_mut().get_mut(surface_id) {
                    surface.rejected = Some(rejected);
                }
            });
            return Err(RasterUploadRefusal::Busy);
        }
        None => return Err(RasterUploadRefusal::Busy),
    };
    let accepted = PENDING_RASTER_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let Some(surface) = surfaces.get_mut(surface_id) else { return false };
        match surface.queue.push_back(producer) {
            Ok(()) => true,
            Err(mut producer) => {
                producer.begin_close();
                surface.closing = Some(producer);
                false
            }
        }
    });
    if !accepted {
        return Err(RasterUploadRefusal::Busy);
    }
    Ok(published_key)
}

/// 🖼️ Parks `surface_id`'s in-flight admission as the refusal `reject` turns it into, answering the typed outcome.
fn refuse_raster_admission(surface_id: &str, reject: impl FnOnce(PreparedRasterReservation) -> PreparedRasterRejected) -> RasterUploadRefusal {
    PENDING_RASTER_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let Some(surface) = surfaces.get_mut(surface_id) else { return RasterUploadRefusal::Busy };
        let Some(reservation) = surface.admission.take() else { return RasterUploadRefusal::Busy };
        surface.park_refusal(reject(reservation))
    })
}


/** Clamps checkerboard cell iteration to the world-space rect actually visible through `inner`
 * (intersected with the full `±extent/2` grid) instead of always walking the whole grid — a
 * continuously-rendering surface (paint-2d) was pushing up to `(extent/cell)^2` solid quads every
 * single frame regardless of zoom/pan, which starves headless WebGPU frame pacing. */

/** 📐️ Theme-aware LOD world grid for canvas-2d — same large/medium/small/micro steps as flow and infinite boards. */

//#region Canvas2dFillBlend
/** 🎨️ Reads an `[r,g,b,a?]` channel array (each `0..1`, matches `rgbaToCss` in `canvas-2d-host.tsx`)
 * into an `Rgba`, multiplying alpha by the layer's resolved `opacity`. Missing channels fall back to
 * a neutral slate gray (matches the React reference's `rgba(148, 163, 184, opacity)` default). */

/** 🌈️ Samples a `CanvasGradientStop[]` list at `t ∈ [0,1]`, linearly interpolating between the
 * bracketing stops — mirrors `CanvasGradient.addColorStop` sampling semantics. */

/** 🌗️ A single separable-blend-mode channel formula (W3C Compositing and Blending Level 1 §5.2) —
 * `cb` is the backdrop channel, `cs` the source channel, both `0..1`. */

fn canvas_lum(c: [f32; 3]) -> f32 {
    0.3 * c[0] + 0.59 * c[1] + 0.11 * c[2]
}

fn canvas_clip_color(c: [f32; 3]) -> [f32; 3] {
    let l = canvas_lum(c);
    let n = c[0].min(c[1]).min(c[2]);
    let x = c[0].max(c[1]).max(c[2]);
    let mut out = c;
    if n < 0.0 {
        for ch in out.iter_mut() {
            *ch = l + (*ch - l) * l / (l - n).max(1e-6);
        }
    }
    if x > 1.0 {
        for ch in out.iter_mut() {
            *ch = l + (*ch - l) * (1.0 - l) / (x - l).max(1e-6);
        }
    }
    out
}

fn canvas_set_lum(c: [f32; 3], l: f32) -> [f32; 3] {
    let d = l - canvas_lum(c);
    canvas_clip_color([c[0] + d, c[1] + d, c[2] + d])
}

fn canvas_sat(c: [f32; 3]) -> f32 {
    c[0].max(c[1]).max(c[2]) - c[0].min(c[1]).min(c[2])
}

fn canvas_set_sat(c: [f32; 3], s: f32) -> [f32; 3] {
    let mut idx = [0usize, 1, 2];
    idx.sort_by(|&a, &b| c[a].partial_cmp(&c[b]).unwrap_or(std::cmp::Ordering::Equal));
    let (imin, imid, imax) = (idx[0], idx[1], idx[2]);
    let mut out = [0.0f32; 3];
    if c[imax] > c[imin] {
        out[imid] = (c[imid] - c[imin]) * s / (c[imax] - c[imin]);
        out[imax] = s;
    }
    out[imin] = 0.0;
    out
}

/** 🎨️ Approximates the React `Canvas2dScene`'s 16 CSS blend modes (`BLEND_MODE_TO_COMPOSITE` in
 * `canvas-2d-host.tsx`) by pre-blending the resolved fill/stroke color against `backdrop` — a
 * stand-in for true per-pixel GPU compositing, which would require a `wgpu::BlendState` change per
 * draw call in the shared `ui_wgpu` pipeline (out of scope for this ticket's Canvas2d/Paint2d
 * regions; see ticket 26/07/11/WGPU-RENDERER-FULL-PARITY). The four non-separable modes
 * (hue/saturation/color/luminosity) follow the W3C SetLum/SetSat algorithm exactly. */

//#endregion Canvas2dFillBlend

//#region Canvas2dShapes
/// 🟠️ The band count a linear/radial gradient is approximated with — `ui_wgpu`'s draw list has no
/// gradient primitive, so `fillStyleToPaint`'s `createLinearGradient`/`createRadialGradient` become
/// this many solid quads/rings along the gradient axis.
const CANVAS_GRADIENT_BANDS: usize = 10;
const CANVAS_CIRCLE_SEGMENTS: usize = 28;

fn canvas_circle_points(cx: f32, cy: f32, radius: f32, segments: usize) -> Vec<[f32; 2]> {
    (0..segments)
        .map(|i| {
            let a = (i as f32 / segments as f32) * std::f32::consts::TAU;
            [cx + a.cos() * radius, cy + a.sin() * radius]
        })
        .collect()
}

/** 🌈️ Bands a linear gradient across `clip` (scissor-bounded to the shape's screen bbox) as
 * `CANVAS_GRADIENT_BANDS` solid quads perpendicular to the `(x1,y1)-(x2,y2)` axis — `ui_wgpu::wgpu::
 * DrawList` has no per-vertex gradient primitive, see `canvas_apply_blend_mode` doc comment. */

/** 🌈️ Bands a radial gradient as `CANVAS_GRADIENT_BANDS` concentric circles painted outer-to-inner
 * (painter's algorithm — smaller/later circles overpaint the center), scissor-bounded to `clip`. */

/** 🖌️ Resolves and draws a Canvas2dScene draw record's `fill` (solid / linear / radial gradient) and
 * `stroke`, matching `drawSceneNode`'s fill/stroke resolution in `canvas-2d-host.tsx`. */

//#endregion Canvas2dShapes

/** 🟡️ Selection-ring colors ported verbatim from `drawBoundsLayer`'s literal
 * `"rgba(251, 191, 36, 0.95|0.28)"` strings in `canvas-2d-host.tsx` — an amber that isn't backed by
 * any `Theme` token, so it's kept local to this region rather than mapped onto `theme.accent`
 * (which resolves to the app's red/crimson accent and previously made the ring the wrong hue). */
const CANVAS2D_SELECTION_RING: Rgba = Rgba::new(0.984_314, 0.749_02, 0.141_176, 0.95);
const CANVAS2D_SELECTION_GLOW: Rgba = Rgba::new(0.984_314, 0.749_02, 0.141_176, 0.28);

//#endregion Canvas2d

//#region Canvas2dTests
#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-canvas2d/🦀️.rs"]
mod canvas2d_tests;
//#endregion Canvas2dTests
