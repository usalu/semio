//! 🧪️ Generic compound clone effects agree with Serde and retain every cancellation frontier under original grants.
use semio_framework_test_fixtures::fixture_text;
use crate::{observe_retirement_allocations,ValueError,ValueRefusalKind,retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneGrant,RetainedCloneSource,RetainedCloneStep},retirement::controlled::ControlledRetirement};
use serde::{Serialize,Deserialize,de::DeserializeOwned};

#[derive(Debug,PartialEq,Serialize,Deserialize,crate::RetireOwned,crate::RetainedClone)]#[value(crate="crate")]
struct Record { name:String, values:Vec<String> }
#[derive(Debug,PartialEq,Serialize,Deserialize,crate::RetireOwned,crate::RetainedClone)]#[value(crate="crate")]
#[serde(tag="kind",rename_all="kebab-case")]
enum Variant { Unit, Record {label:String,values:Vec<i32>} }

fn drain<T:crate::retirement::RetireOwned>(value:T) {
    let mut owner=ControlledRetirement::new(value).map_err(|(error,_)|error).unwrap();
    for turn in 0..100_000 {
        if owner.terminal_is_empty(){return;}
        let d=owner.selected_demand(4096).unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(owner.selected_demand(grant.maximum_copy_bytes).unwrap(),d);
        let(step,heap)=observe_retirement_allocations(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(turn<99_999);
    }
}

fn execute<T:RetainedClone+Serialize+DeserializeOwned>(row:&serde_json::Value,policy:&serde_json::Value,cut:Option<usize>)->Result<(Option<serde_json::Value>,usize),ValueError> {
    let original:T=serde_json::from_value(row["source"].clone()).unwrap();
    let expected=serde_json::to_value(&original).unwrap();
    let demand=RetainedCloneSource::<T>::owned_constructor_demand::<()>();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RetainedCloneSource::<T>::constructor_copy_bytes(),maximum_capacity_bytes:demand.capacity_bytes,maximum_depth:demand.depth,..Default::default()};
    let((source,progress),heap)=observe_retirement_allocations(||RetainedCloneSource::admit_owned(original,(),grant).map_err(|(error,..)|error).unwrap());
    assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));assert!(progress.fits(grant));
    let mut cursor=T::retained_clone_cursor();let mut output=None;let mut turns=0;
    let result:Result<(),ValueError>=(|| {
        for turn in 0..policy["maximumTurns"].as_u64().unwrap()as usize {
            if cut==Some(turn){break;}
            let(demand,heap)=observe_retirement_allocations(||cursor.advance_demands(source.borrow(),policy["maximumCopyBytes"].as_u64().unwrap()as usize));assert_eq!(heap,(0,0));let demand=demand?;
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
            let denied=[Some(RetainedCloneGrant{maximum_items:0,..grant}),(demand.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant}),(demand.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant}),(demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant}),(demand.depth>0).then_some(RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..grant})];
            for denied in denied.into_iter().flatten(){
                let(result,heap)=observe_retirement_allocations(||cursor.advance(source.borrow(), policy["maximumCopyBytes"].as_u64().unwrap() as usize, denied));assert_eq!(heap,(0,0),"denied compound row={} cut={cut:?} turn={turn} demand={demand:?} granted={grant:?} denied={denied:?} result={:?}",row["name"],result.as_ref().map(|step|step.progress()).map_err(|error|(&error.kind,error.message.as_ref())));
                match result{Ok(step)=>assert_eq!(step.progress(),Default::default(),"denied compound row={} cut={cut:?} turn={turn} demand={demand:?} grant={grant:?} denied={denied:?}",row["name"]),Err(error)=>assert_eq!(error.kind,ValueRefusalKind::DepthLimit)}
                assert_eq!(cursor.advance_demands(source.borrow(),policy["maximumCopyBytes"].as_u64().unwrap()as usize)?,demand,"denied action keeps exact source-bound frontier");
            }
            let(step,heap)=observe_retirement_allocations(||cursor.advance(source.borrow(), policy["maximumCopyBytes"].as_u64().unwrap() as usize, grant));let step=step?;assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));turns=turn+1;
            if matches!(step,RetainedCloneStep::Complete(_)){output=cursor.take();break;}
            assert_ne!(step.progress(),Default::default(),"exact quoted next action must progress");
        }
        Ok(())
    })();
    let wire=output.as_ref().map(|value|serde_json::to_value(value).unwrap());if let Some(wire)=wire.as_ref(){assert_eq!(wire,&expected);}
    if let Some(value)=output.take(){drain(value);}
    cursor.begin_close();
    for turn in 0..100_000{
        if cursor.terminal_is_empty(){break;}let d=cursor.close_demands(4096).unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(cursor.close_demands(grant.maximum_copy_bytes).unwrap(),d);
        let(step,heap)=observe_retirement_allocations(||cursor.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(step.progress()!=Default::default() || cursor.terminal_is_empty(),"exact close stalled: row={} cut={cut:?} turn={turn} grant={grant:?}",row["name"]);assert!(turn<99_999);
    }
    assert!(cursor.terminal_is_empty());assert_eq!(observe_retirement_allocations(||drop(cursor)).1,(0,0));drain(source);result?;Ok((wire,turns))
}

fn law<T:RetainedClone+Serialize+DeserializeOwned>(row:&serde_json::Value,policy:&serde_json::Value){
    let(output,turns)=execute::<T>(row,policy,None).unwrap();assert_eq!(output.as_ref(),Some(&row["source"]));
    for cut in 0..=turns+1{let(actual,_)=execute::<T>(row,policy,Some(cut)).unwrap();assert_eq!(actual.is_some(),cut>=turns);}
}

#[test]
fn retained_clone_normal_demand_generic_compounds(){
    let policy:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../🧬️retained-clone/🔮️demands/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for row in policy["cases"].as_array().unwrap(){match row["kind"].as_str().unwrap(){
        "vec-scalar"=>law::<Vec<i32>>(row,&policy),"vec-string"=>law::<Vec<String>>(row,&policy),"array-string"=>law::<[String;3]>(row,&policy),"record"=>law::<Record>(row,&policy),"enum"=>law::<Variant>(row,&policy),"option-box"=>law::<Option<Box<String>>>(row,&policy),"tuple"=>law::<(String,Vec<i32>)>(row,&policy),_=>unreachable!()
    }}
}
