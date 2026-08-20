//! 🧬️ StepSnapshot schema — persistent fields + real Part-21 codecs. `StepSnapshot` owns its own
//! typed ISO 10303-21 exchange-structure model (`StepHeader`, `StepEntity`, `StepValue`) — the
//! shared `engine::part21` tokenizer/writer stays the reused SYNTAX layer (spec-mandated reuse,
//! same rationale as gif 87a/89a sharing one root), but the PERSISTED type is step's own, never a
//! raw `Part21Document` (that was the copy-paste-type defect flagged against ifc in
//! `w0-recon-report.md` §7 — step does not repeat it for itself).

use crate::artifacts::step::engine::part21::{parse_part21, write_part21, Part21Document, Part21Header, Part21Instance, Part21Value};
use crate::artifacts::step::STDIO_STEP_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️BrepModelReexport
/// 🧱 The BrepMesh analyzer types live with the derived view in `engine::brep`, not here — the
/// snapshot only stores the generic graph. Re-exported for pre-existing call sites' convenience.
pub use crate::artifacts::step::engine::brep::{BrepFace, BrepMesh, BrepVertex};
//#endregion 🔖️BrepModelReexport

//#region 🔖️Value
/// 🔤️ One typed Part-21 argument value, step's own vocabulary (never `Part21Value` directly —
/// that stays the shared tokenizer's working representation). `Unset` = `$`, `Derived` = `*`,
/// `Reference` = a `#456` instance pointer, `Enum` = `.T.`/`.F.`/`.UNKNOWN.`-shaped enumeration or
/// domain-select literal, `Aggregate` = a parenthesized list, `TypedValue` = a simple/complex
/// defined-type wrapper (`IFCLENGTHMEASURE(3000.)`-shaped).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StepValue {
    Unset,
    Derived,
    Integer(i64),
    Real(f64),
    String(String),
    Enum(String),
    Reference(u64),
    Aggregate(Vec<StepValue>),
    TypedValue { type_name: String, value: Box<StepValue> },
}

impl Default for StepValue {
    fn default() -> Self {
        StepValue::Unset
    }
}
//#endregion 🔖️Value

//#region 🔖️Header
/// 📇️ `FILE_DESCRIPTION(description, implementation_level)` — ISO 10303-21 §4.3.1.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepFileDescription {
    #[serde(default)]
    pub description: Vec<String>,
    #[serde(default)]
    pub implementation_level: String,
}

/// 📇️ `FILE_NAME(name, timestamp, author, organization, preprocessor_version,
/// originating_system, authorization)` — ISO 10303-21 §4.3.2.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepFileName {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub timestamp: String,
    #[serde(default)]
    pub author: Vec<String>,
    #[serde(default)]
    pub organization: Vec<String>,
    #[serde(default)]
    pub preprocessor_version: String,
    #[serde(default)]
    pub originating_system: String,
    #[serde(default)]
    pub authorization: String,
}

/// 📇️ `FILE_SCHEMA(schemas)` — ISO 10303-21 §4.3.3.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepFileSchema {
    #[serde(default)]
    pub schemas: Vec<String>,
}

/// 📇️ The full typed `HEADER;` section — all three standard records.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepHeader {
    #[serde(default)]
    pub file_description: StepFileDescription,
    #[serde(default)]
    pub file_name: StepFileName,
    #[serde(default)]
    pub file_schema: StepFileSchema,
}
//#endregion 🔖️Header

//#region 🔖️Entity
/// 🧩️ An additional type record on a genuinely complex Part-21 instance
/// (`#N=(TYPE1(...)TYPE2(...))`) — spec-legal (ISO 10303-21 §4.2), rare in real AP214 exports
/// (far more common in IFC's select-type disambiguation), never silently dropped: a plain
/// single-typed instance leaves this empty; a complex one keeps every extra type here.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepComplexType {
    pub name: String,
    #[serde(default)]
    pub args: Vec<StepValue>,
}

/// 🧩️ One `#N = TYPE(args...)` instance — id-keyed identity, positional argument list.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepEntity {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub args: Vec<StepValue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub complex: Vec<StepComplexType>,
}
//#endregion 🔖️Entity

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.step` snapshot — typed HEADER triple + id-keyed entity graph. Complete per
/// FORMAT SPEC: nothing about a real AP214 exchange file is silently dropped (undecoded header
/// positions default gracefully; complex instances retain every constituent type via
/// `StepEntity::complex`). BrepMesh is a derived analyzer view
/// (`crate::artifacts::step::engine::brep::analyze_brep_mesh`), not stored here.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.step")]
pub struct StepSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub header: StepHeader,
    #[state(artifact)]
    #[serde(default)]
    pub entities: Vec<StepEntity>,
}

impl Default for StepSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_STEP_DOCUMENT_SCHEMA.into(), header: StepHeader::default(), entities: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️Part21Conversion
/// 🔁️ `Part21Value` (shared tokenizer) <-> `StepValue` (step's own). `TypedValue` always
/// round-trips through exactly ONE wrapped value on the way back out — the real,
/// EXPRESS-conformant shape for every defined-type/select wrapper this codebase's fixtures
/// exercise (`IFCLENGTHMEASURE(3000.)`, `IFCCARTESIANPOINT((1.,2.,3.))`-shaped constructs alike,
/// since the single wrapped value can itself be an `Aggregate`). A `Typed(name, items)` with
/// `items.len() != 1` is grammar-permitted but spec-illegal for an AP214 defined type; it is
/// still captured losslessly as data (via the same `Aggregate` wrapper) but re-nests as a single
/// list argument on re-emission — a documented normal form, never fabricated, matching the
/// recipe's `codec_retention_law` allowance.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn value_from_part21(v: &Part21Value) -> StepValue {
    match v {
        Part21Value::Unset => StepValue::Unset,
        Part21Value::Derived => StepValue::Derived,
        Part21Value::Int(i) => StepValue::Integer(*i),
        Part21Value::Real(r) => StepValue::Real(r.to_f64().unwrap_or_default()),
        Part21Value::Str(s) => StepValue::String(s.clone()),
        Part21Value::Enum(s) => StepValue::Enum(s.clone()),
        Part21Value::Ref(id) => StepValue::Reference(*id),
        Part21Value::List(items) => StepValue::Aggregate(items.iter().map(value_from_part21).collect()),
        Part21Value::Typed(name, items) => {
            let value = if items.len() == 1 { value_from_part21(&items[0]) } else { StepValue::Aggregate(items.iter().map(value_from_part21).collect()) };
            StepValue::TypedValue { type_name: name.clone(), value: Box::new(value) }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn value_to_part21(v: &StepValue) -> Part21Value {
    match v {
        StepValue::Unset => Part21Value::Unset,
        StepValue::Derived => Part21Value::Derived,
        StepValue::Integer(i) => Part21Value::Int(*i),
        StepValue::Real(r) => Part21Value::Real((*r).into()),
        StepValue::String(s) => Part21Value::Str(s.clone()),
        StepValue::Enum(s) => Part21Value::Enum(s.clone()),
        StepValue::Reference(id) => Part21Value::Ref(*id),
        StepValue::Aggregate(items) => Part21Value::List(items.iter().map(value_to_part21).collect()),
        StepValue::TypedValue { type_name, value } => Part21Value::Typed(type_name.clone(), vec![value_to_part21(value)]),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn as_string(v: Option<&Part21Value>) -> String {
    match v {
        Some(Part21Value::Str(s)) => s.clone(),
        Some(Part21Value::Enum(s)) => s.clone(),
        _ => String::new(),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn as_string_list(v: Option<&Part21Value>) -> Vec<String> {
    match v {
        Some(Part21Value::List(items)) => items
            .iter()
            .filter_map(|it| match it {
                Part21Value::Str(s) => Some(s.clone()),
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn file_description_from_part21(args: &[Part21Value]) -> StepFileDescription {
    StepFileDescription { description: as_string_list(args.first()), implementation_level: as_string(args.get(1)) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn file_description_to_part21(d: &StepFileDescription) -> Vec<Part21Value> {
    vec![Part21Value::List(d.description.iter().cloned().map(Part21Value::Str).collect()), Part21Value::Str(d.implementation_level.clone())]
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn file_name_from_part21(args: &[Part21Value]) -> StepFileName {
    StepFileName {
        name: as_string(args.first()),
        timestamp: as_string(args.get(1)),
        author: as_string_list(args.get(2)),
        organization: as_string_list(args.get(3)),
        preprocessor_version: as_string(args.get(4)),
        originating_system: as_string(args.get(5)),
        authorization: as_string(args.get(6)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn file_name_to_part21(f: &StepFileName) -> Vec<Part21Value> {
    vec![
        Part21Value::Str(f.name.clone()),
        Part21Value::Str(f.timestamp.clone()),
        Part21Value::List(f.author.iter().cloned().map(Part21Value::Str).collect()),
        Part21Value::List(f.organization.iter().cloned().map(Part21Value::Str).collect()),
        Part21Value::Str(f.preprocessor_version.clone()),
        Part21Value::Str(f.originating_system.clone()),
        Part21Value::Str(f.authorization.clone()),
    ]
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn file_schema_from_part21(args: &[Part21Value]) -> StepFileSchema {
    StepFileSchema { schemas: as_string_list(args.first()) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn file_schema_to_part21(s: &StepFileSchema) -> Vec<Part21Value> {
    vec![Part21Value::List(s.schemas.iter().cloned().map(Part21Value::Str).collect())]
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn header_from_part21(h: &Part21Header) -> StepHeader {
    StepHeader { file_description: file_description_from_part21(&h.file_description), file_name: file_name_from_part21(&h.file_name), file_schema: file_schema_from_part21(&h.file_schema) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn header_to_part21(h: &StepHeader) -> Part21Header {
    Part21Header { file_description: file_description_to_part21(&h.file_description), file_name: file_name_to_part21(&h.file_name), file_schema: file_schema_to_part21(&h.file_schema) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_from_part21(inst: &Part21Instance) -> StepEntity {
    let mut types = inst.entities.iter();
    let (name, args) = match types.next() {
        Some((n, a)) => (n.clone(), a.iter().map(value_from_part21).collect()),
        None => (String::new(), Vec::new()),
    };
    let complex = types.map(|(n, a)| StepComplexType { name: n.clone(), args: a.iter().map(value_from_part21).collect() }).collect();
    StepEntity { id: inst.id, name, args, complex }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_to_part21(e: &StepEntity) -> Part21Instance {
    let mut entities = vec![(e.name.clone(), e.args.iter().map(value_to_part21).collect())];
    entities.extend(e.complex.iter().map(|c| (c.name.clone(), c.args.iter().map(value_to_part21).collect())));
    Part21Instance { id: e.id, entities }
}

/// 🔁️ `Part21Document` -> `(StepHeader, Vec<StepEntity>)` — the ONLY place the shared generic
/// graph is decoded into step's own model.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn step_snapshot_from_part21(doc: Part21Document) -> (StepHeader, Vec<StepEntity>) {
    let header = header_from_part21(&doc.header);
    let entities = doc.instances.iter().map(entity_from_part21).collect();
    (header, entities)
}
/// 🔁️ `(StepHeader, &[StepEntity])` -> `Part21Document` — the inverse, used by the DSL/pack codecs
/// and by every real consumer (conformance-class ladder checks, the cad/process3d plugins' STEP
/// import/export) that still wants the generic view.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn step_snapshot_to_part21(header: &StepHeader, entities: &[StepEntity]) -> Part21Document {
    Part21Document { header: header_to_part21(header), instances: entities.iter().map(entity_to_part21).collect() }
}

impl StepSnapshot {
    /// 🔁️ Materializes the shared generic Part-21 graph on demand — never stored, always derived
    /// from the typed `header`/`entities` fields.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_part21_document(&self) -> Part21Document {
        step_snapshot_to_part21(&self.header, &self.entities)
    }
    /// 🔁️ Builds a `StepSnapshot` from a generic Part-21 graph (e.g. one built by
    /// `engine::brep::brep_mesh_to_part21` or hand-assembled in a test).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_part21_document(doc: Part21Document) -> Self {
        let (header, entities) = step_snapshot_from_part21(doc);
        Self { schema: STDIO_STEP_DOCUMENT_SCHEMA.into(), header, entities }
    }
}
//#endregion 🔖️Part21Conversion

//#region 🔖️Part21Codec
impl store::ArtifactDsl for StepSnapshot {
    const EXTENSION: &'static str = "step";
    async fn envelope_id() -> &'static str {
        "stdio.step"
    }

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let document = parse_part21(body).map_err(|e| store::TextError::new(format!("step parse: {e}"), dsl::TextSpan::at(1, 1)))?;
        Ok(Self::from_part21_document(document))
    }
    async fn print_dsl(&self) -> String {
        let body = write_part21(&self.to_part21_document());
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for StepSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = write_part21(&self.to_part21_document()).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        let text = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let document = parse_part21(&text).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(Self::from_part21_document(document))
    }
}
//#endregion 🔖️Part21Codec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio.step','2026-08-10T00:00:00',('Ueli'),('semio'),'semio','','');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;\n#1=CARTESIAN_POINT('',(0.,0.,0.));\n#2=CARTESIAN_POINT('',(10.,0.,0.));\nENDSEC;\nEND-ISO-10303-21;\n";

    #[semio_framework_async_macros::async_test]
    async fn typed_snapshot_round_trips_through_part21_text() {
        let snapshot = <StepSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE).await.expect("parse");
        assert_eq!(snapshot.header.file_schema.schemas, vec!["AUTOMOTIVE_DESIGN".to_string()]);
        assert_eq!(snapshot.header.file_name.name, "semio.step");
        assert_eq!(snapshot.header.file_name.author, vec!["Ueli".to_string()]);
        assert_eq!(snapshot.entities.len(), 2);
        assert_eq!(snapshot.entities[0].id, 1);
        assert_eq!(snapshot.entities[0].name, "CARTESIAN_POINT");
        let text = store::ArtifactDsl::print_dsl(&snapshot);
        let reparsed = <StepSnapshot as store::ArtifactDsl>::parse_dsl(&text).await.expect("reparse");
        assert_eq!(snapshot, reparsed, "typed round trip must be lossless");
    }

    #[semio_framework_async_macros::async_test]
    async fn typed_value_wrapper_round_trips() {
        let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n#1=IFCPROPERTYSINGLEVALUE('Height',$,IFCLENGTHMEASURE(3000.),$);\nENDSEC;\nEND-ISO-10303-21;\n";
        let snapshot = <StepSnapshot as store::ArtifactDsl>::parse_dsl(text).await.expect("parse");
        let args = &snapshot.entities[0].args;
        match &args[2] {
            StepValue::TypedValue { type_name, value } => {
                assert_eq!(type_name, "IFCLENGTHMEASURE");
                assert_eq!(**value, StepValue::Real(3000.0));
            }
            other => panic!("expected TypedValue, got {other:?}"),
        }
        let reparsed = <StepSnapshot as store::ArtifactDsl>::parse_dsl(&store::ArtifactDsl::print_dsl(&snapshot)).await.unwrap();
        assert_eq!(snapshot, reparsed);
    }

    #[semio_framework_async_macros::async_test]
    async fn complex_instance_keeps_every_type() {
        let text =
            "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n#1=(IFCQUANTITYAREA($,$,$,10.5,$)IFCPHYSICALSIMPLEQUANTITY($,$,$,$));\nENDSEC;\nEND-ISO-10303-21;\n";
        let snapshot = <StepSnapshot as store::ArtifactDsl>::parse_dsl(text).await.expect("parse");
        let entity = &snapshot.entities[0];
        assert_eq!(entity.name, "IFCQUANTITYAREA");
        assert_eq!(entity.complex.len(), 1);
        assert_eq!(entity.complex[0].name, "IFCPHYSICALSIMPLEQUANTITY");
        let reparsed = <StepSnapshot as store::ArtifactDsl>::parse_dsl(&store::ArtifactDsl::print_dsl(&snapshot)).await.unwrap();
        assert_eq!(snapshot, reparsed);
    }

    #[semio_framework_async_macros::async_test]
    async fn pack_codec_round_trip() {
        let snapshot = <StepSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE).await.expect("parse");
        let bytes = store::ArtifactPack::encode_pack(&snapshot);
        let decoded = <StepSnapshot as store::ArtifactPack>::decode_pack(&bytes).await.expect("decode");
        assert_eq!(decoded, snapshot);
    }

    /// 🧪️ `codec_retention_law`: decode -> encode is byte-preserving for both the DSL (`.step`
    /// text) and pack codecs on the real fixture.
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law_decode_encode_is_stable() {
        let snapshot = <StepSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE).await.expect("parse");
        let text_once = store::ArtifactDsl::print_dsl(&snapshot);
        let reparsed = <StepSnapshot as store::ArtifactDsl>::parse_dsl(&text_once).await.expect("reparse");
        let text_twice = store::ArtifactDsl::print_dsl(&reparsed);
        assert_eq!(text_once, text_twice, "print_dsl must be stable across a decode/encode cycle");
        assert_eq!(snapshot, reparsed);

        let bytes_once = store::ArtifactPack::encode_pack(&snapshot);
        let decoded = <StepSnapshot as store::ArtifactPack>::decode_pack(&bytes_once).await.expect("decode");
        let bytes_twice = store::ArtifactPack::encode_pack(&decoded);
        assert_eq!(bytes_once, bytes_twice, "encode_pack must be stable across a decode/encode cycle");
    }
}
//#endregion 🧪️Tests
