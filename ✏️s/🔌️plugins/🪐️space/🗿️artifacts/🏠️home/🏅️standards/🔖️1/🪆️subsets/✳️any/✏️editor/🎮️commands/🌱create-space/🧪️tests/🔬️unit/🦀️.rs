
use super::*;

async fn view<'a>(history: &'a semio_framework_plugin::HistoryView, doc_snapshot: &'a SHomeSnapshot) -> ArtifactView<'a, SHomeSnapshot> {
    ArtifactView::new(doc_snapshot, history)
}

#[semio_framework_async_macros::async_test]
async fn empty_name_opens_the_dialog_instead_of_relaying() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = view(&history, &doc_snapshot).await;
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config };
    let emit = handle(&CreateSpace { name: String::new(), kind: "atelier".into(), visibility: "private".into() }, &doc, &cfg).expect("handle");
    assert!(matches!(emit.effects.as_slice(), [Effect::OpenDialog { dialog_id, args: None, .. }] if dialog_id == "createSpace"), "empty name must open the dialog, not relay: {:?}", emit.effects);
}

#[semio_framework_async_macros::async_test]
async fn valid_name_emits_the_replay_shell_command_with_the_right_action_id_and_args() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = view(&history, &doc_snapshot).await;
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config };
    let emit = handle(&CreateSpace { name: "Atelier".into(), kind: "atelier".into(), visibility: "private".into() }, &doc, &cfg).expect("handle");
    let (action_id, args) = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { action_id, args } => Some((action_id.clone(), args.clone())),
            _ => None,
        })
        .expect("a ReplayShellCommand effect");
    assert_eq!(action_id, "os.directory.create-space");
    let args_value: pack::JsonValue = pack::json_from_dsl_value(&args.expect("args present"));
    assert_eq!(args_value["name"], "Atelier");
    assert_eq!(args_value["spaceKind"], "atelier");
    assert_eq!(args_value["visibility"], "private");
}

#[semio_framework_async_macros::async_test]
async fn blank_kind_and_visibility_default_to_atelier_and_private() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = view(&history, &doc_snapshot).await;
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config };
    let emit = handle(&CreateSpace { name: "Studio".into(), kind: String::new(), visibility: String::new() }, &doc, &cfg).expect("handle");
    let args = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { args, .. } => args.clone(),
            _ => None,
        })
        .expect("args present");
    let args_value: pack::JsonValue = pack::json_from_dsl_value(&args);
    assert_eq!(args_value["spaceKind"], "atelier");
    assert_eq!(args_value["visibility"], "private");
}
