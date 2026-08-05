//! 🗂️ Raster play app commands — selection/hover (view actions, never a document operation).

use crate::apps::raster::config::{RasterConfig, RasterConfigOperation};
use crate::artifacts::raster::engine::{flatten_raster_layers, layer_node_id};
use crate::artifacts::raster::op::RasterOperation;
use crate::artifacts::raster::RasterProjection;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetSelection
pub mod set_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-selection")]
    pub struct SetSelection {
        pub ids: Vec<String>,
    }

    pub fn handle(payload: &SetSelection, _doc: &DocumentView<'_, RasterProjection>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterOperation, RasterConfigOperation>, Fault> {
        Ok(Emit::config(vec![RasterConfigOperation::SetSelection { ids: payload.ids.clone() }]))
    }
}
//#endregion 🔖️SetSelection

//#region 🔖️SetHover
pub mod set_hover {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-hover")]
    pub struct SetHover {
        pub id: Option<String>,
    }

    pub fn handle(payload: &SetHover, _doc: &DocumentView<'_, RasterProjection>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterOperation, RasterConfigOperation>, Fault> {
        Ok(Emit::config(vec![RasterConfigOperation::SetHovered { id: payload.id.clone() }]))
    }
}
//#endregion 🔖️SetHover

//#region 🔖️SelectAll
pub mod select_all {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "select-all")]
    pub struct SelectAll {}

    pub fn handle(_payload: &SelectAll, doc: &DocumentView<'_, RasterProjection>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterOperation, RasterConfigOperation>, Fault> {
        let ids = flatten_raster_layers(&doc.projection.layers).into_iter().map(|layer| layer_node_id(layer).to_string()).collect();
        Ok(Emit::config(vec![RasterConfigOperation::SetSelection { ids }]))
    }
}
//#endregion 🔖️SelectAll
