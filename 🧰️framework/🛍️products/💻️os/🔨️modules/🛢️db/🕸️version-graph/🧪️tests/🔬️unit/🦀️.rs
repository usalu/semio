
use super::*;

//#region 🔖️VersionGraph
#[semio_framework_async_macros::async_test]
async fn null_version_graph_never_panics_always_reports_unimplemented() {
    let graph = NullVersionGraph;
    let document: ArtifactId = "doc-1".into();
    let change = ChangeRecord { parent: None, content_hash: pack::ContentHash([0u8; 32]), author: "actor-1".into(), message: "msg".to_string(), timestamp_ms: 0 };
    assert!(matches!(graph.record_change(&document, change).await, Err(DbError::Unimplemented(_))));

    let checkpoint = CheckpointRequest { parent_checkpoint: None, change_ids: vec![], message: "msg".to_string(), authors: vec![], timestamp_ms: 0 };
    assert!(matches!(graph.checkpoint(&document, checkpoint).await, Err(DbError::Unimplemented(_))));
    assert!(matches!(graph.merge_base(&document, "a", "b").await, Err(DbError::Unimplemented(_))));
    assert!(matches!(graph.head(&document, "main").await, Err(DbError::Unimplemented(_))));
}

// 🔀️ dedyn-fw-os-misc: was `version_graph_trait_object_is_dyn_compatible` (asserted `Box<dyn
// VersionGraph>` construction) — O1 removed the trait object; the equivalent coverage is that a
// bare `NullVersionGraph` still satisfies every `VersionGraph` method through the trait, which
// is exactly what every real call site (now `db_engine::VersionGraphs::Null(..)`) relies on.
#[semio_framework_async_macros::async_test]
async fn null_version_graph_satisfies_the_version_graph_trait_directly() {
    let graph = NullVersionGraph;
    let document: ArtifactId = "doc-1".into();
    assert!(graph.head(&document, "main").await.is_err());
}
//#endregion 🔖️VersionGraph

//#region 🔖️Emit
struct RecordingEmit {
    events: std::sync::Mutex<Vec<EmitEvent>>,
}

impl Emit for RecordingEmit {
    async fn emit(&self, event: EmitEvent) {
        self.events.lock().unwrap().push(event);
    }
}

#[semio_framework_async_macros::async_test]
// 🔀️ dedyn-emit-runtime, O1/R11: was `emit_trait_object_records_events_with_fields_and_document`
// (asserted `&dyn Emit` construction) — O1 removed the trait object; the equivalent coverage is
// that a bare `RecordingEmit` still satisfies `Emit::emit` directly, which is exactly what every
// real call site now relies on (generic `E: Emit` params, or a concrete `NullEmit`).
async fn emit_satisfies_the_emit_trait_directly_and_records_events() {
    let sink = RecordingEmit { events: std::sync::Mutex::new(Vec::new()) };
    sink.emit(EmitEvent::new("command.applied").with_document("doc-1".into()).field("bytes", EmitField::U64(128)).field("ok", EmitField::Bool(true))).await;
    let events = sink.events.lock().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].name, "command.applied");
    assert_eq!(events[0].document, Some(ArtifactId::from("doc-1")));
    assert_eq!(events[0].fields.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn null_emit_discards_without_panicking() {
    let emit = NullEmit;
    emit.emit(EmitEvent::new("noop")).await;
}
//#endregion 🔖️Emit
