//! 🔬️ Window anatomy, tab strip, compact chrome, ellipsis and the navbar/footer bands (R02 R06 R08 R13 R16 R17 R23).

use crate::tui::cell::{Cell, CellBuffer};
use crate::tui::chrome::{window_chip_layout, ChromeMode, ChromeState, FooterState, KeyHint, NavItem, NavbarState, TabKind, WindowState};
use crate::tui::geometry::{Pos, Rect, Size};
use crate::tui::theme::{Role, Theme};
use crate::tui::widget::WidgetSignal;
use ui_styling::appearance::AppearanceName;

fn paint(state: WindowState, width: u16, height: u16) -> (CellBuffer, ChromeState) {
    let theme = Theme::new(AppearanceName::Dark);
    let mut buf = CellBuffer::new(Size { width, height }, Cell::blank([0, 0, 0], [0, 0, 0]));
    let chrome = ChromeState::Window(Box::new(state));
    chrome.paint(&theme, Rect::new(0, 0, width, height), &mut buf);
    (buf, chrome)
}

fn row(buf: &CellBuffer, y: u16) -> String {
    (0..buf.size.width).filter_map(|x| buf.get(x, y)).map(|c| c.ch).collect()
}

fn many(count: usize) -> WindowState {
    WindowState::new("host").with_stack_tabs((0..count).map(|i| format!("task number {i}")).collect(), 0)
}

#[test]
fn a_long_title_is_capped_to_the_chip_width_and_the_next_tab_stays_visible() {
    let state = WindowState::new("host").with_stack_tabs(vec!["a very long task title that would eat the whole strip".into(), "second".into()], 0);
    let (buf, _) = paint(state, 100, 8);
    let text = row(&buf, 1);
    assert!(text.contains('\u{2026}'), "the long title is elided: {text:?}");
    assert!(text.contains("second"), "the other tab is not squeezed out: {text:?}");
    let state = WindowState::new("host").with_stack_tabs(vec!["a very long task title that would eat the whole strip".into(), "second".into()], 0);
    let layout = window_chip_layout(&state, Rect::new(0, 0, 100, 8));
    let widest = layout.groups.iter().flat_map(|g| g.tabs.iter()).map(|t| t.interior_width + 2).max().unwrap();
    assert!(widest <= 24, "a chip is at most 24 cells wide, got {widest}");
}

#[test]
fn overflowing_tabs_show_counted_chips_and_keep_the_active_tab_in_view() {
    for active in [0usize, 5, 11] {
        let mut state = many(12);
        state.active_stack_tab = active;
        let rect = Rect::new(0, 0, 70, 8);
        let layout = window_chip_layout(&state, rect);
        let tabs: Vec<_> = layout.groups.iter().flat_map(|g| g.tabs.iter()).collect();
        assert!(tabs.iter().any(|t| t.kind == TabKind::Tab && t.index == active), "active tab {active} is always visible");
        assert!(tabs.iter().all(|t| t.x + t.interior_width + 2 <= 70), "nothing is clipped at the window edge");
        let hidden_before = tabs.iter().find(|t| matches!(t.kind, TabKind::OverflowPrev(_)));
        let hidden_after = tabs.iter().find(|t| matches!(t.kind, TabKind::OverflowNext(_)));
        assert_eq!(hidden_before.is_some(), active > 0 || tabs.iter().filter(|t| t.kind == TabKind::Tab).map(|t| t.index).min() != Some(0));
        let shown: Vec<usize> = tabs.iter().filter(|t| t.kind == TabKind::Tab).map(|t| t.index).collect();
        let first = *shown.first().unwrap();
        let last = *shown.last().unwrap();
        if let Some(chip) = hidden_before {
            assert_eq!(chip.interior.trim(), format!("\u{2039}{first}"), "the left chip counts the hidden tabs");
            assert_eq!(chip.kind, TabKind::OverflowPrev(first - 1));
        }
        if let Some(chip) = hidden_after {
            assert_eq!(chip.interior.trim(), format!("{}\u{203a}", 11 - last));
            assert_eq!(chip.kind, TabKind::OverflowNext(last + 1));
        }
        let window = ChromeState::Window(Box::new(state));
        if let Some(chip) = hidden_after {
            assert_eq!(window.window_hit(rect, Pos { x: chip.x + 1, y: 1 }), Some(WidgetSignal::WindowTabActivated(last + 1)), "an overflow chip reveals the next hidden tab");
        }
    }
}

#[test]
fn hit_testing_follows_the_painted_chips_cell_for_cell() {
    let mut state = many(5);
    state.active_stack_tab = 2;
    state.peers = 2;
    state.new_tab = true;
    let rect = Rect::new(0, 0, 120, 8);
    let layout = window_chip_layout(&state, rect);
    let (buf, window) = paint(state, 120, 8);
    let line = row(&buf, 1);
    let chars: Vec<char> = line.chars().collect();
    for x in 0..120u16 {
        let target = window.window_hit(rect, Pos { x, y: 1 });
        let glyph = chars[usize::from(x)];
        if glyph == '\u{2715}' {
            assert!(matches!(target, Some(WidgetSignal::WindowClose(_))), "close glyph at {x} must hit a close control");
        }
        if glyph == '\u{2922}' {
            assert_eq!(target, Some(WidgetSignal::WindowMaximize(2)));
        }
        if glyph == '+' {
            assert_eq!(target, Some(WidgetSignal::WindowNewTab(2)));
        }
    }
    let shown = layout.groups.iter().flat_map(|g| g.tabs.iter()).filter(|t| t.kind == TabKind::Tab).count();
    assert_eq!(shown, 5);
}

#[test]
fn the_maximize_control_needs_a_peer_and_swaps_to_restore_when_zoomed() {
    let (alone, _) = paint(WindowState::new("only"), 60, 8);
    assert!(!row(&alone, 1).contains('\u{2922}'), "a lone window hides maximize");
    let mut peers = WindowState::new("one of two");
    peers.peers = 2;
    let (buf, _) = paint(peers, 60, 8);
    assert!(row(&buf, 1).contains('\u{2922}'));
    let mut zoomed = WindowState::new("zoomed");
    zoomed.peers = 2;
    zoomed.zoomed = true;
    let (buf, _) = paint(zoomed, 60, 8);
    assert!(row(&buf, 1).contains('\u{2921}') && !row(&buf, 1).contains('\u{2922}'), "restore glyph while zoomed");
    let mut sole_zoomed = WindowState::new("zoomed");
    sole_zoomed.zoomed = true;
    sole_zoomed.maximizable = false;
    let (buf, _) = paint(sole_zoomed, 60, 8);
    assert!(!row(&buf, 1).contains('\u{2921}'), "a window that cannot maximize never offers restore either");
}

#[test]
fn the_focused_stack_is_distinguishable_without_colour() {
    let mut focused = WindowState::new("focus");
    focused.focused = true;
    let (heavy, _) = paint(focused, 40, 8);
    let (light, _) = paint(WindowState::new("focus"), 40, 8);
    assert_eq!(heavy.get(0, 7).unwrap().ch, '\u{2517}');
    assert_eq!(light.get(0, 7).unwrap().ch, '\u{2514}');
    let theme = Theme::new(AppearanceName::Dark);
    assert_eq!(heavy.get(0, 7).unwrap().fg, theme.role(Role::ActiveBase));
}

