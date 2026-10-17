//! 🖼️ Scene helpers, rendering, query runs, selection and text edits of the editor laws. See [`super`].

use super::*;

/// 🧸️ The live scene composed from the app's `content` member store (design §20.15: the parent holds no content).
pub(super) async fn live_scene(app: &VcsArtifactApp<EditorApp<TrinityJackPlayApp<SemioJackComposition>>, semio_s_artifact_stdio_semio::SemioMembers>) -> jack::standards::v1::subsets::any::schema::content::JackWorkingScene {
    use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot;
    use store::{pack::snapshot::ArtifactPack, SpaceMember};
    let snapshot = app.snapshot().expect("Jack parent projection");
    let bytes = app.child_store("content", &snapshot.content.child_id).await.expect("Jack content child").document_pack_bytes().await.expect("Jack content child pack");
    let (nodes, edges) = crate::content::working_from_jack_content_snapshot(&SemioGraphSnapshot::decode_pack(&bytes).expect("Jack content child snapshot")).expect("Jack content child scene");
    jack::standards::v1::subsets::any::schema::content::JackWorkingScene { nodes, edges }
}

pub(super) async fn node_id_at(app: &VcsArtifactApp<EditorApp<TrinityJackPlayApp<SemioJackComposition>>, semio_s_artifact_stdio_semio::SemioMembers>, index: usize) -> String {
    live_scene(app).await.nodes[index].id.clone()
}

/// 🏷️ The live names of the nodes `ids` names, every node when `ids` is `None`, in scene order.
pub(super) async fn node_names(app: &VcsArtifactApp<EditorApp<TrinityJackPlayApp<SemioJackComposition>>, semio_s_artifact_stdio_semio::SemioMembers>, ids: Option<&[String]>) -> Vec<String> {
    live_scene(app).await.nodes.into_iter().filter(|node| ids.is_none_or(|ids| ids.contains(&node.id))).map(|node| node.name).collect()
}

/// 🏷️ The live name of node `id`.
pub(super) async fn node_name(app: &VcsArtifactApp<EditorApp<TrinityJackPlayApp<SemioJackComposition>>, semio_s_artifact_stdio_semio::SemioMembers>, id: &str) -> String {
    live_scene(app).await.nodes.into_iter().find(|node| node.id == id).map(|node| node.name).expect("node")
}

/// 🕹️ Dispatches the framework-injected `interactionSelect` verb against domain "ast" — the
/// replacement for the deleted `TrinityJackCommand::SetSelection`.
pub(super) async fn select_ast(app: &mut VcsArtifactApp<EditorApp<TrinityJackPlayApp<SemioJackComposition>>, semio_s_artifact_stdio_semio::SemioMembers>, ids: &[&str],cx:&mut semio_framework_job::StepContext<'_>) {
    let targets: Vec<semio_framework_pack_json::Value> = ids.iter().map(|id| semio_framework_pack_json::json!({ "granularity": "node", "id": id })).collect();
    let args = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({ "domainId": "ast", "targets": semio_framework_pack_json::to_json_string(&targets) }));
    let admitted = semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.app,"interactionSelect",Some(args),&meta("local"),&mut semio_framework_plugin::artifact_app_laws::fixture_identity(),cx,jack::standards::v1::subsets::any::schema::content::trinity_mounted_owner_policy().close).await.expect("interactionSelect");
    // 🕹️ On a mounted app the verb is admitted as a framework-reserved job and publishes later —
    // settle both so the selection is live before the next dispatch reads it.
    semio_framework_plugin::app::settle_framework_reserved_admission(app, admitted).await.expect("interactionSelect reserved-job commit");
    artifact_app_laws::settle_registered_typed_operation(app, JACK_TEST_INSTANCE, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(),cx).await.expect("interactionSelect publication");
}

#[semio_framework_async_macros::async_test]
async fn renders_node_graph_scene() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let node = app.render("trinity.jack.play.main", None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
    assert!(artifact_app_laws::project_and_retire_fixture_tree(node).expect("project semantic UI test tree").contains("node-graph"));

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

#[semio_framework_async_macros::async_test]
async fn renders_jack_editor() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let node = app.render("trinity.jack.play.editor", None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
    let json = artifact_app_laws::project_and_retire_fixture_tree(node).expect("project semantic UI test tree");
    assert!(json.contains("text-editor"));
    // 🚚️ The buffer rides an out-of-doc payload lane, never the projected doc spine — read the
    // assembled scene the way a render host does.
    let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::TextEditorScene>(&json).expect("text-editor scene");
    assert_eq!(scene.buffer, jack::standards::v1::subsets::any::schema::graph::TRINITY_JACK_DEFAULT_QUERY);

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

#[semio_framework_async_macros::async_test]
async fn run_query_populates_results_and_a_set_query_mutates_projection() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let view = query_windows();
    let editor = view.for_window_instance("editor-main").unwrap();
    app.dispatch_typed(
        TrinityJackCommand::RunQuery { query: Some("MATCH (a:Piece) WHERE a.name = 'b' SET a.label = 'ran-label'".into()), results_window_id: "results-main".into() },
        &semio_framework_plugin::ActionMeta { view_state: Some(editor), ..meta("local") },&mut semio_framework_plugin::artifact_app_laws::fixture_identity()
    ,cx)
    .await
    .expect("run");
    drive_query_ownership_operations(&mut app,cx).await.expect("query completes");
    // 🔬 The query's SET is ONE edit of the composed `content` child: the live member store holds the label.
    assert!(live_scene(&app).await.nodes.iter().any(|node| node.properties.get("label") == Some(&semio_framework_graph_manifest::PropertyValue::String("ran-label".into()))));

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

#[semio_framework_async_macros::async_test]
async fn node_graph_select_updates_selection_and_document_tree() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let node_id = node_id_at(&app, 0).await;
    select_ast(&mut app, &[&node_id],cx).await;
    let tree = app.render("trinity.jack.play.artifact", None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
    let json = artifact_app_laws::project_and_retire_fixture_tree(tree).expect("project semantic UI test tree");
    assert!(json.contains(&node_id));
    // 🕹️ Rows carry no `selected` flag of their own any more: the tree binds the framework-owned
    // `"ast"` interaction domain and the host paints the selection from the interaction state — so
    // the proof is the binding plus the selection being the one `deleteSelection` reads
    // (`delete_selection_removes_selected_node`).
    assert!(json.contains("\"interactionDomain\":\"ast\""), "document tree must bind the ast interaction domain: {json}");

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

#[semio_framework_async_macros::async_test]
async fn nakagin_fixture_has_nodes() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    assert!(!jack::standards::v1::subsets::any::schema::snapshot::jack_working_scene(&SemioJackComposition::initial_snapshot()).expect("curated example scene").nodes.is_empty());

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

#[semio_framework_async_macros::async_test]
async fn editor_scene_has_tokens_and_diagnostics() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let node = app.render("trinity.jack.play.editor", None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
    let json = artifact_app_laws::project_and_retire_fixture_tree(node).expect("project semantic UI test tree");
    let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::TextEditorScene>(&json).expect("text-editor scene");
    assert!(scene.tokens_json.is_some_and(|tokens| !tokens.is_empty()));
    assert!(scene.diagnostics_json.is_some());
    assert!(scene.completions_json.is_some());

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

#[semio_framework_async_macros::async_test]
async fn text_edit_puts_the_query_into_the_document() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let view = query_windows();
    let editor = view.for_window_instance("editor-main").unwrap();
    app.dispatch_typed(TrinityJackCommand::TextEdit { text: "MATCH (a:Piece) RETURN a.name".into() }, &semio_framework_plugin::ActionMeta { view_state: Some(editor.clone()), ..meta("local") }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(),cx)
        .await
        .expect("edit");
    drive_query_ownership_operations(&mut app,cx).await.expect("edit completes");
    let node = app.render("trinity.jack.play.editor", None, &editor).await.expect("render");
    let json = artifact_app_laws::project_and_retire_fixture_tree(node).expect("project semantic UI test tree");
    let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::TextEditorScene>(&json).expect("text-editor scene");
    assert_eq!(scene.buffer, "MATCH (a:Piece) RETURN a.name");
    assert_eq!(app.snapshot().expect("projection").query, "MATCH (a:Piece) RETURN a.name", "the typed query is document content, not a window's config");

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

/// 🔎️ The query the jack editor window shows (its text-editor scene buffer).
async fn jack_query(app: &mut JackTestApp, editor: &ViewModel) -> String {
    let node = app.render("trinity.jack.play.editor", None, editor).await.expect("render");
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
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

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
        semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.app,"textEdit",Some(args),&meta_of(),&mut semio_framework_plugin::artifact_app_laws::fixture_identity(),cx,jack::standards::v1::subsets::any::schema::content::trinity_mounted_owner_policy().close).await.unwrap_or_else(|error| panic!("keystroke {index} was refused: {error:?}"));
        drive_query_ownership_operations(&mut app,cx).await.unwrap_or_else(|error| panic!("keystroke {index} did not publish: {error}"));
        artifact_app_laws::drain_maintenance_pressure(&mut app.app,cx);
    }
    assert_eq!(jack_query(&mut app, &editor).await, run.expected, "the query window shows the open run");
    assert_eq!((app.snapshot().expect("projection").query.clone(), app.edit_transactions().len()), (before.clone(), edits), "nothing lands while the run is open");
    app.set_tool_clock_ms(Some(now + 750));
    let commit = semio_framework_value::DslValue::Object(vec![(semio_framework_plugin::TYPING_BUFFER_ARG.into(), semio_framework_value::DslValue::String("trinity.jack.query".into())), (semio_framework_plugin::TYPING_COMMIT_ARG.into(), semio_framework_value::DslValue::String("idle".into()))]);
    semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.app,"textEdit",Some(commit),&meta_of(),&mut semio_framework_plugin::artifact_app_laws::fixture_identity(),cx,jack::standards::v1::subsets::any::schema::content::trinity_mounted_owner_policy().close).await.expect("the idle commit");
    drive_query_ownership_operations(&mut app,cx).await.expect("the run publishes");
    assert_eq!(app.snapshot().expect("projection").query, run.expected);
    let transactions = app.edit_transactions();
    assert_eq!(transactions.len(), edits + 1, "the run lands as ONE edit");
    assert!(transactions.last().cloned().flatten().is_some_and(|transaction| transaction.tool.ends_with("#textEdit")), "the edit carries the run's TransactionRef: {transactions:?}");
    for (verb, expected) in [("undo", &before), ("redo", &run.expected)] {
        let admitted = semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.app,verb,None,&meta_of(),&mut semio_framework_plugin::artifact_app_laws::fixture_identity(),cx,jack::standards::v1::subsets::any::schema::content::trinity_mounted_owner_policy().close).await.unwrap_or_else(|fault| panic!("{verb} admission: {fault:?}"));
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.app, admitted).await.unwrap_or_else(|fault| panic!("{verb} settles: {fault:?}"));
        drive_query_ownership_operations(&mut app,cx).await.unwrap_or_else(|error| panic!("{verb} publishes: {error}"));
        assert_eq!(&jack_query(&mut app, &editor).await, expected, "one {verb} moves the whole run");
    }

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}
