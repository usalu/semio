// ✍️ Text field editing vector laws: every row of the shared `🧫️text-field-editing` corpus pressed on a focused retained field.

//#region 🔖️TextFieldEditingVectorTests
/// 🧰️ A focused field of `kind` holding `text` with `anchor`..`caret` selected, and the router that drives it.
fn editing_field(kind: &str, text: &str, anchor: usize, caret: usize) -> (UiTree, NodeId, NodeId, EventRouter) {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 100.0));
    let field = leaf(&mut tree, Some(root), 1, draft_field(kind, text), (0.0, 0.0, 120.0, 24.0));
    let mut router = EventRouter::new("main");
    router.focus.set_focus(&mut tree, Some(field), true);
    let edit = tree.node_mut(field).and_then(|node| node.state.edit.as_mut()).expect("a focused field opens its edit buffer");
    edit.text = text.to_string();
    edit.anchor = anchor;
    edit.caret = caret;
    (tree, root, field, router)
}

/// 📓️ A field's `(text, caret, anchor)`.
fn edit_snapshot(tree: &UiTree, field: NodeId) -> (String, u64, u64) {
    let edit = tree.node(field).and_then(|node| node.state.edit.as_ref()).expect("an open edit buffer");
    (edit.text.clone(), edit.caret as u64, edit.anchor as u64)
}

/// ⌨️ Every `keys` row: the pressed key leaves the field's text, caret, anchor and clipboard exactly as the corpus says.
#[test]
fn every_text_field_editing_key_row_leaves_the_state_the_shared_corpus_declares() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧬️contract/🧪️tests/🧫️fixtures/🧫️text-field-editing/🔣️.json")).expect("text-field-editing fixture");
    let rows = fixture["keys"].as_array().expect("key rows");
    assert!(rows.len() >= 30, "the corpus declares the editing keys");
    for row in rows {
        let name = row["case"].as_str().expect("case");
        let (mut tree, root, field, mut router) = editing_field(row["kind"].as_str().expect("kind"), row["text"].as_str().expect("text"), row["anchor"].as_u64().expect("anchor") as usize, row["caret"].as_u64().expect("caret") as usize);
        let modifiers = EventModifiers { shift: row["shift"].as_bool().expect("shift"), ctrl: row["ctrl"].as_bool().expect("ctrl"), alt: row["alt"].as_bool().expect("alt"), meta: row["meta"].as_bool().expect("meta") };
        let commands = router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: row["key"].as_str().expect("key").into(), modifiers });
        let expected = &row["expected"];
        assert_eq!(edit_snapshot(&tree, field), (expected["text"].as_str().expect("text").to_string(), expected["caret"].as_u64().expect("caret"), expected["anchor"].as_u64().expect("anchor")), "{name}");
        let copied = commands.iter().find_map(|command| match command {
            UiCommand::ClipboardCopy { text, .. } => Some(("copy", text.as_str())),
            UiCommand::ClipboardCut { text, .. } => Some(("cut", text.as_str())),
            _ => None,
        });
        let wanted = expected["clipboard"].as_object().and_then(|clipboard| clipboard.iter().next().map(|(kind, text)| (kind.as_str(), text.as_str().expect("clipboard text"))));
        assert_eq!(copied, wanted, "{name}: the clipboard");
    }
}

/// 📋️ Every `pastes` row: pasted text replaces the selection, a single-line field turns its line breaks into blanks.
#[test]
fn every_text_field_paste_row_leaves_the_state_the_shared_corpus_declares() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧬️contract/🧪️tests/🧫️fixtures/🧫️text-field-editing/🔣️.json")).expect("text-field-editing fixture");
    for row in fixture["pastes"].as_array().expect("paste rows") {
        let name = row["case"].as_str().expect("case");
        let (mut tree, root, field, mut router) = editing_field(row["kind"].as_str().expect("kind"), row["text"].as_str().expect("text"), row["anchor"].as_u64().expect("anchor") as usize, row["caret"].as_u64().expect("caret") as usize);
        router.dispatch(&mut tree, root, &UiEvent::Paste { text: row["pasted"].as_str().expect("pasted").into() });
        let (text, caret, anchor) = edit_snapshot(&tree, field);
        assert_eq!((text.as_str(), caret, anchor), (row["expected"]["text"].as_str().expect("text"), row["expected"]["caret"].as_u64().expect("caret"), row["expected"]["caret"].as_u64().expect("caret")), "{name}");
    }
}

