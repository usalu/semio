/// 🔎️ Quick search palette keyboard: the palette chord, overlay fields not counting as typing and escape closing the palette.

//#region 🔎️QuickSearchPaletteKeyboard
/// ⌨️ `mod+p`, as the browser delivers it.
fn palette_chord() -> (semio_framework_ui_wgpu_input::KeyAction, PointerModifiers) {
    (semio_framework_ui_wgpu_input::KeyAction::Char("p".into()), PointerModifiers { shift: false, ctrl: true, alt: false, meta: false })
}

/// ⚖️ LAW: the shell's own overlay query fields are CHROME, not content.
///
/// The gate every hardcoded shell chord sits behind used to be a bare `focused_id.is_some()`, and
/// the chord that opens the quick-search palette focuses the palette's own query field — so opening
/// the palette disabled the chord that closes it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn the_shells_own_overlay_fields_do_not_count_as_the_user_typing() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    assert!(!content_is_editing(None, false), "nothing focused is not editing");
    assert!(!content_is_editing(Some("ui.search.input"), false), "the palette's own query field is chrome");
    assert!(!content_is_editing(Some("ui.find.input"), false), "so is the find overlay's");
    assert!(content_is_editing(Some("generation3d.height"), false), "an app content field IS the user typing");
    assert!(content_is_editing(None, true), "and so is the sync-attach draft buffer");
}

/// ⚖️ LAW: `mod+p` TOGGLES the quick-search palette, and the palette owns every key while it is open.
///
/// Measured on 6118 before this fix: the second `mod+p` did not close the palette, it fell through
/// to the open palette's own `Char` arm and typed a literal `p` into the query — which then made
/// every later keystroke search for `p<whatever the user meant>` and `Enter` activate nothing at all.
#[test]
fn the_palette_chord_toggles_and_never_types_itself_into_the_query() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let mut input = InputState::<ActionDescriptor>::default();
    let (action, modifiers) = palette_chord();

    PointerCtx::of(&mut shell).handle_keyboard(action.clone(), &modifiers, &mut input);
    assert_eq!(shell.input.overlay_state, OverlayState::Search, "the chord opens the palette");
    assert_eq!(input.focused_id.as_deref(), Some("ui.search.input"), "and focuses its query field");

    for key in ["D", "e"] {
        PointerCtx::of(&mut shell).handle_keyboard(semio_framework_ui_wgpu_input::KeyAction::Char(key.into()), &PointerModifiers::default(), &mut input);
    }
    assert_eq!(shell.commands.search_query, "De", "plain keys type into the palette's query");

    PointerCtx::of(&mut shell).handle_keyboard(action, &modifiers, &mut input);
    assert_eq!(shell.input.overlay_state, OverlayState::None, "the same chord closes it");
    assert!(!shell.commands.search_open);
    assert_eq!(input.focused_id, None, "and hands focus back to the canvas");
    assert_eq!(shell.commands.search_query, "De", "unactivated dismissal preserves React's owned query without appending a literal `p`");
    PointerCtx::of(&mut shell).handle_keyboard(semio_framework_ui_wgpu_input::KeyAction::Char("x".into()), &PointerModifiers::default(), &mut input);
    assert_eq!(shell.commands.search_query, "De", "closed palette state does not consume ordinary typing");
    PointerCtx::of(&mut shell).handle_keyboard(semio_framework_ui_wgpu_input::KeyAction::Char("p".into()), &PointerModifiers::default(), &mut input);
    assert_eq!(shell.commands.search_query, "De", "the chord's character never leaks into a closed retained query");
}

/// ⚖️ LAW: Escape closes the palette. It used to be claimed by the focused-input commit first, which
/// left the palette with no keyboard route out at all once the toggle was broken too.
#[test]
fn escape_closes_the_palette_rather_than_committing_its_query_field() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let mut input = InputState::<ActionDescriptor>::default();
    let (action, modifiers) = palette_chord();
    PointerCtx::of(&mut shell).handle_keyboard(action, &modifiers, &mut input);
    assert_eq!(shell.input.overlay_state, OverlayState::Search);

    semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_keyboard_async(semio_framework_ui_wgpu_input::KeyAction::Escape, &PointerModifiers::default(), &mut input)).expect("escape routes");
    assert_eq!(shell.input.overlay_state, OverlayState::None, "escape closes the topmost overlay");
    assert_eq!(input.focused_id, None);
}
//#endregion 🔎️QuickSearchPaletteKeyboard
