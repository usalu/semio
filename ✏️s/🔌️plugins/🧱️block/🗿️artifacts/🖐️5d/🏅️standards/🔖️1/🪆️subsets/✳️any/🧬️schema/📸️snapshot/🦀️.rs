//! 🧬️ Block5d snapshot schema — artifact-lane fields only.

use crate::{Block5dGripKind, Block5dGripTemplate, Block5dPart2d, Block5dPart3d, BLOCK_5D_SCHEMA};
use crate::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted block5d document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(id = "block.block5d", layout = "lines")]
#[artifact_schema(id = "s.block.block5d")]
pub struct Block5dSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[dsl(block)]
    #[state(artifact)]
    pub part_kind: BlockKindIdentity,
    #[dsl(block)]
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[state(artifact)]
    pub part_2d: Block5dPart2d,
    #[dsl(block)]
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[state(artifact)]
    pub part_3d: Block5dPart3d,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub representations: Vec<BlockRepresentation>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub grip_kinds: Vec<Block5dGripKind>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub grips: Vec<Block5dGripTemplate>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub compatibility: Vec<BlockCompatibilityRule>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub attributes: Vec<BlockAttribute>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub authors: Vec<BlockAuthor>,
    #[dsl(block)]
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[state(artifact)]
    pub camera2d: BlockCamera2d,
    #[dsl(block)]
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[state(artifact)]
    pub camera3d: BlockCamera3d,
    #[dsl(block)]
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[state(artifact)]
    pub meta: BlockMeta,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for Block5dSnapshot {
    fn default() -> Self {
        Self {
            schema: BLOCK_5D_SCHEMA.to_string(),
            part_kind: BlockKindIdentity::default(),
            part_2d: Block5dPart2d::default(),
            part_3d: Block5dPart3d::default(),
            representations: Vec::new(),
            grip_kinds: Vec::new(),
            grips: Vec::new(),
            compatibility: Vec::new(),
            attributes: Vec::new(),
            authors: Vec::new(),
            camera2d: BlockCamera2d::default(),
            camera3d: BlockCamera3d::default(),
            meta: BlockMeta::default(),
        }
    }
}



