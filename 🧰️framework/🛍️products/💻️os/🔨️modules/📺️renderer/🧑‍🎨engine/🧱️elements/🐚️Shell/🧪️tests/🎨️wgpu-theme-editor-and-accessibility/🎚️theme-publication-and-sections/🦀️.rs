/// 🎚️ Theme publication and sections: retained republish without guest refresh, invalid input, the canonical document, and the colors, spacing, fonts, number, metrics and appearance sections.

#[test]
fn theme_host_mutations_republish_the_mounted_retained_leaf_without_guest_refresh() {
    let fixture = theme_publication_fixture();
    let surface = fixture["surfaceId"].as_str().expect("surface id");
    assert!(!fixture["requiresGuestRefresh"].as_bool().expect("guest refresh contract"));
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    shell.chrome.chrome_build.preferences.theme_id = "semio".into();
    shell.chrome.chrome_build.preferences.custom_themes.remove("custom.retained-publication");
    shell.prefs.clear_theme_draft();
    shell.prefs.theme_save_label.clear();
    shell.input.set_canonical_tree_open("framework.settings.theme.colors", true);
    let primary_row = 1 + 6 + 1 + shell_theme_document_base().colors.keys().position(|key| key == "primary").expect("primary color row");
    shell.input.scroll_offsets.insert(SHELL_THEME_EDITOR_SCROLL_ID.into(), (primary_row + 2) as f32 * SHELL_THEME_EDITOR_ROW_HEIGHT);
    let initial = SessionCtx::of(&mut shell).publish_shell_panel_document(surface).expect("initial Theme publication").expect("Theme owns a retained document");
    shell.docs.panel_documents.insert(surface.into(), initial);
    let mut missing = Vec::new();

    for step in fixture["workflow"].as_array().expect("theme workflow") {
        let action = step["action"].as_str().expect("action");
        let before = shell.docs.panel_documents.get(surface).unwrap().header().expect("Theme header before mutation");
        semio_framework_async::block_on(ActionsCtx::of(&mut shell).dispatch_action(ActionDescriptor { controller_id: "framework".into(), action: action.into(), args: Some(semio_framework::DslValue::from(&step["arguments"])) })).expect("theme host mutation");
        let after = shell.docs.panel_documents.get(surface).unwrap().header().expect("Theme header after mutation");
        if after.revision == before.revision || after.generation <= before.generation {
            missing.push(action.to_string());
            SessionCtx::of(&mut shell).republish_shell_panel_document(surface).expect("test continuation republishes the missing Theme successor");
        }
        for observation in step["observations"].as_array().expect("theme observations") {
            let control_id = observation["controlId"].as_str().expect("control id");
            assert_eq!(retained_theme_control_state(&shell, surface, control_id), observation["expectedState"].as_str().expect("expected state"), "{action}:{control_id}");
        }
        assert!(shell.docs.closing_documents.terminal_is_empty(), "{action} retires the exact replaced Theme owner");
        assert!(shell.settle.owed_refresh_scope.asks_for_nothing(), "{action} stays host-owned");
    }
    delete_custom_theme("custom.retained-publication");
    delete_custom_theme("custom.imported-publication");
    set_active_theme_id("semio");
    shell.prefs.clear_theme_draft();
    assert!(missing.is_empty(), "Theme mutations missing retained publication: {missing:?}");
}

#[test]
fn invalid_theme_field_and_import_inputs_keep_the_exact_retained_revision() {
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let surface = FRAMEWORK_SETTINGS_THEME_TAB_ID;
    let initial = SessionCtx::of(&mut shell).publish_shell_panel_document(surface).expect("initial Theme publication").expect("Theme retained document");
    let before = initial.header().expect("initial Theme header");
    shell.docs.panel_documents.insert(surface.into(), initial);
    for (action, arguments) in [("setThemeRadius", serde_json::json!({ "key": "nodeDefault", "value": "not-a-number" })), ("applyImportedTheme", serde_json::json!({ "value": "{}" }))] {
        semio_framework_async::block_on(ActionsCtx::of(&mut shell).dispatch_action(ActionDescriptor { controller_id: "framework".into(), action: action.into(), args: Some(semio_framework::DslValue::from(arguments)) })).expect("invalid Theme input is ignored");
        assert_eq!(shell.docs.panel_documents.get(surface).unwrap().header().expect("retained Theme header"), before, "{action} does not mint a successor");
        assert!(shell.docs.closing_documents.terminal_is_empty(), "{action} retires no owner");
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
    let text = include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧫️fixtures/🎨️canonical-theme-document/🔣️.json");
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
    let text = include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧫️fixtures/🎨️canonical-theme-document/🔣️.json");
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
    shell.input.set_canonical_tree_open("framework.settings.theme.spacing", true);
    let UiNode::Tree(tree) = shell.input.build_settings_theme_ui(&shell.chrome, &shell.prefs) else { panic!("🌳️ theme body is a tree") };
    assert!(tree.sections.iter().find(|section| section.id == "framework.settings.theme.colors").is_some_and(|section| section.default_open == Some(true)));
    assert!(tree.sections.iter().find(|section| section.id == "framework.settings.theme.spacing").is_some_and(|section| section.default_open == Some(true)));

    let expected: std::collections::BTreeSet<String> = shell_theme_document_base().colors.keys().map(|key| format!("framework.settings.theme.colors.{key}")).collect();
    let mut reached = std::collections::BTreeSet::new();
    for row in (0..expected.len() + 24).step_by(6) {
        shell.input.scroll_offsets.insert(SHELL_THEME_EDITOR_SCROLL_ID.into(), row as f32 * SHELL_THEME_EDITOR_ROW_HEIGHT);
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
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    shell.prefs.apply_theme_editor_commit("setThemeMetric", &serde_json::json!({ "section": section, "key": key, "value": "42" }), &shell.chrome);
    assert_eq!(shell.prefs.theme_document(&shell.chrome).metrics[&section][key], ThemeNumber::Scalar(42.0), "🎨️ React's `setThemeMetric(section, key, value)`");
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
        group_open.input.scroll_offsets.insert(SHELL_THEME_EDITOR_SCROLL_ID.into(), row as f32 * SHELL_THEME_EDITOR_ROW_HEIGHT);
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
