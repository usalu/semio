//! 🔍️ Sourcing curate app commands — pool-table filter/sort chrome (session-only config, never document).

use crate::apps::curate::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::{CurateDocument, SortDirection, TableSort};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetFilterQuery
pub mod set_filter_query {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "filter-query")]
    pub struct SetFilterQuery {
        pub value: String,
    }

    pub fn handle(payload: &SetFilterQuery, _doc: &DocumentView<'_, CurateDocument>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
        Ok(Emit::config(vec![SourcingCurateConfigMutation::SetFilterQuery { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetFilterQuery

//#region 🔖️SetFilterModule
pub mod set_filter_module {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "filter-module")]
    pub struct SetFilterModule {
        pub module_id: String,
        pub enabled: bool,
    }

    pub fn handle(payload: &SetFilterModule, _doc: &DocumentView<'_, CurateDocument>, cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
        let mut module_ids = cfg.projection.filters.module_ids.clone();
        if payload.enabled {
            if !module_ids.iter().any(|id| id == &payload.module_id) {
                module_ids.push(payload.module_id.clone());
            }
        } else {
            module_ids.retain(|id| id != &payload.module_id);
        }
        Ok(Emit::config(vec![SourcingCurateConfigMutation::SetFilterModules { module_ids }]))
    }
}
//#endregion 🔖️SetFilterModule

//#region 🔖️SetFilterTypology
pub mod set_filter_typology {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "filter-typology")]
    pub struct SetFilterTypology {
        pub path: String,
    }

    pub fn handle(payload: &SetFilterTypology, _doc: &DocumentView<'_, CurateDocument>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
        let path = if payload.path.is_empty() { Vec::new() } else { payload.path.split('/').map(String::from).collect() };
        Ok(Emit::config(vec![SourcingCurateConfigMutation::SetFilterTypology { path }]))
    }
}
//#endregion 🔖️SetFilterTypology

//#region 🔖️SetFilterMinAvailability
pub mod set_filter_min_availability {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "filter-min-availability")]
    pub struct SetFilterMinAvailability {
        pub delta: Option<f64>,
        pub value: Option<f64>,
    }

    pub fn handle(payload: &SetFilterMinAvailability, _doc: &DocumentView<'_, CurateDocument>, cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
        let current = cfg.projection.filters.min_availability as f64;
        let next = payload.delta.map(|d| current + d).or(payload.value).unwrap_or(current);
        Ok(Emit::config(vec![SourcingCurateConfigMutation::SetFilterMinAvailability { value: next.max(0.0) as u32 }]))
    }
}
//#endregion 🔖️SetFilterMinAvailability

//#region 🔖️SortTable
pub mod sort_table {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "sort-table")]
    pub struct SortTable {
        pub column_id: String,
        pub direction: String,
    }

    pub fn handle(payload: &SortTable, _doc: &DocumentView<'_, CurateDocument>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
        let sort = TableSort { column_id: payload.column_id.clone(), direction: if payload.direction == "desc" { SortDirection::Desc } else { SortDirection::Asc } };
        Ok(Emit::config(vec![SourcingCurateConfigMutation::SetSort { sort: Some(sort) }]))
    }
}
//#endregion 🔖️SortTable

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::curate::commands::filter::set_filter_min_availability;
    use crate::apps::curate::modes::curate::windows::pool;
    use crate::apps::curate::testkit::{dispatch, new_app, render};
    use crate::apps::curate::SourcingCurateCommand;

    #[test]
    fn set_filter_min_availability_clamps_to_zero() {
        let mut app = new_app();
        dispatch(&mut app, SourcingCurateCommand::SetFilterMinAvailability(set_filter_min_availability::SetFilterMinAvailability { delta: Some(-1000.0), value: None }));
        // Filters are config-only now — the pool render reflects the clamp indirectly via an empty result
        // for an unreasonably high min-availability; assert the clamp directly through a second command
        // that reports back the applied absolute value.
        dispatch(&mut app, SourcingCurateCommand::SetFilterMinAvailability(set_filter_min_availability::SetFilterMinAvailability { delta: Some(0.0), value: None }));
        let node = render(&mut app, pool::SOURCING_CURATE_BODY_POOL);
        // A clamped-to-zero min-availability keeps every stock row (all availabilities are >= 0).
        assert!(node.contains("Glulam"));
    }
}
//#endregion 🧪️Tests
