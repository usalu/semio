use semio_framework_value_control_contract::ValueError;
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value_retirement::{RetireOwned,retirement::{RetirementCursor,RetirementStep}};
use semio_framework_value_control_contract::{OriginalEffectReply,OriginalEffectReplyComplete,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome};
use semio_framework_value_retained_clone::retained_clone::retained_clone_grant_admits;
use semio_framework_value_retirement::RetirementDemand;
use semio_framework_value_testing_allocator::observe_retirement_allocations;
use semio_framework_value_edit::*;
#[path="support.rs"] mod support;
mod actual;
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1024*1024,maximum_release_bytes:1024*1024,maximum_depth:32}}
fn array(law:&serde_json::Value,key:&str)->Vec<usize>{law[key].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as usize).collect()}
fn close_turn(plan:&mut IndexedEditPlan,issued:RetainedCloneGrant)->(RetainedCloneStep,(usize,usize)){let(answer,heap,_)=support::driver::close_turn(plan,IndexedEditPlan::new(plan.shape()),issued);(answer.unwrap(),heap)}
fn close(plan:&mut IndexedEditPlan)->(usize,usize){
 plan.begin_close();let mut born=0;let mut released=0;let mut turns=0;let shape=plan.shape();let limit=64+12*(shape.removed+shape.modified+shape.added);
 while !plan.terminal_is_empty(){
  let d=plan.close_demand(grant().maximum_copy_bytes).unwrap();let funded=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(plan.close_demand(funded.maximum_copy_bytes).unwrap(),d);
  let custody=plan.custody();
  let mut denied=vec![RetainedCloneGrant{maximum_items:0,..funded}];if d.copy_bytes>0{denied.push(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..funded});}if d.depth>0{denied.push(RetainedCloneGrant{maximum_depth:d.depth-1,..funded});}if d.capacity_bytes>0{denied.push(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..funded});}if d.release_bytes>0{denied.push(RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..funded});}
  for denied in denied{let selected=plan.close_demand(denied.maximum_copy_bytes).unwrap();if retained_clone_grant_admits(denied,selected){continue}let(step,heap)=close_turn(plan,denied);assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(plan.close_demand(funded.maximum_copy_bytes).unwrap(),d);assert_eq!(plan.custody(),custody);}
  let(step,heap)=close_turn(plan,funded);let p=step.progress();assert!(p.fits(funded));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;released+=heap.1;turns+=1;assert!(turns<limit);
 }
 (born,released)
}
/// 🧪️ Actual coordinate owners obey the neutral strict triple corpus and physical retirement receipts.
#[test]
fn indexed_edit_plan_matches_portable_coordinates_and_original_custody(){
 let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for law in fixture["cases"].as_array().unwrap(){
  let removed=array(law,"removed");let modified=array(law,"modified");let added=array(law,"added");let pointers=(removed.as_ptr(),modified.as_ptr(),added.as_ptr());
  let shape=IndexedEditShape{base_len:law["base"].as_u64().unwrap()as usize,removed:removed.len(),modified:modified.len(),added:added.len()};
  let(mut plan,heap)=observe_retirement_allocations(||IndexedEditPlan::new(shape));assert_eq!(heap,(0,0));let mut born=0;let mut turns=0;
  while !plan.settled(){
   let state=plan.probe();let demand=plan.demand().unwrap();
   for denied in[RetainedCloneGrant{maximum_items:0,..grant()},RetainedCloneGrant{maximum_copy_bytes:0,..grant()},RetainedCloneGrant{maximum_depth:0,..grant()}]{let(step,heap)=observe_retirement_allocations(||support::effect::settle(denied,|effect|plan.advance(effect,|_,_|panic!("denied source read"))).result.unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(state,plan.probe());}
   if demand.capacity_bytes>0{let(step,heap)=observe_retirement_allocations(||support::effect::settle(RetainedCloneGrant{maximum_capacity_bytes:0,..grant()},|effect|plan.advance(effect,|_,_|panic!("denied source read"))).result.unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));}
   let(step,heap)=observe_retirement_allocations(||support::effect::settle(grant(),|effect|plan.advance(effect,|kind,ordinal|RetainedWorkOutcome{result:Ok(match kind{IndexedEditKind::Removed=>removed[ordinal],IndexedEditKind::Modified=>modified[ordinal],IndexedEditKind::Added=>added[ordinal]}),work:1})).result.unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,0));born+=heap.0;turns+=1;assert!(turns<2000);
  }
  let expected=law["fault"].as_str().map(|kind|match kind{"remove"=>IndexedEditFault::Remove,"modify"=>IndexedEditFault::Modify,"add"=>IndexedEditFault::Add,_=>panic!("fixture fault")});assert_eq!(plan.fault(),expected);
  if expected.is_none(){
   let mut cursor=IndexedOutputCursor::default();let mut output=Vec::new();loop{let work=cursor.next(&plan).unwrap();match work{IndexedOutputWork::Original{index}=>output.push(format!("b{index}")),IndexedOutputWork::Added{ordinal}=>output.push(format!("a{ordinal}")),IndexedOutputWork::Removed{..}=>(),IndexedOutputWork::Complete=>break}cursor.advance(&plan).unwrap();}assert_eq!(serde_json::to_value(output).unwrap(),law["output"]);
  }
  assert_eq!(pointers,(removed.as_ptr(),modified.as_ptr(),added.as_ptr()));let(close_born,released)=close(&mut plan);assert_eq!(born+close_born,released);assert_eq!(observe_retirement_allocations(||drop(plan)).1,(0,0));
 }
}
/// ✂️ Large index directories remain resumable and close original storage at every selected cancellation frontier.
#[test]
fn indexed_edit_plan_wide_order_and_cancellation_remain_bounded(){
 let count=2049;let added:Vec<usize>=(0..count).rev().collect();let pointer=added.as_ptr();
 for cut in[Some(0),Some(1),Some(7),Some(count+12),Some(count*4),None]{
  let mut plan=IndexedEditPlan::new(IndexedEditShape{base_len:0,removed:0,modified:0,added:count});let mut born=0;let mut steps=0;
  while !plan.settled()&&cut.is_none_or(|cut|steps<cut){let(step,heap)=observe_retirement_allocations(||support::effect::settle(grant(),|effect|plan.advance(effect,|kind,ordinal|{assert_eq!(kind,IndexedEditKind::Added);RetainedWorkOutcome{result:Ok(added[ordinal]),work:1}})).result.unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,0));born+=heap.0;steps+=1;assert!(steps<count*32*12);}
  if cut.is_none(){assert!(plan.ready());let rows=plan.entries(IndexedEditKind::Added).unwrap();for(index,row)in rows.iter().enumerate(){assert_eq!((row.index,row.ordinal),(index,count-1-index));}assert_eq!(plan.output_len().unwrap(),count);}
  let(close_born,released)=close(&mut plan);assert_eq!(born+close_born,released);assert_eq!(added.as_ptr(),pointer);assert_eq!(observe_retirement_allocations(||drop(plan)).1,(0,0));
 }
}

