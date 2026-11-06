//! 🧪️ Actual native reservation work follows neutral page frontiers under finite original work and construction loans.
use super::*;
use native_decoding::{NativeConstructionRecipientIssuer,NativeDecodeConstructionRecipient,NativeDecodeWorkIssuer,NativeDecodeWorkRecipient};
use semio_framework_trace_memory_testing::{RequestedAllocator,observe_backing};
#[global_allocator]static ALLOCATOR:RequestedAllocator=RequestedAllocator;
struct Bank{remaining:RetainedCloneGrant,submitted:RetainedCloneProgress}
impl Bank{fn offer(&self)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:self.remaining.maximum_items.min(1),..self.remaining}}}
impl NativeDecodeConstructionRecipient for Bank{
 fn submitted(&self)->RetainedCloneProgress{self.submitted}
 fn grant(&mut self,_:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{Ok(self.offer())}
 fn accept(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),ValueError>{assert!(p.fits(g));self.submitted=self.submitted.checked_add(p)?;self.remaining.maximum_items-=p.copied_items;self.remaining.maximum_copy_bytes-=p.copied_bytes;self.remaining.maximum_capacity_bytes-=p.retained_capacity_bytes;self.remaining.maximum_release_bytes-=p.released_bytes;Ok(())}
}
struct Fuel{remaining:u64}
impl NativeDecodeWorkRecipient for Fuel{
 fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{self.remaining.checked_sub(observed).ok_or_else(invalid)}
 fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|work|work<=self.remaining))}
}
fn seed(row:&serde_json::Value)->semio_framework_value_paged_list::PagedList<u64,4096>{
 let mut list=semio_framework_value_paged_list::PagedList::with_page_items(1);
 for value in 0..row["rows"].as_u64().unwrap(){while !list.has_reserved_slot(){let bytes=list.next_allocation_bytes().unwrap();let mut actual=None;list.reserve_one(bytes,&mut actual).unwrap();}list.push_reserved(value).unwrap();}
 for _ in 0..row["extraReserveSteps"].as_u64().unwrap(){let bytes=list.next_allocation_bytes().unwrap();let mut actual=None;list.reserve_one(bytes,&mut actual).unwrap();}
 list
}
#[test]
fn native_page_parent_matches_neutral_walks_without_quoting_source_headers(){
 let f:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in f["cases"].as_array().unwrap(){
  let mut storage=seed(row);let index=row["rows"].as_u64().unwrap()as usize;let limit=row["limit"].as_u64().unwrap()as usize;
  let mut state=NativePagedReservation::default();let mut bank=Bank{remaining:RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64},submitted:Default::default()};let mut fuel=Fuel{remaining:4096};let mut construction=NativeConstructionRecipientIssuer::new();let mut work_source=NativeDecodeWorkIssuer::new();let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut callback);let mut cursor=0;
  while state.demand(&storage,index,limit).unwrap().is_some(){
   assert!(cursor<row["work"].as_array().unwrap().len());let quote=state.work_demand(&storage,index,limit,fuel.remaining).unwrap();assert!(quote>0);let maximum=fuel.remaining;let offer=bank.offer();let before=(storage.len(),storage.capacity(),storage.allocated_bytes(),bank.submitted);let frontier=(state.context,state.header.is_some(),state.selection.map(|s|s.demand()),state.finished,state.complete,state.effect.phase());
   for denied in [0,quote-1]{let mut recipient=construction.loan(&mut bank);let mut work=work_source.loan(&mut fuel);let(turn,born,released)=observe_backing(||control.with_work_recipient(&mut work,|control|control.with_construction_recipient(&mut recipient,|control|Ok::<_,ValueError>(state.advance(&mut storage,index,limit,control,denied,offer)))).unwrap());assert_eq!((born,released),(0,0));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());drop(work);drop(recipient);assert_eq!((storage.len(),storage.capacity(),storage.allocated_bytes(),bank.submitted),before);assert_eq!((state.context,state.header.is_some(),state.selection.map(|s|s.demand()),state.finished,state.complete,state.effect.phase()),frontier);}
   let mut recipient=construction.loan(&mut bank);let mut work=work_source.loan(&mut fuel);let(turn,born,released)=observe_backing(||control.with_work_recipient(&mut work,|control|control.with_construction_recipient(&mut recipient,|control|Ok::<_,ValueError>(state.advance(&mut storage,index,limit,control,maximum,offer)))).unwrap());let p=turn.result.unwrap().progress();assert_eq!((born,released),(p.retained_capacity_bytes,p.released_bytes));assert_eq!(turn.work,row["work"][cursor].as_u64().unwrap());assert!(turn.work<=quote);drop(work);drop(recipient);fuel.remaining=fuel.remaining.checked_sub(turn.work).unwrap();assert!(!work_source.has_pending_work());assert!(!state.has_pending_recovery());assert_eq!(bank.submitted,before.3.checked_add(p).unwrap());cursor+=1;
  }
  assert_eq!(cursor,row["work"].as_array().unwrap().len());assert!(state.demand(&storage,index,limit).unwrap().is_none());assert_eq!(state.work_demand(&storage,index,limit,0).unwrap(),0);assert_eq!(state.work_demand(&storage,index+1,limit+1,fuel.remaining).unwrap(),4);
  let before=bank.submitted;let maximum=fuel.remaining;let offer=bank.offer();let mut recipient=construction.loan(&mut bank);let mut work=work_source.loan(&mut fuel);let(cached,born,released)=observe_backing(||control.with_work_recipient(&mut work,|control|control.with_construction_recipient(&mut recipient,|control|Ok::<_,ValueError>(state.advance(&mut storage,index,limit,control,maximum,offer)))).unwrap());assert_eq!(cached.work,0);assert_eq!(cached.result.unwrap().progress(),Default::default());assert_eq!((born,released),(0,0));drop(work);drop(recipient);assert_eq!(bank.submitted,before);
  let denied=state.close_step(Default::default());assert_eq!(denied.work,0);assert!(!state.terminal_is_empty());assert_eq!(state.close_work_demand().unwrap(),5);let demand=state.close_demand().unwrap();let mut close_frame=NativeReceivingEffect::<OriginalRetirementFailure>::default();let mut recipient=construction.loan(&mut bank);let mut work=work_source.loan(&mut fuel);let(closed,born,released)=observe_backing(||control.with_work_recipient(&mut work,|control|control.with_construction_recipient(&mut recipient,|control|Ok::<_,ValueError>(control.receive_retirement(demand,&mut close_frame,false,5,|g,_|state.close_step(g))))).unwrap());assert_eq!(closed.work,5);let p=closed.result.unwrap();assert_eq!((born,released),(p.retained_capacity_bytes,p.released_bytes));drop(work);drop(recipient);assert_eq!(bank.submitted,before.checked_add(p).unwrap());fuel.remaining-=closed.work;assert!(state.terminal_is_empty());assert_eq!(close_frame.phase(),NativeReceivingEffectPhase::Empty);assert!(!work_source.has_pending_work());
  while !storage.is_empty(){storage.truncate_retired_last().unwrap();}while !storage.terminal_is_empty(){storage.release_empty_page(usize::MAX).unwrap();}
 }
}
#[test]
fn native_page_parent_rejects_active_context_changes_without_page_work(){
 let mut storage=semio_framework_value_paged_list::PagedList::<u64,4096>::empty();let mut state=NativePagedReservation::default();state.header=storage.inspect_selection_header(1).result.unwrap();state.context=Some((0,1));let empty=semio_framework_value_paged_list::PagedList::<u64,4096>::with_page_items(1);assert_eq!(state.demand(&storage,0,1).unwrap(),state.demand(&empty,0,1).unwrap());assert!(state.work_demand(&storage,1,2,100).is_err());assert!(state.demand(&storage,0,2).is_err());assert_eq!(state.work_demand(&storage,0,1,0).unwrap(),0);let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_depth:64,..Default::default()};assert!(state.close_step(g).result.is_ok());assert!(state.terminal_is_empty());assert!(storage.terminal_is_empty());
}
