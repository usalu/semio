/// 🪟️ Spawned documents and host panel: the fresh shell, spawned document routing, the host panel codec, claim order and stale session identity.

/// 🧪️ `ShellState::new` calls `load_persisted_dock`, which — on native — reads whatever the
/// machine running the test happens to have in its `semio.os.config` document. Every assertion
/// below explicitly sets the state it exercises afterward, so the outcome never depends on that.
fn fresh_state() -> ShellState {
    ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
}

/// 🎯️ The same neutral owner vectors address each retained spawned document and action.
#[test]
fn spawned_documents_and_actions_keep_their_exact_owning_session() {
    let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️spawned-document-routing/🔣️.json")).unwrap();
    let session = |owner: &Value| {
        let mut app = test_app(Vec::new(), Vec::new());
        app.id = owner["appId"].as_str().unwrap().into();
        app.controller_id = owner["controllerId"].as_str().unwrap().into();
        let id = owner["windowId"].as_str().unwrap().to_string();
        let mut view_state = ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
        view_state.window_id = Some(id.clone());
        view_state.focused_window_id = Some(id.clone());
        view_state.window_instances = vec![semio_framework::ViewWindowInstance { id, window_kind_id: "main".into() }];
        ActiveSession { plugin_id: owner["pluginId"].as_str().unwrap().into(), instance_id: owner["instanceId"].as_u64().unwrap() as u32, app, view_state }
    };
    for law in fixture["cases"].as_array().unwrap() {
        let mut shell = fresh_state();
        shell.plugins.space_mode = true;
        shell.docs.session = Some(session(&fixture["host"]));
        shell.docs.spawned_session = if law["retired"] == true { None } else { Some(session(&fixture["spawned"])) };
        if law["sharedController"] == true {
            shell.docs.spawned_session.as_mut().unwrap().app.controller_id = fixture["host"]["controllerId"].as_str().unwrap().into();
        }
        if law["sharedApp"] == true {
            let child = shell.docs.spawned_session.as_mut().unwrap();
            let host = shell.docs.session.as_ref().unwrap();
            child.plugin_id = host.plugin_id.clone();
            child.app.id = host.app.id.clone();
            child.app.controller_id = host.app.controller_id.clone();
        }
        let action = ActionDescriptor { controller_id: law["controllerId"].as_str().unwrap().into(), action: "edit".into(), args: law["surface"].as_str().map(|id| semio_framework::DslValue::from(serde_json::json!({"windowId": id}))) };
        let mut expected = law["ownerPath"].as_str().map(|path| fixture[path.trim_start_matches('/')].clone());
        if law["sharedApp"] == true {
            for field in ["pluginId", "appId", "controllerId"] {
                expected.as_mut().unwrap()[field] = fixture["host"][field].clone();
            }
        }
        let owner = shell.docs.action_session(&action, &shell.plugins);
        assert_eq!(owner.map(|s| (s.plugin_id.as_str(), s.instance_id, s.app.id.as_str())), expected.as_ref().map(|v| (v["pluginId"].as_str().unwrap(), v["instanceId"].as_u64().unwrap() as u32, v["appId"].as_str().unwrap())), "{}", law["id"]);
        if let Some(surface) = law["surface"].as_str().filter(|id| *id == "main" || *id == "spawned") {
            if expected.is_some() {
                assert_eq!(shell.docs.window_document_session(surface, &shell.plugins).map(|s| s.instance_id), owner.map(|s| s.instance_id), "{}", law["id"]);
            }
        }
        if let Some(owner) = owner {
            let live = SessionCtx::of(&mut shell).live_view_state(owner);
            assert!(live.window_instances.iter().any(|window| Some(&window.id) == owner.view_state.window_id.as_ref()), "{}", law["id"]);
            let invocation = shell.prefs.session_action_invocation(owner, &action, &live).expect("exact owner action address");
            assert_eq!(invocation.address.plugin_id, owner.plugin_id);
            assert_eq!(invocation.address.app_id, owner.app.id);
            assert_eq!(Some(&invocation.address.window_instance_id), owner.view_state.window_id.as_ref());
            if law["surface"] == "spawned" {
                assert_eq!(invocation.arguments.get("windowId").and_then(DslValue::as_str), owner.view_state.window_id.as_deref());
            }
            let command_owner = semio_framework::manifest::CommandOwnerAddress::App { plugin_id: owner.plugin_id.clone(), app_id: owner.app.id.clone() };
            assert!(shell.docs.command_session(&command_owner, owner.view_state.window_id.as_deref(), &shell.plugins).is_some_and(|session| same_session(session, owner)));
        }
        if law["ownerPath"] == "/spawned" {
            let selected = shell.docs.action_session(&action, &shell.plugins).unwrap().clone();
            let host_panel = shell.docs.session.as_ref().unwrap().view_state.panel_json.clone();
            let panel = serde_json::json!({"ownerApp": selected.app.id});
            let operations = vec![serde_json::json!({"operation": "setPanel", "panel": panel}).to_string()];
            let result = semio_framework_async::block_on(ActionsCtx::of(&mut shell).apply_ops_inner(&operations, true, UiDirtyScope::None, Some(selected)));
            assert_eq!(result.unwrap_err(), "session program missing");
            assert_eq!(shell.docs.session.as_ref().unwrap().view_state.panel_json, host_panel);
            assert_eq!(shell.docs.spawned_session.as_ref().unwrap().view_state.panel_json, Some(panel.to_string()));
        }
    }
}

