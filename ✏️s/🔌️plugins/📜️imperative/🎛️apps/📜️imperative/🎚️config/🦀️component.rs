//! 🧮️ Imperative play app — view state (`ImperativeConfig`) and its operation enum
//! (`ImperativeConfigOperation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.imperative` document. It still round-trips through a real
//! `DocumentStore` (with a real `backwards`), so selection/run-output/locale edits are VCS'd exactly like
//! document content — absorbing the former app-struct `RefCell` (`ImperativePlayRuntime`'s
//! `selected_step_ids`/`run_output_json`) plus the locale the UI used to read off the deleted
//! `ViewState`.

use serde::{Deserialize, Serialize};

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "imperativecfg")]
#[dsl(layout = "lines")]
pub struct ImperativeConfig {
    /// 👁️ Selected step ids — was `ImperativePlayRuntime::selected_step_ids`.
    pub selected_step_ids: Vec<String>,
    /// 📤️ Last `run` output, JSON-encoded scope — was `ImperativePlayRuntime::run_output_json`.
    pub run_output_json: String,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

impl Default for ImperativeConfig {
    fn default() -> Self {
        Self { selected_step_ids: Vec::new(), run_output_json: String::new(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(ImperativeConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// @emoji 🧮️ `ImperativeConfig`'s operation enum — one variant per settled interaction, plus a generic
/// `Snapshot` every variant's `backwards()` returns — mirrors `shooting_op::ShootingConfigOperation`'s
/// "undo this tick is exactly restore the whole-config snapshot from just before it" pattern:
/// `Operation::Diff` is the WHOLE `ImperativeConfig` (not a granular patch type), `diff()` returns "the
/// full config after this op", and `store::impl_whole_record_config!` supplies the
/// `OperationDiff<ImperativeConfig>` that returns that snapshot verbatim, ignoring `base`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum ImperativeConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: ImperativeConfig,
    },
    #[dsl(key = "selection")]
    SetSelectedSteps { ids: Vec<String> },
    #[dsl(key = "run-output")]
    SetRunOutput { json: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl protocol::Operation<ImperativeConfig> for ImperativeConfigOperation {
    type Diff = ImperativeConfig;

    fn diff(&self, base: &ImperativeConfig) -> ImperativeConfig {
        let mut next = base.clone();
        match self {
            ImperativeConfigOperation::Snapshot { config } => return config.clone(),
            ImperativeConfigOperation::SetSelectedSteps { ids } => next.selected_step_ids = ids.clone(),
            ImperativeConfigOperation::SetRunOutput { json } => next.run_output_json = json.clone(),
            ImperativeConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &ImperativeConfig) -> Vec<Self> {
        vec![ImperativeConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imperative_config_default_is_empty_english_selection() {
        let config = ImperativeConfig::default();
        assert!(config.selected_step_ids.is_empty());
        assert!(config.run_output_json.is_empty());
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn imperative_config_dsl_round_trips() {
        let config = ImperativeConfig { selected_step_ids: vec!["step-1".into(), "step-2".into()], run_output_json: r#"{"counter":1}"#.into(), locale: "de-DE".into() };
        store::test_support::assert_dsl_round_trip(&config);
        store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[test]
    fn config_operation_snapshot_diff_ignores_base() {
        let base = ImperativeConfig::default();
        let mut snapshot = base.clone();
        snapshot.selected_step_ids = vec!["step-1".into()];
        let operation = ImperativeConfigOperation::Snapshot { config: snapshot.clone() };
        assert_eq!(protocol::Operation::diff(&operation, &base), snapshot);
    }

    #[test]
    fn config_operation_set_selected_steps_round_trips() {
        let base = ImperativeConfig::default();
        let operation = ImperativeConfigOperation::SetSelectedSteps { ids: vec!["step-1".into(), "step-2".into()] };
        let next = protocol::Operation::diff(&operation, &base);
        assert_eq!(next.selected_step_ids, vec!["step-1".to_string(), "step-2".to_string()]);
        let backwards = protocol::Operation::backwards(&operation, &base);
        assert_eq!(backwards, vec![ImperativeConfigOperation::Snapshot { config: base }]);
        store::test_support::assert_op_line_round_trip(&operation);
    }

    #[test]
    fn config_operation_set_run_output_and_locale_round_trip() {
        store::test_support::assert_op_line_round_trip(&ImperativeConfigOperation::SetRunOutput { json: r#"{"counter":1}"#.into() });
        store::test_support::assert_op_line_round_trip(&ImperativeConfigOperation::SetLocale { value: "de-DE".into() });
    }
}
//#endregion 🧪️Tests
