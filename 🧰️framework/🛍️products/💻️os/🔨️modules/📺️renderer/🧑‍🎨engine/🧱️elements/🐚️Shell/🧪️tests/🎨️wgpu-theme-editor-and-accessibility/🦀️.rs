//! 🎨️♿️ Laws for ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W15d — the settings panel's DEPTH
//! (the theme editor's eight sections, General/Theme completeness) and control ACCESSIBILITY
//! (accessible names, inline hotkey badges, keybinding-registry drift). Audit `📓️audit-w14-shell-
//! residual.md` items §B8, §B2, §B3, §C4, §B14 and the §E keybindings table.
//!
//! Each law names the React source it mirrors. The React sources are
//! `📌️ChromePanels/🟦️.tsx`'s `buildSettingsThemeTree`/`buildSettingsGeneralTree`,
//! `🏛️ShellHost/🟦️.tsx`'s theme mutators/`exportTheme`/`importTheme`, and
//! `🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx`'s `SHELL_KEYBINDINGS`.

use super::*;

/// 🎨️ The control ids one theme-editor build offers, in build order — the wgpu twin of reading
/// `TreeDataItem.id` off React's rendered tree.
fn theme_editor_control_ids(shell: &ShellState) -> Vec<String> {
    fn walk(node: &UiNode, out: &mut Vec<String>) {
        match node {
            UiNode::Button(button) => out.extend(button.id.clone()),
            UiNode::Select(select) => out.push(select.id.clone()),
            UiNode::Input(input) => out.push(input.id.clone()),
            UiNode::Field(field) => walk(&field.child, out),
            UiNode::Stack(stack) => stack.children.iter().for_each(|child| walk(child, out)),
            UiNode::Section(section) => section.children.iter().for_each(|child| walk(child, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    walk(&shell.build_settings_theme_ui(), &mut out);
    out
}

/// 🎨️ A shell with one theme-editor section open, at page 0.
fn shell_with_open_theme_section(section: &str) -> ShellState {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.theme_editor_open_section = Some(section.to_string());
    shell.theme_editor_page = 0;
    shell
}

/// ⚖️ LAW (§B8 base): the embedded authored token document parses, and carries exactly the eight
/// key groups React's `buildSettingsThemeTree` enumerates (`host.theme.colors` … `.appearances`).
/// The counts are the authored file's own — a drift in `🔣️.json` is a drift in BOTH editors.
#[test]
fn the_embedded_theme_document_carries_every_section_react_enumerates() {
    let document = shell_theme_document_base();
    assert_eq!(document.id, "semio", "🎨️ the base document is the `semio` theme React's `STYLING_SEMIO_THEME` also stamps");
    assert!(!document.colors.is_empty(), "🎨️ colors");
    assert!(!document.spacing.is_empty(), "🎨️ spacing");
    assert!(!document.font_stacks.is_empty(), "🎨️ fontStacks");
    assert!(!document.strokes.is_empty(), "🎨️ strokes");
    assert!(!document.radii.is_empty(), "🎨️ radii");
    assert!(!document.opacities.is_empty(), "🎨️ opacities");
    assert!(!document.metrics.is_empty(), "🎨️ metrics");
    for appearance in ["light", "dark"] {
        let groups = document.appearances.get(appearance).unwrap_or_else(|| panic!("🎨️ the {appearance} appearance"));
        for (group, _) in SHELL_THEME_APPEARANCE_GROUPS {
            assert!(groups.contains_key(*group), "🎨️ React's editor lists the `{group}` group of the {appearance} appearance");
        }
    }
}

/// ⚖️ LAW (§B8 colors): the colors section's rows carry React's `framework.settings.theme.colors.<key>`
/// ids, in ascending key order, and the section header itself is a real control (React's collapsible
/// `TreeDataSection`, which a retained `Section` container cannot be on this target).
#[test]
fn the_colors_section_mints_reacts_row_ids_in_ascending_key_order() {
    let shell = shell_with_open_theme_section("framework.settings.theme.colors");
    let ids = theme_editor_control_ids(&shell);
    assert!(ids.contains(&"framework.settings.theme.colors".to_string()), "🎨️ the section header is its own control");
    let document = shell_theme_document_base();
    let expected: Vec<String> = document.colors.keys().take(SHELL_THEME_EDITOR_PAGE_ROWS).map(|key| format!("framework.settings.theme.colors.{key}")).collect();
    let rows: Vec<String> = ids.iter().filter(|id| id.starts_with("framework.settings.theme.colors.")).cloned().collect();
    assert_eq!(rows, expected, "🎨️ page 0 is React's sorted `Object.keys(host.theme.colors)`, row id for row id");
}

/// ⚖️ LAW (§B8 spacing/fonts): both string-valued sections mint React's row ids off their own maps —
/// note React's `fonts` SECTION id over the document's `fontStacks` map, which this mirrors exactly.
#[test]
fn the_spacing_and_fonts_sections_mint_reacts_row_ids() {
    let document = shell_theme_document_base();
    for (section, keys) in [("spacing", document.spacing.keys()), ("fonts", document.font_stacks.keys())] {
        let shell = shell_with_open_theme_section(&format!("framework.settings.theme.{section}"));
        let ids = theme_editor_control_ids(&shell);
        for key in keys {
            let id = format!("framework.settings.theme.{section}.{key}");
            assert!(ids.contains(&id), "🎨️ {id} is one of React's rows");
        }
    }
}

/// ⚖️ LAW (§B8 strokes/radii/opacities): the three number-valued sections mint React's row ids, and a
/// row prints its value through React's own `number | number[]` rule (`value.join(", ")`).
#[test]
fn the_number_sections_mint_reacts_row_ids_and_number_text() {
    let document = shell_theme_document_base();
    for section in ["strokes", "radii", "opacities"] {
        let shell = shell_with_open_theme_section(&format!("framework.settings.theme.{section}"));
        let ids = theme_editor_control_ids(&shell);
        let map = match section {
            "strokes" => &document.strokes,
            "radii" => &document.radii,
            _ => &document.opacities,
        };
        let first = map.keys().next().expect("🎨️ every number section has rows");
        assert!(ids.contains(&format!("framework.settings.theme.{section}.{first}")), "🎨️ {section}.{first}");
    }
    assert_eq!(ThemeNumber::Scalar(3.0).as_text(), "3", "🔢️ JavaScript's `String(3)` prints no trailing `.0`");
    assert_eq!(ThemeNumber::List(vec![2.0, 4.5]).as_text(), "2, 4.5", "🔢️ React's `value.join(\", \")`");
    assert_eq!(ThemeNumber::parse("2, 4.5"), Some(ThemeNumber::List(vec![2.0, 4.5])), "🔢️ a comma makes it a list");
    assert_eq!(ThemeNumber::parse("not a number"), None, "🔢️ an unparseable buffer commits NOTHING, as React's `onBlur` returns early");
}

/// ⚖️ LAW (§B8 metrics): the metrics section is React's two-level `metricSections` — one collapsible
/// per section, whose rows carry `framework.settings.theme.metrics.<section>.<key>` and name BOTH
/// coordinates in their commit args.
#[test]
fn the_metrics_section_is_two_levels_and_names_both_coordinates() {
    let document = shell_theme_document_base();
    let section = document.metrics.keys().next().expect("🎨️ the document authors metric sections").clone();
    let closed = shell_with_open_theme_section("framework.settings.theme.metrics");
    let closed_ids = theme_editor_control_ids(&closed);
    assert!(closed_ids.contains(&format!("framework.settings.theme.metrics.{section}")), "🎨️ each metric section is its own collapsible");
    let open = shell_with_open_theme_section(&format!("framework.settings.theme.metrics.{section}"));
    let open_ids = theme_editor_control_ids(&open);
    let key = document.metrics[&section].keys().next().expect("🎨️ a metric section has entries");
    assert!(open_ids.contains(&format!("framework.settings.theme.metrics.{section}.{key}")), "🎨️ the open section shows its rows");
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.apply_theme_editor_commit("setThemeMetric", &serde_json::json!({ "section": section, "key": key, "value": "42" }));
    assert_eq!(shell.theme_document().metrics[&section][key], ThemeNumber::Scalar(42.0), "🎨️ React's `setThemeMetric(section, key, value)`");
}

/// ⚖️ LAW (§B8 appearances): per-appearance paints are React's three-level tree (appearance → group →
/// paint), each paint offering the resolved hex AND the `<paint>.alpha` box React's
/// `buildThemeAppearanceGroupItems` puts beside it.
#[test]
fn the_appearance_section_offers_reacts_paint_and_alpha_rows() {
    let appearance_open = shell_with_open_theme_section("framework.settings.theme.appearances.light");
    let ids = theme_editor_control_ids(&appearance_open);
    for (group, _) in SHELL_THEME_APPEARANCE_GROUPS {
        assert!(ids.contains(&format!("framework.settings.theme.appearances.light.{group}")), "🎨️ React lists the {group} group");
    }
    let group_open = shell_with_open_theme_section("framework.settings.theme.appearances.light.chrome");
    let ids = theme_editor_control_ids(&group_open);
    assert!(ids.contains(&"framework.settings.theme.appearances.light.chrome.base".to_string()), "🖌️ the paint row");
    assert!(ids.contains(&"framework.settings.theme.appearances.light.chrome.base.alpha".to_string()), "🖌️ and React's alpha box beside it");
    let document = shell_theme_document_base();
    let (hex, alpha) = theme_paint_editor_values(&document.colors, &document.appearances["light"]["chrome"]["base"]);
    assert!(hex.starts_with('#') && hex.len() == 7, "🖌️ a token ref shows its RESOLVED `#rrggbb`, as React's `rgba8ToHex` does");
    assert!((alpha - 1.0).abs() < 1e-6, "🖌️ …and its resolved alpha as `rgba[3] / 255`");
}

/// ⚖️ LAW (§B8 live re-tokenisation): editing the document moves the painted `Theme`. A primitive
/// colour moves every paint that references it; `chrome.base` also moves the six formula-derived
/// level surfaces (navbar/panel/menu), which is what `oklabMix(base, foreground, k · shadeStep)`
/// means at runtime.
#[test]
fn editing_the_document_re_tokenises_the_painted_theme() {
    let base = shell_theme_document_base().clone();
    let painted = theme_from_document(&base, false);
    assert_eq!(painted.accent, Theme::light().accent, "🎨️ an unedited document paints the built-in theme");

    let mut edited = base.clone();
    edited.colors.insert("primary".to_string(), "#00ff00".to_string());
    let accented = theme_from_document(&edited, false);
    assert_ne!(accented.accent, painted.accent, "🎨️ `chrome.accent` is `{{token: primary}}` — editing the token moves the paint");

    let mut surfaced = base.clone();
    surfaced.appearances.get_mut("light").expect("light").get_mut("chrome").expect("chrome").insert("base".to_string(), ThemePaintRef { hex: Some("#123456".to_string()), ..ThemePaintRef::default() });
    let resurfaced = theme_from_document(&surfaced, false);
    assert_ne!(resurfaced.background, painted.background, "🪜️ the page background follows `chrome.base`");
    assert_ne!(resurfaced.navbar, painted.navbar, "🪜️ …and so does the navbar, through the recomputed level ramp");
    assert_ne!(resurfaced.panel, painted.panel, "🪜️ …and the panel surface");

    let mut metric = base.clone();
    metric.metrics.get_mut("chrome").expect("chrome metrics").insert("navbarHeightUiSpacing".to_string(), ThemeNumber::Scalar(20.0));
    assert!(theme_from_document(&metric, false).navbar_height > painted.navbar_height, "📐️ a metric edit re-measures the chrome");
}

/// ⚖️ LAW (§B8 bound): the theme leaf publishes as a retained document at every open section —
/// React's DOM tree has no node ceiling, `ui_contract::UI_DOCUMENT_NODES` is 128, and the pager is
/// what keeps the panel publishable instead of faulting.
#[test]
fn every_open_theme_section_publishes_within_the_retained_document_ceiling() {
    let mut sections: Vec<String> = SHELL_THEME_EDITOR_SECTIONS.iter().map(|(suffix, _)| format!("framework.settings.theme.{suffix}")).collect();
    sections.push("framework.settings.theme.metrics.board".to_string());
    sections.push("framework.settings.theme.appearances.light".to_string());
    sections.push("framework.settings.theme.appearances.light.board".to_string());
    for section in sections {
        let shell = shell_with_open_theme_section(&section);
        let records = panel_ui_records(FRAMEWORK_SETTINGS_THEME_TAB_ID, &shell.build_settings_theme_ui())
            .unwrap_or_else(|error| panic!("🎨️ the theme leaf publishes with `{section}` open: {error}"));
        assert!(records.len() <= ui_contract::UI_DOCUMENT_NODES, "🎨️ `{section}` publishes {} records over the {} ceiling", records.len(), ui_contract::UI_DOCUMENT_NODES);
    }
}

/// ⚖️ LAW (§B2): the General leaf carries React's `framework.settings.mergePolicy` selector, with
/// React's own three option values.
#[test]
fn the_general_leaf_offers_reacts_merge_policy_selector() {
    let shell = ShellState::new(Vec::new(), String::new());
    let mut selects = Vec::new();
    fn walk(node: &UiNode, out: &mut Vec<(String, Vec<String>)>) {
        match node {
            UiNode::Select(select) => out.push((select.id.clone(), select.items.iter().map(|item| item.value.clone()).collect())),
            UiNode::Field(field) => walk(&field.child, out),
            UiNode::Stack(stack) => stack.children.iter().for_each(|child| walk(child, out)),
            UiNode::Section(section) => section.children.iter().for_each(|child| walk(child, out)),
            _ => {}
        }
    }
    walk(&shell.build_settings_general_ui(), &mut selects);
    let (_, options) = selects.iter().find(|(id, _)| id == "framework.settings.mergePolicy").expect("⚖️ React's merge-policy row has a wgpu twin");
    assert_eq!(options, &vec!["LaissezFaire".to_string(), "Normal".to_string(), "Vigilant".to_string()], "⚖️ React's `MERGE_POLICY_OPTIONS`, verbatim");
    assert_eq!(shell.merge_policy, SHELL_DEFAULT_MERGE_POLICY, "⚖️ a fresh authority is `Normal`, `protocol::MergePolicy::default()`");
}

/// ⚖️ LAW (§B3): the theme leaf carries React's export/import pair and its save box, and an exported
/// document re-parses — the `.theme.dsl` text React's `serializeUiTheme`/`parseUiTheme` round-trip.
#[test]
fn the_theme_leaf_offers_reacts_export_import_and_save_controls() {
    let shell = ShellState::new(Vec::new(), String::new());
    let ids = theme_editor_control_ids(&shell);
    for id in ["framework.settings.theme.select", "framework.settings.theme.saveLabel", "framework.settings.theme.save", "framework.settings.theme.reset", "framework.settings.theme.export", "framework.settings.theme.import"] {
        assert!(ids.contains(&id.to_string()), "🎨️ {id} is one of React's theme-leaf controls");
    }
    let document = shell.theme_document();
    let text = serde_json::to_string_pretty(&document).expect("💾️ the canonical 2-space JSON React's `serializeUiTheme` writes");
    let parsed: ThemeDocument = serde_json::from_str(&text).expect("💾️ …and `parseUiTheme` reads back");
    assert_eq!(parsed, document, "💾️ export/import is lossless, so a theme crosses between renderers");
    assert_eq!(custom_theme_id_for_label("My Nice Theme"), Some("custom.my-nice-theme".to_string()), "💾️ React's `saveTheme` slug");
    assert_eq!(custom_theme_id_for_label("   "), None, "💾️ a blank name saves nothing, as React returns early");
}

/// ⚖️ LAW (§B8 dispatch): every editor verb writes the SAME document, and the row's authored
/// coordinates survive the commit — the merge that also restores the Default Apps leaf's own `row`
/// argument.
#[test]
fn an_editor_commit_keeps_the_rows_authored_coordinates() {
    let merged = merge_committed_args(Some(&DslValue::Object(vec![("key".to_string(), DslValue::String("primary".to_string()))])), DslValue::String("#00ff00".to_string()));
    let json = dsl_value_as_json(&merged.expect("🔀️ a merged argument object"));
    assert_eq!(json["key"], "primary", "🔀️ the node's authored coordinate survives");
    assert_eq!(json["value"], "#00ff00", "🔀️ …beside the committed value");

    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.apply_theme_editor_commit("setThemeColor", &serde_json::json!({ "key": "primary", "value": "#00ff00" }));
    assert_eq!(shell.theme_document().colors["primary"], "#00ff00", "🎨️ React's `setThemeColor`");
    shell.apply_theme_editor_commit("setThemeAppearancePaint", &serde_json::json!({ "appearance": "light", "group": "chrome", "paint": "base", "channel": "hex", "value": "#101112" }));
    let reference = shell.theme_document().appearances["light"]["chrome"]["base"].clone();
    assert_eq!(reference.hex.as_deref(), Some("#101112"), "🖌️ React's `setThemeAppearancePaint` replaces the ref outright");
    assert!(reference.alpha.is_some(), "🖌️ …carrying the row's current alpha, as React's colour input does");
}

/// ⚖️ LAW (§C4): the shell CHROME publishes accessible names — the production path that gives a GPU
/// canvas's navbar/footer/panel chips ARIA elements through `🚀️browser-boot/🟦️.ts`'s mirror. Names
/// are the painted text; a control with a chord also publishes its `aria-keyshortcuts`.
#[test]
fn chrome_controls_publish_accessible_names_and_shortcuts() {
    let shell = ShellState::new(Vec::new(), String::new());
    with_chrome_control_names(|names| {
        names.clear();
    });
    note_chrome_control_name("playground.navbar.roles.editor", Some("Editor"));
    note_chrome_control_name("", Some("an id-less chip is not a control"));
    let hits = vec![
        HitTarget { rect: Rect::new(1.0, 2.0, 3.0, 4.0), event: None, control_id: Some("playground.navbar.roles.editor".to_string()), kind: HitKind::NavbarItem, drag_axis: None, drag_data: None },
        HitTarget { rect: Rect::new(0.0, 0.0, 1.0, 1.0), event: None, control_id: Some("framework.panelTab.settings.general".to_string()), kind: HitKind::PanelTab, drag_axis: None, drag_data: None },
    ];
    let nodes = shell.chrome_accessibility_nodes(&hits);
    assert_eq!(nodes.len(), 2, "♿️ every registered chrome target is announced");
    assert_eq!(nodes[0].label.as_deref(), Some("Editor"), "♿️ the announced name is the text the chip painted");
    assert_eq!(nodes[0].role, "button", "♿️ a navbar item announces as a button, as React's element does");
    assert_eq!(nodes[0].shortcut.as_deref(), Some(format_keybinding_shortcut("mod+alt+e").as_str()), "⌨️ `aria-keyshortcuts` comes from this session's own remappable table");
    assert_eq!(nodes[1].role, "tab", "♿️ a panel tab announces as a tab");
    assert!(nodes[1].label.is_some(), "♿️ a control that painted no label still gets a name rather than none");
    assert!(nodes.iter().all(|node| node.actionable && node.focusable), "♿️ a chrome target is always actionable");
}

/// ⚖️ LAW (§B14): a bound chrome control carries its chord INLINE, like React's `ControlHotkeyBadge`
/// beside any bound control's own text — and appending it twice is not a thing that can happen.
#[test]
fn a_bound_chrome_control_carries_its_chord_inline_exactly_once() {
    let shell = ShellState::new(Vec::new(), String::new());
    let badged = shell.chrome_control_label("playground.navbar.roles.editor", "Editor");
    assert!(badged.starts_with("Editor "), "⌨️ the badge follows the label, as React's `ms-auto` span does");
    assert_eq!(shell.chrome_control_label("playground.navbar.roles.editor", &badged), badged, "⌨️ idempotent: a builder that already badged its own label is left alone");
    assert_eq!(shell.chrome_control_label("playground.navbar.fixture", "Concrete Forest"), "Concrete Forest", "⌨️ an unbound control carries no badge, as `useControlHotkey` answers `undefined`");
}

/// ⚖️ LAW (§E2/§E3): the two keybinding rows the W14 audit flagged as UNVERIFIED drift. Both are
/// already aligned with React's `SHELL_KEYBINDINGS`, and this law is what stops them drifting back:
/// fullscreen is `mod+shift+f` (never `F11`/`ctrl+meta+f`), and the Windows redo alias `mod+y`
/// resolves to Redo beside `mod+shift+z`.
#[test]
fn the_flagged_keybinding_rows_match_reacts_registry() {
    let fullscreen = SHELL_SHORTCUT_ROWS.iter().find(|(id, _)| *id == "os.toggleFullscreen").expect("⌨️ the fullscreen row");
    assert_eq!(fullscreen.1, "mod+shift+f", "⌨️ React's `SHELL_KEYBINDINGS['os.toggleFullscreen']`");
    for (chord, expected) in [("mod+z", ShellEditVerb::Undo), ("mod+shift+z", ShellEditVerb::Redo), ("mod+y", ShellEditVerb::Redo)] {
        let mut modifiers = PointerModifiers::default();
        let mut key = "";
        for token in chord.split('+') {
            match token {
                "mod" | "meta" => modifiers.meta = true,
                "ctrl" => modifiers.ctrl = true,
                "shift" => modifiers.shift = true,
                "alt" => modifiers.alt = true,
                other => key = other,
            }
        }
        let action = ui_wgpu::wgpu::KeyAction::Char(key.to_string());
        assert_eq!(shell_edit_verb_for(&action, &modifiers), Some(expected), "⌨️ {chord} is React's framework-universal history tail");
    }
}
