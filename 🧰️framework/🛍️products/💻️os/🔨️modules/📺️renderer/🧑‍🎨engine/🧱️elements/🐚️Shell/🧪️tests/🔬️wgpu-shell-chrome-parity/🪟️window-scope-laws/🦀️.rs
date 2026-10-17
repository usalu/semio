/// 🪟️ Window scope laws: the window scope fixture, seeded active windows, instance icon overrides and app-wide chord routing to the owning window.

//#region ⌨️WindowScope

const WINDOW_SCOPE_FIXTURE: &str = include_str!("../../../../🏛️ShellHost/🧪️tests/🧫️fixtures/⌨️window-scope/🔣️.json");

/// 🪟️ Rebuilds a fixture layout node as the wgpu dock tree. The fixture is authored in the React
/// shell's own layout vocabulary (`window` / `stack` + `activeId` / `row` / `column`), which is the
/// point: ONE corpus, two renderers.
fn window_scope_node(value: &Value) -> semio_framework_os_renderer_wgpu_dock::DockNode {
    match value["kind"].as_str().expect("⌨️ fixture layout node kind") {
        "window" => {
            let id = value["id"].as_str().expect("⌨️ fixture window id").to_string();
            semio_framework_os_renderer_wgpu_dock::DockNode::Stack { windows: vec![DockStackTab::new(id.clone())], active: id }
        }
        "stack" => {
            let windows: Vec<DockStackTab> = value["children"].as_array().expect("⌨️ fixture stack children").iter().map(|child| DockStackTab::new(child["id"].as_str().expect("⌨️ fixture window id"))).collect();
            let active = value["activeId"].as_str().map(str::to_string).or_else(|| windows.first().map(|tab| tab.window_id.clone())).unwrap_or_default();
            semio_framework_os_renderer_wgpu_dock::DockNode::Stack { windows, active }
        }
        "row" => semio_framework_os_renderer_wgpu_dock::DockNode::Row(value["children"].as_array().expect("⌨️ fixture row children").iter().map(|child| (window_scope_node(child), 1.0)).collect()),
        "column" => semio_framework_os_renderer_wgpu_dock::DockNode::Column(value["children"].as_array().expect("⌨️ fixture column children").iter().map(|child| (window_scope_node(child), 1.0)).collect()),
        other => panic!("⌨️ unknown fixture layout node kind {other}"),
    }
}

/// 📇️ An app whose window kinds declare exactly the fixture's per-kind action ids — the shape
/// `window_kind_action_refs` produces in a plugin manifest.
fn window_scope_app(fixture: &Value, app_id: &str) -> AppDefinition {
    let kinds: Vec<WindowKindDefinition> = fixture["apps"][app_id]["kinds"]
        .as_array()
        .expect("⌨️ fixture app kinds")
        .iter()
        .map(|kind| WindowKindDefinition {
            initial_utility_id: None,
            id: kind["id"].as_str().expect("⌨️ fixture kind id").into(),
            label: LocalizedLabel::native(kind["id"].as_str().expect("⌨️ fixture kind id"), kind["id"].as_str().expect("⌨️ fixture kind id")),
            body_key: format!("{}.body", kind["id"].as_str().expect("⌨️ fixture kind id")),
            surface_kind: semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d,
            icon_id: "app-window".into(),
            options: Default::default(),
            actions: kind["actionIds"]
                .as_array()
                .expect("⌨️ fixture kind actionIds")
                .iter()
                .map(|id| {
                    let id = id.as_str().expect("⌨️ fixture action id");
                    semio_framework::ActionDefinition::new(id, LocalizedLabel::native(id, id), semio_framework::ActionKind::Mutation, "create")
                })
                .collect(),
            utilities: vec![],
            interactions: vec![],
            params_schema: None,
            artifact_snapshot_schema: None,
            input_event_schema: None,
            output_schema: None,
            capabilities: vec![],
        })
        .collect();
    let mut app = parity_app(app_id, AppRole::Editor, dialectless(), app_id, app_id, &["edit"]);
    app.window_kinds = WindowKinds::try_from(kinds).expect("⌨️ a fixture app declares at least one window kind");
    app
}

#[test]
fn a_seeded_mode_layout_always_opens_with_one_window_active() {
    let fixture: Value = serde_json::from_str(WINDOW_SCOPE_FIXTURE).expect("⌨️ the shared window-scope fixture parses");
    let mut projections = 0;
    for case in fixture["stacks"].as_array().expect("⌨️ fixture stack rows") {
        let node = window_scope_node(&fixture["apps"][case["app"].as_str().expect("⌨️ fixture app")]["modes"][case["mode"].as_str().expect("⌨️ fixture mode")]);
        let stacks = mode_layout_stacks_v1(&node);
        let expected: Vec<WindowScopeStackV1> = case["expected"]
            .as_array()
            .expect("⌨️ fixture expected stacks")
            .iter()
            .map(|stack| WindowScopeStackV1 {
                window_ids: stack["windowIds"].as_array().expect("⌨️ fixture windowIds").iter().map(|id| id.as_str().expect("⌨️ fixture window id").to_string()).collect(),
                active_window_id: stack["activeWindowId"].as_str().map(str::to_string),
            })
            .collect();
        assert_eq!(stacks, expected, "{}: the dock flattens to the same stacks the React dock does", case["id"]);
        projections += 1;
    }
    let mut seeds = 0;
    for case in fixture["dockSeed"].as_array().expect("⌨️ fixture dock-seed rows") {
        let node = window_scope_node(&fixture["apps"][case["app"].as_str().expect("⌨️ fixture app")]["modes"][case["mode"].as_str().expect("⌨️ fixture mode")]);
        let seeded = dock_seed_active_window_id_v1(&mode_layout_stacks_v1(&node), case["activeWindowId"].as_str());
        assert_eq!(seeded.as_deref(), case["expected"].as_str(), "{}: the seeded active window", case["id"]);
        if let Some(seeded) = seeded {
            assert_eq!(dock_seed_active_window_id_v1(&mode_layout_stacks_v1(&node), Some(seeded.as_str())), None, "{}: re-seeding over its own answer is a no-operation", case["id"]);
        }
        seeds += 1;
    }
    assert!(seeds >= 6 && projections >= 3, "⌨️ the shared corpus must stay populated");
}

#[test]
fn projection_instance_tab_icons_match_reacts_ephemeral_overrides() {
    let fixture: Value = serde_json::from_str(WINDOW_SCOPE_FIXTURE).expect("🖼️ the shared window-scope fixture parses");
    for case in fixture["tabPresentation"].as_array().expect("🖼️ fixture tab-presentation rows") {
        let actual = dock_tab_icon_id_v1(case["kindIconId"].as_str().expect("🖼️ kind icon"), None, case["templateId"].as_str());
        assert_eq!(actual, case["expectedIconId"].as_str().expect("🖼️ expected icon"), "{}: wgpu tab icon", case["id"]);
    }
}

#[test]
fn accepted_window_icon_commands_are_instance_scoped_precede_projection_and_retire_before_id_reuse() {
    let fixture: Value = serde_json::from_str(WINDOW_ICON_OVERRIDE_FIXTURE).expect("🖼️ the shared window-icon fixture parses");
    let kind_id = fixture["windowKindId"].as_str().expect("🖼️ fixture window kind");
    let kind_icon_id = fixture["kindIconId"].as_str().expect("🖼️ fixture kind icon");
    let projection = &fixture["projectionFallback"];
    let window_ids = fixture["windowIds"].as_array().expect("🖼️ fixture window ids");
    let layout = |include_retired: bool| {
        let tabs = window_ids
            .iter()
            .filter_map(|window_id| {
                let window_id = window_id.as_str().expect("🖼️ fixture window id");
                if !include_retired && window_id == fixture["retireReuse"]["windowId"].as_str().expect("🖼️ retired window id") {
                    return None;
                }
                let template_id = (window_id == projection["windowId"].as_str().expect("🖼️ projection window id")).then(|| projection["templateId"].as_str().expect("🖼️ projection template").to_string());
                Some(DockStackTab::instance_template(window_id, kind_id, template_id, WindowStackCorner::TopLeft))
            })
            .collect();
        semio_framework_os_renderer_wgpu_dock::DockNode::Stack { windows: tabs, active: window_ids[0].as_str().expect("🖼️ active window id").to_string() }
    };
    let full_root = layout(true);
    let mut shell = host_test_shell();
    shell.dock.layout_override = None;
    let session = shell.docs.session.as_mut().expect("🖼️ test session");
    let kind = session.app.window_kinds.first_mut();
    kind.id = kind_id.to_string();
    kind.icon_id = kind_icon_id.into();
    session.app.default_layout = Some(WindowLayout { root: semio_framework_os_renderer_wgpu_dock::dock_node_to_layout_root(&full_root) });
    SessionCtx::of(&mut shell).sync_dock();

    let (_, initial_icons) = shell.dock.dock_chrome_maps(&shell.prefs, &shell.docs, &shell.windows);
    assert_eq!(initial_icons.get(projection["windowId"].as_str().unwrap()).map(String::as_str), projection["expectedIconId"].as_str(), "a projection icon is the pre-publication fallback");
    assert_eq!(initial_icons.get(fixture["kindFallback"]["windowId"].as_str().unwrap()).map(String::as_str), fixture["kindFallback"]["expectedIconId"].as_str(), "an ordinary instance keeps its kind icon");

    for command in fixture["commands"].as_array().expect("🖼️ fixture icon commands") {
        let window_id = command["windowId"].as_str().expect("🖼️ command window id");
        assert!(shell.dock.apply_window_icon_host_command(window_id, command["iconId"].as_str().expect("🖼️ command icon id"), &shell.docs, &mut shell.windows), "a command for the exact live instance is accepted");
        let (_, icons) = shell.dock.dock_chrome_maps(&shell.prefs, &shell.docs, &shell.windows);
        assert_eq!(icons.get(window_id).map(String::as_str), command["expectedIconId"].as_str(), "{window_id}: command icon wins");
    }
    let (_, independent) = shell.dock.dock_chrome_maps(&shell.prefs, &shell.docs, &shell.windows);
    assert_eq!(independent.get("main").map(String::as_str), Some("diamond"), "replacing one instance changes only its own slot");
    assert_eq!(independent.get("main-2").map(String::as_str), Some("circle"), "the sibling instance keeps its independent override");
    assert_eq!(independent.get("main-projection").map(String::as_str), Some("star"), "a generic override precedes a valid projection icon");
    for rejected in fixture["rejected"].as_array().expect("🖼️ rejected icon commands") {
        assert!(!shell.dock.apply_window_icon_host_command(rejected["windowId"].as_str().unwrap(), rejected["iconId"].as_str().unwrap(), &shell.docs, &mut shell.windows), "a missing instance or empty icon is refused");
    }

    shell.docs.session.as_mut().expect("🖼️ test session").app.default_layout = Some(WindowLayout { root: semio_framework_os_renderer_wgpu_dock::dock_node_to_layout_root(&layout(false)) });
    SessionCtx::of(&mut shell).sync_dock();
    let retired_id = fixture["retireReuse"]["windowId"].as_str().expect("🖼️ retired id");
    assert!(!shell.windows.window_icon_overrides.contains_key(retired_id), "retirement erases the renderer-local owner");
    shell.docs.session.as_mut().expect("🖼️ test session").app.default_layout = Some(WindowLayout { root: semio_framework_os_renderer_wgpu_dock::dock_node_to_layout_root(&full_root) });
    SessionCtx::of(&mut shell).sync_dock();
    let (_, reused) = shell.dock.dock_chrome_maps(&shell.prefs, &shell.docs, &shell.windows);
    assert_eq!(reused.get(retired_id).map(String::as_str), fixture["retireReuse"]["expectedReusedIconId"].as_str(), "an id reused by a successor starts from kind fallback");
}

#[test]
fn dock_sync_seeds_the_authored_instance_and_publishes_its_projection_icons() {
    let fixture: Value = serde_json::from_str(WINDOW_SCOPE_FIXTURE).expect("🖼️ the shared window-scope fixture parses");
    let rows = fixture["tabPresentation"].as_array().expect("🖼️ fixture tab-presentation rows");
    let top_template = rows[0]["templateId"].as_str().expect("🖼️ top template");
    let perspective_template = rows[1]["templateId"].as_str().expect("🖼️ perspective template");
    let root = semio_framework_os_renderer_wgpu_dock::DockNode::Row(vec![
        (semio_framework_os_renderer_wgpu_dock::DockNode::Stack { windows: vec![DockStackTab::instance_template("main-top", "main", Some(top_template.into()), WindowStackCorner::TopLeft)], active: "main-top".into() }, 1.0),
        (semio_framework_os_renderer_wgpu_dock::DockNode::Stack { windows: vec![DockStackTab::instance_template("main-perspective", "main", Some(perspective_template.into()), WindowStackCorner::TopLeft)], active: "main-perspective".into() }, 2.0),
    ]);
    let mut shell = host_test_shell();
    shell.dock.layout_override = None;
    shell.windows.active_window_id = Some("main".into());
    let session = shell.docs.session.as_mut().expect("🖼️ test session");
    session.app.window_kinds.first_mut().icon_id = "puzzle".into();
    session.app.default_layout = Some(WindowLayout { root: semio_framework_os_renderer_wgpu_dock::dock_node_to_layout_root(&root) });

    SessionCtx::of(&mut shell).sync_dock();

    assert_eq!(shell.windows.active_window_id.as_deref(), Some("main-top"));
    assert_eq!(shell.dock.dock.active_window_id.as_deref(), Some("main-top"));
    assert_eq!(shell.dock.dock.active_stack.as_deref(), Some([0].as_slice()));
    let (_, icons) = shell.dock.dock_chrome_maps(&shell.prefs, &shell.docs, &shell.windows);
    assert_eq!(icons.get("main-top").map(String::as_str), Some("projection-orthographic"));
    assert_eq!(icons.get("main-perspective").map(String::as_str), Some("projection-three-point"));
}

