//! 🧺️ 🧺️ Sourcing curation app commands command — `curation-add`.

use crate::artifacts::curation::op::SourcingMutation;
use crate::artifacts::curation::schema::{curation_decision_for_delta, CurationDecision};
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
#[dsl(keyword = "curation-add")]
pub struct CurationAdd {
    pub object_id: String,
}

pub fn handle(payload: &CurationAdd, doc: &ArtifactView<'_, CurationSnapshot>, _cfg: &ConfigView<'_, SourcingCurationConfig>) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation>, Fault> {
    Ok(emit_decision(curation_decision_for_delta(doc.snapshot, &payload.object_id, 1)))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::curation::schema::curated_count;
    use crate::editor::sourcing::commands::{curation_remove, curation_set_count, drop_on_curated, drop_on_pool};
    use crate::editor::sourcing::testkit::{dispatch, new_app};
    use crate::editor::sourcing::SourcingCurationCommand;

    #[semio_framework_async_macros::async_test]
    async fn curation_add_and_remove_round_trip_through_operations() {
        let mut app = new_app().await;
        let document = app.snapshot().expect("snapshot");
        // stock[2] isn't part of the fixture's pre-curated set, so a single add lands on count 1.
        let object_id = document.stock_extra[2].id.clone();
        dispatch(&mut app, SourcingCurationCommand::CurationAdd(CurationAdd { object_id: object_id.clone() })).await;
        assert_eq!(curated_count(&app.snapshot().expect("snapshot"), &object_id), 1);

        dispatch(&mut app, SourcingCurationCommand::CurationRemove(curation_remove::CurationRemove { object_id: object_id.clone() })).await;
        assert_eq!(curated_count(&app.snapshot().expect("snapshot"), &object_id), 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn curation_set_count_supports_both_delta_and_absolute_value() {
        let mut app = new_app().await;
        let object_id = app.snapshot().expect("snapshot").stock_extra[2].id.clone();
        dispatch(&mut app, SourcingCurationCommand::CurationSetCount(curation_set_count::CurationSetCount { object_id: object_id.clone(), delta: Some(3.0), value: None })).await;
        assert_eq!(curated_count(&app.snapshot().expect("snapshot"), &object_id), 3);
        dispatch(&mut app, SourcingCurationCommand::CurationSetCount(curation_set_count::CurationSetCount { object_id: object_id.clone(), delta: None, value: Some(2.0) })).await;
        assert_eq!(curated_count(&app.snapshot().expect("snapshot"), &object_id), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn drop_on_curated_and_drop_on_pool_mirror_add_and_remove() {
        let mut app = new_app().await;
        let document = app.snapshot().expect("snapshot");
        // stock[2] isn't part of the fixture's pre-curated set, so a single drop lands on count 1.
        let object_id = document.stock_extra[2].id.clone();
        dispatch(&mut app, SourcingCurationCommand::DropOnCurated(drop_on_curated::DropOnCurated { object_id: object_id.clone() })).await;
        assert_eq!(curated_count(&app.snapshot().expect("snapshot"), &object_id), 1);

        dispatch(&mut app, SourcingCurationCommand::DropOnPool(drop_on_pool::DropOnPool { object_id: object_id.clone() })).await;
        assert_eq!(curated_count(&app.snapshot().expect("snapshot"), &object_id), 0);
    }

    /// 🧬️ A no-op adjustment (removing an object that was never curated) must emit NOTHING —
    /// `SourcingMutation` has no whole-snapshot no-op sentinel to fall back on any more.
    #[semio_framework_async_macros::async_test]
    async fn curation_remove_on_an_uncurated_object_emits_no_mutation() {
        let mut app = new_app().await;
        let object_id = app.snapshot().expect("snapshot").stock_extra[2].id.clone();
        assert_eq!(curated_count(&app.snapshot().expect("snapshot"), &object_id), 0);
        let result = dispatch(&mut app, SourcingCurationCommand::CurationRemove(curation_remove::CurationRemove { object_id })).await;
        assert!(result.mutations.is_empty(), "removing an already-uncurated object is a no-op");
    }
}
//#endregion 🧪️Tests
