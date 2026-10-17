/// 🔁️ Theme tokens and leaves: re-tokenised painting, the publication ceiling, the reset predicate, the General and Theme leaves and the rows' authored coordinates.

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
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    for (suffix, _) in SHELL_THEME_EDITOR_SECTIONS {
        shell.input.set_canonical_tree_open(&format!("framework.settings.theme.{suffix}"), true);
    }
    for section in shell_theme_document_base().metrics.keys() {
        shell.input.set_canonical_tree_open(&format!("framework.settings.theme.metrics.{section}"), true);
    }
    for appearance in ["light", "dark"] {
        shell.input.set_canonical_tree_open(&format!("framework.settings.theme.appearances.{appearance}"), true);
        for (group, _) in SHELL_THEME_APPEARANCE_GROUPS {
            shell.input.set_canonical_tree_open(&format!("framework.settings.theme.appearances.{appearance}.{group}"), true);
        }
    }
    for row in (0..960).step_by(24) {
        shell.input.scroll_offsets.insert(SHELL_THEME_EDITOR_SCROLL_ID.into(), row as f32 * SHELL_THEME_EDITOR_ROW_HEIGHT);
        let records = panel_ui_records(FRAMEWORK_SETTINGS_THEME_TAB_ID, &shell.input.build_settings_theme_ui(&shell.chrome, &shell.prefs)).unwrap_or_else(|error| panic!("🎨️ the all-open theme leaf publishes at row {row}: {error}"));
        assert!(records.len() <= ui_contract::UI_DOCUMENT_NODES, "🎨️ row {row} publishes {} records over the {} ceiling", records.len(), ui_contract::UI_DOCUMENT_NODES);
        drop(records);
        while !ui_contract::close_ui_value_page_one() {}
        assert_eq!(ui_contract::ui_value_headroom(), baseline, "♻️ row {row} returns every temporary action value before the next viewport");
    }
}

/// ⚖️ React disables Reset only for the clean Semio baseline.
#[test]
fn reset_disabled_state_matches_reacts_baseline_predicate() {
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    shell.chrome.chrome_build.preferences.theme_id = "semio".into();
    let records = panel_ui_records(FRAMEWORK_SETTINGS_THEME_TAB_ID, &shell.input.build_settings_theme_ui(&shell.chrome, &shell.prefs)).expect("clean theme document");
    let reset = records.iter().find(|record| record.key.as_str().ends_with("/framework.settings.theme.reset")).expect("reset control");
    assert!(reset.disabled);
    shell.prefs.theme_draft = Some(shell_theme_document_base().clone());
    let records = panel_ui_records(FRAMEWORK_SETTINGS_THEME_TAB_ID, &shell.input.build_settings_theme_ui(&shell.chrome, &shell.prefs)).expect("dirty theme document");
    let reset = records.iter().find(|record| record.key.as_str().ends_with("/framework.settings.theme.reset")).expect("reset control");
    assert!(!reset.disabled);
}

/// ⚖️ LAW (§B2): the General leaf carries React's `framework.settings.mergePolicy` selector, with
/// React's own three option values.
#[test]
fn the_general_leaf_offers_reacts_merge_policy_selector() {
    let shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
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
    walk(&shell.docs.build_settings_general_ui(&shell.chrome, &shell.prefs), &mut selects);
    let (_, options) = selects.iter().find(|(id, _)| id == "framework.settings.mergePolicy").expect("⚖️ React's merge-policy row has a wgpu twin");
    assert_eq!(options, &vec!["LaissezFaire".to_string(), "Normal".to_string(), "Vigilant".to_string()], "⚖️ React's `MERGE_POLICY_OPTIONS`, verbatim");
    assert_eq!(shell.prefs.merge_policy, SHELL_DEFAULT_MERGE_POLICY, "⚖️ a fresh authority is `Normal`, `protocol::MergePolicy::default()`");
}

/// ⚖️ LAW (§B3): the theme leaf carries React's export/import pair and its save box, and an exported
/// document re-parses — the `.theme.dsl` text React's `serializeUiTheme`/`parseUiTheme` round-trip.
#[test]
fn the_theme_leaf_offers_reacts_export_import_and_save_controls() {
    let shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let ids = theme_editor_control_ids(&shell);
    for id in ["framework.settings.theme.select", "framework.settings.theme.saveLabel", "framework.settings.theme.save", "framework.settings.theme.reset", "framework.settings.theme.export", "framework.settings.theme.import"] {
        assert!(ids.contains(&id.to_string()), "🎨️ {id} is one of React's theme-leaf controls");
    }
    let document = shell.prefs.theme_document(&shell.chrome);
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

    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    shell.prefs.apply_theme_editor_commit("setThemeColor", &serde_json::json!({ "key": "primary", "value": "#00ff00" }), &shell.chrome);
    assert_eq!(shell.prefs.theme_document(&shell.chrome).colors["primary"], "#00ff00", "🎨️ React's `setThemeColor`");
    shell.prefs.apply_theme_editor_commit("setThemeSpacing", &serde_json::json!({ "key": "compact", "value": "5px" }), &shell.chrome);
    assert_eq!(shell.prefs.theme_document(&shell.chrome).spacing["compact"], "5px", "🎨️ React's `setThemeSpacing`");
    shell.prefs.apply_theme_editor_commit("setThemeFontStack", &serde_json::json!({ "key": "sans", "value": "Fixture Sans" }), &shell.chrome);
    assert_eq!(shell.prefs.theme_document(&shell.chrome).font_stacks["sans"], "Fixture Sans", "🎨️ React's `setThemeFontStack`");
    shell.prefs.apply_theme_editor_commit("setThemeStroke", &serde_json::json!({ "key": "edgeBase", "value": "7" }), &shell.chrome);
    assert_eq!(shell.prefs.theme_document(&shell.chrome).strokes["edgeBase"], ThemeNumber::Scalar(7.0), "🎨️ React's `setThemeStroke`");
    shell.prefs.apply_theme_editor_commit("setThemeRadius", &serde_json::json!({ "key": "nodeDefault", "value": "11" }), &shell.chrome);
    assert_eq!(shell.prefs.theme_document(&shell.chrome).radii["nodeDefault"], ThemeNumber::Scalar(11.0), "🎨️ React's `setThemeRadius`");
    shell.prefs.apply_theme_editor_commit("setThemeOpacity", &serde_json::json!({ "key": "gridMinorAlpha", "value": "0.25" }), &shell.chrome);
    assert_eq!(shell.prefs.theme_document(&shell.chrome).opacities["gridMinorAlpha"], ThemeNumber::Scalar(0.25), "🎨️ React's `setThemeOpacity`");
    shell.prefs.apply_theme_editor_commit("setThemeMetric", &serde_json::json!({ "section": "camera", "key": "zoomMin", "value": "0.2" }), &shell.chrome);
    assert_eq!(shell.prefs.theme_document(&shell.chrome).metrics["camera"]["zoomMin"], ThemeNumber::Scalar(0.2), "🎨️ React's `setThemeMetric`");
    shell.prefs.apply_theme_editor_commit("setThemeAppearancePaint", &serde_json::json!({ "appearance": "light", "group": "chrome", "paint": "base", "channel": "hex", "value": "#101112" }), &shell.chrome);
    let reference = shell.prefs.theme_document(&shell.chrome).appearances["light"]["chrome"]["base"].clone();
    assert_eq!(reference.hex.as_deref(), Some("#101112"), "🖌️ React's `setThemeAppearancePaint` replaces the ref outright");
    assert!(reference.alpha.is_some(), "🖌️ …carrying the row's current alpha, as React's colour input does");
}
