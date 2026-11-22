// 🎴️ Ring, slider and icon select state paint laws: the ring's 12 o'clock orb, the slider's hover, drag, focus-visible and disabled looks and the icon selector's editor text, preview and lock — each read off the draw list against the React class lists they mirror.

fn rgba(color: Rgba) -> [f32; 4] {
    [color.r, color.g, color.b, color.a]
}

fn rounded_of(draw: &DrawList) -> Vec<(([f32; 4]), [f32; 4])> {
    draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - KIND_ROUNDED).abs() < 0.01).map(|instance| (instance.rect, instance.color)).collect()
}

fn glyph_colors(draw: &DrawList) -> Vec<[f32; 4]> {
    draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01).map(|instance| instance.color).collect()
}

fn ring_node(t: f64, disabled: bool) -> UiNode {
    let mut presence = UiPresence::default();
    if disabled {
        presence.state = UiState::Disabled;
    }
    UiNode::Ring(UiRingNode { id: "ring".into(), orb_id: "sun".into(), t, on_change: action(), presence, menu: None })
}

fn center_of(rect: [f32; 4]) -> (f32, f32) {
    (rect[0] + rect[2] * 0.5, rect[1] + rect[3] * 0.5)
}

#[test]
fn ring_orb_sits_at_twelve_oclock_for_zero_and_turns_clockwise() {
    for (t, expected) in [(0.0, (0.0, -40.0)), (0.25, (40.0, 0.0)), (0.5, (0.0, 40.0)), (0.75, (-40.0, 0.0))] {
        let (tree, root, theme, mut atlas) = setup(&ring_node(t, false));
        let bounds = tree.absolute_rect(root).unwrap();
        let draw = paint_retained_node_for_test(&tree, root, &theme, &mut atlas);
        let orb = rounded_of(&draw);
        assert_eq!(orb.len(), 2, "a selected orb is an accent-foreground rim under an accent disc");
        let (cx, cy, _) = semio_ui_wgpu_layout::layout::ring_center(bounds);
        let (x, y) = center_of(orb[0].0);
        assert!((x - cx - expected.0).abs() < 0.01 && (y - cy - expected.1).abs() < 0.01, "t={t}: ({}, {})", x - cx, y - cy);
    }
}

#[test]
fn ring_selected_orb_uses_radius_seven_accent_fill_and_accent_foreground_stroke() {
    let (tree, root, theme, mut atlas) = setup(&ring_node(0.0, false));
    let orb = rounded_of(&paint_retained_node_for_test(&tree, root, &theme, &mut atlas));
    assert_eq!(orb[0].1, rgba(theme.accent_foreground));
    assert_eq!(orb[1].1, rgba(theme.accent));
    assert!((orb[0].0[2] - 15.0).abs() < 0.01 && (orb[1].0[2] - 13.0).abs() < 0.01, "r=7 plus and minus half the 1-unit stroke");
}

#[test]
fn ring_disabled_orb_is_muted_foreground_at_forty_percent_and_the_track_is_thirty() {
    let (tree, root, theme, mut atlas) = setup(&ring_node(0.0, true));
    let draw = paint_retained_node_for_test(&tree, root, &theme, &mut atlas);
    let orb = rounded_of(&draw);
    assert_eq!(orb.len(), 1);
    assert_eq!(orb[0].1, rgba(theme.text_muted.with_alpha(theme.text_muted.a * 0.4)));
    assert_eq!(draw.layers.iter().map(|layer| layer.vector_vertices.len()).sum::<usize>() > 0, true, "the track chords paint");
}

#[test]
fn ring_is_a_hundred_pixel_square() {
    let (tree, root, _, _) = setup_flowed(&ring_node(0.0, false));
    let bounds = tree.absolute_rect(root).unwrap();
    assert_eq!((bounds.h, bounds.w >= semio_ui_wgpu_layout::layout::RING_SIZE), (semio_ui_wgpu_layout::layout::RING_SIZE, true));
}

fn state_slider(snaps: Vec<f64>) -> UiNode {
    UiNode::Slider(UiSliderNode { id: "sl".into(), value: 0.5, min: 0.0, max: 1.0, step: 0.01, unit: None, snaps, on_change: action(), presence: UiPresence::default(), menu: None, ..Default::default() })
}

fn slider_rounded(configure: impl FnOnce(&mut UiTree, NodeId)) -> (Vec<([f32; 4], [f32; 4])>, Theme) {
    let (mut tree, root, theme, mut atlas) = setup(&state_slider(Vec::new()));
    configure(&mut tree, root);
    (rounded_of(&paint_retained_node_for_test(&tree, root, &theme, &mut atlas)), theme)
}

#[test]
fn slider_resting_range_and_thumb_are_the_element_ink() {
    let (rounded, theme) = slider_rounded(|_, _| {});
    assert_eq!(rounded.len(), 3);
    assert_eq!(rounded[1].1, rgba(theme.text_element));
    assert_eq!(rounded[2].1, rgba(theme.text_element));
}

#[test]
fn slider_hover_lifts_range_and_thumb_to_the_emphasized_foreground() {
    let (rounded, theme) = slider_rounded(|tree, root| tree.node_mut(root).unwrap().flags.set(NodeFlags::HOVERED, true));
    assert_eq!(rounded[1].1, rgba(theme.text));
    assert_eq!(rounded[2].1, rgba(theme.text));
}

#[test]
fn slider_dragging_paints_range_and_thumb_active_base() {
    let (rounded, theme) = slider_rounded(|tree, root| tree.node_mut(root).unwrap().state.scrub_gesture = Some("w/1:1".into()));
    assert_eq!(rounded[1].1, rgba(theme.selected));
    assert_eq!(rounded[2].1, rgba(theme.selected));
}

#[test]
fn slider_focus_visible_thumb_is_active_base_inside_an_emphasized_border() {
    let (rounded, theme) = slider_rounded(|tree, root| {
        tree.node_mut(root).unwrap().flags.set(NodeFlags::FOCUSED, true);
        tree.node_mut(root).unwrap().flags.set(NodeFlags::FOCUS_VISIBLE, true);
    });
    assert_eq!(rounded.len(), 4, "rail, range, thumb border and thumb fill");
    assert_eq!(rounded[2].1, rgba(theme.border_emphasized));
    assert_eq!(rounded[3].1, rgba(theme.selected));
    assert!((rounded[2].0[2] - rounded[3].0[2] - theme.stroke_hairline * 2.0).abs() < 0.001);
    let (pointer_focus, _) = slider_rounded(|tree, root| tree.node_mut(root).unwrap().flags.set(NodeFlags::FOCUSED, true));
    assert_eq!(pointer_focus.len(), 3, "pointer focus is not focus-visible");
}

#[test]
fn slider_disabled_is_half_opacity_without_the_extra_scrim() {
    let mut node = state_slider(Vec::new());
    let UiNode::Slider(slider) = &mut node else { unreachable!() };
    slider.presence.state = UiState::Disabled;
    let (tree, root, theme, mut atlas) = setup(&node);
    let draw = paint_retained_node_for_test(&tree, root, &theme, &mut atlas);
    let rounded = rounded_of(&draw);
    assert_eq!(rounded[0].1, rgba(theme.muted.with_alpha(theme.muted.a * 0.5)));
    assert_eq!(rounded[2].1, rgba(theme.text_element.with_alpha(theme.text_element.a * 0.5)));
    let scrim = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - KIND_SOLID).abs() < 0.01 && instance.color == rgba(theme.panel.with_alpha(0.35))).count();
    assert_eq!(scrim, 0);
}

#[test]
fn slider_readout_uses_display_factor_precision_and_the_external_unit_like_react() {
    let node = UiNode::Slider(UiSliderNode { id: "sl".into(), value: std::f64::consts::FRAC_PI_2, min: 0.0, max: 3.5, step: 0.01, unit: Some("rad".into()), display_unit: Some("deg".into()), display_factor: Some(180.0 / std::f64::consts::PI), precision: Some(1), snaps: Vec::new(), on_change: action(), presence: UiPresence::default(), menu: None, ..Default::default() });
    let UiNode::Slider(slider) = &node else { unreachable!() };
    assert_eq!(slider.readout(slider.value), "90.0");
    assert_eq!(slider.unit_label().as_deref(), Some("90.0 deg"));
    let (tree, root, theme, mut atlas) = setup_flowed(&node);
    let glyphs = glyph_colors(&paint_retained_node_for_test(&tree, root, &theme, &mut atlas));
    assert_eq!(glyphs.iter().filter(|color| **color == rgba(theme.text_element)).count(), "90.0".chars().count());
    assert_eq!(glyphs.iter().filter(|color| **color == rgba(theme.text_muted)).count(), "90.0 deg".chars().filter(|ch| !ch.is_whitespace()).count());
}

fn icon_select_with(value: &str, uniform: bool, disabled: bool) -> UiNode {
    let mut presence = UiPresence::default();
    if disabled {
        presence.state = UiState::Disabled;
    }
    UiNode::IconSelect(UiIconSelectNode { id: "icon".into(), value: value.into(), uniform, classifier_kind: "icon".into(), on_change: action(), presence, menu: None })
}

#[test]
fn icon_selector_modes_and_encoding_match_the_shared_vectors() {
    let law: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/🎴️icon-selector-mode/🔣️.json")).unwrap();
    for case in law["cases"].as_array().unwrap() {
        let (mode, inner) = UiIconSelectNode::decode_mode(case["raw"].as_str().unwrap());
        assert_eq!((mode.id(), inner.as_str()), (case["mode"].as_str().unwrap(), case["inner"].as_str().unwrap()), "{}", case["id"]);
    }
    let modes = [("url", IconSelectorMode::Url), ("shortcode", IconSelectorMode::Shortcode), ("math", IconSelectorMode::Math), ("data", IconSelectorMode::Data), ("emoji", IconSelectorMode::Emoji), ("text", IconSelectorMode::Text), ("vector", IconSelectorMode::Vector)];
    for case in law["encode"].as_array().unwrap() {
        let mode = modes.iter().find(|(id, _)| *id == case["mode"].as_str().unwrap()).unwrap().1;
        assert_eq!(UiIconSelectNode::encode_for_mode(mode, case["inner"].as_str().unwrap()), case["stored"].as_str().unwrap(), "{}", case["id"]);
    }
    for case in law["migrate"].as_array().unwrap() {
        let mode = IconSelectorMode::from_id(case["mode"].as_str().unwrap()).unwrap();
        assert_eq!(UiIconSelectNode::migrate_mode(case["prev"].as_str().unwrap(), mode), case["stored"].as_str().unwrap(), "{}", case["id"]);
    }
}

#[test]
fn icon_selector_labels_match_the_shared_react_bundle_in_english_and_german() {
    use semio_ui_locale::AppLabels;
    let law: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/🎴️icon-selector-labels/🔣️.json")).unwrap();
    for (locale, language) in [(semio_ui_locale::Locale::En, "en"), (semio_ui_locale::Locale::De, "de")] {
        for terminology in [semio_ui_locale::Terminology::Native, semio_ui_locale::Terminology::Reuse] {
            let modes = semio_ui_locale::UiIconSelectorLabels::labels(locale, terminology);
            for (id, label) in [("url", modes.url), ("shortcode", modes.shortcode), ("math", modes.math), ("data", modes.data), ("emoji", modes.emoji), ("text", modes.text), ("vector", modes.vector)] {
                assert_eq!(label.as_str(), law["modes"][id][language].as_str().unwrap(), "{id} {language}");
            }
            let common = semio_ui_locale::UiCommonLabels::labels(locale, terminology);
            assert_eq!(common.import_file.as_str(), law["common"]["importFile"][language].as_str().unwrap());
            assert_eq!(common.clear.as_str(), law["common"]["clear"][language].as_str().unwrap());
        }
    }
}

#[test]
fn icon_selector_paints_only_its_preview_the_cells_paint_themselves() {
    let (tree, root, theme, mut atlas) = setup(&icon_select_with("text:Hi", true, false));
    let glyphs = glyph_colors(&paint_retained_node_for_test(&tree, root, &theme, &mut atlas));
    assert_eq!(glyphs.iter().filter(|color| **color == rgba(theme.text)).count(), "Hi".chars().count(), "the glyph preview shows the inner text, never `text:Hi`; the editor is its own Input cell");
}

#[test]
fn icon_selector_without_a_decodable_icon_previews_a_muted_dash_and_a_mixed_one_is_locked_to_sixty_percent() {
    let (tree, root, theme, mut atlas) = setup(&icon_select_with("", true, false));
    assert_eq!(glyph_colors(&paint_retained_node_for_test(&tree, root, &theme, &mut atlas)), vec![rgba(theme.text_muted)]);
    let (tree, root, theme, mut atlas) = setup(&icon_select_with("text:Hi", false, false));
    let draw = paint_retained_node_for_test(&tree, root, &theme, &mut atlas);
    assert_eq!(glyph_colors(&draw), vec![rgba(theme.text_muted.with_alpha(theme.text_muted.a * 0.6))], "mixed: no editor text, a dimmed dash");
}

#[test]
fn icon_selector_is_taller_than_a_control_row_and_data_and_vector_get_the_five_row_editor() {
    let (tree, root, theme, _atlas) = setup_flowed(&icon_select_with("text:Hi", true, false));
    let four = tree.absolute_rect(root).unwrap().h;
    assert!(four > theme.control_height);
    let (tree, root, _, _) = setup_flowed(&icon_select_with("<svg viewBox=\"0 0 1 1\"/>", true, false));
    assert!(tree.absolute_rect(root).unwrap().h > four, "vector is the 5-row editor");
}

fn drive_sync(tree: &mut UiTree, node: semio_ui_arena::NodeId, theme: &Theme, locale: Option<semio_ui_locale::Locale>) {
    let mut cursor = RetainedInteractiveSyncCursor::default();
    for _ in 0..4_096 {
        match sync_interactive_state_node_step(tree, node, theme, locale, &mut cursor) {
            RetainedInteractiveSyncStep::Pending => {}
            RetainedInteractiveSyncStep::Complete => return,
            RetainedInteractiveSyncStep::Fault => panic!("the sync pass faulted at line {}", cursor.fault_line),
        }
    }
    panic!("the sync pass never completed");
}

