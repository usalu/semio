//! 📄️ `shell-docs` staging of `window-actions-and-commands`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ActionPanelAndUtilities
/// ⌨️ The platform every `PlatformKeybinding.platform` filter is answered against — ONE door, the same
/// `⌨️HostPlatform` region (`🧊️renderer/🦀️.rs`) `format_keybinding_shortcut` reads through
/// [`crate::host_platform_uses_meta`]. Natively the door's cell IS `cfg!(target_os = …)`; in the
/// browser the page resolves `userAgentData.platform`/`navigator.platform` and publishes it with the
/// boot message.
///
/// 🩸️ What this replaces: the `wasm32` arm re-derived the platform from `web_sys::window()`. The wgpu
/// shell runs inside the frame Worker, whose realm owns NO `window`, so the read was always `None`,
/// `unwrap_or_default()` always `""`, and the answer always `Linux` — on every machine. A macOS
/// browser user therefore matched and was shown the `Platform::Windows`/`Linux` chord of every
/// platform-scoped keybinding (`os.toggleFullscreen`'s `f11` instead of `control+meta+f`, and the same
/// for every app-declared Mac-scoped binding), while `format_keybinding_shortcut` — fixed by W7a at
/// the same file — painted `⌘️`. Two functions, one question, two answers
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️audit-w14-transport-residual.md` §3).
pub(crate) fn command_host_platform() -> semio_framework::manifest::Platform {
    crate::host_platform()
}

//#region ⌨️WindowScope
/// 🛡️ Whether a chord carries the platform ACCELERATOR (`mod`/`ctrl`/`meta`) — twin of React's
/// `chordCarriesAcceleratorV1`. The rule the wgpu shell's own [`is_reserved_shell_chord`] already
/// applies: a bare key belongs to whichever surface has focus, an accelerator chord the shell's
/// chrome answers is never an app's to shadow.
pub(crate) fn chord_carries_accelerator_v1(chord: &str) -> bool {
    let lowered = chord.to_ascii_lowercase();
    let segments: Vec<&str> = lowered.split('+').collect();
    segments.split_last().map(|(_, modifiers)| modifiers.iter().any(|segment| matches!(*segment, "mod" | "ctrl" | "meta" | "cmd"))).unwrap_or(false)
}

/// 🛡️ The chords the shell's own chrome keeps out of the app-keybinding loop, after the user's
/// overrides — twin of React's `reservedShellChordsV1`.
pub(crate) fn reserved_shell_chords_v1(shell_table: &[(String, String)], overrides: &[(String, String)]) -> std::collections::BTreeSet<String> {
    let mut reserved = std::collections::BTreeSet::new();
    for (control_id, keys) in shell_table {
        let keys = overrides.iter().find(|(id, _)| id == control_id).map_or(keys.as_str(), |(_, keys)| keys.as_str());
        for chord in keys.split(',').map(|key| key.trim().to_ascii_lowercase()).filter(|key| !key.is_empty()) {
            if chord_carries_accelerator_v1(&chord) {
                reserved.insert(chord);
            }
        }
    }
    reserved
}

//#endregion ⌨️WindowScope
/// ⌨️ Whether a key event matches a keybinding chord such as `"mod+shift+z"`, `"ctrl+k"`, or `"escape"`.
/// `"mod"` is the platform accelerator (meta OR ctrl). Declared modifiers must be present and no
/// undeclared accelerator/shift/alt may be held, so `mod+z` never fires for `mod+shift+z`.
pub(crate) fn key_event_matches_chord(action: &ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers, chord: &str) -> bool {
    let mut want_mod = false;
    let mut want_shift = false;
    let mut want_alt = false;
    let mut want_ctrl = false;
    let mut want_meta = false;
    let mut key_token = String::new();
    for token in chord.split('+') {
        match token.trim().to_ascii_lowercase().as_str() {
            "" => {}
            "mod" => want_mod = true,
            "shift" => want_shift = true,
            "alt" | "option" => want_alt = true,
            "ctrl" | "control" => want_ctrl = true,
            "cmd" | "meta" | "super" | "win" => want_meta = true,
            other => key_token = other.to_string(),
        }
    }
    if key_token.is_empty() {
        return false;
    }
    if modifiers.shift != want_shift || modifiers.alt != want_alt {
        return false;
    }
    if want_mod {
        if want_ctrl || want_meta || modifiers.ctrl == modifiers.meta {
            return false;
        }
    } else if modifiers.ctrl != want_ctrl || modifiers.meta != want_meta {
        return false;
    }
    match action {
        ui_wgpu::wgpu::KeyAction::Char(c) => c.eq_ignore_ascii_case(&key_token),
        ui_wgpu::wgpu::KeyAction::Enter => key_token == "enter" || key_token == "return",
        ui_wgpu::wgpu::KeyAction::Escape => key_token == "escape" || key_token == "esc",
        ui_wgpu::wgpu::KeyAction::Backspace => key_token == "backspace",
        ui_wgpu::wgpu::KeyAction::Delete => key_token == "delete" || key_token == "del",
        ui_wgpu::wgpu::KeyAction::Tab => key_token == "tab",
        ui_wgpu::wgpu::KeyAction::ArrowLeft => key_token == "arrowleft" || key_token == "left",
        ui_wgpu::wgpu::KeyAction::ArrowRight => key_token == "arrowright" || key_token == "right",
        ui_wgpu::wgpu::KeyAction::ArrowUp => key_token == "arrowup" || key_token == "up",
        ui_wgpu::wgpu::KeyAction::ArrowDown => key_token == "arrowdown" || key_token == "down",
        ui_wgpu::wgpu::KeyAction::Home => key_token == "home",
        ui_wgpu::wgpu::KeyAction::End => key_token == "end",
        ui_wgpu::wgpu::KeyAction::PageUp => key_token == "pageup",
        ui_wgpu::wgpu::KeyAction::PageDown => key_token == "pagedown",
        ui_wgpu::wgpu::KeyAction::Function(number) => key_token == format!("f{number}"),
        ui_wgpu::wgpu::KeyAction::Space(_) => key_token == "space",
    }
}

impl DocumentState {
    // #endregion
    // #region command-registry
    /// 🔌️ The current session's program manifest (for its `commands: Vec<CommandDefinition>` — Plugin-scope
    /// commands apply whenever any of that plugin's apps is focused, mirroring `os-shell.tsx`'s
    /// `activePluginManifest`).
    pub(super) fn active_plugin_manifest(&self, plugins: &PluginState) -> Option<&semio_framework::PluginManifest> {
        let session = self.session.as_ref()?;
        plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).map(|entry| &entry.manifest)
    }

    /// 🎛️ Os-level built-in commands — the wgpu mirror of `os-shell.tsx`'s `buildOsCommands`, scoped to
    /// mutation paths that already exist in this shell: `appearance_id`/`driver_id`/`locale_id`/
    /// `terminology_id` (all wired through the existing `"framework"` controller `dispatch_action` switch
    /// — see `apply_os_command`), and the dock's `layout_override`
    /// (`os.resetDock`, applied locally like `RESET_DOCK` resets `dockLayoutStore`/`dockUiStateStore` in
    /// React — no plugin round-trip). Deliberately omits `os.introduceApp`/`os.setThemeId`/`os.setLayout`:
    /// none of the three has any persisted shell state yet (no introduction-playback step, no named
    /// `UiTheme` list, no desktop/tablet layout flag) — inventing that storage is out of this region's
    /// scope (`shell::ShellTypes` owns `ShellState`'s fields and is off-limits this wave).
    pub(crate) fn build_os_commands(&self, chrome: &ChromeState) -> Vec<semio_framework::CommandDefinition> {
        use semio_framework::manifest::Platform;
        use semio_framework::{ActionArgDef, ActionArgOption, ActionKind, CommandDefinition, PlatformKeybinding};
        let terminology_options: Vec<ActionArgOption> =
            self.active_terminologies().into_iter().map(|id| ActionArgOption { label: if id == "native" { LocalizedLabel::native("Native", "Nativ") } else { LocalizedLabel::data(id.clone()) }, value: id }).collect();
        vec![
            CommandDefinition::new("os.toggleFullscreen", LocalizedLabel::native("Toggle Full Screen", "Vollbild umschalten"), "window", "code", ActionKind::Shell)
                .with_keybinding(PlatformKeybinding::for_platform("f11", Platform::Windows))
                .with_keybinding(PlatformKeybinding::for_platform("f11", Platform::Linux))
                .with_keybinding(PlatformKeybinding::for_platform("control+meta+f", Platform::MacOs)),
            CommandDefinition::new("os.setAppearance", LocalizedLabel::native("Set Appearance", "Erscheinungsbild festlegen"), "appearance", "settings", ActionKind::Shell).with_args([ActionArgDef::select(
                "value",
                LocalizedLabel::native("Appearance", "Erscheinungsbild"),
                vec![
                    ActionArgOption { value: "system".into(), label: LocalizedLabel::native("System", "System") },
                    ActionArgOption { value: "light".into(), label: LocalizedLabel::native("Light", "Hell") },
                    ActionArgOption { value: "dark".into(), label: LocalizedLabel::native("Dark", "Dunkel") },
                ],
            )
            .required()]),
            CommandDefinition::new("os.setDriver", LocalizedLabel::native("Set Driver", "Treiber festlegen"), "layout", "settings", ActionKind::Shell).with_args([ActionArgDef::select(
                "value",
                LocalizedLabel::native("Driver", "Treiber"),
                vec![ActionArgOption { value: "default".into(), label: LocalizedLabel::native("Default", "Standard") }, ActionArgOption { value: "compact".into(), label: LocalizedLabel::native("Compact", "Kompakt") }],
            )
            .required()]),
            CommandDefinition::new("os.setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"), "language", "settings", ActionKind::Shell).with_args([ActionArgDef::select(
                "value",
                LocalizedLabel::native("Locale", "Sprache"),
                vec![ActionArgOption { value: "en".into(), label: LocalizedLabel::data("English") }, ActionArgOption { value: "de".into(), label: LocalizedLabel::data("Deutsch") }],
            )
            .required()]),
            CommandDefinition::new("os.setTerminology", LocalizedLabel::native("Set Terminology", "Terminologie festlegen"), "language", "settings", ActionKind::Shell).with_args([ActionArgDef::select(
                "value",
                LocalizedLabel::native("Terminology", "Terminologie"),
                terminology_options,
            )
            .required()]),
            CommandDefinition::new("os.setThemeId", LocalizedLabel::native("Set Theme", "Thema festlegen"), "appearance", "settings", ActionKind::Shell).with_args([ActionArgDef::select(
                "value",
                LocalizedLabel::native("Theme", "Design"),
                std::iter::once(ActionArgOption { value: "semio".into(), label: LocalizedLabel::data("Semio") })
                    .chain(std::iter::once(ActionArgOption { value: "mono".into(), label: LocalizedLabel::data("Mono") }))
                    .chain(chrome.chrome_build.preferences.custom_themes.keys().cloned().map(|id| {
                        let label = custom_theme_document_from(&chrome.chrome_build.preferences, &id).map(|theme| theme.label).unwrap_or_else(|| id.clone());
                        ActionArgOption { value: id, label: LocalizedLabel::data(label) }
                    }))
                    .collect(),
            )
            .required()]),
            CommandDefinition::new("os.openTaskManager", LocalizedLabel::native("Open Task Manager", "Task-Manager öffnen"), "window", "cpu", ActionKind::Shell),
            CommandDefinition::new("os.openHub", LocalizedLabel::native("Open Hub", "Hub öffnen"), "general", "users", ActionKind::Shell),
            CommandDefinition::new("os.resetDock", LocalizedLabel::native("Reset Dock Layout", "Dock-Layout zurücksetzen"), "layout", "panel-left", ActionKind::Shell),
        ]
    }

    /// 🎛️ Every command visible for the current session — os built-ins, the active plugin's Plugin-scope
    /// commands, and the app's App-/active-Mode-scope commands. The wgpu mirror of `os-shell.tsx`'s
    /// `resolveCommands(buildOsCommands(...), activePluginManifest, session?.app, activeModeId)` call site.
    pub(crate) fn resolved_commands(&self, chrome: &ChromeState, plugins: &PluginState) -> Vec<ResolvedCommand> {
        let os_commands = self.build_os_commands(chrome);
        let Some(session) = self.session.as_ref() else {
            return os_commands.into_iter().map(|definition| ResolvedCommand::new(definition, semio_framework::manifest::CommandOwnerAddress::Os)).collect();
        };
        let active_mode_id = session.view_state.active_mode_id.as_deref().unwrap_or(session.app.default_mode_id.as_str());
        resolve_commands(os_commands, self.active_plugin_manifest(plugins), &session.plugin_id, &session.app, active_mode_id)
    }

    /// 🔍️ Flattens `resolved_commands()` into quick-search-palette entries — EVERY in-palette command,
    /// whatever its owner, exactly as React's `searchItems` memo lists every `resolvedCommands` entry.
    ///
    /// 🩸️ This used to SKIP every arg-carrying Plugin/App/Mode-scope command on the premise that "there
    /// is no `ArtifactApp::handle_command` RPC wired on the plugin bridge yet (only `handle_action`
    /// exists)". That premise is stale: `ProgramBridgeEntry::handle_command` is wired on both the JS and
    /// the wasm backend (`🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`) and `dispatch_command` already drives
    /// it — so those commands were merely invisible, not unexecutable.
    ///
    /// Two shapes, both reachable:
    /// - zero args → fires immediately (os commands locally, everything else across the command boundary);
    /// - any command carrying args → React's single `…`-suffixed row that redirects to the
    ///   command's staged form (`command-form:`) rather than firing a guess.
    pub(crate) fn command_search_items(&self, chrome: &ChromeState, plugins: &PluginState, prefs: &PrefsState) -> Vec<SearchPaletteItem> {
        let mut items = Vec::new();
        for entry in self.resolved_commands(chrome, plugins) {
            let ResolvedCommand { definition, address } = entry;
            if !definition.in_palette {
                continue;
            }
            let category = address.owner.clone();
            let group = command_category_label(&definition.category);
            let is_os = matches!(address.owner, semio_framework::manifest::CommandOwnerAddress::Os);
            let platform = command_host_platform();
            let chords: Vec<String> = definition.keybindings.iter().filter(|binding| binding.platform.is_none_or(|declared| declared == platform)).map(|binding| format_keybinding_shortcut(&binding.chord)).collect();
            let description = (!chords.is_empty()).then(|| chords.join(","));
            let label = definition.label.resolve(prefs.active_terminology(), prefs.active_locale()).to_string();
            if definition.args.is_empty() {
                let invocation = semio_framework::manifest::CommandInvocation { address: address.clone(), arguments: BTreeMap::new() };
                items.push(SearchPaletteItem {
                    id: format!("command.{}", command_address_stable_key(&address).replace(':', ".")),
                    label,
                    description,
                    group,
                    dispatch_action: None,
                    action: Some(if is_os { format!("os-command:{}", definition.id) } else { format!("command:{}", semio_framework_pack_json::to_json_string(&invocation)) }),
                    category: Some(category.clone()),
                });
                continue;
            }
            items.push(SearchPaletteItem {
                id: format!("command.{}", command_address_stable_key(&address).replace(':', ".")),
                label: format!("{label}…"),
                description,
                group,
                dispatch_action: None,
                action: Some(format!("command-form:{}", command_address_stable_key(&address))),
                category: Some(category),
            });
        }
        items
    }

    /// 🚀️ Executes an os-level command by id (the wgpu mirror of `os-shell.tsx`'s `dispatchOsCommand`):
    /// `os.resetDock` clears the persisted layout override locally (no document round-trip, exactly like
    /// React's `RESET_DOCK` resetting `dockLayoutStore`/`dockUiStateStore`); every other command reuses the
    /// existing `"framework"` controller `dispatch_action` switch (`setAppearance`/`setDriver`/
    /// `setLocale`/`setTerminology`) that already backs the Settings panel's selects, so this
    /// never invents a new mutation path.
    /// 🗣️ The resolved locale, and what it actually resolves the chrome, the app's own window labels
    /// and the os command registry TO.
    ///
    /// This is the ONE runtime witness of a locale on a GPU canvas. Every translated string this
    /// target renders is painted into pixels: it is not in the retained document (which carries the
    /// user's own node names, identical in every tongue) and not in the DOM, so two lanes in a row
    /// could neither prove nor disprove a German session from outside the canvas
    /// (`📓️wgpu-status-a11y-i18n-2026-09-13.md` §5.3, ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    /// Printed once per RESOLUTION — a preferences load and a `setLocale` — never per frame.
    pub(crate) fn trace_resolved_locale(&self, reason: &str, chrome_state: &ChromeState, prefs: &PrefsState) {
        let is_de = prefs.locale_id == "de";
        let chrome = ["overlay.search.title", "overlay.find.title", "example.overlay.title", "common.cancel", "common.close", "common.focus", "common.loading", "nodeGraph.fitGraph", "contextMenu.goHome"]
            .iter()
            .map(|key| format!("{key}={}", shell_chrome_string(key, is_de)))
            .collect::<Vec<_>>();
        let windows = self.session.as_ref().map(|session| session.app.window_kinds.iter().map(|kind| format!("{}={}", kind.id, kind.label.resolve(prefs.active_terminology(), prefs.active_locale()))).collect::<Vec<_>>()).unwrap_or_default();
        let commands = self.build_os_commands(chrome_state).into_iter().map(|definition| format!("{}={}", definition.id, definition.label.resolve(prefs.active_terminology(), prefs.active_locale()))).collect::<Vec<_>>();
        debug_log(&format!("[TRACE] wgpu-shell locale resolved={} terminology={} reason={reason} chrome=[{}] windows=[{}] commands=[{}]", prefs.locale_id, prefs.terminology_id, chrome.join(", "), windows.join(", "), commands.join(", ")));
    }

    //#region ShellCommandHistory
    /// 🕒️ Looks up an `os.*` setting command's display label from `build_os_commands`' registry
    /// (`os.setAppearance`/`os.setDriver`/`os.setLocale`/`os.setTerminology`/`os.setThemeId`), falling
    /// back to the theme reset/delete buttons' own `shell_chrome_string` i18n labels for the two
    /// `build_settings_theme_ui` commands that never got a `CommandDefinition` (see that fn's own
    /// `os.setThemeId`-only doc comment) — the single label source `note_shell_command_action` call
    /// sites below draw from, so a chrome label edit never drifts from what the history panel shows.
    pub(crate) fn shell_command_label_for_setting(&self, command_id: &str, chrome: &ChromeState, prefs: &PrefsState) -> String {
        match command_id {
            "os.resetThemeId" => shell_chrome_string("settings.theme.reset", prefs.locale_id == "de").to_string(),
            "os.deleteThemeId" => shell_chrome_string("settings.theme.delete", prefs.locale_id == "de").to_string(),
            _ => self.build_os_commands(chrome).into_iter().find(|definition| definition.id == command_id).map(|definition| definition.label.resolve(prefs.active_terminology(), prefs.active_locale()).to_string()).unwrap_or_else(|| command_id.to_string()),
        }
    }

    pub(crate) fn arm_control_command(&mut self, control_id: &str, detail: Option<Value>, settle: &mut SettleState, plugins: &PluginState, prefs: &PrefsState) {
        let Some((command_id, label)) = shell_command_for_control(control_id, prefs.locale_id == "de") else {
            return;
        };
        let Some(controller_id) = self.shell_command_controller_id(plugins) else {
            return;
        };
        settle.deferred_actions.push(note_shell_command_action(&controller_id, command_id, &label, detail));
    }

    //#endregion ShellCommandHistory
    /// 🧾️ Every SHELL-OWNED panel leaf id this frame publishes a retained document for — the Command
    /// categories, the five Settings leaves, the Marketplace leaf, each mode tool's leaf and the chat
    /// transcript. Derived from `default_dock`'s own sources, so a leaf can never be dockable without
    /// a body or carry a body without being dockable.
    pub(crate) fn shell_owned_panel_leaves(&self, chrome: &ChromeState, plugins: &PluginState, prefs: &PrefsState) -> Vec<String> {
        let mut leaves: Vec<String> = self.command_category_tabs(chrome, plugins).into_iter().map(|tab| tab.id).collect();
        leaves.push(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID.to_string());
        leaves.push(FRAMEWORK_DISPLAY_LAYOUT_TAB_ID.to_string());
        leaves.extend(prefs.framework_settings_tabs().into_iter().map(|tab| tab.id));
        leaves.push(FRAMEWORK_MARKETPLACE_TAB_ID.to_string());
        leaves.push(FRAMEWORK_SYNC_PANEL_TAB_ID.to_string());
        leaves.push(FRAMEWORK_TASK_MANAGER_PANEL_ID.to_string());
        leaves.push(FRAMEWORK_CHAT_PANEL_ID.to_string());
        leaves.push(crate::hub_connection::FRAMEWORK_HUB_PANEL_ID.to_string());
        leaves.extend(self.tool_panel_tabs(prefs).into_iter().map(|tab| tab.id));
        leaves
    }

    /// 🎛️ ALL categories' command rows in one tree — the language-agnostic ORACLE the command-registry
    /// law asserts against. What the `bottom-middle` Command anchor actually publishes is
    /// {@link ShellState::build_command_category_ui}, one document per category leaf with the staged
    /// form for the expanded command; this function's value is that it enumerates every category and
    /// every row in one structure, which is exactly what a parity assertion needs and what a per-leaf
    /// body must never be.
    /// Every row for a command whose id already has a `"framework"` `dispatch_action` arm
    /// (appearance/driver/locale/terminology/themeId) is fully interactive; `os.resetDock` has no such
    /// arm to attach a plain `ActionDescriptor` to (only the ⌘️K search's `"os-command:"` string redirect
    /// can reach `apply_os_command` for it), so it renders as a pointer to command search instead of a
    /// non-functional button.
    #[cfg(test)]
    pub(crate) fn build_command_panel_ui(&self, chrome: &ChromeState, plugins: &PluginState, prefs: &PrefsState) -> UiNode {
        let resolved: Vec<_> = self.resolved_commands(chrome, plugins).into_iter().filter(|entry| entry.definition.in_palette).collect();
        let categories = command_categories(&resolved);
        let mut sections = Vec::new();
        for (category_id, category_label) in categories {
            let mut rows = Vec::new();
            rows.extend(resolved.iter().filter(|entry| entry.definition.category == category_id).map(|entry| prefs.build_command_panel_row(entry)));
            sections.push(UiTreeSectionNode { id: format!("shell.commands.category.{category_id}"), label: Some(Label::data(category_label)), default_open: None, header_toolbar: None, presence: UiPresence::default(), items: rows, window: None });
        }
        UiNode::Tree(UiTreeNode { presentation: Default::default(), sections, presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None })
    }
}

/// 🎛️ Merges os-built-in, Plugin-scope, App-scope, and active-Mode-scope commands into one list — the
/// wgpu mirror of `os-shell.tsx`'s `resolveCommands`. A `Mode`-scope command only resolves when
/// `active_mode_id`'s `ModeDefinition.commands` references it, exactly like the React source.
pub(crate) fn resolve_commands(os_commands: Vec<semio_framework::CommandDefinition>, plugin_manifest: Option<&semio_framework::PluginManifest>, plugin_id: &str, app: &AppDefinition, active_mode_id: &str) -> Vec<ResolvedCommand> {
    let mut resolved: Vec<ResolvedCommand> = os_commands.into_iter().map(|definition| ResolvedCommand::new(definition, semio_framework::manifest::CommandOwnerAddress::Os)).collect();
    if let Some(manifest) = plugin_manifest {
        resolved.extend(manifest.commands.iter().cloned().map(|definition| ResolvedCommand::new(definition, semio_framework::manifest::CommandOwnerAddress::Plugin { plugin_id: plugin_id.to_string() })));
    }
    resolved.extend(app.commands.iter().cloned().map(|definition| ResolvedCommand::new(definition, semio_framework::manifest::CommandOwnerAddress::App { plugin_id: plugin_id.to_string(), app_id: app.id.clone() })));
    if let Some(mode) = app.modes.iter().find(|mode| mode.id == active_mode_id) {
        resolved.extend(mode.commands.iter().cloned().map(|definition| ResolvedCommand::new(definition, semio_framework::manifest::CommandOwnerAddress::Mode { plugin_id: plugin_id.to_string(), app_id: app.id.clone(), mode_id: mode.id.clone() })));
    }
    resolved
}

/// 🎛️ Loose title-case for an open-set command category id (e.g. `"appearance"` -> `"Appearance"`,
/// `"named-layout"` -> `"Named Layout"`) — the wgpu mirror of `os-shell.tsx`'s `titleizeCommandCategory`.
/// wgpu has no `ui.settings.tab.*` translation table to special-case chrome-known ids against, so every
/// category (chrome-known or app/plugin-invented) goes through this uniformly.
pub(crate) fn command_category_label(category: &str) -> String {
    category
        .split(|c: char| c == '-' || c == '_')
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// 🎛️ Ordered, deduped `(category id, display label)` pairs derived from whatever commands actually
/// resolved — the wgpu mirror of `os-shell.tsx`'s `commandCategories`.
pub(crate) fn command_categories(commands: &[ResolvedCommand]) -> Vec<(String, String)> {
    let mut seen = std::collections::HashSet::new();
    let mut categories = Vec::new();
    for entry in commands {
        let category = entry.definition.category.as_str();
        if seen.insert(category.to_string()) {
            categories.push((category.to_string(), command_category_label(category)));
        }
    }
    categories
}

//#endregion ActionPanelAndUtilities

/// 🕒️ Builds a `noteShellCommand` `ActionDescriptor` — same inline-construction shape as
/// `map_action`/`scene_action`/`board_action` elsewhere in this crate, targeting the
/// framework-injected, session-only command-history tap (Shell-kind, intercepted before the app
/// ever sees it — see the plugin crate's universal command-recording mechanism). `detail` is the
/// caller-supplied JSON payload (e.g. `{"value": ...}`/`{"windowId": ...}`), left out of `args`
/// entirely when `None` rather than serialized as `null`.
pub(crate) fn note_shell_command_action(controller_id: &str, command_id: &str, label: &str, detail: Option<Value>) -> ActionDescriptor {
    let mut args = serde_json::json!({ "commandId": command_id, "label": label });
    if let Some(detail) = detail {
        args["detail"] = detail;
    }
    ActionDescriptor { controller_id: controller_id.to_string(), action: "noteShellCommand".into(), args: Some(semio_framework::DslValue::from(args)) }
}

/// 🕒️ Maps a chrome control id to its `noteShellCommand` `(commandId, label)` pair — factored out
/// of `handle_shell_hit`'s per-arm dispatch so the mapping is unit-testable without a full
/// `ShellState` fixture. Only control ids that represent a discrete, loggable user command are
/// covered; everything else is `None` (`handle_shell_hit` keeps its existing behavior either way).
///
/// 🈳️ A dock tab's FOCUS chip is deliberately absent. React's chip is
/// `dock.activateWindow(tab.id)` plus `dock.toggleMaximize(stackPath)`
/// (`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1076-1086`): the activation is journaled by
/// `handleActiveWindowChange`'s own `shell.windowActivate` note — which this shell arms through
/// [`Self::arm_window_activation_note`] — and the maximize is pure local state that React notes
/// NOWHERE (`🏛️ShellHost/🟦️.tsx` declares no `shell.windowMaximize` command at all). This renderer
/// used to note one, so `window-cap-focus` journaled a `noteShellCommand` React never emits
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `🗑️generated/w12c-parity-run-19/steps.json` step 16).
pub(crate) fn shell_command_for_control(control_id: &str, is_de: bool) -> Option<(&'static str, String)> {
    if control_id.starts_with("dock.tab.") && control_id.ends_with(".close") {
        return Some(("shell.windowClose", shell_chrome_string("common.close", is_de).to_string()));
    }
    if control_id == "ui.window.close" {
        return Some(("shell.windowClose", shell_chrome_string("common.close", is_de).to_string()));
    }
    if control_id.starts_with("shell.layout.") {
        return Some(("shell.applyNamedLayout", shell_chrome_string("command.applyLayout", is_de).to_string()));
    }
    if control_id == "ui.panelToggle.details" {
        return Some(("shell.panelToggle", shell_chrome_string("panelToggle.details", is_de).to_string()));
    }
    if control_id == "ui.panelToggle.settings" {
        return Some(("shell.panelToggle", shell_chrome_string("panelToggle.settings", is_de).to_string()));
    }
    if control_id == "ui.panelToggle.chat" {
        return Some(("shell.panelToggle", shell_chrome_string("panelToggle.chat", is_de).to_string()));
    }
    None
}
