//! 🧪️ Exact primitive words cross actual original receiving, strict candidate rules and central inverse.
use super::*;
fn contract() -> serde_json::Value { serde_json::from_str(include_str!("🧪️tests/type-number-binary-input.json")).unwrap() }
fn value(raw:&serde_json::Value)->IfcValue {
    match raw["kind"].as_str().unwrap() {
        "unset"=>IfcValue::Unset,"derived"=>IfcValue::Derived,
        "real"=>IfcValue::Real(f64::from_bits(raw["value"]["bits"].as_str().unwrap().parse().unwrap())),
        "integer"=>IfcValue::Integer(raw["value"].as_str().unwrap().parse().unwrap()),
        "reference"=>IfcValue::Reference(raw["value"].as_str().unwrap().parse().unwrap()),
        "string"=>IfcValue::String(raw["value"].as_str().unwrap().into()),
        "enum"=>IfcValue::Enum(raw["value"].as_str().unwrap().into()),
        "binary"=>IfcValue::Binary(raw["value"].as_str().unwrap().into()),
        "aggregate"=>IfcValue::Aggregate(raw["value"].as_array().unwrap().iter().map(value).collect()),
        kind=>panic!("unsupported fixture {kind}"),
    }
}
pub(super) fn snapshot(law:&serde_json::Value,name:&str)->Arc<IfcSnapshot> {
    Arc::new(IfcSnapshot {entities:law["sources"][name]["entities"].as_array().unwrap().iter().map(|row|IfcEntity{
        id:row["id"].as_str().unwrap().parse().unwrap(),name:row["name"].as_str().unwrap().into(),args:row["args"].as_array().unwrap().iter().map(value).collect(),
        complex:row["complex"].as_array().unwrap().iter().map(|part|IfcComplexType{name:part["name"].as_str().unwrap().into(),args:part["args"].as_array().unwrap().iter().map(value).collect()}).collect(),
    }).collect(),..Default::default()})
}
fn event(row:&serde_json::Value)->editing::SnapshotEditEvent {
    let payload=serde_json::json!({"$ifcType":row["target"],"$ifcInputs":[{"index":row["index"].as_u64().unwrap().to_string(),"value":row["value"]}]});
    editing::SnapshotEditEvent::SetValue{path:"/entities/0".into(),value:super::type_input_tests::raw(&payload)}
}
#[test]
fn original_numeric_binary_words_match_neutral_output_and_progressive_inverse() {
    if DetailsTestProfile::catalog().schema != "IFC4X3_ADD2" { return; }
    let law=contract();for row in law["cases"].as_array().unwrap(){let source=snapshot(&law,row["source"].as_str().unwrap());let mut caller=OriginalWorkFixture::new();let mut work=Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&source,event(row),&mut caller,&mut work,|input,caller,work|{
            let mut result=None;
            for _ in 0..1000000 {match turn(caller,1,false,|cx|work.step(input,cx)){Ok(ArtifactCommandWorkStep::Complete(emit))=>{result=Some(Ok(emit));break;},Err(error)=>{result=Some(Err(error));break;},_=>{}}}
            let result=result.expect("exact primitive candidate terminates");assert_eq!(result.is_ok(),row["admitted"].as_bool().unwrap(),"{}",row["id"]);
            if let Ok(emit)=result {assert_eq!(emit.artifact_mutations.len(),1);assert!(matches!(emit.artifact_mutations[0],IfcMutation::SetEntityArg(_)));let mut next=source.as_ref().clone();let mut inverse=Vec::new();for mutation in &emit.artifact_mutations {inverse.push(mutation.inverse(&next).unwrap());next=protocol::apply_diff_cold(mutation.diff(&next).diff(),&next).unwrap();}let expected:Vec<_>=row["args"].as_array().unwrap().iter().map(value).collect();assert!(ifc_arguments_identical(&next.entities[0].args,&expected));for batch in inverse.into_iter().rev(){for mutation in batch {next=protocol::apply_diff_cold(mutation.diff(&next).diff(),&next).unwrap();}}assert!(ifc_arguments_identical(&next.entities[0].args,&source.entities[0].args));assert!(ifc_complex_identical(&next.entities[0].complex,&source.entities[0].complex));}
            super::constituent_optional_shape_tests::close_with_denials(caller,work);assert!(work.terminal_is_empty());
        });caller.finish();
    }
}
#[test]
fn original_numeric_binary_input_denials_are_stationary_and_cancel_retains_every_owner() {
    if DetailsTestProfile::catalog().schema != "IFC4X3_ADD2" { return; }
    let law=contract();for row in law["cases"].as_array().unwrap().iter().filter(|row|matches!(row["id"].as_str(),Some("real-negative-zero"|"number-large-integer"|"binary-bytes"))){let source=snapshot(&law,row["source"].as_str().unwrap());let mut caller=OriginalWorkFixture::new();let mut work=Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&source,event(row),&mut caller,&mut work,|input,caller,work|{
            let mut reached=false;
            for _ in 0..1000000 {
                if work.constituent_roster.receiving_primitives()||work.domain_validated&&!work.rules_validated {
                    let demand=work.work_demands(input,4096).unwrap();for axis in 0..6 {let mut loan=grant();let mut fuel=1;match axis {0=>loan.maximum_items=0,1 if demand.copy_bytes>0=>loan.maximum_copy_bytes=demand.copy_bytes-1,2 if demand.capacity_bytes>0=>loan.maximum_capacity_bytes=demand.capacity_bytes-1,3 if demand.release_bytes>0=>loan.maximum_release_bytes=demand.release_bytes-1,4=>loan.maximum_depth=0,5=>fuel=0,_=>continue};let state=work.constituent_roster.metadata();caller.work_turn(loan,fuel,|cx|{let before=cx.retained_progress();let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||work.step(input,cx));assert!(!matches!(answer?,ArtifactCommandWorkStep::Complete(_)));assert_eq!(before,cx.retained_progress());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));Ok(())}).unwrap();assert_eq!(state,work.constituent_roster.metadata());}
                    if work.domain_validated {reached=true;break;}
                }
                assert!(!matches!(turn(caller,1,false,|cx|work.step(input,cx)).unwrap(),ArtifactCommandWorkStep::Complete(_)));
            }
            assert!(reached);caller.cancel().cancel_now();assert!(caller.work_turn(grant(),1,|cx|work.step(input,cx)).is_err());super::constituent_optional_shape_tests::close_with_denials(caller,work);assert!(work.terminal_is_empty());
        });caller.finish();
    }
}
