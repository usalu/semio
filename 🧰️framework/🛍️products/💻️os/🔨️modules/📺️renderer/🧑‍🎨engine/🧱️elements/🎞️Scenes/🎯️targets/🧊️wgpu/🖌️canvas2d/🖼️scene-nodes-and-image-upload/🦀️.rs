//! 🖼️ Scene nodes and image upload: the paint order, the scene node render and path fills, the canvas constants, the raster ledger reservation, the image upload and the admission refusal. See [`super`].

use super::*;

//#region Canvas2d
/// 🧅️ Keeps the Canvas2d paint in record order across the draw list's buckets: one draw layer paints its quads (solid
/// fills, glyphs) before its vector triangles, so a quad that follows vector paint opens a fresh draw layer. Every layer
/// sits under the canvas scissor, which clips the records to the canvas like React's `<canvas>` element.
pub(super) struct CanvasPaintOrder {
    pub(crate) inner: Rect,
    vectors: bool,
}

impl CanvasPaintOrder {
    pub(super) fn open(draw: &mut semio_framework_ui_draw_contract::DrawList, inner: Rect) -> Self {
        draw.push_scissor(inner);
        Self { inner, vectors: false }
    }

    /// 🔺️ Records that vector triangles were pushed into the current draw layer.
    pub(super) fn vector(&mut self) {
        self.vectors = true;
    }

    /// 🟦️ Readies the draw list for a quad: a fresh draw layer when the current one already holds vector paint.
    pub(super) fn quad(&mut self, draw: &mut semio_framework_ui_draw_contract::DrawList) {
        if self.vectors {
            draw.pop_scissor();
            draw.push_scissor(self.inner);
            self.vectors = false;
        }
    }

    pub(super) fn close(self, draw: &mut semio_framework_ui_draw_contract::DrawList) {
        draw.pop_scissor();
    }
}

/// 🧩️ Whether React paints `layer` through `drawSceneNode`: it carries `segments`, a `text` or an `image.src`.
pub(super) fn canvas_layer_is_scene_node(layer: &CanvasLayer) -> bool {
    layer.segments.as_ref().is_some_and(|segments| !segments.is_empty()) || layer.text.is_some() || layer.image.as_ref().and_then(|image| image.src.as_deref()).is_some_and(|src| !src.is_empty())
}

/// 🎨️ `rgbaToCss`: an `[r, g, b, a?]` channel list with its alpha times `opacity`; the slate fallback below three channels.
fn scene_rgba(color: Option<&[f64]>, opacity: f32) -> Rgba {
    canvas_color_channels(color.filter(|channels| channels.len() >= 3).unwrap_or(&[]), opacity)
}

pub fn screen_points(points: &[[f64; 2]]) -> Vec<[f32; 2]> {
    points.iter().map(|point| [point[0] as f32, point[1] as f32]).collect()
}

/// 🖌️ The wgpu twin of `drawSceneNode` (`📐️Canvas2dHost/🎨️paint`): under the node `transform` and the camera, the path's
/// fill (`evenodd` unless `nonzero`) and stroke (width, caps, joins, dash in layer units) from `canvas2d_paint`, then the
/// text lines (`◻️2d` line law, sized with the map) and the `image.src` raster at the node origin. Approximations of this
/// renderer: group opacity is applied per piece and a stroke is at least one pixel wide.
pub(super) fn render_canvas_scene_node(scene: &UiComponentSceneNode, layer: &CanvasLayer, viewport: &Viewport, inner: Rect, order: &mut CanvasPaintOrder, ctx: &mut FrameworkWidgetContext<'_>) {
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
                semio_framework_ui_wgpu_widget_kit::draw_text_face_affine_on(ctx.draw,ctx.atlas,face,line,origin,pixels,color,matrix);
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
fn push_scene_path_fill(draw: &mut semio_framework_ui_draw_contract::DrawList, contours: &[crate::canvas2d_paint::Contour], rule: crate::canvas2d_paint::FillRule, clip: (f64, f64), fill: &CanvasFillJson, map: &crate::canvas2d_paint::Affine, opacity: f32, blend: Option<&str>, backdrop: Rgba) {
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
pub const CANVAS_2D_EMPTY_LABEL: &str = "Empty canvas";

/** 🧾️ Reserves the fixed raster ledger before source preparation or the one-backing decoder. */
/// 🖼️📥️ Queues ONE already-decoded RGBA8 bitmap under its own verbatim `key` — everything
/// [`queue_canvas_image_upload_with`] does for an encoded Canvas layer minus the decode and minus the
/// `canvas-image:<surface>:<layer>` key convention. Its caller is the renderer frame's drain of
/// `ui_wgpu::wgpu::take_ui_image_upload`: `admit_ui_image` already decoded that `data:` PNG in
/// first-party code (`semio-framework-pixels`) and the retained paint keys its `KIND_RASTER` quad by
/// the image `src` itself, so the key must survive unchanged or the quad finds no texture.
pub fn queue_decoded_raster_upload(surface_id: &str, key: String, width: u32, height: u32, mut pixels: Vec<u8>) -> Option<String> {
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
pub fn queue_canvas_image_upload_with(
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
//#endregion Canvas2d
