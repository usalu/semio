//! 🔺️ Note artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::note::schema::diff::{
    NoteAddedBlockEntry, NoteAssetsDelta, NoteBlockPatch, NoteBlockPatchEntry, NoteBlocksDelta, NoteDiff, NoteStringList,
};
use crate::artifacts::note::schema::{block_id, insert_block, remove_block_from_tree, update_block_in_tree};
use crate::artifacts::note::schema::NoteArtifact;
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Apply
impl NoteDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &NoteArtifact) -> NoteArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(id) = &self.id {
            next.id = id.clone();
        }
        if let Some(title) = &self.title {
            next.title = title.clone();
        }
        if let Some(delta) = &self.blocks {
            next.blocks = apply_blocks_delta(&next.blocks, delta);
        }
        if let Some(value) = &self.grid_visible {
            next.grid_visible = *value;
        }
        if let Some(value) = &self.grid_spacing {
            next.grid_spacing = *value;
        }
        if let Some(value) = &self.grid_subdivisions {
            next.grid_subdivisions = *value;
        }
        if let Some(value) = &self.grid_opacity {
            next.grid_opacity = *value;
        }
        if let Some(value) = &self.snap_enabled {
            next.snap_enabled = *value;
        }
        if let Some(value) = &self.snap_grid_spacing {
            next.snap_grid_spacing = *value;
        }
        if let Some(value) = &self.pencil_width {
            next.pencil_width = *value;
        }
        if let Some(value) = &self.eraser_radius {
            next.eraser_radius = *value;
        }
        if let Some(assets) = &self.assets {
            for (key, value) in &assets.entries {
                match value {
                    Some(asset) => {
                        next.assets.insert(key.clone(), asset.clone());
                    }
                    None => {
                        next.assets.remove(key);
                    }
                }
            }
        }
        if let Some(list) = &self.selected_block_ids {
            next.selected_block_ids = list.values.clone();
        }
        if let Some(value) = &self.active_utility_id {
            next.active_utility_id = value.clone();
        }
        if let Some(value) = &self.engagement_input {
            next.engagement_input = value.clone();
        }
        if let Some(value) = self.camera_x {
            next.camera_x = value;
        }
        if let Some(value) = self.camera_y {
            next.camera_y = value;
        }
        if let Some(value) = self.camera_zoom {
            next.camera_zoom = value;
        }
        if let Some(value) = &self.locale {
            next.locale = value.clone();
        }
        if let Some(value) = &self.hovered_block_id {
            next.hovered_block_id = value.clone();
        }
        next
    }
}

/// 🧩 Applies an identified-collection delta to a block tree (adds/removes/patches/reorder) — `added`
/// entries carry their own `parent_id`/`index` so a nested `create-block`/`move-block-to-container`
/// places the node exactly, never a root-only push.
pub fn apply_blocks_delta(blocks: &[NoteBlockNode], delta: &NoteBlocksDelta) -> Vec<NoteBlockNode> {
    let mut next = blocks.to_vec();
    for id in &delta.removed {
        remove_block_from_tree(&mut next, id);
    }
    for entry in &delta.added {
        insert_block(&mut next, entry.parent_id.as_deref(), entry.index.unwrap_or(usize::MAX), entry.block.clone());
    }
    for entry in &delta.patched {
        apply_block_patch_entry(&mut next, entry);
    }
    if let Some(order) = &delta.reordered {
        let mut by_id: std::collections::BTreeMap<_, _> = next
            .into_iter()
            .map(|block| (block_id(&block).to_string(), block))
            .collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(block) = by_id.remove(id) {
                ordered.push(block);
            }
        }
        ordered.extend(by_id.into_values());
        next = ordered;
    }
    next
}

fn apply_block_patch_entry(blocks: &mut Vec<NoteBlockNode>, entry: &NoteBlockPatchEntry) {
    if let Some(block_json) = &entry.patch.block_json {
        if let Ok(replacement) = serde_json::from_str::<NoteBlockNode>(block_json) {
            update_block_in_tree(blocks, &entry.id, replacement);
        }
    }
}

impl MutationDiff<NoteSnapshot> for NoteDiff {
    fn apply(&self, snapshot: &NoteSnapshot) -> NoteSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(id) = &self.id {
            next.id = id.clone();
        }
        if let Some(title) = &self.title {
            next.title = title.clone();
        }
        if let Some(delta) = &self.blocks {
            next.blocks = apply_blocks_delta(&next.blocks, delta);
        }
        if let Some(value) = &self.grid_visible {
            next.grid_visible = *value;
        }
        if let Some(value) = &self.grid_spacing {
            next.grid_spacing = *value;
        }
        if let Some(value) = &self.grid_subdivisions {
            next.grid_subdivisions = *value;
        }
        if let Some(value) = &self.grid_opacity {
            next.grid_opacity = *value;
        }
        if let Some(value) = &self.snap_enabled {
            next.snap_enabled = *value;
        }
        if let Some(value) = &self.snap_grid_spacing {
            next.snap_grid_spacing = *value;
        }
        if let Some(value) = &self.pencil_width {
            next.pencil_width = *value;
        }
        if let Some(value) = &self.eraser_radius {
            next.eraser_radius = *value;
        }
        if let Some(assets) = &self.assets {
            for (key, value) in &assets.entries {
                match value {
                    Some(asset) => {
                        next.assets.insert(key.clone(), asset.clone());
                    }
                    None => {
                        next.assets.remove(key);
                    }
                }
            }
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(id);
        take!(title);
        take!(grid_visible);
        take!(grid_spacing);
        take!(grid_subdivisions);
        take!(grid_opacity);
        take!(snap_enabled);
        take!(snap_grid_spacing);
        take!(pencil_width);
        take!(eraser_radius);
        take!(selected_block_ids);
        take!(active_utility_id);
        take!(engagement_input);
        take!(camera_x);
        take!(camera_y);
        take!(camera_zoom);
        take!(locale);
        take!(hovered_block_id);
        match (&mut self.blocks, other.blocks) {
            (Some(dst), Some(src)) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            (None, Some(src)) => self.blocks = Some(src),
            _ => {}
        }
        match (&mut self.assets, other.assets) {
            (Some(dst), Some(src)) => {
                dst.entries.extend(src.entries);
            }
            (None, Some(src)) => self.assets = Some(src),
            _ => {}
        }
    }
}
//#endregion 🔖️Apply

//#region 🔖️Builders
/// 🩹 Sparse single-block whole-value patch — shared by every `change-block-*`/`rename-block`/
/// `move-block`/`resize-block`/`edit-block-*`/table-row-column mutation leaf: each computes the
/// updated `NoteBlockNode` value from `(payload, base)` and hands it here.
pub fn note_block_patch_diff(id: &str, block: NoteBlockNode) -> NoteDiff {
    NoteDiff {
        blocks: Some(NoteBlocksDelta { patched: vec![NoteBlockPatchEntry { id: id.to_string(), patch: NoteBlockPatch { block_json: Some(serde_json::to_string(&block).expect("NoteBlockNode is always json-serializable")) } }], ..Default::default() }),
        ..Default::default()
    }
}

/// ➕ Sparse single-block insertion at `(parent_id, index)` — shared by `create-block`,
/// `duplicate-block(s)`, and the added-half of `move-block-to-container`.
pub fn note_block_added_diff(parent_id: Option<String>, index: Option<usize>, block: NoteBlockNode) -> NoteDiff {
    NoteDiff {
        blocks: Some(NoteBlocksDelta { added: vec![NoteAddedBlockEntry { parent_id, index, block }], ..Default::default() }),
        ..Default::default()
    }
}

/// 🗑️ Sparse single/multi-id removal — shared by `delete-block(s)` and the removed-half of
/// `move-block-to-container`.
pub fn note_block_removed_diff(ids: Vec<String>) -> NoteDiff {
    NoteDiff {
        blocks: Some(NoteBlocksDelta { removed: ids, ..Default::default() }),
        ..Default::default()
    }
}

/// 🖼️ Sparse single-key asset upsert — shared by `create-asset`/`replace-asset-payload`.
pub fn note_asset_upsert_diff(key: &str, asset: &crate::artifacts::note::NoteImageAsset) -> NoteDiff {
    let mut entries = std::collections::BTreeMap::new();
    entries.insert(key.to_string(), Some(asset.clone()));
    NoteDiff { assets: Some(NoteAssetsDelta { entries }), ..Default::default() }
}

/// 🗑️ Sparse single-key asset removal — shared by `delete-asset`.
pub fn note_asset_removed_diff(key: &str) -> NoteDiff {
    let mut entries = std::collections::BTreeMap::new();
    entries.insert(key.to_string(), None);
    NoteDiff { assets: Some(NoteAssetsDelta { entries }), ..Default::default() }
}
//#endregion 🔖️Builders

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
