// 🧭️ Flow direction, focus visible and search moment laws: the right-to-left flow mirroring, the focus visible rule and the search line moment.

//#region 🧭️FlowDirectionTests
// 🧭️ W2k: this target had ZERO flow-direction support — `events`' own positioner docstring said so.
// React takes `rtl` from `useFlow().inline` (`🧱️elements/🗨️Popover/🟦️.tsx:311, :326`) and inverts the
// horizontal arrow pair per control (`🎚️Slider/🟦️.tsx:383-384`, `📑️Tabs/🟦️.tsx:164-165`).

#[semio_framework_async_macros::async_test]
async fn a_menu_mirrors_its_inline_edge_once_the_window_carries_an_rtl_flow() {
    let anchor = Rect::new(100.0, 40.0, 60.0, 20.0);
    let ltr = resolve_anchored_placement(anchor, (30.0, 40.0), viewport(), AnchoredPlacement::MENU, FlowInline::Ltr);
    let rtl = resolve_anchored_placement(anchor, (30.0, 40.0), viewport(), AnchoredPlacement::MENU, FlowInline::Rtl);
    assert_eq!(ltr.x, 100.0);
    assert_eq!(rtl.x, 130.0);
    assert_eq!(ltr.y, rtl.y, "only the inline axis mirrors");
    assert_eq!(ltr.side, rtl.side, "React never mirrors `side`, only `align`");
}

#[semio_framework_async_macros::async_test]
async fn a_routers_flow_defaults_to_ltr_down_and_only_changes_once() {
    let mut router = EventRouter::new("w");
    assert_eq!(router.flow(), UiFlow::DEFAULT);
    assert_eq!(router.flow_inline(), FlowInline::Ltr);
    let mirrored = UiFlow { inline: FlowInline::Rtl, block: ui_contract::FlowBlock::Up };
    assert!(router.set_flow(mirrored), "a real change reports true so the caller can invalidate layout");
    assert!(!router.set_flow(mirrored), "re-setting the same flow is a no-op");
    assert_eq!(router.flow_inline(), FlowInline::Rtl);
}

#[test]
fn an_up_flow_tree_section_routes_its_bottom_header_through_the_event_router() {
    let section = UiTreeSectionNode { header_toolbar: None, window: None, id: "drivers".into(), label: Some(Label::data("Drivers")), default_open: Some(true), presence: UiPresence::default(), items: Vec::new() };
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, tree_ui(vec![section]), (0.0, 0.0, 200.0, 100.0));
    let disclosure = insert_tree_row(&mut tree, root, "drivers", (0.0, 0.0, 200.0, 100.0));
    let mut router = EventRouter::new("w");
    router.set_flow(UiFlow { inline: FlowInline::Ltr, block: ui_contract::FlowBlock::Up });

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 100.0, y: 88.0, button: PointerButton::Primary, modifiers: EventModifiers::default() });
    assert_eq!(router.capture.target.map(|(_, id, _)| id), Some(disclosure), "the painted bottom header owns the press");
    router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 100.0, y: 88.0, button: PointerButton::Primary, modifiers: EventModifiers::default() });
    assert_eq!(tree.disclosure_open(disclosure), Some(false), "release over the same bottom band closes the disclosure");
}

#[semio_framework_async_macros::async_test]
async fn an_rtl_window_swaps_the_horizontal_arrow_pair_and_leaves_every_other_key_alone() {
    let mut router = EventRouter::new("w");
    assert_eq!(router.mirrored_inline_key("ArrowLeft"), "ArrowLeft");
    router.set_flow(UiFlow { inline: FlowInline::Rtl, block: ui_contract::FlowBlock::Down });
    assert_eq!(router.mirrored_inline_key("ArrowLeft"), "ArrowRight");
    assert_eq!(router.mirrored_inline_key("ArrowRight"), "ArrowLeft");
    for invariant in ["ArrowUp", "ArrowDown", "Home", "End", "PageUp", "PageDown", "Enter", " "] {
        assert_eq!(router.mirrored_inline_key(invariant), invariant, "{invariant} carries no flow direction");
    }
}

#[semio_framework_async_macros::async_test]
async fn an_rtl_slider_moves_the_other_way_for_the_same_arrow_key() {
    let slider = UiSliderNode { id: "s".into(), value: 5.0, min: 0.0, max: 10.0, step: 1.0, unit: None, snaps: Vec::new(), on_change: action(), presence: UiPresence::default(), menu: None, ..Default::default() };
    let mut router = EventRouter::new("w");
    let ltr_right = slider_key_value(&slider, router.mirrored_inline_key("ArrowRight"), false);
    router.set_flow(UiFlow { inline: FlowInline::Rtl, block: ui_contract::FlowBlock::Down });
    let rtl_right = slider_key_value(&slider, router.mirrored_inline_key("ArrowRight"), false);
    assert_eq!(ltr_right, Some(6.0));
    assert_eq!(rtl_right, Some(4.0), "React's `positiveHorizontal` is ArrowLeft under rtl (🎚️Slider/🟦️.tsx:383)");
}
//#endregion 🧭️FlowDirectionTests

//#region ⌨️FocusVisibleTests
// ⌨️ W2k: `focus-visible:border-accent` is a `:focus-visible` selector, so a CLICKED control must not
// paint the accent ring. `EventRouter` carries the modality latch and stamps `NodeFlags::FOCUS_VISIBLE`.

#[semio_framework_async_macros::async_test]
async fn a_pointer_press_focuses_without_the_keyboard_ring_and_a_tab_move_restores_it() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 100.0, 100.0));
    let pressed = leaf(&mut tree, Some(root), 1, button_ui("a"), (0.0, 0.0, 50.0, 20.0));
    leaf(&mut tree, Some(root), 2, button_ui("b"), (0.0, 20.0, 50.0, 20.0));
    let mut router = EventRouter::new("w");

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 5.0, y: 5.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert!(!router.focus_visible(), "a press clears the modality latch");
    let flags = tree.node(pressed).expect("focused node is live").flags;
    assert!(flags.contains(NodeFlags::FOCUSED), "the press still focuses the control");
    assert!(!flags.contains(NodeFlags::FOCUS_VISIBLE), "a pointer-focused control paints no focus ring");

    router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Tab".into(), modifiers: EventModifiers::default() });
    assert!(router.focus_visible(), "a key sets the modality latch");
    let tabbed = router.focus.focused.expect("Tab moved focus");
    let tabbed_flags = tree.node(tabbed).expect("tabbed node is live").flags;
    assert!(tabbed_flags.contains(NodeFlags::FOCUSED) && tabbed_flags.contains(NodeFlags::FOCUS_VISIBLE), "a keyboard-focused control paints the ring");
    assert!(!tree.node(pressed).expect("blurred node is live").flags.contains(NodeFlags::FOCUS_VISIBLE), "blurring clears the bit");
}
//#endregion ⌨️FocusVisibleTests

//#region ✍️SearchLineMomentTests

/// ✍️ One window-search line — React's `SearchInput` shape: `onChange` per keystroke plus the
/// `onSubmit`/`onAbort`/`onRepeatLast` trio, each a DISTINCT verb so a dispatched intent names which
/// moment fired it.
fn search_line_ui(id: &str, value: &str, repeat_last: bool) -> UiNode {
    let verb = |name: &str| ActionDescriptor { controller_id: "ctrl".into(), action: name.into(), args: None };
    UiNode::Input(UiInputNode {
        id: id.into(),
        input_kind: "text".into(),
        value: value.into(),
        placeholder: None,
        accessibility_label: None,
        commit: None,
        min: None,
        max: None,
        step: None,
        accept: None,
        precision: None,
        snaps: Vec::new(),
        on_change: verb("engagementInput"),
        on_submit: Some(verb("engagementSubmit")),
        on_abort: Some(verb("engagementAbort")),
        on_repeat_last: repeat_last.then(|| verb("engagementRepeatLast")),
        presence: UiPresence::default(),
        menu: None,
        ..Default::default()
    })
}

fn fired_verbs(commands: &[UiCommand]) -> Vec<(String, String)> {
    commands.iter().filter_map(|cmd| if let UiCommand::App { intent, .. } = cmd { Some((format!("{:?}", intent.trigger), intent.action_name())) } else { None }).collect()
}

