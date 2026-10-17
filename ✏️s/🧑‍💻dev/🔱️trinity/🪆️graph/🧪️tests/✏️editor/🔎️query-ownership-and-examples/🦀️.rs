//! 🔎️ Query ownership, the shared document query, window kind actions and the shipped queries of the editor laws. See [`super`].

use super::*;

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
        app.dispatch_typed(
            TrinityJackCommand::RunQuery { query: Some("MATCH (a:Piece) RETURN a.name".into()), results_window_id: "results-main".into() },
            &semio_framework_plugin::ActionMeta { view_state: Some(editor), ..meta("query-owner") },&mut semio_framework_plugin::artifact_app_laws::fixture_identity()
        ,cx)
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
        let tree = app.render("trinity.jack.play.results", None, &results).await.map_err(|error| format!("{error:?}"))?;
        let rendered = artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)?;
        if !rendered.contains(&expected_name) {
            return Err("query result table did not contain the document's matching Piece".into());
        }
        Ok((generation, rendered))
    }
    .await;
    app.close(cx);
    let (generation, rendered) = outcome.expect("owned query runtime");
    assert!(rendered.contains("table"));

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

pub(super) async fn drive_query_ownership_operations(app: &mut VcsArtifactApp<EditorApp<TrinityJackPlayApp<SemioJackComposition>>, semio_s_artifact_stdio_semio::SemioMembers>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(u64, u64, u64), String> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    let mut transient_receipts = 0;
    let mut window_config_receipts = 0;
    let mut window_transient_receipts = 0;
    while app.has_pending_typed_operations() {
        if std::time::Instant::now() >= deadline {
            return Err("query operations did not finish".into());
        }
        app.maintenance_step(cx,jack::standards::v1::subsets::any::schema::content::trinity_mounted_owner_policy().maintenance).map_err(|error| format!("{error:?}"))?;
        app.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), cx).await.map_err(|error| format!("{error:?}"))?;
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
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let view = ViewModel {
        window_instances: vec![
            semio_framework_plugin::ViewWindowInstance { id: "editor-left".into(), window_kind_id: "trinity-jack-editor".into() },
            semio_framework_plugin::ViewWindowInstance { id: "editor-right".into(), window_kind_id: "trinity-jack-editor".into() },
            semio_framework_plugin::ViewWindowInstance { id: "results-left".into(), window_kind_id: "trinity-jack-results".into() },
            semio_framework_plugin::ViewWindowInstance { id: "results-right".into(), window_kind_id: "trinity-jack-results".into() },
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
            &semio_framework_plugin::ActionMeta { view_state: Some(editor_left.clone()), ..meta("query-owner") },&mut semio_framework_plugin::artifact_app_laws::fixture_identity()
        ,cx)
        .await;
    assert!(denied.is_err(), "an attached editor cannot be promoted to results mutation authority by command payload");

    app.dispatch_typed(TrinityJackCommand::TextEdit { text: left_query.into() }, &semio_framework_plugin::ActionMeta { view_state: Some(editor_left.clone()), ..meta("query-owner") }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(),cx)
        .await
        .expect("query edit");
    assert_eq!(drive_query_ownership_operations(&mut app,cx).await.expect("query edit"), (0, 0, 0));
    let document = app.snapshot().expect("document after the query edit");
    assert_eq!(document.query, left_query, "the typed query is the document's");
    for context in [&editor_left, &editor_right] {
        let tree = app.render("trinity.jack.play.editor", None, context).await.expect("editor render");
        let rendered = artifact_app_laws::project_and_retire_fixture_tree(tree).expect("editor projection");
        let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::TextEditorScene>(&rendered).expect("editor scene");
        assert_eq!(scene.buffer, left_query, "every editor window shows the document's one query");
    }

    for (context, target, query) in [(&editor_left, "results-left", None), (&editor_right, "results-right", Some(right_query.to_string()))] {
        app.dispatch_typed(TrinityJackCommand::RunQuery { query, results_window_id: target.into() }, &semio_framework_plugin::ActionMeta { view_state: Some(context.clone()), ..meta("query-owner") }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(),cx)
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
        let tree = app.render("trinity.jack.play.results", None, context).await.expect("result render");
        assert!(artifact_app_laws::project_and_retire_fixture_tree(tree).expect("result projection").contains("table"));
    }

    let reloaded = <jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&pack::snapshot::ArtifactPack::encode_pack(&document)).expect("the document pack decodes");
    assert_eq!(reloaded.query, left_query, "the query persists with the document");
    let admitted = semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.app,"undo",None,&semio_framework_plugin::ActionMeta { view_state: Some(editor_right.clone()), ..meta("query-owner") },&mut semio_framework_plugin::artifact_app_laws::fixture_identity(),cx,jack::standards::v1::subsets::any::schema::content::trinity_mounted_owner_policy().close).await.unwrap_or_else(|fault| panic!("undo admission: {fault:?}"));
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.app, admitted).await.unwrap_or_else(|fault| panic!("undo settles: {fault:?}"));
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

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");

    reopened.close(cx);
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
        semio_framework_plugin::resolve_window_actions(&definition, window).into_iter().map(|action| action.id.clone()).collect()
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
    let result = app.dispatch_typed(TrinityJackCommand::SetActiveExample { example_id: jack::examples::demo::ID.into() }, &meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(),cx).await.expect("set active example");
    let receipt = settle(&mut app,cx).await;
    assert!(!result.requested_effects.is_empty() || !receipt.effects.is_empty(), "the registered example id must request the LoadDocument effect");
    app.close(cx);

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
