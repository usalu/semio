
use super::*;
use crate::editor::space_index::{SpaceIndexCommand, unit_tests::context};

#[semio_framework_async_macros::async_test]
async fn rename_artifact_updates_the_name() {
    let (mut app, id) = context::new_app_with_artifact().await;
    app.dispatch_typed(SpaceIndexCommand::RenameArtifact(RenameArtifact { id: id.clone(), new_name: "Renamed".into() }), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("rename artifact");
    let snapshot = app.snapshot().expect("projection");
    assert_eq!(snapshot.artifacts.iter().find(|row| row.id == id).map(|row| row.name.clone()), Some("Renamed".into()));
}
