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
    let Some(width) = retained_chrome_group_item_width(&mut atlas, &theme, &item) else {
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
    let mut atlas = FontAtlas::builtin();
    let theme = Theme::light();
    let item = ChromeGroupItem { control_id: "retained.group", icon_id: None, label: Some(value.as_str()), active: false, disabled: false, kind: HitKind::Button };
    assert!(retained_chrome_group_item_width(&mut atlas, &theme, &item).is_none());
    assert_eq!(value.as_ptr(), identity);
}

//#region 📏️ChipMeasurePaintLaws
/// 📏️ Every chrome label the OS shell paints into a chip: the navbar/footer panel-tab and shortcut
/// labels of the first real wgpu boot (all of which wrapped mid-word — "Concrete F/orest",
/// "Edit/or", "Transf/orm"), both framework branch label sets, and both locales of every chrome
/// string a chip carries. The set is the regression evidence, so a label that once wrapped can
/// never silently start wrapping again.
fn chrome_chip_label_set() -> Vec<String> {
    let mut labels: Vec<String> = [
        "Concrete Forest",
        "Editor",
        "Viewer",
        "Catalogue",
        "Artifact",
        "Chat",
        "Tool runs",
        "Inspection",
        "Fullscreen",
        "Transform",
        "Brush",
        "Volume Brush",
        "Relocate",
        "File",
        "Folder",
        "Remote",
        "Tool",
        "Command",
        "Settings",
        "Marketplace",
        "History",
        "Display",
        "Windows",
        "Layout",
        "Puzzle 3D",
        "Top",
        "Perspective",
        "Remote: detached",
        "No one else is here",
    ]
    .into_iter()
    .map(ToOwned::to_owned)
    .collect();
    for is_de in [false, true] {
        labels.extend(ShellState::framework_display_tabs(is_de).into_iter().map(|tab| tab.label));
        for key in ["settings.tab.general", "settings.tab.theme", "settings.tab.keybindings", "settings.tab.defaultApps", "conflict.panel", "display.tab.windows", "display.tab.layout", "panelToggle.display", "panelToggle.sync", "panelToggle.tool", "panelToggle.command", "panelToggle.settings", "panelToggle.chat", "marketplace.tab", "fullscreen.toggle", "fullscreen.exit", "common.close", "common.cancel"] {
            labels.push(shell_chrome_string(key, is_de).to_string());
        }
    }
    labels.sort();
    labels.dedup();
    labels
}

/// 📐️ The label box one chip grants its own run — byte-identical to the arithmetic
/// `render_retained_chrome_group_item_step`'s label phase performs.
fn chip_label_box(theme: &Theme, chip: Rect, item: &ChromeGroupItem<'_>) -> (f32, f32) {
    let icon_w = item.icon_id.map(|_| CHROME_ICON_TINY + theme.gap_standard).unwrap_or(0.0);
    let x = chip.x + theme.padding_standard + icon_w;
    (x, (chip.x + chip.w - theme.padding_standard - x).max(1.0))
}

/// 📏️ **The chip law.** For every shell chip label, at the default chrome font, the box
/// `retained_chrome_group_item_width` lays out is wide enough for the advances the painter actually
/// pens: painting the label into that box WRAPPING keeps every glyph on line 0, and painting it
/// CLIPPING (which is what chrome does) drops no glyph at all.
///
/// This is the law the first real wgpu boot broke: the width priced a label at
/// `len × font_size_small × 0.6` while the painter walked the atlas's own advances, so the retained
/// painter broke every chip label onto a second line inside a 22px chip.
#[test]
fn every_chip_lays_out_at_least_its_labels_painted_width() {
    let theme = Theme::light();
    let color = Rgba::new(1.0, 1.0, 1.0, 1.0);
    for mut atlas in [FontAtlas::builtin(), FontAtlas::shaped_default()] {
        for label in chrome_chip_label_set() {
            for icon_id in [None, Some("play")] {
                let item = ChromeGroupItem { control_id: "chip.law", icon_id, label: Some(label.as_str()), active: false, disabled: false, kind: HitKind::Button };
                let width = retained_chrome_group_item_width(&mut atlas, &theme, &item).expect("a chrome label is far below the retained text ceiling");
                let chip = Rect::new(0.0, 0.0, width, theme.control_height);
                let (x, box_w) = chip_label_box(&theme, chip, &item);
                let measured = atlas.measure_text(label.as_str(), theme.font_size_small).0;
                assert!(box_w + 0.01 >= measured, "📏️ chip for {label:?} grants {box_w} px to a label that paints {measured} px wide");

                let mut clipped = DrawList::default();
                let mut cursor = RetainedGlyphCursor::default();
                for _ in 0..=label.chars().count() {
                    if chrome_text_step(&mut clipped, &mut atlas, label.as_str(), x, theme.font_size_small, box_w, theme.font_size_small, color, &mut cursor) == RetainedGlyphStep::Complete {
                        break;
                    }
                }
                let painted = clipped.layers.iter().map(|layer| layer.ui_instances.len()).sum::<usize>();
                assert_eq!(painted, label.chars().count(), "✂️ chrome clips {} of {label:?}'s glyphs out of its own chip", label.chars().count() - painted);

                let mut wrapped = DrawList::default();
                let mut cursor = RetainedGlyphCursor::default();
                for _ in 0..=label.chars().count() {
                    if ui_wgpu::wgpu::paint_retained_glyph_step(label.as_str(), Rect::new(x, 0.0, box_w, theme.font_size_small), theme.font_size_small, color, &mut atlas, &mut wrapped, &mut cursor) == RetainedGlyphStep::Complete {
                        break;
                    }
                }
                let baselines: Vec<f32> = wrapped.layers.iter().flat_map(|layer| layer.ui_instances.iter().map(|instance| instance.rect[1])).collect();
                let lowest = baselines.iter().copied().fold(f32::NEG_INFINITY, f32::max);
                let highest = baselines.iter().copied().fold(f32::INFINITY, f32::min);
                assert!(lowest - highest < ui_wgpu::wgpu::text::line_height(theme.font_size_small), "↩️ {label:?} wrapped onto a second line inside its own chip");
            }
        }
    }
}

/// ✂️ Chrome text NEVER wraps, whatever box it is handed: React's chip labels are
/// `whitespace-nowrap truncate`, so an under-sized box loses the label's tail on ONE line rather
/// than growing a second line the 22px chip has no room for.
#[test]
fn chrome_text_clips_instead_of_wrapping_a_narrow_box() {
    let theme = Theme::light();
    let color = Rgba::new(1.0, 1.0, 1.0, 1.0);
    let mut atlas = FontAtlas::builtin();
    let label = "Volume Brush";
    let narrow = atlas.measure_text(label, theme.font_size_small).0 * 0.5;
    let mut draw = DrawList::default();
    let mut cursor = RetainedGlyphCursor::default();
    for _ in 0..=label.chars().count() {
        if chrome_text_step(&mut draw, &mut atlas, label, 0.0, theme.font_size_small, narrow, theme.font_size_small, color, &mut cursor) == RetainedGlyphStep::Complete {
            break;
        }
    }
    let rows: Vec<f32> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter().map(|instance| instance.rect[1])).collect();
    assert!(!rows.is_empty(), "a clipped run still paints the glyphs that do fit");
    assert!(rows.iter().all(|row| (row - rows[0]).abs() < f32::EPSILON), "✂️ clipped chrome text stays on one line");
    assert!(rows.len() < label.chars().count(), "✂️ a half-width box cannot fit every glyph");
}

/// 📝️ The measured advance of a label IS the advance the painter pens: stepping the retained
/// painter across a run and reading how far the pen travelled must reproduce
/// `FontAtlas::measure_text` exactly, on both the shaped UI face and the fixed-pitch fallback.
#[test]
fn measured_label_advance_equals_the_painted_advance() {
    let theme = Theme::light();
    let color = Rgba::new(1.0, 1.0, 1.0, 1.0);
    for mut atlas in [FontAtlas::builtin(), FontAtlas::shaped_default()] {
        for label in ["Concrete Forest", "Volume Brush", "Fullscreen", "Chat", "Remote: detached"] {
            let measured = atlas.measure_text(label, theme.font_size_small).0;
            let mut draw = DrawList::default();
            let mut cursor = RetainedGlyphCursor::default();
            for _ in 0..=label.chars().count() {
                if chrome_text_step(&mut draw, &mut atlas, label, 0.0, theme.font_size_small, measured, theme.font_size_small, color, &mut cursor) == RetainedGlyphStep::Complete {
                    break;
                }
            }
            let instances: Vec<[f32; 4]> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter().map(|instance| instance.rect)).collect();
            assert_eq!(instances.len(), label.chars().count(), "📝️ {label:?} paints one glyph per scalar inside its measured width");
            let last = instances.last().copied().expect("a non-empty label paints at least one glyph");
            assert!(last[0] + last[2] <= measured + theme.font_size_small, "📝️ {label:?}'s last glyph ends inside its measured advance");
        }
    }
}
//#endregion 📏️ChipMeasurePaintLaws
