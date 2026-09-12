
use super::*;
use crate::editor::space_index::{SpaceIndexCommand, unit_tests::context};

#[semio_framework_async_macros::async_test]
async fn touch_artifact_stamps_the_row() {
    let (mut app, id) = context::new_app_with_artifact().await;
    app.dispatch_typed(SpaceIndexCommand::TouchArtifact(TouchArtifact { id: id.clone(), now_ms: 99, actor: "user:2".into() }), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("touch artifact");
    let snapshot = app.snapshot().expect("projection");
    let row = snapshot.artifacts.iter().find(|row| row.id == id).expect("row");
    assert_eq!(row.updated_at_ms, 99);
    assert_eq!(row.updated_by, "user:2");
}
