//! 🧪️ Actual renderer and canonical text integration laws.
use semio_framework_ui_text::*;
use semio_framework_ui_text::text_reference::KERNED_PANGRAM;

#[test]
fn the_retained_painter_pens_the_kerned_caret_and_breaks_where_the_kerned_wrap_does() {
    use semio_framework_ui_draw_contract::{DrawList, KIND_GLYPH};
    use semio_framework_ui_theme::geometry::Rect;
    use semio_framework_ui_wgpu_paint::{paint_retained_glyph_step_flowed, RetainedGlyphCursor, RetainedGlyphStep, RetainedTextFlow};
    use semio_framework_ui_theme::Rgba;

    let (text, size) = (KERNED_PANGRAM, 12.8);
    let mut atlas = FontAtlas::shaped_default();
    let paint = |atlas: &mut FontAtlas, bounds: Rect, flow: RetainedTextFlow| {
        let (mut draw, mut cursor, mut lines) = (DrawList::default(), RetainedGlyphCursor::default(), Vec::new());
        loop {
            let byte = cursor.byte();
            match paint_retained_glyph_step_flowed(text, bounds, size, Rgba::new(1.0, 1.0, 1.0, 1.0), flow, atlas, &mut draw, &mut cursor) {
                RetainedGlyphStep::Pending => lines.push((byte, cursor.line())),
                RetainedGlyphStep::Complete => break,
                RetainedGlyphStep::Fault => panic!("the pangram never faults the retained painter"),
            }
        }
        let glyphs: Vec<f32> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01).map(|instance| instance.rect[0]).collect();
        (glyphs, lines)
    };
    let (glyphs, _) = paint(&mut atlas, Rect::new(10.0, 0.0, 1_000.0, 40.0), RetainedTextFlow::Clip);
    assert_eq!(glyphs.len(), text.chars().count(), "one glyph per scalar");
    for ((byte, ch), x) in text.char_indices().zip(glyphs) {
        let caret = 10.0 + atlas.pen_at(text, byte, size) + atlas.ensure_glyph(ch, size).bearing_x;
        assert!((x - caret).abs() < 0.001, "{ch:?} at byte {byte} painted at {x}, its kerned caret is {caret}");
    }
    let first_line = "The quick brown fox jumps over ";
    let width = atlas.measure_text(first_line.trim_end(), size).0 + 0.01;
    let expected = atlas.wrap_lines(text, width, size);
    assert_eq!(expected.iter().map(|line| line.start).collect::<Vec<_>>(), [0, first_line.len()], "the kerned first line keeps `over`");
    let (_, lines) = paint(&mut atlas, Rect::new(0.0, 0.0, width, 400.0), RetainedTextFlow::Wrap);
    assert_eq!(lines.len(), text.chars().count(), "every scalar is stepped exactly once");
    for (byte, line) in lines {
        let assigned = expected.iter().position(|range| range.contains(&byte)).unwrap_or_else(|| panic!("byte {byte} lands on no measured line"));
        assert_eq!(line, assigned, "scalar at byte {byte} painted on line {line}, measured onto line {assigned}");
    }
}

#[semio_framework_async_macros::async_test]
async fn a_semibold_run_strikes_twice_at_the_regular_faces_own_advances() {
    let mut atlas = FontAtlas::builtin();
    let size = 19.2;
    let mut regular = semio_framework_ui_draw_contract::DrawList::default();
    semio_framework_ui_wgpu_widget_kit::draw_text_weighted(&mut regular, &mut atlas, "Ab", 10.0, 20.0, size, semio_framework_ui_theme::Rgba::new(1.0, 1.0, 1.0, 1.0), semio_framework_ui_text::TextWeight::Regular);
    let mut semibold = semio_framework_ui_draw_contract::DrawList::default();
    semio_framework_ui_wgpu_widget_kit::draw_text_weighted(&mut semibold, &mut atlas, "Ab", 10.0, 20.0, size, semio_framework_ui_theme::Rgba::new(1.0, 1.0, 1.0, 1.0), semio_framework_ui_text::TextWeight::Semibold);
    let count = |draw: &semio_framework_ui_draw_contract::DrawList| draw.layers.iter().map(|layer| layer.ui_instances.len()).sum::<usize>();
    assert_eq!(count(&semibold), count(&regular) * 2, "the second strike is the whole synthetic weight");
    let widths: Vec<f32> = semibold.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|instance| instance.rect[0]).collect();
    assert!(widths.iter().any(|x| (*x - 10.0).abs() < 0.001), "the first strike sits at the authored origin");
    assert!(widths.iter().any(|x| (*x - (10.0 + semio_framework_ui_text::faux_bold_offset(size))).abs() < 0.001), "the second strike is offset by exactly the faux-bold offset");
}

#[test]
fn the_retained_painter_breaks_where_the_measure_says() {
    use semio_framework_ui_draw_contract::DrawList;
    use semio_framework_ui_theme::geometry::Rect;
    use semio_framework_ui_wgpu_paint::{paint_retained_glyph_step_flowed, RetainedGlyphCursor, RetainedGlyphStep, RetainedTextFlow};
    use semio_framework_ui_theme::Rgba;

    let size = ui_styling::metrics::typography::TEXT_XS_PX as f32;
    let mut atlas = FontAtlas::builtin();
    let max_width = atlas.measure_text("A quick tour of the viewport,", size).0;
    let expected = atlas.wrap_lines(TOUR_BODY, max_width, size);
    let mut draw = DrawList::default();
    let mut cursor = RetainedGlyphCursor::default();
    let bounds = Rect::new(0.0, 0.0, max_width, 400.0);
    let mut painted: Vec<(usize, usize)> = Vec::new();
    for _ in 0..TOUR_BODY.len() + 4 {
        let byte = cursor.byte();
        match paint_retained_glyph_step_flowed(TOUR_BODY, bounds, size, Rgba::new(1.0, 1.0, 1.0, 1.0), RetainedTextFlow::Wrap, &mut atlas, &mut draw, &mut cursor) {
            RetainedGlyphStep::Pending => painted.push((byte, cursor.line())),
            RetainedGlyphStep::Complete => break,
            RetainedGlyphStep::Fault => panic!("the tour body must never fault the retained painter"),
        }
    }
    assert_eq!(painted.len(), TOUR_BODY.chars().count(), "every scalar is stepped exactly once");
    for (byte, line) in painted {
        let assigned = expected.iter().position(|range| range.contains(&byte)).unwrap_or_else(|| panic!("byte {byte} lands on no measured line"));
        assert_eq!(line, assigned, "scalar at byte {byte} ({:?}) painted on line {line}, measured onto line {assigned}", &TOUR_BODY[byte..byte + 1]);
    }
    assert_eq!(expected.len(), 3, "this box holds the paragraph in three lines");
}