#[test]
fn host_panel_json_codec_matches_the_neutral_fixture_and_rejects_invalid_carriage() {
    let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/📌️panel-state/🔣️.json")).expect("panel fixture");
    let base: SpacePanelState = serde_json::from_value(fixture["base"].clone()).expect("fixture base");
    let encoded = panel_json(&base).expect("strict panel JSON");
    assert!(encoded.starts_with('{'));
    let mut view = ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    view.panel_json = Some(encoded.clone());
    assert_eq!(panel_state_from_view(&view).expect("strict decode"), Some(base.clone()));
    view.panel_json = Some(serde_json::json!({ "activePanelTab": "s-play-catalogue", "spawnedApps": [], "programs": [] }).to_string());
    assert_eq!(panel_state_from_view(&view).unwrap_err(), "host-panel.invalid-json");
    let mut duplicate = base.clone();
    duplicate.spawned_apps.push(base.spawned_apps[0].clone());
    assert_eq!(panel_json(&duplicate).unwrap_err(), "host-panel.duplicate-spawned-identity");
    let mut missing_focus = base;
    missing_focus.active_spawned_id = Some("spawned-missing".into());
    assert_eq!(panel_json(&missing_focus).unwrap_err(), "host-panel.invalid-active-spawned");
}

#[test]
fn host_panel_action_is_claimed_before_guest_and_preserves_the_session_roster_and_home_projection() {
    let mut shell = host_test_shell();
    let before = panel_state_from_view(&shell.docs.session.as_ref().unwrap().view_state).unwrap().unwrap();
    semio_framework_async::block_on(ActionsCtx::of(&mut shell).dispatch_action(ActionDescriptor { controller_id: "space.studio".into(), action: "setActivePanelTab".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "tabId": "s-play-inspector" }) }))
        .expect("configured leaf is host-owned");
    let after = panel_state_from_view(&shell.docs.session.as_ref().unwrap().view_state).unwrap().unwrap();
    assert_eq!(after.active_panel_tab, "s-play-inspector");
    assert_eq!(after.spawned_apps, before.spawned_apps);
    assert_eq!(after.active_spawned_id, before.active_spawned_id);
    assert!(shell.dock.anchor_open(PanelAnchor::TopRight), "a Details-group leaf reveals the top-right anchor");
    assert_eq!(shell.dock.anchor_state(PanelAnchor::TopRight).active_tab(), Some("s-play-inspector"));

    let accepted_json = shell.docs.session.as_ref().unwrap().view_state.panel_json.clone();
    let container_error =
        semio_framework_async::block_on(ActionsCtx::of(&mut shell).dispatch_action(ActionDescriptor { controller_id: "space.studio".into(), action: "setActivePanelTab".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "tabId": "studio-settings" }) })).unwrap_err();
    assert_eq!(container_error, "host-panel.tab-is-not-configured-leaf");
    assert_eq!(shell.docs.session.as_ref().unwrap().view_state.panel_json, accepted_json);
    let stale_error = semio_framework_async::block_on(ActionsCtx::of(&mut shell).dispatch_action(ActionDescriptor { controller_id: "space.studio".into(), action: "setActivePanelTab".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "tabId": "missing" }) })).unwrap_err();
    assert_eq!(stale_error, "host-panel.tab-is-not-configured-leaf");
    let missing_error = semio_framework_async::block_on(ActionsCtx::of(&mut shell).dispatch_action(ActionDescriptor { controller_id: "space.studio".into(), action: "setActivePanelTab".into(), args: None })).unwrap_err();
    assert_eq!(missing_error, "host-panel.invalid-tab-id");

    let guest_error = semio_framework_async::block_on(ActionsCtx::of(&mut shell).dispatch_action(ActionDescriptor { controller_id: "foreign.controller".into(), action: "setActivePanelTab".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "tabId": "s-play-inspector" }) }))
        .expect_err("unclaimed controller reaches the missing guest fixture");
    assert!(!guest_error.starts_with("host-panel."), "unclaimed route must expose the actual missing guest failure: {guest_error}");

    let (home, _) = host_test_apps();
    let home_panel = SpacePanelState { active_panel_tab: "s-play-catalogue".into(), spawned_apps: after.spawned_apps.clone(), active_spawned_id: after.active_spawned_id.clone() };
    let mut home_view = ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    home_view.active_mode_id = Some(home.default_mode_id.clone());
    home_view.active_window_kind_id = Some(home.window_kinds.first().id.clone());
    home_view.panel_json = Some(panel_json(&home_panel).unwrap());
    shell.directory.directory_home = Some(DirectoryHomeProjection::new("space".into(), 88, home.clone(), home_view.clone()).expect("directory home projection"));
    shell.docs.session = Some(ActiveSession { plugin_id: "space".into(), instance_id: 88, app: home, view_state: home_view });
    semio_framework_async::block_on(ActionsCtx::of(&mut shell).dispatch_action(ActionDescriptor { controller_id: "space.home".into(), action: "setActivePanelTab".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "tabId": "home-library" }) }))
        .expect("active host session owns its configured leaf");
    let restored = shell.directory.directory_home.as_ref().unwrap().active_session();
    let restored_panel = panel_state_from_view(&restored.view_state).unwrap().unwrap();
    assert_eq!(restored_panel.active_panel_tab, "home-library");
    assert_eq!(restored_panel.spawned_apps, after.spawned_apps);
    assert_eq!(restored_panel.active_spawned_id, after.active_spawned_id);
    assert!(shell.dock.anchor_open(PanelAnchor::BottomLeft), "a Display-group leaf reveals the bottom-left anchor");
    assert_eq!(shell.dock.anchor_state(PanelAnchor::BottomLeft).active_tab(), Some("home-library"));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn host_panel_view_context_replaces_stale_session_identity_for_every_native_call() {
    let mut shell = host_test_shell();
    let session = shell.docs.session.as_mut().expect("session");
    session.view_state.session_identity = Some(ViewSessionIdentity { user_id: "stale".into(), display_name: "Stale".into() });
    let session = shell.docs.session.as_ref().expect("session").clone();
    assert_eq!(SessionCtx::of(&mut shell).live_view_state(&session).session_identity, None, "an unauthenticated host must remove a persisted stale identity");
    shell.directory.identity = Some(Identity { user_id: "user-a".into(), email: "a@example.test".into(), display_name: "Ada".into(), hub_base_url: "https://hub.example".into(), issued_at_ms: 1 });
    assert_eq!(SessionCtx::of(&mut shell).live_view_state(&session).session_identity, Some(ViewSessionIdentity { user_id: "user-a".into(), display_name: "Ada".into() }));
    shell.directory.identity = Some(Identity { user_id: "user-b".into(), email: "b@example.test".into(), display_name: "Berta".into(), hub_base_url: "https://hub.example".into(), issued_at_ms: 2 });
    assert_eq!(SessionCtx::of(&mut shell).live_view_state(&session).session_identity, Some(ViewSessionIdentity { user_id: "user-b".into(), display_name: "Berta".into() }));
}
