//! 🧬️ S Home diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the S Home artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.space.home")]
pub struct SHomeDiff {
    #[state(persistent)]
    pub schema: Option<String>,
    #[state(persistent)]
    pub catalog_generation: Option<u64>,
    #[state(local_ui)]
    pub active_panel_tab: Option<String>,
    #[state(local_ui)]
    pub locale: Option<String>,
}
//#endregion 🔖️Diff
