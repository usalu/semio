//! 🧺️ 🧺️ Sourcing curation app commands command — `curation-set-count`.

use crate::artifacts::curation::op::SourcingMutation;
use crate::artifacts::curation::schema::{curation_decision_for_delta, curation_decision_for_set, CurationDecision};
use crate::artifacts::curation::CurationSnapshot;
use crate::editor::sourcing::config::{SourcingCurationConfig, SourcingCurationConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🔀️ Turns a resolved curation decision into the real `SourcingMutation` it corresponds to — `None`
/// for a no-op adjustment (e.g. dropping an already-zero item), which must NOT be recorded in
/// history at all (mirrors `apply_sourcing_mutation`'s former no-op-if-unknown-id silence, now
/// expressed as "emit nothing" instead of "emit a snapshot no-op").
fn mutation_for(decision: CurationDecision) -> Option<SourcingMutation> {
    match decision {
        CurationDecision::NoOp => None,
        CurationDecision::Create(item) => Some(crate::artifacts::curation::mutations::create_curated_item(item)),
        CurationDecision::ChangeCount { object_id, new_count } => Some(crate::artifacts::curation::mutations::change_curated_item_count(object_id, new_count)),
        CurationDecision::Delete { object_id } => Some(crate::artifacts::curation::mutations::delete_curated_item(object_id)),
    }
}

fn emit_decision(decision: CurationDecision) -> Emit<SourcingMutation, SourcingCurationConfigMutation> {
    match mutation_for(decision) {
        Some(mutation) => Emit::mutations(vec![mutation]),
        None => Emit::default(),
    }
}

//#region 🔖️CurationAdd
//#endregion 🔖️CurationAdd

//#region 🔖️CurationSetCount
//#endregion 🔖️CurationSetCount

//#region 🔖️CurationRemove
//#endregion 🔖️CurationRemove

//#region 🔖️DropOnPool
//#endregion 🔖️DropOnPool

//#region 🔖️DropOnCurated
//#endregion 🔖️DropOnCurated

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "curation-set-count")]
pub struct CurationSetCount {
    pub object_id: String,
    pub delta: Option<f64>,
    pub value: Option<f64>,
}

/// 🎚️ The pool/curated tables' count stepper cell dispatches this SAME action for both a relative
/// drag tick (`delta`) and an absolute typed value (`value`) — `delta` is checked first.
pub fn handle(payload: &CurationSetCount, doc: &ArtifactView<'_, CurationSnapshot>, _cfg: &ConfigView<'_, SourcingCurationConfig>) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation>, Fault> {
    let decision = if let Some(delta) = payload.delta {
        curation_decision_for_delta(doc.snapshot, &payload.object_id, delta as i64)
    } else if let Some(value) = payload.value {
        curation_decision_for_set(doc.snapshot, &payload.object_id, value.max(0.0) as u32)
    } else {
        CurationDecision::NoOp
    };
    Ok(emit_decision(decision))
}
