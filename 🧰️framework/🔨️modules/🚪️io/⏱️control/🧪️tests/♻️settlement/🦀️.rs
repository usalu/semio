//! 🧪️ Snapshot callback and unwind custody survives typed retirement and same-original recovery.
use semio_framework_io_control::{NativeSnapshotEncodeOwner,NativeSnapshotEncodeSettlement};
use semio_framework_value::{ErasedSnapshotRetirement,RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,ValueError,retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement}};
use semio_framework_value::native_encoding::*;
use std::{sync::{Arc,Mutex},panic::{catch_unwind,AssertUnwindSafe}};
#[global_allocator]
static ALLOCATOR:semio_framework_trace_memory_testing::RequestedAllocator=semio_framework_trace_memory_testing::RequestedAllocator;
use semio_framework_trace_memory_testing::observe_backing as observe;
type Cursor=<NativeSnapshotEncodeSettlement as RetireOwned>::Retirement;
fn close<T:RetireOwned>(value:T,bank:&mut RetainedCloneGrant,fuel:&mut u64){
 let mut owner=ControlledRetirement::retain(value);
 for _ in 0..4096{if owner.terminal_is_empty(){return}let quote=owner.next_computational_work(65536).unwrap();assert!(quote>0&&quote<=*fuel);let grant=RetainedCloneGrant{maximum_items:bank.maximum_items.min(1),..*bank};let mut actual=None;let(turn,born,released)=observe(||owner.step(grant,&mut actual));let p=turn.unwrap().progress();assert_eq!((born,released),(p.retained_capacity_bytes,p.released_bytes));assert_eq!(actual,if born==0{None}else{Some(born)});assert!(p.fits(grant));assert!(owner.last_turn_work()<=quote);*fuel-=owner.last_turn_work();bank.maximum_items-=p.copied_items;bank.maximum_copy_bytes-=p.copied_bytes;bank.maximum_capacity_bytes-=p.retained_capacity_bytes;bank.maximum_release_bytes-=p.released_bytes;}panic!("snapshot typed owner exceeded finite cleanup turns")
}
fn return_original(slot:&mut Option<NativeSnapshotEncodeSettlement>,bank:&mut RetainedCloneGrant,fuel:&mut u64){
 let identity=slot.as_ref().unwrap().original().unwrap().identity();let unwind=slot.as_ref().unwrap().has_operation_unwind();let(cursor,born,released)=observe(||slot.take().unwrap().retirement());assert_eq!((born,released),(0,0));let mut cursor=cursor;let mut actual=Some(73);let(turn,born,released)=observe(||cursor.close_step(*bank,&mut actual));assert_eq!((born,released),(0,0));assert_eq!(actual,Some(73));let failure=match turn{RetirementStep::Failure(failure,0)=>failure,_=>panic!("pending snapshot owner was consumed")};close(failure,bank,fuel);assert!(cursor.requires_original_control());
 let demand=Cursor::pending_return_demand().unwrap();let quote=Cursor::pending_return_work_demand().unwrap();assert!(quote<=*fuel&&demand.copy_bytes<=bank.maximum_copy_bytes&&bank.maximum_items>0);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_depth:1,..Default::default()};let(denied,born,released)=observe(||cursor.return_pending(slot,grant,quote-1));assert_eq!((born,released),(0,0));assert_eq!(denied.work,0);assert!(slot.is_none());let(turn,born,released)=observe(||cursor.return_pending(slot,grant,quote));assert_eq!((born,released),(0,0));let p=turn.result.unwrap().progress();assert_eq!(turn.work,quote);assert!(cursor.terminal_is_empty());bank.maximum_items-=p.copied_items;bank.maximum_copy_bytes-=p.copied_bytes;*fuel-=turn.work;assert_eq!(slot.as_ref().unwrap().original().unwrap().identity(),identity);assert_eq!(slot.as_ref().unwrap().has_operation_unwind(),unwind);
}
struct Work{remaining:u64,quote:u64,actual:u64,interrupt:bool,response:Arc<Mutex<Option<Result<(),ValueError>>>>}
impl NativeEncodeWorkRecipient for Work{
 fn admits(&mut self,_:u64,additional:u64)->Result<bool,ValueError>{assert_eq!(additional,self.quote);Ok(additional<=self.remaining)}
 fn committed(&mut self,actual:u64)->Result<(),ValueError>{assert_eq!(actual,self.actual);self.remaining=self.remaining.checked_sub(actual).unwrap();if self.interrupt{*self.response.lock().unwrap()=Some(Ok(()));panic!("actual original work post interrupted")}Ok(())}
}
struct Physical{remaining:RetainedCloneGrant,submitted:RetainedCloneProgress,interrupt:bool,response:Arc<Mutex<Option<Result<(),ValueError>>>>}
impl NativeEncodeConstructionRecipient for Physical{
 fn submitted(&self)->RetainedCloneProgress{self.submitted}
 fn grant(&mut self,demand:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{assert_eq!(demand.copy_bytes,3);Ok(self.remaining)}
 fn accept(&mut self,grant:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),ValueError>{assert!(p.fits(grant)&&p.fits(self.remaining));self.remaining.maximum_items-=p.copied_items;self.remaining.maximum_copy_bytes-=p.copied_bytes;self.submitted=self.submitted.checked_add(p)?;if self.interrupt{*self.response.lock().unwrap()=Some(Ok(()));panic!("actual original physical post interrupted")}Ok(())}
}
#[test]
fn snapshot_typed_settlement_preserves_original_callbacks_and_actual_unwind(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🌱️value/🛫️encode/♻️retirement/🧪️tests/🔣️.json")).unwrap();let policy:serde_json::Value=serde_json::from_str(include_str!("../🧾️receipt/cursor-effect.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap().iter().filter(|row|row["kind"]=="settlement"){
  let quote=row["selectedWork"].as_u64().unwrap();let actual=row["actualWork"].as_u64().unwrap();let interrupt_work=row["id"]=="settlement-work-pending";let interrupt_physical=row["id"]=="settlement-physical-pending";let mut fuel=policy["workGrant"].as_u64().unwrap();let mut bank=RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};let effect_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:3,maximum_depth:1,..Default::default()};assert!(quote<=fuel);bank.maximum_items-=effect_grant.maximum_items;bank.maximum_copy_bytes-=effect_grant.maximum_copy_bytes;fuel-=quote;let response=Arc::new(Mutex::new(None));let mut work=Work{remaining:quote,quote,actual,interrupt:interrupt_work,response:response.clone()};let mut physical=Physical{remaining:effect_grant,submitted:Default::default(),interrupt:interrupt_physical,response:response.clone()};let mut wi=NativeEncodeWorkIssuer::new();let mut pi=NativeEncodeConstructionIssuer::new();let mut callback=|_|true;let mut native=NativeEncodeControl::new(0,&mut callback);let mut slot=None;let mut output=[0u8;3];
  {let mut wl=wi.loan(&mut work);let mut pl=pi.loan(&mut physical);native.with_work_recipient(&mut wl,|native|native.with_construction_recipient(&mut pl,|native|{
   let mut owner=NativeSnapshotEncodeOwner::new(native,effect_grant);owner.begin_original_settlement(&mut slot,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:3,maximum_depth:1,..Default::default()},quote)?;assert_eq!(actual,output.len()as u64);for byte in &mut output{*byte=1;}let progress=RetainedCloneProgress{copied_items:1,copied_bytes:3,..Default::default()};slot.as_mut().unwrap().record_effect(progress,actual)?;
   if interrupt_physical{let payload=catch_unwind(||std::panic::panic_any(42usize)).unwrap_err();assert!(slot.as_mut().unwrap().retain_operation_unwind(payload).is_ok());assert_eq!(policy["unwindRetainsFrame"],true);}
   let result=catch_unwind(AssertUnwindSafe(||owner.settle_original_effect(&mut slot)));assert_eq!(result.is_err(),interrupt_work||interrupt_physical);assert_eq!(owner.progress(),progress);
   if row["pending"].as_bool().unwrap(){assert!(!slot.as_ref().unwrap().retirement_ready());return_original(&mut slot,&mut bank,&mut fuel);let mut result=response.lock().unwrap().take();owner.resolve_original_post(&mut slot,interrupt_work,&mut result)?;assert!(result.is_none());owner.settle_original_effect(&mut slot)?;}else{result.unwrap()?;}
   assert_eq!(owner.progress(),progress);
   if interrupt_physical{assert!(!slot.as_ref().unwrap().retirement_ready());let receipt=NativeSnapshotEncodeSettlement::take_receipt(&mut slot)?;close(receipt,&mut bank,&mut fuel);let resumed=catch_unwind(AssertUnwindSafe(||NativeSnapshotEncodeSettlement::resume_operation_unwind(&mut slot)));let payload=resumed.unwrap_err().downcast::<usize>().unwrap();assert_eq!(*payload,42);assert!(slot.is_none());}else{assert!(slot.as_ref().unwrap().retirement_ready());}
   Ok::<(),ValueError>(())
  })).unwrap();}
  assert_eq!(output,[1;3]);assert_eq!(native.actual_work(),actual);assert_eq!(work.remaining,row["unspentWork"].as_u64().unwrap());assert_eq!(physical.submitted.copied_items,1);assert_eq!(physical.submitted.copied_bytes,3);bank.maximum_items+=physical.remaining.maximum_items;bank.maximum_copy_bytes+=physical.remaining.maximum_copy_bytes;fuel+=work.remaining;if let Some(original)=slot.take(){close(original,&mut bank,&mut fuel);}
 }
}
