/// 📐️ Passive regions and geometry: passive chrome regions, compact theme spacing, named metrics and their localized authoring guidance.

#[test]
fn passive_chrome_regions_are_not_announced_as_buttons() {
    let shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    with_chrome_control_names(|names| names.clear());
    let fixture = accessibility_fixture();
    let cases = fixture["passiveChrome"].as_array().unwrap();
    let hits = cases
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let kind = match row["kind"].as_str().unwrap() {
                "PanelResize" => HitKind::PanelResize,
                "ScrollRegion" => HitKind::ScrollRegion,
                "DockSplit" => HitKind::DockSplit,
                "DockJoinCorner" => HitKind::DockJoinCorner,
                "Generic" => HitKind::Generic,
                other => panic!("unknown fixture kind {other}"),
            };
            let id = row["id"].as_str().unwrap();
            if let Some(label) = row["label"].as_str() {
                note_chrome_control_name(id, Some(label));
            }
            HitTarget {
                rect: Rect::new(index as f32 * 10.0, 0.0, 10.0, 10.0),
                event: row["action"].as_bool().unwrap().then(|| ActionDescriptor { controller_id: "fixture".into(), action: "activate".into(), args: None }),
                control_id: Some(id.into()),
                kind,
                drag_axis: None,
                drag_data: None,
            }
        })
        .collect::<Vec<_>>();
    let nodes = PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(&hits);
    for row in cases {
        assert_eq!(nodes.iter().any(|node| Some(node.key.as_str()) == row["id"].as_str()), row["announced"].as_bool().unwrap());
    }
}

#[test]
fn compact_theme_spacing_and_named_metrics_match_neutral_css_pixels() {
    let fixture = theme_geometry_fixture();
    let bindings: Value = serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🌓️theme/📐️geometry/🔣️.json")).expect("theme geometry bindings");
    for case in fixture["cases"].as_array().expect("geometry cases") {
        let mut document = shell_theme_document_base().clone();
        document.spacing.insert("compact".into(), case["compact"].as_str().expect("compact spacing").into());
        for (section, rows) in case["metrics"].as_object().expect("metrics") {
            for (key, value) in rows.as_object().expect("metric rows") {
                document.metrics.entry(section.clone()).or_default().insert(key.clone(), ThemeNumber::Scalar(value.as_f64().expect("scalar metric")));
            }
        }
        document.metrics.entry("dom".into()).or_default().insert("rootRemPx".into(), ThemeNumber::Scalar(case["rootRemPx"].as_f64().expect("root rem")));
        let theme = theme_from_document(&document, false);
        let actual = BTreeMap::from([
            ("navbar_height", theme.navbar_height), ("footer_height", theme.footer_height),
            ("panel_header_height", theme.panel_header_height), ("control_height", theme.control_height),
            ("control_height_small", theme.control_height_small), ("gap_standard", theme.gap_standard),
            ("padding_standard", theme.padding_standard), ("panel_inset", theme.panel_inset),
            ("tree_row_height", theme.tree_row_height), ("tree_indent_per_level", theme.tree_indent_per_level),
            ("tree_toggle_width", theme.tree_toggle_width), ("panel_min_width", theme.panel_min_width),
            ("panel_max_width", theme.panel_max_width), ("window_measures_default_width", theme.window_measures_default_width),
            ("window_engagement_max_width", theme.window_engagement_max_width), ("font_size_body", theme.font_size_body),
            ("font_size_small", theme.font_size_small), ("font_size_emphasized", theme.font_size_emphasized),
        ]);
        assert_eq!(actual.len(), bindings["bindings"].as_array().expect("geometry bindings").len());
        for binding in bindings["bindings"].as_array().expect("geometry bindings") {
            let field = binding["themeField"].as_str().expect("theme field");
            let expected = case["expected"][field].as_f64().expect("expected pixels");
            let value = f64::from(actual[field]);
            assert!((value - expected).abs() < 0.0001, "{} {field}: expected {expected}, actual {value}", case["name"]);
        }
    }
    for compact in fixture["invalid"].as_array().expect("invalid compact lengths") {
        let mut document = shell_theme_document_base().clone();
        document.spacing.insert("compact".into(), compact.as_str().expect("invalid compact").into());
        assert!(ThemeDocument::parse(&serde_json::to_string(&document).expect("theme json")).is_none(), "invalid compact accepted: {compact}");
    }
    for row in fixture["invalidMetrics"].as_array().expect("invalid metrics") {
        let mut document = shell_theme_document_base().clone();
        let number: ThemeNumber = serde_json::from_value(row["value"].clone()).expect("metric value");
        document.metrics.entry(row["section"].as_str().unwrap().into()).or_default().insert(row["key"].as_str().unwrap().into(), number);
        assert!(ThemeDocument::parse(&serde_json::to_string(&document).expect("theme json")).is_none(), "invalid geometry metric accepted: {row}");
    }
}

fn theme_geometry_fixture() -> Value {
    serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧫️fixtures/📐️theme-geometry/🔣️.json")).expect("theme geometry fixture")
}

