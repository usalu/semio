//! 👁️ Imperative play app commands — ephemeral view state / runtime effect. Selection is scratch, `run`
//! evaluates into config, `setLocale` is config-only (was ephemeral `ViewModel::locale`).

use crate::apps::imperative::config::ImperativeConfigMutation;
use crate::artifacts::imperative::engine::ImperativeHost;
use crate::artifacts::imperative::mutations::ImperativeMutation;
use crate::artifacts::imperative::ImperativeSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
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

    pub fn handle(payload: &SetSelection, _doc: &ArtifactView<'_, ImperativeSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeMutation, ImperativeConfigMutation>, Fault> {
        Ok(Emit::config(vec![ImperativeConfigMutation::SetSelectedSteps { ids: payload.ids.clone() }]))
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

    pub fn handle(_payload: &Run, doc: &ArtifactView<'_, ImperativeSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeMutation, ImperativeConfigMutation>, Fault> {
        let host = ImperativeHost::from_snapshot(doc.snapshot.clone());
        let result = host.run();
        let json = serde_json::to_string(&result.scope).unwrap_or_else(|_| format!("{:?}", result.scope));
        Ok(Emit::config(vec![ImperativeConfigMutation::SetRunOutput { json }]))
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

    pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, ImperativeSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeMutation, ImperativeConfigMutation>, Fault> {
        Ok(Emit::config(vec![ImperativeConfigMutation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale
