//! 🔐️ Shared source quotes serialize real Send-only retirement cursors without lending mutable custody.
use crate::{RetainedCloneSource,RetainedOwnedProjection,RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind,observe_retirement_allocations};
use crate::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use std::{cell::Cell,sync::{Arc,atomic::{AtomicBool,Ordering}},time::{Duration,Instant}};
#[derive(Default)]
struct QueryControl{probed:AtomicBool,created:AtomicBool,armed:AtomicBool,entered:AtomicBool,released:AtomicBool}
struct SendOnlyAuthority(Cell<u8>,Arc<QueryControl>);
struct SharedAuthority(Arc<QueryControl>);
struct QueryCursor{value:Cell<u8>,control:Arc<QueryControl>,empty:bool}
fn cursor(control:Arc<QueryControl>,value:u8)->Box<dyn RetirementCursor>{control.created.store(true,Ordering::Release);Box::new(QueryCursor{value:Cell::new(value),control,empty:false})}
impl RetireOwned for SendOnlyAuthority{
 fn retirement(self)->Box<dyn RetirementCursor>{cursor(self.1,self.0.get())}
 fn retirement_birth_bytes(&self)->Option<usize>{self.1.probed.store(true,Ordering::Release);Some(size_of::<QueryCursor>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetireOwned for SharedAuthority{
 fn retirement(self)->Box<dyn RetirementCursor>{cursor(self.0,7)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<QueryCursor>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for QueryCursor{
 fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{if self.control.armed.swap(false,Ordering::AcqRel){self.control.entered.store(true,Ordering::Release);let deadline=Instant::now()+Duration::from_secs(5);while !self.control.released.load(Ordering::Acquire){assert!(Instant::now()<deadline,"shared query release timed out");std::thread::yield_now();}}self.value.set(self.value.get());Ok(RetirementDemand{depth:usize::from(!self.empty),..Default::default()})}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let g=effect.grant();if self.empty{return RetirementStep::Complete(0)}if g.maximum_items==0||g.maximum_depth==0{return RetirementStep::BudgetExhausted;}assert_eq!(self.value.get(),7);self.empty=true;RetirementStep::Complete(1)}
 fn terminal_is_empty(&self)->bool{self.empty}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
 fn terminal_release_bytes(&self)->Option<usize>{self.empty.then_some(size_of::<Self>())}
}
use crate::source_law_bank::Bank;
fn grant(d:RetirementDemand,bank:&Bank)->RetainedCloneGrant{bank.grant(d)}
fn birth<A:RetireOwned+Sync,B:RetireOwned>(a:A,b:B,bank:&mut Bank)->(RetainedCloneSource<A>,usize){let(source,p)=bank.source(RetainedCloneSource::admission_owned(a,b));assert_eq!(p.copied_items,6);(source,p.retained_capacity_bytes)}

fn paid(result:crate::OriginalRetirementOutcome,heap:(usize,usize),g:RetainedCloneGrant,born:&mut usize,freed:&mut usize){assert!(result.work<=1);let p=result.result.unwrap().progress();assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));*born+=heap.0;*freed+=heap.1;}
fn source_turn<A:RetireOwned+Sync>(source:&mut RetainedCloneSource<A>,born:&mut usize,freed:&mut usize,bank:&mut Bank){let d=source.next_close_demand(4096).unwrap();let g=grant(d,bank);assert_eq!(source.next_close_demand(g.maximum_copy_bytes).unwrap(),d);let(result,heap)=bank.observe(g,||source.close_step(g),|s|s.progress());paid(result,heap,g,born,freed);}
fn projection_turn(source:&mut RetainedOwnedProjection<()>,born:&mut usize,freed:&mut usize,bank:&mut Bank){let d=source.next_close_demand(4096).unwrap();let g=grant(d,bank);assert_eq!(source.next_close_demand(g.maximum_copy_bytes).unwrap(),d);let(result,heap)=bank.observe(g,||source.close_step(g),|s|s.progress());paid(result,heap,g,born,freed);}
fn shared_queries<Q:Fn()->Result<RetirementDemand,ValueError>+Sync>(query:Q,control:&QueryControl){control.armed.store(true,Ordering::Release);std::thread::scope(|scope|{let pending=scope.spawn(||observe_retirement_allocations(&query));let deadline=Instant::now()+Duration::from_secs(5);while !control.entered.load(Ordering::Acquire){assert!(Instant::now()<deadline,"paid cursor query was not reached");std::thread::yield_now();}let(result,heap)=observe_retirement_allocations(&query);control.released.store(true,Ordering::Release);let(first,first_heap)=pending.join().unwrap();assert!(first.is_ok());assert_eq!(first_heap,(0,0));assert_eq!(heap,(0,0));assert_eq!(result.unwrap_err().kind,ValueRefusalKind::WorkLimit);});assert!(query().is_ok());}
#[test]
fn shared_source_and_projection_quotes_gate_actual_send_only_cell_cursors(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../../♻️retirement/🔐️mutex/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for row in law["sharedQueries"].as_array().unwrap(){let mut bank=Bank::new();let control=Arc::new(QueryControl::default());let mut freed=0;
  match row["port"].as_str().unwrap(){
   "source-owner"=>{let(mut source,mut born)=birth(SharedAuthority(control.clone()),(),&mut bank);for _ in 0..128{if source.try_borrow().is_err(){break;}let d=source.next_take_demand(4096).unwrap();let g=grant(d,&bank);let(result,heap)=bank.observe(g,||source.take_authority(g),|step|match step{crate::RetainedCloneSourceTake::Pending(p)|crate::RetainedCloneSourceTake::Ready(_,p)=>*p});let crate::RetainedCloneSourceTake::Pending(p)=result.result.unwrap_or_else(|_|panic!("funded lease handback"))else{panic!("owner was taken before query");};assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;freed+=heap.1;}assert!(source.try_borrow().is_err());for _ in 0..128{if control.created.load(Ordering::Acquire){break;}source_turn(&mut source,&mut born,&mut freed,&mut bank);}assert!(control.created.load(Ordering::Acquire));shared_queries(||source.next_close_demand(4096),&control);for _ in 0..128{if source.terminal_is_empty(){break;}source_turn(&mut source,&mut born,&mut freed,&mut bank);}assert!(source.terminal_is_empty());assert_eq!(born,freed);assert_eq!(observe_retirement_allocations(||drop(source)).1,(0,0));},
   port=>{let(mut source,mut born)=birth((),SendOnlyAuthority(Cell::new(7),control.clone()),&mut bank);let mut projection=if port=="projection"{let g=bank.grant(RetirementDemand{depth:1,..Default::default()});let((p,receipt),_)=bank.success(g,||source.retain_original(g),|(_,p)|*p);assert_eq!(receipt,RetainedCloneProgress{copied_items:1,..Default::default()});Some(p)}else{None};for _ in 0..128{if source.terminal_is_empty()||control.created.load(Ordering::Acquire){break;}source_turn(&mut source,&mut born,&mut freed,&mut bank);}if let Some(projection)=projection.as_mut(){assert!(source.terminal_is_empty());for _ in 0..128{if control.created.load(Ordering::Acquire){break;}projection_turn(projection,&mut born,&mut freed,&mut bank);}assert!(control.created.load(Ordering::Acquire));shared_queries(||projection.next_close_demand(4096),&control);for _ in 0..128{if projection.terminal_is_empty(){break;}projection_turn(projection,&mut born,&mut freed,&mut bank);}assert!(projection.terminal_is_empty());}else{assert!(control.created.load(Ordering::Acquire));shared_queries(||source.next_close_demand(4096),&control);for _ in 0..128{if source.terminal_is_empty(){break;}source_turn(&mut source,&mut born,&mut freed,&mut bank);}}assert!(source.terminal_is_empty());assert_eq!(born,freed);assert_eq!(observe_retirement_allocations(||{drop(projection);drop(source);}).1,(0,0));}
  }
 }
}

#[test]
fn partial_admission_shared_quotes_never_enter_private_send_only_authority_or_cursor(){
 let f:serde_json::Value=serde_json::from_str(include_str!("../../🎟️admission/case.json")).unwrap();let schema:serde_json::Value=serde_json::from_str(include_str!("../../🎟️admission/case.schema.json")).unwrap();for(k,v)in f.as_object().unwrap(){assert_eq!(&schema["properties"][k]["const"],v)}let law=&f["sharedQuote"];
 for cut in f["cancelCuts"].as_array().unwrap(){let stock=&f["stock"];let mut bank=Bank::from_stock(RetainedCloneGrant{maximum_items:stock["items"].as_u64().unwrap()as usize,maximum_copy_bytes:stock["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:stock["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:stock["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:stock["depth"].as_u64().unwrap()as usize},stock["fuel"].as_u64().unwrap());let control=Arc::new(QueryControl::default());let mut owner=RetainedCloneSource::admission_owned(7usize,SendOnlyAuthority(Cell::new(7),Arc::clone(&control)));
  let mut allocation_actual=None;for _ in 0..cut.as_u64().unwrap(){let g=bank.grant(owner.next_demand().unwrap());let(_,heap)=bank.success(g,||owner.advance(g,&mut allocation_actual),|s|s.progress());if let Some(actual)=allocation_actual.take(){assert_eq!(actual,heap.0)}}
  let mut returned=None;for state in law["states"].as_array().unwrap(){if state=="returning"{let g=bank.grant(owner.next_return_demand().unwrap());bank.success(g,||owner.return_inputs(&mut returned,g),|s|s.progress());assert!(returned.is_none());}
   let before=(owner.phase(),owner.next_demand().unwrap(),owner.next_return_demand().unwrap(),owner.original_authority().copied(),owner.terminal_is_empty());let barrier=std::sync::Barrier::new(law["threads"].as_u64().unwrap()as usize);
   std::thread::scope(|scope|{for _ in 0..law["threads"].as_u64().unwrap(){let owner=&owner;let barrier=&barrier;let control=&control;let before=&before;scope.spawn(move||{barrier.wait();for _ in 0..law["iterations"].as_u64().unwrap(){let(actual,heap)=observe_retirement_allocations(||(owner.phase(),owner.next_demand(),owner.next_return_demand(),owner.original_authority().copied(),owner.terminal_is_empty()));assert_eq!(heap,(0,0));assert_eq!(actual.0,before.0);assert_eq!(actual.1.unwrap(),before.1);assert_eq!(actual.2.unwrap(),before.2);assert_eq!(actual.3,before.3);assert_eq!(actual.4,before.4);assert!(!control.probed.load(Ordering::Acquire));assert!(!control.created.load(Ordering::Acquire));}});}});
  }
  for _ in 0..10000{if owner.terminal_is_empty(){break}let g=bank.grant(owner.next_return_demand().unwrap());bank.success(g,||owner.return_inputs(&mut returned,g),|s|s.progress());}assert!(owner.terminal_is_empty());assert!(!control.probed.load(Ordering::Acquire));assert!(!control.created.load(Ordering::Acquire));let returned=returned.unwrap();assert_eq!(returned.0,7);assert_eq!(returned.1.0.get(),7);let mut close=crate::retirement::controlled::ControlledRetirement::retain(returned);
  for _ in 0..10000{if close.terminal_is_empty(){break}let g=bank.grant(close.next_demand(4096).unwrap());bank.success(g,||{let result=close.step(g);crate::RetainedWorkOutcome{result,work:close.last_turn_work()}},|s|s.progress());}assert!(close.terminal_is_empty());assert!(control.created.load(Ordering::Acquire));assert!(control.probed.load(Ordering::Acquire));assert_eq!(observe_retirement_allocations(||drop((owner,close))).1,(0,0));
 }
}
