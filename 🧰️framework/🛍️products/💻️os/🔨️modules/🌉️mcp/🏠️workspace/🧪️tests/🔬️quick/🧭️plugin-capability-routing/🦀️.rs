/// 🧭️ Plugin capability routing: capability resolution, routed channels, multi-app instance slots and the routing channel capability and exchange laws.

#[test]
fn resolve_plugin_for_capability_is_not_found_for_an_unknown_id() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), note_and_cad_catalog(), crate::test_host_driver_policy(), serde_json::from_str(include_str!("../../../../🎚️config/💡️host-operation-policy.json")).expect("explicit host funding policy")).expect("opens");
    let error = workspace.resolve_plugin_for_capability("totally.unknown.capability").expect_err("no such capability");
    assert_eq!(error.code, GatewayErrorCode::NotFound);
}

#[test]
fn resolve_plugin_for_capability_is_plugin_unavailable_for_a_gateway_owned_id() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), note_and_cad_catalog(), crate::test_host_driver_policy(), serde_json::from_str(include_str!("../../../../🎚️config/💡️host-operation-policy.json")).expect("explicit host funding policy")).expect("opens");
    let error = workspace.resolve_plugin_for_capability("capabilities.search").expect_err("gateway-owned, not a plugin");
    assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
    assert!(error.retryable);
}

#[test]
fn resolve_plugin_for_capability_routes_note_and_cad_to_different_plugins() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), note_and_cad_catalog(), crate::test_host_driver_policy(), serde_json::from_str(include_str!("../../../../🎚️config/💡️host-operation-policy.json")).expect("explicit host funding policy")).expect("opens");
    assert_eq!(workspace.resolve_plugin_for_capability("note.editor.setGridVisible").expect("note-owned"), "note");
    assert_eq!(workspace.resolve_plugin_for_capability("cad.editor.addObject").expect("cad-owned"), "cad");
}

/// 🗿️ A routing channel with no artifact bound to any plugin — the shape every routing test below
/// asserts against, since none of them creates an artifact.
fn unbound_plugin_artifacts() -> Arc<Mutex<HashMap<String, PluginArtifactBinding>>> {
    Arc::new(Mutex::new(HashMap::new()))
}

/// 🗿️ Revision stamps identify one bound artifact and refuse ambiguous bindings.
#[test]
fn a_routed_channel_resolves_the_artifact_its_plugin_session_document_is() {
    let bound = unbound_plugin_artifacts();
    let router = RoutingArtifactChannel::new(Arc::new(WorkspaceCatalog::fixed(note_and_cad_catalog())), None, "agent:test#sess".to_string(), Arc::clone(&bound), None, crate::test_host_driver_policy(), serde_json::from_str(include_str!("../../../../🎚️config/💡️host-operation-policy.json")).expect("explicit host funding policy"));
    let note = AppRoute { plugin_id: "note".to_string(), app_id: Some("note.editor".to_string()) };
    let cad = AppRoute { plugin_id: "cad".to_string(), app_id: Some("cad.editor".to_string()) };
    assert_eq!(router.session_artifact_for(&note), None, "nothing bound yet");
    bound
        .lock()
        .expect("binding map")
        .insert("journey-note-typed".to_string(), PluginArtifactBinding { schema: "s.note.note".to_string(), plugin_id: "note".to_string(), app_id: "note.editor".to_string(), surface_id: None, archive: None, document: Some(Arc::new(SessionDocumentPair { pack: vec![1], spr: vec![2] })), backbone: None, backbone_blocked_by: None, relayed: Arc::new(std::sync::atomic::AtomicU64::new(0)), relay: Arc::default(), used: 0 });
    assert_eq!(router.session_artifact_for(&note).as_deref(), Some("journey-note-typed"));
    assert_eq!(router.session_artifact_for(&cad), None, "a plugin with no bound artifact stays unnamed");
    assert_eq!(router.session_artifact_for(&AppRoute { plugin_id: "note".to_string(), app_id: Some("note.viewer".to_string()) }), None, "an artifact of one app is never another app's session document");
    bound
        .lock()
        .expect("binding map")
        .insert("journey-note-second".to_string(), PluginArtifactBinding { schema: "s.note.note".to_string(), plugin_id: "note".to_string(), app_id: "note.editor".to_string(), surface_id: None, archive: None, document: Some(Arc::new(SessionDocumentPair { pack: vec![1], spr: vec![2] })), backbone: None, backbone_blocked_by: None, relayed: Arc::new(std::sync::atomic::AtomicU64::new(0)), relay: Arc::default(), used: 0 });
    assert_eq!(router.session_artifact_for(&note), None, "two artifacts on one app: no single stamp could name either truthfully");
}

