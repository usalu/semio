use super::*;
use crate::editor_domain::editor_laws::context::{app, dispatch};
use semio_s_artifact_procedural_generation3d::editor::generation3d::Generation3dCommand;
use crate::editor_domain::editor_laws::context;

#[semio_framework_async_macros::async_test]
async fn toggle_sun_never_mutates_the_document() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app().await;
    let before = context::snapshot(&app);
    dispatch(&mut app, Generation3dCommand::ToggleSun(ToggleSun {})).await;
    assert_eq!(context::snapshot(&app), before, "toggleSun must not mutate the document");
}
