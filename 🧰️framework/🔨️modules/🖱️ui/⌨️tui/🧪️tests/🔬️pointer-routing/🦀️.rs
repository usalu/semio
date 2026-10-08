//! 🔬️ Adapter for the language-agnostic pointer routing table `🧫️fixtures/🖱️pointer-routing/🔣️.json`
//! (feature `🧪️tests/🖱️pointer-routing/🥒️.feature`). The vitest oracle in that folder re-derives every
//! chrome expectation from the golden frame and the declared chips, so engine, painter and table agree.

use crate::tui::chrome::{shell, window_chip_layout, ChromeState, FooterState, NavbarState, TabKind, WindowState};
use crate::tui::engine::Tui;
use crate::tui::event::{Event, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Size};
use crate::tui::layout::{Constraint, Dimension, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};
use crate::tui::scene::{Node, NodeContent, NodeId};
use crate::tui::theme::Theme;
use crate::tui::widget::{TabsState, WidgetSignal, WidgetState, WizardState};
use semio_framework_pack_json::{parse, JsonMemberPolicy, Value};
use std::collections::BTreeMap;
use ui_styling::appearance::AppearanceName;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🖱️pointer-routing/🔣️.json");

/// 🥒️ The language-agnostic feature this adapter runs, bound at compile time so renaming it breaks the build.
const FEATURE: &str = include_str!("../🖱️pointer-routing/🥒️.feature");

#[test]
fn the_adapter_runs_the_declared_feature() {
    assert_eq!(FEATURE.lines().next(), Some("@capability-tui-pointer-routing"));
}

struct Fixture {
    tui: Tui,
    names: BTreeMap<String, NodeId>,
}

fn field<'a>(value: &'a Value, key: &str) -> &'a Value {
    value.get(key).unwrap_or_else(|| panic!("fixture field {key}"))
}

fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    field(value, key).as_str().unwrap_or_else(|| panic!("fixture string {key}"))
}

fn number(value: &Value, key: &str) -> u64 {
    field(value, key).as_u64().unwrap_or_else(|| panic!("fixture number {key}"))
}

fn build(scene: &Value) -> Fixture {
    let size = field(scene, "size").as_array().expect("size");
    let size = Size { width: size[0].as_u64().unwrap() as u16, height: size[1].as_u64().unwrap() as u16 };
    let mut tui = Tui::new(size, Theme::new(AppearanceName::Dark));
    let mut stacks = Vec::new();
    for stack in field(field(scene, "layout"), "stacks").as_array().expect("stacks") {
        let windows = field(stack, "windows").as_array().expect("windows");
        stacks.push(WindowLayoutChild::Stack(WindowLayoutStackNode {
            size: None,
            active_window_kind_id: Some(text(stack, "active").to_string()),
            children: windows.iter().map(|w| WindowLayoutWindowNode { window_kind_id: text(w, "id").to_string(), title: Some(text(w, "title").to_string()), corner: None }).collect(),
        }));
    }
    let layout = WindowLayout { root: WindowLayoutRoot::Axis(WindowLayoutAxisNode { kind: text(field(scene, "layout"), "kind").to_string(), size: None, children: stacks }), zoomed: None };
    let mut built = shell(&mut tui.scene, NavbarState { left: vec![], center: vec![], right: vec![] }, FooterState { hints: vec![], status: String::new() }, &layout);
    let canvas = built.canvas;
    let mut names = BTreeMap::new();
    for stack in field(field(scene, "layout"), "stacks").as_array().unwrap() {
        for window in field(stack, "windows").as_array().unwrap() {
            let id = text(window, "id").to_string();
            if !built.windows.iter().any(|(known, _)| *known == id) {
                let node = tui.scene.add(canvas, Node::new(NodeContent::Chrome(ChromeState::Window(Box::new(WindowState::new(id.clone()))))));
                built.windows.push((id, node));
            }
        }
    }
    for (id, node) in &built.windows {
        names.insert(id.clone(), *node);
    }
    for (window, kind) in field(scene, "widgets").as_object().expect("widgets").iter() {
        let chrome = names[window];
        let widget = match kind.as_str().unwrap() {
            "wizard" => WidgetState::Wizard(WizardState::new((0..6).map(|i| format!("option {i}")).collect())),
            "tabs" => WidgetState::Tabs(TabsState::new(vec!["one".into(), "two".into(), "three".into()], 0)),
            other => panic!("unknown widget kind {other}"),
        };
        let node = tui.scene.add(chrome, Node::new(NodeContent::Widget(widget)));
        tui.scene.node_mut(node).set_constraint(Constraint { width: Dimension::Weight(1), height: Dimension::Weight(1), ..Default::default() });
        names.insert(format!("{window}.{}", kind.as_str().unwrap()), node);
    }
    built.remount(&mut tui.scene, &layout);
    tui.render_full();
    fn name_axes(tui: &Tui, id: NodeId, names: &mut BTreeMap<String, NodeId>) {
        if let NodeContent::Axis(axis) = &tui.scene.node(id).content {
            let path: Vec<String> = axis.path.iter().map(usize::to_string).collect();
            names.insert(if path.is_empty() { "axis".to_string() } else { format!("axis/{}", path.join("/")) }, id);
        }
        for &child in tui.scene.node(id).children() {
            name_axes(tui, child, names);
        }
    }
    name_axes(&tui, tui.scene.root(), &mut names);
    Fixture { tui, names }
}

fn signal_name(signal: &WidgetSignal) -> String {
    match signal {
        WidgetSignal::WindowClose(i) => format!("WindowClose({i})"),
        WidgetSignal::WindowMaximize(i) => format!("WindowMaximize({i})"),
        WidgetSignal::WindowNewTab(i) => format!("WindowNewTab({i})"),
        WidgetSignal::WindowTabActivated(i) => format!("WindowTabActivated({i})"),
        WidgetSignal::WindowFocus => "WindowFocus".into(),
        WidgetSignal::TabMoved { from, to } => format!("TabMoved({from}->{to})"),
        WidgetSignal::SplitterDragged { path, delta } => format!("SplitterDragged({},{delta})", path.iter().map(usize::to_string).collect::<Vec<_>>().join("/")),
        WidgetSignal::ContextMenu { pos, item } => format!("ContextMenu({},{},{})", pos.x, pos.y, item.map_or("-".into(), |i| i.to_string())),
        WidgetSignal::TabChanged(i) => format!("TabChanged({i})"),
        WidgetSignal::Activated(i) => format!("Activated({i})"),
        WidgetSignal::SelectionChanged(i) => format!("SelectionChanged({i})"),
        other => format!("{other:?}"),
    }
}

fn event(spec: &Value) -> (Event, u64) {
    let (x, y) = (number(spec, "x") as u16, number(spec, "y") as u16);
    let button = match spec.get("b").and_then(Value::as_str).unwrap_or("left") {
        "left" => MouseButton::Left,
        "middle" => MouseButton::Middle,
        _ => MouseButton::Right,
    };
    let kind = match text(spec, "t") {
        "down" => MouseKind::Down(button),
        "up" => MouseKind::Up(button),
        "drag" => MouseKind::Drag(button),
        "move" => MouseKind::Move,
        "wheel" => MouseKind::Scroll { dx: 0, dy: spec.get("dy").and_then(Value::as_i64).unwrap_or(1) as i16 },
        other => panic!("unknown event {other}"),
    };
    let at = spec.get("at").and_then(Value::as_u64).unwrap_or(0);
    (Event::Mouse(MouseEvent { kind, pos: Pos { x, y }, mods: 0, clicks: 1 }), at)
}

fn name_of(fixture: &Fixture, node: Option<NodeId>) -> Option<String> {
    let node = node?;
    fixture.names.iter().find(|(_, id)| **id == node).map(|(name, _)| name.clone())
}

fn rows(fixture: &Fixture, first: u16, last: u16) -> Vec<String> {
    (first..=last).map(|y| (0..fixture.tui.frame().size.width).map(|x| fixture.tui.frame().get(x, y).map_or(' ', |c| if c.ch == '\0' { ' ' } else { c.ch })).collect()).collect()
}

