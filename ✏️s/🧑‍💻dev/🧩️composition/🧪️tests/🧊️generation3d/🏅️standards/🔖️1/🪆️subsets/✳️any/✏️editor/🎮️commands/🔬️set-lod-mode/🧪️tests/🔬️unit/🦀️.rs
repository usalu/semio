use super::*;
use crate::editor_domain::editor_laws::context::{app, dispatch};
use semio_s_artifact_procedural_generation3d::editor::generation3d::Generation3dCommand;
use crate::editor_domain::editor_laws::context;

#[semio_framework_async_macros::async_test]
async fn set_lod_mode_is_a_view_action_with_no_artifact_mutations() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app().await;
    let before = context::snapshot(&app);
    dispatch(&mut app, Generation3dCommand::SetLodMode(SetLodMode { value: "wireframe".into() })).await;
    assert_eq!(context::snapshot(&app), before, "setLodMode must not mutate the document");
}
