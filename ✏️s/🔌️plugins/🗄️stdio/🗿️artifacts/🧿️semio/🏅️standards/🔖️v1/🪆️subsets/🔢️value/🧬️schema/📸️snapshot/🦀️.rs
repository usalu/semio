//! 🧬️ SemioValueSnapshot — an ordered, lexeme-preserving typed value GRAPH — from json.
//! Owned by the `value` subset: `SemioValueSnapshot`, `SemioValue`, `SemioValueEntry` (per
//! `w1b-type-ownership.md`), plus the supporting `ValueId`/`SemioValueNode` graph-backing types
//! this subset needs to make `SemioValue::Ref` genuinely referential rather than a dangling-by-
//! construction stub.

use framework_schema::ArtifactSchema;

//#region 🔖️Ids
pub const STDIO_SEMIOVALUE_DOCUMENT_SCHEMA: &str = "stdio.semio.value";
//#endregion 🔖️Ids

//#region 🔖️ValueId
/// 🪪 Stable identity for a node in the value graph — a NAMED single-field struct, never a bare
/// tuple newtype: `dsl` has no blanket `DslField` impl for tuples of any arity
/// (f6-final-summary.md §4.3, las/jpg-confirmed gap), and every other id-shaped type this program
/// introduces (`SemioQuaternion` in the shared `🧮️geometry` engine) follows the same named-field
/// convention rather than risk the same class of bug.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ValueId {
    pub value: String,
}

impl ValueId {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
/// to a `#[value(default)]`-triggered bound-inference quirk on ITS generic fields).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, value_derive::RetireOwned)]
#[value(tag = "kind", rename_all = "camelCase")]
#[derive(Default)]
pub enum SemioValue {
    #[default]
    Null,
    Bool {
        value: bool,
    },
    Int {
        lexeme: String,
    },
    Float {
        lexeme: String,
    },
    Str {
        value: String,
    },
    Bytes {
        value: Vec<u8>,
    },
    List {
        items: Vec<SemioValue>,
    },
    Map {
        entries: Vec<SemioValueEntry>,
    },
    Ref {
        id: ValueId,
    },
}

//#endregion 🔖️SemioValue

//#region 🔖️ScalarLabel
/// 🏷️ The (en, de) display text of a scalar value for history labels — numbers trimmed to at most three decimals with a
/// German decimal comma, strings quoted, booleans and null as words; `None` for bytes, lists, maps and references.
pub fn semio_value_scalar_label(value: &SemioValue) -> Option<(String, String)> {
    let number = |lexeme: &str| lexeme.parse::<f64>().ok().filter(|value| value.is_finite()).map(|value| {
        let en = format!("{}", (value * 1_000.0).round() / 1_000.0);
        let de = en.replace('.', ",");
        (en, de)
    });
    match value {
        SemioValue::Null => Some(("nothing".into(), "nichts".into())),
        SemioValue::Bool { value } => Some(if *value { ("on".into(), "an".into()) } else { ("off".into(), "aus".into()) }),
        SemioValue::Int { lexeme } | SemioValue::Float { lexeme } => number(lexeme),
        SemioValue::Str { value } => Some((format!("\"{value}\""), format!("\"{value}\""))),
        SemioValue::Bytes { .. } | SemioValue::List { .. } | SemioValue::Map { .. } | SemioValue::Ref { .. } => None,
    }
}
//#endregion 🔖️ScalarLabel

//#region 🔖️ValueGraph
/// 📦️ One id-addressable node in the graph's backing store — the strong, keyed entity `Ref`
/// values resolve against. Real per-node diffability (see `🔺️diff`) makes this the format's
/// "keyed repeating structure" per the recipe, not just a scalar container. Derives `Default` for
/// the same `NamedTripleDiff<K,D,T>: Deserialize` bound-inference reason as `SemioValueEntry`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioValueNode {
    pub id: ValueId,
    pub value: SemioValue,
}
//#endregion 🔖️ValueGraph

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.value")]
pub struct SemioValueSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 🌱 The graph's entry point — any `SemioValue`, including a `Ref` into `nodes`.
    #[state(artifact)]
    pub root: SemioValue,
    /// 🕸️ The id-keyed backing store `Ref` values resolve against — ordered (insertion order
    /// preserved), id-addressable, a real strong-entity collection (never a `HashMap`, so decode
    /// -> encode never silently reorders it).
    #[state(artifact)]
    #[value(default)]
    pub nodes: Vec<SemioValueNode>,
}

impl Default for SemioValueSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root: SemioValue::default(), nodes: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️SnapshotTextCodec


//#endregion 🔖️SnapshotTextCodec

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🌉️ExternalCodecBridge



//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Wire







//#endregion 🔖️Wire

//#region 🔖️Demo
/// 📄️ The demo `stdio.semio.value` snapshot — exercises every `SemioValue` variant (`Null`/
/// `Bool`/`Int`/`Float`/`Str`/`Bytes`/`List`/`Map`/`Ref`) at least once, plus a real `Ref` into
/// `nodes`. The single source of truth for
/// `📚️examples/…/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` (both are literally this
/// snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law` in
/// `🎹️composer/🦀️.rs`) and for this file's own round-trip tests below — same convention
/// `json`'s `demo_json_snapshot()`/`flow`'s `demo_flow_snapshot()` use.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_semio_value_snapshot() -> SemioValueSnapshot {
    SemioValueSnapshot {
        schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(),
        root: SemioValue::Map {
            entries: vec![
                SemioValueEntry { key: "name".into(), value: SemioValue::Str { value: "semio".into() } },
                SemioValueEntry { key: "count".into(), value: SemioValue::Int { lexeme: "42".into() } },
                SemioValueEntry { key: "ratio".into(), value: SemioValue::Float { lexeme: "3.500".into() } },
                SemioValueEntry { key: "blob".into(), value: SemioValue::Bytes { value: vec![0, 1, 2, 255] } },
                SemioValueEntry { key: "tags".into(), value: SemioValue::List { items: vec![SemioValue::Str { value: "a".into() }, SemioValue::Null] } },
                SemioValueEntry { key: "linked".into(), value: SemioValue::Ref { id: ValueId::new("n1") } },
            ],
        },
        nodes: vec![SemioValueNode { id: ValueId::new("n1"), value: SemioValue::Bool { value: true } }],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests






