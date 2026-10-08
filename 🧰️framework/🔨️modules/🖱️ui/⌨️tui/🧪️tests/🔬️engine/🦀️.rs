//! 🔬️ Engine behaviour: the focus tree, the cursor, hover, capture, click counting, wheel routing, context menus,
//! timed tooltips, fresh hit-testing, zoom visibility and splitter/tab drags (D07 D09 D10 D25 D32 D37, R14).

use crate::tui::chrome::{shell, window_chip_layout, ChromeLabels, ChromeState, FooterState, NavbarState, Shell, TabKind, WindowState};
use crate::tui::engine::Tui;
use crate::tui::event::{Event, Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Size};
use crate::tui::layout::{create_default_layout, push_window_to_stack, reorder_stack_tab, resize_split, solve_window_layout, zoom_window, Constraint, Dimension, Direction, WindowLayout, WindowLayoutWindowNode};
use crate::tui::scene::{Node, NodeContent, NodeId};
use crate::tui::theme::Theme;
use crate::tui::widget::{Align, InputState, LabelState, ListState, TabsState, WidgetSignal, WidgetState, WizardState};
use ui_styling::appearance::AppearanceName;

fn mouse(kind: MouseKind, x: u16, y: u16) -> Event {
    Event::Mouse(MouseEvent { kind, pos: Pos { x, y }, mods: 0, clicks: 1 })
}

fn down(x: u16, y: u16) -> Event {
    mouse(MouseKind::Down(MouseButton::Left), x, y)
}

fn row_of(widgets: Vec<(WidgetState, u16)>) -> (Tui, Vec<NodeId>) {
    let mut tui = Tui::new(Size { width: 60, height: 10 }, Theme::new(AppearanceName::Dark));
    let root = tui.scene.root();
    tui.scene.node_mut(root).set_constraint(Constraint { direction: Direction::Row, ..Default::default() });
    let mut ids = Vec::new();
    for (widget, width) in widgets {
        let id = tui.scene.add(root, Node::new(NodeContent::Widget(widget)));
        tui.scene.node_mut(id).set_constraint(Constraint { width: Dimension::Cells(width), ..Default::default() });
        ids.push(id);
    }
    tui.render_full();
    (tui, ids)
}

fn list(name: &str) -> WidgetState {
    WidgetState::List(ListState::new(vec![name.to_string(), "second".into()]))
}

fn tabs() -> WidgetState {
    WidgetState::Tabs(TabsState::new(vec!["one".into(), "two".into(), "three".into()], 0))
}

fn label() -> WidgetState {
    WidgetState::Label(LabelState { text: "static".into(), align: Align::Left, role: crate::tui::theme::Role::Foreground })
}

//#region 🎯 Focus tree

#[test]
fn a_click_focuses_the_containing_focusable_and_labels_never_take_focus() {
    let (mut tui, ids) = row_of(vec![(list("a"), 10), (label(), 10), (list("b"), 10)]);
    tui.set_focus(Some(ids[0]));
    tui.dispatch(&down(15, 0));
    assert_eq!(tui.focus(), Some(ids[0]), "a label is not focusable, so focus stays");
    tui.dispatch(&down(25, 0));
    assert_eq!(tui.focus(), Some(ids[2]));
    assert_eq!(tui.focus_ring(), vec![ids[0], ids[2]], "the ring holds interactive visible widgets only");
}

#[test]
fn clicking_a_window_focuses_its_first_widget_and_names_the_window() {
    let mut tui = Tui::new(Size { width: 80, height: 40 }, Theme::new(AppearanceName::Dark));
    let layout = create_default_layout(&["w1".into(), "w2".into()], "row", None, None);
    let mut built = shell(&mut tui.scene, NavbarState { left: vec![], center: vec![], right: vec![] }, FooterState { hints: vec![], status: String::new() }, &layout);
    let widgets: Vec<NodeId> = built.windows.iter().map(|(_, window)| tui.scene.add(*window, Node::new(NodeContent::Widget(list("x"))))).collect();
    for widget in &widgets {
        tui.scene.node_mut(*widget).set_constraint(Constraint { width: Dimension::Weight(1), height: Dimension::Weight(1), ..Default::default() });
    }
    built.remount(&mut tui.scene, &layout);
    tui.render_full();
    let (second, second_window) = (widgets[1], built.windows[1].1);
    let rect = tui.scene.rect(second_window);
    let signals = tui.dispatch(&down(rect.x + 1, rect.y + rect.height - 1));
    assert_eq!(tui.focus(), Some(second), "the wall of a window focuses the widget inside it");
    assert_eq!(signals, vec![(second_window, WidgetSignal::WindowFocus)]);
    assert_eq!(tui.focused_window(), Some(second_window));
    tui.render();
    let NodeContent::Chrome(ChromeState::Window(state)) = &tui.scene.node(second_window).content else { panic!("window") };
    assert!(state.focused, "the engine marks the focused window's chrome");
}

#[test]
fn a_removed_focus_target_never_dangles() {
    let (mut tui, ids) = row_of(vec![(list("a"), 10), (list("b"), 10)]);
    tui.set_focus(Some(ids[1]));
    tui.scene.remove(ids[1]);
    assert_eq!(tui.focus(), None);
    assert_eq!(tui.cursor(), None);
    tui.dispatch(&Event::Key(KeyEvent { key: Key::Down, mods: 0 }));
    tui.dispatch(&Event::Key(KeyEvent { key: Key::Tab, mods: 0 }));
    assert_eq!(tui.focus(), Some(ids[0]), "Tab restarts the ring after the target vanished");
    tui.render();
}

#[test]
fn hidden_widgets_leave_the_ring_and_zoom_hides_the_other_windows() {
    let mut tui = Tui::new(Size { width: 80, height: 40 }, Theme::new(AppearanceName::Dark));
    let mut layout = create_default_layout(&["w1".into(), "w2".into(), "w3".into()], "row", None, None);
    let mut built = shell(&mut tui.scene, NavbarState { left: vec![], center: vec![], right: vec![] }, FooterState { hints: vec![], status: String::new() }, &layout);
    let widgets: Vec<NodeId> = built.windows.iter().map(|(_, window)| tui.scene.add(*window, Node::new(NodeContent::Widget(list("x"))))).collect();
    built.remount(&mut tui.scene, &layout);
    tui.render_full();
    assert_eq!(tui.focus_ring().len(), 3);
    zoom_window(&mut layout, Some("w2"));
    built.remount(&mut tui.scene, &layout);
    tui.render_full();
    assert_eq!(tui.focus_ring(), vec![widgets[1]], "zoom leaves one window in the ring");
    let covered = tui.scene.rect(built.windows[0].1);
    let hit = tui.scene.hit(Pos { x: covered.x + 1, y: covered.y + 5 }).expect("something is under the pointer");
    assert!(!tui.scene.lineage(hit).contains(&built.windows[0].1), "a zoomed-out window is not hit-testable");
    let NodeContent::Chrome(ChromeState::Window(zoomed)) = &tui.scene.node(built.windows[1].1).content else { panic!("window") };
    assert!(zoomed.zoomed && zoomed.show_maximize(), "the zoomed window offers restore");
    zoom_window(&mut layout, None);
    built.remount(&mut tui.scene, &layout);
    tui.render_full();
    assert_eq!(tui.focus_ring().len(), 3);
}

//#endregion 🎯 Focus tree

//#region ✏️ Cursor

#[test]
fn the_cursor_belongs_to_the_focused_widget() {
    let input = WidgetState::Input(InputState { value: "h\u{e9}llo".into(), cursor: 3, placeholder: String::new() });
    let wizard = WidgetState::Wizard({
        let mut w = WizardState::new(vec!["alpha".into()]);
        w.list.set_query("al");
        w
    });
    let (mut tui, ids) = row_of(vec![(list("a"), 10), (input, 20), (wizard, 20)]);
    assert_eq!(tui.cursor(), None, "no focus, no cursor");
    tui.set_focus(Some(ids[0]));
    assert_eq!(tui.cursor(), None, "a list shows no text cursor");
    tui.set_focus(Some(ids[1]));
    let at = tui.cursor().expect("input caret");
    assert_eq!((at.pos.x, at.pos.y), (10 + 2, 0), "two cells before the caret: h and the two-byte e-acute");
    tui.set_focus(Some(ids[2]));
    let at = tui.cursor().expect("wizard filter caret");
    assert_eq!((at.pos.x, at.pos.y), (30 + 2 + 2, 0));
}

//#endregion ✏️ Cursor

//#region 🖱️ Pointer

#[test]
fn hover_follows_the_pointer_and_clears_when_it_leaves() {
    let (mut tui, ids) = row_of(vec![(tabs(), 30), (list("b"), 10)]);
    tui.dispatch(&mouse(MouseKind::Move, 8, 0));
    assert_eq!(tui.hovered(), Some(ids[0]));
    let NodeContent::Widget(WidgetState::Tabs(state)) = &tui.scene.node(ids[0]).content else { panic!("tabs") };
    assert_eq!(state.hover, Some(1), "the tab under the pointer is lit");
    assert!(!tui.render().0.is_empty(), "lighting the tab repaints it");
    assert!(tui.render().0.is_empty());
    tui.dispatch(&mouse(MouseKind::Move, 35, 0));
    assert_eq!(tui.hovered(), Some(ids[1]));
    let NodeContent::Widget(WidgetState::Tabs(state)) = &tui.scene.node(ids[0]).content else { panic!("tabs") };
    assert_eq!(state.hover, None);
    tui.dispatch(&Event::FocusLost);
    assert_eq!(tui.hovered(), None);
    assert!(!tui.render().0.is_empty(), "clearing the hover repaints the tab");
}

#[test]
fn the_wheel_goes_to_the_widget_under_the_pointer_not_the_focused_one() {
    let (mut tui, ids) = row_of(vec![(list("a"), 10), (tabs(), 30), (label(), 10)]);
    tui.set_focus(Some(ids[0]));
    let signals = tui.dispatch(&mouse(MouseKind::Scroll { dx: 0, dy: 1 }, 14, 0));
    assert_eq!(signals, vec![(ids[1], WidgetSignal::TabChanged(1))]);
    assert_eq!(tui.focus(), Some(ids[0]), "scrolling never moves focus");
    assert_eq!(tui.dispatch(&mouse(MouseKind::Scroll { dx: 0, dy: -1 }, 45, 0)), vec![], "a label has no wheel, so nothing is reported");
}

#[test]
fn capture_sends_pointer_events_to_the_captured_node_wherever_they_land() {
    let (mut tui, ids) = row_of(vec![(list("a"), 10), (tabs(), 30)]);
    tui.capture(Some(ids[1]));
    let signals = tui.dispatch(&mouse(MouseKind::Scroll { dx: 0, dy: 1 }, 2, 0));
    assert_eq!(signals, vec![(ids[1], WidgetSignal::TabChanged(1))]);
    tui.capture(None);
    assert_eq!(tui.dispatch(&mouse(MouseKind::Scroll { dx: 0, dy: 1 }, 2, 0)), vec![]);
    tui.scene.remove(ids[1]);
    tui.capture(Some(ids[1]));
    tui.dispatch(&mouse(MouseKind::Scroll { dx: 0, dy: 1 }, 2, 0));
}

#[test]
fn a_right_click_reports_a_context_menu_with_the_pointer_cell() {
    let (mut tui, ids) = row_of(vec![(list("a"), 10), (tabs(), 30)]);
    let signals = tui.dispatch(&mouse(MouseKind::Down(MouseButton::Right), 4, 0));
    assert_eq!(signals, vec![(ids[0], WidgetSignal::ContextMenu { pos: Pos { x: 4, y: 0 }, item: Some(0) })]);
    assert_eq!(tui.focus(), Some(ids[0]), "right-click focuses what it opens a menu for");
    assert_eq!(tui.dispatch(&mouse(MouseKind::Down(MouseButton::Middle), 14, 0)), vec![], "a middle press neither activates nor opens a menu");
}

#[test]
fn consecutive_presses_on_one_cell_count_as_double_clicks_only_inside_the_window() {
    let (mut tui, ids) = row_of(vec![(tabs(), 30)]);
    let at = |x: u64| x;
    assert_eq!(tui.dispatch_at(&down(8, 0), at(1000)), vec![(ids[0], WidgetSignal::TabChanged(1))]);
    assert_eq!(tui.dispatch_at(&down(8, 0), at(1200)), vec![(ids[0], WidgetSignal::Activated(1))], "second press within 500 ms is a double click");
    assert_eq!(tui.dispatch_at(&down(8, 0), at(1300)), vec![], "the third press is a triple click, which a tab ignores");
    assert_eq!(tui.dispatch_at(&down(9, 0), at(1350)), vec![], "a press on another cell starts counting again");
    assert_eq!(tui.dispatch_at(&down(8, 0), at(4000)), vec![], "after a pause the press counts as a single click again");
    assert_eq!(tui.dispatch_at(&down(8, 0), at(4100)), vec![(ids[0], WidgetSignal::Activated(1))]);
}

#[test]
fn only_the_left_button_activates_and_up_or_drag_without_a_press_is_harmless() {
    let (mut tui, ids) = row_of(vec![(tabs(), 30)]);
    assert_eq!(tui.dispatch(&mouse(MouseKind::Down(MouseButton::Middle), 8, 0)), vec![]);
    assert_eq!(tui.dispatch(&mouse(MouseKind::Up(MouseButton::Left), 8, 0)), vec![]);
    assert_eq!(tui.dispatch(&mouse(MouseKind::Drag(MouseButton::Right), 8, 0)), vec![], "a phantom drag is treated as a move");
    assert_eq!(tui.hovered(), Some(ids[0]));
}

#[test]
fn hit_testing_uses_the_layout_of_the_change_that_just_happened() {
    let (mut tui, ids) = row_of(vec![(tabs(), 30), (list("b"), 10)]);
    tui.scene.node_mut(ids[0]).set_constraint(Constraint { width: Dimension::Cells(5), ..Default::default() });
    tui.dispatch(&down(12, 0));
    assert_eq!(tui.focus(), Some(ids[1]), "x=12 belongs to the list once the tabs shrank, with no render in between");
}

//#endregion 🖱️ Pointer

//#region 🪟 Windows

fn two_stacks(labels: bool) -> (Tui, Shell, WindowLayout, Vec<NodeId>) {
    let mut tui = Tui::new(Size { width: 80, height: 40 }, Theme::new(AppearanceName::Dark));
    let mut layout = create_default_layout(&["w1".into(), "w2".into()], "row", None, Some(&["alpha".into(), "delta".into()]));
    for (id, title) in [("w3", "beta"), ("w4", "gamma")] {
        push_window_to_stack(&mut layout, "w1", WindowLayoutWindowNode { window_kind_id: id.into(), title: Some(title.into()), corner: None });
    }
    crate::tui::layout::activate_stack_tab(&mut layout, "w1");
    let mut built = shell(&mut tui.scene, NavbarState { left: vec![], center: vec![], right: vec![] }, FooterState { hints: vec![], status: String::new() }, &layout);
    let canvas = built.canvas;
    for id in ["w3", "w4"] {
        let node = tui.scene.add(canvas, Node::new(NodeContent::Chrome(ChromeState::Window(Box::new(WindowState::new(id))))));
        built.windows.push((id.into(), node));
    }
    for (_, window) in built.windows.clone() {
        if let Some(ChromeState::Window(state)) = tui.scene.node_mut(window).chrome() {
            if labels {
                state.labels = ChromeLabels { close: "Close tab".into(), maximize: "Maximize".into(), restore: "Restore".into(), new_tab: "New tab".into(), previous_tabs: "Earlier tabs".into(), next_tabs: "Later tabs".into() };
            }
        }
    }
    built.remount(&mut tui.scene, &layout);
    tui.render_full();
    let windows = built.windows.iter().map(|(_, n)| *n).collect();
    (tui, built, layout, windows)
}

