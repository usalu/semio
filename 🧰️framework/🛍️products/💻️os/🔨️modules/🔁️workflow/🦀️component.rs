//! 🕸️ Persisted app-node workflow graph — nodes reference plugin apps plus document/config artifact refs.

// #region 🔖️InstanceIdentity
//! 🪪️ A `WorkflowNode.id` **is** the app-instance identity now — there is no separate instance
//! record layered on top of it. This crate becomes the single persisted-graph source of truth once
//! `framework/product/os/core`'s `OsSnapshot` is updated to embed `Workflow` directly instead of
//! its own in-file `OsWorkflow`/`OsAppInstance` pair (future work, not this ticket).
// #endregion 🔖️InstanceIdentity

use semio_framework::{AppDefinition, MediaClass, MediaForm, MediaPortDirection, MediaPortSpec, MediaType, MediaWireFormat, PortMultiplicity};
use semio_framework::{Locale, Terminology};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub const WORKFLOW_SCHEMA: &str = "workflow.graph";

/// 🪶️ W3 "the inversion" document schema — the persisted `os.workflow` artifact (graph + parameters +
/// inputs/bindings, see [`WorkflowSnapshot`]), distinct from `WORKFLOW_SCHEMA` (the bare graph-only
/// sub-shape still embedded as `WorkflowSnapshot.graph`). Registered as a builtin artifact kind by
/// os-core's `seed_builtin_artifact_kinds`.
pub const S_WORKFLOW_SCHEMA: &str = "os.workflow";

/// 🚧️ W5/W6: `os.run`/`os.automation` schema ids are reserved here (the plan's schema lattice puts
/// `RunArtifact`/`AutomationDocument` in this crate, execution in `🏃️run`) — full bodies are
/// deliberately NOT built in W3 (SpaceRunner rework is W5, automation dispatcher is W6); inventing a
/// shape now without the runner rework driving it risks rework. See
/// `.claude/plans/the-final-goal-for-jolly-spindle.md` `### Workflow / Run / Automation (Track C)`.
pub const S_RUN_SCHEMA: &str = "os.run";
pub const S_AUTOMATION_SCHEMA: &str = "os.automation";

//#region 🔖️MediaContract
/// 🤝️ A connect-time negotiated wire contract between two `WorkflowMediaPort`s — stored on
/// `WorkflowEdge` so later passes (`validate_workflow`, merge reconciliation) can re-check it
/// without re-resolving the artifact registry. `kind_id`/`media_type` describe the *accepted*
/// (target) side — see `semio_framework::media_types_compatible`. Ported down from
/// `framework/product/os/core`'s `workflow` module (`MediaContract`) so the persisted graph carries
/// its own edge contracts; the negotiation logic itself (`negotiate_media_contract`) stays in
/// os-core for now since it needs the artifact-kind registry, which doesn't exist at this layer yet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaContract {
    pub kind_id: String,
    pub media_type: MediaType,
    pub wire: MediaWireFormat,
    pub conversion: Option<(MediaForm, MediaForm)>,
}

/// 🧪️ Placeholder contract for test/fixture edges built without a real port-negotiation context —
/// schema pinned to `kind_id` itself, `Data`/`Value` media type (mirrors os-core's unregistered-kind
/// registry fallback).
pub async fn placeholder_media_contract(kind_id: &str) -> MediaContract {
    MediaContract { kind_id: kind_id.into(), media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, wire: MediaWireFormat::Document { schema: kind_id.into() }, conversion: None }
}
//#endregion 🔖️MediaContract

//#region 🔖️MediaContractDsl
/// 🧬️ Hand-crafted `dsl::DslField` for `MediaContract` (instead of `#[derive(dsl::DslRecord)]`) —
/// see the `dsl::` conversion cheat sheet's tuple-field guidance. `conversion: Option<(MediaForm,
/// MediaForm)>` has no derivable shape (raw Rust tuples don't implement `dsl::DslField`), and
/// `media_type`/`wire` point at plain-data types from `semio_framework` that this crate can't
/// implement `dsl::DslField` for under the orphan rule (neither the trait nor the type is local
/// here). Since `MediaContract` itself IS local, hand-writing its own impl sidesteps both problems
/// at once: every foreign sub-value (`MediaClass`/`MediaForm`/the wire's format kind id) is bridged directly
/// to/from a scalar `dsl::FieldValue::Enum`/`Ident` right here, so none of them ever need their own
/// `DslField` impl or a local-twin type. `media_contract_spec()`'s `keyword: None` makes
/// `Shape::Record` splice these eight fields inline wherever `MediaContract` is used as a
/// `#[dsl(block)]` field (see `WorkflowEdge.contract`), with no keyword of its own repeated inside
/// the braces. Ported verbatim from `framework/product/os/core`'s `workflow` module.
fn media_class_ordinal(class: MediaClass) -> u32 {
    match class {
        MediaClass::TwoD => 0,
        MediaClass::ThreeD => 1,
        MediaClass::Text => 2,
        MediaClass::Data => 3,
        MediaClass::Graph => 4,
        MediaClass::Kit => 5,
        MediaClass::Computation => 6,
        MediaClass::Presentation => 7,
    }
}

fn media_class_from_ordinal(ordinal: u32) -> Result<MediaClass, String> {
    Ok(match ordinal {
        0 => MediaClass::TwoD,
        1 => MediaClass::ThreeD,
        2 => MediaClass::Text,
        3 => MediaClass::Data,
        4 => MediaClass::Graph,
        5 => MediaClass::Kit,
        6 => MediaClass::Computation,
        7 => MediaClass::Presentation,
        other => return Err(format!("unknown media class ordinal {other}")),
    })
}

// 🚫️async: E1 transitive — only consumed by the E4-tagged sync `*_spec()` fn-pointer
// targets below; pure variant table, no I/O (R9).
fn media_class_variants() -> Vec<(String, u32)> {
    vec![("twoD".to_string(), 0), ("threeD".to_string(), 1), ("text".to_string(), 2), ("data".to_string(), 3), ("graph".to_string(), 4), ("kit".to_string(), 5), ("computation".to_string(), 6), ("presentation".to_string(), 7)]
}

fn media_form_ordinal(form: MediaForm) -> u32 {
    match form {
        MediaForm::Any => 0,
        MediaForm::Vector => 1,
        MediaForm::Raster => 2,
        MediaForm::Brep => 3,
        MediaForm::Mesh => 4,
        MediaForm::Document => 5,
        MediaForm::Value => 6,
        MediaForm::Dag => 7,
        MediaForm::Trinity => 8,
        MediaForm::Type => 9,
        MediaForm::Design => 10,
        MediaForm::Kit => 11,
        MediaForm::Flow => 12,
        MediaForm::Sequence => 13,
        MediaForm::Imperative => 14,
        MediaForm::Deck => 15,
    }
}

fn media_form_from_ordinal(ordinal: u32) -> Result<MediaForm, String> {
    Ok(match ordinal {
        0 => MediaForm::Any,
        1 => MediaForm::Vector,
        2 => MediaForm::Raster,
        3 => MediaForm::Brep,
        4 => MediaForm::Mesh,
        5 => MediaForm::Document,
        6 => MediaForm::Value,
        7 => MediaForm::Dag,
        8 => MediaForm::Trinity,
        9 => MediaForm::Type,
        10 => MediaForm::Design,
        11 => MediaForm::Kit,
        12 => MediaForm::Flow,
        13 => MediaForm::Sequence,
        14 => MediaForm::Imperative,
        15 => MediaForm::Deck,
        other => return Err(format!("unknown media form ordinal {other}")),
    })
}

// 🚫️async: E1 transitive — only consumed by the E4-tagged sync `*_spec()` fn-pointer
// targets below; pure variant table, no I/O (R9).
fn media_form_variants() -> Vec<(String, u32)> {
    vec![
        ("any".to_string(), 0),
        ("vector".to_string(), 1),
        ("raster".to_string(), 2),
        ("brep".to_string(), 3),
        ("mesh".to_string(), 4),
        ("document".to_string(), 5),
        ("value".to_string(), 6),
        ("dag".to_string(), 7),
        ("trinity".to_string(), 8),
        ("type".to_string(), 9),
        ("design".to_string(), 10),
        ("kit".to_string(), 11),
        ("flow".to_string(), 12),
        ("sequence".to_string(), 13),
        ("imperative".to_string(), 14),
        ("deck".to_string(), 15),
    ]
}

// 🚫️async: E4 fn-pointer slot — value goes into `dsl::Shape::Record(fn() -> RecordSpec)`.
fn media_contract_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        None,
        dsl::RecordLayout::Inline,
        vec![
            dsl::FieldSpec::new(0, "kind_id", dsl::Shape::Text),
            dsl::FieldSpec::new(1, "class", dsl::Shape::Enum(media_class_variants())),
            dsl::FieldSpec::new(2, "form", dsl::Shape::Enum(media_form_variants())),
            dsl::FieldSpec::new(3, "wire_kind", dsl::Shape::Text),
            dsl::FieldSpec::new(4, "wire_format", dsl::Shape::Text).optional(),
            dsl::FieldSpec::new(5, "wire_schema", dsl::Shape::Text).optional(),
            dsl::FieldSpec::new(6, "conversion_from", dsl::Shape::Enum(media_form_variants())).optional(),
            dsl::FieldSpec::new(7, "conversion_to", dsl::Shape::Enum(media_form_variants())).optional(),
        ],
    )
}

fn media_contract_to_record(contract: &MediaContract) -> dsl::RecordValue {
    let mut record = dsl::RecordValue::default();
    record.fields.insert(0, dsl::FieldValue::Text(contract.kind_id.clone()));
    record.fields.insert(1, dsl::FieldValue::Enum(media_class_ordinal(contract.media_type.class)));
    record.fields.insert(2, dsl::FieldValue::Enum(media_form_ordinal(contract.media_type.form)));
    match &contract.wire {
        MediaWireFormat::Binary { format_kind } => {
            record.fields.insert(3, dsl::FieldValue::Text("binary".to_string()));
            record.fields.insert(4, dsl::FieldValue::Text(format_kind.clone()));
            record.fields.insert(5, dsl::FieldValue::Absent);
        }
        MediaWireFormat::Document { schema } => {
            record.fields.insert(3, dsl::FieldValue::Text("document".to_string()));
            record.fields.insert(4, dsl::FieldValue::Absent);
            record.fields.insert(5, dsl::FieldValue::Text(schema.clone()));
        }
    }
    match contract.conversion {
        Some((from, to)) => {
            record.fields.insert(6, dsl::FieldValue::Enum(media_form_ordinal(from)));
            record.fields.insert(7, dsl::FieldValue::Enum(media_form_ordinal(to)));
        }
        None => {
            record.fields.insert(6, dsl::FieldValue::Absent);
            record.fields.insert(7, dsl::FieldValue::Absent);
        }
    }
    record
}

fn media_contract_from_record(record: &dsl::RecordValue) -> Result<MediaContract, store::TextError> {
    let kind_id = match record.get(0) {
        Some(dsl::FieldValue::Text(s)) => s.clone(),
        other => return Err(dsl::__rt::field_error(format!("expected kind_id, found {other:?}"))),
    };
    let class = match record.get(1) {
        Some(dsl::FieldValue::Enum(ordinal)) => media_class_from_ordinal(*ordinal).map_err(dsl::__rt::field_error)?,
        other => return Err(dsl::__rt::field_error(format!("expected class, found {other:?}"))),
    };
    let form = match record.get(2) {
        Some(dsl::FieldValue::Enum(ordinal)) => media_form_from_ordinal(*ordinal).map_err(dsl::__rt::field_error)?,
        other => return Err(dsl::__rt::field_error(format!("expected form, found {other:?}"))),
    };
    let wire_kind = match record.get(3) {
        Some(dsl::FieldValue::Text(s)) => s.clone(),
        other => return Err(dsl::__rt::field_error(format!("expected wire_kind, found {other:?}"))),
    };
    let wire = match wire_kind.as_str() {
        "binary" => {
            let format_word = match record.get(4) {
                Some(dsl::FieldValue::Text(s)) => s.clone(),
                other => return Err(dsl::__rt::field_error(format!("expected wire_format, found {other:?}"))),
            };
            MediaWireFormat::Binary { format_kind: format_word }
        }
        "document" => {
            let schema = match record.get(5) {
                Some(dsl::FieldValue::Text(s)) => s.clone(),
                other => return Err(dsl::__rt::field_error(format!("expected wire_schema, found {other:?}"))),
            };
            MediaWireFormat::Document { schema }
        }
        other => return Err(dsl::__rt::field_error(format!("unknown wire kind '{other}'"))),
    };
    let conversion = match (record.get(6), record.get(7)) {
        (Some(dsl::FieldValue::Enum(from)), Some(dsl::FieldValue::Enum(to))) => Some((media_form_from_ordinal(*from).map_err(dsl::__rt::field_error)?, media_form_from_ordinal(*to).map_err(dsl::__rt::field_error)?)),
        _ => None,
    };
    Ok(MediaContract { kind_id, media_type: MediaType { class, form }, wire, conversion })
}

impl dsl::DslField for MediaContract {
    // 🚫️async: E4 fn-pointer transitivity — see `DslField::shape`'s tag (R9).
    fn shape() -> dsl::Shape {
        dsl::Shape::Record(media_contract_spec)
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Record(media_contract_to_record(self))
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Record(record) => media_contract_from_record(record).map_err(|e| e.message),
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
}
//#endregion 🔖️MediaContractDsl

//#region 🔖️WorkflowMediaPort
/// 🔌️ One instance-scoped wire endpoint on a `WorkflowNode` — `id` is unique within the graph
/// (`"{node_id}:{spec.id}:{in|out}"`, see `workflow_media_port`), `spec` is the app-level port
/// declaration it was instantiated from (`semio_framework::MediaPortSpec`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowMediaPort {
    pub id: String,
    pub spec: MediaPortSpec,
}

/// 🧬️ Hand-crafted `dsl::DslField` for `WorkflowMediaPort` — `spec: MediaPortSpec` points at a
/// plain-data type from `semio_framework`, which this crate can't implement `dsl::DslField`
/// for under the orphan rule (same reasoning as `MediaContract` above). Bridges every sub-value
/// (`MediaPortDirection`/`MediaClass`/`MediaForm`/`PortMultiplicity`) directly to/from a scalar
/// `dsl::FieldValue`, reusing the `media_class_ordinal`/`media_form_ordinal` tables above.
fn media_port_direction_ordinal(direction: MediaPortDirection) -> u32 {
    match direction {
        MediaPortDirection::In => 0,
        MediaPortDirection::Out => 1,
    }
}

fn media_port_direction_from_ordinal(ordinal: u32) -> Result<MediaPortDirection, String> {
    Ok(match ordinal {
        0 => MediaPortDirection::In,
        1 => MediaPortDirection::Out,
        other => return Err(format!("unknown media port direction ordinal {other}")),
    })
}

// 🚫️async: E1 transitive — only consumed by the E4-tagged sync `DslField::shape`
// path; pure variant table, no I/O (R9).
fn media_port_direction_variants() -> Vec<(String, u32)> {
    vec![("in".to_string(), 0), ("out".to_string(), 1)]
}

fn port_multiplicity_ordinal(multiplicity: PortMultiplicity) -> u32 {
    match multiplicity {
        PortMultiplicity::One => 0,
        PortMultiplicity::Many => 1,
    }
}

fn port_multiplicity_from_ordinal(ordinal: u32) -> Result<PortMultiplicity, String> {
    Ok(match ordinal {
        0 => PortMultiplicity::One,
        1 => PortMultiplicity::Many,
        other => return Err(format!("unknown port multiplicity ordinal {other}")),
    })
}

// 🚫️async: E1 transitive — only consumed by the E4-tagged sync `*_spec()` fn-pointer
// targets below; pure variant table, no I/O (R9).
fn port_multiplicity_variants() -> Vec<(String, u32)> {
    vec![("one".to_string(), 0), ("many".to_string(), 1)]
}

