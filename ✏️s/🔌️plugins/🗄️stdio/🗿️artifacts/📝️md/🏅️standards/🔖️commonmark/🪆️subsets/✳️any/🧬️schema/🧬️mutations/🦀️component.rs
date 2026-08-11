//! 🧬️ MdMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) and every variant's `inverse()` is handcrafted, path/index-aware.

use crate::artifacts::md::schema::diff::{diff_at_path, diff_set_snapshot, MdBlockDiff, MdBlocksLeafDiff, MdDiff};
pub use crate::artifacts::md::schema::diff::MdPathStep;
use crate::artifacts::md::schema::diff::navigate_container;
use crate::artifacts::md::schema::snapshot::{MdBlock, MdInline};
use crate::artifacts::md::MdSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.md`. Every `path`-carrying variant addresses the
/// CONTAINER (the `Vec<MdBlock>` -- top level, a block-quote's `blocks`, or a list item's
/// content) the mutation's `index` lives in; `path == []` addresses the top-level `blocks`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum MdMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: MdSnapshot,
    },
    /// ➕️ Inserts `block` at `index` within the container addressed by `path`.
    InsertBlock {
        path: Vec<MdPathStep>,
        index: usize,
        block: MdBlock,
    },
    /// ➖️ Removes the block at `index` within the container addressed by `path`.
    RemoveBlock {
        path: Vec<MdPathStep>,
        index: usize,
    },
    /// 🔁 Wholesale-replaces the block at `index` (documented "your call" per the brief: a
    /// generic full-block replace instead of a per-field mutation for every one of `MdBlock`'s 7
    /// variants -- `SetInlines` below covers the one field (`inlines`) that's actually common and
    /// worth its own targeted mutation; every other field-level edit goes through `ReplaceBlock`).
    ReplaceBlock {
        path: Vec<MdPathStep>,
        index: usize,
        block: MdBlock,
    },
    /// ✏️ Whole-value replaces the `inlines` of the `Heading`/`Paragraph` block at `index`
    /// (`MdInline` is a weak entity -- recipe: whole-value replaced, never sub-diffed). A
    /// graceful no-op (empty diff, `NoMutation`-shaped inverse) if the addressed block isn't one
    /// of those two kinds -- documented degrade-gracefully behavior, never a panic.
    SetInlines {
        path: Vec<MdPathStep>,
        index: usize,
        inlines: Vec<MdInline>,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` -- the diff is the single semantics source, never a separate imperative
/// apply path.
pub fn apply_md_mutation(snapshot: &mut MdSnapshot, mutation: &MdMutation) -> MdDiff {
    let diff = Mutation::diff(mutation, snapshot);
    *snapshot = protocol::MutationDiff::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<MdSnapshot> for MdMutation {
    type Diff = MdDiff;

    fn diff(&self, base: &MdSnapshot) -> Self::Diff {
        match self {
            MdMutation::NoMutation => MdDiff::default(),
            MdMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            MdMutation::InsertBlock { path, index, block } => diff_at_path(path, *index, MdBlocksLeafDiff::Added(block.clone())),
            MdMutation::RemoveBlock { path, index } => diff_at_path(path, *index, MdBlocksLeafDiff::Removed),
            MdMutation::ReplaceBlock { path, index, block } => {
                diff_at_path(path, *index, MdBlocksLeafDiff::Modified(MdBlockDiff::Replace { block: block.clone() }))
            }
            MdMutation::SetInlines { path, index, inlines } => {
                match navigate_container(&base.blocks, path).and_then(|c| c.get(*index)) {
                    Some(MdBlock::Heading { .. }) => diff_at_path(
                        path,
                        *index,
                        MdBlocksLeafDiff::Modified(MdBlockDiff::Heading { level: None, inlines: Some(inlines.clone()) }),
                    ),
                    Some(MdBlock::Paragraph { .. }) => diff_at_path(
                        path,
                        *index,
                        MdBlocksLeafDiff::Modified(MdBlockDiff::Paragraph { inlines: Some(inlines.clone()) }),
                    ),
                    _ => MdDiff::default(),
                }
            }
        }
    }

    fn inverse(&self, base: &MdSnapshot) -> Vec<Self> {
        match self {
            MdMutation::NoMutation => vec![MdMutation::NoMutation],
            MdMutation::SetSnapshot { .. } => vec![MdMutation::SetSnapshot { snapshot: base.clone() }],
            MdMutation::InsertBlock { path, index, .. } => vec![MdMutation::RemoveBlock { path: path.clone(), index: *index }],
            MdMutation::RemoveBlock { path, index } => {
                match navigate_container(&base.blocks, path).and_then(|c| c.get(*index)).cloned() {
                    Some(block) => vec![MdMutation::InsertBlock { path: path.clone(), index: *index, block }],
                    None => vec![MdMutation::NoMutation],
                }
            }
            MdMutation::ReplaceBlock { path, index, .. } => {
                match navigate_container(&base.blocks, path).and_then(|c| c.get(*index)).cloned() {
                    Some(block) => vec![MdMutation::ReplaceBlock { path: path.clone(), index: *index, block }],
                    None => vec![MdMutation::NoMutation],
                }
            }
            MdMutation::SetInlines { path, index, .. } => {
                let original = match navigate_container(&base.blocks, path).and_then(|c| c.get(*index)) {
                    Some(MdBlock::Heading { inlines, .. }) => Some(inlines.clone()),
                    Some(MdBlock::Paragraph { inlines }) => Some(inlines.clone()),
                    _ => None,
                };
                match original {
                    Some(inlines) => vec![MdMutation::SetInlines { path: path.clone(), index: *index, inlines }],
                    None => vec![MdMutation::NoMutation],
                }
            }
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for MdMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for MdMutation {
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
