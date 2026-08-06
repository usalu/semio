//! 👁️ Imperative play app commands — ephemeral view state / runtime effect. Selection is scratch, `run`
//! evaluates into config, `setLocale` is config-only (was ephemeral `ViewModel::locale`).

use crate::apps::imperative::config::ImperativeConfigOperation;
use crate::artifacts::imperative::engine::ImperativeHost;
use crate::artifacts::imperative::op::ImperativeOperation;
use crate::artifacts::imperative::ImperativeDocument;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetSelection
pub mod set_selection {
    use super::*;
    use crate::apps::imperative::config::ImperativeConfig;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-selection")]
    pub struct SetSelection {
        pub ids: Vec<String>,
    }

    pub fn handle(payload: &SetSelection, _doc: &DocumentView<'_, ImperativeDocument>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeOperation, ImperativeConfigOperation>, Fault> {
        Ok(Emit::config(vec![ImperativeConfigOperation::SetSelectedSteps { ids: payload.ids.clone() }]))
    }
}
//#endregion 🔖️SetSelection

//#region 🔖️Run
pub mod run {
    use super::*;
    use crate::apps::imperative::config::ImperativeConfig;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "run")]
    pub struct Run {}

    pub fn handle(_payload: &Run, doc: &DocumentView<'_, ImperativeDocument>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeOperation, ImperativeConfigOperation>, Fault> {
        let host = ImperativeHost::from_document(doc.projection.clone());
        let result = host.run();
        let json = serde_json::to_string(&result.scope).unwrap_or_else(|_| format!("{:?}", result.scope));
        Ok(Emit::config(vec![ImperativeConfigOperation::SetRunOutput { json }]))
    }
}
//#endregion 🔖️Run

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;
    use crate::apps::imperative::config::ImperativeConfig;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &DocumentView<'_, ImperativeDocument>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeOperation, ImperativeConfigOperation>, Fault> {
        Ok(Emit::config(vec![ImperativeConfigOperation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale
