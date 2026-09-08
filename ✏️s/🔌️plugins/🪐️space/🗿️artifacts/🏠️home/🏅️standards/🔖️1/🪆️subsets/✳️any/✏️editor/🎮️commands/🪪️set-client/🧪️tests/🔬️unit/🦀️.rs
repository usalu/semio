
use super::*;

#[semio_framework_async_macros::async_test]
async fn set_client_emits_exactly_one_config_mutation() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = ArtifactView::new(&doc_snapshot, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config };
    let emit = handle(&SetClient { client_id: "u1".into(), client_name: "Ada".into() }, &doc, &cfg).expect("handle");
    assert_eq!(emit.config_mutations, vec![HomeConfigMutation::SetClient { client_id: "u1".into(), client_name: "Ada".into() }]);
    assert!(emit.artifact_mutations.is_empty());
}
