// ⌨️ Tui widget key handling laws: the list, select, tabs, input, log and table key handling and the widget preferred sizes.

//#region 🔖️Widget
#[test]
fn list_on_key_boundaries_toggle_and_activate() {
    let mut widget = WidgetState::List(ListState::new(vec!["a".to_string(), "b".to_string()]));
    let up = KeyEvent { key: Key::Up, mods: 0 };
    assert_eq!(widget.on_key(&up), None, "already at the top");
    let down = KeyEvent { key: Key::Down, mods: 0 };
    assert_eq!(widget.on_key(&down), Some(WidgetSignal::SelectionChanged(1)));
    assert_eq!(widget.on_key(&down), None, "already at the bottom");
    let space = KeyEvent { key: Key::Char(' '), mods: 0 };
    assert_eq!(widget.on_key(&space), Some(WidgetSignal::Toggled(true)));
    assert_eq!(widget.on_key(&space), Some(WidgetSignal::Toggled(false)));
    let enter = KeyEvent { key: Key::Enter, mods: 0 };
    assert_eq!(widget.on_key(&enter), Some(WidgetSignal::Activated(1)));

    let mut empty_widget = WidgetState::List(ListState::new(vec![]));
    assert_eq!(empty_widget.on_key(&space), None, "toggling with no items is a no-op");
}

#[test]
fn select_on_key_wraps_and_ignores_empty_options() {
    let mut widget = WidgetState::Select(SelectState { label: "L".to_string(), options: vec!["x".to_string(), "y".to_string()], index: 0 });
    let left = KeyEvent { key: Key::Left, mods: 0 };
    assert_eq!(widget.on_key(&left), Some(WidgetSignal::SelectionChanged(1)), "Left from index 0 wraps to the last option");
    let right = KeyEvent { key: Key::Right, mods: 0 };
    assert_eq!(widget.on_key(&right), Some(WidgetSignal::SelectionChanged(0)));

    let mut empty_widget = WidgetState::Select(SelectState { label: "L".to_string(), options: vec![], index: 0 });
    assert_eq!(empty_widget.on_key(&right), None);
}

#[test]
fn tabs_on_key_wraps_and_ignores_empty_tabs() {
    let mut widget = WidgetState::Tabs(TabsState::new(vec!["one".to_string(), "two".to_string()], 0));
    let left = KeyEvent { key: Key::Left, mods: 0 };
    assert_eq!(widget.on_key(&left), Some(WidgetSignal::TabChanged(1)), "Left from index 0 wraps to the last tab");
    let right = KeyEvent { key: Key::Right, mods: 0 };
    assert_eq!(widget.on_key(&right), Some(WidgetSignal::TabChanged(0)));

    let mut empty_widget = WidgetState::Tabs(TabsState::new(vec![], 0));
    assert_eq!(empty_widget.on_key(&right), None);
}

#[test]
fn input_on_key_inserts_utf8_and_respects_cursor_bounds() {
    let mut widget = WidgetState::Input(InputState { value: String::new(), cursor: 0, placeholder: "ph".to_string() });
    let type_u = KeyEvent { key: Key::Char('ü'), mods: 0 };
    assert_eq!(widget.on_key(&type_u), Some(WidgetSignal::ValueChanged("ü".to_string())));
    let cursor_after_insert = match &widget {
        WidgetState::Input(i) => i.cursor,
        _ => unreachable!(),
    };
    assert_eq!(cursor_after_insert, 'ü'.len_utf8());

    let left = KeyEvent { key: Key::Left, mods: 0 };
    assert_eq!(widget.on_key(&left), None);
    let cursor_after_left = match &widget {
        WidgetState::Input(i) => i.cursor,
        _ => unreachable!(),
    };
    assert_eq!(cursor_after_left, 0, "Left steps over the whole multi-byte character");
    assert_eq!(widget.on_key(&left), None);
    let cursor_after_second_left = match &widget {
        WidgetState::Input(i) => i.cursor,
        _ => unreachable!(),
    };
    assert_eq!(cursor_after_second_left, 0);
    assert_eq!(widget.on_key(&left), None, "already at cursor 0");

    let backspace = KeyEvent { key: Key::Backspace, mods: 0 };
    assert_eq!(widget.on_key(&backspace), None, "nothing before the cursor to delete");

    let right = KeyEvent { key: Key::Right, mods: 0 };
    assert_eq!(widget.on_key(&right), None);
    assert_eq!(widget.on_key(&right), None, "already at the end");

    assert_eq!(widget.on_key(&backspace), Some(WidgetSignal::ValueChanged(String::new())), "backspace at the end removes the multi-byte char");
}

#[test]
fn log_state_push_evicts_oldest_beyond_capacity() {
    let mut log = LogState::new(3);
    log.push("a");
    log.push("b");
    log.push("c");
    log.push("d");
    assert_eq!(log.lines().len(), 3);
    assert_eq!(log.lines().front(), Some(&"b".to_string()), "the oldest line is evicted once capacity is exceeded");
    log.clear();
    assert!(log.lines().is_empty());
    assert_eq!(log.scroll, LogScroll::Follow, "clearing resets scroll to Follow");
}

#[test]
fn log_on_key_page_and_home_end_scroll_states() {
    let mut log = LogState::new(20);
    for i in 0..20 {
        log.push(&format!("l{i}"));
    }
    let mut widget = WidgetState::Log(log);
    let theme = Theme::new(AppearanceName::Dark);
    let mut buf = CellBuffer::new(Size { width: 10, height: 5 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    widget.paint(&theme, Rect::new(0, 0, 10, 5), &mut buf, false);

    let page_down = KeyEvent { key: Key::PageDown, mods: 0 };
    widget.on_key(&page_down);
    assert_eq!(log_scroll(&widget), LogScroll::Follow, "PageDown while already following is a no-op");

    let home = KeyEvent { key: Key::Home, mods: 0 };
    widget.on_key(&home);
    assert_eq!(log_scroll(&widget), LogScroll::At(0), "Home shows the first page, not a single line");

    widget.on_key(&page_down);
    assert_eq!(log_scroll(&widget), LogScroll::At(5), "PageDown scrolls forward by the painted page");

    widget.on_key(&page_down);
    widget.on_key(&page_down);
    assert_eq!(log_scroll(&widget), LogScroll::Follow, "PageDown onto the last page resumes following");

    let page_up = KeyEvent { key: Key::PageUp, mods: 0 };
    widget.on_key(&page_up);
    assert_eq!(log_scroll(&widget), LogScroll::At(10), "the first PageUp from Follow moves a whole page up from the tail view");

    widget.on_key(&page_up);
    assert_eq!(log_scroll(&widget), LogScroll::At(5), "a second PageUp scrolls back one more page");

    let end = KeyEvent { key: Key::End, mods: 0 };
    widget.on_key(&end);
    assert_eq!(log_scroll(&widget), LogScroll::Follow);
}

fn log_scroll(widget: &WidgetState) -> LogScroll {
    match widget {
        WidgetState::Log(l) => l.scroll,
        _ => unreachable!(),
    }
}

#[test]
fn table_on_key_left_right_expand_collapse_and_boundaries() {
    let mut table = sample_table();
    table.selected = 0;
    let mut widget = WidgetState::Table(table);
    let up = KeyEvent { key: Key::Up, mods: 0 };
    assert_eq!(widget.on_key(&up), None, "already at the first visible row");

    let left = KeyEvent { key: Key::Left, mods: 0 };
    assert_eq!(widget.on_key(&left), Some(WidgetSignal::SelectionChanged(0)), "Left on an expanded parent collapses it");
    assert_eq!(widget.on_key(&left), None, "Left on an already-collapsed parent is a no-op");

    let right = KeyEvent { key: Key::Right, mods: 0 };
    assert_eq!(widget.on_key(&right), Some(WidgetSignal::SelectionChanged(0)));
    assert_eq!(widget.on_key(&right), None, "Right on an already-expanded parent is a no-op");

    let WidgetState::Table(t) = &mut widget else { unreachable!() };
    t.selected = 1;
    assert_eq!(widget.on_key(&left), None, "Left on a leaf row is a no-op");
    assert_eq!(widget.on_key(&right), None, "Right on a leaf row is a no-op");

    let mut empty_widget = WidgetState::Table(TableState::new(vec![], vec![]));
    assert_eq!(empty_widget.on_key(&up), None, "no rows at all means every key is a no-op");
}

#[test]
fn widget_preferred_size_for_label_select_chip_and_divider() {
    let label = WidgetState::Label(LabelState { text: "hello".to_string(), align: Align::Left, role: Role::Foreground });
    assert_eq!(label.preferred_size(), Size { width: 5, height: 1 });

    let select = WidgetState::Select(SelectState { label: "L".to_string(), options: vec!["opt".to_string()], index: 0 });
    let expected = display_width("L \u{2039} opt \u{203a}");
    assert_eq!(select.preferred_size(), Size { width: expected, height: 1 });

    let chip = WidgetState::Chip(ChipState { label: "tag".to_string(), on: true });
    assert_eq!(chip.preferred_size(), Size { width: 5, height: 1 });

    let divider = WidgetState::Divider(DividerState::default());
    assert_eq!(divider.preferred_size(), Size { width: 1, height: 1 });

    let list = WidgetState::List(ListState::new(vec!["x".to_string()]));
    assert_eq!(list.preferred_size(), Size { width: 0, height: 0 }, "list has no measured preferred size");
}
//#endregion 🔖️Widget