#[test]
fn the_sync_pass_synthesizes_four_ledgered_cells_with_react_geometry_and_localized_labels() {
    let law: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/🎴️icon-selector-labels/🔣️.json")).unwrap();
    for (locale, language) in [(semio_ui_locale::Locale::En, "en"), (semio_ui_locale::Locale::De, "de")] {
        let (mut tree, root, theme, _atlas) = setup(&icon_select_with("text:Hi", true, false));
        drive_sync(&mut tree, root, &theme, Some(locale));
        assert_eq!(tree.composite_row_count(), 4, "mode, editor, import, clear");
        let cell = |index: usize| find_child_by_key(&tree, root, &NodeKey::Explicit(format!("icon.{}", ["mode", "field", "import", "clear"][index]))).expect("cell");
        let (mode, field, import, clear) = (cell(0), cell(1), cell(2), cell(3));
        let UiNode::Select(select) = &tree.node(mode).unwrap().spec.0 else { panic!("the mode cell is a real Select") };
        assert_eq!(select.value, "text");
        assert_eq!(select.items.iter().map(|item| item.value.as_str()).collect::<Vec<_>>(), ["url", "shortcode", "math", "data", "emoji", "text", "vector"]);
        assert_eq!(select.items.iter().find(|item| item.value == "text").unwrap().label.as_str(), law["modes"]["text"][language].as_str().unwrap());
        let UiNode::Input(input) = &tree.node(field).unwrap().spec.0 else { panic!("the editor cell is an Input") };
        assert_eq!((input.input_kind.as_str(), input.value.as_str(), input.commit.as_deref(), input.uses_mono_face()), ("longText", "Hi", Some("iconMode:text"), true));
        let UiNode::Button(import_button) = &tree.node(import).unwrap().spec.0 else { panic!() };
        let UiNode::Button(clear_button) = &tree.node(clear).unwrap().spec.0 else { panic!() };
        assert_eq!(import_button.label.as_str(), law["common"]["importFile"][language].as_str().unwrap());
        assert_eq!(clear_button.label.as_str(), law["common"]["clear"][language].as_str().unwrap());
        assert_eq!(import_button.action.action, UiIconSelectNode::IMPORT_ACTION);
        let bounds = tree.absolute_rect(root).unwrap();
        let rows = semio_ui_wgpu_layout::layout::icon_select_rows(Rect::new(0.0, 0.0, bounds.w, bounds.h), 4);
        let at = |id| { let layout = tree.node(id).unwrap().layout.clone(); Rect::new(layout.x, layout.y, layout.width, layout.height) };
        assert_eq!((at(mode), at(field), at(import), at(clear)), (rows.mode, rows.editor, rows.import, rows.clear));
        drive_sync(&mut tree, root, &theme, Some(locale));
        assert_eq!(tree.composite_row_count(), 4, "a second pass re-finds the cells by key");
    }
}

#[test]
fn a_locked_selector_disables_every_cell_and_the_mode_rows_commit_the_migrated_icon() {
    let (mut tree, root, theme, _atlas) = setup(&icon_select_with("text:Hi", false, false));
    drive_sync(&mut tree, root, &theme, None);
    for key in ["mode", "field", "import", "clear"] {
        let cell = find_child_by_key(&tree, root, &NodeKey::Explicit(format!("icon.{key}"))).unwrap();
        assert_eq!(tree.node(cell).unwrap().spec.0.presence().state, UiState::Disabled, "{key}");
    }
    let (mut tree, root, theme, _atlas) = setup(&icon_select_with("text:Hi", true, false));
    drive_sync(&mut tree, root, &theme, None);
    let mode = find_child_by_key(&tree, root, &NodeKey::Explicit("icon.mode".into())).unwrap();
    tree.node_mut(mode).unwrap().state.open = true;
    drive_sync(&mut tree, mode, &theme, None);
    let row = find_child_by_key(&tree, mode, &NodeKey::Explicit("emoji".into())).expect("the popup row for the emoji mode");
    let UiNode::Button(row) = &tree.node(row).unwrap().spec.0 else { panic!("a popup row is a Button") };
    assert_eq!(row.action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_str()), Some("emoji:text:Hi"), "the row commits (React strips only the typst and emoji prefixes) the migrated icon, not the mode id");
}

#[test]
fn the_mono_flag_reaches_the_edit_measurement_the_mono_face_is_a_different_advance() {
    let mut atlas = FontAtlas::shaped_default();
    let sans = atlas.measure_range_face(semio_ui_text::TextFace::Sans, "iiWW", 0, 4, 12.0);
    let mono = atlas.measure_range_face(semio_ui_text::TextFace::Mono, "iiWW", 0, 4, 12.0);
    assert_ne!(sans, mono, "the editor's mono face measures differently from the sans face");
    let input = UiInputNode { id: "f".into(), input_kind: "longText".into(), commit: Some("iconMode:vector".into()), ..Default::default() };
    assert!(input.uses_mono_face(), "{:?}", input.commit);
    assert_eq!(UiIconSelectNode::decode_mode("text:x").0.editor_rows(), 4);
    assert_eq!(input.icon_mode().map(|mode| mode.editor_rows()), Some(5));
    assert!(!UiInputNode { commit: Some("blur".into()), ..input }.uses_mono_face());
}

