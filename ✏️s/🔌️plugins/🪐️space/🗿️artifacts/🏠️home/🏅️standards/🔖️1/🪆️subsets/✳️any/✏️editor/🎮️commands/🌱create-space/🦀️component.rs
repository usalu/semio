//! 🌱️ S Home launcher app command — `create-space`. Contract §C6: the plugin never touches the
//! network — a validated create request is relayed to the shell as `Effect::ReplayShellCommand`;
//! the resulting `space.created` event returns over `/directory/ws` and is folded back in by
//! `fold-directory-events`. No optimistic mutation of the read model. An empty (raw toolbar click, not
//! yet a dialog submit) `name` opens the declared `createSpace` dialog instead of relaying — the local-
//! only "create ephemeral studio" path (`create-studio`) is untouched and still works with no hub.

use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, Effect};
use serde::{Deserialize, Serialize};
use serde_json::json;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "create-space")]
pub struct CreateSpace {
    pub name: String,
    pub kind: String,
    pub visibility: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
pub async fn handle(payload: &CreateSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    if payload.name.trim().is_empty() {
        return Ok(Emit::effect(Effect::OpenDialog {req: semio_framework_plugin::RequestId(123),  dialog_id: "createSpace".into(), args: None }));
    }
    let kind = if payload.kind.trim().is_empty() { "atelier".to_string() } else { payload.kind.clone() };
    let visibility = if payload.visibility.trim().is_empty() { "private".to_string() } else { payload.visibility.clone() };
    let args = dsl::to_dsl_value(&json!({ "name": payload.name, "spaceKind": kind, "visibility": visibility })).ok();
    Ok(Emit::effect(Effect::ReplayShellCommand { action_id: "os.directory.create-space".into(), args }))
}
//#endregion 🔖️Handle

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    async fn view<'a>(history: &'a semio_framework_plugin::HistoryView, doc_snapshot: &'a SHomeSnapshot) -> ArtifactView<'a, SHomeSnapshot> {
        ArtifactView::new(doc_snapshot, history)
    }

    #[test]
    async fn empty_name_opens_the_dialog_instead_of_relaying() {
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_snapshot = SHomeSnapshot::default();
        let doc = view(&history, &doc_snapshot);
        let config = HomeConfig::default();
        let cfg = ConfigView { snapshot: &config };
        let emit = handle(&CreateSpace { name: String::new(), kind: "atelier".into(), visibility: "private".into() }, &doc, &cfg).expect("handle");
        assert!(matches!(emit.effects.as_slice(), [Effect::OpenDialog { dialog_id, args: None, .. }] if dialog_id == "createSpace"), "empty name must open the dialog, not relay: {:?}", emit.effects);
    }

    #[test]
    async fn valid_name_emits_the_replay_shell_command_with_the_right_action_id_and_args() {
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_snapshot = SHomeSnapshot::default();
        let doc = view(&history, &doc_snapshot);
        let config = HomeConfig::default();
        let cfg = ConfigView { snapshot: &config };
        let emit = handle(&CreateSpace { name: "Atelier".into(), kind: "atelier".into(), visibility: "private".into() }, &doc, &cfg).expect("handle");
        let (action_id, args) = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                Effect::ReplayShellCommand { action_id, args } => Some((action_id.clone(), args.clone())),
                _ => None,
            })
            .expect("a ReplayShellCommand effect");
        assert_eq!(action_id, "os.directory.create-space");
        let args_value: serde_json::Value = dsl::from_dsl_value(args.expect("args present")).expect("args decode as json");
        assert_eq!(args_value["name"], "Atelier");
        assert_eq!(args_value["spaceKind"], "atelier");
        assert_eq!(args_value["visibility"], "private");
    }

    #[test]
    async fn blank_kind_and_visibility_default_to_atelier_and_private() {
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_snapshot = SHomeSnapshot::default();
        let doc = view(&history, &doc_snapshot);
        let config = HomeConfig::default();
        let cfg = ConfigView { snapshot: &config };
        let emit = handle(&CreateSpace { name: "Studio".into(), kind: String::new(), visibility: String::new() }, &doc, &cfg).expect("handle");
        let args = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                Effect::ReplayShellCommand { args, .. } => args.clone(),
                _ => None,
            })
            .expect("args present");
        let args_value: serde_json::Value = dsl::from_dsl_value(args).expect("json");
        assert_eq!(args_value["spaceKind"], "atelier");
        assert_eq!(args_value["visibility"], "private");
    }
}
//#endregion 🧪️Tests
