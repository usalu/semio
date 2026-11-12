//! 🧪️ Actual native reservation work follows neutral page frontiers under finite original work and construction loans.
use super::*;
use semio_framework_value_testing_admission as admission;
use semio_framework_value_control_contract::RetainedWorkPoll;
use semio_framework_trace_memory_testing::{RequestedAllocator,observe_backing};
use semio_framework_value_testing_allocator as _;
struct Bank{remaining:RetainedCloneGrant,submitted:RetainedCloneProgress}
impl Bank{
 fn offer(&self)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:self.remaining.maximum_items.min(1),..self.remaining}}
 fn accept(&mut self,p:RetainedCloneProgress){assert!(p.fits(self.remaining));self.submitted=self.submitted.checked_add(p).unwrap();self.remaining.maximum_items-=p.copied_items;self.remaining.maximum_copy_bytes-=p.copied_bytes;self.remaining.maximum_capacity_bytes-=p.retained_capacity_bytes;self.remaining.maximum_release_bytes-=p.released_bytes;}
}
struct Turn<'a,'c>{payer:&'a mut admission::Authorities,control:&'a mut NativeDecodeControl<'c>}
impl Turn<'_,'_>{
 fn submitted(&self)->RetainedCloneProgress{self.payer.submitted()}
 fn advance(&mut self,state:&mut NativePagedReservation,storage:&mut semio_framework_value_paged_list::PagedList<u64,4096>,index:usize,limit:usize,maximum:u64,offer:RetainedCloneGrant)->(RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>,usize,usize){
  let mut observed=None;
  let poll=self.payer.nested(self.control,offer,|control,_|{let(turn,born,released)=observe_backing(||state.advance(storage,index,limit,control,maximum,offer));observed=Some((born,released));RetainedWorkPoll::Complete(turn)});
  let RetainedWorkPoll::Complete(turn)=poll else{panic!("native reservation turn stayed pending")};let(born,released)=observed.expect("reservation turn ran");(turn,born,released)
 }
 fn close(&mut self,state:&mut NativePagedReservation,demand:RetirementDemand,grant:RetainedCloneGrant)->(RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>,usize,usize){
  let mut observed=None;let mut frame=NativeReceivingEffect::<OriginalRetirementFailure>::default();
  let poll=self.payer.nested(self.control,grant,|control,_|{let(turn,born,released)=observe_backing(||control.receive_retirement(demand,&mut frame,false,5,|g,_|{let turn=state.close_step(g);RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work}}));observed=Some((born,released));assert_eq!(frame.phase(),NativeReceivingEffectPhase::Empty);RetainedWorkPoll::Complete(turn)});
  let RetainedWorkPoll::Complete(turn)=poll else{panic!("native reservation close stayed pending")};let(born,released)=observed.expect("reservation close ran");(turn,born,released)
 }
}
#[path="../../../../../♻️retirement/📦️owner/🧪️work/effect.rs"]
mod effect;
fn fx<R>(grant:RetainedCloneGrant,run:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->R)->R{effect::with_effect(grant,1<<20,&mut None,run)}
fn reserve(list:&mut semio_framework_value_paged_list::PagedList<u64,4096>){
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:list.next_reserve_copy_byte_demand().unwrap(),maximum_capacity_bytes:list.next_allocation_bytes().unwrap(),maximum_release_bytes:0,maximum_depth:list.next_reserve_depth_demand().unwrap()};
 assert!(matches!(fx(grant,|effect|list.reserve_one(effect)).result.unwrap(),semio_framework_value_paged_list::PagedListReservationOutcome::Ready(_)));
}
fn seed(row:&serde_json::Value)->semio_framework_value_paged_list::PagedList<u64,4096>{
 let mut list=semio_framework_value_paged_list::PagedList::with_page_items(1);
 for value in 0..row["rows"].as_u64().unwrap(){while !list.has_reserved_slot(){reserve(&mut list);}list.push_reserved(value).unwrap();}
 for _ in 0..row["extraReserveSteps"].as_u64().unwrap(){reserve(&mut list);}
 list
}
#[test]
fn native_page_parent_matches_neutral_walks_without_quoting_source_headers(){
 let f:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in f["cases"].as_array().unwrap(){
  let mut storage=seed(row);let index=row["rows"].as_u64().unwrap()as usize;let limit=row["limit"].as_u64().unwrap()as usize;
  let mut state=NativePagedReservation::default();let mut bank=Bank{remaining:RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64},submitted:Default::default()};let mut fuel=4096u64;let mut payer=admission::Authorities::new();let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut callback);let _=payer.nested(&mut control,bank.offer(),|control,_|{control.prepare_work(1).unwrap();control.commit_work(1).unwrap();RetainedWorkPoll::<RetainedCloneStep,OriginalReceivingPrimary>::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:1})});let mut run=Turn{payer:&mut payer,control:&mut control};let mut cursor=0;
  while state.demand(&storage,index,limit).unwrap().is_some(){
   assert!(cursor<row["work"].as_array().unwrap().len());let quote=state.work_demand(&storage,index,limit,fuel).unwrap();assert!(quote>0);let maximum=fuel;let offer=bank.offer();let before=(storage.len(),storage.capacity(),storage.allocated_bytes(),bank.submitted);let frontier=state.testkit_frontier();
   for denied in [0,quote-1]{let(turn,born,released)=run.advance(&mut state,&mut storage,index,limit,denied,offer);assert_eq!((born,released),(0,0));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!((storage.len(),storage.capacity(),storage.allocated_bytes(),bank.submitted),before);assert_eq!(state.testkit_frontier(),frontier);}
   let(turn,born,released)=run.advance(&mut state,&mut storage,index,limit,maximum,offer);let p=turn.result.unwrap().progress();assert_eq!((born,released),(p.retained_capacity_bytes,p.released_bytes));assert_eq!(turn.work,u128::from(row["work"][cursor].as_u64().unwrap()));assert!(turn.work<=u128::from(quote));fuel=fuel.checked_sub(u64::try_from(turn.work).unwrap()).unwrap();assert!(!state.has_pending_recovery());bank.accept(p);assert_eq!(bank.submitted,before.3.checked_add(p).unwrap());assert_eq!(run.submitted(),bank.submitted);cursor+=1;
  }
  assert_eq!(cursor,row["work"].as_array().unwrap().len());assert!(state.demand(&storage,index,limit).unwrap().is_none());assert_eq!(state.work_demand(&storage,index,limit,0).unwrap(),0);assert_eq!(state.work_demand(&storage,index+1,limit+1,fuel).unwrap(),4);
  let before=bank.submitted;let maximum=fuel;let offer=bank.offer();let(cached,born,released)=run.advance(&mut state,&mut storage,index,limit,maximum,offer);assert_eq!(cached.work,0);assert_eq!(cached.result.unwrap().progress(),Default::default());assert_eq!((born,released),(0,0));assert_eq!(bank.submitted,before);assert_eq!(run.submitted(),bank.submitted);
  let denied=state.close_step(Default::default());assert_eq!(denied.work,0);assert!(!state.terminal_is_empty());assert_eq!(state.close_work_demand().unwrap(),5);let demand=state.close_demand().unwrap();let(closed,born,released)=run.close(&mut state,demand,offer);assert_eq!(closed.work,5);let p=closed.result.unwrap();assert_eq!((born,released),(p.retained_capacity_bytes,p.released_bytes));bank.accept(p);assert_eq!(bank.submitted,before.checked_add(p).unwrap());assert_eq!(run.submitted(),bank.submitted);fuel-=5;assert!(state.terminal_is_empty());
  while !storage.is_empty(){storage.truncate_retired_last().unwrap();}while !storage.terminal_is_empty(){let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:storage.next_release_copy_byte_demand().unwrap(),maximum_capacity_bytes:0,maximum_release_bytes:storage.allocated_bytes(),maximum_depth:storage.next_release_depth_demand().unwrap()};assert!(matches!(fx(grant,|effect|storage.release_empty_page(effect)).result.unwrap(),semio_framework_value_paged_list::PagedListReleaseOutcome::Ready(_)));}
 }
}
#[test]
fn native_page_parent_rejects_active_context_changes_without_page_work(){
 let mut storage=semio_framework_value_paged_list::PagedList::<u64,4096>::empty();let mut state=NativePagedReservation::testkit_resume(storage.inspect_selection_header(1).result.unwrap(),Some((0,1)));let empty=semio_framework_value_paged_list::PagedList::<u64,4096>::with_page_items(1);assert_eq!(state.demand(&storage,0,1).unwrap(),state.demand(&empty,0,1).unwrap());assert!(state.work_demand(&storage,1,2,100).is_err());assert!(state.demand(&storage,0,2).is_err());assert_eq!(state.work_demand(&storage,0,1,0).unwrap(),0);let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_depth:64,..Default::default()};assert!(state.close_step(g).result.is_ok());assert!(state.terminal_is_empty());assert!(storage.terminal_is_empty());
}
