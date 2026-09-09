
use super::*;

async fn dispatch(payload: RenameSpace, config: &HomeConfig) -> Emit<SHomeMutation, HomeConfigMutation> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = ArtifactView::new(&doc_snapshot, &history);
    let cfg = ConfigView { snapshot: config, window: None };
    handle(&payload, &doc, &cfg).expect("handle")
}

#[semio_framework_async_macros::async_test]
async fn empty_name_opens_the_dialog_preseeded_with_the_current_name() {
    let event_json = pack::json!({
        "seq": 1, "id": "evt-1", "hlc": {"physicalMs": 0, "logical": 0}, "actor": {"kind": "user", "id": "u"}, "spaceId": "sp-1",
        "body": {"kind": "space.created", "spaceId": "sp-1", "name": "Old Name", "spaceKind": "atelier", "visibility": "private", "ownerUserId": "u1"},
        "recordedAtMs": 1000
    })
    .to_string();
    let config = protocol::Mutation::diff(&HomeConfigMutation::FoldDirectoryEvent { event_json }, &HomeConfig::default()).diff().clone();
    let emit = dispatch(RenameSpace { space_id: "sp-1".into(), name: String::new() }, &config).await;
    let (dialog_id, args) = match &emit.effects[0] {
        Effect::OpenDialog { dialog_id, args, .. } => (dialog_id.clone(), args.clone()),
        other => panic!("expected OpenDialog, got {other:?}"),
    };
    assert_eq!(dialog_id, "renameSpace");
    let args_value: pack::JsonValue = pack::json_from_dsl_value(&args.expect("args"));
    assert_eq!(args_value["name"], "Old Name");
}

#[semio_framework_async_macros::async_test]
async fn non_empty_name_relays_the_rename() {
    let emit = dispatch(RenameSpace { space_id: "sp-1".into(), name: "New Name".into() }, &HomeConfig::default()).await;
    let (action_id, args) = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { action_id, args } => Some((action_id.clone(), args.clone())),
            _ => None,
        })
        .expect("a ReplayShellCommand effect");
    assert_eq!(action_id, "os.directory.rename-space");
    let args_value: pack::JsonValue = pack::json_from_dsl_value(&args.expect("args"));
    assert_eq!(args_value["spaceId"], "sp-1");
    assert_eq!(args_value["name"], "New Name");
}
