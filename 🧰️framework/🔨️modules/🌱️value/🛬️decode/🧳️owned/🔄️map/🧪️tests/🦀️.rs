use super::*;
use crate::{DslValue,retained_clone::{RetainedCloneSource,retained_clone_grant_admits},retirement::controlled::ControlledRetirement};
#[derive(crate::RetireOwned)]#[value(crate="crate")]
struct Wrapped(String);
impl NativeReceiveMap<String> for Wrapped{fn from_native(value:String)->Self{Self(value)}}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn actual<R>(run:impl FnOnce()->R)->(R,(usize,usize)){semio_framework_value_testing_allocator::observe_retirement_allocations(run)}
fn check(p:RetainedCloneProgress,h:(usize,usize),g:RetainedCloneGrant){assert!(p.fits(g));assert_eq!((p.retained_capacity_bytes,p.released_bytes),h)}
fn drain<T:RetireOwned>(value:T){let mut close=ControlledRetirement::new(value).map_err(|(error,_)|error).unwrap();for _ in 0..100000{if close.terminal_is_empty(){return}let g=grant(close.next_demand(4096).unwrap());let(step,h)=actual(||close.step(g).unwrap());check(step.progress(),h,g)}panic!("original native map cleanup stalled")}
fn check_mapping<I:crate::owned::tree::NativeValueSource+?Sized+'static>(project:for<'a>fn(RetainedCloneRef<'a,DslValue>)->RetainedCloneRef<'a,I>){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let text=law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap()as usize);let oracle:serde_json::Value=serde_json::from_str(&serde_json::to_string(&text).unwrap()).unwrap();
 for cut in law["cancelPrefixes"].as_array().unwrap().iter().map(|v|Some(v.as_u64().unwrap()as usize)).chain([None]){
  let source=DslValue::String(text.clone());let d=RetainedCloneSource::<DslValue>::owned_constructor_demand::<()>();let mut source=RetainedCloneSource::admit_owned(source,(),grant(RetirementDemand{copy_bytes:RetainedCloneSource::<DslValue>::constructor_copy_bytes(),capacity_bytes:d.capacity_bytes,depth:d.depth,..Default::default()})).map_err(|(error,..)|error).unwrap().0;
  let d=NativeMappedReceiver::<I,String,Wrapped>::birth().unwrap();let g=grant(d);let(mut receiver,p)=NativeMappedReceiver::<I,String,Wrapped>::begin(g).unwrap();assert!(p.fits(g));
  let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<24,&mut callback);let mut construction=crate::owned::OriginalConstructionFixture;
  for turn in 0..100000{if cut==Some(turn)||receiver.output_ready(){break}let d=receiver.demands(project(source.borrow()),4096).unwrap();let g=grant(d);
   for axis in 0..5{let mut denied=g;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes-=1,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes-=1,3 if d.release_bytes>0=>denied.maximum_release_bytes-=1,4 if d.depth>0=>denied.maximum_depth-=1,_=>continue}let(step,h)=actual(||control.with_construction_recipient(&mut construction,|control|receiver.advance(project(source.borrow()),control,4096,denied)).unwrap());check(step.progress(),h,denied);assert_eq!(step.progress(),Default::default());assert_eq!(receiver.demands(project(source.borrow()),4096).unwrap(),d);}
   let(step,h)=actual(||control.with_construction_recipient(&mut construction,|control|receiver.advance(project(source.borrow()),control,4096,g)).unwrap());check(step.progress(),h,g);assert!(step.progress()!=Default::default());assert!(turn<99999);
  }
  if cut.is_none(){assert!(receiver.output_ready());let d=receiver.take_demand().unwrap();let g=grant(d);if d.copy_bytes>0{let denied=RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g};let(output,h)=actual(||receiver.take(denied).unwrap());assert!(output.is_none());assert_eq!(h,(0,0));assert!(receiver.output_ready());}let((output,p),h)=actual(||receiver.take(g).unwrap().unwrap());check(p,h,g);assert_eq!(serde_json::Value::String(output.0.clone()),oracle);drain(output);}
  receiver.begin_close();for turn in 0..100000{if receiver.terminal_is_empty(){break}let d=receiver.close_demands(4096).unwrap();let g=grant(d);let(step,h)=actual(||receiver.close_step(g).unwrap());check(step.progress(),h,g);assert!(step.progress()!=Default::default());assert!(turn<99999);}assert!(receiver.terminal_is_empty());for _ in 0..100000{if source.terminal_is_empty(){break}let g=grant(source.next_close_demand(4096).unwrap());let(step,h)=actual(||source.close_step(g).unwrap());check(step.progress(),h,g);}assert!(source.terminal_is_empty());
 }
}

#[test]
fn native_mapping_preserves_completed_original_text_and_every_cancelled_child(){
 check_mapping::<DslValue>(|input|input);
 check_mapping::<dyn crate::owned::tree::NativeValueSource>(|input|input.project(0,|source|source as &dyn crate::owned::tree::NativeValueSource));
}