/// 🈶️ An IME update parks the preedit's own cursor on the node, every other composition step clears it.
#[test]
fn the_input_methods_preedit_cursor_is_kept_while_composing_and_dropped_after() {
    let (mut tree, root, field, mut router) = editing_field("text", "ab", 1, 1);
    router.dispatch(&mut tree, root, &UiEvent::Ime(ImeEvent::Update { text: "ねこ".into(), cursor: 3 }));
    let state = &tree.node(field).expect("field").state;
    assert_eq!((state.edit.as_ref().and_then(|edit| edit.composition.as_deref()), state.composition_cursor), (Some("ねこ"), Some(3)));
    router.dispatch(&mut tree, root, &UiEvent::Ime(ImeEvent::Commit { text: "ね".into() }));
    let state = &tree.node(field).expect("field").state;
    assert_eq!((state.edit.as_ref().map(|edit| edit.text.as_str()), state.composition_cursor), (Some("aねb"), None));
}
/// 📐️ A hand-laid layout: lines of `[(byte, pen)]` stops at `line_height` 10 in a 100 x 24 box, as the paint walk would park them.
fn laid_text(text: &str, multi: bool, lines: &[&[(usize, f32)]]) -> semio_ui_retained_tree::tree::EditTextLayout {
    let stops = lines.iter().enumerate().flat_map(|(line, stops)| stops.iter().map(move |(byte, x)| semio_ui_retained_tree::tree::EditTextStop { byte: *byte, line, x: *x })).collect();
    semio_ui_retained_tree::tree::EditTextLayout { text: text.into(), inner: [0.0, 0.0, 100.0, 24.0], line_height: 10.0, multi, lines: lines.len(), stops, revealed_caret: None }
}

fn wrapped_abcd() -> semio_ui_retained_tree::tree::EditTextLayout {
    laid_text("abcd", true, &[&[(0, 0.0), (1, 5.0)], &[(2, 0.0), (3, 5.0), (4, 10.0)]])
}

fn pointer_down(x: f32, y: f32) -> UiEvent {
    UiEvent::PointerDown { x, y, button: PointerButton::Primary, modifiers: Default::default() }
}

fn pointer_up(x: f32, y: f32) -> UiEvent {
    UiEvent::PointerUp { x, y, button: PointerButton::Primary, modifiers: Default::default() }
}

/// ↕️ Up and Down move by VISUAL line through the parked layout, keeping the pen while the caret crosses a short line.
#[test]
fn vertical_motion_follows_the_visual_lines_and_keeps_the_pen() {
    let (mut tree, root, field, mut router) = editing_field("longText", "abcd", 1, 1);
    tree.node_mut(field).expect("field").state.edit_layout = Some(wrapped_abcd());
    router.dispatch(&mut tree, root, &key("ArrowDown"));
    assert_eq!(edit_snapshot(&tree, field), ("abcd".into(), 3, 3), "pen 5 on line 0 lands on pen 5 of line 1");
    assert_eq!(tree.node(field).expect("field").state.caret_goal_x, Some(5.0));
    router.dispatch(&mut tree, root, &key("ArrowDown"));
    assert_eq!(edit_snapshot(&tree, field).1, 3, "no line below");
    router.dispatch(&mut tree, root, &key("ArrowUp"));
    assert_eq!(edit_snapshot(&tree, field).1, 1);
    router.dispatch(&mut tree, root, &key("ArrowRight"));
    assert_eq!(tree.node(field).expect("field").state.caret_goal_x, None, "any other key forgets the pen");
}

/// 👆️ A press places the caret at the stop nearest the pointer, a drag moves the selection's caret end, release ends the drag.
#[test]
fn a_press_places_the_caret_and_a_drag_selects() {
    let (mut tree, root, field, mut router) = editing_field("longText", "abcd", 4, 4);
    tree.node_mut(field).expect("field").state.edit_layout = Some(wrapped_abcd());
    router.dispatch(&mut tree, root, &pointer_down(6.0, 12.0));
    assert_eq!(edit_snapshot(&tree, field), ("abcd".into(), 3, 3));
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 11.0, y: 12.0, modifiers: Default::default() });
    assert_eq!(edit_snapshot(&tree, field), ("abcd".into(), 4, 3), "dragging extends from the press");
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 1.0, y: 2.0, modifiers: Default::default() });
    assert_eq!(edit_snapshot(&tree, field), ("abcd".into(), 0, 3), "dragging up and left reaches the first line");
    router.dispatch(&mut tree, root, &pointer_up(1.0, 2.0));
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 11.0, y: 12.0, modifiers: Default::default() });
    assert_eq!(edit_snapshot(&tree, field).1, 0, "a released press drags nothing");
}

/// 🖱️ Two quick presses select the word, three the line; a slow one starts over.
#[test]
fn double_click_selects_the_word_and_triple_click_the_line() {
    let (mut tree, root, field, mut router) = editing_field("text", "ab cd", 0, 0);
    let stops: Vec<(usize, f32)> = (0..=5).map(|byte| (byte, byte as f32 * 5.0)).collect();
    tree.node_mut(field).expect("field").state.edit_layout = Some(laid_text("ab cd", false, &[&stops]));
    for _ in 0..2 {
        router.dispatch(&mut tree, root, &pointer_down(16.0, 5.0));
        router.dispatch(&mut tree, root, &pointer_up(16.0, 5.0));
    }
    assert_eq!(edit_snapshot(&tree, field), ("ab cd".into(), 5, 3), "the word under the pointer");
    router.dispatch(&mut tree, root, &pointer_down(16.0, 5.0));
    router.dispatch(&mut tree, root, &pointer_up(16.0, 5.0));
    assert_eq!(edit_snapshot(&tree, field), ("ab cd".into(), 5, 0), "the whole line");
    router.advance_clock(&mut tree, 5.0);
    router.dispatch(&mut tree, root, &pointer_down(16.0, 5.0));
    assert_eq!(edit_snapshot(&tree, field), ("ab cd".into(), 3, 3), "a slow press is a first click again");
}

/// 🛞️ A wheel turn over a focused textarea scrolls its text, clamped to the layout, and yields once it cannot.
#[test]
fn the_wheel_scrolls_a_textarea_within_its_layout() {
    let (mut tree, root, field, mut router) = editing_field("longText", "abcd", 0, 0);
    tree.node_mut(field).expect("field").state.edit_layout = Some(laid_text("abcd", true, &[&[(0, 0.0)], &[(1, 0.0)], &[(2, 0.0)], &[(3, 0.0), (4, 5.0)]]));
    let wheel = |delta_y: f32| UiEvent::Scroll { x: 10.0, y: 10.0, delta_x: 0.0, delta_y, modifiers: Default::default() };
    router.dispatch(&mut tree, root, &wheel(10.0));
    assert_eq!(tree.node(field).expect("field").state.scroll_offset.1, 10.0);
    router.dispatch(&mut tree, root, &wheel(10.0));
    assert_eq!(tree.node(field).expect("field").state.scroll_offset.1, 16.0, "four 10px lines in a 24px box leave 16px to scroll");
    router.dispatch(&mut tree, root, &wheel(10.0));
    assert_eq!(tree.node(field).expect("field").state.scroll_offset.1, 16.0);
    router.dispatch(&mut tree, root, &wheel(-30.0));
    assert_eq!(tree.node(field).expect("field").state.scroll_offset.1, 0.0);
}

/// 🛞️ A wheel turn over a textarea that is not focused scrolls its text all the same, as a browser's native textarea does.
#[test]
fn the_wheel_scrolls_an_unfocused_textarea() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 100.0));
    let field = leaf(&mut tree, Some(root), 1, draft_field("longText", "abcd"), (0.0, 0.0, 120.0, 24.0));
    tree.node_mut(field).expect("field").state.edit_layout = Some(laid_text("abcd", true, &[&[(0, 0.0)], &[(1, 0.0)], &[(2, 0.0)], &[(3, 0.0), (4, 5.0)]]));
    let mut router = EventRouter::new("main");
    assert!(tree.node(field).expect("field").state.edit.is_none(), "the field holds no edit buffer");
    router.dispatch(&mut tree, root, &UiEvent::Scroll { x: 10.0, y: 10.0, delta_x: 0.0, delta_y: 10.0, modifiers: Default::default() });
    assert_eq!(tree.node(field).expect("field").state.scroll_offset.1, 10.0);
    router.dispatch(&mut tree, root, &UiEvent::Scroll { x: 10.0, y: 10.0, delta_x: 0.0, delta_y: 30.0, modifiers: Default::default() });
    assert_eq!(tree.node(field).expect("field").state.scroll_offset.1, 16.0, "clamped to the layout like a focused one");
}

/// ✋️ A double click held and dragged extends the selection by word, keeping the clicked word selected on both sides.
#[test]
fn double_click_then_drag_extends_the_selection_by_word() {
    let (mut tree, root, field, mut router) = editing_field("text", "ab cd", 0, 0);
    let stops: Vec<(usize, f32)> = (0..=5).map(|byte| (byte, byte as f32 * 5.0)).collect();
    tree.node_mut(field).expect("field").state.edit_layout = Some(laid_text("ab cd", false, &[&stops]));
    router.dispatch(&mut tree, root, &pointer_down(16.0, 5.0));
    router.dispatch(&mut tree, root, &pointer_up(16.0, 5.0));
    router.dispatch(&mut tree, root, &pointer_down(16.0, 5.0));
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 1.0, y: 5.0, modifiers: Default::default() });
    assert_eq!(edit_snapshot(&tree, field), ("ab cd".into(), 0, 5), "dragging back over the first word selects both whole words, the anchor at the clicked word's far end");
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 16.0, y: 5.0, modifiers: Default::default() });
    assert_eq!(edit_snapshot(&tree, field), ("ab cd".into(), 5, 3), "back inside the clicked word it is just that word");
    router.dispatch(&mut tree, root, &pointer_up(16.0, 5.0));
}

