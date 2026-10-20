/// 🧲️ Window docking: tabs into every corner and zone with preview and committed geometry, pointer and keyboard drags, cancellation, identity-owned controls and splitter capture.

fn docking_vectors() -> serde_json::Value { serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../🧫️fixtures/🪟️windows/🔣️.json")).unwrap() }

fn docking_placement(value: &str) -> semio_framework_ui_tui_core::docking::DockPlacement {
    use semio_framework_ui_tui_core::docking::DockPlacement::*;
    use semio_framework_ui_tui_core::docking::WindowStackCorner::*;
    match value { "tabs-top-left" => Corner(TopLeft), "tabs-top-right" => Corner(TopRight), "tabs-bottom-left" => Corner(BottomLeft), "tabs-bottom-right" => Corner(BottomRight), "left" => Left, "right" => Right, "top" => Top, "bottom" => Bottom, "tabs" => Tabs, _ => panic!("unknown docking placement") }
}

fn tab_position(dashboard: &Dashboard, tui: &Tui, id: &str) -> Pos {
    for window in &dashboard.windows {
        let node = tui.scene.node(window.chrome);
        let semio_framework_ui_tui_frame::scene::NodeContent::Chrome(chrome) = &node.content else { continue };
        let ChromeState::Window(state) = chrome else { continue };
        let Some(index) = state.tab_ids.iter().position(|tab| tab == id) else { continue };
        let rect = tui.scene.rect(window.chrome);
        for y in rect.y..rect.y + rect.height {
            for x in rect.x..rect.x + rect.width {
                if chrome.window_target(rect, Pos { x, y }) == Some(semio_framework_ui_tui_frame::chrome::WindowHit::Tab(index)) { return Pos { x, y }; }
            }
        }
    }
    panic!("no visible tab for {id}")
}

fn docking_ghost(tui: &mut Tui, placement: semio_framework_ui_tui_core::docking::DockPlacement) -> (semio_framework_ui_tui_core::geometry::Rect, semio_framework_ui_tui_core::cell::CellBuffer) {
    let (actual, rect) = tui.docking_preview().expect("destination tab preview");
    assert_eq!(actual, placement);
    assert!(rect.width > 0 && rect.height > 0);
    tui.render();
    (rect, tui.frame().clone())
}

fn assert_committed_ghost(dashboard: &Dashboard, tui: &mut Tui, source: &str, ghost: &(semio_framework_ui_tui_core::geometry::Rect, semio_framework_ui_tui_core::cell::CellBuffer)) {
    use semio_framework_ui_tui_frame::chrome::WindowHit;
    use semio_framework_ui_tui_core::geometry::Rect;
    tui.render();
    let chrome = dashboard.windows.iter().find(|window| window.id == source).unwrap().chrome;
    let rect = tui.scene.rect(chrome);
    let semio_framework_ui_tui_frame::scene::NodeContent::Chrome(state @ ChromeState::Window(window)) = &tui.scene.node(chrome).content else { panic!("window chrome") };
    let index = window.tab_ids.iter().position(|id| id == source).unwrap();
    let cells: Vec<_> = (rect.y..rect.y + rect.height).flat_map(|y| (rect.x..rect.x + rect.width).map(move |x| Pos { x, y })).filter(|pos| matches!(state.window_target(rect, *pos), Some(WindowHit::Tab(tab) | WindowHit::Close(tab) | WindowHit::Maximize(tab)) if tab == index)).collect();
    let left = cells.iter().map(|pos| pos.x).min().unwrap() - 1;
    let right = cells.iter().map(|pos| pos.x).max().unwrap() + 1;
    let text_y = cells[0].y;
    let compact = text_y == rect.y;
    let expected = Rect::new(left, if compact { text_y } else { text_y - 1 }, right - left + 1, if compact { 1 } else { 3 });
    assert_eq!(ghost.0, expected, "ghost bounds agree with committed tab hit geometry");
    for y in expected.y..expected.y + expected.height { for x in expected.x..expected.x + expected.width {
        assert_eq!(ghost.1.get(x, y).unwrap().ch, tui.frame().get(x, y).unwrap().ch, "committed ghost {source} at {x}:{y}");
    } }
}

#[test]
fn docking_window_handle_tooltips_follow_dashboard_language() {
    for language in ["en", "de"] {
        let (dashboard, mut tui) = small_view(language);
        tui.render();
        let source = &dashboard.windows[0].id;
        let grip = tab_position(&dashboard, &tui, source);
        let node = dashboard.windows[0].chrome;
        let semio_framework_ui_tui_frame::scene::NodeContent::Chrome(chrome) = &tui.scene.node(node).content else { panic!("chrome") };
        assert_eq!(dashboard.text().chrome_drag.as_str(), if language == "en" { "Drag window" } else { "Fenster ziehen" });
        assert_eq!(tui.frame().get(grip.x, grip.y).unwrap().ch, '⠿');
        assert_eq!(chrome.tooltip(tui.scene.rect(node), grip).as_deref(), Some(dashboard.text().chrome_drag.as_str()));
        assert_eq!(chrome.window_control_at(tui.scene.rect(node), grip), None);
    }
}

#[test]
fn docking_tabs_reach_all_four_corners_with_preview_and_identity_owned_controls() {
    use semio_framework_ui_tui_frame::layout::{stack_hosting};
    use semio_framework_ui_tui_core::docking::{DockPlacement, WindowStackCorner};
    use semio_framework_ui_tui_frame::chrome::WindowHit;
    let corners = [WindowStackCorner::TopLeft, WindowStackCorner::TopRight, WindowStackCorner::BottomLeft, WindowStackCorner::BottomRight];
    for corner in corners {
        let (mut dashboard, mut tui) = view();
        dashboard.perform(&mut tui, Action::NewTask);
        dashboard.frame(&mut tui);
        let source = dashboard.windows[1].id.clone();
        let root = dashboard.windows[1].root;
        if corner == WindowStackCorner::TopLeft { semio_framework_ui_tui_frame::layout::dock_window(&mut dashboard.layout, &source, &source, DockPlacement::Corner(WindowStackCorner::BottomRight), None); dashboard.remount(&mut tui); dashboard.frame(&mut tui); }
        let start = tab_position(&dashboard, &tui, &source);
        let area = tui.scene.rect(dashboard.windows[1].chrome);
        let drop = Pos { x: if corner.is_left() { area.x + 1 } else { area.x + area.width - 2 }, y: if corner.is_top() { area.y + 1 } else { area.y + area.height - 2 } };
        let before = dashboard.layout.clone();
        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), start), pointer(MouseKind::Drag(MouseButton::Left), drop)]);
        let ghost = docking_ghost(&mut tui, DockPlacement::Corner(corner));
        let preview = ghost.0;
        assert_eq!(dashboard.layout, before);
        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Up(MouseButton::Left), drop)]);
        assert_committed_ghost(&dashboard, &mut tui, &source, &ghost);
        let stack = stack_hosting(&dashboard.layout, &source).unwrap();
        let index = stack.children.iter().position(|node| node.window_kind_id == source).unwrap();
        assert_eq!(stack.children[index].corner, Some(corner));
        assert_eq!(dashboard.windows[1].root, root);
        let node = dashboard.windows[1].chrome;
        let rect = tui.scene.rect(node);
        let semio_framework_ui_tui_frame::scene::NodeContent::Chrome(chrome) = &tui.scene.node(node).content else { panic!("chrome") };
        let mut controls = Vec::new();
        for y in rect.y..rect.y + rect.height { for x in rect.x..rect.x + rect.width {
            if matches!(chrome.window_target(rect, Pos { x, y }), Some(WindowHit::Maximize(tab) | WindowHit::Close(tab)) if tab == index) { controls.push(Pos { x, y }); }
        }}
        assert_eq!(controls.len(), 2);
        println!("[DEBUG] corner={corner:?} source={source} preview={preview:?} tab-controls={controls:?}");
    }
}

