//! 🧬️ Note diff schema — sparse field delta over the artifact, its apply/absorb algebra, and its
//! sparse-diff builder helpers. `Apply`/`Builders` regions relocated here (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM) from the old `🔺️diff/📝️text/🦀️.rs`:
//! despite living in a `📝️text` directory, that content was never a text CODEC (`NoteDiff` has no
//! `impl store::ArtifactDsl`/`ArtifactPack` anywhere in this plugin) — it was pure snapshot-algebra
//! transform logic, which design.md rule 2 requires stay in `🧬️schema`. Only the genuine
//! `📖️.grammar.semio` spec-asset declaration (used by `io()`'s `note.diff` `LanguageSpec`
//! registration, for LSP/verification tooling — a grammar can be registered and validated without a
//! literal parser impl backing it at runtime) stayed at `🚪️io/📝️text/🔺️diff/`.

use crate::schema::{block_id, find_block, flatten_blocks, insert_block, remove_block_from_tree, update_block_in_tree};
use crate::{NoteBlockNode, NoteImageAsset, NoteSnapshot};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the note artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.note.note")]
pub struct NoteDiff {
    #[state(artifact)]
    pub artifact: Option<Box<NoteArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub id: Option<String>,
    #[state(artifact)]
    pub title: Option<Option<String>>,
    #[state(artifact)]
    pub blocks: Option<NoteBlocksDelta>,
    #[state(artifact)]
    pub grid_visible: Option<Option<bool>>,
    #[state(artifact)]
    pub grid_spacing: Option<Option<f64>>,
    #[state(artifact)]
    pub grid_subdivisions: Option<Option<f64>>,
    #[state(artifact)]
    pub grid_opacity: Option<Option<f64>>,
    #[state(artifact)]
    pub snap_enabled: Option<Option<bool>>,
    #[state(artifact)]
    pub snap_grid_spacing: Option<Option<f64>>,
    #[state(artifact)]
    pub pencil_width: Option<Option<f64>>,
    #[state(artifact)]
    pub eraser_radius: Option<Option<f64>>,
    #[state(artifact)]
    pub assets: Option<NoteAssetsDelta>,
    /// 🔗️ Same double-`Option` shape as every optional-slot field in this ticket's plugins, for the
    /// `R:any` forward link slot — schema/codec-complete, currently unset by any mutation (see the
    /// snapshot field's own doc comment).
    #[state(artifact)]
    pub linked_artifact: Option<Option<store::ArtifactLink>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🗂️ Asset-map wrapper so optional map diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", default)]
#[value(rename_all = "camelCase", default)]
pub struct NoteAssetsDelta {
    pub entries: BTreeMap<String, Option<NoteImageAsset>>,
}

/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", default)]
#[value(rename_all = "camelCase", default)]
pub struct NoteStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `blocks`.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct NoteBlocksDelta {
    pub added: Vec<NoteAddedBlockEntry>,
    pub removed: Vec<String>,
    pub patched: Vec<NoteBlockPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// ➕ One added/reparented block: `parent_id` (`None` = document root) and `index` (`None` =
/// append) place it — `create-block`/`duplicate-block(s)`/`move-block-to-container` all diff
/// through this, never a whole-`blocks` vec swap. No `Default` derive: `NoteBlockNode` (a tagged
/// enum) has no sensible default value, and every construction site fills all three fields anyway.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct NoteAddedBlockEntry {
    pub parent_id: Option<String>,
    pub index: Option<usize>,
    pub block: NoteBlockNode,
}

/// 🩹 One patched block entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct NoteBlockPatchEntry {
    pub id: String,
    pub patch: NoteBlockPatch,
}

/// 🩹 Sparse block field patch (JSON blob for whole-block replacement).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", default)]
#[value(rename_all = "camelCase", default)]
pub struct NoteBlockPatch {
    pub block_json: Option<String>,
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️Apply
impl NoteDiff {
    /// 🧬️ Applies sparse document changes to the artifact.
    pub fn apply_to_artifact(&self, artifact: &NoteArtifact) -> protocol::MutationApplyResult<NoteArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
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
                next.blocks = apply_blocks_delta(&next.blocks, delta).map_err(|error| error.under(["blocks"]))?;
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
                apply_assets_delta(&mut next.assets, assets).map_err(|error| error.under(["assets"]))?;
            }
            if let Some(value) = &self.linked_artifact {
                next.linked_artifact = value.clone();
            }
            next
        })
    }
}



fn apply_assets_delta(assets: &mut BTreeMap<String, NoteImageAsset>, delta: &NoteAssetsDelta) -> protocol::MutationApplyResult<()> {
    for (key, value) in &delta.entries {
        if value.is_none() && !assets.contains_key(key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed asset does not exist").at([key.as_str()]));
        }
    }
    let mut candidate = assets.clone();
    for (key, value) in &delta.entries {
        match value {
            Some(asset) => {
                candidate.insert(key.clone(), asset.clone());
            }
            None => {
                candidate.remove(key);
            }
        }
    }
    *assets = candidate;
    Ok(())
}

impl MutationDiff<NoteSnapshot> for NoteDiff {
    fn apply(&self, snapshot: &NoteSnapshot) -> protocol::MutationApplyResult<NoteSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
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
                next.blocks = apply_blocks_delta(&next.blocks, delta).map_err(|error| error.under(["blocks"]))?;
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
                apply_assets_delta(&mut next.assets, assets).map_err(|error| error.under(["assets"]))?;
            }
            if let Some(value) = &self.linked_artifact {
                next.linked_artifact = value.clone();
            }
            next
        })
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
        take!(linked_artifact);
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


/// ➕ Sparse single-block insertion at `(parent_id, index)` — shared by `create-block`,
/// `duplicate-block(s)`, and the added-half of `move-block-to-container`.
pub fn note_block_added_diff(parent_id: Option<String>, index: Option<usize>, block: NoteBlockNode) -> NoteDiff {
    NoteDiff { blocks: Some(NoteBlocksDelta { added: vec![NoteAddedBlockEntry { parent_id, index, block }], ..Default::default() }), ..Default::default() }
}

/// 🗑️ Sparse single/multi-id removal — shared by `delete-block(s)` and the removed-half of
/// `move-block-to-container`.
pub fn note_block_removed_diff(ids: Vec<String>) -> NoteDiff {
    NoteDiff { blocks: Some(NoteBlocksDelta { removed: ids, ..Default::default() }), ..Default::default() }
}

/// 🖼️ Sparse single-key asset upsert — shared by `create-asset`/`replace-asset-payload`.
pub fn note_asset_upsert_diff(key: &str, asset: &NoteImageAsset) -> NoteDiff {
    let mut entries = BTreeMap::new();
    entries.insert(key.to_string(), Some(asset.clone()));
    NoteDiff { assets: Some(NoteAssetsDelta { entries }), ..Default::default() }
}

/// 🗑️ Sparse single-key asset removal — shared by `delete-asset`.
pub fn note_asset_removed_diff(key: &str) -> NoteDiff {
    let mut entries = BTreeMap::new();
    entries.insert(key.to_string(), None);
    NoteDiff { assets: Some(NoteAssetsDelta { entries }), ..Default::default() }
}
//#endregion 🔖️Builders

#[cfg(test)]
#[path = "🧪️tests/🔬️diff-apply/🦀️.rs"]
mod diff_apply_tests;

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::schema::NoteArtifact;
//#endregion 🔁️Re-exports
