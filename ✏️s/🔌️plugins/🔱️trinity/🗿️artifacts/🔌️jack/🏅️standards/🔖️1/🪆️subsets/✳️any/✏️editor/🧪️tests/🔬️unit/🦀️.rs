use super::*;
use protocol::{OpBinary, OpText};
use semio_framework_plugin::{artifact_app_laws, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel};

/// 🎫️ `artifact_app_laws::assert_declared_actions_bridge_to_commands`/`new_app_with_registry`'s own signature
/// is still `fn(manifest: fn() -> App)`, unchanged for this ticket (SDK gap, see this packet's
/// notes file) — `create_trinity_jack_app` now returns a bare `AppDefinition`, so this tiny local
/// wrapper adapts it back into the `App { definition, examples }` shape those test context fns expect.
fn trinity_jack_manifest_for_tests() -> App {
    App { definition: create_trinity_jack_app(), examples: Vec::new() }
}

/// 🎫️ Permanent wire guard (TEMPLATE.md §7): every `TrinityJackCommand` variant round-trips
/// through both its binary (`OpBinary`, via `#[derive(dsl::DslOps)]`) and text (`OpText`) codecs.
#[semio_framework_async_macros::async_test]
async fn trinity_jack_command_text_and_binary_round_trip() {
    let commands = vec![
        TrinityJackCommand::LoadDocumentJson { json: "{}".into() },
        TrinityJackCommand::DeleteSelection,
        TrinityJackCommand::PatchNodes { node_ids: vec!["a".into()], field: "name".into(), value: "Renamed".into() },
        TrinityJackCommand::RunQuery { query: Some("MATCH (a:Piece) RETURN a".into()), results_window_id: "results".into() },
        TrinityJackCommand::RunQuery { query: None, results_window_id: "results".into() },
        TrinityJackCommand::LoadExampleQuery { query: "MATCH (a:Piece) RETURN a.name".into(), results_window_id: "results".into() },
        TrinityJackCommand::FormatDocument,
        TrinityJackCommand::SetActiveExample { example_id: "branch-chain".into() },
        TrinityJackCommand::SetViewport { surface_id: Some("trinity.jack.play".into()), viewport: semio_framework_os_kernel::Viewport2d { x: 1.0, y: 2.0, zoom: 1.0 } },
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
    artifact_app_laws::meta(actor)
}

fn query_windows() -> ViewModel {
    ViewModel {
        window_instances: vec![
            semio_framework_plugin::ViewWindowInstance { id: "editor-main".into(), window_kind_id: TRINITY_JACK_PLAY_WINDOW_EDITOR.into() },
            semio_framework_plugin::ViewWindowInstance { id: "results-main".into(), window_kind_id: TRINITY_JACK_PLAY_WINDOW_RESULTS.into() },
        ],
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    }
}

/// 🕹️ Registry-backed (not the bare `artifact_app_laws::new_app`): `interactionSelect`/`interactionHover`
/// resolve the dispatching app's declared `AppActionRegistry.interactions`, so any test exercising
/// domain "ast" selection needs the real manifest's `.interaction(...)` declaration present.
/// 🧩️ Members-aware (`SemioMembers`, not `NoMembers`): the snapshot composes the `s.stdio.semio` content
/// child `genesis_child_pack` derives, which a `NoMembers` roster can never open.
/// 🔌️ Mounted (`bind_instance_id`): `dispatch_typed` refuses `interactive-job.live-instance` for an
/// unbound app, and the mounted app answers BEFORE its retained typed operation publishes, so a
/// dispatching test settles through `settle(&mut app)` before reading the projection.
/// 🔚 Self-closing: the store's `Drop` demands the terminal-empty witness, so the guard retires the
/// app through the framework's exact close loop unless the test already closed it (or is unwinding).
async fn new_app() -> JackTestApp {
    let mut app = artifact_app_laws::new_app_with_registry_and_members::<EditorApp<TrinityJackPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(trinity_jack_manifest_for_tests).await;
    app.bind_instance_id(JACK_TEST_INSTANCE).await;
    JackTestApp { app, closed: false }
}

const JACK_TEST_INSTANCE: u32 = 1;
const JACK_LIVE_LOAD_STEP_BYTES: usize = 32 * 1_024;

pub(crate) struct JackTestApp {
    app: VcsArtifactApp<EditorApp<TrinityJackPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>,
    closed: bool,
}

impl JackTestApp {
    fn close(&mut self) {
        if !self.closed {
            self.closed = true;
            artifact_app_laws::close_registered_fixture_app(&mut self.app);
        }
    }
}

impl std::ops::Deref for JackTestApp {
    type Target = VcsArtifactApp<EditorApp<TrinityJackPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>;
    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl std::ops::DerefMut for JackTestApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

impl Drop for JackTestApp {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.close();
        }
    }
}

/// 📬️ Drives the mounted app's retained typed operation to quiescence (pages ACKed, effects, events
/// and completions drained) — the receipt carries what the host would have seen.
async fn settle(app: &mut JackTestApp) -> artifact_app_laws::TypedOperationFixtureReceipt {
    artifact_app_laws::settle_registered_typed_operation(&mut app.app, JACK_TEST_INSTANCE).await.expect("settle the typed operation")
}

fn jack_envelope_wire() -> Vec<u8> {
    jack_envelope_wire_of(crate::empty_trinity_graph_snapshot())
}

/// 📦️ The exact envelope wire a host hands the guest for `snapshot` (initial snapshot, no edits).
fn jack_envelope_wire_of(snapshot: crate::JackSnapshot) -> Vec<u8> {
    use store::ArtifactPack;

    let snapshot_pack = snapshot.encode_pack();
    let snapshot_hex = snapshot_pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let wire = semio_framework_pack_json::to_string(&semio_framework_pack_json::json!({
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
    let mut retirement = crate::standards::v1::subsets::any::io::binary::mutations::jack_envelope_decode_owner_bundle().retire_envelope(envelope);
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

fn admit_jack_envelope(app: &mut VcsArtifactApp<EditorApp<TrinityJackPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
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

fn drive_jack_live_load(app: &mut VcsArtifactApp<EditorApp<TrinityJackPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
    // 📏️ The runtime host pumps a document load with 32 KiB turns (`DOCUMENT_ARCHIVE_POLL_STEP_BYTES`);
    // a 4 KiB envelope-page grant can never release the decode worker's 16 KiB job-payload pages
    // (`JOB_PAYLOAD_PAGE_BYTES`), so the load spun on its own close step forever.
    for _ in 0..100_000 {
        app.maintenance_step(1, JACK_LIVE_LOAD_STEP_BYTES).expect("one Jack live maintenance turn");
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

/// 🧸️ The live scene composed from the app's `content` member store (design §20.15: the parent holds no content).
async fn live_scene(app: &VcsArtifactApp<EditorApp<TrinityJackPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>) -> crate::JackWorkingScene {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
    use store::{ArtifactPack, SpaceMember};
    let snapshot = app.snapshot().expect("Jack parent projection");
    let bytes = app.child_store("content", &snapshot.content.child_id).await.expect("Jack content child").document_pack_bytes().await.expect("Jack content child pack");
    let (nodes, edges) = crate::working_from_jack_content_snapshot(&SemioGraphSnapshot::decode_pack(&bytes).expect("Jack content child snapshot")).expect("Jack content child scene");
    crate::JackWorkingScene { nodes, edges }
}

async fn node_id_at(app: &VcsArtifactApp<EditorApp<TrinityJackPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>, index: usize) -> String {
    live_scene(app).await.nodes[index].id.clone()
}

/// 🏷️ The live names of the nodes `ids` names, every node when `ids` is `None`, in scene order.
async fn node_names(app: &VcsArtifactApp<EditorApp<TrinityJackPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>, ids: Option<&[String]>) -> Vec<String> {
    live_scene(app).await.nodes.into_iter().filter(|node| ids.is_none_or(|ids| ids.contains(&node.id))).map(|node| node.name).collect()
}

/// 🏷️ The live name of node `id`.
async fn node_name(app: &VcsArtifactApp<EditorApp<TrinityJackPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>, id: &str) -> String {
    live_scene(app).await.nodes.into_iter().find(|node| node.id == id).map(|node| node.name).expect("node")
}

/// 🕹️ Dispatches the framework-injected `interactionSelect` verb against domain "ast" — the
/// replacement for the deleted `TrinityJackCommand::SetSelection`.
async fn select_ast(app: &mut VcsArtifactApp<EditorApp<TrinityJackPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>, ids: &[&str]) {
    let targets: Vec<semio_framework_pack_json::Value> = ids.iter().map(|id| semio_framework_pack_json::json!({ "granularity": "node", "id": id })).collect();
    let args = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({ "domainId": "ast", "targets": semio_framework_pack_json::to_json_string(&targets) }));
    let admitted = app.handle_action("interactionSelect", Some(&args), &meta("local")).await.expect("interactionSelect");
    // 🕹️ On a mounted app the verb is admitted as a framework-reserved job and publishes later —
    // settle both so the selection is live before the next dispatch reads it.
    semio_framework_plugin::app::settle_framework_reserved_admission(app, admitted).await.expect("interactionSelect reserved-job commit");
    artifact_app_laws::settle_registered_typed_operation(app, JACK_TEST_INSTANCE).await.expect("interactionSelect publication");
}

#[semio_framework_async_macros::async_test]
async fn renders_node_graph_scene() {
    let mut app = new_app().await;
    let node = app.render(TRINITY_JACK_PLAY_BODY_GRAPH, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
    assert!(artifact_app_laws::project_and_retire_fixture_tree(node).expect("project semantic UI test tree").contains("node-graph"));
}

#[semio_framework_async_macros::async_test]
async fn renders_jack_editor() {
    let mut app = new_app().await;
    let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
    let json = artifact_app_laws::project_and_retire_fixture_tree(node).expect("project semantic UI test tree");
    assert!(json.contains("text-editor"));
    // 🚚️ The buffer rides an out-of-doc payload lane, never the projected doc spine — read the
    // assembled scene the way a render host does.
    let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::TextEditorScene>(&json).expect("text-editor scene");
    assert_eq!(scene.buffer, crate::TRINITY_JACK_DEFAULT_QUERY);
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
    // 🔬 The query's SET is ONE edit of the composed `content` child: the live member store holds the label.
    assert!(live_scene(&app).await.nodes.iter().any(|node| node.properties.get("label") == Some(&crate::PropertyValue::String("ran-label".into()))));
}

#[semio_framework_async_macros::async_test]
async fn node_graph_select_updates_selection_and_document_tree() {
    let mut app = new_app().await;
    let node_id = node_id_at(&app, 0).await;
    select_ast(&mut app, &[&node_id]).await;
    let tree = app.render(TRINITY_JACK_PLAY_BODY_ARTIFACT, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
    let json = artifact_app_laws::project_and_retire_fixture_tree(tree).expect("project semantic UI test tree");
    assert!(json.contains(&node_id));
    // 🕹️ Rows carry no `selected` flag of their own any more: the tree binds the framework-owned
    // `"ast"` interaction domain and the host paints the selection from the interaction state — so
    // the proof is the binding plus the selection being the one `deleteSelection` reads
    // (`delete_selection_removes_selected_node`).
    assert!(json.contains("\"interactionDomain\":\"ast\""), "document tree must bind the ast interaction domain: {json}");
}

#[semio_framework_async_macros::async_test]
async fn nakagin_fixture_has_nodes() {
    assert!(!crate::jack_working_scene(&default_snapshot()).expect("curated example scene").nodes.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn editor_scene_has_tokens_and_diagnostics() {
    let mut app = new_app().await;
    let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
    let json = artifact_app_laws::project_and_retire_fixture_tree(node).expect("project semantic UI test tree");
    let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::TextEditorScene>(&json).expect("text-editor scene");
    assert!(scene.tokens_json.is_some_and(|tokens| !tokens.is_empty()));
    assert!(scene.diagnostics_json.is_some());
    assert!(scene.completions_json.is_some());
}

#[semio_framework_async_macros::async_test]
async fn text_edit_puts_the_query_into_the_document() {
    let mut app = new_app().await;
    let view = query_windows();
    let editor = view.for_window_instance("editor-main").unwrap();
    app.dispatch_typed(TrinityJackCommand::TextEdit { text: "MATCH (a:Piece) RETURN a.name".into() }, &semio_framework_plugin::ActionMeta { view_state: Some(editor.clone()), ..meta("local") })
        .await
        .expect("edit");
    drive_query_ownership_operations(&mut app).await.expect("edit completes");
    let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &editor).await.expect("render");
    let json = artifact_app_laws::project_and_retire_fixture_tree(node).expect("project semantic UI test tree");
    let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::TextEditorScene>(&json).expect("text-editor scene");
    assert_eq!(scene.buffer, "MATCH (a:Piece) RETURN a.name");
    assert_eq!(app.snapshot().expect("projection").query, "MATCH (a:Piece) RETURN a.name", "the typed query is document content, not a window's config");
}

/// 🔎️ The query the jack editor window shows (its text-editor scene buffer).
async fn jack_query(app: &mut JackTestApp, editor: &ViewModel) -> String {
    let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, editor).await.expect("render");
    let json = artifact_app_laws::project_and_retire_fixture_tree(node).expect("project semantic UI test tree");
    artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::TextEditorScene>(&json).expect("text-editor scene").buffer
}

/// ⌨️ A typing run far longer than the store's fixed applied-edit ledger (64) — 1137 typed characters with pauses, caret moves
/// and corrections, one full-text `textEdit` typing delivery per changed key, never idle past the run's bound — is ONE run
/// (design §13.2 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): the query window shows every key while nothing lands, the
/// idle commit lands it as ONE edit stamped with its `TransactionRef`, and ONE undo reverts the whole run, ONE redo restores it
/// (ticket 26/09/23 F1: the 65th character was refused with `batched publication requires preinstalled fixed applied and
/// revision capacity`).
#[semio_framework_async_macros::async_test]
async fn a_typing_run_longer_than_the_edit_ledger_keeps_saving_and_undoes_as_one_step() {
    let run = artifact_app_laws::typing_run();
    let mut app = new_app().await;
    let view = query_windows();
    let editor = view.for_window_instance("editor-main").unwrap();
    let meta_of = || semio_framework_plugin::ActionMeta { view_state: Some(editor.clone()), ..meta("local") };
    let before = jack_query(&mut app, &editor).await;
    let edits = app.edit_transactions().len();
    let mut now = 50_000;
    for (index, text) in run.texts.iter().enumerate() {
        now += 40;
        app.set_tool_clock_ms(Some(now));
        let args = semio_framework_value::DslValue::Object(vec![("text".into(), semio_framework_value::DslValue::String(text.clone())), (semio_framework_plugin::TYPING_BUFFER_ARG.into(), semio_framework_value::DslValue::String("trinity.jack.query".into()))]);
        app.handle_action("textEdit", Some(&args), &meta_of()).await.unwrap_or_else(|error| panic!("keystroke {index} was refused: {error:?}"));
        drive_query_ownership_operations(&mut app).await.unwrap_or_else(|error| panic!("keystroke {index} did not publish: {error}"));
        artifact_app_laws::drain_maintenance_pressure(&mut app.app);
    }
    assert_eq!(jack_query(&mut app, &editor).await, run.expected, "the query window shows the open run");
    assert_eq!((app.snapshot().expect("projection").query.clone(), app.edit_transactions().len()), (before.clone(), edits), "nothing lands while the run is open");
    app.set_tool_clock_ms(Some(now + 750));
    let commit = semio_framework_value::DslValue::Object(vec![(semio_framework_plugin::TYPING_BUFFER_ARG.into(), semio_framework_value::DslValue::String("trinity.jack.query".into())), (semio_framework_plugin::TYPING_COMMIT_ARG.into(), semio_framework_value::DslValue::String("idle".into()))]);
    app.handle_action("textEdit", Some(&commit), &meta_of()).await.expect("the idle commit");
    drive_query_ownership_operations(&mut app).await.expect("the run publishes");
    assert_eq!(app.snapshot().expect("projection").query, run.expected);
    let transactions = app.edit_transactions();
    assert_eq!(transactions.len(), edits + 1, "the run lands as ONE edit");
    assert!(transactions.last().cloned().flatten().is_some_and(|transaction| transaction.tool.ends_with("#textEdit")), "the edit carries the run's TransactionRef: {transactions:?}");
    for (verb, expected) in [("undo", &before), ("redo", &run.expected)] {
        let admitted = app.handle_action(verb, None, &meta_of()).await.unwrap_or_else(|fault| panic!("{verb} admission: {fault:?}"));
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.app, admitted).await.unwrap_or_else(|fault| panic!("{verb} settles: {fault:?}"));
        drive_query_ownership_operations(&mut app).await.unwrap_or_else(|error| panic!("{verb} publishes: {error}"));
        assert_eq!(&jack_query(&mut app, &editor).await, expected, "one {verb} moves the whole run");
    }
}

#[semio_framework_async_macros::async_test]
async fn graph_scene_has_lod_json() {
    let mut app = new_app().await;
    let node = app.render(TRINITY_JACK_PLAY_BODY_GRAPH, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
    let json = artifact_app_laws::project_and_retire_fixture_tree(node).expect("project semantic UI test tree");
    let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(&json).expect("node-graph scene");
    assert!(scene.lod_json.as_deref().is_some_and(|lod| lod.contains("automatic")), "lodJson lane: {:?}", scene.lod_json);
}

#[semio_framework_async_macros::async_test]
async fn set_lod_mode_reflects_in_window_measures() {
    let mut app = new_app().await;
    let view = ViewModel { window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: "jack-graph-main".into(), window_kind_id: TRINITY_JACK_PLAY_WINDOW_GRAPH.into() }], ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let addressed = view.for_window_instance("jack-graph-main").unwrap();
    app.dispatch_typed(TrinityJackCommand::SetLodMode { value: "compact".into() }, &semio_framework_plugin::ActionMeta { view_state: Some(addressed), ..meta("local") }).await.expect("lod");
    // 📬️ `settle` drains the completion outbox too — a page-only loop spins forever on it.
    let receipt = settle(&mut app).await;
    assert!(!receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Fault));
    let measures = app.window_measures(&view).await;
    assert!(measures["jack-graph-main"].iter().any(|measure| matches!(measure, WindowMeasure::Select { value, .. } if value == "compact")));
}

#[semio_framework_async_macros::async_test]
async fn catalogue_tree_renders() {
    let mut app = new_app().await;
    let node = app.render(TRINITY_JACK_PLAY_BODY_CATALOGUE, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
    assert!(artifact_app_laws::project_and_retire_fixture_tree(node).expect("project semantic UI test tree").contains("trinity-jack-catalogue"));
}

#[semio_framework_async_macros::async_test]
async fn inspection_panel_renders_the_selection_prompt() {
    let mut app = new_app().await;
    let node_id = node_id_at(&app, 0).await;
    select_ast(&mut app, &[&node_id]).await;
    let node = app.render(TRINITY_JACK_PLAY_BODY_INSPECTION, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
    // 🕹️ `render` has no `InteractionView` (see the panel's own doc comment) — it can no longer
    // build per-selection fields, so it always renders the static prompt.
    assert!(artifact_app_laws::project_and_retire_fixture_tree(node).expect("project semantic UI test tree").contains("trinity-inspector.empty"));
}

#[semio_framework_async_macros::async_test]
async fn document_tree_de_locale_translates_labels() {
    let mut app = new_app().await;
    let view = ViewModel { locale: semio_framework_ui_locale::Locale::De, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let node = app.render(TRINITY_JACK_PLAY_BODY_ARTIFACT, None, &view).await.expect("render");
    assert!(artifact_app_laws::project_and_retire_fixture_tree(node).expect("project semantic UI test tree").contains("Stücke"));
}

#[semio_framework_async_macros::async_test]
async fn set_active_example_loads_the_example_with_its_preset_query() {
    let mut app = new_app().await;
    let result = app.dispatch_typed(TrinityJackCommand::SetActiveExample { example_id: "branch-chain".into() }, &meta("local")).await.expect("set active example");
    let receipt = settle(&mut app).await;
    // 🩹 Pre-existing test/implementation mismatch (traced to commit `a445617c`, 2026-08-12
    // 15:50:51 +0200 — predates this migration, not introduced by it): `set_active_example`
    // routes the fixture swap through `Effect::LoadDocument` (whole-document replace is
    // banned from the `Mutation` enum outright), never through `artifact_mutations`, so
    // `InvocationResult.mutations` is always empty for this command — `requested_effects` is
    // the field that actually carries the swap.
    let loaded = result.requested_effects.iter().chain(receipt.effects.iter()).find_map(|effect| match effect {
        semio_framework_plugin::Effect::LoadDocument { pack, .. } => Some(<crate::JackSnapshot as store::ArtifactPack>::decode_pack(pack).expect("the loaded example decodes")),
        _ => None,
    });
    let loaded = loaded.expect("setActiveExample must request the LoadDocument effect");
    assert_eq!(loaded.query, crate::editor::jack::commands::preset_query("branch-chain"), "the example document carries its own preset query");
    assert!(!crate::jack_working_scene(&loaded).expect("the example's content child").nodes.is_empty(), "the example document carries its graph");
}

#[semio_framework_async_macros::async_test]
async fn delete_selection_removes_selected_node() {
    let mut app = new_app().await;
    let node_id = node_id_at(&app, 0).await;
    select_ast(&mut app, &[&node_id]).await;
    let result = app.dispatch_typed(TrinityJackCommand::DeleteSelection, &meta("local")).await.expect("delete");
    let receipt = settle(&mut app).await;
    assert!(result.mutations.is_empty(), "deleteSelection writes no parent leaf: {:?}", result.mutations);
    assert!(receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Child), "deleteSelection must publish one content child edit: lanes {:?}", receipt.lanes);
    assert!(!live_scene(&app).await.nodes.iter().any(|node| node.id == node_id));
}

#[semio_framework_async_macros::async_test]
async fn context_menu_stays_within_row_budget_and_ends_with_delete_selection() {
    let mut app = new_app().await;
    let node_id = node_id_at(&app, 0).await;
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
    let menu = app.context_menu(&request, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
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
    let document_out = app.export_media("artifact:out").await.expect("document:out export");
    let graph_out = app.export_media("graph:out").await.expect("graph:out export");
    assert_eq!(document_out.payload, graph_out.payload);
}

#[semio_framework_async_macros::async_test]
async fn jack_io_declares_graph_out_fan_out_port() {
    let io = jack_io();
    assert_eq!(io.artifact_schema, TRINITY_GRAPH_SCHEMA);
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
        let expected_name = live_scene(&app).await.nodes.into_iter().find(|node| node.kind == "Piece").ok_or_else(|| "query runtime fixture has no Piece".to_string())?.name;
        let view = query_windows();
        let editor = view.for_window_instance("editor-main").ok_or("missing editor window")?;
        let results = view.for_window_instance("results-main").ok_or("missing results window")?;
        app.dispatch_typed(
            TrinityJackCommand::RunQuery { query: Some("MATCH (a:Piece) RETURN a.name".into()), results_window_id: "results-main".into() },
            &semio_framework_plugin::ActionMeta { view_state: Some(editor), ..meta("query-owner") },
        )
        .await
        .map_err(|error| format!("{error:?}"))?;
        if drive_query_ownership_operations(&mut app).await? != (0, 0, 1) {
            return Err("query operation did not produce exactly one results-transient receipt".into());
        }
        if app.snapshot().map_err(|error| format!("{error:?}"))? != document {
            return Err("read query modified the document".into());
        }
        let generation = app.window_transient_generation(&results).map_err(|error| format!("{error:?}"))?.ok_or("missing results transient generation")?;
        if generation != 1 || app.ephemeral_snapshot().await.transient_generation != before {
            return Err("query result did not publish exactly once to only the results-window transient".into());
        }
        let tree = app.render(TRINITY_JACK_PLAY_BODY_RESULTS, None, &results).await.map_err(|error| format!("{error:?}"))?;
        let rendered = artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)?;
        if !rendered.contains(&expected_name) {
            return Err("query result table did not contain the document's matching Piece".into());
        }
        Ok((generation, rendered))
    }
    .await;
    app.close();
    let (generation, rendered) = outcome.expect("owned query runtime");
    assert!(rendered.contains("table"));
}

async fn drive_query_ownership_operations(app: &mut VcsArtifactApp<EditorApp<TrinityJackPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>) -> Result<(u64, u64, u64), String> {
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
        // 🧹️ The terminal witness lands in its own completion outbox, which `has_pending_typed_operations`
        // counts — left undrained this loop spins to its deadline (`query operations did not finish`).
        while app.take_typed_operation_completion().await.map_err(|error| format!("{error:?}"))?.is_some() {}
        while let Some(reply) = app.take_local_interaction_query_reply() {
            if let protocol::LocalInteractionQueryReply::Page { page } = reply {
                let token = protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal };
                if !app.acknowledge_local_interaction_query(&token) {
                    return Err("local interaction query rejected its exact ACK".into());
                }
            }
        }
        std::thread::yield_now();
    }
    Ok((transient_receipts, window_config_receipts, window_transient_receipts))
}

/// 🔎️ The Jack query is DOCUMENT content (ticket 26/09/23 C12, decision (a)): every editor window shows the one query the
/// document holds, an edit from either editor is one undoable document edit and persists with the document's pack, and running
/// a query — from either editor, into either results window — reads the document without editing it. Only the results stay
/// per window, and they are never persisted: a reopened app starts with fresh results windows.
#[semio_framework_async_macros::async_test]
async fn jack_query_is_document_content_shared_by_every_editor_while_results_stay_per_window() {
    let mut app = new_app().await;
    let view = ViewModel {
        window_instances: vec![
            semio_framework_plugin::ViewWindowInstance { id: "editor-left".into(), window_kind_id: TRINITY_JACK_PLAY_WINDOW_EDITOR.into() },
            semio_framework_plugin::ViewWindowInstance { id: "editor-right".into(), window_kind_id: TRINITY_JACK_PLAY_WINDOW_EDITOR.into() },
            semio_framework_plugin::ViewWindowInstance { id: "results-left".into(), window_kind_id: TRINITY_JACK_PLAY_WINDOW_RESULTS.into() },
            semio_framework_plugin::ViewWindowInstance { id: "results-right".into(), window_kind_id: TRINITY_JACK_PLAY_WINDOW_RESULTS.into() },
        ],
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    let editor_left = view.for_window_instance("editor-left").unwrap();
    let editor_right = view.for_window_instance("editor-right").unwrap();
    let results_left = view.for_window_instance("results-left").unwrap();
    let results_right = view.for_window_instance("results-right").unwrap();
    let left_query = "MATCH (a:Piece) WHERE a.name = 'b' RETURN a.name";
    let right_query = "MATCH (a:Piece) WHERE a.name != 'b' RETURN a.name";
    let app_config_before = app.config_pack().await.expect("app config pack");
    let denied = app
        .dispatch_typed(
            TrinityJackCommand::RunQuery { query: Some(left_query.into()), results_window_id: "editor-right".into() },
            &semio_framework_plugin::ActionMeta { view_state: Some(editor_left.clone()), ..meta("query-owner") },
        )
        .await;
    assert!(denied.is_err(), "an attached editor cannot be promoted to results mutation authority by command payload");

    app.dispatch_typed(TrinityJackCommand::TextEdit { text: left_query.into() }, &semio_framework_plugin::ActionMeta { view_state: Some(editor_left.clone()), ..meta("query-owner") })
        .await
        .expect("query edit");
    assert_eq!(drive_query_ownership_operations(&mut app).await.expect("query edit"), (0, 0, 0));
    let document = app.snapshot().expect("document after the query edit");
    assert_eq!(document.query, left_query, "the typed query is the document's");
    for context in [&editor_left, &editor_right] {
        let tree = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, context).await.expect("editor render");
        let rendered = artifact_app_laws::project_and_retire_fixture_tree(tree).expect("editor projection");
        let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::TextEditorScene>(&rendered).expect("editor scene");
        assert_eq!(scene.buffer, left_query, "every editor window shows the document's one query");
    }

    for (context, target, query) in [(&editor_left, "results-left", None), (&editor_right, "results-right", Some(right_query.to_string()))] {
        app.dispatch_typed(TrinityJackCommand::RunQuery { query, results_window_id: target.into() }, &semio_framework_plugin::ActionMeta { view_state: Some(context.clone()), ..meta("query-owner") })
            .await
            .expect("paired query");
    }
    assert_eq!(drive_query_ownership_operations(&mut app).await.expect("paired queries"), (0, 0, 2));
    assert_eq!(app.snapshot().expect("document after queries"), document, "running a read query never edits the document");
    let app_config_after = app.config_pack().await.expect("app config after");
    assert_eq!(app_config_after.pack, app_config_before.pack);
    assert_eq!(app_config_after.spr, app_config_before.spr);
    assert!(app.window_config_packs().await.expect("persisted window configs").is_empty(), "no editor window persists a query of its own");
    assert_eq!(app.ephemeral_snapshot().await.transient_generation, 0);
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
    for context in [&results_left, &results_right] {
        let tree = app.render(TRINITY_JACK_PLAY_BODY_RESULTS, None, context).await.expect("result render");
        assert!(artifact_app_laws::project_and_retire_fixture_tree(tree).expect("result projection").contains("table"));
    }

    let reloaded = <crate::JackSnapshot as store::ArtifactPack>::decode_pack(&store::ArtifactPack::encode_pack(&document)).expect("the document pack decodes");
    assert_eq!(reloaded.query, left_query, "the query persists with the document");
    let admitted = app.handle_action("undo", None, &semio_framework_plugin::ActionMeta { view_state: Some(editor_right.clone()), ..meta("query-owner") }).await.unwrap_or_else(|fault| panic!("undo admission: {fault:?}"));
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.app, admitted).await.unwrap_or_else(|fault| panic!("undo settles: {fault:?}"));
    drive_query_ownership_operations(&mut app).await.expect("undo publishes");
    assert_eq!(app.snapshot().expect("document after undo").query, crate::TRINITY_JACK_DEFAULT_QUERY, "one undo from either editor reverts the query edit");
    app.close();

    let mut reopened = new_app().await;
    for context in [&results_left, &results_right] {
        assert_eq!(reopened.window_transient_generation(context).expect("fresh result generation"), Some(0));
        let snapshot = reopened.window_transient_snapshot(context).expect("fresh result snapshot").expect("fresh results owner");
        let state = snapshot.get::<JackResultsWindowTransientOwner>().expect("fresh results state");
        assert!(state.query_execution_id.is_none() && state.result.is_none() && state.query_error.is_none());
    }
    reopened.close();
}

/// 🎫️ Slice B3b (ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END). Every retained WINDOW-CONFIG
/// verb reads the config of the window it was dispatched from, so leaving one unowned is a live
/// fault: `build_definition` copies an unowned action onto EVERY window kind, and the graph pane's
/// Actions list then offered `formatDocument`, which the reducer refused with "Jack query formatting
/// requires the exact editor-window config snapshot". Measured on the React shell at 6054 before the
/// fix (`🗑️generated/b3b-trinity-jack-console.txt`).
#[semio_framework_async_macros::async_test]
async fn window_kind_actions_scope_text_verbs_to_the_query_editor() {
    let definition = create_trinity_jack_app();
    let resolve = |window_id: &str| -> Vec<String> {
        let window = definition.window_kinds.iter().find(|window| window.id == window_id).expect("window kind");
        semio_framework_plugin::resolve_window_actions(&definition, window).into_iter().map(|action| action.id.clone()).collect()
    };
    let graph = resolve(TRINITY_JACK_PLAY_WINDOW_GRAPH);
    let editor = resolve(TRINITY_JACK_PLAY_WINDOW_EDITOR);
    let results = resolve(TRINITY_JACK_PLAY_WINDOW_RESULTS);
    for text_verb in ["textEdit", "textSelect", "formatDocument"] {
        assert!(editor.contains(&text_verb.to_string()), "the query editor must expose {text_verb}");
        assert!(!graph.contains(&text_verb.to_string()), "the graph pane must NOT expose {text_verb}");
        assert!(!results.contains(&text_verb.to_string()), "the results pane must NOT expose {text_verb}");
    }
    for graph_verb in ["nodeGraphViewport", "setLodMode"] {
        assert!(graph.contains(&graph_verb.to_string()), "the graph pane must expose {graph_verb}");
        assert!(!editor.contains(&graph_verb.to_string()), "the query editor must NOT expose {graph_verb}");
    }
    for shared in ["setActiveExample", "runQuery", "deleteSelection", "patchNodes"] {
        assert!(graph.contains(&shared.to_string()) && editor.contains(&shared.to_string()), "{shared} reads the document, so it stays on every pane");
    }
}

/// 🎫️ Slice B3b. The navbar example picker dispatches a REGISTERED example id, and `demo` is the only
/// example this subset registers — `example_dsl_for_preset` knew `nakagin`/`branch-chain` only, so
/// every navbar pick resolved to `None` and loaded nothing with no fault anywhere. The staged argument
/// form must offer the same registered id, never an id the resolver would drop.
#[semio_framework_async_macros::async_test]
async fn set_active_example_resolves_every_id_the_shell_can_send() {
    let mut app = new_app().await;
    let result = app.dispatch_typed(TrinityJackCommand::SetActiveExample { example_id: crate::examples::demo::ID.into() }, &meta("local")).await.expect("set active example");
    let receipt = settle(&mut app).await;
    assert!(!result.requested_effects.is_empty() || !receipt.effects.is_empty(), "the registered example id must request the LoadDocument effect");
    app.close();

    // 📇️ An action is resolved either on the app roster (`AppDefinition.actions`, where an unscoped
    // verb such as `setActiveExample` stays) or on the window kind that owns it
    // (`WindowKindDefinition.actions`) — the shell's `undeclaredActionDiagnostic` accepts both — and
    // `ArgSchema::Select` folded into `ArgSchema::String { options, .. }` (`ActionArgDef::select`).
    // Wherever it is resolved, the staged form it offers must still be the subset's registered
    // example set.
    let definition = create_trinity_jack_app();
    let offered: Vec<String> = definition
        .actions
        .iter()
        .chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter()))
        .find(|action| action.id == "setActiveExample")
        .expect("setActiveExample declared")
        .args
        .iter()
        .flat_map(|arg| match &arg.schema {
            semio_framework_plugin::ArgSchema::String { options, .. } => options.iter().map(|option| option.value.clone()).collect::<Vec<_>>(),
            _ => Vec::new(),
        })
        .collect();
    assert!(offered.contains(&crate::examples::demo::ID.to_string()), "the staged form must offer the registered example, offered {offered:?}");
}

/// ➖️ The Jack Query editor lints its buffer through the framework graph DSL
/// (`core::lint` → `semio_framework_graph::dsl::lint`), and the pane opens on
/// `TRINITY_JACK_DEFAULT_QUERY` over the curated Nakagin example. That query writes the Cypher connector
/// `(a:Piece)-[r:Connection]->(b:Piece)`, which the framework lexer used to refuse, so the live
/// trinity-jack pane reported `unexpected character '-'` on a query its own executor runs (ticket
/// 26/09/19, `📓️knowledge.md` §6, §13.2). Every query the editor ships must lint clean on the example it
/// ships with, and must run there.
#[semio_framework_async_macros::async_test]
async fn every_shipped_query_lints_clean_and_runs_on_the_curated_example() {
    let example = <crate::JackSnapshot as store::ArtifactDsl>::parse_dsl(crate::editor::jack::NAKAGIN_EXAMPLE_DSL).expect("curated example parses");
    for query in [crate::TRINITY_JACK_DEFAULT_QUERY, "MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a, r, b"] {
        let graph = crate::Graph::from_snapshot(example.clone()).expect("curated example graph");
        let diagnostics = crate::core::lint(&graph, query);
        assert!(diagnostics.is_empty(), "{query} must lint clean on the curated example, got {diagnostics:?}");
        let mut graph = graph;
        crate::executor::run(&mut graph, query).unwrap_or_else(|error| panic!("{query} must run on the curated example: {error}"));
    }
}

//#region 🩹️RailVerbLaws
/// 🕹️ Dispatches `action` with rail-staged text `args` (the Actions pane's own shape) and settles it.
async fn dispatch_rail(app: &mut JackTestApp, action: &str, args: &[(&str, &str)]) -> Result<(), semio_framework_plugin::Fault> {
    let args = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::Value::Object(args.iter().map(|(key, value)| ((*key).to_string(), semio_framework_pack_json::Value::String((*value).to_string()))).collect()));
    app.handle_action(action, Some(&args), &meta("local")).await?;
    artifact_app_laws::settle_registered_typed_operation(&mut app.app, JACK_TEST_INSTANCE).await.map(|_| ())
}

/// ⚖️ LAW: `patchNodes` pressed from the rail with an EMPTY `nodeIds` renames the selected nodes, and a
/// comma list in the text field names several nodes (S15: the verb "moved nothing").
#[semio_framework_async_macros::async_test]
async fn patch_nodes_from_the_rail_renames_the_selection_or_the_listed_nodes() {
    let mut app = new_app().await;
    let (first, second) = (node_id_at(&app, 0).await, node_id_at(&app, 1).await);
    select_ast(&mut app, &[&first]).await;
    dispatch_rail(&mut app, "patchNodes", &[("field", "name"), ("value", "S15 Selected")]).await.expect("an empty nodeIds patches the selection");
    assert_eq!(node_name(&app, &first).await, "S15 Selected");
    dispatch_rail(&mut app, "patchNodes", &[("nodeIds", &format!("{first}, {second}")), ("field", "name"), ("value", "S15 Listed")]).await.expect("a comma list names both nodes");
    assert_eq!((node_name(&app, &first).await, node_name(&app, &second).await), ("S15 Listed".to_string(), "S15 Listed".to_string()));
}

/// ⏪️ LAW: a rail `patchNodes` is one undoable edit of the local user, whether it renames one node or the whole
/// selection — undo restores every renamed node and redo renames them again (S15, session 12: after `selectAll` the
/// row had no ↶ and undo left the document renamed).
#[semio_framework_async_macros::async_test]
async fn a_rail_patch_nodes_is_undone_and_redone_as_one_local_edit() {
    for selected in [1usize, 2] {
        let mut app = new_app().await;
        let mut ids = Vec::new();
        for index in 0..selected {
            ids.push(node_id_at(&app, index).await);
        }
        let before = node_names(&app, Some(&ids)).await;
        select_ast(&mut app, &ids.iter().map(String::as_str).collect::<Vec<_>>()).await;
        dispatch_rail(&mut app, "patchNodes", &[("field", "name"), ("value", "S15 Undo")]).await.expect("patch the selection");
        assert_eq!(node_names(&app, Some(&ids)).await, vec!["S15 Undo".to_string(); selected], "{selected} selected");
        artifact_app_laws::settle_history_verb(&mut app.app, "undo", JACK_TEST_INSTANCE).await;
        assert_eq!(node_names(&app, Some(&ids)).await, before, "undo restores all {selected} renamed nodes");
        artifact_app_laws::settle_history_verb(&mut app.app, "redo", JACK_TEST_INSTANCE).await;
        assert_eq!(node_names(&app, Some(&ids)).await, vec!["S15 Undo".to_string(); selected], "redo renames all {selected} again");
    }
}

/// 🕹️ Presses one argument-less framework verb (`selectAll`, `clearSelection`, `undo`, `redo`) the whole way a shell
/// does: admission, its reserved job, then the typed publication.
async fn framework_verb(app: &mut JackTestApp, action: &str) {
    let admitted = app.app.handle_action(action, None, &meta("local")).await.unwrap_or_else(|fault| panic!("{action}: {fault:?}"));
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.app, admitted).await.unwrap_or_else(|fault| panic!("{action} reserved job: {fault:?}"));
    artifact_app_laws::settle_registered_typed_operation(&mut app.app, JACK_TEST_INSTANCE).await.unwrap_or_else(|fault| panic!("{action} publication: {fault:?}"));
}

/// ⏪️ LAW: the shell's own round trip — `selectAll`, `patchNodes` with an empty `nodeIds`, a neutral
/// `clearSelection`, then undo and redo — renames every node, restores every name, and renames them again.
#[semio_framework_async_macros::async_test]
async fn the_shells_select_all_patch_undo_redo_round_trip_restores_every_node() {
    let mut app = new_app().await;
    let before = node_names(&app, None).await;
    framework_verb(&mut app, "selectAll").await;
    dispatch_rail(&mut app, "patchNodes", &[("field", "name"), ("value", "S15 All")]).await.expect("patch the whole selection");
    let patched = node_names(&app, None).await;
    assert!(patched.iter().all(|name| name == "S15 All"), "every selected node renamed: {patched:?}");
    framework_verb(&mut app, "clearSelection").await;
    framework_verb(&mut app, "undo").await;
    assert_eq!(node_names(&app, None).await, before, "undo restores every node");
    framework_verb(&mut app, "redo").await;
    assert_eq!(node_names(&app, None).await, patched, "redo renames them again");
}

/// ⏪️ LAW: after the host LOADS the curated example (envelope ingress, swapped store), the shell's `selectAll` →
/// `patchNodes` → `clearSelection` → undo → redo round trip still treats the rename as the local user's edit.
#[semio_framework_async_macros::async_test]
async fn a_loaded_example_keeps_the_local_users_patch_undoable() {
    let mut app = new_app().await;
    let example = <crate::JackSnapshot as store::ArtifactDsl>::parse_dsl(crate::editor::jack::NAKAGIN_EXAMPLE_DSL).expect("curated example parses");
    let handle = admit_jack_envelope(&mut app, &jack_envelope_wire_of(example));
    assert_eq!(drive_jack_live_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("exact load acknowledgement"));
    let before = node_names(&app, None).await;
    assert!(!before.is_empty(), "the curated example has nodes");
    framework_verb(&mut app, "selectAll").await;
    dispatch_rail(&mut app, "patchNodes", &[("field", "name"), ("value", "S15 Loaded")]).await.expect("patch the whole selection");
    let patched = node_names(&app, None).await;
    assert!(patched.iter().all(|name| name == "S15 Loaded"), "every selected node renamed: {patched:?}");
    framework_verb(&mut app, "clearSelection").await;
    framework_verb(&mut app, "undo").await;
    assert_eq!(node_names(&app, None).await, before, "undo restores every node of the loaded example");
    framework_verb(&mut app, "redo").await;
    assert_eq!(node_names(&app, None).await, patched, "redo renames them again");
}

/// 🧹️ LAW: `clearSelection` on a graph with nothing selected settles like any other turn and leaves the program's
/// dispatch lane free — the next verb and undo run (S15, session 12: the empty clear never settled and held every
/// later undo, Cmd+Z and patchNodes).
#[semio_framework_async_macros::async_test]
async fn clear_selection_on_an_empty_selection_settles_and_frees_the_lane() {
    let mut app = new_app().await;
    framework_verb(&mut app, "clearSelection").await;
    let first = node_id_at(&app, 0).await;
    dispatch_rail(&mut app, "patchNodes", &[("nodeIds", &first), ("field", "name"), ("value", "S15 After Clear")]).await.expect("the lane is free after an empty clear");
    assert_eq!(node_name(&app, &first).await, "S15 After Clear");
    framework_verb(&mut app, "clearSelection").await;
    framework_verb(&mut app, "undo").await;
    assert_ne!(node_name(&app, &first).await, "S15 After Clear", "undo after a clear still reaches the edit");
}

/// ⚖️ LAW: a `patchNodes` that cannot move the document is refused by name — an unknown id is
/// `mutation.target-missing`, no id and no selection is `app.command.targets-required` (the precondition an agent,
/// which has no selection, meets by naming `nodeIds`), an unsupported field or an empty value is
/// `app.command.invalid-args` — and the document stays untouched. It used to answer an empty emit that read as an
/// accepted edit.
#[semio_framework_async_macros::async_test]
async fn patch_nodes_refuses_what_it_cannot_apply_and_leaves_the_document_untouched() {
    let mut app = new_app().await;
    let first = node_id_at(&app, 0).await;
    let before = node_names(&app, None).await;
    let refusal = |result: Result<(), Fault>| result.err().expect("refused").code.0;
    assert_eq!(refusal(dispatch_rail(&mut app, "patchNodes", &[("nodeIds", "no-such-node"), ("field", "name"), ("value", "x")]).await), "mutation.target-missing");
    assert_eq!(refusal(dispatch_rail(&mut app, "patchNodes", &[("field", "name"), ("value", "x")]).await), "app.command.targets-required");
    assert_eq!(refusal(dispatch_rail(&mut app, "patchNodes", &[("nodeIds", first.as_str()), ("field", "kind"), ("value", "x")]).await), "app.command.invalid-args");
    assert_eq!(refusal(dispatch_rail(&mut app, "patchNodes", &[("nodeIds", first.as_str()), ("field", "name"), ("value", "  ")]).await), "app.command.invalid-args");
    assert_eq!(node_names(&app, None).await, before, "every refusal leaves the content child untouched");
}

