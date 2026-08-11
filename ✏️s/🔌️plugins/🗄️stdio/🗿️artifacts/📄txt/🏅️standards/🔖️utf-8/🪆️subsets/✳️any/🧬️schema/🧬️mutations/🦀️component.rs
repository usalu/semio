//! 🧬️ TxtMutation — document mutation dispatch. Every variant's `diff()`/`inverse()` is
//! handcrafted directly against `TxtDiff`/`TxtLinesDiff` -- no apply-and-capture.

use crate::artifacts::txt::schema::diff::{diff_set_snapshot, TxtDiff, TxtLineAdded, TxtLineModified, TxtLinesDiff};
use crate::artifacts::txt::schema::snapshot::LineEnding;
use crate::artifacts::txt::TxtSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.txt`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum TxtMutation {
    #[default]
    NoMutation,
    SetSnapshot { snapshot: TxtSnapshot },
    SetTrailingNewline { value: bool },
    SetLineEnding { value: LineEnding },
    InsertLine { index: usize, text: String },
    RemoveLine { index: usize },
    SetLine { index: usize, text: String },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Diff is the single semantics source: computed once,
/// applied once, returned once.
pub fn apply_txt_mutation(snapshot: &mut TxtSnapshot, mutation: &TxtMutation) -> TxtDiff {
    let d = <TxtMutation as protocol::Mutation<TxtSnapshot>>::diff(mutation, &*snapshot);
    *snapshot = <TxtDiff as protocol::MutationDiff<TxtSnapshot>>::apply(&d, snapshot);
    d
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<TxtSnapshot> for TxtMutation {
    type Diff = TxtDiff;

    fn diff(&self, base: &TxtSnapshot) -> Self::Diff {
        match self {
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
            TxtMutation::InsertLine { index, text } => TxtDiff {
                lines: Some(TxtLinesDiff { removed: vec![], modified: vec![], added: vec![TxtLineAdded { index: *index, text: text.clone() }] }),
                ..Default::default()
            },
            TxtMutation::RemoveLine { index } => {
                if *index >= base.lines.len() {
                    TxtDiff::default()
                } else {
                    TxtDiff {
                        lines: Some(TxtLinesDiff { removed: vec![*index], modified: vec![], added: vec![] }),
                        ..Default::default()
                    }
                }
            }
            TxtMutation::SetLine { index, text } => {
                if base.lines.get(*index).map_or(true, |cur| cur == text) {
                    TxtDiff::default()
                } else {
                    TxtDiff {
                        lines: Some(TxtLinesDiff { removed: vec![], modified: vec![TxtLineModified { index: *index, text: text.clone() }], added: vec![] }),
                        ..Default::default()
                    }
                }
            }
        }
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
impl protocol::OpText for TxtMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for TxtMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op encode",
            offset: 0,
            detail: e.to_string(),
        })
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op decode",
            offset: 0,
            detail: e.to_string(),
        })
    }
}
//#endregion OpCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{DiffAlgebra, MutationDiff};

    fn base() -> TxtSnapshot {
        TxtSnapshot {
            lines: vec!["a".into(), "b".into(), "c".into()],
            trailing_newline: true,
            line_ending: LineEnding::Lf,
            ..Default::default()
        }
    }

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

    #[test]
    fn mutation_diff_law() {
        let b = base();
        for m in all_variants(&b) {
            let mut via_apply = b.clone();
            let returned = apply_txt_mutation(&mut via_apply, &m);
            let expected_diff = m.diff(&b);
            assert_eq!(returned, expected_diff, "returned diff mismatch for {m:?}");
            assert_eq!(via_apply, expected_diff.apply(&b), "apply mismatch for {m:?}");
        }
    }

    #[test]
    fn inverse_law() {
        let b = base();
        for m in all_variants(&b) {
            let mut mutated = b.clone();
            apply_txt_mutation(&mut mutated, &m);
            for undo in m.inverse(&b) {
                apply_txt_mutation(&mut mutated, &undo);
            }
            assert_eq!(mutated, b, "mutation-level inverse round-trip failed for {m:?}");

            let d = m.diff(&b);
            let next = d.apply(&b);
            let inv = d.inverse(&b);
            assert_eq!(inv.apply(&next), b, "diff-level inverse round-trip failed for {m:?}");
        }
    }

    #[test]
    fn absorb_law_cartesian() {
        let b = base();
        let variants = all_variants(&b);
        for m1 in &variants {
            let d1 = m1.diff(&b);
            let mid = d1.apply(&b);
            for m2 in &variants {
                let d2 = m2.diff(&mid);
                let after = d2.apply(&mid);
                let mut merged = d1.clone();
                merged.absorb(d2.clone());
                assert_eq!(merged.apply(&b), after, "absorb({m1:?}, {m2:?}) mismatch");
            }
        }
    }
}
//#endregion 🧪️Tests
