//! 🧭️ PDF page editor geometry: page object discovery, placement, display coordinates, vector segments and matrix algebra.

use super::*;

//#region 🧭️Geometry
pub(super) fn page_objects(snapshot: &PdfSnapshot, page_index: usize, page: &PdfPage) -> Vec<PageObject> {
    let mut found = Vec::new();
    let mut stack = vec![Graphics::default()];
    let mut path_start = None;
    let mut path_points = Vec::new();
    for (index, op) in page.content.iter().enumerate() {
        match op {
            PdfOp::Save => {
                let state = stack.last().expect("graphics stack").clone();
                stack.push(state);
            }
            PdfOp::Restore => {
                if stack.len() > 1 {
                    stack.pop();
                }
            }
            PdfOp::Transform { matrix } => {
                let state = stack.last_mut().expect("graphics stack");
                state.parent_ctm = state.ctm;
                state.ctm = multiply(*matrix, state.ctm);
                state.transform_op = Some(index);
            }
            PdfOp::SetFillRgb { r, g, b } => stack.last_mut().expect("graphics").fill = [*r, *g, *b],
            PdfOp::SetStrokeRgb { r, g, b } => stack.last_mut().expect("graphics").stroke = [*r, *g, *b],
            PdfOp::SetFillGray { gray } => stack.last_mut().expect("graphics").fill = [*gray, *gray, *gray],
            PdfOp::SetStrokeGray { gray } => stack.last_mut().expect("graphics").stroke = [*gray, *gray, *gray],
            PdfOp::SetLineWidth { width } => stack.last_mut().expect("graphics").line_width = *width,
            PdfOp::SetFont { name, size } => {
                let state = stack.last_mut().expect("graphics");
                state.font = name.clone();
                state.font_size = *size;
            }
            PdfOp::SetLeading { leading } => stack.last_mut().expect("graphics").leading = *leading,
            PdfOp::SetCharSpacing { spacing } => stack.last_mut().expect("graphics").char_spacing = *spacing,
            PdfOp::SetWordSpacing { spacing } => stack.last_mut().expect("graphics").word_spacing = *spacing,
            PdfOp::SetHorizontalScale { scale } => stack.last_mut().expect("graphics").horizontal_scale = *scale,
            PdfOp::BeginText => {
                let state = stack.last_mut().expect("graphics");
                state.text_matrix = PDF_IDENTITY_MATRIX;
                state.line_matrix = PDF_IDENTITY_MATRIX;
                state.position_op = None;
            }
            PdfOp::SetTextMatrix { matrix } => {
                let state = stack.last_mut().expect("graphics");
                state.text_matrix = *matrix;
                state.line_matrix = *matrix;
                state.position_op = Some(index);
            }
            PdfOp::MoveText { tx, ty } => move_text(stack.last_mut().expect("graphics"), *tx, *ty, index, false),
            PdfOp::MoveTextSetLeading { tx, ty } => move_text(stack.last_mut().expect("graphics"), *tx, *ty, index, true),
            PdfOp::NextLine => {
                let leading = stack.last().expect("graphics").leading;
                move_text(stack.last_mut().expect("graphics"), 0.0, -leading, index, false);
            }
            PdfOp::MoveTo { x, y } => {
                if path_start.is_none() {
                    path_start = Some(index);
                    path_points.clear();
                }
                path_points.push(apply(stack.last().expect("graphics").ctm, *x, *y));
            }
            PdfOp::LineTo { x, y } | PdfOp::CurveTo { x3: x, y3: y, .. } | PdfOp::CurveToInitial { x3: x, y3: y, .. } | PdfOp::CurveToFinal { x3: x, y3: y, .. } => {
                if path_start.is_none() {
                    path_start = Some(index);
                }
                path_points.push(apply(stack.last().expect("graphics").ctm, *x, *y));
            }
            PdfOp::Rectangle { x, y, width, height } => {
                if path_start.is_none() {
                    path_start = Some(index);
                    path_points.clear();
                }
                let ctm = stack.last().expect("graphics").ctm;
                path_points.extend([apply(ctm, *x, *y), apply(ctm, *x + *width, *y), apply(ctm, *x + *width, *y + *height), apply(ctm, *x, *y + *height)]);
            }
            PdfOp::Stroke | PdfOp::CloseStroke | PdfOp::Fill | PdfOp::FillEvenOdd | PdfOp::FillStroke | PdfOp::FillStrokeEvenOdd | PdfOp::CloseFillStroke | PdfOp::CloseFillStrokeEvenOdd | PdfOp::EndPath => {
                if let Some(start) = path_start.take() {
                    let state = stack.last().expect("graphics");
                    found.push(bounded_object(page_index, ObjectKind::Vector, start, index, None, None, state, &path_points, String::new(), state.font_size));
                    path_points.clear();
                }
            }
            PdfOp::ShowText { text } => found.push(text_object(page_index, index, stack.last_mut().expect("graphics"), shown(text))),
            PdfOp::ShowTextArray { items } => {
                let text = items.iter().filter_map(|item| match item { PdfTextArrayItem::Text { text } => Some(text.as_str()), _ => None }).collect::<String>();
                found.push(text_object(page_index, index, stack.last_mut().expect("graphics"), text));
            }
            PdfOp::NextLineShowText { text } => {
                let leading = stack.last().expect("graphics").leading;
                move_text(stack.last_mut().expect("graphics"), 0.0, -leading, index, false);
                found.push(text_object(page_index, index, stack.last_mut().expect("graphics"), shown(text)));
            }
            PdfOp::NextLineShowTextSpaced { word_spacing, char_spacing, text } => {
                let state = stack.last_mut().expect("graphics");
                state.word_spacing = *word_spacing;
                state.char_spacing = *char_spacing;
                let leading = state.leading;
                move_text(state, 0.0, -leading, index, false);
                found.push(text_object(page_index, index, stack.last_mut().expect("graphics"), shown(text)));
            }
            PdfOp::PaintXObject { name } => {
                let state = stack.last().expect("graphics");
                let (kind, label) = if snapshot.images.iter().any(|image| image.id == *name) {
                    (ObjectKind::Image, name.clone())
                } else if snapshot.forms.iter().any(|form| form.id == *name) {
                    (ObjectKind::Form, name.clone())
                } else {
                    (ObjectKind::Form, name.clone())
                };
                let (start, end, transform_op) = paint_span(&page.content, index);
                let corners = unit_square(state.ctm);
                found.push(bounded_object(page_index, kind, start, end, None, transform_op, state, &corners, label, 0.0));
            }
            PdfOp::InlineImage { image } => {
                let state = stack.last().expect("graphics");
                let corners = unit_square(state.ctm);
                found.push(bounded_object(page_index, ObjectKind::Image, index, index, None, state.transform_op, state, &corners, format!("{}×{}", image.width, image.height), 0.0));
            }
            PdfOp::PaintShading { name } => {
                let state = stack.last().expect("graphics");
                let corners = shading_corners(snapshot, name, page, state.ctm);
                found.push(bounded_object(page_index, ObjectKind::Shading, index, index, None, None, state, &corners, name.clone(), 0.0));
            }
            _ => {}
        }
    }
    for (index, annotation) in page.annotations.iter().enumerate() {
        let state = Graphics::default();
        let corners = [(annotation.rect[0], annotation.rect[1]), (annotation.rect[2], annotation.rect[3])];
        let mut object = bounded_object(page_index, ObjectKind::Annotation, index, index, None, None, &state, &corners, annotation.contents.clone().unwrap_or_default(), 12.0);
        object.id = format!("p{page_index}:a{index}");
        found.push(object);
    }
    found
}

fn text_object(page_index: usize, index: usize, state: &mut Graphics, text: String) -> PageObject {
    let origin = apply(multiply(state.text_matrix, state.ctm), 0.0, 0.0);
    let width = text_width(&text, state);
    let advance = width;
    state.text_matrix[4] += state.text_matrix[0] * advance;
    state.text_matrix[5] += state.text_matrix[1] * advance;
    let corners = [origin, (origin.0 + width, origin.1), (origin.0 + width, origin.1 + state.font_size), (origin.0, origin.1 + state.font_size)];
    bounded_object(page_index, ObjectKind::Text, index, index, state.position_op, None, state, &corners, text, state.font_size)
}

fn bounded_object(page_index: usize, kind: ObjectKind, start: usize, end: usize, position_op: Option<usize>, transform_op: Option<usize>, state: &Graphics, points: &[(f64, f64)], text: String, font_size: f64) -> PageObject {
    let (x, y, width, height) = bounds(points);
    PageObject {
        id: format!("p{page_index}:{}{end}", kind.letter()),
        page: page_index,
        kind,
        start,
        end,
        position_op,
        transform_op,
        ctm: state.ctm,
        parent_ctm: state.parent_ctm,
        x,
        y,
        width,
        height,
        text,
        font: state.font.clone(),
        font_size,
        fill: state.fill,
        stroke: state.stroke,
        line_width: state.line_width,
    }
}

fn shading_corners(snapshot: &PdfSnapshot, name: &str, page: &PdfPage, ctm: PdfMatrix) -> Vec<(f64, f64)> {
    let media = vec![(page.media_box[0], page.media_box[1]), (page.media_box[2], page.media_box[3])];
    let Some(shading) = snapshot.shadings.iter().find(|item| item.id == name) else { return media };
    if let Some(bbox) = shading.bbox {
        return vec![apply(ctm, bbox[0], bbox[1]), apply(ctm, bbox[2], bbox[3])];
    }
    match &shading.kind {
        PdfShadingKind::Axial { coords, .. } => vec![apply(ctm, coords[0], coords[1]), apply(ctm, coords[2], coords[3])],
        PdfShadingKind::Radial { coords, .. } => vec![apply(ctm, coords[0] - coords[2], coords[1] - coords[2]), apply(ctm, coords[3] + coords[5], coords[4] + coords[5])],
        PdfShadingKind::FunctionBased { domain, matrix, .. } => {
            let domain = domain.unwrap_or([0.0, 1.0, 0.0, 1.0]);
            let placed = multiply(ctm, matrix.unwrap_or(PDF_IDENTITY_MATRIX));
            vec![apply(placed, domain[0], domain[2]), apply(placed, domain[1], domain[3])]
        }
        PdfShadingKind::Mesh { decode, .. } if decode.len() >= 4 => vec![apply(ctm, decode[0], decode[2]), apply(ctm, decode[1], decode[3])],
        PdfShadingKind::Mesh { .. } => media,
    }
}

fn paint_span(ops: &[PdfOp], paint: usize) -> (usize, usize, Option<usize>) {
    let transform_op = paint.checked_sub(1).filter(|index| matches!(ops.get(*index), Some(PdfOp::Transform { .. })));
    let mut start = transform_op.unwrap_or(paint);
    let mut end = paint;
    if let Some(transform) = transform_op {
        if transform > 0 && matches!(ops.get(transform - 1), Some(PdfOp::Save)) && matches!(ops.get(paint + 1), Some(PdfOp::Restore)) {
            start = transform - 1;
            end = paint + 1;
        }
    }
    (start, end, transform_op)
}

fn move_text(state: &mut Graphics, tx: f64, ty: f64, index: usize, set_leading: bool) {
    if set_leading {
        state.leading = -ty;
    }
    let matrix = state.line_matrix;
    state.line_matrix[4] = matrix[0] * tx + matrix[2] * ty + matrix[4];
    state.line_matrix[5] = matrix[1] * tx + matrix[3] * ty + matrix[5];
    state.text_matrix = state.line_matrix;
    state.position_op = Some(index);
}

struct Placed {
    object: PageObject,
    top: f64,
    canvas_x: f64,
    canvas_y: f64,
    canvas_w: f64,
    canvas_h: f64,
}

