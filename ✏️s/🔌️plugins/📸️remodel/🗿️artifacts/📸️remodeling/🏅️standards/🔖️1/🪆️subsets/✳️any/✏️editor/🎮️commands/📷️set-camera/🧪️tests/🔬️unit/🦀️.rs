
use super::*;
use crate::editor::remodeling::RemodelingCommand;
use crate::editor::remodeling::commands::{set_active_utility, set_frame_cursor, set_layer_visibility, set_locale, set_report_table};
use crate::editor::remodeling::testkit::{app, dispatch, render};

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

#[semio_framework_async_macros::async_test]
async fn set_active_utility_switches_host_view_state_without_ops_or_history() {
    let mut app = app().await;
    let result = dispatch(&mut app, RemodelingCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "measure".into() })).await;
    assert!(result.mutations.is_empty(), "utility switch is host-owned config state, never a document operation");
}

/// 🗣️ `setLocale` rewrites the config's locale tag, which is what every panel's label resolution
/// reads — asserted through rendered output, since `VcsArtifactApp` exposes no config accessor.
#[semio_framework_async_macros::async_test]
async fn set_locale_switches_the_rendered_label_language() {
    let mut app = app().await;
    assert!(render(&mut app, crate::editor::remodeling::panels::quality::REMODELING_PLAY_BODY_QC).await.contains("No quality report yet"));
    dispatch(&mut app, RemodelingCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() })).await;
    assert!(render(&mut app, crate::editor::remodeling::panels::quality::REMODELING_PLAY_BODY_QC).await.contains("Noch kein"));
}