#[test]
fn docking_interleaved_layout_events_keep_the_held_tab_identity() {
    use semio_framework_ui_tui_frame::layout::stack_hosting;
    let (mut dashboard, mut tui) = view();
    dashboard.perform(&mut tui, Action::NewTask);
    dashboard.perform(&mut tui, Action::NewTask);
    dashboard.frame(&mut tui);
    let source = dashboard.windows[1].id.clone();
    let target = dashboard.windows[0].id.clone();
    let other = dashboard.windows[2].id.clone();
    let body = dashboard.windows[1].root;
    let start = tab_position(&dashboard, &tui, &source);
    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), start)]);
    dashboard.perform(&mut tui, Action::TabRight);
    dashboard.frame(&mut tui);
    let drop = tab_position(&dashboard, &tui, &target);
    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Drag(MouseButton::Left), drop), pointer(MouseKind::Up(MouseButton::Left), drop)]);
    let group = stack_hosting(&dashboard.layout, &source).unwrap();
    assert_eq!(group.children.iter().map(|node| node.window_kind_id.as_str()).collect::<Vec<_>>(), vec![source.as_str(), target.as_str(), other.as_str()]);
    assert_eq!(dashboard.windows[1].root, body);
    println!("[DEBUG] a layout event during pointer capture retained held identity={source} and its body");
}

#[test]
fn docking_corner_pointer_moves_between_groups_and_orders_within_the_chosen_corner() {
    use semio_framework_ui_tui_frame::layout::{dock_window, stack_hosting};
    use semio_framework_ui_tui_core::docking::{DockPlacement, WindowStackCorner::*};
    for corner in [TopLeft, TopRight, BottomLeft, BottomRight] {
        let (mut dashboard, mut tui) = view();
        dashboard.perform(&mut tui, Action::SplitRight);
        dashboard.frame(&mut tui);
        let source = dashboard.windows[1].id.clone();
        let target = dashboard.windows[0].id.clone();
        let body = dashboard.windows[1].root;
        let start = tab_position(&dashboard, &tui, &source);
        let area = tui.scene.rect(dashboard.windows[0].chrome);
        let drop = Pos { x: if corner.is_left() { area.x + 1 } else { area.x + area.width - 2 }, y: if corner.is_top() { area.y + 1 } else { area.y + area.height - 2 } };
        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), start), pointer(MouseKind::Drag(MouseButton::Left), drop)]);
        let ghost = docking_ghost(&mut tui, DockPlacement::Corner(corner));
        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Up(MouseButton::Left), drop)]);
        assert_committed_ghost(&dashboard, &mut tui, &source, &ghost);
        let group = stack_hosting(&dashboard.layout, &target).unwrap();
        assert_eq!(group.children.len(), 2);
        assert_eq!(group.children.iter().find(|node| node.window_kind_id == source).unwrap().corner, Some(corner));
        assert_eq!(dashboard.windows[1].root, body);
        assert_eq!(dashboard.focused_index(&tui), 1);
        dashboard.perform(&mut tui, Action::NewTask);
        let other = dashboard.windows[2].id.clone();
        assert!(dock_window(&mut dashboard.layout, &other, &target, DockPlacement::Corner(corner), Some(0)));
        assert!(dock_window(&mut dashboard.layout, &source, &target, DockPlacement::Corner(corner), Some(0)));
        let group = stack_hosting(&dashboard.layout, &target).unwrap();
        let order: Vec<_> = group.children.iter().filter(|node| node.corner.unwrap_or_default() == corner).map(|node| node.window_kind_id.as_str()).collect();
        assert_eq!(&order[..2], &[source.as_str(), other.as_str()]);
        println!("[DEBUG] cross-group corner={corner:?} retained body and focus; corner-local order={order:?}");
    }
}

#[test]
fn docking_corner_contract_covers_cross_group_and_sole_tab_moves() {
    use semio_framework_ui_tui_frame::layout::{create_default_layout, dock_window, stack_hosting};
    use semio_framework_ui_tui_core::docking::{DockPlacement};
    use semio_framework_ui_tui_core::geometry::Rect;
    let vectors = docking_vectors();
    let area = Rect::new(10, 4, 80, 40);
    for vector in vectors["corners"].as_array().unwrap() {
        let placement = docking_placement(vector["placement"].as_str().unwrap());
        let preview = placement.preview(area);
        assert_eq!(serde_json::json!([preview.x, preview.y, preview.width, preview.height]), vector["rect"]);
        assert_eq!(DockPlacement::at(area, Pos { x: vector["point"][0].as_u64().unwrap() as u16, y: vector["point"][1].as_u64().unwrap() as u16 }), Some(placement));
        let DockPlacement::Corner(corner) = placement else { panic!("corner") };
        for ids in [vec!["a".into()], vec!["a".into(), "b".into()]] {
            let mut layout = create_default_layout(&ids, "row", None, None);
            let target = ids.last().unwrap();
            assert!(dock_window(&mut layout, "a", target, placement, Some(0)));
            let stack = stack_hosting(&layout, "a").unwrap();
            assert_eq!(stack.children.iter().find(|node| node.window_kind_id == "a").unwrap().corner, Some(corner));
            assert_eq!(stack.children.len(), ids.len());
        }
    }
}

#[test]
fn keyboard_docking_places_tabs_in_all_corners_and_localizes_every_action() {
    use semio_framework_ui_tui_frame::layout::{stack_hosting};
    use semio_framework_ui_tui_core::docking::{WindowStackCorner::*};
    for (action, corner) in [(Action::TabTopLeft, TopLeft), (Action::TabTopRight, TopRight), (Action::TabBottomLeft, BottomLeft), (Action::TabBottomRight, BottomRight)] {
        for language in ["en", "de"] {
            let (mut dashboard, mut tui) = small_view(language);
            let id = dashboard.windows[0].id.clone();
            assert_eq!(Action::parse(action.id()), Some(action));
            assert!(!dashboard.text().action(action.id()).is_empty());
            assert!(dashboard.keymap.keys_label(dashboard_preferences::keymap::Scope::Prefix, action.id()).is_some());
            let digit = match action { Action::TabTopLeft => '1', Action::TabTopRight => '2', Action::TabBottomLeft => '3', _ => '4' };
            press(&mut dashboard, &mut tui, &[ctrl('b'), Event::Key(KeyEvent { key: Key::Char(digit), mods: mods::ALT })]);
            assert_eq!(stack_hosting(&dashboard.layout, &id).unwrap().children[0].corner, Some(corner));
            assert_eq!(dashboard.focused_index(&tui), 0);
        }
    }
}

#[test]
fn docking_fullscreen_and_close_on_an_inactive_tab_target_its_identity() {
    use semio_framework_ui_tui_frame::chrome::WindowHit;
    let (mut dashboard, mut tui) = view();
    dashboard.perform(&mut tui, Action::NewTask);
    dashboard.focus_window(&mut tui, 0);
    dashboard.frame(&mut tui);
    let source = dashboard.windows[1].id.clone();
    let find = |dashboard: &Dashboard, tui: &Tui, wanted: WindowHit| {
        let chrome = dashboard.windows[dashboard.focused_index(tui)].chrome;
        let rect = tui.scene.rect(chrome);
        let semio_framework_ui_tui_frame::scene::NodeContent::Chrome(state) = &tui.scene.node(chrome).content else { panic!("chrome") };
        (rect.y..rect.y + rect.height).flat_map(|y| (rect.x..rect.x + rect.width).map(move |x| Pos { x, y })).find(|pos| state.window_target(rect, *pos) == Some(wanted)).unwrap()
    };
    let maximize = find(&dashboard, &tui, WindowHit::Maximize(1));
    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), maximize)]);
    assert_eq!(dashboard.layout.zoomed.as_deref(), Some(source.as_str()));
    assert_eq!(dashboard.focused_index(&tui), 1);
    for index in [0, 1] {
        let tab = find(&dashboard, &tui, WindowHit::Tab(index));
        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), tab)]);
        assert_eq!(dashboard.layout.zoomed.as_deref(), Some(dashboard.windows[index].id.as_str()));
        assert_eq!(dashboard.focused_index(&tui), index);
    }
    let restore = find(&dashboard, &tui, WindowHit::Maximize(0));
    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), restore)]);
    assert!(dashboard.layout.zoomed.is_none());
    assert_eq!(dashboard.focused_index(&tui), 0);
    dashboard.focus_window(&mut tui, 0);
    dashboard.frame(&mut tui);
    let close = find(&dashboard, &tui, WindowHit::Close(1));
    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), close)]);
    assert!(!dashboard.windows.iter().any(|window| window.id == source));
    assert_eq!(dashboard.focused_index(&tui), 0);
    dashboard.perform(&mut tui, Action::NewTask);
    dashboard.perform(&mut tui, Action::NewTask);
    let retained = dashboard.windows[2].id.clone();
    dashboard.perform(&mut tui, Action::Zoom);
    dashboard.frame(&mut tui);
    for remaining in [2, 1] {
        let close = find(&dashboard, &tui, WindowHit::Close(0));
        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), close)]);
        assert_eq!(dashboard.windows.len(), remaining);
        assert_eq!(dashboard.windows[dashboard.focused_index(&tui)].id, retained);
        assert_eq!(dashboard.layout.zoomed.as_deref(), (remaining > 1).then_some(retained.as_str()));
    }
    dashboard.perform(&mut tui, Action::Zoom);
    assert!(dashboard.layout.zoomed.is_none(), "a lone window has no fullscreen action");
    println!("[DEBUG] inactive tab fullscreen activated its identity; inactive close retained active focus and fullscreen until only one tab remained");
}

#[test]
fn docking_preview_matches_the_committed_split_geometry_in_every_zone() {
    use semio_framework_ui_tui_core::geometry::Rect;
    use semio_framework_ui_tui_frame::layout::{create_default_layout, dock_window, push_window_to_stack, solve_window_layout, WindowLayoutWindowNode};
    use semio_framework_ui_tui_core::docking::{DockPlacement};
    let vectors = docking_vectors();
    let area = Rect::new(10, 4, 80, 40);
    for vector in vectors["previews"].as_array().unwrap() {
        let placement = docking_placement(vector["placement"].as_str().unwrap());
        if let Some(point) = vector.get("point") {
            assert_eq!(DockPlacement::at(area, Pos { x: point[0].as_u64().unwrap() as u16, y: point[1].as_u64().unwrap() as u16 }), Some(placement));
        }
        let preview = placement.preview(area);
        assert_eq!(serde_json::json!([preview.x, preview.y, preview.width, preview.height]), vector["rect"]);
        let mut layout = create_default_layout(&["b".into()], "row", None, None);
        push_window_to_stack(&mut layout, "b", WindowLayoutWindowNode { window_kind_id: "a".into(), title: Some("Held task".into()), corner: None });
        assert!(dock_window(&mut layout, "a", "b", placement, None));
        let measures = solve_window_layout(&layout, area);
        let held = measures.iter().find(|measure| measure.window_kind_id == "a").unwrap();
        assert_eq!(held.rect, preview);
        assert_eq!(held.title.as_deref(), Some("Held task"));
    }
    for vector in vectors["bodyZones"].as_array().unwrap() {
        assert_eq!(DockPlacement::at(area, Pos { x: vector["point"][0].as_u64().unwrap() as u16, y: vector["point"][1].as_u64().unwrap() as u16 }), Some(docking_placement(vector["placement"].as_str().unwrap())));
    }
    assert_eq!(DockPlacement::at(area, Pos { x: 0, y: 0 }), None);
}

#[test]
fn docking_commands_preserve_windows_prune_empty_groups_and_reject_invalid_targets() {
    use semio_framework_ui_tui_frame::layout::{create_default_layout, dock_window, push_window_to_stack, stack_hosting, WindowLayoutRoot, WindowLayoutWindowNode};
    let vectors = docking_vectors();
    let mut layout = create_default_layout(&["a".into()], "row", None, None);
    for id in ["b", "c"] { push_window_to_stack(&mut layout, "a", WindowLayoutWindowNode { window_kind_id: id.into(), title: Some(id.into()), corner: None }); }
    for command in vectors["commands"].as_array().unwrap() {
        assert!(dock_window(&mut layout, command["source"].as_str().unwrap(), command["target"].as_str().unwrap(), docking_placement(command["placement"].as_str().unwrap()), command["tabIndex"].as_u64().map(|value| value as usize)));
    }
    assert!(matches!(layout.root, WindowLayoutRoot::Stack(_)));
    let group = stack_hosting(&layout, "b").unwrap();
    assert_eq!(serde_json::json!(group.children.iter().map(|node| node.window_kind_id.clone()).collect::<Vec<_>>()), vectors["finalTabs"]);
    for command in vectors["invalid"].as_array().unwrap() {
        let before = layout.clone();
        assert!(!dock_window(&mut layout, command["source"].as_str().unwrap(), command["target"].as_str().unwrap(), docking_placement(command["placement"].as_str().unwrap()), None));
        assert_eq!(layout, before);
    }
}

#[test]
fn real_pointer_drags_preview_every_split_then_join_tabs_without_recreating_their_bodies() {
    use semio_framework_ui_tui_frame::layout::{solve_window_layout};
    use semio_framework_ui_tui_core::docking::{DockPlacement};
    for placement in [DockPlacement::Left, DockPlacement::Right, DockPlacement::Top, DockPlacement::Bottom] {
        let (mut dashboard, mut tui) = view();
        dashboard.perform(&mut tui, Action::NewTask);
        dashboard.frame(&mut tui);
        let source = dashboard.windows[1].id.clone();
        let root = dashboard.windows[1].root;
        let before = dashboard.layout.clone();
        let start = tab_position(&dashboard, &tui, &source);
        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), start)]);
        let area = tui.scene.rect(dashboard.windows[1].chrome);
        let center = Pos { x: area.x + area.width / 2, y: area.y + area.height / 2 };
        let drop = match placement { DockPlacement::Left => Pos { x: area.x + 1, ..center }, DockPlacement::Right => Pos { x: area.x + area.width - 2, ..center }, DockPlacement::Top => Pos { y: area.y + 4, ..center }, DockPlacement::Bottom => Pos { y: area.y + area.height - 2, ..center }, _ => center };
        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Drag(MouseButton::Left), drop)]);
        let ghost = docking_ghost(&mut tui, placement);
        assert_eq!(dashboard.layout, before, "preview must not change the layout");
        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Up(MouseButton::Left), drop)]);
        assert_committed_ghost(&dashboard, &mut tui, &source, &ghost);
        assert!(tui.docking_preview().is_none());
        assert_eq!(dashboard.windows[1].root, root);
        assert_eq!(dashboard.focused_index(&tui), 1);
        let measure = solve_window_layout(&dashboard.layout, area).into_iter().find(|window| window.window_kind_id == source).unwrap();
        assert_eq!(measure.rect, placement.preview(area));
        let start = tab_position(&dashboard, &tui, &source);
        let target = tui.scene.rect(dashboard.windows[0].chrome);
        let center = Pos { x: target.x + 2, y: target.y + 1 };
        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), start), pointer(MouseKind::Drag(MouseButton::Left), center)]);
        let ghost = docking_ghost(&mut tui, DockPlacement::Corner(semio_framework_ui_tui_core::docking::WindowStackCorner::TopLeft));
        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Up(MouseButton::Left), center)]);
        assert_committed_ghost(&dashboard, &mut tui, &source, &ghost);
        assert_eq!(super::windows::stack_of(&dashboard.layout, &source).len(), 2);
        assert_eq!(tui.scene.rect(dashboard.windows[1].chrome), area);
        assert_eq!(dashboard.windows[1].root, root);
    }
}

#[test]
fn escape_focus_loss_resize_and_an_outside_release_cancel_docking() {
    for cancel in [named(Key::Esc), Event::FocusLost, Event::Resize(Size { width: 120, height: 40 }), pointer(MouseKind::Up(MouseButton::Left), Pos { x: 0, y: 0 })] {
        let (mut dashboard, mut tui) = view();
        dashboard.perform(&mut tui, Action::NewTask);
        dashboard.frame(&mut tui);
        let before = dashboard.layout.clone();
        let source = dashboard.windows[1].id.clone();
        let start = tab_position(&dashboard, &tui, &source);
        let area = tui.scene.rect(dashboard.windows[1].chrome);
        let drop = Pos { x: area.x + area.width - 2, y: area.y + area.height / 2 };
        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), start), pointer(MouseKind::Drag(MouseButton::Left), drop)]);
        assert!(tui.docking_preview().is_some());
        press(&mut dashboard, &mut tui, &[cancel, pointer(MouseKind::Up(MouseButton::Left), drop)]);
        assert!(tui.docking_preview().is_none());
        assert_eq!(dashboard.layout, before);
    }
}

#[test]
fn keyboard_docking_and_tab_reordering_have_localized_customizable_actions() {
    let (mut dashboard, mut tui) = view();
    dashboard.perform(&mut tui, Action::NewTask);
    let source = dashboard.windows[1].id.clone();
    for action in [Action::DockLeft, Action::DockRight, Action::DockTop, Action::DockBottom] {
        dashboard.perform(&mut tui, action);
        dashboard.frame(&mut tui);
        assert_eq!(super::windows::stack_of(&dashboard.layout, &source).len(), 1);
        dashboard.perform(&mut tui, Action::JoinWindow);
        assert_eq!(super::windows::stack_of(&dashboard.layout, &source).len(), 2);
    }
    dashboard.perform(&mut tui, Action::TabLeft);
    assert_eq!(super::windows::stack_of(&dashboard.layout, &source)[0], source);
    dashboard.perform(&mut tui, Action::TabRight);
    assert_eq!(super::windows::stack_of(&dashboard.layout, &source)[1], source);
    for action in [Action::DockLeft, Action::DockRight, Action::DockTop, Action::DockBottom, Action::JoinWindow, Action::TabLeft, Action::TabRight] {
        assert!(dashboard_labels::labels(ui_locale::Locale::parse("de").unwrap()).try_action(action.id()).is_some());
    }
}

#[test]
fn every_window_docking_scenario_has_a_native_test() { every_scenario_of_the_window_view_features_is_proved_by_a_test(); }

#[test]
fn docking_a_live_terminal_preserves_its_session_output_and_reports_the_new_pty_size() {
    let (mut dashboard, mut tui) = view();
    dashboard.update_session(&mut tui, session("held-task", label("test", "quiz", ""), SessionStatus::Running, None));
    dashboard.focus_window(&mut tui, 1);
    dashboard.feed(&mut tui, 1, "HELD-OUTPUT");
    dashboard.frame(&mut tui);
    let terminal = dashboard.windows[1].list;
    let source = dashboard.windows[1].id.clone();
    let start = tab_position(&dashboard, &tui, &source);
    let area = tui.scene.rect(dashboard.windows[1].chrome);
    let drop = Pos { x: area.x + area.width - 2, y: area.y + area.height / 2 };
    let before = dashboard.link.outbox.len();
    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), start), pointer(MouseKind::Drag(MouseButton::Left), drop)]);
    assert_eq!(dashboard.link.outbox.len(), before, "preview cannot resize the child");
    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Up(MouseButton::Left), drop)]);
    assert_eq!(dashboard.windows[1].list, terminal);
    assert_eq!(dashboard.windows[1].session().unwrap().session_id, "held-task");
    let rect = tui.scene.rect(terminal);
    assert!(dashboard.link.outbox[before..].iter().any(|message| matches!(message, ClientMsg::Resize { session_id, cols, rows } if session_id == "held-task" && (*cols, *rows) == (rect.width, rect.height))));
    assert!(!dashboard.link.outbox[before..].iter().any(|message| matches!(message, ClientMsg::SpawnGroup { .. } | ClientMsg::Stop { .. } | ClientMsg::Kill { .. })));
    if let Some(semio_framework_ui_tui_widgets::widget::WidgetState::Terminal(state)) = tui.scene.node_mut(terminal).widget() {
        let line: String = (0..11).filter_map(|x| state.screen.cell_at(x, 0).map(|cell| cell.ch)).collect();
        assert_eq!(line, "HELD-OUTPUT");
    } else { panic!("the same terminal survives docking"); }
}

#[test]
fn docking_survives_zoom_and_removing_the_held_window_cancels_its_preview() {
    let (mut dashboard, mut tui) = view();
    dashboard.perform(&mut tui, Action::NewTask);
    dashboard.perform(&mut tui, Action::Zoom);
    dashboard.frame(&mut tui);
    let source = dashboard.windows[1].id.clone();
    let start = tab_position(&dashboard, &tui, &source);
    let area = tui.scene.rect(dashboard.windows[1].chrome);
    let drop = Pos { x: area.x + area.width - 2, y: area.y + area.height / 2 };
    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), start), pointer(MouseKind::Drag(MouseButton::Left), drop), pointer(MouseKind::Up(MouseButton::Left), drop)]);
    assert!(dashboard.layout.zoomed.is_none());
    dashboard.perform(&mut tui, Action::JoinWindow);
    dashboard.frame(&mut tui);
    let start = tab_position(&dashboard, &tui, &source);
    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), start), pointer(MouseKind::Drag(MouseButton::Left), drop)]);
    assert!(tui.docking_preview().is_some());
    assert!(!dashboard.close_window(&mut tui, 1));
    dashboard.frame(&mut tui);
    assert!(tui.docking_preview().is_none());
    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Up(MouseButton::Left), drop)]);
    assert_eq!(dashboard.windows.len(), 1);
}

#[test]
fn nested_splits_preserve_the_weight_of_their_existing_sibling_slot() {
    use semio_framework_ui_tui_core::geometry::Rect;
    use semio_framework_ui_tui_frame::layout::{create_default_layout, solve_window_layout, split_window};
    let mut layout = create_default_layout(&["a".into(), "b".into()], "row", Some(&[4.0, 1.0]), None);
    let area = Rect::new(0, 0, 160, 48);
    let before = solve_window_layout(&layout, area).into_iter().find(|window| window.window_kind_id == "b").unwrap().rect;
    assert!(split_window(&mut layout, "a", "column", "c", None));
    let after = solve_window_layout(&layout, area).into_iter().find(|window| window.window_kind_id == "b").unwrap().rect;
    assert_eq!(before, after);
}

#[test]
fn docking_changes_keyboard_focus_order_to_match_the_visible_layout() {
    let (mut dashboard, mut tui) = view();
    dashboard.perform(&mut tui, Action::NewTask);
    let first = dashboard.windows[0].id.clone();
    let second = dashboard.windows[1].id.clone();
    dashboard.perform(&mut tui, Action::DockLeft);
    assert_eq!(dashboard.window_order(), [second.clone(), first.clone()]);
    dashboard.cycle_window(&mut tui, 1);
    assert_eq!(dashboard.focused(&tui).unwrap().id, first);
    dashboard.cycle_window(&mut tui, 1);
    assert_eq!(dashboard.focused(&tui).unwrap().id, second);
}

#[test]
fn splitter_drag_keeps_its_capture_across_each_layout_remount() {
    let (mut dashboard, mut tui) = view();
    dashboard.perform(&mut tui, Action::SplitRight);
    dashboard.frame(&mut tui);
    let left = tui.scene.rect(dashboard.windows[0].chrome);
    let start = Pos { x: left.x + left.width, y: left.y + left.height / 2 };
    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Down(MouseButton::Left), start)]);
    for delta in [3, 7, 5] {
        press(&mut dashboard, &mut tui, &[pointer(MouseKind::Drag(MouseButton::Left), Pos { x: start.x + delta, ..start })]);
        assert_eq!(tui.scene.rect(dashboard.windows[0].chrome).width, left.width + delta, "capture must survive remount at delta {delta}");
    }
    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Up(MouseButton::Left), Pos { x: start.x + 5, ..start })]);
    let width = tui.scene.rect(dashboard.windows[0].chrome).width;
    press(&mut dashboard, &mut tui, &[pointer(MouseKind::Move, Pos { x: start.x + 20, ..start })]);
    assert_eq!(tui.scene.rect(dashboard.windows[0].chrome).width, width);
}
