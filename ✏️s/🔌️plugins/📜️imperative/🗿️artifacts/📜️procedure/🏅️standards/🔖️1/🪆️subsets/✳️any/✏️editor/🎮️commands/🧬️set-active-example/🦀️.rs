//! 🧬️ Imperative play app command — `set-active-example`.
//!
//! 🧬️ Whole-document replace has no in-history mutation (the whole-snapshot mutation variant is
//! banned outright — `📓️taxonomy.md`'s forbidden vocabulary), so loading a named example emits
//! `crate::editor::procedure::reset_procedure_document_effect` (an `Effect::LoadDocument`, outside
//! undo history) rather than an `artifact_mutations` entry. The react/wgpu shells dispatch this verb
//! at boot for every app whose subset registers examples; imperative declared no such verb at all,
//! so every boot and every navbar pick was answered `undeclared-action`.

use crate::editor::procedure::config::{ImperativeConfig, ImperativeConfigMutation};
use crate::mutations::ProcedureMutation;
use crate::ProcedureSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
/// 🧬️ The subset registers exactly one example (`crate::examples::demo`, `ID = "demo"`), whose
/// `.dsl.semio` asset IS this app's canonical demo program; every other id loads the empty procedure.
pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, ProcedureSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ProcedureMutation, ImperativeConfigMutation>, Fault> {
    let next = if payload.example_id.as_str() == crate::examples::demo::ID {
        <ProcedureSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).map_err(|error| Fault::from(format!("imperative-example-unparsable: {error}")))?
    } else {
        ProcedureSnapshot::default()
    };
    Ok(Emit { effects: vec![crate::editor::procedure::reset_procedure_document_effect(&next)], ..Default::default() })
}
//#endregion 🔖️Handler
