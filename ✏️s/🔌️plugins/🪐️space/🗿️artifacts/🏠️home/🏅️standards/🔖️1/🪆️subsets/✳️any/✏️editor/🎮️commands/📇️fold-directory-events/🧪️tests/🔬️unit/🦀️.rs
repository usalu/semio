
use super::*;

#[semio_framework_async_macros::async_test]
async fn one_config_mutation_per_event() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = ArtifactView::new(&doc_snapshot, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config };
    let events_json = pack::json!([
        {"seq": 1, "id": "e1", "hlc": {"physicalMs": 0, "logical": 0}, "actor": {"kind": "user", "id": "u"}, "spaceId": "sp-1",
         "body": {"kind": "space.created", "spaceId": "sp-1", "name": "A", "spaceKind": "atelier", "visibility": "private", "ownerUserId": "u1"}, "recordedAtMs": 1},
        {"seq": 2, "id": "e2", "hlc": {"physicalMs": 0, "logical": 0}, "actor": {"kind": "user", "id": "u"}, "spaceId": "sp-2",
         "body": {"kind": "space.created", "spaceId": "sp-2", "name": "B", "spaceKind": "studio", "visibility": "public", "ownerUserId": "u2"}, "recordedAtMs": 2}
    ])
    .to_string();
    let emit = handle(&FoldDirectoryEvents { events_json }, &doc, &cfg).expect("handle");
    assert_eq!(emit.config_mutations.len(), 2);
    assert!(emit.artifact_mutations.is_empty(), "never an artifact mutation");
    assert!(emit.effects.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn malformed_events_json_yields_no_mutations() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = ArtifactView::new(&doc_snapshot, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config };
    let emit = handle(&FoldDirectoryEvents { events_json: "not json".into() }, &doc, &cfg).expect("handle");
    assert!(emit.config_mutations.is_empty());
}
