//! 🏷️ S Home launcher app command — `rename-space`. An empty `name` (a raw row click) opens the
//! declared `renameSpace` dialog pre-seeded with the space's CURRENT name (read from the already-folded
//! directory read model, `HomeConfig::directory`); a non-empty `name` (the dialog's own submit) relays
//! the rename to the hub (contract §C6) — no optimistic local rename.

use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "rename-space")]
pub struct RenameSpace {
    pub space_id: String,
    pub name: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
pub fn handle(payload: &RenameSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    if payload.name.trim().is_empty() {
        let current_name = cfg.snapshot.directory()?.spaces.get(&payload.space_id).map(|space| space.view.name.clone()).unwrap_or_default();
        let args = Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": payload.space_id.clone(), "name": current_name })));
        return Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(125), dialog_id: "renameSpace".into(), args }));
    }
    let args = Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": payload.space_id.clone(), "name": payload.name.clone() })));
    Ok(Emit::effect(Effect::ReplayShellCommand { action_id: "os.directory.rename-space".into(), args }))
}
//#endregion 🔖️Handle

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    async fn dispatch(payload: RenameSpace, config: &HomeConfig) -> Emit<SHomeMutation, HomeConfigMutation> {
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_snapshot = SHomeSnapshot::default();
        let doc = ArtifactView::new(&doc_snapshot, &history);
        let cfg = ConfigView { snapshot: config };
        handle(&payload, &doc, &cfg).expect("handle")
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_name_opens_the_dialog_preseeded_with_the_current_name() {
        let event_json = pack::json!({
            "seq": 1, "id": "evt-1", "hlc": {"physicalMs": 0, "logical": 0}, "actor": {"kind": "user", "id": "u"}, "spaceId": "sp-1",
            "body": {"kind": "space.created", "spaceId": "sp-1", "name": "Old Name", "spaceKind": "atelier", "visibility": "private", "ownerUserId": "u1"},
            "recordedAtMs": 1000
        })
        .to_string();
        let config = protocol::Mutation::diff(&HomeConfigMutation::FoldDirectoryEvent { event_json }, &HomeConfig::default()).diff().clone();
        let emit = dispatch(RenameSpace { space_id: "sp-1".into(), name: String::new() }, &config);
        let (dialog_id, args) = match &emit.effects[0] {
            Effect::OpenDialog { dialog_id, args, .. } => (dialog_id.clone(), args.clone()),
            other => panic!("expected OpenDialog, got {other:?}"),
        };
        assert_eq!(dialog_id, "renameSpace");
        let args_value: pack::JsonValue = pack::json_from_dsl_value(&args.expect("args"));
        assert_eq!(args_value["name"], "Old Name");
    }

    #[semio_framework_async_macros::async_test]
    async fn non_empty_name_relays_the_rename() {
        let emit = dispatch(RenameSpace { space_id: "sp-1".into(), name: "New Name".into() }, &HomeConfig::default());
        let (action_id, args) = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                Effect::ReplayShellCommand { action_id, args } => Some((action_id.clone(), args.clone())),
                _ => None,
            })
            .expect("a ReplayShellCommand effect");
        assert_eq!(action_id, "os.directory.rename-space");
        let args_value: pack::JsonValue = pack::json_from_dsl_value(&args.expect("args"));
        assert_eq!(args_value["spaceId"], "sp-1");
        assert_eq!(args_value["name"], "New Name");
    }
}
//#endregion 🧪️Tests
