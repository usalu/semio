//! ❕️ SpaceIndexEditor commands command — `request-invite-member`. View-only: opens the
//! `inviteMember` staged-form dialog (email + role — worker-brief task 3's "invite-by-email + role");
//! the dialog's own submit re-dispatches the real `💌invite-member` command with the staged args.

use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "request-invite-member")]
pub struct RequestInviteMember {}

pub async fn handle(_payload: &RequestInviteMember, _doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    Ok(Emit::effect(Effect::OpenDialog {req: semio_framework_plugin::RequestId(129),  dialog_id: "inviteMember".into(), args: None }))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::space_index::{testkit, SpaceIndexCommand};
    

    #[test]
    async fn request_invite_member_opens_the_dialog() {
        let mut app = testkit::new_app();
        let result = app.dispatch_typed(SpaceIndexCommand::RequestInviteMember(RequestInviteMember {}), &semio_framework_plugin::testkit::meta("local")).expect("request invite");
        assert!(result.mutations.is_empty());
        assert_eq!(result.requested_effects.len(), 1);
        match &result.requested_effects[0] {
            Effect::OpenDialog { dialog_id, .. } => assert_eq!(dialog_id, "inviteMember"),
            other => panic!("expected OpenDialog, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
