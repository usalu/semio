
use super::*;

async fn doc_view<'a>(history: &'a semio_framework_plugin::HistoryView, doc_snapshot: &'a SHomeSnapshot) -> ArtifactView<'a, SHomeSnapshot> {
    ArtifactView::new(doc_snapshot, history)
}

#[semio_framework_async_macros::async_test]
async fn empty_email_opens_the_share_dialog() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = doc_view(&history, &doc_snapshot).await;
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config };
    let emit = handle(&ShareSpace { space_id: "sp-1".into(), email: String::new(), role: String::new() }, &doc, &cfg).expect("handle");
    assert!(matches!(emit.effects.as_slice(), [Effect::OpenDialog { dialog_id, .. }] if dialog_id == "shareSpace"));
}

#[semio_framework_async_macros::async_test]
async fn email_and_role_relay_upsert_member() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = doc_view(&history, &doc_snapshot).await;
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config };
    let emit = handle(&ShareSpace { space_id: "sp-1".into(), email: "ada@semio.dev".into(), role: "author".into() }, &doc, &cfg).expect("handle");
    let (action_id, args) = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { action_id, args } => Some((action_id.clone(), args.clone())),
            _ => None,
        })
        .expect("a ReplayShellCommand effect");
    assert_eq!(action_id, "os.directory.upsert-member");
    let args_value: pack::JsonValue = pack::json_from_dsl_value(&args.expect("args"));
    assert_eq!(args_value["email"], "ada@semio.dev");
    assert_eq!(args_value["role"], "author");
}

#[semio_framework_async_macros::async_test]
async fn blank_role_defaults_to_spectator() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = doc_view(&history, &doc_snapshot).await;
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config };
    let emit = handle(&ShareSpace { space_id: "sp-1".into(), email: "ada@semio.dev".into(), role: String::new() }, &doc, &cfg).expect("handle");
    let args = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { args, .. } => args.clone(),
            _ => None,
        })
        .expect("args");
    let args_value: pack::JsonValue = pack::json_from_dsl_value(&args);
    assert_eq!(args_value["role"], "spectator");
}
