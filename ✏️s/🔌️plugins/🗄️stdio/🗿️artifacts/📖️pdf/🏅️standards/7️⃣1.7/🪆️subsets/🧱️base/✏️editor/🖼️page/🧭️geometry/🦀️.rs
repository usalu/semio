//! 🧭️ PDF page editor geometry: page object discovery, placement, display coordinates, vector segments and matrix algebra.

use super::*;

//#region 🧭️Geometry
pub fn page_objects(snapshot: &PdfSnapshot, page_index: usize, page: &PdfPage, metrics: &mut FontMeasurements<'_>) -> Vec<PageObject> {
    let mut found = content_objects(snapshot, page_index, page, &page.content, Graphics::default(), metrics);
    for (index, annotation) in page.annotations.iter().enumerate() {
        let state = Graphics::default();
        let corners = [(annotation.rect[0], annotation.rect[1]), (annotation.rect[2], annotation.rect[3])];
        let mut object = bounded_object(page_index, ObjectKind::Annotation, index, index, &state, &corners, annotation.contents.clone().unwrap_or_default(), 12.0);
        object.id = format!("p{page_index}:a{index}");
        found.push(object);
    }
    found
}

pub fn find_object(snapshot: &PdfSnapshot, address: &str) -> Option<PageObject> {
    let (page, _) = address.strip_prefix('p')?.split_once(':')?;
    let index = page.parse::<usize>().ok()?;
    page_objects(snapshot, index, snapshot.pages.get(index)?, &mut FontMeasurements::new(snapshot)).into_iter().find(|object| object.id == address)
}

pub fn form_objects(snapshot: &PdfSnapshot, form: &PdfFormXObject, placement: &PageObject, ctm: PdfMatrix, metrics: &mut FontMeasurements<'_>) -> Vec<PageObject> {
    let mut state = placement.form_graphics.clone().unwrap_or_default();
    state.ctm = multiply(ctm, form.matrix);
    content_objects(snapshot, placement.page, &snapshot.pages[placement.page], &form.content, state, metrics)
}

pub fn content_objects(snapshot: &PdfSnapshot, page_index: usize, page: &PdfPage, content: &[PdfOp], initial: Graphics, metrics: &mut FontMeasurements<'_>) -> Vec<PageObject> {
    let mut found = Vec::new();
    let mut stack = vec![initial];
    let mut pending_clip = None;
    let mut text_clip = None::<Vec<VectorSegment>>;
    let mut text_clip_budget = 50_000;
    let mut path_start = None;
    let mut path = Vec::new();
    let mut current = (0.0, 0.0);
    let mut subpath_start = current;
    for (index, op) in content.iter().enumerate() {
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
                state.ctm = multiply(state.ctm, *matrix);
            }
            PdfOp::SetFillRgb { r, g, b } => stack.last_mut().expect("graphics").fill = [*r, *g, *b],
            PdfOp::SetStrokeRgb { r, g, b } => stack.last_mut().expect("graphics").stroke = [*r, *g, *b],
            PdfOp::SetFillGray { gray } => stack.last_mut().expect("graphics").fill = [*gray, *gray, *gray],
            PdfOp::SetStrokeGray { gray } => stack.last_mut().expect("graphics").stroke = [*gray, *gray, *gray],
            PdfOp::SetExtGState { name } => {
                if let Some(settings) = snapshot.ext_g_state(name) {
                    let state = stack.last_mut().expect("graphics");
                    if let Some(width) = settings.line_width { state.line_width = width; }
                    if let Some(cap) = &settings.line_cap { state.vector_stroke.cap = canvas_line_cap(cap); }
                    if let Some(join) = &settings.line_join { state.vector_stroke.join = canvas_line_join(join); }
                    if let Some(limit) = settings.miter_limit { state.vector_stroke.set_miter_limit(limit); }
                    if let Some((dash, phase)) = &settings.dash { state.vector_stroke.set_dash(dash, *phase); }
                }
            }
            PdfOp::SetLineWidth { width } => stack.last_mut().expect("graphics").line_width = *width,
            PdfOp::SetLineCap { cap } => stack.last_mut().expect("graphics").vector_stroke.cap = canvas_line_cap(cap),
            PdfOp::SetLineJoin { join } => stack.last_mut().expect("graphics").vector_stroke.join = canvas_line_join(join),
            PdfOp::SetMiterLimit { limit } => stack.last_mut().expect("graphics").vector_stroke.set_miter_limit(*limit),
            PdfOp::SetDash { array, phase } => stack.last_mut().expect("graphics").vector_stroke.set_dash(array, *phase),
            PdfOp::SetFont { name, size } => {
                let state = stack.last_mut().expect("graphics");
                state.font = name.clone();
                state.font_size = *size;
            }
            PdfOp::SetLeading { leading } => stack.last_mut().expect("graphics").leading = *leading,
            PdfOp::SetTextRenderingMode { mode } if *mode <= 7 => stack.last_mut().expect("graphics").text_render_mode = *mode,
            PdfOp::SetTextRise { rise } => stack.last_mut().expect("graphics").text_rise = *rise,
            PdfOp::SetCharSpacing { spacing } => stack.last_mut().expect("graphics").char_spacing = *spacing,
            PdfOp::SetWordSpacing { spacing } => stack.last_mut().expect("graphics").word_spacing = *spacing,
            PdfOp::SetHorizontalScale { scale } => stack.last_mut().expect("graphics").horizontal_scale = *scale,
            PdfOp::BeginText => {
                text_clip = Some(Vec::new());
                let state = stack.last_mut().expect("graphics");
                state.text_matrix = PDF_IDENTITY_MATRIX;
                state.line_matrix = PDF_IDENTITY_MATRIX;
            }
            PdfOp::EndText => {
                if let Some(path) = text_clip.take().filter(|path| !path.is_empty()) {
                    let state = stack.last_mut().expect("graphics");
                    std::sync::Arc::make_mut(&mut state.clip_paths).push(PathClip { path: path.into(), even_odd: false });
                }
            }
            PdfOp::SetTextMatrix { matrix } => {
                let state = stack.last_mut().expect("graphics");
                state.text_matrix = *matrix;
                state.line_matrix = *matrix;
            }
            PdfOp::MoveText { tx, ty } => move_text(stack.last_mut().expect("graphics"), *tx, *ty, false),
            PdfOp::MoveTextSetLeading { tx, ty } => move_text(stack.last_mut().expect("graphics"), *tx, *ty, true),
            PdfOp::NextLine => {
                let leading = stack.last().expect("graphics").leading;
                move_text(stack.last_mut().expect("graphics"), 0.0, -leading, false);
            }
            PdfOp::MoveTo { .. } | PdfOp::LineTo { .. } | PdfOp::CurveTo { .. } | PdfOp::CurveToInitial { .. } | PdfOp::CurveToFinal { .. } | PdfOp::Rectangle { .. } | PdfOp::ClosePath => {
                path_start.get_or_insert(index);
                let ctm = stack.last().expect("graphics").ctm;
                append_path_segment(&mut path, &mut current, &mut subpath_start, ctm, op);
            }
            PdfOp::Clip | PdfOp::ClipEvenOdd => pending_clip = Some(matches!(op, PdfOp::ClipEvenOdd)),
            PdfOp::Stroke | PdfOp::CloseStroke | PdfOp::Fill | PdfOp::FillEvenOdd | PdfOp::FillStroke | PdfOp::FillStrokeEvenOdd | PdfOp::CloseFillStroke | PdfOp::CloseFillStrokeEvenOdd => {
                if let Some(start) = path_start.take() {
                    if matches!(op, PdfOp::CloseStroke | PdfOp::CloseFillStroke | PdfOp::CloseFillStrokeEvenOdd) {
                        path.push(VectorSegment::Close);
                    }
                    let state = stack.last().expect("graphics");
                    let points = vector_bound_points(&path);
                    let mut object = bounded_object(page_index, ObjectKind::Vector, start, index, state, &points, String::new(), state.font_size);
                    push_clip(&mut stack, &path, &mut pending_clip);
                    object.path = std::mem::take(&mut path);
                    found.push(object);
                }
            }
            PdfOp::EndPath => {
                push_clip(&mut stack, &path, &mut pending_clip);
                path_start = None;
                path.clear();
            }
            PdfOp::ShowText { text } => found.push(show_text(metrics, page_index, index, stack.last_mut().expect("graphics"), text)),
            PdfOp::ShowTextArray { items } => {
                let state = stack.last_mut().expect("graphics");
                let mut text = String::new();
                let matrix = state.text_matrix;
                let mut advance = 0.0;
                let mut left = 0.0f64;
                let mut right = 0.0f64;
                let mut has_glyphs = false;
                for item in items {
                    let (fragment, width) = match item {
                        PdfTextArrayItem::Adjust { amount } => {
                            let distance = -amount * state.font_size * state.horizontal_scale / 100_000.0;
                            if has_glyphs { advance += distance; } else { advance_text(state, distance); }
                            continue;
                        }
                        PdfTextArrayItem::Text { text } => measure_fragment(metrics, state, Some(text), &[]),
                        PdfTextArrayItem::Codes { codes } => measure_fragment(metrics, state, None, codes),
                    };
                    if fragment.is_empty() && width == 0.0 { continue; }
                    has_glyphs = true;
                    left = left.min(advance).min(advance + width);
                    right = right.max(advance).max(advance + width);
                    advance += width;
                    text.push_str(&fragment);
                }
                if has_glyphs {
                    let mut object = text_object(page_index, index, state, text, advance, [left, right]);
                    if let Some(placement) = &mut object.text_placement { placement.matrix = matrix; }
                    found.push(object);
                }
            }
            PdfOp::NextLineShowText { text } => {
                let leading = stack.last().expect("graphics").leading;
                move_text(stack.last_mut().expect("graphics"), 0.0, -leading, false);
                found.push(show_text(metrics, page_index, index, stack.last_mut().expect("graphics"), text));
            }
            PdfOp::NextLineShowTextSpaced { word_spacing, char_spacing, text } => {
                let state = stack.last_mut().expect("graphics");
                state.word_spacing = *word_spacing;
                state.char_spacing = *char_spacing;
                let leading = state.leading;
                move_text(state, 0.0, -leading, false);
                found.push(show_text(metrics, page_index, index, stack.last_mut().expect("graphics"), text));
            }
            PdfOp::PaintXObject { name } => {
                let state = stack.last().expect("graphics");
                let is_image = snapshot.image(name).is_some();
                let kind = if is_image { ObjectKind::Image } else { ObjectKind::Form };
                let corners = match snapshot.form(name).filter(|_| !is_image) {
                    Some(form) => form_corners(form, state.ctm),
                    None => unit_square(state.ctm),
                };
                let mut object = bounded_object(page_index, kind, index, index, state, &corners, name.clone(), 0.0);
                if kind == ObjectKind::Form {
                    object.form_graphics = Some(state.clone());
                }
                found.push(object);
            }
            PdfOp::InlineImage { image } => {
                let state = stack.last().expect("graphics");
                let corners = unit_square(state.ctm);
                found.push(bounded_object(page_index, ObjectKind::Image, index, index, state, &corners, format!("{}×{}", image.width, image.height), 0.0));
            }
            PdfOp::PaintShading { name } => {
                let state = stack.last().expect("graphics");
                let corners = shading_corners(snapshot, name, page, state.ctm);
                found.push(bounded_object(page_index, ObjectKind::Shading, index, index, state, &corners, name.clone(), 0.0));
            }
            _ => {}
        }
        let Some(path) = &mut text_clip else { continue; };
        let Some(object) = found.last().filter(|object| object.start == index && object.kind == ObjectKind::Text) else { continue; };
        if !object.text_placement.as_ref().is_some_and(|position| position.render_mode & 4 != 0)
            || snapshot.font(&object.font).is_some_and(|font| matches!(&font.kind, PdfFontKind::Type3 { .. }))
        {
            continue;
        }
        let map = |x, y| apply(object.text_transform, x, y);
        match super::scene::truetype_text::truetype_outline(object, Some(op), metrics, &mut text_clip_budget, map) {
            Some(segments) => path.extend(segments),
            None => text_clip = None,
        }
    }
    let mut counts = std::collections::HashMap::new();
    for object in &mut found {
        let kind = object.kind.letter();
        let ordinal = counts.entry(kind).or_insert(0usize);
        object.id = format!("p{page_index}:{kind}{ordinal}");
        *ordinal += 1;
    }
    found
}

fn push_clip(stack: &mut [Graphics], path: &[VectorSegment], pending: &mut Option<bool>) {
    let Some(even_odd) = pending.take() else { return; };
    let state = stack.last_mut().expect("graphics stack");
    if let Some(quad) = semio_s_artifact_stdio_pdf_page_coordinates::clip_quad(path) {
        std::sync::Arc::make_mut(&mut state.clip_quads).push(quad);
    } else {
        std::sync::Arc::make_mut(&mut state.clip_paths).push(PathClip { path: path.into(), even_odd });
    }
}

fn advance_text(state: &mut Graphics, distance: f64) {
    state.text_matrix[4] += state.text_matrix[0] * distance;
    state.text_matrix[5] += state.text_matrix[1] * distance;
}

fn show_text(metrics: &mut FontMeasurements<'_>, page: usize, index: usize, state: &mut Graphics, text: &PdfTextString) -> PageObject {
    let (text, width) = match text {
        PdfTextString::Text { text } => measure_fragment(metrics, state, Some(text), &[]),
        PdfTextString::Codes { codes } => measure_fragment(metrics, state, None, codes),
    };
    text_object(page, index, state, text, width, [width.min(0.0), width.max(0.0)])
}

fn measure_fragment(metrics: &mut FontMeasurements<'_>, state: &Graphics, text: Option<&str>, codes: &[u32]) -> (String, f64) {
    if let Some((text, width, glyphs, spaces)) = metrics.measure(&state.font, text, codes) {
        let advance = (width * state.font_size + glyphs as f64 * state.char_spacing + spaces as f64 * state.word_spacing) * state.horizontal_scale / 100.0;
        return (text, advance);
    }
    let text = text.map(str::to_owned).unwrap_or_else(|| codes.iter().map(|&code| char::from_u32(code).unwrap_or(char::REPLACEMENT_CHARACTER)).collect());
    let glyphs = text.chars().count() as f64;
    let spaces = text.chars().filter(|&character| character == ' ').count() as f64;
    let advance = (glyphs * 0.5 * state.font_size + glyphs * state.char_spacing + spaces * state.word_spacing) * state.horizontal_scale / 100.0;
    (text, advance)
}

fn text_object(page_index: usize, index: usize, state: &mut Graphics, text: String, width: f64, [left, right]: [f64; 2]) -> PageObject {
    let matrix = state.text_matrix;
    let baseline = multiply(matrix, [1.0, 0.0, 0.0, 1.0, 0.0, state.text_rise]);
    let text_to_page = multiply(state.ctm, baseline);
    advance_text(state, width);
    let corners = [
        apply(text_to_page, left, 0.0),
        apply(text_to_page, right, 0.0),
        apply(text_to_page, right, state.font_size),
        apply(text_to_page, left, state.font_size),
    ];
    let mut object = bounded_object(page_index, ObjectKind::Text, index, index, state, &corners, text, state.font_size);
    object.text_placement = Some(Box::new(TextPlacement {
        matrix, line: state.line_matrix, next_origin: (state.text_matrix[4], state.text_matrix[5]),
        rise: state.text_rise, render_mode: state.text_render_mode, horizontal_scale: state.horizontal_scale,
        char_spacing: state.char_spacing, word_spacing: state.word_spacing,
    }));
    object.text_transform = multiply(text_to_page, [state.horizontal_scale / 100.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
    object
}

fn canvas_line_cap(cap: &PdfLineCap) -> &'static str {
    match cap { PdfLineCap::Butt => "butt", PdfLineCap::Round => "round", PdfLineCap::Square => "square" }
}

fn canvas_line_join(join: &PdfLineJoin) -> &'static str {
    match join { PdfLineJoin::Miter => "miter", PdfLineJoin::Round => "round", PdfLineJoin::Bevel => "bevel" }
}

