//! 🌱️ SpaceIndexEditor command — `create-artifact`. The isolated guest relays only the
//! user's catalog-choice token and name. The host re-resolves the kind, mints the identity, and owns
//! the durable creation saga before any open effect.

use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "create-artifact")]
pub struct CreateArtifact {
    pub name: String,
    pub kind_choice: String,
}

/// 🐙️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 4-F: mirrors Home's
/// `createSpace` handler (`🏠️home/…/🎮️commands/🌱create-space/🦀️.rs`) — a raw toolbar-button
/// click (`#s-space-create-artifact`, contract §C0) dispatches with no args at all, and this must open
/// the already-declared `createArtifact` dialog instead of failing on an unknown empty `kind_id`.
pub fn handle(payload: &CreateArtifact, _doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    if payload.name.trim().is_empty() || payload.kind_choice.trim().is_empty() {
        return Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(130), dialog_id: "createArtifact".into(), args: None }));
    }
    Ok(Emit::effect(Effect::ReplayShellCommand {
        action_id: "os.create-space-artifact".into(),
        args: Some(pack::json_to_dsl_value(&pack::json!({ "kindChoice": payload.kind_choice, "name": payload.name.trim() }))),
    }))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::space_index::{testkit, SpaceIndexCommand};

    #[semio_framework_async_macros::async_test]
    async fn create_artifact_relays_only_the_catalog_choice_and_name_without_local_publication() {
        let mut app = testkit::new_app().await;
        let kind_choice = "{\"kindId\":\"s.gis.gismap\",\"schema\":\"gis.map\"}";
        let result = app.dispatch_typed(SpaceIndexCommand::CreateArtifact(CreateArtifact { name: " First ".into(), kind_choice: kind_choice.into() }), &semio_framework_plugin::testkit::meta("local")).await.expect("create artifact");
        let snapshot = app.snapshot().expect("projection");
        assert!(snapshot.artifacts.is_empty(), "the guest must not mint or publish a document identity");
        assert_eq!(result.requested_effects.len(), 1);
        match &result.requested_effects[0] {
            Effect::ReplayShellCommand { action_id, args } => {
                assert_eq!(action_id, "os.create-space-artifact");
                let args = pack::json_from_dsl_value(&args.clone().expect("args"));
                assert_eq!(args.get("kindChoice").and_then(|v| v.as_str()), Some(kind_choice));
                assert_eq!(args.get("name").and_then(|v| v.as_str()), Some("First"));
                assert_eq!(args.as_object().map(|value| value.len()), Some(2));
            }
            other => panic!("expected ReplayShellCommand, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_name_and_kind_open_the_dialog_instead_of_failing() {
        let mut app = testkit::new_app().await;
        let result = app
            .dispatch_typed(SpaceIndexCommand::CreateArtifact(CreateArtifact { name: String::new(), kind_choice: String::new() }), &semio_framework_plugin::testkit::meta("local"))
            .await.expect("empty args must open the dialog, not fail");
        assert_eq!(result.requested_effects.len(), 1);
        match &result.requested_effects[0] {
            Effect::OpenDialog { dialog_id, args, .. } => {
                assert_eq!(dialog_id, "createArtifact");
                assert!(args.is_none());
            }
            other => panic!("expected OpenDialog, got {other:?}"),
        }
        let snapshot = app.snapshot().expect("projection");
        assert!(snapshot.artifacts.is_empty(), "opening the dialog must not create a row");
    }

}
//#endregion 🧪️Tests
