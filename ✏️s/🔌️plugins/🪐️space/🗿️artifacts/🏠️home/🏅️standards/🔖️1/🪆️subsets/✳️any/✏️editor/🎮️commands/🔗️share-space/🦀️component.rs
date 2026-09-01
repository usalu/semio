//! 🔗️ S Home launcher app command — `share-space` (member email + role → `os.directory.upsert-
//! member`, contract §C6). Companion "copy invite link" action lives at its own command leaf,
//! `🎮️commands/📋copy-invite-link` (one struct per `app_commands!` module, mirroring every other leaf
//! in this directory). An empty `email` (a raw row click) opens the declared `shareSpace` dialog; a
//! non-empty `email` (the dialog's own submit) relays the membership upsert to the hub — no optimistic
//! local mutation.

use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "share-space")]
pub struct ShareSpace {
    pub space_id: String,
    pub email: String,
    pub role: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
pub async fn handle(payload: &ShareSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    if payload.email.trim().is_empty() {
        let args = Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": payload.space_id })));
        return Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(126), dialog_id: "shareSpace".into(), args }));
    }
    let role = if payload.role.trim().is_empty() { "spectator".to_string() } else { payload.role.clone() };
    let args = Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": payload.space_id, "email": payload.email, "role": role })));
    Ok(Emit::effect(Effect::ReplayShellCommand { action_id: "os.directory.upsert-member".into(), args }))
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
    async fn empty_email_opens_the_share_dialog() {
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_snapshot = SHomeSnapshot::default();
        let doc = doc_view(&history, &doc_snapshot);
        let config = HomeConfig::default();
        let cfg = ConfigView { snapshot: &config };
        let emit = handle(&ShareSpace { space_id: "sp-1".into(), email: String::new(), role: String::new() }, &doc, &cfg).expect("handle");
        assert!(matches!(emit.effects.as_slice(), [Effect::OpenDialog { dialog_id, .. }] if dialog_id == "shareSpace"));
    }

    #[semio_framework_async_macros::async_test]
    async fn email_and_role_relay_upsert_member() {
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_snapshot = SHomeSnapshot::default();
        let doc = doc_view(&history, &doc_snapshot);
        let config = HomeConfig::default();
        let cfg = ConfigView { snapshot: &config };
        let emit = handle(&ShareSpace { space_id: "sp-1".into(), email: "ada@semio.dev".into(), role: "author".into() }, &doc, &cfg).expect("handle");
        let (action_id, args) = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                Effect::ReplayShellCommand { action_id, args } => Some((action_id.clone(), args.clone())),
                _ => None,
            })
            .expect("a ReplayShellCommand effect");
        assert_eq!(action_id, "os.directory.upsert-member");
        let args_value: pack::JsonValue = pack::json_from_dsl_value(&args.expect("args"));
        assert_eq!(args_value["email"], "ada@semio.dev");
        assert_eq!(args_value["role"], "author");
    }

    #[semio_framework_async_macros::async_test]
    async fn blank_role_defaults_to_spectator() {
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_snapshot = SHomeSnapshot::default();
        let doc = doc_view(&history, &doc_snapshot);
        let config = HomeConfig::default();
        let cfg = ConfigView { snapshot: &config };
        let emit = handle(&ShareSpace { space_id: "sp-1".into(), email: "ada@semio.dev".into(), role: String::new() }, &doc, &cfg).expect("handle");
        let args = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                Effect::ReplayShellCommand { args, .. } => args.clone(),
                _ => None,
            })
            .expect("args");
        let args_value: pack::JsonValue = pack::json_from_dsl_value(&args);
        assert_eq!(args_value["role"], "spectator");
    }
}
//#endregion 🧪️Tests