#[test]
fn hovering_a_chip_or_control_lights_it_with_the_hover_fill() {
    let theme = Theme::new(AppearanceName::Dark);
    let mut state = many(3);
    state.peers = 2;
    let rect = Rect::new(0, 0, 100, 8);
    let layout = window_chip_layout(&state, rect);
    let second = layout.groups[0].tabs.iter().find(|t| t.index == 1).unwrap();
    let (x, close) = (second.x + 2, second.close_x.unwrap());
    let mut window = ChromeState::Window(Box::new(state));
    assert!(window.set_hover(rect, Some(Pos { x, y: 1 })));
    assert!(!window.set_hover(rect, Some(Pos { x: x + 1, y: 1 })), "same chip, no repaint");
    let mut buf = CellBuffer::new(Size { width: 100, height: 8 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    window.paint(&theme, rect, &mut buf);
    assert_eq!(buf.get(x, 1).unwrap().bg, theme.role(Role::HoverInteractive));
    assert!(window.set_hover(rect, Some(Pos { x: close, y: 1 })));
    assert!(window.set_hover(rect, None));
}

#[test]
fn short_terminals_get_one_row_chrome_with_the_tabs_on_the_border() {
    let mut state = WindowState::new("host").with_stack_tabs(vec!["dev".into(), "tasks".into()], 1);
    state.compact = true;
    state.peers = 2;
    let rect = Rect::new(0, 0, 50, 6);
    assert_eq!(window_chip_layout(&state, rect).mode, ChromeMode::Compact);
    assert_eq!(crate::tui::chrome::window_content_padding(&state, rect), [1, 1, 1, 1]);
    let (buf, window) = paint(state, 50, 6);
    let top = row(&buf, 0);
    assert!(top.starts_with('\u{250c}') && top.ends_with('\u{2510}'), "{top:?}");
    assert!(top.contains("dev") && top.contains("tasks") && top.contains('\u{2922}'), "{top:?}");
    assert_eq!(row(&buf, 1).chars().next(), Some('\u{2502}'), "side walls start right under the border");
    assert_eq!(row(&buf, 5).chars().next(), Some('\u{2514}'), "no bottom chrome");
    let active = top.find("tasks").unwrap();
    let theme = Theme::new(AppearanceName::Dark);
    assert_eq!(buf.get(top[..active].chars().count() as u16, 0).unwrap().bg, theme.role(Role::ActiveBase));
    let dev_x = top.find("dev").unwrap() as u16;
    assert!(matches!(window.window_hit(Rect::new(0, 0, 50, 6), Pos { x: dev_x, y: 0 }), Some(WidgetSignal::WindowTabActivated(0))));
    let close = top.chars().position(|c| c == '\u{2715}').unwrap() as u16;
    assert_eq!(window.window_hit(Rect::new(0, 0, 50, 6), Pos { x: close, y: 0 }), Some(WidgetSignal::WindowClose(0)));
}

#[test]
fn a_full_chrome_window_too_short_for_two_rows_falls_back_to_the_compact_row() {
    let state = WindowState::new("tiny");
    assert_eq!(window_chip_layout(&state, Rect::new(0, 0, 30, 3)).mode, ChromeMode::Compact);
    assert_eq!(window_chip_layout(&state, Rect::new(0, 0, 30, 2)).mode, ChromeMode::Flat);
    let (buf, window) = paint(WindowState::new("tiny"), 30, 3);
    assert!(row(&buf, 0).contains("tiny") && row(&buf, 0).contains('\u{2715}'));
    let close = row(&buf, 0).chars().position(|c| c == '\u{2715}').unwrap() as u16;
    assert_eq!(window.window_hit(Rect::new(0, 0, 30, 3), Pos { x: close, y: 0 }), Some(WidgetSignal::WindowClose(0)), "even a three-row window can be closed");
}

#[test]
fn no_wall_sticks_out_beside_the_chip_above_the_body_corner() {
    let (buf, _) = paint(WindowState::new("Commands"), 40, 8);
    let right = 39;
    assert_eq!(buf.get(right, 1).unwrap().ch, ' ', "nothing beside the chip at the right edge of the cap row");
    assert_eq!(buf.get(right, 2).unwrap().ch, '\u{2510}');
    assert_eq!(buf.get(right, 3).unwrap().ch, '\u{2502}');
}

#[test]
fn the_navbar_centre_never_covers_the_left_or_right_items() {
    let nav = |label: &str| NavItem { id: label.into(), label: label.into(), active: false };
    let theme = Theme::new(AppearanceName::Dark);
    for width in [16u16, 20, 40, 60, 120] {
        let state = NavbarState { left: vec![nav("semio"), nav("dashboard")], center: vec![nav("a centred title")], right: vec![nav("connected"), nav("dark")] };
        let mut buf = CellBuffer::new(Size { width, height: 2 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        ChromeState::Navbar(state).paint(&theme, Rect::new(0, 0, width, 2), &mut buf);
        let line = row(&buf, 0);
        assert_eq!(line.chars().count(), usize::from(width));
        assert!(line.starts_with(" semio "), "{width}: left items keep their place: {line:?}");
        if width >= 40 {
            assert!(line.ends_with(" connected  dark "), "{width}: right items keep their place: {line:?}");
            let left_end = " semio  dashboard ".len();
            let right_start = width as usize - " connected  dark ".len();
            let centre = line.find("a ").filter(|&at| at > left_end - 1 && line[at..].starts_with("a "));
            let at = centre.expect("the centre text starts inside the free band");
            assert!(at > left_end && at + 2 < right_start, "{width}: the centre stays between the bands: {line:?}");
        }
        if width >= 60 {
            assert!(line.contains(" a centred title "), "{width}: the whole centre text fits: {line:?}");
        }
    }
}

#[test]
fn the_footer_status_keeps_a_share_of_the_row_and_cut_hints_end_in_an_ellipsis() {
    let hints: Vec<KeyHint> = ["quit", "zoom", "split", "close", "new", "search", "help"].iter().map(|l| KeyHint { key: l[..1].into(), label: (*l).into() }).collect();
    let theme = Theme::new(AppearanceName::Dark);
    for width in [16u16, 20, 40, 60, 120] {
        let footer = FooterState { hints: hints.clone(), status: "connected 148 commands".into() };
        let mut buf = CellBuffer::new(Size { width, height: 2 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        ChromeState::Footer(footer).paint(&theme, Rect::new(0, 0, width, 2), &mut buf);
        let line = row(&buf, 1);
        assert_eq!(line.chars().count(), usize::from(width));
        assert!(line.starts_with(" q quit"), "{width}: hints come first: {line:?}");
        if width >= 60 {
            assert!(line.ends_with("connected 148 commands"), "{width}: the status survives in full: {line:?}");
        } else {
            assert!(line.contains('\u{2026}'), "{width}: something is elided: {line:?}");
        }
        if (20..120).contains(&width) {
            assert!(line.contains("\u{2026}") || line.contains("  "), "{width}: hints end in an ellipsis when cut: {line:?}");
        }
    }
}

#[test]
fn controls_and_status_are_drawn_from_the_glyph_set_and_keep_their_columns() {
    use crate::tui::layout::WindowStackCorner;
    use crate::tui::theme::{GlyphSet, Status};
    let mut state = WindowState::new("host").with_stack_tab_states(
        vec![
            crate::tui::chrome::WindowStackTabState::new("done", WindowStackCorner::TopLeft).with_status(Some(Status::Success)),
            crate::tui::chrome::WindowStackTabState::new("busy", WindowStackCorner::TopLeft).with_status(Some(Status::Running)),
        ],
        1,
    );
    state.peers = 2;
    let rect = Rect::new(0, 0, 60, 8);
    let layout = window_chip_layout(&state, rect);
    let mut theme = Theme::new(AppearanceName::Dark);
    let columns: Vec<u16> = layout.groups.iter().flat_map(|g| g.tabs.iter()).flat_map(|t| [t.close_x, t.maximize_x]).flatten().collect();
    let chrome = ChromeState::Window(Box::new(state));
    let mut unicode = CellBuffer::new(Size { width: 60, height: 8 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    chrome.paint(&theme, rect, &mut unicode);
    let line = row(&unicode, 1);
    assert!(line.contains('\u{2715}') && line.contains('\u{2922}') && line.contains('\u{2713}') && line.contains('\u{25d0}'), "{line:?}");
    let done = line.chars().position(|c| c == '\u{2713}').unwrap() as u16;
    assert_eq!(unicode.get(done, 1).unwrap().fg, theme.role(Role::Success), "a finished task is drawn in the success role");
    theme.set_glyphs(GlyphSet::Ascii);
    let mut ascii = CellBuffer::new(Size { width: 60, height: 8 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    chrome.paint(&theme, rect, &mut ascii);
    let line = row(&ascii, 1);
    assert!(line.is_ascii() || line.chars().all(|c| c.is_ascii() || "\u{2500}\u{2502}\u{250c}\u{2510}\u{2514}\u{2518}\u{2501}\u{2503}\u{250f}\u{2513}\u{2517}\u{251b}\u{2534}\u{252c}\u{251c}\u{2524}".contains(c)), "no symbol outside the ASCII repertoire and box drawing: {line:?}");
    assert!(line.contains('x') && line.contains('^') && line.contains('+') && !line.contains('\u{2715}'), "{line:?}");
    for x in columns {
        assert!(ascii.get(x, 1).unwrap().ch.is_ascii_graphic(), "control column {x} keeps a one-cell glyph in ASCII");
    }
}
