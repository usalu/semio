use super::*;
use crate::default_remodeling_scene;
use crate::editor::remodeling::commands::set_report_table::SetReportTable;
use crate::editor::remodeling::testkit::{app, dispatch, render as render_body};
use crate::editor::remodeling::RemodelingCommand;

#[semio_framework_async_macros::async_test]
async fn every_dataset_name_yields_its_own_column_set_and_unknown_falls_back_to_frames() {
    let scene = default_remodeling_scene();
    for (table, marker) in [("cameras", "RMS (px)"), ("tracks", "Mean Speed (m/s)"), ("gcps", "Observations"), ("qcStages", "Status"), ("matches", "Note"), ("nonsense", "Timestamp (ms)")] {
        let (columns, _rows) = report_table_json(&scene, table);
        assert!(columns.contains(marker), "table {table} must expose {marker}: {columns}");
    }
}

#[semio_framework_async_macros::async_test]
async fn switching_the_selected_table_changes_the_rendered_columns() {
    let mut app = app().await;
    dispatch(&mut app, RemodelingCommand::SetReportTable(SetReportTable { table: "gcps".into() })).await;
    assert!(render_body(&mut app, REMODELING_PLAY_BODY_REPORT).await.contains("Observations"));
}
