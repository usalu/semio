use super::*;
use semio_ui_wgpu_input::InputState;

#[test]
fn context_menu_path_for_ordinal_selects_enabled_rows() {
    let items = vec![ContextMenuItem { id: "a".into(), label: "A".into(), ..Default::default() }, ContextMenuItem { id: "b".into(), label: "B".into(), ..Default::default() }];
    assert_eq!(context_menu_path_for_ordinal(&items, &[], 2), Some(vec![1]));
}

#[test]
fn context_menu_submenu_open_follows_active_path() {
    assert!(context_menu_submenu_open(&[0, 0], &[0], false, true));
    assert!(context_menu_submenu_open(&[0], &[0], true, true));
    assert!(!context_menu_submenu_open(&[1], &[0], false, true));
}

#[test]
fn context_menu_click_on_group_row_control_id_opens_its_submenu_instead_of_dispatching() {
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native);
    shell.input.context_menu = Some(ContextMenuState {
        items: vec![ContextMenuItem { id: "menu.group.view".into(), label: "View".into(), children: vec![ContextMenuItem { id: "menu.group.view.child".into(), label: "Child".into(), ..Default::default() }], ..Default::default() }],
        ..Default::default()
    });
    let hit = HitTarget { rect: Rect::new(0.0, 0.0, 10.0, 10.0), event: None, control_id: Some("menu.group.view".into()), kind: HitKind::ContextMenu, drag_axis: None, drag_data: None };
    let consumed = semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_shell_hit(&hit, &InputState::<ActionDescriptor>::default())).expect("group-row click never errors");
    assert!(consumed);
    let menu = shell.input.context_menu.as_ref().expect("a group-row click opens its submenu instead of closing the menu");
    assert_eq!(menu.active, vec![0]);
}

#[test]
fn render_context_menu_level_renders_a_labeled_separator_as_a_header_without_a_hit() {
    let items = vec![ContextMenuItem { id: "header-1".into(), label: "Header".into(), separator: true, ..Default::default() }, ContextMenuItem { id: "leaf-1".into(), label: "Leaf".into(), ..Default::default() }];
    let menu = ContextMenuState { items: items.clone(), ..Default::default() };
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    render_context_menu_level(&mut draw, &mut atlas, &icons, &mut input, &theme, &menu, &menu.items, &[], 0.0, 0.0, 800.0, 600.0);
    assert!(input.staged_hits().iter().all(|hit| hit.control_id.as_deref() != Some("header-1")), "a labeled separator must stay non-interactive");
    assert!(input.staged_hits().iter().any(|hit| hit.control_id.as_deref() == Some("leaf-1")), "the leaf row after the header must still register a hit");
}

#[test]
fn render_context_menu_level_clips_to_viewport_height_and_scrolls_hidden_rows_into_view() {
    let items: Vec<ContextMenuItem> = (0..20).map(|index| ContextMenuItem { id: format!("item-{index}"), label: format!("Item {index}"), ..Default::default() }).collect();
    let theme = Theme::default();
    let row_h = theme.chip_row_height();
    let viewport_h = row_h * 4.0;
    let menu_at = |scroll_offset: f32| ContextMenuState { items: items.clone(), scroll_offset, ..Default::default() };
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();

    let mut input = InputState::<ActionDescriptor>::default();
    let menu = menu_at(0.0);
    render_context_menu_level(&mut draw, &mut atlas, &icons, &mut input, &theme, &menu, &menu.items, &[], 0.0, 0.0, 800.0, viewport_h);
    let visible_ids: Vec<String> = input.staged_hits().iter().filter_map(|hit| hit.control_id.clone()).collect();
    assert!(visible_ids.len() < items.len(), "expected the viewport clip to hide some rows, got {} of {}", visible_ids.len(), items.len());
    assert!(!visible_ids.contains(&"item-19".to_string()), "the last row should be scrolled out of view without scrolling");

    let mut input2 = InputState::<ActionDescriptor>::default();
    let menu2 = menu_at(row_h * 16.0);
    render_context_menu_level(&mut draw, &mut atlas, &icons, &mut input2, &theme, &menu2, &menu2.items, &[], 0.0, 0.0, 800.0, viewport_h);
    let scrolled_ids: Vec<String> = input2.staged_hits().iter().filter_map(|hit| hit.control_id.clone()).collect();
    assert!(scrolled_ids.contains(&"item-19".to_string()), "scrolling down should bring the last row into view");
}

#[test]
fn render_context_menu_level_flips_a_submenu_left_when_it_would_overflow_the_right_edge() {
    let theme = Theme::default();
    let child_items = vec![ContextMenuItem { id: "child-1".into(), label: "Child one".into(), ..Default::default() }];
    let parent_items = vec![ContextMenuItem { id: "menu.group.view".into(), label: "View".into(), children: child_items, ..Default::default() }];
    let menu = ContextMenuState { items: parent_items.clone(), active: vec![0], ..Default::default() };
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let viewport_w = 220.0;
    render_context_menu_level(&mut draw, &mut atlas, &icons, &mut input, &theme, &menu, &menu.items, &[], 0.0, 0.0, viewport_w, 600.0);
    let parent_w = context_menu_level_width(&parent_items, &theme);
    let child_hit = input.staged_hits().iter().find(|hit| hit.control_id.as_deref() == Some("child-1")).expect("submenu row registers a hit");
    assert!(child_hit.rect.x < parent_w, "expected the submenu to flip left of the parent row, got x={}", child_hit.rect.x);
}

/// ⚖️ LAW 12: the rows of a context menu follow `context-menu-rows/v1`: chip-row high, full width, no inset, single lines between them, the active row filled and emphasised.
#[test]
fn context_menu_rows_follow_the_language_neutral_cases() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/rows.cases.json")).expect("context menu rows cases");
    assert_eq!(cases["schema"], "context-menu-rows/v1");
    let theme = Theme::light();
    let row_h = semio_renderer_wgpu_shell_view_overlays::context_menu_row_height(&theme);
    assert!((row_h - theme.chip_row_height()).abs() < f32::EPSILON);
    for case in cases["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("id");
        let items: Vec<ContextMenuItem> = case["items"].as_array().expect("items").iter().enumerate().map(|(index, kind)| match kind.as_str().expect("kind") {
            "separator" => ContextMenuItem { id: format!("sep-{index}"), separator: true, ..Default::default() },
            "header" => ContextMenuItem { id: format!("head-{index}"), label: "Header".into(), separator: true, ..Default::default() },
            _ => ContextMenuItem { id: format!("item-{index}"), label: format!("Item {index}"), ..Default::default() },
        }).collect();
        let slots = semio_renderer_wgpu_shell_view_overlays::context_menu_row_slots(&items);
        let expected: Vec<Option<usize>> = case["rowYRows"].as_array().expect("rows").iter().map(|row| row.as_u64().map(|value| value as usize)).collect();
        assert_eq!(slots, expected, "{id}: row slots");
        let height = semio_renderer_wgpu_shell_view_overlays::context_menu_content_height(&items, &theme);
        assert!((height - case["contentRows"].as_u64().expect("content rows") as f32 * row_h).abs() < 0.001, "{id}: content height has no padding");
        let menu = ContextMenuState { x: 10.0, y: 10.0, items: items.clone(), active: items.iter().position(|item| !item.separator).map(|index| vec![index]).unwrap_or_default(), ..Default::default() };
        let ops = semio_renderer_wgpu_shell_view_overlays::context_menu_paint_ops(&theme, &menu, (800.0, 600.0));
        let lines = ops.iter().filter(|op| matches!(op, semio_renderer_wgpu_shell_view_overlays::ContextMenuPaintOp::Solid(rect, color) if (rect.h - theme.stroke_hairline).abs() < f32::EPSILON && color.r == theme.border_normal.r && color.g == theme.border_normal.g)).count();
        assert_eq!(lines as u64, case["lines"].as_u64().expect("lines"), "{id}: single lines between rows");
        let fills = ops.iter().filter(|op| matches!(op, semio_renderer_wgpu_shell_view_overlays::ContextMenuPaintOp::Solid(rect, color) if (rect.h - row_h).abs() < 0.001 && color.r == theme.row_hover.r && color.g == theme.row_hover.g)).count();
        assert_eq!(fills, usize::from(menu.active.first().is_some()), "{id}: only the active row is filled with the hover fill");
        assert!(!ops.iter().any(|op| matches!(op, semio_renderer_wgpu_shell_view_overlays::ContextMenuPaintOp::Icon { id, .. } if id == "check")), "{id}: no tick");
    }
}

fn text_selection_schema() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧬️schema/📝️text-selection-menu/🔣️.json")).expect("text-selection menu schema")
}

/// ⚖️ LAW: an editable selection offers every row of the shared text-selection schema in its order, with the schema's chords, icons and en and de labels.
#[test]
fn the_text_selection_menu_offers_the_shared_schemas_rows() {
    let schema = text_selection_schema();
    for is_de in [false, true] {
        let items = semio_renderer_wgpu_shell_input::text_selection_menu_items((10.0, 20.0), true, true, is_de, |keys| keys.to_string());
        let rows = schema["rows"].as_array().expect("rows");
        assert_eq!(items.iter().map(|item| item.id.as_str()).collect::<Vec<_>>(), rows.iter().map(|row| row["id"].as_str().unwrap()).collect::<Vec<_>>());
        for (item, row) in items.iter().zip(rows) {
            assert_eq!(item.separator, row["verb"].is_null());
            if item.separator {
                continue;
            }
            assert_eq!(item.label, row["label"][if is_de { "de" } else { "en" }].as_str().unwrap());
            assert_eq!(item.shortcut.as_deref(), row["chord"].as_str());
            assert_eq!(item.icon.as_deref(), row["icon"].as_str());
            assert!(!item.disabled);
        }
    }
}

/// ⚖️ LAW: a readable target offers only Copy and Select all, and Copy and Cut are disabled without a selection while Paste and Select all stay enabled.
#[test]
fn the_text_selection_menu_gates_rows_by_editability_and_selection() {
    let readable = semio_renderer_wgpu_shell_input::text_selection_menu_items((0.0, 0.0), false, false, false, |keys| keys.to_string());
    assert_eq!(readable.iter().filter(|item| !item.separator).map(|item| item.id.as_str()).collect::<Vec<_>>(), ["text-copy", "text-select-all"]);
    assert!(readable.iter().find(|item| item.id == "text-copy").unwrap().disabled);
    assert!(!readable.iter().find(|item| item.id == "text-select-all").unwrap().disabled);
    let editable = semio_renderer_wgpu_shell_input::text_selection_menu_items((0.0, 0.0), true, false, false, |keys| keys.to_string());
    assert!(editable.iter().find(|item| item.id == "text-cut").unwrap().disabled);
    assert!(!editable.iter().find(|item| item.id == "text-paste").unwrap().disabled);
}

/// ⚖️ LAW: every row's action names its verb and carries the press point; no other action is mistaken for a text-selection verb.
#[test]
fn a_text_selection_row_names_its_verb_and_carries_the_press_point() {
    let items = semio_renderer_wgpu_shell_input::text_selection_menu_items((3.0, 4.0), true, true, false, |keys| keys.to_string());
    let verbs: Vec<_> = items.iter().filter_map(|item| item.action.as_ref()).map(|action| {
        assert_eq!((action.args.as_ref().and_then(|args| args.get("x")).and_then(DslValue::as_f64), action.args.as_ref().and_then(|args| args.get("y")).and_then(DslValue::as_f64)), (Some(3.0), Some(4.0)));
        format!("{:?}", semio_renderer_wgpu_shell_input::text_selection_verb_of(action))
    }).collect();
    assert_eq!(verbs, ["Some(Cut)", "Some(Copy)", "Some(Paste)", "Some(SelectAll)"]);
    assert_eq!(semio_renderer_wgpu_shell_input::text_selection_verb_of(&ActionDescriptor { controller_id: "framework".into(), action: "copy".into(), args: None }), None);
}