fn chip_geometry(tui: &Tui, window: NodeId, index: usize) -> (u16, u16, Option<u16>, u16) {
    let rect = tui.scene.rect(window);
    let NodeContent::Chrome(ChromeState::Window(state)) = &tui.scene.node(window).content else { panic!("window") };
    let layout = window_chip_layout(state, rect);
    let tab = layout.groups.iter().flat_map(|g| g.tabs.iter()).find(|t| t.kind == TabKind::Tab && t.index == index).expect("chip");
    (tab.x, tab.x + tab.interior_width + 1, tab.close_x, rect.y + 1)
}

#[test]
fn every_chrome_control_carries_its_own_tab_index_and_only_the_left_button_fires() {
    let (mut tui, _built, _layout, windows) = two_stacks(false);
    let w1 = windows[0];
    let (_, _, close_of_second, y) = chip_geometry(&tui, w1, 1);
    let close = close_of_second.expect("close glyph");
    assert_eq!(tui.dispatch(&down(close, y)), vec![(w1, WidgetSignal::WindowClose(1))], "closing an inactive chip names that chip, not the active tab");
    assert_eq!(tui.dispatch(&mouse(MouseKind::Down(MouseButton::Right), close, y)), vec![]);
    assert_eq!(tui.dispatch(&mouse(MouseKind::Down(MouseButton::Middle), close, y)), vec![]);
    let (left, right, _, _) = chip_geometry(&tui, w1, 2);
    let signals = tui.dispatch(&down(left + 2, y));
    assert!(signals.contains(&(w1, WidgetSignal::WindowTabActivated(2))), "{signals:?}");
    assert!(right > left);
    let rect = tui.scene.rect(w1);
    let maximize = (rect.x..rect.x + rect.width).find(|&x| tui.frame().get(x, y).is_some_and(|c| c.ch == '\u{2922}')).expect("two stacks show maximize");
    assert_eq!(tui.dispatch(&down(maximize, y)), vec![(w1, WidgetSignal::WindowMaximize(0))]);
}

#[test]
fn a_right_click_on_a_tab_asks_for_its_context_menu() {
    let (mut tui, _built, _layout, windows) = two_stacks(false);
    let (left, _, _, y) = chip_geometry(&tui, windows[0], 1);
    let signals = tui.dispatch(&mouse(MouseKind::Down(MouseButton::Right), left + 2, y));
    assert_eq!(signals, vec![(windows[0], WidgetSignal::ContextMenu { pos: Pos { x: left + 2, y }, item: Some(1) })]);
}

