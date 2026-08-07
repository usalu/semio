//! 🧮️ Sourcing curate app — view state (`SourcingCurateConfig`) and its operation enum
//! (`SourcingCurateConfigOperation`).
//!
//! This is APP state, not document state: `filters` (search/sort) and the selected-object runtime
//! pointer used to live on `CurateDocument` itself (`Filters`/`CurateRuntime`) but are session-only view
//! state, not VCS'd content — both moved here so they round-trip through their own real `DocumentStore`
//! (with a real `backwards`) instead of polluting the VCS'd document. `locale` is the config-derived
//! counterpart to a host-pushed `ViewModel.locale` — `DocumentApp::render`/`handle` no longer receive a
//! `ViewModel` at all, so locale-aware label resolution reads it off here (see
//! `crate::apps::curate::terminology::sourcing_curate_labels`).

use crate::artifacts::curate::{Filters, TableSort};
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "sourcingcuratecfg")]
#[dsl(layout = "lines")]
pub struct SourcingCurateConfig {
    /// 🔍️ The pool table's active filter/search/sort state.
    #[dsl(block)]
    pub filters: Filters,
    /// 👁️ The single object selected for the preview/grid windows.
    pub selected_object_id: Option<String>,
    /// 🗣️ BCP-47 locale tag.
    pub locale: String,
    /// 🧩️ Host-pushed `ProgramContributionEntry[]` JSON for `sourcing.module` hot-swap installs.
    #[serde(default = "default_contributions_json")]
    pub contributions_json: String,
}

fn default_contributions_json() -> String {
    "[]".into()
}

impl Default for SourcingCurateConfig {
    fn default() -> Self {
        Self { filters: Filters::default(), selected_object_id: None, locale: "en-US".into(), contributions_json: default_contributions_json() }
    }
}

store::impl_whole_record_config!(SourcingCurateConfig);

/// 👁️ The single selected object, as a 0-or-1 id list — the shape `world3d_selection_json` wants.
/// Shared by the grid window's selection highlighting.
pub fn selected_ids(cfg: &SourcingCurateConfig) -> Vec<String> {
    cfg.selected_object_id.clone().into_iter().collect()
}

/// 👁️ `selected_ids` wrapped as the `{"selectedIds": […]}` JSON the table scenes' `selection_json`
/// slot wants. Shared by the pool and curated windows.
pub fn selection_json_for(cfg: &SourcingCurateConfig) -> String {
    serde_json::json!({ "selectedIds": selected_ids(cfg) }).to_string()
}
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ [`SourcingCurateConfig`]'s operation enum — one variant per settled interaction (search query,
/// module/typology/availability filters, sort, selection, locale), plus a generic `Snapshot` every
/// variant's `backwards()` returns. Since a config-only dispatch is a plain `Apply` (never an
/// `AmendLast`), each tick is its own distinct, real config edit, and "undo this tick" is exactly
/// "restore the whole-config snapshot from just before it" — no per-field reverse-patch bookkeeping
/// needed. `Operation::Diff` is the WHOLE `SourcingCurateConfig` (not a granular patch type): `diff()`
/// returns "the full config after this op", and `store::impl_whole_record_config!` supplies the
/// `OperationDiff<SourcingCurateConfig>` that returns that snapshot verbatim, ignoring `base`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum SourcingCurateConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: SourcingCurateConfig,
    },
    #[dsl(key = "filter-query")]
    SetFilterQuery { value: String },
    #[dsl(key = "filter-modules")]
    SetFilterModules { module_ids: Vec<String> },
    #[dsl(key = "filter-typology")]
    SetFilterTypology { path: Vec<String> },
    #[dsl(key = "filter-min-availability")]
    SetFilterMinAvailability { value: u32 },
    #[dsl(key = "sort")]
    SetSort {
        #[dsl(block)]
        sort: Option<TableSort>,
    },
    #[dsl(key = "selected-object")]
    SetSelectedObject { object_id: Option<String> },
    #[dsl(key = "locale")]
    SetLocale { value: String },
    #[dsl(key = "contributions")]
    SetContributions { json: String },
}

impl Operation<SourcingCurateConfig> for SourcingCurateConfigOperation {
    type Diff = SourcingCurateConfig;