pub(super) fn placement(snapshot: &PdfSnapshot) -> Vec<Placed> {
    let mut tops = Vec::new();
    let mut top = 0.0;
    for page in &snapshot.pages {
        tops.push(top);
        top += displayed_size(page).1 + 16.0;
    }
    objects(snapshot)
        .into_iter()
        .map(|object| {
            let page = &snapshot.pages[object.page];
            let page_top = tops[object.page];
            let (canvas_x, canvas_y, canvas_w, canvas_h) = display_rect(page, page_top, object.x, object.y, object.width, object.height);
            Placed { object, top: page_top, canvas_x, canvas_y, canvas_w, canvas_h }
        })
        .collect()
}

fn display_rect(page: &PdfPage, top: f64, x: f64, y: f64, width: f64, height: f64) -> (f64, f64, f64, f64) {
    let corners = [display_point(page, top, x, y), display_point(page, top, x + width, y), display_point(page, top, x + width, y + height), display_point(page, top, x, y + height)];
    let min_x = corners.iter().map(|point| point.0).fold(f64::INFINITY, f64::min);
    let min_y = corners.iter().map(|point| point.1).fold(f64::INFINITY, f64::min);
    let max_x = corners.iter().map(|point| point.0).fold(f64::NEG_INFINITY, f64::max);
    let max_y = corners.iter().map(|point| point.1).fold(f64::NEG_INFINITY, f64::max);
    (min_x, min_y, (max_x - min_x).max(0.0), (max_y - min_y).max(0.0))
}

fn display_point(page: &PdfPage, top: f64, x: f64, y: f64) -> (f64, f64) {
    let left = page.media_box[0];
    let bottom = page.media_box[1];
    let width = page.width();
    let height = page.height();
    let x = x - left;
    let y = y - bottom;
    let (dx, dy) = match page.rotate.rem_euclid(360) {
        90 => (y, x),
        180 => (width - x, y),
        270 => (height - y, width - x),
        _ => (x, height - y),
    };
    (dx, top + dy)
}

pub(super) fn displayed_size(page: &PdfPage) -> (f64, f64) {
    match page.rotate.rem_euclid(360) {
        90 | 270 => (page.height(), page.width()),
        _ => (page.width(), page.height()),
    }
}

pub(super) fn vector_segments(page: &PdfPage, top: f64, ops: &[PdfOp]) -> Value {
    let mut segments = Vec::new();
    let mut current = (0.0, 0.0);
    let mut start = (0.0, 0.0);
    let push = |segments: &mut Vec<Value>, kind: &str, point: Option<(f64, f64)>| {
        if let Some((x, y)) = point {
            let shown = display_point(page, top, x, y);
            segments.push(json!({ "kind": kind, "to": [shown.0, shown.1] }));
        } else {
            segments.push(json!({ "kind": kind }));
        }
    };
    for op in ops {
        match op {
            PdfOp::MoveTo { x, y } => {
                current = (*x, *y);
                start = current;
                push(&mut segments, "move", Some(current));
            }
            PdfOp::LineTo { x, y } => {
                current = (*x, *y);
                push(&mut segments, "line", Some(current));
            }
            PdfOp::Rectangle { x, y, width, height } => {
                push(&mut segments, "move", Some((*x, *y)));
                push(&mut segments, "line", Some((*x + *width, *y)));
                push(&mut segments, "line", Some((*x + *width, *y + *height)));
                push(&mut segments, "line", Some((*x, *y + *height)));
                push(&mut segments, "close", None);
                current = (*x, *y);
            }
            PdfOp::CurveTo { x1, y1, x2, y2, x3, y3 } => {
                for step in 1..=8 {
                    let point = cubic(current, (*x1, *y1), (*x2, *y2), (*x3, *y3), step as f64 / 8.0);
                    push(&mut segments, "line", Some(point));
                }
                current = (*x3, *y3);
            }
            PdfOp::ClosePath | PdfOp::CloseStroke | PdfOp::CloseFillStroke | PdfOp::CloseFillStrokeEvenOdd => {
                push(&mut segments, "close", None);
                current = start;
            }
            _ => {}
        }
    }
    Value::Array(segments)
}

