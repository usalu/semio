//! 🗣️ Raster play app commands — active locale (view action, never a document operation).

use crate::apps::raster::config::{RasterConfig, RasterConfigOperation};
use crate::artifacts::raster::op::RasterOperation;
use crate::artifacts::raster::RasterProjection;
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

    pub fn handle(payload: &SetLocale, _doc: &DocumentView<'_, RasterProjection>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterOperation, RasterConfigOperation>, Fault> {
        Ok(Emit::config(vec![RasterConfigOperation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale
