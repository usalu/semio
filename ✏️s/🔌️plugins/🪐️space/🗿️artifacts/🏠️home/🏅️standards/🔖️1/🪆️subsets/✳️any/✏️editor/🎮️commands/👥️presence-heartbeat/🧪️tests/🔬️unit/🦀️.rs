
use super::*;

#[semio_framework_async_macros::async_test]
async fn heartbeat_is_dispatchable_and_emits_nothing() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = ArtifactView::new(&doc_snapshot, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&PresenceHeartbeat {}, &doc, &cfg).expect("handle");
    assert!(emit.artifact_mutations.is_empty());
    assert!(emit.config_mutations.is_empty());
    assert!(emit.effects.is_empty());
}
