
use super::*;
use crate::editor::space_index::{SpaceIndexCommand, unit_tests::context};

#[semio_framework_async_macros::async_test]
async fn copy_invite_link_relays_share_link() {
    let mut app = artifact_app_laws::new_app().await;
    let result = app.dispatch_typed(SpaceIndexCommand::CopyInviteLink(CopyInviteLink { role: "spectator".into(), ttl_secs: 3600 }), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("copy link");
    assert_eq!(result.requested_effects.len(), 1);
    match &result.requested_effects[0] {
        Effect::ReplayShellCommand { action_id, args } => {
            assert_eq!(action_id, "os.directory.share-link");
            let args = pack::json_from_dsl_value(&args.clone().unwrap());
            assert_eq!(args.get("role").and_then(|v| v.as_str()), Some("spectator"));
            // 🔢️ `DslValue`'s numeric lane round-trips through f64 (confirmed empirically: a JSON
            // `u64` comes back as `3600.0`, not `3600`) — `serde_json::Number::as_u64()` only
            // succeeds for values that were themselves parsed/stored as an unsigned integer, so
            // asserting via `.as_f64()` is the honest check here, not a workaround.
            assert_eq!(args.get("ttlSecs").and_then(|v| v.as_f64()), Some(3600.0));
        }
        other => panic!("expected ReplayShellCommand, got {other:?}"),
    }
}
