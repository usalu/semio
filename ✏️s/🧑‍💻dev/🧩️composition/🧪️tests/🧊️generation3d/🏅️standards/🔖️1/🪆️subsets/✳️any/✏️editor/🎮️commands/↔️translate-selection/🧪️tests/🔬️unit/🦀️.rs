use super::*;
use crate::editor_domain::editor_laws::context::{app, dispatch};
use semio_s_artifact_procedural_generation3d::editor::generation3d::Generation3dCommand;
use semio_s_artifact_procedural_generation3d::widget_id;
use semio_framework_artifact_flow_flow::Widget;
use crate::editor_domain::editor_laws::context;

#[semio_framework_async_macros::async_test]
async fn translate_selection_persists_transform_into_flow_graph() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app().await;
    let before = context::snapshot(&app);
    assert!(before.host_snapshot.synapses.iter().any(|synapse| synapse.from == "extrude" && synapse.to == "column-preview"));
    dispatch(&mut app, Generation3dCommand::TranslateSelection(TranslateSelection { node_ids: vec!["extrude".into()], dx: 1.0, dy: 2.0, dz: 3.0, phase: None, reason: None, window_id: None })).await;
    let projection = context::snapshot(&app);
    let transform_id = "extrude__gumball_translate";
    let transform = projection.host_snapshot.widgets.iter().find(|widget| widget_id(widget) == transform_id).expect("transform neuron created");
    assert!(matches!(transform, Widget::Neuron { neuron_kind, .. } if neuron_kind == "brep.xform.translate"));
    let offset = crate::gumball_param_vector(&projection.host_snapshot, transform_id, "offset", [0.0; 3]);
    assert_eq!(offset, [1.0, 2.0, 3.0]);

    // Re-grabbing the same transform accumulates the delta instead of creating a second node.
    dispatch(&mut app, Generation3dCommand::TranslateSelection(TranslateSelection { node_ids: vec![transform_id.into()], dx: 1.0, dy: 0.0, dz: 0.0, phase: None, reason: None, window_id: None })).await;
    let projection2 = context::snapshot(&app);
    assert_eq!(projection2.host_snapshot.widgets.iter().filter(|widget| widget_id(widget) == transform_id).count(), 1);
    assert_eq!(crate::gumball_param_vector(&projection2.host_snapshot, transform_id, "offset", [0.0; 3]), [2.0, 2.0, 3.0]);
}

//#region 🛠️GumballTool
/// 🧾️ Every applied history row that carries document operations, oldest first.
async fn edit_rows(app: &mut context::Generation3dApp) -> Vec<semio_framework::kernel::HistoryEntry> {
    let mut rows: Vec<semio_framework::kernel::HistoryEntry> = semio_framework_plugin::PluginApp::history_snapshot(app).await.expect("history").upserts.into_iter().filter(|entry| entry.applied && !entry.op_lines.is_empty()).collect();
    rows.sort_by_key(|entry| entry.seq);
    rows
}

/// 🎚️ One gumball tick exactly as the live `World3dHost` sends it (`phase` stream / commit / abort, the owning window).
async fn tick(app: &mut context::Generation3dApp, args: serde_json::Value) {
    let action_meta = semio_framework_plugin::artifact_app_laws::meta("local");
    let args: semio_framework_value::DslValue = args.into();
    app.handle_action("translateSelection", Some(&args), &action_meta).await.expect("translateSelection admitted");
    context::settle(app).await;
}

/// ⚖️ LAW: a one-shot gumball drag is ONE tool transaction — one edit, one history row stamped with its `TransactionRef`
/// whose ops are the splice that inserts the missing operator and the RELATIVE `drag-transforms` leaf, labelled from that
/// leaf; a second drag on the same shape reuses the operator and is a second transaction carrying the leaf alone.
#[semio_framework_async_macros::async_test]
async fn a_gumball_drag_is_one_transaction_of_the_relative_leaf() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app().await;
    let before = edit_rows(&mut app).await.len();
    dispatch(&mut app, Generation3dCommand::TranslateSelection(TranslateSelection { node_ids: vec!["extrude".into()], dx: 1.0, dy: 2.0, dz: 3.0, phase: None, reason: None, window_id: None })).await;
    dispatch(&mut app, Generation3dCommand::TranslateSelection(TranslateSelection { node_ids: vec!["extrude".into()], dx: 0.5, dy: 0.0, dz: 0.0, phase: None, reason: None, window_id: None })).await;
    let rows = edit_rows(&mut app).await;
    let rows = &rows[before..];
    assert_eq!(rows.len(), 2, "two drags, two rows: {rows:?}");
    let refs: Vec<&semio_framework::kernel::HistoryTransaction> = rows.iter().map(|entry| entry.transaction.as_ref().expect("every drag row is a tool transaction")).collect();
    assert!(refs.iter().all(|transaction| transaction.id.starts_with("tx-") && transaction.tool == "s.procedural.generation3d@1/*#editor#translateSelection"), "{refs:?}");
    assert_ne!(refs[0].id, refs[1].id, "two drags are two transactions");
    assert!(rows[0].op_lines.last().is_some_and(|line| line.starts_with("drag-transforms")), "the first drag ends on its relative leaf: {:?}", rows[0].op_lines);
    assert!(rows[0].label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En).starts_with("Drag 1 shape(s) by (1, 2, 3)"), "the declared intent leaf labels a first grab, never its splice (design §19.1)");
    assert!(rows[1].op_lines.iter().all(|line| line.starts_with("drag-transforms")), "a re-grab is the relative leaf alone: {:?}", rows[1].op_lines);
    assert_eq!(rows[1].label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Drag 1 shape(s) by (0.5, 0, 0)");
    assert_eq!(rows[1].label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "1 Form(en) um (0,5; 0; 0) ziehen");
    assert_eq!(crate::gumball_param_vector(&context::snapshot(&app).host_snapshot, "extrude__gumball_translate", "offset", [0.0; 3]), [1.5, 2.0, 3.0]);
}

/// ⚖️ LAW: a streamed (live) gumball drag keeps its ticks in the window's ONE open transaction — the committed graph
/// does not move until the release, which commits the NET offset as ONE edit — and a host abort leaves zero trace.
#[semio_framework_async_macros::async_test]
async fn a_streamed_gumball_drag_is_one_edit_and_an_abort_is_zero_trace() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app().await;
    let before = edit_rows(&mut app).await.len();
    let rows_before = semio_framework_plugin::PluginApp::history_snapshot(&mut *app).await.expect("history").upserts.len();
    let committed = |app: &context::Generation3dApp| crate::gumball_param_vector(&context::snapshot(app).host_snapshot, "extrude__gumball_translate", "offset", [0.0; 3]);
    for (dx, phase) in [(1.0, "stream"), (2.0, "stream"), (0.5, "commit")] {
        tick(&mut app, serde_json::json!({ "nodeIds": ["extrude"], "dx": dx, "dy": 0.0, "dz": 0.0, "phase": phase, "windowId": "preview-1" })).await;
        if phase == "stream" {
            assert_eq!(committed(&*app), [0.0; 3], "a tick is provisional, never history");
        }
    }
    assert_eq!(committed(&*app), [3.5, 0.0, 0.0], "the release commits the net offset");
    let rows = edit_rows(&mut app).await;
    assert_eq!(rows.len() - before, 1, "one streamed drag, one row: {:?}", &rows[before..]);
    tick(&mut app, serde_json::json!({ "nodeIds": ["extrude__gumball_translate"], "dx": 4.0, "dy": 0.0, "dz": 0.0, "phase": "stream", "windowId": "preview-1" })).await;
    tick(&mut app, serde_json::json!({ "phase": "abort", "reason": "captureLost", "windowId": "preview-1" })).await;
    assert_eq!(committed(&*app), [3.5, 0.0, 0.0], "an aborted drag leaves zero trace");
    assert_eq!(edit_rows(&mut app).await.len() - before, 1, "and no row");
    let rows_after = semio_framework_plugin::PluginApp::history_snapshot(&mut *app).await.expect("history").upserts.len();
    assert_eq!(rows_after - rows_before, 1, "a tick or an abort logs no session row of its own either");
}
/// 🎚️ One streamed (or released) gumball tick of `extrude` in window `preview-1`, as the live `World3dHost` sends it.
fn streamed(phase: &str, offset: [f64; 3]) -> semio_s_artifact_procedural_generation3d::editor::generation3d::transform_commands::GumballDispatch<'static> {
    use semio_s_artifact_procedural_generation3d::editor::generation3d::transform_commands::{GesturePhase, GumballDispatch, GumballMotion};
    let phase = match phase { "abort" => GesturePhase::parse(Some("abort"), Some("blur")), other => GesturePhase::parse(Some(other), None) }.expect("a host phase");
    GumballDispatch { verb: "translateSelection", window: "preview-1", ids: vec!["extrude".into()], motion: GumballMotion::Translate(offset), phase, authoring_seed: "seed", base_revision: [0; 32] }
}

