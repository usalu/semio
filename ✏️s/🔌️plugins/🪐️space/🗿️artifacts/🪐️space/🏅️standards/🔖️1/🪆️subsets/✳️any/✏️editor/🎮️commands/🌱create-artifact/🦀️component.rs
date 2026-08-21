//! 🌱️ SpaceIndexEditor commands command — `create-artifact`. Mints the new artifact's document id
//! itself (contract §C4/worker-brief task 2), then relays `os.open-artifact` so the new artifact
//! opens immediately in its editor (`Effect::ReplayShellCommand`, contract §C6's opening-relay
//! convention — the shell's own `os.open-artifact` handler, see `📓️w2-c-report.md`).

use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::{create_artifact, SSpaceMutation};
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::{mint_artifact_id, SSpaceSnapshot, SpaceArtifactDialect, SpaceArtifactRow};
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use crate::editor::space_index::known_artifact_kind;
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "create-artifact")]
pub struct CreateArtifact {
    pub name: String,
    pub kind_id: String,
    pub now_ms: u64,
    pub actor: String,
}

/// 🐙️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 4-F: mirrors Home's
/// `createSpace` handler (`🏠️home/…/🎮️commands/🌱create-space/🦀️component.rs`) — a raw toolbar-button
/// click (`#s-space-create-artifact`, contract §C0) dispatches with no args at all, and this must open
/// the already-declared `createArtifact` dialog instead of failing on an unknown empty `kind_id`.
pub async fn handle(payload: &CreateArtifact, doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    if payload.name.trim().is_empty() || payload.kind_id.trim().is_empty() {
        return Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(130), dialog_id: "createArtifact".into(), args: None }));
    }
    let known = known_artifact_kind(&payload.kind_id).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("s.space.unknown-kind"), format!("unknown artifact kind `{}`", payload.kind_id)))?;
    let id = mint_artifact_id(&doc.snapshot.artifacts, payload.now_ms);
    let row = SpaceArtifactRow {
        id: id.clone(),
        name: payload.name.clone(),
        kind_id: known.dialect_artifact_kind.into(),
        schema: known.schema.into(),
        dialect: SpaceArtifactDialect { artifact_kind: known.dialect_artifact_kind.into(), standard: known.standard.into(), subset: known.subset.into() },
        created_at_ms: payload.now_ms,
        created_by: payload.actor.clone(),
        updated_at_ms: payload.now_ms,
        updated_by: payload.actor.clone(),
    };
    let artifact_ref = format!("{}@{}/{}", known.dialect_artifact_kind, known.standard, known.subset);
    let relay = Effect::ReplayShellCommand { action_id: "os.open-artifact".into(), args: semio_framework::optional_json_to_dsl(Some(json!({ "artifactRef": artifact_ref, "role": "editor", "documentId": id, "spaceId": doc.snapshot.space_id }))) };
    Ok(Emit { artifact_mutations: vec![create_artifact(row)], effects: vec![relay], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::space_index::{testkit, SpaceIndexCommand};

    #[semio_framework_async_macros::async_test]
    async fn create_artifact_mints_an_id_adds_a_row_and_relays_the_open_command() {
        let mut app = testkit::new_app();
        let result = app.dispatch_typed(SpaceIndexCommand::CreateArtifact(CreateArtifact { name: "First".into(), kind_id: "draw".into(), now_ms: 1, actor: "user:1".into() }), &semio_framework_plugin::testkit::meta("local")).expect("create artifact");
        let snapshot = app.snapshot().expect("projection");
        assert_eq!(snapshot.artifacts.len(), 1);
        let row = &snapshot.artifacts[0];
        assert!(!row.id.is_empty());
        assert_eq!(row.name, "First");
        assert_eq!(row.dialect.artifact_kind, "s.draw.draw");
        assert_eq!(result.requested_effects.len(), 1, "creating an artifact relays exactly one open command");
        match &result.requested_effects[0] {
            Effect::ReplayShellCommand { action_id, args } => {
                assert_eq!(action_id, "os.open-artifact");
                let args = semio_framework_os_kernel::pack_rt::dsl_value_to_json(args.clone().expect("args"));
                assert_eq!(args.get("documentId").and_then(|v| v.as_str()), Some(row.id.as_str()));
                assert_eq!(args.get("spaceId").and_then(|v| v.as_str()), Some(""));
            }
            other => panic!("expected ReplayShellCommand, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_name_and_kind_open_the_dialog_instead_of_failing() {
        let mut app = testkit::new_app();
        let result = app
            .dispatch_typed(SpaceIndexCommand::CreateArtifact(CreateArtifact { name: String::new(), kind_id: String::new(), now_ms: 0, actor: String::new() }), &semio_framework_plugin::testkit::meta("local"))
            .expect("empty args must open the dialog, not fail");
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

    #[semio_framework_async_macros::async_test]
    async fn create_artifact_rejects_an_unknown_kind() {
        let mut app = testkit::new_app();
        let error = app
            .dispatch_typed(SpaceIndexCommand::CreateArtifact(CreateArtifact { name: "First".into(), kind_id: "nope".into(), now_ms: 1, actor: "user:1".into() }), &semio_framework_plugin::testkit::meta("local"))
            .expect_err("unknown kind must fail");
        assert_eq!(error.code.0, "s.space.unknown-kind");
    }

    #[semio_framework_async_macros::async_test]
    async fn create_artifact_mints_distinct_ids_for_two_rows_created_at_the_same_instant() {
        let mut app = testkit::new_app();
        app.dispatch_typed(SpaceIndexCommand::CreateArtifact(CreateArtifact { name: "A".into(), kind_id: "draw".into(), now_ms: 5, actor: "user:1".into() }), &semio_framework_plugin::testkit::meta("local")).expect("create a");
        app.dispatch_typed(SpaceIndexCommand::CreateArtifact(CreateArtifact { name: "B".into(), kind_id: "draw".into(), now_ms: 5, actor: "user:1".into() }), &semio_framework_plugin::testkit::meta("local")).expect("create b");
        let snapshot = app.snapshot().expect("projection");
        assert_eq!(snapshot.artifacts.len(), 2);
        assert_ne!(snapshot.artifacts[0].id, snapshot.artifacts[1].id);
    }
}
//#endregion 🧪️Tests
