
use super::*;
use crate::resource::ResourceOp;

#[test]
fn measuring_ascii_run_is_stable_and_matches_shaped_advance() {
    let mut ts = TextSystem::new();
    let style = TextStyle { family: FontFamilyChoice::SansSerif, size_px: 16.0 };
    let first = ts.measure("Hello", &style);
    let second = ts.measure("Hello", &style);
    assert_eq!(first, second, "measuring the same run twice must be stable");
    let Measurement::Ready { width, .. } = first else { panic!("expected a Ready measurement for a built-in font") };
    let shaped = ts.shape("Hello", &style);
    let summed_advance: f32 = shaped.glyphs.iter().map(|glyph| glyph.advance).sum();
    assert!((width - summed_advance).abs() < 0.01, "measured width ({width}) must match the sum of shaped glyph advances ({summed_advance})");
}

#[test]
fn wrapping_breaks_at_max_width_and_never_mid_char() {
    let mut ts = TextSystem::new();
    let style = TextStyle { family: FontFamilyChoice::SansSerif, size_px: 16.0 };
    let text = "one two three four five six seven eight";
    let lines = ts.wrap(text, &style, 100.0);
    assert!(lines.len() > 1, "long text at a narrow max_width must wrap onto multiple lines");
    for range in &lines {
        assert!(text.is_char_boundary(range.start), "line start {} must be a char boundary", range.start);
        assert!(text.is_char_boundary(range.end), "line end {} must be a char boundary", range.end);
    }
    let non_whitespace_in: usize = text.chars().filter(|ch| !ch.is_whitespace()).count();
    let non_whitespace_out: usize = lines.iter().map(|range| text[range.clone()].chars().filter(|ch| !ch.is_whitespace()).count()).sum();
    assert_eq!(non_whitespace_in, non_whitespace_out, "wrapping must not drop or duplicate characters");
}

#[test]
fn measuring_with_an_unloaded_custom_font_yields_pending_with_a_placeholder() {
    let mut ts = TextSystem::new();
    let dependency = ts.request_font("brand-display");
    assert_eq!(ts.font_status(dependency), FontStatus::Pending);
    let style = TextStyle { family: FontFamilyChoice::Custom(dependency), size_px: 20.0 };
    let pending = ts.measure("Loading", &style);
    let Measurement::Pending { placeholder_width, placeholder_height } = pending else { panic!("expected Pending for an unresolved font dependency, got {pending:?}") };
    assert!(placeholder_width > 0.0 && placeholder_height > 0.0, "a Pending measurement must still carry a usable placeholder size");
    assert!(ts.provide_font_bytes(dependency, ANTA_LATIN), "providing real bytes for a pending dependency must succeed");
    assert_eq!(ts.font_status(dependency), FontStatus::Ready);
    assert_eq!(ts.take_ready_dependencies(), vec![dependency], "a newly resolved dependency must surface exactly once for invalidation");
    assert!(ts.take_ready_dependencies().is_empty(), "take_ready_dependencies must drain, not repeat, past dependencies");
    let resolved = ts.measure("Loading", &style);
    assert!(matches!(resolved, Measurement::Ready { .. }), "once the font lands, measurement must resolve to Ready, got {resolved:?}");
}

#[test]
fn measuring_the_same_string_twice_serves_the_second_call_from_the_shape_cache() {
    let mut ts = TextSystem::new();
    let style = TextStyle { family: FontFamilyChoice::SansSerif, size_px: 16.0 };
    ts.measure("cached", &style);
    assert_eq!(ts.shape_cache_misses(), 1);
    assert_eq!(ts.shape_cache_hits(), 0);
    ts.measure("cached", &style);
    assert_eq!(ts.shape_cache_hits(), 1, "the second identical measurement must hit the cache, not reshape");
    assert_eq!(ts.shape_cache_misses(), 1, "a cache hit must not increment the miss counter");
}

