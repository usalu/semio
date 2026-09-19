//! 🚪️ LAWS: packet W15f — the host/boot-door remainder `📓️audit-w14-transport-residual.md` left open
//! (its rows 6, 19, 10, 21, 9, 20). One law per item, each stated against React's own source:
//!
//! 1. `command_host_platform()` answers the ⌨️HostPlatform DOOR, never `web_sys::window()` (row 6);
//! 2. the framework-universal undo/redo chords are React's exact modifier gate and are not reserved,
//!    so an app can shadow them (row 19);
//! 3. OS-level file drag-and-drop exists on NEITHER renderer — parity by absence, pinned so React
//!    growing one turns this law red instead of leaving wgpu silently behind (row 10);
//! 4. `namedLayouts` round-trips through the `semio.os.config` document React's `NamedLayoutStore`
//!    owns, with React's own `origin: "user"` filter; `windowPanes` stays consumer-less on both sides
//!    (row 21);
//! 5. the introduction's persisted-seen key is BRAND-scoped, React's `introductionSeenKey` (row 9);
//! 6. every arg shape `command_search_items` emits has a live route in `activate_search_item`
//!    (row 20).
//!
//! Oracles: `🏛️ShellHost/🟦️.tsx` (`handleAppKeydown`'s `mod+z`/`mod+shift+z`/`mod+y` tail at its very
//! end, `introductionSeenKey`), `🧰️framework/🔨️modules/🖥️platform/🟦️.ts` (`NamedLayoutStore`,
//! `WindowPaneStateStore`), `📌️ChromePanels/🟦️.tsx` (`useNamedLayoutHost`), and
//! `🧑‍💻dev/🏷️brand/🟦️.ts` (`resolveShellBrandById`).

use super::*;

/// 🌳️ `…/🧑‍🎨engine` — the same derivation the sibling law files use.
fn engine_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..").canonicalize().expect("engine root")
}

fn read_engine(relative: &str) -> String {
    let path = engine_root().join(relative);
    std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

/// 🌳️ Every `🟦️.tsx` under `🧱️elements` — React's whole half of this renderer, which is where an
/// OS-level file-drop handler would have to live if one existed.
fn react_element_sources() -> Vec<(String, String)> {
    fn walk(directory: &std::path::Path, out: &mut Vec<(String, String)>) {
        let Ok(entries) = std::fs::read_dir(directory) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().is_some_and(|name| name == "node_modules") {
                    continue;
                }
                walk(&path, out);
            } else if path.extension().is_some_and(|extension| extension == "tsx") {
                if let Ok(source) = std::fs::read_to_string(&path) {
                    out.push((path.display().to_string(), source));
                }
            }
        }
    }
    let mut out = Vec::new();
    walk(&engine_root().join("🧱️elements"), &mut out);
    assert!(out.len() > 10, "the React element tree was found (got {} sources)", out.len());
    out
}

/// 🧾️ The body of one `fn <name>(` in a Rust source, up to its closing brace at column 0.
fn rust_fn_body(source: &str, signature: &str) -> String {
    let start = source.find(signature).unwrap_or_else(|| panic!("`{signature}` is declared")) + signature.len();
    let body = &source[start..];
    let end = body.find("\n}").unwrap_or_else(|| panic!("`{signature}` closes at column 0"));
    body[..end].to_string()
}

fn restore_boot_descriptor(descriptor: crate::WgpuBootDescriptor) {
    crate::apply_boot_descriptor(descriptor).expect("the captured descriptor was already valid");
}

// #region ⌨️CommandHostPlatform

/// ⚖️ LAW (item 2 / audit row 6): the shell has ONE answer to "what platform is this", and it comes
/// from the `⌨️HostPlatform` door the page publishes into — the same door `format_keybinding_shortcut`
/// reads through `host_platform_uses_meta()`.
///
/// 🩸️ The wasm32 arm of `command_host_platform()` used to re-derive the platform from
/// `web_sys::window()`. The wgpu shell runs inside the frame Worker, whose realm owns no `window`, so
/// that read was `None` on every machine, `unwrap_or_default()` was `""`, and the answer was always
/// `Linux` — while the chord FORMATTER, fixed by W7a at the same file, painted `⌘️`. A macOS browser
/// user was therefore shown and matched the `f11` arm of `os.toggleFullscreen` and of every
/// app-declared Mac-scoped keybinding.
#[test]
fn the_shell_reads_its_platform_from_the_host_platform_door() {
    let source = read_engine("🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs");
    let body = rust_fn_body(&source, "fn command_host_platform() -> semio_framework::manifest::Platform {");
    assert!(body.contains("crate::host_platform()"), "`command_host_platform` answers the door: {body}");
    assert!(!body.contains("web_sys::window()"), "and never re-derives the platform from a `window` the frame Worker realm does not own: {body}");

    let restore = crate::boot_descriptor();
    for (published, expected, uses_meta) in [
        ("MacIntel", semio_framework::manifest::Platform::MacOs, true),
        ("macOS", semio_framework::manifest::Platform::MacOs, true),
        ("iPhone", semio_framework::manifest::Platform::MacOs, true),
        ("Win32", semio_framework::manifest::Platform::Windows, false),
        ("Windows", semio_framework::manifest::Platform::Windows, false),
        ("Linux x86_64", semio_framework::manifest::Platform::Linux, false),
        ("", semio_framework::manifest::Platform::Linux, false),
    ] {
        crate::set_host_platform(published);
        assert_eq!(crate::host_platform(), expected, "⌨️ the door resolves {published:?} to {expected:?}");
        assert_eq!(crate::host_platform_uses_meta(), uses_meta, "⌨️ and the `mod` rule agrees with it for {published:?}");
        assert_eq!(command_host_platform(), expected, "⌨️ and the shell's keybinding filter reads exactly that");
    }

    // 🖥️ The concrete consequence the audit demonstrated: `os.toggleFullscreen` declares
    // `MacOs → control+meta+f` and `Windows`/`Linux → f11` (`build_os_commands`). The palette row's
    // description is the resolved chord list, so it must follow the door.
    let shell = ShellState::new(Vec::new(), String::new());
    let fullscreen_description = |shell: &ShellState| shell.command_search_items().into_iter().find(|item| item.id == "command.os:os.toggleFullscreen").and_then(|item| item.description).unwrap_or_default();
    crate::set_host_platform("MacIntel");
    let apple = fullscreen_description(&shell);
    crate::set_host_platform("Win32");
    let windows = fullscreen_description(&shell);
    assert_ne!(apple, windows, "⌨️ the palette describes the platform's OWN chord — {apple:?} on a Mac, {windows:?} on Windows");
    assert!(windows.to_ascii_lowercase().contains("f11"), "the non-Apple arm is F11: {windows:?}");
    assert!(!apple.to_ascii_lowercase().contains("f11"), "the Apple arm is not: {apple:?}");

    crate::set_host_platform_uses_meta(cfg!(target_os = "macos"));
    crate::set_host_platform_kind(if cfg!(target_os = "macos") {
        semio_framework::manifest::Platform::MacOs
    } else if cfg!(target_os = "windows") {
        semio_framework::manifest::Platform::Windows
    } else {
        semio_framework::manifest::Platform::Linux
    });
    restore_boot_descriptor(restore);
}
// #endregion ⌨️CommandHostPlatform

// #region ⏪️UndoRedoChords

/// ⚖️ LAW (item 5 / audit row 19): the framework-universal edit chords are React's exact gate —
/// `(ctrlKey || metaKey) && !altKey` on `z` (shift picks redo) and on `y` — and they are NOT reserved
/// shell chords, which is what lets an app that declares `mod+z` itself shadow them, exactly as
/// React's tail runs only after its app-keybinding loop `continue`s.
///
/// The audit listed this as still open ("absent in `handle_keyboard_async`"); it is not — W12c wired
/// `shell_edit_verb_for` into the tail of `handle_keyboard_async`. This law pins the two properties
/// the routing depends on so the row cannot regress into being true again.
#[test]
fn the_undo_redo_chords_match_reacts_gate_and_stay_shadowable() {
    let key = |character: char| ui_wgpu::wgpu::KeyAction::Char(character.to_string());
    let modifiers = |ctrl: bool, meta: bool, shift: bool, alt: bool| PointerModifiers { ctrl, meta, shift, alt };

    for (meta, ctrl) in [(true, false), (false, true)] {
        // ⏪️ React reads `event.ctrlKey || event.metaKey` — NOT the platform's `mod` — so both
        // accelerators answer on every platform.
        assert_eq!(shell_edit_verb_for(&key('z'), &modifiers(ctrl, meta, false, false)), Some(ShellEditVerb::Undo));
        assert_eq!(shell_edit_verb_for(&key('z'), &modifiers(ctrl, meta, true, false)), Some(ShellEditVerb::Redo), "shift picks redo");
        assert_eq!(shell_edit_verb_for(&key('y'), &modifiers(ctrl, meta, false, false)), Some(ShellEditVerb::Redo), "the `mod+y` redo alias");
    }
    assert_eq!(shell_edit_verb_for(&key('z'), &modifiers(false, false, false, false)), None, "a bare `z` is text, not undo");
    assert_eq!(shell_edit_verb_for(&key('z'), &modifiers(true, false, false, true)), None, "React's branch refuses the alt axis");
    assert_eq!(shell_edit_verb_for(&key('y'), &modifiers(true, false, true, false)), None, "and `mod+shift+y` is nobody's redo");
    assert_eq!(ShellEditVerb::Undo.action_id(), "undo");
    assert_eq!(ShellEditVerb::Redo.action_id(), "redo");

    // 🛡️ Never reserved: React dispatches this tail AFTER the app-keybinding loop, so an app that
    // declares `mod+z` wins. A reserved chord would win here instead and the app's binding would die.
    let shell = ShellState::new(Vec::new(), String::new());
    let table = shell.shortcut_table();
    for chord in ["mod+z", "mod+shift+z", "mod+y"] {
        assert!(!reserved_shell_chords_v1(&table, &[]).contains(chord), "⏪️ `{chord}` is not a reserved shell chord — an app must be able to shadow it");
    }
}
// #endregion ⏪️UndoRedoChords

// #region 📥️OsFileDrop

/// ⚖️ LAW (item 6 / audit row 10): OS-level file drag-and-drop onto the canvas exists on NEITHER
/// renderer. The audit filed it as a wgpu gap on the strength of a `DragEvent` import in the ui React
/// target and admitted (its §4) that React's handler was never traced. It was traced for this packet:
/// every `drop`/`dragover` handler in the React tree reads an in-app `dataTransfer` MIME
/// (`CATALOGUE_DRAG_MIME`, `PANEL_TREE_UNIT_MIME`, `PALETTE_DRAG_MIME`,
/// `COMPOSE_WINDOW_TEMPLATE_MIME`, the tree-unit/reorder mimes) — not one reads `dataTransfer.files`.
/// The only file-open door on either renderer is the picker.
///
/// So the parity-correct action is NOT to build one on wgpu alone — that is the divergence `📓️w1d`
/// refused for `?mode=` — but to pin the absence on BOTH sides. React growing an OS file drop turns
/// this law red, which is the moment the wgpu port becomes owed.
#[test]
fn neither_renderer_accepts_an_os_file_drop() {
    let mut react_file_drops = Vec::new();
    for (path, source) in react_element_sources() {
        for needle in ["dataTransfer.files", "dataTransfer.items", "dataTransfer?.files"] {
            if source.contains(needle) {
                react_file_drops.push(format!("{path}: {needle}"));
            }
        }
    }
    assert!(
        react_file_drops.is_empty(),
        "📥️ React now reads dropped FILES ({react_file_drops:?}) — wgpu owes the twin: page-side `dragover`/`drop` listeners in 🚀️browser-boot/🟦️.ts feeding the existing `request-file-open` host-io door"
    );

    let trunk = read_engine("🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts");
    for listener in ["\"drop\"", "\"dragover\"", "\"dragenter\"", "\"dragleave\""] {
        assert!(!trunk.contains(&format!("addEventListener({listener}")), "📥️ the wgpu trunk page registers {listener} — if React now has an OS file drop this is right, and this law needs rewriting into the positive form");
    }
    // 📤️ What DOES exist on both sides, so the absence above is about the OS drop and nothing else.
    let host_io = read_engine("🎯️targets/🧊️wgpu/🚪️host-io/🟦️.ts");
    assert!(host_io.contains("\"request-file-open\""), "📤️ the picker-based open door is the one that exists");
    assert!(host_io.contains("\"download-media-export\""), "⬇️ and the download door beside it");
}
// #endregion 📥️OsFileDrop

// #region 🖥️NamedLayoutPersistence

/// ⚖️ LAW (item 7 / audit row 21): a SAVED layout round-trips through the one `semio.os.config`
/// document React's `NamedLayoutStore` owns — React's flat `namedLayouts[<appId>]` record (not the
/// `{os, apps}` layer shape the dock projections use), React's `"framework-os"` fallback key, and
/// React's `origin: "user"` read filter.
///
/// 🩸️ W5a made the whole document round-trip on disk, so a layout saved in the React shell survived a
/// switch to wgpu and was then IGNORED: no wgpu lane read the projection.
#[test]
fn saved_named_layouts_round_trip_through_reacts_own_document() {
    let layout = ui_wgpu::wgpu::create_stack_layout(&["main".to_string()], None);
    let user = ui_wgpu::wgpu::NamedLayout { id: "user-1".into(), label: "Mine".into(), icon_id: None, layout: layout.clone(), origin: "user".into(), group_path: None };
    let builtin = ui_wgpu::wgpu::NamedLayout { id: "builtin-1".into(), label: "Theirs".into(), icon_id: None, layout, origin: "builtin".into(), group_path: None };

    let mut config = empty_os_shell_config();
    write_named_layouts_in(&mut config, Some("puzzle3d"), std::slice::from_ref(&user));
    assert!(config["namedLayouts"]["puzzle3d"].is_array(), "🖥️ filed under React's flat per-app key, never a `{{os, apps}}` layer: {config}");
    assert_eq!(read_named_layouts(&config, Some("puzzle3d")), vec![user.clone()], "and read back whole");
    assert!(read_named_layouts(&config, Some("other-app")).is_empty(), "another app's layer is another app's");

    // 🖥️ React's `NamedLayoutStore` has no app id of its own when there is no session: `session?.app.id
    // ?? "framework-os"`.
    let mut sessionless = empty_os_shell_config();
    write_named_layouts_in(&mut sessionless, None, std::slice::from_ref(&user));
    assert!(sessionless["namedLayouts"][NAMED_LAYOUT_STORE_FALLBACK_APP_ID].is_array(), "🖥️ the sessionless layer is React's `framework-os`: {sessionless}");

    // 🖥️ React's `readPersisted` filter: `entry.origin === "user"`. A builtin written into the document
    // must never come back and shadow the app's own declaration.
    let mut mixed = empty_os_shell_config();
    write_named_layouts_in(&mut mixed, Some("puzzle3d"), &[user.clone(), builtin]);
    assert_eq!(read_named_layouts(&mixed, Some("puzzle3d")), vec![user], "🖥️ only `origin: \"user\"` rows survive the read");

    // 🖥️ The sibling projections are untouched by a layout write — the same whole-document rewrite rule
    // W5a pinned for the dock layers.
    assert!(mixed["dockLayouts"]["apps"].is_object() && mixed["windowPanes"]["apps"].is_object(), "🗄️ the sibling projections survive: {mixed}");
}

/// ⚖️ LAW (item 7, second half): `windowPanes` is carried and consumed by NEITHER renderer.
/// `WindowPaneStateStore` (`🧰️framework/🔨️modules/🖥️platform/🟦️.ts`) has no production caller in React
/// either — only its own unit suite — so giving wgpu a reader would invent behaviour React does not
/// have. The projection stays carried, and this law says where the port becomes owed.
#[test]
fn window_panes_stay_consumer_less_on_both_renderers() {
    let react_consumers: Vec<String> = react_element_sources().into_iter().filter(|(_, source)| source.contains("WindowPaneStateStore")).map(|(path, _)| path).collect();
    assert!(
        react_consumers.is_empty(),
        "🪟️ React now mounts `WindowPaneStateStore` ({react_consumers:?}) — wgpu owes the twin reader beside `load_persisted_named_layouts`, over the `windowPanes` layer `read_os_shell_config_layer` already understands"
    );
    // 🗄️ Carried regardless, so the document a React shell writes survives a wgpu write untouched.
    assert!(empty_os_shell_config()["windowPanes"]["apps"].is_object(), "🗄️ the projection is still carried");
}
// #endregion 🖥️NamedLayoutPersistence

// #region 🏷️BrandRegistry

/// ⚖️ LAW (item 8 / audit row 9): the introduction's device-local seen flag is filed under React's
/// `introductionSeenKey` — `"<brandId>:<appId>"` on a branded shell, the bare app id otherwise — and a
/// brand that replays its introduction writes no flag at all.
///
/// 🩸️ The wgpu shell carried a brand ID and nothing else (`📓️w1d` gap 1), so on `aggregator` it wrote
/// `ui.introduction.seen.<appId>` where React writes `ui.introduction.seen.<brandId>:<appId>` — the one
/// concretely demonstrated brand divergence (`📓️w5a` §7.1). The brand REGISTRY stays TypeScript; what
/// crosses is the resolved row (`WgpuBootBrand`), exactly as `locks`/`defaults` already do.
#[test]
fn the_introduction_seen_key_is_brand_scoped() {
    let restore = crate::boot_descriptor();
    let shell = appearance_tour_and_footer_pill_tests::tour_shell(Some(appearance_tour_and_footer_pill_tests::tour_introduction()));
    let app_id = shell.session.as_ref().expect("the fixture opens a session").app.id.clone();

    let mut unbranded = crate::boot_descriptor();
    unbranded.brand_id = String::new();
    unbranded.brand = crate::WgpuBootBrand::default();
    crate::apply_boot_descriptor(unbranded).expect("an empty brand is a bounded field");
    assert_eq!(shell.introduction_seen_key().as_deref(), Some(app_id.as_str()), "🎓️ an unbranded shell keys the flag on the bare app id");
    assert!(crate::boot_brand().persists_introduction_seen(), "🏷️ and persists it, React's `shouldPersistIntroductionSeen(undefined)`");
    assert!(!crate::boot_brand().replays_introduction());

    let mut branded = crate::boot_descriptor();
    branded.brand_id = "entwerfen-mit-bestand-aggregator".into();
    crate::apply_boot_descriptor(branded).expect("a brand id is a bounded field");
    assert_eq!(shell.introduction_seen_key().as_deref(), Some(format!("entwerfen-mit-bestand-aggregator:{app_id}").as_str()), "🏷️ a branded shell carries React's `${{brand.id}}:${{app.id}}`");

    // 🏷️ The two brand predicates, term for term with `shouldReplayIntroductionOnLoad` /
    // `shouldPersistIntroductionSeen` (`🧱️elements/🐚️Shell/🟦️.tsx`): `ephemeral` implies replay.
    for (ephemeral, replay_flag, replays) in [(false, false, false), (false, true, true), (true, false, true), (true, true, true)] {
        let brand = crate::WgpuBootBrand { window_title: String::new(), ephemeral, replay_introduction_on_load: replay_flag };
        assert_eq!(brand.replays_introduction(), replays, "🏷️ ephemeral={ephemeral} replayIntroductionOnLoad={replay_flag}");
        assert_eq!(brand.persists_introduction_seen(), !replays, "🏷️ and persistence is its exact negation");
    }

    restore_boot_descriptor(restore);
}
// #endregion 🏷️BrandRegistry

// #region 🎛️PaletteCommandRoutes

/// ⚖️ LAW (item 9 / audit row 20): every action string `command_search_items` emits has a live arm in
/// `activate_search_item`. The audit listed arg-carrying Plugin/App/Mode palette commands as still
/// dead ("listed for completeness"); they are not — an earlier wave replaced that premise and wired
/// all three shapes. This law pins the two halves TOGETHER, so a new shape cannot be listed without a
/// route (a live-looking row that silently no-ops) nor a route dropped from under a listed row.
#[test]
fn every_palette_command_shape_has_a_live_route() {
    let source = read_engine("🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs");
    let routes = rust_fn_body(&source, "pub async fn activate_search_item(&mut self, index: usize) -> Result<(), String> {");
    for prefix in ["os-command:", "command:", "command-form:"] {
        assert!(routes.contains(&format!("strip_prefix(\"{prefix}\")")), "🎛️ `{prefix}` is routed in `activate_search_item`");
    }

    let shell = ShellState::new(Vec::new(), String::new());
    let items = shell.command_search_items();
    assert!(!items.is_empty(), "🎛️ the os built-ins alone fill the palette");
    for item in &items {
        let action = item.action.as_deref().unwrap_or_default();
        assert!(
            action.starts_with("os-command:") || action.starts_with("command:") || action.starts_with("command-form:"),
            "🎛️ palette row `{}` carries an action shape `activate_search_item` can route, got {action:?}",
            item.id
        );
    }
}
// #endregion 🎛️PaletteCommandRoutes