#[test]
fn dragging_a_tab_onto_another_reports_a_move_on_release() {
    let (mut tui, _built, mut layout, windows) = two_stacks(false);
    let w1 = windows[0];
    let (from_left, _, _, y) = chip_geometry(&tui, w1, 0);
    let (to_left, _, _, _) = chip_geometry(&tui, w1, 2);
    assert_eq!(tui.dispatch(&down(from_left + 2, y)), vec![(w1, WidgetSignal::WindowTabActivated(0))]);
    assert_eq!(tui.dispatch(&mouse(MouseKind::Drag(MouseButton::Left), to_left + 2, y)), vec![]);
    let NodeContent::Chrome(ChromeState::Window(state)) = &tui.scene.node(w1).content else { panic!("window") };
    assert_eq!(state.drop_target, Some(2), "the drop target is marked while dragging");
    let signals = tui.dispatch(&mouse(MouseKind::Up(MouseButton::Left), to_left + 2, y));
    assert_eq!(signals, vec![(w1, WidgetSignal::TabMoved { from: 0, to: 2 })]);
    let NodeContent::Chrome(ChromeState::Window(state)) = &tui.scene.node(w1).content else { panic!("window") };
    assert_eq!(state.drop_target, None);
    assert!(reorder_stack_tab(&mut layout, "w1", 0, 2));
    let stack = crate::tui::layout::stack_hosting(&layout, "w1").unwrap();
    assert_eq!(stack.children.iter().map(|c| c.window_kind_id.as_str()).collect::<Vec<_>>(), vec!["w3", "w4", "w1"]);
}

#[test]
fn dragging_the_gutter_between_stacks_reports_splitter_deltas_the_layout_applies() {
    let (mut tui, mut built, mut layout, windows) = two_stacks(false);
    let (a, b) = (tui.scene.rect(windows[0]), tui.scene.rect(windows[1]));
    assert_eq!(b.x, a.x + a.width + 1, "one gutter cell separates the stacks");
    let gutter = a.x + a.width;
    let y = a.y + 8;
    assert_eq!(tui.dispatch(&down(gutter, y)), vec![]);
    let signals = tui.dispatch(&mouse(MouseKind::Drag(MouseButton::Left), gutter + 3, y));
    assert_eq!(signals, vec![(tui.scene.node(windows[0]).parent().and_then(|p| tui.scene.node(p).parent()).expect("axis"), WidgetSignal::SplitterDragged { path: vec![0], delta: 3 })]);
    let more = tui.dispatch(&mouse(MouseKind::Drag(MouseButton::Left), gutter + 5, y));
    assert!(matches!(&more[0].1, WidgetSignal::SplitterDragged { delta: 2, .. }), "deltas are incremental: {more:?}");
    assert_eq!(tui.dispatch(&mouse(MouseKind::Up(MouseButton::Left), gutter + 5, y)), vec![]);
    let area = tui.scene.rect(built.canvas);
    assert!(resize_split(&mut layout, area, &[0], 5));
    built.remount(&mut tui.scene, &layout);
    tui.render_full();
    let (a2, b2) = (tui.scene.rect(windows[0]), tui.scene.rect(windows[1]));
    assert_eq!(a2.width, a.width + 5);
    assert_eq!(b2.width, b.width - 5);
    let measures = solve_window_layout(&layout, area);
    assert_eq!(measures[0].rect.width, a2.width, "the pure solver and the mounted tree agree after a drag");
    assert!(!resize_split(&mut layout, area, &[0], -500) || tui.scene.rect(windows[0]).width > 0);
}

#[test]
fn tooltips_open_after_the_dwell_time_and_close_when_the_pointer_moves_on() {
    let (mut tui, _built, _layout, windows) = two_stacks(true);
    let (_, _, close, y) = chip_geometry(&tui, windows[0], 1);
    let close = close.unwrap();
    tui.dispatch_at(&mouse(MouseKind::Move, close, y), 1000);
    tui.tick(1399);
    assert!(tui.overlays().is_empty(), "no tooltip before 400 ms");
    tui.render();
    assert_eq!(tui.deadline_ms(), Some(1), "the host is told when the tooltip falls due");
    tui.tick(1400);
    let overlays = tui.overlays();
    assert_eq!(overlays.len(), 1, "the tooltip opens after the dwell");
    let NodeContent::Widget(WidgetState::Tooltip(tip)) = &tui.scene.node(overlays[0]).content else { panic!("tooltip") };
    assert_eq!(tip.text, "Close tab");
    assert_eq!(tui.scene.hit(Pos { x: close + 1, y: y + 1 }).map(|hit| hit == overlays[0]), Some(false), "a tooltip never takes the pointer");
    tui.render();
    tui.dispatch_at(&mouse(MouseKind::Move, 0, 0), 1500);
    assert!(tui.overlays().is_empty(), "moving away dismisses it");
}

