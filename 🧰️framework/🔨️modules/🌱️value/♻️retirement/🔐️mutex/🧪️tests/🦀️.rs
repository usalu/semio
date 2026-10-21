use super::*;
use crate::allocation_testing::observe_retirement_allocations as observe;
use semio_framework_test_fixtures::fixture_text;

#[test]
fn original_mutex_owned_and_poisoned_payloads_preserve_exact_heap_custody(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../🔐️mutex/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let ((),probe)=observe(||{let probe=std::sync::Mutex::new(0u8);drop(probe.lock());drop(probe);});let lock_backing=probe.0;
 for row in corpus["cases"].as_array().unwrap(){
  let original=row["owner"].as_str().unwrap().to_owned();let pointer=original.as_ptr();let bytes=original.capacity();let mutex=std::sync::Mutex::new(original);
  if row["poisoned"].as_bool().unwrap(){let _=std::panic::catch_unwind(||{let _guard=mutex.lock().unwrap();panic!("test-only poison original owned mutex");});}
  assert_eq!(mutex.is_poisoned(),row["poisoned"].as_bool().unwrap());let (result,heap)=observe(||ControlledRetirement::new(mutex));assert_eq!(heap,(0,0));let mut owner=result.unwrap_or_else(|_|panic!("original mutex retirement refused"));
  assert_eq!(owner.original().unwrap().lock().unwrap_or_else(|error|error.into_inner()).as_ptr(),pointer);
  let mut born=0;let mut released=0;
  for _ in 0..512{if owner.terminal_is_empty(){break;}let copy=owner.next_copy_byte_demand().unwrap().max(1);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
   for axis in 0..5{let mut denied=grant;let active=match axis{0=>{denied.maximum_items=0;true},1=>{let demand=owner.next_copy_byte_demand().unwrap();denied.maximum_copy_bytes=demand.saturating_sub(1);demand>0},2=>{denied.maximum_capacity_bytes=grant.maximum_capacity_bytes.saturating_sub(1);grant.maximum_capacity_bytes>0},3=>{denied.maximum_release_bytes=grant.maximum_release_bytes.saturating_sub(1);grant.maximum_release_bytes>0},_=>{denied.maximum_depth=grant.maximum_depth.saturating_sub(1);grant.maximum_depth>0}};if active{let(result,heap)=observe(||owner.step(denied));assert_eq!(heap,(0,0));if let Ok(step)=result{assert_eq!(step.progress(),Default::default());}}}
   let held=owner.original().is_some();let(step,heap)=observe(||owner.step(grant).unwrap());let platform=if held&&owner.original().is_none(){lock_backing}else{0};assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes+platform));born+=heap.0;released+=heap.1;
  }
  assert!(owner.terminal_is_empty());assert_eq!(released,bytes+born+lock_backing);assert_eq!(observe(||drop(owner)).1,(0,0));assert_eq!(serde_json::json!({"preserved":true,"terminal":true}),row["expected"]);
 }
}

struct SendOnlyCell(std::cell::Cell<usize>);
struct CellCursor(Option<std::cell::Cell<usize>>);
impl RetireOwned for SendOnlyCell{fn retirement(self)->Box<dyn RetirementCursor>{Box::new(CellCursor(Some(self.0)))}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<CellCursor>())}fn controlled_retirement_supported()->bool{true}}
impl RetirementCursor for CellCursor{fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,ValueError>{Ok(crate::RetirementDemand{depth:usize::from(self.0.is_some()),..Default::default()})}fn close_step(&mut self,g:RetainedCloneGrant)->RetirementStep{if g.maximum_items==0||g.maximum_depth==0{return RetirementStep::BudgetExhausted;}self.0.take();RetirementStep::Complete}fn terminal_is_empty(&self)->bool{self.0.is_none()}fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}fn next_close_byte_demand(&self)->Option<usize>{Some(0)}fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}}
impl Drop for CellCursor{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"cell cursor requires funded terminal closure");}}

#[test]
fn send_only_original_cursor_retires_through_exclusive_cross_thread_gate(){
 use super::super::controlled::RetainedOwnerGate;
 fn require_send<T:Send>(){}fn require_shared<T:Send+Sync>(){}
 require_send::<CellCursor>();require_send::<ControlledRetirement<SendOnlyCell>>();require_shared::<RetainedOwnerGate<ControlledRetirement<SendOnlyCell>>>();
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../🔐️mutex/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for row in corpus["exclusiveGate"]["cases"].as_array().unwrap(){let value=row["value"].as_u64().unwrap()as usize;let(owner,heap)=observe(||ControlledRetirement::new(SendOnlyCell(std::cell::Cell::new(value))).ok().unwrap());assert_eq!(heap,(0,0));let gate=RetainedOwnerGate::new(owner);let mut held=None;let mut accepted=Vec::new();
  for event in row["events"].as_array().unwrap(){accepted.push(if event=="acquire"{match gate.try_lock(){Ok(guard)=>{assert_eq!(guard.original().unwrap().0.get(),value);held=Some(guard);true},Err(_)=>false}}else if let Some(guard)=held.take(){drop(guard);true}else{false});}
  assert_eq!(serde_json::json!(accepted),row["accepted"]);assert!(held.is_none());drop(held);
  {let guard=gate.try_lock().unwrap();assert_eq!(guard.original().unwrap().0.get(),value);std::thread::scope(|scope|assert!(scope.spawn(||gate.try_lock().is_err()).join().unwrap()));drop(guard);}
  std::thread::scope(|scope|scope.spawn(||{let mut owner=gate.try_lock().unwrap();let(mut born,mut released)=(0,0);for _ in 0..128{if owner.terminal_is_empty(){break;}let d=owner.next_demand(0).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let mut denied=vec![RetainedCloneGrant{maximum_items:0,..grant}];if d.copy_bytes>0{denied.push(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..grant});}if d.capacity_bytes>0{denied.push(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..grant});}if d.release_bytes>0{denied.push(RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..grant});}if d.depth>0{denied.push(RetainedCloneGrant{maximum_depth:d.depth-1,..grant});}
   for g in denied{let(result,heap)=observe(||owner.step(g));assert_eq!(heap,(0,0));if let Ok(step)=result{assert_eq!(step.progress(),Default::default());}assert_eq!(owner.next_demand(0).unwrap(),d);}
   let(step,heap)=observe(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;
  }assert!(owner.terminal_is_empty());assert_eq!(born,released);}).join().unwrap());
  let owner=gate.into_inner();assert!(owner.terminal_is_empty());assert_eq!(observe(||drop(owner)).1,(0,0));assert_eq!(row["terminal"],true);
 }
}
