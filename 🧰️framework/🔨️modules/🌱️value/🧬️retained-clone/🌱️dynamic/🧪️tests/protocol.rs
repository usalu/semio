const MAXIMUM_CLONE_WORK:u64=4096;
use semio_framework_test_fixtures::fixture_text;
use super::*;
use crate::{Number,observe_retirement_allocations as observe};

pub(super) fn decode(value:&serde_json::Value)->DslValue {
    match value["kind"].as_str().unwrap(){
        "null"=>DslValue::Null,
        "bool"=>DslValue::Bool(value["value"].as_bool().unwrap()),
        "uint"=>DslValue::Number(Number::UInt(value["value"].as_str().unwrap().parse().unwrap())),
        "int"=>DslValue::Number(Number::Int(value["value"].as_str().unwrap().parse().unwrap())),
        "float"=>DslValue::Number(Number::Float(f64::from_bits(u64::from_str_radix(value["bits"].as_str().unwrap(),16).unwrap()))),
        "text"=>DslValue::String(value["value"].as_str().unwrap().into()),
        "bytes"=>DslValue::Bytes(value["value"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect()),
        "array"=>DslValue::Array(value["value"].as_array().unwrap().iter().map(decode).collect()),
        "object"=>DslValue::Object(value["value"].as_array().unwrap().iter().map(|entry|(entry[0].as_str().unwrap().into(),decode(&entry[1]))).collect()),
        _=>unreachable!(),
    }
}

pub(super) fn grant(demand:RetirementDemand,copy:usize)->RetainedCloneGrant {RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy.max(demand.copy_bytes),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}

pub(super) fn denied(grant:RetainedCloneGrant,demand:RetirementDemand)->Vec<RetainedCloneGrant>{
    [Some(RetainedCloneGrant {maximum_items:0,..grant}),(demand.copy_bytes>0).then_some(RetainedCloneGrant {maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant}),(demand.capacity_bytes>0).then_some(RetainedCloneGrant {maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant}),(demand.release_bytes>0).then_some(RetainedCloneGrant {maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant}),(demand.depth>0).then_some(RetainedCloneGrant {maximum_depth:demand.depth.saturating_sub(1),..grant})].into_iter().flatten().collect()
}

pub(super) fn check(outcome:crate::RetainedWorkOutcome,heap:(usize,usize),grant:RetainedCloneGrant)->RetainedCloneProgress {
    let work=outcome.work;let progress=outcome.result.unwrap_or_else(|error|panic!("Original dynamic granted step refused work={work} {:?}: {}",error.kind,error)).progress();assert!(progress==RetainedCloneProgress::default()||work>0,"retained effects have no actual work");assert!(progress.fits(grant));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));progress
}

pub(super) fn check_denied(outcome:crate::RetainedWorkOutcome,heap:(usize,usize),grant:RetainedCloneGrant,demand:RetirementDemand){
    let work=outcome.work;let result=outcome.result;assert_eq!(work,0);assert_eq!(heap,(0,0));match result {Ok(step)=>assert_eq!(step.progress(),Default::default()),Err(error)=>{assert!(grant.maximum_depth<demand.depth);assert_eq!(error.kind,ValueRefusalKind::DepthLimit);assert_eq!(error.retained_progress(),Default::default());}}
}

pub(super) fn close_cursor(cursor:&mut DslValueRetainedCloneCursor,copy:usize,born:&mut usize,freed:&mut usize){
    assert_eq!(observe(||cursor.begin_close()).1,(0,0));let mut stalled=0;
    for _ in 0..100000 {
        if cursor.terminal_is_empty(){return;}
        let(demand,heap)=observe(||cursor.close_demands(copy).unwrap());assert_eq!(heap,(0,0));let admitted=grant(demand,copy);
        for denied in denied(admitted,demand){let(result,heap)=observe(||cursor.close_selected(copy,denied));check_denied(result,heap,denied,demand);assert_eq!(cursor.close_demands(copy).unwrap(),demand);}
        if demand.release_bytes>8 {let denied=RetainedCloneGrant {maximum_release_bytes:8,..admitted};let(result,heap)=observe(||cursor.close_selected(copy,denied));check_denied(result,heap,denied,demand);}
        let(result,heap)=observe(||cursor.close_selected(copy,admitted));let progress=check(result,heap,admitted);*born+=heap.0;*freed+=heap.1;stalled=if progress==Default::default(){stalled+1}else{0};assert!(stalled<=143,"Original dynamic cursor closure stalled copy={copy} demand={demand:?}");
    }
    panic!("Original dynamic cursor did not close its retained original graph");
}

