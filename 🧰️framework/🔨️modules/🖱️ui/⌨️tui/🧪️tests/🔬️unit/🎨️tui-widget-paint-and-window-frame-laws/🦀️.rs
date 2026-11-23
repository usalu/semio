// 🎨️ Tui widget paint and window frame laws: the list, select, tabs, log, input, divider, chip, navbar and footer painting, the terminal widget passthrough and the window frame rectangle and the stack tab chips inside it.

//#region 🔖️Widget
#[test]
fn paint_list_highlights_selected_row_only_when_focused() {
    let theme = Theme::new(AppearanceName::Dark);
    let mut list = ListState::new(vec!["a".to_string(), "b".to_string()]);
    list.selected = 1;
    list.marks = vec![false, true];
    let rect = Rect::new(0, 0, 10, 2);
    let mut buf = CellBuffer::new(Size { width: 10, height: 2 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::List(list).paint(&theme, rect, &mut buf, true);
    assert_eq!(buf.get(0, 1).unwrap().bg, theme.role(Role::ActiveBase));
    assert_ne!(buf.get(0, 0).unwrap().bg, theme.role(Role::ActiveBase));
    assert!(row_text(&buf, 1).starts_with('\u{2713}'), "a marked row shows a check prefix");
    assert!(row_text(&buf, 0).starts_with(' '), "an unmarked row has a blank prefix");
}

#[test]
fn paint_select_renders_label_and_current_option() {
    let theme = Theme::new(AppearanceName::Dark);
    let select = SelectState { label: "Mode".to_string(), options: vec!["A".to_string(), "B".to_string()], index: 1 };
    let rect = Rect::new(0, 0, 20, 1);
    let mut buf = CellBuffer::new(Size { width: 20, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Select(select).paint(&theme, rect, &mut buf, true);
    assert_eq!(row_text(&buf, 0).trim_end(), "Mode: \u{2039} B \u{203a}");
    assert_eq!(buf.get(0, 0).unwrap().fg, theme.role(Role::Accent), "a focused select uses the accent color");
}

#[test]
fn paint_tabs_bolds_the_active_tab() {
    let theme = Theme::new(AppearanceName::Dark);
    let tabs = TabsState::new(vec!["One".to_string(), "Two".to_string()], 1);
    let rect = Rect::new(0, 0, 20, 1);
    let mut buf = CellBuffer::new(Size { width: 20, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Tabs(tabs).paint(&theme, rect, &mut buf, false);
    let t_x = (0..20).find(|&x| buf.get(x, 0).unwrap().ch == 'T').expect("active tab rendered");
    assert_eq!(buf.get(t_x, 0).unwrap().attrs & attr::BOLD, attr::BOLD);
    let o_x = (0..20).find(|&x| buf.get(x, 0).unwrap().ch == 'O').expect("inactive tab rendered");
    assert_eq!(buf.get(o_x, 0).unwrap().attrs & attr::BOLD, 0, "inactive tab is not bold");
}

#[test]
fn terminal_widget_passes_keys_through_and_opens_search_only_by_command() {
    let mut term = TerminalState::new(Size { width: 8, height: 3 }, 50);
    term.feed(b"line1\nline2\nline3\nline4\n");
    let mut widget = WidgetState::Terminal(term);
    let page = KeyEvent { key: Key::PageUp, mods: 0 };
    assert_eq!(widget.on_key(&page), Some(WidgetSignal::TerminalInput(b"\x1b[5~".to_vec())));
    let slash = KeyEvent { key: Key::Char('/'), mods: 0 };
    assert_eq!(widget.on_key(&slash), Some(WidgetSignal::TerminalInput(b"/".to_vec())));
    if let WidgetState::Terminal(t) = &mut widget {
        t.begin_search();
    }
    let a = KeyEvent { key: Key::Char('a'), mods: 0 };
    assert_eq!(widget.on_key(&a), Some(WidgetSignal::ValueChanged("a".into())));
    let esc = KeyEvent { key: Key::Esc, mods: 0 };
    assert_eq!(widget.on_key(&esc), Some(WidgetSignal::ValueChanged(String::new())));
    let x = KeyEvent { key: Key::Char('x'), mods: 0 };
    assert_eq!(widget.on_key(&x), Some(WidgetSignal::TerminalInput(b"x".to_vec())));
    let mut buf = CellBuffer::new(Size { width: 8, height: 3 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    widget.paint(&Theme::new(AppearanceName::Dark), Rect::new(0, 0, 8, 3), &mut buf, true);
}

#[test]
fn window_stack_tabs_paint_on_body_top() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 50, 6);
    let mut buf = CellBuffer::new(Size { width: 50, height: 6 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    let w = WindowState::new("Main").with_stack_tabs(vec!["a".into(), "b".into()], 1);
    ChromeState::Window(Box::new(w)).paint(&theme, rect, &mut buf);
    let row: String = (0..50).filter_map(|x| buf.get(x, 1).map(|c| c.ch)).collect();
    assert!(row.contains('a') && row.contains('b'), "corner stack tabs missing: {row:?}");
    assert!(row.contains('\u{2715}') && !row.contains('\u{2922}'), "close glyph present, no maximize for a lone window: {row:?}");
}

#[test]
fn window_stack_tabs_respect_bottom_corners() {
    use semio_ui_tui_frame::chrome::WindowStackTabState;
    use semio_ui_tui_core::docking::WindowStackCorner;
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 50, 8);
    let mut buf = CellBuffer::new(Size { width: 50, height: 8 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    let w = WindowState::new("Main").with_stack_tab_states(vec![WindowStackTabState::new("top", WindowStackCorner::TopLeft), WindowStackTabState::new("bot", WindowStackCorner::BottomRight)], 1);
    ChromeState::Window(Box::new(w)).paint(&theme, rect, &mut buf);
    let top = row_text(&buf, 1);
    let bottom = row_text(&buf, 6);
    assert!(top.contains("top"), "top tab missing: {top:?}");
    assert!(bottom.contains("bot"), "bottom tab missing: {bottom:?}");
}

#[test]
fn window_frame_is_one_plain_rectangle_with_its_chips_inside() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 40, 8);
    let sentinel = Cell { ch: '.', fg: [1, 2, 3], bg: [4, 5, 6], attrs: 0, width: 1 };
    let mut buf = CellBuffer::new(Size { width: 40, height: 8 }, sentinel);
    let w = WindowState::new("Commands");
    let layout = window_chip_layout(&w, rect);
    ChromeState::Window(Box::new(w.clone())).paint(&theme, rect, &mut buf);
    for y in 0..8 {
        for x in 0..40 {
            assert_ne!(buf.get(x, y).copied(), Some(sentinel), "no notch, gap or cutout at {x},{y}");
        }
    }
    let top = row_text(&buf, 0);
    assert!(top.starts_with('\u{250c}') && top.ends_with('\u{2510}'), "the top edge is the plain frame edge: {top:?}");
    assert_eq!(row_text(&buf, 7).chars().next(), Some('\u{2514}'));
    assert_eq!(row_text(&buf, 7).chars().last(), Some('\u{2518}'));
    assert_eq!(buf.get(38, 0).unwrap().bg, theme.surface(Surface::Window), "the cells beside the title chip stay on the frame surface");
    assert_eq!(buf.get(2, 4).unwrap().bg, theme.surface(Surface::Window));
    for tab in layout.groups.iter().flat_map(|group| &group.tabs) {
        assert!(tab.x + tab.interior_width + 2 <= rect.width, "chips lie within the box");
        assert_eq!(buf.get(tab.x + 1, 1).unwrap().bg, theme.surface(Surface::Pane), "the chip is one elevation above the content");
    }
    assert_eq!(window_content_padding(&w, rect), [2, 1, 1, 1]);
    assert_eq!(window_safe_clearances(&w, rect), [1, 0, 0, 0]);
    assert_eq!(window_content_rect(rect), Rect::new(1, 1, 38, 6));
}

#[test]
fn window_bottom_chip_sits_in_the_last_interior_row_of_the_same_rectangle() {
    use semio_ui_tui_frame::chrome::WindowStackTabState;
    use semio_ui_tui_core::docking::WindowStackCorner;
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 50, 8);
    let sentinel = Cell { ch: '.', fg: [1, 2, 3], bg: [4, 5, 6], attrs: 0, width: 1 };
    let mut buf = CellBuffer::new(Size { width: 50, height: 8 }, sentinel);
    let window = WindowState::new("Main").with_stack_tab_states(vec![WindowStackTabState::new("top", WindowStackCorner::TopLeft), WindowStackTabState::new("bot", WindowStackCorner::BottomRight)], 0);
    assert_eq!(window_safe_clearances(&window, rect), [1, 0, 1, 0]);
    ChromeState::Window(Box::new(window)).paint(&theme, rect, &mut buf);
    assert_eq!(buf.get(0, 7).unwrap().ch, '\u{2514}', "the bottom-left corner closes the plain rectangle");
    assert_eq!(buf.get(0, 6).unwrap().ch, '\u{2502}');
    assert_eq!(buf.get(49, 7).unwrap().ch, '\u{2518}');
    assert!(row_text(&buf, 6).contains("bot"));
    assert!((0..50).all(|x| buf.get(x, 7).unwrap().ch != '.'));
}

#[test]
fn composed_window_paints_its_chips_over_the_content() {
    let mut tui = semio_ui_tui_engine::Tui::new(Size { width: 60, height: 40 }, Theme::new(AppearanceName::Dark));
    let navbar = NavbarState { left: vec![], center: vec![NavItem { id: "d".into(), label: "dashboard".into(), active: false }], right: vec![] };
    let footer = FooterState { hints: vec![], status: String::new() };
    let layout = create_default_layout(&["w1".into()], Axis::Horizontal, None, Some(&["Commands".into()]));
    let mut built = shell(&mut tui.scene, navbar, footer, &layout);
    let (_id, chrome) = built.windows[0].clone();
    let widget = tui.scene.add(chrome, Node::new(NodeContent::Widget(WidgetState::Wizard(WizardState::new(vec!["alpha".into()])))));
    tui.scene.node_mut(widget).set_constraint(Constraint { width: Dimension::Weight(1), height: Dimension::Weight(1), ..Default::default() });
    built.remount(&mut tui.scene, &layout);
    tui.set_focus(Some(widget));
    let _ = tui.render_full();
    let win = tui.scene.rect(chrome);
    let frame = tui.frame();
    assert!(row_text(frame, win.y + 1).contains("Commands"), "the chip is painted above the content");
    assert!(row_text(frame, win.y + 2).contains('/'), "the wizard starts below the chip row, the safe clearance");
    assert_eq!(tui.scene.rect(widget).y, win.y + 2);
    assert_eq!(frame.get(win.x + win.width - 2, win.y).unwrap().bg, tui.theme.surface(Surface::Window), "no floor shows through beside the chip");
    let chip = Pos { x: win.x + 2, y: win.y + 1 };
    let beside = Pos { x: win.x + win.width - 2, y: win.y + 1 };
    assert_eq!(tui.scene.hit(chip), Some(chrome), "chip cells hit the chip");
    assert_eq!(tui.scene.hit(beside), Some(chrome), "uncovered chip-row cells hit the content plane, the window");
    assert_eq!(tui.scene.hit(Pos { x: win.x + 4, y: win.y + 2 }), Some(widget), "below the chip row the content is hit");
}

fn wall_role(group: &semio_ui_tui_frame::chrome::WindowCornerChipGroup, x: u16, active: usize, edges: [u16; 2], body: Role) -> Role {
    let owned = group.tabs.iter().any(|tab| tab.index == active && (tab.x == x || tab.x + tab.interior_width + 1 == x));
    if edges.contains(&x) && body == Role::ActiveBase { Role::ActiveBase } else if owned { Role::ActiveBase } else { Role::BorderNormal }
}

#[test]
fn window_chip_walls_meet_the_outline_in_the_tab_state() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 60, 8);
    for active in [0usize, 1] {
        let mut w = WindowState::new("Main").with_stack_tabs(vec!["alpha".into(), "beta".into()], active);
        w.focused = true;
        let layout = window_chip_layout(&w, rect);
        let mut buf = CellBuffer::new(Size { width: 60, height: 8 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        ChromeState::Window(Box::new(w)).paint(&theme, rect, &mut buf);
        let group = layout.groups.iter().find(|g| g.corner == semio_ui_tui_core::docking::WindowStackCorner::TopLeft).expect("top-left tabs");
        for tab in &group.tabs {
            for x in [tab.x, tab.x + tab.interior_width + 1] {
                let role = wall_role(group, x, active, [0, 59], Role::ActiveBase);
                assert_eq!(buf.get(x, 1).unwrap().fg, theme.role(role), "tab {} wall at {x}", tab.index);
                assert_eq!(buf.get(x, 1).unwrap().ch, '\u{2503}', "a focused window draws every wall heavy");
                assert_eq!(buf.get(x, 0).unwrap().fg, theme.role(Role::ActiveBase), "a focused body outline stays one active stroke over every tab: junction at {x}");
            }
            for x in (tab.x + 1)..(tab.x + tab.interior_width + 1) {
                assert_eq!(buf.get(x, 0).unwrap().ch, '\u{2501}', "the frame edge runs unbroken over tab {}", tab.index);
                assert_eq!(buf.get(x, 0).unwrap().fg, theme.role(Role::ActiveBase));
            }
        }
        let hair = row_text(&buf, 0);
        assert!(hair.chars().all(|ch| ch != ' ') && hair.ends_with('\u{2513}'), "the frame edge never breaks: {hair:?}");
        assert!(hair.contains('\u{2533}'), "inner walls meet the edge in a junction: {hair:?}");
        assert_eq!(buf.get(0, rect.height - 1).unwrap().fg, theme.role(Role::ActiveBase), "focused body border remains independent of tab selection");
    }
    let plain = WindowState::new("Main").with_stack_tabs(vec!["alpha".into(), "beta".into()], 0);
    let mut buf = CellBuffer::new(Size { width: 60, height: 8 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    ChromeState::Window(Box::new(plain)).paint(&theme, rect, &mut buf);
    assert_eq!(buf.get(0, 0).unwrap().fg, theme.role(Role::ActiveBase), "selected tab keeps its active outline in an unfocused window");
    assert_eq!(buf.get(rect.width - 1, 2).unwrap().fg, theme.role(Role::BorderNormal), "unfocused body retains its normal border");
}

#[test]
fn window_bottom_chip_walls_meet_the_bottom_outline() {
    use semio_ui_tui_frame::chrome::WindowStackTabState;
    use semio_ui_tui_core::docking::WindowStackCorner;
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 60, 8);
    let mut w = WindowState::new("Main").with_stack_tab_states(
        vec![WindowStackTabState::new("one", WindowStackCorner::BottomLeft), WindowStackTabState::new("two", WindowStackCorner::BottomLeft)],
        1,
    );
    w.focused = true;
    let layout = window_chip_layout(&w, rect);
    let mut buf = CellBuffer::new(Size { width: 60, height: 8 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    ChromeState::Window(Box::new(w)).paint(&theme, rect, &mut buf);
    let group = layout.groups.iter().find(|g| g.corner == WindowStackCorner::BottomLeft).expect("bottom tabs");
    assert!(group.tabs.len() >= 2, "two bottom tabs");
    let (chip_y, edge_y) = (rect.height - 2, rect.height - 1);
    for tab in &group.tabs {
        for x in [tab.x, tab.x + tab.interior_width + 1] {
            let role = wall_role(group, x, 1, [0, 59], Role::ActiveBase);
            assert_eq!(buf.get(x, chip_y).unwrap().fg, theme.role(role), "bottom tab {} wall at {x}", tab.index);
            assert_eq!(buf.get(x, edge_y).unwrap().fg, theme.role(Role::ActiveBase), "focused bottom outline at {x}");
        }
        for x in (tab.x + 1)..(tab.x + tab.interior_width + 1) {
            assert_eq!(buf.get(x, edge_y).unwrap().ch, '\u{2501}', "bottom edge runs unbroken under tab {}", tab.index);
        }
    }
    assert!(row_text(&buf, edge_y).contains('\u{253b}'), "inner walls meet the bottom edge in a junction");
    assert_eq!(buf.get(0, 0).unwrap().fg, theme.role(Role::ActiveBase), "focused body retains its own border role");
}

#[test]
fn paint_log_shows_the_tail_when_following() {
    let theme = Theme::new(AppearanceName::Dark);
    let mut log = LogState::new(10);
    for i in 0..5 {
        log.push(&format!("line{i}"));
    }
    let rect = Rect::new(0, 0, 10, 3);
    let mut buf = CellBuffer::new(Size { width: 10, height: 3 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Log(log).paint(&theme, rect, &mut buf, false);
    assert!(row_text(&buf, 0).starts_with("line2"), "{:?}", row_text(&buf, 0));
    assert!(row_text(&buf, 1).starts_with("line3"));
    assert!(row_text(&buf, 2).starts_with("line4"));
    assert_eq!(buf.get(9, 2).unwrap().ch, '\u{2503}', "five lines in three rows draw a scroll bar whose thumb sits at the bottom while following");
}

#[test]
fn paint_input_shows_placeholder_and_draws_cursor_when_focused() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 20, 1);
    let empty_input = InputState { value: String::new(), cursor: 0, placeholder: "type here".to_string() };
    let mut buf = CellBuffer::new(Size { width: 20, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Input(empty_input).paint(&theme, rect, &mut buf, false);
    assert_eq!(row_text(&buf, 0).trim_end(), "type here");
    assert_eq!(buf.get(0, 0).unwrap().fg, theme.role(Role::MutedForeground));

    let filled = InputState { value: "hi".to_string(), cursor: 2, placeholder: "ph".to_string() };
    let mut buf2 = CellBuffer::new(Size { width: 20, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Input(filled).paint(&theme, rect, &mut buf2, true);
    assert_eq!(buf2.get(2, 0).unwrap().ch, ' ', "the painter draws no caret glyph; the terminal cursor marks the caret");
    assert_eq!(buf2.get(0, 0).unwrap().ch, 'h', "the caret never hides a glyph");
    let spec = WidgetState::Input(InputState { value: "hi".to_string(), cursor: 2, placeholder: "ph".to_string() }).cursor(rect).expect("a focused input places the terminal cursor");
    assert_eq!(spec.pos, Pos { x: 2, y: 0 });
}

#[test]
fn paint_divider_centers_its_label() {
    let theme = Theme::new(AppearanceName::Dark);
    let divider = DividerState { label: Some("Hi".to_string()) };
    let rect = Rect::new(0, 0, 10, 1);
    let mut buf = CellBuffer::new(Size { width: 10, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Divider(divider).paint(&theme, rect, &mut buf, false);
    let row = row_text(&buf, 0);
    assert!(row.contains("Hi"));
    assert_eq!(row.chars().next().unwrap(), '\u{2500}', "the hairline still draws under the label");
}

#[test]
fn paint_chip_reflects_on_and_off_colors() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 10, 1);
    let on = ChipState { label: "x".to_string(), on: true };
    let mut buf_on = CellBuffer::new(Size { width: 10, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Chip(on).paint(&theme, rect, &mut buf_on, false);
    assert_eq!(buf_on.get(0, 0).unwrap().bg, theme.role(Role::Accent));

    let off = ChipState { label: "x".to_string(), on: false };
    let mut buf_off = CellBuffer::new(Size { width: 10, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Chip(off).paint(&theme, rect, &mut buf_off, false);
    assert_eq!(buf_off.get(0, 0).unwrap().bg, theme.surface(Surface::Panel));
}
//#endregion 🔖️Widget
