
use super::*;

fn dispatch(payload: RenameSpace, row: Option<&store::os_directory::DirectorySpace>) -> Emit<SHomeMutation, HomeConfigMutation> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = ArtifactView::new(&doc_snapshot, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    handle_with_row(&payload, &doc, &cfg, row).expect("handle")
}

fn folded_row(name: &str) -> store::os_directory::DirectorySpace {
    let event_json = pack::json!({
        "seq": 1, "id": "evt-1", "hlc": {"physicalMs": 0, "logical": 0}, "actor": {"kind": "user", "id": "u"}, "spaceId": "sp-1",
        "body": {"kind": "space.created", "spaceId": "sp-1", "name": name, "spaceKind": "atelier", "visibility": "private", "ownerUserId": "u1"},
        "recordedAtMs": 1000
    })
    .to_string();
    let event = pack::from_json_str::<store::os_directory::DirectoryEvent>(&event_json).expect("fixture directory event");
    store::os_directory::fold(store::os_directory::DirectoryReadModel::default(), &event).spaces.remove("sp-1").expect("folded row")
}

#[semio_framework_async_macros::async_test]
async fn empty_name_opens_the_dialog_preseeded_with_the_current_name() {
    let row = folded_row("Old Name");
    let emit = dispatch(RenameSpace { space_id: "sp-1".into(), name: String::new() }, Some(&row));
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
    let emit = dispatch(RenameSpace { space_id: "sp-1".into(), name: "New Name".into() }, None);
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

/// 🚫️ Without the retained job's captured projection there is no current name to seed: the direct lane refuses by name.
#[test]
fn the_direct_lane_refuses_by_name() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = ArtifactView::new(&doc_snapshot, &history);
    let config = HomeConfig::default();
    let refused = handle(&RenameSpace { space_id: "sp-1".into(), name: String::new() }, &doc, &ConfigView { snapshot: &config, window: None });
    assert!(matches!(refused, Err(fault) if fault.code.0.as_str() == "s.home.rename-space.requires-retained-job"));
}
