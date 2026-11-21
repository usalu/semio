//! 🧮️ PDF page editor content operators: text replacement, translation, resizing and path shifting inside page content.

use super::*;

//#region 🧮️ContentOps
pub(super) fn ensure_font(snapshot: &PdfSnapshot, text: &str) -> Result<(String, Option<PdfMutation>), Fault> {
    let mut fonts = FontMeasurements::new(snapshot);
    let encodes = |codec: &semio_s_artifact_stdio_pdf_fonts::standards::v1_7::subsets::base::modules::fonts::FontCodec| text.split('\n').all(|line| codec.shows_text(line.strip_suffix('\r').unwrap_or(line)));
    for font in &snapshot.fonts {
        if fonts.codec(&font.id).is_some_and(encodes) { return Ok((font.id.clone(), None)); }
    }
    let font = new_standard_font(snapshot, "Helvetica");
    if !fonts.font_codec(&font).as_ref().is_some_and(encodes) { return Err(fault("no admitted PDF font can show the requested text")); }
    Ok((font.id.clone(), Some(PdfMutation::SetFont(SetFont { font, index: None }))))
}

pub(super) fn new_standard_font(snapshot: &PdfSnapshot, base_font: &str) -> PdfFont {
    let occupied: std::collections::HashSet<_> = snapshot.fonts.iter().map(|font| font.id.as_str()).collect();
    let id = (1..).map(|index| format!("F{index}")).find(|id| !occupied.contains(id.as_str())).expect("available PDF font id");
    PdfFont::standard(id, base_font)
}

pub(super) fn replace_shown_text(content: &mut Vec<PdfOp>, object: &PageObject, text: &str) -> Result<(), Fault> {
    let restore = text_cursor_after(content, object)?;
    match &mut content[object.end] {
        PdfOp::ShowText { text: shown } | PdfOp::NextLineShowText { text: shown } | PdfOp::NextLineShowTextSpaced { text: shown, .. } => *shown = PdfTextString::text(text),
        PdfOp::ShowTextArray { items } => *items = vec![PdfTextArrayItem::Text { text: text.to_owned() }],
        _ => return Err(fault("selected object has no text show operation")),
    }
    content.splice(object.end + 1..object.end + 1, restore);
    Ok(())
}

pub(super) fn text_cursor_after(content: &[PdfOp], object: &PageObject) -> Result<Vec<PdfOp>, Fault> {
    let mut restore = Vec::new();
    for op in &content[object.end + 1..] {
        match op {
            PdfOp::EndText | PdfOp::BeginText | PdfOp::SetTextMatrix { .. } | PdfOp::NextLine | PdfOp::MoveText { .. } | PdfOp::NextLineShowText { .. } | PdfOp::NextLineShowTextSpaced { .. } => break,
            PdfOp::ShowText { .. } | PdfOp::ShowTextArray { .. } => {
                let placement = object.text_placement.as_ref().ok_or_else(|| fault("text placement is missing"))?;
                let inverse = invert(placement.line).ok_or_else(|| fault("cannot restore a singular text line matrix"))?;
                let (advance, _) = apply(inverse, placement.next_origin.0, placement.next_origin.1);
                restore.push(PdfOp::SetTextMatrix { matrix: placement.line });
                append_advance(&mut restore, advance, object.font_size, placement.horizontal_scale)?;
                break;
            }
            _ => {}
        }
    }
    Ok(restore)
}

pub(super) fn delete_paint(content: &mut Vec<PdfOp>, object: &PageObject) {
    let span = object.start..=object.end;
    match object.kind {
        ObjectKind::Vector => {
            if content[span.clone()].iter().any(|op| matches!(op, PdfOp::Clip | PdfOp::ClipEvenOdd)) {
                content[object.end] = PdfOp::EndPath;
                return;
            }
            content.remove(object.end);
            content.extract_if(object.start..object.end, |op| matches!(op,
                PdfOp::MoveTo { .. } | PdfOp::LineTo { .. } | PdfOp::Rectangle { .. } | PdfOp::ClosePath |
                PdfOp::CurveTo { .. } | PdfOp::CurveToInitial { .. } | PdfOp::CurveToFinal { .. }
            )).for_each(drop);
        }
        ObjectKind::Image | ObjectKind::Form | ObjectKind::Text | ObjectKind::Shading => { content.remove(object.end); }
        ObjectKind::Annotation => unreachable!("annotations use page mutations"),
    }
}

