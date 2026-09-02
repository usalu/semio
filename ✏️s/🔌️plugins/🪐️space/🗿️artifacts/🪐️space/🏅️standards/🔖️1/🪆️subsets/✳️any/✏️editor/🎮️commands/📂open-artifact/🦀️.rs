//! 📂️ SpaceIndexEditor commands command — `open-artifact`. Relays `os.open-artifact` with no
//! explicit `role`, so the shell resolves the user's `OpeningPreferences` default (contract §C3/C6,
//! worker-brief task 2: "respects the user's `OpeningPreferences` default" — the space app has no
//! access to that host-side config facet, so it deliberately omits `role` rather than guessing one;
//! see `👁️set-visibility`-sibling command `🗃️open-artifact-with` for the explicit-role "Open with…"
//! chooser).

use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "open-artifact")]
pub struct OpenArtifact {
    pub id: String,
}

pub fn handle(payload: &OpenArtifact, doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    let row = doc.snapshot.artifacts.iter().find(|row| row.id == payload.id).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("s.space.mutation.target-missing"), format!("artifact `{}` not found", payload.id)))?;
    let artifact_ref = format!("{}@{}/{}", row.dialect.artifact_kind, row.dialect.standard, row.dialect.subset);
    Ok(Emit::effect(Effect::ReplayShellCommand { action_id: "os.open-artifact".into(), args: Some(pack::json_to_dsl_value(&pack::json!({ "artifactRef": artifact_ref, "documentId": row.id.clone(), "spaceId": doc.snapshot.space_id.clone() }))) }))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::space_index::commands::create_artifact;
    use crate::editor::space_index::{testkit, SpaceIndexCommand};

    #[semio_framework_async_macros::async_test]
    async fn open_artifact_relays_with_document_and_space_ids() {
        let mut app = testkit::new_app();
        app.dispatch_typed(SpaceIndexCommand::CreateArtifact(create_artifact::CreateArtifact { name: "First".into(), kind_id: "draw".into(), now_ms: 1, actor: "user:1".into() }), &semio_framework_plugin::testkit::meta("local"))
            .expect("create artifact");
        let id = app.snapshot().unwrap().artifacts[0].id.clone();
        let result = app.dispatch_typed(SpaceIndexCommand::OpenArtifact(OpenArtifact { id: id.clone() }), &semio_framework_plugin::testkit::meta("local")).expect("open artifact");
        assert!(result.mutations.is_empty());
        assert_eq!(result.requested_effects.len(), 1);
        match &result.requested_effects[0] {
            Effect::ReplayShellCommand { action_id, args } => {
                assert_eq!(action_id, "os.open-artifact");
                let args = pack::json_from_dsl_value(&args.clone().unwrap());
                assert_eq!(args.get("documentId").and_then(|v| v.as_str()), Some(id.as_str()));
                assert!(args.get("role").is_none(), "role is omitted so the shell resolves OpeningPreferences");
            }
            other => panic!("expected ReplayShellCommand, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn open_artifact_of_a_missing_row_faults() {
        let mut app = testkit::new_app();
        let error = app.dispatch_typed(SpaceIndexCommand::OpenArtifact(OpenArtifact { id: "ghost".into() }), &semio_framework_plugin::testkit::meta("local")).expect_err("missing row must fault");
        assert_eq!(error.code.0, "s.space.mutation.target-missing");
    }
}
//#endregion 🧪️Tests
