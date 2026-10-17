/// 🎛️ Example picker and role groups: the dialect-keyed example picker, boot example resolution, the roles group, role switching and mode stepping.

//#region 📚️ExamplePicker

#[test]
fn the_example_picker_offers_every_example_of_the_open_dialect_and_nothing_else() {
    let fixture: Value = serde_json::from_str(EXAMPLE_PICKER_FIXTURE).expect("📚️ the shared example-picker fixture parses");
    let examples: Vec<ExampleDefinition> = fixture["examples"]
        .as_array()
        .expect("fixture examples")
        .iter()
        .map(|row| ExampleDefinition {
            id: row["id"].as_str().expect("fixture example id").to_string(),
            label: LocalizedLabel::native(row["id"].as_str().expect("fixture example id"), row["id"].as_str().expect("fixture example id")),
            icon_id: "file".into(),
            artifact_json: "{}".into(),
            dialect: fixture_dialect(&row["dialect"]),
        })
        .collect();
    let mut cases = 0;
    for case in fixture["cases"].as_array().expect("fixture cases") {
        let mut app = parity_app(case["app"]["id"].as_str().expect("fixture app id"), fixture_role(case["app"]["role"].as_str().expect("fixture app role")), fixture_dialect(&case["app"]["dialect"]), "Surface", "Fläche", &["edit"]);
        if app.role == AppRole::Viewer {
            app.actions.push(semio_framework::ActionDefinition::new_catalog("setActiveExample", LocalizedLabel::native("Example", "Beispiel"), semio_framework::ActionKind::View));
        }
        let expected: Vec<String> = case["expected"].as_array().expect("fixture expectation").iter().map(|id| id.as_str().expect("fixture example id").to_string()).collect();
        let rows = shell_example_rows(&examples, &app, None, Terminology::Native, Locale::En);
        let control_ids: Vec<String> = rows.iter().map(|row| row.control_id.clone()).collect();
        let expected_control_ids: Vec<String> = expected.iter().map(|id| format!("shell.example.{id}")).collect();
        assert_eq!(control_ids, expected_control_ids, "{}: the picker's rows carry the React `shell.example.<id>` control ids, in manifest order", case["name"]);
        assert_eq!(
            shell_example_control(&rows, false, false).map(|control| control.control_id),
            (!expected.is_empty()).then(|| "playground.navbar.fixture".to_string()),
            "{}: the navbar hit target exists exactly when the open dialect authored an example",
            case["name"]
        );
        cases += 1;
    }
    assert!(cases >= 5, "the shared fixture is expected to carry every surface/dialect case");
}

#[test]
fn viewer_example_offer_matches_declared_action_authority() {
    let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/📚️example-offer/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let dialect = ArtifactDialect { artifact_kind: "neutral.media".into(), standard: "1".into(), subset: "*".into() };
        let mut app = parity_app(row["name"].as_str().unwrap(), fixture_role(row["role"].as_str().unwrap()), dialect.clone(), "Surface", "Fläche", &["view"]);
        let action = |id: &str| semio_framework::ActionDefinition::new_catalog(id, LocalizedLabel::native(id, id), semio_framework::ActionKind::View);
        app.actions = row["appActions"].as_array().unwrap().iter().map(|id| action(id.as_str().unwrap())).collect();
        let mut kinds = Vec::new();
        for (index, actions) in row["windowActions"].as_array().unwrap().iter().enumerate() {
            let mut kind = parity_window_kinds().iter().next().unwrap().clone();
            kind.id = index.to_string();
            kind.actions = actions.as_array().unwrap().iter().map(|id| action(id.as_str().unwrap())).collect();
            kinds.push(kind);
        }
        if !kinds.is_empty() { app.window_kinds = WindowKinds::try_from(kinds).unwrap(); }
        let examples = (0..row["exampleCount"].as_u64().unwrap()).map(|index| ExampleDefinition { id: format!("demo-{index}"), label: LocalizedLabel::native("Demo", "Demo"), icon_id: "file".into(), artifact_json: "{}".into(), dialect: dialect.clone() }).collect::<Vec<_>>();
        let rows = shell_example_rows(&examples, &app, Some("demo-0"), Terminology::Native, Locale::En);
        assert_eq!(!rows.is_empty(), row["expected"].as_bool().unwrap(), "{}", row["name"]);
        let options = rows.iter().map(|row| row.control_id.trim_start_matches("shell.example.").to_string()).collect::<Vec<_>>();
        assert_eq!(resolve_boot_example_id("demo-0", &options, Some("demo-0")).is_empty(), !row["expected"].as_bool().unwrap(), "{}", row["name"]);
    }
}

#[test]
fn the_example_picker_trigger_shows_the_picked_row_and_the_open_dropdown_is_its_pressed_state() {
    let dialect = ArtifactDialect { artifact_kind: "s.procedural.generation3d".into(), standard: "1".into(), subset: "*".into() };
    let app = parity_app("s.procedural.generation3d@1/*#editor", AppRole::Editor, dialect.clone(), "Editor", "Editor", &["edit"]);
    let examples = vec![
        ExampleDefinition { id: "hex".into(), label: LocalizedLabel::native("Hexagonal Column", "Sechseckige Säule"), icon_id: "file".into(), artifact_json: "{}".into(), dialect: dialect.clone() },
        ExampleDefinition { id: "box".into(), label: LocalizedLabel::native("Box Shell", "Boxschale"), icon_id: "file".into(), artifact_json: "{}".into(), dialect },
    ];
    let rows = shell_example_rows(&examples, &app, Some("box"), Terminology::Native, Locale::De);
    assert_eq!(rows.iter().filter(|row| row.selected).map(|row| row.control_id.as_str()).collect::<Vec<_>>(), vec!["shell.example.box"]);
    assert_eq!(rows[0].label, "Sechseckige Säule", "row labels are the example's OWN localized label — no shell dictionary, no default language");
    let control = shell_example_control(&rows, true, true).expect("two examples offer a picker");
    assert_eq!(control.label, "Boxschale", "the trigger shows the picked row, the way a select shows its value");
    assert!(control.active, "an open dropdown is the trigger's pressed state");
    assert!(!shell_example_control(&rows, false, true).expect("picker").active);
    let empty = shell_example_rows(&[], &app, None, Terminology::Native, Locale::En);
    assert!(shell_example_control(&empty, false, false).is_none(), "a dialect with no authored example paints no trigger at all");
}

#[test]
fn the_boot_example_is_resolved_the_way_the_shared_fixture_declares() {
    let fixture: Value = serde_json::from_str(BOOT_EXAMPLE_FIXTURE).expect("📚️ the shared boot-example fixture parses");
    assert_eq!(fixture["bootQueryParam"].as_str(), Some("example"), "the boot query axis is spelled the same on both shells");
    let mut rows = 0;
    for row in fixture["rows"].as_array().expect("fixture rows") {
        let options: Vec<String> = row["options"].as_array().expect("fixture options").iter().map(|id| id.as_str().expect("fixture option id").to_string()).collect();
        let answered = resolve_boot_example_id(row["activeExampleId"].as_str().expect("fixture activeExampleId"), &options, row["defaultExampleId"].as_str());
        assert_eq!(answered, row["expected"].as_str().expect("fixture expectation"), "{}", row["id"]);
        rows += 1;
    }
    assert!(rows >= 8, "the shared fixture is expected to carry every precedence and fallback case");
}

/// 📚️ The `?example=` axis reaches the picker through ONE door — `sync_session_chrome` handing
/// `crate::boot_app_example()` to the shared resolver — so the query cannot drift from the rule the
/// fixture pins. A source law, because the thread-local the query lands in is set by a `#[wasm_bindgen]`
/// hook that has no native caller to drive from a test.
#[test]
fn the_boot_example_query_reaches_the_picker_through_the_shared_resolver() {
    let chrome = WGPU_SHELL_SOURCE().split("fn sync_session_chrome").nth(1).expect("the shell declares sync_session_chrome");
    let body = &chrome[..["\n    fn ", "\n    pub(crate) fn "].iter().filter_map(|marker| chrome.find(marker)).min().unwrap_or(chrome.len())];
    assert!(body.contains("shell_offered_examples("), "boot and role sync use the declared example offer authority");
    assert!(body.contains("resolve_boot_example_id("), "sync_session_chrome resolves through the shared predicate, never its own inline rule");
    assert!(body.contains("crate::boot_app_example()"), "`?example=` is the declared default the resolver is handed");
    let apply = WGPU_SHELL_SOURCE().split("async fn announce_session_example").nth(1).expect("the shell declares announce_session_example");
    let apply_body = apply.split("\n    /// ").next().expect("the body of announce_session_example");
    assert!(apply_body.contains("\"setActiveExample\""), "the resolved example is ANNOUNCED to the guest, not only painted on the trigger");
    assert!(!apply_body.contains("boot_app_example"), "the announcement never depends on a `?example=` query: a boot without one opens the dialect's first example as React does");
    assert!(!WGPU_SHELL_SOURCE().contains("fn apply_boot_example"), "the boot has no example path of its own");
    let settle = WGPU_SHELL_SOURCE().split("async fn settle_boot").nth(1).expect("the shell declares settle_boot");
    let settle_body = settle.split("\n    /// ").next().expect("the body of settle_boot");
    let push = settle_body.find("push_contributions").expect("settle_boot pushes contributions");
    let example = settle_body.find("announce_session_example").expect("settle_boot announces the resolved example");
    let refresh = settle_body.find("refresh_ui").expect("settle_boot refreshes");
    assert!(push < example && example < refresh, "the boot example is announced AFTER the flow-extension registry is armed and BEFORE the first render");
}

//#endregion 📚️ExamplePicker

//#region 🎛️ModeAndRoleGroups

#[test]
fn the_roles_group_exists_exactly_when_the_plugin_declares_both_surfaces_of_the_open_dialect() {
    let fixture: Value = serde_json::from_str(SURFACE_SWITCH_FIXTURE).expect("🔀️ the shared surface-switch fixture parses");
    let mut rows = 0;
    for case in fixture["group"].as_array().expect("fixture group rows") {
        let apps = manifest_apps(&fixture, case["manifest"].as_str().expect("fixture manifest id"));
        let dialect = case["dialect"].as_str().map_or_else(dialectless, |key| fixture_dialect(&fixture["dialects"][key]));
        let open = parity_app("open", AppRole::Editor, dialect.clone(), "Editor", "Editor", &["edit"]);
        let controls = shell_role_controls(&apps, &open, Terminology::Native, Locale::En, &[]);
        match case["expected"].as_object() {
            Some(expected) => {
                assert_eq!(
                    controls.iter().map(|control| control.control_id.as_str()).collect::<Vec<_>>(),
                    vec!["playground.navbar.roles.editor", "playground.navbar.roles.viewer"],
                    "{}: the group renders both React control ids in editor → viewer focus order",
                    case["id"]
                );
                let resolved = surface_role_apps(&apps, &dialect).expect("both surfaces resolve");
                assert_eq!(resolved.0.id, expected["editor"].as_str().expect("fixture editor id"));
                assert_eq!(resolved.1.id, expected["viewer"].as_str().expect("fixture viewer id"));
                assert!(controls[0].active, "the open editor surface is the pressed button");
                assert!(!controls[1].active);
            }
            None => {
                assert!(controls.is_empty(), "{}: no sibling surface means no group at all", case["id"]);
                assert!(surface_role_apps(&apps, &dialect).is_none());
            }
        }
        rows += 1;
    }
    assert_eq!(rows, 4, "the shared fixture's four group rows");
}

#[test]
fn a_role_switch_resolves_the_app_the_shared_fixture_declares() {
    let fixture: Value = serde_json::from_str(SURFACE_SWITCH_FIXTURE).expect("🔀️ fixture parses");
    let mut rows = 0;
    for case in fixture["roleTargets"].as_array().expect("fixture roleTargets rows") {
        let apps = manifest_apps(&fixture, case["manifest"].as_str().expect("fixture manifest id"));
        let dialect = fixture_dialect(&fixture["dialects"][case["dialect"].as_str().expect("fixture dialect key")]);
        let target = role_switch_target(&apps, &dialect, fixture_role(case["currentRole"].as_str().expect("fixture current role")), fixture_role(case["requested"].as_str().expect("fixture requested role")));
        assert_eq!(target.map(|app| app.id.as_str()), case["expected"].as_str(), "{}", case["id"]);
        rows += 1;
    }
    assert_eq!(rows, 4, "the shared fixture's four role-target rows");
}

#[test]
fn the_mode_group_renders_one_pressed_button_per_declared_mode_and_none_for_a_single_mode_surface() {
    let dialect = ArtifactDialect { artifact_kind: "s.procedural.generation3d".into(), standard: "1".into(), subset: "*".into() };
    let editor = parity_app("s.procedural.generation3d@1/*#editor", AppRole::Editor, dialect.clone(), "Editor", "Editor", &["edit", "generate"]);
    let controls = shell_mode_controls(&editor, Some("generate"), Terminology::Native, Locale::En);
    assert_eq!(controls.iter().map(|control| control.control_id.as_str()).collect::<Vec<_>>(), vec!["playground.navbar.modes.edit", "playground.navbar.modes.generate"]);
    assert_eq!(controls.iter().map(|control| control.active).collect::<Vec<_>>(), vec![false, true]);
    let viewer = parity_app("s.procedural.generation3d@1/*#viewer", AppRole::Viewer, dialect, "Viewer", "Betrachter", &["view"]);
    assert!(shell_mode_controls(&viewer, Some("view"), Terminology::Native, Locale::En).is_empty(), "a one-mode surface renders no mode switcher, exactly as React renders none");
}

#[test]
fn stepping_the_mode_wraps_the_way_the_shared_fixture_declares() {
    let fixture: Value = serde_json::from_str(SURFACE_SWITCH_FIXTURE).expect("🔀️ fixture parses");
    let mut rows = 0;
    for case in fixture["modeSteps"].as_array().expect("fixture modeSteps rows") {
        let mode_ids: Vec<String> = case["modeIds"].as_array().expect("fixture mode ids").iter().map(|id| id.as_str().expect("fixture mode id").to_string()).collect();
        let step = i32::try_from(case["step"].as_i64().expect("fixture step")).expect("fixture step fits");
        assert_eq!(step_mode_id(&mode_ids, case["activeModeId"].as_str(), step).as_deref(), case["expected"].as_str(), "{}", case["id"]);
        rows += 1;
    }
    assert_eq!(rows, 6, "the shared fixture's six mode-step rows");
}

//#endregion 🎛️ModeAndRoleGroups