/// 🔁️ Recovery is refused unless a child invocation is actually unfinished, so no close replays a settled child.
#[test]
fn indexed_edit_plan_close_refuses_recovery_without_an_unfinished_invocation(){
 let settled=||{let mut plan=IndexedEditPlan::new(IndexedEditShape{base_len:0,removed:0,modified:0,added:1});while !plan.settled(){support::effect::settle(grant(),|effect|plan.advance(effect,|_,_|RetainedWorkOutcome{result:Ok(0),work:1})).result.unwrap();}plan.begin_close();plan};
 let quote=|plan:&IndexedEditPlan|{let d=plan.close_demand(32768).unwrap();RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}};
 let plan=settled();let funded=quote(&plan);let body=funded.maximum_copy_bytes;
 let error=support::driver::Rig::new(plan).refused(funded,|plan,_,effect,reply|plan.recover_close_into(body,effect,reply));assert_eq!(error.kind,ValueRefusalKind::InvariantViolated);
 let mut rig=support::driver::Rig::new(settled());
 while !rig.terminal_is_empty(){let funded=quote(&rig);let body=funded.maximum_copy_bytes;let(answer,heap,work)=rig.turn(funded,|plan,_,effect,reply|plan.close_into(body,effect,reply));let p=answer.unwrap().progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert!(work>0);}
 let cached=rig.turn(Default::default(),|plan,_,effect,reply|plan.close_into(0,effect,reply));assert_eq!(cached.2,0);assert!(matches!(cached.0,Ok(RetainedCloneStep::Complete(p))if p==Default::default()));
}

