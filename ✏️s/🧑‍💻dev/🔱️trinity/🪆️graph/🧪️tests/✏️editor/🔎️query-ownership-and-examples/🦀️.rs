//! 🔎️ Query ownership, the shared document query, window kind actions and the shipped queries of the editor laws. See [`super`].

use super::*;

#[semio_framework_async_macros::async_test]
async fn query_action_publishes_one_live_child_edit_and_undo_restores_it() {
    use jack::standards::v1::subsets::any::schema::inferences::JackInference;

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/▶️query-publication.json")).unwrap();
    let mut caller = artifact_app_laws::OriginalFixtureCaller::admit().unwrap();
    let mut context = caller.context();
    let cx = &mut context;
    let mut app = new_app().await;
    let view = query_windows().for_panel();
    let results = view.for_window_instance("results-main").unwrap();
    let meta = semio_plugin::ActionMeta { view_state: Some(view.into()), ..meta(semio_os_kernel::LOCAL_ACTOR_ID) };
    let initial = live_scene(&app).await;
    let source = app.snapshot().unwrap().query.clone();
    let root = fixture["initialRoot"].as_str().unwrap();
    assert_eq!(app.snapshot().unwrap().root_node_id.as_deref(), Some(root));
    assert_eq!(initial.nodes.len(), fixture["initialNodes"].as_u64().unwrap() as usize);

    let args = semio_framework_pack_json::json!({ "query": fixture["create"].as_str().unwrap() });
    let admitted = artifact_app_laws::dispatch_fixture_action(&mut app.app, "runQuery", Some(semio_framework_pack_json::to_dsl_value(&args)), &meta, &mut artifact_app_laws::fixture_identity(), cx, trinity_mounted_owner_policy().close).await.unwrap();
    semio_plugin::app::settle_framework_reserved_admission(&mut app.app, admitted, cx).await.unwrap();
    assert_eq!(drive_query_ownership_operations(&mut app, cx).await.unwrap(), (0, 0, 1));
    {
        let window = app.window_transient_snapshot(&results).unwrap().unwrap();
        let result = window.get::<JackResultsWindowTransientOwner>().unwrap();
        assert!(result.query_execution_id.is_some());
        assert!(result.query_error.is_none(), "{:?}", result.query_error);
        assert_eq!(result.result.as_ref().unwrap().rows.len(), 1);
    }
    let invalid_example = TrinityJackCommand::LoadExampleQuery { query: fixture["reject"].as_str().unwrap().into(), results_window_id: String::new() };
    artifact_app_laws::dispatch_original_fixture_command(&mut *app, &invalid_example, &meta, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut artifact_app_laws::fixture_identity(), cx).await.unwrap();
    assert_eq!(drive_query_ownership_operations(&mut app, cx).await.unwrap(), (0, 0, 1));
    {
        let window = app.window_transient_snapshot(&results).unwrap().unwrap();
        let result = window.get::<JackResultsWindowTransientOwner>().unwrap();
        assert!(result.query_execution_id.is_some());
        assert!(result.result.is_none());
        assert!(result.query_error.is_some());
    }
    assert_eq!(app.snapshot().unwrap().query, source);
    assert_eq!(app.window_transient_generation(&results).unwrap(), Some(2));
    let published = live_scene(&app).await;
    assert_eq!(published.nodes.len(), fixture["publishedNodes"].as_u64().unwrap() as usize);
    let created = published.nodes.iter().find(|node| !initial.nodes.iter().any(|before| before.id == node.id)).unwrap();
    assert_eq!(created.kind, fixture["kind"].as_str().unwrap());
    assert_eq!(published.edges, initial.edges);
    {
        let parent = app.snapshot().unwrap();
        let children = app.test_child_content_view();
        let graph = SemioJackComposition::bind(&parent, &children).unwrap();
        let inferred = JackInference::infer_from_graph(&parent, &graph).unwrap();
        assert_eq!(inferred.topology.node_count as usize, published.nodes.len());
        assert_eq!(u64::from(inferred.topology.depth[&created.id]), fixture["createdDepth"].as_u64().unwrap());
        assert!(inferred.flat_position.positions.contains_key(root));
    }

    let admitted = artifact_app_laws::dispatch_fixture_action(&mut app.app, "undo", None, &meta, &mut artifact_app_laws::fixture_identity(), cx, trinity_mounted_owner_policy().close).await.unwrap();
    semio_plugin::app::settle_framework_reserved_admission(&mut app.app, admitted, cx).await.unwrap();
    drive_query_ownership_operations(&mut app, cx).await.unwrap();
    let restored = live_scene(&app).await;
    assert_eq!(restored.nodes, initial.nodes);
    assert_eq!(restored.edges, initial.edges);
    assert_eq!(app.window_transient_generation(&results).unwrap(), Some(2));

    let delete = TrinityJackCommand::RunQuery { query: Some(fixture["deleteRoot"].as_str().unwrap().into()), results_window_id: String::new() };
    artifact_app_laws::dispatch_original_fixture_command(&mut *app, &delete, &meta, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut artifact_app_laws::fixture_identity(), cx).await.unwrap();
    assert_eq!(drive_query_ownership_operations(&mut app, cx).await.unwrap(), (0, 0, 1));
    {
        let parent = app.snapshot().unwrap();
        assert!(parent.root_node_id.is_none());
        let children = app.test_child_content_view();
        let graph = SemioJackComposition::bind(&parent, &children).unwrap();
        let inferred = JackInference::infer_from_graph(&parent, &graph).unwrap();
        assert_eq!(u64::from(inferred.topology.node_count), fixture["afterRootDeletionNodes"].as_u64().unwrap());
        assert!(!inferred.topology.depth.contains_key(root));
        assert!(!inferred.flat_position.positions.contains_key(root));
    }
    assert_eq!(live_scene(&app).await.edges.len(), fixture["afterRootDeletionEdges"].as_u64().unwrap() as usize);

    let admitted = artifact_app_laws::dispatch_fixture_action(&mut app.app, "undo", None, &meta, &mut artifact_app_laws::fixture_identity(), cx, trinity_mounted_owner_policy().close).await.unwrap();
    semio_plugin::app::settle_framework_reserved_admission(&mut app.app, admitted, cx).await.unwrap();
    drive_query_ownership_operations(&mut app, cx).await.unwrap();
    let restored = live_scene(&app).await;
    assert_eq!(app.snapshot().unwrap().root_node_id.as_deref(), Some(root));
    assert_eq!(restored.nodes, initial.nodes);
    assert_eq!(restored.edges, initial.edges);

    app.close(cx);
    let fuel = cx.fuel_remaining();
    drop(context);
    caller.retire(fuel).unwrap();
}