pub(super) fn translate_object(content: &mut Vec<PdfOp>, object: &PageObject, dx: f64, dy: f64) -> Result<(), Fault> {
    match object.kind {
        ObjectKind::Text => return translate_text(content, object, dx, dy),
        ObjectKind::Image | ObjectKind::Form => return transform_xobject(content, object, [1.0, 0.0, 0.0, 1.0, dx, dy]),
        ObjectKind::Vector => return transform_path(content, object, [1.0, 0.0, 0.0, 1.0, dx, dy]),
        ObjectKind::Shading | ObjectKind::Annotation => {}
    }
    Ok(())
}

pub(super) fn resize_object(content: &mut Vec<PdfOp>, object: &PageObject, x: f64, y: f64, width: f64, height: f64) -> Result<(), Fault> {
    match object.kind {
        ObjectKind::Vector | ObjectKind::Image | ObjectKind::Form => {
            let sx = if object.width == 0.0 { 1.0 } else { width / object.width };
            let sy = if object.height == 0.0 { 1.0 } else { height / object.height };
            let transform = [sx, 0.0, 0.0, sy, x - object.x * sx, y - object.y * sy];
            return if object.kind == ObjectKind::Vector {
                transform_path(content, object, transform)
            } else {
                transform_xobject(content, object, transform)
            };
        }
        ObjectKind::Text => return resize_text(content, object, x, y, width, height),
        ObjectKind::Shading | ObjectKind::Annotation => {}
    }
    Ok(())
}

fn previous_color(content: &[PdfOp], stroking: bool) -> Vec<PdfOp> {
    use PdfOp::*;
    let mut nested = 0;
    let mut components = None;
    for op in content.iter().rev() {
        match (stroking, op) {
            (_, Restore) => nested += 1,
            (_, Save) if nested > 0 => nested -= 1,
            _ if nested > 0 => {}
            (true, SetStrokeColor { .. } | SetStrokeColorN { .. }) |
            (false, SetFillColor { .. } | SetFillColorN { .. }) if components.is_none() => components = Some(op.clone()),
            (true, SetStrokeGray { .. } | SetStrokeRgb { .. } | SetStrokeCmyk { .. } | SetStrokeColorSpace { .. }) |
            (false, SetFillGray { .. } | SetFillRgb { .. } | SetFillCmyk { .. } | SetFillColorSpace { .. }) => {
                return std::iter::once(op.clone()).chain(components).collect();
            }
            _ => {}
        }
    }
    let initial = if stroking { SetStrokeGray { gray: 0.0 } } else { SetFillGray { gray: 0.0 } };
    std::iter::once(initial).chain(components).collect()
}

pub(super) fn set_fill(content: &mut Vec<PdfOp>, object: &PageObject, red: f64, green: f64, blue: f64) {
    let at = object.end;
    let restore = previous_color(&content[..at], false);
    match content[at] {
        PdfOp::Stroke => content[at] = PdfOp::FillStroke,
        PdfOp::CloseStroke => content[at] = PdfOp::CloseFillStroke,
        _ => {}
    }
    content.splice(at + 1..at + 1, restore);
    content.insert(at, PdfOp::SetFillRgb { r: red, g: green, b: blue });
}

pub(super) fn set_stroke(content: &mut Vec<PdfOp>, object: &PageObject, red: f64, green: f64, blue: f64, width: f64) {
    if let Some(op) = content.get_mut(object.end) {
        match op {
            PdfOp::Fill => *op = PdfOp::FillStroke,
            PdfOp::FillEvenOdd => *op = PdfOp::FillStrokeEvenOdd,
            _ => {}
        }
    }
    let at = object.end;
    let mut restore = previous_color(&content[..at], true);
    restore.push(PdfOp::SetLineWidth { width: object.line_width });
    content.splice(at + 1..at + 1, restore);
    content.splice(at..at, [PdfOp::SetStrokeRgb { r: red, g: green, b: blue }, PdfOp::SetLineWidth { width: width.max(0.0) }]);
}

fn translate_text(content: &mut Vec<PdfOp>, object: &PageObject, dx: f64, dy: f64) -> Result<(), Fault> {
    if dx == 0.0 && dy == 0.0 { return Ok(()); }
    let placement = object.text_placement.as_ref().ok_or_else(|| fault("text placement is missing"))?;
    let inverse = invert(object.text_transform).ok_or_else(|| fault("cannot move text drawn with a singular matrix"))?;
    let horizontal = inverse[0] * dx + inverse[2] * dy;
    let vertical = inverse[1] * dx + inverse[3] * dy;
    if horizontal != 0.0 && object.font_size == 0.0 { return Err(fault("cannot shift text with zero font size")); }
    let adjust = |amount| PdfOp::ShowTextArray { items: vec![PdfTextArrayItem::Adjust { amount }] };
    let mut replacement = Vec::new();
    let paint = show_with_explicit_line(content[object.end].clone(), &mut replacement);
    if vertical != 0.0 { replacement.push(PdfOp::SetTextRise { rise: placement.rise + vertical }); }
    if horizontal != 0.0 { replacement.push(adjust(-horizontal * 1000.0 / object.font_size)); }
    replacement.push(paint);
    if horizontal != 0.0 { replacement.push(adjust(horizontal * 1000.0 / object.font_size)); }
    if vertical != 0.0 { replacement.push(PdfOp::SetTextRise { rise: placement.rise }); }
    content.splice(object.end..=object.end, replacement);
    Ok(())
}

fn show_with_explicit_line(op: PdfOp, replacement: &mut Vec<PdfOp>) -> PdfOp {
    match op {
        PdfOp::NextLineShowText { text } => {
            replacement.push(PdfOp::NextLine);
            PdfOp::ShowText { text }
        }
        PdfOp::NextLineShowTextSpaced { word_spacing, char_spacing, text } => {
            replacement.extend([PdfOp::SetWordSpacing { spacing: word_spacing }, PdfOp::SetCharSpacing { spacing: char_spacing }, PdfOp::NextLine]);
            PdfOp::ShowText { text }
        }
        op => op,
    }
}

pub(super) fn restyle_text(content: &mut Vec<PdfOp>, object: &PageObject, paint: PdfOp, name: String, size: f64) -> Result<(), Fault> {
    let mut replacement = Vec::new();
    let paint = show_with_explicit_line(paint, &mut replacement);
    replacement.extend([PdfOp::SetFont { name, size }, paint]);
    if !object.font.is_empty() { replacement.push(PdfOp::SetFont { name: object.font.clone(), size: object.font_size }); }
    replacement.extend(text_cursor_after(content, object)?);
    content.splice(object.end..=object.end, replacement);
    Ok(())
}

pub(super) fn delete_text(content: &mut Vec<PdfOp>, object: &PageObject) -> Result<(), Fault> {
    let placement = object.text_placement.as_ref().ok_or_else(|| fault("text placement is missing"))?;
    let inverse = invert(placement.matrix).ok_or_else(|| fault("cannot restore a singular text matrix"))?;
    let (advance, _) = apply(inverse, placement.next_origin.0, placement.next_origin.1);
    let mut replacement = Vec::new();
    show_with_explicit_line(content[object.end].clone(), &mut replacement);
    append_advance(&mut replacement, advance, object.font_size, placement.horizontal_scale)?;
    content.splice(object.end..=object.end, replacement);
    Ok(())
}

fn resize_text(content: &mut Vec<PdfOp>, object: &PageObject, x: f64, y: f64, width: f64, height: f64) -> Result<(), Fault> {
    let placement = object.text_placement.as_ref().ok_or_else(|| fault("text placement is missing"))?;
    if object.width <= 0.0 || object.height <= 0.0 || object.font_size * placement.horizontal_scale == 0.0 {
        return Err(fault("cannot resize text without an advance and height"));
    }
    let sx = width / object.width;
    let sy = height / object.height;
    let world = [sx, 0.0, 0.0, sy, x - sx * object.x, y - sy * object.y];
    let inverse = invert(object.ctm).ok_or_else(|| fault("cannot resize text drawn with a singular matrix"))?;
    let matrix = multiply(inverse, multiply(world, multiply(object.ctm, placement.matrix)));
    let line_inverse = invert(placement.line).ok_or_else(|| fault("cannot restore a singular text line matrix"))?;
    let (advance, _) = apply(line_inverse, placement.next_origin.0, placement.next_origin.1);
    let mut replacement = Vec::new();
    let paint = show_with_explicit_line(content[object.end].clone(), &mut replacement);
    replacement.extend([PdfOp::SetTextMatrix { matrix }, paint, PdfOp::SetTextMatrix { matrix: placement.line }]);
    append_advance(&mut replacement, advance, object.font_size, placement.horizontal_scale)?;
    content.splice(object.end..=object.end, replacement);
    Ok(())
}

fn transform_xobject(content: &mut Vec<PdfOp>, object: &PageObject, transform: PdfMatrix) -> Result<(), Fault> {
    let inverse = invert(object.ctm).ok_or_else(|| fault("cannot transform an object drawn with a singular matrix"))?;
    let matrix = multiply(inverse, multiply(transform, object.ctm));
    let at = object.end;
    content.insert(at + 1, PdfOp::Restore);
    content.splice(at..at, [PdfOp::Save, PdfOp::Transform { matrix }]);
    Ok(())
}

fn advance_ctm(stack: &mut Vec<PdfMatrix>, op: &PdfOp) {
    match op {
        PdfOp::Save => stack.push(*stack.last().unwrap()),
        PdfOp::Restore if stack.len() > 1 => { stack.pop(); }
        PdfOp::Transform { matrix } => {
            let current = stack.last_mut().unwrap();
            *current = multiply(*current, *matrix);
        }
        _ => {}
    }
}

fn transform_path(content: &mut Vec<PdfOp>, object: &PageObject, transform: PdfMatrix) -> Result<(), Fault> {
    let mut stack = vec![PDF_IDENTITY_MATRIX];
    for op in &content[..object.start] {
        advance_ctm(&mut stack, op);
    }
    let mut replacement = Vec::new();
    for mut op in content[object.start..object.end].iter().cloned() {
        advance_ctm(&mut stack, &op);
        if !matches!(op, PdfOp::MoveTo { .. } | PdfOp::LineTo { .. } | PdfOp::CurveTo { .. } |
            PdfOp::CurveToInitial { .. } | PdfOp::CurveToFinal { .. } | PdfOp::Rectangle { .. }) {
            replacement.push(op);
            continue;
        }
        let ctm = *stack.last().unwrap();
        let inverse = invert(ctm).ok_or_else(|| fault("cannot transform a path drawn with a singular matrix"))?;
        let local = multiply(inverse, multiply(transform, ctm));
        let point = |x: &mut f64, y: &mut f64| { (*x, *y) = apply(local, *x, *y); };
        match &mut op {
            PdfOp::MoveTo { x, y } | PdfOp::LineTo { x, y } => point(x, y),
            PdfOp::CurveTo { x1, y1, x2, y2, x3, y3 } => { point(x1, y1); point(x2, y2); point(x3, y3); }
            PdfOp::CurveToInitial { x2, y2, x3, y3 } => { point(x2, y2); point(x3, y3); }
            PdfOp::CurveToFinal { x1, y1, x3, y3 } => { point(x1, y1); point(x3, y3); }
            PdfOp::Rectangle { x, y, width, height } => {
                let (left, bottom) = apply(local, *x, *y);
                if local[1] == 0.0 && local[2] == 0.0 {
                    (*x, *y) = (left, bottom);
                    *width *= local[0];
                    *height *= local[3];
                } else {
                    replacement.push(PdfOp::MoveTo { x: left, y: bottom });
                    for (px, py) in [(*x + *width, *y), (*x + *width, *y + *height), (*x, *y + *height)] {
                        let (x, y) = apply(local, px, py);
                        replacement.push(PdfOp::LineTo { x, y });
                    }
                    replacement.push(PdfOp::ClosePath);
                    continue;
                }
            }
            _ => unreachable!(),
        }
        replacement.push(op);
    }
    content.splice(object.start..object.end, replacement);
    Ok(())
}
//#endregion 🧮️ContentOps
