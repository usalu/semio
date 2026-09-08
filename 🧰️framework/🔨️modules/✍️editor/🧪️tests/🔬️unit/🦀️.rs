
use super::*;

#[test]
fn insert_space_inside_token_inserts_without_replacing() {
    let mut host = EditorHost::new();
    host.set_text("MATCH".into());
    host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
    host.set_caret_anchor(3);
    host.insert_text(" ");
    assert_eq!(host.text(), "MAT CH");
    assert_eq!(host.caret(), 4);
}

#[test]
fn insert_space_at_token_end_appends() {
    let mut host = EditorHost::new();
    host.set_text("MATCH".into());
    host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
    host.set_caret_anchor(5);
    host.insert_text(" ");
    assert_eq!(host.text(), "MATCH ");
    assert_eq!(host.caret(), 6);
}

#[test]
fn auto_space_before_next_token_at_token_end() {
    let mut host = EditorHost::new();
    host.set_text("MATCH".into());
    host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
    host.set_caret_anchor(5);
    host.insert_text("(");
    assert_eq!(host.text(), "MATCH (");
}

#[test]
fn insert_and_caret() {
    let mut host = EditorHost::new();
    host.insert_text("MATCH");
    assert_eq!(host.text(), "MATCH");
    assert_eq!(host.caret(), 5);
}

#[test]
fn sync_from_scene_json_sets_and_clears_hover_range() {
    let mut host = EditorHost::new();
    host.sync_from_scene_json(r#"{"buffer":"abc","hoverJson":"{\"start\":1,\"end\":2}"}"#).unwrap();
    assert_eq!(host.hover_token_range(), Some((1, 2)));
    host.sync_from_scene_json(r#"{"buffer":"abc","hoverJson":"null"}"#).unwrap();
    assert_eq!(host.hover_token_range(), None);
}

#[test]
fn theme_merge_from_json_updates_clear() {
    let mut host = EditorHost::new();
    let json = r#"{"rasterClear":[240,236,221,255],"labelFill":[0,17,23,255]}"#;
    host.set_canvas_theme_from_json(json).expect("theme json");
    let _scene = host.build_scene();
    assert!(host.set_canvas_theme_from_json(json).is_ok());
}

#[test]
fn select_all_sets_range() {
    let mut host = EditorHost::new();
    host.set_text("abc".into());
    host.select_all();
    assert_eq!(host.anchor(), 0);
    assert_eq!(host.caret(), 3);
}

#[test]
fn selection_snaps_var_label_composite() {
    let mut host = EditorHost::new();
    host.set_text("MATCH (a1:Piece)".into());
    host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"},{"start":7,"end":9,"class":"ident"},{"start":9,"end":10,"class":"operator"},{"start":10,"end":15,"class":"ident"}]"#);
    host.set_selectable_spans_json(r#"[{"start":7,"end":9,"kind":"atomic"},{"start":7,"end":15,"kind":"varLabel","headEnd":9},{"start":10,"end":15,"kind":"atomic"}]"#);
    host.set_selection(8, 12);
    assert_eq!(host.anchor(), 7);
    assert_eq!(host.caret(), 15);
}

#[test]
fn select_span_at_picks_ident() {
    let mut host = EditorHost::new();
    host.set_text("RETURN a1.name".into());
    host.set_semantic_tokens_json(r#"[{"start":0,"end":6,"class":"keyword"},{"start":7,"end":9,"class":"ident"},{"start":9,"end":10,"class":"operator"},{"start":10,"end":14,"class":"ident"}]"#);
    host.set_selectable_spans_json(r#"[{"start":7,"end":9,"kind":"atomic"},{"start":10,"end":14,"kind":"atomic"},{"start":7,"end":14,"kind":"propertyAccess","headEnd":9,"tailStart":10}]"#);
    host.select_span_at(11);
    assert_eq!(host.anchor(), 10);
    assert_eq!(host.caret(), 14);
}

#[test]
fn selection_snaps_fixed_keywords() {
    let mut host = EditorHost::new();
    host.set_text("MATCH x".into());
    host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
    host.set_selection(2, 4);
    assert_eq!(host.anchor(), 0);
    assert_eq!(host.caret(), 5);
}

#[test]
fn dead_line_ignores_wheel_when_content_fits() {
    let mut host = EditorHost::new();
    host.set_size(400, 300, 1.0);
    host.set_text("line one\nline two".into());
    host.set_dead_line_y(32.0);
    assert!(!host.scroll_overflows());
    host.wheel_scroll_screen(-40.0);
    assert!(!host.chrome_edgeless_scroll());
}

#[test]
fn dead_line_toggles_edgeless_and_restores_on_scroll_back() {
    let mut host = EditorHost::new();
    host.set_size(400, 120, 1.0);
    let lines: String = (0..20).map(|i| format!("line {i}")).collect::<Vec<_>>().join("\n");
    host.set_text(lines);
    host.set_dead_line_y(32.0);
    assert!(host.scroll_overflows());
    let screen_at_rest: serde_json::Value = serde_json::from_str(&host.world_to_screen_json(offset_to_world(&host, 0).0, offset_to_world(&host, 0).1)).unwrap();
    assert!(screen_at_rest["y"].as_f64().unwrap() >= 32.0);
    host.wheel_scroll_screen(-40.0);
    assert!(host.chrome_edgeless_scroll());
    let screen_edgeless: serde_json::Value = serde_json::from_str(&host.world_to_screen_json(offset_to_world(&host, 0).0, offset_to_world(&host, 0).1)).unwrap();
    assert!(screen_edgeless["y"].as_f64().unwrap() < screen_at_rest["y"].as_f64().unwrap());
    host.wheel_scroll_screen(40.0);
    assert!(!host.chrome_edgeless_scroll());
    let screen_restored: serde_json::Value = serde_json::from_str(&host.world_to_screen_json(offset_to_world(&host, 0).0, offset_to_world(&host, 0).1)).unwrap();
    assert!((screen_restored["y"].as_f64().unwrap() - screen_at_rest["y"].as_f64().unwrap()).abs() < 0.5);
}

#[test]
fn editor_viewport_maps_text_to_top_left() {
    let mut host = EditorHost::new();
    host.set_size(400, 300, 1.0);
    host.set_text("hello".into());
    let (wx, wy) = offset_to_world(&host, 0);
    let screen: serde_json::Value = serde_json::from_str(&host.world_to_screen_json(wx, wy)).unwrap();
    let sx = screen["x"].as_f64().unwrap();
    let sy = screen["y"].as_f64().unwrap();
    assert!(sx > 50.0 && sx < 90.0);
    assert!(sy > PAD_Y && sy < PAD_Y + DEFAULT_LINE_HEIGHT);
}

#[test]
fn drag_select_extends_range() {
    let mut host = EditorHost::new();
    host.set_size(800, 600, 1.0);
    host.set_text("hello world".into());
    host.pointer_down_screen(68.0, 24.5, 0);
    host.pointer_move_screen(250.0, 24.5, 1);
    host.pointer_up_screen(250.0, 24.5, 0);
    assert_ne!(host.caret(), host.anchor());
    assert_eq!(host.anchor(), 0);
    assert!(host.caret() > host.anchor());
}

#[test]
fn punctuated_token_line_builds_scene() {
    let mut host = EditorHost::new();
    host.set_text("MATCH (a:Piece)".into());
    host.set_semantic_tokens_json(
        r#"[{"start":0,"end":5,"class":"keyword"},{"start":5,"end":6,"class":"operator"},{"start":6,"end":7,"class":"operator"},{"start":7,"end":8,"class":"plain"},{"start":8,"end":9,"class":"operator"},{"start":9,"end":14,"class":"plain"}]"#,
    );
    let _scene = host.build_scene();
    assert!(!host.text().is_empty());
}

#[test]
fn build_scene_has_content() {
    let mut host = EditorHost::new();
    host.set_text("MATCH (a:Piece)\nRETURN a.name".into());
    let _scene = host.build_scene();
    assert!(!host.text().is_empty());
}

#[test]
fn backspace_deletes_fixed_keyword_tokenwise() {
    let mut host = EditorHost::new();
    host.set_text("MATCH (a:Piece)".into());
    host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"},{"start":5,"end":6,"class":"operator"}]"#);
    host.set_caret_anchor(3);
    host.backspace();
    assert_eq!(host.text(), " (a:Piece)");
    assert_eq!(host.caret(), 0);
}

#[test]
fn label_span_world_x_matches_scaled_render() {
    let line = "MATCH (a:Piece)";
    let origin = DEFAULT_GUTTER_WIDTH + PAD_X;
    let (x0, x5) = canvas_text::label_span_world_x(line, 0, 5, origin, DEFAULT_FONT_PX);
    let estimate = canvas_text::label_advance("MATCH", DEFAULT_FONT_PX);
    assert!(x5 - x0 < estimate);
    assert!(x5 > x0);
}

#[test]
fn apply_text_edits() {
    let mut host = EditorHost::new();
    host.set_text("abc def".into());
    host.apply_text_edits_json(r#"[{"range":{"start":{"line":0,"character":4},"end":{"line":0,"character":7}},"newText":"xyz"}]"#);
    assert_eq!(host.text(), "abc xyz");
}

#[test]
fn set_dead_line_y_clamps_negative_to_zero() {
    let mut host = EditorHost::new();
    host.set_text("a".into());
    host.set_dead_line_y(-50.0);
    let (_, y) = offset_to_world(&host, 0);
    assert_eq!(y, PAD_Y + DEFAULT_LINE_HEIGHT * 0.75);
}

#[test]
fn set_chrome_edgeless_scroll_toggles_flag() {
    let mut host = EditorHost::new();
    assert!(!host.chrome_edgeless_scroll());
    host.set_chrome_edgeless_scroll(true);
    assert!(host.chrome_edgeless_scroll());
}

#[test]
fn editor_settings_json_clamps_and_updates_flags() {
    let mut host = EditorHost::new();
    host.set_editor_settings_json(r#"{"fontPx":100,"lineHeight":100,"showLineNumbers":false,"tabSize":20}"#);
    assert_eq!(host.font_px, 28.0);
    assert_eq!(host.line_height, 48.0);
    assert!(!host.show_line_numbers);
    assert_eq!(host.tab_size, 8);
    assert_eq!(host.gutter_width(), 0.0);
}

#[test]
fn editor_settings_json_invalid_falls_back_to_defaults() {
    let mut host = EditorHost::new();
    host.set_editor_settings_json("not json");
    assert_eq!(host.font_px, DEFAULT_FONT_PX);
    assert_eq!(host.line_height, DEFAULT_LINE_HEIGHT);
    assert!(host.show_line_numbers);
    assert_eq!(host.tab_size, DEFAULT_TAB_SIZE);
}

#[test]
fn editor_settings_json_clamps_tab_size_minimum() {
    let mut host = EditorHost::new();
    host.set_editor_settings_json(r#"{"tabSize":0}"#);
    assert_eq!(host.tab_insert_text(), " ");
}

#[test]
fn set_selection_range_clamps_to_text_length() {
    let mut host = EditorHost::new();
    host.set_text("abc".into());
    host.set_selection_range(0, 100);
    assert_eq!(host.caret(), 3);
    assert_eq!(host.anchor(), 0);
}

#[test]
fn hover_token_range_reflects_set_hover_range() {
    let mut host = EditorHost::new();
    assert_eq!(host.hover_token_range(), None);
    host.set_hover_range(Some(2), Some(5));
    assert_eq!(host.hover_token_range(), Some((2, 5)));
    host.set_hover_range(None, Some(5));
    assert_eq!(host.hover_token_range(), None);
}

#[test]
fn selection_text_returns_selected_substring() {
    let mut host = EditorHost::new();
    host.set_text("hello world".into());
    host.set_selection(0, 5);
    assert_eq!(host.selection_text(), "hello");
}

#[test]
fn selection_text_empty_when_collapsed() {
    let mut host = EditorHost::new();
    host.set_text("hello".into());
    host.set_caret_anchor(2);
    assert_eq!(host.selection_text(), "");
}

#[test]
fn replace_selection_inserts_when_collapsed() {
    let mut host = EditorHost::new();
    host.set_text("hello".into());
    host.set_caret_anchor(5);
    host.replace_selection(" world");
    assert_eq!(host.text(), "hello world");
}

#[test]
fn replace_selection_replaces_range() {
    let mut host = EditorHost::new();
    host.set_text("hello world".into());
    host.set_selection(0, 5);
    host.replace_selection("bye");
    assert_eq!(host.text(), "bye world");
    assert_eq!(host.caret(), 3);
    assert_eq!(host.anchor(), 3);
}

#[test]
fn set_json_collections_parse_into_fields() {
    let mut host = EditorHost::new();
    host.set_diagnostics_json(r#"[{"start":0,"end":2,"severity":"warning","message":"x"}]"#);
    assert_eq!(host.diagnostics.len(), 1);
    host.set_placeholders_json(r#"[{"offset":0,"label":"?"}]"#);
    assert_eq!(host.placeholders.len(), 1);
    host.set_hover_occurrences_json(r#"[{"start":0,"end":1}]"#);
    assert_eq!(host.hover_occurrences.len(), 1);
    host.set_selection_occurrences_json(r#"[{"start":0,"end":1}]"#);
    assert_eq!(host.selection_occurrences.len(), 1);
    host.set_extra_carets_json(r#"[2,4]"#);
    assert_eq!(host.extra_carets, vec![2, 4]);
}

#[test]
fn set_json_collections_invalid_json_defaults_empty() {
    let mut host = EditorHost::new();
    host.set_diagnostics_json("nope");
    assert!(host.diagnostics.is_empty());
}

#[test]
fn apply_text_edits_multiple_edits_apply_in_reverse_order() {
    let mut host = EditorHost::new();
    host.set_text("one two three".into());
    let json = r#"[
            {"range":{"start":{"line":0,"character":0},"end":{"line":0,"character":3}},"newText":"ONE"},
            {"range":{"start":{"line":0,"character":8},"end":{"line":0,"character":13}},"newText":"THREE"}
        ]"#;
    host.apply_text_edits_json(json);
    assert_eq!(host.text(), "ONE two THREE");
}

#[test]
fn camera_json_reports_y_after_set_camera() {
    let mut host = EditorHost::new();
    host.set_size(400, 100, 1.0);
    host.set_text((0..50).map(|i| format!("line {i}")).collect::<Vec<_>>().join("\n"));
    host.set_camera(0.0, 500.0, 2.0);
    let camera: serde_json::Value = serde_json::from_str(&host.camera_json()).unwrap();
    assert_eq!(camera["x"], 0);
    assert_eq!(camera["zoom"], 1);
    assert!(camera["y"].as_f64().unwrap() > 0.0);
}

#[test]
fn set_size_clamps_to_minimum() {
    let mut host = EditorHost::new();
    host.set_size(0, 0, 0.0);
    assert_eq!(host.viewport.width, 1);
    assert_eq!(host.viewport.height, 1);
    assert_eq!(host.viewport.dpr, 1.0);
}

#[test]
fn sync_from_scene_json_applies_all_optional_fields() {
    let mut host = EditorHost::new();
    let occurrences_inner = serde_json::json!({
        "hover": serde_json::json!([{"start":0,"end":1}]).to_string(),
        "selection": serde_json::json!([{"start":1,"end":2}]).to_string(),
    })
    .to_string();
    let outer = serde_json::json!({
        "buffer": "abc",
        "selectionJson": serde_json::json!({"start":0,"end":2}).to_string(),
        "tokensJson": serde_json::json!([{"start":0,"end":1,"class":"keyword"}]).to_string(),
        "diagnosticsJson": serde_json::json!([{"start":0,"end":1,"severity":"error","message":"x"}]).to_string(),
        "placeholdersJson": serde_json::json!([{"offset":0,"label":"?"}]).to_string(),
        "occurrencesJson": occurrences_inner,
        "extraCaretsJson": serde_json::json!([1,2]).to_string(),
        "selectableSpansJson": serde_json::json!([{"start":0,"end":1,"kind":"atomic"}]).to_string(),
        "settingsJson": serde_json::json!({"fontPx":18}).to_string(),
        "cameraJson": serde_json::json!({"y":5}).to_string(),
        "overlaysJson": serde_json::json!({"deadLineY":10}).to_string(),
    })
    .to_string();
    host.sync_from_scene_json(&outer).unwrap();
    assert_eq!(host.text(), "abc");
    assert_eq!(host.anchor(), 0);
    assert_eq!(host.caret(), 2);
    assert_eq!(host.diagnostics.len(), 1);
    assert_eq!(host.placeholders.len(), 1);
    assert_eq!(host.hover_occurrences.len(), 1);
    assert_eq!(host.selection_occurrences.len(), 1);
    assert_eq!(host.extra_carets, vec![1, 2]);
    assert_eq!(host.selectable_spans.len(), 1);
    assert_eq!(host.font_px, 18.0);
    assert_eq!(host.dead_line_y, 10.0);
}

#[test]
fn wheel_scroll_moves_camera_when_overflowing_without_dead_line() {
    let mut host = EditorHost::new();
    host.set_size(400, 60, 1.0);
    let lines: String = (0..20).map(|i| format!("line {i}")).collect::<Vec<_>>().join("\n");
    host.set_text(lines);
    assert!(host.scroll_overflows());
    host.wheel_scroll_screen(100.0);
    assert!(host.camera.y > 0.0);
}

#[test]
fn wheel_scroll_clamps_camera_to_zero() {
    let mut host = EditorHost::new();
    host.set_size(400, 60, 1.0);
    let lines: String = (0..20).map(|i| format!("line {i}")).collect::<Vec<_>>().join("\n");
    host.set_text(lines);
    host.wheel_scroll_screen(-1000.0);
    assert_eq!(host.camera.y, 0.0);
}

/// 🐛️ W4 fix (`.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w4-scene-input.md`): a right-click
/// used to be a total no-op here (this test used to assert the caret/anchor stayed put) — now it
/// still repositions the caret to the click point (matching left-click, for the right-click
/// context-menu's "open where you clicked" UX) but must never start a drag-selection.
#[test]
fn pointer_down_screen_repositions_caret_for_non_primary_button_but_does_not_start_a_drag_selection() {
    let mut host = EditorHost::new();
    host.set_text("hello world".into());
    host.set_caret_anchor(3);
    let expected_offset = host.hit_test_offset_screen(0.0, 0.0);
    host.pointer_down_screen(0.0, 0.0, 2);
    assert_eq!(host.caret(), expected_offset, "a right-click should still reposition the caret to the click point");
    assert_eq!(host.anchor(), expected_offset);
    assert!(!host.drag_selecting, "a right-click must not start a drag-selection");
}

#[test]
fn pointer_down_screen_primary_button_still_starts_a_drag_selection() {
    let mut host = EditorHost::new();
    host.set_text("hello world".into());
    host.pointer_down_screen(0.0, 0.0, 0);
    assert!(host.drag_selecting, "a primary-button press must still start a drag-selection");
}

#[test]
fn pointer_move_screen_sets_hover_without_drag() {
    let mut host = EditorHost::new();
    host.set_text("MATCH".into());
    host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
    host.pointer_move_screen(70.0, 24.5, 0);
    assert_eq!(host.hover_token_range(), Some((0, 5)));
}

#[test]
fn pointer_up_screen_ignores_non_primary_button() {
    let mut host = EditorHost::new();
    host.set_text("hello".into());
    host.drag_selecting = true;
    host.pointer_up_screen(0.0, 0.0, 2);
    assert!(host.drag_selecting);
}

#[test]
fn insert_text_no_auto_space_for_leading_punctuation() {
    let mut host = EditorHost::new();
    host.set_text("MATCH".into());
    host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
    host.set_caret_anchor(5);
    host.insert_text(":");
    assert_eq!(host.text(), "MATCH:");
}

#[test]
fn insert_text_no_auto_space_without_preceding_token() {
    let mut host = EditorHost::new();
    host.set_text("hello".into());
    host.set_caret_anchor(5);
    host.insert_text("world");
    assert_eq!(host.text(), "helloworld");
}

#[test]
fn insert_text_no_auto_space_when_next_char_is_whitespace() {
    let mut host = EditorHost::new();
    host.set_text("MATCH x".into());
    host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
    host.set_caret_anchor(5);
    host.insert_text("y");
    assert_eq!(host.text(), "MATCHy x");
}

#[test]
fn backspace_deletes_selection_when_not_collapsed() {
    let mut host = EditorHost::new();
    host.set_text("hello world".into());
    host.set_selection(0, 5);
    host.backspace();
    assert_eq!(host.text(), " world");
    assert_eq!(host.caret(), 0);
}

#[test]
fn backspace_at_start_is_noop() {
    let mut host = EditorHost::new();
    host.set_text("hello".into());
    host.set_caret_anchor(0);
    host.backspace();
    assert_eq!(host.text(), "hello");
}

#[test]
fn backspace_removes_single_char_without_token() {
    let mut host = EditorHost::new();
    host.set_text("hello".into());
    host.set_caret_anchor(5);
    host.backspace();
    assert_eq!(host.text(), "hell");
    assert_eq!(host.caret(), 4);
}

#[test]
fn delete_forward_deletes_selection_when_not_collapsed() {
    let mut host = EditorHost::new();
    host.set_text("hello world".into());
    host.set_selection(0, 5);
    host.delete_forward();
    assert_eq!(host.text(), " world");
}

#[test]
fn delete_forward_at_end_is_noop() {
    let mut host = EditorHost::new();
    host.set_text("hello".into());
    host.set_caret_anchor(5);
    host.delete_forward();
    assert_eq!(host.text(), "hello");
}

#[test]
fn delete_forward_removes_token_wholly() {
    let mut host = EditorHost::new();
    host.set_text("MATCH x".into());
    host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
    host.set_caret_anchor(0);
    host.delete_forward();
    assert_eq!(host.text(), " x");
    assert_eq!(host.caret(), 0);
    assert_eq!(host.anchor(), 0);
}

#[test]
fn delete_forward_removes_single_char_without_token() {
    let mut host = EditorHost::new();
    host.set_text("hello".into());
    host.set_caret_anchor(0);
    host.delete_forward();
    assert_eq!(host.text(), "ello");
}

#[test]
fn move_line_start_and_end_navigate_and_extend() {
    let mut host = EditorHost::new();
    host.set_text("hello\nworld".into());
    host.set_caret_anchor(8);
    host.move_line_start(false);
    assert_eq!(host.caret(), 6);
    assert_eq!(host.anchor(), 6);
    host.set_caret_anchor(8);
    host.move_line_end(true);
    assert_eq!(host.caret(), 11);
    assert_eq!(host.anchor(), 8);
}

#[test]
fn move_left_jumps_token_boundary() {
    let mut host = EditorHost::new();
    host.set_text("MATCH x".into());
    host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
    host.set_caret_anchor(3);
    host.move_left(false);
    assert_eq!(host.caret(), 0);
    assert_eq!(host.anchor(), 0);
}

#[test]
fn move_right_jumps_token_boundary_and_extends() {
    let mut host = EditorHost::new();
    host.set_text("MATCH x".into());
    host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
    host.set_caret_anchor(2);
    host.move_right(true);
    assert_eq!(host.caret(), 5);
    assert_eq!(host.anchor(), 2);
}

#[test]
fn move_left_at_start_stays_at_zero() {
    let mut host = EditorHost::new();
    host.set_text("hi".into());
    host.set_caret_anchor(0);
    host.move_left(false);
    assert_eq!(host.caret(), 0);
}

#[test]
fn move_right_at_end_stays_at_end() {
    let mut host = EditorHost::new();
    host.set_text("hi".into());
    host.set_caret_anchor(2);
    host.move_right(false);
    assert_eq!(host.caret(), 2);
}

#[test]
fn move_up_at_first_line_goes_to_zero() {
    let mut host = EditorHost::new();
    host.set_text("hello\nworld".into());
    host.set_caret_anchor(3);
    host.move_up(false);
    assert_eq!(host.caret(), 0);
}

#[test]
fn move_down_clamps_to_last_line() {
    let mut host = EditorHost::new();
    host.set_text("a\nb".into());
    host.set_caret_anchor(0);
    host.move_down(false);
    assert_eq!(host.caret(), 2);
    host.move_down(false);
    assert_eq!(host.caret(), 2);
}

#[test]
fn caret_world_json_reports_position() {
    let mut host = EditorHost::new();
    host.set_text("hi".into());
    host.set_caret_anchor(2);
    let v: serde_json::Value = serde_json::from_str(&host.caret_world_json()).unwrap();
    assert!(v["x"].as_f64().unwrap() > 0.0);
    assert!(v["y"].as_f64().unwrap() > 0.0);
}

#[test]
fn pick_targets_reports_line_and_token() {
    let mut host = EditorHost::new();
    host.set_text("MATCH x".into());
    host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
    let (wx, wy) = offset_to_world(&host, 2);
    let screen: serde_json::Value = serde_json::from_str(&host.world_to_screen_json(wx, wy)).unwrap();
    let json = host.pick_targets_at_screen_json(screen["x"].as_f64().unwrap(), screen["y"].as_f64().unwrap());
    let rows: serde_json::Value = serde_json::from_str(&json).unwrap();
    let arr = rows.as_array().unwrap();
    assert_eq!(arr[0]["domain"], "line");
    assert!(arr.iter().any(|r| r["domain"] == "token"));
}

#[test]
fn pick_targets_without_token_only_reports_line() {
    let mut host = EditorHost::new();
    host.set_text("hello".into());
    let (wx, wy) = offset_to_world(&host, 2);
    let screen: serde_json::Value = serde_json::from_str(&host.world_to_screen_json(wx, wy)).unwrap();
    let json = host.pick_targets_at_screen_json(screen["x"].as_f64().unwrap(), screen["y"].as_f64().unwrap());
    let rows: serde_json::Value = serde_json::from_str(&json).unwrap();
    let arr = rows.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["domain"], "line");
}

#[test]
fn select_span_at_screen_selects_atomic_span() {
    let mut host = EditorHost::new();
    host.set_text("RETURN a1.name".into());
    host.set_semantic_tokens_json(r#"[{"start":0,"end":6,"class":"keyword"},{"start":7,"end":9,"class":"ident"},{"start":9,"end":10,"class":"operator"},{"start":10,"end":14,"class":"ident"}]"#);
    host.set_selectable_spans_json(r#"[{"start":7,"end":9,"kind":"atomic"},{"start":10,"end":14,"kind":"atomic"}]"#);
    let (wx, wy) = offset_to_world(&host, 8);
    let screen: serde_json::Value = serde_json::from_str(&host.world_to_screen_json(wx, wy)).unwrap();
    host.select_span_at_screen(screen["x"].as_f64().unwrap(), screen["y"].as_f64().unwrap());
    assert_eq!(host.anchor(), 7);
    assert_eq!(host.caret(), 9);
}

#[test]
fn selection_snaps_property_access_tail_allowed() {
    let mut host = EditorHost::new();
    host.set_text("RETURN a1.name".into());
    host.set_semantic_tokens_json(r#"[{"start":0,"end":6,"class":"keyword"},{"start":7,"end":9,"class":"ident"},{"start":9,"end":10,"class":"operator"},{"start":10,"end":14,"class":"ident"}]"#);
    host.set_selectable_spans_json(r#"[{"start":7,"end":9,"kind":"atomic"},{"start":10,"end":14,"kind":"atomic"},{"start":7,"end":14,"kind":"propertyAccess","headEnd":9,"tailStart":10}]"#);
    host.set_selection(10, 14);
    assert_eq!(host.anchor(), 10);
    assert_eq!(host.caret(), 14);
}

#[test]
fn var_label_without_head_end_falls_back_to_span_end() {
    let mut host = EditorHost::new();
    host.set_text("RETURN a1".into());
    host.set_selectable_spans_json(r#"[{"start":7,"end":9,"kind":"varLabel"}]"#);
    host.set_selection(7, 9);
    assert_eq!(host.anchor(), 7);
    assert_eq!(host.caret(), 9);
}

#[test]
fn build_scene_multiline_selection_and_hover_render_without_panic() {
    let mut host = EditorHost::new();
    host.set_text("line one\nline two\nline three".into());
    host.set_selection(2, 20);
    host.set_hover_range(Some(0), Some(4));
    let _scene = host.build_scene();
    assert_eq!(host.selection_text().len(), 18);
}

#[test]
fn build_scene_uses_occurrences_when_present() {
    let mut host = EditorHost::new();
    host.set_text("abc abc abc".into());
    host.set_selection_occurrences_json(r#"[{"start":0,"end":3},{"start":8,"end":11}]"#);
    host.set_hover_occurrences_json(r#"[{"start":4,"end":7}]"#);
    let _scene = host.build_scene();
    assert_eq!(host.selection_occurrences.len(), 2);
}

#[test]
fn build_scene_renders_diagnostics_with_and_without_warning_severity() {
    let mut host = EditorHost::new();
    host.set_text("abc def".into());
    host.set_diagnostics_json(r#"[{"start":0,"end":3,"severity":"warning","message":"w"},{"start":4,"end":7,"message":"e"}]"#);
    let _scene = host.build_scene();
    assert_eq!(host.diagnostics.len(), 2);
}

#[test]
fn build_scene_renders_placeholders() {
    let mut host = EditorHost::new();
    host.set_text("abc".into());
    host.set_placeholders_json(r#"[{"offset":1,"label":"?"}]"#);
    let _scene = host.build_scene();
    assert_eq!(host.placeholders.len(), 1);
}

#[test]
fn build_scene_renders_extra_carets() {
    let mut host = EditorHost::new();
    host.set_text("abc def".into());
    host.set_extra_carets_json(r#"[1,3]"#);
    host.set_caret_anchor(5);
    let _scene = host.build_scene();
    assert_eq!(host.extra_carets, vec![1, 3]);
}

#[test]
fn build_scene_with_caret_hidden_does_not_panic() {
    let mut host = EditorHost::new();
    host.set_text("abc".into());
    host.set_caret_visible(false);
    let _scene = host.build_scene();
    assert!(!host.caret_visible);
}

#[test]
fn build_scene_without_line_numbers_skips_gutter() {
    let mut host = EditorHost::new();
    host.set_editor_settings_json(r#"{"showLineNumbers":false}"#);
    host.set_text("abc".into());
    let _scene = host.build_scene();
    assert_eq!(host.gutter_width(), 0.0);
}

#[test]
fn is_insert_whitespace_detects_whitespace_only() {
    assert!(is_insert_whitespace("  \t\n"));
    assert!(!is_insert_whitespace("a "));
    assert!(!is_insert_whitespace(""));
}

#[test]
fn ranges_overlap_detects_overlap_and_disjoint() {
    assert!(ranges_overlap(0, 5, 3, 8));
    assert!(!ranges_overlap(0, 5, 5, 8));
    assert!(!ranges_overlap(0, 5, 6, 8));
}

#[test]
fn offset_line_col_roundtrip() {
    let text = "abc\ndef\nghi";
    assert_eq!(offset_line_col(text, 5), (1, 1));
    assert_eq!(offset_at_line_col(text, 1, 1), 5);
    assert_eq!(offset_line_col(text, 100), (2, 3));
}

#[test]
fn offset_at_line_col_beyond_last_line_clamps_to_end() {
    let text = "abc\ndef";
    assert_eq!(offset_at_line_col(text, 5, 0), text.len());
}

#[test]
fn char_boundary_helpers_handle_multibyte() {
    let text = "a\u{1f600}\u{fe0f}b";
    for [start, end] in [[0, 1], [1, 5], [5, 8], [8, 9]] {
        assert_eq!(next_char_boundary(text, start), end);
        assert_eq!(prev_char_boundary(text, end), start);
    }
    assert_eq!(prev_char_boundary(text, 0), 0);
    assert_eq!(next_char_boundary(text, text.len()), text.len());
}

#[test]
fn position_to_offset_converts_line_and_character() {
    let text = "abc\ndef";
    let pos = TextPosJson { line: 1, character: 2 };
    assert_eq!(position_to_offset(text, &pos), 6);
}

#[test]
fn hit_byte_in_line_empty_line_returns_zero() {
    assert_eq!(hit_byte_in_line("", 100.0, 0.0, DEFAULT_FONT_PX), 0);
}

#[test]
fn snap_offset_for_atomic_snaps_to_nearest_boundary() {
    let mut host = EditorHost::new();
    host.set_text("MATCH".into());
    host.set_semantic_tokens_json(r#"[{"start":0,"end":6,"class":"keyword"}]"#);
    assert_eq!(host.snap_offset_for_atomic(2), 0);
    assert_eq!(host.snap_offset_for_atomic(4), 6);
    assert_eq!(host.snap_offset_for_atomic(10), 10);
}

#[test]
fn token_span_at_offset_returns_none_outside_tokens() {
    let mut host = EditorHost::new();
    host.set_text("abc".into());
    host.set_semantic_tokens_json(r#"[{"start":0,"end":1,"class":"x"}]"#);
    assert_eq!(host.token_span_at_offset(2), None);
    assert_eq!(host.token_span_at_offset(0), Some((0, 1)));
}

#[test]
fn token_boundaries_detect_adjacent_tokens() {
    let mut host = EditorHost::new();
    host.set_text("ab cd".into());
    host.set_semantic_tokens_json(r#"[{"start":0,"end":2,"class":"x"},{"start":3,"end":5,"class":"y"}]"#);
    assert_eq!(host.token_left_boundary(1), Some(0));
    assert_eq!(host.token_left_boundary(2), Some(0));
    assert_eq!(host.token_left_boundary(6), None);
    assert_eq!(host.token_right_boundary(4), Some(5));
    assert_eq!(host.token_right_boundary(2), None);
}

#[test]
fn allowed_composite_selection_matches_full_span_or_default_false() {
    let mut host = EditorHost::new();
    host.set_text("abc".into());
    let span = SelectableSpanJson { start: 0, end: 3, kind: "custom".into(), head_end: None, tail_start: None };
    assert!(host.allowed_composite_selection(0, 3, &span));
    assert!(!host.allowed_composite_selection(0, 2, &span));
}
