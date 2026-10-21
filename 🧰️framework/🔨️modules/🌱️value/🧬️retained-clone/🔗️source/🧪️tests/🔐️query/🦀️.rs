//! 🔐️ Shared source quotes serialize real Send-only retirement cursors without lending mutable custody.
use crate::{RetainedCloneSource,RetainedOwnedProjection,RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind,observe_retirement_allocations};
use crate::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use std::{cell::Cell,sync::{Arc,atomic::{AtomicBool,Ordering}},time::{Duration,Instant}};
#[derive(Default)]
struct QueryControl{created:AtomicBool,armed:AtomicBool,entered:AtomicBool,released:AtomicBool}
struct SendOnlyAuthority(Cell<u8>,Arc<QueryControl>);
struct SharedAuthority(Arc<QueryControl>);
struct QueryCursor{value:Cell<u8>,control:Arc<QueryControl>,empty:bool}
fn cursor(control:Arc<QueryControl>,value:u8)->Box<dyn RetirementCursor>{control.created.store(true,Ordering::Release);Box::new(QueryCursor{value:Cell::new(value),control,empty:false})}
impl RetireOwned for SendOnlyAuthority{
 fn retirement(self)->Box<dyn RetirementCursor>{cursor(self.1,self.0.get())}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<QueryCursor>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetireOwned for SharedAuthority{
 fn retirement(self)->Box<dyn RetirementCursor>{cursor(self.0,7)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<QueryCursor>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for QueryCursor{
 fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{if self.control.armed.swap(false,Ordering::AcqRel){self.control.entered.store(true,Ordering::Release);let deadline=Instant::now()+Duration::from_secs(5);while !self.control.released.load(Ordering::Acquire){assert!(Instant::now()<deadline,"shared query release timed out");std::thread::yield_now();}}self.value.set(self.value.get());Ok(RetirementDemand{depth:usize::from(!self.empty),..Default::default()})}
 fn close_step(&mut self,g:RetainedCloneGrant)->RetirementStep{if g.maximum_items==0||g.maximum_depth==0{return RetirementStep::BudgetExhausted;}assert_eq!(self.value.get(),7);self.empty=true;RetirementStep::Complete}
 fn terminal_is_empty(&self)->bool{self.empty}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
 fn terminal_release_bytes(&self)->Option<usize>{self.empty.then_some(size_of::<Self>())}
}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn birth<A:RetireOwned+Sync,B:RetireOwned>(a:A,b:B)->(RetainedCloneSource<A>,usize){let d=RetainedCloneSource::<A>::constructor_demand::<B>();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RetainedCloneSource::<A>::constructor_copy_bytes(),maximum_capacity_bytes:d.capacity_bytes,maximum_depth:d.depth,..Default::default()};let(result,heap)=observe_retirement_allocations(||RetainedCloneSource::admit_owned(a,b,g));let(source,p)=result.unwrap_or_else(|_|panic!("funded original query source birth"));assert_eq!(heap,(p.retained_capacity_bytes,0));(source,heap.0)}
fn paid(result:Result<crate::RetainedCloneStep,ValueError>,heap:(usize,usize),g:RetainedCloneGrant,born:&mut usize,freed:&mut usize){let p=result.unwrap().progress();assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));*born+=heap.0;*freed+=heap.1;}
fn source_turn<A:RetireOwned+Sync>(source:&mut RetainedCloneSource<A>,born:&mut usize,freed:&mut usize){let d=source.next_close_demand(4096).unwrap();let g=grant(d);assert_eq!(source.next_close_demand(g.maximum_copy_bytes).unwrap(),d);let(result,heap)=observe_retirement_allocations(||source.close_step(g));paid(result,heap,g,born,freed);}
fn projection_turn(source:&mut RetainedOwnedProjection<()>,born:&mut usize,freed:&mut usize){let d=source.next_close_demand(4096).unwrap();let g=grant(d);assert_eq!(source.next_close_demand(g.maximum_copy_bytes).unwrap(),d);let(result,heap)=observe_retirement_allocations(||source.close_step(g));paid(result,heap,g,born,freed);}
fn shared_queries<Q:Fn()->Result<RetirementDemand,ValueError>+Sync>(query:Q,control:&QueryControl){control.armed.store(true,Ordering::Release);std::thread::scope(|scope|{let pending=scope.spawn(||observe_retirement_allocations(&query));let deadline=Instant::now()+Duration::from_secs(5);while !control.entered.load(Ordering::Acquire){assert!(Instant::now()<deadline,"paid cursor query was not reached");std::thread::yield_now();}let(result,heap)=observe_retirement_allocations(&query);control.released.store(true,Ordering::Release);let(first,first_heap)=pending.join().unwrap();assert!(first.is_ok());assert_eq!(first_heap,(0,0));assert_eq!(heap,(0,0));assert_eq!(result.unwrap_err().kind,ValueRefusalKind::WorkLimit);});assert!(query().is_ok());}
#[test]
fn shared_source_and_projection_quotes_gate_actual_send_only_cell_cursors(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../../♻️retirement/🔐️mutex/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for row in law["sharedQueries"].as_array().unwrap(){let control=Arc::new(QueryControl::default());let mut freed=0;
  match row["port"].as_str().unwrap(){
   "source-owner"=>{let(mut source,mut born)=birth(SharedAuthority(control.clone()),());for _ in 0..128{if source.try_borrow().is_err(){break;}let d=source.next_take_demand(4096).unwrap();let g=grant(d);let(result,heap)=observe_retirement_allocations(||source.take_authority(g));let crate::RetainedCloneSourceTake::Pending(p)=result.unwrap_or_else(|_|panic!("funded lease handback"))else{panic!("owner was taken before query");};assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;freed+=heap.1;}assert!(source.try_borrow().is_err());for _ in 0..128{if control.created.load(Ordering::Acquire){break;}source_turn(&mut source,&mut born,&mut freed);}assert!(control.created.load(Ordering::Acquire));shared_queries(||source.next_close_demand(4096),&control);for _ in 0..128{if source.terminal_is_empty(){break;}source_turn(&mut source,&mut born,&mut freed);}assert!(source.terminal_is_empty());assert_eq!(born,freed);assert_eq!(observe_retirement_allocations(||drop(source)).1,(0,0));},
   port=>{let(mut source,mut born)=birth((),SendOnlyAuthority(Cell::new(7),control.clone()));let mut projection=if port=="projection"{let(p,receipt)=source.retain_original(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()}).unwrap();assert_eq!(receipt,RetainedCloneProgress{copied_items:1,..Default::default()});Some(p)}else{None};for _ in 0..128{if source.terminal_is_empty()||control.created.load(Ordering::Acquire){break;}source_turn(&mut source,&mut born,&mut freed);}if let Some(projection)=projection.as_mut(){assert!(source.terminal_is_empty());for _ in 0..128{if control.created.load(Ordering::Acquire){break;}projection_turn(projection,&mut born,&mut freed);}assert!(control.created.load(Ordering::Acquire));shared_queries(||projection.next_close_demand(4096),&control);for _ in 0..128{if projection.terminal_is_empty(){break;}projection_turn(projection,&mut born,&mut freed);}assert!(projection.terminal_is_empty());}else{assert!(control.created.load(Ordering::Acquire));shared_queries(||source.next_close_demand(4096),&control);for _ in 0..128{if source.terminal_is_empty(){break;}source_turn(&mut source,&mut born,&mut freed);}}assert!(source.terminal_is_empty());assert_eq!(born,freed);assert_eq!(observe_retirement_allocations(||{drop(projection);drop(source);}).1,(0,0));}
  }
 }
}
