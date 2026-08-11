//! 🧬️ BinaryMutation — document mutation dispatch. Every variant's `diff()`/`inverse()` is
//! handcrafted directly against `BinaryDiff`/`ByteSplice` -- no apply-and-capture.

use crate::artifacts::binary::schema::diff::{diff_set_snapshot, ByteSplice, BinaryDiff};
use crate::artifacts::binary::BinarySnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.binary`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum BinaryMutation {
    #[default]
    NoMutation,
    SetSnapshot { snapshot: BinarySnapshot },
    /// ✂️ Replaces `[offset, offset+remove_len)` with `insert`.
    Splice { offset: usize, remove_len: usize, insert: Vec<u8> },
    /// ➕️ Appends `data` at the end of the buffer.
    AppendBytes { data: Vec<u8> },
    /// ✂️ Drops everything at/after `offset` (a no-op if `offset >= len`).
    TruncateAt { offset: usize },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Diff is the single semantics source.
pub fn apply_binary_mutation(snapshot: &mut BinarySnapshot, mutation: &BinaryMutation) -> BinaryDiff {
    let d = <BinaryMutation as protocol::Mutation<BinarySnapshot>>::diff(mutation, &*snapshot);
    *snapshot = <BinaryDiff as protocol::MutationDiff<BinarySnapshot>>::apply(&d, snapshot);
    d
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<BinarySnapshot> for BinaryMutation {
    type Diff = BinaryDiff;

    fn diff(&self, base: &BinarySnapshot) -> Self::Diff {
        match self {
            BinaryMutation::NoMutation => BinaryDiff::default(),
            BinaryMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            BinaryMutation::Splice { offset, remove_len, insert } => {
                BinaryDiff { splices: vec![ByteSplice { offset: *offset, remove_len: *remove_len, insert: insert.clone() }] }
            }
            BinaryMutation::AppendBytes { data } => {
                BinaryDiff { splices: vec![ByteSplice { offset: base.bytes.len(), remove_len: 0, insert: data.clone() }] }
            }
            BinaryMutation::TruncateAt { offset } => {
                if *offset >= base.bytes.len() {
                    BinaryDiff::default()
                } else {
                    BinaryDiff { splices: vec![ByteSplice { offset: *offset, remove_len: base.bytes.len() - offset, insert: vec![] }] }
                }
            }
        }
    }

    fn inverse(&self, base: &BinarySnapshot) -> Vec<Self> {
        match self {
            BinaryMutation::NoMutation => vec![BinaryMutation::NoMutation],
            BinaryMutation::SetSnapshot { .. } => vec![BinaryMutation::SetSnapshot { snapshot: base.clone() }],
            BinaryMutation::Splice { offset, remove_len, insert } => {
                let start = (*offset).min(base.bytes.len());
                let end = (*offset + *remove_len).min(base.bytes.len());
                let removed_bytes = base.bytes[start..end].to_vec();
                vec![BinaryMutation::Splice { offset: start, remove_len: insert.len(), insert: removed_bytes }]
            }
            BinaryMutation::AppendBytes { .. } => {
                // ↩️ Undo an append by truncating back to the pre-append length.
                vec![BinaryMutation::TruncateAt { offset: base.bytes.len() }]
            }
            BinaryMutation::TruncateAt { offset } => {
                if *offset >= base.bytes.len() {
                    vec![BinaryMutation::NoMutation]
                } else {
                    vec![BinaryMutation::Splice { offset: *offset, remove_len: 0, insert: base.bytes[*offset..].to_vec() }]
                }
            }
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for BinaryMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for BinaryMutation {
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

    fn base() -> BinarySnapshot {
        BinarySnapshot { bytes: vec![1, 2, 3, 4, 5], ..Default::default() }
    }

    fn all_variants(b: &BinarySnapshot) -> Vec<BinaryMutation> {
        vec![
            BinaryMutation::NoMutation,
            BinaryMutation::SetSnapshot { snapshot: BinarySnapshot { bytes: vec![9, 9], ..Default::default() } },
            BinaryMutation::Splice { offset: 1, remove_len: 2, insert: vec![0xAA, 0xBB, 0xCC] },
            BinaryMutation::AppendBytes { data: vec![0xEE, 0xFF] },
            BinaryMutation::TruncateAt { offset: b.bytes.len().saturating_sub(1) },
        ]
    }

    #[test]
    fn mutation_diff_law() {
        let b = base();
        for m in all_variants(&b) {
            let mut via_apply = b.clone();
            let returned = apply_binary_mutation(&mut via_apply, &m);
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
            apply_binary_mutation(&mut mutated, &m);
            for undo in m.inverse(&b) {
                apply_binary_mutation(&mut mutated, &undo);
            }
            assert_eq!(mutated, b, "mutation-level inverse round-trip failed for {m:?}");
        }
        for m in all_variants(&b) {
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