#[semio_framework_async_macros::async_test]
async fn query_ownership_runtime_publishes_transient_result_without_document_edit() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let outcome: Result<(u64, String), String> = async {
        let before = app.ephemeral_snapshot().await.transient_generation;
        let document = app.snapshot().map_err(|error| format!("{error:?}"))?.clone();
        let expected_name = live_scene(&app).await.nodes.into_iter().find(|node| node.kind == "Piece").ok_or_else(|| "query runtime fixture has no Piece".to_string())?.name;
        let view = query_windows();
        let editor = view.for_window_instance("editor-main").ok_or("missing editor window")?;
        let results = view.for_window_instance("results-main").ok_or("missing results window")?;
        semio_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut *app, &TrinityJackCommand::RunQuery { query: Some("MATCH (a:Piece) RETURN a.name".into()), results_window_id: "results-main".into() }, &semio_plugin::ActionMeta { view_state: Some(editor.into()), ..meta(semio_os_kernel::LOCAL_ACTOR_ID) }, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), cx)
        .await
        .map_err(|error| format!("{error:?}"))?;
        if drive_query_ownership_operations(&mut app,cx).await? != (0, 0, 1) {
            return Err("query operation did not produce exactly one results-transient receipt".into());
        }
        if app.snapshot().map_err(|error| format!("{error:?}"))? != document {
            return Err("read query modified the document".into());
        }
        let generation = app.window_transient_generation(&results).map_err(|error| format!("{error:?}"))?.ok_or("missing results transient generation")?;
        if generation != 1 || app.ephemeral_snapshot().await.transient_generation != before {
            return Err("query result did not publish exactly once to only the results-window transient".into());
        }
        let rendered = artifact_app_laws::render_registered_fixture_app(&mut app.app, "trinity.jack.play.results", &results, cx).await.map_err(|error| format!("{error:?}"))?;
        if !rendered.contains(&expected_name) {
            return Err("query result table did not contain the document's matching Piece".into());
        }
        Ok((generation, rendered))
    }
    .await;
    app.close(cx);
    let (generation, rendered) = outcome.expect("owned query runtime");
    assert_eq!(generation, 1);
    assert!(rendered.contains("table"));

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

