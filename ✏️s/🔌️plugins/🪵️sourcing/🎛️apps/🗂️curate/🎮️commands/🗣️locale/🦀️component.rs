//! 🗣️ Sourcing curate app commands — the host-pushed locale switch. Undeclared in the manifest (like
//! flow's `setLocale`), so `dsl(key)`/`command_id()` diverge on purpose (`"setLocale" as "locale"`).

use crate::apps::curate::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::CurateSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
        Ok(Emit::config(vec![SourcingCurateConfigMutation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale
