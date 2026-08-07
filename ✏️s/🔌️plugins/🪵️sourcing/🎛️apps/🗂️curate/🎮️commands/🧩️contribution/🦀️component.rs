//! 🧩️ Sourcing curate app commands — host-pushed plugin contributions (sourcing modules).

use crate::apps::curate::config::{SourcingCurateConfig, SourcingCurateConfigOperation};
use crate::artifacts::curate::{op::SourcingOperation, CurateDocument};
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

    pub fn handle(payload: &SetContributions, _doc: &DocumentView<'_, CurateDocument>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingOperation, SourcingCurateConfigOperation>, Fault> {
        Ok(Emit::config(vec![SourcingCurateConfigOperation::SetContributions { json: payload.json.clone() }]))
    }
}
//#endregion 🔖️SetContributions
