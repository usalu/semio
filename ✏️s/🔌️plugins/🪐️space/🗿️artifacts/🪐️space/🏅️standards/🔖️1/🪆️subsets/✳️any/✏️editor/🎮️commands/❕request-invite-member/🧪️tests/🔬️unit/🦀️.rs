
use super::*;
use crate::editor::space_index::{SpaceIndexCommand, testkit};

#[semio_framework_async_macros::async_test]
async fn request_invite_member_opens_the_dialog() {
    let mut app = testkit::new_app().await;
    let result = app.dispatch_typed(SpaceIndexCommand::RequestInviteMember(RequestInviteMember {}), &semio_framework_plugin::testkit::meta("local")).await.expect("request invite");
    assert!(result.mutations.is_empty());
    assert_eq!(result.requested_effects.len(), 1);
    match &result.requested_effects[0] {
        Effect::OpenDialog { dialog_id, .. } => assert_eq!(dialog_id, "inviteMember"),
        other => panic!("expected OpenDialog, got {other:?}"),
    }
}
