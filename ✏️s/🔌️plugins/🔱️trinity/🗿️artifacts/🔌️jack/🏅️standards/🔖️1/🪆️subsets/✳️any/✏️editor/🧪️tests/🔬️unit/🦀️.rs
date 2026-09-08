
use super::*;
use protocol::{OpBinary, OpText};
use semio_framework_plugin::{App, EditorApp, PluginApp, VcsArtifactApp, ViewModel, testkit};

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
        TrinityJackCommand::RunQuery { query: Some("MATCH (a:Piece) RETURN a".into()) },
        TrinityJackCommand::RunQuery { query: None },
        TrinityJackCommand::SetActiveExample { example_id: "branch-chain".into() },
        TrinityJackCommand::SetViewport { viewport_json: "{\"x\":1.0,\"y\":2.0,\"zoom\":1.0}".into() },
        TrinityJackCommand::TextSelect { start: 3, end: 9 },
        TrinityJackCommand::SetLodMode { window_id: "trinity-jack-graph".into(), value: "compact".into() },
        TrinityJackCommand::SetLocale { value: "de-DE".into() },
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
    app.render(TRINITY_JACK_PLAY_BODY_RESULTS, None, &ViewModel::default()).await.expect("render");
    let result = app.dispatch_typed(TrinityJackCommand::RunQuery { query: Some("MATCH (a:Piece) WHERE a.name = 'b' SET a.label = 'ran-label'".into()) }, &meta("local")).await.expect("run");
    assert!(!result.mutations.is_empty(), "a SET query emits operations");
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
    let result = app.dispatch_typed(TrinityJackCommand::TextEdit { text: "MATCH (a:Piece) RETURN a.name".into() }, &meta("local")).await.expect("edit");
    assert!(result.mutations.is_empty());
    let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &ViewModel::default()).await.expect("render");
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
    app.dispatch_typed(TrinityJackCommand::SetLodMode { window_id: TRINITY_JACK_PLAY_WINDOW_GRAPH.into(), value: "compact".into() }, &meta("local")).await.expect("lod");
    let measures = app.window_measures().await;
    assert!(measures[TRINITY_JACK_PLAY_WINDOW_GRAPH].iter().any(|measure| matches!(measure, WindowMeasure::Select { value, .. } if value == "compact")));
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
    app.dispatch_typed(TrinityJackCommand::SetLocale { value: "de-DE".into() }, &meta("local")).await.expect("set locale");
    let node = app.render(TRINITY_JACK_PLAY_BODY_DOCUMENT, None, &ViewModel::default()).await.expect("render");
    assert!(serde_json::to_string(&node.root).expect("serialize semantic UI test tree").contains("Stücke"));
}

#[semio_framework_async_macros::async_test]
async fn set_active_example_swaps_fixture_and_seeds_query() {
    let mut app = new_app().await;
    let result = app.dispatch_typed(TrinityJackCommand::SetActiveExample { example_id: "branch-chain".into() }, &meta("local")).await.expect("set active example");
    // 🩹 Pre-existing test/implementation mismatch (traced to commit `a445617c`, 2026-08-12
    // 15:50:51 +0200 — predates this migration, not introduced by it): `set_active_example`
    // routes the fixture swap through `Effect::LoadDocument` (whole-document replace is
    // banned from the `Mutation` enum outright), never through `artifact_mutations`, so
    // `InvocationResult.mutations` is always empty for this command — `requested_effects` is
    // the field that actually carries the swap.
    assert!(!result.requested_effects.is_empty());
    let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &ViewModel::default()).await.expect("render");
    assert!(serde_json::to_string(&node.root).expect("serialize semantic UI test tree").contains("RETURN a, r, b"));
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
    let menu = app.context_menu(&request).await;
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
