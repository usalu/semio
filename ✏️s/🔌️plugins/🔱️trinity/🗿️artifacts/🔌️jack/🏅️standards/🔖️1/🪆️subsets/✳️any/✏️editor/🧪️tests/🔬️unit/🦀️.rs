use super::*;
use protocol::{OpBinary, OpText};
use semio_framework_plugin::{testkit, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel};

/// 🎫️ `testkit::assert_declared_actions_bridge_to_commands`/`new_app_with_registry`'s own signature
/// is still `fn(manifest: fn() -> App)`, unchanged for this ticket (SDK gap, see this packet's
/// notes file) — `create_trinity_jack_app` now returns a bare `AppDefinition`, so this tiny local
/// wrapper adapts it back into the `App { definition, examples }` shape those testkit fns expect.
fn trinity_jack_manifest_for_testkit() -> App {
    App { definition: create_trinity_jack_app(), examples: Vec::new() }
}

/// 🎫️ Permanent wire guard (TEMPLATE.md §7): every `TrinityJackCommand` variant round-trips
/// through both its binary (`OpBinary`, via `#[derive(dsl::DslOps)]`) and text (`OpText`) codecs.
#[semio_framework_async_macros::async_test]
async fn trinity_jack_command_text_and_binary_round_trip() {
    let commands = vec![
        TrinityJackCommand::SetFixtureJson { json: "{}".into() },
        TrinityJackCommand::DeleteSelection,
        TrinityJackCommand::PatchNodes { node_ids: vec!["a".into()], field: "name".into(), value: "Renamed".into() },
        TrinityJackCommand::Reorganize,
        TrinityJackCommand::RunQuery { query: Some("MATCH (a:Piece) RETURN a".into()), results_window_id: "results".into() },
        TrinityJackCommand::RunQuery { query: None, results_window_id: "results".into() },
        TrinityJackCommand::LoadExampleQuery { query: "MATCH (a:Piece) RETURN a.name".into(), results_window_id: "results".into() },
        TrinityJackCommand::FormatDocument,
        TrinityJackCommand::SetActiveExample { example_id: "branch-chain".into() },
        TrinityJackCommand::SetViewport { viewport_json: "{\"x\":1.0,\"y\":2.0,\"zoom\":1.0}".into() },
        TrinityJackCommand::TextSelect { start: 3, end: 9 },
        TrinityJackCommand::SetLodMode { value: "compact".into() },
    ];
    for command in commands {
        let bytes = command.encode_op().expect("encode");
        assert_eq!(TrinityJackCommand::decode_op(&bytes).expect("decode"), command);
        let text = command.print_op();
        assert_eq!(TrinityJackCommand::parse_op(&text).expect("parse"), command);
    }
}

fn meta(actor: &str) -> semio_framework_plugin::ActionMeta {
    testkit::meta(actor)
}

fn query_windows() -> ViewModel {
    ViewModel {
        window_instances: vec![
            semio_framework_plugin::ViewWindowInstance { id: "editor-main".into(), window_kind_id: TRINITY_JACK_PLAY_WINDOW_EDITOR.into() },
            semio_framework_plugin::ViewWindowInstance { id: "results-main".into(), window_kind_id: TRINITY_JACK_PLAY_WINDOW_RESULTS.into() },
        ],
        ..Default::default()
    }
}

/// 🕹️ Registry-backed (not the bare `testkit::new_app`): `interactionSelect`/`interactionHover`
/// resolve the dispatching app's declared `AppActionRegistry.interactions`, so any test exercising
/// domain "ast" selection needs the real manifest's `.interaction(...)` declaration present.
async fn new_app() -> VcsArtifactApp<EditorApp<TrinityJackPlayApp>> {
    testkit::new_app_with_registry::<EditorApp<TrinityJackPlayApp>>(trinity_jack_manifest_for_testkit).await
}

fn jack_envelope_wire() -> Vec<u8> {
    use store::ArtifactPack;

    let snapshot = crate::empty_trinity_graph_fixture();
    let snapshot_pack = snapshot.encode_pack();
    let snapshot_hex = snapshot_pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let wire = pack::json_to_string(&pack::json!({
        "schema": TRINITY_GRAPH_SCHEMA,
        "id": "jack-live-load",
        "vcs": {
            "initialSnapshot": snapshot_hex,
            "edits": [],
            "changes": [],
            "checkpoints": [],
            "alternatives": []
        },
        "editMessages": [],
        "conflicts": []
    }))
    .into_bytes();
    let envelope = store::create_document_envelope(TRINITY_GRAPH_SCHEMA, "jack-live-load", snapshot, None);
    let mut retirement = crate::standards::v1::subsets::any::schema::mutations::binary::jack_envelope_decode_owner_bundle().retire_envelope(envelope);
    for _ in 0..100_000 {
        match retirement.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Jack fixture envelope retirement") {
            store::SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                drop(retirement);
                return wire;
            }
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
            }
            store::SnapshotRetirementStep::Blocked => panic!("unshared Jack fixture envelope retirement blocked"),
        }
    }
    panic!("Jack fixture envelope retirement did not reach terminal")
}

fn admit_jack_envelope(app: &mut VcsArtifactApp<EditorApp<TrinityJackPlayApp>>, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
    let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("Jack live envelope ingress credits");
    for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
        let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..chunk.len()].copy_from_slice(chunk);
        let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded Jack live envelope page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("Jack live envelope page admission failed: {fault:?}"));
    }
    assert!(app.seal_artifact_envelope_ingress(handle).expect("Jack live envelope seal/submit"));
    handle
}

fn drive_jack_live_load(app: &mut VcsArtifactApp<EditorApp<TrinityJackPlayApp>>, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
    for _ in 0..100_000 {
        app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one Jack live maintenance turn");
        let poll = app.advance_artifact_envelope_load(handle).expect("Jack live load advancement");
        if matches!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
            return poll;
        }
        std::thread::yield_now();
    }
    panic!("Jack live envelope load did not reach terminal")
}

#[semio_framework_async_macros::async_test]
async fn jack_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed() {
    let mut app = new_app().await;
    let base_generation = app.artifact_generation_now();
    let handle = admit_jack_envelope(&mut app, &jack_envelope_wire());
    assert_eq!(handle.generation, base_generation);
    assert_eq!(drive_jack_live_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert_eq!(app.artifact_generation_now().0, base_generation.0 + 1);
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("first exact Jack load acknowledgement"));
    assert!(!app.acknowledge_artifact_store_replacement(handle).expect("duplicate Jack load acknowledgement is a no-op"));
}

#[semio_framework_async_macros::async_test]
async fn jack_live_envelope_cancel_closes_retained_pages_without_publication() {
    let mut app = new_app().await;
    let base_generation = app.artifact_generation_now();
    let wire = jack_envelope_wire();
    let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len()).expect("cancelled Jack ingress credits");
    let first = &wire[..wire.len().min(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)];
    let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
    bytes[..first.len()].copy_from_slice(first);
    let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, first.len()).expect("cancelled Jack first page");
    app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("cancelled Jack page admission failed: {fault:?}"));
    app.cancel_artifact_envelope_load(handle).expect("cancel exact Jack ingress");
    assert_eq!(drive_jack_live_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
    assert_eq!(app.artifact_generation_now(), base_generation);
}

fn node_id_at(app: &VcsArtifactApp<EditorApp<TrinityJackPlayApp>>, index: usize) -> String {
    app.snapshot().expect("projection").nodes()[index].id.clone()
}

/// 🕹️ Dispatches the framework-injected `interactionSelect` verb against domain "ast" — the
/// replacement for the deleted `TrinityJackCommand::SetSelection`.
async fn select_ast(app: &mut VcsArtifactApp<EditorApp<TrinityJackPlayApp>>, ids: &[&str]) {
    let targets: Vec<pack::JsonValue> = ids.iter().map(|id| pack::json!({ "granularity": "node", "id": id })).collect();
    let args = pack::json_to_dsl_value(&pack::json!({ "domainId": "ast", "targets": pack::to_json_string(&targets) }));
    app.handle_action("interactionSelect", Some(&args), &meta("local")).await.expect("interactionSelect");
}

