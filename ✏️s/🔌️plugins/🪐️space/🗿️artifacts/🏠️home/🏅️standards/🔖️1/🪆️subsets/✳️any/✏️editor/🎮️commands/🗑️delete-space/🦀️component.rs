//! 🗑️ S Home launcher app command — `delete-space`. Two-phase, entirely from ONE command id: the
//! first dispatch (`confirmed: false`, the normal row-click default) emits the declared `deleteSpace`
//! confirm dialog and NEVER touches the network; only a SECOND dispatch with `confirmed: true` (the
//! dialog's own submit, re-carrying `spaceId`/`confirmed` from `OpenDialog`'s pre-seeded args — an
//! empty-`.args()` dialog degenerates to a plain confirm/cancel per `DialogDefinition`'s own doc)
//! emits the real `HostEffect::ReplayShellCommand` (contract §C6).

use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, HostEffect};
use serde::{Deserialize, Serialize};
use serde_json::json;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "delete-space")]
pub struct DeleteSpace {
    pub space_id: String,
    #[serde(default)]
    pub confirmed: bool,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
pub fn handle(payload: &DeleteSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    if !payload.confirmed {
        let args = dsl::to_dsl_value(&json!({ "spaceId": payload.space_id, "confirmed": true })).ok();
        return Ok(Emit::effect(HostEffect::OpenDialog { dialog_id: "deleteSpace".into(), args }));
    }
    let args = dsl::to_dsl_value(&json!({ "spaceId": payload.space_id })).ok();
    Ok(Emit::effect(HostEffect::ReplayShellCommand { action_id: "os.directory.delete-space".into(), args }))
}
//#endregion 🔖️Handle

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn dispatch(payload: DeleteSpace) -> Emit<SHomeMutation, HomeConfigMutation> {
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_snapshot = SHomeSnapshot::default();
        let doc = ArtifactView::new(&doc_snapshot, &history);
        let config = HomeConfig::default();
        let cfg = ConfigView { snapshot: &config };
        handle(&payload, &doc, &cfg).expect("handle")
    }

    #[test]
    fn unconfirmed_delete_emits_the_confirm_dialog_and_never_the_command() {
        let emit = dispatch(DeleteSpace { space_id: "sp-1".into(), confirmed: false });
        assert_eq!(emit.effects.len(), 1);
        let (dialog_id, args) = match &emit.effects[0] {
            HostEffect::OpenDialog { dialog_id, args } => (dialog_id.clone(), args.clone()),
            other => panic!("expected OpenDialog, got {other:?}"),
        };
        assert_eq!(dialog_id, "deleteSpace");
        let args_value: serde_json::Value = dsl::from_dsl_value(args.expect("pre-seeded args")).expect("json");
        assert_eq!(args_value["spaceId"], "sp-1");
        assert_eq!(args_value["confirmed"], true);
        assert!(!emit.effects.iter().any(|e| matches!(e, HostEffect::ReplayShellCommand { .. })), "the confirm dialog must be emitted BEFORE any command");
    }

    #[test]
    fn confirmed_delete_emits_the_replay_shell_command() {
        let emit = dispatch(DeleteSpace { space_id: "sp-1".into(), confirmed: true });
        let (action_id, args) = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                HostEffect::ReplayShellCommand { action_id, args } => Some((action_id.clone(), args.clone())),
                _ => None,
            })
            .expect("a ReplayShellCommand effect");
        assert_eq!(action_id, "os.directory.delete-space");
        let args_value: serde_json::Value = dsl::from_dsl_value(args.expect("args")).expect("json");
        assert_eq!(args_value["spaceId"], "sp-1");
        assert!(!emit.effects.iter().any(|e| matches!(e, HostEffect::OpenDialog { .. })), "a confirmed dispatch never re-opens the dialog");
    }
}
//#endregion 🧪️Tests