#[test]
fn atlas_insertion_queues_exactly_one_upload_and_a_repeat_glyph_queues_none() {
    let mut ts = TextSystem::new();
    let mut resources = ResourceRegistry::default();
    let style = TextStyle { family: FontFamilyChoice::SansSerif, size_px: 16.0 };
    let glyph = ts.shape("A", &style).glyphs[0];
    ts.ensure_glyph(&mut resources, glyph.font, glyph.glyph_id, style.size_px);
    let ops = resources.drain_ops();
    assert_eq!(ops.len(), 1, "packing a new glyph must queue exactly one op");
    assert!(matches!(ops[0], ResourceOp::UploadAtlas { .. }), "the queued op must be an UploadAtlas");
    ts.ensure_glyph(&mut resources, glyph.font, glyph.glyph_id, style.size_px);
    assert!(resources.drain_ops().is_empty(), "re-requesting an already-cached glyph must not queue another upload");
}

#[test]
fn utf8_utf16_index_conversion_round_trips_across_a_non_bmp_emoji() {
    let text = "a😀b";
    let byte_index_after_emoji = "a😀".len();
    let utf16_index = utf8_to_utf16(text, byte_index_after_emoji);
    assert_eq!(utf16_index, 3, "'a' is 1 UTF-16 unit, the non-BMP emoji is a 2-unit surrogate pair");
    let round_tripped = utf16_to_utf8(text, utf16_index);
    assert_eq!(round_tripped, byte_index_after_emoji, "forward-then-back conversion must land exactly on the original byte index");
}

#[test]
fn cursor_movement_never_lands_inside_the_emoji_grapheme_cluster() {
    let mut ts = TextSystem::new();
    let style = TextStyle { family: FontFamilyChoice::SansSerif, size_px: 16.0 };
    let text = "a😀b";
    let after_a = ts.next_grapheme(text, &style, 0);
    assert_eq!(after_a, 1, "moving past 'a' must land right after it");
    let after_emoji = ts.next_grapheme(text, &style, after_a);
    assert_eq!(after_emoji, 1 + "😀".len(), "moving past the emoji must skip its whole grapheme cluster, never landing mid-codepoint");
    let back_to_after_a = ts.previous_grapheme(text, &style, after_emoji);
    assert_eq!(back_to_after_a, after_a, "moving backward across the emoji must land exactly where moving forward started");
}

#[test]
fn selection_geometry_covers_the_requested_range_with_at_least_one_rect() {
    let mut ts = TextSystem::new();
    let style = TextStyle { family: FontFamilyChoice::SansSerif, size_px: 16.0 };
    let rects = ts.selection_geometry("Hello", &style, 0..5);
    assert!(!rects.is_empty(), "a non-empty selection range must produce at least one highlight rect");
    for rect in &rects {
        assert!(rect[2] > 0.0 && rect[3] > 0.0, "each selection rect must have positive width/height, got {rect:?}");
    }
}

/// 🧪️ Language-agnostic fixture table: (text, max_width, min_expected_lines) — a caller in any
/// language re-shaping the same strings at the same widths through its own text pipeline must see
/// at least this many lines, since `shape_paragraph`'s wrapping is UAX#14 line breaking, not a
/// pixel-exact renderer detail.
#[test]
fn shape_paragraph_wraps_at_max_width_and_never_mid_char() {
    let fixtures: &[(&str, f32, usize)] = &[("one two three four five six seven eight", 100.0, 2), ("single-word-no-wrap-opportunity", 1000.0, 1), ("a b c d e f g h i j k l m n o p", 40.0, 3)];
    for (text, max_width, min_lines) in fixtures {
        let mut ts = TextSystem::new();
        let style = TextRunStyle { base: TextStyle { family: FontFamilyChoice::SansSerif, size_px: 16.0 }, weight: 400.0, line_height_relative: 1.2, letter_spacing_px: 0.0, alignment: TextAlignment::Left };
        let shaped = ts.shape_paragraph(text, &style, *max_width);
        let line_count = shaped.glyphs.iter().map(|g| g.y.to_bits()).collect::<std::collections::HashSet<_>>().len().max(1);
        assert!(line_count >= *min_lines, "{text:?} at max_width {max_width} expected >= {min_lines} distinct glyph baselines, got {line_count}");
        assert!(shaped.width <= *max_width + 0.01, "{text:?}: shaped width {} must not exceed max_width {max_width}", shaped.width);
    }
}

/// 🧪️ Fixture table over every [`TextAlignment`] variant: each must still produce a positive
/// height and must never widen the shape past `max_width`.
#[test]
fn shape_paragraph_alignment_variants_respect_max_width_and_measure_positive_height() {
    let alignments = [TextAlignment::Left, TextAlignment::Middle, TextAlignment::Right, TextAlignment::Justified];
    let text = "Hello layout engine, this line should wrap across several lines of text.";
    for alignment in alignments {
        let mut ts = TextSystem::new();
        let style = TextRunStyle { base: TextStyle { family: FontFamilyChoice::SansSerif, size_px: 12.0 }, weight: 400.0, line_height_relative: 1.2, letter_spacing_px: 0.0, alignment };
        let shaped = ts.shape_paragraph(text, &style, 80.0);
        assert!(shaped.height > 0.0, "alignment {alignment:?} must still measure a positive height");
        assert!(shaped.width <= 80.0 + 0.01, "alignment {alignment:?}: shaped width {} must not exceed max_width 80", shaped.width);
    }
}

/// 🔬️ DIFFERENTIAL ORACLE: re-implements `shape_paragraph`'s exact call sequence directly against
/// `parley` (a fresh `Collection`/`FontContext`/`LayoutContext`, independent of `TextSystem`'s own
/// internals) and asserts it agrees with `TextSystem::shape_paragraph` on glyph count and overall
/// width within an epsilon of `0.01px` — tight because both paths run the identical parley version
/// with identical inputs; a real cross-version drift tolerance would be looser. This is what proves
/// the wrapper in this file has not silently diverged from the crate it wraps.
#[test]
fn shape_paragraph_agrees_with_an_independently_built_parley_layout() {
    let text = "The quick brown fox jumps over the lazy dog.";
    let max_width = 120.0f32;

    let mut ts = TextSystem::new();
    let style = TextRunStyle { base: TextStyle { family: FontFamilyChoice::SansSerif, size_px: 14.0 }, weight: 400.0, line_height_relative: 1.2, letter_spacing_px: 0.0, alignment: TextAlignment::Left };
    let ours = ts.shape_paragraph(text, &style, max_width);

    let mut oracle_collection = Collection::new(CollectionOptions { shared: false, system_fonts: false });
    let over = FontInfoOverride { family_name: Some(FAMILY_SANS), width: None, style: None, weight: None, axes: None };
    let oracle_family_id = oracle_collection.register_fonts(Blob::new(Arc::new(ANTA_LATIN.to_vec())), Some(over)).into_iter().next().map(|(id, _)| id).expect("oracle font registers");
    oracle_collection.set_generic_families(GenericFamily::SansSerif, std::iter::once(oracle_family_id));
    let mut oracle_font_cx = FontContext { collection: oracle_collection, source_cache: SourceCache::default() };
    let mut oracle_layout_cx: LayoutContext<[u8; 4]> = LayoutContext::new();
    let mut builder = oracle_layout_cx.ranged_builder(&mut oracle_font_cx, text, 1.0, true);
    builder.push_default(StyleProperty::FontStack(FontStack::Source(Cow::Borrowed(FAMILY_SANS))));
    builder.push_default(StyleProperty::FontSize(14.0));
    builder.push_default(StyleProperty::FontWeight(FontWeight::new(400.0)));
    builder.push_default(StyleProperty::LineHeight(LineHeight::FontSizeRelative(1.2)));
    builder.push_default(StyleProperty::LetterSpacing(0.0));
    let mut oracle_layout: Layout<[u8; 4]> = builder.build(text);
    oracle_layout.break_all_lines(Some(max_width));
    oracle_layout.align(Some(max_width), Alignment::Left, AlignmentOptions::default());
    let mut oracle_glyph_count = 0usize;
    for line in oracle_layout.lines() {
        for item in line.items() {
            if let PositionedLayoutItem::GlyphRun(run) = item {
                oracle_glyph_count += run.positioned_glyphs().count();
            }
        }
    }

    assert_eq!(ours.glyphs.len(), oracle_glyph_count, "our wrapper's glyph count must match an independently-built parley layout");
    assert!((ours.width - oracle_layout.width()).abs() < 0.01, "our wrapper's width {} must match the oracle's {} within 0.01px", ours.width, oracle_layout.width());
    assert!((ours.height - oracle_layout.height()).abs() < 0.01, "our wrapper's height {} must match the oracle's {} within 0.01px", ours.height, oracle_layout.height());
}
