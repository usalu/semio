//! 🏷️️ SpaceIndexEditor commands command — `rename-artifact`.

use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::{rename_artifact, SSpaceMutation};
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "rename-artifact")]
pub struct RenameArtifact {
    pub id: String,
    pub new_name: String,
}

pub async fn handle(payload: &RenameArtifact, _doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![rename_artifact(payload.id.clone(), payload.new_name.clone())]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::space_index::commands::create_artifact;
    use crate::editor::space_index::{testkit, SpaceIndexCommand};

    #[semio_framework_async_macros::async_test]
    async fn rename_artifact_updates_the_name() {
        let mut app = testkit::new_app();
        app.dispatch_typed(SpaceIndexCommand::CreateArtifact(create_artifact::CreateArtifact { name: "First".into(), kind_id: "draw".into(), now_ms: 1, actor: "user:1".into() }), &semio_framework_plugin::testkit::meta("local"))
            .expect("create artifact");
        let id = app.snapshot().unwrap().artifacts[0].id.clone();
        app.dispatch_typed(SpaceIndexCommand::RenameArtifact(RenameArtifact { id: id.clone(), new_name: "Renamed".into() }), &semio_framework_plugin::testkit::meta("local")).expect("rename artifact");
        let snapshot = app.snapshot().expect("projection");
        assert_eq!(snapshot.artifacts.iter().find(|row| row.id == id).map(|row| row.name.clone()), Some("Renamed".into()));
    }
}
//#endregion 🧪️Tests
