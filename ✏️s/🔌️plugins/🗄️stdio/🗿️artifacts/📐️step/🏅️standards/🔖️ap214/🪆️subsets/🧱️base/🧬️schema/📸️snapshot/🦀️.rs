//! 🧬️ StepSnapshot schema — persistent fields + real Part-21 codecs. `StepSnapshot` owns its own
//! typed ISO 10303-21 exchange-structure model (`StepHeader`, `StepEntity`, `StepValue`) — the
//! shared `semio_s_artifact_stdio_contract::part21` tokenizer/writer stays the reused SYNTAX layer (spec-mandated reuse,
//! same rationale as gif 87a/89a sharing one root), but the PERSISTED type is step's own, never a
//! raw `Part21Document` (that was the copy-paste-type defect flagged against ifc in
//! `w0-recon-report.md` §7 — step does not repeat it for itself).

use semio_s_artifact_stdio_contract::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};
use crate::STDIO_STEP_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
//#region 🔖️BrepModelReexport
/// 🧱 The BrepMesh analyzer types live with the derived view in `engine::brep`, not here — the
/// snapshot only stores the generic graph. Re-exported for pre-existing call sites' convenience.
pub use crate::engine::brep::{BrepFace, BrepMesh, BrepVertex};
//#endregion 🔖️BrepModelReexport

//#region 🔖️Value
/// 🔤️ One typed Part-21 argument value, step's own vocabulary (never `Part21Value` directly —
/// that stays the shared tokenizer's working representation). `Unset` = `, `Derived` = `*`,
/// `Reference` = a `#456` instance pointer, `Enum` = `.T.`/`.F.`/`.UNKNOWN.`-shaped enumeration or
/// domain-select literal, `Aggregate` = a parenthesized list, `TypedValue` = a simple/complex
/// defined-type wrapper (`IFCLENGTHMEASURE(3000.)`-shaped).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase")]
#[derive(Default)]
pub enum StepValue {
    #[default]
    Unset,
    Derived,
    Integer(i64),
    Real(f64),
    String(String),
    Enum(String),
    Reference(u64),
    Aggregate(Vec<StepValue>),
    TypedValue(StepTypedValue),
}

/// 🏷️ The payload record of [`StepValue::TypedValue`]: the defined-type keyword plus its wrapped value.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct StepTypedValue {
    pub type_name: String,
    pub value: Box<StepValue>,
}

//#endregion 🔖️Value

//#region 🔖️Header
/// 📜️ The one empty-but-populated `LIST[1:?] OF STRING` ISO 10303-21 §8.2 leaves a producer with
/// nothing to say: exactly one empty string. The standard's lower bound of one is a population
/// constraint, so `()` is not a legal spelling of "no description" — `('')` is, and it is what
/// every conformant writer (including the ruststep reference this repository measures against)
/// emits.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn unpopulated_string_list() -> Vec<String> {
    vec![String::new()]
}

/// 📜️ `implementation_level` for a file this codebase authored from scratch: ISO 10303-21 §8.2.2's
/// `'2;1'` — version 2 of the exchange structure, conformance option 1 (internal mapping, no
/// external references), which is what a document built from a `StepSnapshot` alone genuinely is.
pub const ISO_10303_21_IMPLEMENTATION_LEVEL: &str = "2;1";

/// 📇️ `FILE_DESCRIPTION(description, implementation_level)` — ISO 10303-21 §8.2.2. `description`
/// is `LIST[1:?] OF STRING`, hence the hand-written [`Default`]: a derived one would give the
/// empty list the standard forbids.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct StepFileDescription {
    #[value(default = "unpopulated_string_list")]
    pub description: Vec<String>,
    #[value(default)]
    pub implementation_level: String,
}

impl Default for StepFileDescription {
    fn default() -> Self {
        Self { description: unpopulated_string_list(), implementation_level: ISO_10303_21_IMPLEMENTATION_LEVEL.into() }
    }
}

/// 📇️ `FILE_NAME(name, timestamp, author, organization, preprocessor_version,
/// originating_system, authorization)` — ISO 10303-21 §8.2.3. `author` and `organization` are
/// `LIST[1:?] OF STRING`, so this carries the same hand-written [`Default`] as
/// [`StepFileDescription`] for the same reason.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct StepFileName {
    #[value(default)]
    pub name: String,
    #[value(default)]
    pub timestamp: String,
    #[value(default = "unpopulated_string_list")]
    pub author: Vec<String>,
    #[value(default = "unpopulated_string_list")]
    pub organization: Vec<String>,
    #[value(default)]
    pub preprocessor_version: String,
    #[value(default)]
    pub originating_system: String,
    #[value(default)]
    pub authorization: String,
}

impl Default for StepFileName {
    fn default() -> Self {
        Self { name: String::new(), timestamp: String::new(), author: unpopulated_string_list(), organization: unpopulated_string_list(), preprocessor_version: String::new(), originating_system: String::new(), authorization: String::new() }
    }
}

