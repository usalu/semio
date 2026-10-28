//! 🧪️ Whole-entity removal receives one original primitive without eager entity preparation.
use super::*;
fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/entity-removal.json")).unwrap()
}
fn source(contract: &serde_json::Value, duplicate: bool) -> Arc<IfcSnapshot> {
    let mut entities = contract["ids"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(index, id)| IfcEntity { id: id.as_str().unwrap().parse().unwrap(), name: "IFCMATERIAL".into(), args: vec![IfcValue::String(contract["names"][index].as_str().unwrap().into()), IfcValue::Unset, IfcValue::Unset], complex: vec![] })
        .collect::<Vec<_>>();
    if duplicate {
        entities.push(entities[1].clone());
    }
    Arc::new(IfcSnapshot { entities, ..Default::default() })
}
#[test]
fn original_entity_removal_neutral_outputs_and_central_inverse_preserve_exact_ids_and_order() {
    let contract = contract();
    for row in contract["cases"].as_array().unwrap() {
        let snapshot = source(&contract, row["duplicate"] == true);
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID);
        let event = editing::SnapshotEditEvent::RemoveValue { path: format!("/entities/{}", row["position"].as_u64().unwrap()) };
        inputs(&snapshot, event, &mut caller, &mut work, |input, caller, work| {
            let mut result = None;
            for turn in 0..100000 {
                if work.prepared && work.reference.complete() && work.pending.is_none() && entity_remove::EntityRemoval::address(work.original_event.as_ref().unwrap(), &work.address).is_some() {
                    let demand = work.entity_removal.demands();
                    for axis in 0..6 {
                        let mut denied = grant();
                        let mut fuel = 1;
                        match axis {
                            0 => denied.maximum_items = 0,
                            1 => denied.maximum_copy_bytes = demand.copy_bytes - 1,
                            2 if demand.capacity_bytes > 0 => denied.maximum_capacity_bytes = demand.capacity_bytes - 1,
                            3 if demand.release_bytes > 0 => denied.maximum_release_bytes = demand.release_bytes - 1,
                            4 => denied.maximum_depth = 0,
                            5 => fuel = 0,
                            _ => continue,
                        }
                        let before = work.entity_removal.metadata();
                        caller
                            .work_turn(denied, fuel, |cx| {
                                let receipt = cx.retained_progress();
                                let step = work.step(input, cx)?;
                                assert!(!matches!(step, ArtifactCommandWorkStep::Complete(_)));
                                assert_eq!(receipt, cx.retained_progress());
                                Ok(())
                            })
                            .unwrap();
                        assert_eq!(before, work.entity_removal.metadata());
                    }
                }
                let next = caller.work_turn(RetainedCloneGrant { maximum_items: 1, ..grant() }, 1, |cx| work.step(input, cx));
                assert_eq!(work.reduction_ticks(), 0);
                assert!(work.entity.is_none() && work.semantic_emit.is_none() && work.cursor.terminal_is_empty());
                match next {
                    Ok(ArtifactCommandWorkStep::Complete(emit)) => {
                        result = Some(Ok(emit));
                        break;
                    }
                    Err(error) => {
                        result = Some(Err(error));
                        break;
                    }
                    _ => {}
                }
                assert!(turn < 99999);
            }
            let result = result.unwrap();
            assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{}", row["id"]);
            if let Ok(emit) = result {
                assert_eq!(emit.artifact_mutations.len(), 1);
                assert!(matches!(&emit.artifact_mutations[0], IfcMutation::RemoveEntity(_)));
                let mutation = &emit.artifact_mutations[0];
                let next = protocol::apply_diff_cold(mutation.diff(&snapshot).diff(), &snapshot).unwrap();
                assert_eq!(next.entities.iter().map(|entity| entity.id.to_string()).collect::<Vec<_>>(), row["remaining"].as_array().unwrap().iter().map(|id| id.as_str().unwrap().to_owned()).collect::<Vec<_>>());
                let mut restored = next;
                for inverse in mutation.inverse(&snapshot).unwrap() {
                    restored = protocol::apply_diff_cold(inverse.diff(&restored).diff(), &restored).unwrap();
                }
                assert_eq!(restored, *snapshot);
            }
            caller.cancel().cancel_now();
            close(caller, work);
            assert!(work.terminal_is_empty());
        });
        caller.finish();
        assert_eq!(Arc::strong_count(&snapshot), 1);
    }
}
#[test]
fn original_entity_removal_cancels_every_preparation_prefix_and_closes_original_owners() {
    let contract = contract();
    let snapshot = source(&contract, false);
    let original = snapshot.as_ref().clone();
    let mut turns = 0;
    for cancel_at in std::iter::once(None).chain((1..100000).map(Some)) {
        if cancel_at.is_some_and(|turn| turn > turns) {
            break;
        }
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, editing::SnapshotEditEvent::RemoveValue { path: "/entities/1".into() }, &mut caller, &mut work, |input, caller, work| {
            for turn in 1..100000 {
                if cancel_at == Some(turn) {
                    caller.cancel().cancel_now();
                }
                let result = caller.work_turn(RetainedCloneGrant { maximum_items: 1, ..grant() }, 1, |cx| work.step(input, cx));
                assert_eq!(snapshot.as_ref(), &original);
                assert!(work.entity.is_none() && work.semantic_emit.is_none() && work.cursor.terminal_is_empty());
                if cancel_at == Some(turn) {
                    assert!(result.is_err());
                    break;
                }
                if let ArtifactCommandWorkStep::Complete(emit) = result.unwrap() {
                    assert_eq!(emit.artifact_mutations.len(), 1);
                    turns = turn;
                    break;
                }
                assert!(turn < 99999);
            }
            caller.cancel().cancel_now();
            close(caller, work);
            assert!(work.terminal_is_empty());
        });
        caller.finish();
        assert_eq!(Arc::strong_count(&snapshot), 1);
    }
    assert!(turns > 10);
}
