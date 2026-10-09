//! 📸️ Persisted workflow run snapshot and codecs.
use crate::S_RUN_SCHEMA;
use semio_framework_value::{ValueError,ValueRefusalKind};

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
impl semio_framework_dsl_record::BorrowedDslField for RunStatus {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Enum(&[("pending", 0), ("running", 1), ("succeeded", 2), ("failed", 3), ("canceled", 4)]);
}

pub(crate) fn run_status_ordinal(status: RunStatus) -> u32 {
    match status {
        RunStatus::Pending => 0,
        RunStatus::Running => 1,
        RunStatus::Succeeded => 2,
        RunStatus::Failed => 3,
        RunStatus::Canceled => 4,
    }
}

fn run_status_from_ordinal(ordinal: u32) -> Result<RunStatus, ValueError> {
    Ok(match ordinal {
        0 => RunStatus::Pending,
        1 => RunStatus::Running,
        2 => RunStatus::Succeeded,
        3 => RunStatus::Failed,
        4 => RunStatus::Canceled,
        other => return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("unknown run status ordinal {other}"))),
    })
}

fn run_status_variants() -> Vec<(String, u32)> {
    vec![("pending".to_string(), 0), ("running".to_string(), 1), ("succeeded".to_string(), 2), ("failed".to_string(), 3), ("canceled".to_string(), 4)]
}

fn run_enum_shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(labels:&[(&str,u32)],control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{
    control.scoped_stage(|control|{control.begin_stage(labels.len())?;let mut values=control.allocate_vec::<(String,u32)>(labels.len())?;for(label,ordinal)in labels{values.push((control.copy_text(label)?,*ordinal));control.step()?;}Ok(semio_framework_dsl_record::Shape::Enum(values))})
}

impl semio_framework_dsl_record::DslField for RunStatus {
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{control.step()?;Ok(semio_framework_dsl_record::FieldValue::Enum(run_status_ordinal(*self)))}
    fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{control.step()?;match value{semio_framework_dsl_record::FieldValue::Enum(ordinal)=>run_status_from_ordinal(*ordinal),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected declared run status enum"))}}
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{run_enum_shape_controlled(&[("pending",0),("running",1),("succeeded",2),("failed",3),("canceled",4)],control)}
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Enum(run_status_variants())
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Enum(run_status_ordinal(*self))
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Enum(ordinal) => run_status_from_ordinal(*ordinal).map_err(ValueError::into_message),
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
impl semio_framework_dsl_record::BorrowedDslField for RunNodeStatus {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Enum(&[("computed", 0), ("cacheHit", 1), ("failed", 2)]);
}

pub(crate) fn run_node_status_ordinal(status: RunNodeStatus) -> u32 {
    match status {
        RunNodeStatus::Computed => 0,
        RunNodeStatus::CacheHit => 1,
        RunNodeStatus::Failed => 2,
    }
}

fn run_node_status_from_ordinal(ordinal: u32) -> Result<RunNodeStatus, ValueError> {
    Ok(match ordinal {
        0 => RunNodeStatus::Computed,
        1 => RunNodeStatus::CacheHit,
        2 => RunNodeStatus::Failed,
        other => return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("unknown run node status ordinal {other}"))),
    })
}

fn run_node_status_variants() -> Vec<(String, u32)> {
    vec![("computed".to_string(), 0), ("cacheHit".to_string(), 1), ("failed".to_string(), 2)]
}

impl semio_framework_dsl_record::DslField for RunNodeStatus {
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{control.step()?;Ok(semio_framework_dsl_record::FieldValue::Enum(run_node_status_ordinal(*self)))}
    fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{control.step()?;match value{semio_framework_dsl_record::FieldValue::Enum(ordinal)=>run_node_status_from_ordinal(*ordinal),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected declared run status enum"))}}
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{run_enum_shape_controlled(&[("computed",0),("cacheHit",1),("failed",2)],control)}
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Enum(run_node_status_variants())
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Enum(run_node_status_ordinal(*self))
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Enum(ordinal) => run_node_status_from_ordinal(*ordinal).map_err(ValueError::into_message),
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
impl semio_framework_dsl_record::BorrowedDslRecord for RunTrigger {
    const RECORD: semio_framework_dsl_record::BorrowedRecordSpec = semio_framework_dsl_record::BorrowedRecordSpec {
        keyword: None,
        layout: semio_framework_dsl_record::RecordLayout::Inline,
        fields: &[
            semio_framework_dsl_record::BorrowedFieldSpec::new(0, "kind", semio_framework_dsl_record::BorrowedShape::Text),
            semio_framework_dsl_record::BorrowedFieldSpec { optional: true, ..semio_framework_dsl_record::BorrowedFieldSpec::new(1, "actor", semio_framework_dsl_record::BorrowedShape::Text) },
            semio_framework_dsl_record::BorrowedFieldSpec { optional: true, ..semio_framework_dsl_record::BorrowedFieldSpec::new(2, "automation_ref", semio_framework_dsl_record::BorrowedShape::Text) },
            semio_framework_dsl_record::BorrowedFieldSpec { optional: true, ..semio_framework_dsl_record::BorrowedFieldSpec::new(3, "event_fingerprint", semio_framework_dsl_record::BorrowedShape::Text) },
        ],
    };
}
impl semio_framework_dsl_record::BorrowedDslField for RunTrigger {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>);
}

fn run_trigger_to_record_controlled(value:&RunTrigger,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,ValueError>{
 control.scoped_stage(|control|{control.begin_stage(4)?;let mut record=semio_framework_dsl_record::native_encoding::EncodedRecord::new(4,control)?;match value{
 RunTrigger::Manual{actor}=>{record.insert(0,semio_framework_dsl_record::FieldValue::Text(control.copy_text("manual")?))?;control.step()?;record.insert(1,semio_framework_dsl_record::FieldValue::Text(control.copy_text(actor)?))?;control.step()?;record.insert(2,semio_framework_dsl_record::FieldValue::Absent)?;control.step()?;record.insert(3,semio_framework_dsl_record::FieldValue::Absent)?;control.step()?;},
 RunTrigger::Automation{automation_ref,event_fingerprint}=>{record.insert(0,semio_framework_dsl_record::FieldValue::Text(control.copy_text("automation")?))?;control.step()?;record.insert(1,semio_framework_dsl_record::FieldValue::Absent)?;control.step()?;record.insert(2,semio_framework_dsl_record::FieldValue::Text(control.copy_text(automation_ref)?))?;control.step()?;record.insert(3,semio_framework_dsl_record::FieldValue::Text(control.copy_text(event_fingerprint)?))?;control.step()?;}
 }Ok(record.take())})
}
fn run_trigger_from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<RunTrigger,ValueError>{
 control.scoped_stage(|control|{control.begin_stage(4)?;control.step()?;let kind=match record.get(0){Some(semio_framework_dsl_record::FieldValue::Text(value))=>value.as_str(),_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected run trigger kind"))};
 let mut text=|id|->Result<Option<String>,ValueError>{let value=match record.get(id){None|Some(semio_framework_dsl_record::FieldValue::Absent)=>None,Some(value)=>Some(control.scoped_stage(|control|{control.begin_stage(0)?;<String as semio_framework_dsl_record::DslField>::from_value_controlled(value,control)})?)};control.step()?;Ok(value)};
 let actor=text(1)?;let automation_ref=text(2)?;let event_fingerprint=text(3)?;match(kind,actor,automation_ref,event_fingerprint){("manual",Some(actor),None,None)=>Ok(RunTrigger::Manual{actor}),("automation",None,Some(automation_ref),Some(event_fingerprint))=>Ok(RunTrigger::Automation{automation_ref,event_fingerprint}),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid run trigger fields"))}})
}

fn run_trigger_spec_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::RecordSpec,semio_framework_value::ValueError>{
    control.scoped_stage(|control|{
        control.begin_stage(4)?;let mut fields=control.allocate_vec::<semio_framework_dsl_record::FieldSpec>(4)?;
        fields.push(semio_framework_dsl_record::producer::field(0,"kind",semio_framework_dsl_record::Shape::Text,control)?);control.step()?;
        fields.push(semio_framework_dsl_record::producer::field(1,"actor",semio_framework_dsl_record::Shape::Text,control)?.optional());control.step()?;
        fields.push(semio_framework_dsl_record::producer::field(2,"automation_ref",semio_framework_dsl_record::Shape::Text,control)?.optional());control.step()?;
        fields.push(semio_framework_dsl_record::producer::field(3,"event_fingerprint",semio_framework_dsl_record::Shape::Text,control)?.optional());control.step()?;
        semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Inline,fields,control)
    })
}
fn run_trigger_spec_producer()->semio_framework_dsl_record::RecordSpecProducer{semio_framework_dsl_record::RecordSpecProducer{ordinary:run_trigger_spec,decoding:|control|run_trigger_spec_controlled(control),encoding:|control|run_trigger_spec_controlled(control)}}

fn run_trigger_spec() -> semio_framework_dsl_record::RecordSpec {
    semio_framework_dsl_record::RecordSpec::new(
        None,
        semio_framework_dsl_record::RecordLayout::Inline,
        vec![
            semio_framework_dsl_record::FieldSpec::new(0, "kind", semio_framework_dsl_record::Shape::Text),
            semio_framework_dsl_record::FieldSpec::new(1, "actor", semio_framework_dsl_record::Shape::Text).optional(),
            semio_framework_dsl_record::FieldSpec::new(2, "automation_ref", semio_framework_dsl_record::Shape::Text).optional(),
            semio_framework_dsl_record::FieldSpec::new(3, "event_fingerprint", semio_framework_dsl_record::Shape::Text).optional(),
        ],
    )
}

fn run_trigger_to_record(trigger: &RunTrigger) -> semio_framework_dsl_record::RecordValue {
    let mut record = semio_framework_dsl_record::RecordValue::default();
    match trigger {
        RunTrigger::Manual { actor } => {
            record.fields.insert(0, semio_framework_dsl_record::FieldValue::Text("manual".to_string()));
            record.fields.insert(1, semio_framework_dsl_record::FieldValue::Text(actor.clone()));
            record.fields.insert(2, semio_framework_dsl_record::FieldValue::Absent);
            record.fields.insert(3, semio_framework_dsl_record::FieldValue::Absent);
        }
        RunTrigger::Automation { automation_ref, event_fingerprint } => {
            record.fields.insert(0, semio_framework_dsl_record::FieldValue::Text("automation".to_string()));
            record.fields.insert(1, semio_framework_dsl_record::FieldValue::Absent);
            record.fields.insert(2, semio_framework_dsl_record::FieldValue::Text(automation_ref.clone()));
            record.fields.insert(3, semio_framework_dsl_record::FieldValue::Text(event_fingerprint.clone()));
        }
    }
    record
}

fn run_trigger_from_record(record: &semio_framework_dsl_record::RecordValue) -> Result<RunTrigger, semio_framework_diagnostic::TextError> {
    let kind = match record.get(0) {
        Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
        other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected kind, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
    };
    match kind.as_str() {
        "manual" => {
            let actor = match record.get(1) {
                Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
                other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected actor, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
            };
            Ok(RunTrigger::Manual { actor })
        }
        "automation" => {
            let automation_ref = match record.get(2) {
                Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
                other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected automation_ref, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
            };
            let event_fingerprint = match record.get(3) {
                Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
                other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected event_fingerprint, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
            };
            Ok(RunTrigger::Automation { automation_ref, event_fingerprint })
        }
        other => Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown run trigger kind '{other}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
    }
}

impl semio_framework_dsl_record::DslField for RunTrigger {
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{run_trigger_to_record_controlled(self,control).map(semio_framework_dsl_record::FieldValue::Record)}
    fn to_record_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,ValueError>{run_trigger_to_record_controlled(self,control)}
    fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;match value{semio_framework_dsl_record::FieldValue::Record(record)=>run_trigger_from_record_controlled(record,control),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected declared record"))}}
    fn from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{run_trigger_from_record_controlled(record,control)}
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(run_trigger_spec_producer())
    }
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{control.checkpoint()?;Ok(semio_framework_dsl_record::Shape::Record(run_trigger_spec_producer()))}
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Record(run_trigger_to_record(self))
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Record(record) => run_trigger_from_record(record).map_err(|e| e.message),
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
}

const fn run_borrowed_optional(mut field: semio_framework_dsl_record::BorrowedFieldSpec) -> semio_framework_dsl_record::BorrowedFieldSpec {
    field.optional = true;
    field
}

const RUN_STATUS_LABELS: &[(&str, u32)] = &[("pending", 0), ("running", 1), ("succeeded", 2), ("failed", 3), ("canceled", 4)];
const RUN_NODE_STATUS_LABELS: &[(&str, u32)] = &[("computed", 0), ("cacheHit", 1), ("failed", 2)];

impl semio_framework_dsl_record::BorrowedDslField for RunStatus {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Enum(RUN_STATUS_LABELS);
}

impl semio_framework_dsl_record::BorrowedDslField for RunNodeStatus {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Enum(RUN_NODE_STATUS_LABELS);
}

impl semio_framework_dsl_record::BorrowedDslRecord for RunTrigger {
    const RECORD: semio_framework_dsl_record::BorrowedRecordSpec = semio_framework_dsl_record::BorrowedRecordSpec {
        keyword: None,
        layout: semio_framework_dsl_record::RecordLayout::Inline,
        fields: &[
            semio_framework_dsl_record::BorrowedFieldSpec::new(0, "kind", semio_framework_dsl_record::BorrowedShape::Text),
            run_borrowed_optional(semio_framework_dsl_record::BorrowedFieldSpec::new(1, "actor", semio_framework_dsl_record::BorrowedShape::Text)),
            run_borrowed_optional(semio_framework_dsl_record::BorrowedFieldSpec::new(2, "automation_ref", semio_framework_dsl_record::BorrowedShape::Text)),
            run_borrowed_optional(semio_framework_dsl_record::BorrowedFieldSpec::new(3, "event_fingerprint", semio_framework_dsl_record::BorrowedShape::Text)),
        ],
    };
}

impl semio_framework_dsl_record::BorrowedDslField for RunTrigger {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>);
}

/// 🎛️ One resolved config-overlay value for a run — `value` carries a JSON-encoded scalar/text as
/// plain `Text` (not a raw `dsl::DslValue` field): a `dsl::DslValue` embeds arbitrary nested
/// object/array shapes, which risks not being self-delimiting as a bare `#[dsl(table)]` column (see
/// `dsl_schema`'s `table_rejects_non_self_delimiting_column_shapes_at_spec_build_time` regression) —
/// plain JSON text sidesteps that risk entirely while staying a lossless round trip. `run::SpaceRunner`
/// parses it back to `serde_json::Value` when applying the overlay onto a node's config (see
/// `WorkflowParameterBinding.field_path`).
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct RunParameterValue {
    pub parameter_id: String,
    pub value: String,
}

/// 🔑️ One port's fingerprint — reused for both a `RunNodeRecord`'s `input_fingerprints` and
/// `output_fingerprints` (same shape, different table column on the owning row).
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct PortFingerprint {
    pub port_id: String,
    pub fingerprint: String,
}

/// 📤️ Where one node's out-port materialized in the run's own write-only output area — `path` is
/// relative to the run's own sink (see `run::RunContext`'s doc), never a source-bundle path.
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact(id = "os.run")]
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