    fn diff(&self, base: &SourcingCurateConfig) -> SourcingCurateConfig {
        let mut next = base.clone();
        match self {
            SourcingCurateConfigOperation::Snapshot { config } => return config.clone(),
            SourcingCurateConfigOperation::SetFilterQuery { value } => next.filters.query = value.clone(),
            SourcingCurateConfigOperation::SetFilterModules { module_ids } => next.filters.module_ids = module_ids.clone(),
            SourcingCurateConfigOperation::SetFilterTypology { path } => next.filters.typology_path = path.clone(),
            SourcingCurateConfigOperation::SetFilterMinAvailability { value } => next.filters.min_availability = *value,
            SourcingCurateConfigOperation::SetSort { sort } => next.filters.sort = sort.clone(),
            SourcingCurateConfigOperation::SetSelectedObject { object_id } => next.selected_object_id = object_id.clone(),
            SourcingCurateConfigOperation::SetLocale { value } => next.locale = value.clone(),
            SourcingCurateConfigOperation::SetContributions { json } => {
                next.contributions_json = json.clone();
                crate::artifacts::curate::engine::sync_sourcing_module_contributions(json);
            }
        }
        next
    }

    fn backwards(&self, base: &SourcingCurateConfig) -> Vec<Self> {
        vec![SourcingCurateConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::curate::SortDirection;

    #[test]
    fn sourcing_curate_config_default_matches_the_prior_document_defaults() {
        let config = SourcingCurateConfig::default();
        assert_eq!(config.filters, Filters::default());
        assert_eq!(config.selected_object_id, None);
        assert_eq!(config.locale, "en-US");
    }

    fn sample_config() -> SourcingCurateConfig {
        SourcingCurateConfig {
            filters: Filters {
                query: "glulam".into(),
                module_ids: vec!["beams".into()],
                typology_path: vec!["beams".into(), "steel".into()],
                min_availability: 5,
                sort: Some(TableSort { column_id: "availability".into(), direction: SortDirection::Desc }),
            },
            selected_object_id: Some("beam-glulam-gl24h".into()),
            locale: "de-DE".into(),
        }
    }

    /// 🎞️ Every variant's `backwards()` must exactly restore the pre-operation config.
    fn round_trip(config: &SourcingCurateConfig, operation: &SourcingCurateConfigOperation) -> SourcingCurateConfig {
        let forward = operation.diff(config);
        let backwards = operation.backwards(config);
        let mut restored = forward.clone();
        for back in &backwards {
            restored = back.diff(&restored);
        }
        assert_eq!(&restored, config, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[test]
    fn config_operations_round_trip_every_variant() {
        let config = sample_config();
        round_trip(&config, &SourcingCurateConfigOperation::SetFilterQuery { value: "kvh".into() });
        round_trip(&config, &SourcingCurateConfigOperation::SetFilterModules { module_ids: vec!["windows".into(), "slabs".into()] });
        round_trip(&config, &SourcingCurateConfigOperation::SetFilterTypology { path: vec!["slabs".into()] });
        round_trip(&config, &SourcingCurateConfigOperation::SetFilterMinAvailability { value: 12 });
        round_trip(&config, &SourcingCurateConfigOperation::SetSort { sort: None });
        round_trip(&config, &SourcingCurateConfigOperation::SetSelectedObject { object_id: None });
        round_trip(&config, &SourcingCurateConfigOperation::SetLocale { value: "en-US".into() });
        round_trip(&config, &SourcingCurateConfigOperation::SetContributions { json: "[]".into() });
        let snapshot = round_trip(&config, &SourcingCurateConfigOperation::Snapshot { config: SourcingCurateConfig::default() });
        assert_eq!(snapshot, SourcingCurateConfig::default());
    }

    #[test]
    fn config_op_text_round_trips_every_variant() {
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigOperation::Snapshot { config: sample_config() });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigOperation::SetFilterQuery { value: "kvh".into() });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigOperation::SetFilterModules { module_ids: vec!["beams".into(), "slabs".into()] });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigOperation::SetFilterTypology { path: vec!["beams".into(), "steel".into()] });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigOperation::SetFilterMinAvailability { value: 7 });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigOperation::SetSort { sort: Some(TableSort { column_id: "name".into(), direction: SortDirection::Asc }) });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigOperation::SetSort { sort: None });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigOperation::SetSelectedObject { object_id: Some("beam-glulam-gl24h".into()) });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigOperation::SetSelectedObject { object_id: None });
        store::test_support::assert_op_text_binary_equivalence(&SourcingCurateConfigOperation::SetLocale { value: "de-DE".into() });
    }
}
//#endregion 🧪️Tests