/// ⚖️ LAW: the query editor's gesture verbs stay out of the palette and the Actions rail — a press there
/// carries no buffer or caret, so `textSelect` was refused `missing start` (S15) and `textEdit` would
/// have emptied the query — while the editor window still declares both for its text host.
#[semio_framework_async_macros::async_test]
async fn the_text_gesture_verbs_are_kept_off_the_rail() {
    let definition = create_trinity_jack_app();
    let editor = definition.window_kinds.iter().find(|window| window.id == TRINITY_JACK_PLAY_WINDOW_EDITOR).expect("the query editor window");
    let actions = semio_framework_plugin::resolve_window_actions(&definition, editor);
    for gesture in ["textEdit", "textSelect"] {
        let action = actions.iter().find(|action| action.id == gesture).expect("the query editor still declares its text-host gesture");
        assert!(!action.in_palette, "{gesture} is a text-host gesture, never a rail row");
    }
    let patch = actions.iter().find(|action| action.id == "patchNodes").expect("declared");
    assert!(patch.args.iter().any(|arg| arg.id == "nodeIds" && !arg.required), "nodeIds is optional: empty means the selection");
}
//#endregion 🩹️RailVerbLaws

//#region 🤖️AgentLaneLaws
/// 🤖️ LAW: an agent — no window, no selection — names the nodes `patchNodes` renames in its declared `nodeIds`
/// argument, published as `entityId`s of the `ast/node` granularity. Through the agent lane (the MCP gateway's
/// `action_prepare`, `artifact_app_laws::probe_agent_lane`) the named node is renamed exactly as the shell renames it;
/// without `nodeIds` the agent is refused by name with `app.command.targets-required`, as the shell is with nothing
/// selected. The rail keeps its text field: empty still means the selection.
#[semio_framework_async_macros::async_test]
async fn an_agent_names_the_nodes_patch_nodes_renames_and_is_refused_by_name_without_them() {
    use semio_framework_plugin::artifact_app_laws::{declared_verb_agent_divergences, declared_verb_probe, declared_verb_staged_wrote_document, declared_verb_wrote_document, probe_agent_lane, DeclaredVerbOutcome};
    let definition = create_trinity_jack_app();
    let patch = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).find(|action| action.id == "patchNodes").expect("declared");
    let node_ids = patch.args.iter().find(|arg| arg.id == "nodeIds").expect("nodeIds");
    assert!(!node_ids.required, "the rail may leave nodeIds empty to act on the selection");
    let schema = node_ids.json_schema(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En);
    let items = schema.get("items").expect("nodeIds is a list");
    assert_eq!(
        (schema.get("type").and_then(semio_framework_plugin::DslValue::as_str), items.get("x-semio-format").and_then(semio_framework_plugin::DslValue::as_str), items.get("x-semio-entity-kind").and_then(semio_framework_plugin::DslValue::as_str)),
        (Some("array"), Some("entityId"), Some("ast/node"))
    );
    let first = crate::jack_working_scene(&default_snapshot()).expect("curated example scene").nodes[0].id.clone();
    let named = probe_agent_lane::<EditorApp<TrinityJackPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(
        create_trinity_jack_app,
        Some(&format!(r#"{{"verbs":{{"patchNodes":{{"nodeIds":["{first}"],"field":"name","value":"Agent Named"}}}}}}"#)),
        &["patchNodes"],
    )
    .await;
    let probe = declared_verb_probe(&named, "patchNodes");
    assert!(declared_verb_wrote_document(probe.agent.as_ref().expect("patchNodes is agent-facing")), "the agent's preview renames the named node: {:?}", probe.agent);
    assert!(declared_verb_staged_wrote_document(probe), "the shell renames the named node too");
    let unnamed = probe_agent_lane::<EditorApp<TrinityJackPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(create_trinity_jack_app, Some(r#"{"verbs":{"patchNodes":{"field":"name","value":"Agent Unnamed"}}}"#), &["patchNodes"]).await;
    match declared_verb_probe(&unnamed, "patchNodes").agent.as_ref() {
        Some(DeclaredVerbOutcome::Refused { code, .. }) => assert_eq!(code, "app.command.targets-required"),
        other => panic!("an agent naming no node is refused by name: {other:?}"),
    }
    assert_eq!((declared_verb_agent_divergences(&named), declared_verb_agent_divergences(&unnamed)), (Vec::<String>::new(), Vec::<String>::new()), "agent-lane divergences");
}
//#endregion 🤖️AgentLaneLaws

semio_framework_plugin::history_edit_acceptance_law!("trinity", super::TrinityJackPlayApp, || semio_framework_plugin::App { definition: super::create_trinity_jack_app(), examples: Vec::new() }, "../../🏅️standards/🔖️1/🪆️subsets/✳️any");
semio_framework_plugin::composed_reload_law!("trinity", super::TrinityJackPlayApp, || semio_framework_plugin::App { definition: super::create_trinity_jack_app(), examples: Vec::new() }, "../../🏅️standards/🔖️1/🪆️subsets/✳️any");
semio_framework_plugin::composed_child_history_law!("trinity", super::TrinityJackPlayApp, || semio_framework_plugin::App { definition: super::create_trinity_jack_app(), examples: Vec::new() }, [("patchNodes", r#"{"nodeIds":["7dc5b737-3b6b-4068-b315-b7bacc91c2e1"],"field":"name","value":"Renamed core"}"#)]);
