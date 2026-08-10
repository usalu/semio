//! 🗂️ Raster play app commands — whole-document setters (`setSnapshot`, `setActiveExample`).

use crate::apps::raster::config::{RasterConfig, RasterConfigMutation};
use crate::artifacts::raster::engine::{empty_raster_document, semio_example_document};
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetSnapshot
pub mod set_snapshot {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-snapshot")]
    pub struct SetSnapshot {
        #[dsl(block)]
        pub snapshot: RasterSnapshot,
    }

    pub fn handle(payload: &SetSnapshot, _doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![RasterMutation::SetSnapshot { snapshot: payload.snapshot.clone() }]))
    }
}
//#endregion 🔖️SetSnapshot

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
        let replacement = if payload.example_id == "semio" { semio_example_document() } else { empty_raster_document() };
        Ok(Emit { artifact_mutations: vec![RasterMutation::SetSnapshot { snapshot: replacement }], config_mutations: vec![RasterConfigMutation::SetSelection { ids: Vec::new() }], ..Default::default() })
    }
}
//#endregion 🔖️SetActiveExample
