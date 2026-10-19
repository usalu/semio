//! 🔬️ Window anatomy, tab strip, compact chrome, ellipsis and the navbar/footer bands (R02 R06 R08 R13 R16 R17 R23).

use semio_framework_ui_tui_core::cell::{Cell, CellBuffer};
use semio_framework_ui_tui_frame::chrome::{window_chip_layout, ChromeMode, ChromeState, FooterState, KeyHint, NavItem, NavbarState, TabKind, WindowState};
use semio_framework_ui_tui_core::geometry::{Pos, Rect, Size};
use semio_framework_ui_tui_core::theme::{Role, Theme};
use semio_framework_ui_tui_core::signal::WidgetSignal;
use ui_styling::appearance::AppearanceName;

#[test]
fn chrome_emphasis_fixture_matches_independent_raster_outline_oracle() {
    use semio_framework_ui_tui_frame::chrome::{WindowHit, WindowStackTabState};
    use semio_framework_ui_tui_core::docking::WindowStackCorner;
    use semio_framework_ui_tui_core::theme::Surface;
    use semio_framework_pack_json::{parse, JsonMemberPolicy};
    use tiny_skia::{Paint, Pixmap, Transform};
    let fixture = parse(include_str!("../🧫️fixtures/🖼️chrome-emphasis/🔣️.json"), JsonMemberPolicy::Reject).unwrap();
    let feature = include_str!("../🖼️chrome-emphasis/🥒️.feature");
    assert!(feature.starts_with("@capability-tui-chrome-emphasis"));
    let size = fixture.get("size").unwrap().as_array().unwrap();
    let (width, height) = (size[0].as_u64().unwrap() as u16, size[1].as_u64().unwrap() as u16);
    let theme = Theme::new(AppearanceName::Dark);
    for corner in [WindowStackCorner::TopLeft, WindowStackCorner::TopRight, WindowStackCorner::BottomLeft, WindowStackCorner::BottomRight] {
        for focused in [false, true] {
            for case in fixture.get("cases").unwrap().as_array().unwrap() {
                let active = case.get("active").unwrap().as_u64().unwrap() as usize;
                let labels = fixture.get("labels").unwrap().as_array().unwrap();
                let mut state = WindowState::new("host").with_stack_tab_states(labels.iter().map(|label| WindowStackTabState::new(label.as_str().unwrap(), corner)).collect(), active);
                state.focused = focused;
                state.hover = case.get("hover").unwrap().as_u64().map(|index| WindowHit::Tab(index as usize));
                let rect = Rect::new(0, 0, width, height);
                let layout = window_chip_layout(&state, rect);
                let mut buf = CellBuffer::new(Size { width, height }, Cell::blank([0, 0, 0], theme.surface(Surface::Base)));
                ChromeState::Window(Box::new(state)).paint(&theme, rect, &mut buf);
                let mut oracle = Pixmap::new(width as u32, height as u32).unwrap();
                for tab in layout.groups.iter().flat_map(|group| &group.tabs).filter(|tab| tab.kind == TabKind::Tab) {
                    let role = match case.get("roles").unwrap().as_array().unwrap()[tab.index].as_str().unwrap() {
                        "active" => Role::ActiveBase,
                        "emphasized" => Role::BorderEmphasized,
                        "normal" => Role::BorderNormal,
                        _ => unreachable!(),
                    };
                    let rgb = theme.role(role);
                    let mut ink = Paint::default();
                    ink.set_color_rgba8(rgb[0], rgb[1], rgb[2], 255);
                    ink.anti_alias = false;
                    let outer_y = if corner.is_top() { 0 } else { height - 1 };
                    let text_y = if corner.is_top() { 1 } else { height - 2 };
                    let seam_y = if corner.is_top() { 2 } else { height - 3 };
                    for (x, y, w, h) in [(tab.x, outer_y, tab.interior_width + 2, 1), (tab.x, text_y, 1, 1), (tab.x + tab.interior_width + 1, text_y, 1, 1)] {
                        oracle.fill_rect(tiny_skia::Rect::from_xywh(x as f32, y as f32, w as f32, h as f32).unwrap(), &ink, Transform::identity(), None);
                    }
                    let body_rgb = theme.role(if focused { Role::ActiveBase } else { Role::BorderNormal });
                    let seam_rgb = if tab.index == active { rgb } else { body_rgb };
                    ink.set_color_rgba8(seam_rgb[0], seam_rgb[1], seam_rgb[2], 255);
                    for x in [tab.x, tab.x + tab.interior_width + 1] {
                        oracle.fill_rect(tiny_skia::Rect::from_xywh(x as f32, seam_y as f32, 1.0, 1.0).unwrap(), &ink, Transform::identity(), None);
                    }
                    for x in tab.x + 1..tab.x + tab.interior_width + 1 {
                        let hovered = case.get("hover").unwrap().as_u64() == Some(tab.index as u64);
                        let bg = if hovered && tab.index != active { theme.role(Role::HoverInteractive) } else { theme.surface(Surface::Window) };
                        let fg = if hovered || tab.index == active { Role::Foreground } else { Role::MutedForeground };
                        assert_eq!(buf.get(x, text_y).unwrap().bg, bg);
                        assert_eq!(buf.get(x, text_y).unwrap().fg, theme.role(fg));
                        assert_eq!(buf.get(x, seam_y).unwrap().ch, if tab.index == active { ' ' } else if focused { '━' } else { '─' });
                        if tab.index != active { assert_eq!(buf.get(x, seam_y).unwrap().fg, body_rgb); }
                    }
                }
                for (index, pixel) in oracle.pixels().iter().enumerate().filter(|(_, pixel)| pixel.alpha() > 0) {
                    assert_eq!(buf.get((index % width as usize) as u16, (index / width as usize) as u16).unwrap().fg, [pixel.red(), pixel.green(), pixel.blue()], "corner {corner:?}, focused {focused}, case {case:?}, pixel {index}");
                }
            }
        }
    }
}