pub(super) async fn drive_query_ownership_operations(app: &mut VcsArtifactApp<EditorApp<TrinityJackPlayApp<SemioJackComposition>>, semio_s_artifact_stdio_semio::SemioMembers>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(u64, u64, u64), String> {
    use semio_plugin::app::TypedOperationResultLane::{Transient, WindowConfig, WindowTransient};

    let receipt = artifact_app_laws::settle_registered_typed_operation(app, JACK_TEST_INSTANCE, trinity_mounted_owner_policy(), &mut artifact_app_laws::fixture_identity(), cx)
        .await
        .map_err(|error| format!("{error:?}"))?;
    let count = |lane| receipt.lanes.iter().filter(|&&observed| observed == lane).count() as u64;
    Ok((count(Transient), count(WindowConfig), count(WindowTransient)))
}

#[semio_framework_async_macros::async_test]
async fn query_ownership_rejected_editor_target_retires_without_publication() {
    let mut caller = artifact_app_laws::OriginalFixtureCaller::admit().unwrap();
    let mut context = caller.context();
    let cx = &mut context;
    let mut app = new_app().await;
    let view = query_windows();
    let editor = view.for_window_instance("editor-main").unwrap();
    let results = view.for_window_instance("results-main").unwrap();
    let document = app.snapshot().unwrap();
    let fixture_grant = cx.retained_grant();
    let rejection = semio_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut *app, &TrinityJackCommand::RunQuery { query: None, results_window_id: "editor-main".into() }, &semio_plugin::ActionMeta { view_state: Some(editor.into()), ..meta(semio_os_kernel::LOCAL_ACTOR_ID) }, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), cx)
        .await.expect_err("an attached editor cannot own query results");
    assert!(format!("{rejection:?}").contains("targeted window transient capture does not match the command owner's expected window kind"), "{rejection:?}");
    let close = trinity_mounted_owner_policy().close;
    assert!(!app.maintenance_source_terminal_is_empty(cx), "rejected typed ingress still owns its source");
    for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns() {
        if app.maintenance_source_terminal_is_empty(cx) {
            break;
        }
        artifact_app_laws::fixture_renew_turn(cx, artifact_app_laws::OriginalFixtureCaller::turn_fuel(), fixture_grant).unwrap();
        cx.with_original_retirement(|cx| cx.with_retained_loan(close, |cx| {
            let grant = cx.retained_grant();
            let demand = app.maintenance_source_close_demands(grant.maximum_copy_bytes, cx)?;
            assert!(semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand), "unfunded rejected command retirement: {demand:?} against {grant:?}");
            app.maintenance_source_close_step(cx, grant).map_err(Into::into)
        })).expect("rejected command source retires");
    }
    assert!(app.maintenance_source_terminal_is_empty(cx), "rejected command retained {:?}: {}", app.maintenance_source_close_demands(close.maximum_copy_bytes, cx), app.typed_operation_diagnostic());
    assert_eq!(drive_query_ownership_operations(&mut app, cx).await.expect("rejected query settles"), (0, 0, 0));
    assert_eq!(app.snapshot().unwrap(), document);
    assert_eq!(app.window_transient_generation(&results).unwrap(), Some(0));
    app.close(cx);
    let fuel = cx.fuel_remaining();
    drop(context);
    caller.retire(fuel).unwrap();
}

/// 🔎️ The Jack query is DOCUMENT content (ticket 26/09/23 C12, decision (a)): every editor window shows the one query the
/// document holds, an edit from either editor is one undoable document edit and persists with the document's pack, and running
/// a query — from either editor, into either results window — reads the document without editing it. Only the results stay
/// per window, and they are never persisted: a reopened app starts with fresh results windows.
#[semio_framework_async_macros::async_test]
async fn jack_query_is_document_content_shared_by_every_editor_while_results_stay_per_window() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let view = ViewModel {
        window_instances: vec![
            semio_plugin::ViewWindowInstance { id: "editor-left".into(), window_kind_id: "trinity-jack-editor".into() },
            semio_plugin::ViewWindowInstance { id: "editor-right".into(), window_kind_id: "trinity-jack-editor".into() },
            semio_plugin::ViewWindowInstance { id: "results-left".into(), window_kind_id: "trinity-jack-results".into() },
            semio_plugin::ViewWindowInstance { id: "results-right".into(), window_kind_id: "trinity-jack-results".into() },
        ],
        ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
    };
    let editor_left = view.for_window_instance("editor-left").unwrap();
    let editor_right = view.for_window_instance("editor-right").unwrap();
    let results_left = view.for_window_instance("results-left").unwrap();
    let results_right = view.for_window_instance("results-right").unwrap();
    let left_query = "MATCH (a:Piece) WHERE a.name = 'b' RETURN a.name";
    let right_query = "MATCH (a:Piece) WHERE a.name != 'b' RETURN a.name";
    let app_config_before = app.config_pack().await.expect("app config pack");

    semio_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut *app, &TrinityJackCommand::TextEdit { text: left_query.into() }, &semio_plugin::ActionMeta { view_state: Some(editor_left.clone().into()), ..meta(semio_os_kernel::LOCAL_ACTOR_ID) }, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), cx)
        .await
        .expect("query edit");
    assert_eq!(drive_query_ownership_operations(&mut app,cx).await.expect("query edit"), (0, 0, 0));
    let document = app.snapshot().expect("document after the query edit");
    assert_eq!(document.query, left_query, "the typed query is the document's");
    for context in [&editor_left, &editor_right] {
        let rendered = artifact_app_laws::render_registered_fixture_app(&mut app.app, "trinity.jack.play.editor", context, cx).await.expect("editor render");
        let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_plugin::TextEditorScene>(&rendered).expect("editor scene");
        assert_eq!(scene.buffer, left_query, "every editor window shows the document's one query");
    }

    let editor_right_with_focused_results = ViewModel { focused_window_id: Some("results-right".into()), ..editor_right.clone() };
    for (context, target, query) in [(&editor_left, "results-left", None), (&editor_right_with_focused_results, "", Some(right_query.to_string()))] {
        semio_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut *app, &TrinityJackCommand::RunQuery { query, results_window_id: target.into() }, &semio_plugin::ActionMeta { view_state: Some(context.clone().into()), ..meta(semio_os_kernel::LOCAL_ACTOR_ID) }, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), cx)
            .await
            .expect("paired query");
    }
    assert_eq!(drive_query_ownership_operations(&mut app,cx).await.expect("paired queries"), (0, 0, 2));
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
        let tree = artifact_app_laws::render_registered_fixture_app(&mut app.app, "trinity.jack.play.results", context, cx).await.expect("result render");
        assert!(tree.contains("table"));
    }

    let reloaded = <jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&pack::snapshot::ArtifactPack::encode_pack(&document)).expect("the document pack decodes");
    assert_eq!(reloaded.query, left_query, "the query persists with the document");
    let admitted = semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.app,"undo",None,&semio_plugin::ActionMeta { view_state: Some(editor_right.clone().into()), ..meta(semio_os_kernel::LOCAL_ACTOR_ID) },&mut semio_plugin::artifact_app_laws::fixture_identity(),cx,jack::standards::v1::subsets::any::schema::content::trinity_mounted_owner_policy().close).await.unwrap_or_else(|fault| panic!("undo admission: {fault:?}"));
    semio_plugin::app::settle_framework_reserved_admission(&mut app.app, admitted, cx).await.unwrap_or_else(|fault| panic!("undo settles: {fault:?}"));
    drive_query_ownership_operations(&mut app,cx).await.expect("undo publishes");
    assert_eq!(app.snapshot().expect("document after undo").query, jack::standards::v1::subsets::any::schema::graph::TRINITY_JACK_DEFAULT_QUERY, "one undo from either editor reverts the query edit");
    app.close(cx);

    let mut reopened = new_app().await;
    for context in [&results_left, &results_right] {
        assert_eq!(reopened.window_transient_generation(context).expect("fresh result generation"), Some(0));
        let snapshot = reopened.window_transient_snapshot(context).expect("fresh result snapshot").expect("fresh results owner");
        let state = snapshot.get::<JackResultsWindowTransientOwner>().expect("fresh results state");
        assert!(state.query_execution_id.is_none() && state.result.is_none() && state.query_error.is_none());
    }
    reopened.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

/// 🎫️ Slice B3b (ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END). Every retained WINDOW-CONFIG
/// verb reads the config of the window it was dispatched from, so leaving one unowned is a live
/// fault: `build_definition` copies an unowned action onto EVERY window kind, and the graph pane's
/// Actions list then offered `formatDocument`, which the reducer refused with "Jack query formatting
/// requires the exact editor-window config snapshot". Measured on the React shell at 6054 before the
/// fix (`🗑️generated/b3b-trinity-jack-console.txt`).
#[semio_framework_async_macros::async_test]
async fn window_kind_actions_scope_text_verbs_to_the_query_editor() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let definition = create_trinity_jack_app();
    let resolve = |window_id: &str| -> Vec<String> {
        let window = definition.window_kinds.iter().find(|window| window.id == window_id).expect("window kind");
        semio_plugin::resolve_window_actions(&definition, window).into_iter().map(|action| action.id.clone()).collect()
    };
    let graph = resolve("trinity-jack-graph");
    let editor = resolve("trinity-jack-editor");
    let results = resolve("trinity-jack-results");
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

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

/// 🎫️ Slice B3b. The navbar example picker dispatches a REGISTERED example id, and `demo` is the only
/// example this subset registers — `example_dsl_for_preset` knew `nakagin`/`branch-chain` only, so
/// every navbar pick resolved to `None` and loaded nothing with no fault anywhere. The staged argument
/// form must offer the same registered id, never an id the resolver would drop.
#[semio_framework_async_macros::async_test]
async fn set_active_example_resolves_every_id_the_shell_can_send() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let result = semio_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut *app, &TrinityJackCommand::SetActiveExample { example_id: jack::examples::demo::ID.into() }, &meta("local"), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), cx).await.expect("set active example");
    let receipt = settle(&mut app,cx).await;
    assert!(!result.requested_effects.is_empty() || !receipt.effects.is_empty(), "the registered example id must request the LoadDocument effect");
    app.close(cx);

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
            semio_plugin::ArgSchema::String { options, .. } => options.iter().map(|option| option.value.clone()).collect::<Vec<_>>(),
            _ => Vec::new(),
        })
        .collect();
    assert!(offered.contains(&jack::examples::demo::ID.to_string()), "the staged form must offer the registered example, offered {offered:?}");

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
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
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let example = SemioJackComposition::initial_snapshot();
    for query in [jack::standards::v1::subsets::any::schema::graph::TRINITY_JACK_DEFAULT_QUERY, "MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a, r, b"] {
        let graph = jack::standards::v1::subsets::any::schema::memory::Graph::from_snapshot(example.clone()).expect("curated example graph");
        let diagnostics = jack::standards::v1::subsets::any::schema::language_service::lint(&graph, query);
        assert!(diagnostics.is_empty(), "{query} must lint clean on the curated example, got {diagnostics:?}");
        let mut graph = graph;
        jack::standards::v1::subsets::any::schema::runner::run(&mut graph, query).unwrap_or_else(|error| panic!("{query} must run on the curated example: {error}"));
    }

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}
