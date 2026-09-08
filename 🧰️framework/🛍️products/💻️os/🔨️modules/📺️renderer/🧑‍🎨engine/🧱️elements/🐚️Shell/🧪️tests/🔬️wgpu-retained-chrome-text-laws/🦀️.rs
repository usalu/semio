
use super::*;

/// 🪟️ A multi-megabyte dialog or tour string consumes exactly one scalar per grant.
#[test]
fn dialog_and_tour_text_advance_one_scalar_and_one_glyph_per_grant() {
    let value = "x".repeat(2 * 1_024 * 1_024);
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let mut cursor = RetainedGlyphCursor::default();
    assert_eq!(chrome_text_step(&mut draw, &mut atlas, &value, 0.0, 16.0, 320.0, 14.0, Rgba::new(1.0, 1.0, 1.0, 1.0), &mut cursor), RetainedGlyphStep::Pending);
    assert_eq!(cursor.byte(), 1);
    assert_eq!(draw.layers.iter().map(|layer| layer.ui_instances.len()).sum::<usize>(), 1);
    assert_eq!(chrome_text_step(&mut draw, &mut atlas, &value, 0.0, 16.0, 320.0, 14.0, Rgba::new(1.0, 1.0, 1.0, 1.0), &mut cursor), RetainedGlyphStep::Pending);
    assert_eq!(cursor.byte(), 2);
    assert_eq!(draw.layers.iter().map(|layer| layer.ui_instances.len()).sum::<usize>(), 2);
}

/// 🚫️ The retained Shell text boundary fails closed before shaping MAX + 1 bytes.
#[test]
fn dialog_and_tour_text_max_plus_one_fails_without_output() {
    let value = "x".repeat(ui_wgpu::wgpu::RETAINED_NODE_TEXT_MAX_BYTES + 1);
    let identity = value.as_ptr();
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let mut cursor = RetainedGlyphCursor::default();
    assert_eq!(chrome_text_step(&mut draw, &mut atlas, &value, 0.0, 16.0, 320.0, 14.0, Rgba::new(1.0, 1.0, 1.0, 1.0), &mut cursor), RetainedGlyphStep::Fault);
    assert_eq!(value.as_ptr(), identity);
    assert_eq!(draw.layers.iter().map(|layer| layer.ui_instances.len()).sum::<usize>(), 0);
    assert!(cursor.terminal_is_empty());
}

/// 🧱️ A dynamic chrome group advances through exactly one admitted output or hit per grant.
#[test]
fn dynamic_chrome_group_retains_every_glyph_and_border_opportunity() {
    let value = "x".repeat(32);
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::light();
    let item = ChromeGroupItem { control_id: "retained.group", icon_id: Some("play"), label: Some(value.as_str()), active: false, disabled: false, kind: HitKind::Button };
    let Some(width) = retained_chrome_group_item_width(&theme, &item) else {
        assert!(false);
        return;
    };
    let mut group_phase = 0;
    let mut glyph = RetainedGlyphCursor::default();
    let mut complete = false;
    for _ in 0..64 {
        let before = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum::<usize>();
        let step = render_retained_chrome_group_item_step(&mut group_phase, &mut glyph, &mut draw, &mut atlas, &icons, &mut input, &theme, Rect::new(0.0, 0.0, width, theme.control_height), &item, true);
        let after = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum::<usize>();
        assert!(after.saturating_sub(before) <= 1);
        if step == RetainedChromeGroupStep::Complete {
            complete = true;
            break;
        }
    }
    assert!(complete);
}

/// 🚫️ A MAX + 1 chrome label is rejected before a retained owner or output is staged.
#[test]
fn dynamic_chrome_group_max_plus_one_is_identity_preserving() {
    let value = "x".repeat(ui_wgpu::wgpu::RETAINED_NODE_TEXT_MAX_BYTES + 1);
    let identity = value.as_ptr();
    let theme = Theme::light();
    let item = ChromeGroupItem { control_id: "retained.group", icon_id: None, label: Some(value.as_str()), active: false, disabled: false, kind: HitKind::Button };
    assert!(retained_chrome_group_item_width(&theme, &item).is_none());
    assert_eq!(value.as_ptr(), identity);
}
