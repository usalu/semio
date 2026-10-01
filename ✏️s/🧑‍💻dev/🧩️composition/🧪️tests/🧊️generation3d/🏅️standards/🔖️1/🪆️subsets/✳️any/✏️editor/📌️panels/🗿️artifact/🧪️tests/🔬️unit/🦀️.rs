use super::*;
use crate::editor_domain::editor_laws::context;
use crate::editor_domain::editor_laws::context::{app_with_registry, render as render_body};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::{dag_host_snapshot_to_workflow, with_host};
use semio_framework_plugin::{TreeWindowRequest, TreeWindows, ViewModel};
use semio_framework_ui::wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};

const DOCUMENT_ROWS_LAW: &str = include_str!("../../../../../../../../../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧫️fixtures/🔬️unit/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn document_lists_widgets() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;
    let rendered = render_body(&mut app, GENERATION_3D_PLAY_BODY_ARTIFACT).await;
    let fixture_widgets: Vec<String> = context::snapshot(&app).host_snapshot.widgets.iter().map(|widget| semio_s_artifact_procedural_generation3d::widget_id(widget).to_string()).collect();
    let first = fixture_widgets.first().expect("default fixture has at least one widget");
    assert!(rendered.contains(first), "document tree missing widget id {first}: {rendered}");
}

//#region 🪟️WindowLaws
//#endregion 🪟️WindowLaws
