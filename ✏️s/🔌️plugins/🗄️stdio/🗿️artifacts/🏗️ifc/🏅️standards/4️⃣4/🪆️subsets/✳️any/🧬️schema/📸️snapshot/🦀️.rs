//! 🧬️ IfcSnapshot schema — OWN typed model of the IFC4 EXPRESS-schema data (ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, W0 §7's most severe
//! finding: the prior `IfcSnapshot.document: step::engine::part21::Part21Document` used STEP's
//! own persisted type verbatim as this artifact's snapshot). IFC4 rides the same ISO 10303-21
//! Part-21 EXCHANGE-STRUCTURE grammar as STEP (both real, both documented on
//! `step::engine::part21`'s own module doc as a legitimate shared low-level tokenizer — same
//! spirit as OPC being shared by the OOXML trio), but the DATA MODEL is IFC4's own EXPRESS
//! schema, semantically unrelated to AP214 — so this snapshot declares its OWN
//! `IfcEntity`/`IfcValue`/`IfcHeader` types (a near-duplicate of STEP's value grammar shape,
//! which is CORRECT per the plan's specific-over-generic mandate) and converts to/from the
//! shared `Part21Document` only at the parse/write boundary, never storing it.
//! https://www.iso.org/standard/70303.html (IFC4) / https://www.iso.org/standard/63141.html (Part 21)

use crate::artifacts::ifc::STDIO_IFC_DOCUMENT_SCHEMA;
use crate::artifacts::step::engine::part21::{parse_part21, write_part21, Part21Document, Part21Header, Part21Instance, Part21Value};
use schema::ArtifactSchema;

//#region 🔖️Value
/// 🔤️ One typed value in IFC4's Part-21 argument-list syntax — own enum, mirrors
/// `step::engine::part21::Part21Value`'s shape but is IFC's own type (never shared cross-artifact).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum IfcValue {
    /// `$` — attribute explicitly unset.
    Unset,
    /// `*` — attribute derived from a supertype, not stored here.
    Derived,
    Integer(i64),
    Real(f64),
    String(String),
    /// `.EDGE.` style enumeration literal (name kept without the surrounding dots).
    Enum(String),
    /// `#N` — a reference to another entity's id.
    Reference(u64),
    /// `(a, b, c)` — a parenthesized list of values (SET/LIST/ARRAY, all indistinguishable at the
    /// Part-21 syntax level).
    Aggregate(Vec<IfcValue>),
    /// `IFCLENGTHMEASURE(3000.)` — a "defined type" wrapper: EXPRESS keyword + its own arg list.
    TypedValue { name: String, items: Vec<IfcValue> },
}

impl Default for IfcValue {
    fn default() -> Self {
        IfcValue::Unset
    }
}

impl IfcValue {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_reference(&self) -> Option<u64> {
        if let IfcValue::Reference(id) = self {
            Some(*id)
        } else {
            None
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_str(&self) -> Option<&str> {
        if let IfcValue::String(s) = self {
            Some(s.as_str())
        } else {
            None
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_enum(&self) -> Option<&str> {
        if let IfcValue::Enum(s) = self {
            Some(s.as_str())
        } else {
            None
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_real(&self) -> Option<f64> {
        match self {
            IfcValue::Real(r) => Some(*r),
            IfcValue::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_aggregate(&self) -> Option<&[IfcValue]> {
        if let IfcValue::Aggregate(items) = self {
            Some(items.as_slice())
        } else {
            None
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_typed(&self) -> Option<(&str, &[IfcValue])> {
        if let IfcValue::TypedValue { name, items } = self {
            Some((name.as_str(), items.as_slice()))
        } else {
            None
        }
    }
}
//#endregion 🔖️Value

//#region 🔖️Entity
/// 🧩️ One additional `(TYPE(args...) ...)` member of an IFC4 Part-21 COMPLEX instance — beyond
/// the primary `name`/`args` carried on [`IfcEntity`] itself. Ordinary (non-complex) instances
/// carry an empty `complex` vec; nothing about a real complex instance's extra type members is
/// ever silently dropped (typed raw-retention, per the recipe).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IfcComplexType {
    pub name: String,
    pub args: Vec<IfcValue>,
}

/// 📦️ One `#N = TYPE(args...);` IFC4 instance — id-keyed strong entity (per the recipe: numeric
/// id key, like STEP's own `#id` and PDF's `(id,gen)`). `name` is the EXPRESS entity type keyword
/// (e.g. `"IFCWALL"`, `"IFCPROJECT"`) — the generic `{id, name, args}` shape uniformly covers every
/// IFC4 entity type, matching how the format itself is structured; a derived analyzer view can
/// filter by `name` for domain-specific queries (see `engine::spatial`) without this snapshot
/// needing a hand-modeled Rust type per IFC entity kind.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IfcEntity {
    pub id: u64,
    pub name: String,
    pub args: Vec<IfcValue>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub complex: Vec<IfcComplexType>,
}
//#endregion 🔖️Entity

//#region 🔖️Header
/// 📇️ The three standard `HEADER;` records (`FILE_DESCRIPTION`/`FILE_NAME`/`FILE_SCHEMA`), typed
/// via IFC's own [`IfcValue`] — kept as their raw tuple-of-values shape (not schema-interpreted
/// into named sub-fields), matching the recipe's "typed HEADER section" completeness target.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IfcHeader {
    pub file_description: Vec<IfcValue>,
    pub file_name: Vec<IfcValue>,
    pub file_schema: Vec<IfcValue>,
}
//#endregion 🔖️Header

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.ifc` snapshot — the full, lossless IFC4 Part-21 graph in IFC's OWN typed
/// model (never `step::engine::part21::Part21Document`). Spatial structure/placement
/// matrices/property sets stay a derived analyzer view (`engine::spatial::analyze_spatial`,
/// which is handed a `Part21Document` built on demand via [`to_part21_document`]), not stored here.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc")]
pub struct IfcSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub header: IfcHeader,
    /// 🆔️ Id-keyed, order-preserving — the strong collection this artifact's diff/mutations work
    /// against (see `schema::diff::IfcEntitiesDiff`).
    #[state(artifact)]
    #[value(default)]
    pub entities: Vec<IfcEntity>,
}

impl Default for IfcSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_IFC_DOCUMENT_SCHEMA.into(), header: IfcHeader::default(), entities: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️Part21Conversion
/// 🔁️ `Part21Value` -> `IfcValue`, structurally 1:1 (own enum, recursive).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ifc_value_from_part21(v: &Part21Value) -> IfcValue {
    match v {
        Part21Value::Ref(id) => IfcValue::Reference(*id),
        Part21Value::Str(s) => IfcValue::String(s.clone()),
        Part21Value::Enum(s) => IfcValue::Enum(s.clone()),
        Part21Value::Int(i) => IfcValue::Integer(*i),
        Part21Value::Real(r) => IfcValue::Real(r.to_f64().unwrap_or_default()),
        Part21Value::List(items) => IfcValue::Aggregate(items.iter().map(ifc_value_from_part21).collect()),
        Part21Value::Typed { name, items } => IfcValue::TypedValue { name: name.clone(), items: items.iter().map(ifc_value_from_part21).collect() },
        Part21Value::Unset => IfcValue::Unset,
        Part21Value::Derived => IfcValue::Derived,
    }
}

/// 🔁️ `IfcValue` -> `Part21Value`, the exact inverse of [`ifc_value_from_part21`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn part21_value_from_ifc(v: &IfcValue) -> Part21Value {
    match v {
        IfcValue::Reference(id) => Part21Value::Ref(*id),
        IfcValue::String(s) => Part21Value::Str(s.clone()),
        IfcValue::Enum(s) => Part21Value::Enum(s.clone()),
        IfcValue::Integer(i) => Part21Value::Int(*i),
        IfcValue::Real(r) => Part21Value::Real((*r).into()),
        IfcValue::Aggregate(items) => Part21Value::List(items.iter().map(part21_value_from_ifc).collect()),
        IfcValue::TypedValue { name, items } => Part21Value::Typed { name: name.clone(), items: items.iter().map(part21_value_from_ifc).collect() },
        IfcValue::Unset => Part21Value::Unset,
        IfcValue::Derived => Part21Value::Derived,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ifc_header_from_part21(h: &Part21Header) -> IfcHeader {
    IfcHeader { file_description: h.file_description.iter().map(ifc_value_from_part21).collect(), file_name: h.file_name.iter().map(ifc_value_from_part21).collect(), file_schema: h.file_schema.iter().map(ifc_value_from_part21).collect() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn part21_header_from_ifc(h: &IfcHeader) -> Part21Header {
    Part21Header { file_description: h.file_description.iter().map(part21_value_from_ifc).collect(), file_name: h.file_name.iter().map(part21_value_from_ifc).collect(), file_schema: h.file_schema.iter().map(part21_value_from_ifc).collect() }
}

/// 🔁️ One `Part21Instance` -> `IfcEntity`: the first `(name, args)` pair becomes the entity's
/// primary `name`/`args`, any further pairs (real IFC4 COMPLEX instances, e.g.
/// `IfcQuantityArea`+`IfcPhysicalSimpleQuantity`) are retained verbatim in `complex`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ifc_entity_from_instance(inst: &Part21Instance) -> IfcEntity {
    let mut pairs = inst.entities.iter();
    let (name, args) = match pairs.next() {
        Some((name, args)) => (name.clone(), args.iter().map(ifc_value_from_part21).collect()),
        None => (String::new(), Vec::new()),
    };
    let complex = pairs.map(|(name, args)| IfcComplexType { name: name.clone(), args: args.iter().map(ifc_value_from_part21).collect() }).collect();
    IfcEntity { id: inst.id, name, args, complex }
}

/// 🔁️ Exact inverse of [`ifc_entity_from_instance`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn instance_from_ifc_entity(e: &IfcEntity) -> Part21Instance {
    let mut entities = vec![(e.name.clone(), e.args.iter().map(part21_value_from_ifc).collect())];
    for c in &e.complex {
        entities.push((c.name.clone(), c.args.iter().map(part21_value_from_ifc).collect()));
    }
    Part21Instance { id: e.id, entities }
}

/// 📤️ Builds the shared generic Part-21 graph from `snapshot` — used at the parse/write boundary
/// (codecs below) and by the derived spatial analyzer (`engine::spatial::analyze_spatial`), which
/// still walks the generic graph shape for its relationship-graph traversal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn to_part21_document(snapshot: &IfcSnapshot) -> Part21Document {
    Part21Document { header: part21_header_from_ifc(&snapshot.header), instances: snapshot.entities.iter().map(instance_from_ifc_entity).collect() }
}

/// 📥️ Builds an `IfcSnapshot` from a parsed generic Part-21 graph.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn from_part21_document(schema: impl Into<String>, doc: &Part21Document) -> IfcSnapshot {
    IfcSnapshot { schema: schema.into(), header: ifc_header_from_part21(&doc.header), entities: doc.instances.iter().map(ifc_entity_from_instance).collect() }
}
//#endregion 🔖️Part21Conversion

//#region 🔖️Part21Codec
impl store::ArtifactDsl for IfcSnapshot {
    const EXTENSION: &'static str = "ifc";
    fn envelope_id() -> &'static str {
        "stdio.ifc"
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let document = parse_part21(body).map_err(|e| store::TextError::new(format!("ifc parse: {e}"), dsl::TextSpan::at(1, 1)))?;
        Ok(from_part21_document(STDIO_IFC_DOCUMENT_SCHEMA, &document))
    }
    fn print_dsl(&self) -> String {
        let body = write_part21(&to_part21_document(self));
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for IfcSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = write_part21(&to_part21_document(self)).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        let text = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let document = parse_part21(&text).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(from_part21_document(STDIO_IFC_DOCUMENT_SCHEMA, &document))
    }
}
//#endregion 🔖️Part21Codec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio.ifc','2026-08-10T00:00:00',('Ueli'),('semio'),'semio','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n#1=IFCPROJECT('gid-project',#2,'Demo Project',$,$,$,$,(#10),#11);\n#2=IFCOWNERHISTORY($,$,$,$,$,$,$,0);\n#6=IFCWALL('gid-wall',#2,'Wall-01',$,$,#80,$,$,$);\nENDSEC;\nEND-ISO-10303-21;\n";

    #[semio_framework_async_macros::async_test]
    async fn part21_round_trip_is_lossless() {
        let doc = parse_part21(FIXTURE).expect("parse");
        let snapshot = from_part21_document("stdio.ifc", &doc);
        assert_eq!(snapshot.entities.len(), 3);
        let wall = snapshot.entities.iter().find(|e| e.id == 6).expect("wall entity");
        assert_eq!(wall.name, "IFCWALL");
        assert_eq!(wall.args[2], IfcValue::String("Wall-01".into()));
        let back = to_part21_document(&snapshot);
        assert_eq!(back, doc, "snapshot <-> Part21Document must be lossless");
    }

    #[semio_framework_async_macros::async_test]
    async fn complex_instance_retains_every_type() {
        let text =
            "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n#1=(IFCQUANTITYAREA($,$,$,10.5,$)IFCPHYSICALSIMPLEQUANTITY($,$,$,$));\nENDSEC;\nEND-ISO-10303-21;\n";
        let doc = parse_part21(text).expect("parse complex instance");
        let snapshot = from_part21_document("stdio.ifc", &doc);
        let e = &snapshot.entities[0];
        assert_eq!(e.name, "IFCQUANTITYAREA");
        assert_eq!(e.complex.len(), 1);
        assert_eq!(e.complex[0].name, "IFCPHYSICALSIMPLEQUANTITY");
        assert_eq!(to_part21_document(&snapshot), doc);
    }

    #[semio_framework_async_macros::async_test]
    async fn codec_round_trip_via_dsl_and_pack() {
        let snapshot = from_part21_document("stdio.ifc", &parse_part21(FIXTURE).expect("parse"));
        let text = store::ArtifactDsl::print_dsl(&snapshot);
        let parsed = <IfcSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse_dsl");
        assert_eq!(parsed, snapshot);
        let bytes = store::ArtifactPack::encode_pack(&snapshot);
        let decoded = <IfcSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode_pack");
        assert_eq!(decoded, snapshot);
    }
}
//#endregion 🧪️Tests
