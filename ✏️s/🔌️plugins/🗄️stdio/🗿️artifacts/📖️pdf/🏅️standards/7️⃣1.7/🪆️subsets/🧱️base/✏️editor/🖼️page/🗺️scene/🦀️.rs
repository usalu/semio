//! 🗺️ PDF page editor scene: the addressable page objects, the canvas layers and the hit test.

use super::*;

#[path = "🔡️type3-text/🦀️.rs"]
mod type3_text;

#[path = "🔠️truetype-text/🦀️.rs"]
pub(super) mod truetype_text;

//#region 🗺️Scene
/// 👁 Every addressable object in painting order.
pub fn objects(snapshot: &PdfSnapshot) -> Vec<PageObject> {
    measured_objects(snapshot, &mut FontMeasurements::new(snapshot))
}

pub(super) fn measured_objects(snapshot: &PdfSnapshot, metrics: &mut FontMeasurements<'_>) -> Vec<PageObject> {
    let mut found = Vec::new();
    for (page_index, page) in snapshot.pages.iter().enumerate() {
        found.extend(page_objects(snapshot, page_index, page, metrics));
    }
    found
}

/// 🖼️ Host canvas layers for the whole document.
pub fn canvas_layers(snapshot: &PdfSnapshot, selected: Option<&str>) -> String {
    let viewports: Vec<_> = page_viewports(snapshot).collect();
    let mut metrics = FontMeasurements::new(snapshot);
    let placed = placement(snapshot, &viewports, &mut metrics);
    let mut layers = Vec::new();
    for (page_index, viewport) in viewports.iter().enumerate() {
        let [_, top, width, height] = viewport.clip_rect();
        layers.push(json!({ "id": format!("p{page_index}:paper"), "segments": rect_segments(0.0, top, width, height), "fill": { "color": [1.0, 1.0, 1.0, 1.0] }, "stroke": { "color": [0.75, 0.75, 0.78, 1.0], "width": 1.0 } }));
    }
    let mut forms = FormPaint { active: Vec::new(), active_fonts: Vec::new(), remaining_ops: MAX_FORM_PAINT_OPERATIONS, metrics };
    for item in &placed {
        let start = layers.len();
        let page = &snapshot.pages[item.object.page];
        paint_object(snapshot, page, &page.content, item, &mut layers, &mut forms);
        if selected == Some(item.object.id.as_str()) {
            layers.push(json!({ "id": format!("{}.selection", item.object.id), "segments": rect_segments(item.canvas_x, item.canvas_y, item.canvas_w.max(1.0), item.canvas_h.max(1.0)), "stroke": { "color": [0.1, 0.45, 0.95, 1.0], "width": 2.0 } }));
        }
        for layer in &mut layers[start..] {
            constrain_clip(layer, item.viewport.clip_rect());
        }
    }
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}

const MAX_FORM_PAINT_DEPTH: usize = 24;
const MAX_FORM_PAINT_OPERATIONS: usize = 50_000;

struct FormPaint<'a> {
    active: Vec<&'a str>,
    active_fonts: Vec<&'a str>,
    remaining_ops: usize,
    metrics: FontMeasurements<'a>,
}

fn paint_object<'a>(snapshot: &'a PdfSnapshot, page: &PdfPage, content: &[PdfOp], item: &Placed, layers: &mut Vec<Value>, forms: &mut FormPaint<'a>) {
    let form = match item.object.kind {
        ObjectKind::Form => snapshot.form(&item.object.text).map(|form| (form, item.object.ctm)),
        ObjectKind::Annotation => match page.annotations.get(item.object.start) {
            Some(annotation) if !annotation_visible(annotation) => return,
            Some(annotation) => annotation_appearance(snapshot, annotation),
            None => None,
        },
        _ => None,
    };
    if let Some((form, ctm)) = form {
        paint_form(snapshot, page, item, form, ctm, layers, forms);
        return;
    }
    let first = layers.len();
    let color = [item.object.fill[0] as f32, item.object.fill[1] as f32, item.object.fill[2] as f32, 1.0];
    let stroke = [item.object.stroke[0] as f32, item.object.stroke[1] as f32, item.object.stroke[2] as f32, 1.0];
    match item.object.kind {
        ObjectKind::Text => {
            let mode = item.object.text_placement.as_ref().map_or(0, |text| text.render_mode);
            if mode == 3 { return; }
            if type3_text::paint_type3_text(snapshot, page, item, content, layers, forms) {
                attach_clips(page, item, &mut layers[first..]);
                return;
            }
            let paint = mode & 3;
            if paint == 3 { return; }
            if truetype_text::paint_truetype_text(item, content, paint, color, stroke, layers, forms) {
                attach_clips(page, item, &mut layers[first..]);
                return;
            }
            let display = |x: f64, y: f64| {
                let (x, y) = apply(item.object.text_transform, x, -y);
                display_point(page, item.top, x, y)
            };
            let (x, y) = display(0.0, 0.0);
            let (along_x, along_y) = display(1.0, 0.0);
            let (down_x, down_y) = display(0.0, 1.0);
            let font_size = item.object.font_size.max(1.0);
            let base_font = snapshot.font(&item.object.font).map(|font| font.base_font()).unwrap_or(&item.object.font);
            let family = if base_font.starts_with("Courier") {
                "Share Tech Mono"
            } else if base_font.starts_with("Times") {
                "Kelly Slab"
            } else {
                "Anta"
            };
            let mut layer = json!({
                "id": item.object.id,
                "kind": "text",
                "x": 0.0,
                "y": -font_size,
                "transform": [along_x - x, along_y - y, down_x - x, down_y - y, x, y],
                "width": item.canvas_w.max(1.0),
                "height": item.canvas_h.max(1.0),
                "text": { "content": item.object.text, "size": font_size, "fontFamily": family },
            });
            if paint != 1 { layer["fill"] = json!({ "color": color }); }
            if paint != 0 { layer["stroke"] = json!({ "color": stroke, "width": item.object.line_width }); }
            layers.push(layer);
        }
        ObjectKind::Vector => {
            let (fill, even_odd, stroke_path) = vector_paint(item, &content[item.object.end]);
            let segments = match stroke_path {
                Some((inverse, _, _)) => mapped_vector_segments(&item.object.path, |(x, y)| apply(inverse, x, y), !item.object.vector_stroke.dash.is_empty()),
                None => vector_segments(page, item.top, &item.object.path),
            };
            let mut layer = json!({ "id": item.object.id, "segments": segments });
            if fill {
                layer["fill"] = json!({ "color": color });
                layer["fillRule"] = json!(if even_odd { "evenodd" } else { "nonzero" });
            }
            if let Some((_, transform, width)) = stroke_path {
                let style = &item.object.vector_stroke;
                layer["transform"] = json!(transform);
                layer["stroke"] = json!({
                    "color": stroke, "width": width, "dash": style.dash.as_ref(), "dashOffset": style.phase,
                    "cap": style.cap, "join": style.join, "miterLimit": style.miter_limit,
                });
            }
            layers.push(layer);
        }
        ObjectKind::Image | ObjectKind::Form | ObjectKind::Shading | ObjectKind::Annotation => {
            let preview = match item.object.kind {
                ObjectKind::Image => image_layer(snapshot, content, item),
                ObjectKind::Shading => shading_layer(snapshot, page, item),
                _ => None,
            };
            if let Some(layer) = preview {
                layers.push(layer);
            } else {
                layers.push(json!({ "id": item.object.id, "segments": rect_segments(item.canvas_x, item.canvas_y, item.canvas_w.max(1.0), item.canvas_h.max(1.0)), "fill": { "color": if item.object.kind == ObjectKind::Image { [0.85, 0.88, 0.92, 1.0] } else { color } }, "stroke": { "color": stroke, "width": 1.0 } }));
                if !item.object.text.is_empty() {
                    layers.push(json!({ "id": format!("{}.label", item.object.id), "kind": "text", "x": item.canvas_x + 4.0, "y": item.canvas_y + 4.0, "width": item.canvas_w.max(1.0), "height": 12.0, "text": { "content": item.object.text, "size": 12.0, "fontFamily": "Anta" }, "fill": { "color": [0.1, 0.1, 0.12, 1.0] } }));
                }
            }
        }
    }
    attach_clips(page, item, &mut layers[first..]);
}

fn attach_clips(page: &PdfPage, item: &Placed, layers: &mut [Value]) {
    if !item.object.clip_quads.is_empty() {
        let clips: Vec<_> = item.object.clip_quads.iter().map(|quad| json!(display_clip(item, *quad))).collect();
        for layer in layers.iter_mut() { add_clips(layer, "clipPolygons", &clips); }
    }
    if !item.object.clip_paths.is_empty() {
        let paths: Vec<_> = item.object.clip_paths.iter().map(|clip| json!({
            "fillRule": if clip.even_odd { "evenodd" } else { "nonzero" },
            "segments": vector_segments(page, item.top, &clip.path),
        })).collect();
        for layer in layers { add_clips(layer, "clipPaths", &paths); }
    }
}

fn add_clips(layer: &mut Value, field: &str, clips: &[Value]) {
    if let Some(existing) = layer.get_mut(field).and_then(Value::as_array_mut) { existing.extend_from_slice(clips); }
    else { layer[field] = Value::Array(clips.to_vec()); }
}

fn annotation_visible(annotation: &PdfAnnotation) -> bool { annotation.flags & (2 | 32) == 0 }

fn annotation_appearance<'a>(snapshot: &'a PdfSnapshot, annotation: &PdfAnnotation) -> Option<(&'a PdfFormXObject, PdfMatrix)> {
    let form_id = match &annotation.appearance.as_ref()?.normal {
        PdfAppearanceEntry::Single { form } => form,
        PdfAppearanceEntry::States { states } => {
            let state = annotation.appearance_state.as_deref()?;
            &states.iter().find(|appearance| appearance.state == state)?.form
        }
    };
    let form = snapshot.form(form_id)?;
    let (left, bottom, width, height) = semio_s_artifact_stdio_pdf_page_coordinates::bounds(&form_corners(form, PDF_IDENTITY_MATRIX));
    let [x0, y0, x1, y1] = annotation.rect;
    let [x, y, w, h] = [x0.min(x1), y0.min(y1), (x1 - x0).abs(), (y1 - y0).abs()];
    if width <= 0.0 || height <= 0.0 || w <= 0.0 || h <= 0.0 { return None; }
    let [sx, sy] = [w / width, h / height];
    let matrix = [sx, 0.0, 0.0, sy, x - left * sx, y - bottom * sy];
    matrix.iter().all(|value| value.is_finite()).then_some((form, matrix))
}

fn paint_form<'a>(snapshot: &'a PdfSnapshot, page: &PdfPage, item: &Placed, form: &'a PdfFormXObject, ctm: PdfMatrix, layers: &mut Vec<Value>, paint: &mut FormPaint<'a>) {
    if paint.active.len() >= MAX_FORM_PAINT_DEPTH || paint.active.contains(&form.id.as_str()) || form.content.len() > paint.remaining_ops {
        return;
    }
    paint.active.push(&form.id);
    paint.remaining_ops -= form.content.len();
    let clip = [item.canvas_x, item.canvas_y, item.canvas_w, item.canvas_h];
    let first = layers.len();
    for mut object in form_objects(snapshot, form, &item.object, ctm, &mut paint.metrics) {
        object.id = format!("{}/{}", item.object.id, object.id);
        let (canvas_x, canvas_y, canvas_w, canvas_h) = item.viewport.rect(object.x, object.y, object.width, object.height);
        let child = Placed { object, top: item.top, viewport: item.viewport, canvas_x, canvas_y, canvas_w, canvas_h };
        paint_object(snapshot, page, &form.content, &child, layers, paint);
    }
    paint.active.pop();
    let quad = canvas_form_quad(page, item, form, ctm);
    let oriented = quad.iter().zip(quad.iter().cycle().skip(1)).any(|(a, b)| a[0] != b[0] && a[1] != b[1]);
    for layer in &mut layers[first..] {
        constrain_clip(layer, clip);
        if oriented {
            let mut polygons = layer.get_mut("clipPolygons").and_then(Value::as_array_mut);
            if let Some(polygons) = &mut polygons {
                polygons.push(json!(quad));
            } else {
                layer["clipPolygons"] = json!([quad]);
            }
        }
    }
}

fn display_clip(item: &Placed, quad: ClipQuad) -> [[f64; 2]; 4] {
    quad.map(|(x, y)| {
        let (x, y) = item.viewport.point(x, y);
        [x, y]
    })
}

fn canvas_form_quad(page: &PdfPage, item: &Placed, form: &PdfFormXObject, ctm: PdfMatrix) -> [[f64; 2]; 4] {
    form_corners(form, ctm).map(|(x, y)| {
        let (x, y) = display_point(page, item.top, x, y);
        [x, y]
    })
}

use semio_s_artifact_stdio_pdf_page_coordinates::{quad_contains, stroke_contains, vector_contains};

pub(super) fn constrain_clip(layer: &mut Value, clip: [f64; 4]) {
    let current = layer.get("clipRect").and_then(|rect| {
        Some([rect[0].as_f64()?, rect[1].as_f64()?, rect[2].as_f64()?, rect[3].as_f64()?])
    });
    let [x, y, width, height] = clip;
    layer["clipRect"] = if let Some([left, top, w, h]) = current {
        let (intersection_x, intersection_y) = (x.max(left), y.max(top));
        json!([intersection_x, intersection_y, ((x + width).min(left + w) - intersection_x).max(0.0), ((y + height).min(top + h) - intersection_y).max(0.0)])
    } else {
        json!(clip)
    };
}

fn vector_paint(item: &Placed, op: &PdfOp) -> (bool, bool, Option<(PdfMatrix, PdfMatrix, f64)>) {
    let fill = !matches!(op, PdfOp::Stroke | PdfOp::CloseStroke);
    let even_odd = matches!(op, PdfOp::FillEvenOdd | PdfOp::FillStrokeEvenOdd | PdfOp::CloseFillStrokeEvenOdd);
    let mut stroke = None;
    if !matches!(op, PdfOp::Fill | PdfOp::FillEvenOdd) {
        let ctm = item.object.ctm;
        if let Some(inverse) = invert(ctm) {
            let display = |x, y| { let (x, y) = apply(ctm, x, y); item.viewport.point(x, y) };
            let ((x, y), (a, b), (c, d)) = (display(0.0, 0.0), display(1.0, 0.0), display(0.0, 1.0));
            let transform = [a - x, b - y, c - x, d - y, x, y];
            let width = item.object.line_width;
            let width = if width == 0.0 { 0.5 / (a - x).hypot(b - y).max((c - x).hypot(d - y)) } else { width };
            if width > 0.0 && width.is_finite() { stroke = Some((inverse, transform, width)); }
        }
    }
    (fill, even_odd, stroke)
}

/// 🎯 Top-most selectable object at the canvas point.
pub fn hit_test(snapshot: &PdfSnapshot, x: f64, y: f64) -> Option<String> {
    let (page_index, viewport) = page_viewports(snapshot).enumerate().find(|(_, viewport)| viewport.contains(x, y))?;
    let page = &snapshot.pages[page_index];
    page_objects(snapshot, page_index, page, &mut FontMeasurements::new(snapshot)).into_iter().rev().map(|object| Placed::new(object, viewport)).find(|item| {
        if !item.object.clip_quads.iter().all(|quad| quad_contains(display_clip(item, *quad), x, y))
            || !item.object.clip_paths.iter().all(|clip| vector_contains(&clip.path, item.viewport, (x, y), true, clip.even_odd, None)) { return false; }
        if item.object.kind == ObjectKind::Vector {
            let op = &snapshot.pages[item.object.page].content[item.object.end];
            let (fill, even_odd, stroke) = vector_paint(item, op);
            let padding = stroke.map_or(0.0, |(_, [a, b, c, d, ..], width)| {
                width * (a.hypot(b) + c.hypot(d)) * item.object.vector_stroke.miter_limit.max(2.0) / 2.0
            });
            if x < item.canvas_x - padding || x > item.canvas_x + item.canvas_w + padding
                || y < item.canvas_y - padding || y > item.canvas_y + item.canvas_h + padding { return false; }
            return (fill && vector_contains(&item.object.path, item.viewport, (x, y), true, even_odd, None))
                || stroke.is_some_and(|(inverse, transform, width)| {
                    stroke_contains(&item.object.path, inverse, transform, (x, y), width, &item.object.vector_stroke)
                });
        }
        x >= item.canvas_x && x <= item.canvas_x + item.canvas_w && y >= item.canvas_y && y <= item.canvas_y + item.canvas_h
            && match item.object.kind {
                ObjectKind::Image => {
                    let quad = semio_s_artifact_stdio_pdf_page_coordinates::unit_square(item.object.ctm).map(|(u, v)| {
                        let (cx, cy) = item.viewport.point(u, v);
                        [cx, cy]
                    });
                    quad_contains(quad, x, y)
                }
                ObjectKind::Form => snapshot.form(&item.object.text).is_none_or(|form| quad_contains(canvas_form_quad(&snapshot.pages[item.object.page], item, form, item.object.ctm), x, y)),
                ObjectKind::Annotation => snapshot.pages[item.object.page].annotations.get(item.object.start).is_some_and(annotation_visible),
                _ => true,
            }
    }).map(|item| item.object.id)
}
//#endregion 🗺️Scene
