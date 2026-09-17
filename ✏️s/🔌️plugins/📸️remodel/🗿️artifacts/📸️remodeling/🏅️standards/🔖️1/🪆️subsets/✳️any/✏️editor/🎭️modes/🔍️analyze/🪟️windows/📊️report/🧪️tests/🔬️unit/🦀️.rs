use super::*;
use crate::default_remodeling_scene;
use crate::editor::remodeling::commands::set_report_table::SetReportTable;
use crate::editor::remodeling::unit_tests::context::{app, dispatch, render_in_window};
use crate::editor::remodeling::RemodelingCommand;

#[semio_framework_async_macros::async_test]
async fn every_dataset_name_yields_its_own_column_set_and_unknown_falls_back_to_frames() {
    let scene = default_remodeling_scene();
    for (table, marker) in [("cameras", "RMS (px)"), ("tracks", "Mean Speed (m/s)"), ("gcps", "Observations"), ("qcStages", "Value"), ("matches", "Note"), ("nonsense", "Timestamp (ms)")] {
        let (columns, _rows) = report_table_json(&scene, table);
        assert!(columns.contains(marker), "table {table} must expose {marker}: {columns}");
    }
}

#[semio_framework_async_macros::async_test]
async fn switching_the_selected_table_changes_the_rendered_columns() {
    let mut app = app().await;
    dispatch(&mut app, RemodelingCommand::SetReportTable(SetReportTable { table: "gcps".into() })).await;
    // 📊️ The table lives in the surface's owned document bytes, not in a projected label — decode the scene.
    let scene: semio_framework_plugin::TableScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene(&render_in_window(&mut app, REMODELING_PLAY_BODY_REPORT, REMODELING_PLAY_WINDOW_REPORT).await).expect("the report window renders a table scene");
    assert!(scene.columns_json.contains("Observations"), "the gcps table carries its Observations column: {}", scene.columns_json);
}
