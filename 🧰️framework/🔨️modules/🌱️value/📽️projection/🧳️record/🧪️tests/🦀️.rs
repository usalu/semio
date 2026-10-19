//! 🧪️ A paid active field projects wide records and closes every partial original without payload clones.
use semio_framework_test_fixtures::fixture_text;
use super::*;
macro_rules! wide_record{($($field:ident),*)=>{
 #[derive(crate::RetireOwned,crate::SnapshotProjection)]
 struct WideRecord{$($field:SealedShared<String>,)*}
 impl WideRecord{
  fn new(values:&serde_json::Value)->Self{let mut values=values.as_array().unwrap().iter();Self{$($field:{let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:SealedShared::<String>::birth_bytes(),maximum_depth:1,..Default::default()};SealedShared::admit(values.next().unwrap().as_str().unwrap().to_owned(),SharedIssuer::owned(),grant).map_err(|(error,_)|error).unwrap().0},)*}}
  fn roots(&self)->[&SealedShared<String>;32]{[$(&self.$field,)*]}
 }
};}
wide_record!(f00,f01,f02,f03,f04,f05,f06,f07,f08,f09,f10,f11,f12,f13,f14,f15,f16,f17,f18,f19,f20,f21,f22,f23,f24,f25,f26,f27,f28,f29,f30,f31);
#[derive(crate::RetireOwned,crate::SnapshotProjection)]
struct NestedRecord{left:WideRecord,right:WideRecord}
fn exact(d:RetirementDemand)->RetainedCloneGrant{assert!(d.copy_bytes<=4096&&d.capacity_bytes<=4096,"original record frontier exceeds actual 4096 copy/capacity allowance: {d:?}");funded(d)}
fn check(p:crate::retained_clone::RetainedCloneProgress,g:RetainedCloneGrant,heap:(usize,usize)){assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));}
fn close<T:crate::retirement::RetireOwned>(value:T){let mut cursor=ControlledRetirement::new(value).map_err(|(error,_)|error).unwrap();for _ in 0..100000{if cursor.terminal_is_empty(){return;}let g=exact(cursor.next_demand(4096).unwrap());let(step,heap)=observe_retirement_allocations(||cursor.step(g).unwrap());check(step.progress(),g,heap);}panic!("wide original record owner did not close");}
fn run<S:SnapshotProjection>(original:S,cut:usize,verify:impl Fn(&S,&S)){
 let d=RetainedCloneSource::<S>::owned_constructor_demand::<()>();let g=exact(RetirementDemand{copy_bytes:RetainedCloneSource::<S>::constructor_copy_bytes(),capacity_bytes:d.capacity_bytes,depth:d.depth,..Default::default()});let(source,_)=RetainedCloneSource::admit_owned(original,(),g).map_err(|(error,..)|error).unwrap();
 let g=exact(S::projection_birth_demand());let((mut cursor,p),heap)=observe_retirement_allocations(||S::begin_projection(g).unwrap());check(p,g,heap);
 for _ in 0..cut{if cursor.ready(){break;}let d=cursor.demands(source.borrow(),4096).unwrap();let g=exact(d);for axis in 0..5{let mut denied=g;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes-=1,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes-=1,3 if d.release_bytes>0=>denied.maximum_release_bytes-=1,4 if d.depth>0=>denied.maximum_depth-=1,_=>continue};let(step,heap)=observe_retirement_allocations(||cursor.advance(source.borrow(),denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(cursor.demands(source.borrow(),4096).unwrap(),d);}let(step,heap)=observe_retirement_allocations(||cursor.advance(source.borrow(),g).unwrap());check(step.progress(),g,heap);assert_ne!(step.progress(),Default::default());}
 if cursor.ready(){let g=exact(cursor.take_demand().unwrap());let(result,heap)=observe_retirement_allocations(||cursor.take(g).unwrap());let(output,p)=result.unwrap();check(p,g,heap);verify(source.borrow().get(),&output);close(output);}else{assert!(cut<10000,"fully funded original record must complete");}
 cursor.begin_close();for _ in 0..100000{if cursor.terminal_is_empty(){break;}let d=cursor.close_demands(4096).unwrap();let g=exact(d);for axis in 0..5{let mut denied=g;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes-=1,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes-=1,3 if d.release_bytes>0=>denied.maximum_release_bytes-=1,4 if d.depth>0=>denied.maximum_depth-=1,_=>continue};let(step,heap)=observe_retirement_allocations(||cursor.close_step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(cursor.close_demands(4096).unwrap(),d);}let(step,heap)=observe_retirement_allocations(||cursor.close_step(g).unwrap());check(step.progress(),g,heap);assert!(step.progress()!=Default::default()||cursor.terminal_is_empty());}assert!(cursor.terminal_is_empty());assert_eq!(observe_retirement_allocations(||drop(cursor)).1,(0,0));close(source);
}
fn verify(original:&WideRecord,output:&WideRecord,expected:&serde_json::Value){let values:Vec<_>=output.roots().iter().map(|root|root.get().as_str()).collect();assert_eq!(serde_json::to_value(values).unwrap(),*expected);for(before,after)in original.roots().into_iter().zip(output.roots()){assert_eq!(before.identity(),after.identity());assert_eq!(before.get().as_ptr(),after.get().as_ptr());}}
#[test]
fn original_record_projection_stages_one_paid_field_and_preserves_all_cancellation_prefixes(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../📽️projection/🧳️record/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();for law in corpus["cases"].as_array().unwrap(){for cut in corpus["cancelCuts"].as_array().unwrap(){let cut=cut.as_u64().unwrap()as usize;run(WideRecord::new(&law["values"]),cut,|original,output|verify(original,output,&law["expected"]));run(NestedRecord{left:WideRecord::new(&law["values"]),right:WideRecord::new(&law["values"])},cut,|original,output|{verify(&original.left,&output.left,&law["expected"]);verify(&original.right,&output.right,&law["expected"]);});}}
 println!("[DEBUG] original 32-field and nested record projection: 2 portable outputs x 8 cancellation cuts, every native copy/capacity frontier<=4096, original backing and all physical receipts preserved");
}