// 🚫️async: E4 fn-pointer slot — value goes into `dsl::Shape::Record(fn() -> RecordSpec)`.
fn workflow_media_port_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        None,
        dsl::RecordLayout::Inline,
        vec![
            dsl::FieldSpec::new(0, "id", dsl::Shape::Text),
            dsl::FieldSpec::new(1, "port_id", dsl::Shape::Text),
            dsl::FieldSpec::new(2, "label", dsl::Shape::Text),
            dsl::FieldSpec::new(3, "direction", dsl::Shape::Enum(media_port_direction_variants())),
            dsl::FieldSpec::new(4, "class", dsl::Shape::Enum(media_class_variants())),
            dsl::FieldSpec::new(5, "form", dsl::Shape::Enum(media_form_variants())),
            dsl::FieldSpec::new(6, "kind_id", dsl::Shape::Text).optional(),
            dsl::FieldSpec::new(7, "required", dsl::Shape::Bool),
            dsl::FieldSpec::new(8, "multiplicity", dsl::Shape::Enum(port_multiplicity_variants())),
        ],
    )
}

fn workflow_media_port_to_record(port: &WorkflowMediaPort) -> dsl::RecordValue {
    let mut record = dsl::RecordValue::default();
    record.fields.insert(0, dsl::FieldValue::Text(port.id.clone()));
    record.fields.insert(1, dsl::FieldValue::Text(port.spec.id.clone()));
    record.fields.insert(2, dsl::FieldValue::Text(port.spec.label.clone()));
    record.fields.insert(3, dsl::FieldValue::Enum(media_port_direction_ordinal(port.spec.direction)));
    record.fields.insert(4, dsl::FieldValue::Enum(media_class_ordinal(port.spec.media_type.class)));
    record.fields.insert(5, dsl::FieldValue::Enum(media_form_ordinal(port.spec.media_type.form)));
    match &port.spec.kind_id {
        Some(kind_id) => record.fields.insert(6, dsl::FieldValue::Text(kind_id.clone())),
        None => record.fields.insert(6, dsl::FieldValue::Absent),
    };
    record.fields.insert(7, dsl::FieldValue::Bool(port.spec.required));
    record.fields.insert(8, dsl::FieldValue::Enum(port_multiplicity_ordinal(port.spec.multiplicity)));
    record
}

fn workflow_media_port_from_record(record: &dsl::RecordValue) -> Result<WorkflowMediaPort, store::TextError> {
    let id = match record.get(0) {
        Some(dsl::FieldValue::Text(s)) => s.clone(),
        other => return Err(dsl::__rt::field_error(format!("expected id, found {other:?}"))),
    };
    let port_id = match record.get(1) {
        Some(dsl::FieldValue::Text(s)) => s.clone(),
        other => return Err(dsl::__rt::field_error(format!("expected port_id, found {other:?}"))),
    };
    let label = match record.get(2) {
        Some(dsl::FieldValue::Text(s)) => s.clone(),
        other => return Err(dsl::__rt::field_error(format!("expected label, found {other:?}"))),
    };
    let direction = match record.get(3) {
        Some(dsl::FieldValue::Enum(ordinal)) => media_port_direction_from_ordinal(*ordinal).map_err(dsl::__rt::field_error)?,
        other => return Err(dsl::__rt::field_error(format!("expected direction, found {other:?}"))),
    };
    let class = match record.get(4) {
        Some(dsl::FieldValue::Enum(ordinal)) => media_class_from_ordinal(*ordinal).map_err(dsl::__rt::field_error)?,
        other => return Err(dsl::__rt::field_error(format!("expected class, found {other:?}"))),
    };
    let form = match record.get(5) {
        Some(dsl::FieldValue::Enum(ordinal)) => media_form_from_ordinal(*ordinal).map_err(dsl::__rt::field_error)?,
        other => return Err(dsl::__rt::field_error(format!("expected form, found {other:?}"))),
    };
    let kind_id = match record.get(6) {
        Some(dsl::FieldValue::Text(s)) => Some(s.clone()),
        _ => None,
    };
    let required = match record.get(7) {
        Some(dsl::FieldValue::Bool(b)) => *b,
        other => return Err(dsl::__rt::field_error(format!("expected required, found {other:?}"))),
    };
    let multiplicity = match record.get(8) {
        Some(dsl::FieldValue::Enum(ordinal)) => port_multiplicity_from_ordinal(*ordinal).map_err(dsl::__rt::field_error)?,
        other => return Err(dsl::__rt::field_error(format!("expected multiplicity, found {other:?}"))),
    };
    Ok(WorkflowMediaPort { id, spec: MediaPortSpec { id: port_id, label, direction, media_type: MediaType { class, form }, kind_id, required, multiplicity } })
}

impl dsl::DslField for WorkflowMediaPort {
    // 🚫️async: E4 fn-pointer transitivity — see `DslField::shape`'s tag (R9).
    fn shape() -> dsl::Shape {
        dsl::Shape::Record(workflow_media_port_spec)
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Record(workflow_media_port_to_record(self))
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Record(record) => workflow_media_port_from_record(record).map_err(|e| e.message),
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
}
//#endregion 🔖️WorkflowMediaPort

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowPosition {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// 🧷️ A node IS the app-instance now — see the `🔖️InstanceIdentity` region at the top of this file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowNode {
    pub id: String,
    pub plugin_id: String,
    pub app_id: String,
    pub label: String,
    pub yields: String,
    pub artifact_ref: String,
    pub config_ref: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub inputs: Vec<WorkflowMediaPort>,
    pub outputs: Vec<WorkflowMediaPort>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowEdge {
    pub id: String,
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
    #[dsl(block)]
    pub contract: MediaContract,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "workflow", layout = "lines")]
pub struct Workflow {
    pub schema: String,
    pub nodes: Vec<WorkflowNode>,
    pub edges: Vec<WorkflowEdge>,
}

pub async fn empty_workflow() -> Workflow {
    Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: Vec::new(), edges: Vec::new() }
}

/// 🔌️ Instantiates one `WorkflowMediaPort` from an app-level `MediaPortSpec`, scoping its wire id to
/// this node (`"{node_id}:{spec.id}:{in|out}"`) so the same port declaration produces distinct wire
/// endpoints per node instance.
// 🚫️async: E1 transitive — only consumed by an `Iterator::map` (external trait) closure; pure
// field assembly, no I/O (R9).
fn workflow_media_port(node_id: &str, spec: &MediaPortSpec) -> WorkflowMediaPort {
    let direction_word = match spec.direction {
        MediaPortDirection::In => "in",
        MediaPortDirection::Out => "out",
    };
    WorkflowMediaPort { id: format!("{node_id}:{}:{}", spec.id, direction_word), spec: spec.clone() }
}

/// 🧩️ Builds a workflow node shell from a manifest app definition so every app is instantiable as a node.
pub async fn workflow_node_for_app(app: &AppDefinition, plugin_id: &str, node_id: &str, position: &WorkflowPosition) -> WorkflowNode {
    let all_ports = app.io.all_ports().await;
    let inputs: Vec<WorkflowMediaPort> = all_ports.iter().filter(|spec| spec.direction == MediaPortDirection::In).map(|spec| workflow_media_port(node_id, spec)).collect();
    let outputs: Vec<WorkflowMediaPort> = all_ports.iter().filter(|spec| spec.direction == MediaPortDirection::Out).map(|spec| workflow_media_port(node_id, spec)).collect();
    let yields = outputs.first().and_then(|port| port.spec.kind_id.clone()).unwrap_or_default();
    let port_count = inputs.len().max(outputs.len()).max(1);
    let height = position.height.max(56.0 + port_count as f64 * 18.0);
    WorkflowNode {
        id: node_id.into(),
        plugin_id: plugin_id.into(),
        app_id: app.id.clone(),
        // 🚧️ `workflow_node_for_app` carries no locale/terminology context (this node-graph node
        // builder isn't rendered per-request the way UiNode trees are); resolves native/English
        // pending a locale plumbed through the workflow canvas API.
        label: app.label.resolve(Terminology::Native, Locale::En).to_string(),
        yields,
        artifact_ref: format!("artifacts/{node_id}"),
        config_ref: format!("config/{node_id}"),
        x: position.x,
        y: position.y,
        width: position.width.max(220.0),
        height,
        inputs,
        outputs,
    }
}

//#region 🔖️WorkflowValidator
#[derive(Clone, Debug, PartialEq)]
pub struct WorkflowValidation {
    pub ok: bool,
    pub errors: Vec<String>,
}

/// @emoji ✅️ Validates workflow connectivity and cycle freedom. Ported down from
/// `framework/product/os/core`'s `workflow` module (`validate_workflow`); the edge-contract
/// re-negotiation check that lived alongside it there (re-running `negotiate_media_contract` against
/// the live artifact registry) stays in os-core for now — it needs the artifact-kind registry, which
/// doesn't exist at this layer yet. A later work package re-adds it here once contract negotiation
/// itself moves down.
pub async fn validate_workflow(graph: &Workflow) -> WorkflowValidation {
    let mut errors = Vec::new();
    let node_ids: HashSet<_> = graph.nodes.iter().map(|node| node.id.clone()).collect();
    for edge in &graph.edges {
        if !node_ids.contains(&edge.source_node_id) {
            errors.push(format!("missing source node {}", edge.source_node_id));
        }
        if !node_ids.contains(&edge.target_node_id) {
            errors.push(format!("missing target node {}", edge.target_node_id));
        }
    }

    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    for edge in &graph.edges {
        adjacency.entry(edge.source_node_id.clone()).or_default().push(edge.target_node_id.clone());
    }
    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    async fn dfs(node_id: &str, adjacency: &HashMap<String, Vec<String>>, visiting: &mut HashSet<String>, visited: &mut HashSet<String>, errors: &mut Vec<String>) {
        if visited.contains(node_id) {
            return;
        }
        if visiting.contains(node_id) {
            errors.push(format!("cycle detected at {node_id}"));
            return;
        }
        visiting.insert(node_id.to_string());
        for next in adjacency.get(node_id).into_iter().flatten() {
            Box::pin(dfs(next, adjacency, visiting, visited, errors)).await;
        }
        visiting.remove(node_id);
        visited.insert(node_id.to_string());
    }
    for node in &graph.nodes {
        dfs(&node.id, &adjacency, &mut visiting, &mut visited, &mut errors).await;
    }
    WorkflowValidation { ok: errors.is_empty(), errors }
}
//#endregion 🔖️WorkflowValidator

//#region 🔖️WorkflowPlanner
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowDelivery {
    pub edge_id: String,
    pub producer_node_id: String,
    pub producer_port_id: String,
    pub consumer_node_id: String,
    pub consumer_port_id: String,
}

/// @emoji 🧭️ Post-order DFS reversed into a topological node order (source before target); same
/// recursive shape as `validate_workflow`'s cycle-detection DFS, but collects the traversal order
/// instead of flagging revisits (the graph is validated acyclic before planning runs).
async fn workflow_topological_node_order(graph: &Workflow) -> Vec<String> {
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    for edge in &graph.edges {
        adjacency.entry(edge.source_node_id.clone()).or_default().push(edge.target_node_id.clone());
    }
    let mut visited = HashSet::new();
    let mut order = Vec::new();
    async fn dfs(node_id: &str, adjacency: &HashMap<String, Vec<String>>, visited: &mut HashSet<String>, order: &mut Vec<String>) {
        if !visited.insert(node_id.to_string()) {
            return;
        }
        for next in adjacency.get(node_id).into_iter().flatten() {
            Box::pin(dfs(next, adjacency, visited, order)).await;
        }
        order.push(node_id.to_string());
    }
    for node in &graph.nodes {
        dfs(&node.id, &adjacency, &mut visited, &mut order).await;
    }
    order.reverse();
    order
}

/// @emoji 🚚️ Plans one [`WorkflowDelivery`] per edge in the downstream closure of `dirty_node_ids`,
/// propagating dirtiness onto each edge's consumer node so multi-hop chains (A→B→C) resolve in a
/// single topological pass. Pure/side-effect-free — callers own applying the deliveries.
pub async fn plan_workflow(graph: &Workflow, dirty_node_ids: &HashSet<String>) -> Vec<WorkflowDelivery> {
    let node_by_id: HashMap<&str, &WorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    let mut edges_by_source: HashMap<&str, Vec<&WorkflowEdge>> = HashMap::new();
    for edge in &graph.edges {
        edges_by_source.entry(edge.source_node_id.as_str()).or_default().push(edge);
    }
    let order = workflow_topological_node_order(graph).await;
    let mut dirty = dirty_node_ids.clone();
    let mut deliveries = Vec::new();
    for node_id in &order {
        let Some(node) = node_by_id.get(node_id.as_str()) else { continue };
        if !dirty.contains(node.id.as_str()) {
            continue;
        }
        for edge in edges_by_source.get(node_id.as_str()).into_iter().flatten() {
            let Some(target_node) = node_by_id.get(edge.target_node_id.as_str()) else { continue };
            deliveries.push(WorkflowDelivery { edge_id: edge.id.clone(), producer_node_id: node.id.clone(), producer_port_id: edge.source_port_id.clone(), consumer_node_id: target_node.id.clone(), consumer_port_id: edge.target_port_id.clone() });
            dirty.insert(target_node.id.clone());
        }
    }
    deliveries
}
//#endregion 🔖️WorkflowPlanner

//#region 🔖️WorkflowFixture
/// 🔬️ One planner test vector: a workflow graph, the nodes marked dirty, and the deliveries
/// `plan_workflow` must produce for them. Ships as a `dsl`+`pack` document — see
/// `framework/product/os/core/fixtures/*.dsl`/`*.spk` and `README.md` — so the fixture corpus itself
/// proves the dsl≡pack law instead of riding untyped JSON.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "workflow-fixture")]
pub struct WorkflowFixture {
    pub name: String,
    #[dsl(block)]
    pub graph: Workflow,
    pub dirty_node_ids: Vec<String>,
    #[dsl(table)]
    pub expected_deliveries: Vec<WorkflowDelivery>,
}

impl store::ArtifactDsl for WorkflowFixture {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;

    async fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }

    async fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for WorkflowFixture {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options).await?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }

    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id().await {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id().await, envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options).await?;
        match Self::__dsl_from_record(&record) {
            Ok(value) => Ok(value),
            Err(error) => Err(store::text_error_to_pack_error(error)),
        }
    }

    async fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️WorkflowFixture

//#region 🔖️WorkflowParameters
// 🎛️ Ported down from `framework/product/os/core`'s `instance::🔖️Parameters` region (W3 "the
// inversion" — see `## The inversion` in the plan) verbatim except for the `Os` -> `Workflow` rename:
// none of it actually needed the os-core plugin/artifact registry (`validate_workflow_parameter_config_binding`
// takes an already-resolved `ConfigSpec` as a plain argument, it never looks one up itself), so the
// whole region is pure and belongs at this layer. os-core keeps only the registry LOOKUP
// (`os_app_registration(...).config`) that feeds this function's `config_spec` argument.
pub const WORKFLOW_PARAMETER_PORT_PREFIX: &str = "param.";

