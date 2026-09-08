
use super::*;

async fn dispatch(payload: DeleteSpace) -> Emit<SHomeMutation, HomeConfigMutation> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = ArtifactView::new(&doc_snapshot, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config };
    handle(&payload, &doc, &cfg).expect("handle")
}

#[semio_framework_async_macros::async_test]
async fn unconfirmed_delete_emits_the_confirm_dialog_and_never_the_command() {
    let emit = dispatch(DeleteSpace { space_id: "sp-1".into(), confirmed: false }).await;
    assert_eq!(emit.effects.len(), 1);
    let (dialog_id, args) = match &emit.effects[0] {
        Effect::OpenDialog { dialog_id, args, .. } => (dialog_id.clone(), args.clone()),
        other => panic!("expected OpenDialog, got {other:?}"),
    };
    assert_eq!(dialog_id, "deleteSpace");
    let args_value: pack::JsonValue = pack::json_from_dsl_value(&args.expect("pre-seeded args"));
    assert_eq!(args_value["spaceId"], "sp-1");
    assert_eq!(args_value["confirmed"], true);
    assert!(!emit.effects.iter().any(|e| matches!(e, Effect::ReplayShellCommand { .. })), "the confirm dialog must be emitted BEFORE any command");
}

#[semio_framework_async_macros::async_test]
async fn confirmed_delete_emits_the_replay_shell_command() {
    let emit = dispatch(DeleteSpace { space_id: "sp-1".into(), confirmed: true }).await;
    let (action_id, args) = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { action_id, args } => Some((action_id.clone(), args.clone())),
            _ => None,
        })
        .expect("a ReplayShellCommand effect");
    assert_eq!(action_id, "os.directory.delete-space");
    let args_value: pack::JsonValue = pack::json_from_dsl_value(&args.expect("args"));
    assert_eq!(args_value["spaceId"], "sp-1");
    assert!(!emit.effects.iter().any(|e| matches!(e, Effect::OpenDialog { .. })), "a confirmed dispatch never re-opens the dialog");
}
