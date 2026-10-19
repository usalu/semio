//! ⌨️ Keyboard handling: the synchronous and asynchronous keyboard handlers and the keyboard ring advance. See [`super`].

use super::*;

//#region ShellInput
impl<'ctx> PointerCtx<'ctx> {
    /// 🪟️ Escape closes exactly the topmost overlay first — matches ui_wgpu's overlay-manager
    /// precedence (`EventRouter::close_topmost_overlay`, report-w1d-events-overlay.md: "Escape
    /// closes only the topmost") even though these ad-hoc chrome overlays predate that stack and
    /// aren't routed through it yet. A Select dropdown is the most local/transient overlay, so it
    /// wins over the context menu; neither used to close on Escape at all before this fix.
    ///
    /// 🎓️ Tour keys take precedence over every other chord below (mirrors Escape closing the topmost
    /// overlay above) and — unlike every other chord in this function — fire even while a field has
    /// focus: React binds all three with `enableOnFormTags: true` (`UIIntroduction`), because a veil
    /// that owns every pointer must always be answerable from the keyboard. The sync-attach card
    /// still outranks them, since it is the one overlay painted above the tour.
    ///
    /// ⌨️ Hardcoded shell chords never fire while a text field (or the sync-attach draft buffer)
    /// has focus — matches `os-shell.tsx`'s `isEditableEventTarget`/`useActionHotkey` (backed by
    /// react-hotkeys-hook, which by default does not fire on form tags): "hotkeys never fire while
    /// the user is typing". Previously these six chords fired unconditionally, so e.g. Ctrl+B while
    /// typing in a focused Input would silently toggle the left panel instead of inserting "b".
    ///
    /// 🛡️ Every chord is now one row of `SHELL_SHORTCUT_ROWS` (the transcription of React's
    /// `SHELL_KEYBINDINGS`) rather than an open-coded `meta && Char("b")` ladder: the palette/find
    /// toggles, the three nav verbs, all eight panel anchors and the three window verbs resolve
    /// through the same table `is_reserved_shell_chord` reserves, so a chord can no longer be
    /// reserved-but-dead (which is exactly how the six missing panel anchors and the three window
    /// verbs read before this wave).
    ///
    /// ⌨️ An ARMED Keybindings row swallows the next key event and becomes that row's override —
    /// React's `chordFromKeyboardEvent` on the capture button, ahead of every other rung so a chord
    /// being remapped never also fires its own verb.
    ///
    /// ⌨️ Tab/Shift+Tab walks the keyboard ring ([`Self::keyboard_ring`]) whenever nothing else claims
    /// focus: every focusable chrome control, then every dock window. This renderer has no DOM, so
    /// before this the chrome had no keyboard order at all and Tab only cycled windows. Content-level
    /// Tab among a window's own widgets stays the retained content's (`EventRouter`), routed above.
    ///
    /// 📌️ ticket §C5 item 3 — same shell-owned keyboard-routed draft-field idiom as
    /// `sync_card_kind` above, for the check-in message-prompt card.
    pub fn handle_keyboard(&mut self, action: semio_framework_ui_wgpu_input::KeyAction, modifiers: &PointerModifiers, input: &mut InputState<ActionDescriptor>) {
        if self.chrome.chrome_build.dialog_open() && self.input.handle_chrome_dialog_key(&action, modifiers, &mut self.chrome, &mut self.settle) {
            return;
        }
        if action == semio_framework_ui_wgpu_input::KeyAction::Escape {
            if semio_framework_os_renderer_wgpu_scene_render::cancel_canvas_interactions(input) {
                if let Some((window_id, pointer_id)) = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_any_pointer_capture() {
                    semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::dispatch_ui_pointer_event(&window_id, pointer_id, semio_framework_ui_wgpu_events::UiEvent::PointerCancel, input);
                }
                return;
            }
            if semio_framework_os_renderer_wgpu_scene_render::cancel_scene_list_transfer() {
                self.input.chrome_pointer_owner = None;
                return;
            }
            if self.dock.dock_drag.take().is_some() || self.dock.pending_dock_drag.take().is_some() {
                self.input.chrome_pointer_owner = None;
                self.input.pending_chrome_release = None;
                return;
            }
            if self.input.dismiss_dismissable_layers() {
                return;
            }
            if self.input.context_menu.is_some() {
                match self.input.context_menu_handle_key(semio_framework_ui_wgpu_input::KeyAction::Escape) {
                    ContextMenuKeyOutcome::Activate(_) => return,
                    ContextMenuKeyOutcome::Ignored => {}
                    ContextMenuKeyOutcome::Consumed | ContextMenuKeyOutcome::CloseMenu => return,
                }
            }
        }
        if self.sync.sync_card_kind.is_none() {
            if let Some(step) = self.docs.chrome_tour_active_step(&self.chrome) {
                match action {
                    semio_framework_ui_wgpu_input::KeyAction::Escape => {
                        if let Some(seen_key) = self.docs.introduction_seen_key() {
                            self.chrome.chrome_build.dismiss_introduction(&seen_key);
                        } else {
                            self.chrome.chrome_build.skip_introduction();
                        }
                        return;
                    }
                    semio_framework_ui_wgpu_input::KeyAction::Enter | semio_framework_ui_wgpu_input::KeyAction::ArrowRight => {
                        if step.interactions.is_empty() {
                            self.docs.chrome_tour_advance_current_step(&step, &mut self.chrome);
                            return;
                        }
                    }
                    semio_framework_ui_wgpu_input::KeyAction::ArrowLeft => {
                        self.chrome.chrome_build.back_introduction();
                        return;
                    }
                    _ => {}
                }
            }
        }
        let editing = content_is_editing(input.focused_id.as_deref(), self.sync.sync_card_kind.is_some());
        let palette_open = matches!(self.input.overlay_state, OverlayState::Search | OverlayState::Find);
        if self.input.keybinding_capture_control_id.is_some() {
            if self.input.capture_keybinding_chord(&action, modifiers, &mut self.chrome) {
                return;
            }
        }
        if !editing {
            if let Some(shortcut) = shell_shortcut_for_in(&self.chrome.shortcut_table(), &action, modifiers) {
                if self.as_session().apply_sync_shell_shortcut(shortcut, palette_open, input) {
                    return;
                }
            }
        }
        if !editing && !palette_open && self.dock.dock_drag.is_none() && action == semio_framework_ui_wgpu_input::KeyAction::Tab {
            self.advance_keyboard_ring(!modifiers.shift, input);
            return;
        }
        if self.sync.sync_card_kind.is_some() {
            match action {
                semio_framework_ui_wgpu_input::KeyAction::Escape => {
                    self.sync.sync_card_kind = None;
                    return;
                }
                semio_framework_ui_wgpu_input::KeyAction::Enter => {
                    self.settle.deferred_actions.push(ActionDescriptor {
                        controller_id: "framework.sync".into(),
                        action: "attach".into(),
                        args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({
                            "path": self.sync.sync_card_draft,
                            "kind": self.sync.sync_card_kind,
                        }),
                    });
                    return;
                }
                semio_framework_ui_wgpu_input::KeyAction::Char(key) => {
                    self.sync.sync_card_draft.push_str(&key);
                    return;
                }
                semio_framework_ui_wgpu_input::KeyAction::Backspace => {
                    self.sync.sync_card_draft.pop();
                    return;
                }
                _ => {}
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if self.sync.checkin_dialog_draft.is_some() {
            match action {
                semio_framework_ui_wgpu_input::KeyAction::Escape => {
                    self.sync.checkin_dialog_draft = None;
                    return;
                }
                semio_framework_ui_wgpu_input::KeyAction::Enter => {
                    let message = self.sync.checkin_dialog_draft.clone().unwrap_or_default();
                    self.sync.checkin_dialog_draft = None;
                    self.settle.deferred_actions.push(ActionDescriptor { controller_id: "framework.checkin".into(), action: "submit".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "message": message }) });
                    return;
                }
                semio_framework_ui_wgpu_input::KeyAction::Char(key) => {
                    if let Some(draft) = self.sync.checkin_dialog_draft.as_mut() {
                        draft.push_str(&key);
                    }
                    return;
                }
                semio_framework_ui_wgpu_input::KeyAction::Backspace => {
                    if let Some(draft) = self.sync.checkin_dialog_draft.as_mut() {
                        draft.pop();
                    }
                    return;
                }
                _ => {}
            }
        }
        if palette_open {
            match action {
                semio_framework_ui_wgpu_input::KeyAction::Escape => {
                    self.commands.close_command_palette(input, &mut self.input);
                }
                semio_framework_ui_wgpu_input::KeyAction::ArrowDown => {
                    if self.input.overlay_state == OverlayState::Search {
                        let len = self.as_session().filtered_search_items().len();
                        if len > 0 {
                            self.commands.search_selected = (self.commands.search_selected + 1).min(len - 1);
                        }
                    } else {
                        let len = self.commands.filtered_find_items().len();
                        if len > 0 {
                            self.commands.find_selected = (self.commands.find_selected + 1).min(len - 1);
                        }
                    }
                }
                semio_framework_ui_wgpu_input::KeyAction::ArrowUp => {
                    if self.input.overlay_state == OverlayState::Search {
                        self.commands.search_selected = self.commands.search_selected.saturating_sub(1);
                    } else {
                        self.commands.find_selected = self.commands.find_selected.saturating_sub(1);
                    }
                }
                semio_framework_ui_wgpu_input::KeyAction::Enter => {
                    let runtime = ();
                    let _ = runtime;
                }
                semio_framework_ui_wgpu_input::KeyAction::Char(key) => {
                    let kind = if self.input.overlay_state == OverlayState::Search { ShellPaletteKind::Search } else { ShellPaletteKind::Find };
                    let mut query = self.commands.palette_query(kind).to_string();
                    query.push_str(&key);
                    self.commands.set_palette_query(kind, query, input, &mut self.input);
                }
                semio_framework_ui_wgpu_input::KeyAction::Backspace => {
                    let kind = if self.input.overlay_state == OverlayState::Search { ShellPaletteKind::Search } else { ShellPaletteKind::Find };
                    let mut query = self.commands.palette_query(kind).to_string();
                    query.pop();
                    self.commands.set_palette_query(kind, query, input, &mut self.input);
                }
                _ => {}
            }
            return;
        }
        if input.focused_id.is_some() {
            match action {
                semio_framework_ui_wgpu_input::KeyAction::Char(key) => {
                    if let Some(ch) = key.chars().next() {
                        input.insert_char(ch);
                    }
                }
                semio_framework_ui_wgpu_input::KeyAction::Backspace => input.backspace(),
                semio_framework_ui_wgpu_input::KeyAction::Delete => input.delete_forward(),
                _ => {}
            }
        }
    }

    /// 📋️⎋️ Dismissing the menu does NOT consume the chord. React's menu is a DOM popover: the
    /// same Escape keydown closes it AND reaches `handleAppKeydown`, so the app's own binding still
    /// fires — the reference journals `engagementAbort` for the `context-menu-dismiss` step
    /// (`🗑️generated/parity-run-2/steps.json`), where this target journalled nothing at all
    /// because `CloseMenu` returned here. `Consumed` is the menu's own navigation (arrows, Enter)
    /// and stays consumed, exactly as React's menu stops propagation for those.
    ///
    /// ⌨️ Enter/Space press the chrome control the keyboard ring focused — through the same accessibility
    /// activation a platform assistive technology and the browser mirror use, so all three press it alike.
    ///
    /// 🩸️ A CONTENT field's Enter/Escape commits it. The shell's own overlay query fields are not
    /// content: claiming Escape here swallowed the quick-search palette's own Escape arm, so the
    /// palette — already unable to close on its opening chord — had no keyboard route out at all.
    ///
    /// ⎋️ A dismissable layer closes on this very keydown and does NOT consume it — see
    /// [`Self::dismiss_dismissable_layers`]. Dismissing here, ahead of `idle`, is what lets the
    /// app's own Escape binding still fire under an open picker, as React's does.
    ///
    /// 👁️✏️🔁️ The `SHELL_SHORTCUT_ROWS` verbs that need the async funnel — the surface-role axis
    /// (`mod+alt+e`/`mod+alt+v`) and the mode-cycle axis (`mod+alt+→`/`mod+alt+←`). They are
    /// matched here, ahead of every app-declared chord, exactly as
    /// `is_reserved_shell_chord` reserves them. Every other row of the same table is a synchronous
    /// chrome-state transition answered by `handle_keyboard`'s own table dispatch below.
    ///
    /// 🎯️🕹️ Content-focus routing (w2-input-wiring): whenever the active window's retained
    /// content — not chrome — is the one holding focus, real keys belong there via
    /// `interpreter::dispatch_ui_event` (Escape/Tab/edit keys/clipboard chords/text), taking
    /// priority over the idle-Escape-deactivate-utility and app-keybinding dispatch below, both
    /// of which are chrome-level concerns. `content_has_focus` is this module's own best-effort
    /// tracker (see its doc comment for the one documented gap: pointer-click-driven focus
    /// changes, entirely inside the off-limits `interpreter` region, never reach it). `Tab`
    /// reaching `dispatch_event` here is ALSO the full Tab-traversal fix: `events::EventRouter::
    /// dispatch`'s own `KeyDown{key:"Tab"}` arm already calls `FocusState::focus_next`/
    /// `focus_prev` internally — nothing else to wire for that. When content does NOT have
    /// tracked focus, this is a no-operation and `handle_keyboard`'s keyboard ring
    /// (`advance_keyboard_ring`) takes Tab below.
    ///
    /// 🧰️🛠️ Escape deactivates the focused window's active utility (P5) — and, when no utility is
    /// active, the active MODE-LEVEL TOOL. Both rungs and their order are React's
    /// (`🏛️ShellHost/🟦️.tsx` `handleAppKeydown`: utility first, then `SET_ACTIVE_TOOL_ACTION_ID` with
    /// an empty id); the tool rung was missing here, so Escape left an armed tool armed.
    ///
    /// ⌨️ App-declared keybinding dispatch (Architecture Decision 8, P4) — NET-NEW for the wgpu
    /// shell, which previously only handled hardcoded shell chords. Reserved shell chords still
    /// win, now inside `match_app_keybinding` per CHORD exactly as React's own loop reserves them,
    /// so a binding whose second chord is the shell's still fires on its first.
    ///
    /// ⌨️ An app binding that ANSWERED ends the ladder; one whose verb no mounted window owns
    /// does NOT, because React's own keybinding loop `continue`s past an unresolved target and
    /// its `mod+z`/`mod+shift+z` tail still runs. Measured live: after the journey's
    /// `window-cap-close` steps the wgpu dock held zero windows, the app's own `undo`/`redo`/
    /// `escape` bindings all resolved, every one became the unowned-chord banner, and the
    /// framework tail below never ran — so `chord-escape`, `chord-undo` and `chord-redo`
    /// journalled NOTHING where React journals `engagementAbort`, `undo` and `redo`
    /// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w12c`,
    /// `🗑️generated/w12c-parity-run-17/wgpu/console.txt`'s `chord gate … docked=0`).
    ///
    /// 🖐️ A chord is a user gesture exactly as a click is: `mod+o` is the ONLY door the
    /// import picker has, and a browser grants user activation to the task that handled the
    /// key and to nothing after it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🗨️ An open chrome dialog owns the keyboard before every rung below: a focused chrome control, a retained
    /// content field, a shell chord or an app binding never takes a key from a modal — the finalize prompt of a
    /// history edit is operated by keyboard alone ([`Self::handle_chrome_dialog_key`]).
    ///
    /// ⏪️ Framework-universal undo/redo chords — LAST, after the app-keybinding loop, so an app that
    /// declares `mod+z` itself shadows them, exactly as React's `handleAppKeydown` orders its own
    /// `mod+z`/`mod+shift+z`/`mod+y` tail. Routed through the same `dispatch_action` funnel the
    /// History panel's Undo/Redo rows and the palette's `studio.undo`/`studio.redo` entries use, so
    /// the three doors to the verb cannot diverge. Before this wave the wgpu shell had NO undo chord
    /// at all: undo was reachable only by opening the palette and typing "undo".
    pub async fn handle_keyboard_async(&mut self, action: semio_framework_ui_wgpu_input::KeyAction, modifiers: &PointerModifiers, input: &mut InputState<ActionDescriptor>) -> Result<(), String> {
        if self.chrome.chrome_build.dialog_open() {
            self.handle_keyboard(action, modifiers, input);
            self.settle.owe_settle();
            return Ok(());
        }
        if self.input.context_menu.is_some() {
            match self.input.context_menu_handle_key(action.clone()) {
                ContextMenuKeyOutcome::Ignored | ContextMenuKeyOutcome::CloseMenu => {}
                ContextMenuKeyOutcome::Consumed => return Ok(()),
                ContextMenuKeyOutcome::Activate(descriptor) => {
                    self.as_actions().dispatch_action(descriptor).await?;
                    return Ok(());
                }
            }
        }
        if matches!(action, semio_framework_ui_wgpu_input::KeyAction::Enter | semio_framework_ui_wgpu_input::KeyAction::Space(true)) {
            if let Some(target) = self.input.keyboard_focused_chrome_target(input, &self.chrome) {
                self.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Activate, input).await?;
                return Ok(());
            }
        }
        if matches!(self.input.overlay_state, OverlayState::Search) && action == semio_framework_ui_wgpu_input::KeyAction::Enter {
            self.activate_search_item(self.commands.search_selected).await?;
            input.blur_input();
            self.input.accessibility_focused_control_id = None;
            return Ok(());
        }
        if matches!(self.input.overlay_state, OverlayState::Find) && action == semio_framework_ui_wgpu_input::KeyAction::Enter {
            self.activate_find_item(self.commands.find_selected).await?;
            input.blur_input();
            self.input.accessibility_focused_control_id = None;
            return Ok(());
        }
        if content_is_editing(input.focused_id.as_deref(), false) {
            match action {
                semio_framework_ui_wgpu_input::KeyAction::Enter | semio_framework_ui_wgpu_input::KeyAction::Escape => {
                    self.commit_focused_input(input).await?;
                    return Ok(());
                }
                _ => {}
            }
        }
        if action == semio_framework_ui_wgpu_input::KeyAction::Escape {
            self.input.dismiss_dismissable_layers();
        }
        let idle = input.focused_id.is_none() && self.input.overlay_state == OverlayState::None && self.sync.sync_card_kind.is_none() && self.dock.dock_drag.is_none();
        debug_log(&format!(
            "[TRACE] wgpu-shell chord gate action={action:?} idle={idle} focused={:?} overlay={:?} syncCard={} dockDrag={} menu={} window={:?} session={:?} editVerb={:?} appBinding={:?} reserved={} docked={}",
            input.focused_id,
            self.input.overlay_state,
            self.sync.sync_card_kind.is_some(),
            self.dock.dock_drag.is_some(),
            self.input.context_menu.is_some(),
            self.windows.active_window_id,
            self.docs.session.as_ref().map(|session| session.app.controller_id.clone()),
            shell_edit_verb_for(&action, modifiers).map(ShellEditVerb::action_id),
            self.docs.match_app_keybinding(&action, modifiers, &self.chrome).map(|descriptor| descriptor.action),
            is_reserved_shell_chord_in(&self.chrome.shortcut_table(), &action, modifiers),
            self.dock.dock.window_instances().len()
        ));
        if idle {
            if let Some(shortcut) = shell_shortcut_for_in(&self.chrome.shortcut_table(), &action, modifiers) {
                if shortcut.is_async() && self.apply_async_shell_shortcut(shortcut).await? {
                    return Ok(());
                }
            }
        }
        if idle && !is_reserved_shell_chord_in(&self.chrome.shortcut_table(), &action, modifiers) {
            let platform = command_host_platform();
            let command = self.docs
                .resolved_commands(&self.chrome, &self.plugins)
                .into_iter()
                .rev()
                .find(|entry| entry.definition.in_palette && entry.definition.keybindings.iter().any(|binding| binding.platform.is_none_or(|declared| declared == platform) && key_event_matches_chord(&action, modifiers, &binding.chord)));
            if let Some(entry) = command {
                if entry.definition.args.is_empty() {
                    if matches!(entry.address.owner, semio_framework::manifest::CommandOwnerAddress::Os) {
                        self.as_actions().apply_os_command(&entry.definition.id, None).await?;
                    } else {
                        let invocation = semio_framework::manifest::CommandInvocation { address: entry.address, arguments: BTreeMap::new() };
                        self.as_actions().dispatch_command(invocation).await?;
                    }
                } else {
                    self.input.overlay_state = OverlayState::Search;
                    self.commands.search_query = entry.definition.label.resolve(self.prefs.active_terminology(), self.prefs.active_locale()).to_string();
                    self.commands.search_selected = 0;
                }
                return Ok(());
            }
        }
        if idle {
            if let Some((surface, node)) = self.as_session().retained_keyboard_focus() {
                debug_log(&format!("[TRACE] wgpu-shell key routing surface={surface} node={node:?} action={action:?}"));
                let focused_select = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_node_is_select(&surface, node);
                if let Some(event) = ui_event_from_key_action(&action, modifiers, focused_select) {
                    let commands = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::dispatch_ui_event(&surface, event, input);
                    self.chrome.chrome_build.note_content_focus_commands(&commands);
                    return Ok(());
                }
            }
        }
        if idle && action == semio_framework_ui_wgpu_input::KeyAction::Escape {
            if let Some(window_id) = self.windows.active_window_id.clone() {
                if self.windows.active_utility_by_window.remove(&window_id).is_some() {
                    self.as_window_actions().refresh_ui(UiDirtyScope::Full).await?;
                    return Ok(());
                }
            }
            let armed_tool = self.docs.session.as_ref().and_then(|session| session.view_state.active_tool_id.clone()).is_some_and(|tool_id| !tool_id.is_empty());
            if armed_tool {
                let controller_id = self.docs.session.as_ref().map(|session| session.app.controller_id.clone()).unwrap_or_default();
                self.as_actions().dispatch_action(ActionDescriptor { controller_id, action: "setActiveTool".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "toolId": "" }) }).await?;
                return Ok(());
            }
        }
        if idle {
            if let Some(descriptor) = self.docs.match_app_keybinding(&action, modifiers, &self.chrome) {
                if self.dispatch_app_keybinding(descriptor).await? {
                    self.as_actions().drain_gesture_bound_work().await;
                    self.settle.owe_settle();
                    return Ok(());
                }
            }
        }
        if idle {
            if let Some(verb) = shell_edit_verb_for(&action, modifiers) {
                let controller_id = self.docs.session.as_ref().map(|session| session.app.controller_id.clone());
                if let Some(controller_id) = controller_id {
                    debug_log(&format!("[TRACE] wgpu-shell edit chord verb={} controller={controller_id}", verb.action_id()));
                    self.as_actions().dispatch_action(ActionDescriptor { controller_id, action: verb.action_id().into(), args: None }).await?;
                    self.settle.owe_settle();
                    return Ok(());
                }
            }
        }
        self.handle_keyboard(action, modifiers, input);
        self.as_actions().drain_gesture_bound_work().await;
        self.settle.owe_settle();
        Ok(())
    }

    /// ⌨️ Moves keyboard focus one stop along the ring: onto a chrome control (the accessibility focus the
    /// platform adapter and the mirror set, painted as the focus ring) or onto a dock window (the active
    /// window, whose own content focus takes the keyboard again).
    pub fn advance_keyboard_ring(&mut self, forward: bool, input: &mut InputState<ActionDescriptor>) {
        let ring = self.dock.keyboard_ring(&self.chrome);
        if ring.is_empty() {
            return;
        }
        let current = match self.input.accessibility_focused_control_id.as_deref() {
            Some(key) => ring.iter().position(|stop| matches!(stop, ShellKeyboardStop::Chrome(control) if control == key)),
            None => self.windows.active_window_id.as_deref().and_then(|active| ring.iter().position(|stop| matches!(stop, ShellKeyboardStop::Window(_, window) if window == active))),
        };
        let next = match current {
            Some(index) if forward => (index + 1) % ring.len(),
            Some(index) => (index + ring.len() - 1) % ring.len(),
            None if forward => 0,
            None => ring.len() - 1,
        };
        match ring[next].clone() {
            ShellKeyboardStop::Chrome(key) => {
                input.blur_input();
                self.chrome.chrome_build.focused_retained_surface = None;
                self.input.accessibility_focused_control_id = Some(key);
            }
            ShellKeyboardStop::Window(path, window_id) => {
                self.input.accessibility_focused_control_id = None;
                self.dock.dock.set_stack_active(&path, &window_id);
                self.dock.dock.sync_active_window(&window_id);
                self.windows.active_window_id = Some(window_id);
            }
        }
    }
}
//#endregion ShellInput
