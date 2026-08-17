//! 🗑️️ SpaceIndexEditor commands command — `delete-artifact`.

use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::{delete_artifact, SSpaceMutation};
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

/// 🗑️️ The real, immediate delete — never dispatched directly from a row's "Delete" affordance
/// (that dispatches `❔request-delete-artifact` first, which opens the `deleteArtifact` confirm
/// dialog; the dialog's own submit re-dispatches THIS command, worker-brief task 2). Kept as its own
/// undecorated command so a caller that already confirmed out-of-band (tests, scripted flows) is
/// never forced through the dialog.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "delete-artifact")]
pub struct DeleteArtifact {
    pub id: String,
}

pub fn handle(payload: &DeleteArtifact, _doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![delete_artifact(payload.id.clone())]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::space_index::commands::create_artifact;
    use crate::editor::space_index::{testkit, SpaceIndexCommand};
    

    #[test]
    fn delete_artifact_removes_the_row() {
        let mut app = testkit::new_app();
        app.dispatch_typed(SpaceIndexCommand::CreateArtifact(create_artifact::CreateArtifact { name: "First".into(), kind_id: "draw".into(), now_ms: 1, actor: "user:1".into() }), &semio_framework_plugin::testkit::meta("local")).expect("create artifact");
        let id = app.snapshot().unwrap().artifacts[0].id.clone();
        app.dispatch_typed(SpaceIndexCommand::DeleteArtifact(DeleteArtifact { id: id.clone() }), &semio_framework_plugin::testkit::meta("local")).expect("delete artifact");
        let snapshot = app.snapshot().expect("projection");
        assert!(snapshot.artifacts.iter().all(|row| row.id != id));
    }
}
//#endregion 🧪️Tests