#[test]
fn the_golden_chrome_rows_and_declared_chips_match_what_the_engine_paints() {
    let fixture_value = parse(FIXTURE, JsonMemberPolicy::Reject).expect("fixture parses");
    let fixture = build(field(&fixture_value, "scene"));
    let golden = field(&fixture_value, "frame");
    let first = number(golden, "first") as u16;
    let expected: Vec<&str> = field(golden, "rows").as_array().unwrap().iter().map(|row| row.as_str().unwrap()).collect();
    let painted = rows(&fixture, first, first + expected.len() as u16 - 1);
    for (row, (want, got)) in expected.iter().zip(&painted).enumerate() {
        assert_eq!(got, want, "golden chrome row {}", first as usize + row);
    }
    for chip in field(&fixture_value, "chips").as_array().unwrap() {
        let window = fixture.names[text(chip, "window")];
        let rect = fixture.tui.scene.rect(window);
        let NodeContent::Chrome(ChromeState::Window(state)) = &fixture.tui.scene.node(window).content else { panic!("window chrome") };
        let layout = window_chip_layout(state, rect);
        let found = layout.groups.iter().flat_map(|group| group.tabs.iter()).find(|tab| tab.kind == TabKind::Tab && tab.index == number(chip, "index") as usize).expect("declared chip exists");
        assert_eq!(u64::from(found.x), number(chip, "x"), "chip x of {chip:?}");
        assert_eq!(u64::from(found.x + found.interior_width + 1), number(chip, "right"), "chip right wall of {chip:?}");
        assert_eq!(found.close_x.map(u64::from), field(chip, "close").as_u64(), "close column of {chip:?}");
    }
    for control in field(&fixture_value, "controls").as_array().unwrap() {
        let window = fixture.names[text(control, "window")];
        let rect = fixture.tui.scene.rect(window);
        let NodeContent::Chrome(ChromeState::Window(state)) = &fixture.tui.scene.node(window).content else { panic!("window chrome") };
        let layout = window_chip_layout(state, rect);
        let found = layout.groups.iter().flat_map(|group| group.tabs.iter()).find(|tab| tab.kind == TabKind::Controls).expect("declared controls chip exists");
        assert_eq!(u64::from(found.x), number(control, "x"), "controls x of {control:?}");
        assert_eq!(u64::from(found.x + found.interior_width + 1), number(control, "right"), "controls right wall of {control:?}");
        assert_eq!(found.maximize_x.map(u64::from), field(control, "maximize").as_u64(), "maximize column of {control:?}");
    }
    for window in field(&fixture_value, "windows").as_array().unwrap() {
        let rect = fixture.tui.scene.rect(fixture.names[text(window, "window")]);
        assert_eq!((u64::from(rect.x), u64::from(rect.x + rect.width - 1), u64::from(rect.y + 1)), (number(window, "x"), number(window, "right"), number(window, "textRow")));
    }
}

#[test]
fn elision_vectors_match_the_declared_outputs() {
    let fixture_value = parse(FIXTURE, JsonMemberPolicy::Reject).expect("fixture parses");
    for case in field(&fixture_value, "elision").as_array().unwrap() {
        let out = crate::tui::text::elide_end(text(case, "text"), number(case, "max") as u16);
        assert_eq!(&*out, text(case, "out"), "elide {case:?}");
    }
}

#[test]
fn every_case_of_the_routing_table_produces_the_declared_signals_focus_and_hover() {
    let fixture_value = parse(FIXTURE, JsonMemberPolicy::Reject).expect("fixture parses");
    for case in field(&fixture_value, "cases").as_array().unwrap() {
        let name = text(case, "name");
        let mut fixture = build(field(&fixture_value, "scene"));
        if let Some(capture) = case.get("capture").and_then(Value::as_str) {
            let node = fixture.names[capture];
            fixture.tui.capture(Some(node));
        }
        if let Some(focus) = case.get("start_focus").and_then(Value::as_str) {
            let node = fixture.names[focus];
            fixture.tui.set_focus(Some(node));
        }
        let mut signals = Vec::new();
        for spec in field(case, "events").as_array().unwrap() {
            let (event, at) = event(spec);
            for (node, signal) in fixture.tui.dispatch_at(&event, at) {
                signals.push(format!("{}:{}", name_of(&fixture, Some(node)).unwrap_or_else(|| "?".into()), signal_name(&signal)));
            }
        }
        let expected: Vec<&str> = field(case, "signals").as_array().unwrap().iter().map(|s| s.as_str().unwrap()).collect();
        assert_eq!(signals, expected, "case {name}: signals");
        assert_eq!(name_of(&fixture, fixture.tui.focus()).as_deref(), case.get("focus").and_then(Value::as_str), "case {name}: focus");
        assert_eq!(name_of(&fixture, fixture.tui.hovered()).as_deref(), case.get("hovered").and_then(Value::as_str), "case {name}: hovered");
    }
}
