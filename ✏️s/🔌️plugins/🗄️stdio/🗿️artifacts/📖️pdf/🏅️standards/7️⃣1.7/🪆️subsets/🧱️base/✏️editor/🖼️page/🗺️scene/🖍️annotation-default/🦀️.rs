//! 🖍️ Paints annotations that carry no appearance stream the way readers synthesize them.
//! [PDF 32000-1 §12.5.6 Annotation types](https://opensource.adobe.com/dc-acrobat-sdk-docs/pdfstandards/PDF32000_2008.pdf)

use super::super::*;

const KAPPA: f64 = 0.552_284_749_830_793_4;

/// 🪄 Paints a subtype default appearance; `false` when the subtype has none to synthesize and nothing was painted.
pub(super) fn paint_default_annotation(snapshot: &PdfSnapshot, page: &PdfPage, item: &Placed, annotation: &PdfAnnotation, layers: &mut Vec<Value>) -> bool {
    let colour = |values: &[f64], fallback: [f64; 3]| -> [f64; 3] {
        let space = match values.len() { 1 => PdfColorSpace::DeviceGray, 3 => PdfColorSpace::DeviceRgb, 4 => PdfColorSpace::DeviceCmyk, _ => return fallback };
        function_colour::rgb(snapshot, &space, values).unwrap_or(fallback)
    };
    let opacity = annotation.markup.as_ref().and_then(|markup| markup.opacity).unwrap_or(1.0).clamp(0.0, 1.0);
    let width = annotation.border.as_ref().map_or(1.0, |border| border.width);
    let dash = annotation.border.as_ref().and_then(|border| border.dash.clone()).filter(|dash| !dash.is_empty());
    let point = |x: f64, y: f64| { let (x, y) = display_point(page, item.top, x, y); json!([x, y]) };
    let polygon = |points: &[(f64, f64)], close: bool| {
        let mut segments: Vec<Value> = points.iter().enumerate().map(|(index, &(x, y))| json!({ "kind": if index == 0 { "move" } else { "line" }, "to": point(x, y) })).collect();
        if close { segments.push(json!({ "kind": "close" })); }
        Value::Array(segments)
    };
    let stroke = |segments: Value, rgb: [f64; 3], suffix: &str| json!({
        "id": format!("{}.{suffix}", item.object.id), "segments": segments, "opacity": opacity,
        "stroke": { "color": [rgb[0], rgb[1], rgb[2], 1.0], "width": width.max(0.0), "dash": dash },
    });
    let [x0, y0, x1, y1] = annotation.rect;
    let (left, bottom, right, top) = (x0.min(x1), y0.min(y1), x0.max(x1), y0.max(y1));
    let stroke_colour = colour(&annotation.color, [0.0, 0.0, 0.0]);
    let quads = |points: &[f64]| points.chunks_exact(8).map(|quad| [(quad[0], quad[1]), (quad[2], quad[3]), (quad[4], quad[5]), (quad[6], quad[7])]).collect::<Vec<_>>();
    match &annotation.kind {
        PdfAnnotationKind::Highlight { quad_points } => {
            let rgb = colour(&annotation.color, [1.0, 1.0, 0.0]);
            for (index, [a, b, c, d]) in quads(quad_points).into_iter().enumerate() {
                layers.push(json!({ "id": format!("{}.{index}", item.object.id), "segments": polygon(&[a, b, d, c], true), "fill": { "color": [rgb[0], rgb[1], rgb[2], 1.0] }, "opacity": opacity, "blendMode": "multiply" }));
            }
        }
        PdfAnnotationKind::Underline { quad_points } | PdfAnnotationKind::Squiggly { quad_points } => {
            for (index, [_, _, c, d]) in quads(quad_points).into_iter().enumerate() {
                layers.push(stroke(polygon(&[c, d], false), stroke_colour, &index.to_string()));
            }
        }
        PdfAnnotationKind::StrikeOut { quad_points } => {
            for (index, [a, b, c, d]) in quads(quad_points).into_iter().enumerate() {
                let mid = |p: (f64, f64), q: (f64, f64)| ((p.0 + q.0) / 2.0, (p.1 + q.1) / 2.0);
                layers.push(stroke(polygon(&[mid(a, c), mid(b, d)], false), stroke_colour, &index.to_string()));
            }
        }
        PdfAnnotationKind::Square { interior_color, rect_differences } | PdfAnnotationKind::Circle { interior_color, rect_differences } => {
            let [dl, db, dr, dt] = rect_differences.unwrap_or([0.0; 4]);
            let half = width.max(0.0) / 2.0;
            let (l, b, r, t) = (left + dl + half, bottom + db + half, right - dr - half, top - dt - half);
            if r <= l || t <= b { return false; }
            let outline = if matches!(annotation.kind, PdfAnnotationKind::Square { .. }) {
                polygon(&[(l, b), (r, b), (r, t), (l, t)], true)
            } else {
                let (cx, cy, rx, ry) = ((l + r) / 2.0, (b + t) / 2.0, (r - l) / 2.0, (t - b) / 2.0);
                let cubic = |c1: (f64, f64), c2: (f64, f64), end: (f64, f64)| json!({ "kind": "cubic", "ctrl1": point(c1.0, c1.1), "ctrl2": point(c2.0, c2.1), "to": point(end.0, end.1) });
                json!([
                    { "kind": "move", "to": point(cx + rx, cy) },
                    cubic((cx + rx, cy + KAPPA * ry), (cx + KAPPA * rx, cy + ry), (cx, cy + ry)),
                    cubic((cx - KAPPA * rx, cy + ry), (cx - rx, cy + KAPPA * ry), (cx - rx, cy)),
                    cubic((cx - rx, cy - KAPPA * ry), (cx - KAPPA * rx, cy - ry), (cx, cy - ry)),
                    cubic((cx + KAPPA * rx, cy - ry), (cx + rx, cy - KAPPA * ry), (cx + rx, cy)),
                    { "kind": "close" },
                ])
            };
            if let Some(values) = interior_color.as_ref().filter(|values| !values.is_empty()) {
                let rgb = colour(values, [1.0, 1.0, 1.0]);
                layers.push(json!({ "id": format!("{}.fill", item.object.id), "segments": outline.clone(), "fill": { "color": [rgb[0], rgb[1], rgb[2], 1.0] }, "opacity": opacity }));
            }
            if width > 0.0 { layers.push(stroke(outline, stroke_colour, "stroke")); }
        }
        PdfAnnotationKind::Line { points: [ax, ay, bx, by], .. } => layers.push(stroke(polygon(&[(*ax, *ay), (*bx, *by)], false), stroke_colour, "line")),
        PdfAnnotationKind::Ink { paths } => {
            for (index, path) in paths.iter().enumerate() {
                let points: Vec<_> = path.chunks_exact(2).map(|pair| (pair[0], pair[1])).collect();
                if points.len() > 1 { layers.push(stroke(polygon(&points, false), stroke_colour, &index.to_string())); }
            }
        }
        PdfAnnotationKind::Polygon { vertices, .. } | PdfAnnotationKind::PolyLine { vertices, .. } => {
            let points: Vec<_> = vertices.chunks_exact(2).map(|pair| (pair[0], pair[1])).collect();
            if points.len() > 1 { layers.push(stroke(polygon(&points, matches!(annotation.kind, PdfAnnotationKind::Polygon { .. })), stroke_colour, "path")); }
        }
        PdfAnnotationKind::Link { .. } | PdfAnnotationKind::Popup { .. } => {}
        PdfAnnotationKind::Text { .. } => {
            let rgb = colour(&annotation.color, [1.0, 1.0, 0.0]);
            let icon = [(left, top - 20.0), (left + 20.0, top - 20.0), (left + 20.0, top), (left, top)];
            layers.push(json!({ "id": format!("{}.note", item.object.id), "segments": polygon(&icon, true), "opacity": opacity, "fill": { "color": [rgb[0], rgb[1], rgb[2], 1.0] }, "stroke": { "color": [0.0, 0.0, 0.0, 1.0], "width": 1.0 } }));
            for (index, offset) in [6.0, 10.0, 14.0].into_iter().enumerate() {
                layers.push(json!({ "id": format!("{}.line{index}", item.object.id), "segments": polygon(&[(left + 4.0, top - offset), (left + 16.0, top - offset)], false), "stroke": { "color": [0.0, 0.0, 0.0, 1.0], "width": 1.0 } }));
            }
        }
        PdfAnnotationKind::FreeText { default_appearance, .. } => {
            let tokens: Vec<&str> = default_appearance.split_whitespace().collect();
            let size = tokens.iter().position(|token| *token == "Tf").and_then(|at| at.checked_sub(1)).and_then(|at| tokens[at].parse::<f64>().ok()).filter(|size| *size > 0.0).unwrap_or(12.0);
            let content = annotation.contents.clone().unwrap_or_default();
            let (x, y) = display_point(page, item.top, left, top);
            if width > 0.0 { layers.push(stroke(polygon(&[(left, bottom), (right, bottom), (right, top), (left, top)], true), stroke_colour, "frame")); }
            layers.push(json!({ "id": format!("{}.text", item.object.id), "kind": "text", "x": x, "y": y, "width": right - left, "height": top - bottom, "text": { "content": content, "size": size, "fontFamily": "Anta" }, "fill": { "color": [stroke_colour[0], stroke_colour[1], stroke_colour[2], 1.0] } }));
        }
        PdfAnnotationKind::Stamp { icon } => {
            let red = if annotation.color.is_empty() { [0.8, 0.0, 0.0] } else { stroke_colour };
            let (x, y) = display_point(page, item.top, left + 4.0, top - 4.0);
            layers.push(json!({ "id": format!("{}.frame", item.object.id), "segments": polygon(&[(left, bottom), (right, bottom), (right, top), (left, top)], true), "opacity": opacity, "stroke": { "color": [red[0], red[1], red[2], 1.0], "width": 2.0 } }));
            layers.push(json!({ "id": format!("{}.text", item.object.id), "kind": "text", "x": x, "y": y, "width": right - left, "height": top - bottom, "text": { "content": icon.clone().unwrap_or_else(|| "Draft".into()), "size": ((top - bottom) / 3.0).clamp(8.0, 36.0), "fontFamily": "Anta" }, "fill": { "color": [red[0], red[1], red[2], 1.0] } }));
        }
        _ => return false,
    }
    true
}