pub(super) fn close_source(source:&mut semio_framework_value_clone_source_testing::FixtureSource<DslValue>,copy:usize,born:&mut usize,freed:&mut usize){
    let mut stalled=0;
    for _ in 0..100000 {
        if source.terminal_is_empty(){return;}
        let demand=source.next_close_demand(copy).unwrap();let admitted=grant(demand,0);assert_eq!(source.next_close_demand(admitted.maximum_copy_bytes).unwrap(),demand);
        if demand.release_bytes>8 {let denied=RetainedCloneGrant {maximum_release_bytes:8,..admitted};let(result,heap)=semio_framework_value_clone_source_testing::observe_fixture_source_allocations(||source.close_step(denied));check_denied(result,heap,denied,demand);}
        let(result,heap)=semio_framework_value_clone_source_testing::observe_fixture_source_allocations(||source.close_step(admitted));let progress=check(result,heap,admitted);*born+=heap.0;*freed+=heap.1;stalled=if progress==Default::default(){stalled+1}else{0};assert!(stalled<=143,"Original dynamic source closure stalled copy={copy} demand={demand:?}");
    }
    panic!("Original dynamic source graph remained live");
}

pub(super) fn close_plain_source(source:&mut RetainedCloneSource<DslValue>,copy:usize,born:&mut usize,freed:&mut usize){
    let mut stalled=0;
    for _ in 0..100000 {
        if source.terminal_is_empty(){return;}
        let demand=source.next_close_demand(copy).unwrap();let admitted=grant(demand,0);assert_eq!(source.next_close_demand(admitted.maximum_copy_bytes).unwrap(),demand);
        if demand.release_bytes>8 {let denied=RetainedCloneGrant {maximum_release_bytes:8,..admitted};let(result,heap)=observe(||source.close_step(denied));check_denied(result,heap,denied,demand);}
        let(result,heap)=observe(||source.close_step(admitted));let progress=check(result,heap,admitted);*born+=heap.0;*freed+=heap.1;stalled=if progress==Default::default(){stalled+1}else{0};assert!(stalled<=143,"Original dynamic source closure stalled copy={copy} demand={demand:?}");
    }
    panic!("Original dynamic source graph remained live");
}

pub(super) fn cases()->Vec<(String,serde_json::Value)>{
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🌱️dynamic/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let mut cases=law["cases"].as_array().unwrap().iter().map(|row|(row["name"].as_str().unwrap().to_owned(),row["value"].clone())).collect::<Vec<_>>();
    for depth in law["depths"].as_array().unwrap(){let depth=depth.as_u64().unwrap()as usize;let mut value=serde_json::json!({"kind":"text","value":"ä日🌱"});for _ in 0..depth{value=serde_json::json!({"kind":"array","value":[value]});}cases.push((format!("depth-{depth}"),value));}cases
}
pub(super) fn admit_source(owner:DslValue)->(semio_framework_value_clone_source_testing::FixtureSource<DslValue>,(usize,usize)){
    use semio_framework_value_clone_source_testing::FixtureSourceBirth;
    semio_framework_value_clone_source_testing::observe_fixture_source_allocations(||RetainedCloneSource::from_owner(owner))
}
pub(super) struct Advanced{pub complete:bool,pub turns:usize,pub born:usize,pub freed:usize}
pub(super) fn advance_original(source:&RetainedCloneSource<DslValue>,cursor:&mut DslValueRetainedCloneCursor,cancelled:Option<usize>,mut binding:impl FnMut(&DslValueRetainedCloneCursor,bool))->Advanced{
    let root=source.borrow().get()as*const DslValue as usize;let mut actual=Advanced{complete:false,turns:0,born:0,freed:0};
    for _ in 0..100000{
        if cancelled==Some(actual.turns){break;}
        let(demand,heap)=observe(||cursor.demands(source.borrow(),64,MAXIMUM_CLONE_WORK).unwrap());assert_eq!(heap,(0,0));let admitted=grant(demand,64);
        for denied in denied(admitted,demand){binding(cursor,true);let(result,heap)=observe(||cursor.advance(source.borrow(),64,MAXIMUM_CLONE_WORK,denied));check_denied(result,heap,denied,demand);binding(cursor,false);assert_eq!(cursor.demands(source.borrow(),64,MAXIMUM_CLONE_WORK).unwrap(),demand);assert_eq!(source.borrow().get()as*const DslValue as usize,root);}
        let(result,heap)=observe(||cursor.advance(source.borrow(),64,MAXIMUM_CLONE_WORK,admitted));let work=result.work;let done=matches!(&result.result,Ok(RetainedCloneStep::Complete(_)));let progress=check(result,heap,admitted);assert!(work>0);assert_eq!(progress.copied_items,1);actual.born+=heap.0;actual.freed+=heap.1;actual.turns+=1;if done{actual.complete=true;break;}
    }actual
}
