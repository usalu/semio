//! 🧬️ S Home diff schema — sparse field delta over the artifact.

use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the S Home artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.space.home")]
pub struct SHomeDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub catalog_generation: Option<u64>,
}
//#endregion 🔖️Diff
