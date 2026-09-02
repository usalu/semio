//! 🕒️ SpaceIndexEditor commands command — `touch-artifact`. Dispatched by the shell's own
//! post-checkpoint hook (contract §C5 "After every checkpoint the shell dispatches `TouchArtifact` to
//! the space index") — a real user-visible action too, so it is declared through the same typed
//! command channel every other row uses, not a bespoke side door.

use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::{touch_artifact, SSpaceMutation};
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "touch-artifact")]
pub struct TouchArtifact {
    pub id: String,
    pub now_ms: u64,
    pub actor: String,
}

pub fn handle(payload: &TouchArtifact, _doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![touch_artifact(payload.id.clone(), payload.now_ms, payload.actor.clone())]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::space_index::commands::create_artifact;
    use crate::editor::space_index::{testkit, SpaceIndexCommand};

    #[semio_framework_async_macros::async_test]
    async fn touch_artifact_stamps_the_row() {
        let mut app = testkit::new_app();
        app.dispatch_typed(SpaceIndexCommand::CreateArtifact(create_artifact::CreateArtifact { name: "First".into(), kind_id: "draw".into(), now_ms: 1, actor: "user:1".into() }), &semio_framework_plugin::testkit::meta("local"))
            .expect("create artifact");
        let id = app.snapshot().unwrap().artifacts[0].id.clone();
        app.dispatch_typed(SpaceIndexCommand::TouchArtifact(TouchArtifact { id: id.clone(), now_ms: 99, actor: "user:2".into() }), &semio_framework_plugin::testkit::meta("local")).expect("touch artifact");
        let snapshot = app.snapshot().expect("projection");
        let row = snapshot.artifacts.iter().find(|row| row.id == id).expect("row");
        assert_eq!(row.updated_at_ms, 99);
        assert_eq!(row.updated_by, "user:2");
    }
}
//#endregion 🧪️Tests
