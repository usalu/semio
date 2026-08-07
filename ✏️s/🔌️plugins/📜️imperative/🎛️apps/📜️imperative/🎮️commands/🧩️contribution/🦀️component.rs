//! 🧩️ Imperative play app commands — host-pushed plugin contributions (imperative modules).

use crate::apps::imperative::config::{ImperativeConfig, ImperativeConfigOperation};
use crate::artifacts::imperative::op::ImperativeOperation;
use crate::artifacts::imperative::ImperativeDocument;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetContributions
pub mod set_contributions {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "contributions")]
    pub struct SetContributions {
        pub json: String,
    }

    pub fn handle(payload: &SetContributions, _doc: &DocumentView<'_, ImperativeDocument>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeOperation, ImperativeConfigOperation>, Fault> {
        Ok(Emit::config(vec![ImperativeConfigOperation::SetContributions { json: payload.json.clone() }]))
    }
}
//#endregion 🔖️SetContributions
