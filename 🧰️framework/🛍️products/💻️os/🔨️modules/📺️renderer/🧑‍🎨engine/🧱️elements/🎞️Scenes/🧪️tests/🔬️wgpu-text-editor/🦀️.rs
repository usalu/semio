
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
        framework_widget_context(&mut self.draw, None, &mut self.atlas, None, &mut self.input, &self.theme, &mut self.scroll_offsets, &mut self.collapsed_sections, &mut self.open_selects, None, 0.0)
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

//#region CompletionPrefix
#[test]
fn identifier_prefix_start_stops_at_the_nearest_non_identifier_char() {
    let text = "let my_var";
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
    let scene = text_editor_scene("editor.rename.parse", "", None, Some(r#"{"name":"count","occurrences":[{"start":0,"end":5},{"start":10,"end":15}]}"#));
    let info = text_editor_rename_info(scene.text_editor.as_ref().unwrap()).expect("rename info");
    assert_eq!(info.name, "count");
    assert_eq!(info.occurrences.len(), 2);
}
//#endregion CompletionsParsing

/// 🧪️ Drains the bounded action authority — the only way out of an `InputState`, since an action is
/// taken (not read) and materializes through `into_descriptor`.
fn drain_actions(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    let mut actions = Vec::new();
    while let Some(action) = input.take_action_step().expect("action authority remains live") {
        actions.push(action.into_descriptor().expect("bounded action materializes"));
    }
    actions
}

//#region MultiSpanRename
/// ✏️ `multiSpanReplace` (`✏️TextEditor/🟦️.tsx:80`) rewrites back-to-front so earlier spans keep
/// their offsets, and reports the POST-rewrite spans in document order.
#[test]
fn multi_span_replace_rewrites_every_occurrence_and_reports_new_spans() {
    let (text, spans) = multi_span_replace("count + count", &[(0, 5), (8, 13)], "total");
    assert_eq!(text, "total + total");
    assert_eq!(spans, vec![(0, 5), (8, 13)]);
}

/// ✏️ PARITY, including React's own quirk: the reported spans are `{ start: occ.start, end:
/// occ.start + name.length }` off the ORIGINAL starts (`✏️TextEditor/🟦️.tsx:86`), so a replacement
/// that changes the name's length does not shift the later spans. Pinned here because the wgpu twin
/// must report what React reports — see this packet's report for the shared-defect note.
#[test]
fn multi_span_replace_reports_react_spans_without_shifting_later_ones() {
    let (text, spans) = multi_span_replace("a + a", &[(0, 1), (4, 5)], "abcd");
    assert_eq!(text, "abcd + abcd");
    assert_eq!(spans, vec![(0, 4), (4, 8)]);
}
//#endregion MultiSpanRename

//#region CompletionPopupStateMachine
/// 📋️ `openCompletions` is a no-operation without completions, and opens at index 0 with them —
/// React's `if (completions.length === 0) return; setCompletionsOpen(true); setCompletionIndex(0)`.
#[test]
fn opening_completions_needs_completions_and_starts_at_the_first_row() {
    let empty = text_editor_scene("editor.popup.empty", "", None, None);
    assert!(!text_editor_open_completions(&empty));
    assert!(!text_editor_completions_open(&empty.surface_id));

    let scene = text_editor_scene("editor.popup.open", "", Some(r#"[{"label":"alpha"},{"label":"beta"}]"#), None);
    assert!(text_editor_open_completions(&scene));
    assert!(text_editor_completions_open(&scene.surface_id));
    assert_eq!(text_editor_ui(&scene.surface_id).completion_index, 0);
}

/// 📋️ The highlight WRAPS in both directions — `(index ± 1 + length) % length`.
#[test]
fn completion_highlight_wraps_in_both_directions() {
    let scene = text_editor_scene("editor.popup.wrap", "", Some(r#"[{"label":"a"},{"label":"b"}]"#), None);
    assert!(text_editor_open_completions(&scene));
    assert!(text_editor_move_completion(&scene, true));
    assert_eq!(text_editor_ui(&scene.surface_id).completion_index, 1);
    assert!(text_editor_move_completion(&scene, true));
    assert_eq!(text_editor_ui(&scene.surface_id).completion_index, 0);
    assert!(text_editor_move_completion(&scene, false));
    assert_eq!(text_editor_ui(&scene.surface_id).completion_index, 1);
}

/// ⌨️ `Ctrl/Cmd+Space` opens the dropdown, and `Escape` closes it — the first and last branches of
/// React's own `onKeyDown` prelude (`✏️TextEditor/🟦️.tsx:563`, `:604`).
#[test]
fn ctrl_space_opens_completions_and_escape_closes_them() {
    let scene = text_editor_scene("editor.popup.keys", "", Some(r#"[{"label":"alpha"}]"#), None);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let accelerator = ui_wgpu::wgpu::PointerModifiers { ctrl: true, ..Default::default() };
    assert_eq!(text_editor_popup_key(&scene, &KeyAction::Space(true), &accelerator, &mut input), Ok(true));
    assert!(text_editor_completions_open(&scene.surface_id));
    assert_eq!(text_editor_popup_key(&scene, &KeyAction::Escape, &ui_wgpu::wgpu::PointerModifiers::default(), &mut input), Ok(true));
    assert!(!text_editor_completions_open(&scene.surface_id));
}

/// ⌨️ A plain Space is NOT the completions gesture, and an arrow key with the dropdown closed is a
/// buffer key — the popup must not swallow ordinary editing.
#[test]
fn popup_keys_are_declined_when_no_popup_is_open() {
    let scene = text_editor_scene("editor.popup.decline", "", Some(r#"[{"label":"alpha"}]"#), None);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let plain = ui_wgpu::wgpu::PointerModifiers::default();
    assert_eq!(text_editor_popup_key(&scene, &KeyAction::Space(true), &plain, &mut input), Ok(false));
    assert_eq!(text_editor_popup_key(&scene, &KeyAction::ArrowDown, &plain, &mut input), Ok(false));
    assert_eq!(text_editor_popup_key(&scene, &KeyAction::Enter, &plain, &mut input), Ok(false));
}

/// ⌨️ With the dropdown OPEN the arrows move the highlight instead of the caret — React returns
/// early from the same branch.
#[test]
fn open_completions_claim_the_arrow_keys() {
    let scene = text_editor_scene("editor.popup.arrows", "", Some(r#"[{"label":"a"},{"label":"b"}]"#), None);
    assert!(text_editor_open_completions(&scene));
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let plain = ui_wgpu::wgpu::PointerModifiers::default();
    assert_eq!(text_editor_popup_key(&scene, &KeyAction::ArrowDown, &plain, &mut input), Ok(true));
    assert_eq!(text_editor_ui(&scene.surface_id).completion_index, 1);
    assert_eq!(text_editor_popup_key(&scene, &KeyAction::ArrowUp, &plain, &mut input), Ok(true));
    assert_eq!(text_editor_ui(&scene.surface_id).completion_index, 0);
    text_editor_close_completions(&scene.surface_id);
}

/// 🖱️ A press on an OPEN dropdown row commits it; a press anywhere else dismisses the dropdown and
/// is handed back to the caret path. With no registered `EditorHost` the commit itself is a graceful
/// no-operation, which is what makes this assertable GPU-free.
#[test]
fn a_press_outside_the_dropdown_dismisses_it_and_falls_through() {
    let scene = text_editor_scene("editor.popup.pointer", "", Some(r#"[{"label":"alpha"}]"#), None);
    assert!(text_editor_open_completions(&scene));
    let inner = Rect::new(0.0, 0.0, 300.0, 300.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert_eq!(text_editor_popup_pointer(&scene, inner, 280.0, 280.0, false, &mut input), Ok(false));
    assert!(!text_editor_completions_open(&scene.surface_id));
}

/// 🖱️ ALT + press opens the dropdown — React's `event.altKey && completions.length > 0` branch.
#[test]
fn alt_press_opens_the_completions_dropdown() {
    let scene = text_editor_scene("editor.popup.alt", "", Some(r#"[{"label":"alpha"}]"#), None);
    let inner = Rect::new(0.0, 0.0, 300.0, 300.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert_eq!(text_editor_popup_pointer(&scene, inner, 40.0, 40.0, true, &mut input), Ok(true));
    assert!(text_editor_completions_open(&scene.surface_id));
    text_editor_close_completions(&scene.surface_id);
}
//#endregion CompletionPopupStateMachine

//#region RenamePopupStateMachine
/// ✏️ `F2` arms the rename only when the scene published `rename_json` — React guards the same key
/// with `renameInfo` (`✏️TextEditor/🟦️.tsx:568`).
#[test]
fn f2_starts_a_rename_only_with_rename_info() {
    let bare = text_editor_scene("editor.rename.none", "count", None, None);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let plain = ui_wgpu::wgpu::PointerModifiers::default();
    assert_eq!(text_editor_popup_key(&bare, &KeyAction::Function(2), &plain, &mut input), Ok(false));
    assert!(!text_editor_rename_active(&bare.surface_id));

    let scene = text_editor_scene("editor.rename.armed", "count + count", None, Some(r#"{"name":"count","occurrences":[{"start":0,"end":5},{"start":8,"end":13}]}"#));
    assert_eq!(text_editor_popup_key(&scene, &KeyAction::Function(2), &plain, &mut input), Ok(true));
    assert!(text_editor_rename_active(&scene.surface_id));
    assert_eq!(text_editor_ui(&scene.surface_id).rename.expect("draft").occurrences, vec![(0, 5), (8, 13)]);
    assert!(text_editor_cancel_rename(&scene));
}

/// ✏️ Arming a rename takes keyboard focus into the rename input (React's `autoFocus`), and the
/// draft starts at the current name; typing and Backspace then retype it, so the NEXT keystrokes
/// never reach the buffer.
#[test]
fn an_armed_rename_focuses_its_input_and_consumes_every_key() {
    let scene = text_editor_scene("editor.rename.focus", "count", None, Some(r#"{"name":"count","occurrences":[{"start":0,"end":5}]}"#));
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(text_editor_start_rename(&scene, &mut input));
    assert_eq!(input.focused_id.as_deref(), Some("editor.rename.focus.editor.rename"));
    assert_eq!(text_editor_ui(&scene.surface_id).rename.expect("draft").text, "count");
    let plain = ui_wgpu::wgpu::PointerModifiers::default();
    assert_eq!(text_editor_popup_key(&scene, &KeyAction::Char("s".into()), &plain, &mut input), Ok(true));
    assert_eq!(text_editor_ui(&scene.surface_id).rename.expect("draft").text, "counts");
    assert_eq!(text_editor_popup_key(&scene, &KeyAction::Backspace, &plain, &mut input), Ok(true));
    assert_eq!(text_editor_ui(&scene.surface_id).rename.expect("draft").text, "count");
    assert_eq!(text_editor_popup_key(&scene, &KeyAction::ArrowDown, &plain, &mut input), Ok(true), "an armed rename input swallows every key");
    assert!(text_editor_cancel_rename(&scene));
}

/// ✏️ `Escape` cancels (nothing dispatched) and `Enter` commits `commitRename { occurrences, text }`
/// — React's `cancelRename`/`commitRename` pair.
#[test]
fn enter_commits_the_rename_and_escape_cancels_it() {
    let scene = text_editor_scene("editor.rename.commit", "count", None, Some(r#"{"name":"count","occurrences":[{"start":0,"end":5}]}"#));
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(text_editor_start_rename(&scene, &mut input));
    assert_eq!(text_editor_popup_key(&scene, &KeyAction::Escape, &ui_wgpu::wgpu::PointerModifiers::default(), &mut input), Ok(true));
    assert!(!text_editor_rename_active(&scene.surface_id));
    assert!(drain_actions(&mut input).is_empty(), "a cancelled rename must dispatch nothing");

    assert!(text_editor_start_rename(&scene, &mut input));
    assert_eq!(text_editor_popup_key(&scene, &KeyAction::Enter, &ui_wgpu::wgpu::PointerModifiers::default(), &mut input), Ok(true));
    assert!(!text_editor_rename_active(&scene.surface_id));
    let actions = drain_actions(&mut input);
    let commit = actions.iter().find(|action| action.action == "commitRename").unwrap_or_else(|| panic!("expected commitRename, got {:?}", actions.iter().map(|action| action.action.clone()).collect::<Vec<_>>()));
    assert_eq!(commit.args.as_ref().and_then(|args| args.get("surfaceId")).and_then(semio_framework::DslValue::as_str), Some("editor.rename.commit"));
    assert_eq!(commit.args.as_ref().and_then(|args| args.get("text")).and_then(semio_framework::DslValue::as_str), Some("count"));
    assert_eq!(commit.args.as_ref().and_then(|args| args.get("occurrences")).and_then(semio_framework::DslValue::as_array).map(<[semio_framework::DslValue]>::len), Some(1));
}
//#endregion RenamePopupStateMachine

//#region LocalMenuActions
/// 🖱️ The six rows a text editor answers ITSELF are claimed and parked; every other row is declined
/// so the shell dispatches it to the guest — React's `localActions` short-circuit.
#[test]
fn only_the_editors_own_menu_rows_are_claimed() {
    let scene = text_editor_scene("editor.menu.claim", "hello", None, None);
    for action in ["requestCompletions", "selectToken", "selectLine", "selectAll", "commitRename"] {
        assert!(text_editor_queue_menu_action(&scene.surface_id, action, 4.0, 4.0), "{action} must be claimed locally");
    }
    for action in ["formatDocument", "lintDocument", "cut", "copy", "paste", "somePluginVerb"] {
        assert!(!text_editor_queue_menu_action(&scene.surface_id, action, 4.0, 4.0), "{action} must stay dispatchable");
    }
}

/// 🖱️ A claimed row is executed against the surface. `selectAll` needs no engine host to be
/// resolvable, so its span is the buffer's whole length regardless of GPU state.
#[test]
fn a_claimed_row_runs_against_the_surface() {
    let scene = text_editor_scene("editor.menu.run", "hello", None, None);
    let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert_eq!(text_editor_local_menu_action(&scene, inner, "selectAll", 4.0, 4.0, &mut input), Ok(true));
    assert_eq!(text_editor_local_menu_action(&scene, inner, "somePluginVerb", 4.0, 4.0, &mut input), Ok(false));
}

/// 📏️ `lineRangeAt` (`✏️TextEditor/🟦️.tsx:91`) — the "Select Line" range, including the last line
/// without a trailing newline.
#[test]
fn line_range_covers_the_caret_line_only() {
    assert_eq!(text_editor_line_range("alpha\nbeta\ngamma", 7), (6, 10));
    assert_eq!(text_editor_line_range("alpha\nbeta", 10), (6, 10));
    assert_eq!(text_editor_line_range("alpha", 0), (0, 5));
}
//#endregion LocalMenuActions

//#region PopupChromePaintTests
/// 📋️ `rounded border border-border bg-popover` on the dropdown, `bg-accent` on the active row, and
/// one hit target per row so a click can commit it.
#[test]
fn completions_popup_paints_a_bordered_container_an_accent_row_and_per_row_hit_targets() {
    let scene = text_editor_scene("editor.completions.paint", "", Some(r#"[{"label":"alpha"},{"label":"beta"}]"#), None);
    assert!(text_editor_open_completions(&scene));
    let bounds = Rect::new(0.0, 0.0, 300.0, 300.0);
    let mut fixture = Fixture::new();
    {
        let mut ctx = fixture.ctx();
        render_text_editor_overlays(&scene, bounds, &mut ctx);
    }
    let theme = fixture.theme;
    let border = theme.panel_border;
    assert!(
        fixture.draw.layers.iter().flat_map(|layer| layer.vector_vertices.iter()).any(|vertex| vertex.color == [border.r, border.g, border.b, border.a]),
        "expected the completions popup to draw an outer container border"
    );
    let colors: Vec<[f32; 4]> = fixture.draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|instance| instance.color).collect();
    let accent = theme.accent;
    assert!(colors.contains(&[accent.r, accent.g, accent.b, accent.a]), "expected the active completion row's background to be theme.accent, got {colors:?}");
    let rows: Vec<String> = fixture.input.staged_hits().iter().filter_map(|target| target.control_id.clone()).filter(|id| id.contains(".editor.completion.")).collect();
    assert_eq!(rows.len(), 2, "expected one hit target per completion row, got {rows:?}");
    text_editor_close_completions(&scene.surface_id);
}

/// ✏️ The rename input paints only while a draft is armed, with `border border-border bg-panel`.
#[test]
fn rename_input_paints_only_while_a_draft_is_armed() {
    let scene = text_editor_scene("editor.rename.paint", "count", None, Some(r#"{"name":"count","occurrences":[{"start":0,"end":5}]}"#));
    let bounds = Rect::new(0.0, 0.0, 300.0, 300.0);
    {
        let mut idle = Fixture::new();
        {
            let mut ctx = idle.ctx();
            render_text_editor_overlays(&scene, bounds, &mut ctx);
        }
        assert!(idle.input.staged_hits().iter().all(|target| target.control_id.as_deref() != Some("editor.rename.paint.editor.rename")));
    }
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(text_editor_start_rename(&scene, &mut input));
    let mut fixture = Fixture::new();
    {
        let mut ctx = fixture.ctx();
        render_text_editor_overlays(&scene, bounds, &mut ctx);
    }
    let theme = fixture.theme;
    let border = theme.panel_border;
    assert!(
        fixture.draw.layers.iter().flat_map(|layer| layer.vector_vertices.iter()).any(|vertex| vertex.color == [border.r, border.g, border.b, border.a]),
        "expected the rename input to draw a border stroke"
    );
    let colors: Vec<[f32; 4]> = fixture.draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|instance| instance.color).collect();
    let panel = theme.panel;
    assert!(colors.contains(&[panel.r, panel.g, panel.b, panel.a]), "expected the rename input fill to use theme.panel, got {colors:?}");
    assert!(text_editor_cancel_rename(&scene));
}

/// 🍿️ With no popup armed the overlay pass draws nothing at all — a text editor that is merely
/// focused must not paint chrome over its own buffer.
#[test]
fn the_overlay_pass_draws_nothing_when_no_popup_is_open() {
    let scene = text_editor_scene("editor.overlays.idle", "hello", Some(r#"[{"label":"alpha"}]"#), None);
    let bounds = Rect::new(0.0, 0.0, 300.0, 300.0);
    let mut fixture = Fixture::new();
    {
        let mut ctx = fixture.ctx();
        render_text_editor_overlays(&scene, bounds, &mut ctx);
    }
    assert!(fixture.draw.layers.iter().all(|layer| layer.ui_instances.is_empty() && layer.vector_vertices.is_empty()));
}
//#endregion PopupChromePaintTests
