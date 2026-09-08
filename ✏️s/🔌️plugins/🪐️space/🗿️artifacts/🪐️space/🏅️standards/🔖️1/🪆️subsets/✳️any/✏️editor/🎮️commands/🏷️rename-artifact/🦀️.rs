//! 🏷️️ SpaceIndexEditor commands command — `rename-artifact`.

use crate::standards::v1::subsets::any::schema::mutations::{rename_artifact, SSpaceMutation};
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "rename-artifact")]
pub struct RenameArtifact {
    pub id: String,
    pub new_name: String,
}

pub fn handle(payload: &RenameArtifact, _doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![rename_artifact(payload.id.clone(), payload.new_name.clone())]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
