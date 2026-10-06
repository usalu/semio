//! 🧬️ S Home snapshot schema — artifact-lane fields only.

use crate::S_HOME_DOCUMENT_SCHEMA;
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted S Home launcher document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.space.home")]
#[dsl(extension = "shome")]
#[dsl(layout = "lines")]
pub struct SHomeSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    #[dsl(key = "gen")]
    pub catalog_generation: u64,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for SHomeSnapshot {
    fn default() -> Self {
        Self { schema: S_HOME_DOCUMENT_SCHEMA.into(), catalog_generation: 0 }
    }
}

//#region 🌉️IdentityBridge

//#endregion 🌉️IdentityBridge


