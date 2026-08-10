//! 🕸️ Version graph seam and Emit observability.

use crate::db_ids::{ActorId, DbError, ArtifactId};
use pack::ContentHash;

//#region 🔖️VersionGraph
/// @emoji 📝️ One committed, content-addressed change to record in the version graph — the
/// `VersionGraph::record_change` argument shape, deliberately vcs-type-free (see trait doc).
#[derive(Clone, Debug)]
pub struct ChangeRecord {
    pub parent: Option<String>,
    pub content_hash: pack::ContentHash,
    pub author: ActorId,
    pub message: String,
    pub timestamp_ms: u64,
}

/// @emoji 🏁️ A checkpoint (a named, authored group of changes) to record — the
/// `VersionGraph::checkpoint` argument shape.
#[derive(Clone, Debug)]
pub struct CheckpointRequest {
    pub parent_checkpoint: Option<String>,
    pub change_ids: Vec<String>,
    pub message: String,
    pub authors: Vec<ActorId>,
    pub timestamp_ms: u64,
}

/// @emoji 🌿️ The vcs seam: per the contract's hard dependency rule, only `db_engine` (behind the
/// `vcs` Cargo feature) may depend on the `vcs` crate — every crate below it, including
/// `db_artifact` (which drives commits), talks to version history ONLY through this
/// `vcs`-type-free trait. `db_engine` supplies the real implementation over `vcs::ArtifactVcs*`;
/// anything vcs-agnostic (e.g. a deployment with the `vcs` feature disabled) can supply
/// `NullVersionGraph` instead.
pub trait VersionGraph: Send + Sync {
    /// @emoji 📝️ Records `change` against `document`, returning its assigned change id.
    fn record_change(&self, document: &ArtifactId, change: ChangeRecord) -> Result<String, DbError>;

    /// @emoji 🏁️ Records a checkpoint over previously-recorded changes, returning its assigned
    /// content-addressed checkpoint id (`vcs`'s own concern how that id is derived).
    fn checkpoint(&self, document: &ArtifactId, request: CheckpointRequest) -> Result<String, DbError>;

    /// @emoji 🔀️ The nearest common ancestor checkpoint of `a` and `b`, or `None` if they share
    /// none (disjoint histories).
    fn merge_base(&self, document: &ArtifactId, a: &str, b: &str) -> Result<Option<String>, DbError>;

    /// @emoji 🎯️ The current head checkpoint id of `alternative`, or `None` if it has none yet.
    fn head(&self, document: &ArtifactId, alternative: &str) -> Result<Option<String>, DbError>;
}

/// @emoji 🚫️ A `VersionGraph` that answers every call with `DbError::Unimplemented` rather than
/// panicking — the extension seam this crate offers for a `vcs`-feature-disabled deployment (or a
/// unit test that doesn't need real version history). Genuinely a placeholder, not a fake: it
/// never silently drops a change, it always tells the caller version history isn't wired up.
#[derive(Clone, Copy, Default, Debug)]
pub struct NullVersionGraph;

impl VersionGraph for NullVersionGraph {
    fn record_change(&self, _document: &ArtifactId, _change: ChangeRecord) -> Result<String, DbError> {
        Err(DbError::Unimplemented("VersionGraph is not wired up (vcs feature disabled)"))
    }

    fn checkpoint(&self, _document: &ArtifactId, _request: CheckpointRequest) -> Result<String, DbError> {
        Err(DbError::Unimplemented("VersionGraph is not wired up (vcs feature disabled)"))
    }

    fn merge_base(&self, _document: &ArtifactId, _a: &str, _b: &str) -> Result<Option<String>, DbError> {
        Err(DbError::Unimplemented("VersionGraph is not wired up (vcs feature disabled)"))
    }

    fn head(&self, _document: &ArtifactId, _alternative: &str) -> Result<Option<String>, DbError> {
        Err(DbError::Unimplemented("VersionGraph is not wired up (vcs feature disabled)"))
    }
}
//#endregion 🔖️VersionGraph

//#region 🔖️Emit
/// @emoji 🏷️ One field attached to an `EmitEvent`, kept as a small closed set of primitive
/// shapes (no dynamic `Any`) so a sink can serialize/aggregate without reflection.
#[derive(Clone, Debug)]
pub enum EmitField {
    U64(u64),
    I64(i64),
    F64(f64),
    Bool(bool),
    Text(String),
}

/// @emoji 📣️ One observability event: a stable name plus an optional document scope and a small
/// bag of typed fields. `Emit::emit` takes this by value (not by reference) since a mailbox-
/// adjacent hot path may hand it across a thread boundary to a sink.
#[derive(Clone, Debug)]
pub struct EmitEvent {
    pub name: &'static str,
    pub document: Option<ArtifactId>,
    pub fields: Vec<(&'static str, EmitField)>,
}

impl EmitEvent {
    /// @emoji 🆕️ A bare event with `name` and no document/fields yet.
    pub fn new(name: &'static str) -> Self {
        Self { name, document: None, fields: Vec::new() }
    }

    /// @emoji 🪪️ Scopes the event to `document` (builder-style).
    pub fn with_document(mut self, document: ArtifactId) -> Self {
        self.document = Some(document);
        self
    }

    /// @emoji ➕️ Appends one field (builder-style).
    pub fn field(mut self, key: &'static str, value: EmitField) -> Self {
        self.fields.push((key, value));
        self
    }
}

/// @emoji 📡️ The observability seam: every `db_*` crate that wants to emit a metric/span/log
/// event takes `&dyn Emit` (or `Arc<dyn Emit>`) rather than depending on `db_observe` directly —
/// inverts the dependency so `db_core..db_cluster` stay `db_observe`-free while `db_observe`'s
/// real sinks (structured/audit JSON-lines, metric registries) implement this trait.
pub trait Emit: Send + Sync {
    fn emit(&self, event: EmitEvent);
}

/// @emoji 🔇️ An `Emit` that discards every event — the default when no observability sink is
/// configured, and a convenient no-op for tests that don't care about emitted events.
#[derive(Clone, Copy, Default, Debug)]
pub struct NullEmit;

impl Emit for NullEmit {
    fn emit(&self, _event: EmitEvent) {}
}
//#endregion 🔖️Emit

#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️VersionGraph
    #[test]
    fn null_version_graph_never_panics_always_reports_unimplemented() {
        let graph = NullVersionGraph;
        let document: ArtifactId = "doc-1".into();
        let change = ChangeRecord { parent: None, content_hash: pack::ContentHash([0u8; 32]), author: "actor-1".into(), message: "msg".to_string(), timestamp_ms: 0 };
        assert!(matches!(graph.record_change(&document, change), Err(DbError::Unimplemented(_))));

        let checkpoint = CheckpointRequest { parent_checkpoint: None, change_ids: vec![], message: "msg".to_string(), authors: vec![], timestamp_ms: 0 };
        assert!(matches!(graph.checkpoint(&document, checkpoint), Err(DbError::Unimplemented(_))));
        assert!(matches!(graph.merge_base(&document, "a", "b"), Err(DbError::Unimplemented(_))));
        assert!(matches!(graph.head(&document, "main"), Err(DbError::Unimplemented(_))));
    }

    #[test]
    fn version_graph_trait_object_is_dyn_compatible() {
        let graph: Box<dyn VersionGraph> = Box::new(NullVersionGraph);
        let document: ArtifactId = "doc-1".into();
        assert!(graph.head(&document, "main").is_err());
    }
    //#endregion 🔖️VersionGraph

    //#region 🔖️Emit
    struct RecordingEmit {
        events: std::sync::Mutex<Vec<EmitEvent>>,
    }

    impl Emit for RecordingEmit {
        fn emit(&self, event: EmitEvent) {
            self.events.lock().unwrap().push(event);
        }
    }

    #[test]
    fn emit_trait_object_records_events_with_fields_and_document() {
        let sink = RecordingEmit { events: std::sync::Mutex::new(Vec::new()) };
        let emit: &dyn Emit = &sink;
        emit.emit(EmitEvent::new("command.applied").with_document("doc-1".into()).field("bytes", EmitField::U64(128)).field("ok", EmitField::Bool(true)));
        let events = sink.events.lock().unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, "command.applied");
        assert_eq!(events[0].document, Some(ArtifactId::from("doc-1")));
        assert_eq!(events[0].fields.len(), 2);
    }

    #[test]
    fn null_emit_discards_without_panicking() {
        let emit = NullEmit;
        emit.emit(EmitEvent::new("noop"));
    }
    //#endregion 🔖️Emit
}