pub(super) fn rect_segments(x: f64, y: f64, width: f64, height: f64) -> Value {
    json!([{ "kind": "move", "to": [x, y] }, { "kind": "line", "to": [x + width, y] }, { "kind": "line", "to": [x + width, y + height] }, { "kind": "line", "to": [x, y + height] }, { "kind": "close" }])
}

fn cubic(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), p3: (f64, f64), t: f64) -> (f64, f64) {
    let u = 1.0 - t;
    let a = u * u * u;
    let b = 3.0 * u * u * t;
    let c = 3.0 * u * t * t;
    let d = t * t * t;
    (a * p0.0 + b * p1.0 + c * p2.0 + d * p3.0, a * p0.1 + b * p1.1 + c * p2.1 + d * p3.1)
}

pub(super) fn shown(text: &PdfTextString) -> String {
    match text {
        PdfTextString::Text { text } => text.clone(),
        PdfTextString::Codes { codes } => codes.iter().map(|code|char::from_u32(*code).unwrap_or('\u{FFFD}')).collect(),
    }
}

fn text_width(text: &str, state: &Graphics) -> f64 {
    let chars = text.chars().count() as f64;
    let spaces = text.chars().filter(|char| *char == ' ').count() as f64;
    (chars * state.font_size * 0.5 + chars * state.char_spacing + spaces * state.word_spacing) * state.horizontal_scale / 100.0
}

fn unit_square(ctm: PdfMatrix) -> [(f64, f64); 4] {
    [apply(ctm, 0.0, 0.0), apply(ctm, 1.0, 0.0), apply(ctm, 1.0, 1.0), apply(ctm, 0.0, 1.0)]
}

fn bounds(points: &[(f64, f64)]) -> (f64, f64, f64, f64) {
    if points.is_empty() {
        return (0.0, 0.0, 0.0, 0.0);
    }
    let min_x = points.iter().map(|point| point.0).fold(f64::INFINITY, f64::min);
    let min_y = points.iter().map(|point| point.1).fold(f64::INFINITY, f64::min);
    let max_x = points.iter().map(|point| point.0).fold(f64::NEG_INFINITY, f64::max);
    let max_y = points.iter().map(|point| point.1).fold(f64::NEG_INFINITY, f64::max);
    (min_x, min_y, (max_x - min_x).max(0.0), (max_y - min_y).max(0.0))
}

pub(super) fn user_delta(ctm: PdfMatrix, dx: f64, dy: f64) -> (f64, f64) {
    let inverse = invert(ctm).unwrap_or(PDF_IDENTITY_MATRIX);
    (inverse[0] * dx + inverse[2] * dy, inverse[1] * dx + inverse[3] * dy)
}

pub(super) fn unapply(matrix: PdfMatrix, x: f64, y: f64) -> (f64, f64) {
    apply(invert(matrix).unwrap_or(PDF_IDENTITY_MATRIX), x, y)
}

pub(super) fn apply(matrix: PdfMatrix, x: f64, y: f64) -> (f64, f64) {
    (matrix[0] * x + matrix[2] * y + matrix[4], matrix[1] * x + matrix[3] * y + matrix[5])
}

fn multiply(left: PdfMatrix, right: PdfMatrix) -> PdfMatrix {
    [
        left[0] * right[0] + left[2] * right[1],
        left[1] * right[0] + left[3] * right[1],
        left[0] * right[2] + left[2] * right[3],
        left[1] * right[2] + left[3] * right[3],
        left[0] * right[4] + left[2] * right[5] + left[4],
        left[1] * right[4] + left[3] * right[5] + left[5],
    ]
}

pub(super) fn invert(matrix: PdfMatrix) -> Option<PdfMatrix> {
    let det = matrix[0] * matrix[3] - matrix[1] * matrix[2];
    if det.abs() < 1.0e-12 {
        return None;
    }
    let inv = 1.0 / det;
    Some([matrix[3] * inv, -matrix[1] * inv, -matrix[2] * inv, matrix[0] * inv, (matrix[2] * matrix[5] - matrix[3] * matrix[4]) * inv, (matrix[1] * matrix[4] - matrix[0] * matrix[5]) * inv])
}
//#endregion 🧭️Geometry
