const MAXIMUM_CLONE_WORK:u64=4096;
use semio_framework_test_fixtures::fixture_text;
use super::*;
use crate::{Number,observe_retirement_allocations as observe};
use crate::retained_clone::effect;
use semio_framework_value_clone_source_testing::{FixtureSource,FixtureSourceBirth,FixtureCloseStep,observe_fixture_source_allocations};
#[path="../../🧬️contract/🔤️string-cursor/🧪️tests/producer.rs"] mod producer;
pub(super) use producer::{Harness,Op};
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


pub(super) fn check(step:RetainedCloneStep,heap:(usize,usize),grant:RetainedCloneGrant)->RetainedCloneProgress{let progress=step.progress();assert!(progress.fits(grant));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));progress}
fn run(source:&mut FixtureSource<DslValue>,cursor:&mut Harness<DslValue>,op:Op,body:usize,g:RetainedCloneGrant)->(RetainedCloneStep,(usize,usize)){let maximum=source.wallet().remaining_work();let(_,work)=cursor.quote(source,op,body,maximum);cursor.drive(source,op,body,maximum,g,work)}
fn deny(source:&mut FixtureSource<DslValue>,cursor:&mut Harness<DslValue>,op:Op,body:usize,g:RetainedCloneGrant){let before=source.wallet().remaining_work();let(step,heap)=run(source,cursor,op,body,g);assert_eq!(source.wallet().remaining_work(),before);assert_eq!(heap,(0,0));assert_eq!(step.progress(),Default::default());}
pub(super) fn close_cursor(source:&mut FixtureSource<DslValue>,cursor:&mut Harness<DslValue>,copy:usize,born:&mut usize,freed:&mut usize){
 let maximum=source.wallet().remaining_work();let(d,_)=cursor.quote(source,Op::Start,0,maximum);let(step,heap)=run(source,cursor,Op::Start,0,grant(d,0));check(step,heap,grant(d,0));*born+=heap.0;*freed+=heap.1;let mut stalled=0;
 for _ in 0..100000{if cursor.terminal(){return}let maximum=source.wallet().remaining_work();let(demand,heap)=observe(||cursor.quote(source,Op::Close,copy,maximum));assert_eq!(heap,(0,0));let demand=demand.0;let admitted=grant(demand,copy);
  for denied in denied(admitted,demand){deny(source,cursor,Op::Close,copy,denied);assert_eq!(cursor.quote(source,Op::Close,copy,maximum).0,demand);}
  if demand.release_bytes>8{deny(source,cursor,Op::Close,copy,RetainedCloneGrant{maximum_release_bytes:8,..admitted});}
  let(step,heap)=run(source,cursor,Op::Close,copy,admitted);let progress=check(step,heap,admitted);*born+=heap.0;*freed+=heap.1;stalled=if progress==Default::default(){stalled+1}else{0};assert!(stalled<=143,"Original dynamic cursor closure stalled copy={copy} demand={demand:?}");
 }panic!("Original dynamic cursor did not close its retained original graph");
}
pub(super) fn close_output(source:&mut FixtureSource<DslValue>,cursor:&mut Harness<DslValue>,copy:usize,born:&mut usize,freed:&mut usize){
 for _ in 0..100000{if !cursor.has_output()&&cursor.output_close_empty(){return}let op=if cursor.has_output(){Op::OutputStart}else{Op::OutputClose};let maximum=source.wallet().remaining_work();let(d,_)=cursor.quote(source,op,copy,maximum);let g=grant(d,copy);let(step,heap)=run(source,cursor,op,copy,g);check(step,heap,g);*born+=heap.0;*freed+=heap.1;}panic!("Original dynamic output graph remained live");
}
pub(super) fn close_source(source:&mut FixtureSource<DslValue>,copy:usize,born:&mut usize,freed:&mut usize){
 let(mut stalled,mut observed)=(0,(0usize,0usize));for _ in 0..100000{if source.terminal_is_empty(){return}let d=source.next_close_demand(copy).unwrap();let admitted=grant(d,0);assert_eq!(source.next_close_demand(admitted.maximum_copy_bytes).unwrap(),d);
  if d.release_bytes>8{let before=source.wallet().remaining_work();let(state,heap)=observe_fixture_source_allocations(||source.close_with(copy,RetainedCloneGrant{maximum_release_bytes:8,..admitted}).unwrap());assert_eq!(heap,(0,0));assert_eq!(source.wallet().remaining_work(),before);assert!(match state{FixtureCloseStep::Denied=>true,FixtureCloseStep::Progress(p)=>p==Default::default(),_=>false});}
  let(state,heap)=observe_fixture_source_allocations(||source.close_with(copy,admitted).unwrap());*born+=heap.0;*freed+=heap.1;observed.0+=heap.0;observed.1+=heap.1;let progress=match state{FixtureCloseStep::Pending=>continue,FixtureCloseStep::Progress(p)=>p,FixtureCloseStep::Complete=>Default::default(),FixtureCloseStep::Denied|FixtureCloseStep::Blocked=>panic!("Original Source exact quote was denied")};assert!(progress.fits(admitted));assert_eq!(observed,(progress.retained_capacity_bytes,progress.released_bytes));observed=(0,0);stalled=if progress==Default::default(){stalled+1}else{0};assert!(stalled<=143,"Original dynamic source closure stalled copy={copy} demand={d:?}");
 }panic!("Original dynamic source graph remained live");
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
pub(super) fn advance_original(source:&mut FixtureSource<DslValue>,cursor:&mut Harness<DslValue>,cancelled:Option<usize>,mut binding:impl FnMut(&DslValueRetainedCloneCursor,bool))->Advanced{
 let root=source.borrow().get()as*const DslValue as usize;let mut actual=Advanced{complete:false,turns:0,born:0,freed:0};
 for _ in 0..100000{if cancelled==Some(actual.turns){break}let maximum=source.wallet().remaining_work().min(MAXIMUM_CLONE_WORK);let(demand,heap)=observe(||cursor.quote(source,Op::Advance,64,maximum));assert_eq!(heap,(0,0));let demand=demand.0;let admitted=grant(demand,64);
  for denied in denied(admitted,demand){cursor.inspect_cursor(|c|binding(c,true));let before=source.wallet().remaining_work();let(_,work)=cursor.quote(source,Op::Advance,64,maximum);let(step,heap)=cursor.drive(source,Op::Advance,64,maximum,denied,work);assert_eq!(heap,(0,0));assert_eq!(step.progress(),Default::default());assert_eq!(source.wallet().remaining_work(),before);cursor.inspect_cursor(|c|binding(c,false));assert_eq!(cursor.quote(source,Op::Advance,64,maximum).0,demand);assert_eq!(source.borrow().get()as*const DslValue as usize,root);}
  let(_,work)=cursor.quote(source,Op::Advance,64,maximum);let before=source.wallet().remaining_work();let(step,heap)=cursor.drive(source,Op::Advance,64,maximum,admitted,work);let progress=check(step,heap,admitted);assert!(source.wallet().remaining_work()<before,"dynamic paid turn stalled: variant={:?}, turn={}, quotedWork={}, before={}, ready={}, phase={}, child={:?}",std::mem::discriminant(source.borrow().get()),actual.turns,work,before,cursor.ready(),cursor.inspect_cursor(|c|c.state.phase),cursor.inspect_cursor(|c|c.state.child.as_ref().map(|v|(v.ready(),v.has_value()))));assert_eq!(progress.copied_items,1);actual.born+=heap.0;actual.freed+=heap.1;actual.turns+=1;if matches!(step,RetainedCloneStep::Complete(_)){actual.complete=true;break}
 }actual
}
pub(super) fn drive(source:&mut FixtureSource<DslValue>,cursor:&mut Harness<DslValue>,op:Op,body:usize,g:RetainedCloneGrant)->(RetainedCloneStep,(usize,usize)){run(source,cursor,op,body,g)}
