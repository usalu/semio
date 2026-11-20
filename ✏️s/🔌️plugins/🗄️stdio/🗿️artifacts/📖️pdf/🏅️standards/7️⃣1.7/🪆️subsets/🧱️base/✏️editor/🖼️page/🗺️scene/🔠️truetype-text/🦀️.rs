//! 🔠️ Paints glyph outlines from the admitted PDF TrueType font program.

use super::*;
use super::type3_text::{text_pieces, Piece};
use semio_s_artifact_stdio_pdf_fonts::standards::v1_7::subsets::base::modules::fonts::truetype::TrueTypeOutlineSegment;
use std::collections::HashMap;

pub(super) fn paint_truetype_text(
    item: &Placed, content: &[PdfOp], mode: u32, color: [f32; 4], stroke: [f32; 4],
    layers: &mut Vec<Value>, paint: &mut FormPaint<'_>,
) -> bool {
    let Some(segments) = truetype_outline(&item.object, content.get(item.object.start), &mut paint.metrics, &mut paint.remaining_ops, |x, y| (x, y)) else { return false; };
    if segments.is_empty() { return true; }
    let display = |x, y| {
        let (x, y) = apply(item.object.text_transform, x, y);
        item.viewport.point(x, y)
    };
    let ((x, y), (a, b), (c, d)) = (display(0.0, 0.0), display(1.0, 0.0), display(0.0, 1.0));
    let mut layer = json!({
        "id": item.object.id,
        "segments": mapped_vector_segments(&segments, |point| point, false),
        "transform": [a - x, b - y, c - x, d - y, x, y],
        "fillRule": "nonzero",
    });
    if mode != 1 { layer["fill"] = json!({ "color": color }); }
    if mode != 0 { layer["stroke"] = json!({ "color": stroke, "width": item.object.line_width }); }
    layers.push(layer);
    true
}

pub(crate) fn truetype_outline(
    object: &PageObject, operator: Option<&PdfOp>, metrics: &mut FontMeasurements<'_>, remaining: &mut usize,
    map: impl Fn(f64, f64) -> (f64, f64),
) -> Option<Vec<VectorSegment>> {
    let position = object.text_placement.as_ref()?;
    let codec = metrics.codec(&object.font)?;
    let font = codec.outline_program()?;
    let pieces = text_pieces(operator?, *remaining)?;
    let mut outline_cache = HashMap::new();
    let mut segments = Vec::new();
    let size = object.font_size;
    let scale = size / f64::from(font.units_per_em);
    let mut displacement = 0.0;
    let mut first = true;
    for piece in pieces {
        let glyphs = match piece {
            Piece::Adjust(amount) => {
                if !first { displacement -= amount * size / 1000.0; }
                continue;
            }
            Piece::Codes(codes) => codec.decode_codes(codes),
            Piece::Text(text) => {
                let bytes = codec.encode(text)?;
                codec.decode(&bytes)
            }
        };
        for glyph in glyphs {
            first = false;
            let outline = outline_cache.entry(glyph.glyph_id.unwrap_or(0)).or_insert_with_key(|&id| font.glyph_outline(id));
            let outline = outline.as_ref()?;
            *remaining = remaining.checked_sub(outline.len() + 1)?;
            append_outline(&mut segments, outline, scale, displacement, &map);
            displacement += glyph.width * size / 1000.0 + position.char_spacing
                + if glyph.is_space_char { position.word_spacing } else { 0.0 };
        }
    }
    Some(segments)
}

fn append_outline(output: &mut Vec<VectorSegment>, outline: &[TrueTypeOutlineSegment], scale: f64, displacement: f64, map: impl Fn(f64, f64) -> (f64, f64)) {
    use TrueTypeOutlineSegment::*;
    let point = |[x, y]: [f64; 2]| map(x * scale + displacement, y * scale);
    let mut current = (0.0, 0.0);
    let mut start = current;
    let interpolate = |a: (f64, f64), b: (f64, f64)| (a.0 + (b.0 - a.0) * 2.0 / 3.0, a.1 + (b.1 - a.1) * 2.0 / 3.0);
    for segment in outline {
        output.push(match *segment {
            MoveTo(to) => { current = point(to); start = current; VectorSegment::Move(current) }
            LineTo(to) => { current = point(to); VectorSegment::Line(current) }
            QuadraticTo { control, to } => {
                let end = point(to);
                let control = point(control);
                let first = interpolate(current, control);
                let second = interpolate(end, control);
                current = end;
                VectorSegment::Cubic { first, second, end }
            }
            Close => { current = start; VectorSegment::Close }
        });
    }
}