/// ✋️ A triple click held and dragged extends the selection by whole logical lines.
#[test]
fn triple_click_then_drag_extends_the_selection_by_line() {
    let (mut tree, root, field, mut router) = editing_field("longText", "ab\ncd\nef", 0, 0);
    tree.node_mut(field).expect("field").state.edit_layout = Some(laid_text("ab\ncd\nef", true, &[&[(0, 0.0), (1, 5.0), (2, 10.0)], &[(3, 0.0), (4, 5.0), (5, 10.0)], &[(6, 0.0), (7, 5.0), (8, 10.0)]]));
    for _ in 0..2 {
        router.dispatch(&mut tree, root, &pointer_down(2.0, 15.0));
        router.dispatch(&mut tree, root, &pointer_up(2.0, 15.0));
    }
    router.dispatch(&mut tree, root, &pointer_down(2.0, 15.0));
    assert_eq!(edit_snapshot(&tree, field), ("ab\ncd\nef".into(), 5, 3), "the middle line");
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 2.0, y: 5.0, modifiers: Default::default() });
    assert_eq!(edit_snapshot(&tree, field), ("ab\ncd\nef".into(), 0, 5), "dragging up takes the first line too");
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 2.0, y: 25.0, modifiers: Default::default() });
    assert_eq!(edit_snapshot(&tree, field), ("ab\ncd\nef".into(), 8, 3), "dragging down takes the last line instead");
}

/// 👆️ A first press on a field whose text has not been laid out yet still puts the caret under the pointer, at the layout the next frame parks.
#[test]
fn a_first_press_before_the_first_layout_places_the_caret_once_the_layout_arrives() {
    let (mut tree, root, field, mut router) = editing_field("text", "ab cd", 5, 5);
    router.dispatch(&mut tree, root, &pointer_down(16.0, 5.0));
    router.dispatch(&mut tree, root, &pointer_up(16.0, 5.0));
    assert_eq!(edit_snapshot(&tree, field), ("ab cd".into(), 5, 5), "no layout yet: the caret waits where focus put it");
    let stops: Vec<(usize, f32)> = (0..=5).map(|byte| (byte, byte as f32 * 5.0)).collect();
    tree.node_mut(field).expect("field").state.edit_layout = Some(laid_text("ab cd", false, &[&stops]));
    router.settle_text_press(&mut tree, field);
    assert_eq!(edit_snapshot(&tree, field), ("ab cd".into(), 3, 3), "the caret goes where the pointer was");
    router.settle_text_press(&mut tree, field);
    assert_eq!(edit_snapshot(&tree, field), ("ab cd".into(), 3, 3), "and the press is spent");
}

/// 📜️ The layout's pure geometry: nearest stop, vertical neighbours, the scroll limits and the least scroll that reveals a caret.
#[test]
fn an_edit_layout_answers_nearest_stop_neighbours_and_scroll() {
    let laid = wrapped_abcd();
    assert_eq!((laid.byte_at(2.4, 3.0), laid.byte_at(2.6, 3.0), laid.byte_at(99.0, 15.0), laid.byte_at(-5.0, 99.0)), (0, 1, 4, 2));
    assert_eq!((laid.vertical(0, None, true), laid.vertical(3, Some(0.0), false), laid.vertical(3, None, true)), (Some((2, 0.0)), Some((0, 0.0)), None));
    let tall = laid_text("abcd", true, &[&[(0, 0.0)], &[(1, 0.0)], &[(2, 0.0)], &[(3, 0.0), (4, 5.0)]]);
    assert_eq!(tall.max_scroll(), (0.0, 16.0));
    assert_eq!((tall.reveal(4, (0.0, 0.0)), tall.reveal(0, (0.0, 16.0)), tall.reveal(2, (0.0, 4.0))), ((0.0, 16.0), (0.0, 0.0), (0.0, 6.0)));
    let wide = laid_text("x", false, &[&[(0, 0.0), (1, 150.0)]]);
    assert_eq!((wide.max_scroll(), wide.reveal(1, (0.0, 0.0))), ((51.0, 0.0), (51.0, 0.0)));
}

//#endregion 🔖️TextFieldEditingVectorTests