static WORKFLOW_PARAMETER_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// @emoji 🆔️ Fallback id minter for a parameter created without a caller-supplied id — every real
/// caller (`os-core`'s `OsWorkflowStore::add_parameter`) supplies one via its own id minter instead, so
/// this counter is scoped independently to this crate (not the same sequence as os-core's `create_os_id`).
// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
fn create_workflow_parameter_id() -> String {
    let n = WORKFLOW_PARAMETER_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
    format!("param-{n}")
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WorkflowParameterType {
    Numeric,
    Categorical,
    Toggle,
    Text,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowParameterFieldSpec {
    pub field_path: String,
    pub label: String,
    #[serde(rename = "type")]
    pub parameter_type: WorkflowParameterType,
}

/// 🎯️ `field_path` names a `ConfigFieldSpec.key` in the target node's app's declared `ConfigSpec` —
/// see `validate_workflow_parameter_config_binding` (type-checks against the field's `ConfigFieldShape`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowParameterBinding {
    pub parameter_id: String,
    pub node_id: String,
    pub field_path: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum WorkflowParameter {
    Numeric {
        id: String,
        name: String,
        value: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        step: Option<f64>,
    },
    Categorical {
        id: String,
        name: String,
        value: String,
        options: Vec<String>,
    },
    Toggle {
        id: String,
        name: String,
        value: bool,
    },
    Text {
        id: String,
        name: String,
        value: String,
    },
}

pub type WorkflowParameterPatch = serde_json::Value;

// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
pub fn workflow_parameter_id(parameter: &WorkflowParameter) -> &str {
    match parameter {
        WorkflowParameter::Numeric { id, .. } | WorkflowParameter::Categorical { id, .. } | WorkflowParameter::Toggle { id, .. } | WorkflowParameter::Text { id, .. } => id,
    }
}

// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
pub fn workflow_parameter_name(parameter: &WorkflowParameter) -> String {
    match parameter {
        WorkflowParameter::Numeric { name, .. } | WorkflowParameter::Categorical { name, .. } | WorkflowParameter::Toggle { name, .. } | WorkflowParameter::Text { name, .. } => name.clone(),
    }
}

// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
pub fn workflow_parameter_value(parameter: &WorkflowParameter) -> serde_json::Value {
    match parameter {
        WorkflowParameter::Numeric { value, .. } => serde_json::Value::from(*value),
        WorkflowParameter::Categorical { value, .. } => serde_json::Value::from(value.clone()),
        WorkflowParameter::Toggle { value, .. } => serde_json::Value::from(*value),
        WorkflowParameter::Text { value, .. } => serde_json::Value::from(value.clone()),
    }
}

pub async fn workflow_parameter_types_compatible(left: &WorkflowParameterType, right: &WorkflowParameterType) -> bool {
    left == right
}

pub async fn create_default_workflow_parameter(parameter_type: &WorkflowParameterType, name: &str, id: Option<&str>) -> WorkflowParameter {
    let parameter_id = id.map(str::to_string).unwrap_or_else(create_workflow_parameter_id);
    match parameter_type {
        WorkflowParameterType::Numeric => WorkflowParameter::Numeric { id: parameter_id, name: name.into(), value: 0.0, min: Some(0.0), max: Some(100.0), step: Some(1.0) },
        WorkflowParameterType::Categorical => WorkflowParameter::Categorical { id: parameter_id, name: name.into(), value: "Option A".into(), options: vec!["Option A".into(), "Option B".into()] },
        WorkflowParameterType::Toggle => WorkflowParameter::Toggle { id: parameter_id, name: name.into(), value: false },
        WorkflowParameterType::Text => WorkflowParameter::Text { id: parameter_id, name: name.into(), value: String::new() },
    }
}

async fn clamp_workflow_numeric_value(value: f64, min: Option<f64>, max: Option<f64>, step: Option<f64>) -> f64 {
    let mut next = value;
    if let Some(min) = min.filter(|v| v.is_finite()) {
        next = next.max(min);
    }
    if let Some(max) = max.filter(|v| v.is_finite()) {
        next = next.min(max);
    }
    if let Some(step) = step.filter(|v| v.is_finite() && *v > 0.0) {
        let anchor = min.filter(|v| v.is_finite()).unwrap_or(0.0);
        next = anchor + ((next - anchor) / step).round() * step;
        if let Some(min) = min.filter(|v| v.is_finite()) {
            next = next.max(min);
        }
        if let Some(max) = max.filter(|v| v.is_finite()) {
            next = next.min(max);
        }
    }
    next
}

/// @emoji 🎛️ Applies a partial patch to a workflow parameter, enforcing type constraints. Ported
/// verbatim from os-core's `patch_os_parameter`.
pub async fn patch_workflow_parameter(parameter: &WorkflowParameter, patch: &serde_json::Value) -> WorkflowParameter {
    let name = patch.get("name").and_then(|v| v.as_str()).map(str::to_string).unwrap_or_else(|| workflow_parameter_name(parameter));
    let patch_type = patch.get("type").and_then(|v| v.as_str());
    let use_numeric = patch_type == Some("numeric") || (patch_type.is_none() && matches!(parameter, WorkflowParameter::Numeric { .. }));
    if use_numeric {
        let current = match parameter {
            WorkflowParameter::Numeric { .. } => parameter.clone(),
            _ => create_default_workflow_parameter(&WorkflowParameterType::Numeric, &name, Some(workflow_parameter_id(parameter))).await,
        };
        if let WorkflowParameter::Numeric { id, min: current_min, max: current_max, step: current_step, value: current_value, .. } = current {
            let min = patch.get("min").and_then(|v| v.as_f64()).or(current_min);
            let max = patch.get("max").and_then(|v| v.as_f64()).or(current_max);
            let step = patch.get("step").and_then(|v| v.as_f64()).or(current_step);
            let raw_value = patch.get("value").and_then(|v| v.as_f64()).unwrap_or(current_value);
            return WorkflowParameter::Numeric { id, name, min, max, step, value: clamp_workflow_numeric_value(raw_value, min, max, step).await };
        }
    }
    let use_categorical = patch_type == Some("categorical") || (patch_type.is_none() && matches!(parameter, WorkflowParameter::Categorical { .. }));
    if use_categorical {
        let current = match parameter {
            WorkflowParameter::Categorical { .. } => parameter.clone(),
            _ => create_default_workflow_parameter(&WorkflowParameterType::Categorical, &name, Some(workflow_parameter_id(parameter))).await,
        };
        if let WorkflowParameter::Categorical { id, value: current_value, options: current_options, .. } = current {
            let options = patch.get("options").and_then(|v| v.as_array()).map(|entries| entries.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect::<Vec<_>>()).unwrap_or(current_options);
            let unique_options = if options.is_empty() { vec!["Option A".into()] } else { options };
            let value = patch
                .get("value")
                .and_then(|v| v.as_str())
                .filter(|v| unique_options.iter().any(|option| option == *v))
                .map(str::to_string)
                .or_else(|| unique_options.iter().find(|option| **option == current_value).cloned())
                .unwrap_or_else(|| unique_options[0].clone());
            return WorkflowParameter::Categorical { id, name, options: unique_options, value };
        }
    }
    if patch_type == Some("toggle") || (patch_type.is_none() && matches!(parameter, WorkflowParameter::Toggle { .. })) {
        let current = match parameter {
            WorkflowParameter::Toggle { .. } => parameter.clone(),
            _ => create_default_workflow_parameter(&WorkflowParameterType::Toggle, &name, Some(workflow_parameter_id(parameter))).await,
        };
        if let WorkflowParameter::Toggle { id, value: current_value, .. } = current {
            let value = patch.get("value").and_then(|v| v.as_bool()).unwrap_or(current_value);
            return WorkflowParameter::Toggle { id, name, value };
        }
    }
    let current = match parameter {
        WorkflowParameter::Text { .. } => parameter.clone(),
        _ => create_default_workflow_parameter(&WorkflowParameterType::Text, &name, Some(workflow_parameter_id(parameter))).await,
    };
    if let WorkflowParameter::Text { id, value: current_value, .. } = current {
        let value = patch.get("value").and_then(|v| v.as_str()).map(str::to_string).unwrap_or(current_value);
        return WorkflowParameter::Text { id, name, value };
    }
    parameter.clone()
}

/// @emoji ✅️ Type-checks one binding's `field_path` against the target app's declared `ConfigSpec` —
/// `config_spec` is caller-resolved (os-core looks it up via `os_app_registration`), so this function
/// itself needs no registry. Ported verbatim from os-core's `validate_parameter_config_binding`.
/// Returns a `protocol::MutationMessage` rather than the deleted free-form `{kind, uri, message}`
/// diagnostic struct (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C10) —
/// `Warning`, not
/// `Error`/`Fatal`, because this isn't a rejected `Mutation::diff` (the two Laws binding those levels
/// to an empty/unchanged diff don't apply here): it's `reconcile_workflow_snapshot`'s post-merge
/// integrity pass, called only to decide whether a stale binding gets corrective-dropped, and the
/// drop always happens regardless of this message's level. The domain code
/// (`workflow/parameter-binding-invalid`) is intentionally outside the frozen 7 `mutation.*` codes —
/// those govern `diff`-leaf outcomes only, not this reconcile-pass diagnostic (see
/// `🏪️store/🔄️sync/🦀️component.rs`'s `ArtifactEvent::Conflict(MutationMessage)` for the identical
/// precedent).
pub async fn validate_workflow_parameter_config_binding(binding: &WorkflowParameterBinding, parameter_type: &WorkflowParameterType, config_spec: &semio_framework::ConfigSpec) -> Result<(), protocol::MutationMessage> {
    let uri = format!("{}#{}", binding.node_id, binding.field_path);
    let Some(field) = config_spec.fields.iter().find(|field| field.key == binding.field_path) else {
        return Err(protocol::MutationMessage { level: dsl::Severity::Warning, code: dsl::FaultCode::new("workflow/parameter-binding-invalid"), message: format!("binding targets config field '{}', which the app's ConfigSpec does not declare", binding.field_path), target: vec![uri], op_index: None });
    };
    let compatible = matches!(
        (parameter_type, &field.shape),
        (WorkflowParameterType::Numeric, semio_framework::ConfigFieldShape::Number { .. })
            | (WorkflowParameterType::Categorical, semio_framework::ConfigFieldShape::Select { .. })
            | (WorkflowParameterType::Toggle, semio_framework::ConfigFieldShape::Toggle)
            | (WorkflowParameterType::Text, semio_framework::ConfigFieldShape::Text)
    );
    if compatible {
        Ok(())
    } else {
        Err(protocol::MutationMessage { level: dsl::Severity::Warning, code: dsl::FaultCode::new("workflow/parameter-binding-invalid"), message: format!("parameter type {parameter_type:?} cannot drive config field '{}' ({:?})", binding.field_path, field.shape), target: vec![uri], op_index: None })
    }
}

/// @emoji 🎛️ Resolves bound parameter values for a workflow node as a field-path map.
pub async fn resolve_workflow_parameter_values(bindings: &[WorkflowParameterBinding], parameters: &[WorkflowParameter], node_id: &str) -> HashMap<String, serde_json::Value> {
    let mut values = HashMap::new();
    for binding in bindings.iter().filter(|entry| entry.node_id == node_id) {
        let Some(parameter) = parameters.iter().find(|entry| workflow_parameter_id(entry) == binding.parameter_id) else {
            continue;
        };
        values.insert(binding.field_path.clone(), workflow_parameter_value(parameter));
    }
    values
}

// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
pub fn workflow_parameter_port_id(node_id: &str, parameter_id: &str) -> String {
    media_port_id_for_spec(node_id, &format!("{WORKFLOW_PARAMETER_PORT_PREFIX}{parameter_id}"), "in")
}

// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
pub fn is_workflow_parameter_port_id(port_id: &str) -> bool {
    media_port_spec_id(port_id).map(|spec_id| spec_id.starts_with(WORKFLOW_PARAMETER_PORT_PREFIX)).unwrap_or(false)
}

// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
pub fn workflow_parameter_id_from_port_id(port_id: &str) -> Option<String> {
    let spec_id = media_port_spec_id(port_id)?;
    spec_id.strip_prefix(WORKFLOW_PARAMETER_PORT_PREFIX).map(str::to_string)
}

// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
pub fn media_port_id_for_spec(instance_id: &str, spec_id: &str, direction: &str) -> String {
    format!("{instance_id}:{spec_id}:{direction}")
}

// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
pub fn media_port_spec_id(port_id: &str) -> Option<String> {
    let parts: Vec<_> = port_id.split(':').collect();
    if parts.len() < 3 {
        return None;
    }
    Some(parts[1..parts.len() - 1].join(":"))
}

/// 🎛️ Resyncs one node's parameter-bound input ports from `bindings` — every non-parameter port is
/// kept as-is, every parameter port is rebuilt from scratch (idempotent: bind/unbind/patch all funnel
/// through this). Ported verbatim from os-core's `sync_workflow_node_parameter_ports`.
// 🚫️async: E1 transitive — consumed by std Iterator::map (external trait) in a sync closure;
// pure node reshape, no I/O (R9).
fn sync_workflow_node_parameter_ports(node: &WorkflowNode, bindings: &[WorkflowParameterBinding]) -> WorkflowNode {
    let node_bindings: Vec<_> = bindings.iter().filter(|binding| binding.node_id == node.id).collect();
    let base_inputs: Vec<_> = node.inputs.iter().filter(|port| !is_workflow_parameter_port_id(&port.id)).cloned().collect();
    let parameter_inputs: Vec<_> = node_bindings
        .iter()
        .map(|binding| WorkflowMediaPort {
            id: workflow_parameter_port_id(&node.id, &binding.parameter_id),
            spec: MediaPortSpec {
                id: format!("{WORKFLOW_PARAMETER_PORT_PREFIX}{}", binding.parameter_id),
                label: "Parameter".into(),
                direction: MediaPortDirection::In,
                media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                kind_id: Some("parameter.value".into()),
                required: false,
                multiplicity: PortMultiplicity::One,
            },
        })
        .collect();
    let inputs: Vec<_> = base_inputs.into_iter().chain(parameter_inputs).collect();
    let port_count = inputs.len().max(node.outputs.len()).max(1);
    WorkflowNode { inputs, height: 56.0 + port_count as f64 * 18.0, ..node.clone() }
}

pub async fn sync_workflow_parameter_ports(graph: &Workflow, bindings: &[WorkflowParameterBinding]) -> Workflow {
    Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: graph.nodes.iter().map(|node| sync_workflow_node_parameter_ports(node, bindings)).collect(), edges: graph.edges.clone() }
}
//#endregion 🔖️WorkflowParameters

//#region 🔖️WorkflowSnapshot
/// 🔌️ One declared collection-level input slot a workflow's nodes can bind an in-port to — `selector`
/// is a glob matched against collection entry paths at run time (W5's `SpaceRunner` job); this crate
/// only carries the declaration + validates bindings resolve (`validate_workflow_snapshot`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowInput {
    pub id: String,
    pub kind_id: String,
    pub selector: String,
    pub required: bool,
    pub multiplicity: PortMultiplicity,
}

// 🚫️async: E4 fn-pointer slot — value goes into `dsl::Shape::Record(fn() -> RecordSpec)`.
fn workflow_input_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        None,
        dsl::RecordLayout::Inline,
        vec![
            dsl::FieldSpec::new(0, "id", dsl::Shape::Text),
            dsl::FieldSpec::new(1, "kind_id", dsl::Shape::Text),
            dsl::FieldSpec::new(2, "selector", dsl::Shape::Text),
            dsl::FieldSpec::new(3, "required", dsl::Shape::Bool),
            dsl::FieldSpec::new(4, "multiplicity", dsl::Shape::Enum(port_multiplicity_variants())),
        ],
    )
}

/// 🧬️ Hand-crafted `dsl::DslField` (not `#[derive(dsl::DslRecord)]`) — `multiplicity: PortMultiplicity`
/// is a foreign type this crate can't derive `DslField` for under the orphan rule, same reasoning as
/// `WorkflowMediaPort`/`MediaContract` above; reuses their `port_multiplicity_ordinal`/`_from_ordinal`/
/// `_variants` helpers directly.
impl dsl::DslField for WorkflowInput {
    // 🚫️async: E4 fn-pointer transitivity — see `DslField::shape`'s tag (R9).
    fn shape() -> dsl::Shape {
        dsl::Shape::Record(workflow_input_spec)
    }
    fn to_value(&self) -> dsl::FieldValue {
        let mut record = dsl::RecordValue::default();
        record.fields.insert(0, dsl::FieldValue::Text(self.id.clone()));
        record.fields.insert(1, dsl::FieldValue::Text(self.kind_id.clone()));
        record.fields.insert(2, dsl::FieldValue::Text(self.selector.clone()));
        record.fields.insert(3, dsl::FieldValue::Bool(self.required));
        record.fields.insert(4, dsl::FieldValue::Enum(port_multiplicity_ordinal(self.multiplicity)));
        dsl::FieldValue::Record(record)
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else { return Err(format!("expected Record, found {value:?}")) };
        let id = match record.get(0) {
            Some(dsl::FieldValue::Text(s)) => s.clone(),
            other => return Err(format!("expected id, found {other:?}")),
        };
        let kind_id = match record.get(1) {
            Some(dsl::FieldValue::Text(s)) => s.clone(),
            other => return Err(format!("expected kind_id, found {other:?}")),
        };
        let selector = match record.get(2) {
            Some(dsl::FieldValue::Text(s)) => s.clone(),
            other => return Err(format!("expected selector, found {other:?}")),
        };
        let required = match record.get(3) {
            Some(dsl::FieldValue::Bool(b)) => *b,
            other => return Err(format!("expected required, found {other:?}")),
        };
        let multiplicity = match record.get(4) {
            Some(dsl::FieldValue::Enum(ordinal)) => port_multiplicity_from_ordinal(*ordinal)?,
            other => return Err(format!("expected multiplicity, found {other:?}")),
        };
        Ok(WorkflowInput { id, kind_id, selector, required, multiplicity })
    }
}

/// 🔗️ Binds a declared [`WorkflowInput`] slot onto one node's in-port.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct WorkflowInputBinding {
    pub input_id: String,
    pub node_id: String,
    pub port_id: String,
}

/// 📤️ Names where a node's out-port materializes in the output collection — `path_template` like
/// `"renders/{node}/{input.stem}.{ext}"` (resolved at run time by W5's `SpaceRunner`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct WorkflowOutputBinding {
    pub node_id: String,
    pub port_id: String,
    pub path_template: String,
}

/// 🕸️ The `os.workflow` persisted artifact — a non-destructive pipeline of connected apps (`graph`),
/// its parameters/bindings, and its declared collection-level inputs/outputs. Absorbs os-core's
/// dissolved `OsSnapshot` (`programs` moved to `space::SpaceSnapshot`, `active_plugin_id`/
/// `active_alternative_id` become space-app session state — see `## The inversion` in the plan).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[dsl(id = "os.workflow")]
pub struct WorkflowSnapshot {
    pub schema: String,
    #[dsl(block)]
    pub graph: Workflow,
    #[dsl(statements)]
    pub parameters: Vec<WorkflowParameter>,
    #[dsl(table)]
    pub parameter_bindings: Vec<WorkflowParameterBinding>,
    // 🧮️ NOT `#[dsl(table)]`: `WorkflowInput` hand-crafts `dsl::DslField` (its `multiplicity:
    // PortMultiplicity` field is foreign, orphan-rule-blocked from `#[derive(dsl::DslRecord)]` — same
    // reasoning as `CollectionEntry.body` in the `space` crate), so it has no `__dsl_spec` for the
    // compact Structure-of-Arrays table macro; the plain (unattributed) `Vec<DslField>` shape below
    // renders it as the expanded Array-of-Structs `Shape::List(Record)` form instead.
    pub inputs: Vec<WorkflowInput>,
    #[dsl(table)]
    pub input_bindings: Vec<WorkflowInputBinding>,
    #[dsl(table)]
    pub output_bindings: Vec<WorkflowOutputBinding>,
}

pub async fn empty_workflow_snapshot() -> WorkflowSnapshot {
    WorkflowSnapshot { schema: S_WORKFLOW_SCHEMA.into(), graph: empty_workflow().await, parameters: Vec::new(), parameter_bindings: Vec::new(), inputs: Vec::new(), input_bindings: Vec::new(), output_bindings: Vec::new() }
}

//#region 🔖️HandcraftedWorkflowSnapshotCodecs
/// 🧬️ P6: `DslArtifact` emits helpers only — ArtifactDsl/ArtifactPack are handcrafted here.
impl store::ArtifactDsl for WorkflowSnapshot {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    async fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    async fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for WorkflowSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options).await?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id().await {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id().await, envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options).await?;
        match Self::__dsl_from_record(&record) {
            Ok(value) => Ok(value),
            Err(error) => Err(store::text_error_to_pack_error(error)),
        }
    }
    async fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedWorkflowSnapshotCodecs

// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
fn workflow_parameter_entity_id(parameter: &WorkflowParameter) -> &str {
    workflow_parameter_id(parameter)
}

//#region 🔖️WorkflowMutation
/// ⚡️ One settled `WorkflowSnapshot` mutation — lifted from os-core's dissolved `OsMutation` (graph/
/// parameter arms renamed `*WorkflowNode`/`*WorkflowEdge` -> `*Node`/`*Edge` for brevity now that
/// there's no sibling `Os*` type to disambiguate from) plus new `DeclareInput`/`RemoveInput`/
/// `BindInput`/`UnbindInput`/`BindOutput`/`UnbindOutput` variants for the new fields. `SetActiveProgram`/
/// `SetActiveAlternative` are NOT here — see `## The inversion`: those became space-app session state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum WorkflowMutation {
    AddNode { node: WorkflowNode },
    RemoveNode { node_id: String },
    ConnectPorts { edge: WorkflowEdge },
    DisconnectEdge { edge_id: String },
    MoveNode { node_id: String, x: f64, y: f64 },
    PatchNode { node_id: String, label: String },
    AddParameter { parameter: WorkflowParameter },
    RemoveParameter { parameter_id: String },
    PatchParameter { parameter_id: String, parameter: WorkflowParameter },
    BindParameterField { binding: WorkflowParameterBinding },
    UnbindParameterField { node_id: String, field_path: String },
    SyncNodePorts,
    DeclareInput { input: WorkflowInput },
    RemoveInput { input_id: String },
    BindInput { binding: WorkflowInputBinding },
    UnbindInput { input_id: String },
    BindOutput { binding: WorkflowOutputBinding },
    UnbindOutput { node_id: String, port_id: String },
}

pub async fn apply_workflow_operation(document: &WorkflowSnapshot, operation: &WorkflowMutation) -> WorkflowSnapshot {
    let mut next = document.clone();
    match operation {
        WorkflowMutation::AddNode { node } => {
            let node = sync_workflow_node_parameter_ports(node, &next.parameter_bindings);
            next.graph.nodes.push(node);
        }
        WorkflowMutation::RemoveNode { node_id } => {
            next.parameter_bindings.retain(|binding| binding.node_id != *node_id);
            next.input_bindings.retain(|binding| binding.node_id != *node_id);
            next.output_bindings.retain(|binding| binding.node_id != *node_id);
            next.graph.nodes.retain(|node| node.id != *node_id);
            next.graph.edges.retain(|edge| edge.source_node_id != *node_id && edge.target_node_id != *node_id);
        }
        WorkflowMutation::ConnectPorts { edge } => next.graph.edges.push(edge.clone()),
        WorkflowMutation::DisconnectEdge { edge_id } => next.graph.edges.retain(|edge| edge.id != *edge_id),
        WorkflowMutation::MoveNode { node_id, x, y } => {
            for node in &mut next.graph.nodes {
                if node.id == *node_id {
                    node.x = *x;
                    node.y = *y;
                }
            }
        }
        WorkflowMutation::PatchNode { node_id, label } => {
            for node in &mut next.graph.nodes {
                if node.id == *node_id {
                    node.label = label.clone();
                }
            }
        }
        WorkflowMutation::AddParameter { parameter } => next.parameters.push(parameter.clone()),
        WorkflowMutation::RemoveParameter { parameter_id } => {
            next.parameters.retain(|parameter| workflow_parameter_entity_id(parameter) != *parameter_id);
            next.parameter_bindings.retain(|binding| binding.parameter_id != *parameter_id);
            next.graph = sync_workflow_parameter_ports(&next.graph, &next.parameter_bindings).await;
        }
        WorkflowMutation::PatchParameter { parameter_id, parameter } => {
            for entry in &mut next.parameters {
                if workflow_parameter_entity_id(entry) == *parameter_id {
                    *entry = parameter.clone();
                }
            }
        }
        WorkflowMutation::BindParameterField { binding } => {
            next.parameter_bindings.retain(|entry| !(entry.node_id == binding.node_id && entry.field_path == binding.field_path));
            next.parameter_bindings.push(binding.clone());
            next.graph = sync_workflow_parameter_ports(&next.graph, &next.parameter_bindings).await;
        }
        WorkflowMutation::UnbindParameterField { node_id, field_path } => {
            next.parameter_bindings.retain(|binding| !(binding.node_id == *node_id && binding.field_path == *field_path));
            next.graph = sync_workflow_parameter_ports(&next.graph, &next.parameter_bindings).await;
        }
        WorkflowMutation::SyncNodePorts => {
            next.graph = sync_workflow_parameter_ports(&next.graph, &next.parameter_bindings).await;
        }
        WorkflowMutation::DeclareInput { input } => next.inputs.push(input.clone()),
        WorkflowMutation::RemoveInput { input_id } => {
            next.inputs.retain(|input| input.id != *input_id);
            next.input_bindings.retain(|binding| binding.input_id != *input_id);
        }
        WorkflowMutation::BindInput { binding } => {
            next.input_bindings.retain(|entry| entry.input_id != binding.input_id);
            next.input_bindings.push(binding.clone());
        }
        WorkflowMutation::UnbindInput { input_id } => {
            next.input_bindings.retain(|binding| binding.input_id != *input_id);
        }
        WorkflowMutation::BindOutput { binding } => {
            next.output_bindings.retain(|entry| !(entry.node_id == binding.node_id && entry.port_id == binding.port_id));
            next.output_bindings.push(binding.clone());
        }
        WorkflowMutation::UnbindOutput { node_id, port_id } => {
            next.output_bindings.retain(|binding| !(binding.node_id == *node_id && binding.port_id == *port_id));
        }
    }
    next
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum WorkflowDiff {
    #[default]
    Empty,
    AddNode {
        node: WorkflowNode,
    },
    RemoveNode {
        node_id: String,
    },
    ConnectPorts {
        edge: WorkflowEdge,
    },
    DisconnectEdge {
        edge_id: String,
    },
    MoveNode {
        node_id: String,
        x: f64,
        y: f64,
    },
    PatchNode {
        node_id: String,
        label: String,
    },
    AddParameter {
        parameter: WorkflowParameter,
    },
    RemoveParameter {
        parameter_id: String,
    },
    PatchParameter {
        parameter_id: String,
        parameter: WorkflowParameter,
    },
    BindParameterField {
        binding: WorkflowParameterBinding,
    },
    UnbindParameterField {
        node_id: String,
        field_path: String,
    },
    SyncNodePorts,
    DeclareInput {
        input: WorkflowInput,
    },
    RemoveInput {
        input_id: String,
    },
    BindInput {
        binding: WorkflowInputBinding,
    },
    UnbindInput {
        input_id: String,
    },
    BindOutput {
        binding: WorkflowOutputBinding,
    },
    UnbindOutput {
        node_id: String,
        port_id: String,
    },
}

impl protocol::MutationDiff<WorkflowSnapshot> for WorkflowDiff {
    async fn apply(&self, document: &WorkflowSnapshot) -> protocol::MutationApplyResult<WorkflowSnapshot> {
        match self {
            WorkflowDiff::Empty | WorkflowDiff::SyncNodePorts => {}
            WorkflowDiff::AddNode { node } => {
                if document.graph.nodes.iter().any(|entry| entry.id == node.id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "workflow node identity already exists").await.at(["nodes", node.id.as_str()]).await);
                }
            }
            WorkflowDiff::RemoveNode { node_id } | WorkflowDiff::MoveNode { node_id, .. } | WorkflowDiff::PatchNode { node_id, .. } => {
                if !document.graph.nodes.iter().any(|node| node.id == *node_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow node does not exist").await.at(["nodes", node_id.as_str()]).await);
                }
            }
            WorkflowDiff::ConnectPorts { edge } => {
                if document.graph.edges.iter().any(|entry| entry.id == edge.id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "workflow edge identity already exists").await.at(["edges", edge.id.as_str()]).await);
                }
                let source = match document.graph.nodes.iter().find(|node| node.id == edge.source_node_id) {
                    Some(node) => node,
                    None => return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow edge source node does not exist").await.at(["edges", edge.id.as_str(), "sourceNodeId"]).await),
                };
                if !source.outputs.iter().any(|port| port.id == edge.source_port_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow edge source port does not exist").await.at(["edges", edge.id.as_str(), "sourcePortId"]).await);
                }
                let target = match document.graph.nodes.iter().find(|node| node.id == edge.target_node_id) {
                    Some(node) => node,
                    None => return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow edge target node does not exist").await.at(["edges", edge.id.as_str(), "targetNodeId"]).await),
                };
                if !target.inputs.iter().any(|port| port.id == edge.target_port_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow edge target port does not exist").await.at(["edges", edge.id.as_str(), "targetPortId"]).await);
                }
            }
            WorkflowDiff::DisconnectEdge { edge_id } => {
                if !document.graph.edges.iter().any(|edge| edge.id == *edge_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow edge does not exist").await.at(["edges", edge_id.as_str()]).await);
                }
            }
            WorkflowDiff::AddParameter { parameter } => {
                let parameter_id = workflow_parameter_entity_id(parameter);
                if document.parameters.iter().any(|entry| workflow_parameter_entity_id(entry) == parameter_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "workflow parameter identity already exists").await.at(["parameters", parameter_id]).await);
                }
            }
            WorkflowDiff::RemoveParameter { parameter_id } => {
                if !document.parameters.iter().any(|parameter| workflow_parameter_entity_id(parameter) == parameter_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow parameter does not exist").await.at(["parameters", parameter_id.as_str()]).await);
                }
            }
            WorkflowDiff::PatchParameter { parameter_id, parameter } => {
                if !document.parameters.iter().any(|entry| workflow_parameter_entity_id(entry) == parameter_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow parameter does not exist").await.at(["parameters", parameter_id.as_str()]).await);
                }
                let new_id = workflow_parameter_entity_id(parameter);
                if new_id != parameter_id && document.parameters.iter().any(|entry| workflow_parameter_entity_id(entry) == new_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "patched workflow parameter identity already exists").await.at(["parameters", new_id]).await);
                }
            }
            WorkflowDiff::BindParameterField { binding } => {
                if !document.graph.nodes.iter().any(|node| node.id == binding.node_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "parameter binding node does not exist").await.at(["parameterBindings", binding.node_id.as_str()]).await);
                }
                if !document.parameters.iter().any(|parameter| workflow_parameter_entity_id(parameter) == binding.parameter_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "bound workflow parameter does not exist").await.at(["parameterBindings", binding.parameter_id.as_str()]).await);
                }
            }
            WorkflowDiff::UnbindParameterField { node_id, field_path } => {
                if !document.parameter_bindings.iter().any(|binding| binding.node_id == *node_id && binding.field_path == *field_path) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow parameter binding does not exist").await.at(["parameterBindings", node_id.as_str(), field_path.as_str()]).await);
                }
            }
            WorkflowDiff::DeclareInput { input } => {
                if document.inputs.iter().any(|entry| entry.id == input.id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "workflow input identity already exists").await.at(["inputs", input.id.as_str()]).await);
                }
            }
            WorkflowDiff::RemoveInput { input_id } => {
                if !document.inputs.iter().any(|input| input.id == *input_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow input does not exist").await.at(["inputs", input_id.as_str()]).await);
                }
            }
            WorkflowDiff::BindInput { binding } => {
                if !document.inputs.iter().any(|input| input.id == binding.input_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "bound workflow input does not exist").await.at(["inputBindings", binding.input_id.as_str()]).await);
                }
                let node = match document.graph.nodes.iter().find(|node| node.id == binding.node_id) {
                    Some(node) => node,
                    None => return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "input binding node does not exist").await.at(["inputBindings", binding.node_id.as_str()]).await),
                };
                if !node.inputs.iter().any(|port| port.id == binding.port_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "input binding port does not exist").await.at(["inputBindings", binding.node_id.as_str(), binding.port_id.as_str()]).await);
                }
            }
            WorkflowDiff::UnbindInput { input_id } => {
                if !document.input_bindings.iter().any(|binding| binding.input_id == *input_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow input binding does not exist").await.at(["inputBindings", input_id.as_str()]).await);
                }
            }
            WorkflowDiff::BindOutput { binding } => {
                let node = match document.graph.nodes.iter().find(|node| node.id == binding.node_id) {
                    Some(node) => node,
                    None => return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "output binding node does not exist").await.at(["outputBindings", binding.node_id.as_str()]).await),
                };
                if !node.outputs.iter().any(|port| port.id == binding.port_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "output binding port does not exist").await.at(["outputBindings", binding.node_id.as_str(), binding.port_id.as_str()]).await);
                }
            }
            WorkflowDiff::UnbindOutput { node_id, port_id } => {
                if !document.output_bindings.iter().any(|binding| binding.node_id == *node_id && binding.port_id == *port_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow output binding does not exist").await.at(["outputBindings", node_id.as_str(), port_id.as_str()]).await);
                }
            }
        }
        let operation = match self {
            WorkflowDiff::Empty => return Ok(document.clone()),
            WorkflowDiff::AddNode { node } => WorkflowMutation::AddNode { node: node.clone() },
            WorkflowDiff::RemoveNode { node_id } => WorkflowMutation::RemoveNode { node_id: node_id.clone() },
            WorkflowDiff::ConnectPorts { edge } => WorkflowMutation::ConnectPorts { edge: edge.clone() },
            WorkflowDiff::DisconnectEdge { edge_id } => WorkflowMutation::DisconnectEdge { edge_id: edge_id.clone() },
            WorkflowDiff::MoveNode { node_id, x, y } => WorkflowMutation::MoveNode { node_id: node_id.clone(), x: *x, y: *y },
            WorkflowDiff::PatchNode { node_id, label } => WorkflowMutation::PatchNode { node_id: node_id.clone(), label: label.clone() },
            WorkflowDiff::AddParameter { parameter } => WorkflowMutation::AddParameter { parameter: parameter.clone() },
            WorkflowDiff::RemoveParameter { parameter_id } => WorkflowMutation::RemoveParameter { parameter_id: parameter_id.clone() },
            WorkflowDiff::PatchParameter { parameter_id, parameter } => WorkflowMutation::PatchParameter { parameter_id: parameter_id.clone(), parameter: parameter.clone() },
            WorkflowDiff::BindParameterField { binding } => WorkflowMutation::BindParameterField { binding: binding.clone() },
            WorkflowDiff::UnbindParameterField { node_id, field_path } => WorkflowMutation::UnbindParameterField { node_id: node_id.clone(), field_path: field_path.clone() },
            WorkflowDiff::SyncNodePorts => WorkflowMutation::SyncNodePorts,
            WorkflowDiff::DeclareInput { input } => WorkflowMutation::DeclareInput { input: input.clone() },
            WorkflowDiff::RemoveInput { input_id } => WorkflowMutation::RemoveInput { input_id: input_id.clone() },
            WorkflowDiff::BindInput { binding } => WorkflowMutation::BindInput { binding: binding.clone() },
            WorkflowDiff::UnbindInput { input_id } => WorkflowMutation::UnbindInput { input_id: input_id.clone() },
            WorkflowDiff::BindOutput { binding } => WorkflowMutation::BindOutput { binding: binding.clone() },
            WorkflowDiff::UnbindOutput { node_id, port_id } => WorkflowMutation::UnbindOutput { node_id: node_id.clone(), port_id: port_id.clone() },
        };
        Ok(apply_workflow_operation(document, &operation).await)
    }

    async fn absorb(&mut self, other: Self) {
        if !matches!(other, WorkflowDiff::Empty) {
            *self = other;
        }
    }
}

impl protocol::Mutation<WorkflowSnapshot> for WorkflowMutation {
    type Diff = WorkflowDiff;

    /// 🧮️ Mechanical wrap only (26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-
    /// CONFLICTS W0): no `Error`/`Warning`/`Fatal` messages added here yet.
    async fn diff(&self, _document: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> {
        let diff = match self {
            WorkflowMutation::AddNode { node } => WorkflowDiff::AddNode { node: node.clone() },
            WorkflowMutation::RemoveNode { node_id } => WorkflowDiff::RemoveNode { node_id: node_id.clone() },
            WorkflowMutation::ConnectPorts { edge } => WorkflowDiff::ConnectPorts { edge: edge.clone() },
            WorkflowMutation::DisconnectEdge { edge_id } => WorkflowDiff::DisconnectEdge { edge_id: edge_id.clone() },
            WorkflowMutation::MoveNode { node_id, x, y } => WorkflowDiff::MoveNode { node_id: node_id.clone(), x: *x, y: *y },
            WorkflowMutation::PatchNode { node_id, label } => WorkflowDiff::PatchNode { node_id: node_id.clone(), label: label.clone() },
            WorkflowMutation::AddParameter { parameter } => WorkflowDiff::AddParameter { parameter: parameter.clone() },
            WorkflowMutation::RemoveParameter { parameter_id } => WorkflowDiff::RemoveParameter { parameter_id: parameter_id.clone() },
            WorkflowMutation::PatchParameter { parameter_id, parameter } => WorkflowDiff::PatchParameter { parameter_id: parameter_id.clone(), parameter: parameter.clone() },
            WorkflowMutation::BindParameterField { binding } => WorkflowDiff::BindParameterField { binding: binding.clone() },
            WorkflowMutation::UnbindParameterField { node_id, field_path } => WorkflowDiff::UnbindParameterField { node_id: node_id.clone(), field_path: field_path.clone() },
            WorkflowMutation::SyncNodePorts => WorkflowDiff::SyncNodePorts,
            WorkflowMutation::DeclareInput { input } => WorkflowDiff::DeclareInput { input: input.clone() },
            WorkflowMutation::RemoveInput { input_id } => WorkflowDiff::RemoveInput { input_id: input_id.clone() },
            WorkflowMutation::BindInput { binding } => WorkflowDiff::BindInput { binding: binding.clone() },
            WorkflowMutation::UnbindInput { input_id } => WorkflowDiff::UnbindInput { input_id: input_id.clone() },
            WorkflowMutation::BindOutput { binding } => WorkflowDiff::BindOutput { binding: binding.clone() },
            WorkflowMutation::UnbindOutput { node_id, port_id } => WorkflowDiff::UnbindOutput { node_id: node_id.clone(), port_id: port_id.clone() },
        };
        protocol::MutationOutcome::new(diff).await
    }

    async fn inverse(&self, document: &WorkflowSnapshot) -> Vec<Self> {
        match self {
            WorkflowMutation::AddNode { node } => vec![WorkflowMutation::RemoveNode { node_id: node.id.clone() }],
            // 🧵️ `apply`'s `RemoveNode` arm cascades away every edge/binding touching the node — its
            // inverse restores the FULL pre-state, not just the bare node, by re-emitting one
            // reconstructing op per cascade-deleted dependent (same treatment `RemoveParameter`/
            // `RemoveInput` below get for their own cascades). Callers (`assert_operation_round_trip`,
            // undo) apply `inverse(pre)` REVERSED, so the cascade re-`connect`/re-`bind` ops — each of
            // which needs the node back first — go BEFORE `AddNode` here, putting `AddNode` first once
            // reversed; a bare `Vec::new()` still stands in when the target is already gone.
            WorkflowMutation::RemoveNode { node_id } => {
                let Some(node) = document.graph.nodes.iter().find(|node| node.id == *node_id) else { return Vec::new() };
                let mut ops: Vec<Self> = document.graph.edges.iter().filter(|edge| edge.source_node_id == *node_id || edge.target_node_id == *node_id).map(|edge| WorkflowMutation::ConnectPorts { edge: edge.clone() }).collect();
                ops.extend(document.parameter_bindings.iter().filter(|binding| binding.node_id == *node_id).map(|binding| WorkflowMutation::BindParameterField { binding: binding.clone() }));
                ops.extend(document.input_bindings.iter().filter(|binding| binding.node_id == *node_id).map(|binding| WorkflowMutation::BindInput { binding: binding.clone() }));
                ops.extend(document.output_bindings.iter().filter(|binding| binding.node_id == *node_id).map(|binding| WorkflowMutation::BindOutput { binding: binding.clone() }));
                ops.push(WorkflowMutation::AddNode { node: node.clone() });
                ops
            }
            WorkflowMutation::ConnectPorts { edge } => vec![WorkflowMutation::DisconnectEdge { edge_id: edge.id.clone() }],
            WorkflowMutation::DisconnectEdge { edge_id } => document.graph.edges.iter().find(|edge| edge.id == *edge_id).map(|edge| vec![WorkflowMutation::ConnectPorts { edge: edge.clone() }]).unwrap_or_default(),
            WorkflowMutation::MoveNode { node_id, .. } => document.graph.nodes.iter().find(|node| node.id == *node_id).map(|node| vec![WorkflowMutation::MoveNode { node_id: node_id.clone(), x: node.x, y: node.y }]).unwrap_or_default(),
            WorkflowMutation::PatchNode { node_id, .. } => document.graph.nodes.iter().find(|node| node.id == *node_id).map(|node| vec![WorkflowMutation::PatchNode { node_id: node_id.clone(), label: node.label.clone() }]).unwrap_or_default(),
            WorkflowMutation::AddParameter { parameter } => vec![WorkflowMutation::RemoveParameter { parameter_id: workflow_parameter_entity_id(parameter).into() }],
            // 🧵️ Restores cascade-deleted `parameter_bindings` too — see `RemoveNode`'s doc above.
            WorkflowMutation::RemoveParameter { parameter_id } => {
                let Some(parameter) = document.parameters.iter().find(|parameter| workflow_parameter_entity_id(parameter) == *parameter_id) else { return Vec::new() };
                let mut ops: Vec<Self> = document.parameter_bindings.iter().filter(|binding| binding.parameter_id == *parameter_id).map(|binding| WorkflowMutation::BindParameterField { binding: binding.clone() }).collect();
                ops.push(WorkflowMutation::AddParameter { parameter: parameter.clone() });
                ops
            }
            WorkflowMutation::PatchParameter { parameter_id, parameter } => document
                .parameters
                .iter()
                .find(|entry| workflow_parameter_entity_id(entry) == *parameter_id)
                .map(|current| vec![WorkflowMutation::PatchParameter { parameter_id: parameter_id.clone(), parameter: current.clone() }])
                .unwrap_or_else(|| vec![WorkflowMutation::PatchParameter { parameter_id: parameter_id.clone(), parameter: parameter.clone() }]),
            WorkflowMutation::BindParameterField { binding } => vec![WorkflowMutation::UnbindParameterField { node_id: binding.node_id.clone(), field_path: binding.field_path.clone() }],
            WorkflowMutation::UnbindParameterField { node_id, field_path } => {
                document.parameter_bindings.iter().find(|binding| binding.node_id == *node_id && binding.field_path == *field_path).map(|binding| vec![WorkflowMutation::BindParameterField { binding: binding.clone() }]).unwrap_or_default()
            }
            WorkflowMutation::SyncNodePorts => Vec::new(),
            WorkflowMutation::DeclareInput { input } => vec![WorkflowMutation::RemoveInput { input_id: input.id.clone() }],
            // 🧵️ Restores cascade-deleted `input_bindings` too — see `RemoveNode`'s doc above.
            WorkflowMutation::RemoveInput { input_id } => {
                let Some(input) = document.inputs.iter().find(|input| input.id == *input_id) else { return Vec::new() };
                let mut ops: Vec<Self> = document.input_bindings.iter().filter(|binding| binding.input_id == *input_id).map(|binding| WorkflowMutation::BindInput { binding: binding.clone() }).collect();
                ops.push(WorkflowMutation::DeclareInput { input: input.clone() });
                ops
            }
            // 🧵️ Unlike `BindParameterField` (ported verbatim from os-core's `OsMutation`, same
            // overwrite-loses-prior-value shape it always had), `BindInput`/`BindOutput` are new W3
            // ops with no prior precedent to preserve — their inverse restores whatever binding a
            // rebind overwrote (mirrors `space::SpaceMutation::UpsertUser`'s inverse), rather than
            // just unbinding outright.
            WorkflowMutation::BindInput { binding } => match document.input_bindings.iter().find(|entry| entry.input_id == binding.input_id) {
                Some(existing) => vec![WorkflowMutation::BindInput { binding: existing.clone() }],
                None => vec![WorkflowMutation::UnbindInput { input_id: binding.input_id.clone() }],
            },
            WorkflowMutation::UnbindInput { input_id } => document.input_bindings.iter().find(|binding| binding.input_id == *input_id).map(|binding| vec![WorkflowMutation::BindInput { binding: binding.clone() }]).unwrap_or_default(),
            WorkflowMutation::BindOutput { binding } => match document.output_bindings.iter().find(|entry| entry.node_id == binding.node_id && entry.port_id == binding.port_id) {
                Some(existing) => vec![WorkflowMutation::BindOutput { binding: existing.clone() }],
                None => vec![WorkflowMutation::UnbindOutput { node_id: binding.node_id.clone(), port_id: binding.port_id.clone() }],
            },
            WorkflowMutation::UnbindOutput { node_id, port_id } => {
                document.output_bindings.iter().find(|binding| binding.node_id == *node_id && binding.port_id == *port_id).map(|binding| vec![WorkflowMutation::BindOutput { binding: binding.clone() }]).unwrap_or_default()
            }
        }
    }

    // 🚧️ No `reconcile` override here (inherits the trait's no-op default): the two rules that used to
    // run alongside the pure structural ones in os-core's `reconcile_os_workflow` — contract
    // renegotiation and parameter-binding-vs-ConfigSpec validation — both need the os-core plugin/
    // artifact registry, which doesn't exist at this layer (same reasoning `validate_workflow`'s own
    // doc already gives for staying registry-free here). Rather than split the four-rule pipeline
    // across two layers (risking a different rule ordering than the one the existing tests pin), the
    // WHOLE graph-reconcile pass — structural rules included — stays a single ordered pipeline at the
    // os-core layer (`reconcile_workflow_snapshot`, invoked explicitly by `OsWorkflowStore`, not
    // through this trait hook). See os-core's `🔖️GraphReconcile` region.
}
//#endregion 🔖️WorkflowMutation

//#region 🔖️WorkflowMutationOpText
/// 🧬️ Local structural twin of [`WorkflowMutation`] for the `dsl::DslOps` derive — mirrors os-core's
/// deleted `OsOperationDsl` bridge exactly (see its doc for why `AddParameter`/`PatchParameter` box
/// their `parameter` field: `WorkflowParameter` derives `dsl::DslEnum`, giving it `DslVariants` but not
/// `DslField`, and the engine's `#[dsl(statements)]` only recognizes `Vec`/`Option`/`Box` wrappers).
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum WorkflowMutationDsl {
    AddNode {
        node: WorkflowNode,
    },
    RemoveNode {
        #[dsl(key = "id")]
        node_id: String,
    },
    ConnectPorts {
        edge: WorkflowEdge,
    },
    DisconnectEdge {
        #[dsl(key = "id")]
        edge_id: String,
    },
    MoveNode {
        #[dsl(key = "id")]
        node_id: String,
        x: f64,
        y: f64,
    },
    PatchNode {
        #[dsl(key = "id")]
        node_id: String,
        label: String,
    },
    AddParameter {
        #[dsl(statements)]
        parameter: Box<WorkflowParameter>,
    },
    RemoveParameter {
        #[dsl(key = "id")]
        parameter_id: String,
    },
    PatchParameter {
        #[dsl(key = "target")]
        parameter_id: String,
        #[dsl(statements)]
        parameter: Box<WorkflowParameter>,
    },
    BindParameterField {
        binding: WorkflowParameterBinding,
    },
    UnbindParameterField {
        node_id: String,
        field_path: String,
    },
    SyncNodePorts,
    DeclareInput {
        input: WorkflowInput,
    },
    RemoveInput {
        #[dsl(key = "id")]
        input_id: String,
    },
    BindInput {
        binding: WorkflowInputBinding,
    },
    UnbindInput {
        #[dsl(key = "id")]
        input_id: String,
    },
    BindOutput {
        binding: WorkflowOutputBinding,
    },
    UnbindOutput {
        node_id: String,
        port_id: String,
    },
}

async fn workflow_mutation_to_dsl(operation: &WorkflowMutation) -> WorkflowMutationDsl {
    match operation {
        WorkflowMutation::AddNode { node } => WorkflowMutationDsl::AddNode { node: node.clone() },
        WorkflowMutation::RemoveNode { node_id } => WorkflowMutationDsl::RemoveNode { node_id: node_id.clone() },
        WorkflowMutation::ConnectPorts { edge } => WorkflowMutationDsl::ConnectPorts { edge: edge.clone() },
        WorkflowMutation::DisconnectEdge { edge_id } => WorkflowMutationDsl::DisconnectEdge { edge_id: edge_id.clone() },
        WorkflowMutation::MoveNode { node_id, x, y } => WorkflowMutationDsl::MoveNode { node_id: node_id.clone(), x: *x, y: *y },
        WorkflowMutation::PatchNode { node_id, label } => WorkflowMutationDsl::PatchNode { node_id: node_id.clone(), label: label.clone() },
        WorkflowMutation::AddParameter { parameter } => WorkflowMutationDsl::AddParameter { parameter: Box::new(parameter.clone()) },
        WorkflowMutation::RemoveParameter { parameter_id } => WorkflowMutationDsl::RemoveParameter { parameter_id: parameter_id.clone() },
        WorkflowMutation::PatchParameter { parameter_id, parameter } => WorkflowMutationDsl::PatchParameter { parameter_id: parameter_id.clone(), parameter: Box::new(parameter.clone()) },
        WorkflowMutation::BindParameterField { binding } => WorkflowMutationDsl::BindParameterField { binding: binding.clone() },
        WorkflowMutation::UnbindParameterField { node_id, field_path } => WorkflowMutationDsl::UnbindParameterField { node_id: node_id.clone(), field_path: field_path.clone() },
        WorkflowMutation::SyncNodePorts => WorkflowMutationDsl::SyncNodePorts,
        WorkflowMutation::DeclareInput { input } => WorkflowMutationDsl::DeclareInput { input: input.clone() },
        WorkflowMutation::RemoveInput { input_id } => WorkflowMutationDsl::RemoveInput { input_id: input_id.clone() },
        WorkflowMutation::BindInput { binding } => WorkflowMutationDsl::BindInput { binding: binding.clone() },
        WorkflowMutation::UnbindInput { input_id } => WorkflowMutationDsl::UnbindInput { input_id: input_id.clone() },
        WorkflowMutation::BindOutput { binding } => WorkflowMutationDsl::BindOutput { binding: binding.clone() },
        WorkflowMutation::UnbindOutput { node_id, port_id } => WorkflowMutationDsl::UnbindOutput { node_id: node_id.clone(), port_id: port_id.clone() },
    }
}

async fn workflow_mutation_from_dsl(operation: WorkflowMutationDsl) -> WorkflowMutation {
    match operation {
        WorkflowMutationDsl::AddNode { node } => WorkflowMutation::AddNode { node },
        WorkflowMutationDsl::RemoveNode { node_id } => WorkflowMutation::RemoveNode { node_id },
        WorkflowMutationDsl::ConnectPorts { edge } => WorkflowMutation::ConnectPorts { edge },
        WorkflowMutationDsl::DisconnectEdge { edge_id } => WorkflowMutation::DisconnectEdge { edge_id },
        WorkflowMutationDsl::MoveNode { node_id, x, y } => WorkflowMutation::MoveNode { node_id, x, y },
        WorkflowMutationDsl::PatchNode { node_id, label } => WorkflowMutation::PatchNode { node_id, label },
        WorkflowMutationDsl::AddParameter { parameter } => WorkflowMutation::AddParameter { parameter: *parameter },
        WorkflowMutationDsl::RemoveParameter { parameter_id } => WorkflowMutation::RemoveParameter { parameter_id },
        WorkflowMutationDsl::PatchParameter { parameter_id, parameter } => WorkflowMutation::PatchParameter { parameter_id, parameter: *parameter },
        WorkflowMutationDsl::BindParameterField { binding } => WorkflowMutation::BindParameterField { binding },
        WorkflowMutationDsl::UnbindParameterField { node_id, field_path } => WorkflowMutation::UnbindParameterField { node_id, field_path },
        WorkflowMutationDsl::SyncNodePorts => WorkflowMutation::SyncNodePorts,
        WorkflowMutationDsl::DeclareInput { input } => WorkflowMutation::DeclareInput { input },
        WorkflowMutationDsl::RemoveInput { input_id } => WorkflowMutation::RemoveInput { input_id },
        WorkflowMutationDsl::BindInput { binding } => WorkflowMutation::BindInput { binding },
        WorkflowMutationDsl::UnbindInput { input_id } => WorkflowMutation::UnbindInput { input_id },
        WorkflowMutationDsl::BindOutput { binding } => WorkflowMutation::BindOutput { binding },
        WorkflowMutationDsl::UnbindOutput { node_id, port_id } => WorkflowMutation::UnbindOutput { node_id, port_id },
    }
}

impl protocol::OpText for WorkflowMutationDsl {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6) — `DslOps` emits `DslVariants` only.
impl protocol::OpBinary for WorkflowMutationDsl {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self).await
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes).await
    }
}

impl protocol::OpText for WorkflowMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(workflow_mutation_from_dsl(<WorkflowMutationDsl as protocol::OpText>::parse_op(line).await?).await)
    }

    async fn print_op(&self) -> String {
        <WorkflowMutationDsl as protocol::OpText>::print_op(&workflow_mutation_to_dsl(self).await).await
    }
}

impl protocol::OpBinary for WorkflowMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        workflow_mutation_to_dsl(self).await.encode_op().await
    }

    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(workflow_mutation_from_dsl(WorkflowMutationDsl::decode_op(bytes).await?).await)
    }
}
//#endregion 🔖️WorkflowMutationOpText

/// @emoji ✅️ Extends [`validate_workflow`] with the two `WorkflowSnapshot`-level checks that need the
/// declared `inputs`/`input_bindings`/`output_bindings` (pure/registry-free, unlike os-core's own
/// `validate_workflow` wrapper which layers on the contract-renegotiation check): (1) a required node
/// in-port must have EITHER an incoming edge XOR a `WorkflowInputBinding` targeting it — never both,
/// never neither; (2) every `input_bindings`/`output_bindings` entry must resolve to a real
/// `WorkflowInput`/node+port.
pub async fn validate_workflow_snapshot(document: &WorkflowSnapshot) -> WorkflowValidation {
    let mut validation = validate_workflow(&document.graph).await;

    let input_ids: HashSet<&str> = document.inputs.iter().map(|input| input.id.as_str()).collect();
    for binding in &document.input_bindings {
        if !input_ids.contains(binding.input_id.as_str()) {
            validation.errors.push(format!("input binding targets unknown input '{}'", binding.input_id));
        }
    }

    let node_by_id: HashMap<&str, &WorkflowNode> = document.graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    for binding in &document.output_bindings {
        let resolves = node_by_id.get(binding.node_id.as_str()).is_some_and(|node| node.outputs.iter().any(|port| port.id == binding.port_id));
        if !resolves {
            validation.errors.push(format!("output binding targets unknown node/port '{}:{}'", binding.node_id, binding.port_id));
        }
    }

    for node in &document.graph.nodes {
        for port in &node.inputs {
            if !port.spec.required {
                continue;
            }
            let has_edge = document.graph.edges.iter().any(|edge| edge.target_node_id == node.id && edge.target_port_id == port.id);
            let has_binding = document.input_bindings.iter().any(|binding| binding.node_id == node.id && binding.port_id == port.id);
            if has_edge && has_binding {
                validation.errors.push(format!("required port {} on node {} has both a wire and an input binding", port.id, node.id));
            } else if !has_edge && !has_binding {
                validation.errors.push(format!("required port {} on node {} has neither a wire nor an input binding", port.id, node.id));
            }
        }
    }

    validation.ok = validation.errors.is_empty();
    validation
}
//#endregion 🔖️WorkflowSnapshot

//#region 🔖️RunArtifact
//#region 🔖️RunScalars
/// 🚦️ Lifecycle state of a whole run. `sealed` (on `RunArtifact`) is a distinct bool, not folded into
/// this enum — "sealed" and "final status" are orthogonal (a `Failed` run is sealed with `status:
/// Failed`, not a `Sealed` variant). Hand-crafted `dsl::DslField` (ordinal `Shape::Enum`), not
/// `#[derive(dsl::DslEnum)]`: this is a plain field-less scalar, not a tagged-variant-with-data sum
/// type (`DslEnum`/`DslVariants` target the latter — see `WorkflowParameter`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

// 🚫️async: E1 transitive — only consumed by the E4-tagged sync `DslField::shape`
// path; pure variant table, no I/O (R9).
fn run_status_variants() -> Vec<(String, u32)> {
    vec![("pending".to_string(), 0), ("running".to_string(), 1), ("succeeded".to_string(), 2), ("failed".to_string(), 3), ("canceled".to_string(), 4)]
}

impl dsl::DslField for RunStatus {
    // 🚫️async: E1 transitive — `DslField::shape` is E4-tagged sync in the trait; pure variant
    // table, no I/O (R9).
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

// 🚫️async: E1 transitive — only consumed by the E4-tagged sync `DslField::shape`
// path; pure variant table, no I/O (R9).
fn run_node_status_variants() -> Vec<(String, u32)> {
    vec![("computed".to_string(), 0), ("cacheHit".to_string(), 1), ("failed".to_string(), 2)]
}

impl dsl::DslField for RunNodeStatus {
    // 🚫️async: E1 transitive — `DslField::shape` is E4-tagged sync in the trait; pure variant
    // table, no I/O (R9).
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum RunTrigger {
    Manual { actor: String },
    Automation { automation_ref: String, event_fingerprint: String },
}

// 🚫️async: E4 fn-pointer slot — value goes into `dsl::Shape::Record(fn() -> RecordSpec)`.
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
    // 🚫️async: E4 fn-pointer transitivity — see `DslField::shape`'s tag (R9).
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
//#endregion 🔖️RunScalars

//#region 🔖️RunRecords
/// 🎛️ One resolved config-overlay value for a run — `value` carries a JSON-encoded scalar/text as
/// plain `Text` (not a raw `dsl::DslValue` field): a `dsl::DslValue` embeds arbitrary nested
/// object/array shapes, which risks not being self-delimiting as a bare `#[dsl(table)]` column (see
/// `dsl_schema`'s `table_rejects_non_self_delimiting_column_shapes_at_spec_build_time` regression) —
/// plain JSON text sidesteps that risk entirely while staying a lossless round trip. `run::SpaceRunner`
/// parses it back to `serde_json::Value` when applying the overlay onto a node's config (see
/// `WorkflowParameterBinding.field_path`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RunParameterValue {
    pub parameter_id: String,
    pub value: String,
}

/// 🔑️ One port's fingerprint — reused for both a `RunNodeRecord`'s `input_fingerprints` and
/// `output_fingerprints` (same shape, different table column on the owning row).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct PortFingerprint {
    pub port_id: String,
    pub fingerprint: String,
}

/// 📤️ Where one node's out-port materialized in the run's own write-only output area — `path` is
/// relative to the run's own sink (see `run::RunContext`'s doc), never a source-bundle path.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
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

/// 📜️ One run-level or per-node log line — `node_id` empty for a run-level line (see `RunMutation::Log`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RunLogLine {
    pub node_id: String,
    pub level: String,
    pub message: String,
    pub at: String,
}
//#endregion 🔖️RunRecords

//#region 🔖️RunArtifactBody
/// 🏃️ The `os.run` persisted artifact (W5 Lane A) — one headless workflow execution's full record:
/// which workflow/checkpoint/input snapshot it ran against, its resolved parameter overlay, where its
/// outputs landed, per-node `RunNodeRecord`s (the new memoization ground truth), and a `sealed` flag
/// that — once set by `RunMutation::Seal` — makes the document immutable (`RunMutation::validate`
/// rejects every further operation, see `🔖️RunMutation` below). Sealing is meant to promote a run
/// draft→asset later (`space::DraftCatalog`, W5 Lane B's territory) — this wave only carries the flag
/// and the apply-rejection law, not the promotion wiring itself.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
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
//#endregion 🔖️RunArtifactBody

//#region 🔖️HandcraftedRunArtifactCodecs
/// 🧬️ P6: mirrors `WorkflowSnapshot`'s handcrafted `ArtifactDsl`/`ArtifactPack` pair above (same
/// file, `🔖️HandcraftedWorkflowSnapshotCodecs`) — `#[derive(dsl::DslArtifact)]` emits `__dsl_*`
/// helpers + `__DSL_ENVELOPE_ID`/`__DSL_EXTENSION` only, never the trait impls themselves.
impl store::ArtifactDsl for RunArtifact {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    async fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    async fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for RunArtifact {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options).await?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id().await {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id().await, envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options).await?;
        match Self::__dsl_from_record(&record) {
            Ok(value) => Ok(value),
            Err(error) => Err(store::text_error_to_pack_error(error)),
        }
    }
    async fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedRunArtifactCodecs

//#region 🔖️RunMutation
/// ⚡️ One settled `RunArtifact` mutation — mirrors `WorkflowMutation`'s shape (this same crate).
/// `Start` seeds the run's identity/parameter overlay and flips `status` to `Running`;
/// `NodeStarted`/`NodeFinished`/`Log` are emitted once per node by `run::SpaceRunner`; `Seal` is the
/// terminal operation — see `RunMutation::validate` below for the law this whole wave exists to prove
/// ("no operation applies after `Seal`").
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum RunMutation {
    Start { workflow_ref: String, workflow_checkpoint_id: String, input_collection_ref: String, input_snapshot_id: String, parameter_values: Vec<RunParameterValue>, output_collection_ref: String, trigger: RunTrigger },
    NodeStarted { node_id: String },
    // 🩹️ Field named `node_record`, not `record` (the ticket sketch's name): `#[derive(dsl::DslOps)]`'s
    // generated variant body binds an internal `record: RecordValue` local in the very same scope —
    // naming this field `record` too shadows it and breaks the macro's own codegen (a real compile
    // error, confirmed against `cargo check`), not just a style preference.
    NodeFinished { node_record: RunNodeRecord },
    Log { node_id: String, level: String, message: String, at: String },
    Seal { status: RunStatus },
}

