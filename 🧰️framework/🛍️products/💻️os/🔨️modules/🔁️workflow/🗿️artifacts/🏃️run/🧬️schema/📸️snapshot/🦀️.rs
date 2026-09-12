//! 📸️ Persisted workflow run snapshot and codecs.
use crate::S_RUN_SCHEMA;

/// 🚦️ Lifecycle state of a whole run. `sealed` (on `RunArtifact`) is a distinct bool, not folded into
/// this enum — "sealed" and "final status" are orthogonal (a `Failed` run is sealed with `status:
/// Failed`, not a `Sealed` variant). Hand-crafted `dsl::DslField` (ordinal `Shape::Enum`), not
/// `#[derive(dsl::DslEnum)]`: this is a plain field-less scalar, not a tagged-variant-with-data sum
/// type (`DslEnum`/`DslVariants` target the latter — see `WorkflowParameter`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum RunStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
    Canceled,
}

fn run_status_ordinal(status: RunStatus) -> u32 {
    match status {
        RunStatus::Pending => 0,
        RunStatus::Running => 1,
        RunStatus::Succeeded => 2,
        RunStatus::Failed => 3,
        RunStatus::Canceled => 4,
    }
}

fn run_status_from_ordinal(ordinal: u32) -> Result<RunStatus, String> {
    Ok(match ordinal {
        0 => RunStatus::Pending,
        1 => RunStatus::Running,
        2 => RunStatus::Succeeded,
        3 => RunStatus::Failed,
        4 => RunStatus::Canceled,
        other => return Err(format!("unknown run status ordinal {other}")),
    })
}

fn run_status_variants() -> Vec<(String, u32)> {
    vec![("pending".to_string(), 0), ("running".to_string(), 1), ("succeeded".to_string(), 2), ("failed".to_string(), 3), ("canceled".to_string(), 4)]
}

impl dsl::DslField for RunStatus {
    fn shape() -> dsl::Shape {
        dsl::Shape::Enum(run_status_variants())
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Enum(run_status_ordinal(*self))
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Enum(ordinal) => run_status_from_ordinal(*ordinal),
            other => Err(format!("expected Enum, found {other:?}")),
        }
    }
}

/// 🚦️ Per-node outcome of one run — `Computed` (ran fresh), `CacheHit` (memoized against the prior
/// sealed run's `RunNodeRecord`), `Failed` (the node's `AppChannelHost` exchange errored).
#[derive(Clone, Copy, Debug, PartialEq, Eq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum RunNodeStatus {
    Computed,
    CacheHit,
    Failed,
}

fn run_node_status_ordinal(status: RunNodeStatus) -> u32 {
    match status {
        RunNodeStatus::Computed => 0,
        RunNodeStatus::CacheHit => 1,
        RunNodeStatus::Failed => 2,
    }
}

fn run_node_status_from_ordinal(ordinal: u32) -> Result<RunNodeStatus, String> {
    Ok(match ordinal {
        0 => RunNodeStatus::Computed,
        1 => RunNodeStatus::CacheHit,
        2 => RunNodeStatus::Failed,
        other => return Err(format!("unknown run node status ordinal {other}")),
    })
}

fn run_node_status_variants() -> Vec<(String, u32)> {
    vec![("computed".to_string(), 0), ("cacheHit".to_string(), 1), ("failed".to_string(), 2)]
}

impl dsl::DslField for RunNodeStatus {
    fn shape() -> dsl::Shape {
        dsl::Shape::Enum(run_node_status_variants())
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Enum(run_node_status_ordinal(*self))
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Enum(ordinal) => run_node_status_from_ordinal(*ordinal),
            other => Err(format!("expected Enum, found {other:?}")),
        }
    }
}

/// 🎬️ Who/what started a run — `Manual` (a dev/CLI invocation; `actor` mirrors `AppCommand::Hello`'s
/// own actor string) or `Automation` (W6's dispatcher, referencing the triggering `os.automation`
/// artifact + the event fingerprint that fired it — not built this wave, field carried for forward
/// compat only). Hand-crafted `dsl::DslField` (`Shape::Record`) mirroring `MediaContract`'s own
/// tag-plus-optional-fields encoding above — a real Rust sum type stays the API surface; the wire
/// encoding is just a `kind` discriminator text field plus each variant's own optional columns.
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum RunTrigger {
    Manual { actor: String },
    Automation { automation_ref: String, event_fingerprint: String },
}

fn run_trigger_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        None,
        dsl::RecordLayout::Inline,
        vec![
            dsl::FieldSpec::new(0, "kind", dsl::Shape::Text),
            dsl::FieldSpec::new(1, "actor", dsl::Shape::Text).optional(),
            dsl::FieldSpec::new(2, "automation_ref", dsl::Shape::Text).optional(),
            dsl::FieldSpec::new(3, "event_fingerprint", dsl::Shape::Text).optional(),
        ],
    )
}

fn run_trigger_to_record(trigger: &RunTrigger) -> dsl::RecordValue {
    let mut record = dsl::RecordValue::default();
    match trigger {
        RunTrigger::Manual { actor } => {
            record.fields.insert(0, dsl::FieldValue::Text("manual".to_string()));
            record.fields.insert(1, dsl::FieldValue::Text(actor.clone()));
            record.fields.insert(2, dsl::FieldValue::Absent);
            record.fields.insert(3, dsl::FieldValue::Absent);
        }
        RunTrigger::Automation { automation_ref, event_fingerprint } => {
            record.fields.insert(0, dsl::FieldValue::Text("automation".to_string()));
            record.fields.insert(1, dsl::FieldValue::Absent);
            record.fields.insert(2, dsl::FieldValue::Text(automation_ref.clone()));
            record.fields.insert(3, dsl::FieldValue::Text(event_fingerprint.clone()));
        }
    }
    record
}

fn run_trigger_from_record(record: &dsl::RecordValue) -> Result<RunTrigger, store::TextError> {
    let kind = match record.get(0) {
        Some(dsl::FieldValue::Text(s)) => s.clone(),
        other => return Err(dsl::__rt::field_error(format!("expected kind, found {other:?}"))),
    };
    match kind.as_str() {
        "manual" => {
            let actor = match record.get(1) {
                Some(dsl::FieldValue::Text(s)) => s.clone(),
                other => return Err(dsl::__rt::field_error(format!("expected actor, found {other:?}"))),
            };
            Ok(RunTrigger::Manual { actor })
        }
        "automation" => {
            let automation_ref = match record.get(2) {
                Some(dsl::FieldValue::Text(s)) => s.clone(),
                other => return Err(dsl::__rt::field_error(format!("expected automation_ref, found {other:?}"))),
            };
            let event_fingerprint = match record.get(3) {
                Some(dsl::FieldValue::Text(s)) => s.clone(),
                other => return Err(dsl::__rt::field_error(format!("expected event_fingerprint, found {other:?}"))),
            };
            Ok(RunTrigger::Automation { automation_ref, event_fingerprint })
        }
        other => Err(dsl::__rt::field_error(format!("unknown run trigger kind '{other}'"))),
    }
}

impl dsl::DslField for RunTrigger {
    fn shape() -> dsl::Shape {
        dsl::Shape::Record(run_trigger_spec)
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Record(run_trigger_to_record(self))
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Record(record) => run_trigger_from_record(record).map_err(|e| e.message),
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
}

/// 🎛️ One resolved config-overlay value for a run — `value` carries a JSON-encoded scalar/text as
/// plain `Text` (not a raw `dsl::DslValue` field): a `dsl::DslValue` embeds arbitrary nested
/// object/array shapes, which risks not being self-delimiting as a bare `#[dsl(table)]` column (see
/// `dsl_schema`'s `table_rejects_non_self_delimiting_column_shapes_at_spec_build_time` regression) —
/// plain JSON text sidesteps that risk entirely while staying a lossless round trip. `run::SpaceRunner`
/// parses it back to `serde_json::Value` when applying the overlay onto a node's config (see
/// `WorkflowParameterBinding.field_path`).
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct RunParameterValue {
    pub parameter_id: String,
    pub value: String,
}

/// 🔑️ One port's fingerprint — reused for both a `RunNodeRecord`'s `input_fingerprints` and
/// `output_fingerprints` (same shape, different table column on the owning row).
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct PortFingerprint {
    pub port_id: String,
    pub fingerprint: String,
}

/// 📤️ Where one node's out-port materialized in the run's own write-only output area — `path` is
/// relative to the run's own sink (see `run::RunContext`'s doc), never a source-bundle path.
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct RunOutputArtifact {
    pub port_id: String,
    pub artifact_id: String,
    pub path: String,
}

/// 📇️ Everything one run remembers about one workflow node — the `RunArtifact`-native replacement
/// for `run`'s old `NodeRunRecord`/`RunState` (deleted by W5 Lane A): memoization now compares
/// against the PRIOR sealed run's `node_records`, not a side-channel state file. `duration_ms` is
/// `f64` (not `u64`): the `dsl` engine's scalar `DslField` impls cover `bool`/`f32`/`f64`/`String`
/// only, no integer width — see `dsl/rs/lib.rs`'s `impl DslField for f64` and neighbors.
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct RunNodeRecord {
    pub node_id: String,
    pub status: RunNodeStatus,
    pub document_fingerprint: String,
    pub config_fingerprint: String,
    #[dsl(table)]
    pub input_fingerprints: Vec<PortFingerprint>,
    #[dsl(table)]
    pub output_fingerprints: Vec<PortFingerprint>,
    #[dsl(table)]
    pub outputs: Vec<RunOutputArtifact>,
    pub duration_ms: f64,
}

/// 📜️ One run-level or per-node log line — `node_id` empty for a run-level line (see `RunMutation::AppendRunLog`).
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct RunLogLine {
    pub node_id: String,
    pub level: String,
    pub message: String,
    pub at: String,
}

/// 🏃️ The `os.run` persisted artifact (W5 Lane A) — one headless workflow execution's full record:
/// which workflow/checkpoint/input snapshot it ran against, its resolved parameter overlay, where its
/// outputs landed, per-node `RunNodeRecord`s (the new memoization ground truth), and a `sealed` flag
/// that — once set by `RunMutation::SealRun` — makes the document immutable (`RunMutation::validate`
/// rejects every further operation, see `🔖️RunMutation` below). Sealing is meant to promote a run
/// draft→asset later (`space::DraftCatalog`, W5 Lane B's territory) — this wave only carries the flag
/// and the apply-rejection law, not the promotion wiring itself.
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, dsl::DslArtifact)]
#[dsl(id = "os.run")]
pub struct RunArtifact {
    pub schema: String,
    pub workflow_ref: String,
    pub workflow_checkpoint_id: String,
    pub input_collection_ref: String,
    pub input_snapshot_id: String,
    #[dsl(table)]
    pub parameter_values: Vec<RunParameterValue>,
    pub output_collection_ref: String,
    pub status: RunStatus,
    #[dsl(block)]
    pub trigger: RunTrigger,
    #[dsl(table)]
    pub node_records: Vec<RunNodeRecord>,
    #[dsl(table)]
    pub logs: Vec<RunLogLine>,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub sealed: bool,
}

pub async fn empty_run_document() -> RunArtifact {
    RunArtifact {
        schema: S_RUN_SCHEMA.into(),
        workflow_ref: String::new(),
        workflow_checkpoint_id: String::new(),
        input_collection_ref: String::new(),
        input_snapshot_id: String::new(),
        parameter_values: Vec::new(),
        output_collection_ref: String::new(),
        status: RunStatus::Pending,
        trigger: RunTrigger::Manual { actor: String::new() },
        node_records: Vec::new(),
        logs: Vec::new(),
        started_at: String::new(),
        finished_at: None,
        sealed: false,
    }
}

/// 🧬️ Encodes run snapshots using their domain schema and document envelope.
impl store::ArtifactDsl for RunArtifact {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for RunArtifact {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        match Self::__dsl_from_record(&record) {
            Ok(value) => Ok(value),
            Err(error) => Err(store::text_error_to_pack_error(error)),
        }
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
