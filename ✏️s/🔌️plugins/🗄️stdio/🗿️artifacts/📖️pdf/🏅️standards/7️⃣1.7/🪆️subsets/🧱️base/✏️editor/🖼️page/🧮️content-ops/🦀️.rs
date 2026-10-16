//! 🧮️ PDF page editor content operators: text replacement, translation, resizing and path shifting inside page content.

use super::*;

//#region 🧮️ContentOps
pub(super) fn ensure_font(snapshot: &PdfSnapshot) -> (String, Option<PdfMutation>) {
    if let Some(font) = snapshot.fonts.first() {
        return (font.id.clone(), None);
    }
    let font = PdfFont::standard("F1", "Helvetica");
    ("F1".into(), Some(PdfMutation::SetFont(SetFont { font, index: None })))
}

pub(super) fn replace_shown_text(content: &mut [PdfOp], index: usize, text: &str) {
    let Some(op) = content.get_mut(index) else { return };
    match op {
        PdfOp::ShowText { text: shown } | PdfOp::NextLineShowText { text: shown } | PdfOp::NextLineShowTextSpaced { text: shown, .. } => *shown = PdfTextString::text(text),
        PdfOp::ShowTextArray { items } => *items = vec![PdfTextArrayItem::Text { text: text.to_string() }],
        _ => {}
    }
}

pub(super) fn translate_object(content: &mut [PdfOp], object: &PageObject, dx: f64, dy: f64) {
    let delta = user_delta(object.ctm, dx, dy);
    match object.kind {
        ObjectKind::Text => {
            if let Some(index) = object.position_op {
                shift_position(&mut content[index], delta);
            }
        }
        ObjectKind::Image | ObjectKind::Form => {
            if let Some(index) = object.transform_op {
                if let PdfOp::Transform { matrix } = &mut content[index] {
                    let delta = user_delta(object.parent_ctm, dx, dy);
                    matrix[4] += delta.0;
                    matrix[5] += delta.1;
                }
            }
        }
        ObjectKind::Vector => shift_path(&mut content[object.start..=object.end], object.ctm, dx, dy),
        ObjectKind::Shading | ObjectKind::Annotation => {}
    }
}

pub(super) fn resize_object(content: &mut [PdfOp], object: &PageObject, x: f64, y: f64, width: f64, height: f64) {
    match object.kind {
        ObjectKind::Vector => {
            for op in &mut content[object.start..=object.end] {
                if let PdfOp::Rectangle { x: rx, y: ry, width: rw, height: rh } = op {
                    *rx = x;
                    *ry = y;
                    *rw = width;
                    *rh = height;
                }
            }
            shift_path_bounds(&mut content[object.start..=object.end], object, x, y, width, height);
        }
        ObjectKind::Image | ObjectKind::Form => {
            if let Some(index) = object.transform_op {
                if let PdfOp::Transform { matrix } = &mut content[index] {
                    matrix[0] = width;
                    matrix[3] = height;
                    matrix[4] = x;
                    matrix[5] = y;
                }
            }
        }
        ObjectKind::Text => {
            if let Some(index) = object.position_op {
                shift_position(&mut content[index], (x - object.x, y - object.y));
            }
            let size = height.max(1.0);
            let font = content.iter().rev().find_map(|op| match op {
                PdfOp::SetFont { name, .. } => Some(name.clone()),
                _ => None,
            });
            if let Some(name) = font {
                let show = object.end;
                if show <= content.len() {
                    // Font size is applied by rewriting the nearest SetFont before this show.
                    if let Some(op) = content[..show].iter_mut().rev().find(|op| matches!(op, PdfOp::SetFont { name: current, .. } if current == &name)) {
                        if let PdfOp::SetFont { size: font_size, .. } = op {
                            *font_size = size;
                        }
                    }
                }
            }
        }
        ObjectKind::Shading | ObjectKind::Annotation => {}
    }
}

pub(super) fn set_fill(content: &mut Vec<PdfOp>, object: &PageObject, red: f64, green: f64, blue: f64) {
    let found = content[..=object.end].iter_mut().rev().find(|op| matches!(op, PdfOp::SetFillRgb { .. } | PdfOp::SetFillGray { .. }));
    if let Some(op) = found {
        *op = PdfOp::SetFillRgb { r: red, g: green, b: blue };
        return;
    }
    content.insert(object.start, PdfOp::SetFillRgb { r: red, g: green, b: blue });
}

pub(super) fn set_stroke(content: &mut Vec<PdfOp>, object: &PageObject, red: f64, green: f64, blue: f64, width: f64) {
    if let Some(op) = content.get_mut(object.end) {
        match op {
            PdfOp::Fill => *op = PdfOp::FillStroke,
            PdfOp::FillEvenOdd => *op = PdfOp::FillStrokeEvenOdd,
            _ => {}
        }
    }
    content.insert(object.start, PdfOp::SetLineWidth { width: if width > 0.0 { width } else { 1.0 } });
    content.insert(object.start, PdfOp::SetStrokeRgb { r: red, g: green, b: blue });
}

fn shift_position(op: &mut PdfOp, delta: (f64, f64)) {
    match op {
        PdfOp::SetTextMatrix { matrix } => {
            matrix[4] += delta.0;
            matrix[5] += delta.1;
        }
        PdfOp::MoveText { tx, ty } | PdfOp::MoveTextSetLeading { tx, ty } => {
            *tx += delta.0;
            *ty += delta.1;
        }
        _ => {}
    }
}

fn shift_path(ops: &mut [PdfOp], ctm: PdfMatrix, dx: f64, dy: f64) {
    let inverse = invert(ctm).unwrap_or(PDF_IDENTITY_MATRIX);
    let map = |x: f64, y: f64| {
        let (px, py) = apply(ctm, x, y);
        apply(inverse, px + dx, py + dy)
    };
    for op in ops {
        match op {
            PdfOp::MoveTo { x, y } | PdfOp::LineTo { x, y } => {
                let (nx, ny) = map(*x, *y);
                *x = nx;
                *y = ny;
            }
            PdfOp::Rectangle { x, y, .. } => {
                let (nx, ny) = map(*x, *y);
                *x = nx;
                *y = ny;
            }
            PdfOp::CurveTo { x1, y1, x2, y2, x3, y3 } => {
                let (a, b) = map(*x1, *y1);
                let (c, d) = map(*x2, *y2);
                let (e, f) = map(*x3, *y3);
                *x1 = a;
                *y1 = b;
                *x2 = c;
                *y2 = d;
                *x3 = e;
                *y3 = f;
            }
            PdfOp::CurveToInitial { x2, y2, x3, y3 } => {
                let (c, d) = map(*x2, *y2);
                let (e, f) = map(*x3, *y3);
                *x2 = c;
                *y2 = d;
                *x3 = e;
                *y3 = f;
            }
            PdfOp::CurveToFinal { x1, y1, x3, y3 } => {
                let (a, b) = map(*x1, *y1);
                let (e, f) = map(*x3, *y3);
                *x1 = a;
                *y1 = b;
                *x3 = e;
                *y3 = f;
            }
            _ => {}
        }
    }
}

fn shift_path_bounds(ops: &mut [PdfOp], object: &PageObject, x: f64, y: f64, width: f64, height: f64) {
    if ops.iter().any(|op| matches!(op, PdfOp::Rectangle { .. })) {
        return;
    }
    let inverse = invert(object.ctm).unwrap_or(PDF_IDENTITY_MATRIX);
    let map = |px: f64, py: f64| {
        let (ux, uy) = apply(object.ctm, px, py);
        let nx = if object.width.abs() < 1.0e-6 { x } else { x + (ux - object.x) / object.width * width };
        let ny = if object.height.abs() < 1.0e-6 { y } else { y + (uy - object.y) / object.height * height };
        apply(inverse, nx, ny)
    };
    for op in ops {
        match op {
            PdfOp::MoveTo { x, y } | PdfOp::LineTo { x, y } => {
                let (nx, ny) = map(*x, *y);
                *x = nx;
                *y = ny;
            }
            _ => {}
        }
    }
}
//#endregion 🧮️ContentOps
