//! 🪣️ The bucket gesture end to end through the registry-backed app and its retained route: one click is ONE edit and ONE
//! history row stamped with the bucket's `TransactionRef`, holding ONE editable `fill-region` leaf built from the session
//! colour and tolerance (a session edit, never history); time travel edits the
//! leaf's seed, tolerance and colour through the real `historyEdit*` verbs, and the overwritten head equals the fresh fold
//! of the edited log — the region re-flooded and re-filled on its base by the one shared engine.

use super::unit_tests::context::{app, dispatch, raster_view_state, RasterAppFixture};
use super::*;
use crate::mutations::apply_raster_mutation;
use crate::{RasterMutation, RasterSnapshot};
use crate::mutations::fill_region::{FillRegion as FillRegionLeaf, RasterSeed};
use semio_framework::kernel::{HistoryEntry, HistoryTimeTravel, HistoryTimeTravelStage};
use semio_framework_plugin::PluginApp;

pub(super) fn retire(snapshot: RasterSnapshot) {
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(snapshot);
}

/// ⏪️ One `historyEdit*` verb from the composite window; a refused verb fails the law.
async fn history_edit(app: &mut RasterAppFixture, verb: &str, args: serde_json::Value) {
    let mut meta = semio_framework_plugin::artifact_app_laws::meta("local");
    meta.view_state = Some(raster_view_state());
    let result = app.handle_action(verb, Some(&semio_framework_value::DslValue::from(&args)), &meta).await.unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));
    assert!(result.output.get("rejected").is_none(), "{verb} was refused: {:?}", result.output);
}

async fn time_travel(app: &mut RasterAppFixture) -> Option<HistoryTimeTravel> {
    app.history_snapshot().await.expect("history").time_travel
}

/// ⏳️ Drives the session's replay job until `done` holds for its stage (absent = time travel closed).
async fn pump(app: &mut RasterAppFixture, done: impl Fn(Option<HistoryTimeTravelStage>) -> bool) {
    for _ in 0..10_000 {
        if done(time_travel(app).await.map(|status| status.stage)) {
            return;
        }
        app.advance_typed_operation_publication().await.expect("a driver turn");
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    panic!("the history edit never settled: {:?}", time_travel(app).await);
}

/// 🧾️ The history rows that carry a document mutation, oldest first.
pub(super) async fn document_rows(app: &mut RasterAppFixture) -> Vec<HistoryEntry> {
    let mut rows: Vec<_> = app.history_snapshot().await.expect("history").upserts.into_iter().filter(|entry| entry.edit_id.is_some() && !entry.mutations.is_empty()).collect();
    rows.sort_by_key(|entry| entry.seq);
    rows
}

/// ➕️ A fresh blank pixel layer on the boot document, and its id.
pub(super) async fn blank_layer(app: &mut RasterAppFixture) -> String {
    let before = app.snapshot().expect("boot head");
    let known: Vec<String> = crate::standards::v1::subsets::any::schema::flatten_raster_layers(&before.layers).iter().map(|layer| crate::standards::v1::subsets::any::schema::layer_node_id(layer).to_string()).collect();
    retire(before);
    dispatch(app, RasterCommand::AddLayer(crate::editor::raster::commands::add_layer::AddLayer { kind: "pixel".into() })).await;
    let after = app.snapshot().expect("head with the new layer");
    let id = crate::standards::v1::subsets::any::schema::flatten_raster_layers(&after.layers).iter().map(|layer| crate::standards::v1::subsets::any::schema::layer_node_id(layer).to_string()).find(|id| !known.contains(id)).expect("the added layer");
    retire(after);
    id
}

#[semio_framework_async_macros::async_test]
async fn a_bucket_click_is_one_transaction_whose_seed_tolerance_and_colour_time_travel_edits_replay_exactly() {
    let mut app = app().await;
    let layer = blank_layer(&mut app).await;
    let base = app.snapshot().expect("the base the fill lands on");
    let rows_before = document_rows(&mut app).await.len();
    dispatch(&mut app, RasterCommand::SetFillTolerance(crate::editor::raster::commands::set_fill_tolerance::SetFillTolerance { value: 7 })).await;
    assert_eq!(document_rows(&mut app).await.len(), rows_before, "the session tolerance is never a history row");
    dispatch(&mut app, RasterCommand::FillRegion(fill_region::FillRegion { layer_id: layer.clone(), x: 1.25, y: 2.75 })).await;
    let rows = document_rows(&mut app).await;
    assert_eq!(rows.len(), rows_before + 1, "one click, one row");
    let row = rows.last().expect("the bucket row");
    assert_eq!(row.transaction.as_ref().map(|transaction| transaction.tool.as_str()), Some(fill_region::RASTER_FILL_TOOL_ID), "the row is the bucket's tool transaction");
    assert_eq!(row.mutations.len(), 1);
    assert!(row.mutations[0].editable, "the fill-region leaf is editable");
    let fill = row.mutations[0].mutation_id.clone();
    let committed = app.snapshot().expect("committed head");
    let clicked = RasterMutation::FillRegion(FillRegionLeaf { layer_id: layer.clone(), target: "pixels".into(), seed: RasterSeed { x: 1, y: 2 }, tolerance: 7, color: vec![40.0 / 255.0, 120.0 / 255.0, 220.0 / 255.0, 1.0], selection: None });
    let clicked_head = apply_raster_mutation(&base, &clicked).expect("the clicked fill applies on its base");
    assert_eq!(committed, clicked_head, "the click is the session colour and tolerance flooded from the clicked pixel");

    history_edit(&mut app, "historyEditBegin", serde_json::json!({ "mutationId": fill })).await;
    history_edit(&mut app, "historyEditInput", serde_json::json!({ "path": "/seed/x", "value": 3 })).await;
    history_edit(&mut app, "historyEditInput", serde_json::json!({ "path": "/tolerance", "value": 40 })).await;
    history_edit(&mut app, "historyEditInput", serde_json::json!({ "path": "/color", "value": [0.0, 1.0, 0.0, 1.0] })).await;
    history_edit(&mut app, "historyEditAccept", serde_json::json!({})).await;
    pump(&mut app, |stage| stage != Some(HistoryTimeTravelStage::Replaying)).await;
    let status = time_travel(&mut app).await.expect("a live session");
    assert_eq!((status.stage, status.blocking), (HistoryTimeTravelStage::Reviewing, false), "a clean replay reviews: {status:?}");
    let reviewed = app.snapshot().expect("committed head while reviewing");
    assert_eq!(reviewed, committed, "reviewing never touches the committed document");
    history_edit(&mut app, "historyEditFinalize", serde_json::json!({})).await;
    history_edit(&mut app, "historyEditCommit", serde_json::json!({ "choice": "overwrite" })).await;
    pump(&mut app, |stage| stage.is_none()).await;

    let edited = RasterMutation::FillRegion(FillRegionLeaf { layer_id: layer.clone(), target: "pixels".into(), seed: RasterSeed { x: 3, y: 2 }, tolerance: 40, color: vec![0.0, 1.0, 0.0, 1.0], selection: None });
    let fresh = apply_raster_mutation(&base, &edited).expect("the edited fill applies on its base");
    let head = app.snapshot().expect("edited head");
    assert_eq!(head, fresh, "the overwritten head is the fresh fold of the edited log: the region re-flooded and re-filled");
    let ids: Vec<_> = document_rows(&mut app).await[rows_before..].iter().map(|row| row.mutations[0].mutation_id.clone()).collect();
    assert_eq!(ids, vec![fill], "editing never re-mints the gesture's mutation or row");
    for snapshot in [base, committed, clicked_head, reviewed, fresh, head] {
        retire(snapshot);
    }
}
