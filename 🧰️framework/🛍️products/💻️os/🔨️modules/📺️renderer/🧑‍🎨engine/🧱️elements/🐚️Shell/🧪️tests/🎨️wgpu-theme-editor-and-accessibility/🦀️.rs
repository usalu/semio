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

fn accessibility_fixture() -> Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/♿️wgpu-accessibility-interaction/🔣️.json")).expect("accessibility fixture")
}

fn shell_with_nested_app_settings() -> ShellState {
    let fixture = accessibility_fixture();
    let selection = &fixture["nestedPanelSelection"];
    let mut shell = super::window_pane_chrome_tests::split_pane_shell();
    shell.sync_dock_tabs();
    shell.session = None;
    let branch = shell.dock_tabs.tabs_mut(PanelAnchor::BottomRight).iter_mut().find(|tab| tab.id == selection["rootId"].as_str().expect("root id")).expect("settings branch");
    branch.children.insert(0, DockTabNode::leaf(selection["appFirstLeafId"].as_str().expect("app settings id"), "App Settings", "settings", -1));
    shell.anchor_state_mut(PanelAnchor::BottomRight).visible = false;
    shell
}

/// 🎨️ The control ids one theme-editor build offers, in build order — the wgpu twin of reading
/// `TreeDataItem.id` off React's rendered tree.
fn theme_editor_control_ids(shell: &ShellState) -> Vec<String> {
    fn control(control: &UiControlNode, out: &mut Vec<String>) {
        match control {
            UiControlNode::Button(node) => out.extend(node.id.clone()),
            UiControlNode::Input(node) => out.push(node.id.clone()),
            UiControlNode::Select(node) => out.push(node.id.clone()),
            _ => {}
        }
    }
    fn item(row: &UiTreeItemNode, out: &mut Vec<String>) {
        if let Some(value) = row.control.as_ref() {
            control(value, out);
        } else {
            out.push(row.id.clone());
        }
        row.items.iter().flatten().for_each(|child| item(child, out));
    }
    fn walk(node: &UiNode, out: &mut Vec<String>) {
        match node {
            UiNode::Button(button) => out.extend(button.id.clone()),
            UiNode::Select(select) => out.push(select.id.clone()),
            UiNode::Input(input) => out.push(input.id.clone()),
            UiNode::Field(field) => walk(&field.child, out),
            UiNode::Stack(stack) => stack.children.iter().for_each(|child| walk(child, out)),
            UiNode::Section(section) => section.children.iter().for_each(|child| walk(child, out)),
            UiNode::Tree(tree) => tree.sections.iter().for_each(|section| {
                out.push(section.id.clone());
                section.items.iter().for_each(|row| item(row, out));
            }),
            _ => {}
        }
    }
    let mut out = Vec::new();
    walk(&shell.build_settings_theme_ui(), &mut out);
    out
}

/// 🎨️ A shell with one theme-editor section open.
fn shell_with_open_theme_section(section: &str) -> ShellState {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let parts: Vec<_> = section.split('.').collect();
    let root = parts.iter().take(4).copied().collect::<Vec<_>>().join(".");
    shell.set_canonical_tree_open(&root, true);
    for end in 5..=parts.len() {
        shell.set_canonical_tree_open(&parts[..end].join("."), true);
    }
    shell
}

fn theme_publication_fixture() -> Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/🎨️settings-theme-publication/🔣️.json")).expect("theme publication fixture")
}

fn retained_theme_control_state(shell: &ShellState, surface: &str, control_id: &str) -> String {
    let document = shell.panel_documents.get(surface).expect("Theme retained document");
    let read = document.try_read().expect("Theme retained document is readable");
    let key = format!("{surface}/{control_id}");
    for ordinal in 0..read.len() {
        let Some(record) = read.node_at(ordinal) else { continue };
        if record.key.as_str() != key {
            continue;
        }
        match &record.component {
            ui_contract::Component::Select(select) => return format!("select:{}", select.value.as_str()),
            ui_contract::Component::Input(input) => return format!("input:{}", input.value.as_str()),
            ui_contract::Component::Button(_) => return format!("button:{}", if record.disabled { "disabled" } else { "enabled" }),
            _ => continue,
        }
    }
    "absent".into()
}

#[test]
fn theme_host_mutations_republish_the_mounted_retained_leaf_without_guest_refresh() {
    let fixture = theme_publication_fixture();
    let surface = fixture["surfaceId"].as_str().expect("surface id");
    assert!(!fixture["requiresGuestRefresh"].as_bool().expect("guest refresh contract"));
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.chrome_build.preferences.theme_id = "semio".into();
    shell.chrome_build.preferences.custom_themes.remove("custom.retained-publication");
    shell.clear_theme_draft();
    shell.theme_save_label.clear();
    shell.set_canonical_tree_open("framework.settings.theme.colors", true);
    let primary_row = 1 + 6 + 1 + shell_theme_document_base().colors.keys().position(|key| key == "primary").expect("primary color row");
    shell.scroll_offsets.insert(SHELL_THEME_EDITOR_SCROLL_ID.into(), (primary_row + 2) as f32 * SHELL_THEME_EDITOR_ROW_HEIGHT);
    let initial = shell.publish_shell_panel_document(surface).expect("initial Theme publication").expect("Theme owns a retained document");
    shell.panel_documents.insert(surface.into(), initial);
    let mut missing = Vec::new();

    for step in fixture["workflow"].as_array().expect("theme workflow") {
        let action = step["action"].as_str().expect("action");
        let before = shell.panel_documents.get(surface).unwrap().header().expect("Theme header before mutation");
        semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "framework".into(), action: action.into(), args: semio_framework::optional_json_to_dsl(Some(step["arguments"].clone())) })).expect("theme host mutation");
        let after = shell.panel_documents.get(surface).unwrap().header().expect("Theme header after mutation");
        if after.revision == before.revision || after.generation <= before.generation {
            missing.push(action.to_string());
            shell.republish_shell_panel_document(surface).expect("test continuation republishes the missing Theme successor");
        }
        for observation in step["observations"].as_array().expect("theme observations") {
            let control_id = observation["controlId"].as_str().expect("control id");
            assert_eq!(retained_theme_control_state(&shell, surface, control_id), observation["expectedState"].as_str().expect("expected state"), "{action}:{control_id}");
        }
        assert!(shell.closing_documents.terminal_is_empty(), "{action} retires the exact replaced Theme owner");
        assert!(shell.owed_refresh_scope.asks_for_nothing(), "{action} stays host-owned");
    }
    delete_custom_theme("custom.retained-publication");
    delete_custom_theme("custom.imported-publication");
    set_active_theme_id("semio");
    shell.clear_theme_draft();
    assert!(missing.is_empty(), "Theme mutations missing retained publication: {missing:?}");
}

#[test]
fn invalid_theme_field_and_import_inputs_keep_the_exact_retained_revision() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let surface = FRAMEWORK_SETTINGS_THEME_TAB_ID;
    let initial = shell.publish_shell_panel_document(surface).expect("initial Theme publication").expect("Theme retained document");
    let before = initial.header().expect("initial Theme header");
    shell.panel_documents.insert(surface.into(), initial);
    for (action, arguments) in [("setThemeRadius", serde_json::json!({ "key": "nodeDefault", "value": "not-a-number" })), ("applyImportedTheme", serde_json::json!({ "value": "{}" }))] {
        semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "framework".into(), action: action.into(), args: semio_framework::optional_json_to_dsl(Some(arguments)) })).expect("invalid Theme input is ignored");
        assert_eq!(shell.panel_documents.get(surface).unwrap().header().expect("retained Theme header"), before, "{action} does not mint a successor");
        assert!(shell.closing_documents.terminal_is_empty(), "{action} retires no owner");
    }
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
    assert!(!document.canvas_fonts.is_empty(), "🎨️ canvasFonts");
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

/// 🧬️ A language-neutral fixture is accepted by the canonical parser and by serde_json's
/// independent value oracle with the same id, label, paint token and metric.
#[test]
fn canonical_theme_fixture_matches_the_json_oracle() {
    let text = include_str!("../🧱️fixtures/🎨️canonical-theme-document/🔣️.json");
    let document = ThemeDocument::parse(text).expect("canonical theme fixture");
    let oracle: Value = serde_json::from_str(text).expect("serde_json oracle");
    assert_eq!(document.id, oracle["id"].as_str().expect("id"));
    assert_eq!(document.label, oracle["label"].as_str().expect("label"));
    assert_eq!(document.colors["primary"], oracle["colors"]["primary"].as_str().expect("color"));
    assert_eq!(document.canvas_fonts["mapLabelSansFallback"], oracle["canvasFonts"]["mapLabelSansFallback"].as_str().expect("canvas font"));
    assert_eq!(document.metrics["chrome"]["navbarHeightUiSpacing"], ThemeNumber::Scalar(oracle["metrics"]["chrome"]["navbarHeightUiSpacing"].as_f64().expect("number")));
    assert_eq!(document.appearances["light"].keys().map(String::as_str).collect::<Vec<_>>(), ["board", "canvas", "chrome", "diagram", "map", "outcome"]);
    assert_eq!(ThemeDocument::parse(&document.canonical_json().expect("canonical serializer")), Some(document));
}

#[test]
fn canonical_theme_parser_rejects_shapes_reacts_parser_rejects() {
    let text = include_str!("../🧱️fixtures/🎨️canonical-theme-document/🔣️.json");
    let mut value: Value = serde_json::from_str(text).expect("canonical fixture");
    value.as_object_mut().expect("theme object").remove("canvasFonts");
    assert_eq!(ThemeDocument::parse(&value.to_string()), None);
    let mut value: Value = serde_json::from_str(text).expect("canonical fixture");
    value["appearances"]["light"].as_object_mut().expect("light appearance").remove("diagram");
    assert_eq!(ThemeDocument::parse(&value.to_string()), None);
    let mut value: Value = serde_json::from_str(text).expect("canonical fixture");
    value["appearances"]["dark"]["chrome"]["accent"]["token"] = Value::String("missing".into());
    assert_eq!(ThemeDocument::parse(&value.to_string()), None);
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
    let expected: Vec<String> = document.colors.keys().map(|key| format!("framework.settings.theme.colors.{key}")).collect();
    let rows: Vec<String> = ids.iter().filter(|id| id.starts_with("framework.settings.theme.colors.")).cloned().collect();
    assert!(!rows.is_empty());
    assert_eq!(rows, expected[..rows.len()], "🎨️ the first viewport is React's sorted `Object.keys(host.theme.colors)`, row id for row id");
}

/// ⚖️ LAW (theme interaction): Colors and Spacing stay open together, the tree owns a stable
/// scroll range, and successive viewport windows make every color row reachable without page controls.
#[test]
fn theme_groups_expand_independently_and_scrolling_reaches_every_color() {
    let mut shell = shell_with_open_theme_section("framework.settings.theme.colors");
    shell.set_canonical_tree_open("framework.settings.theme.spacing", true);
    let UiNode::Tree(tree) = shell.build_settings_theme_ui() else { panic!("🌳️ theme body is a tree") };
    assert!(tree.sections.iter().find(|section| section.id == "framework.settings.theme.colors").is_some_and(|section| section.default_open == Some(true)));
    assert!(tree.sections.iter().find(|section| section.id == "framework.settings.theme.spacing").is_some_and(|section| section.default_open == Some(true)));

    let expected: std::collections::BTreeSet<String> = shell_theme_document_base().colors.keys().map(|key| format!("framework.settings.theme.colors.{key}")).collect();
    let mut reached = std::collections::BTreeSet::new();
    for row in (0..expected.len() + 24).step_by(6) {
        shell.scroll_offsets.insert(SHELL_THEME_EDITOR_SCROLL_ID.into(), row as f32 * SHELL_THEME_EDITOR_ROW_HEIGHT);
        reached.extend(theme_editor_control_ids(&shell).into_iter().filter(|id| expected.contains(id)));
    }
    assert_eq!(reached, expected, "📜️ every color row enters a retained viewport while scrolling");
    assert!(theme_editor_control_ids(&shell).iter().all(|id| !id.contains(".page.")), "📜️ no page controls exist");
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
    let mut group_open = shell_with_open_theme_section("framework.settings.theme.appearances.light.chrome");
    let expected = ["framework.settings.theme.appearances.light.chrome.base", "framework.settings.theme.appearances.light.chrome.base.alpha"];
    let mut reached = std::collections::BTreeSet::new();
    for row in (0..96).step_by(4) {
        group_open.scroll_offsets.insert(SHELL_THEME_EDITOR_SCROLL_ID.into(), row as f32 * SHELL_THEME_EDITOR_ROW_HEIGHT);
        reached.extend(theme_editor_control_ids(&group_open).into_iter().filter(|id| expected.contains(&id.as_str())));
    }
    for id in expected {
        assert!(reached.contains(id), "🖌️ the virtualized viewport reaches {id}");
    }
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
    let expected = Theme::light().accent;
    for (actual, expected) in [(painted.accent.r, expected.r), (painted.accent.g, expected.g), (painted.accent.b, expected.b), (painted.accent.a, expected.a)] {
        assert!((actual - expected).abs() <= f32::EPSILON, "🎨️ an unedited document preserves the built-in theme within one f32 rounding step");
    }

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
/// React's DOM tree has no node ceiling; virtualized windows keep the retained document under 128.
#[test]
fn every_open_theme_section_publishes_within_the_retained_document_ceiling() {
    while !ui_contract::close_ui_value_page_one() {}
    let baseline = ui_contract::ui_value_headroom();
    let mut shell = ShellState::new(Vec::new(), String::new());
    for (suffix, _) in SHELL_THEME_EDITOR_SECTIONS {
        shell.set_canonical_tree_open(&format!("framework.settings.theme.{suffix}"), true);
    }
    for section in shell_theme_document_base().metrics.keys() {
        shell.set_canonical_tree_open(&format!("framework.settings.theme.metrics.{section}"), true);
    }
    for appearance in ["light", "dark"] {
        shell.set_canonical_tree_open(&format!("framework.settings.theme.appearances.{appearance}"), true);
        for (group, _) in SHELL_THEME_APPEARANCE_GROUPS {
            shell.set_canonical_tree_open(&format!("framework.settings.theme.appearances.{appearance}.{group}"), true);
        }
    }
    for row in (0..960).step_by(24) {
        shell.scroll_offsets.insert(SHELL_THEME_EDITOR_SCROLL_ID.into(), row as f32 * SHELL_THEME_EDITOR_ROW_HEIGHT);
        let records = panel_ui_records(FRAMEWORK_SETTINGS_THEME_TAB_ID, &shell.build_settings_theme_ui()).unwrap_or_else(|error| panic!("🎨️ the all-open theme leaf publishes at row {row}: {error}"));
        assert!(records.len() <= ui_contract::UI_DOCUMENT_NODES, "🎨️ row {row} publishes {} records over the {} ceiling", records.len(), ui_contract::UI_DOCUMENT_NODES);
        drop(records);
        while !ui_contract::close_ui_value_page_one() {}
        assert_eq!(ui_contract::ui_value_headroom(), baseline, "♻️ row {row} returns every temporary action value before the next viewport");
    }
}

/// ⚖️ React disables Reset only for the clean Semio baseline.
#[test]
fn reset_disabled_state_matches_reacts_baseline_predicate() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.chrome_build.preferences.theme_id = "semio".into();
    let records = panel_ui_records(FRAMEWORK_SETTINGS_THEME_TAB_ID, &shell.build_settings_theme_ui()).expect("clean theme document");
    let reset = records.iter().find(|record| record.key.as_str().ends_with("/framework.settings.theme.reset")).expect("reset control");
    assert!(reset.disabled);
    shell.theme_draft = Some(shell_theme_document_base().clone());
    let records = panel_ui_records(FRAMEWORK_SETTINGS_THEME_TAB_ID, &shell.build_settings_theme_ui()).expect("dirty theme document");
    let reset = records.iter().find(|record| record.key.as_str().ends_with("/framework.settings.theme.reset")).expect("reset control");
    assert!(!reset.disabled);
}

