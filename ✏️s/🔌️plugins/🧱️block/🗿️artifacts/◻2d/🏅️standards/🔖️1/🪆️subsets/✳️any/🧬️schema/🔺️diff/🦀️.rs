//! 🧬️ Block2d diff schema — sparse field delta over the artifact.

use crate::artifacts::block2d::{Block2dHandleKind, Block2dHandleTemplate, Block2dPresentation};
use crate::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the block2d artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.block.block2d")]
pub struct Block2dDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::block2d::schema::Block2dArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub node_kind: Option<BlockKindIdentity>,
    #[state(artifact)]
    pub presentation: Option<Block2dPresentation>,
    #[state(artifact)]
    pub handle_kinds: Option<Block2dHandleKindsDelta>,
    #[state(artifact)]
    pub handles: Option<Block2dHandlesDelta>,
    #[state(artifact)]
    pub compatibility: Option<Block2dCompatibilityDelta>,
    #[state(artifact)]
    pub attributes: Option<Block2dAttributesDelta>,
    #[state(artifact)]
    pub authors: Option<Block2dAuthorList>,
    #[state(artifact)]
    pub camera2d: Option<BlockCamera2d>,
    #[state(artifact)]
    pub meta: Option<BlockMeta>,
    #[state(presence)]
    pub selected_ids: Option<Block2dStringList>,
    #[state(config)]
    pub locale: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block2dStringList {
    pub values: Vec<String>,
}

/// 👤️ Author-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block2dAuthorList {
    pub values: Vec<BlockAuthor>,
}

/// 📂 Identified-collection delta for handle kinds.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block2dHandleKindsDelta {
    pub added: Vec<Block2dHandleKind>,
    pub removed: Vec<String>,
    pub patched: Vec<Block2dHandleKindsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched handle-kind entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block2dHandleKindsPatchEntry {
    pub id: String,
    pub patch: Block2dHandleKindsPatch,
}

/// 🩹 Sparse patch over a handle kind — whole-item replacement via `replacement`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block2dHandleKindsPatch {
    pub replacement: Option<Block2dHandleKind>,
}

/// 📂 Identified-collection delta for handle templates.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block2dHandlesDelta {
    pub added: Vec<Block2dHandleTemplate>,
    pub removed: Vec<String>,
    pub patched: Vec<Block2dHandlesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched handle-template entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block2dHandlesPatchEntry {
    pub id: String,
    pub patch: Block2dHandlesPatch,
}

/// 🩹 Sparse patch over a handle template.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block2dHandlesPatch {
    pub replacement: Option<Block2dHandleTemplate>,
}

/// 📂 Identified-collection delta for compatibility rules.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block2dCompatibilityDelta {
    pub added: Vec<BlockCompatibilityRule>,
    pub removed: Vec<String>,
    pub patched: Vec<Block2dCompatibilityPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched compatibility-rule entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block2dCompatibilityPatchEntry {
    pub id: String,
    pub patch: Block2dCompatibilityPatch,
}

/// 🩹 Sparse patch over a compatibility rule.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block2dCompatibilityPatch {
    pub replacement: Option<BlockCompatibilityRule>,
}

/// 📂 Identified-collection delta for attributes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block2dAttributesDelta {
    pub added: Vec<BlockAttribute>,
    pub removed: Vec<String>,
    pub patched: Vec<Block2dAttributesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched attribute entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block2dAttributesPatchEntry {
    pub id: String,
    pub patch: Block2dAttributesPatch,
}

/// 🩹 Sparse patch over an attribute.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block2dAttributesPatch {
    pub replacement: Option<BlockAttribute>,
}
//#endregion 🔖️DeltaHelpers
