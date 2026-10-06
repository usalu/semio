//! 🧬️ BinarySnapshot schema — persistent fields + real codecs.

use crate::STDIO_BINARY_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;



//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.binary` snapshot.
///
/// 🧪️ F6-PILOT: `semio_framework_dsl_record_derive::DslRecord` added alongside the existing hand-rolled `store::ArtifactDsl`/
/// `store::ArtifactPack` below — NOT a replacement. `DslRecord` only gives this type `DslField`
/// (so it can be embedded as a variant payload, e.g. `BinaryMutation::SetSnapshot(set_snapshot::SetSnapshot{snapshot})`),
/// it does not touch the artifact's own honest hex-text/raw-binary envelope format.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.binary")]
pub struct BinarySnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    #[dsl(base64)]
    pub bytes: Vec<u8>,
}

impl Default for BinarySnapshot {
    fn default() -> Self {
        Self { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs
