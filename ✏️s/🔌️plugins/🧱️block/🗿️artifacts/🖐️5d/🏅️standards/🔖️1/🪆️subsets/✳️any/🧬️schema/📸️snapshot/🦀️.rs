//! 🧬️ Block5d snapshot schema — artifact-lane fields only.

use crate::artifacts::block5d::{Block5dGripKind, Block5dGripTemplate, Block5dPart2d, Block5dPart3d, BLOCK_5D_SCHEMA};
use crate::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted block5d document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, ArtifactSchema)]
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
    #[value(default, rename = "2d")]
    #[cfg_attr(test, serde(default, rename = "2d"))]
    #[state(artifact)]
    pub part_2d: Block5dPart2d,
    #[dsl(block)]
    #[value(default, rename = "3d")]
    #[cfg_attr(test, serde(default, rename = "3d"))]
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
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for Block5dSnapshot {
    const EXTENSION: &'static str = "block5d";
    async fn envelope_id() -> &'static str {
        "block.block5d"
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    async fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Block5dSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    async fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
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
