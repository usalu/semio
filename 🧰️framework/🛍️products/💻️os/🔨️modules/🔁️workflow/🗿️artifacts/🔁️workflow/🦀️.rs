//! 🕸️ Persisted app-node workflow graph — nodes reference plugin apps plus document/config artifact refs.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "♻️retirement/🦀️.rs"]
mod retirement;

use semio_framework::{AppDefinition, MediaClass, MediaForm, MediaPortDirection, MediaPortSpec, MediaType, MediaWireFormat, PortMultiplicity};
use semio_framework::{Locale, Terminology};
use std::collections::{HashMap, HashSet};

pub const WORKFLOW_SCHEMA: &str = "workflow.graph";

/// 🪶️ W3 "the inversion" document schema — the persisted `os.workflow` artifact (graph + parameters +
/// inputs/bindings, see [`WorkflowSnapshot`]), distinct from `WORKFLOW_SCHEMA` (the bare graph-only
/// sub-shape still embedded as `WorkflowSnapshot.graph`). Registered as a builtin artifact kind by
/// os-core's `seed_builtin_artifact_kinds`.
pub const S_WORKFLOW_SCHEMA: &str = "os.workflow";

//#region 🔖️MediaContract
/// 🤝️ A connect-time negotiated wire contract between two `WorkflowMediaPort`s — stored on
/// `WorkflowEdge` so later passes (`validate_workflow`, merge reconciliation) can re-check it
/// without re-resolving the artifact registry. `kind_id`/`media_type` describe the *accepted*
/// (target) side — see `semio_framework::media_types_compatible`. Ported down from
/// `framework/product/os/core`'s `workflow` module (`MediaContract`) so the persisted graph carries
/// its own edge contracts; the negotiation logic itself (`negotiate_media_contract`) stays in
/// os-core for now since it needs the artifact-kind registry, which doesn't exist at this layer yet.
#[derive(Clone, Debug, PartialEq)]
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
        MediaForm::Procedure => 14,
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
        14 => MediaForm::Procedure,
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

/// 🔀️ Hand-written, not derived: `conversion: Option<(MediaForm, MediaForm)>` is a raw Rust tuple,
/// which has no blanket `ToValue`/`FromValue` (same reasoning as this type's hand-crafted
/// `dsl::DslField` above) — bridged directly as a two-element `DslValue::Array`. `kind_id`/
/// `media_type`/`wire` reuse their own `ToValue`/`FromValue` impls (`MediaType`/`MediaWireFormat`
/// gained them in `🛂️manifest/🦀️.rs`).
impl ::semio_framework_os_kernel::ToValue for MediaContract {
    fn to_value(&self) -> ::semio_framework_os_kernel::DslValue {
        ::semio_framework_os_kernel::DslValue::object([
            ("kindId".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.kind_id)),
            ("mediaType".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.media_type)),
            ("wire".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.wire)),
            (
                "conversion".to_string(),
                match &self.conversion {
                    Some((from, to)) => ::semio_framework_os_kernel::DslValue::Array(vec![::semio_framework_os_kernel::ToValue::to_value(from), ::semio_framework_os_kernel::ToValue::to_value(to)]),
                    None => ::semio_framework_os_kernel::DslValue::Null,
                },
            ),
        ])
    }
}
impl ::semio_framework_os_kernel::FromValue for MediaContract {
    fn from_value(value: ::semio_framework_os_kernel::DslValue) -> Result<Self, ::semio_framework_os_kernel::ValueError> {
        let ::semio_framework_os_kernel::DslValue::Object(fields) = value else {
            return Err(::semio_framework_os_kernel::ValueError::new(format!("expected an object for MediaContract, found {value:?}")));
        };
        let mut kind_id = None;
        let mut media_type = None;
        let mut wire = None;
        let mut conversion = None;
        for (key, entry) in fields {
            match key.as_str() {
                "kindId" => kind_id = Some(<String as ::semio_framework_os_kernel::FromValue>::from_value(entry).map_err(|e| e.under("kindId"))?),
                "mediaType" => media_type = Some(<MediaType as ::semio_framework_os_kernel::FromValue>::from_value(entry).map_err(|e| e.under("mediaType"))?),
                "wire" => wire = Some(<MediaWireFormat as ::semio_framework_os_kernel::FromValue>::from_value(entry).map_err(|e| e.under("wire"))?),
                "conversion" => {
                    conversion = Some(match entry {
                        ::semio_framework_os_kernel::DslValue::Null => None,
                        ::semio_framework_os_kernel::DslValue::Array(items) => {
                            let mut iter = items.into_iter();
                            let from = iter.next().ok_or_else(|| ::semio_framework_os_kernel::ValueError::new("MediaContract.conversion missing from"))?;
                            let to = iter.next().ok_or_else(|| ::semio_framework_os_kernel::ValueError::new("MediaContract.conversion missing to"))?;
                            Some((
                                <MediaForm as ::semio_framework_os_kernel::FromValue>::from_value(from).map_err(|e| e.under("conversion.0"))?,
                                <MediaForm as ::semio_framework_os_kernel::FromValue>::from_value(to).map_err(|e| e.under("conversion.1"))?,
                            ))
                        }
                        other => return Err(::semio_framework_os_kernel::ValueError::new(format!("expected array or null for MediaContract.conversion, found {other:?}"))),
                    });
                }
                _ => {}
            }
        }
        Ok(MediaContract {
            kind_id: kind_id.ok_or_else(|| ::semio_framework_os_kernel::ValueError::new("MediaContract missing kindId"))?,
            media_type: media_type.ok_or_else(|| ::semio_framework_os_kernel::ValueError::new("MediaContract missing mediaType"))?,
            wire: wire.ok_or_else(|| ::semio_framework_os_kernel::ValueError::new("MediaContract missing wire"))?,
            conversion: conversion.unwrap_or(None),
        })
    }
}
//#endregion 🔖️MediaContractDsl

//#region 🔖️WorkflowMediaPort
/// 🔌️ One instance-scoped wire endpoint on a `WorkflowNode` — `id` is unique within the graph
/// (`"{node_id}:{spec.id}:{in|out}"`, see `workflow_media_port`), `spec` is the app-level port
/// declaration it was instantiated from (`semio_framework::MediaPortSpec`).
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct WorkflowPosition {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// 🧷️ A node IS the app-instance now — see the `🔖️InstanceIdentity` region at the top of this file.
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct WorkflowEdge {
    pub id: String,
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
    #[dsl(block)]
    pub contract: MediaContract,
}

#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase")]
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

impl store::ArtifactPack for WorkflowFixture {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
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

#[derive(Clone, Debug, PartialEq, Eq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[value(rename_all = "lowercase")]
pub enum WorkflowParameterType {
    Numeric,
    Categorical,
    Toggle,
    Text,
}

/// 🎯️ `field_path` names a `ConfigFieldSpec.key` in the target node's app's declared `ConfigSpec` —
/// see `validate_workflow_parameter_config_binding` (type-checks against the field's `ConfigFieldShape`).
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct WorkflowParameterBinding {
    pub parameter_id: String,
    pub node_id: String,
    pub field_path: String,
}

#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, dsl::DslEnum)]
#[value(tag = "type", rename_all = "lowercase")]
pub enum WorkflowParameter {
    Numeric { id: String, name: String, value: f64, min: Option<f64>, max: Option<f64>, step: Option<f64> },
    Categorical { id: String, name: String, value: String, options: Vec<String> },
    Toggle { id: String, name: String, value: bool },
    Text { id: String, name: String, value: String },
}

pub type WorkflowParameterPatch = dsl::DslValue;

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
pub fn workflow_parameter_value(parameter: &WorkflowParameter) -> dsl::DslValue {
    match parameter {
        WorkflowParameter::Numeric { value, .. } => dsl::ToValue::to_value(value),
        WorkflowParameter::Categorical { value, .. } => dsl::ToValue::to_value(value),
        WorkflowParameter::Toggle { value, .. } => dsl::ToValue::to_value(value),
        WorkflowParameter::Text { value, .. } => dsl::ToValue::to_value(value),
    }
}

pub async fn workflow_parameter_types_compatible(left: &WorkflowParameterType, right: &WorkflowParameterType) -> bool {
    left == right
}

pub async fn create_default_workflow_parameter(parameter_type: &WorkflowParameterType, name: &str, id: Option<&str>) -> WorkflowParameter {
    let parameter_id = id.map_or_else(create_workflow_parameter_id, str::to_string);
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
pub async fn patch_workflow_parameter(parameter: &WorkflowParameter, patch: &dsl::DslValue) -> WorkflowParameter {
    let name = patch.get("name").and_then(|v| v.as_str()).map_or_else(|| workflow_parameter_name(parameter), str::to_string);
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
            let options = patch.get("options").and_then(|v| v.as_array()).map_or(current_options, |entries| entries.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect::<Vec<_>>());
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
        let value = patch.get("value").and_then(|v| v.as_str()).map_or(current_value, str::to_string);
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
/// `🏪️store/🔄️sync/🦀️.rs`'s `ArtifactEvent::Conflict(MutationMessage)` for the identical
/// precedent).
pub async fn validate_workflow_parameter_config_binding(binding: &WorkflowParameterBinding, parameter_type: &WorkflowParameterType, config_spec: &semio_framework::ConfigSpec) -> Result<(), protocol::MutationMessage> {
    let uri = format!("{}#{}", binding.node_id, binding.field_path);
    let Some(field) = config_spec.fields.iter().find(|field| field.key == binding.field_path) else {
        return Err(protocol::MutationMessage {
            level: dsl::Severity::Warning,
            code: dsl::FaultCode::new("workflow/parameter-binding-invalid"),
            message: format!("binding targets config field '{}', which the app's ConfigSpec does not declare", binding.field_path),
            target: vec![uri],
            op_index: None,
        });
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
        Err(protocol::MutationMessage {
            level: dsl::Severity::Warning,
            code: dsl::FaultCode::new("workflow/parameter-binding-invalid"),
            message: format!("parameter type {parameter_type:?} cannot drive config field '{}' ({:?})", binding.field_path, field.shape),
            target: vec![uri],
            op_index: None,
        })
    }
}

// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
pub fn workflow_parameter_port_id(node_id: &str, parameter_id: &str) -> String {
    media_port_id_for_spec(node_id, &format!("{WORKFLOW_PARAMETER_PORT_PREFIX}{parameter_id}"), "in")
}

// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
pub fn is_workflow_parameter_port_id(port_id: &str) -> bool {
    media_port_spec_id(port_id).is_some_and(|spec_id| spec_id.starts_with(WORKFLOW_PARAMETER_PORT_PREFIX))
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

pub fn sync_workflow_parameter_ports(graph: &Workflow, bindings: &[WorkflowParameterBinding]) -> Workflow {
    Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: graph.nodes.iter().map(|node| sync_workflow_node_parameter_ports(node, bindings)).collect(), edges: graph.edges.clone() }
}
//#endregion 🔖️WorkflowParameters

//#region 🔖️WorkflowSnapshot
/// 🔌️ One declared collection-level input slot a workflow's nodes can bind an in-port to — `selector`
/// is a glob matched against collection entry paths at run time (W5's `SpaceRunner` job); this crate
/// only carries the declaration + validates bindings resolve (`validate_workflow_snapshot`).
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
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
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, dsl::DslRecord)]
pub struct WorkflowInputBinding {
    pub input_id: String,
    pub node_id: String,
    pub port_id: String,
}

/// 📤️ Names where a node's out-port materializes in the output collection — `path_template` like
/// `"renders/{node}/{input.stem}.{ext}"` (resolved at run time by W5's `SpaceRunner`).
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, dsl::DslRecord)]
pub struct WorkflowOutputBinding {
    pub node_id: String,
    pub port_id: String,
    pub path_template: String,
}

/// 🕸️ The `os.workflow` persisted artifact — a non-destructive pipeline of connected apps (`graph`),
/// its parameters/bindings, and its declared collection-level inputs/outputs. Absorbs os-core's
/// dissolved `OsSnapshot` (`programs` moved to `space::SpaceSnapshot`, `active_plugin_id`/
/// `active_alternative_id` become space-app session state — see `## The inversion` in the plan).
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, dsl::DslArtifact)]
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
impl store::ArtifactPack for WorkflowSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
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
//#endregion 🔖️HandcraftedWorkflowSnapshotCodecs

// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
fn workflow_parameter_entity_id(parameter: &WorkflowParameter) -> &str {
    workflow_parameter_id(parameter)
}

//#region 🔖️WorkflowMutation
#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod workflow_mutations;
pub use workflow_mutations::{
    AddInput, AddNode, AddParameter, BindInput, BindOutput, BindParameterField, ChangeParameter, ConnectPorts, DisconnectEdge, MoveNode, RemoveInput, RemoveNode, RemoveParameter, RenameNode, UnbindInput, UnbindOutput, UnbindParameterField,
    UpdateNodePorts, WorkflowMutation,
};

pub fn apply_workflow_operation(document: &WorkflowSnapshot, operation: &WorkflowMutation) -> WorkflowSnapshot {
    let mut next = document.clone();
    match operation {
        WorkflowMutation::AddNode(AddNode { node }) => {
            let node = sync_workflow_node_parameter_ports(node, &next.parameter_bindings);
            next.graph.nodes.push(node);
        }
        WorkflowMutation::RemoveNode(RemoveNode { node_id }) => {
            next.parameter_bindings.retain(|binding| binding.node_id != *node_id);
            next.input_bindings.retain(|binding| binding.node_id != *node_id);
            next.output_bindings.retain(|binding| binding.node_id != *node_id);
            next.graph.nodes.retain(|node| node.id != *node_id);
            next.graph.edges.retain(|edge| edge.source_node_id != *node_id && edge.target_node_id != *node_id);
        }
        WorkflowMutation::ConnectPorts(ConnectPorts { edge }) => next.graph.edges.push(edge.clone()),
        WorkflowMutation::DisconnectEdge(DisconnectEdge { edge_id }) => next.graph.edges.retain(|edge| edge.id != *edge_id),
        WorkflowMutation::MoveNode(MoveNode { node_id, x, y }) => {
            for node in &mut next.graph.nodes {
                if node.id == *node_id {
                    node.x = *x;
                    node.y = *y;
                }
            }
        }
        WorkflowMutation::RenameNode(RenameNode { node_id, label }) => {
            for node in &mut next.graph.nodes {
                if node.id == *node_id {
                    node.label = label.clone();
                }
            }
        }
        WorkflowMutation::AddParameter(AddParameter { parameter }) => next.parameters.push((**parameter).clone()),
        WorkflowMutation::RemoveParameter(RemoveParameter { parameter_id }) => {
            next.parameters.retain(|parameter| workflow_parameter_entity_id(parameter) != *parameter_id);
            next.parameter_bindings.retain(|binding| binding.parameter_id != *parameter_id);
            next.graph = sync_workflow_parameter_ports(&next.graph, &next.parameter_bindings);
        }
        WorkflowMutation::ChangeParameter(ChangeParameter { parameter_id, parameter }) => {
            for entry in &mut next.parameters {
                if workflow_parameter_entity_id(entry) == *parameter_id {
                    *entry = (**parameter).clone();
                }
            }
        }
        WorkflowMutation::BindParameterField(BindParameterField { binding }) => {
            next.parameter_bindings.retain(|entry| !(entry.node_id == binding.node_id && entry.field_path == binding.field_path));
            next.parameter_bindings.push(binding.clone());
            next.graph = sync_workflow_parameter_ports(&next.graph, &next.parameter_bindings);
        }
        WorkflowMutation::UnbindParameterField(UnbindParameterField { node_id, field_path }) => {
            next.parameter_bindings.retain(|binding| !(binding.node_id == *node_id && binding.field_path == *field_path));
            next.graph = sync_workflow_parameter_ports(&next.graph, &next.parameter_bindings);
        }
        WorkflowMutation::UpdateNodePorts(UpdateNodePorts {}) => {
            next.graph = sync_workflow_parameter_ports(&next.graph, &next.parameter_bindings);
        }
        WorkflowMutation::AddInput(AddInput { input }) => next.inputs.push(input.clone()),
        WorkflowMutation::RemoveInput(RemoveInput { input_id }) => {
            next.inputs.retain(|input| input.id != *input_id);
            next.input_bindings.retain(|binding| binding.input_id != *input_id);
        }
        WorkflowMutation::BindInput(BindInput { binding }) => {
            next.input_bindings.retain(|entry| entry.input_id != binding.input_id);
            next.input_bindings.push(binding.clone());
        }
        WorkflowMutation::UnbindInput(UnbindInput { input_id }) => {
            next.input_bindings.retain(|binding| binding.input_id != *input_id);
        }
        WorkflowMutation::BindOutput(BindOutput { binding }) => {
            next.output_bindings.retain(|entry| !(entry.node_id == binding.node_id && entry.port_id == binding.port_id));
            next.output_bindings.push(binding.clone());
        }
        WorkflowMutation::UnbindOutput(UnbindOutput { node_id, port_id }) => {
            next.output_bindings.retain(|binding| !(binding.node_id == *node_id && binding.port_id == *port_id));
        }
    }
    next
}

