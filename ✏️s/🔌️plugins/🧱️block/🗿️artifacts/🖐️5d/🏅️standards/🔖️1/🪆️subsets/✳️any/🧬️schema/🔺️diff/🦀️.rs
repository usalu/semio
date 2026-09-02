//! 🧬️ Block5d diff schema — sparse field delta over the artifact.

use crate::artifacts::block5d::{Block5dGripKind, Block5dGripTemplate, Block5dPart2d, Block5dPart3d};
use crate::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the block5d artifact.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact_schema(id = "s.block.block5d")]
pub struct Block5dDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::block5d::schema::Block5dArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub part_kind: Option<BlockKindIdentity>,
    #[state(artifact)]
    pub part_2d: Option<Block5dPart2d>,
    #[state(artifact)]
    pub part_3d: Option<Block5dPart3d>,
    #[state(artifact)]
    pub representations: Option<Block5dRepresentationsDelta>,
    #[state(artifact)]
    pub grip_kinds: Option<Block5dGripKindsDelta>,
    #[state(artifact)]
    pub grips: Option<Block5dGripsDelta>,
    #[state(artifact)]
    pub compatibility: Option<Block5dCompatibilityDelta>,
    #[state(artifact)]
    pub attributes: Option<Block5dAttributesDelta>,
    #[state(artifact)]
    pub authors: Option<Block5dAuthorList>,
    #[state(artifact)]
    pub camera2d: Option<BlockCamera2d>,
    #[state(artifact)]
    pub camera3d: Option<BlockCamera3d>,
    #[state(artifact)]
    pub meta: Option<BlockMeta>,
    #[state(presence)]
    pub selected_ids: Option<Block5dStringList>,
    #[state(config)]
    pub locale: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct Block5dStringList {
    pub values: Vec<String>,
}

/// 👤️ Author-list wrapper.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct Block5dAuthorList {
    pub values: Vec<BlockAuthor>,
}

/// 📂 Identified-collection delta for Representations.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct Block5dRepresentationsDelta {
    pub added: Vec<BlockRepresentation>,
    pub removed: Vec<String>,
    pub patched: Vec<Block5dRepresentationsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched Representations entry.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block5dRepresentationsPatchEntry {
    pub id: String,
    pub patch: Block5dRepresentationsPatch,
}

/// 🩹 Sparse patch over Representations.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct Block5dRepresentationsPatch {
    pub replacement: Option<BlockRepresentation>,
}

/// 📂 Identified-collection delta for GripKinds.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct Block5dGripKindsDelta {
    pub added: Vec<Block5dGripKind>,
    pub removed: Vec<String>,
    pub patched: Vec<Block5dGripKindsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched GripKinds entry.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block5dGripKindsPatchEntry {
    pub id: String,
    pub patch: Block5dGripKindsPatch,
}

/// 🩹 Sparse patch over GripKinds.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct Block5dGripKindsPatch {
    pub replacement: Option<Block5dGripKind>,
}

/// 📂 Identified-collection delta for Grips.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct Block5dGripsDelta {
    pub added: Vec<Block5dGripTemplate>,
    pub removed: Vec<String>,
    pub patched: Vec<Block5dGripsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched Grips entry.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block5dGripsPatchEntry {
    pub id: String,
    pub patch: Block5dGripsPatch,
}

/// 🩹 Sparse patch over Grips.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct Block5dGripsPatch {
    pub replacement: Option<Block5dGripTemplate>,
}

/// 📂 Identified-collection delta for Compatibility.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct Block5dCompatibilityDelta {
    pub added: Vec<BlockCompatibilityRule>,
    pub removed: Vec<String>,
    pub patched: Vec<Block5dCompatibilityPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched Compatibility entry.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block5dCompatibilityPatchEntry {
    pub id: String,
    pub patch: Block5dCompatibilityPatch,
}

/// 🩹 Sparse patch over Compatibility.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct Block5dCompatibilityPatch {
    pub replacement: Option<BlockCompatibilityRule>,
}

/// 📂 Identified-collection delta for Attributes.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct Block5dAttributesDelta {
    pub added: Vec<BlockAttribute>,
    pub removed: Vec<String>,
    pub patched: Vec<Block5dAttributesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched Attributes entry.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block5dAttributesPatchEntry {
    pub id: String,
    pub patch: Block5dAttributesPatch,
}

/// 🩹 Sparse patch over Attributes.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct Block5dAttributesPatch {
    pub replacement: Option<BlockAttribute>,
}

//#endregion 🔖️DeltaHelpers
