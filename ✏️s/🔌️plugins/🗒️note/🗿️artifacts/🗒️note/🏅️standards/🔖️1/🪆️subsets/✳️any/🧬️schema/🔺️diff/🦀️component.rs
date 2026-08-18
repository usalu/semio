//! 🧬️ Note diff schema — sparse field delta over the artifact, its apply/absorb algebra, and its
//! sparse-diff builder helpers. `Apply`/`Builders` regions relocated here (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM) from the old `🔺️diff/📝️text/🦀️component.rs`:
//! despite living in a `📝️text` directory, that content was never a text CODEC (`NoteDiff` has no
//! `impl store::ArtifactDsl`/`ArtifactPack` anywhere in this plugin) — it was pure snapshot-algebra
//! transform logic, which design.md rule 2 requires stay in `🧬️schema`. Only the genuine
//! `📖️component.grammar.semio` spec-asset declaration (used by `io()`'s `note.diff` `LanguageSpec`
//! registration, for LSP/verification tooling — a grammar can be registered and validated without a
//! literal parser impl backing it at runtime) stayed at `🚪️io/🔺️diff/📝️text/`.

use crate::artifacts::note::schema::{block_id, find_block, flatten_blocks, insert_block, remove_block_from_tree, update_block_in_tree};
use crate::artifacts::note::schema::NoteArtifact;
use crate::artifacts::note::{NoteBlockNode, NoteImageAsset, NoteSnapshot};
use protocol::MutationDiff;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the note artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.note.note")]
pub struct NoteDiff {
    #[state(artifact)] pub artifact: Option<Box<crate::artifacts::note::schema::NoteArtifact>>,
    #[state(artifact)] pub schema: Option<String>,
    #[state(artifact)] pub id: Option<String>,
    #[state(artifact)] pub title: Option<Option<String>>,
    #[state(artifact)] pub blocks: Option<NoteBlocksDelta>,
    #[state(artifact)] pub grid_visible: Option<Option<bool>>,
    #[state(artifact)] pub grid_spacing: Option<Option<f64>>,
    #[state(artifact)] pub grid_subdivisions: Option<Option<f64>>,
    #[state(artifact)] pub grid_opacity: Option<Option<f64>>,
    #[state(artifact)] pub snap_enabled: Option<Option<bool>>,
    #[state(artifact)] pub snap_grid_spacing: Option<Option<f64>>,
    #[state(artifact)] pub pencil_width: Option<Option<f64>>,
    #[state(artifact)] pub eraser_radius: Option<Option<f64>>,
    #[state(artifact)] pub assets: Option<NoteAssetsDelta>,
    /// 🔗️ Same double-`Option` shape as every optional-slot field in this ticket's plugins, for the
    /// `R:any` forward link slot — schema/codec-complete, currently unset by any mutation (see the
    /// snapshot field's own doc comment).
    #[state(artifact)] pub linked_artifact: Option<Option<store::ArtifactLink>>,
    #[state(presence)] pub selected_block_ids: Option<NoteStringList>,
    #[state(presence)] pub active_utility_id: Option<String>,
    #[state(config)] pub engagement_input: Option<String>,
    #[state(config)] pub camera_x: Option<f64>,
    #[state(config)] pub camera_y: Option<f64>,
    #[state(config)] pub camera_zoom: Option<f64>,
    #[state(config)] pub locale: Option<String>,
    #[state(artifact)] pub hovered_block_id: Option<Option<String>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🗂️ Asset-map wrapper so optional map diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NoteAssetsDelta {
    pub entries: BTreeMap<String, Option<NoteImageAsset>>,
}

/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NoteStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `blocks`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteAddedBlockEntry {
    pub parent_id: Option<String>,
    pub index: Option<usize>,
    pub block: NoteBlockNode,
}

/// 🩹 One patched block entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteBlockPatchEntry {
    pub id: String,
    pub patch: NoteBlockPatch,
}

/// 🩹 Sparse block field patch (JSON blob for whole-block replacement).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NoteBlockPatch {
    pub block_json: Option<String>,
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️Apply
impl NoteDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
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
                next.blocks = apply_blocks_delta(&next.blocks, delta)
                    .map_err(|error| error.under(["blocks"]))?;
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
                apply_assets_delta(&mut next.assets, assets)
                    .map_err(|error| error.under(["assets"]))?;
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
            if let Some(value) = &self.linked_artifact {
                next.linked_artifact = value.clone();
            }
            next
        })
    }
}

/// 🧩 Applies an identified-collection delta to a block tree (adds/removes/patches/reorder) — `added`
/// entries carry their own `parent_id`/`index` so a nested `create-block`/`move-block-to-container`
/// places the node exactly, never a root-only push.
pub fn apply_blocks_delta(
    blocks: &[NoteBlockNode],
    delta: &NoteBlocksDelta,
) -> protocol::MutationApplyResult<Vec<NoteBlockNode>> {
    let base_ids: Vec<String> = flatten_blocks(blocks)
        .into_iter()
        .map(|block| block_id(block).to_string())
        .collect();
    if base_ids
        .iter()
        .enumerate()
        .any(|(index, id)| base_ids[..index].contains(id))
    {
        return Err(protocol::MutationApplyError::new(
            "mutation.apply.duplicate-target",
            "base block tree contains duplicate identities",
        )
        .at(["base"]));
    }
    for (index, id) in delta.removed.iter().enumerate() {
        if !base_ids.contains(id) {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.missing-target",
                "removed block does not exist",
            )
            .at(["removed".to_string(), index.to_string()]));
        }
        if delta.removed[..index].contains(id) {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.duplicate-target",
                "block is removed more than once",
            )
            .at(["removed".to_string(), index.to_string()]));
        }
    }
    let replacements: Vec<Option<NoteBlockNode>> = delta
        .patched
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            if !base_ids.contains(&entry.id) {
                return Err(protocol::MutationApplyError::new(
                    "mutation.apply.missing-target",
                    "patched block does not exist",
                )
                .at(["patched".to_string(), index.to_string()]));
            }
            if delta.removed.contains(&entry.id) {
                return Err(protocol::MutationApplyError::new(
                    "mutation.apply.conflicting-target",
                    "block cannot be removed and patched",
                )
                .at(["patched".to_string(), index.to_string()]));
            }
            if delta.patched[..index]
                .iter()
                .any(|prior| prior.id == entry.id)
            {
                return Err(protocol::MutationApplyError::new(
                    "mutation.apply.duplicate-target",
                    "block is patched more than once",
                )
                .at(["patched".to_string(), index.to_string()]));
            }
            entry
                .patch
                .block_json
                .as_ref()
                .map(|json| {
                    serde_json::from_str::<NoteBlockNode>(json).map_err(|error| {
                        protocol::MutationApplyError::new(
                            "mutation.apply.invalid-value",
                            format!("block patch is not valid JSON: {error}"),
                        )
                        .at(["patched".to_string(), index.to_string(), "blockJson".to_string()])
                    })
                })
                .transpose()
        })
        .collect::<protocol::MutationApplyResult<_>>()?;
    let mut next = blocks.to_vec();
    for id in &delta.removed {
        remove_block_from_tree(&mut next, id);
    }
    for (position, entry) in delta.added.iter().enumerate() {
        let added_ids: Vec<String> = flatten_blocks(std::slice::from_ref(&entry.block))
            .into_iter()
            .map(|block| block_id(block).to_string())
            .collect();
        if added_ids.iter().enumerate().any(|(index, id)| {
            added_ids[..index].contains(id) || find_block(&next, id).is_some()
        }) {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.duplicate-target",
                "added block tree contains an existing or duplicate identity",
            )
            .at(["added".to_string(), position.to_string()]));
        }
        let container_len = match entry.parent_id.as_deref() {
            None => next.len(),
            Some(parent_id) => match find_block(&next, parent_id) {
                Some(NoteBlockNode::Group { children, .. }) => children.len(),
                Some(_) => {
                    return Err(protocol::MutationApplyError::new(
                        "mutation.apply.invalid-target",
                        "added block parent is not a group",
                    )
                    .at(["added".to_string(), position.to_string(), "parentId".to_string()]));
                }
                None => {
                    return Err(protocol::MutationApplyError::new(
                        "mutation.apply.missing-target",
                        "added block parent does not exist",
                    )
                    .at(["added".to_string(), position.to_string(), "parentId".to_string()]));
                }
            },
        };
        let index = entry.index.unwrap_or(container_len);
        if index > container_len {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.invalid-index",
                format!("block insertion index {index} exceeds length {container_len}"),
            )
            .at(["added".to_string(), position.to_string(), "index".to_string()]));
        }
        insert_block(&mut next, entry.parent_id.as_deref(), index, entry.block.clone());
    }
    for (entry, replacement) in delta.patched.iter().zip(replacements) {
        if let Some(replacement) = replacement {
            if block_id(&replacement) != entry.id {
                return Err(protocol::MutationApplyError::new(
                    "mutation.apply.invalid-target",
                    "block patch cannot change the target identity",
                )
                .at(["patched".to_string(), entry.id.clone()]));
            }
            if !update_block_in_tree(&mut next, &entry.id, replacement) {
                return Err(protocol::MutationApplyError::new(
                    "mutation.apply.missing-target",
                    "patched block does not exist after structural edits",
                )
                .at(["patched".to_string(), entry.id.clone()]));
            }
        }
    }
    if let Some(order) = &delta.reordered {
        if order.len() != next.len()
            || order.iter().enumerate().any(|(index, id)| {
                order[..index].contains(id)
                    || !next.iter().any(|block| block_id(block) == id)
            })
        {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.invalid-order",
                "root block reorder must be a complete unique permutation",
            )
            .at(["reordered"]));
        }
        let mut by_id: std::collections::BTreeMap<_, _> = next
            .into_iter()
            .map(|block| (block_id(&block).to_string(), block))
            .collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            ordered.push(by_id.remove(id).ok_or_else(|| {
                protocol::MutationApplyError::new(
                    "mutation.apply.missing-target",
                    "reordered root block does not exist",
                )
                .at(["reordered".to_string(), id.clone()])
            })?);
        }
        next = ordered;
    }
    let next_ids: Vec<_> = flatten_blocks(&next)
        .into_iter()
        .map(|block| block_id(block))
        .collect();
    if next_ids
        .iter()
        .enumerate()
        .any(|(index, id)| next_ids[..index].contains(id))
    {
        return Err(protocol::MutationApplyError::new(
            "mutation.apply.duplicate-target",
            "resulting block tree contains duplicate identities",
        )
        .at(["identities"]));
    }
    Ok(next)
}

fn apply_assets_delta(
    assets: &mut std::collections::BTreeMap<String, crate::artifacts::note::NoteImageAsset>,
    delta: &NoteAssetsDelta,
) -> protocol::MutationApplyResult<()> {
    for (key, value) in &delta.entries {
        if value.is_none() && !assets.contains_key(key) {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.missing-target",
                "removed asset does not exist",
            )
            .at([key.as_str()]));
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
                next.blocks = apply_blocks_delta(&next.blocks, delta)
                    .map_err(|error| error.under(["blocks"]))?;
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
                apply_assets_delta(&mut next.assets, assets)
                    .map_err(|error| error.under(["assets"]))?;
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
        take!(selected_block_ids);
        take!(active_utility_id);
        take!(engagement_input);
        take!(camera_x);
        take!(camera_y);
        take!(camera_zoom);
        take!(locale);
        take!(hovered_block_id);
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
mod diff_apply_tests {
    use super::*;

    #[test]
    fn malformed_nested_parent_rejects_without_changing_the_base() {
        let base = NoteSnapshot::default();
        let diff = NoteDiff {
            blocks: Some(NoteBlocksDelta {
                added: vec![NoteAddedBlockEntry {
                    parent_id: Some("missing-group".into()),
                    index: Some(0),
                    block: crate::artifacts::note::schema::create_block_by_kind(
                        "text", 0.0, 0.0,
                    ),
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let error = diff.apply(&base).expect_err("missing nested parent must reject");
        assert_eq!(error.code, "mutation.apply.missing-target");
        assert_eq!(error.target, ["blocks", "added", "0", "parentId"]);
        assert!(base.blocks.is_empty());
    }
}
