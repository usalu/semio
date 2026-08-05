//! 🏠️ S Home launcher artifact — document entity (constitutional: general).

use serde::{Deserialize, Serialize};

//#region 🔖️Types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "shome")]
pub struct SHomeDocument {
    pub schema: String,
    #[serde(default)]
    #[dsl(key = "gen")]
    pub catalog_generation: u64,
}
//#endregion 🔖️Types