#[derive(Clone, Debug, Default, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
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
    fn apply(&self, document: &WorkflowSnapshot) -> protocol::MutationApplyResult<WorkflowSnapshot> {
        match self {
            WorkflowDiff::Empty | WorkflowDiff::SyncNodePorts => {}
            WorkflowDiff::AddNode { node } => {
                if document.graph.nodes.iter().any(|entry| entry.id == node.id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "workflow node identity already exists").at(["nodes", node.id.as_str()]));
                }
            }
            WorkflowDiff::RemoveNode { node_id } | WorkflowDiff::MoveNode { node_id, .. } | WorkflowDiff::PatchNode { node_id, .. } => {
                if !document.graph.nodes.iter().any(|node| node.id == *node_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow node does not exist").at(["nodes", node_id.as_str()]));
                }
            }
            WorkflowDiff::ConnectPorts { edge } => {
                if document.graph.edges.iter().any(|entry| entry.id == edge.id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "workflow edge identity already exists").at(["edges", edge.id.as_str()]));
                }
                let source = match document.graph.nodes.iter().find(|node| node.id == edge.source_node_id) {
                    Some(node) => node,
                    None => return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow edge source node does not exist").at(["edges", edge.id.as_str(), "sourceNodeId"])),
                };
                if !source.outputs.iter().any(|port| port.id == edge.source_port_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow edge source port does not exist").at(["edges", edge.id.as_str(), "sourcePortId"]));
                }
                let target = match document.graph.nodes.iter().find(|node| node.id == edge.target_node_id) {
                    Some(node) => node,
                    None => return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow edge target node does not exist").at(["edges", edge.id.as_str(), "targetNodeId"])),
                };
                if !target.inputs.iter().any(|port| port.id == edge.target_port_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow edge target port does not exist").at(["edges", edge.id.as_str(), "targetPortId"]));
                }
            }
            WorkflowDiff::DisconnectEdge { edge_id } => {
                if !document.graph.edges.iter().any(|edge| edge.id == *edge_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow edge does not exist").at(["edges", edge_id.as_str()]));
                }
            }
            WorkflowDiff::AddParameter { parameter } => {
                let parameter_id = workflow_parameter_entity_id(parameter);
                if document.parameters.iter().any(|entry| workflow_parameter_entity_id(entry) == parameter_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "workflow parameter identity already exists").at(["parameters", parameter_id]));
                }
            }
            WorkflowDiff::RemoveParameter { parameter_id } => {
                if !document.parameters.iter().any(|parameter| workflow_parameter_entity_id(parameter) == parameter_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow parameter does not exist").at(["parameters", parameter_id.as_str()]));
                }
            }
            WorkflowDiff::PatchParameter { parameter_id, parameter } => {
                if !document.parameters.iter().any(|entry| workflow_parameter_entity_id(entry) == parameter_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow parameter does not exist").at(["parameters", parameter_id.as_str()]));
                }
                let new_id = workflow_parameter_entity_id(parameter);
                if new_id != parameter_id && document.parameters.iter().any(|entry| workflow_parameter_entity_id(entry) == new_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "patched workflow parameter identity already exists").at(["parameters", new_id]));
                }
            }
            WorkflowDiff::BindParameterField { binding } => {
                if !document.graph.nodes.iter().any(|node| node.id == binding.node_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "parameter binding node does not exist").at(["parameterBindings", binding.node_id.as_str()]));
                }
                if !document.parameters.iter().any(|parameter| workflow_parameter_entity_id(parameter) == binding.parameter_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "bound workflow parameter does not exist").at(["parameterBindings", binding.parameter_id.as_str()]));
                }
            }
            WorkflowDiff::UnbindParameterField { node_id, field_path } => {
                if !document.parameter_bindings.iter().any(|binding| binding.node_id == *node_id && binding.field_path == *field_path) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow parameter binding does not exist").at(["parameterBindings", node_id.as_str(), field_path.as_str()]));
                }
            }
            WorkflowDiff::DeclareInput { input } => {
                if document.inputs.iter().any(|entry| entry.id == input.id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "workflow input identity already exists").at(["inputs", input.id.as_str()]));
                }
            }
            WorkflowDiff::RemoveInput { input_id } => {
                if !document.inputs.iter().any(|input| input.id == *input_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow input does not exist").at(["inputs", input_id.as_str()]));
                }
            }
            WorkflowDiff::BindInput { binding } => {
                if !document.inputs.iter().any(|input| input.id == binding.input_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "bound workflow input does not exist").at(["inputBindings", binding.input_id.as_str()]));
                }
                let node = match document.graph.nodes.iter().find(|node| node.id == binding.node_id) {
                    Some(node) => node,
                    None => return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "input binding node does not exist").at(["inputBindings", binding.node_id.as_str()])),
                };
                if !node.inputs.iter().any(|port| port.id == binding.port_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "input binding port does not exist").at(["inputBindings", binding.node_id.as_str(), binding.port_id.as_str()]));
                }
            }
            WorkflowDiff::UnbindInput { input_id } => {
                if !document.input_bindings.iter().any(|binding| binding.input_id == *input_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow input binding does not exist").at(["inputBindings", input_id.as_str()]));
                }
            }
            WorkflowDiff::BindOutput { binding } => {
                let node = match document.graph.nodes.iter().find(|node| node.id == binding.node_id) {
                    Some(node) => node,
                    None => return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "output binding node does not exist").at(["outputBindings", binding.node_id.as_str()])),
                };
                if !node.outputs.iter().any(|port| port.id == binding.port_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "output binding port does not exist").at(["outputBindings", binding.node_id.as_str(), binding.port_id.as_str()]));
                }
            }
            WorkflowDiff::UnbindOutput { node_id, port_id } => {
                if !document.output_bindings.iter().any(|binding| binding.node_id == *node_id && binding.port_id == *port_id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "workflow output binding does not exist").at(["outputBindings", node_id.as_str(), port_id.as_str()]));
                }
            }
        }
        let operation = match self {
            WorkflowDiff::Empty => return Ok(document.clone()),
            WorkflowDiff::AddNode { node } => WorkflowMutation::AddNode(AddNode { node: node.clone() }),
            WorkflowDiff::RemoveNode { node_id } => WorkflowMutation::RemoveNode(RemoveNode { node_id: node_id.clone() }),
            WorkflowDiff::ConnectPorts { edge } => WorkflowMutation::ConnectPorts(ConnectPorts { edge: edge.clone() }),
            WorkflowDiff::DisconnectEdge { edge_id } => WorkflowMutation::DisconnectEdge(DisconnectEdge { edge_id: edge_id.clone() }),
            WorkflowDiff::MoveNode { node_id, x, y } => WorkflowMutation::MoveNode(MoveNode { node_id: node_id.clone(), x: *x, y: *y }),
            WorkflowDiff::PatchNode { node_id, label } => WorkflowMutation::RenameNode(RenameNode { node_id: node_id.clone(), label: label.clone() }),
            WorkflowDiff::AddParameter { parameter } => WorkflowMutation::AddParameter(AddParameter { parameter: Box::new(parameter.clone()) }),
            WorkflowDiff::RemoveParameter { parameter_id } => WorkflowMutation::RemoveParameter(RemoveParameter { parameter_id: parameter_id.clone() }),
            WorkflowDiff::PatchParameter { parameter_id, parameter } => WorkflowMutation::ChangeParameter(ChangeParameter { parameter_id: parameter_id.clone(), parameter: Box::new(parameter.clone()) }),
            WorkflowDiff::BindParameterField { binding } => WorkflowMutation::BindParameterField(BindParameterField { binding: binding.clone() }),
            WorkflowDiff::UnbindParameterField { node_id, field_path } => WorkflowMutation::UnbindParameterField(UnbindParameterField { node_id: node_id.clone(), field_path: field_path.clone() }),
            WorkflowDiff::SyncNodePorts => WorkflowMutation::UpdateNodePorts(UpdateNodePorts {}),
            WorkflowDiff::DeclareInput { input } => WorkflowMutation::AddInput(AddInput { input: input.clone() }),
            WorkflowDiff::RemoveInput { input_id } => WorkflowMutation::RemoveInput(RemoveInput { input_id: input_id.clone() }),
            WorkflowDiff::BindInput { binding } => WorkflowMutation::BindInput(BindInput { binding: binding.clone() }),
            WorkflowDiff::UnbindInput { input_id } => WorkflowMutation::UnbindInput(UnbindInput { input_id: input_id.clone() }),
            WorkflowDiff::BindOutput { binding } => WorkflowMutation::BindOutput(BindOutput { binding: binding.clone() }),
            WorkflowDiff::UnbindOutput { node_id, port_id } => WorkflowMutation::UnbindOutput(UnbindOutput { node_id: node_id.clone(), port_id: port_id.clone() }),
        };
        Ok(apply_workflow_operation(document, &operation))
    }

    fn absorb(&mut self, other: Self) {
        if !matches!(other, WorkflowDiff::Empty) {
            *self = other;
        }
    }
}

//#endregion 🔖️WorkflowMutation

//#region 🔖️WorkflowMutationOpText
impl protocol::OpText for WorkflowMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
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

    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(candidate, _)| candidate == &keyword).map(|(_, spec)| *spec).expect("variant spec");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for WorkflowMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
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

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
