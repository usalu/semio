
use super::*;
use crate::editor::space_index::{SpaceIndexCommand, testkit};

#[semio_framework_async_macros::async_test]
async fn remove_member_relays_remove_member() {
    let mut app = testkit::new_app().await;
    let result = app.dispatch_typed(SpaceIndexCommand::RemoveMember(RemoveMember { user_id: "u-1".into() }), &semio_framework_plugin::testkit::meta("local")).await.expect("remove");
    assert_eq!(result.requested_effects.len(), 1);
    match &result.requested_effects[0] {
        Effect::ReplayShellCommand { action_id, args } => {
            assert_eq!(action_id, "os.directory.remove-member");
            let args = pack::json_from_dsl_value(&args.clone().unwrap());
            assert_eq!(args.get("userId").and_then(|v| v.as_str()), Some("u-1"));
        }
        other => panic!("expected ReplayShellCommand, got {other:?}"),
    }
}
