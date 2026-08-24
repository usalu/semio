//! 🧬️ TxtMutation — document mutation dispatch. Every variant's `diff()`/`inverse()` is
//! handcrafted directly against `TxtDiff`/`TxtLinesDiff` -- no apply-and-capture.
//!
//! 🔒️ REPRESENTABILITY, decided here rather than worked around downstream. `TxtSnapshot::to_body`
//! is `lines.join(sep)` plus an optional terminator, so `(L, true)` and `(L ++ [""], false)` render
//! the SAME bytes, as do `(vec![], true)` and `(vec![""], true)`; `from_body` can only return one
//! pre-image and returns the terminated one. The pair is therefore injective over exactly the
//! images `from_body` produces and nowhere else, and a snapshot outside that image is a document
//! this carrier cannot write down: exporting it and reading it back yields a DIFFERENT snapshot,
//! silently one line short. Measured, not argued — the real 170-line interview transcript ends
//! `…conversation.\n\n`, so `SetTrailingNewline { value: false }` on it renders 170 lines with no
//! terminator, which reads back as 169 WITH one, and the inverse can no longer recover the lost
//! blank line (ticket `26/08/23/END-TO-END-TESTING-REFACTOR`,
//! `every_feature_row_inverts_back_to_the_real_document`).
//!
//! The remedy is the NARROWING below rather than a wider snapshot type: every variant is gated on
//! [`non_canonical_reason`], so the only `TxtSnapshot` values this vocabulary can reach are the
//! canonical ones, on which `from_body`/`to_body` IS a bijection. A mutation that would leave the
//! document outside that image is REJECTED with [`CODE_NOT_REPRESENTABLE`] and changes nothing —
//! the same "refuse rather than silently lose" discipline the sibling `📰xml ✳️valid` vocabulary
//! applies to §2.8. The cost is stated plainly: on a document whose last line is empty,
//! `set-trailing-newline false` has no result at all, because the document it would name is already
//! spelled `(L, true)` with one line fewer. Reaching that document is `remove-line`'s job, not this
//! kind's.

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

//#region 🔖️Kinds
/// 🗂️ Kebab-case spelling of every `TxtMutation` variant, declaration order, mirrored by this
/// subset's `🧪️oracle/🔣️component.json` mutation catalog (`txt-utf-8-any`). The completeness gate
/// reads that JSON catalog, never this enum, so `kinds_match_enum_variants_and_catalog` below is
/// what keeps the two lists honest.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-trailing-newline", "set-line-ending", "insert-line", "remove-line", "set-line"];

/// 🏷️ The kebab-case kind one mutation value spells — the same names [`KINDS`] lists, used by the
/// rejection diagnostics so a refusal names the kind that was refused.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn kind_of(mutation: &TxtMutation) -> &'static str {
    match mutation {
        TxtMutation::NoMutation => "no-mutation",
        TxtMutation::SetSnapshot { .. } => "set-snapshot",
        TxtMutation::SetTrailingNewline { .. } => "set-trailing-newline",
        TxtMutation::SetLineEnding { .. } => "set-line-ending",
        TxtMutation::InsertLine { .. } => "insert-line",
        TxtMutation::RemoveLine { .. } => "remove-line",
        TxtMutation::SetLine { .. } => "set-line",
    }
}
//#endregion 🔖️Kinds

//#region 🔖️Representability
/// 🚫 The fault code every mutation refused for leaving the document unrepresentable reports under.
pub const CODE_NOT_REPRESENTABLE: &str = "stdio.txt.mutation-not-representable";

/// 🔒️ Why `(lines, trailing_newline)` is not the canonical decomposition of the body it renders, or
/// `None` when it is. Two families of pairs collide under [`TxtSnapshot::to_body`] and `from_body`
/// resolves both in favour of the terminated reading, so exactly those two shapes are outside its
/// image: a terminated document with no lines at all, and an unterminated document whose last line
/// is empty. Everything else round-trips byte-for-byte through the carrier.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn non_canonical_reason(lines: &[String], trailing_newline: bool) -> Option<String> {
    non_canonical_shape(lines.len(), lines.last().is_some_and(|line| line.is_empty()), trailing_newline)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn non_canonical_shape(line_count: usize, last_line_is_empty: bool, trailing_newline: bool) -> Option<String> {
    if trailing_newline && line_count == 0 {
        return Some("a document with no lines cannot carry a trailing terminator — that pair renders the very bytes the one-empty-line document renders, and reading them back returns the latter".to_string());
    }
    if !trailing_newline && last_line_is_empty {
        return Some("a document whose last line is empty cannot drop its trailing terminator — that pair renders the very bytes the same document one line shorter renders, and reading them back returns the latter, losing the empty line".to_string());
    }
    None
}

/// 📐️ The line count and last-line emptiness `mutation` would leave behind, derived per variant from
/// `base` rather than by applying anything — the gate has to answer before a diff exists, and
/// apply-and-inspect is the same banned shortcut as apply-and-capture.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn resulting_shape(base: &TxtSnapshot, mutation: &TxtMutation) -> (usize, bool, bool) {
    let ends_empty = |lines: &[String]| lines.last().is_some_and(|line| line.is_empty());
    let unchanged = (base.lines.len(), ends_empty(&base.lines), base.trailing_newline);
    match mutation {
        TxtMutation::SetSnapshot { snapshot } => (snapshot.lines.len(), ends_empty(&snapshot.lines), snapshot.trailing_newline),
        TxtMutation::SetTrailingNewline { value } => (unchanged.0, unchanged.1, *value),
        TxtMutation::InsertLine { index, text } => {
            let at = (*index).min(base.lines.len());
            (base.lines.len() + 1, if at == base.lines.len() { text.is_empty() } else { unchanged.1 }, base.trailing_newline)
        }
        TxtMutation::RemoveLine { index } if *index < base.lines.len() => {
            let remaining = base.lines.len() - 1;
            (remaining, remaining > 0 && base.lines[if *index == remaining { remaining - 1 } else { remaining }].is_empty(), base.trailing_newline)
        }
        TxtMutation::SetLine { index, text } if *index + 1 == base.lines.len() => (base.lines.len(), text.is_empty(), base.trailing_newline),
        _ => unchanged,
    }
}

/// 🛡️ The message naming why `mutation` may not be applied to `base`, or `None` when it may. A
/// mutation that leaves the document exactly as shaped as it found it is never refused, so a base
/// built by hand outside this vocabulary does not turn `NoMutation` into a fault.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn representability_violation(base: &TxtSnapshot, mutation: &TxtMutation) -> Option<String> {
    let shape = resulting_shape(base, mutation);
    if shape == (base.lines.len(), base.lines.last().is_some_and(|line| line.is_empty()), base.trailing_newline) {
        return None;
    }
    non_canonical_shape(shape.0, shape.1, shape.2).map(|reason| format!("{}: {reason}", kind_of(mutation)))
}
//#endregion 🔖️Representability

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
        if let Some(reason) = representability_violation(base, self) {
            return protocol::MutationOutcome::error(CODE_NOT_REPRESENTABLE, reason, Vec::<String>::new());
        }
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

    /// 🧪️ `kinds_match_enum_variants_and_catalog`: `KINDS` lists every `TxtMutation` variant
    /// exactly once (the `match` below has no wildcard arm, so a new variant fails to compile
    /// here first) AND matches the mutation catalog this subset's `🧪️oracle/🔣️component.json`
    /// declares, in the same order — the framework's completeness gate reads that JSON, never this
    /// enum, so this test is the only thing tying the two declarations together.
    #[semio_framework_async_macros::async_test]
    async fn kinds_match_enum_variants_and_catalog() {
        let b = base();
        let variant_kinds: std::collections::BTreeSet<&str> = all_variants(&b).iter().map(kind_of).collect();
        let declared_kinds: std::collections::BTreeSet<&str> = KINDS.iter().copied().collect();
        assert_eq!(variant_kinds, declared_kinds, "KINDS must list every TxtMutation variant exactly once");

        let manifest: serde_json::Value = serde_json::from_str(include_str!("../../🧪️oracle/🔣️component.json")).expect("valid catalog JSON");
        let catalog_kinds: Vec<&str> = manifest["mutationCatalogs"][0]["kinds"].as_array().expect("mutationCatalogs[0].kinds array").iter().map(|value| value.as_str().expect("kind is a string")).collect();
        assert_eq!(catalog_kinds, KINDS.to_vec(), "the manifest's mutationCatalogs[0].kinds must match KINDS exactly, declaration order included");
    }

    //#region 🔖️RepresentabilityLaw
    /// 📄️ A document shaped like the real interview transcript this subset's case mutates: an
    /// LF-terminated body whose LAST line is empty, i.e. one that ends `…\n\n`.
    // 🚫️async: E1 pure test-fixture builder, no I/O — see R9
    fn ends_with_a_blank_line() -> TxtSnapshot {
        TxtSnapshot { lines: vec!["a".into(), "b".into(), String::new()], trailing_newline: true, line_ending: LineEnding::Lf, ..Default::default() }
    }

    /// 🔒️ The collision itself, pinned: two different pairs render the same bytes and the carrier
    /// returns only the terminated one. This is the fact the narrowing exists for, so it is asserted
    /// rather than described — and the second half asserts the narrowing, that the losing pre-image
    /// is now named as unrepresentable instead of being silently reachable.
    #[test]
    fn the_line_terminator_pairs_that_collide_are_exactly_the_ones_refused() {
        let terminated = TxtSnapshot { lines: vec!["a".into()], trailing_newline: true, ..Default::default() };
        let empty_last_line = TxtSnapshot { lines: vec!["a".into(), String::new()], trailing_newline: false, ..Default::default() };
        assert_eq!(terminated.to_body(), empty_last_line.to_body(), "these are the two pre-images that collide");
        assert_eq!(TxtSnapshot::from_body(&terminated.to_body()).lines, terminated.lines, "the carrier resolves the tie in favour of the terminator");
        assert_eq!(non_canonical_reason(&terminated.lines, terminated.trailing_newline), None, "the reachable pre-image is canonical");
        assert!(non_canonical_reason(&empty_last_line.lines, empty_last_line.trailing_newline).is_some(), "the losing pre-image must be named unrepresentable");
        assert!(non_canonical_reason(&[], true).is_some(), "so must the no-lines-but-terminated pair, which renders what the one-empty-line document renders");
    }

    /// 🚫 `set-trailing-newline false` on a document whose last line is empty is refused, reports
    /// [`CODE_NOT_REPRESENTABLE`] and leaves the document exactly where it was — the whole point
    /// being that its result is already spelled by a document one line shorter.
    #[test]
    fn set_trailing_newline_false_is_refused_when_the_last_line_is_empty() {
        let base = ends_with_a_blank_line();
        let mut next = base.clone();
        let outcome = apply_txt_mutation(&mut next, &TxtMutation::SetTrailingNewline { value: false });
        assert!(outcome.messages().iter().any(|message| message.code.0 == CODE_NOT_REPRESENTABLE), "got {:?}", outcome.messages());
        assert_eq!(next, base, "a refused mutation must leave the document untouched");

        let ordinary = TxtSnapshot { lines: vec!["a".into(), "b".into()], trailing_newline: true, line_ending: LineEnding::Lf, ..Default::default() };
        let mut dropped = ordinary.clone();
        let allowed = apply_txt_mutation(&mut dropped, &TxtMutation::SetTrailingNewline { value: false });
        assert!(allowed.messages().iter().all(|message| message.code.0 != CODE_NOT_REPRESENTABLE), "the same kind must still work where its result IS representable");
        assert_eq!(dropped.to_body(), "a\nb");
    }

    /// 🔁️ Every kind, on the very document shape the collision lives on: whatever the vocabulary
    /// lets through must be a snapshot the carrier can write down and read back UNCHANGED, and
    /// whatever it refuses must change nothing. This is the property the narrowing buys — no
    /// reachable `TxtSnapshot` loses a line to its own serialization.
    #[test]
    fn no_reachable_snapshot_survives_its_own_carrier_differently() {
        let base = ends_with_a_blank_line();
        let candidates = vec![
            TxtMutation::NoMutation,
            TxtMutation::SetSnapshot { snapshot: TxtSnapshot { lines: vec!["z".into(), String::new()], trailing_newline: false, ..Default::default() } },
            TxtMutation::SetTrailingNewline { value: false },
            TxtMutation::SetLineEnding { value: LineEnding::CrLf },
            TxtMutation::InsertLine { index: 3, text: String::new() },
            TxtMutation::RemoveLine { index: 2 },
            TxtMutation::SetLine { index: 2, text: "no longer blank".into() },
        ];
        let mut refused = 0usize;
        for mutation in &candidates {
            let mut next = base.clone();
            let outcome = apply_txt_mutation(&mut next, mutation);
            if outcome.messages().iter().any(|message| message.code.0 == CODE_NOT_REPRESENTABLE) {
                refused += 1;
                assert_eq!(next, base, "the refused {mutation:?} must leave the document untouched");
                continue;
            }
            let round_tripped = TxtSnapshot::from_body(&next.to_body());
            assert_eq!(round_tripped.lines, next.lines, "{mutation:?} reached a snapshot its own carrier reads back differently");
            assert_eq!(round_tripped.trailing_newline, next.trailing_newline, "{mutation:?} reached a snapshot its own carrier reads back differently");
        }
        assert_eq!(refused, 2, "exactly `set-trailing-newline false` and the `set-snapshot` carrying the losing pre-image are unrepresentable here");
    }

    /// ↩️ The inverse law over the blank-line-ending document, which is where it used to break:
    /// every kind the vocabulary now admits still returns the document to exactly where it started.
    #[test]
    fn every_admitted_kind_inverts_on_a_document_ending_in_a_blank_line() {
        let base = ends_with_a_blank_line();
        for mutation in [
            TxtMutation::NoMutation,
            TxtMutation::SetSnapshot { snapshot: TxtSnapshot { lines: vec!["z".into()], trailing_newline: true, ..Default::default() } },
            TxtMutation::SetLineEnding { value: LineEnding::CrLf },
            TxtMutation::InsertLine { index: 1, text: "x".into() },
            TxtMutation::RemoveLine { index: 2 },
            TxtMutation::SetLine { index: 0, text: "changed".into() },
        ] {
            let mut next = base.clone();
            let outcome = apply_txt_mutation(&mut next, &mutation);
            assert!(outcome.messages().iter().all(|message| message.code.0 != CODE_NOT_REPRESENTABLE), "{mutation:?} must be admitted here");
            assert_ne!(next, base, "{mutation:?} must actually move the document, or it proves nothing");
            for undo in mutation.inverse(&base) {
                apply_txt_mutation(&mut next, &undo);
            }
            assert_eq!(next, base, "inverse round trip failed for {mutation:?}");
        }
    }
    //#endregion 🔖️RepresentabilityLaw
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
