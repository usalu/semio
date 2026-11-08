//! 🧭️ Actual generic close dispatch and paid lifecycle observations shared by four semantic owners.
use super::*;
use std::cell::Cell;
use crate::observe_retirement_allocations as observe;
use semio_framework_value_clone_source_testing::{FixtureSourceBirth,FixtureCloseStep,observe_fixture_source_allocations};
#[path="../../../🔤️string-cursor/🧪️tests/producer.rs"] mod producer;
use producer::{Harness,Op};
thread_local!{static ACTIVE:Cell<bool>=const{Cell::new(false)};static CALLS:Cell<(usize,[(usize,usize);64])>=const{Cell::new((0,[(0,0);64]))};}
pub(crate) fn record(cursor:usize,body:usize){ACTIVE.with(|active|{if active.get(){CALLS.with(|state|{let(mut n,mut rows)=state.get();if n<rows.len(){rows[n]=(cursor,body);}n+=1;state.set((n,rows));});}});}
fn reset(){CALLS.with(|state|state.set((0,[(0,0);64])));ACTIVE.with(|active|active.set(true));}
fn calls(cursor:usize)->Vec<usize>{ACTIVE.with(|active|active.set(false));CALLS.with(|state|{let(n,rows)=state.get();assert!(n<=rows.len(),"selected close observation overflow");rows[..n].iter().filter_map(|(owner,body)|(*owner==cursor).then_some(*body)).collect()})}

fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
pub(super) fn verify<T:RetainedClone<Cursor=C>,C:RetainedCloneCursor<T>>(make:impl Fn()->T,select:fn(&C,usize)->Result<RetirementDemand,ValueError>){
 let policy:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let bound=policy["maximumTurns"].as_u64().unwrap()as usize;
 for cut in policy["cuts"].as_array().unwrap(){
  let(mut source,heap)=observe_fixture_source_allocations(||RetainedCloneSource::<T>::from_owner(make()));assert!(heap.0>0);assert_eq!(heap.1,0);let identity=source.borrow().get()as*const T;let mut cursor=Harness::<T>::new(&source);
  for _ in 0..cut.as_u64().unwrap(){if cursor.ready(){break}let maximum=source.wallet().remaining_work();let(d,work)=cursor.quote(&source,Op::Advance,4096,maximum);let g=grant(d);let(step,heap)=cursor.drive(&mut source,Op::Advance,4096,maximum,g,work);assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}
  let maximum=source.wallet().remaining_work();let(d,work)=cursor.quote(&source,Op::Start,0,maximum);cursor.drive(&mut source,Op::Start,0,maximum,grant(d),work);let address=cursor.inspect_cursor(|c|c as*const C as usize);
  for body in policy["bodies"].as_array().unwrap(){let body=body.as_u64().unwrap()as usize;let(expected,heap)=observe(||cursor.inspect_cursor(|c|select(c,body)));assert_eq!(heap,(0,0));reset();let(actual,heap)=observe(||cursor.inspect_cursor(|c|RetainedCloneCursor::<T>::close_demands(c,body)));assert_eq!(heap,(0,0));assert_eq!(actual.unwrap(),expected.unwrap());assert_eq!(calls(address),vec![body]);assert_eq!(source.borrow().get()as*const T,identity);}
  for turn in 0..bound{if cursor.terminal(){break}let maximum=source.wallet().remaining_work();let(d,work)=cursor.quote(&source,Op::Close,4096,maximum);let g=grant(d);assert_eq!(cursor.quote(&source,Op::Close,g.maximum_copy_bytes,maximum).0,d);
   let denied=[Some(RetainedCloneGrant{maximum_items:0,..g}),(d.copy_bytes>0).then(||RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g}),(d.capacity_bytes>0).then(||RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..g}),(d.release_bytes>0).then(||RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..g}),(d.depth>0).then(||RetainedCloneGrant{maximum_depth:d.depth-1,..g})];
   for denied in denied.into_iter().flatten(){let before=source.wallet().remaining_work();let(step,heap)=cursor.drive(&mut source,Op::Close,4096,maximum,denied,work);assert_eq!(heap,(0,0),"selected close denial {denied:?} demand {d:?}");assert_eq!(source.wallet().remaining_work(),before);assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(cursor.quote(&source,Op::Close,4096,maximum).0,d);assert_eq!(source.borrow().get()as*const T,identity);}
   let(step,heap)=cursor.drive(&mut source,Op::Close,4096,maximum,g,work);let p=step.progress();assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert!(turn+1<bound||cursor.terminal(),"selected close cursor did not finish");}
  assert!(cursor.terminal());cursor.release(&mut source);assert_eq!(observe(||drop(cursor)).1,(0,0));assert_eq!(source.borrow().get()as*const T,identity);
  let mut observed=(0usize,0usize);for turn in 0..bound{if source.terminal_is_empty(){break}let d=source.next_close_demand(4096).unwrap();let g=grant(d);assert_eq!(source.next_close_demand(g.maximum_copy_bytes).unwrap(),d);let(state,heap)=observe_fixture_source_allocations(||source.close_with(4096,g).unwrap());observed.0+=heap.0;observed.1+=heap.1;let p=match state{FixtureCloseStep::Pending=>continue,FixtureCloseStep::Progress(p)=>p,FixtureCloseStep::Complete=>Default::default(),FixtureCloseStep::Denied|FixtureCloseStep::Blocked=>panic!("selected Source close refused its exact quote")};assert!(p.fits(g));assert_eq!(observed,(p.retained_capacity_bytes,p.released_bytes));observed=(0,0);assert!(turn+1<bound||source.terminal_is_empty(),"selected close source did not finish");}
  assert!(source.terminal_is_empty());assert_eq!(observe_fixture_source_allocations(||drop(source)).1,(0,0));
 }
}