/// ⚖️ LAW (live consumer): while the first grab of a shape streams, the previews paint exactly what its release commits —
/// the overlay splices the transform operator (with its identity input, design §19.4) in place of the shape with the NET
/// offset, the marks follow the selection onto that operator, the committed snapshot stays untouched — and a host abort
/// leaves nothing to paint.
#[test]
fn an_open_gesture_previews_exactly_what_its_release_commits() {
    use semio_s_artifact_procedural_generation3d::editor::generation3d::{generation3d_gumball_preview, transform_commands::GumballGestures, PreviewInteractionMarks};
    use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::{example_snapshot, mutations::apply_generation3d_mutation, PROCEDURAL_EXAMPLE_HEX_COLUMN};
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let committed = example_snapshot(PROCEDURAL_EXAMPLE_HEX_COLUMN).expect("the hexagonal column example");
    let operator = "extrude__gumball_translate";
    let marks = PreviewInteractionMarks { selected: ["extrude".to_string()].into(), ..Default::default() };
    let mut gestures = GumballGestures::default();
    for offset in [[1.0, 0.0, 0.0], [0.5, 2.0, 0.0]] {
        assert!(gestures.dispatch(streamed("stream", offset), &committed.host_snapshot).expect("a tick streams").artifact_mutations.is_empty(), "a tick is provisional");
    }
    let (overlay, following) = generation3d_gumball_preview(&committed, &marks, &gestures).expect("an open gesture paints");
    let previews = |snapshot: &semio_s_artifact_procedural_generation3d::Generation3dSnapshot, id: &str| snapshot.host_snapshot.widgets.iter().find(|widget| widget_id(widget) == id).map(|widget| matches!(widget, Widget::Neuron { preview: true, .. }));
    assert_eq!((previews(&overlay, operator), previews(&overlay, "extrude")), (Some(true), Some(false)), "the operator is painted in place of the shape");
    assert_eq!(previews(&committed, operator), None, "the committed snapshot never sees the open gesture");
    assert_eq!(following.selected, [operator.to_string()].into(), "the marks follow the selection onto the operator");
    assert_eq!(crate::gumball_param_vector(&overlay.host_snapshot, operator, "offset", [f64::NAN; 3]), [1.5, 2.0, 0.0], "the preview composes the net offset from the identity");
    let released = gestures.dispatch(streamed("commit", [0.0; 3]), &committed.host_snapshot).expect("the release commits");
    assert!(released.transaction.is_some(), "the release is ONE tool transaction");
    let mut landed = committed.clone();
    for row in released.artifact_mutations {
        apply_generation3d_mutation(&mut landed, &row).expect("the committed rows apply");
        row.retire_cold();
    }
    assert_eq!(landed, overlay, "the preview painted exactly what the release committed");
    assert!(generation3d_gumball_preview(&committed, &marks, &gestures).is_none(), "a released gesture paints nothing more");
    gestures.dispatch(streamed("stream", [1.0, 0.0, 0.0]), &committed.host_snapshot).expect("a second gesture opens");
    gestures.dispatch(streamed("abort", [0.0; 3]), &committed.host_snapshot).expect("the host aborts");
    assert!(generation3d_gumball_preview(&committed, &marks, &gestures).is_none(), "an aborted gesture leaves nothing to paint");
    for snapshot in [landed, overlay, committed] {
        snapshot.retire_cold();
    }
}
/// 🛠️ LAW — the World3d live-consumer API end to end (`🌐️World3dHost/🧫️fixtures/🛠️gumball-live-protocol.json`, audit S2):
/// every verb dispatch the host owes for a scripted gesture, sent with its wire args exactly as the host sends them (the
/// pinned targets swapped for generation3d's `extrude` shape), publishes exactly the fixture's guest edits and moves the shape
/// by its offset — a streamed gesture is ONE edit, an aborted one leaves zero trace, one that never moved publishes nothing.
#[semio_framework_async_macros::async_test]
async fn the_world3d_gumball_live_protocol_lands_as_its_guest_edits() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/🛠️gumball-live-protocol.json"))).expect("the protocol fixture parses");
    let offset_of = |app: &context::Generation3dApp| crate::gumball_param_vector(&context::snapshot(app).host_snapshot, "extrude__gumball_translate", "offset", [0.0; 3]);
    let mut consumed = 0;
    for case in fixture["cases"].as_array().expect("cases").iter().filter(|case| case.get("guest").is_some()) {
        let name = case["name"].as_str().expect("name");
        let mut app = app().await;
        let before = edit_rows(&mut app).await.len();
        for dispatch in case["steps"].as_array().expect("steps").iter().map(|step| &step["dispatch"]).filter(|dispatch| !dispatch.is_null()) {
            let mut args = dispatch["args"].clone();
            args["ids"] = serde_json::json!(["extrude"]);
            let args: semio_framework_value::DslValue = args.into();
            let action = dispatch["action"].as_str().expect("action");
            app.handle_action(action, Some(&args), &semio_framework_plugin::artifact_app_laws::meta("local")).await.unwrap_or_else(|fault| panic!("{name}: {action} is admitted from the host's wire args: {fault:?}"));
            context::settle(&mut app).await;
        }
        let offset: Vec<f64> = case["guest"]["offset"].as_array().expect("offset").iter().map(|value| value.as_f64().expect("number")).collect();
        assert_eq!((edit_rows(&mut app).await.len() - before) as u64, case["guest"]["edits"].as_u64().expect("edits"), "{name}: the published edits");
        assert_eq!(offset_of(&app), [offset[0], offset[1], offset[2]], "{name}: the shape's offset");
        consumed += 1;
    }
    assert!(consumed >= 8, "every guest case of the protocol fixture is consumed: {consumed}");
}
//#endregion 🛠️GumballTool