/// 📇️ `FILE_SCHEMA(schemas)` — ISO 10303-21 §8.2.4, `LIST[1:?] OF schema_name`. An unnamed schema
/// is a real (and diagnosable) state — `check_ccN_conformance` reports it as a hard `CODE_FILE_
/// SCHEMA` violation — but an empty LIST is not a state the exchange structure can even carry, so
/// the default is the one unpopulated entry rather than none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct StepFileSchema {
    #[value(default = "unpopulated_string_list")]
    pub schemas: Vec<String>,
}

impl Default for StepFileSchema {
    fn default() -> Self {
        Self { schemas: unpopulated_string_list() }
    }
}

/// 📇️ The full typed `HEADER;` section — all three standard records. Its `Default` is ISO
/// 10303-21 §8.2's conformant minimum, so `StepSnapshot::default()` writes an exchange structure a
/// conformant reader accepts instead of one it refuses.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct StepHeader {
    #[value(default)]
    pub file_description: StepFileDescription,
    #[value(default)]
    pub file_name: StepFileName,
    #[value(default)]
    pub file_schema: StepFileSchema,
}
//#endregion 🔖️Header

//#region 🔖️Entity
/// 🧩️ An additional type record on a genuinely complex Part-21 instance
/// (`#N=(TYPE1(...)TYPE2(...))`) — spec-legal (ISO 10303-21 §4.2), rare in real AP214 exports
/// (far more common in IFC's select-type disambiguation), never silently dropped: a plain
/// single-typed instance leaves this empty; a complex one keeps every extra type here.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct StepComplexType {
    pub name: String,
    #[value(default)]
    pub args: Vec<StepValue>,
}

/// 🧩️ One `#N = TYPE(args...)` instance — id-keyed identity, positional argument list.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct StepEntity {
    pub id: u64,
    pub name: String,
    #[value(default)]
    pub args: Vec<StepValue>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub complex: Vec<StepComplexType>,
}
//#endregion 🔖️Entity

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.step` snapshot — typed HEADER triple + id-keyed entity graph. Complete per
/// FORMAT SPEC: nothing about a real AP214 exchange file is silently dropped (undecoded header
/// positions default gracefully; complex instances retain every constituent type via
/// `StepEntity::complex`). BrepMesh is a derived analyzer view
/// (`crate::engine::brep::analyze_brep_mesh`), not stored here.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.step")]
pub struct StepSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub header: StepHeader,
    #[state(artifact)]
    #[value(default)]
    pub entities: Vec<StepEntity>,
}

impl Default for StepSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_STEP_DOCUMENT_SCHEMA.into(), header: StepHeader::default(), entities: Vec::new() }
    }
}

/// 🏭️ Authors an AP214 product identity with complete context and reference relationships.
pub fn initial_ap214_snapshot()->StepSnapshot{
 use StepValue::{Aggregate,Integer,Reference,String as Text};
 let entity=|id,name:&str,args|StepEntity{id,name:name.into(),args,complex:Vec::new()};
 StepSnapshot{
  schema:STDIO_STEP_DOCUMENT_SCHEMA.into(),
  header:StepHeader{file_schema:StepFileSchema{schemas:vec!["AUTOMOTIVE_DESIGN".into()]},..StepHeader::default()},
  entities:vec![
   entity(1,"APPLICATION_CONTEXT",vec![Text("automotive_design".into())]),
   entity(2,"APPLICATION_PROTOCOL_DEFINITION",vec![Text("international standard".into()),Text("automotive_design".into()),Integer(1994),Reference(1)]),
   entity(3,"PRODUCT_CONTEXT",vec![Text(String::new()),Reference(1),Text("mechanical".into())]),
   entity(4,"PRODUCT",vec![Text(String::new()),Text(String::new()),Text(String::new()),Aggregate(vec![Reference(3)])]),
   entity(5,"PRODUCT_DEFINITION_FORMATION",vec![Text(String::new()),Text(String::new()),Reference(4)]),
   entity(6,"PRODUCT_DEFINITION_CONTEXT",vec![Text("part definition".into()),Reference(1),Text("design".into())]),
   entity(7,"PRODUCT_DEFINITION",vec![Text(String::new()),Text(String::new()),Reference(5),Reference(6)]),
  ],
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
        Part21Value::Typed { name, items } => {
            let value = if items.len() == 1 { value_from_part21(&items[0]) } else { StepValue::Aggregate(items.iter().map(value_from_part21).collect()) };
            StepValue::TypedValue(StepTypedValue { type_name: name.clone(), value: Box::new(value) })
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
        StepValue::TypedValue(StepTypedValue { type_name, value }) => Part21Value::Typed { name: type_name.clone(), items: vec![value_to_part21(value)] },
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
pub fn step_snapshot_from_part21(doc: &Part21Document) -> (StepHeader, Vec<StepEntity>) {
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
    pub fn from_part21_document(doc: &Part21Document) -> Self {
        let (header, entities) = step_snapshot_from_part21(doc);
        Self { schema: STDIO_STEP_DOCUMENT_SCHEMA.into(), header, entities }
    }
}
//#endregion 🔖️Part21Conversion

//#region 🔖️NativeCodec



//#endregion 🔖️NativeCodec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "🔗️references/🦀️.rs"]
pub mod references;
