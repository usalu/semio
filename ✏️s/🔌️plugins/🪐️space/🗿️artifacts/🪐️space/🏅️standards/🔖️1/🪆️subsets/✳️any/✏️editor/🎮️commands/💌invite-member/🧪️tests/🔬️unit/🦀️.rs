
use super::*;
use crate::editor::space_index::{SpaceIndexCommand, testkit};

#[semio_framework_async_macros::async_test]
async fn invite_member_relays_upsert_member() {
    let mut app = testkit::new_app().await;
    let result = app.dispatch_typed(SpaceIndexCommand::InviteMember(InviteMember { email: "a@example.com".into(), role: "author".into() }), &semio_framework_plugin::testkit::meta("local")).await.expect("invite");
    assert!(result.mutations.is_empty());
    assert_eq!(result.requested_effects.len(), 1);
    match &result.requested_effects[0] {
        Effect::ReplayShellCommand { action_id, args } => {
            assert_eq!(action_id, "os.directory.upsert-member");
            let args = pack::json_from_dsl_value(&args.clone().unwrap());
            assert_eq!(args.get("email").and_then(|v| v.as_str()), Some("a@example.com"));
            assert_eq!(args.get("role").and_then(|v| v.as_str()), Some("author"));
        }
        other => panic!("expected ReplayShellCommand, got {other:?}"),
    }
}
