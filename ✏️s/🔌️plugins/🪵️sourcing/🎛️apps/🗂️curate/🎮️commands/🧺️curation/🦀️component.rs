//! 🧺️ Sourcing curate app commands — curated-set mutations (add/remove/set-count/drag-drop). Distinct
//! from `crate::apps::curate::modes::curate::windows::curated` (the "Curated" window this pushes into).

use crate::apps::curate::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use crate::artifacts::curate::engine::{curation_decision_for_delta, curation_decision_for_set, CurationDecision};
use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::CurateSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

/// 🔀️ Turns a resolved curation decision into the real `SourcingMutation` it corresponds to — `None`
/// for a no-op adjustment (e.g. dropping an already-zero item), which must NOT be recorded in
/// history at all (mirrors `apply_sourcing_mutation`'s former no-op-if-unknown-id silence, now
/// expressed as "emit nothing" instead of "emit a snapshot no-op").
fn mutation_for(decision: CurationDecision) -> Option<SourcingMutation> {
    match decision {
        CurationDecision::NoOp => None,
        CurationDecision::Create(item) => Some(crate::artifacts::curate::mutations::create_curated_item(item)),
        CurationDecision::ChangeCount { object_id, new_count } => Some(crate::artifacts::curate::mutations::change_curated_item_count(object_id, new_count)),
        CurationDecision::Delete { object_id } => Some(crate::artifacts::curate::mutations::delete_curated_item(object_id)),
    }
}

fn emit_decision(decision: CurationDecision) -> Emit<SourcingMutation, SourcingCurateConfigMutation> {
    match mutation_for(decision) {
        Some(mutation) => Emit::mutations(vec![mutation]),
        None => Emit::default(),
    }
}

//#region 🔖️CurateAdd
pub mod curate_add {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "curate-add")]
    pub struct CurateAdd {
        pub object_id: String,
    }

    pub fn handle(payload: &CurateAdd, doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
        Ok(emit_decision(curation_decision_for_delta(doc.snapshot, &payload.object_id, 1)))
    }
}
//#endregion 🔖️CurateAdd

//#region 🔖️CurateSetCount
pub mod curate_set_count {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "curate-set-count")]
    pub struct CurateSetCount {
        pub object_id: String,
        pub delta: Option<f64>,
        pub value: Option<f64>,
    }

    /// 🎚️ The pool/curated tables' count stepper cell dispatches this SAME action for both a relative
    /// drag tick (`delta`) and an absolute typed value (`value`) — `delta` is checked first.
    pub fn handle(payload: &CurateSetCount, doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
        let decision = if let Some(delta) = payload.delta {
            curation_decision_for_delta(doc.snapshot, &payload.object_id, delta as i64)
        } else if let Some(value) = payload.value {
            curation_decision_for_set(doc.snapshot, &payload.object_id, value.max(0.0) as u32)
        } else {
            CurationDecision::NoOp
        };
        Ok(emit_decision(decision))
    }
}
//#endregion 🔖️CurateSetCount

//#region 🔖️CurateRemove
pub mod curate_remove {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "curate-remove")]
    pub struct CurateRemove {
        pub object_id: String,
    }

    pub fn handle(payload: &CurateRemove, doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
        Ok(emit_decision(curation_decision_for_set(doc.snapshot, &payload.object_id, 0)))
    }
}
//#endregion 🔖️CurateRemove

//#region 🔖️DropOnPool
pub mod drop_on_pool {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "drop-on-pool")]
    pub struct DropOnPool {
        pub object_id: String,
    }

    /// 🪂️ Dropping a curated row back onto the pool mirrors `curate_remove`: zero its curated count.
    pub fn handle(payload: &DropOnPool, doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
        Ok(emit_decision(curation_decision_for_set(doc.snapshot, &payload.object_id, 0)))
    }
}
//#endregion 🔖️DropOnPool

//#region 🔖️DropOnCurated
pub mod drop_on_curated {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "drop-on-curated")]
    pub struct DropOnCurated {
        pub object_id: String,
    }

    pub fn handle(payload: &DropOnCurated, doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
        Ok(emit_decision(curation_decision_for_delta(doc.snapshot, &payload.object_id, 1)))
    }
}
//#endregion 🔖️DropOnCurated

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::curate::commands::curation::{curate_add, curate_remove, curate_set_count, drop_on_curated, drop_on_pool};
    use crate::apps::curate::testkit::{dispatch, new_app};
    use crate::apps::curate::SourcingCurateCommand;
    use crate::artifacts::curate::engine::curated_count;

    #[test]
    fn curate_add_and_remove_round_trip_through_operations() {
        let mut app = new_app();
        let document = app.snapshot().expect("snapshot");
        // stock[2] isn't part of the fixture's pre-curated set, so a single add lands on count 1.
        let object_id = document.stock[2].id.clone();
        dispatch(&mut app, SourcingCurateCommand::CurateAdd(curate_add::CurateAdd { object_id: object_id.clone() }));
        assert_eq!(curated_count(&app.snapshot().expect("snapshot"), &object_id), 1);

        dispatch(&mut app, SourcingCurateCommand::CurateRemove(curate_remove::CurateRemove { object_id: object_id.clone() }));
        assert_eq!(curated_count(&app.snapshot().expect("snapshot"), &object_id), 0);
    }

    #[test]
    fn curate_set_count_supports_both_delta_and_absolute_value() {
        let mut app = new_app();
        let object_id = app.snapshot().expect("snapshot").stock[2].id.clone();
        dispatch(&mut app, SourcingCurateCommand::CurateSetCount(curate_set_count::CurateSetCount { object_id: object_id.clone(), delta: Some(3.0), value: None }));
        assert_eq!(curated_count(&app.snapshot().expect("snapshot"), &object_id), 3);
        dispatch(&mut app, SourcingCurateCommand::CurateSetCount(curate_set_count::CurateSetCount { object_id: object_id.clone(), delta: None, value: Some(2.0) }));
        assert_eq!(curated_count(&app.snapshot().expect("snapshot"), &object_id), 2);
    }

    #[test]
    fn drop_on_curated_and_drop_on_pool_mirror_add_and_remove() {
        let mut app = new_app();
        let document = app.snapshot().expect("snapshot");
        // stock[2] isn't part of the fixture's pre-curated set, so a single drop lands on count 1.
        let object_id = document.stock[2].id.clone();
        dispatch(&mut app, SourcingCurateCommand::DropOnCurated(drop_on_curated::DropOnCurated { object_id: object_id.clone() }));
        assert_eq!(curated_count(&app.snapshot().expect("snapshot"), &object_id), 1);

        dispatch(&mut app, SourcingCurateCommand::DropOnPool(drop_on_pool::DropOnPool { object_id: object_id.clone() }));
        assert_eq!(curated_count(&app.snapshot().expect("snapshot"), &object_id), 0);
    }

    /// 🧬️ A no-op adjustment (removing an object that was never curated) must emit NOTHING —
    /// `SourcingMutation` has no whole-snapshot no-op sentinel to fall back on any more.
    #[test]
    fn curate_remove_on_an_uncurated_object_emits_no_mutation() {
        let mut app = new_app();
        let object_id = app.snapshot().expect("snapshot").stock[2].id.clone();
        assert_eq!(curated_count(&app.snapshot().expect("snapshot"), &object_id), 0);
        let result = crate::apps::curate::testkit::dispatch(&mut app, SourcingCurateCommand::CurateRemove(curate_remove::CurateRemove { object_id }));
        assert!(result.mutations.is_empty(), "removing an already-uncurated object is a no-op");
    }
}
//#endregion 🧪️Tests