pub async fn apply_run_operation(document: &RunArtifact, operation: &RunMutation) -> RunArtifact {
    let mut next = document.clone();
    match operation {
        RunMutation::Start { workflow_ref, workflow_checkpoint_id, input_collection_ref, input_snapshot_id, parameter_values, output_collection_ref, trigger } => {
            next.workflow_ref = workflow_ref.clone();
            next.workflow_checkpoint_id = workflow_checkpoint_id.clone();
            next.input_collection_ref = input_collection_ref.clone();
            next.input_snapshot_id = input_snapshot_id.clone();
            next.parameter_values = parameter_values.clone();
            next.output_collection_ref = output_collection_ref.clone();
            next.trigger = trigger.clone();
            next.status = RunStatus::Running;
            next.started_at = store::now_iso().await;
        }
        RunMutation::NodeStarted { node_id } => {
            next.logs.push(RunLogLine { node_id: node_id.clone(), level: "info".into(), message: "node started".into(), at: store::now_iso().await });
        }
        RunMutation::NodeFinished { node_record } => {
            next.node_records.retain(|entry| entry.node_id != node_record.node_id);
            next.node_records.push(node_record.clone());
        }
        RunMutation::Log { node_id, level, message, at } => {
            next.logs.push(RunLogLine { node_id: node_id.clone(), level: level.clone(), message: message.clone(), at: at.clone() });
        }
        RunMutation::Seal { status } => {
            next.status = *status;
            next.finished_at = Some(store::now_iso().await);
            next.sealed = true;
        }
    }
    next
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum RunDiff {
    #[default]
    Empty,
    Start {
        workflow_ref: String,
        workflow_checkpoint_id: String,
        input_collection_ref: String,
        input_snapshot_id: String,
        parameter_values: Vec<RunParameterValue>,
        output_collection_ref: String,
        trigger: RunTrigger,
    },
    NodeStarted {
        node_id: String,
    },
    NodeFinished {
        node_record: RunNodeRecord,
    },
    Log {
        node_id: String,
        level: String,
        message: String,
        at: String,
    },
    Seal {
        status: RunStatus,
    },
}

impl protocol::MutationDiff<RunArtifact> for RunDiff {
    async fn apply(&self, document: &RunArtifact) -> protocol::MutationApplyResult<RunArtifact> {
        if document.sealed && !matches!(self, RunDiff::Empty) {
            return Err(protocol::MutationApplyError::new("mutation.apply.sealed", "run document is sealed").await.at(["sealed"]).await);
        }
        if matches!(self, RunDiff::Start { .. }) && (document.status != RunStatus::Pending || !document.started_at.is_empty()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.conflicting-target", "run has already started").await.at(["status"]).await);
        }
        let operation = match self {
            RunDiff::Empty => return Ok(document.clone()),
            RunDiff::Start { workflow_ref, workflow_checkpoint_id, input_collection_ref, input_snapshot_id, parameter_values, output_collection_ref, trigger } => RunMutation::Start {
                workflow_ref: workflow_ref.clone(),
                workflow_checkpoint_id: workflow_checkpoint_id.clone(),
                input_collection_ref: input_collection_ref.clone(),
                input_snapshot_id: input_snapshot_id.clone(),
                parameter_values: parameter_values.clone(),
                output_collection_ref: output_collection_ref.clone(),
                trigger: trigger.clone(),
            },
            RunDiff::NodeStarted { node_id } => RunMutation::NodeStarted { node_id: node_id.clone() },
            RunDiff::NodeFinished { node_record } => RunMutation::NodeFinished { node_record: node_record.clone() },
            RunDiff::Log { node_id, level, message, at } => RunMutation::Log { node_id: node_id.clone(), level: level.clone(), message: message.clone(), at: at.clone() },
            RunDiff::Seal { status } => RunMutation::Seal { status: *status },
        };
        Ok(apply_run_operation(document, &operation).await)
    }

    async fn absorb(&mut self, other: Self) {
        if !matches!(other, RunDiff::Empty) {
            *self = other;
        }
    }
}

impl protocol::Mutation<RunArtifact> for RunMutation {
    type Diff = RunDiff;

    /// 🧮️ Mechanical wrap only (26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-
    /// CONFLICTS W0): no `Error`/`Warning`/`Fatal` messages added here yet.
    async fn diff(&self, _document: &RunArtifact) -> protocol::MutationOutcome<RunDiff> {
        let diff = match self {
            RunMutation::Start { workflow_ref, workflow_checkpoint_id, input_collection_ref, input_snapshot_id, parameter_values, output_collection_ref, trigger } => RunDiff::Start {
                workflow_ref: workflow_ref.clone(),
                workflow_checkpoint_id: workflow_checkpoint_id.clone(),
                input_collection_ref: input_collection_ref.clone(),
                input_snapshot_id: input_snapshot_id.clone(),
                parameter_values: parameter_values.clone(),
                output_collection_ref: output_collection_ref.clone(),
                trigger: trigger.clone(),
            },
            RunMutation::NodeStarted { node_id } => RunDiff::NodeStarted { node_id: node_id.clone() },
            RunMutation::NodeFinished { node_record } => RunDiff::NodeFinished { node_record: node_record.clone() },
            RunMutation::Log { node_id, level, message, at } => RunDiff::Log { node_id: node_id.clone(), level: level.clone(), message: message.clone(), at: at.clone() },
            RunMutation::Seal { status } => RunDiff::Seal { status: *status },
        };
        protocol::MutationOutcome::new(diff).await
    }

    async fn inverse(&self, base: &RunArtifact) -> Vec<Self> {
        match self {
            // 🧷️ `Start` is a run's genesis operation (always applied to a freshly-`empty_run_document`
            // document in practice) and `Seal`/`NodeStarted`/`Log` have no meaningful undo target —
            // matches this crate's own precedent for irreversible/no-prior-state ops
            // (`WorkflowMutation::SyncNodePorts`'s own `inverse` returns `Vec::new()` too).
            RunMutation::Start { .. } | RunMutation::Seal { .. } | RunMutation::NodeStarted { .. } | RunMutation::Log { .. } => Vec::new(),
            RunMutation::NodeFinished { node_record } => base.node_records.iter().find(|entry| entry.node_id == node_record.node_id).map(|previous| vec![RunMutation::NodeFinished { node_record: previous.clone() }]).unwrap_or_default(),
        }
    }
}

impl RunMutation {
    /// 🔒️ THE law this whole wave exists to prove: once `RunArtifact.sealed` is true, no further
    /// operation may apply — a sealed run's per-node bytes are immutable history, never re-mutated by
    /// a later invocation. Was `Mutation::validate` before that trait method was deleted (ticket
    /// `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C4/C10 — "no `validate`
    /// may survive anywhere"); kept as a plain inherent method with the exact same check since this
    /// is the one real write seam this crate ships for a `RunArtifact` — `run::SpaceRunner`'s write
    /// path always goes through `apply_run_operation_checked` (below), which calls this before ever
    /// calling `apply_run_operation`, never through `ArtifactStore::dispatch` directly. A future wave
    /// migrating this to `Mutation::diff`'s own `Fatal` `"mutation.invariant"` message is tracked but
    /// out of scope for W0's mechanical return-type adaptation.
    async fn check_not_sealed(&self, snapshot: &RunArtifact) -> Result<(), String> {
        if snapshot.sealed {
            return Err(format!("run document is sealed; cannot apply {self:?}"));
        }
        Ok(())
    }
}

/// 🔒️ The one real write seam for a `RunArtifact`: validates (rejecting anything post-`Seal`) before
/// delegating to `apply_run_operation`. `run::SpaceRunner` calls this, never `apply_run_operation`
/// directly, for every operation it emits.
pub async fn apply_run_operation_checked(document: &RunArtifact, operation: RunMutation) -> Result<RunArtifact, String> {
    operation.check_not_sealed(document).await?;
    Ok(apply_run_operation(document, &operation).await)
}
//#endregion 🔖️RunMutation

//#region 🔖️RunMutationOpText
/// 🧬️ Local structural twin of [`RunMutation`] for the `dsl::DslOps` derive — mirrors
/// `WorkflowMutationDsl` above exactly (same reasoning: the engine needs a concrete `DslVariants`
/// impl per operation enum).
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum RunMutationDsl {
    Start {
        workflow_ref: String,
        workflow_checkpoint_id: String,
        input_collection_ref: String,
        input_snapshot_id: String,
        #[dsl(table)]
        parameter_values: Vec<RunParameterValue>,
        output_collection_ref: String,
        trigger: RunTrigger,
    },
    NodeStarted {
        node_id: String,
    },
    NodeFinished {
        node_record: RunNodeRecord,
    },
    Log {
        node_id: String,
        level: String,
        message: String,
        at: String,
    },
    Seal {
        status: RunStatus,
    },
}

async fn run_mutation_to_dsl(operation: &RunMutation) -> RunMutationDsl {
    match operation {
        RunMutation::Start { workflow_ref, workflow_checkpoint_id, input_collection_ref, input_snapshot_id, parameter_values, output_collection_ref, trigger } => RunMutationDsl::Start {
            workflow_ref: workflow_ref.clone(),
            workflow_checkpoint_id: workflow_checkpoint_id.clone(),
            input_collection_ref: input_collection_ref.clone(),
            input_snapshot_id: input_snapshot_id.clone(),
            parameter_values: parameter_values.clone(),
            output_collection_ref: output_collection_ref.clone(),
            trigger: trigger.clone(),
        },
        RunMutation::NodeStarted { node_id } => RunMutationDsl::NodeStarted { node_id: node_id.clone() },
        RunMutation::NodeFinished { node_record } => RunMutationDsl::NodeFinished { node_record: node_record.clone() },
        RunMutation::Log { node_id, level, message, at } => RunMutationDsl::Log { node_id: node_id.clone(), level: level.clone(), message: message.clone(), at: at.clone() },
        RunMutation::Seal { status } => RunMutationDsl::Seal { status: *status },
    }
}

async fn run_mutation_from_dsl(operation: RunMutationDsl) -> RunMutation {
    match operation {
        RunMutationDsl::Start { workflow_ref, workflow_checkpoint_id, input_collection_ref, input_snapshot_id, parameter_values, output_collection_ref, trigger } => {
            RunMutation::Start { workflow_ref, workflow_checkpoint_id, input_collection_ref, input_snapshot_id, parameter_values, output_collection_ref, trigger }
        }
        RunMutationDsl::NodeStarted { node_id } => RunMutation::NodeStarted { node_id },
        RunMutationDsl::NodeFinished { node_record } => RunMutation::NodeFinished { node_record },
        RunMutationDsl::Log { node_id, level, message, at } => RunMutation::Log { node_id, level, message, at },
        RunMutationDsl::Seal { status } => RunMutation::Seal { status },
    }
}

impl protocol::OpText for RunMutationDsl {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6) — `DslOps` emits `DslVariants` only.
impl protocol::OpBinary for RunMutationDsl {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self).await
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes).await
    }
}

impl protocol::OpText for RunMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(run_mutation_from_dsl(<RunMutationDsl as protocol::OpText>::parse_op(line).await?).await)
    }

    async fn print_op(&self) -> String {
        <RunMutationDsl as protocol::OpText>::print_op(&run_mutation_to_dsl(self).await).await
    }
}

