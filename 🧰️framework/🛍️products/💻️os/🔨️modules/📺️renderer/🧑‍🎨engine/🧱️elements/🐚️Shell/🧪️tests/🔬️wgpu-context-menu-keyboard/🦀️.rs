
use super::*;

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
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.context_menu = Some(ContextMenuState {
        items: vec![ContextMenuItem { id: "menu.group.view".into(), label: "View".into(), children: vec![ContextMenuItem { id: "menu.group.view.child".into(), label: "Child".into(), ..Default::default() }], ..Default::default() }],
        ..Default::default()
    });
    let hit = HitTarget { rect: Rect::new(0.0, 0.0, 10.0, 10.0), event: None, control_id: Some("menu.group.view".into()), kind: HitKind::ContextMenu, drag_axis: None, drag_data: None };
    let consumed = semio_framework_async::block_on(shell.handle_shell_hit(&hit)).expect("group-row click never errors");
    assert!(consumed);
    let menu = shell.context_menu.as_ref().expect("a group-row click opens its submenu instead of closing the menu");
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
    ShellState::render_context_menu_level(&mut draw, &mut atlas, &icons, &mut input, &theme, &menu, &menu.items, &[], 0.0, 0.0, 800.0, 600.0);
    assert!(input.hit_targets.iter().all(|hit| hit.control_id.as_deref() != Some("header-1")), "a labeled separator must stay non-interactive");
    assert!(input.hit_targets.iter().any(|hit| hit.control_id.as_deref() == Some("leaf-1")), "the leaf row after the header must still register a hit");
}

#[test]
fn render_context_menu_level_clips_to_viewport_height_and_scrolls_hidden_rows_into_view() {
    let items: Vec<ContextMenuItem> = (0..20).map(|index| ContextMenuItem { id: format!("item-{index}"), label: format!("Item {index}"), ..Default::default() }).collect();
    let theme = Theme::default();
    let row_h = theme.control_height;
    let viewport_h = row_h * 4.0;
    let menu_at = |scroll_offset: f32| ContextMenuState { items: items.clone(), scroll_offset, ..Default::default() };
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();

    let mut input = InputState::<ActionDescriptor>::default();
    let menu = menu_at(0.0);
    ShellState::render_context_menu_level(&mut draw, &mut atlas, &icons, &mut input, &theme, &menu, &menu.items, &[], 0.0, 0.0, 800.0, viewport_h);
    let visible_ids: Vec<String> = input.hit_targets.iter().filter_map(|hit| hit.control_id.clone()).collect();
    assert!(visible_ids.len() < items.len(), "expected the viewport clip to hide some rows, got {} of {}", visible_ids.len(), items.len());
    assert!(!visible_ids.contains(&"item-19".to_string()), "the last row should be scrolled out of view without scrolling");

    let mut input2 = InputState::<ActionDescriptor>::default();
    let menu2 = menu_at(row_h * 16.0);
    ShellState::render_context_menu_level(&mut draw, &mut atlas, &icons, &mut input2, &theme, &menu2, &menu2.items, &[], 0.0, 0.0, 800.0, viewport_h);
    let scrolled_ids: Vec<String> = input2.hit_targets.iter().filter_map(|hit| hit.control_id.clone()).collect();
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
    ShellState::render_context_menu_level(&mut draw, &mut atlas, &icons, &mut input, &theme, &menu, &menu.items, &[], 0.0, 0.0, viewport_w, 600.0);
    let parent_w = ShellState::context_menu_level_width(&parent_items, &theme);
    let child_hit = input.hit_targets.iter().find(|hit| hit.control_id.as_deref() == Some("child-1")).expect("submenu row registers a hit");
    assert!(child_hit.rect.x < parent_w, "expected the submenu to flip left of the parent row, got x={}", child_hit.rect.x);
}
