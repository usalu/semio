// 🗝️ Editor keys, history patch and actors: the shared corpus editor keys, history view patches, the worst severity of an edit and every route authoring as its acting actor.

//#region 🗝️EditorKeys
/// 🗝️ The shared band corpus whose `editorKeys` every shell's focus resolution reads (React `timeTravelFocusElementV1`).
const TIME_TRAVEL_BAND_CORPUS_JSON: &str = include_str!("../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🧫️fixtures/🧫️time-travel-band/🔣️.json");

/// ⚖️ LAW (design §16, G13): the draft editor's node keys are exactly the shared corpus's — the History panel tab the body
/// mounts in, Accept inside its row, and per input pointer (object fields flattened) one row holding its control — so a shell's
/// focus lands on the first input, else on Accept, whichever shell renders the body.
#[semio_framework_async_macros::async_test]
async fn the_draft_editor_keys_are_the_shared_corpus_keys_every_shell_focuses() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let corpus: Value = serde_json::from_str(TIME_TRAVEL_BAND_CORPUS_JSON).expect("band corpus parses");
    let keys = &corpus["editorKeys"];
    assert_eq!(text(&keys["panel"]), semio_framework_ui_layout::layout::FRAMEWORK_PANEL_TAB_HISTORY_ID, "the body mounts in the History panel tab");
    let rows = keys["inputs"].as_array().expect("editor input keys");
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 }), mounted_policy, &mut identity,cx).await;
    let editor = app.time_travel.editor_mut().expect("the draft editor");
    editor.inputs = vec![
        semio_framework::ActionArgDef::number("/dx", LocalizedLabel::native("dx", "dx")),
        semio_framework::ActionArgDef::object("/pivot", LocalizedLabel::native("Pivot", "Drehpunkt"), vec![semio_framework::ActionArgDef::number("/x", LocalizedLabel::native("x", "x"))]),
    ];
    editor.value = dsl(&serde_json::json!({ "dx": 1.0, "pivot": { "x": 2.0 } }));
    let history = render_history(&mut app, Locale::En).await;
    let accept_row = find_node(&history, text(&keys["accept"]["row"])).unwrap_or_else(|| panic!("the Accept row: {history}"));
    assert!(find_node(accept_row, text(&keys["accept"]["control"])).is_some(), "Accept sits in its row: {accept_row}");
    let inputs = find_node(&history, "framework.history.editor.inputs").expect("the inputs section");
    let produced: Vec<&str> = inputs["children"].as_array().expect("input rows").iter().filter_map(|row| row["key"].as_str()).collect();
    assert_eq!(produced, rows.iter().map(|row| text(&row["row"])).collect::<Vec<_>>(), "one row per corpus pointer, in order");
    for row in rows {
        let holder = find_node(inputs, text(&row["row"])).expect("the input row");
        assert!(find_node(holder, text(&row["control"])).is_some(), "{} edits through {}: {holder}", text(&row["pointer"]), text(&row["control"]));
    }
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🗝️EditorKeys

//#region 🩹️HistoryViewPatch
/// ✏️ Applies one toy edit straight on the document store — a row the command log backfills, like a seeded or ingested edit.
async fn apply_toy_edit(app: &mut ToyApp, value: i32, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
    app.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value }.into()], transaction: None }, identity).await.expect("toy edit applies");
}

/// 🧮️ Refreshes the history and answers how many rows it built and whether the cached view equals a full rebuild.
async fn refreshed_rows(app: &mut ToyApp) -> usize {
    HISTORY_ROWS_BUILT.with(|built| built.set(0));
    app.render_ctx().refresh_cache().await.expect("history refreshes");
    let built = HISTORY_ROWS_BUILT.with(std::cell::Cell::get);
    let cached = std::sync::Arc::clone(&app.history.cache.as_ref().expect("history cache").3);
    assert_eq!(cached.as_ref(), &app.render_ref().build_history_view(None).await, "the patched history equals a full rebuild");
    built
}

