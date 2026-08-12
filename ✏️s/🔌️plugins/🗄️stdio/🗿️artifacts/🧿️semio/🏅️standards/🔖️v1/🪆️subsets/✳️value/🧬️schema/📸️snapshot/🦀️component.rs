//! 🧬️ SemioValueSnapshot — an ordered, lexeme-preserving typed value GRAPH — from json.
//! Owned by the `value` subset: `SemioValueSnapshot`, `SemioValue`, `SemioValueEntry` (per
//! `w1b-type-ownership.md`), plus the supporting `ValueId`/`SemioValueNode` graph-backing types
//! this subset needs to make `SemioValue::Ref` genuinely referential rather than a dangling-by-
//! construction stub.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_SEMIOVALUE_DOCUMENT_SCHEMA: &str = "stdio.semio.value";
//#endregion 🔖️Ids

//#region 🔖️ValueId
/// 🪪 Stable identity for a node in the value graph — a NAMED single-field struct, never a bare
/// tuple newtype: `dsl` has no blanket `DslField` impl for tuples of any arity
/// (f6-final-summary.md §4.3, las/jpg-confirmed gap), and every other id-shaped type this program
/// introduces (`SemioQuaternion` in the shared `🧮️geometry` engine) follows the same named-field
/// convention rather than risk the same class of bug.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueId {
    pub value: String,
}

impl ValueId {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
    }
}
//#endregion 🔖️ValueId

//#region 🔖️SemioValue
/// 🍃️ One `Map` entry, in source order (never a `HashMap` — member insertion order is preserved,
/// the same convention `json`'s `JsonMember` uses, this subset's own informing source). Derives
/// `Default` (never constructed as a "real" empty entry — required by the shared
/// `engine::triples::NamedTripleDiff<K,D,T>`'s `Deserialize` derive, which needs `T: Default` due
/// to a `#[serde(default)]`-triggered bound-inference quirk on ITS generic fields).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioValueEntry {
    pub key: String,
    pub value: SemioValue,
}

/// 🌳 An ordered, lexeme-preserving typed value graph node — the master plan's spec row verbatim:
/// `SemioValue enum{Null,Bool,Int,Float,Str,Bytes,List,Map,Ref(ValueId)}`. `Int`/`Float` keep the
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
    Map { entries: Vec<SemioValueEntry> },
    Ref { id: ValueId },
}

impl Default for SemioValue {
    fn default() -> Self {
        SemioValue::Null
    }
}
//#endregion 🔖️SemioValue

//#region 🔖️ValueGraph
/// 📦️ One id-addressable node in the graph's backing store — the strong, keyed entity `Ref`
/// values resolve against. Real per-node diffability (see `🔺️diff`) makes this the format's
/// "keyed repeating structure" per the recipe, not just a scalar container. Derives `Default` for
/// the same `NamedTripleDiff<K,D,T>: Deserialize` bound-inference reason as `SemioValueEntry`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioValueNode {
    pub id: ValueId,
    pub value: SemioValue,
}
//#endregion 🔖️ValueGraph

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.value")]
pub struct SemioValueSnapshot {
    #[state(persistent)]
    pub schema: String,
    /// 🌱 The graph's entry point — any `SemioValue`, including a `Ref` into `nodes`.
    #[state(persistent)]
    pub root: SemioValue,
    /// 🕸️ The id-keyed backing store `Ref` values resolve against — ordered (insertion order
    /// preserved), id-addressable, a real strong-entity collection (never a `HashMap`, so decode
    /// -> encode never silently reorders it).
    #[state(persistent)]
    #[serde(default)]
    pub nodes: Vec<SemioValueNode>,
}

impl Default for SemioValueSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(),
            root: SemioValue::default(),
            nodes: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️SnapshotTextCodec
/// 🌳️ `SemioValueSnapshot`'s own real recursive text encoding — `[hex(schema),<value>,[<node>,...]]`
/// — genuinely walked/parsed field-by-field (never a hex dump of a `serde_json` blob). Reuses the
/// SAME tag-prefixed `SemioValue` grammar (`enc_semio_value`/`dec_semio_value`) the sibling
/// `🔺️diff`/`🧬️mutations` facets already define for their own text codecs — this subset has no
/// natural "on-disk file format" of its own the way `json`/`csv` do (`SemioValueSnapshot` is a
/// NEUTRAL semio type), so reusing one already-real, already-hand-rolled grammar as the single
/// source of truth for every facet is the honest choice, not a shortcut (`json`'s own `JsonValue`
/// text codec is likewise shared verbatim by its diff/mutations facets' `value=` token). Single
/// source of truth: `🧬️mutations/🦀️component.rs`'s `SetSnapshot` argument encoding calls THESE
/// functions directly rather than keeping its own second copy.
pub(crate) fn enc_semio_value_snapshot(s: &SemioValueSnapshot) -> String {
    let nodes = s.nodes.iter().map(crate::artifacts::semio::standards::v1::subsets::value::schema::diff::enc_semio_value_node).collect::<Vec<_>>().join(",");
    format!(
        "[{},{},[{}]]",
        crate::artifacts::semio::standards::v1::subsets::value::schema::diff::enc_str(&s.schema),
        crate::artifacts::semio::standards::v1::subsets::value::schema::diff::enc_semio_value(&s.root),
        nodes
    )
}
pub(crate) fn dec_semio_value_snapshot(s: &str) -> Result<SemioValueSnapshot, String> {
    use crate::artifacts::semio::standards::v1::engine::triples::{split_top_level, strip_brackets};
    use crate::artifacts::semio::standards::v1::subsets::value::schema::diff::{dec_semio_value_node, dec_semio_value, dec_str};
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [schema_s, root_s, nodes_s] = parts.as_slice() else {
        return Err(format!("semio value snapshot: expected 3 top-level fields, got {}", parts.len()));
    };
    let nodes_inner = strip_brackets(nodes_s)?;
    let nodes = split_top_level(nodes_inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_semio_value_node).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioValueSnapshot { schema: dec_str(schema_s)?, root: dec_semio_value(root_s)?, nodes })
}
//#endregion 🔖️SnapshotTextCodec

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁️ Real recursive text/binary round trip — NOT a per-on-disk-file-format codec (this subset's
/// snapshot is a NEUTRAL semio type, like `json`'s own `JsonSnapshot`, not a real-world file
/// format), so — mirroring `json`'s own text-native precedent exactly (`🔣️json/…/📸️snapshot/
/// 🦀️component.rs`'s `ArtifactPack::encode_pack_with`: `write_json_text(&self.value).into_bytes()`
/// passed straight to `wrap_binary`, no distinct "binary JsonValue" layout) — the PACK bytes are
/// the SAME real compact text this facet's DSL emits, wrapped in the semio envelope. No
/// `serde_json` anywhere in this impl block.
impl store::ArtifactDsl for SemioValueSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str { STDIO_SEMIOVALUE_DOCUMENT_SCHEMA }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        dec_semio_value_snapshot(body.trim()).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = enc_semio_value_snapshot(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioValueSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = enc_semio_value_snapshot(self).into_bytes();
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
        let text = std::str::from_utf8(&inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        dec_semio_value_snapshot(text).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Demo
/// 📄️ The demo `stdio.semio.value` snapshot — exercises every `SemioValue` variant (`Null`/
/// `Bool`/`Int`/`Float`/`Str`/`Bytes`/`List`/`Map`/`Ref`) at least once, plus a real `Ref` into
/// `nodes`. The single source of truth for
/// `📚️examples/…/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` (both are literally this
/// snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law` in
/// `🎹️composer/🦀️component.rs`) and for this file's own round-trip tests below — same convention
/// `json`'s `demo_json_snapshot()`/`workflow`'s `demo_workflow_snapshot()` use.
#[cfg(test)]
pub(crate) fn demo_semio_value_snapshot() -> SemioValueSnapshot {
    SemioValueSnapshot {
        schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(),
        root: SemioValue::Map {
            entries: vec![
                SemioValueEntry { key: "name".into(), value: SemioValue::Str { value: "semio".into() } },
                SemioValueEntry { key: "count".into(), value: SemioValue::Int { lexeme: "42".into() } },
                SemioValueEntry { key: "ratio".into(), value: SemioValue::Float { lexeme: "3.500".into() } },
                SemioValueEntry { key: "blob".into(), value: SemioValue::Bytes { value: vec![0, 1, 2, 255] } },
                SemioValueEntry {
                    key: "tags".into(),
                    value: SemioValue::List { items: vec![SemioValue::Str { value: "a".into() }, SemioValue::Null] },
                },
                SemioValueEntry { key: "linked".into(), value: SemioValue::Ref { id: ValueId::new("n1") } },
            ],
        },
        nodes: vec![SemioValueNode { id: ValueId::new("n1"), value: SemioValue::Bool { value: true } }],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_pack_round_trips() {
        let snap = demo_semio_value_snapshot();
        let bytes = <SemioValueSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioValueSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[test]
    fn dsl_text_round_trips() {
        let snap = demo_semio_value_snapshot();
        let text = <SemioValueSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioValueSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    /// 🧪️ codec_retention_law: decode(encode(x)) == x, with an explicit lexeme-fidelity assertion
    /// (an arbitrary-precision int lexeme and a trailing-zero float lexeme — both would silently
    /// corrupt if either variant were ever routed through `i64`/`f64`) plus the `Ref`/graph shape
    /// surviving intact.
    #[test]
    fn codec_retention_law_preserves_lexemes_bytes_and_graph_shape() {
        let snap = SemioValueSnapshot {
            schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(),
            root: SemioValue::List {
                items: vec![
                    SemioValue::Int { lexeme: "9007199254740993".into() },
                    SemioValue::Float { lexeme: "1.2300".into() },
                    SemioValue::Bytes { value: (0..=255u8).collect() },
                    SemioValue::Ref { id: ValueId::new("root-child") },
                ],
            },
            nodes: vec![SemioValueNode { id: ValueId::new("root-child"), value: SemioValue::Str { value: "leaf".into() } }],
        };
        let bytes = <SemioValueSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioValueSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
        match &back.root {
            SemioValue::List { items } => {
                assert_eq!(items[0], SemioValue::Int { lexeme: "9007199254740993".into() }, "int lexeme must survive verbatim");
                assert_eq!(items[1], SemioValue::Float { lexeme: "1.2300".into() }, "float lexeme (incl. trailing zero) must survive verbatim");
                assert_eq!(items[2], SemioValue::Bytes { value: (0..=255u8).collect() });
            }
            other => panic!("expected list root, got {other:?}"),
        }
        assert_eq!(back.nodes.len(), 1);
        assert_eq!(back.nodes[0].id, ValueId::new("root-child"));
    }
}
//#endregion 🔖️Tests
