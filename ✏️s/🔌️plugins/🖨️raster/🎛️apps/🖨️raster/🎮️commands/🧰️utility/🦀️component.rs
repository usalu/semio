//! 🧰️ Raster play app commands — the composite-window active utility (framework `ActionKind::View`,
//! host-owned, never a document operation).

use crate::apps::raster::config::{RasterConfig, RasterConfigMutation};
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::RasterProjection;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetActiveUtility
pub mod set_active_utility {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-utility")]
    pub struct SetActiveUtility {
        pub utility_id: String,
    }

    pub fn handle(payload: &SetActiveUtility, _doc: &DocumentView<'_, RasterProjection>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
        Ok(Emit::config(vec![RasterConfigMutation::SetActiveUtility { utility_id: payload.utility_id.clone() }]))
    }
}
//#endregion 🔖️SetActiveUtility
