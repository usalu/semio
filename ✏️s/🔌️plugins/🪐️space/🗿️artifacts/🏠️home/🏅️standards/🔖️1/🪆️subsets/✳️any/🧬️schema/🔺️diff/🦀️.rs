//! 🧬️ S Home diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;

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
    #[state(config)]
    pub active_panel_tab: Option<String>,
    #[state(config)]
    pub locale: Option<String>,
}
//#endregion 🔖️Diff
