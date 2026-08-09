//! 🗣️ Raster play app commands — active locale (view action, never a document operation).

use crate::apps::raster::config::{RasterConfig, RasterConfigMutation};
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &DocumentView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
        Ok(Emit::config(vec![RasterConfigMutation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale
