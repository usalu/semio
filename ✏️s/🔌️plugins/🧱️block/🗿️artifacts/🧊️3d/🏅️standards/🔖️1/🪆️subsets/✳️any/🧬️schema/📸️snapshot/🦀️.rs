//! 🧬️ Block3d snapshot schema — artifact-lane fields only.

use crate::{Block3dVortexKindExtra, Block3dVortexTemplate, BLOCK_3D_SCHEMA};
use crate::{BlockAttribute, BlockAuthor, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use ::semio_framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Snapshot
/// 📸️ Persisted block3d document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[dsl(id = "block.block3d", layout = "lines")]
#[artifact_schema(id = "s.block.block3d")]
pub struct Block3dSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[dsl(block)]
    #[state(artifact)]
    pub object_kind: BlockKindIdentity,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub representations: Vec<BlockRepresentation>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub catalog: store::ArtifactChild<SemioKitSnapshot>,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub vortex_kind_extra: Vec<Block3dVortexKindExtra>,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub vortices: Vec<Block3dVortexTemplate>,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub compatibility: Vec<BlockCompatibilityRule>,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub attributes: Vec<BlockAttribute>,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub authors: Vec<BlockAuthor>,
    #[dsl(block)]
    #[value(default)]
    #[state(artifact)]
    pub camera3d: BlockCamera3d,
    #[dsl(block)]
    #[value(default)]
    #[state(artifact)]
    pub meta: BlockMeta,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for Block3dSnapshot {
    fn default() -> Self {
        Self {
            schema: BLOCK_3D_SCHEMA.to_string(),
            object_kind: BlockKindIdentity::default(),
            representations: Vec::new(),
            catalog: crate::catalog_child_handle(&[]),
            vortex_kind_extra: Vec::new(),
            vortices: Vec::new(),
            compatibility: Vec::new(),
            attributes: Vec::new(),
            authors: Vec::new(),
            camera3d: BlockCamera3d::default(),
            meta: BlockMeta::default(),
        }
    }
}




#[path = "🔢️transport/🦀️.rs"]
pub(crate) mod json_transport;