#[test]
fn an_elided_tab_shows_its_full_title_as_a_tooltip_and_a_node_tooltip_works_anywhere() {
    let mut tui = Tui::new(Size { width: 80, height: 40 }, Theme::new(AppearanceName::Dark));
    let layout = create_default_layout(&["w1".into()], "row", None, Some(&["a very long task title that cannot fit a tab".into()]));
    let mut built = shell(&mut tui.scene, NavbarState { left: vec![], center: vec![], right: vec![] }, FooterState { hints: vec![], status: String::new() }, &layout);
    let node = tui.scene.add(built.windows[0].1, Node::new(NodeContent::Widget(list("x"))));
    tui.scene.node_mut(node).set_constraint(Constraint { width: Dimension::Weight(1), height: Dimension::Weight(1), ..Default::default() });
    tui.scene.node_mut(node).set_tooltip(Some("the list".into()));
    built.remount(&mut tui.scene, &layout);
    tui.render_full();
    let window = built.windows[0].1;
    let (left, _, _, y) = chip_geometry(&tui, window, 0);
    tui.dispatch_at(&mouse(MouseKind::Move, left + 2, y), 0);
    tui.tick(1);
    tui.tick(450);
    let NodeContent::Widget(WidgetState::Tooltip(tip)) = &tui.scene.node(tui.overlays()[0]).content else { panic!("tooltip") };
    assert_eq!(tip.text, "a very long task title that cannot fit a tab");
    let rect = tui.scene.rect(node);
    tui.dispatch_at(&mouse(MouseKind::Move, rect.x + 1, rect.y), 500);
    tui.tick(900);
    tui.tick(1000);
    let NodeContent::Widget(WidgetState::Tooltip(tip)) = &tui.scene.node(*tui.overlays().last().unwrap()).content else { panic!("tooltip") };
    assert_eq!(tip.text, "the list");
}

//#endregion 🪟 Windows

#[test]
fn focus_changes_and_window_focus_events_reach_the_widget_as_on_focus() {
    use crate::tui::widget::TerminalState;
    let mut pane = TerminalState::new(Size { width: 20, height: 5 }, 100);
    pane.feed(b"\x1b[?1004h");
    let (mut tui, ids) = row_of(vec![(WidgetState::Terminal(pane), 30), (list("b"), 10)]);
    tui.set_focus(Some(ids[0]));
    assert_eq!(
        tui.dispatch(&Event::FocusLost),
        vec![(ids[0], WidgetSignal::TerminalInput(b"\x1b[I".to_vec())), (ids[0], WidgetSignal::TerminalInput(b"\x1b[O".to_vec()))],
        "the child asked for focus reports: the earlier gain, then the loss"
    );
    assert_eq!(tui.dispatch(&Event::FocusGained), vec![(ids[0], WidgetSignal::TerminalInput(b"\x1b[I".to_vec()))]);
    tui.set_focus(Some(ids[1]));
    assert_eq!(tui.dispatch(&Event::FocusGained), vec![(ids[0], WidgetSignal::TerminalInput(b"\x1b[O".to_vec()))], "the terminal was told it lost focus, the list reports nothing");
    tui.set_focus(Some(ids[0]));
    let signals = tui.dispatch(&Event::Key(KeyEvent { key: Key::Char('x'), mods: 0 }));
    assert_eq!(signals[0], (ids[0], WidgetSignal::TerminalInput(b"\x1b[I".to_vec())), "focus gained while idle is reported with the next result");
}

