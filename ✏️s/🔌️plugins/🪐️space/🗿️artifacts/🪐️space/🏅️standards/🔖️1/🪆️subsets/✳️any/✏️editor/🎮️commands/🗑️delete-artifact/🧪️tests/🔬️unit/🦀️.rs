
use super::*;
use crate::editor::space_index::{SpaceIndexCommand, unit_tests::context};

#[semio_framework_async_macros::async_test]
async fn delete_artifact_removes_the_row() {
    let (mut app, id) = context::new_app_with_artifact().await;
    app.dispatch_typed(SpaceIndexCommand::DeleteArtifact(DeleteArtifact { id: id.clone() }), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("delete artifact");
    let snapshot = app.snapshot().expect("projection");
    assert!(snapshot.artifacts.iter().all(|row| row.id != id));
}
