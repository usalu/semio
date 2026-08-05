//! 🗣️ Sourcing curate app commands — the host-pushed locale switch. Undeclared in the manifest (like
//! flow's `setLocale`), so `dsl(key)`/`command_id()` diverge on purpose (`"setLocale" as "locale"`).

use crate::apps::curate::config::{SourcingCurateConfig, SourcingCurateConfigOperation};
use crate::artifacts::curate::op::SourcingOperation;
use crate::artifacts::curate::CurateDocument;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &DocumentView<'_, CurateDocument>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingOperation, SourcingCurateConfigOperation>, Fault> {
        Ok(Emit::config(vec![SourcingCurateConfigOperation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale
