//! 🔺️ Note artifact — the operation diff (constitutional: diff).

use crate::artifacts::note::op::{apply_note_mutation, NoteMutation};
use crate::artifacts::note::NoteDocument;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


//#region 🔖️Diff
/// 🧩️ Snapshot diff wrapping the forward `NoteMutation` — `apply` replays it, `absorb` keeps the latest
/// (coalescing a whole gesture's `SetBlocks` stream into one edit).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslDiff)]
pub struct NoteDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(statements)]
    pub operation: Option<NoteMutation>,
}

impl MutationDiff<NoteDocument> for NoteDiff {
    fn apply(&self, projection: &NoteDocument) -> NoteDocument {
        match &self.operation {
            Some(operation) => apply_note_mutation(projection, operation),
            None => projection.clone(),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.operation.is_some() {
            self.operation = other.operation;
        }
    }
}
//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧬️ W1 `DiffCodec` law: `parse_diff(print_diff(d)) == d` and `decode_diff(encode_diff(d)) == d`
    /// — `NoteDiff` is one of the handful of real diff types proving the `#[derive(dsl::DslDiff)]`
    /// mechanism (see `POLICY_DIFF_COMPLETENESS_ALLOWLIST` in `script.ts` for the rest, deferred to W6).
    #[test]
    fn note_diff_print_parse_round_trips() {
        use protocol::DiffCodec;
        let diffs = vec![
            NoteDiff { operation: Some(NoteMutation::SetGridSpacing { spacing: Some(48.0) }) },
            NoteDiff { operation: Some(NoteMutation::SetDocument { document: crate::artifacts::note::engine::empty_note_document() }) },
            NoteDiff::default(),
        ];
        for diff in diffs {
            let printed = diff.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line: {printed:?}");
            let parsed = NoteDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff failed for {printed:?}: {e}"));
            assert_eq!(parsed, diff, "DiffCodec text round trip diverged for {printed:?}");
        }
    }

    #[test]
    fn note_diff_encode_decode_round_trips() {
        use protocol::DiffCodec;
        let diffs = vec![NoteDiff { operation: Some(NoteMutation::SetGridSpacing { spacing: Some(48.0) }) }, NoteDiff::default()];
        for diff in diffs {
            let bytes = diff.encode_diff().expect("encode_diff");
            let decoded = NoteDiff::decode_diff(&bytes).expect("decode_diff");
            assert_eq!(decoded, diff, "DiffCodec binary round trip diverged");
        }
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semio_grammar_conformance {
    use super::*;

    #[test]
    fn component_grammar_semio_is_grammar_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_GRAMMAR_SEMIO).expect("parse grammar.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Grammar);
        assert!(!COMPONENT_GRAMMAR_SEMIO.is_empty());
        let _ = COMPONENT_GRAMMAR_PATH;
    }
}

