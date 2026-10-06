//! 🔺️ Note diff — grammar spec asset only. `NoteDiff` has no `impl store::ArtifactDsl`/`ArtifactPack`
//! anywhere in this plugin — this facet exists purely to register `note.diff`'s handcrafted text
//! grammar for LSP/verification tooling via `io()`'s `LanguageSpec` (design.md §2's `LanguagePair`
//! doc: "a subset with no hand-authored grammar for a channel still owns that channel's codec, just
//! with no `.grammar.semio` registered" — the inverse case is equally legal: a registered grammar
//! with no literal runtime parser backing it). The real apply/absorb/builder logic that used to live
//! here moved to `🧬️schema/🔺️diff/🦀️.rs` (pure snapshot-algebra transforms, design.md rule 2).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
mod semio_grammar_conformance;

#[allow(unused_imports)]
mod diff_codec {
use crate::standards::v1::subsets::any::schema::diff::*;
use crate::schema::{block_id, find_block, flatten_blocks, insert_block, remove_block_from_tree, update_block_in_tree};
use crate::{NoteBlockNode, NoteImageAsset, NoteSnapshot};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::NoteArtifact;

/// 🩹 Sparse single-block whole-value patch — shared by every `change-block-*`/`rename-block`/
/// `move-block`/`resize-block`/`edit-block-*`/table-row-column mutation leaf: each computes the
/// updated `NoteBlockNode` value from `(payload, base)` and hands it here.
pub fn note_block_patch_diff(id: &str, block: &NoteBlockNode) -> NoteDiff {
    NoteDiff { blocks: Some(NoteBlocksDelta { patched: vec![NoteBlockPatchEntry { id: id.to_string(), patch: NoteBlockPatch { block_json: Some(semio_framework_pack_json::to_json_string(block)) } }], ..Default::default() }), ..Default::default() }
}
}
pub use diff_codec::*;

#[allow(unused_imports)]
mod diff_wire_codec {
use crate::standards::v1::subsets::any::schema::diff::*;
use crate::schema::{block_id, find_block, flatten_blocks, insert_block, remove_block_from_tree, update_block_in_tree};
use crate::{NoteBlockNode, NoteImageAsset, NoteSnapshot};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::NoteArtifact;

/// 🧩 Applies an identified-collection delta to a block tree (adds/removes/patches/reorder) — `added`
/// entries carry their own `parent_id`/`index` so a nested `create-block`/`move-block-to-container`
/// places the node exactly, never a root-only push.
pub fn apply_blocks_delta(blocks: &[NoteBlockNode], delta: &NoteBlocksDelta) -> protocol::MutationApplyResult<Vec<NoteBlockNode>> {
    let base_ids: Vec<String> = flatten_blocks(blocks).into_iter().map(|block| block_id(block).to_string()).collect();
    if base_ids.iter().enumerate().any(|(index, id)| base_ids[..index].contains(id)) {
        return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "base block tree contains duplicate identities").at(["base"]));
    }
    for (index, id) in delta.removed.iter().enumerate() {
        if !base_ids.contains(id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed block does not exist").at(["removed".to_string(), index.to_string()]));
        }
        if delta.removed[..index].contains(id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "block is removed more than once").at(["removed".to_string(), index.to_string()]));
        }
    }
    let replacements: Vec<Option<NoteBlockNode>> = delta
        .patched
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            if !base_ids.contains(&entry.id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "patched block does not exist").at(["patched".to_string(), index.to_string()]));
            }
            if delta.removed.contains(&entry.id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.conflicting-target", "block cannot be removed and patched").at(["patched".to_string(), index.to_string()]));
            }
            if delta.patched[..index].iter().any(|prior| prior.id == entry.id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "block is patched more than once").at(["patched".to_string(), index.to_string()]));
            }
            entry
                .patch
                .block_json
                .as_ref()
                .map(|json| {
                    semio_framework_pack_json::from_json_str::<NoteBlockNode>(json, semio_framework_pack_json::JsonMemberPolicy::Reject)
                        .map_err(|error| protocol::MutationApplyError::new("mutation.apply.invalid-value", format!("block patch is not valid JSON: {error}")).at(["patched".to_string(), index.to_string(), "blockJson".to_string()]))
                })
                .transpose()
        })
        .collect::<protocol::MutationApplyResult<_>>()?;
    let mut next = blocks.to_vec();
    for id in &delta.removed {
        remove_block_from_tree(&mut next, id);
    }
    for (position, entry) in delta.added.iter().enumerate() {
        let added_ids: Vec<String> = flatten_blocks(std::slice::from_ref(&entry.block)).into_iter().map(|block| block_id(block).to_string()).collect();
        if added_ids.iter().enumerate().any(|(index, id)| added_ids[..index].contains(id) || find_block(&next, id).is_some()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "added block tree contains an existing or duplicate identity").at(["added".to_string(), position.to_string()]));
        }
        let container_len = match entry.parent_id.as_deref() {
            None => next.len(),
            Some(parent_id) => match find_block(&next, parent_id) {
                Some(NoteBlockNode::Group { children, .. }) => children.len(),
                Some(_) => {
                    return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target", "added block parent is not a group").at(["added".to_string(), position.to_string(), "parentId".to_string()]));
                }
                None => {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "added block parent does not exist").at(["added".to_string(), position.to_string(), "parentId".to_string()]));
                }
            },
        };
        let index = entry.index.unwrap_or(container_len);
        if index > container_len {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index", format!("block insertion index {index} exceeds length {container_len}")).at(["added".to_string(), position.to_string(), "index".to_string()]));
        }
        insert_block(&mut next, entry.parent_id.as_deref(), index, entry.block.clone());
    }
    for (entry, replacement) in delta.patched.iter().zip(replacements) {
        if let Some(replacement) = replacement {
            if block_id(&replacement) != entry.id {
                return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target", "block patch cannot change the target identity").at(["patched".to_string(), entry.id.clone()]));
            }
            if !update_block_in_tree(&mut next, &entry.id, replacement) {
                return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "patched block does not exist after structural edits").at(["patched".to_string(), entry.id.clone()]));
            }
        }
    }
    if let Some(order) = &delta.reordered {
        if order.len() != next.len() || order.iter().enumerate().any(|(index, id)| order[..index].contains(id) || !next.iter().any(|block| block_id(block) == id)) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-order", "root block reorder must be a complete unique permutation").at(["reordered"]));
        }
        let mut by_id: BTreeMap<_, _> = next.into_iter().map(|block| (block_id(&block).to_string(), block)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            ordered.push(by_id.remove(id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "reordered root block does not exist").at(["reordered".to_string(), id.clone()]))?);
        }
        next = ordered;
    }
    let next_ids: Vec<_> = flatten_blocks(&next).into_iter().map(block_id).collect();
    if next_ids.iter().enumerate().any(|(index, id)| next_ids[..index].contains(id)) {
        return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "resulting block tree contains duplicate identities").at(["identities"]));
    }
    Ok(next)
}
}
pub use diff_wire_codec::*;
