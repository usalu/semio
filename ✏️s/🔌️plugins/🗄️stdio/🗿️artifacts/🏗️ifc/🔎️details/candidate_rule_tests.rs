//! 🧪️ The actual retained strict repair pins its caller and prepared batch through local rules and close.
use super::*;
fn source_and_event() -> (Arc<IfcSnapshot>, editing::SnapshotEditEvent) {
    let law: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/type-enum-input.json")).unwrap();
    let row = law["cases"].as_array().unwrap().iter().find(|row| row["target"] == "IFCDERIVEDUNIT").unwrap();
    let source = &law["sources"][row["source"].as_str().unwrap()];
    let value = super::equal_arity_type_tests::value;
    let snapshot = Arc::new(IfcSnapshot { entities: source["entities"].as_array().unwrap().iter().map(|entity| IfcEntity {
        id: entity["id"].as_str().unwrap().parse().unwrap(), name: entity["name"].as_str().unwrap().into(), args: entity["args"].as_array().unwrap().iter().map(value).collect(),
        complex: entity["complex"].as_array().unwrap().iter().map(|part| IfcComplexType { name: part["name"].as_str().unwrap().into(), args: part["args"].as_array().unwrap().iter().map(value).collect() }).collect(),
    }).collect(), ..Default::default() });
    let supplied = row["required"].as_array().unwrap().iter().map(|field| {
        let index = field["index"].as_u64().unwrap() as usize;
        DslValue::Object(vec![("index".into(), DslValue::String(index.to_string())), ("value".into(), super::type_input_tests::raw(&row["args"][index]))])
    }).collect();
    (snapshot, editing::SnapshotEditEvent::SetValue { path: "/entities/0".into(), value: DslValue::Object(vec![("$ifcType".into(), DslValue::String("IFCDERIVEDUNIT".into())), ("$ifcInputs".into(), DslValue::Array(supplied))]) })
}
#[test]
fn original_local_candidate_denials_and_foreign_prepared_caller_never_publish() {
    if std::mem::size_of::<<DetailsTestProfile as IfcDetailsProfile>::CandidateRules>() == 0 { return; }
    let (snapshot, event) = source_and_event();
    let foreign = snapshot.as_ref().clone();
    let mut caller = OriginalWorkFixture::new();
    let mut other = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(772), semio_framework_job::Generation(9));
    let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
    inputs(&snapshot, event, &mut caller, &mut work, |input, caller, work| {
        let mut complete = false;
        for step in 0..100000 {
            if work.domain_validated && !work.rules_validated {
                let event = work.original_event.as_ref().unwrap();
                let mutations = work.pending.as_deref().unwrap();
                let candidate = candidate_rules::IfcTypeCandidate::new::<DetailsTestProfile>(work.constituent_roster.type_admission().unwrap(), event, &snapshot, &work.address, mutations).unwrap();
                other.work_turn(grant(), 1, |cx| { let before = cx.retained_progress(); assert!(candidate.check(cx).is_err()); assert_eq!(before, cx.retained_progress()); Ok(()) }).unwrap();
                caller.work_turn(grant(), 1, |cx| {
                    let changed = candidate_rules::IfcTypeCandidate::new::<DetailsTestProfile>(work.constituent_roster.type_admission().unwrap(), event, &foreign, &work.address, mutations).unwrap();
                    assert!(changed.check(cx).is_err());
                    let copied = mutations.to_vec();
                    let changed = candidate_rules::IfcTypeCandidate::new::<DetailsTestProfile>(work.constituent_roster.type_admission().unwrap(), event, &snapshot, &work.address, &copied).unwrap();
                    assert!(changed.check(cx).is_err());
                    Ok(())
                }).unwrap();
                let demand = work.work_demands(input, 4096).unwrap();
                for axis in 0..6 {
                    let mut loan = grant(); let mut fuel = 1;
                    match axis { 0 => loan.maximum_items=0, 1 if demand.copy_bytes>0 => loan.maximum_copy_bytes=demand.copy_bytes-1, 2 if demand.capacity_bytes>0 => loan.maximum_capacity_bytes=demand.capacity_bytes-1, 3 if demand.release_bytes>0 => loan.maximum_release_bytes=demand.release_bytes-1, 4 => loan.maximum_depth=0, 5 => fuel=0, _ => continue }
                    caller.work_turn(loan, fuel, |cx| { let before=cx.retained_progress(); assert!(!matches!(work.step(input,cx)?,ArtifactCommandWorkStep::Complete(_))); assert_eq!(before,cx.retained_progress()); Ok(()) }).unwrap();
                }
                assert!(!work.publication_validated);
            }
            let mut loan=grant();if work.domain_validated&&!work.rules_validated {loan.maximum_items=1;}
            if let ArtifactCommandWorkStep::Complete(emit)=caller.work_turn(loan,1,|cx|work.step(input,cx)).unwrap() {
                assert!(emit.artifact_mutations.iter().all(|mutation|matches!(mutation,IfcMutation::SetEntityArg(_)))); complete=true; break;
            }
            assert!(step<99999);
        }
        assert!(complete);
        caller.cancel().cancel_now();
        super::constituent_optional_shape_tests::close_with_denials(caller,work);
        assert!(work.terminal_is_empty());
    });
    other.finish(); caller.finish();
}
#[test]
fn original_local_candidate_every_prefix_cancellation_retires_before_inputs() {
    if std::mem::size_of::<<DetailsTestProfile as IfcDetailsProfile>::CandidateRules>() == 0 { return; }
    let mut total=0;
    let (snapshot,event)=source_and_event();
    let mut caller=OriginalWorkFixture::new();let mut work=Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
    inputs(&snapshot,event,&mut caller,&mut work,|input,caller,work|{
        for _ in 0..100000 {total+=1;if matches!(turn(caller,1,false,|cx|work.step(input,cx)).unwrap(),ArtifactCommandWorkStep::Complete(_)){break;}}
        close(caller,work);
    });caller.finish();
    for prefix in 0..total {
        let (snapshot,event)=source_and_event();let mut caller=OriginalWorkFixture::new();let mut work=Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot,event,&mut caller,&mut work,|input,caller,work|{
            for _ in 0..prefix {assert!(!matches!(turn(caller,1,false,|cx|work.step(input,cx)).unwrap(),ArtifactCommandWorkStep::Complete(_)));}
            caller.cancel().cancel_now();
            caller.work_turn(grant(),1,|cx|{let before=cx.retained_progress();let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||work.step(input,cx));assert!(answer.is_err());assert_eq!(before,cx.retained_progress());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));Ok(())}).unwrap();
            super::constituent_optional_shape_tests::close_with_denials(caller,work);assert!(work.terminal_is_empty());
        });caller.finish();
    }
}