#[test]
fn theme_geometry_editor_commits_only_resolved_compact_lengths_and_reference_metrics() {
    let fixture = theme_geometry_fixture();
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    for case in fixture["cases"].as_array().expect("valid geometry cases") {
        shell.prefs.clear_theme_draft();
        shell.prefs.apply_theme_editor_commit("setThemeMetric", &serde_json::json!({"section":"dom", "key":"rootRemPx", "value":case["rootRemPx"].to_string()}), &shell.chrome);
        shell.prefs.apply_theme_editor_commit("setThemeSpacing", &serde_json::json!({"key":"compact", "value":case["compact"]}), &shell.chrome);
        assert_eq!(shell.prefs.theme_document(&shell.chrome).spacing["compact"], case["compact"].as_str().expect("compact length"));
    }
    for compact in fixture["invalid"].as_array().expect("invalid lengths") {
        shell.prefs.clear_theme_draft();
        let before = shell.prefs.theme_document(&shell.chrome);
        shell.prefs.apply_theme_editor_commit("setThemeSpacing", &serde_json::json!({"key":"compact", "value":compact}), &shell.chrome);
        assert_eq!(shell.prefs.theme_document(&shell.chrome), before, "invalid compact committed: {compact}");
        assert!(shell.prefs.theme_draft.is_none());
    }
    for row in fixture["invalidMetrics"].as_array().expect("invalid geometry metric rows") {
        shell.prefs.clear_theme_draft();
        let before = shell.prefs.theme_document(&shell.chrome);
        let text = row["value"].to_string();
        shell.prefs.apply_theme_editor_commit("setThemeMetric", &serde_json::json!({"section":row["section"], "key":row["key"], "value":text}), &shell.chrome);
        assert_eq!(shell.prefs.theme_document(&shell.chrome), before, "invalid metric committed: {row}");
        assert!(shell.prefs.theme_draft.is_none());
    }
    shell.prefs.clear_theme_draft();
}

#[test]
fn theme_geometry_compact_authoring_has_localized_label_and_description() {
    fn find_items(items: &[UiTreeItemNode]) -> Option<&UiTreeItemNode> {
        items.iter().find_map(|item| {
            if item.id == "framework.settings.theme.spacing.compact" { Some(item) }
            else { item.items.as_deref().and_then(find_items) }
        })
    }
    fn find_row(node: &UiNode) -> Option<&UiTreeItemNode> {
        match node {
            UiNode::Tree(tree) => tree.sections.iter().find_map(|section| find_items(&section.items)),
            UiNode::Stack(stack) => stack.children.iter().find_map(find_row),
            UiNode::Section(section) => section.children.iter().find_map(find_row),
            _ => None,
        }
    }
    let schema: Value = serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🌓️theme/📐️geometry/🧬️contract/🔣️.json")).expect("geometry contract");
    let ui = &schema["properties"]["compact"]["x-semio-ui"];
    let shell = shell_with_open_theme_section("framework.settings.theme.spacing");
    for (locale, is_de) in [("en", false), ("de", true)] {
        let tree = shell.input.build_settings_theme_editor_tree(shell_theme_document_base(), is_de, &shell.chrome, &shell.prefs);
        let row = find_row(&tree).expect("visible compact spacing row");
        let label = ui["label"][locale].as_str().expect("localized compact label");
        let description = ui["description"][locale].as_str().expect("localized compact description");
        assert_eq!(serde_json::to_value(&row.label).unwrap(), serde_json::to_value(Label::data(label)).unwrap());
        assert_eq!(row.description.as_deref(), Some(description));
        let Some(UiControlNode::Input(input)) = row.control.as_ref() else { panic!("compact authoring input") };
        assert_eq!(serde_json::to_value(&input.accessibility_label).unwrap(), serde_json::to_value(Some(Label::data(label))).unwrap());
    }
}

#[test]
fn theme_geometry_metric_authoring_exposes_shared_localized_guidance() {
    let contract: Value = serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🌓️theme/📐️geometry/🧬️contract/🔣️.json")).expect("geometry contract");
    let bindings: Value = serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🌓️theme/📐️geometry/🔣️.json")).expect("geometry bindings");
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    shell.input.set_canonical_tree_open("framework.settings.theme.metrics", true);
    for section in ["chrome", "dom", "typography"] { shell.input.set_canonical_tree_open(&format!("framework.settings.theme.metrics.{section}"), true); }
    for (locale, is_de) in [("en", false), ("de", true)] {
        let section = shell.input.theme_metrics_section(shell_theme_document_base(), is_de, &mut 0, &(0..usize::MAX));
        let rows: BTreeMap<_, _> = section.items.iter().flat_map(|group| group.items.iter().flatten()).map(|row| (row.id.as_str(), row)).collect();
        let mut expected: Vec<(&str, &str, &str)> = bindings["bindings"].as_array().unwrap().iter().map(|binding| (binding["section"].as_str().unwrap(), binding["key"].as_str().unwrap(), "nonnegativeScalar")).collect();
        expected.push(("dom", "rootRemPx", "positiveRoot"));
        for (section, key, guidance) in &expected {
            let id = format!("framework.settings.theme.metrics.{section}.{key}");
            let row = rows.get(id.as_str()).expect("visible geometry metric input");
            let hint = contract["x-semio-resolution"]["metricUi"][guidance][locale].as_str().expect("localized metric guidance");
            assert_eq!(row.description.as_deref(), Some(hint), "{locale} {id}");
            let Some(UiControlNode::Input(input)) = row.control.as_ref() else { panic!("geometry metric input") };
            assert_eq!(serde_json::to_value(&input.accessibility_label).unwrap(), serde_json::to_value(Some(Label::data(*key))).unwrap(), "{locale} {id}");
        }
    }
}
