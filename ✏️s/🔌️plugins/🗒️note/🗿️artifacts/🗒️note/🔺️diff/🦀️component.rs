//! 🔺️ Note artifact — the operation diff (constitutional: diff).

use crate::artifacts::note::op::{apply_note_operation, NoteOperation};
use crate::artifacts::note::NoteDocument;
use protocol::OperationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🧩️ Snapshot diff wrapping the forward `NoteOperation` — `apply` replays it, `absorb` keeps the latest
/// (coalescing a whole gesture's `SetBlocks` stream into one edit).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslDiff)]
pub struct NoteDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(statements)]
    pub operation: Option<NoteOperation>,
}

impl OperationDiff<NoteDocument> for NoteDiff {
    fn apply(&self, projection: &NoteDocument) -> NoteDocument {
        match &self.operation {
            Some(operation) => apply_note_operation(projection, operation),
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
            NoteDiff { operation: Some(NoteOperation::SetGridSpacing { spacing: Some(48.0) }) },
            NoteDiff { operation: Some(NoteOperation::SetDocument { document: crate::artifacts::note::engine::empty_note_document() }) },
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
        let diffs = vec![NoteDiff { operation: Some(NoteOperation::SetGridSpacing { spacing: Some(48.0) }) }, NoteDiff::default()];
        for diff in diffs {
            let bytes = diff.encode_diff().expect("encode_diff");
            let decoded = NoteDiff::decode_diff(&bytes).expect("decode_diff");
            assert_eq!(decoded, diff, "DiffCodec binary round trip diverged");
        }
    }
}
//#endregion 🧪️Tests
