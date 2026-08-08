//! 🗂️ Raster play app commands — whole-document setters (`setDocument`, `setActiveExample`).

use crate::apps::raster::config::{RasterConfig, RasterConfigMutation};
use crate::artifacts::raster::engine::{empty_raster_document, semio_example_document};
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::RasterProjection;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetDocument
pub mod set_document {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-document")]
    pub struct SetDocument {
        #[dsl(block)]
        pub document: RasterProjection,
    }

    pub fn handle(payload: &SetDocument, _doc: &DocumentView<'_, RasterProjection>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![RasterMutation::ReplaceDocument { document: payload.document.clone() }]))
    }
}
//#endregion 🔖️SetDocument

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, _doc: &DocumentView<'_, RasterProjection>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
        let replacement = if payload.example_id == "semio" { semio_example_document() } else { empty_raster_document() };
        Ok(Emit { document_mutations: vec![RasterMutation::ReplaceDocument { document: replacement }], config_mutations: vec![RasterConfigMutation::SetSelection { ids: Vec::new() }], ..Default::default() })
    }
}
//#endregion 🔖️SetActiveExample