#[test]
fn chrome_ancestor_emphasis_follows_descendants_and_clears_stale_lineages() {
    use semio_framework_ui_tui_frame::chrome::shell;
    use semio_framework_ui_tui_engine::Tui;
    use semio_framework_ui_tui_core::event::{Event, MouseEvent, MouseKind};
    use semio_framework_ui_tui_frame::layout::{create_default_layout, Constraint, Dimension};
    use semio_framework_ui_tui_frame::scene::{Node, NodeContent};
    use semio_framework_ui_tui_widgets::widget::{TabsState, WidgetState};
    let theme = Theme::new(AppearanceName::Dark);
    let mut tui = Tui::new(Size { width: 80, height: 24 }, theme);
    let layout = create_default_layout(&["window".into()], "row", None, None);
    let mut built = shell(&mut tui.scene, NavbarState { left: vec![], center: vec![], right: vec![] }, FooterState { hints: vec![], status: String::new() }, &layout);
    let window = built.windows[0].1;
    let mut children = Vec::new();
    for parent in [window, window, built.navbar, built.footer] {
        let child = tui.scene.add(parent, Node::new(NodeContent::Widget(WidgetState::Tabs(TabsState::new(vec!["a".into(), "b".into()], 0)))));
        tui.scene.node_mut(child).set_constraint(Constraint { width: Dimension::Cells(8), height: Dimension::Cells(1), ..Default::default() });
        children.push(child);
    }
    built.remount(&mut tui.scene, &layout);
    tui.render_full();
    let move_to = |pos| Event::Mouse(MouseEvent { kind: MouseKind::Move, pos, mods: 0, clicks: 1 });
    let border = |tui: &Tui, parent| {
        let rect = tui.scene.rect(parent);
        if parent == window { Pos { x: rect.x, y: rect.y + rect.height - 2 } }
        else if parent == built.navbar { Pos { x: rect.x + rect.width - 1, y: rect.y + rect.height - 1 } }
        else { Pos { x: rect.x + rect.width - 1, y: rect.y } }
    };
    tui.capture(Some(children[3]));
    let child_rect = tui.scene.rect(children[0]);
    tui.dispatch(&move_to(Pos { x: child_rect.x + 1, y: child_rect.y }));
    assert_eq!(tui.hovered(), Some(children[0]));
    tui.capture(None);
    for (index, parent) in [window, window, built.navbar, built.footer].into_iter().enumerate() {
        let child_rect = tui.scene.rect(children[index]);
        tui.dispatch(&move_to(Pos { x: child_rect.x + 1, y: child_rect.y }));
        assert_eq!(tui.hovered(), Some(children[index]));
        tui.render();
        for chrome in [window, built.navbar, built.footer] {
            let at = border(&tui, chrome);
            let role = if chrome == parent { Role::BorderEmphasized } else { Role::BorderNormal };
            assert_eq!(tui.frame().get(at.x, at.y).unwrap().fg, tui.theme.role(role));
        }
        assert!(tui.render().0.is_empty());
    }
    let rect = tui.scene.rect(window);
    tui.dispatch(&move_to(Pos { x: rect.x + rect.width / 2, y: rect.y + rect.height / 2 }));
    tui.render();
    let at = border(&tui, window);
    assert_eq!(tui.frame().get(at.x, at.y).unwrap().fg, tui.theme.role(Role::BorderEmphasized));
    tui.dispatch(&Event::FocusLost);
    tui.render();
    assert_eq!(tui.frame().get(at.x, at.y).unwrap().fg, tui.theme.role(Role::BorderNormal));
    tui.dispatch(&Event::FocusGained);
    let child_rect = tui.scene.rect(children[0]);
    tui.dispatch(&move_to(Pos { x: child_rect.x + 1, y: child_rect.y }));
    tui.render();
    tui.scene.reparent(children[0], built.navbar);
    tui.render();
    assert_eq!(tui.frame().get(at.x, at.y).unwrap().fg, tui.theme.role(Role::BorderEmphasized));
    assert_ne!(tui.hovered(), Some(children[0]));
    let child_rect = tui.scene.rect(children[1]);
    tui.dispatch(&move_to(Pos { x: child_rect.x + 1, y: child_rect.y }));
    tui.render();
    tui.scene.remove(children[1]);
    tui.render();
    assert_eq!(tui.frame().get(at.x, at.y).unwrap().fg, tui.theme.role(Role::BorderEmphasized));
    assert_ne!(tui.hovered(), Some(children[1]));
    tui.dispatch(&move_to(Pos { x: rect.x + rect.width / 2, y: rect.y + rect.height / 2 }));
    tui.render();
    let overlay = tui.open_menu(Pos { x: rect.x + 4, y: rect.y + 4 }, vec![]);
    tui.render();
    assert_eq!(tui.frame().get(at.x, at.y).unwrap().fg, tui.theme.role(Role::BorderNormal));
    tui.close_overlay(overlay);
    tui.render();
    assert_eq!(tui.frame().get(at.x, at.y).unwrap().fg, tui.theme.role(Role::BorderNormal));
    tui.dispatch(&move_to(Pos { x: rect.x + rect.width / 2, y: rect.y + rect.height / 2 }));
    tui.render();
    tui.scene.node_mut(window).set_visible(false);
    tui.render();
    assert_ne!(tui.hovered(), Some(window));
}

#[test]
fn chrome_compact_active_tab_owns_both_shared_border_cells() {
    for active in [0, 1, 2] {
        let mut state = WindowState::new("host").with_stack_tabs(vec!["first".into(), "second".into(), "third".into()], active);
        state.compact = true;
        let rect = Rect::new(0, 0, 64, 8);
        let layout = window_chip_layout(&state, rect);
        let (buf, _) = paint(state, 64, 8);
        let theme = Theme::new(AppearanceName::Dark);
        let tab = layout.groups[0].tabs.iter().find(|tab| tab.index == active).unwrap();
        for x in [tab.x, tab.x + tab.interior_width + 1] {
            assert_eq!(buf.get(x, 0).unwrap().fg, theme.role(Role::ActiveBase));
        }
        let inactive = layout.groups[0].tabs.iter().find(|tab| tab.index != active).unwrap();
        let inactive_only = if inactive.x == tab.x + tab.interior_width + 1 { inactive.x + inactive.interior_width + 1 } else { inactive.x };
        assert_eq!(buf.get(inactive_only, 0).unwrap().fg, theme.role(Role::BorderNormal));
        assert_eq!(buf.get(tab.x + 2, 0).unwrap().bg, theme.surface(crate::tui::theme::Surface::Window));
    }
}

#[test]
fn chrome_compact_hovered_inactive_tab_owns_non_active_shared_border_cells() {
    use semio_framework_ui_tui_frame::chrome::WindowHit;
    let theme = Theme::new(AppearanceName::Dark);
    for active in [0, 1, 2] {
        for hovered in [0, 1, 2] {
            if hovered == active { continue; }
            let mut state = WindowState::new("host").with_stack_tabs(vec!["first".into(), "second".into(), "third".into()], active);
            state.compact = true;
            state.hover = Some(WindowHit::Tab(hovered));
            let rect = Rect::new(0, 0, 64, 8);
            let layout = window_chip_layout(&state, rect);
            let (buf, _) = paint(state, 64, 8);
            let tabs = &layout.groups[0].tabs;
            let active_tab = tabs.iter().find(|tab| tab.index == active).unwrap();
            let hovered_tab = tabs.iter().find(|tab| tab.index == hovered).unwrap();
            let active_ends = [active_tab.x, active_tab.x + active_tab.interior_width + 1];
            for x in [hovered_tab.x, hovered_tab.x + hovered_tab.interior_width + 1] {
                let role = if active_ends.contains(&x) { Role::ActiveBase } else { Role::BorderEmphasized };
                assert_eq!(buf.get(x, 0).unwrap().fg, theme.role(role), "active {active}, hovered {hovered}, column {x}");
            }
        }
    }
}

#[test]
fn chrome_narrow_overflow_chips_keep_independent_border_roles() {
    use semio_framework_ui_tui_frame::chrome::WindowHit;
    for width in [4, 18, 30, 44] {
        for active in [0, 5, 11] {
            let mut state = many(12);
            state.focused = true;
            state.active_stack_tab = active;
            let rect = Rect::new(0, 0, width, 12);
            let layout = window_chip_layout(&state, rect);
            let chips: Vec<_> = layout.groups.iter().flat_map(|group| &group.tabs).collect();
            let hovered = chips.iter().find(|tab| matches!(tab.kind, TabKind::OverflowPrev(_) | TabKind::OverflowNext(_))).map(|tab| tab.x);
            state.hover = chips.iter().find(|tab| Some(tab.x) == hovered).and_then(|tab| match tab.kind {
                TabKind::OverflowPrev(index) => Some(WindowHit::OverflowPrev(index)),
                TabKind::OverflowNext(index) => Some(WindowHit::OverflowNext(index)),
                _ => None,
            });
            let (buf, _) = paint(state, width, 12);
            let theme = Theme::new(AppearanceName::Dark);
            if layout.mode == ChromeMode::Flat {
                assert_eq!(width, 4);
                assert!(chips.is_empty());
                assert_eq!(row(&buf, 0), "┏━━┓");
                for x in 0..width {
                    assert_eq!(buf.get(x, 0).unwrap().fg, theme.role(Role::ActiveBase));
                }
                continue;
            }
            assert_eq!(layout.mode, ChromeMode::Full);
            assert!(!chips.is_empty());
            let overflow_width: u16 = chips.iter().filter(|tab| matches!(tab.kind, TabKind::OverflowPrev(_) | TabKind::OverflowNext(_))).map(|tab| tab.interior_width + 2).sum();
            let tab_fits = width.saturating_sub(overflow_width) >= 7;
            assert_eq!(chips.iter().any(|tab| tab.kind == TabKind::Tab && tab.index == active), tab_fits, "active {active}, width {width}, overflow {overflow_width}");
            if !tab_fits {
                assert!(chips.iter().all(|tab| matches!(tab.kind, TabKind::OverflowPrev(_) | TabKind::OverflowNext(_))));
            }
            for tab in chips {
                let role = if tab.kind == TabKind::Tab && tab.index == active { Role::ActiveBase }
                    else if Some(tab.x) == hovered { Role::BorderEmphasized }
                    else { Role::BorderNormal };
                assert_eq!(buf.get(tab.x, 0).unwrap().fg, theme.role(role));
                assert_eq!(buf.get(tab.x + tab.interior_width + 1, 0).unwrap().fg, theme.role(role));
            }
        }
    }
}

#[test]
fn chrome_close_and_maximize_hover_keep_active_perimeter_and_emphasize_inactive_tabs() {
    use semio_framework_ui_tui_frame::chrome::WindowHit;
    for focused in [false, true] {
        for active in [0, 1] {
            let mut state = WindowState::new("host").with_stack_tabs(vec!["first".into(), "second".into()], active);
            state.focused = focused;
            state.peers = 2;
            let rect = Rect::new(0, 0, 64, 12);
            let layout = window_chip_layout(&state, rect);
            let tabs: Vec<_> = layout.groups.iter().flat_map(|group| &group.tabs).filter(|tab| tab.kind == TabKind::Tab).collect();
            let mut chrome = ChromeState::Window(Box::new(state));
            let theme = Theme::new(AppearanceName::Dark);
            for tab in &tabs {
                for (x, hit) in [(tab.close_x.unwrap(), WindowHit::Close(tab.index)), (tab.maximize_x.unwrap(), WindowHit::Maximize(tab.index))] {
                    let pos = Pos { x, y: 1 };
                    assert_eq!(chrome.window_target(rect, pos), Some(hit));
                    chrome.set_hover(rect, Some(pos));
                    let mut buf = CellBuffer::new(Size { width: 64, height: 12 }, Cell::blank([0, 0, 0], [0, 0, 0]));
                    chrome.paint_emphasized(&theme, rect, &mut buf, true);
                    for candidate in &tabs {
                        let role = if candidate.index == active { Role::ActiveBase } else { Role::BorderEmphasized };
                        for (x, y) in [(candidate.x, 0), (candidate.x, 1), (candidate.x, 2), (candidate.x + candidate.interior_width + 1, 2)] {
                            assert_eq!(buf.get(x, y).unwrap().fg, theme.role(role));
                        }
                    }
                    assert_eq!(buf.get(0, 11).unwrap().fg, theme.role(Role::BorderEmphasized));
                    assert_eq!(buf.get(0, 11).unwrap().ch, if focused { '┗' } else { '└' });
                }
            }
        }
    }
}

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
            let tab = layout.groups.iter().flat_map(|group| &group.tabs).find(|tab| tab.maximize_x == Some(x)).unwrap();
            assert_eq!(target, Some(WidgetSignal::WindowMaximize(tab.index)));
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
    assert_eq!(semio_framework_ui_tui_frame::chrome::window_content_padding(&state, rect), [1, 1, 1, 1]);
    let (buf, window) = paint(state, 50, 6);
    let top = row(&buf, 0);
    assert!(top.starts_with('\u{250c}') && top.ends_with('\u{2510}'), "{top:?}");
    assert!(top.contains("dev") && top.contains("tasks") && top.contains('\u{2922}'), "{top:?}");
    assert_eq!(row(&buf, 1).chars().next(), Some('\u{2502}'), "side walls start right under the border");
    assert_eq!(row(&buf, 5).chars().next(), Some('\u{2514}'), "no bottom chrome");
    let active = top.find("tasks").unwrap();
    let theme = Theme::new(AppearanceName::Dark);
    assert_eq!(buf.get(top[..active].chars().count() as u16, 0).unwrap().bg, theme.surface(crate::tui::theme::Surface::Window));
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
    use semio_framework_ui_tui_core::docking::WindowStackCorner;
    use semio_framework_ui_tui_core::theme::{GlyphSet, Status};
    let mut state = WindowState::new("host").with_stack_tab_states(
        vec![
            semio_framework_ui_tui_frame::chrome::WindowStackTabState::new("done", WindowStackCorner::TopLeft).with_status(Some(Status::Success)),
            semio_framework_ui_tui_frame::chrome::WindowStackTabState::new("busy", WindowStackCorner::TopLeft).with_status(Some(Status::Running)),
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