/// ✍️ **LAW: one retained line fires FOUR moments, not one.** React's window search binds
/// `onChange` on every keystroke, `onSubmit` on Enter (with the TRIMMED line), `onAbort` on Escape and
/// `onRepeatLast` on Space over an empty idle line — all on the SAME field
/// (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`'s `Search`, `onKeyDown`; `applySearchSpaceAction` at `:10512`).
///
/// 🩸️ This target produced only `Trigger::Change`/`Trigger::Commit` from an editable node, so a
/// command line could be EITHER a typing feed or a verb runner and never both: the os shell's search
/// pane resolved `on_submit` into the single `on_change` slot, committed it on blur, and the program
/// never saw a keystroke (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
/// `📓️w13b-actions-search-pane-bodies.md` §6 gap 4).
#[test]
fn a_search_line_fires_change_while_typing_and_submit_on_enter() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), 1, search_line_ui("puzzle3d-engagement", "", false), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });

    let typed = router.dispatch(&mut tree, root, &UiEvent::TextInput { text: "box ".into() });
    assert_eq!(fired_verbs(&typed), vec![("Change".to_string(), "engagementInput".to_string())], "✍️ every keystroke feeds the program's autocomplete through `onChange`");

    let entered = router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Enter".into(), modifiers: EventModifiers::default() });
    assert_eq!(fired_verbs(&entered), vec![("Submit".to_string(), "engagementSubmit".to_string())], "⏎️ Enter confirms through `onSubmit`, and never re-fires the change");
    let submitted = entered.iter().find_map(|cmd| if let UiCommand::App { intent, .. } = cmd { intent.payload() } else { None }).expect("⏎️ the submit carries the line");
    assert_eq!(submitted, DslValue::Object(vec![("value".into(), DslValue::String("box".into()))]), "⏎️ React submits `draft.trim()`");
}

/// ⎋️🔁️ **LAW: Escape aborts and Space repeats — and only where React routes them.** Escape
/// dispatches `onAbort` when no overlay is open (React's `Search` closes its possibles popover first);
/// Space over an EMPTY line dispatches `onRepeatLast`, falls back to the submit the program did bind,
/// and types an ordinary space once the line carries text.
#[test]
fn a_search_line_aborts_on_escape_and_repeats_last_on_space_over_an_empty_line() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let line = leaf(&mut tree, Some(root), 1, search_line_ui("puzzle3d-engagement", "", true), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });

    let spaced = router.dispatch(&mut tree, root, &UiEvent::TextInput { text: " ".into() });
    assert_eq!(fired_verbs(&spaced), vec![("RepeatLast".to_string(), "engagementRepeatLast".to_string())], "🔁️ Space over an empty idle line restarts the last engagement");
    assert_eq!(tree.node(line).unwrap().state.edit.as_ref().map(|edit| edit.text.clone()).unwrap_or_default(), "", "🔁️ and it never types the space it consumed");

    router.dispatch(&mut tree, root, &UiEvent::TextInput { text: "box".into() });
    let spaced = router.dispatch(&mut tree, root, &UiEvent::TextInput { text: " ".into() });
    assert!(fired_verbs(&spaced).iter().all(|(_, verb)| verb != "engagementRepeatLast"), "🔁️ a line that carries text types its space: {:?}", fired_verbs(&spaced));
    assert_eq!(tree.node(line).unwrap().state.edit.as_ref().map(|edit| edit.text.clone()).unwrap_or_default(), "box ", "🔁️ and the space lands in the buffer");

    let escaped = router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Escape".into(), modifiers: EventModifiers::default() });
    assert_eq!(fired_verbs(&escaped), vec![("Abort".to_string(), "engagementAbort".to_string())], "⎋️ Escape cancels the session through `onAbort`");

    let popup = leaf(&mut tree, Some(root), 2, stack_ui(), (10.0, 10.0, 50.0, 50.0));
    router.open_overlay(&mut tree, popup, OverlayKind::SelectPopup, OverlayAnchor::Node(root));
    let escaped = router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Escape".into(), modifiers: EventModifiers::default() });
    assert!(fired_verbs(&escaped).is_empty(), "⎋️ the first Escape only closes the popover: {:?}", fired_verbs(&escaped));
}

/// ✍️ **LAW: a line that binds none of the three keeps its old behaviour.** Every other retained
/// input in the fleet binds `on_change` alone, so Escape must stay an overlay key and Space must stay
/// a character — the producers above are opt-in per node, never a new global key grammar.
#[test]
fn an_ordinary_input_keeps_escape_and_space_to_itself() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let input = leaf(&mut tree, Some(root), 1, input_ui("name", ""), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });

    let spaced = router.dispatch(&mut tree, root, &UiEvent::TextInput { text: " ".into() });
    assert_eq!(tree.node(input).unwrap().state.edit.as_ref().map(|edit| edit.text.clone()).unwrap_or_default(), " ", "✍️ an ordinary field types its space");
    assert!(fired_verbs(&spaced).iter().all(|(trigger, _)| trigger == "Change"), "✍️ and reports it as an ordinary change: {:?}", fired_verbs(&spaced));

    let escaped = router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Escape".into(), modifiers: EventModifiers::default() });
    assert!(fired_verbs(&escaped).is_empty(), "✍️ and dispatches nothing on Escape: {:?}", fired_verbs(&escaped));
}
//#endregion ✍️SearchLineMomentTests
