
use super::*;
use crate::editor::remodeling::RemodelingCommand;
use crate::editor::remodeling::commands::{set_frame_cursor, set_layer_visibility, set_report_table};
use crate::editor::remodeling::testkit::{app, dispatch};

/// 🕹️ Relocated from the deleted `set-selection` command file (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM): remodeling's selection now lives in the
/// framework-owned "assets" interaction domain, not app config — this asserts the surviving
/// View-kind commands still emit config-only mutations.
#[semio_framework_async_macros::async_test]
async fn view_actions_emit_config_mutations_not_artifact_mutations() {
    let mut app = app().await;
    let result = dispatch(&mut app, RemodelingCommand::SetCamera(SetCamera { camera: RemodelingWorldCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 60.0 } })).await;
    assert!(result.mutations.is_empty());
    let result = dispatch(&mut app, RemodelingCommand::SetLayerVisibility(set_layer_visibility::SetLayerVisibility { layer: "dense".into(), visible: false })).await;
    assert!(result.mutations.is_empty());
    let result = dispatch(&mut app, RemodelingCommand::SetFrameCursor(set_frame_cursor::SetFrameCursor { stream_id: Some("stream-1".into()), frame_index: 2 })).await;
    assert!(result.mutations.is_empty());
    let result = dispatch(&mut app, RemodelingCommand::SetReportTable(set_report_table::SetReportTable { table: "gcps".into() })).await;
    assert!(result.mutations.is_empty());
}