#[semio_framework_async_macros::async_test]
async fn renders_node_graph_scene() {
    let mut app = new_app().await;
    let node = app.render(TRINITY_JACK_PLAY_BODY_GRAPH, None, &ViewModel::default()).await.expect("render");
    assert!(serde_json::to_string(&node.root).expect("serialize semantic UI test tree").contains("node-graph"));
}

#[semio_framework_async_macros::async_test]
async fn renders_jack_editor() {
    let mut app = new_app().await;
    let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &ViewModel::default()).await.expect("render");
    let json = serde_json::to_string(&node.root).expect("serialize semantic UI test tree");
    assert!(json.contains("text-editor"));
    assert!(json.contains(TRINITY_JACK_DEFAULT_QUERY));
}

#[semio_framework_async_macros::async_test]
async fn run_query_populates_results_and_a_set_query_mutates_projection() {
    let mut app = new_app().await;
    let view = query_windows();
    let editor = view.for_window_instance("editor-main").unwrap();
    app.dispatch_typed(
        TrinityJackCommand::RunQuery { query: Some("MATCH (a:Piece) WHERE a.name = 'b' SET a.label = 'ran-label'".into()), results_window_id: "results-main".into() },
        &semio_framework_plugin::ActionMeta { view_state: Some(editor), ..meta("local") },
    )
    .await
    .expect("run");
    drive_query_ownership_operations(&mut app).await.expect("query completes");
    let projection = app.snapshot().expect("projection");
    // 🔬 `content` is now an opaque composed-child handle — `pack::to_json_string(&projection)`
    // no longer surfaces node property data directly (ticket
    // `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`); inspect through the working-scene
    // accessor instead of the raw derived JSON serialization.
    assert!(projection.nodes().iter().any(|node| node.properties.get("label") == Some(&crate::PropertyValue::String("ran-label".into()))));
}

#[semio_framework_async_macros::async_test]
async fn node_graph_select_updates_selection_and_document_tree() {
    let mut app = new_app().await;
    let node_id = node_id_at(&app, 0);
    select_ast(&mut app, &[&node_id]).await;
    let tree = app.render(TRINITY_JACK_PLAY_BODY_DOCUMENT, None, &ViewModel::default()).await.expect("render");
    let json = serde_json::to_string(&tree.root).expect("serialize semantic UI test tree");
    assert!(json.contains(&node_id));
    assert!(json.contains("\"selected\":true"));
}

#[semio_framework_async_macros::async_test]
async fn nakagin_fixture_has_nodes() {
    assert!(!default_fixture().nodes().is_empty());
}

#[semio_framework_async_macros::async_test]
async fn editor_scene_has_tokens_and_diagnostics() {
    let mut app = new_app().await;
    let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &ViewModel::default()).await.expect("render");
    let json = serde_json::to_string(&node.root).expect("serialize semantic UI test tree");
    assert!(json.contains("tokensJson"));
    assert!(json.contains("diagnosticsJson"));
    assert!(json.contains("completionsJson"));
}

#[semio_framework_async_macros::async_test]
async fn text_edit_updates_query_without_operations() {
    let mut app = new_app().await;
    let view = query_windows();
    let editor = view.for_window_instance("editor-main").unwrap();
    let result = app
        .dispatch_typed(TrinityJackCommand::TextEdit { text: "MATCH (a:Piece) RETURN a.name".into() }, &semio_framework_plugin::ActionMeta { view_state: Some(editor.clone()), ..meta("local") })
        .await
        .expect("edit");
    assert!(result.mutations.is_empty());
    drive_query_ownership_operations(&mut app).await.expect("edit completes");
    let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &editor).await.expect("render");
    assert!(serde_json::to_string(&node.root).expect("serialize semantic UI test tree").contains("MATCH (a:Piece) RETURN a.name"));
}

#[semio_framework_async_macros::async_test]
async fn graph_scene_has_lod_json() {
    let mut app = new_app().await;
    let node = app.render(TRINITY_JACK_PLAY_BODY_GRAPH, None, &ViewModel::default()).await.expect("render");
    let json = serde_json::to_string(&node.root).expect("serialize semantic UI test tree");
    assert!(json.contains("lodJson"));
    assert!(json.contains("automatic"));
}

