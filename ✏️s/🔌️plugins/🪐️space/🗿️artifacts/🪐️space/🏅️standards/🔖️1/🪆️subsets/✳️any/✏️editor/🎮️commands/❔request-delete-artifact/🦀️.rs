//! ❔️ SpaceIndexEditor commands command — `request-delete-artifact`. View-only: opens the
//! `deleteArtifact` confirm dialog (worker-brief task 2, "delete-artifact (confirm dialog first)")
//! pre-seeded with the target `id`; the dialog's own submit re-dispatches the real, undecorated
//! `🗑️delete-artifact` command (unchanged — see that file's own doc comment).

use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "request-delete-artifact")]
pub struct RequestDeleteArtifact {
    pub id: String,
}

pub fn handle(payload: &RequestDeleteArtifact, doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    let row = doc.snapshot.artifacts.iter().find(|row| row.id == payload.id).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("s.space.mutation.target-missing"), format!("artifact `{}` not found", payload.id)))?;
    Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(128), dialog_id: "deleteArtifact".into(), args: Some(pack::json_to_dsl_value(&pack::json!({ "id": row.id.clone(), "name": row.name.clone() }))) }))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::space_index::commands::create_artifact;
    use crate::editor::space_index::{testkit, SpaceIndexCommand};

    #[semio_framework_async_macros::async_test]
    async fn request_delete_opens_the_confirm_dialog_without_mutating() {
        let mut app = testkit::new_app();
        app.dispatch_typed(SpaceIndexCommand::CreateArtifact(create_artifact::CreateArtifact { name: "First".into(), kind_id: "draw".into(), now_ms: 1, actor: "user:1".into() }), &semio_framework_plugin::testkit::meta("local"))
            .expect("create artifact");
        let id = app.snapshot().unwrap().artifacts[0].id.clone();
        let result = app.dispatch_typed(SpaceIndexCommand::RequestDeleteArtifact(RequestDeleteArtifact { id: id.clone() }), &semio_framework_plugin::testkit::meta("local")).expect("request delete");
        assert!(result.mutations.is_empty(), "requesting delete never mutates the document directly");
        assert_eq!(app.snapshot().unwrap().artifacts.len(), 1, "the row survives until the dialog is confirmed");
        assert_eq!(result.requested_effects.len(), 1);
        match &result.requested_effects[0] {
            Effect::OpenDialog { dialog_id, args, .. } => {
                assert_eq!(dialog_id, "deleteArtifact");
                let args = pack::json_from_dsl_value(&args.clone().unwrap());
                assert_eq!(args.get("id").and_then(|v| v.as_str()), Some(id.as_str()));
            }
            other => panic!("expected OpenDialog, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn request_delete_of_a_missing_row_faults() {
        let mut app = testkit::new_app();
        let error = app.dispatch_typed(SpaceIndexCommand::RequestDeleteArtifact(RequestDeleteArtifact { id: "ghost".into() }), &semio_framework_plugin::testkit::meta("local")).expect_err("missing row must fault");
        assert_eq!(error.code.0, "s.space.mutation.target-missing");
    }
}
//#endregion 🧪️Tests
