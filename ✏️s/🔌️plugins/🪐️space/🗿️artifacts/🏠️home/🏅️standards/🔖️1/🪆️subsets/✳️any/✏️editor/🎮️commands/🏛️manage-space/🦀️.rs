//! 🏛️ S Home launcher app command — `manage-space`. Opens the Shell-owned Administration pane for
//! exactly one space. It carries no role, no capability, and no page: the pane the host effect opens
//! renders solely from the hub's own canonical `DirectorySpaceAdministrationPageV1`, so a client that
//! reached this action without authority still gets a server denial rather than an administration UI.

use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "manage-space")]
pub struct ManageSpace {
    pub space_id: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
pub fn handle(payload: &ManageSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    if payload.space_id.trim().is_empty() {
        return Err(Fault::from("s.home.manage-space-requires-a-space"));
    }
    let args = Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": payload.space_id.clone() })));
    Ok(Emit::effect(Effect::ReplayShellCommand { action_id: "os.directory.open-administration".into(), args }))
}
//#endregion 🔖️Handle

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    async fn doc_view<'a>(history: &'a semio_framework_plugin::HistoryView, doc_snapshot: &'a SHomeSnapshot) -> ArtifactView<'a, SHomeSnapshot> {
        ArtifactView::new(doc_snapshot, history)
    }

    #[semio_framework_async_macros::async_test]
    async fn a_space_id_relays_the_shell_administration_effect_without_a_local_mutation() {
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_snapshot = SHomeSnapshot::default();
        let doc = doc_view(&history, &doc_snapshot).await;
        let config = HomeConfig::default();
        let cfg = ConfigView { snapshot: &config };
        let emit = handle(&ManageSpace { space_id: "space-a".into() }, &doc, &cfg).expect("manage space relays");
        let (action_id, args) = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                Effect::ReplayShellCommand { action_id, args } => Some((action_id.clone(), args.clone())),
                _ => None,
            })
            .expect("a ReplayShellCommand effect");
        assert_eq!(action_id, "os.directory.open-administration");
        let args_value: pack::JsonValue = pack::json_from_dsl_value(&args.expect("args"));
        assert_eq!(args_value["spaceId"], "space-a");
    }

    #[semio_framework_async_macros::async_test]
    async fn an_empty_space_id_is_a_fault_not_a_blank_pane() {
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_snapshot = SHomeSnapshot::default();
        let doc = doc_view(&history, &doc_snapshot).await;
        let config = HomeConfig::default();
        let cfg = ConfigView { snapshot: &config };
        assert!(handle(&ManageSpace { space_id: "  ".into() }, &doc, &cfg).is_err());
    }
}
//#endregion 🧪️Tests
