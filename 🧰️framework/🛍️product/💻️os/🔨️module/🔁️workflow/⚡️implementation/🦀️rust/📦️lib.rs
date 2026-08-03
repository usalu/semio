//! 🕸️ Persisted app-node workflow graph — nodes reference plugin apps plus document/config artifact refs.

// #region 🔖️InstanceIdentity
//! 🪪️ A `WorkflowNode.id` **is** the app-instance identity now — there is no separate instance
//! record layered on top of it. This crate becomes the single persisted-graph source of truth once
//! `framework/product/os/core`'s `OsProjection` is updated to embed `Workflow` directly instead of
//! its own in-file `OsWorkflow`/`OsAppInstance` pair (future work, not this ticket).
// #endregion 🔖️InstanceIdentity

use semio_framework_core::{AppDefinition, MediaClass, MediaForm, MediaPortDirection, MediaPortSpec, MediaType, MediaWireFormat, OsMediaFormat, PortMultiplicity};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub const WORKFLOW_SCHEMA: &str = "workflow.graph";

//#region 🔖️MediaContract
/// 🤝️ A connect-time negotiated wire contract between two `WorkflowMediaPort`s — stored on
/// `WorkflowEdge` so later passes (`validate_workflow`, merge reconciliation) can re-check it
/// without re-resolving the artifact registry. `kind_id`/`media_type` describe the *accepted*
/// (target) side — see `semio_framework_core::media_types_compatible`. Ported down from
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
pub fn placeholder_media_contract(kind_id: &str) -> MediaContract {
    MediaContract { kind_id: kind_id.into(), media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, wire: MediaWireFormat::Document { schema: kind_id.into() }, conversion: None }
}
//#endregion 🔖️MediaContract

//#region 🔖️MediaContractDsl
/// 🧬️ Hand-crafted `dsl::DslField` for `MediaContract` (instead of `#[derive(dsl::DslRecord)]`) —
/// see the `dsl::` conversion cheat sheet's tuple-field guidance. `conversion: Option<(MediaForm,
/// MediaForm)>` has no derivable shape (raw Rust tuples don't implement `dsl::DslField`), and
/// `media_type`/`wire` point at plain-data types from `semio_framework_core` that this crate can't
/// implement `dsl::DslField` for under the orphan rule (neither the trait nor the type is local
/// here). Since `MediaContract` itself IS local, hand-writing its own impl sidesteps both problems
/// at once: every foreign sub-value (`MediaClass`/`MediaForm`/`OsMediaFormat`) is bridged directly
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

fn media_class_variants() -> Vec<(String, u32)> {
    vec![
        ("twoD".to_string(), 0),
        ("threeD".to_string(), 1),
        ("text".to_string(), 2),
        ("data".to_string(), 3),
        ("graph".to_string(), 4),
        ("kit".to_string(), 5),
        ("computation".to_string(), 6),
        ("presentation".to_string(), 7),
    ]
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
        MediaWireFormat::Binary { format } => {
            record.fields.insert(3, dsl::FieldValue::Text("binary".to_string()));
            record.fields.insert(4, dsl::FieldValue::Text(format.as_str().to_string()));
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
            let format = OsMediaFormat::parse(&format_word).ok_or_else(|| dsl::__rt::field_error(format!("unknown wire format '{format_word}'")))?;
            MediaWireFormat::Binary { format }
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
/// declaration it was instantiated from (`semio_framework_core::MediaPortSpec`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowMediaPort {
    pub id: String,
    pub spec: MediaPortSpec,
}

/// 🧬️ Hand-crafted `dsl::DslField` for `WorkflowMediaPort` — `spec: MediaPortSpec` points at a
/// plain-data type from `semio_framework_core`, which this crate can't implement `dsl::DslField`
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

fn port_multiplicity_variants() -> Vec<(String, u32)> {
    vec![("one".to_string(), 0), ("many".to_string(), 1)]
}

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
    pub document_ref: String,
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "workflow", layout = "lines")]
pub struct Workflow {
    pub schema: String,
    pub nodes: Vec<WorkflowNode>,
    pub edges: Vec<WorkflowEdge>,
}

