//! 🗂️ Sourcing curate app commands — the preview/grid selection pointer (table row picks + world picks).

use crate::apps::curate::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::CurateSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SelectRow
pub mod select_row {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "select-row")]
    pub struct SelectRow {
        pub object_id: Option<String>,
    }

    pub fn handle(payload: &SelectRow, _doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
        Ok(Emit::config(vec![SourcingCurateConfigMutation::SetSelectedObject { object_id: payload.object_id.clone() }]))
    }
}
//#endregion 🔖️SelectRow

//#region 🔖️WorldSelect
pub mod world_select {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "world-select")]
    pub struct WorldSelect {
        pub ids: Vec<String>,
    }

    /// 🖱️ `worldSelect` keeps only the LAST id as the single selection (matches the pool/curated tables'
    /// single-select semantics — sourcing has no multi-select surface).
    pub fn handle(payload: &WorldSelect, _doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
        match payload.ids.last() {
            Some(id) => Ok(Emit::config(vec![SourcingCurateConfigMutation::SetSelectedObject { object_id: Some(id.clone()) }])),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️WorldSelect

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::curate::commands::selection::{select_row, world_select};
    use crate::apps::curate::modes::curate::windows::grid;
    use crate::apps::curate::testkit::{dispatch, new_app, render};
    use crate::apps::curate::SourcingCurateCommand;

    #[test]
    fn select_row_and_world_select_update_config_selection() {
        let mut app = new_app();
        let document = app.snapshot().expect("snapshot");
        let object_id = document.stock_extra[0].id.clone();
        let other_id = document.stock_extra[1].id.clone();

        dispatch(&mut app, SourcingCurateCommand::SelectRow(select_row::SelectRow { object_id: Some(object_id.clone()) }));
        let selected = render(&mut app, grid::SOURCING_CURATE_BODY_GRID);
        assert!(selected.contains(&object_id));

        dispatch(&mut app, SourcingCurateCommand::WorldSelect(world_select::WorldSelect { ids: vec![object_id, other_id.clone()] }));
        let selected = render(&mut app, grid::SOURCING_CURATE_BODY_GRID);
        let json: serde_json::Value = serde_json::from_str(&selected).unwrap();
        let instances_json = json.pointer("/world3d/instancesJson").and_then(|value| value.as_str()).unwrap();
        let instances: Vec<serde_json::Value> = serde_json::from_str(instances_json).unwrap();
        let selected_instance = instances.iter().find(|instance| instance["id"] == other_id).unwrap();
        assert_eq!(selected_instance["selected"], serde_json::json!(true), "worldSelect keeps the LAST id as the single selection");
    }
}
//#endregion 🧪️Tests
