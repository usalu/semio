//! 📄️ 📄️ Sourcing curate app commands command — `set-active-example`.

use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::CurateSnapshot;
use crate::editor::sourcing::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use crate::editor::sourcing::{reset_document_effect, DEMO_STOCK_EXAMPLE_ID, EMPTY_EXAMPLE_ID};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

//#region 📚️BoundedExample
const MAXIMUM_EXAMPLE_BYTES: usize = 8_192;
const _: () = assert!(crate::artifacts::curate::dsl::DEMO_STOCK_TEXT.len() <= MAXIMUM_EXAMPLE_BYTES);
const _: () = assert!(crate::artifacts::curate::dsl::EMPTY_CURATION_TEXT.len() <= MAXIMUM_EXAMPLE_BYTES);

pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
    let text = match payload.example_id.as_str() {
        "" | EMPTY_EXAMPLE_ID => crate::artifacts::curate::dsl::EMPTY_CURATION_TEXT,
        DEMO_STOCK_EXAMPLE_ID => crate::artifacts::curate::dsl::DEMO_STOCK_TEXT,
        _ => return Err(Fault::from("sourcing.example.unknown")),
    };
    let next = <CurateSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| Fault::from(error.to_string()))?;
    Ok(Emit { effects: vec![reset_document_effect(&next)], ..Default::default() })
}
//#endregion 📚️BoundedExample
