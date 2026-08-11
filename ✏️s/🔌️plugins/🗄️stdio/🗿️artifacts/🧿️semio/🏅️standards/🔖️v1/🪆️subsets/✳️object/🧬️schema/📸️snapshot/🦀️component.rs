//! 🧬️ SemioObjectSnapshot — an ordered, lexeme-preserving typed object GRAPH — from json.
//! Owned by the `object` subset: `SemioObjectSnapshot`, `SemioValue`, `SemioObjectEntry` (per
//! `w1b-type-ownership.md`), plus the supporting `ObjectId`/`SemioObjectNode` graph-backing types
//! this subset needs to make `SemioValue::Ref` genuinely referential rather than a dangling-by-
//! construction stub.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA: &str = "stdio.semio.object";
//#endregion 🔖️Ids

//#region 🔖️ObjectId
/// 🪪 Stable identity for a node in the object graph — a NAMED single-field struct, never a bare
/// tuple newtype: `dsl` has no blanket `DslField` impl for tuples of any arity
/// (f6-final-summary.md §4.3, las/jpg-confirmed gap), and every other id-shaped type this program
/// introduces (`SemioQuaternion` in the shared `🧮️geometry` engine) follows the same named-field
/// convention rather than risk the same class of bug.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectId {
    pub value: String,
}

impl ObjectId {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
    }
}
//#endregion 🔖️ObjectId

//#region 🔖️SemioValue
/// 🍃️ One `Map` entry, in source order (never a `HashMap` — member insertion order is preserved,
/// the same convention `json`'s `JsonMember` uses, this subset's own informing source). Derives
/// `Default` (never constructed as a "real" empty entry — required by the shared
/// `engine::triples::NamedTripleDiff<K,D,T>`'s `Deserialize` derive, which needs `T: Default` due
/// to a `#[serde(default)]`-triggered bound-inference quirk on ITS generic fields).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioObjectEntry {
    pub key: String,
    pub value: SemioValue,
}

/// 🌳 An ordered, lexeme-preserving typed value graph node — the master plan's spec row verbatim:
/// `SemioValue enum{Null,Bool,Int,Float,Str,Bytes,List,Map,Ref(ObjectId)}`. `Int`/`Float` keep the
/// ORIGINAL SOURCE LEXEME verbatim (never round-tripped through `i64`/`f64` — an import codec may
/// see e.g. a 19-digit id or a high-precision decimal that a native numeric type would silently
/// corrupt), split into two variants (unlike `json`'s single `Number`) because this graph is
/// explicitly TYPED, not merely textual — `codec_retention_law` below proves both survive a pack
/// round trip byte-for-byte. `List`/`Map` are the format's strong, ordered, keyed repeating
/// structures. `Ref` is what makes this a GRAPH rather than a plain tree — `json`'s own `JsonValue`
/// (this subset's informing source) has no equivalent; every `JsonValue` is strictly a tree. Every
/// non-unit variant is a struct (named-field) variant, never a bare tuple variant — serde's
/// internally-tagged (`tag = "kind"`) representation can only merge the tag into map-shaped
/// content; a tuple variant wrapping a non-map payload compiles but fails at RUNTIME serialization
/// (identical citation in `json`'s own `JsonValue` doc comment).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SemioValue {
    Null,
    Bool { value: bool },
    Int { lexeme: String },
    Float { lexeme: String },
    Str { value: String },
    Bytes { value: Vec<u8> },
    List { items: Vec<SemioValue> },
    Map { entries: Vec<SemioObjectEntry> },
    Ref { id: ObjectId },
}

impl Default for SemioValue {
    fn default() -> Self {
        SemioValue::Null
    }
}
//#endregion 🔖️SemioValue

//#region 🔖️ObjectGraph
/// 📦️ One id-addressable node in the graph's backing store — the strong, keyed entity `Ref`
/// values resolve against. Real per-node diffability (see `🔺️diff`) makes this the format's
/// "keyed repeating structure" per the recipe, not just a scalar container. Derives `Default` for
/// the same `NamedTripleDiff<K,D,T>: Deserialize` bound-inference reason as `SemioObjectEntry`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioObjectNode {
    pub id: ObjectId,
    pub value: SemioValue,
}
//#endregion 🔖️ObjectGraph

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.object")]
pub struct SemioObjectSnapshot {
    #[state(persistent)]
    pub schema: String,
    /// 🌱 The graph's entry point — any `SemioValue`, including a `Ref` into `objects`.
    #[state(persistent)]
    pub root: SemioValue,
    /// 🕸️ The id-keyed backing store `Ref` values resolve against — ordered (insertion order
    /// preserved), id-addressable, a real strong-entity collection (never a `HashMap`, so decode
    /// -> encode never silently reorders it).
    #[state(persistent)]
    #[serde(default)]
    pub objects: Vec<SemioObjectNode>,
}