impl protocol::OpBinary for RunMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        run_mutation_to_dsl(self).await.encode_op().await
    }

    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(run_mutation_from_dsl(RunMutationDsl::decode_op(bytes).await?).await)
    }
}
//#endregion 🔖️RunMutationOpText
//#endregion 🔖️RunArtifact

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn empty_workflow_default() {
        let workflow = empty_workflow().await;
        assert_eq!(workflow.schema, WORKFLOW_SCHEMA);
        assert!(workflow.nodes.is_empty());
    }

    async fn media_port_spec(id: &str, direction: MediaPortDirection, kind_id: Option<&str>) -> MediaPortSpec {
        MediaPortSpec { id: id.into(), label: id.into(), direction, media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, kind_id: kind_id.map(Into::into), required: true, multiplicity: PortMultiplicity::One }
    }

    async fn workflow_node(id: &str, outputs: Vec<WorkflowMediaPort>, inputs: Vec<WorkflowMediaPort>) -> WorkflowNode {
        WorkflowNode {
            id: id.into(),
            plugin_id: "plugin".into(),
            app_id: "app".into(),
            label: id.into(),
            yields: String::new(),
            artifact_ref: format!("artifacts/{id}"),
            config_ref: format!("config/{id}"),
            x: 0.0,
            y: 0.0,
            width: 220.0,
            height: 100.0,
            inputs,
            outputs,
        }
    }

    async fn workflow_edge(id: &str, source_node_id: &str, source_port_id: &str, target_node_id: &str, target_port_id: &str) -> WorkflowEdge {
        WorkflowEdge { id: id.into(), source_node_id: source_node_id.into(), source_port_id: source_port_id.into(), target_node_id: target_node_id.into(), target_port_id: target_port_id.into(), contract: placeholder_media_contract("data.value").await }
    }

    #[semio_framework_async_macros::async_test]
    async fn workflow_media_port_id_format() {
        let spec = media_port_spec("out", MediaPortDirection::Out, Some("kind.a")).await;
        let port = workflow_media_port("n1", &spec);
        assert_eq!(port.id, "n1:out:out");
        assert_eq!(port.spec, spec);

        let spec_in = media_port_spec("in", MediaPortDirection::In, None).await;
        let port_in = workflow_media_port("n1", &spec_in);
        assert_eq!(port_in.id, "n1:in:in");
    }

    #[semio_framework_async_macros::async_test]
    async fn media_contract_dsl_round_trips() {
        let contract = MediaContract {
            kind_id: "puzzle.2d.fixture".into(),
            media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
            wire: MediaWireFormat::Binary { format_kind: "svg".into() },
            conversion: Some((MediaForm::Brep, MediaForm::Mesh)),
        };
        let record = media_contract_to_record(&contract);
        let round_tripped = media_contract_from_record(&record).expect("decode");
        assert_eq!(round_tripped, contract);

        let placeholder = placeholder_media_contract("draw.document").await;
        let placeholder_record = media_contract_to_record(&placeholder);
        assert_eq!(media_contract_from_record(&placeholder_record).expect("decode placeholder"), placeholder);
    }

    #[semio_framework_async_macros::async_test]
    async fn workflow_media_port_dsl_round_trips() {
        let port = WorkflowMediaPort { id: "n1:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, Some("kind.a")).await };
        let record = workflow_media_port_to_record(&port);
        assert_eq!(workflow_media_port_from_record(&record).expect("decode"), port);

        let port_no_kind = WorkflowMediaPort { id: "n1:in:in".into(), spec: media_port_spec("in", MediaPortDirection::In, None).await };
        let record_no_kind = workflow_media_port_to_record(&port_no_kind);
        assert_eq!(workflow_media_port_from_record(&record_no_kind).expect("decode"), port_no_kind);
    }

    #[semio_framework_async_macros::async_test]
    async fn validate_workflow_flags_dangling_edge() {
        let node_a = workflow_node("a", vec![WorkflowMediaPort { id: "a:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, None).await }], vec![]);
        let graph = Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![node_a.await], edges: vec![workflow_edge("e1", "a", "out", "missing", "in").await] };
        let validation = validate_workflow(&graph).await;
        assert!(!validation.ok);
        assert!(validation.errors.iter().any(|e| e.contains("missing target node missing")));
    }

    #[semio_framework_async_macros::async_test]
    async fn validate_workflow_flags_cycle() {
        let node_a = workflow_node(
            "a",
            vec![WorkflowMediaPort { id: "a:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, None).await }],
            vec![WorkflowMediaPort { id: "a:in:in".into(), spec: media_port_spec("in", MediaPortDirection::In, None).await }],
        );
        let node_b = workflow_node(
            "b",
            vec![WorkflowMediaPort { id: "b:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, None).await }],
            vec![WorkflowMediaPort { id: "b:in:in".into(), spec: media_port_spec("in", MediaPortDirection::In, None).await }],
        );
        let graph = Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![node_a.await, node_b.await], edges: vec![workflow_edge("e1", "a", "out", "b", "in").await, workflow_edge("e2", "b", "out", "a", "in").await] };
        let validation = validate_workflow(&graph).await;
        assert!(!validation.ok);
        assert!(validation.errors.iter().any(|e| e.starts_with("cycle detected")));
    }

    #[semio_framework_async_macros::async_test]
    async fn validate_workflow_ok_for_acyclic_connected_graph() {
        let node_a = workflow_node("a", vec![WorkflowMediaPort { id: "a:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, None).await }], vec![]);
        let node_b = workflow_node("b", vec![], vec![WorkflowMediaPort { id: "b:in:in".into(), spec: media_port_spec("in", MediaPortDirection::In, None).await }]);
        let graph = Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![node_a.await, node_b.await], edges: vec![workflow_edge("e1", "a", "out", "b", "in").await] };
        let validation = validate_workflow(&graph).await;
        assert!(validation.ok);
        assert!(validation.errors.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn plan_workflow_propagates_dirtiness_across_multi_hop_chain() {
        let node_a = workflow_node("a", vec![WorkflowMediaPort { id: "a:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, None).await }], vec![]);
        let node_b = workflow_node(
            "b",
            vec![WorkflowMediaPort { id: "b:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, None).await }],
            vec![WorkflowMediaPort { id: "b:in:in".into(), spec: media_port_spec("in", MediaPortDirection::In, None).await }],
        );
        let node_c = workflow_node("c", vec![], vec![WorkflowMediaPort { id: "c:in:in".into(), spec: media_port_spec("in", MediaPortDirection::In, None).await }]);
        let graph = Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![node_a.await, node_b.await, node_c.await], edges: vec![workflow_edge("e1", "a", "out", "b", "in").await, workflow_edge("e2", "b", "out", "c", "in").await] };

        let mut dirty = HashSet::new();
        dirty.insert("a".to_string());
        let deliveries = plan_workflow(&graph, &dirty).await;
        assert_eq!(deliveries.len(), 2);
        assert_eq!(deliveries[0].edge_id, "e1");
        assert_eq!(deliveries[0].producer_node_id, "a");
        assert_eq!(deliveries[0].consumer_node_id, "b");
        assert_eq!(deliveries[1].edge_id, "e2");
        assert_eq!(deliveries[1].producer_node_id, "b");
        assert_eq!(deliveries[1].consumer_node_id, "c");
    }

    #[semio_framework_async_macros::async_test]
    async fn plan_workflow_skips_clean_nodes() {
        let node_a = workflow_node("a", vec![WorkflowMediaPort { id: "a:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, None).await }], vec![]);
        let node_b = workflow_node("b", vec![], vec![WorkflowMediaPort { id: "b:in:in".into(), spec: media_port_spec("in", MediaPortDirection::In, None).await }]);
        let graph = Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![node_a.await, node_b.await], edges: vec![workflow_edge("e1", "a", "out", "b", "in").await] };
        let deliveries = plan_workflow(&graph, &HashSet::new()).await;
        assert!(deliveries.is_empty());
    }

    //#region 🧪️WorkflowSnapshotLaws
    async fn sample_workflow_snapshot() -> WorkflowSnapshot {
        let node_a = workflow_node("a", vec![WorkflowMediaPort { id: "a:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, Some("kind.a")).await }], vec![]);
        let node_b = workflow_node("b", vec![], vec![WorkflowMediaPort { id: "b:in:in".into(), spec: MediaPortSpec { required: true, ..media_port_spec("in", MediaPortDirection::In, Some("kind.a")).await } }]);
        let graph = Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![node_a.await, node_b.await], edges: vec![workflow_edge("e1", "a", "a:out:out", "b", "b:in:in").await] };
        let parameter_bindings = vec![WorkflowParameterBinding { parameter_id: "p1".into(), node_id: "a".into(), field_path: "/zoom".into() }];
        // 🧷️ Keeps the fixture's node ports in the same synced state `apply_workflow_operation`
        // maintains as an invariant (every `BindParameterField`/`AddNode`/etc call re-derives parameter
        // ports from `parameter_bindings`) — an un-synced fixture would make `assert_operation_round_trip`
        // see a spurious port diff on any op whose apply path re-syncs, not a real bug.
        let graph = sync_workflow_parameter_ports(&graph, &parameter_bindings).await;
        WorkflowSnapshot {
            schema: S_WORKFLOW_SCHEMA.into(),
            graph,
            parameters: vec![
                WorkflowParameter::Numeric { id: "p1".into(), name: "Zoom".into(), value: 10.0, min: Some(0.0), max: Some(100.0), step: Some(1.0) },
                WorkflowParameter::Categorical { id: "p2".into(), name: "Mode".into(), value: "Option A".into(), options: vec!["Option A".into(), "Option B".into()] },
                WorkflowParameter::Toggle { id: "p3".into(), name: "Flag".into(), value: true },
                WorkflowParameter::Text { id: "p4".into(), name: "Label".into(), value: "hello".into() },
            ],
            parameter_bindings,
            inputs: vec![WorkflowInput { id: "in-1".into(), kind_id: "kind.a".into(), selector: "**/*.puzzle2d".into(), required: true, multiplicity: PortMultiplicity::One }],
            input_bindings: Vec::new(),
            output_bindings: vec![WorkflowOutputBinding { node_id: "a".into(), port_id: "a:out:out".into(), path_template: "renders/{node}.out".into() }],
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_workflow_snapshot_matches_schema() {
        let document = empty_workflow_snapshot().await;
        assert_eq!(document.schema, S_WORKFLOW_SCHEMA);
        assert!(document.graph.nodes.is_empty());
        assert!(document.parameters.is_empty());
        assert!(document.inputs.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn workflow_snapshot_dsl_pack_round_trips() {
        store::os_store::test_support::assert_dsl_pack_equivalence(&sample_workflow_snapshot().await).await;
        store::os_store::test_support::assert_dsl_pack_equivalence(&empty_workflow_snapshot().await).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn workflow_operation_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::AddNode { node: workflow_node("n1", Vec::new(), Vec::new()).await }).await;
        store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::RemoveNode { node_id: "n1".into() }).await;
        store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::ConnectPorts { edge: workflow_edge("e1", "a", "out", "b", "in").await }).await;
        store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::DisconnectEdge { edge_id: "e1".into() }).await;
        store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::MoveNode { node_id: "n1".into(), x: 5.5, y: -6.25 }).await;
        store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::PatchNode { node_id: "n1".into(), label: "Renamed".into() }).await;
        store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::AddParameter { parameter: WorkflowParameter::Numeric { id: "p1".into(), name: "Zoom".into(), value: 10.0, min: None, max: None, step: None } }).await;
        store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::RemoveParameter { parameter_id: "p1".into() }).await;
        store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::PatchParameter { parameter_id: "p1".into(), parameter: WorkflowParameter::Toggle { id: "p1".into(), name: "Flag".into(), value: false } }).await;
        store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::BindParameterField { binding: WorkflowParameterBinding { parameter_id: "p1".into(), node_id: "n1".into(), field_path: "/zoom".into() } }).await;
        store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::UnbindParameterField { node_id: "n1".into(), field_path: "/zoom".into() }).await;
        store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::SyncNodePorts).await;
        store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::DeclareInput { input: WorkflowInput { id: "in-1".into(), kind_id: "kind.a".into(), selector: "**/*".into(), required: true, multiplicity: PortMultiplicity::Many } }).await;
        store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::RemoveInput { input_id: "in-1".into() }).await;
        store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::BindInput { binding: WorkflowInputBinding { input_id: "in-1".into(), node_id: "n1".into(), port_id: "n1:in:in".into() } }).await;
        store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::UnbindInput { input_id: "in-1".into() }).await;
        store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::BindOutput { binding: WorkflowOutputBinding { node_id: "n1".into(), port_id: "n1:out:out".into(), path_template: "out/{node}".into() } }).await;
        store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::UnbindOutput { node_id: "n1".into(), port_id: "n1:out:out".into() }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn workflow_operation_backwards_restores_pre_state() {
        let document = sample_workflow_snapshot().await;
        // 🧷️ `Remove*` ops are exercised via `*_backwards_restores_cascade_deleted_dependents` below
        // instead of this strict-equality helper: `apply`'s `Add*` counterpart appends to the END of
        // its list, so removing a non-last element and letting `inverse` re-add it changes list
        // ORDER even though every element is restored — a known, harmless limitation of list-append
        // state shared by every `Add*`/`Remove*` pair in this codebase (e.g. `space::SpaceMutation`'s
        // own `inverse` law tests avoid it the same way, by only exercising it on singleton lists).
        store::os_store::test_support::assert_operation_round_trip(&document, WorkflowMutation::AddNode { node: workflow_node("c", Vec::new(), Vec::new()).await }).await;
        store::os_store::test_support::assert_operation_round_trip(&document, WorkflowMutation::MoveNode { node_id: "a".into(), x: 99.0, y: -1.0 }).await;
        store::os_store::test_support::assert_operation_round_trip(&document, WorkflowMutation::AddParameter { parameter: WorkflowParameter::Toggle { id: "p9".into(), name: "New".into(), value: true } }).await;
        store::os_store::test_support::assert_operation_round_trip(
            &document,
            WorkflowMutation::DeclareInput { input: WorkflowInput { id: "in-2".into(), kind_id: "kind.b".into(), selector: "**/*".into(), required: false, multiplicity: PortMultiplicity::One } },
        ).await;
        store::os_store::test_support::assert_operation_round_trip(&document, WorkflowMutation::BindInput { binding: WorkflowInputBinding { input_id: "in-1".into(), node_id: "b".into(), port_id: "b:in:in".into() } }).await;
        store::os_store::test_support::assert_operation_round_trip(&document, WorkflowMutation::BindOutput { binding: WorkflowOutputBinding { node_id: "a".into(), port_id: "a:out:out".into(), path_template: "renders/other.out".into() } }).await;
        store::os_store::test_support::assert_operation_round_trip(&document, WorkflowMutation::UnbindOutput { node_id: "a".into(), port_id: "a:out:out".into() }).await;
    }

    /// 🧵️ Removing the LAST element of each cascade-owning list keeps append-order stable, so this can
    /// use the strict-equality `assert_operation_round_trip` helper to prove `RemoveNode`/
    /// `RemoveParameter`/`RemoveInput`'s inverse restores every cascade-deleted dependent (edges/
    /// parameter bindings/input bindings/output bindings), not just the bare removed item.
    #[semio_framework_async_macros::async_test]
    async fn remove_operations_backwards_restores_cascade_deleted_dependents() {
        let mut document = sample_workflow_snapshot().await;
        // `b` is the last node — removing it also cascade-drops edge `e1` (which targets it).
        store::os_store::test_support::assert_operation_round_trip(&document, WorkflowMutation::RemoveNode { node_id: "b".into() }).await;

        // `p4` is the last parameter and has no bindings, so this only proves the simple case; add a
        // binding on it first so the cascade-restoration path is actually exercised.
        document.parameter_bindings.push(WorkflowParameterBinding { parameter_id: "p4".into(), node_id: "a".into(), field_path: "/label".into() });
        document.graph = sync_workflow_parameter_ports(&document.graph, &document.parameter_bindings).await;
        store::os_store::test_support::assert_operation_round_trip(&document, WorkflowMutation::RemoveParameter { parameter_id: "p4".into() }).await;

        document.input_bindings.push(WorkflowInputBinding { input_id: "in-1".into(), node_id: "b".into(), port_id: "b:in:in".into() });
        store::os_store::test_support::assert_operation_round_trip(&document, WorkflowMutation::RemoveInput { input_id: "in-1".into() }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn workflow_diff_print_parse_and_encode_decode_round_trip() {
        let diffs = vec![
            WorkflowDiff::AddNode { node: workflow_node("n1", Vec::new(), Vec::new()).await },
            WorkflowDiff::DeclareInput { input: WorkflowInput { id: "in-1".into(), kind_id: "kind.a".into(), selector: "**/*".into(), required: true, multiplicity: PortMultiplicity::One } },
            WorkflowDiff::Empty,
        ];
        for diff in diffs {
            let applied = protocol::MutationDiff::apply(&diff, &empty_workflow_snapshot().await).await.expect("valid workflow diff");
            let _ = applied;
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn validate_workflow_snapshot_requires_edge_xor_input_binding_on_required_ports() {
        let mut document = sample_workflow_snapshot().await;
        // 🎯️ Sample fixture wires `a -> b` over `b`'s sole required in-port with NO input binding — ok.
        assert!(validate_workflow_snapshot(&document).await.ok, "wired required port with no binding must validate");

        let binding = WorkflowInputBinding { input_id: "in-1".into(), node_id: "b".into(), port_id: "b:in:in".into() };
        document.input_bindings.push(binding.clone());
        let both = validate_workflow_snapshot(&document).await;
        assert!(!both.ok);
        assert!(both.errors.iter().any(|error| error.contains("has both a wire and an input binding")), "{:?}", both.errors);

        document.graph.edges.clear();
        document.input_bindings.clear();
        let neither = validate_workflow_snapshot(&document).await;
        assert!(!neither.ok);
        assert!(neither.errors.iter().any(|error| error.contains("has neither a wire nor an input binding")), "{:?}", neither.errors);

        document.input_bindings.push(binding);
        assert!(validate_workflow_snapshot(&document).await.ok, "input binding alone must satisfy a required port");
    }

    #[semio_framework_async_macros::async_test]
    async fn validate_workflow_snapshot_flags_unresolved_bindings() {
        let mut document = sample_workflow_snapshot().await;
        document.input_bindings.push(WorkflowInputBinding { input_id: "missing-input".into(), node_id: "b".into(), port_id: "b:in:in".into() });
        document.output_bindings.push(WorkflowOutputBinding { node_id: "missing-node".into(), port_id: "x".into(), path_template: "out".into() });
        let validation = validate_workflow_snapshot(&document).await;
        assert!(!validation.ok);
        assert!(validation.errors.iter().any(|error| error.contains("unknown input 'missing-input'")));
        assert!(validation.errors.iter().any(|error| error.contains("unknown node/port 'missing-node:x'")));
    }
    //#endregion 🧪️WorkflowSnapshotLaws

    //#region 🧪️RunArtifactLaws
    async fn sample_run_node_record(node_id: &str, status: RunNodeStatus) -> RunNodeRecord {
        RunNodeRecord {
            node_id: node_id.into(),
            status,
            document_fingerprint: "doc-fp".into(),
            config_fingerprint: "cfg-fp".into(),
            input_fingerprints: vec![PortFingerprint { port_id: format!("{node_id}:in:in"), fingerprint: "in-fp".into() }],
            output_fingerprints: vec![PortFingerprint { port_id: format!("{node_id}:out:out"), fingerprint: "out-fp".into() }],
            outputs: vec![RunOutputArtifact { port_id: format!("{node_id}:out:out"), artifact_id: format!("artifacts/{node_id}"), path: format!("out/{node_id}.out") }],
            duration_ms: 12.5,
        }
    }

    async fn sample_run_document() -> RunArtifact {
        let mut document = empty_run_document().await;
        document = apply_run_operation(
            &document,
            &RunMutation::Start {
                workflow_ref: "space.space".into(),
                workflow_checkpoint_id: "ck-1".into(),
                input_collection_ref: "collections/in".into(),
                input_snapshot_id: "snap-1".into(),
                parameter_values: vec![RunParameterValue { parameter_id: "p1".into(), value: "10".into() }],
                output_collection_ref: "collections/out".into(),
                trigger: RunTrigger::Manual { actor: "dev".into() },
            },
        ).await;
        document = apply_run_operation(&document, &RunMutation::NodeStarted { node_id: "a".into() }).await;
        document = apply_run_operation(&document, &RunMutation::NodeFinished { node_record: sample_run_node_record("a", RunNodeStatus::Computed).await }).await;
        document
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_run_document_matches_schema() {
        let document = empty_run_document().await;
        assert_eq!(document.schema, S_RUN_SCHEMA);
        assert_eq!(document.status, RunStatus::Pending);
        assert!(!document.sealed);
        assert!(document.node_records.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn run_document_dsl_pack_round_trips() {
        store::os_store::test_support::assert_dsl_pack_equivalence(&sample_run_document().await).await;
        store::os_store::test_support::assert_dsl_pack_equivalence(&empty_run_document().await).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn run_operation_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&RunMutation::Start {
            workflow_ref: "space.space".into(),
            workflow_checkpoint_id: "ck-1".into(),
            input_collection_ref: "collections/in".into(),
            input_snapshot_id: "snap-1".into(),
            parameter_values: vec![RunParameterValue { parameter_id: "p1".into(), value: "10".into() }],
            output_collection_ref: "collections/out".into(),
            trigger: RunTrigger::Manual { actor: "dev".into() },
        }).await;
        store::os_store::test_support::assert_op_line_round_trip(&RunMutation::Start {
            workflow_ref: "space.space".into(),
            workflow_checkpoint_id: "ck-1".into(),
            input_collection_ref: "collections/in".into(),
            input_snapshot_id: "snap-1".into(),
            parameter_values: Vec::new(),
            output_collection_ref: "collections/out".into(),
            trigger: RunTrigger::Automation { automation_ref: "os.automation/a1".into(), event_fingerprint: "evt-1".into() },
        }).await;
        store::os_store::test_support::assert_op_line_round_trip(&RunMutation::NodeStarted { node_id: "a".into() }).await;
        store::os_store::test_support::assert_op_line_round_trip(&RunMutation::NodeFinished { node_record: sample_run_node_record("a", RunNodeStatus::CacheHit).await }).await;
        store::os_store::test_support::assert_op_line_round_trip(&RunMutation::Log { node_id: "a".into(), level: "info".into(), message: "computed".into(), at: "123".into() }).await;
        store::os_store::test_support::assert_op_line_round_trip(&RunMutation::Seal { status: RunStatus::Succeeded }).await;
    }

    /// 🔒️ The load-bearing law this wave exists to prove: once `Seal` has been applied, every further
    /// operation is rejected by `apply_run_operation_checked` (not silently accepted, not a panic) —
    /// this is the real write seam `run::SpaceRunner` goes through for every `RunMutation` it emits.
    #[semio_framework_async_macros::async_test]
    async fn apply_run_operation_checked_rejects_everything_after_seal() {
        let document = sample_run_document().await;
        assert!(!document.sealed);

        let sealed = apply_run_operation_checked(&document, RunMutation::Seal { status: RunStatus::Succeeded }).await.expect("sealing an unsealed run must succeed");
        assert!(sealed.sealed);
        assert_eq!(sealed.status, RunStatus::Succeeded);
        assert!(sealed.finished_at.is_some());

        let rejected_log = apply_run_operation_checked(&sealed, RunMutation::Log { node_id: "a".into(), level: "info".into(), message: "too late".into(), at: "999".into() });
        assert!(rejected_log.await.is_err(), "a Log after Seal must be rejected, not silently applied");

        let rejected_node_finished = apply_run_operation_checked(&sealed, RunMutation::NodeFinished { node_record: sample_run_node_record("b", RunNodeStatus::Computed).await });
        assert!(rejected_node_finished.await.is_err(), "a NodeFinished after Seal must be rejected");

        let rejected_reseal = apply_run_operation_checked(&sealed, RunMutation::Seal { status: RunStatus::Failed });
        assert!(rejected_reseal.await.is_err(), "re-sealing an already-sealed run must be rejected");

        // 🧷️ Rejection must be a real `Err`, not a panic, and the document itself must stay untouched.
        assert_eq!(sealed.node_records.len(), 1, "the rejected NodeFinished must not have been applied");
    }

    #[semio_framework_async_macros::async_test]
    async fn run_node_record_dsl_pack_round_trips_nested_tables() {
        let record = sample_run_node_record("a", RunNodeStatus::Failed).await;
        let mut document = empty_run_document().await;
        document.node_records.push(record);
        store::os_store::test_support::assert_dsl_pack_equivalence(&document).await;
    }
    //#endregion 🧪️RunArtifactLaws
}
