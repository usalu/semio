//! 🗃️️ SpaceIndexEditor commands command — `open-artifact-with`. The explicit "Open with…" chooser
//! variant of `🗿️open-artifact`: `role`/`plugin_id`/`app_id` are the user's explicit picks from the
//! chooser dialog (worker-brief task 2), always sent, never defaulted by the shell.

use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "open-artifact-with")]
pub struct OpenArtifactWith {
    pub id: String,
    pub role: String,
    pub plugin_id: String,
    pub app_id: String,
}

pub fn handle(payload: &OpenArtifactWith, doc: &ArtifactView<'_, SSpaceSnapshot>, cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    let row = cfg.snapshot.indexed_artifacts.iter().find(|row| row.id == payload.id).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("s.space.index.target-missing"), format!("indexed artifact `{}` not found", payload.id)))?;
    let artifact_ref = format!("{}@{}/{}", row.dialect.artifact_kind, row.dialect.standard, row.dialect.subset);
    Ok(Emit::effect(Effect::ReplayShellCommand {
        action_id: "os.open-artifact-with".into(),
        args: Some(pack::json_to_dsl_value(
            &pack::json!({ "artifactRef": artifact_ref, "documentId": row.id.clone(), "spaceId": doc.snapshot.space_id.clone(), "schema": row.schema.clone(), "role": payload.role.clone(), "pluginId": payload.plugin_id.clone(), "appId": payload.app_id.clone() }),
        )),
    }))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