#[semio_framework_async_macros::async_test]
async fn set_lod_mode_reflects_in_window_measures() {
    let mut app = new_app().await;
    let view = ViewModel { window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: "jack-graph-main".into(), window_kind_id: TRINITY_JACK_PLAY_WINDOW_GRAPH.into() }], ..Default::default() };
    let addressed = view.for_window_instance("jack-graph-main").unwrap();
    app.dispatch_typed(TrinityJackCommand::SetLodMode { value: "compact".into() }, &semio_framework_plugin::ActionMeta { view_state: Some(addressed), ..meta("local") }).await.expect("lod");
    while app.has_pending_typed_operations() {
        app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("maintenance");
        app.advance_typed_operation_publication().await.expect("publish");
        if let Some(page) = app.take_typed_operation_result_page(1) {
            assert_ne!(page.lane, semio_framework_plugin::app::TypedOperationResultLane::Fault);
            app.acknowledge_typed_operation_result(page.token).expect("acknowledge");
        }
    }
    let measures = app.window_measures(&view).await;
    assert!(measures["jack-graph-main"].iter().any(|measure| matches!(measure, WindowMeasure::Select { value, .. } if value == "compact")));
}

#[semio_framework_async_macros::async_test]
async fn catalogue_tree_renders() {
    let mut app = new_app().await;
    let node = app.render(TRINITY_JACK_PLAY_BODY_CATALOGUE, None, &ViewModel::default()).await.expect("render");
    assert!(serde_json::to_string(&node.root).expect("serialize semantic UI test tree").contains("trinity-jack-catalogue"));
}

#[semio_framework_async_macros::async_test]
async fn inspection_panel_renders_the_selection_prompt() {
    let mut app = new_app().await;
    let node_id = node_id_at(&app, 0);
    select_ast(&mut app, &[&node_id]).await;
    let node = app.render(TRINITY_JACK_PLAY_BODY_INSPECTION, None, &ViewModel::default()).await.expect("render");
    // 🕹️ `render` has no `InteractionView` (see the panel's own doc comment) — it can no longer
    // build per-selection fields, so it always renders the static prompt.
    assert!(serde_json::to_string(&node.root).expect("serialize semantic UI test tree").contains("trinity-inspector.empty"));
}

#[semio_framework_async_macros::async_test]
async fn document_tree_de_locale_translates_labels() {
    let mut app = new_app().await;
    let view = ViewModel { locale: semio_framework_plugin::Locale::De, ..ViewModel::default() };
    let node = app.render(TRINITY_JACK_PLAY_BODY_DOCUMENT, None, &view).await.expect("render");
    assert!(serde_json::to_string(&node.root).expect("serialize semantic UI test tree").contains("Stücke"));
}

#[semio_framework_async_macros::async_test]
async fn set_active_example_swaps_fixture_without_changing_editor_query() {
    let mut app = new_app().await;
    let result = app.dispatch_typed(TrinityJackCommand::SetActiveExample { example_id: "branch-chain".into() }, &meta("local")).await.expect("set active example");
    // 🩹 Pre-existing test/implementation mismatch (traced to commit `a445617c`, 2026-08-12
    // 15:50:51 +0200 — predates this migration, not introduced by it): `set_active_example`
    // routes the fixture swap through `Effect::LoadDocument` (whole-document replace is
    // banned from the `Mutation` enum outright), never through `artifact_mutations`, so
    // `InvocationResult.mutations` is always empty for this command — `requested_effects` is
    // the field that actually carries the swap.
    assert!(!result.requested_effects.is_empty());
    let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &query_windows().for_window_instance("editor-main").unwrap()).await.expect("render");
    assert!(serde_json::to_string(&node.root).expect("serialize semantic UI test tree").contains(TRINITY_JACK_DEFAULT_QUERY));
}

#[semio_framework_async_macros::async_test]
async fn delete_selection_removes_selected_node() {
    let mut app = new_app().await;
    let node_id = node_id_at(&app, 0);
    select_ast(&mut app, &[&node_id]).await;
    let result = app.dispatch_typed(TrinityJackCommand::DeleteSelection, &meta("local")).await.expect("delete");
    assert!(!result.mutations.is_empty());
    let projection = app.snapshot().expect("projection");
    assert!(!projection.nodes().iter().any(|node| node.id == node_id));
}

