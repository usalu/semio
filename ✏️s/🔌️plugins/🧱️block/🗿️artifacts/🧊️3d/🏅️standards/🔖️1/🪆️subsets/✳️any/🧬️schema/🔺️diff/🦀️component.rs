//! 🧬️ Block3d diff schema — sparse field delta over the artifact.

use crate::artifacts::block3d::{Block3dVortexKind, Block3dVortexTemplate};
use crate::artifacts::block3d::{Block3dBrushPreview, Block3dWindowView};
use crate::{BlockAttribute, BlockAuthor, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the block3d artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.block.block3d")]
pub struct Block3dDiff {
    #[state(artifact)] pub artifact: Option<Box<crate::artifacts::block3d::schema::Block3dArtifact>>,
    #[state(artifact)] pub schema: Option<String>,
    #[state(artifact)] pub object_kind: Option<BlockKindIdentity>,
    #[state(artifact)] pub representations: Option<Block3dRepresentationsDelta>,
    #[state(artifact)] pub vortex_kinds: Option<Block3dVortexKindsDelta>,
    #[state(artifact)] pub vortices: Option<Block3dVorticesDelta>,
    #[state(artifact)] pub compatibility: Option<Block3dCompatibilityDelta>,
    #[state(artifact)] pub attributes: Option<Block3dAttributesDelta>,
    #[state(artifact)] pub authors: Option<Block3dAuthorList>,
    #[state(artifact)] pub camera3d: Option<BlockCamera3d>,
    #[state(artifact)] pub meta: Option<BlockMeta>,
    #[state(presence)] pub selected_ids: Option<Block3dStringList>,
    #[state(presence)] pub active_representation_id: Option<Option<String>>,
    #[state(presence)] pub wanted_tags: Option<Block3dStringList>,
    #[state(config)] pub locale: Option<String>,
    #[state(config)] pub windows: Option<Block3dWindowsList>,
    #[state(config)] pub brush_vortex_kind_id: Option<Option<String>>,
    #[state(config)] pub brush_radius: Option<f64>,
    #[state(config)] pub brush_flip: Option<bool>,
    #[state(artifact)] pub brush_preview: Option<Option<Block3dBrushPreview>>,
    #[state(config)] pub camera: Option<Option<BlockCamera3d>>,
    #[state(artifact)] pub hovered_vortex_full_id: Option<Option<String>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block3dStringList {
    pub values: Vec<String>,
}

/// 👤️ Author-list wrapper.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block3dAuthorList {
    pub values: Vec<BlockAuthor>,
}

/// 🪟 Windows-list wrapper.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block3dWindowsList {
    pub values: Vec<Block3dWindowView>,
}

/// 📂 Identified-collection delta for Representations.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block3dRepresentationsDelta {
    pub added: Vec<BlockRepresentation>,
    pub removed: Vec<String>,
    pub patched: Vec<Block3dRepresentationsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched Representations entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block3dRepresentationsPatchEntry {
    pub id: String,
    pub patch: Block3dRepresentationsPatch,
}

/// 🩹 Sparse patch over Representations.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block3dRepresentationsPatch {
    pub replacement: Option<BlockRepresentation>,
}


/// 📂 Identified-collection delta for VortexKinds.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block3dVortexKindsDelta {
    pub added: Vec<Block3dVortexKind>,
    pub removed: Vec<String>,
    pub patched: Vec<Block3dVortexKindsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched VortexKinds entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block3dVortexKindsPatchEntry {
    pub id: String,
    pub patch: Block3dVortexKindsPatch,
}

/// 🩹 Sparse patch over VortexKinds.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block3dVortexKindsPatch {
    pub replacement: Option<Block3dVortexKind>,
}


/// 📂 Identified-collection delta for Vortices.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block3dVorticesDelta {
    pub added: Vec<Block3dVortexTemplate>,
    pub removed: Vec<String>,
    pub patched: Vec<Block3dVorticesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched Vortices entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block3dVorticesPatchEntry {
    pub id: String,
    pub patch: Block3dVorticesPatch,
}

/// 🩹 Sparse patch over Vortices.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block3dVorticesPatch {
    pub replacement: Option<Block3dVortexTemplate>,
}


/// 📂 Identified-collection delta for Compatibility.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block3dCompatibilityDelta {
    pub added: Vec<BlockCompatibilityRule>,
    pub removed: Vec<String>,
    pub patched: Vec<Block3dCompatibilityPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched Compatibility entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block3dCompatibilityPatchEntry {
    pub id: String,
    pub patch: Block3dCompatibilityPatch,
}

/// 🩹 Sparse patch over Compatibility.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block3dCompatibilityPatch {
    pub replacement: Option<BlockCompatibilityRule>,
}


/// 📂 Identified-collection delta for Attributes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block3dAttributesDelta {
    pub added: Vec<BlockAttribute>,
    pub removed: Vec<String>,
    pub patched: Vec<Block3dAttributesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched Attributes entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block3dAttributesPatchEntry {
    pub id: String,
    pub patch: Block3dAttributesPatch,
}

/// 🩹 Sparse patch over Attributes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Block3dAttributesPatch {
    pub replacement: Option<BlockAttribute>,
}

//#endregion 🔖️DeltaHelpers
