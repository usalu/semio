//! 🧵️ The hand-rolled `PlyDiff` text and binary codec and the demo diff cases.

use super::*;

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `PlyDiff` — `#[derive(dsl::DslDiff)]` is not
/// usable (see the module doc comment for the real `cargo check` citation: `PlyProperty`/
/// `PlyValue` are data-carrying enums reachable from `PlyElementDiff::properties` and
/// `PlyRowFieldChange::value`, the 3a "enum-in-tree" blocker per `f6-recon-report.md` §3a).
///
/// **Grammar** (real, not `serde_json`), following the ticket's §5 template verbatim: one
/// space-separated `name=value` token per changed top-level field (absent token = unchanged).
/// Bytes/strings are lowercase hex (`enc_str`/`dec_str` — no external base64 dep, no escaping).
/// `Option<T>` values use the uniform `[0]`=None / `[1,<T>]`=Some(T) tag. Structs are positional
/// `[f1,f2,...]` tuples. Data-carrying enums (`PlyProperty`, `PlyValue`) use a single-uppercase
/// (or, for `PlyValue`'s eight scalar kinds, single-lowercase) tag prefix immediately followed by
/// the bracketed payload. Collection triples print as `{[removed];[modified];[added]}` — for the
/// index-keyed `rows` triple, `removed`/`modified` are index-keyed; for the name-keyed `elements`
/// triple, `removed`/`modified` are NAME-keyed (hex) while `added` stays index-keyed (matches
/// `PlyElementsDiff`'s own real shape — see `f6-ply-report.md` for why this deliberately deviates
/// from gif89a's uniform-index-keyed triple helper).
///
/// Worked example (captured from a real test run, see `diff_codec_text_binary_roundtrip_law`):
/// `format=6c comments=[68656c6c6f] elements={[666163e5];[];[76657274657865:[P:[...],R:{...}]]}`
/// (illustrative shape only — see the test for the literal printed string).
//#region 🔖️Primitives






//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs




















//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs















//#endregion 🔖️DiffValueCodecs

//#region 🔖️RealBinaryPrimitives























//#endregion 🔖️RealBinaryPrimitives

//#region 🔖️RealBinaryDiffFrame










//#endregion 🔖️RealBinaryDiffFrame

//#region 🔖️TopLevel




//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoDiffCases
/// ✅️ Every representative `PlyDiff` shape (empty, plus a real `between()` result in BOTH
/// directions over `sweep_a()`/`sweep_b()`) — the single case list `diff_codec_text_binary_
/// roundtrip_law` (this file) AND `diff_grammar_conformance_law`/`protocol_walk_law`
/// (`⚙️engine/🦀️.rs`) all exercise. Covers every scalar field, the name-keyed `elements`
/// triple in all three flavors (removed/modified/added) simultaneously, the nested index-keyed
/// `rows` triple, the weak `properties` replace, and both `PlyProperty`/`PlyValue` enum tag
/// families (incl. `PlyValue::List`'s recursion).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sweep_a() -> PlySnapshot {
    PlySnapshot {
        schema: crate::STDIO_PLY_DOCUMENT_SCHEMA.into(),
        format: PlyFormat::Ascii,
        comments: vec!["a".into()],
        elements: vec![
            PlyElement {
                name: "vertex".into(),
                count: 2,
                properties: vec![PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Float }, PlyProperty::Scalar { name: "y".into(), kind: PlyScalarType::Float }],
                rows: vec![PlyRow { values: vec![PlyValue::Float(0.0), PlyValue::Float(0.0)] }, PlyRow { values: vec![PlyValue::Float(1.0), PlyValue::Float(1.0)] }],
            },
            PlyElement {
                name: "face".into(),
                count: 1,
                properties: vec![PlyProperty::List { name: "vertex_indices".into(), count_kind: PlyScalarType::UChar, value_kind: PlyScalarType::Int }],
                rows: vec![PlyRow { values: vec![PlyValue::List(vec![PlyValue::Int(0), PlyValue::Int(1), PlyValue::Int(2)])] }],
            },
        ],
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sweep_b() -> PlySnapshot {
    PlySnapshot {
        schema: crate::STDIO_PLY_DOCUMENT_SCHEMA.into(),
        format: PlyFormat::BinaryLittleEndian,
        comments: vec!["a".into(), "b".into()],
        elements: vec![
            PlyElement {
                name: "vertex".into(),
                count: 1,
                properties: vec![PlyProperty::Scalar { name: "nx".into(), kind: PlyScalarType::Double }, PlyProperty::Scalar { name: "ny".into(), kind: PlyScalarType::Double }],
                rows: vec![PlyRow { values: vec![PlyValue::Double(9.0), PlyValue::Double(-9.5)] }],
            },
            PlyElement { name: "edge".into(), count: 1, properties: vec![PlyProperty::Scalar { name: "weight".into(), kind: PlyScalarType::Double }], rows: vec![PlyRow { values: vec![PlyValue::Double(3.5)] }] },
        ],
    }
}

#[cfg(any(test, feature = "testkit"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_diff_cases() -> Vec<PlyDiff> {
    vec![
        PlyDiff::default(),
        PlyDiff {
            format: Some(PlyFormat::BinaryLittleEndian),
            comments: Some(PlyCommentsDiff {
                removed: vec![0],
                modified: vec![IndexedModified { index: 1, diff: "changed".to_string() }],
                added: vec![IndexedAdded { index: 0, item: "added".to_string() }],
            }),
            elements: Some(PlyElementsDiff { removed: vec!["face".into()], ..Default::default() }),
        },
        PlyDiff { format: Some(PlyFormat::Ascii), ..Default::default() },
    ]
}
//#endregion 🔖️DemoDiffCases
