use semio_framework_ui_tui_core::cell::{Cell, CellBuffer};
use semio_framework_ui_tui_frame::chrome::{window_chip_layout, ChromeMode, ChromeState, TabKind, WindowStackTabState, WindowState};
use semio_framework_ui_tui_core::geometry::{Rect, Size};
use semio_framework_ui_tui_frame::layout::WindowStackCorner;
use semio_framework_ui_tui_core::theme::{Role, Theme};
use semio_framework_pack_json::{parse, JsonMemberPolicy, Value};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use tiny_skia::{Paint, Pixmap, Transform};
use ui_styling::appearance::AppearanceName;

type Point = (u16, u16);
const UP: u8 = 1;
const RIGHT: u8 = 2;
const DOWN: u8 = 4;
const LEFT: u8 = 8;

fn number(value: &Value, field: &str) -> u16 { value.get(field).unwrap().as_u64().unwrap() as u16 }
fn text<'a>(value: &'a Value, field: &str) -> &'a str { value.get(field).unwrap().as_str().unwrap() }
fn nums(value: &Value, field: &str) -> Vec<u16> { value.get(field).unwrap().as_array().unwrap().iter().map(|n| n.as_u64().unwrap() as u16).collect() }
fn corner(name: &str) -> WindowStackCorner {
    match name { "top-left" => WindowStackCorner::TopLeft, "top-right" => WindowStackCorner::TopRight, "bottom-left" => WindowStackCorner::BottomLeft, "bottom-right" => WindowStackCorner::BottomRight, _ => unreachable!() }
}

fn edge(graph: &mut BTreeMap<Point, u8>, a: Point, b: Point, remove: bool) {
    let (out, incoming) = if a.0 < b.0 { (RIGHT, LEFT) } else if a.0 > b.0 { (LEFT, RIGHT) } else if a.1 < b.1 { (DOWN, UP) } else { (UP, DOWN) };
    for (point, mask) in [(a, out), (b, incoming)] {
        if remove { *graph.entry(point).or_default() &= !mask; }
        else { *graph.entry(point).or_default() |= mask; }
    }
}

fn horizontal(graph: &mut BTreeMap<Point, u8>, left: u16, right: u16, y: u16, remove: bool) {
    for x in left..right { edge(graph, (x, y), (x + 1, y), remove); }
}

fn vertical(graph: &mut BTreeMap<Point, u8>, x: u16, top: u16, bottom: u16) {
    for y in top..bottom { edge(graph, (x, y), (x, y + 1), false); }
}

fn mask(ch: char) -> u8 {
    match ch {
        '─' | '━' => LEFT | RIGHT, '│' | '┃' => UP | DOWN,
        '┌' | '┏' => RIGHT | DOWN, '┐' | '┓' => LEFT | DOWN,
        '└' | '┗' => RIGHT | UP, '┘' | '┛' => LEFT | UP,
        '├' | '┣' => UP | DOWN | RIGHT, '┤' | '┫' => UP | DOWN | LEFT,
        '┬' | '┳' => LEFT | RIGHT | DOWN, '┴' | '┻' => LEFT | RIGHT | UP,
        '┼' | '╋' => UP | RIGHT | DOWN | LEFT, _ => 0,
    }
}

#[test]
fn window_contour_fixture_matches_connected_direction_graph_and_independent_raster() {
    assert!(include_str!("🥒️.feature").starts_with("@capability-tui-window-contour"));
    let fixture = parse(include_str!("../../🧫️fixtures/🖼️window-contour/🔣️.json"), JsonMemberPolicy::Reject).unwrap();
    for appearance in [AppearanceName::Dark, AppearanceName::Light] {
        let theme = Theme::new(appearance);
        for case in fixture.get("cases").unwrap().as_array().unwrap() {
            let name = text(case, "name");
            let size = nums(case, "size");
            let body = nums(case, "body");
            let (left, top, right, bottom) = (body[0], body[1], body[0] + body[2] - 1, body[1] + body[3] - 1);
            let active = number(case, "active") as usize;
            let focused = case.get("focused").unwrap().as_bool().unwrap();
            let emphasized = case.get("emphasized").unwrap().as_bool().unwrap();
            let mode = text(case, "mode");
            let tabs = case.get("tabs").unwrap().as_array().unwrap();
            let rendered = case.get("rendered").and_then(Value::as_array).unwrap_or(tabs);
            let body_role = match text(case, "bodyRole") { "active" => Role::ActiveBase, "emphasized" => Role::BorderEmphasized, "normal" => Role::BorderNormal, _ => unreachable!() };
            let mut state = WindowState::new("host").with_stack_tab_states(tabs.iter().map(|tab| WindowStackTabState::new(text(tab, "label"), corner(text(tab, "corner")))).collect(), active);
            state.focused = focused;
            state.closable = false;
            state.maximizable = false;
            state.compact = name == "explicit-compact";
            let rect = Rect::new(0, 0, size[0], size[1]);
            let layout = window_chip_layout(&state, rect);
            assert_eq!(layout.mode, match mode { "full" => ChromeMode::Full, "compact" => ChromeMode::Compact, _ => ChromeMode::Flat }, "{name}");
            if mode != "flat" {
                for tab in layout.groups.iter().flat_map(|group| &group.tabs) {
                    let declared = rendered.iter().enumerate().find(|(index, declared)| match tab.kind {
                        TabKind::Tab => declared.get("index").map(|value| value.as_u64() == Some(tab.index as u64)).unwrap_or(*index == tab.index),
                        TabKind::OverflowPrev(_) => declared.get("kind").and_then(Value::as_str) == Some("overflow-prev"),
                        TabKind::OverflowNext(_) => declared.get("kind").and_then(Value::as_str) == Some("overflow-next"),
                        _ => false,
                    }).unwrap().1;
                    assert_eq!((tab.x, tab.interior_width + 2), (number(declared, "x"), number(declared, "width")), "declared geometry {name}");
                }
            }
            let sentinel = Cell { ch: '.', fg: [1, 2, 3], bg: [4, 5, 6], attrs: 0, width: 1 };
            let mut actual = CellBuffer::new(Size { width: size[0], height: size[1] }, sentinel);
            ChromeState::Window(Box::new(state)).paint_emphasized(&theme, rect, &mut actual, emphasized);
            let mut graph = BTreeMap::new();
            horizontal(&mut graph, left, right, top, false);
            horizontal(&mut graph, left, right, bottom, false);
            vertical(&mut graph, left, top, bottom);
            vertical(&mut graph, right, top, bottom);
            let mut roles = BTreeMap::new();
            let mut captions = BTreeSet::new();
            for (position, tab) in rendered.iter().enumerate().filter(|_| mode != "flat") {
                let index = tab.get("index").map(|value| value.as_u64().map(|n| n as usize)).unwrap_or(Some(position));
                let x = number(tab, "x");
                let end = x + number(tab, "width") - 1;
                let is_top = text(tab, "corner").starts_with("top");
                let seam = if mode == "compact" || is_top { top } else { bottom };
                let outer = if mode == "compact" { top } else if is_top { 0 } else { size[1] - 1 };
                let role = if index == Some(active) { Role::ActiveBase } else if emphasized { Role::BorderEmphasized } else { Role::BorderNormal };
                if mode == "compact" {
                    for xx in x + 1..end { captions.insert((xx, top)); }
                    for xx in [x, end] {
                        if xx > left && xx < right { *graph.entry((xx, top)).or_default() |= DOWN; }
                        if index == Some(active) || roles.get(&(xx, top)) != Some(&Role::ActiveBase) { roles.insert((xx, top), role); }
                    }
                } else {
                    horizontal(&mut graph, x, end, outer, false);
                    vertical(&mut graph, x, outer.min(seam), outer.max(seam));
                    vertical(&mut graph, end, outer.min(seam), outer.max(seam));
                    if index == Some(active) { horizontal(&mut graph, x, end, seam, true); }
                    for xx in x..=end { roles.insert((xx, outer), role); }
                    for yy in outer.min(seam)..=outer.max(seam) {
                        for xx in [x, end] { roles.insert((xx, yy), if yy == seam && index != Some(active) { body_role } else { role }); }
                    }
                }
            }
            graph.retain(|_, mask| *mask != 0);
            let start = *graph.keys().next().unwrap();
            let mut visited = BTreeSet::from([start]);
            let mut queue = VecDeque::from([start]);
            while let Some((x, y)) = queue.pop_front() {
                for (bit, dx, dy, reciprocal) in [(UP, 0, -1, DOWN), (RIGHT, 1, 0, LEFT), (DOWN, 0, 1, UP), (LEFT, -1, 0, RIGHT)] {
                    if graph[&(x, y)] & bit == 0 { continue; }
                    let neighbor = ((x as i32 + dx) as u16, (y as i32 + dy) as u16);
                    if graph.get(&neighbor).is_some_and(|mask| mask & reciprocal != 0) && visited.insert(neighbor) { queue.push_back(neighbor); }
                }
            }
            assert_eq!(visited.len(), graph.len(), "one connected contour {name}");
            let mut oracle = Pixmap::new(size[0] as u32 * 3, size[1] as u32 * 3).unwrap();
            for (&(x, y), &directions) in &graph {
                if captions.contains(&(x, y)) { continue; }
                let cell = actual.get(x, y).unwrap();
                assert_eq!(mask(cell.ch), directions, "direction union {name} at {x},{y}: {:?}", cell.ch);
                let rgb = theme.role(*roles.get(&(x, y)).unwrap_or(&body_role));
                let mut ink = Paint::default();
                ink.set_color_rgba8(rgb[0], rgb[1], rgb[2], 255);
                ink.anti_alias = false;
                for (bit, dx, dy) in [(0, 0, 0), (UP, 0, -1), (RIGHT, 1, 0), (DOWN, 0, 1), (LEFT, -1, 0)] {
                    if bit != 0 && directions & bit == 0 { continue; }
                    oracle.fill_rect(tiny_skia::Rect::from_xywh((x as i32 * 3 + 1 + dx) as f32, (y as i32 * 3 + 1 + dy) as f32, 1.0, 1.0).unwrap(), &ink, Transform::identity(), None);
                }
            }
            for (index, pixel) in oracle.pixels().iter().enumerate().filter(|(_, pixel)| pixel.alpha() != 0) {
                let x = (index % (size[0] as usize * 3) / 3) as u16;
                let y = (index / (size[0] as usize * 3) / 3) as u16;
                assert_eq!(actual.get(x, y).unwrap().fg, [pixel.red(), pixel.green(), pixel.blue()], "raster color {name} at {x},{y}");
            }
            for y in 0..size[1] {
                for x in 0..size[0] {
                    let cell = actual.get(x, y).unwrap();
                    if mask(cell.ch) != 0 { assert!(graph.contains_key(&(x, y)), "unexpected contour {name} at {x},{y}"); }
                    let in_body = x >= left && x <= right && y >= top && y <= bottom;
                    let in_tab = mode == "full" && rendered.iter().any(|tab| x >= number(tab, "x") && x < number(tab, "x") + number(tab, "width") && if text(tab, "corner").starts_with("top") { y <= top } else { y >= bottom });
                    if !in_body && !in_tab { assert_eq!(*cell, sentinel, "parent notch {name} at {x},{y}"); }
                }
            }
        }
    }
}
