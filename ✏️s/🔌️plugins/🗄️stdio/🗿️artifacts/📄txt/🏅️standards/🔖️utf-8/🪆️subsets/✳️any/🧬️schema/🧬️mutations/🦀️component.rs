//! 🧬️ TxtMutation — document mutation dispatch. Every variant's `diff()`/`inverse()` is
//! handcrafted directly against `TxtDiff`/`TxtLinesDiff` -- no apply-and-capture.

use crate::artifacts::txt::schema::diff::{diff_set_snapshot, TxtDiff, TxtLineAdded, TxtLineModified, TxtLinesDiff};
use crate::artifacts::txt::schema::snapshot::LineEnding;
use crate::artifacts::txt::TxtSnapshot;
use protocol::Mutation;
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.txt`.
///
/// 🧪️ F6: `dsl::DslOps` derive added — emits `dsl::DslVariants` only (P6: `OpText`/`OpBinary`
/// are always handcrafted, see the `OpCodecs` region below). Classified DERIVE per
/// `f6-recon-report.md` §3's unified decision rule: walking every variant's fields (incl.
/// `SetSnapshot`'s whole `TxtSnapshot` payload), the only enum reached is `LineEnding`, which is
/// unit-variant-only and binds via `DslScalar` (see the snapshot module) — no data-carrying enum
/// anywhere in the tree, so `DslOps` compiles cleanly with no mirror-enum indirection needed.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum TxtMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        #[dsl(block)]
        snapshot: TxtSnapshot,
    },
    SetTrailingNewline {
        value: bool,
    },
    SetLineEnding {
        value: LineEnding,
    },
    InsertLine {
        index: usize,
        text: String,
    },
    RemoveLine {
        index: usize,
    },
    SetLine {
        index: usize,
        text: String,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Diff is the single semantics source: computed once,
/// applied once, returned once.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_txt_mutation(snapshot: &mut TxtSnapshot, mutation: &TxtMutation) -> protocol::MutationOutcome<TxtDiff> {
    let outcome = <TxtMutation as Mutation<TxtSnapshot>>::diff(mutation, &*snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<TxtSnapshot> for TxtMutation {
    type Diff = TxtDiff;

    fn diff(&self, base: &TxtSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            TxtMutation::NoMutation => TxtDiff::default(),
            TxtMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            TxtMutation::SetTrailingNewline { value } => {
                if base.trailing_newline == *value {
                    TxtDiff::default()
                } else {
                    TxtDiff { trailing_newline: Some(*value), ..Default::default() }
                }
            }
            TxtMutation::SetLineEnding { value } => {
                if base.line_ending == *value {
                    TxtDiff::default()
                } else {
                    TxtDiff { line_ending: Some(*value), ..Default::default() }
                }
            }
            TxtMutation::InsertLine { index, text } => TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![], added: vec![TxtLineAdded { index: *index, text: text.clone() }] }), ..Default::default() },
            TxtMutation::RemoveLine { index } => {
                if *index >= base.lines.len() {
                    TxtDiff::default()
                } else {
                    TxtDiff { lines: Some(TxtLinesDiff { removed: vec![*index], modified: vec![], added: vec![] }), ..Default::default() }
                }
            }
            TxtMutation::SetLine { index, text } => {
                if base.lines.get(*index).map_or(true, |cur| cur == text) {
                    TxtDiff::default()
                } else {
                    TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![TxtLineModified { index: *index, text: text.clone() }], added: vec![] }), ..Default::default() }
                }
            }
        })
    }

    fn inverse(&self, base: &TxtSnapshot) -> Vec<Self> {
        match self {
            TxtMutation::NoMutation => vec![TxtMutation::NoMutation],
            TxtMutation::SetSnapshot { .. } => vec![TxtMutation::SetSnapshot { snapshot: base.clone() }],
            TxtMutation::SetTrailingNewline { .. } => vec![TxtMutation::SetTrailingNewline { value: base.trailing_newline }],
            TxtMutation::SetLineEnding { .. } => vec![TxtMutation::SetLineEnding { value: base.line_ending }],
            TxtMutation::InsertLine { index, .. } => {
                let landed_at = (*index).min(base.lines.len());
                vec![TxtMutation::RemoveLine { index: landed_at }]
            }
            TxtMutation::RemoveLine { index } => match base.lines.get(*index) {
                Some(text) => vec![TxtMutation::InsertLine { index: *index, text: text.clone() }],
                None => vec![TxtMutation::NoMutation],
            },
            TxtMutation::SetLine { index, .. } => match base.lines.get(*index) {
                Some(text) => vec![TxtMutation::SetLine { index: *index, text: text.clone() }],
                None => vec![TxtMutation::NoMutation],
            },
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🎙️ Handcrafted `OpText` (P6: `dsl::DslOps` emits `DslVariants` only) — one-line grammar via
/// the derived `RecordSpec`/`DslVariants`. Body is the same ~15-line shape every `DslOps`-derived
/// enum's `OpText` impl uses (see `SpaceMutation`, `FlowMutationDsl`, and this pilot's own
/// `BinaryMutation`/`GifMutation` for precedent this copies verbatim).
impl OpText for TxtMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// ⚡️ Handcrafted `OpBinary` (P6) — pure forward to `dsl::variants_binary`, the generic
/// `format u8 (=1) | variant ordinal varint | record body` layout shared by every `DslVariants`
/// type. Zero per-artifact logic.
impl OpBinary for TxtMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion OpCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::os_spr::command::DiffAlgebra;
    use protocol::MutationDiff;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn base() -> TxtSnapshot {
        TxtSnapshot { lines: vec!["a".into(), "b".into(), "c".into()], trailing_newline: true, line_ending: LineEnding::Lf, ..Default::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn all_variants(b: &TxtSnapshot) -> Vec<TxtMutation> {
        vec![
            TxtMutation::NoMutation,
            TxtMutation::SetSnapshot { snapshot: TxtSnapshot { lines: vec!["z".into()], trailing_newline: false, line_ending: LineEnding::CrLf, ..Default::default() } },
            TxtMutation::SetTrailingNewline { value: !b.trailing_newline },
            TxtMutation::SetLineEnding { value: LineEnding::CrLf },
            TxtMutation::InsertLine { index: 1, text: "x".into() },
            TxtMutation::RemoveLine { index: 0 },
            TxtMutation::SetLine { index: 0, text: "changed".into() },
        ]
    }

    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        let b = base();
        for m in all_variants(&b) {
            let mut via_apply = b.clone();
            let returned = apply_txt_mutation(&mut via_apply, &m);
            let expected_diff = m.diff(&b);
            assert_eq!(returned, expected_diff, "returned diff mismatch for {m:?}");
            assert_eq!(via_apply, expected_diff.diff().apply(&b).unwrap(), "apply mismatch for {m:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
        let b = base();
        for m in all_variants(&b) {
            let mut mutated = b.clone();
            apply_txt_mutation(&mut mutated, &m);
            for undo in m.inverse(&b) {
                apply_txt_mutation(&mut mutated, &undo);
            }
            assert_eq!(mutated, b, "mutation-level inverse round-trip failed for {m:?}");

            let d = m.diff(&b);
            let next = d.diff().apply(&b).unwrap();
            let inv = d.diff().inverse(&b);
            assert_eq!(inv.apply(&next).unwrap(), b, "diff-level inverse round-trip failed for {m:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law_cartesian() {
        let b = base();
        let variants = all_variants(&b);
        for m1 in &variants {
            let d1 = m1.diff(&b);
            let mid = d1.diff().apply(&b).unwrap();
            for m2 in &variants {
                let d2 = m2.diff(&mid);
                let after = d2.diff().apply(&mid).unwrap();
                let mut merged = d1.diff().clone();
                merged.absorb(d2.diff().clone());
                assert_eq!(merged.apply(&b).unwrap(), after, "absorb({m1:?}, {m2:?}) mismatch");
            }
        }
    }

    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws (handcrafted impls over the
    /// `dsl::DslOps`-derived `DslVariants`), exercised over every variant incl. `SetSnapshot`'s
    /// full nested-record payload.
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        let b = base();
        for m in all_variants(&b) {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = TxtMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = TxtMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }

    //#region 🔖️OpsGrammarConformanceLaw
    /// 🧪️ P2-P3: `dsl::parse_grammar` + `dsl::Recognizer::compile` + `.recognize` against REAL
    /// `print_op` output for every variant, incl. `SetSnapshot`'s full nested-block payload.
    #[semio_framework_async_macros::async_test]
    async fn ops_grammar_conformance_law() {
        let grammar_text = crate::artifacts::txt::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO;
        let grammar = dsl::parse_grammar(grammar_text).expect("parse mutations grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        let b = base();
        for m in all_variants(&b) {
            let printed = m.print_op();
            let ok = recognizer.recognize(&printed).unwrap_or_else(|e| panic!("recognize({printed:?}) errored: {e:?}"));
            assert!(ok, "mutations grammar must recognize real print_op output {printed:?} for {m:?}");
        }
    }
    //#endregion 🔖️OpsGrammarConformanceLaw
}
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `📦️glue.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "📄set-snapshot/🧪️tests/appends-a-third-line-and-switches-to-crlf/🦀️component.rs"]
    mod tests_set_snapshot_appends_a_third_line_and_switches_to_crlf;
}
//#endregion 🧪️FixtureTests