/// ⚖️ LAW (design §20.14, audit W2A-1): over a long history, appending an edit, undoing it, redoing it and re-rendering rebuild
/// at most the touched rows (the new row, the previous newest row and the previous top's row) — never the history — and the
/// patched view always equals a full rebuild; a history edit (a supersede transition) rebuilds the whole view.
#[semio_framework_async_macros::async_test]
async fn the_history_view_patches_only_the_rows_a_change_touched() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    for value in 0..200 {
        apply_toy_edit(&mut app, value, mounted_policy, &mut identity).await;
    }
    let seeded = app.history.command_log.len();
    let built = refreshed_rows(&mut app).await;
    let rows = app.history.command_log.len();
    assert_eq!(rows, seeded + 200, "the 200 edits are backfilled rows");
    assert!(built <= 200 + 2, "the first refresh after 200 backfilled edits builds the new rows once ({built})");
    assert_eq!(refreshed_rows(&mut app).await, 0, "a refresh without a change builds no row");
    apply_toy_edit(&mut app, 1_000, mounted_policy, &mut identity).await;
    assert!(refreshed_rows(&mut app).await <= 3, "an appended edit rebuilds its own row and its neighbours only");
    app.stores.store.dispatch(ArtifactCommand::Undo, &mut identity).await.expect("undo");
    assert!(refreshed_rows(&mut app).await <= 3, "an undo rebuilds the undone row and the uncovered top only");
    app.stores.store.dispatch(ArtifactCommand::Redo, &mut identity).await.expect("redo");
    assert!(refreshed_rows(&mut app).await <= 3, "a redo rebuilds the redone row and the covered top only");
    let _ = render_history(&mut app, Locale::En).await;
    assert_eq!(refreshed_rows(&mut app).await, 0, "rendering the history builds no row");
    let target = app.stores.store.mutation_ops().expect("mutation ops")[0].mutation_id.clone();
    app.stores.store.dispatch(ArtifactCommand::Supersede { scope: None, inputs: vec![store::os_store::store_schema::SupersedeInput { target, replacement: Some(SetCount { value: 7 }.into()) }] }, &mut identity).await.expect("a history edit lands");
    assert!(refreshed_rows(&mut app).await >= rows, "a history edit rebuilds the whole view");
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW (audit W2A-11): a row whose operations past the cap carry outcomes projects its most severe one, so the row's worst
/// severity is the worst over every operation of its edit, however many are flagged; the projection stays bounded and in op
/// order.
#[test]
fn the_projected_rows_carry_the_worst_severity_of_the_whole_edit() {
    use semio_framework_diagnostic::Severity;
    for (len, error_at) in [(200, 199), (200, 33), (65, 64), (40, 39), (10, 5)] {
        let flagged = (0..len).filter(|index| *index >= HISTORY_ROW_MUTATION_ROWS || *index == error_at).map(|index| (index, Some(if index == error_at { Severity::Error } else { Severity::Warning })));
        let shown = time_travel::history_projected_operations(len, flagged);
        assert!(shown.contains(&error_at), "{len} operations: the Error at {error_at} is projected");
        assert!(shown.len() <= 2 * HISTORY_ROW_MUTATION_ROWS, "{len} operations: the projection stays bounded");
        assert!(shown.windows(2).all(|pair| pair[0] < pair[1]), "{len} operations: the projection is in op order");
        assert!(shown.iter().take(len.min(HISTORY_ROW_MUTATION_ROWS)).copied().eq(0..len.min(HISTORY_ROW_MUTATION_ROWS)), "{len} operations: the first ones always lead");
    }
}

/// ⚖️ LAW (design §22.34, the store half — red until a plain `Apply` authors as the store's actor): every route authors
/// as its acting actor. A route without an actor of its own (the text ingest) authors as the instance's admitted actor; a
/// route with one (the plain emit route) authors as its separately admitted user instance.
#[semio_framework_async_macros::async_test]
async fn every_route_authors_as_its_acting_actor() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let acting = |actor: &str| ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(actor) };
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId("ada".into()), mounted_policy, &mut identity).await;
    PluginApp::bind_actor(&mut app, "ada").await;
    let first: TestMutation = SetCount { value: 1 }.into();
    PluginApp::ingest_operations_text(&mut app, &::protocol::OpText::print_op(&first), &mut identity).await.expect("the text ingest applies");
    let ingested = edit_authors(&app);
    let pack = app.document_pack().await.expect("Ada's authoritative document");
    let mut grace = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId("grace".into()), mounted_policy, &mut identity).await;
    artifact_app_laws::load_document(&mut grace, &pack, &mut identity).await.expect("Grace opens Ada's shared history");
    grace.dispatch_emit("setCount", Emit::<TestMutation, TestConfigMutation, NoDraftMutation>::mutations(vec![SetCount { value: 2 }.into()]), &acting("grace"), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("the emit publishes");
    let emitted = edit_authors(&grace);
    close(&mut app,cx);
    close(&mut grace,cx);
    assert_eq!(ingested, [Some("ada".to_string())], "a route without an actor of its own authors as the instance's");
    assert_eq!(emitted, [Some("ada".to_string()), Some("grace".to_string())], "the plain emit route authors as its acting actor");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🪪️ActorIdentity