impl Default for SemioObjectSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA.into(),
            root: SemioValue::default(),
            objects: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁️ JSON-pack round trip (honest, genuinely working — not a per-format binary codec, since
/// this subset's snapshot is a NEUTRAL semio type, not an on-disk file format). Wrapped in the
/// same `store::semio_format` envelope every stdio artifact uses. `serde_json` here is a wire
/// carrier for `SemioValue`'s OWN typed shape (Int/Float lexemes, Bytes, Ref) — not a
/// `serde_json::Value` fallback; the model itself carries no untyped JSON.
impl store::ArtifactDsl for SemioObjectSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str { STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?;
            bytes.push(byte);
            i += 2;
        }
        serde_json::from_slice(&bytes).map_err(|e| store::TextError::new(format!("json decode: {e}"), dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = serde_json::to_vec(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioObjectSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = serde_json::to_vec(self).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        serde_json::from_slice(&inner).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧪️ Exercises every `SemioValue` variant plus a real `Ref` into `objects` — the minimal
    /// non-trivial fixture every other test module's `snap(...)` helper wraps.
    fn sample_snapshot() -> SemioObjectSnapshot {
        SemioObjectSnapshot {
            schema: STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA.into(),
            root: SemioValue::Map {
                entries: vec![
                    SemioObjectEntry { key: "name".into(), value: SemioValue::Str { value: "semio".into() } },
                    SemioObjectEntry { key: "count".into(), value: SemioValue::Int { lexeme: "42".into() } },
                    SemioObjectEntry { key: "ratio".into(), value: SemioValue::Float { lexeme: "3.500".into() } },
                    SemioObjectEntry { key: "blob".into(), value: SemioValue::Bytes { value: vec![0, 1, 2, 255] } },
                    SemioObjectEntry {
                        key: "tags".into(),
                        value: SemioValue::List { items: vec![SemioValue::Str { value: "a".into() }, SemioValue::Null] },
                    },
                    SemioObjectEntry { key: "linked".into(), value: SemioValue::Ref { id: ObjectId::new("n1") } },
                ],
            },
            objects: vec![SemioObjectNode { id: ObjectId::new("n1"), value: SemioValue::Bool { value: true } }],
        }
    }

    #[test]
    fn json_pack_round_trips() {
        let snap = sample_snapshot();
        let bytes = <SemioObjectSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioObjectSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[test]
    fn dsl_text_round_trips() {
        let snap = sample_snapshot();
        let text = <SemioObjectSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    /// 🧪️ codec_retention_law: decode(encode(x)) == x, with an explicit lexeme-fidelity assertion
    /// (an arbitrary-precision int lexeme and a trailing-zero float lexeme — both would silently
    /// corrupt if either variant were ever routed through `i64`/`f64`) plus the `Ref`/graph shape
    /// surviving intact.
    #[test]
    fn codec_retention_law_preserves_lexemes_bytes_and_graph_shape() {
        let snap = SemioObjectSnapshot {
            schema: STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA.into(),
            root: SemioValue::List {
                items: vec![
                    SemioValue::Int { lexeme: "9007199254740993".into() },
                    SemioValue::Float { lexeme: "1.2300".into() },
                    SemioValue::Bytes { value: (0..=255u8).collect() },
                    SemioValue::Ref { id: ObjectId::new("root-child") },
                ],
            },
            objects: vec![SemioObjectNode { id: ObjectId::new("root-child"), value: SemioValue::Str { value: "leaf".into() } }],
        };
        let bytes = <SemioObjectSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioObjectSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
        match &back.root {
            SemioValue::List { items } => {
                assert_eq!(items[0], SemioValue::Int { lexeme: "9007199254740993".into() }, "int lexeme must survive verbatim");
                assert_eq!(items[1], SemioValue::Float { lexeme: "1.2300".into() }, "float lexeme (incl. trailing zero) must survive verbatim");
                assert_eq!(items[2], SemioValue::Bytes { value: (0..=255u8).collect() });
            }
            other => panic!("expected list root, got {other:?}"),
        }
        assert_eq!(back.objects.len(), 1);
        assert_eq!(back.objects[0].id, ObjectId::new("root-child"));
    }
}
//#endregion 🔖️Tests