/// 🎯️ A resolved target always names a MOUNTED window whose kind declares the verb — anything
/// else dies on the undeclared-action gate instead of reaching the plugin.
#[test]
fn an_app_wide_chord_resolves_to_the_window_that_owns_its_verb_in_the_active_mode() {
    let fixture: Value = serde_json::from_str(WINDOW_SCOPE_FIXTURE).expect("⌨️ the shared window-scope fixture parses");
    let mut owners = 0;
    let mut unowned = 0;
    for case in fixture["chords"].as_array().expect("⌨️ fixture chord rows") {
        let app_id = case["app"].as_str().expect("⌨️ fixture app");
        let app = window_scope_app(&fixture, app_id);
        let node = window_scope_node(&fixture["apps"][app_id]["modes"][case["mode"].as_str().expect("⌨️ fixture mode")]);
        let mounted: Vec<WindowScopeInstanceV1> = mode_layout_stacks_v1(&node).into_iter().flat_map(|stack| stack.window_ids).map(|id| WindowScopeInstanceV1 { id: id.clone(), window_kind_id: id }).collect();
        let action_id = case["actionId"].as_str().expect("⌨️ fixture action id");
        let (kind, window_id) = resolve_keybinding_target_window_v1(&app, &mounted, case["focusedWindowId"].as_str(), action_id);
        let expected_kind = match case["expectedKind"].as_str().expect("⌨️ fixture expected kind") {
            "focused" => WindowScopeTargetKindV1::Focused,
            "owner" => WindowScopeTargetKindV1::Owner,
            other => {
                assert_eq!(other, "unowned", "⌨️ the fixture's only three outcomes");
                WindowScopeTargetKindV1::Unowned
            }
        };
        assert_eq!(kind, expected_kind, "{}: outcome", case["id"]);
        assert_eq!(window_id.as_deref(), case["expectedWindowId"].as_str(), "{}: target window", case["id"]);
        match kind {
            WindowScopeTargetKindV1::Unowned => unowned += 1,
            WindowScopeTargetKindV1::Owner => owners += 1,
            WindowScopeTargetKindV1::Focused => {}
        }
        if let Some(window_id) = window_id {
            assert!(mounted.iter().any(|instance| instance.id == window_id), "{}: the target is mounted", case["id"]);
            assert!(window_action_definition(&app, &window_id, action_id).is_some(), "{}: the target declares the verb", case["id"]);
        }
    }
    for case in fixture["unownedHint"].as_array().expect("⌨️ fixture hint rows") {
        let text = keybinding_unowned_text_v1(case["locale"].as_str().expect("⌨️ fixture locale"), case["chord"].as_str().expect("⌨️ fixture chord"), case["label"].as_str().expect("⌨️ fixture label"));
        assert_eq!(text, case["text"].as_str().expect("⌨️ fixture hint text"), "⌨️ the hint text in {}", case["locale"]);
    }
    assert_ne!(KEYBINDING_UNOWNED_LABEL_EN, KEYBINDING_UNOWNED_LABEL_DE, "🇩🇪️ the two languages are authored, not copied");
    assert!(owners >= 4 && unowned >= 2, "⌨️ the corpus must pin both the owner hop and the hinted no-operation");
    eprintln!("window-scope chords: {owners} owner hops, {unowned} hinted no-operations, {} hint locales, code {KEYBINDING_UNOWNED_CODE}", fixture["unownedHint"].as_array().expect("⌨️ fixture hint rows").len());
}
/// ⚖️ LAW: the shell's own chrome chords stay out of the app-keybinding loop, and only
/// accelerator-carrying ones do — a bare key belongs to whichever surface has focus, which is what
/// lets the node-graph canvas own bare arrows. The collision the fixture names is REAL: the chord the
/// navbar's mode step owns is the chord `build_definition` mints for `toolRunStep` on every app that
/// declares a tool run, so without this rule one keystroke had two owners.
#[test]
fn the_shells_own_accelerator_chords_are_reserved_from_the_app_keybinding_loop() {
    let fixture: Value = serde_json::from_str(WINDOW_SCOPE_FIXTURE).expect("⌨️ the shared window-scope fixture parses");
    let block = &fixture["reservedChords"];
    let pairs = |value: &Value| -> Vec<(String, String)> { value.as_object().expect("⌨️ fixture chord table").iter().map(|(id, keys)| (id.clone(), keys.as_str().expect("⌨️ fixture chord").to_string())).collect() };
    let reserved = reserved_shell_chords_v1(&pairs(&block["shellTable"]), &pairs(&block["overrides"]));
    let expected: std::collections::BTreeSet<String> = block["expected"].as_array().expect("⌨️ fixture expected").iter().map(|chord| chord.as_str().expect("⌨️ fixture chord").to_string()).collect();
    assert_eq!(reserved, expected, "🛡️ the reserved set agrees with the React twin");
    for case in block["cases"].as_array().expect("⌨️ fixture reserved-chord rows") {
        let chord = case["chord"].as_str().expect("⌨️ fixture chord");
        assert_eq!(chord_carries_accelerator_v1(chord), case["accelerator"].as_bool().expect("⌨️ fixture accelerator"), "{chord}: accelerator");
        assert_eq!(reserved.contains(chord), case["reserved"].as_bool().expect("⌨️ fixture reserved"), "{chord}: reserved");
    }
    assert_eq!(semio_framework_tool_run::ToolRunAction::Step.chord(), block["collision"]["chord"].as_str().expect("⌨️ fixture collision chord"), "the tool-run step chord still collides with the shell's mode step");
    assert_eq!(block["collision"]["alsoMintedFor"].as_str(), Some(semio_framework_tool_run::ToolRunAction::Step.id()), "the fixture names the action the chord is minted for");
    eprintln!(
        "reserved shell chords: {} of {} fixture rows reserved, collision {}",
        block["cases"].as_array().expect("rows").iter().filter(|case| case["reserved"].as_bool() == Some(true)).count(),
        block["cases"].as_array().expect("rows").len(),
        block["collision"]["chord"]
    );
}

//#endregion ⌨️WindowScope