/// ⚖️ LAW (§B2): the General leaf carries React's `framework.settings.mergePolicy` selector, with
/// React's own three option values.
#[test]
fn the_general_leaf_offers_reacts_merge_policy_selector() {
    let shell = ShellState::new(Vec::new(), String::new());
    let mut selects = Vec::new();
    fn walk(node: &UiNode, out: &mut Vec<(String, Vec<String>)>) {
        fn walk_item(item: &UiTreeItemNode, out: &mut Vec<(String, Vec<String>)>) {
            if let Some(UiControlNode::Select(select)) = item.control.as_ref() {
                out.push((select.id.clone(), select.items.iter().map(|option| option.value.clone()).collect()));
            }
            if let Some(items) = item.items.as_deref() {
                items.iter().for_each(|nested| walk_item(nested, out));
            }
        }
        match node {
            UiNode::Select(select) => out.push((select.id.clone(), select.items.iter().map(|item| item.value.clone()).collect())),
            UiNode::Field(field) => walk(&field.child, out),
            UiNode::Stack(stack) => stack.children.iter().for_each(|child| walk(child, out)),
            UiNode::Section(section) => section.children.iter().for_each(|child| walk(child, out)),
            UiNode::Tree(tree) => tree.sections.iter().flat_map(|section| &section.items).for_each(|item| walk_item(item, out)),
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
    let text = document.canonical_json().expect("💾️ the canonical 2-space JSON React's `serializeUiTheme` writes");
    assert_eq!(ThemeDocument::parse(&text), Some(document), "💾️ export/import is lossless, so a theme crosses between renderers");
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
    shell.apply_theme_editor_commit("setThemeSpacing", &serde_json::json!({ "key": "compact", "value": "5px" }));
    assert_eq!(shell.theme_document().spacing["compact"], "5px", "🎨️ React's `setThemeSpacing`");
    shell.apply_theme_editor_commit("setThemeFontStack", &serde_json::json!({ "key": "sans", "value": "Fixture Sans" }));
    assert_eq!(shell.theme_document().font_stacks["sans"], "Fixture Sans", "🎨️ React's `setThemeFontStack`");
    shell.apply_theme_editor_commit("setThemeStroke", &serde_json::json!({ "key": "edgeBase", "value": "7" }));
    assert_eq!(shell.theme_document().strokes["edgeBase"], ThemeNumber::Scalar(7.0), "🎨️ React's `setThemeStroke`");
    shell.apply_theme_editor_commit("setThemeRadius", &serde_json::json!({ "key": "nodeDefault", "value": "11" }));
    assert_eq!(shell.theme_document().radii["nodeDefault"], ThemeNumber::Scalar(11.0), "🎨️ React's `setThemeRadius`");
    shell.apply_theme_editor_commit("setThemeOpacity", &serde_json::json!({ "key": "gridMinorAlpha", "value": "0.25" }));
    assert_eq!(shell.theme_document().opacities["gridMinorAlpha"], ThemeNumber::Scalar(0.25), "🎨️ React's `setThemeOpacity`");
    shell.apply_theme_editor_commit("setThemeMetric", &serde_json::json!({ "section": "camera", "key": "zoomMin", "value": "0.2" }));
    assert_eq!(shell.theme_document().metrics["camera"]["zoomMin"], ThemeNumber::Scalar(0.2), "🎨️ React's `setThemeMetric`");
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

#[test]
fn field_and_engagement_labels_reach_focusable_child_controls() {
    let action = ActionDescriptor { controller_id: "fixture".into(), action: "setIterations".into(), args: None };
    let field = UiNode::Field(UiFieldNode {
        id: "settings.iterations.field".into(),
        label: Label::data("Iterations"),
        description: Some("Whole-number iterations".into()),
        required: None,
        error: None,
        child: Box::new(UiNode::NumberStepper(UiNumberStepperNode { id: "settings.iterations".into(), value: 3.0, step: 1.0, uniform: false, on_absolute: action.clone(), on_delta: action, presence: UiPresence::default(), menu: None })),
        presence: UiPresence::default(),
        menu: None,
    });
    let records = panel_ui_records("settings", &field).expect("field projection");
    let child = records.iter().find(|record| record.key.as_str() == "settings/settings.iterations").expect("field child");
    let projected = ui_contract::accessibility_projection_node(child, 1);
    assert_eq!(projected.role, "spinbutton");
    assert_eq!(projected.label.as_deref(), Some("Iterations"));
    assert_eq!(projected.description.as_deref(), Some("Whole-number iterations"));

    let rows =
        engagement_control_rows(&ui_wgpu::wgpu::WindowEngagementControl::Stepper { id: None, label: Some("Count".into()), value: 3.0, min: None, max: None, step: Some(1.0), unit: None, disabled: None, on_change: None, on_commit: None }, false);
    assert_eq!(rows.len(), 1, "the semantic field owns the visible label and control");
    let records = panel_ui_records("engagement", &rows[0]).expect("engagement projection");
    let child = records.iter().find(|record| record.key.as_str() == "engagement/engagement-control.stepper").expect("engagement stepper");
    assert_eq!(ui_contract::accessibility_projection_node(child, 1).label.as_deref(), Some("Count"));
}

#[test]
fn accessibility_activation_updates_settings_switch_and_active_tab_projection() {
    let fixture = accessibility_fixture();
    let selection = &fixture["nestedPanelSelection"];
    let mut shell = shell_with_nested_app_settings();
    let mut input = InputState::default();
    input.register_hit(HitTarget { rect: Rect::new(0.0, 0.0, 1.0, 1.0), event: None, control_id: Some(FRAMEWORK_SETTINGS_PANEL_ID.into()), kind: HitKind::Toggle, drag_axis: None, drag_data: None });
    input.publish_hits();
    let initial = shell.chrome_accessibility_nodes(input.hits()).into_iter().next().expect("settings projection");
    assert_eq!(initial.checked, Some(false));
    let target =
        ui_render::AccessibilityTarget { window_id: crate::interpreter::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID.to_string(), window_generation: crate::interpreter::SHELL_CHROME_ACCESSIBILITY_GENERATION, node_id: initial.node_id, node_key: initial.key };
    assert!(semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("settings activation"));
    assert_eq!(shell.chrome_accessibility_nodes(input.hits())[0].checked, Some(true), "the Settings branch switch reflects its now-visible anchor");

    assert_eq!(shell.reveal_dock_tab(FRAMEWORK_SETTINGS_THEME_TAB_ID), Some(PanelAnchor::BottomRight));
    let mut input = InputState::default();
    input.register_hit(HitTarget { rect: Rect::new(0.0, 0.0, 1.0, 1.0), event: None, control_id: Some(FRAMEWORK_SETTINGS_GENERAL_TAB_ID.into()), kind: HitKind::PanelTab, drag_axis: None, drag_data: None });
    input.publish_hits();
    let general = shell.chrome_accessibility_nodes(input.hits()).into_iter().next().expect("general projection");
    assert_eq!(general.selected, Some(false));
    let target =
        ui_render::AccessibilityTarget { window_id: crate::interpreter::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID.to_string(), window_generation: crate::interpreter::SHELL_CHROME_ACCESSIBILITY_GENERATION, node_id: general.node_id, node_key: general.key };
    assert!(semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("general activation"));
    assert_eq!(shell.chrome_accessibility_nodes(input.hits())[0].selected, Some(true), "the activated General leaf publishes aria-selected=true");
    let expected: Vec<&str> = selection["expectedPath"].as_array().expect("expected path").iter().map(|id| id.as_str().expect("path id")).collect();
    assert_eq!(shell.anchor_state(PanelAnchor::BottomRight).path.iter().map(String::as_str).collect::<Vec<_>>(), expected);
    assert_ne!(shell.anchor_state(PanelAnchor::BottomRight).active_tab(), selection["appFirstLeafId"].as_str());
    let records = panel_ui_records(FRAMEWORK_SETTINGS_GENERAL_TAB_ID, &shell.build_settings_general_ui()).expect("General retained projection");
    let required = selection["requiredControlId"].as_str().expect("required control id");
    assert!(records.iter().any(|record| record.key.as_str().ends_with(required)), "the selected General document publishes Appearance");
    drop(records);
    while !ui_contract::close_ui_value_page_one() {}
}

#[test]
fn pointer_activation_resolves_every_nested_leaf_from_root_to_target_independent_of_panel_rect() {
    let fixture = accessibility_fixture();
    let selection = &fixture["nestedPanelSelection"];
    let expected: Vec<&str> = selection["expectedPath"].as_array().expect("expected path").iter().map(|id| id.as_str().expect("path id")).collect();
    for rect in selection["pointerRects"].as_array().expect("pointer rectangles") {
        let mut shell = shell_with_nested_app_settings();
        let x = rect["x"].as_f64().expect("x") as f32;
        let y = rect["y"].as_f64().expect("y") as f32;
        let width = rect["width"].as_f64().expect("width") as f32;
        let height = rect["height"].as_f64().expect("height") as f32;
        let mut input = InputState::default();
        input.register_hit(HitTarget { rect: Rect::new(x, y, width, height), event: None, control_id: Some(selection["targetLeafId"].as_str().expect("target leaf").into()), kind: HitKind::PanelTab, drag_axis: None, drag_data: None });
        input.publish_hits();
        semio_framework_async::block_on(shell.handle_pointer_button(x + width * 0.5, y + height * 0.5, true, 0, &mut input, &Theme::default())).expect("nested panel pointer activation");
        assert_eq!(shell.anchor_state(PanelAnchor::BottomRight).path.iter().map(String::as_str).collect::<Vec<_>>(), expected, "{} panel rectangle", rect["id"].as_str().expect("rectangle id"));
        assert_eq!(shell.anchor_state(PanelAnchor::BottomRight).active_tab(), selection["targetLeafId"].as_str());
        assert_ne!(shell.anchor_state(PanelAnchor::BottomRight).active_tab(), selection["appFirstLeafId"].as_str());
    }
}

#[test]
fn chrome_accessibility_dispatch_validates_current_identity_and_activates_once() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut input = InputState::default();
    input.register_hit(HitTarget { rect: Rect::new(1.0, 2.0, 3.0, 4.0), event: None, control_id: Some("ui.search.toggle".to_string()), kind: HitKind::Button, drag_axis: None, drag_data: None });
    input.publish_hits();
    let node = shell.chrome_accessibility_nodes(input.hits()).into_iter().next().expect("search projection");
    let target = ui_render::AccessibilityTarget {
        window_id: crate::interpreter::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID.to_string(),
        window_generation: crate::interpreter::SHELL_CHROME_ACCESSIBILITY_GENERATION,
        node_id: node.node_id,
        node_key: node.key.clone(),
    };

    assert!(semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).expect("focus dispatch"));
    assert_eq!(shell.chrome_accessibility_nodes(input.hits())[0].focused, true);
    assert!(semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("activation dispatch"));
    assert!(shell.search_open, "the real chrome handler toggles search once");

    let stale = ui_render::AccessibilityTarget { window_generation: target.window_generation + 1, ..target.clone() };
    assert!(!semio_framework_async::block_on(shell.handle_accessibility_event(&stale, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("stale dispatch"));
    assert!(shell.search_open, "a stale generation cannot toggle the live control");
    let reused = ui_render::AccessibilityTarget { node_key: "ui.find.toggle".to_string(), ..target };
    assert!(!semio_framework_async::block_on(shell.handle_accessibility_event(&reused, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("reused identity dispatch"));
    assert!(shell.search_open, "a mismatched node id and key cannot activate another control");
}

#[test]
fn a_delayed_chrome_mirror_address_cannot_activate_after_its_presented_epoch_retires() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.dock_tabs.tabs_mut(PanelAnchor::BottomRight).push(DockTabNode::branch(
        FRAMEWORK_SETTINGS_PANEL_ID,
        "Settings",
        "settings",
        0,
        vec![DockTabNode::leaf(FRAMEWORK_SETTINGS_GENERAL_TAB_ID, "General", "settings", 0)],
    ));
    let mut input = InputState::default();
    let settings_hit = || HitTarget {
        rect: Rect::new(1.0, 2.0, 30.0, 20.0),
        event: None,
        control_id: Some("ui.panelToggle.settings".to_string()),
        kind: HitKind::Toggle,
        drag_axis: None,
        drag_data: None,
    };
    input.register_hit(settings_hit());
    shell.publish_retained_hit_registry(&mut input);
    let initial_epoch = shell.presented_input_epoch;
    let initial = shell.chrome_accessibility_nodes(input.hits()).into_iter().find(|node| node.key == "ui.panelToggle.settings").expect("first accepted Settings mirror node");
    let delayed = ui_render::AccessibilityTarget {
        window_id: crate::interpreter::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID.to_string(),
        window_generation: initial_epoch,
        node_id: initial.node_id,
        node_key: initial.key,
    };

    while input.retire_hit_step() {}
    input.register_hit(settings_hit());
    shell.publish_retained_hit_registry(&mut input);
    assert!(shell.presented_input_epoch > initial_epoch, "the successor mirror owns a distinct accepted epoch");
    assert!(!semio_framework_async::block_on(shell.handle_accessibility_event(&delayed, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("delayed Settings activation"));
    assert!(!shell.anchor_open(PanelAnchor::BottomRight), "a delayed address cannot mutate the successor frame");

    let successor = shell.chrome_accessibility_nodes(input.hits()).into_iter().find(|node| node.key == "ui.panelToggle.settings").expect("successor Settings mirror node");
    let current = ui_render::AccessibilityTarget { window_generation: shell.presented_input_epoch, node_id: successor.node_id, node_key: successor.key, ..delayed };
    assert!(semio_framework_async::block_on(shell.handle_accessibility_event(&current, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("current Settings activation"));
    assert!(shell.anchor_open(PanelAnchor::BottomRight), "the current accepted mirror address activates exactly once");
}

#[test]
fn a_constrained_settings_strip_retains_all_semantic_tabs_and_reveals_an_accessibility_selected_tail() {
    let fixture = accessibility_fixture();
    let contract = &fixture["settingsTabStrip"];
    let mut shell = ShellState::new(Vec::new(), String::new());
    let anchor = PanelAnchor::BottomRight;
    let tabs = contract["tabs"]
        .as_array()
        .expect("Settings tabs")
        .iter()
        .enumerate()
        .map(|(order, tab)| DockTabNode::leaf(tab["id"].as_str().unwrap(), tab["label"].as_str().unwrap(), "settings", order as i32))
        .collect::<Vec<_>>();
    shell.dock_tabs.tabs_mut(anchor).push(DockTabNode::branch(FRAMEWORK_SETTINGS_PANEL_ID, "Settings", "settings", 0, tabs));
    shell.anchor_state_mut(anchor).path = vec![FRAMEWORK_SETTINGS_PANEL_ID.into(), contract["activeId"].as_str().unwrap().into()];
    shell.anchor_state_mut(anchor).visible = true;
    let theme = Theme::default();
    let panel = Rect::new(0.0, 0.0, contract["availableWidth"].as_f64().unwrap() as f32, 240.0);
    let mut input = InputState::<ActionDescriptor>::default();
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    shell.paint_anchor_tab_bar(anchor, &mut draw, &mut atlas, &icons, &mut input, &theme, panel);
    shell.publish_retained_hit_registry(&mut input);

    let expected = contract["tabs"].as_array().unwrap().iter().map(|tab| tab["id"].as_str().unwrap()).collect::<Vec<_>>();
    let pointer_ids = input.hits().iter().filter(|hit| hit.kind == HitKind::PanelTab).filter_map(|hit| hit.control_id.as_deref()).collect::<Vec<_>>();
    assert!(pointer_ids.len() < expected.len(), "the constrained pointer row exposes only nonempty clipped chips");
    let semantic = shell.chrome_accessibility_nodes(input.hits());
    for id in &expected {
        assert!(semantic.iter().any(|node| node.key == *id), "the presented semantic catalogue retains {id} outside the pointer clip");
    }

    let tail_id = contract["tailIds"][1].as_str().unwrap();
    let tail = semantic.iter().find(|node| node.key == tail_id).expect("semantic tail tab");
    let target = ui_render::AccessibilityTarget {
        window_id: crate::interpreter::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID.to_string(),
        window_generation: shell.presented_input_epoch,
        node_id: tail.node_id,
        node_key: tail.key.clone(),
    };
    assert!(semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("tail tab activation"));
    assert_eq!(shell.anchor_state(anchor).active_tab(), Some(tail_id));

    while input.retire_hit_step() {}
    let mut draw = DrawList::default();
    shell.paint_anchor_tab_bar(anchor, &mut draw, &mut atlas, &icons, &mut input, &theme, panel);
    shell.publish_retained_hit_registry(&mut input);
    assert!(input.hits().iter().any(|hit| hit.kind == HitKind::PanelTab && hit.control_id.as_deref() == Some(tail_id)), "the next accepted pointer row reveals the accessibility-selected tail");
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

/// ⚖️ LAW (§E2/§E3): full-screen belongs to the platform-scoped OS command registry, while the
/// framework-universal history aliases stay in the shell tail.
#[test]
fn the_flagged_keybinding_rows_match_reacts_registry() {
    let shell = ShellState::new(Vec::new(), String::new());
    let fullscreen = shell.build_os_commands().into_iter().find(|command| command.id == "os.toggleFullscreen").expect("fullscreen OS command");
    assert!(fullscreen.keybindings.iter().any(|binding| binding.chord == "control+meta+f" && binding.platform == Some(semio_framework::manifest::Platform::MacOs)));
    assert_eq!(fullscreen.keybindings.iter().filter(|binding| binding.chord == "f11").count(), 2);
    assert!(!SHELL_SHORTCUT_ROWS.iter().any(|(id, _)| *id == "os.toggleFullscreen"), "the control-badge fallback is not an executable shell shortcut");
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