#[test]
fn a_pressed_terminal_keeps_receiving_the_drag_and_release_outside_its_rect() {
    use crate::tui::widget::TerminalState;
    let (mut tui, ids) = row_of(vec![(WidgetState::Terminal(TerminalState::new(Size { width: 20, height: 5 }, 100)), 20), (list("b"), 30)]);
    tui.dispatch(&down(3, 1));
    assert_eq!(tui.focus(), Some(ids[0]));
    tui.dispatch(&mouse(MouseKind::Scroll { dx: 0, dy: 1 }, 40, 1));
    assert_eq!(tui.focus(), Some(ids[0]), "scrolling elsewhere never steals focus");
    tui.dispatch(&mouse(MouseKind::Drag(MouseButton::Left), 45, 3));
    tui.dispatch(&mouse(MouseKind::Up(MouseButton::Left), 45, 3));
    assert_eq!(tui.focus(), Some(ids[0]), "the drag and the release stayed with the pressed pane");
}

#[test]
fn a_wheel_that_changes_widget_state_without_a_signal_still_repaints() {
    let (mut tui, ids) = row_of(vec![(WidgetState::List(ListState::new((0..40).map(|i| format!("row {i}")).collect())), 20)]);
    assert!(tui.render().0.is_empty());
    tui.dispatch(&mouse(MouseKind::Scroll { dx: 0, dy: 3 }, 3, 2));
    assert!(!tui.render().0.is_empty(), "the scrolled rows are repainted ({:?})", ids);
}

#[test]
fn the_engine_tick_animates_progress_widgets_and_running_tabs() {
    use crate::tui::chrome::WindowStackTabState;
    use crate::tui::theme::Status;
    use crate::tui::widget::ProgressState;
    let (mut tui, _ids) = row_of(vec![(WidgetState::Progress(ProgressState::indeterminate("working")), 30)]);
    assert!(tui.render().0.is_empty());
    tui.tick(0);
    tui.render();
    let first = (0..30).map(|x| tui.frame().get(x, 0).unwrap().ch).collect::<String>();
    assert!(tui.tick(700), "a repaint is due after the spinner moved");
    tui.render();
    let later = (0..30).map(|x| tui.frame().get(x, 0).unwrap().ch).collect::<String>();
    assert_ne!(first, later, "the indeterminate bar moved");
    assert!(tui.deadline_ms().is_some(), "the host is told to tick again");

    let (mut tui, _built, _layout, windows) = two_stacks(false);
    let w1 = windows[0];
    if let Some(ChromeState::Window(state)) = tui.scene.node_mut(w1).chrome() {
        state.stack_tabs[0] = WindowStackTabState::new("alpha", crate::tui::layout::WindowStackCorner::TopLeft).with_status(Some(Status::Running));
    }
    tui.tick(0);
    tui.render();
    let (left, _, _, y) = chip_geometry(&tui, w1, 0);
    let glyph = |tui: &Tui| tui.frame().get(left + 2, y).unwrap().ch;
    let before = glyph(&tui);
    tui.tick(130);
    tui.render();
    assert_ne!(before, glyph(&tui), "the running tab's spinner advances one frame every 125 ms");
}

#[test]
fn shell_set_status_mirrors_into_every_tab_strip_of_the_stack() {
    use crate::tui::theme::Status;
    let (mut tui, built, _layout, windows) = two_stacks(false);
    built.set_status(&mut tui.scene, "w3", Some(Status::Failure));
    for window in &windows[..1] {
        let NodeContent::Chrome(ChromeState::Window(state)) = &tui.scene.node(*window).content else { panic!("window") };
        assert_eq!(state.stack_tabs[1].status, Some(Status::Failure));
        assert_eq!(state.tab_ids, vec!["w1", "w3", "w4"]);
    }
    tui.render();
    let (left, _, _, y) = chip_geometry(&tui, windows[0], 1);
    assert_eq!(tui.frame().get(left + 2, y).unwrap().ch, '\u{2717}');
}
