//! 🧪️ Original source identity indexing pays actual growth, sorting, cancellation and close.
use super::*;
use semio_framework_plugin::app_testing::OriginalWorkFixture;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcEntity;
struct RequestIds([u64; 3]);
impl IfcSourceIds for RequestIds {
    fn source_len(&self) -> usize {
        self.0.len()
    }
    fn source_id(&self, row: usize) -> Result<u64, ValueError> {
        Ok(self.0[row])
    }
}
#[test]
fn original_id_index_reuses_the_same_bounded_rows_for_actual_request_identity_sources() {
    let source = RequestIds([u64::MAX, 1, 9]);
    let mut index = IfcSourceIndex::default();
    let mut caller = OriginalWorkFixture::new();
    let loan = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65536, maximum_capacity_bytes: 65536, maximum_release_bytes: 65536, maximum_depth: 256 };
    for turn in 0..10000 {
        if caller.work_turn(loan, 1, |cx| index.advance(&source, 71, cx)).unwrap() {
            break;
        }
        assert!(turn < 9999);
    }
    index.identity(&source, 71).unwrap();
    assert_eq!(index.find(u64::MAX), IdMatch::Unique(0));
    assert_eq!(index.find(9), IdMatch::Unique(2));
    let mut owner = Some(index);
    let mut close = semio_framework_value::retained_clone::RetainedCloneClose::default();
    while owner.is_some() || !close.is_empty() {
        let demand = close.next_owner_demands::<IfcSourceIndex>(owner.is_some(), &None, 4096).unwrap();
        caller
            .turn(loan, |cx| {
                cx.with_original_retirement(|cx| semio_framework_job::close_original_retained_step(cx, demand, |grant| if close.is_empty() { close.begin_granted(&mut owner, grant).map(|step| step.unwrap()) } else { close.step_granted(grant) }))
            })
            .unwrap();
    }
    caller.finish();
}
#[test]
fn contextual_source_index_retains_every_exact_id_and_rejects_ambiguous_identity() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("cases.json")).unwrap();
    let snapshot =
        IfcSnapshot { entities: contract["ids"].as_array().unwrap().iter().map(|id| IfcEntity { id: id.as_str().unwrap().parse().unwrap(), name: "IFCPROPERTYABSTRACTION".into(), args: vec![], complex: vec![] }).collect(), ..Default::default() };
    let mut index = IfcSourceIndex::default();
    let mut caller = OriginalWorkFixture::new();
    let loan = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65536, maximum_capacity_bytes: 65536, maximum_release_bytes: 65536, maximum_depth: 256 };
    for turn in 0..10000 {
        let demand = index.demands(&snapshot).unwrap();
        for axis in 0..6 {
            let mut denied = loan;
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
            let state = index.state;
            let shape = (index.rows.len(), index.rows.capacity());
            caller
                .work_turn(denied, fuel, |cx| {
                    let before = cx.retained_progress();
                    assert!(!index.advance(&snapshot, 71, cx)?);
                    assert_eq!(before, cx.retained_progress());
                    Ok(())
                })
                .unwrap();
            assert_eq!(state, index.state);
            assert_eq!((index.rows.len(), index.rows.capacity()), shape);
        }
        if caller
            .work_turn(loan, 1, |cx| {
                let before = cx.retained_progress();
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| index.advance(&snapshot, 71, cx));
                let after = cx.retained_progress();
                assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
                result
            })
            .unwrap()
        {
            break;
        }
        assert!(turn < 9999);
    }
    index.identity(&snapshot, 71).unwrap();
    assert_eq!(index.rows.iter().map(|row| row.ordinal).collect::<Vec<_>>(), contract["ordinals"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize).collect::<Vec<_>>());
    assert_eq!(index.find(u64::MAX), IdMatch::Unique(0));
    assert_eq!(index.find(3), IdMatch::Ambiguous);
    assert_eq!(index.find(11), IdMatch::Missing);
    assert!(index.identity(&snapshot, 72).is_err());
    assert!(index.identity(&snapshot.clone(), 71).is_err());
    caller.cancel().cancel_now();
    let state = index.state;
    assert!(caller.work_turn(loan, 1, |cx| index.advance(&snapshot, 71, cx)).is_err());
    assert_eq!(state, index.state);
    let mut owner = Some(index);
    let mut close = semio_framework_value::retained_clone::RetainedCloneClose::default();
    while owner.is_some() || !close.is_empty() {
        let demand = close.next_owner_demands::<IfcSourceIndex>(owner.is_some(), &None, 4096).unwrap();
        caller
            .turn(loan, |cx| {
                cx.with_original_retirement(|cx| semio_framework_job::close_original_retained_step(cx, demand, |grant| if close.is_empty() { close.begin_granted(&mut owner, grant).map(|step| step.unwrap()) } else { close.step_granted(grant) }))
            })
            .unwrap();
    }
    caller.finish();
}
