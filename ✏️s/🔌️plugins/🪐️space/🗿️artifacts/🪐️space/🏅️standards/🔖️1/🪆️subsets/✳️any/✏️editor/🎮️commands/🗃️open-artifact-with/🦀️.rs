//! 🗃️️ SpaceIndexEditor commands command — `open-artifact-with`. The explicit "Open with…" chooser
//! variant of `📂open-artifact`: `role`/`plugin_id`/`app_id` are the user's explicit picks from the
//! chooser dialog (worker-brief task 2), always sent, never defaulted by the shell.

use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
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

pub fn handle(payload: &OpenArtifactWith, doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    let row = doc.snapshot.artifacts.iter().find(|row| row.id == payload.id).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("s.space.mutation.target-missing"), format!("artifact `{}` not found", payload.id)))?;
    let artifact_ref = format!("{}@{}/{}", row.dialect.artifact_kind, row.dialect.standard, row.dialect.subset);
    Ok(Emit::effect(Effect::ReplayShellCommand {
        action_id: "os.open-artifact-with".into(),
        args: Some(pack::json_to_dsl_value(&pack::json!({ "artifactRef": artifact_ref, "documentId": row.id.clone(), "spaceId": doc.snapshot.space_id.clone(), "schema": row.schema.clone(), "role": payload.role.clone(), "pluginId": payload.plugin_id.clone(), "appId": payload.app_id.clone() }))),
    }))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::space_index::commands::create_artifact;
    use crate::editor::space_index::{testkit, SpaceIndexCommand};

    #[semio_framework_async_macros::async_test]
    async fn open_artifact_with_relays_the_explicit_choice() {
        let mut app = testkit::new_app();
        app.dispatch_typed(SpaceIndexCommand::CreateArtifact(create_artifact::CreateArtifact { name: "First".into(), kind_id: "draw".into(), now_ms: 1, actor: "user:1".into() }), &semio_framework_plugin::testkit::meta("local"))
            .expect("create artifact");
        let id = app.snapshot().unwrap().artifacts[0].id.clone();
        let result = app
            .dispatch_typed(SpaceIndexCommand::OpenArtifactWith(OpenArtifactWith { id: id.clone(), role: "viewer".into(), plugin_id: "draw".into(), app_id: "draw-play".into() }), &semio_framework_plugin::testkit::meta("local"))
            .expect("open with");
        assert_eq!(result.requested_effects.len(), 1);
        match &result.requested_effects[0] {
            Effect::ReplayShellCommand { action_id, args } => {
                assert_eq!(action_id, "os.open-artifact-with");
                let args = pack::json_from_dsl_value(&args.clone().unwrap());
                assert_eq!(args.get("role").and_then(|v| v.as_str()), Some("viewer"));
                assert_eq!(args.get("pluginId").and_then(|v| v.as_str()), Some("draw"));
                assert_eq!(args.get("appId").and_then(|v| v.as_str()), Some("draw-play"));
                assert_eq!(args.get("schema").and_then(|v| v.as_str()), Some("s.draw.draw"));
            }
            other => panic!("expected ReplayShellCommand, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