pub fn empty_workflow() -> Workflow {
    Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: Vec::new(), edges: Vec::new() }
}

/// 🔌️ Instantiates one `WorkflowMediaPort` from an app-level `MediaPortSpec`, scoping its wire id to
/// this node (`"{node_id}:{spec.id}:{in|out}"`) so the same port declaration produces distinct wire
/// endpoints per node instance.
fn workflow_media_port(node_id: &str, spec: &MediaPortSpec) -> WorkflowMediaPort {
    let direction_word = match spec.direction {
        MediaPortDirection::In => "in",
        MediaPortDirection::Out => "out",
    };
    WorkflowMediaPort { id: format!("{node_id}:{}:{}", spec.id, direction_word), spec: spec.clone() }
}

/// 🧩️ Builds a workflow node shell from a manifest app definition so every app is instantiable as a node.
pub fn workflow_node_for_app(app: &AppDefinition, plugin_id: &str, node_id: &str, position: &WorkflowPosition) -> WorkflowNode {
    let all_ports = app.io.all_ports();
    let inputs: Vec<WorkflowMediaPort> = all_ports.iter().filter(|spec| spec.direction == MediaPortDirection::In).map(|spec| workflow_media_port(node_id, spec)).collect();
    let outputs: Vec<WorkflowMediaPort> = all_ports.iter().filter(|spec| spec.direction == MediaPortDirection::Out).map(|spec| workflow_media_port(node_id, spec)).collect();
    let yields = outputs.first().and_then(|port| port.spec.kind_id.clone()).unwrap_or_default();
    let port_count = inputs.len().max(outputs.len()).max(1);
    let height = position.height.max(56.0 + port_count as f64 * 18.0);
    WorkflowNode {
        id: node_id.into(),
        plugin_id: plugin_id.into(),
        app_id: app.id.clone(),
        label: app.label.clone(),
        yields,
        document_ref: format!("documents/{node_id}"),
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
pub fn validate_workflow(graph: &Workflow) -> WorkflowValidation {
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
    fn dfs(node_id: &str, adjacency: &HashMap<String, Vec<String>>, visiting: &mut HashSet<String>, visited: &mut HashSet<String>, errors: &mut Vec<String>) {
        if visited.contains(node_id) {
            return;
        }
        if visiting.contains(node_id) {
            errors.push(format!("cycle detected at {node_id}"));
            return;
        }
        visiting.insert(node_id.to_string());
        for next in adjacency.get(node_id).into_iter().flatten() {
            dfs(next, adjacency, visiting, visited, errors);
        }
        visiting.remove(node_id);
        visited.insert(node_id.to_string());
    }
    for node in &graph.nodes {
        dfs(&node.id, &adjacency, &mut visiting, &mut visited, &mut errors);
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
fn workflow_topological_node_order(graph: &Workflow) -> Vec<String> {
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    for edge in &graph.edges {
        adjacency.entry(edge.source_node_id.clone()).or_default().push(edge.target_node_id.clone());
    }
    let mut visited = HashSet::new();
    let mut order = Vec::new();
    fn dfs(node_id: &str, adjacency: &HashMap<String, Vec<String>>, visited: &mut HashSet<String>, order: &mut Vec<String>) {
        if !visited.insert(node_id.to_string()) {
            return;
        }
        for next in adjacency.get(node_id).into_iter().flatten() {
            dfs(next, adjacency, visited, order);
        }
        order.push(node_id.to_string());
    }
    for node in &graph.nodes {
        dfs(&node.id, &adjacency, &mut visited, &mut order);
    }
    order.reverse();
    order
}

/// @emoji 🚚️ Plans one [`WorkflowDelivery`] per edge in the downstream closure of `dirty_node_ids`,
/// propagating dirtiness onto each edge's consumer node so multi-hop chains (A→B→C) resolve in a
/// single topological pass. Pure/side-effect-free — callers own applying the deliveries.
pub fn plan_workflow(graph: &Workflow, dirty_node_ids: &HashSet<String>) -> Vec<WorkflowDelivery> {
    let node_by_id: HashMap<&str, &WorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    let mut edges_by_source: HashMap<&str, Vec<&WorkflowEdge>> = HashMap::new();
    for edge in &graph.edges {
        edges_by_source.entry(edge.source_node_id.as_str()).or_default().push(edge);
    }
    let order = workflow_topological_node_order(graph);
    let mut dirty = dirty_node_ids.clone();
    let mut deliveries = Vec::new();
    for node_id in &order {
        let Some(node) = node_by_id.get(node_id.as_str()) else { continue };
        if !dirty.contains(node.id.as_str()) {
            continue;
        }
        for edge in edges_by_source.get(node_id.as_str()).into_iter().flatten() {
            let Some(target_node) = node_by_id.get(edge.target_node_id.as_str()) else { continue };
            deliveries.push(WorkflowDelivery {
                edge_id: edge.id.clone(),
                producer_node_id: node.id.clone(),
                producer_port_id: edge.source_port_id.clone(),
                consumer_node_id: target_node.id.clone(),
                consumer_port_id: edge.target_port_id.clone(),
            });
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
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
//#endregion 🔖️WorkflowFixture

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_workflow_default() {
        let workflow = empty_workflow();
        assert_eq!(workflow.schema, WORKFLOW_SCHEMA);
        assert!(workflow.nodes.is_empty());
    }

    fn media_port_spec(id: &str, direction: MediaPortDirection, kind_id: Option<&str>) -> MediaPortSpec {
        MediaPortSpec { id: id.into(), label: id.into(), direction, media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, kind_id: kind_id.map(Into::into), required: true, multiplicity: PortMultiplicity::One }
    }

    fn workflow_node(id: &str, outputs: Vec<WorkflowMediaPort>, inputs: Vec<WorkflowMediaPort>) -> WorkflowNode {
        WorkflowNode { id: id.into(), plugin_id: "plugin".into(), app_id: "app".into(), label: id.into(), yields: String::new(), document_ref: format!("documents/{id}"), config_ref: format!("config/{id}"), x: 0.0, y: 0.0, width: 220.0, height: 100.0, inputs, outputs }
    }

    fn workflow_edge(id: &str, source_node_id: &str, source_port_id: &str, target_node_id: &str, target_port_id: &str) -> WorkflowEdge {
        WorkflowEdge { id: id.into(), source_node_id: source_node_id.into(), source_port_id: source_port_id.into(), target_node_id: target_node_id.into(), target_port_id: target_port_id.into(), contract: placeholder_media_contract("data.value") }
    }

    #[test]
    fn workflow_media_port_id_format() {
        let spec = media_port_spec("out", MediaPortDirection::Out, Some("kind.a"));
        let port = workflow_media_port("n1", &spec);
        assert_eq!(port.id, "n1:out:out");
        assert_eq!(port.spec, spec);

        let spec_in = media_port_spec("in", MediaPortDirection::In, None);
        let port_in = workflow_media_port("n1", &spec_in);
        assert_eq!(port_in.id, "n1:in:in");
    }

    #[test]
    fn media_contract_dsl_round_trips() {
        let contract = MediaContract { kind_id: "puzzle.2d.fixture".into(), media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, wire: MediaWireFormat::Binary { format: OsMediaFormat::Svg }, conversion: Some((MediaForm::Brep, MediaForm::Mesh)) };
        let record = media_contract_to_record(&contract);
        let round_tripped = media_contract_from_record(&record).expect("decode");
        assert_eq!(round_tripped, contract);

        let placeholder = placeholder_media_contract("draw.document");
        let placeholder_record = media_contract_to_record(&placeholder);
        assert_eq!(media_contract_from_record(&placeholder_record).expect("decode placeholder"), placeholder);
    }

    #[test]
    fn workflow_media_port_dsl_round_trips() {
        let port = WorkflowMediaPort { id: "n1:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, Some("kind.a")) };
        let record = workflow_media_port_to_record(&port);
        assert_eq!(workflow_media_port_from_record(&record).expect("decode"), port);

        let port_no_kind = WorkflowMediaPort { id: "n1:in:in".into(), spec: media_port_spec("in", MediaPortDirection::In, None) };
        let record_no_kind = workflow_media_port_to_record(&port_no_kind);
        assert_eq!(workflow_media_port_from_record(&record_no_kind).expect("decode"), port_no_kind);
    }

    #[test]
    fn validate_workflow_flags_dangling_edge() {
        let node_a = workflow_node("a", vec![WorkflowMediaPort { id: "a:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, None) }], vec![]);
        let graph = Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![node_a], edges: vec![workflow_edge("e1", "a", "out", "missing", "in")] };
        let validation = validate_workflow(&graph);
        assert!(!validation.ok);
        assert!(validation.errors.iter().any(|e| e.contains("missing target node missing")));
    }

    #[test]
    fn validate_workflow_flags_cycle() {
        let node_a = workflow_node("a", vec![WorkflowMediaPort { id: "a:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, None) }], vec![WorkflowMediaPort { id: "a:in:in".into(), spec: media_port_spec("in", MediaPortDirection::In, None) }]);
        let node_b = workflow_node("b", vec![WorkflowMediaPort { id: "b:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, None) }], vec![WorkflowMediaPort { id: "b:in:in".into(), spec: media_port_spec("in", MediaPortDirection::In, None) }]);
        let graph = Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![node_a, node_b], edges: vec![workflow_edge("e1", "a", "out", "b", "in"), workflow_edge("e2", "b", "out", "a", "in")] };
        let validation = validate_workflow(&graph);
        assert!(!validation.ok);
        assert!(validation.errors.iter().any(|e| e.starts_with("cycle detected")));
    }

    #[test]
    fn validate_workflow_ok_for_acyclic_connected_graph() {
        let node_a = workflow_node("a", vec![WorkflowMediaPort { id: "a:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, None) }], vec![]);
        let node_b = workflow_node("b", vec![], vec![WorkflowMediaPort { id: "b:in:in".into(), spec: media_port_spec("in", MediaPortDirection::In, None) }]);
        let graph = Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![node_a, node_b], edges: vec![workflow_edge("e1", "a", "out", "b", "in")] };
        let validation = validate_workflow(&graph);
        assert!(validation.ok);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn plan_workflow_propagates_dirtiness_across_multi_hop_chain() {
        let node_a = workflow_node("a", vec![WorkflowMediaPort { id: "a:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, None) }], vec![]);
        let node_b = workflow_node("b", vec![WorkflowMediaPort { id: "b:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, None) }], vec![WorkflowMediaPort { id: "b:in:in".into(), spec: media_port_spec("in", MediaPortDirection::In, None) }]);
        let node_c = workflow_node("c", vec![], vec![WorkflowMediaPort { id: "c:in:in".into(), spec: media_port_spec("in", MediaPortDirection::In, None) }]);
        let graph = Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![node_a, node_b, node_c], edges: vec![workflow_edge("e1", "a", "out", "b", "in"), workflow_edge("e2", "b", "out", "c", "in")] };

        let mut dirty = HashSet::new();
        dirty.insert("a".to_string());
        let deliveries = plan_workflow(&graph, &dirty);
        assert_eq!(deliveries.len(), 2);
        assert_eq!(deliveries[0].edge_id, "e1");
        assert_eq!(deliveries[0].producer_node_id, "a");
        assert_eq!(deliveries[0].consumer_node_id, "b");
        assert_eq!(deliveries[1].edge_id, "e2");
        assert_eq!(deliveries[1].producer_node_id, "b");
        assert_eq!(deliveries[1].consumer_node_id, "c");
    }

    #[test]
    fn plan_workflow_skips_clean_nodes() {
        let node_a = workflow_node("a", vec![WorkflowMediaPort { id: "a:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, None) }], vec![]);
        let node_b = workflow_node("b", vec![], vec![WorkflowMediaPort { id: "b:in:in".into(), spec: media_port_spec("in", MediaPortDirection::In, None) }]);
        let graph = Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![node_a, node_b], edges: vec![workflow_edge("e1", "a", "out", "b", "in")] };
        let deliveries = plan_workflow(&graph, &HashSet::new());
        assert!(deliveries.is_empty());
    }
}