#[semio_framework_async_macros::async_test]
async fn context_menu_stays_within_row_budget_and_ends_with_delete_selection() {
    let mut app = testkit::new_app_with_registry::<EditorApp<TrinityJackPlayApp>>(trinity_jack_manifest_for_testkit).await;
    let node_id = node_id_at(&app, 0);
    let request = ContextMenuRequest {
        menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None },
        surface: Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: TRINITY_JACK_PLAY_SURFACE_GRAPH.into(),
            kind: "nodeGraph".into(),
            hits: vec![semio_framework_plugin::ContextMenuHit { domain: "node".into(), id: node_id.clone(), label: None }],
            selection: vec![semio_framework_plugin::ContextMenuSelectionGroup { domain: "node".into(), ids: vec![node_id] }],
            text: None,
        }),
        window_instance_id: None,
        point: None,
    };
    let menu = app.context_menu(&request, &ViewModel::default()).await;
    assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
    let last = menu.last().expect("grouped disclosure menu should not be empty");
    let last_is_destructive_leaf = last.id == "delete-selection" && last.destructive == Some(true) && last.action.as_deref() == Some("deleteSelection");
    let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).is_some_and(|child| child.destructive == Some(true));
    assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive deleteSelection must be last: {menu:?}");
}

#[semio_framework_async_macros::async_test]
async fn export_media_graph_out_matches_document_pack() {
    use semio_framework_plugin::PluginApp as _;
    let mut app = new_app().await;
    let document_out = app.export_media("document:out").await.expect("document:out export");
    let graph_out = app.export_media("graph:out").await.expect("graph:out export");
    assert_eq!(document_out.payload, graph_out.payload);
}

#[semio_framework_async_macros::async_test]
async fn jack_io_declares_graph_out_fan_out_port() {
    let io = jack_io();
    assert_eq!(io.document_schema, TRINITY_GRAPH_SCHEMA);
    assert_eq!(io.artifact.id, "graph.trinity");
    let graph_out = io.ports.iter().find(|port| port.id == "graph:out").expect("graph:out declared");
    assert_eq!(graph_out.kind_id.as_deref(), Some("graph.trinity"));
    assert_eq!(graph_out.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
}

#[semio_framework_async_macros::async_test]
async fn query_ownership_runtime_publishes_transient_result_without_document_edit() {
    let mut app = new_app().await;
    let outcome: Result<(u64, String), String> = async {
        let before = app.ephemeral_snapshot().await.transient_generation;
        let document = app.snapshot().map_err(|error| format!("{error:?}"))?.clone();
        let expected_name = document.nodes().into_iter().find(|node| node.kind == "Piece").ok_or_else(|| "query runtime fixture has no Piece".to_string())?.name;
        let view = query_windows();
        let editor = view.for_window_instance("editor-main").ok_or("missing editor window")?;
        let results = view.for_window_instance("results-main").ok_or("missing results window")?;
        app.dispatch_typed(
            TrinityJackCommand::RunQuery { query: Some("MATCH (a:Piece) RETURN a.name".into()), results_window_id: "results-main".into() },
            &semio_framework_plugin::ActionMeta { view_state: Some(editor), ..meta("query-owner") },
        )
        .await
        .map_err(|error| format!("{error:?}"))?;
        if drive_query_ownership_operations(&mut app).await? != (0, 1, 1) {
            return Err("query operation did not produce one editor-config and one results-transient receipt".into());
        }
        if app.snapshot().map_err(|error| format!("{error:?}"))? != document {
            return Err("read query modified the document".into());
        }
        let generation = app.window_transient_generation(&results).map_err(|error| format!("{error:?}"))?.ok_or("missing results transient generation")?;
        if generation != 1 || app.ephemeral_snapshot().await.transient_generation != before {
            return Err("query result did not publish exactly once to only the results-window transient".into());
        }
        let tree = app.render(TRINITY_JACK_PLAY_BODY_RESULTS, None, &results).await.map_err(|error| format!("{error:?}"))?;
        let rendered = testkit::project_and_retire_fixture_tree(tree).map_err(str::to_string)?;
        if !rendered.contains(&expected_name) {
            return Err("query result table did not contain the document's matching Piece".into());
        }
        Ok((generation, rendered))
    }
    .await;
    testkit::close_registered_fixture_app(&mut app);
    let (generation, rendered) = outcome.expect("owned query runtime");
    assert!(rendered.contains("table"));
    eprintln!("[DEBUG] query result reached results-window transient generation {generation}, rendered as a table, preserved the document, and retired the app");
}

async fn drive_query_ownership_operations(app: &mut VcsArtifactApp<EditorApp<TrinityJackPlayApp>>) -> Result<(u64, u64, u64), String> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    let mut transient_receipts = 0;
    let mut window_config_receipts = 0;
    let mut window_transient_receipts = 0;
    while app.has_pending_typed_operations() {
        if std::time::Instant::now() >= deadline {
            return Err("query operations did not finish".into());
        }
        app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|error| format!("{error:?}"))?;
        app.advance_typed_operation_publication().await.map_err(|error| format!("{error:?}"))?;
        if let Some(page) = app.take_typed_operation_result_page(1) {
            let fault = (page.lane == semio_framework_plugin::app::TypedOperationResultLane::Fault).then(|| format!("query publication fault: {:?}", page.bytes()));
            transient_receipts += u64::from(page.lane == semio_framework_plugin::app::TypedOperationResultLane::Transient);
            window_config_receipts += u64::from(page.lane == semio_framework_plugin::app::TypedOperationResultLane::WindowConfig);
            window_transient_receipts += u64::from(page.lane == semio_framework_plugin::app::TypedOperationResultLane::WindowTransient);
            app.acknowledge_typed_operation_result(page.token).map_err(|error| format!("{error:?}"))?;
            if let Some(fault) = fault {
                return Err(fault);
            }
        }
        app.take_typed_operation_effect();
        app.take_typed_operation_event();
        app.take_typed_operation_ui_scope();
        std::thread::yield_now();
    }
    Ok((transient_receipts, window_config_receipts, window_transient_receipts))
}

#[semio_framework_async_macros::async_test]
async fn jack_graph_window_config_query_ownership_isolates_two_editor_result_pairs_and_reloads_only_authored_sources() {
    let mut app = new_app().await;
    let view = ViewModel {
        window_instances: vec![
            semio_framework_plugin::ViewWindowInstance { id: "editor-left".into(), window_kind_id: TRINITY_JACK_PLAY_WINDOW_EDITOR.into() },
            semio_framework_plugin::ViewWindowInstance { id: "editor-right".into(), window_kind_id: TRINITY_JACK_PLAY_WINDOW_EDITOR.into() },
            semio_framework_plugin::ViewWindowInstance { id: "results-left".into(), window_kind_id: TRINITY_JACK_PLAY_WINDOW_RESULTS.into() },
            semio_framework_plugin::ViewWindowInstance { id: "results-right".into(), window_kind_id: TRINITY_JACK_PLAY_WINDOW_RESULTS.into() },
        ],
        ..Default::default()
    };
    let editor_left = view.for_window_instance("editor-left").unwrap();
    let editor_right = view.for_window_instance("editor-right").unwrap();
    let results_left = view.for_window_instance("results-left").unwrap();
    let results_right = view.for_window_instance("results-right").unwrap();
    let left_query = "MATCH (a:Piece) WHERE a.name = 'b' RETURN a.name";
    let right_query = "MATCH (a:Piece) WHERE a.name != 'b' RETURN a.name";
    let document_before = app.snapshot().expect("document").clone();
    let app_config_before = app.config_pack().await.expect("app config pack");
    let denied = app
        .dispatch_typed(
            TrinityJackCommand::RunQuery { query: Some(left_query.into()), results_window_id: "editor-right".into() },
            &semio_framework_plugin::ActionMeta { view_state: Some(editor_left.clone()), ..meta("query-owner") },
        )
        .await;
    assert!(denied.is_err(), "an attached editor cannot be promoted to results mutation authority by command payload");

    for (context, query) in [(&editor_left, left_query), (&editor_right, right_query)] {
        app.dispatch_typed(
            TrinityJackCommand::TextEdit { text: query.into() },
            &semio_framework_plugin::ActionMeta { view_state: Some(context.clone()), ..meta("query-owner") },
        )
        .await
        .expect("addressed query edit");
    }
    assert_eq!(drive_query_ownership_operations(&mut app).await.expect("query edits"), (0, 2, 0));

    for (context, target) in [(&editor_left, "results-left"), (&editor_right, "results-right")] {
        app.dispatch_typed(
            TrinityJackCommand::RunQuery { query: None, results_window_id: target.into() },
            &semio_framework_plugin::ActionMeta { view_state: Some(context.clone()), ..meta("query-owner") },
        )
        .await
        .expect("paired query");
    }
    assert_eq!(drive_query_ownership_operations(&mut app).await.expect("paired queries"), (0, 2, 2));
    assert_eq!(app.snapshot().expect("document after queries"), document_before);
    let app_config_after = app.config_pack().await.expect("app config after");
    assert_eq!(app_config_after.pack, app_config_before.pack);
    assert_eq!(app_config_after.spr, app_config_before.spr);
    assert_eq!(app.ephemeral_snapshot().await.transient_generation, 0);
    assert_eq!(app.window_config_generation(&editor_left).await.expect("left query generation"), Some(2));
    assert_eq!(app.window_config_generation(&editor_right).await.expect("right query generation"), Some(2));
    assert_eq!(app.window_transient_generation(&results_left).expect("left result generation"), Some(1));
    assert_eq!(app.window_transient_generation(&results_right).expect("right result generation"), Some(1));

    let left_snapshot = app.window_transient_snapshot(&results_left).expect("left result snapshot").expect("left result owner");
    let right_snapshot = app.window_transient_snapshot(&results_right).expect("right result snapshot").expect("right result owner");
    let left_state = left_snapshot.get::<JackResultsWindowTransientOwner>().expect("left result state");
    let right_state = right_snapshot.get::<JackResultsWindowTransientOwner>().expect("right result state");
    assert!(left_state.query_error.is_none() && right_state.query_error.is_none());
    assert_ne!(left_state.result, right_state.result, "concurrent executions must keep distinct result payloads");
    drop(left_snapshot);
    drop(right_snapshot);

    for (context, expected) in [(&editor_left, left_query), (&editor_right, right_query)] {
        let tree = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, context).await.expect("editor render");
        let rendered = testkit::project_and_retire_fixture_tree(tree).expect("editor projection");
        assert!(rendered.contains(expected));
    }
    for context in [&results_left, &results_right] {
        let tree = app.render(TRINITY_JACK_PLAY_BODY_RESULTS, None, context).await.expect("result render");
        assert!(testkit::project_and_retire_fixture_tree(tree).expect("result projection").contains("table"));
    }

    let packs = app.window_config_packs().await.expect("persisted window configs");
    assert_eq!(packs.len(), 2, "only two authored editor query configs were instantiated");
    testkit::close_registered_fixture_app(&mut app);

    let mut reopened = new_app().await;
    for pack in packs {
        reopened.load_window_config_pack(pack).await.expect("reload editor query config");
    }
    for (context, expected) in [(&editor_left, left_query), (&editor_right, right_query)] {
        let tree = reopened.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, context).await.expect("reloaded editor render");
        assert!(testkit::project_and_retire_fixture_tree(tree).expect("reloaded editor projection").contains(expected));
    }
    for context in [&results_left, &results_right] {
        assert_eq!(reopened.window_transient_generation(context).expect("fresh result generation"), Some(0));
        let snapshot = reopened.window_transient_snapshot(context).expect("fresh result snapshot").expect("fresh results owner");
        let state = snapshot.get::<JackResultsWindowTransientOwner>().expect("fresh results state");
        assert!(state.query_execution_id.is_none() && state.result.is_none() && state.query_error.is_none());
    }
    testkit::close_registered_fixture_app(&mut reopened);
    eprintln!("[DEBUG] two editor/result pairs kept query source and output isolated; reload restored only the authored editor configs and reset both results transients");
}
