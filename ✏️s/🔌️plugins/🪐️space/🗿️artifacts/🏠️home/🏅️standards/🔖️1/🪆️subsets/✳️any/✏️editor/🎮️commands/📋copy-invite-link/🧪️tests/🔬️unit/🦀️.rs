
use super::*;

#[semio_framework_async_macros::async_test]
async fn copy_invite_link_relays_share_link_with_default_ttl() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = ArtifactView::new(&doc_snapshot, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config };
    let emit = handle(&CopyInviteLink { space_id: "sp-1".into(), role: "spectator".into(), ttl_secs: 0 }, &doc, &cfg).expect("handle");
    let (action_id, args) = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { action_id, args } => Some((action_id.clone(), args.clone())),
            _ => None,
        })
        .expect("a ReplayShellCommand effect");
    assert_eq!(action_id, "os.directory.share-link");
    let args_value: pack::JsonValue = pack::json_from_dsl_value(&args.expect("args"));
    assert_eq!(args_value["ttlSecs"], 3600);
    assert_eq!(args_value["spaceId"], "sp-1");
}

#[semio_framework_async_macros::async_test]
async fn explicit_ttl_is_respected() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = ArtifactView::new(&doc_snapshot, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config };
    let emit = handle(&CopyInviteLink { space_id: "sp-1".into(), role: "author".into(), ttl_secs: 60 }, &doc, &cfg).expect("handle");
    let args = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { args, .. } => args.clone(),
            _ => None,
        })
        .expect("args");
    let args_value: pack::JsonValue = pack::json_from_dsl_value(&args);
    assert_eq!(args_value["ttlSecs"], 60);
    assert_eq!(args_value["role"], "author");
}
