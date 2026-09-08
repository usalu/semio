//! ❕️ SpaceIndexEditor commands command — `request-invite-member`. View-only: opens the
//! `inviteMember` staged-form dialog (email + role — worker-brief task 3's "invite-by-email + role");
//! the dialog's own submit re-dispatches the real `💌invite-member` command with the staged args.

use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "request-invite-member")]
pub struct RequestInviteMember {}

pub fn handle(_payload: &RequestInviteMember, _doc: &ArtifactView<'_, SSpaceSnapshot>, _cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(129), dialog_id: "inviteMember".into(), args: None }))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