fn bounded_object(page_index: usize, kind: ObjectKind, start: usize, end: usize, state: &Graphics, points: &[(f64, f64)], text: String, font_size: f64) -> PageObject {
    let (x, y, width, height) = bounds(points);
    PageObject {
        id: String::new(),
        page: page_index,
        kind,
        start,
        end,
        ctm: state.ctm,
        x,
        y,
        width,
        height,
        text,
        font: state.font.clone(),
        font_size,
        text_placement: None,
        fill: state.fill,
        stroke: state.stroke,
        line_width: state.line_width,
        vector_stroke: state.vector_stroke.clone(),
        path: Vec::new(),
        clip_quads: state.clip_quads.clone(),
        clip_paths: state.clip_paths.clone(),
        text_transform: PDF_IDENTITY_MATRIX,
        form_graphics: None,
    }
}

fn shading_corners(snapshot: &PdfSnapshot, name: &str, page: &PdfPage, ctm: PdfMatrix) -> Vec<(f64, f64)> {
    let media = vec![(page.media_box[0], page.media_box[1]), (page.media_box[2], page.media_box[3])];
    let Some(shading) = snapshot.shadings.iter().find(|item| item.id == name) else { return media };
    if let Some(bbox) = shading.bbox {
        return box_corners(ctm, bbox).to_vec();
    }
    match &shading.kind {
        PdfShadingKind::Axial { coords, .. } => vec![apply(ctm, coords[0], coords[1]), apply(ctm, coords[2], coords[3])],
        PdfShadingKind::Radial { coords, .. } => vec![apply(ctm, coords[0] - coords[2], coords[1] - coords[2]), apply(ctm, coords[0] + coords[2], coords[1] + coords[2]), apply(ctm, coords[3] - coords[5], coords[4] - coords[5]), apply(ctm, coords[3] + coords[5], coords[4] + coords[5])],
        PdfShadingKind::FunctionBased { domain, matrix, .. } => {
            let domain = domain.unwrap_or([0.0, 1.0, 0.0, 1.0]);
            let placed = multiply(ctm, matrix.unwrap_or(PDF_IDENTITY_MATRIX));
            vec![apply(placed, domain[0], domain[2]), apply(placed, domain[1], domain[3])]
        }
        PdfShadingKind::Mesh { decode, .. } if decode.len() >= 4 => vec![apply(ctm, decode[0], decode[2]), apply(ctm, decode[1], decode[3])],
        PdfShadingKind::Mesh { .. } => media,
    }
}

fn move_text(state: &mut Graphics, tx: f64, ty: f64, set_leading: bool) {
    if set_leading {
        state.leading = -ty;
    }
    let matrix = state.line_matrix;
    state.line_matrix[4] = matrix[0] * tx + matrix[2] * ty + matrix[4];
    state.line_matrix[5] = matrix[1] * tx + matrix[3] * ty + matrix[5];
    state.text_matrix = state.line_matrix;
}

pub use semio_s_artifact_stdio_pdf_page_coordinates::{apply, invert, multiply, unapply, user_delta, PageViewport};
use semio_s_artifact_stdio_pdf_page_coordinates::{bounds, box_corners, unit_square, vector_bound_points};

pub fn visible_page_box(page: &PdfPage) -> [f64; 4] {
    semio_s_artifact_stdio_pdf_page_coordinates::visible_page_box(page.media_box, page.crop_box)
}

fn viewport(page: &PdfPage, top: f64) -> PageViewport {
    PageViewport::new(page.media_box, page.crop_box, page.rotate, page.user_unit, top)
}

pub struct Placed {
    pub object: PageObject,
    pub top: f64,
    pub viewport: PageViewport,
    pub canvas_x: f64,
    pub canvas_y: f64,
    pub canvas_w: f64,
    pub canvas_h: f64,
}

impl Placed {
    pub fn new(object: PageObject, viewport: PageViewport) -> Self {
        let (canvas_x, canvas_y, canvas_w, canvas_h) = viewport.rect(object.x, object.y, object.width, object.height);
        Self { object, top: viewport.top, viewport, canvas_x, canvas_y, canvas_w, canvas_h }
    }
}

pub fn placement(snapshot: &PdfSnapshot, viewports: &[PageViewport], metrics: &mut FontMeasurements<'_>) -> Vec<Placed> {
    measured_objects(snapshot, metrics)
        .into_iter()
        .map(|object| { let viewport = viewports[object.page]; Placed::new(object, viewport) })
        .collect()
}

pub fn display_point(page: &PdfPage, top: f64, x: f64, y: f64) -> (f64, f64) {
    viewport(page, top).point(x, y)
}

fn append_path_segment(path: &mut Vec<VectorSegment>, current: &mut (f64, f64), start: &mut (f64, f64), ctm: PdfMatrix, op: &PdfOp) {
    let point = |x, y| apply(ctm, x, y);
    match *op {
        PdfOp::MoveTo { x, y } => {
            *current = point(x, y);
            *start = *current;
            path.push(VectorSegment::Move(*current));
        }
        PdfOp::LineTo { x, y } => {
            *current = point(x, y);
            path.push(VectorSegment::Line(*current));
        }
        PdfOp::Rectangle { x, y, width, height } => {
            *current = point(x, y);
            *start = *current;
            path.extend([
                VectorSegment::Move(*current),
                VectorSegment::Line(point(x + width, y)),
                VectorSegment::Line(point(x + width, y + height)),
                VectorSegment::Line(point(x, y + height)),
                VectorSegment::Close,
            ]);
        }
        PdfOp::ClosePath => {
            path.push(VectorSegment::Close);
            *current = *start;
        }
        _ => {
            let (first, second, end) = match *op {
                PdfOp::CurveTo { x1, y1, x2, y2, x3, y3 } => (point(x1, y1), point(x2, y2), point(x3, y3)),
                PdfOp::CurveToInitial { x2, y2, x3, y3 } => (*current, point(x2, y2), point(x3, y3)),
                PdfOp::CurveToFinal { x1, y1, x3, y3 } => (point(x1, y1), point(x3, y3), point(x3, y3)),
                _ => return,
            };
            path.push(VectorSegment::Cubic { first, second, end });
            *current = end;
        }
    }
}

pub fn vector_segments(page: &PdfPage, top: f64, path: &[VectorSegment]) -> Vec<Value> {
    let viewport = viewport(page, top);
    mapped_vector_segments(path, |(x, y)| viewport.point(x, y), false)
}

pub fn mapped_vector_segments(path: &[VectorSegment], display: impl Fn((f64, f64)) -> (f64, f64), open_dash_seam: bool) -> Vec<Value> {
    let mut start = (0.0, 0.0);
    let mut segments = Vec::with_capacity(path.len());
    for (index, segment) in path.iter().enumerate() {
        let segment = match *segment {
            VectorSegment::Move(to) => {
                start = to;
                json!({ "kind": "move", "to": display(to) })
            }
            VectorSegment::Line(to) => json!({ "kind": "line", "to": display(to) }),
            VectorSegment::Cubic { first, second, end } => json!({ "kind": "cubic", "ctrl1": display(first), "ctrl2": display(second), "to": display(end) }),
            VectorSegment::Close if open_dash_seam => {
                segments.push(json!({ "kind": "line", "to": display(start) }));
                if path.get(index + 1).is_none_or(|next| matches!(next, VectorSegment::Move(_))) { continue; }
                json!({ "kind": "move", "to": display(start) })
            }
            VectorSegment::Close => json!({ "kind": "close" }),
        };
        segments.push(segment);
    }
    segments
}

pub fn shown(text: &PdfTextString) -> String {
    match text {
        PdfTextString::Text { text } => text.clone(),
        PdfTextString::Codes { codes } => codes.iter().map(|code|char::from_u32(*code).unwrap_or('\u{FFFD}')).collect(),
    }
}

pub fn form_corners(form: &PdfFormXObject, ctm: PdfMatrix) -> [(f64, f64); 4] {
    box_corners(multiply(ctm, form.matrix), form.bbox)
}

//#endregion 🧭️Geometry
