//! 🧬️ Block2d snapshot schema — artifact-lane fields only.

use crate::{Block2dHandleKind, Block2dHandleTemplate, Block2dPresentation, BLOCK_2D_SCHEMA};
use crate::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta};
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted block2d document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(id = "block.block2d", layout = "lines")]
#[artifact_schema(id = "s.block.block2d")]
pub struct Block2dSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[dsl(block)]
    #[state(artifact)]
    pub node_kind: BlockKindIdentity,
    #[dsl(block)]
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[state(artifact)]
    pub presentation: Block2dPresentation,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub handle_kinds: Vec<Block2dHandleKind>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub handles: Vec<Block2dHandleTemplate>,
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
    pub meta: BlockMeta,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for Block2dSnapshot {
    fn default() -> Self {
        Self {
            schema: BLOCK_2D_SCHEMA.to_string(),
            node_kind: BlockKindIdentity::default(),
            presentation: Block2dPresentation::default(),
            handle_kinds: Vec::new(),
            handles: Vec::new(),
            compatibility: Vec::new(),
            attributes: Vec::new(),
            authors: Vec::new(),
            camera2d: BlockCamera2d::default(),
            meta: BlockMeta::default(),
        }
    }
}



