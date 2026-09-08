
use super::*;

async fn doc_view<'a>(history: &'a semio_framework_plugin::HistoryView, doc_snapshot: &'a SHomeSnapshot) -> ArtifactView<'a, SHomeSnapshot> {
    ArtifactView::new(doc_snapshot, history)
}

#[semio_framework_async_macros::async_test]
async fn a_space_id_relays_the_shell_administration_effect_without_a_local_mutation() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = doc_view(&history, &doc_snapshot).await;
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config };
    let emit = handle(&ManageSpace { space_id: "space-a".into() }, &doc, &cfg).expect("manage space relays");
    let (action_id, args) = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { action_id, args } => Some((action_id.clone(), args.clone())),
            _ => None,
        })
        .expect("a ReplayShellCommand effect");
    assert_eq!(action_id, "os.directory.open-administration");
    let args_value: pack::JsonValue = pack::json_from_dsl_value(&args.expect("args"));
    assert_eq!(args_value["spaceId"], "space-a");
}

#[semio_framework_async_macros::async_test]
async fn an_empty_space_id_is_a_fault_not_a_blank_pane() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = doc_view(&history, &doc_snapshot).await;
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config };
    assert!(handle(&ManageSpace { space_id: "  ".into() }, &doc, &cfg).is_err());
}
