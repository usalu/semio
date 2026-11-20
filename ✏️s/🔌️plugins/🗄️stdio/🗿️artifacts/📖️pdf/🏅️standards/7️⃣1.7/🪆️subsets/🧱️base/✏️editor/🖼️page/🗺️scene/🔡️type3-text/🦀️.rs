//! 🔡️ Native PDF Type 3 text paint using the document's authored glyph programs.

use super::*;
use semio_s_artifact_stdio_pdf_fonts::standards::v1_7::subsets::base::modules::fonts::base_encoding_table;

pub(super) enum Piece<'a> { Text(&'a str), Codes(&'a [u32]), Adjust(f64) }

pub(super) fn paint_type3_text<'a>(snapshot: &'a PdfSnapshot, page: &PdfPage, item: &Placed, content: &[PdfOp], layers: &mut Vec<Value>, paint: &mut FormPaint<'a>) -> bool {
    let Some(font) = snapshot.font(&item.object.font) else { return false; };
    let PdfFontKind::Type3 { font_matrix, encoding, first_char, widths, char_procs, .. } = &font.kind else { return false; };
    let Some(position) = item.object.text_placement.as_ref() else { return false; };
    if paint.active_fonts.len() >= MAX_FORM_PAINT_DEPTH || paint.active_fonts.contains(&font.id.as_str()) { return true; }
    let Some(operator) = content.get(item.object.start) else { return false; };
    let Some(pieces) = text_pieces(operator, paint.remaining_ops) else { return false; };
    let mut displacement = 0.0;
    let mut first = true;
    let mut pending = Vec::new();
    let size = item.object.font_size;
    let mut remaining = paint.remaining_ops;
    for piece in pieces {
        let codes = match piece {
            Piece::Adjust(amount) => {
                if !first { displacement -= amount * size / 1000.0; }
                continue;
            }
            Piece::Codes(codes) => std::borrow::Cow::Borrowed(codes),
            Piece::Text(text) => {
                let Some(codes) = paint.metrics.encoded_codes(&item.object.font, text) else { return false; };
                std::borrow::Cow::Owned(codes)
            }
        };
        for code in codes.iter().copied() {
            first = false;
            let name = glyph_name(encoding, code);
            let glyph = char_procs.iter().find(|glyph| glyph.name == name).or_else(|| char_procs.iter().find(|glyph| glyph.name == ".notdef"));
            let work = glyph.map_or(1, |glyph| glyph.content.len().saturating_add(1));
            let Some(budget) = remaining.checked_sub(work) else { return true; };
            remaining = budget;
            if let Some(glyph) = glyph {
                let placement = [size, 0.0, 0.0, size, displacement, 0.0];
                pending.push((glyph, multiply(item.object.text_transform, multiply(placement, *font_matrix))));
            }
            let width = code.checked_sub(*first_char).and_then(|index| widths.get(index as usize)).copied().unwrap_or(0.0);
            displacement += width * font_matrix[0] * size + position.char_spacing + if code == 32 { position.word_spacing } else { 0.0 };
        }
    }
    paint.remaining_ops = remaining;
    paint.active_fonts.push(&font.id);
    for (index, (glyph, ctm)) in pending.into_iter().enumerate() {
        let graphics = Graphics {
            ctm, fill: item.object.fill, stroke: item.object.stroke, line_width: item.object.line_width,
            vector_stroke: item.object.vector_stroke.clone(), ..Graphics::default()
        };
        for mut object in content_objects(snapshot, item.object.page, page, &glyph.content, graphics, &mut paint.metrics) {
            object.id = format!("{}/g{index}/{}", item.object.id, object.id);
            let (canvas_x, canvas_y, canvas_w, canvas_h) = item.viewport.rect(object.x, object.y, object.width, object.height);
            let child = Placed { object, top: item.top, viewport: item.viewport, canvas_x, canvas_y, canvas_w, canvas_h };
            paint_object(snapshot, page, &glyph.content, &child, layers, paint);
        }
    }
    paint.active_fonts.pop();
    true
}

fn glyph_name(encoding: &PdfSimpleEncoding, code: u32) -> &str {
    if let Some(difference) = encoding.differences.iter().rev().find(|difference| difference.code == code) { return &difference.glyph; }
    base_encoding_table(encoding.base.unwrap_or(PdfBaseEncoding::Standard)).iter()
        .find_map(|&(candidate, name)| (u32::from(candidate) == code).then_some(name)).unwrap_or(".notdef")
}

pub(super) fn text_pieces(operator: &PdfOp, maximum: usize) -> Option<Vec<Piece<'_>>> {
    match operator {
        PdfOp::ShowText { text } | PdfOp::NextLineShowText { text } | PdfOp::NextLineShowTextSpaced { text, .. } => Some(match text {
            PdfTextString::Text { text } if text.len() <= maximum => vec![Piece::Text(text)],
            PdfTextString::Codes { codes } if codes.len() <= maximum => vec![Piece::Codes(codes)],
            _ => return None,
        }),
        PdfOp::ShowTextArray { items } if items.len() <= maximum => {
            let mut left = maximum;
            let mut pieces = Vec::with_capacity(items.len());
            for item in items {
                let (piece, size) = match item {
                    PdfTextArrayItem::Text { text } => (Piece::Text(text), text.len()),
                    PdfTextArrayItem::Codes { codes } => (Piece::Codes(codes), codes.len()),
                    PdfTextArrayItem::Adjust { amount } => (Piece::Adjust(*amount), 1),
                };
                left = left.checked_sub(size)?;
                pieces.push(piece);
            }
            Some(pieces)
        }
        _ => None,
    }
}