static PENDS:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
static CALLS:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
static RECOVERIES:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
const PENDING_TURNS:usize=3;
/// 🧪️ A fixture owner whose close returns without recording its reply for a fixed number of recoveries before it completes.
struct Pends;
struct PendCursor{phase:u8,left:usize}
impl RetireOwned for Pends{type RetirementCursor=PendCursor;fn retirement_seed(self)->PendCursor{PendCursor{phase:0,left:PENDING_TURNS}}fn controlled_retirement_supported()->bool{true}}
impl RetirementCursor for PendCursor{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{Ok(if self.phase==2{Default::default()}else{RetirementDemand{copy_bytes:8+body.min(4),depth:1,..Default::default()}})}
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(if self.phase==2{0}else{2})}
 fn terminal_is_empty(&self)->bool{self.phase==2}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
 fn close_into<'r>(&mut self,_:usize,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{assert_eq!(self.phase,0);assert!(effect.performed(1)?);self.phase=1;CALLS.fetch_add(1,std::sync::atomic::Ordering::Relaxed);Ok(reply.pending())}
 fn recover_close_into<'r>(&mut self,_:usize,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{assert_eq!(self.phase,1);RECOVERIES.fetch_add(1,std::sync::atomic::Ordering::Relaxed);if self.left>0{self.left-=1;PENDS.fetch_add(1,std::sync::atomic::Ordering::Relaxed);return Ok(reply.pending())}assert!(effect.performed(1)?);self.phase=2;Ok(reply.complete(Ok(RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:12,..Default::default()},1))))}
}
impl Drop for PendCursor{fn drop(&mut self){assert!(self.phase==2||std::thread::panicking())}}
/// 🔁️ A close whose child returns Pending without a reply for N turns recovers exactly N times under the same invocation and records one reply with exact work.
#[test]
fn indexed_edit_plan_close_recovers_a_genuinely_pending_child_with_exact_counts(){
 let mut plan=IndexedEditPlan::new(IndexedEditShape{base_len:0,removed:0,modified:0,added:0});
 let mut rig=support::driver::Rig::new({plan.begin_close();plan});
 let adopt_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:8};
 let(answer,_,_)=rig.turn(adopt_grant,|plan,_,effect,reply|{let mut owner=Some(Pends);let adopted=plan.adopt(&mut owner,effect);Ok(reply.complete(adopted.map_err(Into::into)))});answer.unwrap();
 let body=64;let d=rig.close_demand(body).unwrap();let funded=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes.max(body),maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
 let mut pending=0;let mut finished=None;
 while !rig.terminal_is_empty(){
  pending=0;
  loop{
   let stepped=rig.step(funded,|plan,recover,effect,reply|if recover{plan.recover_close_into(body,effect,reply)}else{plan.close_into(body,effect,reply)});
   match stepped{None=>pending+=1,Some((answer,work))=>{finished=Some((answer.unwrap(),work));break}}
   assert!(pending<=PENDING_TURNS+1);
  }
  if pending>0{break}
 }
 assert_eq!(pending,PENDING_TURNS+1);
 assert_eq!(CALLS.load(std::sync::atomic::Ordering::Relaxed),1);
 assert_eq!(RECOVERIES.load(std::sync::atomic::Ordering::Relaxed),PENDING_TURNS+1);
 assert_eq!(PENDS.load(std::sync::atomic::Ordering::Relaxed),PENDING_TURNS);
 let(step,work)=finished.unwrap();assert!(step.progress().fits(funded));assert_eq!(work,1);
 while !rig.terminal_is_empty(){let d=rig.close_demand(body).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes.max(body),maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};rig.turn(g,|plan,_,effect,reply|plan.close_into(body,effect,reply)).0.unwrap();}
}