/// 🪪️ Every declared app has its own reversible route slot.
#[test]
fn every_app_of_a_multi_app_plugin_routes_to_its_own_instance_slot() {
    let capability = |id: &str, app_id: Option<&str>| CapabilityDefinition {
        id: CapabilityRef(id.to_string()),
        version: 1,
        owner: CapabilityOwner::Plugin { plugin_id: "block".to_string(), label: None, app_id: app_id.map(str::to_string), window_kind_id: None, mode_id: None },
        kind: CapabilityKind::Mutation,
        audience: CapabilityAudience::Agent,
        title: id.to_string(),
        description: "test".to_string(),
        artifact_kind: None,
        use_when: Vec::new(),
        input_schema: serde_json::json!({ "type": "object" }),
        output_schema: serde_json::json!({ "type": "object" }),
        effects: Default::default(),
        policy: Default::default(),
        execution: Default::default(),
        exposure: ToolExposure::CatalogOnly,
        presentation: CapabilityPresentation { icon_id: None, category: None, keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    };
    let mut entries = vec![
        capability("block.s.block.block2d@1/*#editor.addHandle", Some("s.block.block2d@1/*#editor")),
        capability("block.s.block.block3d@1/*#editor.addVortex", Some("s.block.block3d@1/*#editor")),
        capability("block.s.block.block5d@1/*#editor.addGrip", Some("s.block.block5d@1/*#editor")),
        capability("block.importLibrary", None),
    ];
    entries.sort_by(|left, right| left.id.as_str().cmp(right.id.as_str()));
    let catalog = Catalog { hash: "multi-app".to_string(), entries };
    assert_eq!(distinct_routes(&catalog).len(), 4, "three apps plus the plugin's default route");
    let slots: Vec<u32> = ["block.s.block.block2d@1/*#editor.addHandle", "block.s.block.block3d@1/*#editor.addVortex", "block.s.block.block5d@1/*#editor.addGrip", "block.importLibrary"].iter().map(|id| capability_instance_slot(&catalog, id).expect("a plugin capability has a slot")).collect();
    assert_eq!(slots.iter().collect::<std::collections::BTreeSet<_>>().len(), 4, "every app and the plugin scope own a distinct slot: {slots:?}");
    assert_eq!(route_for_slot(&catalog, slots[1]), Some(AppRoute { plugin_id: "block".to_string(), app_id: Some("s.block.block3d@1/*#editor".to_string()) }));
    assert_eq!(route_for_slot(&catalog, slots[3]), Some(AppRoute { plugin_id: "block".to_string(), app_id: None }), "a plugin-scope verb runs on the plugin's default route");
    assert_eq!(plugin_instance_slot(&catalog, "block"), Some(slots[3]));
    assert_eq!(app_instance_slot(&catalog, "block", "s.block.block5d@1/*#editor"), Some(slots[2]), "a bound artifact's app finds the slot its verbs use");
}

#[test]
fn routing_artifact_channel_purecommand_unknown_capability_is_not_found_before_opening_any_channel() {
    let mut router = RoutingArtifactChannel::new(Arc::new(WorkspaceCatalog::fixed(note_and_cad_catalog())), None, "agent:test#sess".to_string(), unbound_plugin_artifacts(), None, crate::test_host_driver_policy(), serde_json::from_str(include_str!("../../../../🎚️config/💡️host-operation-policy.json")).expect("explicit host funding policy"));
    let fault = exchange_blocking(&mut router, 0, vec![AppCommand::PureCommand {  capability_id: "totally.unknown.capability".to_string(), input: serde_json::json!({}), view_state: None }]).expect_err("unknown capability must not route to any plugin");
    assert_eq!(fault.code, "capability.not-found");
}

#[test]
fn routing_artifact_channel_purecommand_gateway_owned_capability_is_plugin_unavailable() {
    let mut router = RoutingArtifactChannel::new(Arc::new(WorkspaceCatalog::fixed(note_and_cad_catalog())), None, "agent:test#sess".to_string(), unbound_plugin_artifacts(), None, crate::test_host_driver_policy(), serde_json::from_str(include_str!("../../../../🎚️config/💡️host-operation-policy.json")).expect("explicit host funding policy"));
    let fault = exchange_blocking(&mut router, 0, vec![AppCommand::PureCommand {  capability_id: "capabilities.search".to_string(), input: serde_json::json!({}), view_state: None }]).expect_err("a gateway-owned capability names no plugin channel");
    assert_eq!(fault.code, "plugin.unavailable");
}

#[test]
fn routing_artifact_channel_exchange_on_an_unrouted_instance_without_a_purecommand_is_plugin_unavailable() {
    let mut router = RoutingArtifactChannel::new(Arc::new(WorkspaceCatalog::fixed(empty_catalog())), None, "agent:test#sess".to_string(), unbound_plugin_artifacts(), None, crate::test_host_driver_policy(), serde_json::from_str(include_str!("../../../../🎚️config/💡️host-operation-policy.json")).expect("explicit host funding policy"));
    let fault = exchange_blocking(&mut router, 0, vec![AppCommand::ReadHistory]).expect_err("no known plugin for this instance and no PureCommand to derive one from");
    assert_eq!(fault.code, "plugin.unavailable");
}

/// 🎫️ Built plugin fixtures verify distinct routes and cached channel reuse; absent WASM fixtures are explicitly reported.
#[test]
fn routing_artifact_channel_routes_two_capabilities_to_two_different_plugins_opening_each_once() {
    let repo_root = match find_repo_root() {
        Ok(root) => root,
        Err(_) => {
            eprintln!("skipped: repo root not found from this test binary's CARGO_MANIFEST_DIR");
            return;
        }
    };
    let registry = match load_plugin_registry(&repo_root) {
        Ok(registry) => registry,
        Err(error) => {
            eprintln!("skipped: plugin registry not generated: {error}");
            return;
        }
    };
    for plugin_id in ["note", "cad"] {
        let Ok(entry) = find_plugin_entry(&registry, plugin_id) else {
            eprintln!("skipped: `{plugin_id}` not in the plugin registry");
            return;
        };
        if resolve_plugin_wasm_path(&repo_root, entry).is_err() {
            eprintln!("skipped: {plugin_id} has no shipped component under its crate's dist/components");
            return;
        }
    }
    let descriptors = ["note", "cad"].map(|plugin_id| load_package_descriptor(&find_plugin_entry(&registry, plugin_id).expect("registered above").owner_root).expect("committed descriptor"));
    let catalog = Arc::new(compile(&CatalogSource { descriptors: descriptors.to_vec(), ..Default::default() }, semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native).expect("the real note+cad descriptors compile"));
    let first_app_verb = |plugin_id: &str| {
        catalog
            .entries
            .iter()
            .find(|entry| matches!(&entry.owner, CapabilityOwner::Plugin { plugin_id: owner, app_id: Some(_), .. } if owner == plugin_id) && entry.kind == CapabilityKind::Mutation)
            .map(|entry| entry.id.as_str().to_string())
            .expect("the plugin declares an app mutation")
    };
    let (note_verb, cad_verb) = (first_app_verb("note"), first_app_verb("cad"));
    let note_instance = capability_instance_slot(&catalog, &note_verb).expect("note's verb has a route slot");
    let cad_instance = capability_instance_slot(&catalog, &cad_verb).expect("cad's verb has a route slot");
    let mut router = RoutingArtifactChannel::new(Arc::new(WorkspaceCatalog::fixed(Arc::clone(&catalog))), Some(PluginComponentSource::Repo(repo_root)), "agent:routing-test#sess".to_string(), unbound_plugin_artifacts(), None, crate::test_host_driver_policy(), serde_json::from_str(include_str!("../../../../🎚️config/💡️host-operation-policy.json")).expect("explicit host funding policy"));

    let note_result = exchange_blocking(&mut router, note_instance, vec![AppCommand::PureCommand {  capability_id: note_verb.clone(), input: serde_json::json!({}), view_state: None }]);
    assert_ne!(note_result.as_ref().err().map(|fault| fault.code.as_str()), Some("plugin.unavailable"), "{note_verb} must route to a real note channel: {note_result:?}");

    let cad_result = exchange_blocking(&mut router, cad_instance, vec![AppCommand::PureCommand {  capability_id: cad_verb.clone(), input: serde_json::json!({}), view_state: None }]);
    assert_ne!(cad_result.as_ref().err().map(|fault| fault.code.as_str()), Some("plugin.unavailable"), "{cad_verb} must route to a real cad channel: {cad_result:?}");

    let _ = exchange_blocking(&mut router, note_instance, vec![AppCommand::ReadHistory]);
    assert_eq!(router.channels.lock().expect("cache lock").len(), 2, "exactly one cached channel per distinct app routed to (note, cad), never one per call");
}

/// ⌨️ The shared browser corpus agrees with the native declaration projection and independent JSON decoder.
#[test]
fn command_owner_addresses_match_shared_language_neutral_corpus() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🌉️agent-artifact-route/🧪️fixtures/🔣️.json")).expect("shared command corpus");
    for case in corpus["commandCases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("capability");
        let actual = corpus["commandDeclarations"].as_array().expect("declarations").iter().find_map(|declaration| PluginArtifactChannel::declared_command_address("note", declaration["appId"].as_str(), declaration["modeId"].as_str(), declaration["commandId"].as_str().expect("command"), id));
        if case["kind"] == "error" { assert!(actual.is_none(), "{id}"); continue; }
        let expected = &case["address"];
        let actual = actual.expect("declared owner");
        assert_eq!(serde_json::Value::from(semio_framework_value::ToValue::to_value(&actual)), *expected, "{id}");
        let bytes = store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&actual));
        let roundtrip: semio_framework::manifest::CommandAddress = semio_framework_value::FromValue::from_value(store::pack_rt::decode_wire_value(&bytes).expect("owned Pack decoder")).expect("owned address decoder");
        assert_eq!(roundtrip, actual, "{id}");
    }
}

/// 🧑‍💻️ The genuine pure-frame codec and independent JSON parser share the context admission corpus.
#[test]
fn pure_command_context_matches_independent_json_oracle() {
    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    enum ReferenceInvocation { Action(serde_json::Value), Command(serde_json::Value) }
    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    struct ReferenceRequest { invocation: ReferenceInvocation, view_state: Option<semio_framework::ViewModel> }
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🎠️kernel/🧪️tests/⌨️pure-command/🧫️fixtures/🔣️.json")).expect("pure invocation corpus");
    for row in corpus["cases"].as_array().expect("cases") {
        let json = row["request"].to_string();
        let expected = row["valid"].as_bool().expect("expected admission");
        let independent = serde_json::from_str::<ReferenceRequest>(&json);
        let owned = semio_framework_pack_json::from_json_str::<semio_framework::kernel::PureCommandRequest>(&json, semio_framework_pack_json::JsonMemberPolicy::Reject);
        assert_eq!(independent.is_ok(), expected, "{}", row["id"]);
        assert_eq!(owned.is_ok(), expected, "{}", row["id"]);
        if let Ok(actual) = owned {
            let reference = independent.expect("independent decode");
            assert_eq!(actual.view_state, reference.view_state);
            let expected = match reference.invocation { ReferenceInvocation::Action(value) => serde_json::json!({"action":value}), ReferenceInvocation::Command(value) => serde_json::json!({"command":value}) };
            assert_eq!(serde_json::Value::from(semio_framework_value::ToValue::to_value(&actual.invocation)), expected);
            let bytes = store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&actual));
            let roundtrip: semio_framework::kernel::PureCommandRequest = semio_framework_value::FromValue::from_value(store::pack_rt::decode_wire_value(&bytes).expect("owned Pack")).expect("owned pure envelope");
            assert_eq!(roundtrip, actual);
        }
    }
}

