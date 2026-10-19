use semio_framework_ui_tui_frame::chrome::shell;
use semio_framework_ui_tui_frame::chrome::window_chip_layout;
use semio_framework_ui_tui_frame::chrome::ChromeMode;
use semio_framework_ui_tui_frame::chrome::ChromeState;
use semio_framework_ui_tui_frame::chrome::FooterState;
use semio_framework_ui_tui_frame::chrome::NavbarState;
use semio_framework_ui_tui_frame::chrome::Shell;
use semio_framework_ui_tui_frame::chrome::TabKind;
use semio_framework_ui_tui_core::event::Event;
use semio_framework_ui_tui_core::event::KeyEvent;
use semio_framework_ui_tui_core::event::MouseButton;
use semio_framework_ui_tui_core::event::MouseEvent;
use semio_framework_ui_tui_core::event::MouseKind;
use semio_framework_ui_tui_core::geometry::{Pos, Size};
use semio_framework_ui_tui_frame::layout::{create_default_layout, dock_window, push_window_to_stack, WindowLayout, WindowLayoutWindowNode};
use semio_framework_ui_tui_frame::scene::NodeContent;
use semio_framework_ui_tui_frame::scene::NodeId;
use semio_framework_ui_tui_core::theme::GlyphSet;
use semio_framework_ui_tui_core::theme::Theme;
use semio_framework_ui_tui_core::signal::WidgetSignal;
use semio_framework_pack_json::{parse, JsonMemberPolicy, Value};
use ui_styling::appearance::AppearanceName;
use crate::Tui;
use semio_framework_ui_tui_core::geometry::Rect;
use semio_framework_ui_tui_core::event::Key;

fn fixture() -> Value {
    parse(include_str!("../🧫️fixtures/🪟️drag-preview/🔣️.json"), JsonMemberPolicy::Reject).unwrap()
}

fn pointer(kind: MouseKind, pos: Pos) -> Event {
    Event::Mouse(MouseEvent { kind, pos, mods: 0, clicks: 1 })
}

fn world(size: Size, weights: [f64; 2], peer: bool, same_stack: bool, compact: bool, glyphs: GlyphSet, overflow: bool) -> (Tui, Shell, WindowLayout, NodeId, NodeId) {
    let mut theme = Theme::new(AppearanceName::Dark);
    theme.set_glyphs(glyphs);
    let mut tui = Tui::new(size, theme);
    let mut layout = create_default_layout(&if same_stack { vec!["source".into()] } else { vec!["source".into(), "target".into()] }, "row", Some(&weights), Some(&["Source".into(), "Target".into()]));
    if same_stack { push_window_to_stack(&mut layout, "source", WindowLayoutWindowNode { window_kind_id: "target".into(), title: Some("Target".into()), corner: None }); }
    if peer { push_window_to_stack(&mut layout, "source", WindowLayoutWindowNode { window_kind_id: "peer".into(), title: Some("Peer".into()), corner: None }); }
    if overflow {
        for index in 0..8 {
            push_window_to_stack(&mut layout, "target", WindowLayoutWindowNode { window_kind_id: format!("extra{index}"), title: Some(format!("Long target label {index}")), corner: None });
        }
    }
    let mut ids = vec!["source".into(), "target".into()];
    if peer { ids.push("peer".into()); }
    if overflow { ids.extend((0..8).map(|index| format!("extra{index}"))); }
    let initial = create_default_layout(&ids, "row", None, None);
    let mut built = shell(&mut tui.scene, NavbarState { left: vec![], center: vec![], right: vec![] }, FooterState { hints: vec![], status: String::new() }, &initial);
    semio_framework_ui_tui_frame::layout::activate_stack_tab(&mut layout, "target");
    semio_framework_ui_tui_frame::layout::activate_stack_tab(&mut layout, "source");
    for (_, node) in &built.windows {
        if let Some(ChromeState::Window(state)) = tui.scene.node_mut(*node).chrome() { state.compact = compact; } else { unreachable!(); }
    }
    built.remount(&mut tui.scene, &layout);
    tui.render_full();
    let source = built.windows.iter().find(|(id, _)| id == "source").unwrap().1;
    let target = if same_stack { source } else { built.windows.iter().find(|(id, _)| id == "target").unwrap().1 };
    (tui, built, layout, source, target)
}

fn active_chip(tui: &Tui, node: NodeId) -> (Rect, Pos) {
    let rect = tui.scene.rect(node);
    let NodeContent::Chrome(ChromeState::Window(state)) = &tui.scene.node(node).content else { unreachable!() };
    let layout = window_chip_layout(state, rect);
    let (group, tab) = layout.groups.iter().find_map(|group| group.tabs.iter().find(|tab| tab.kind == TabKind::Tab && tab.index == state.active_stack_tab).map(|tab| (group, tab))).expect("active tab visible");
    let y = if layout.mode == ChromeMode::Compact { rect.y } else if group.corner.is_top() { rect.y } else { rect.y + rect.height - 3 };
    (Rect::new(tab.x, y, tab.interior_width + 2, if layout.mode == ChromeMode::Compact { 1 } else { 3 }), Pos { x: tab.x + 1, y: layout.text_y(group.corner, rect) })
}

#[test]
fn window_drag_grips_are_visible_localized_and_exclude_close_and_maximize() {
    for glyphs in [GlyphSet::Unicode, GlyphSet::Ascii] {
        for label in ["Drag window", "Fenster ziehen"] {
            let (mut tui, _built, _layout, source, _) = world(Size { width: 100, height: 40 }, [1.0, 1.0], false, false, false, glyphs, false);
            let (_, grip) = active_chip(&tui, source);
            if let Some(ChromeState::Window(state)) = tui.scene.node_mut(source).chrome() {
                state.labels.close = "close".into();
                state.labels.maximize = "maximize".into();
                state.labels.drag = label.into();
            } else { unreachable!(); }
            tui.render();
            let expected = fixture().as_object().unwrap().get("handle").unwrap().as_object().unwrap().get(if glyphs == GlyphSet::Unicode { "unicode" } else { "ascii" }).and_then(Value::as_str).unwrap().chars().next().unwrap();
            assert_eq!(tui.frame().get(grip.x, grip.y).unwrap().ch, expected);
            let NodeContent::Chrome(chrome) = &tui.scene.node(source).content else { unreachable!() };
            assert_eq!(chrome.window_control_at(tui.scene.rect(source), grip), None);
            assert!(chrome.window_target(tui.scene.rect(source), grip).is_some());
            assert_eq!(chrome.tooltip(tui.scene.rect(source), grip), Some(label.to_string()));
            let NodeContent::Chrome(ChromeState::Window(state)) = &tui.scene.node(source).content else { unreachable!() };
            let chips = window_chip_layout(state, tui.scene.rect(source));
            let tab = chips.groups.iter().flat_map(|group| &group.tabs).find(|tab| tab.kind == TabKind::Tab && tab.index == 0).unwrap();
            let close = tab.close_x.unwrap();
            let maximize = tab.maximize_x.unwrap();
            assert_eq!(chrome.window_control_at(tui.scene.rect(source), Pos { x: close, y: grip.y }), Some(WidgetSignal::WindowClose(0)));
            assert_eq!(chrome.window_control_at(tui.scene.rect(source), Pos { x: maximize, y: grip.y }), Some(WidgetSignal::WindowMaximize(0)));
            for (x, signal) in [(close, WidgetSignal::WindowClose(0)), (maximize, WidgetSignal::WindowMaximize(0))] {
                assert!(tui.dispatch(&pointer(MouseKind::Down(MouseButton::Left), Pos { x, y: grip.y })).contains(&(source, signal)));
                assert!(!tui.dragging_window());
            }
            let signals = tui.dispatch(&pointer(MouseKind::Down(MouseButton::Left), grip));
            assert!(signals.iter().any(|(_, signal)| matches!(signal, WidgetSignal::WindowTabActivated(0))));
            assert!(tui.dragging_window());
            assert!(tui.cancel_pointer());
        }
    }
}

#[test]
fn window_drag_fixture_previews_the_exact_committed_active_tab_and_seam() {
    let root = fixture();
    for case in root.as_object().unwrap().get("cases").and_then(Value::as_array).unwrap() {
        let case = case.as_object().unwrap();
        let name = case.get("name").and_then(Value::as_str).unwrap();
        let dimensions = case.get("size").and_then(Value::as_array).unwrap();
        let weights = case.get("weights").and_then(Value::as_array).unwrap();
        let size = Size { width: dimensions[0].as_u64().unwrap() as u16, height: dimensions[1].as_u64().unwrap() as u16 };
        let (mut tui, mut built, mut layout, source, target) = world(size, [weights[0].as_f64().unwrap(), weights[1].as_f64().unwrap()], case.get("sourcePeer").and_then(Value::as_bool).unwrap(), case.get("sameStack").and_then(Value::as_bool).unwrap(), case.get("compact").and_then(Value::as_bool).unwrap(), GlyphSet::Unicode, case.get("overflow").and_then(Value::as_bool).unwrap_or(false));
        let (_, grip) = active_chip(&tui, source);
        tui.dispatch(&pointer(MouseKind::Down(MouseButton::Left), grip));
        let area = tui.scene.rect(target);
        let placement = case.get("placement").and_then(Value::as_str).unwrap();
        let mut pos = match placement {
            "top-left" => Pos { x: area.x + 12.min(area.width - 2), y: area.y + 1 },
            "top-right" => Pos { x: area.x + area.width - 2, y: area.y + u16::from(area.height > 3) },
            "bottom-left" => Pos { x: area.x + 1, y: area.y + area.height - 2 },
            "bottom-right" => Pos { x: area.x + area.width - 2, y: area.y + area.height - 2 },
            "left" => Pos { x: area.x + 1, y: area.y + area.height / 2 },
            "right" => Pos { x: area.x + area.width - 2, y: area.y + area.height / 2 },
            "top" => Pos { x: area.x + area.width / 2, y: area.y + 3 },
            "bottom" => Pos { x: area.x + area.width / 2, y: area.y + area.height - 4 },
            "reorder" => {
                let NodeContent::Chrome(ChromeState::Window(state)) = &tui.scene.node(target).content else { unreachable!() };
                let chips = window_chip_layout(state, area);
                let tab = chips.groups.iter().flat_map(|group| &group.tabs).find(|tab| tab.kind == TabKind::Tab && tab.index == 1).unwrap();
                Pos { x: tab.x + tab.interior_width, y: area.y + 1 }
            }
            "overflow-next" => {
                let NodeContent::Chrome(ChromeState::Window(state)) = &tui.scene.node(target).content else { unreachable!() };
                let chips = window_chip_layout(state, area);
                let tab = chips.groups.iter().flat_map(|group| &group.tabs).find(|tab| matches!(tab.kind, TabKind::OverflowNext(_))).expect("visible next overflow arrow");
                Pos { x: tab.x + 2, y: area.y + 1 }
            }
            _ => unreachable!(),
        };
        if case.get("cap").and_then(Value::as_bool).unwrap_or(false) { pos.y = area.y; }
        let proposed = tui.dispatch(&pointer(MouseKind::Drag(MouseButton::Left), pos));
        if placement == "overflow-next" {
            assert!(proposed.iter().any(|(_, signal)| matches!(signal, WidgetSignal::WindowDockPreview { placement: semio_framework_ui_tui_core::docking::DockPlacement::Corner(semio_framework_ui_tui_core::docking::WindowStackCorner::TopLeft), .. })), "{name} occupied arrow corner");
        }
        assert!(tui.docking_preview().is_some(), "{name} must show a destination tab preview");
        let preview_rect = tui.docking_preview().unwrap().1;
        tui.render();
        let preview = tui.frame().clone();
        let signals = tui.dispatch(&pointer(MouseKind::Up(MouseButton::Left), pos));
        let signal = signals.iter().find_map(|(_, signal)| match signal { WidgetSignal::WindowDocked { .. } => Some(signal), _ => None }).expect("canonical window drop signal");
        match signal {
            WidgetSignal::WindowDocked { source, target, placement, tab_index } => assert!(dock_window(&mut layout, source, target, *placement, *tab_index)),
            _ => unreachable!(),
        }
        built.remount(&mut tui.scene, &layout);
        tui.render();
        let (chip, _) = active_chip(&tui, source);
        assert_eq!(preview_rect, chip, "{name} exact preview rectangle");
        for y in 0..size.height {
            for x in 0..size.width {
                if !chip.contains(Pos { x, y }) {
                    assert!(!matches!(preview.get(x, y).unwrap().ch, '+' | ':'), "{name} generic rectangle outside proposed tab at {x}:{y}");
                }
            }
        }
        for y in chip.y..chip.y + chip.height {
            for x in chip.x..chip.x + chip.width {
                assert_eq!(preview.get(x, y).unwrap().ch, tui.frame().get(x, y).unwrap().ch, "{name} committed chip {x}:{y}");
            }
        }
        assert!(tui.docking_preview().is_none(), "{name}");
    }
}

#[test]
fn cancelling_a_visible_destination_tab_preview_keeps_the_layout() {
    let (mut tui, _built, layout, source, target) = world(Size { width: 100, height: 40 }, [1.0, 3.0], false, false, false, GlyphSet::Unicode, false);
    let (_, grip) = active_chip(&tui, source);
    tui.dispatch(&pointer(MouseKind::Down(MouseButton::Left), grip));
    let area = tui.scene.rect(target);
    tui.dispatch(&pointer(MouseKind::Drag(MouseButton::Left), Pos { x: area.x + area.width - 2, y: area.y + 1 }));
    assert!(tui.docking_preview().is_some());
    tui.dispatch(&Event::Key(KeyEvent { key: Key::Esc, mods: 0 }));
    assert!(!tui.dragging_window());
    assert!(tui.docking_preview().is_none());
    assert_eq!(semio_framework_ui_tui_frame::layout::stack_hosting(&layout, "source").unwrap().children.len(), 1);
}

#[test]
fn proposed_tab_dimensions_match_independent_taffy_sizing() {
    use taffy::prelude::{AvailableSpace, Style, TaffyTree};
    for compact in [false, true] {
        let (tui, _, _, source, _) = world(Size { width: 100, height: if compact { 7 } else { 40 } }, [1.0, 1.0], false, false, compact, GlyphSet::Unicode, false);
        let (chip, _) = active_chip(&tui, source);
        let label_cells = unicode_width::UnicodeWidthStr::width("Source") as f32;
        let mut oracle = TaffyTree::<()>::new();
        let child = oracle.new_leaf(Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::length(label_cells + 8.0), height: taffy::style::Dimension::length(if compact { 1.0 } else { 3.0 }) },
            ..Default::default()
        }).unwrap();
        oracle.compute_layout(child, taffy::geometry::Size { width: AvailableSpace::Definite(100.0), height: AvailableSpace::Definite(24.0) }).unwrap();
        let expected = oracle.layout(child).unwrap();
        assert_eq!((chip.width, chip.height), (expected.size.width as u16, expected.size.height as u16));
    }
}

#[test]
fn held_destination_tab_refreshes_after_a_title_status_and_mount_change() {
    let (mut tui, mut built, mut layout, source, target) = world(Size { width: 140, height: 40 }, [1.0, 3.0], false, false, false, GlyphSet::Unicode, false);
    let (_, grip) = active_chip(&tui, source);
    tui.dispatch(&pointer(MouseKind::Down(MouseButton::Left), grip));
    let area = tui.scene.rect(target);
    let pos = Pos { x: area.x + area.width - 2, y: area.y + 1 };
    tui.dispatch(&pointer(MouseKind::Drag(MouseButton::Left), pos));
    tui.render();
    let original = tui.docking_preview().unwrap().1;
    assert!(semio_framework_ui_tui_frame::layout::set_window_title(&mut layout, "source", Some("Renamed source".into())));
    built.set_status(&mut tui.scene, "source", Some(semio_framework_ui_tui_core::theme::Status::Running));
    built.remount(&mut tui.scene, &layout);
    tui.tick(250);
    tui.render();
    let preview_rect = tui.docking_preview().unwrap().1;
    assert_ne!(original.width, preview_rect.width);
    assert!(tui.dragging_window());
    let preview = tui.frame().clone();
    let signals = tui.dispatch(&pointer(MouseKind::Up(MouseButton::Left), pos));
    let (source_id, target_id, placement, index) = signals.iter().find_map(|(_, signal)| match signal {
        WidgetSignal::WindowDocked { source, target, placement, tab_index } => Some((source, target, *placement, *tab_index)),
        _ => None,
    }).unwrap();
    assert!(dock_window(&mut layout, source_id, target_id, placement, index));
    built.remount(&mut tui.scene, &layout);
    tui.render();
    let (committed, _) = active_chip(&tui, source);
    assert_eq!(preview_rect, committed);
    for y in committed.y..committed.y + committed.height {
        for x in committed.x..committed.x + committed.width {
            assert_eq!(preview.get(x, y).unwrap().ch, tui.frame().get(x, y).unwrap().ch, "refreshed source chip {x}:{y}");
        }
    }
}

#[test]
fn repeated_local_window_ids_project_within_their_own_mounted_canvas() {
    let (mut tui, _first, first_layout, first_source, _) = world(Size { width: 160, height: 60 }, [1.0, 3.0], false, false, false, GlyphSet::Unicode, false);
    let mut layout = create_default_layout(&["source".into(), "target".into()], "row", Some(&[1.0, 3.0]), Some(&["Second source".into(), "Second target".into()]));
    let mut built = shell(&mut tui.scene, NavbarState { left: vec![], center: vec![], right: vec![] }, FooterState { hints: vec![], status: String::new() }, &layout);
    built.remount(&mut tui.scene, &layout);
    tui.render_full();
    let source = built.windows.iter().find(|(id, _)| id == "source").unwrap().1;
    let target = built.windows.iter().find(|(id, _)| id == "target").unwrap().1;
    let first_rect = tui.scene.rect(first_source);
    let canvas = tui.scene.rect(built.canvas);
    let (_, grip) = active_chip(&tui, source);
    tui.dispatch(&pointer(MouseKind::Down(MouseButton::Left), grip));
    let area = tui.scene.rect(target);
    let pos = Pos { x: area.x + area.width - 2, y: area.y + 1 };
    tui.dispatch(&pointer(MouseKind::Drag(MouseButton::Left), pos));
    tui.render();
    let preview_rect = tui.docking_preview().unwrap().1;
    assert!(canvas.contains(Pos { x: preview_rect.x, y: preview_rect.y }));
    let preview = tui.frame().clone();
    let signals = tui.dispatch(&pointer(MouseKind::Up(MouseButton::Left), pos));
    let (source_id, target_id, placement, index) = signals.iter().find_map(|(_, signal)| match signal {
        WidgetSignal::WindowDocked { source, target, placement, tab_index } => Some((source, target, *placement, *tab_index)),
        _ => None,
    }).unwrap();
    assert!(dock_window(&mut layout, source_id, target_id, placement, index));
    built.remount(&mut tui.scene, &layout);
    tui.render();
    let (committed, _) = active_chip(&tui, source);
    assert_eq!(preview_rect, committed);
    for y in committed.y..committed.y + committed.height {
        for x in committed.x..committed.x + committed.width {
            assert_eq!(preview.get(x, y).unwrap().ch, tui.frame().get(x, y).unwrap().ch);
        }
    }
    assert_eq!(tui.scene.rect(first_source), first_rect);
    assert_eq!(semio_framework_ui_tui_frame::layout::stack_hosting(&first_layout, "source").unwrap().children.len(), 1);
    let (_, first_grip) = active_chip(&tui, first_source);
    tui.dispatch(&pointer(MouseKind::Down(MouseButton::Left), first_grip));
    assert!(tui.dragging_window());
    assert!(tui.scene.rect(source).contains(pos));
    tui.dispatch(&pointer(MouseKind::Drag(MouseButton::Left), pos));
    assert!(tui.docking_preview().is_none());
    let crossed = tui.dispatch(&pointer(MouseKind::Up(MouseButton::Left), pos));
    assert!(!crossed.iter().any(|(_, signal)| matches!(signal, WidgetSignal::WindowDocked { .. })));
    assert_eq!(tui.scene.rect(first_source), first_rect);
    let (_, second_grip) = active_chip(&tui, source);
    tui.dispatch(&pointer(MouseKind::Down(MouseButton::Left), second_grip));
    assert!(tui.dragging_window());
    tui.scene.remove(source);
    tui.render();
    assert!(!tui.dragging_window());
    assert!(tui.docking_preview().is_none());
}

#[test]
fn narrow_overflow_preserves_the_active_chip_and_its_controls() {
    let mut state = semio_framework_ui_tui_frame::chrome::WindowState::new("Source").with_stack_tabs(vec!["Target".into(), "Source".into(), "Long neighboring target".into()], 1);
    state.tab_ids = vec!["target".into(), "source".into(), "neighbor".into()];
    state.peers = 3;
    let rect = Rect::new(0, 0, 18, 12);
    let layout = window_chip_layout(&state, rect);
    let tab = layout.groups.iter().flat_map(|group| &group.tabs).find(|tab| tab.kind == TabKind::Tab && tab.index == 1).expect("active source chip survives overflow arrows");
    assert!(tab.close_x.is_some());
    assert!(tab.maximize_x.is_some());
    assert!(tab.drag_x.is_some());
    assert_eq!(tab.interior_width + 2, 12);
    let arrows: Vec<_> = layout.groups.iter().flat_map(|group| &group.tabs).filter(|tab| matches!(tab.kind, TabKind::OverflowPrev(_) | TabKind::OverflowNext(_))).collect();
    assert_eq!(arrows.len(), 2);
    assert!(arrows.iter().all(|tab| tab.interior_width == 1));
    let narrow = window_chip_layout(&state, Rect::new(0, 0, 12, 12));
    assert_eq!(narrow.groups.iter().flat_map(|group| &group.tabs).filter(|tab| matches!(tab.kind, TabKind::OverflowPrev(_) | TabKind::OverflowNext(_))).count(), 1);
    assert!(narrow.groups.iter().flat_map(|group| &group.tabs).any(|tab| tab.kind == TabKind::Tab && tab.index == 1 && tab.close_x.is_some() && tab.maximize_x.is_some() && tab.drag_x.is_some()));
}
