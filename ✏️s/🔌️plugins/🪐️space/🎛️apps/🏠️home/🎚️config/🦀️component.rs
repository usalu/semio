//! ⚙️ S Home launcher app — `DocumentApp::Config` + its operation enum (constitutional: engine + op,
//! merged at app level per the per-app recipe: `Config`/`ConfigOperation` are inherently app-scoped,
//! never artifact-scoped).
//!
//! 🕳️ `SHomeDocument` is a two-field counter document (`schema` + `catalog_generation`) with no tree
//! structure, id generation, or media import/export of its own — the original monolith never factored
//! out a pure `empty_home_document()`/compute helper (every call site builds the literal
//! `SHomeDocument { schema: "s.home".into(), catalog_generation: N }` directly), so this app has no
//! document-side `⚙️engine` node under `🗿️artifacts/🏠️home`. What this file owns is `HomeConfig` — the
//! Home launcher's real `DocumentApp::Config`: the one `view_state.locale` read `apps::home`'s labels
//! actually need, plus the `active_panel_tab` action.

use serde::{Deserialize, Serialize};

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "homecfg")]
#[dsl(layout = "lines")]
pub struct HomeConfig {
    /// 👁️ Active launcher panel tab.
    pub active_panel_tab: String,
    /// 🗣️ BCP-47 locale tag.
    pub locale: String,
}

impl Default for HomeConfig {
    fn default() -> Self {
        Self { active_panel_tab: String::new(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(HomeConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// @emoji 🧮️ `HomeConfig`'s operation enum — mirrors `apps::space::config::SpaceConfigOperation`'s
/// whole-record-diff design (see its doc comment for the full rationale).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum HomeConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: HomeConfig,
    },
    #[dsl(key = "active-panel-tab")]
    SetActivePanelTab { tab_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl protocol::Operation<HomeConfig> for HomeConfigOperation {
    type Diff = HomeConfig;

    fn diff(&self, base: &HomeConfig) -> HomeConfig {
        let mut next = base.clone();
        match self {
            HomeConfigOperation::Snapshot { config } => return config.clone(),
            HomeConfigOperation::SetActivePanelTab { tab_id } => next.active_panel_tab = tab_id.clone(),
            HomeConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &HomeConfig) -> Vec<Self> {
        vec![HomeConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Operation;

    #[test]
    fn home_config_default_locale_is_english() {
        let config = HomeConfig::default();
        assert_eq!(config.locale, "en-US");
        assert!(config.active_panel_tab.is_empty());
    }

    #[test]
    fn home_config_dsl_text_round_trips() {
        store::test_support::assert_dsl_round_trip(&HomeConfig::default());
    }

    #[test]
    fn home_config_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&HomeConfigOperation::Snapshot { config: HomeConfig::default() });
        store::test_support::assert_op_line_round_trip(&HomeConfigOperation::SetActivePanelTab { tab_id: "tab-1".into() });
        store::test_support::assert_op_line_round_trip(&HomeConfigOperation::SetLocale { value: "de".into() });
    }

    #[test]
    fn home_config_operation_round_trips_via_apply_and_backwards() {
        let config = HomeConfig::default();
        let operation = HomeConfigOperation::SetLocale { value: "de".into() };
        let next = operation.diff(&config);
        assert_eq!(next.locale, "de");
        let backwards = operation.backwards(&config);
        let restored = backwards[0].diff(&next);
        assert_eq!(restored, config);
    }
}
//#endregion 🧪️Tests
