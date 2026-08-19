//! 💌️ SpaceIndexEditor commands command — `invite-member`. Members panel's invite-by-email
//! affordance (worker-brief task 3): relays `os.directory.upsert-member` (contract §C6) — the guest
//! never talks to the network directly, the shell's command funnel calls `DirectoryClient.command`.

use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "invite-member")]
pub struct InviteMember {
    pub email: String,
    pub role: String,
}

pub async fn handle(payload: &InviteMember, doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    Ok(Emit::effect(Effect::ReplayShellCommand { action_id: "os.directory.upsert-member".into(), args: semio_framework::optional_json_to_dsl(Some(json!({ "spaceId": doc.snapshot.space_id, "email": payload.email, "role": payload.role }))) }))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::space_index::{testkit, SpaceIndexCommand};
    

    #[test]
    async fn invite_member_relays_upsert_member() {
        let mut app = testkit::new_app();
        let result = app.dispatch_typed(SpaceIndexCommand::InviteMember(InviteMember { email: "a@example.com".into(), role: "author".into() }), &semio_framework_plugin::testkit::meta("local")).expect("invite");
        assert!(result.mutations.is_empty());
        assert_eq!(result.requested_effects.len(), 1);
        match &result.requested_effects[0] {
            Effect::ReplayShellCommand { action_id, args } => {
                assert_eq!(action_id, "os.directory.upsert-member");
                let args = semio_framework_os_kernel::pack_rt::dsl_value_to_json(args.clone().unwrap());
                assert_eq!(args.get("email").and_then(|v| v.as_str()), Some("a@example.com"));
                assert_eq!(args.get("role").and_then(|v| v.as_str()), Some("author"));
            }
            other => panic!("expected ReplayShellCommand, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