/// 🪟️ Exact explicit or retained context binds the actual preview target, as the independent relational corpus requires.
#[test]
fn pure_command_binds_the_actual_caller_context() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🎠️kernel/🧪️tests/⌨️pure-command/🧫️fixtures/🔣️.json")).expect("context corpus");
    for row in corpus["contextCases"].as_array().expect("context cases") {
        let mut source = corpus["cases"][row["request"].as_u64().expect("source") as usize]["request"].clone();
        if let Some(context) = row.get("context") { source["viewState"] = context.clone(); }
        let request = semio_framework_pack_json::from_json_str::<semio_framework::kernel::PureCommandRequest>(&source.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("owned request");
        let retained = row.get("retained").map(|context| serde_json::from_value::<semio_framework::ViewModel>(context.clone()).expect("independent context"));
        let result = request.bind_context(retained.as_ref());
        if let Some(expected) = row["expectedFault"].as_str() {
            let fault = result.expect_err("refused target");
            assert_eq!(fault.code.0, expected, "{}", row["id"]);
            if expected == "input.invalid" { assert_eq!(semio_os_mcp_port::channel_port::map_fault(&Fault { code: expected.into(), message: fault.message.clone() }).code, GatewayErrorCode::InputInvalid); }
            continue;
        }
        let actual = result.expect("bound caller");
        let expected: semio_framework::ViewModel = serde_json::from_value(row["expectedContext"].clone()).expect("independent expected context");
        assert_eq!(actual.view_state, Some(expected), "{}", row["id"]);
        if let Some(window) = row["expectedWindow"].as_str() {
            let semio_framework::kernel::PureInvocation::Action(invocation) = actual.invocation else { panic!("window addresses an action") };
            assert_eq!(invocation.address.window_instance_id, window);
            assert_eq!(invocation.address.mode_id, row["expectedMode"].as_str().expect("mode"));
        }
    }
}
