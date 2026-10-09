//! 🔬️ Property test for D08/D19/R07: incremental `Tui::render` patches, applied to a terminal emulator, always
//! reproduce what `render_full` paints, across random scene, pointer, key, overlay, layout and resize mutations.
//! The emulator is the owned VT interpreter, so the oracle shares no code with the diff and patch emitter.

use crate::tui::chrome::{shell, ChromeState, FooterState, KeyHint, NavItem, NavbarState, Shell, WindowState};
use crate::tui::engine::Tui;
use crate::tui::event::{Event, Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Size};
use crate::tui::layout::{push_window_to_stack, zoom_window, Constraint, Dimension, Direction, WindowLayout, WindowLayoutWindowNode};
use crate::tui::menu::MenuItem;
use crate::tui::palette::{PaletteItem, PaletteState};
use crate::tui::scene::{Node, NodeContent, NodeId};
use crate::tui::theme::Theme;
use crate::tui::vt::VtScreen;
use crate::tui::widget::{InputState, ListState, TabsState, WidgetState, WizardState};
use ui_styling::appearance::AppearanceName;

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n.max(1) as u64) as usize
    }
}

struct World {
    tui: Tui,
    shell: Shell,
    layout: WindowLayout,
    created: Vec<NodeId>,
}

fn build(size: Size) -> World {
    let mut tui = Tui::new(size, Theme::new(AppearanceName::Dark));
    let mut layout = crate::tui::layout::create_default_layout(&["w1".into(), "w2".into(), "w3".into()], "row", None, Some(&["One".into(), "Two".into(), "Three".into()]));
    push_window_to_stack(&mut layout, "w1", WindowLayoutWindowNode { window_kind_id: "w4".into(), title: Some("Four".into()), corner: None });
    let nav = |id: &str| NavItem { id: id.into(), label: id.into(), active: false };
    let navbar = NavbarState { left: vec![nav("left"), nav("more")], center: vec![nav("centre")], right: vec![nav("right")] };
    let footer = FooterState { hints: vec![KeyHint { key: "q".into(), label: "quit".into() }, KeyHint { key: "z".into(), label: "zoom".into() }], status: "ready".into() };
    let mut built = shell(&mut tui.scene, navbar, footer, &layout);
    let canvas = built.canvas;
    let w4 = tui.scene.add(canvas, Node::new(NodeContent::Chrome(ChromeState::Window(Box::new(WindowState::new("Four"))))));
    built.windows.push(("w4".into(), w4));
    let mut created = Vec::new();
    for (index, (_, window)) in built.windows.clone().into_iter().enumerate() {
        let widget = match index % 3 {
            0 => WidgetState::List(ListState::new((0..12).map(|i| if i % 3 == 0 { format!("\u{2328}\u{fe0f} row {i}") } else { format!("row {i}") }).collect())),
            1 => WidgetState::Tabs(TabsState::new(vec!["alpha".into(), "beta".into(), "gamma".into()], 0)),
            _ => WidgetState::Wizard(WizardState::new((0..30).map(|i| format!("option {i}")).collect())),
        };
        let id = tui.scene.add(window, Node::new(NodeContent::Widget(widget)));
        tui.scene.node_mut(id).set_constraint(Constraint { width: Dimension::Weight(1), height: Dimension::Weight(1), ..Default::default() });
        created.push(id);
    }
    built.remount(&mut tui.scene, &layout);
    World { tui, shell: built, layout, created }
}

fn nodes(world: &World) -> Vec<NodeId> {
    fn walk(world: &World, id: NodeId, out: &mut Vec<NodeId>) {
        out.push(id);
        for &child in world.tui.scene.node(id).children() {
            walk(world, child, out);
        }
    }
    let mut out = Vec::new();
    walk(world, world.tui.scene.root(), &mut out);
    walk(world, world.tui.scene.overlay_root(), &mut out);
    out
}

#[derive(Clone, Debug)]
enum Op {
    AddWidget { parent: usize, kind: usize },
    AddBox { parent: usize, row: bool },
    Remove { node: usize },
    SetText { node: usize, value: usize },
    SetConstraint { node: usize, w: usize, h: usize, dir: usize, pad: u16 },
    SetVisible { node: usize, visible: bool },
    MutateWidget { node: usize, value: usize },
    MutateChrome { node: usize, value: usize },
    Reparent { node: usize, parent: usize },
    Focus { node: usize },
    Mouse { kind: usize, x: u16, y: u16 },
    Key { key: usize },
    Resize { width: u16, height: u16 },
    Appearance,
    OpenOverlay { kind: usize, x: u16, y: u16 },
    CloseOverlay,
    Zoom { window: usize },
    Status { window: usize, status: usize },
    Tick { ms: u64 },
}

fn random_op(rng: &mut Rng, size: Size) -> Op {
    let x = rng.below(usize::from(size.width)) as u16;
    let y = rng.below(usize::from(size.height)) as u16;
    match rng.below(18) {
        0 => Op::AddWidget { parent: rng.below(1000), kind: rng.below(5) },
        1 => Op::AddBox { parent: rng.below(1000), row: rng.below(2) == 0 },
        2 => Op::Remove { node: rng.below(1000) },
        3 => Op::SetText { node: rng.below(1000), value: rng.below(11) },
        4 => Op::SetConstraint { node: rng.below(1000), w: rng.below(3), h: rng.below(3), dir: rng.below(3), pad: rng.below(3) as u16 },
        5 => Op::SetVisible { node: rng.below(1000), visible: rng.below(3) != 0 },
        6 => Op::MutateWidget { node: rng.below(1000), value: rng.below(8) },
        7 => Op::MutateChrome { node: rng.below(1000), value: rng.below(6) },
        8 => Op::Reparent { node: rng.below(1000), parent: rng.below(1000) },
        9 => Op::Focus { node: rng.below(1000) },
        10 | 11 => Op::Mouse { kind: rng.below(7), x, y },
        12 => Op::Key { key: rng.below(7) },
        13 => Op::Resize { width: 20 + rng.below(70) as u16, height: 6 + rng.below(44) as u16 },
        14 => {
            if rng.below(4) == 0 {
                Op::Appearance
            } else {
                Op::Zoom { window: rng.below(4) }
            }
        }
        15 if rng.below(3) == 0 => Op::Status { window: rng.below(4), status: rng.below(4) },
        15 => Op::OpenOverlay { kind: rng.below(4), x, y },
        16 => Op::CloseOverlay,
        _ => Op::Tick { ms: 100 + rng.below(900) as u64 },
    }
}

fn apply(world: &mut World, op: &Op, clock: &mut u64) -> Vec<(NodeId, crate::tui::widget::WidgetSignal)> {
    let all = nodes(world);
    let pick = |index: usize| all[index % all.len()];
    let tui = &mut world.tui;
    let mut signals = Vec::new();
    match op {
        Op::AddWidget { parent, kind } => {
            let parent = pick(*parent);
            if matches!(tui.scene.node(parent).content, NodeContent::Widget(_)) {
                return signals;
            }
            let content = match kind {
                0 => NodeContent::Text("text node".into()),
                1 => NodeContent::Widget(WidgetState::List(ListState::new(vec!["a".into(), "b".into()]))),
                2 => NodeContent::Widget(WidgetState::Input(InputState { value: "typed".into(), cursor: 2, placeholder: "ph".into() })),
                3 => NodeContent::Widget(WidgetState::Tabs(TabsState::new(vec!["t1".into(), "t2".into()], 1))),
                _ => NodeContent::Widget(WidgetState::Wizard(WizardState::new(vec!["x".into(), "y".into(), "z".into()]))),
            };
            let id = tui.scene.add(parent, Node::new(content));
            world.created.push(id);
        }
        Op::AddBox { parent, row } => {
            let parent = pick(*parent);
            if matches!(tui.scene.node(parent).content, NodeContent::Widget(_)) {
                return signals;
            }
            let id = tui.scene.add(parent, Node::new(NodeContent::Box));
            tui.scene.node_mut(id).set_constraint(Constraint { direction: if *row { Direction::Row } else { Direction::Column }, width: Dimension::Weight(1), height: Dimension::Weight(1), gap: 1, ..Default::default() });
            world.created.push(id);
        }
        Op::Remove { node } => {
            let alive: Vec<NodeId> = world.created.iter().copied().filter(|id| tui.scene.contains(*id)).collect();
            if let Some(&victim) = alive.get(node % alive.len().max(1)) {
                tui.scene.remove(victim);
            }
        }
        Op::SetText { node, value } => {
            let id = pick(*node);
            if matches!(tui.scene.node(id).content, NodeContent::Text(_)) {
                tui.scene.node_mut(id).set_text(["", "a", "longer text value", "x y z", "\u{2500}\u{2500}", "tail", "\u{2328}\u{fe0f} kbd", "\u{1f9f0}\u{fe0f}framework", "\u{65e5}\u{672c}\u{8a9e} wide", "e\u{301}clair", "\u{1f468}\u{200d}\u{1f469} zwj"][*value].to_string());
            }
        }
        Op::SetConstraint { node, w, h, dir, pad } => {
            let id = pick(*node);
            if id == tui.scene.root() || id == tui.scene.overlay_root() {
                return signals;
            }
            let dim = |v: usize| match v {
                0 => Dimension::Auto,
                1 => Dimension::Cells(3 + (v as u16) * 2),
                _ => Dimension::Weight(1 + v as u16),
            };
            let direction = match dir {
                0 => Direction::Row,
                1 => Direction::Column,
                _ => Direction::Stack,
            };
            tui.scene.node_mut(id).set_constraint(Constraint { direction, width: dim(*w), height: dim(*h), gap: *pad % 2, padding: [*pad, 0, *pad, 0] });
        }
        Op::SetVisible { node, visible } => {
            let id = pick(*node);
            if id != tui.scene.root() && id != tui.scene.overlay_root() {
                tui.scene.node_mut(id).set_visible(*visible);
            }
        }
        Op::MutateWidget { node, value } => {
            let id = pick(*node);
            if let Some(widget) = tui.scene.node_mut(id).widget() {
                match widget {
                    WidgetState::List(list) => list.selected = (list.selected + value) % list.items.len().max(1),
                    WidgetState::Tabs(tabs) => tabs.active = value % tabs.tabs.len().max(1),
                    WidgetState::Input(input) => {
                        input.value.push('z');
                        input.cursor = input.value.len();
                    }
                    WidgetState::Wizard(wizard) => {
                        wizard.list.set_query(["", "o", "option 1", "zz"][value % 4]);
                    }
                    _ => {}
                }
            }
        }
        Op::MutateChrome { node, value } => {
            let id = pick(*node);
            if let Some(chrome) = tui.scene.node_mut(id).chrome() {
                match chrome {
                    ChromeState::Window(window) => match value {
                        0 => window.title.push_str("\u{1f9f0}\u{fe0f}"),
                        1 => window.focused = !window.focused,
                        2 => window.new_tab = !window.new_tab,
                        3 => window.closable = !window.closable,
                        4 => window.peers = 1 + value % 3,
                        _ => window.number = Some("7".into()),
                    },
                    ChromeState::Footer(footer) => footer.status = ["", "ok", "a much longer status text for the footer", "\u{2328}\u{fe0f} \u{65e5}\u{672c} status"][value % 4].to_string(),
                    ChromeState::Navbar(navbar) => navbar.center = vec![NavItem { id: "c".into(), label: ["", "dashboard", "a very long centre label"][value % 3].into(), active: false }],
                    ChromeState::Canvas => {}
                }
            }
        }
        Op::Reparent { node, parent } => {
            let alive: Vec<NodeId> = world.created.iter().copied().filter(|id| tui.scene.contains(*id)).collect();
            let Some(&child) = alive.get(node % alive.len().max(1)) else { return signals };
            let target = pick(*parent);
            if !matches!(tui.scene.node(target).content, NodeContent::Widget(_)) {
                tui.scene.reparent(child, target);
            }
        }
        Op::Focus { node } => {
            let id = pick(*node);
            tui.set_focus(Some(id));
        }
        Op::Mouse { kind, x, y } => {
            let pos = Pos { x: *x, y: *y };
            let kind = match kind {
                0 => MouseKind::Move,
                1 => MouseKind::Down(MouseButton::Left),
                2 => MouseKind::Up(MouseButton::Left),
                3 => MouseKind::Drag(MouseButton::Left),
                4 => MouseKind::Down(MouseButton::Right),
                5 => MouseKind::Scroll { dx: 0, dy: 1 },
                _ => MouseKind::Scroll { dx: 0, dy: -1 },
            };
            signals = tui.dispatch_at(&Event::Mouse(MouseEvent { kind, pos, mods: 0, clicks: 1 }), *clock);
            for (_, signal) in &signals {
                if let crate::tui::widget::WidgetSignal::WindowMaximize(_) = signal {
                    let zoom = if world.layout.zoomed.is_some() { None } else { Some("w2") };
                    zoom_window(&mut world.layout, zoom);
                    world.shell.remount(&mut tui.scene, &world.layout);
                }
            }
        }
        Op::Key { key } => {
            let key = match key {
                0 => Key::Tab,
                1 => Key::BackTab,
                2 => Key::Down,
                3 => Key::Up,
                4 => Key::Enter,
                5 => Key::Esc,
                _ => Key::Char('q'),
            };
            signals = tui.dispatch_at(&Event::Key(KeyEvent { key, mods: 0 }), *clock);
        }
        Op::Resize { width, height } => {
            signals = tui.dispatch_at(&Event::Resize(Size { width: *width, height: *height }), *clock);
        }
        Op::Appearance => tui.set_appearance(if tui.theme.appearance == AppearanceName::Dark { AppearanceName::Light } else { AppearanceName::Dark }),
        Op::OpenOverlay { kind, x, y } => {
            let anchor = Pos { x: *x, y: *y };
            match kind {
                0 => {
                    tui.open_menu(anchor, vec![MenuItem::new("copy").with_shortcut("C-c"), MenuItem::separator(), MenuItem::new("paste").disabled(), MenuItem::new("close")]);
                }
                1 => {
                    tui.open_dialog(crate::tui::dialog::DialogState::new("Title", "A body that wraps over more than one line of the dialog", vec!["No".into(), "Yes".into()]));
                }
                2 => {
                    let mut palette = PaletteState::new((0..14).map(|i| PaletteItem::new(format!("command {i}"), if i % 3 == 0 { "C-x" } else { "" })).collect());
                    palette.placeholder = "search".into();
                    tui.open_palette(palette);
                }
                _ => {
                    tui.show_tooltip(anchor, "hint text");
                }
            }
        }
        Op::CloseOverlay => {
            if let Some(&top) = tui.overlays().last() {
                tui.close_overlay(top);
            }
        }
        Op::Zoom { window } => {
            let id = ["w1", "w2", "w3", "w4"][window % 4];
            let zoom = if world.layout.zoomed.as_deref() == Some(id) { None } else { Some(id) };
            zoom_window(&mut world.layout, zoom);
            world.shell.remount(&mut tui.scene, &world.layout);
        }
        Op::Status { window, status } => {
            let id = ["w1", "w2", "w3", "w4"][window % 4];
            let status = [None, Some(crate::tui::theme::Status::Running), Some(crate::tui::theme::Status::Success), Some(crate::tui::theme::Status::Failure)][*status];
            world.shell.set_status(&mut tui.scene, id, status);
        }
        Op::Tick { ms } => {
            *clock += ms;
            tui.tick(*clock);
        }
    }
    signals
}

fn paint(screen: &mut VtScreen, patch: &str) {
    screen.feed(patch.as_bytes());
}

fn assert_same_screen(a: &VtScreen, b: &VtScreen, size: Size, context: &str) {
    for y in 0..size.height {
        for x in 0..size.width {
            let (ca, cb) = (a.cell_at(x, y).expect("cell a"), b.cell_at(x, y).expect("cell b"));
            let blank = |c: char| if c == '\0' { ' ' } else { c };
            assert!(blank(ca.ch) == blank(cb.ch) && ca.fg == cb.fg && ca.bg == cb.bg, "{context}: cell ({x},{y}) incremental {:?} vs full {:?}", (ca.ch, ca.fg, ca.bg), (cb.ch, cb.fg, cb.bg));
        }
    }
}

fn run(seed: u64, steps: usize) {
    let mut rng = Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    let size = Size { width: 40 + rng.below(50) as u16, height: 10 + rng.below(40) as u16 };
    let (mut a, mut b) = (build(size), build(size));
    let mut screen_a = VtScreen::new(size, 0);
    let mut current = size;
    let mut clock = 0u64;
    let mut clock_b = 0u64;
    paint(&mut screen_a, &a.tui.render().0);
    b.tui.render_full();
    for step in 0..steps {
        let op = random_op(&mut rng, current);
        let signals_a = apply(&mut a, &op, &mut clock);
        let signals_b = apply(&mut b, &op, &mut clock_b);
        assert_eq!(signals_a, signals_b, "seed {seed} step {step}: the same input must give the same signals ({op:?})");
        if let Op::Resize { width, height } = op {
            current = Size { width, height };
            screen_a = VtScreen::new(current, 0);
        }
        if rng.below(10) < 7 {
            let patch = a.tui.render();
            paint(&mut screen_a, &patch.0);
            let mut screen_b = VtScreen::new(current, 0);
            let full = b.tui.render_full();
            paint(&mut screen_b, &full.0);
            assert_same_screen(&screen_a, &screen_b, current, &format!("seed {seed} step {step} after {op:?}"));
            for y in 0..current.height {
                for x in 0..current.width {
                    assert_eq!(a.tui.frame().get(x, y), b.tui.frame().get(x, y), "seed {seed} step {step}: engine frames differ at ({x},{y}) after {op:?}");
                }
            }
        }
    }
}

/// 🥒️ The language-agnostic feature this adapter runs, bound at compile time so renaming it breaks the build.
const FEATURE: &str = include_str!("../🎞️render-equivalence/🥒️.feature");

#[test]
fn the_adapter_runs_the_declared_feature() {
    assert_eq!(FEATURE.lines().next(), Some("@capability-tui-incremental-render"));
}

#[test]
fn incremental_render_patches_match_full_render_on_random_mutations() {
    for seed in 1..=64 {
        run(seed, 80);
    }
}

#[test]
fn mutating_a_node_after_the_first_frame_yields_a_non_empty_patch() {
    let mut tui = Tui::new(Size { width: 20, height: 4 }, Theme::new(AppearanceName::Dark));
    let root = tui.scene.root();
    let label = tui.scene.add(root, Node::new(NodeContent::Text("one".into())));
    tui.scene.node_mut(label).set_constraint(Constraint { height: Dimension::Cells(1), ..Default::default() });
    let first = tui.render();
    assert!(!first.0.is_empty());
    assert!(tui.render().0.is_empty(), "an unchanged scene emits nothing");
    for text in ["xyz", "123", "abcd"] {
        tui.scene.node_mut(label).set_text(text);
        let patch = tui.render();
        assert!(patch.0.contains(text), "round {text}: {:?}", patch.0);
        assert!(tui.render().0.is_empty());
    }
}

#[test]
fn a_small_mutation_repaints_only_the_changed_cells() {
    let mut tui = Tui::new(Size { width: 40, height: 6 }, Theme::new(AppearanceName::Dark));
    let root = tui.scene.root();
    tui.scene.node_mut(root).set_constraint(Constraint { direction: Direction::Column, ..Default::default() });
    let list = tui.scene.add(root, Node::new(NodeContent::Widget(WidgetState::List(ListState::new((0..5).map(|i| format!("row {i}")).collect())))));
    tui.scene.node_mut(list).set_constraint(Constraint { height: Dimension::Weight(1), ..Default::default() });
    let full = tui.render().0.len();
    if let Some(WidgetState::List(l)) = tui.scene.node_mut(list).widget() {
        l.marks[1] = true;
    }
    let patch = tui.render().0;
    assert!(patch.len() * 4 < full, "a one-cell change must not re-emit the frame: {} of {}", patch.len(), full);
}

#[test]
fn the_engine_measures_scalars_by_default_so_emulators_that_count_scalars_stay_aligned() {
    use crate::tui::text::WidthMode;
    let size = Size { width: 30, height: 3 };
    let mut tui = Tui::new(size, Theme::new(AppearanceName::Dark));
    assert_eq!(tui.width_mode(), WidthMode::Scalar);
    let root = tui.scene.root();
    let text = tui.scene.add(root, Node::new(NodeContent::Text("\u{2328}\u{fe0f}ab \u{1f9f0}\u{fe0f}cd".into())));
    tui.scene.node_mut(text).set_constraint(Constraint { height: Dimension::Cells(1), ..Default::default() });
    let mut screen = VtScreen::new(size, 0);
    paint(&mut screen, &tui.render().0);
    for x in 0..size.width {
        let (engine, emulator) = (tui.frame().get(x, 0).unwrap(), screen.cell_at(x, 0).unwrap());
        assert_eq!(engine.ch.max(' '), emulator.ch.max(' '), "column {x}: engine and a scalar-counting emulator agree where every glyph sits");
    }
    tui.set_width_mode(WidthMode::Cluster);
    assert_eq!(tui.width_mode(), WidthMode::Cluster);
    assert!(!tui.render().0.is_empty(), "choosing a width mode repaints in full");
}

#[test]
fn scalar_width_rendering_preserves_combining_marks() {
    use crate::tui::cell::{Cell, CellBuffer};
    use crate::tui::geometry::Rect;
    use crate::tui::text::WidthMode;
    use unicode_segmentation::UnicodeSegmentation;
    use unicode_width::UnicodeWidthStr;
    for text in ["e\u{301}", "A界e\u{301}", "a\u{308}\u{301}", "ש\u{5bc}"] {
        let size = Size { width: 20, height: 1 };
        let mut buffer = CellBuffer::new(size, Cell::blank([255; 3], [0; 3]));
        buffer.set_width_mode(WidthMode::Scalar);
        let width = buffer.put_str(Pos { x: 0, y: 0 }, text, [255; 3], [0; 3], 0, Rect::new(0, 0, size.width, size.height));
        assert_eq!(usize::from(width), UnicodeWidthStr::width(text));
        assert_eq!(buffer.row_text(0).trim_end(), text.graphemes(true).collect::<String>());
        let mut patch = crate::tui::ansi::AnsiPatch::default();
        crate::tui::ansi::emit_runs(&buffer, &[crate::tui::cell::DiffRun { x: 0, y: 0, len: width }], &mut patch);
        assert!(patch.0.contains(text), "scalar ANSI glyphs lost the mark: {:?}", patch.0);
    }
}
