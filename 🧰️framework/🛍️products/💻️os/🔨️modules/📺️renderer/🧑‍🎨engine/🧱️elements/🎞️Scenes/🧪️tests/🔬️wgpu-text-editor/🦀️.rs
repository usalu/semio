
use super::*;
use crate::interpreter::framework_widget_context;
use ui_wgpu::wgpu::UiPresence;

fn test_scene(surface_id: &str, kind: SurfaceKind) -> UiComponentSceneNode {
    UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: "controller".into(),
        component_kind: kind,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        diff_view: None,
        event_feed: None,
        block_list: None,
        menu: None,
    }
}

fn text_editor_scene_payload(buffer: &str, completions_json: Option<&str>, rename_json: Option<&str>) -> ui_wgpu::wgpu::TextEditorScene {
    ui_wgpu::wgpu::TextEditorScene {
        buffer: buffer.to_string(),
        language: None,
        selection_json: None,
        tokens_json: None,
        diagnostics_json: None,
        completions_json: completions_json.map(str::to_string),
        overlays_json: None,
        occurrences_json: None,
        placeholders_json: None,
        extra_carets_json: None,
        selectable_spans_json: None,
        settings_json: None,
        camera_json: None,
        hover_json: None,
        newline_gates_json: None,
        rename_json: rename_json.map(str::to_string),
    }
}

fn text_editor_scene(surface_id: &str, buffer: &str, completions_json: Option<&str>, rename_json: Option<&str>) -> UiComponentSceneNode {
    let mut scene = test_scene(surface_id, SurfaceKind::TextEditor);
    scene.text_editor = Some(text_editor_scene_payload(buffer, completions_json, rename_json));
    scene
}

/// 🧰️ GPU-free `FrameworkWidgetContext` fixture, same construction as `render_entry_tests::Fixture`
/// (private to that module, so duplicated here rather than reused).
struct Fixture {
    draw: ui_wgpu::wgpu::DrawList,
    atlas: ui_wgpu::wgpu::FontAtlas,
    theme: Theme,
    input: ui_wgpu::wgpu::InputState<ActionDescriptor>,
    scroll_offsets: HashMap<String, f32>,
    collapsed_sections: HashMap<String, bool>,
    open_selects: HashMap<String, bool>,
}

impl Fixture {
    fn new() -> Self {
        Self {
            draw: ui_wgpu::wgpu::DrawList::default(),
            atlas: ui_wgpu::wgpu::FontAtlas::builtin(),
            theme: Theme::default(),
            input: ui_wgpu::wgpu::InputState::<ActionDescriptor>::default(),
            scroll_offsets: HashMap::new(),
            collapsed_sections: HashMap::new(),
            open_selects: HashMap::new(),
        }
    }

    fn ctx(&mut self) -> FrameworkWidgetContext<'_> {
        framework_widget_context(&mut self.draw, None, &mut self.atlas, None, &mut self.input, &self.theme, &mut self.scroll_offsets, &mut self.collapsed_sections, &mut self.open_selects, None)
    }
}

//#region ClickToCaretGeometry
#[test]
fn cursor_from_click_resolves_the_first_line_offset_at_the_click_x() {
    let scene = text_editor_scene("editor.click", "hello\nworld", None, None);
    let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
    // 8px left padding + ~7px/char advance (see `cursor_from_click`): clicking near x=8 should land at
    // the very start of the line, clicking further right should land later in "hello".
    let start = cursor_from_click(&scene, inner, 8.0, 8.0, 0.0);
    let mid = cursor_from_click(&scene, inner, 8.0 + 7.0 * 3.0, 8.0, 0.0);
    assert_eq!(start, 0);
    assert!(mid >= 2 && mid <= 4, "expected an offset inside \"hello\", got {mid}");
}

#[test]
fn cursor_from_click_accounts_for_line_index_via_y() {
    let scene = text_editor_scene("editor.click.line2", "ab\ncd\nef", None, None);
    let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
    // line_h is 18.0 and the first line starts at y = inner.y + 8.0 (see `cursor_from_click`).
    let offset = cursor_from_click(&scene, inner, 8.0, 8.0 + 18.0 + 2.0, 0.0);
    let (line, col) = line_col_at("ab\ncd\nef", offset);
    assert_eq!((line, col), (1, 0));
}

#[test]
fn line_col_at_reports_line_and_column_for_a_mid_buffer_offset() {
    let (line, col) = line_col_at("alpha\nbeta\ngamma", 7);
    assert_eq!((line, col), (1, 1));
}
//#endregion ClickToCaretGeometry

//#region SelectLine
#[test]
fn text_editor_line_range_returns_the_bounds_of_the_containing_line() {
    let buffer = "first\nsecond line\nthird";
    let (start, end) = text_editor_line_range(buffer, 9);
    assert_eq!(&buffer[start..end], "second line");
}

#[test]
fn text_editor_line_range_handles_the_last_line_without_a_trailing_newline() {
    let buffer = "one\ntwo";
    let (start, end) = text_editor_line_range(buffer, 5);
    assert_eq!(&buffer[start..end], "two");
}
//#endregion SelectLine

//#region CompletionPrefix
#[test]
fn identifier_prefix_start_stops_at_the_nearest_non_identifier_char() {
    let text = "let value = my_var";
    let caret = text.len();
    assert_eq!(&text[identifier_prefix_start(text, caret)..caret], "my_var");
}

#[test]
fn identifier_prefix_start_returns_caret_when_not_inside_an_identifier() {
    let text = "a = ";
    assert_eq!(identifier_prefix_start(text, text.len()), text.len());
}
//#endregion CompletionPrefix

//#region CompletionsParsing
#[test]
fn text_editor_completions_parses_label_and_optional_detail() {
    let scene = text_editor_scene("editor.completions", "", Some(r#"[{"label":"foo","detail":"fn foo()"},{"label":"bar"}]"#), None);
    let items = text_editor_completions(scene.text_editor.as_ref().unwrap());
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].label, "foo");
    assert_eq!(items[0].detail.as_deref(), Some("fn foo()"));
    assert_eq!(items[1].detail, None);
}

#[test]
fn text_editor_completions_is_empty_for_missing_or_malformed_json() {
    let missing = text_editor_scene("editor.completions.missing", "", None, None);
    assert!(text_editor_completions(missing.text_editor.as_ref().unwrap()).is_empty());
    let malformed = text_editor_scene("editor.completions.bad", "", Some("not json"), None);
    assert!(text_editor_completions(malformed.text_editor.as_ref().unwrap()).is_empty());
}

#[test]
fn text_editor_rename_info_parses_name_and_occurrences() {
    let scene = text_editor_scene("editor.rename", "", None, Some(r#"{"name":"count","occurrences":[{"start":0,"end":5},{"start":10,"end":15}]}"#));
    let info = text_editor_rename_info(scene.text_editor.as_ref().unwrap()).expect("rename info");
    assert_eq!(info.name, "count");
    assert_eq!(info.occurrences.len(), 2);
    assert_eq!((info.occurrences[1].start, info.occurrences[1].end), (10, 15));
}
//#endregion CompletionsParsing

//#region ContextMenuItems
#[test]
fn context_menu_items_include_suggest_only_when_completions_are_present() {
    let without = text_editor_scene("editor.menu.no-suggest", "x", None, None);
    let items = text_editor_context_menu_items(without.text_editor.as_ref().unwrap());
    assert!(!items.iter().any(|item| item.id == "suggest"));

    let with = text_editor_scene("editor.menu.suggest", "x", Some(r#"[{"label":"x"}]"#), None);
    let items = text_editor_context_menu_items(with.text_editor.as_ref().unwrap());
    assert!(items.iter().any(|item| item.id == "suggest"));
}

#[test]
fn context_menu_items_include_rename_only_when_rename_info_is_present() {
    let without = text_editor_scene("editor.menu.no-rename", "x", None, None);
    let items = text_editor_context_menu_items(without.text_editor.as_ref().unwrap());
    assert!(!items.iter().any(|item| item.id == "rename"));

    let with = text_editor_scene("editor.menu.rename", "x", None, Some(r#"{"name":"x","occurrences":[]}"#));
    let items = text_editor_context_menu_items(with.text_editor.as_ref().unwrap());
    assert!(items.iter().any(|item| item.id == "rename"));
}

#[test]
fn context_menu_items_always_include_selection_and_document_actions() {
    let scene = text_editor_scene("editor.menu.baseline", "x", None, None);
    let items = text_editor_context_menu_items(scene.text_editor.as_ref().unwrap());
    for expected in ["select-token", "select-line", "select-all", "format", "lint"] {
        assert!(items.iter().any(|item| item.id == expected), "missing {expected}");
    }
}
//#endregion ContextMenuItems

//#region ContextMenuGeometry
#[test]
fn menu_row_rects_stack_vertically_without_overlapping() {
    let menu = TextEditorContextMenu { x: 10.0, y: 20.0, items: vec![TextEditorMenuItem { id: "a", label: "A" }, TextEditorMenuItem { id: "b", label: "B" }, TextEditorMenuItem { id: "c", label: "C" }] };
    let theme = Theme::default();
    let first = text_editor_menu_row_rect(&menu, &theme, 0);
    let second = text_editor_menu_row_rect(&menu, &theme, 1);
    assert_eq!(second.y, first.y + theme.control_height);
    assert_eq!(first.x, second.x);
}

#[test]
fn menu_hit_finds_the_row_under_the_point_and_none_outside_it() {
    let menu = TextEditorContextMenu { x: 0.0, y: 0.0, items: vec![TextEditorMenuItem { id: "a", label: "A" }, TextEditorMenuItem { id: "b", label: "B" }] };
    let theme = Theme::default();
    let row_h = theme.control_height;
    assert_eq!(text_editor_menu_hit(&menu, &theme, 8.0, 8.0), Some(0));
    assert_eq!(text_editor_menu_hit(&menu, &theme, 8.0, row_h + 8.0), Some(1));
    assert_eq!(text_editor_menu_hit(&menu, &theme, 8.0, row_h * 10.0), None);
}
//#endregion ContextMenuGeometry

//#region ContextMenuActionDispatch
#[test]
fn run_menu_action_format_queues_a_format_document_action() {
    let scene = text_editor_scene("editor.action.format", "hello", None, None);
    let editor = scene.text_editor.as_ref().unwrap().clone();
    let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
    let menu = TextEditorContextMenu { x: 4.0, y: 4.0, items: vec![] };
    let mut fixture = Fixture::new();
    let mut ui_state = TextEditorUiState::default();
    {
        let mut ctx = fixture.ctx();
        text_editor_run_menu_action(&scene, &editor, inner, &menu, "format", &mut ctx, &mut ui_state);
    }
    let events = crate::collect_fixture_actions(&mut fixture.input);
    assert!(events.iter().any(|action| action.action == "formatDocument"));
}

#[test]
fn run_menu_action_lint_queues_a_lint_document_action() {
    let scene = text_editor_scene("editor.action.lint", "hello", None, None);
    let editor = scene.text_editor.as_ref().unwrap().clone();
    let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
    let menu = TextEditorContextMenu { x: 4.0, y: 4.0, items: vec![] };
    let mut fixture = Fixture::new();
    let mut ui_state = TextEditorUiState::default();
    {
        let mut ctx = fixture.ctx();
        text_editor_run_menu_action(&scene, &editor, inner, &menu, "lint", &mut ctx, &mut ui_state);
    }
    let events = crate::collect_fixture_actions(&mut fixture.input);
    assert!(events.iter().any(|action| action.action == "lintDocument"));
}

#[test]
fn run_menu_action_suggest_opens_the_completions_popup_at_index_zero() {
    let scene = text_editor_scene("editor.action.suggest", "hello", Some(r#"[{"label":"a"},{"label":"b"}]"#), None);
    let editor = scene.text_editor.as_ref().unwrap().clone();
    let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
    let menu = TextEditorContextMenu { x: 4.0, y: 4.0, items: vec![] };
    let mut fixture = Fixture::new();
    let mut ui_state = TextEditorUiState { completion_index: 3, ..Default::default() };
    {
        let mut ctx = fixture.ctx();
        text_editor_run_menu_action(&scene, &editor, inner, &menu, "suggest", &mut ctx, &mut ui_state);
    }
    assert!(ui_state.completions_open);
    assert_eq!(ui_state.completion_index, 0);
}

#[test]
fn run_menu_action_rename_activates_rename_state_and_focuses_the_rename_input() {
    let scene = text_editor_scene("editor.action.rename", "count", None, Some(r#"{"name":"count","occurrences":[{"start":0,"end":5}]}"#));
    let editor = scene.text_editor.as_ref().unwrap().clone();
    let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
    let menu = TextEditorContextMenu { x: 4.0, y: 4.0, items: vec![] };
    let mut fixture = Fixture::new();
    let mut ui_state = TextEditorUiState::default();
    {
        let mut ctx = fixture.ctx();
        text_editor_run_menu_action(&scene, &editor, inner, &menu, "rename", &mut ctx, &mut ui_state);
    }
    assert!(ui_state.rename_active);
    assert_eq!(ui_state.rename_occurrences, vec![(0, 5)]);
    assert_eq!(fixture.input.focused_id.as_deref(), Some("editor.action.rename.editor.rename"));
    assert_eq!(fixture.input.text_view(), "count");
}

#[test]
fn run_menu_action_rename_is_a_no_op_without_rename_info() {
    let scene = text_editor_scene("editor.action.no-rename", "count", None, None);
    let editor = scene.text_editor.as_ref().unwrap().clone();
    let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
    let menu = TextEditorContextMenu { x: 4.0, y: 4.0, items: vec![] };
    let mut fixture = Fixture::new();
    let mut ui_state = TextEditorUiState::default();
    {
        let mut ctx = fixture.ctx();
        text_editor_run_menu_action(&scene, &editor, inner, &menu, "rename", &mut ctx, &mut ui_state);
    }
    assert!(!ui_state.rename_active);
    assert!(fixture.input.focused_id.is_none());
}

#[test]
fn run_menu_action_select_all_reuses_the_ctrl_a_key_path_without_a_registered_engine_surface() {
    // 🛡️ No GPU / `ENGINE_SURFACES` entry exists for this surface_id in a unit test, so this only
    // asserts the dispatch doesn't panic and gracefully no-operations (see `engine_canvas::text_editor_apply_key`).
    let scene = text_editor_scene("editor.action.select-all", "hello", None, None);
    let editor = scene.text_editor.as_ref().unwrap().clone();
    let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
    let menu = TextEditorContextMenu { x: 4.0, y: 4.0, items: vec![] };
    let mut fixture = Fixture::new();
    let mut ui_state = TextEditorUiState::default();
    let mut ctx = fixture.ctx();
    text_editor_run_menu_action(&scene, &editor, inner, &menu, "select-all", &mut ctx, &mut ui_state);
}
//#endregion ContextMenuActionDispatch

//#region PopupChromePaintTests
#[test]
fn completions_popup_has_a_bordered_container_and_the_active_row_uses_accent() {
    let scene = text_editor_scene("editor.completions.paint", "", None, None);
    let inner = Rect::new(0.0, 0.0, 300.0, 300.0);
    let completions = vec![TextEditorCompletionItem { label: "alpha".into(), detail: None, insert_text: None }, TextEditorCompletionItem { label: "beta".into(), detail: None, insert_text: None }];
    let mut fixture = Fixture::new();
    {
        let mut ctx = fixture.ctx();
        render_text_editor_completions(&mut ctx, inner, &scene, &completions, 0);
    }
    let theme = fixture.theme;
    // 🪟️ `border border-border bg-popover` — the container's own outline color must appear among
    // the drawn vector-line vertices (`draw_ink_rect_outline`'s 4-line/24-vertex shape).
    let border = theme.panel_border;
    let has_container_border = fixture.draw.layers.iter().flat_map(|layer| layer.vector_vertices.iter()).any(|v| v.color == [border.r, border.g, border.b, border.a]);
    assert!(has_container_border, "expected the completions popup to draw an outer container border");

    // 🎯️ `bg-accent text-accent-foreground` on the active (index 0) row.
    let colors: Vec<[f32; 4]> = fixture.draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|i| i.color).collect();
    let accent = theme.accent;
    assert!(colors.contains(&[accent.r, accent.g, accent.b, accent.a]), "expected the active completion row's background to be theme.accent, got {colors:?}");
}

#[test]
fn rename_input_draws_a_bordered_panel_box() {
    let scene = text_editor_scene("editor.rename.paint", "count", None, None);
    let inner = Rect::new(0.0, 0.0, 300.0, 300.0);
    let mut fixture = Fixture::new();
    fixture.input.focus_input_owned("test".to_string(), "count2".to_string());
    {
        let mut ctx = fixture.ctx();
        render_text_editor_rename_input(&mut ctx, inner, &scene);
    }
    let theme = fixture.theme;
    // 🖊️ `border border-border bg-panel` — previously an unbordered `theme.input_bg` fill.
    let border = theme.panel_border;
    let has_border = fixture.draw.layers.iter().flat_map(|layer| layer.vector_vertices.iter()).any(|v| v.color == [border.r, border.g, border.b, border.a]);
    assert!(has_border, "expected the rename input to draw a border stroke");
    let colors: Vec<[f32; 4]> = fixture.draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|i| i.color).collect();
    let panel = theme.panel;
    assert!(colors.contains(&[panel.r, panel.g, panel.b, panel.a]), "expected the rename input fill to use theme.panel, got {colors:?}");
}

#[test]
fn context_menu_draws_a_border_stroke_around_the_flat_panel() {
    let menu = TextEditorContextMenu { x: 10.0, y: 10.0, items: vec![TextEditorMenuItem { id: "rename", label: "Rename" }] };
    let mut fixture = Fixture::new();
    {
        let mut ctx = fixture.ctx();
        render_text_editor_context_menu(&mut ctx, &menu);
    }
    let theme = fixture.theme;
    let border = theme.panel_border;
    let has_border = fixture.draw.layers.iter().flat_map(|layer| layer.vector_vertices.iter()).any(|v| v.color == [border.r, border.g, border.b, border.a]);
    assert!(has_border, "expected the context menu's flat panel to at least draw a border stroke");
}
//#endregion PopupChromePaintTests
