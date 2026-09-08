
use super::*;
use crate::editor::space_index::{SpaceIndexCommand, testkit};

#[semio_framework_async_macros::async_test]
async fn delete_artifact_removes_the_row() {
    let (mut app, id) = testkit::new_app_with_artifact().await;
    app.dispatch_typed(SpaceIndexCommand::DeleteArtifact(DeleteArtifact { id: id.clone() }), &semio_framework_plugin::testkit::meta("local")).await.expect("delete artifact");
    let snapshot = app.snapshot().expect("projection");
    assert!(snapshot.artifacts.iter().all(|row| row.id != id));
}
