use super::*;
use crate::retained_clone::RetainedCloneSource;
#[test]
fn genuine_scalar_projection_preserves_binding_copy_and_take_work_under_one_bank(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let mut stock:RetainedCloneGrant=serde_json::from_value(law["stock"].clone()).unwrap();let mut cpu=law["maximumCpuWork"].as_u64().unwrap();
 let value=law["value"].as_u64().unwrap();let oracle=std::sync::Arc::new(value);
 let birth=RetainedCloneSource::<u64>::owned_constructor_demand::<()>();
 let turn=RetainedCloneSource::admit_owned(value,(),RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:birth.capacity_bytes,maximum_depth:birth.depth,..Default::default()});cpu-=turn.work;let(mut source,p)=turn.result.map_err(|(e,_,_)|e).unwrap();stock.maximum_items-=p.copied_items;stock.maximum_capacity_bytes-=p.retained_capacity_bytes;
 let d=u64::projection_birth_demand();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
 let turn=u64::begin_projection(RetainedCloneGrant{maximum_items:0,..g});assert_eq!(turn.work,law["expected"]["deniedCpu"].as_u64().unwrap());assert!(turn.result.is_err());
 let turn=u64::begin_projection(g);assert_eq!(turn.work,law["expected"]["birthCpu"].as_u64().unwrap());cpu-=turn.work;let(mut cursor,p)=turn.result.unwrap();stock.maximum_items-=p.copied_items;stock.maximum_copy_bytes-=p.copied_bytes;
 for key in ["bindingCpu","copyCpu"]{
  let d=cursor.demands(source.borrow(),stock.maximum_copy_bytes).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
  let turn=cursor.advance(source.borrow(),stock.maximum_copy_bytes,RetainedCloneGrant{maximum_items:0,..g});assert_eq!(turn.work,law["expected"]["deniedCpu"].as_u64().unwrap());assert_eq!(turn.result.unwrap().progress(),Default::default());
  let quoted=cursor.advance_work_demand(source.borrow(),stock.maximum_copy_bytes).unwrap();let turn=cursor.advance(source.borrow(),stock.maximum_copy_bytes,g);assert_eq!(quoted,turn.work);assert_eq!(turn.work,law["expected"][key].as_u64().unwrap());cpu-=turn.work;let p=turn.result.unwrap().progress();assert!(p.fits(g));stock.maximum_items-=p.copied_items;stock.maximum_copy_bytes-=p.copied_bytes;stock.maximum_capacity_bytes-=p.retained_capacity_bytes;stock.maximum_release_bytes-=p.released_bytes;
 }
 assert!(cursor.ready());let d=cursor.take_demand().unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_depth:d.depth,..Default::default()};let quoted=cursor.take_work_demand().unwrap();let turn=cursor.take(g);assert_eq!(quoted,turn.work);assert_eq!(turn.work,law["expected"]["takeCpu"].as_u64().unwrap());cpu-=turn.work;let(output,p)=turn.result.unwrap().unwrap();assert_eq!(output,*oracle);stock.maximum_items-=p.copied_items;stock.maximum_copy_bytes-=p.copied_bytes;
 let quoted=cursor.begin_close_work_demand().unwrap();assert_eq!(quoted,law["expected"]["closeStartCpu"].as_u64().unwrap());let start=cursor.begin_close_demand().unwrap();let turn=cursor.begin_close_paid(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:start.copy_bytes,maximum_depth:start.depth,..Default::default()});assert_eq!(turn.work,quoted);let start_receipt=turn.result.unwrap();stock.maximum_items-=start_receipt.copied_items;stock.maximum_copy_bytes-=start_receipt.copied_bytes;cpu=cpu.checked_sub(turn.work).unwrap();let repeated=cursor.begin_close_paid(Default::default());assert_eq!(repeated.work,law["expected"]["repeatedCloseStartCpu"].as_u64().unwrap());repeated.result.unwrap();
 for _ in 0..law["maximumTurns"].as_u64().unwrap(){
  if cursor.terminal_is_empty(){break}
  let d=cursor.close_demands(stock.maximum_copy_bytes).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
  assert!(g.maximum_copy_bytes<=stock.maximum_copy_bytes&&g.maximum_capacity_bytes<=stock.maximum_capacity_bytes&&g.maximum_release_bytes<=stock.maximum_release_bytes&&g.maximum_depth<=stock.maximum_depth);
  let quoted=cursor.close_work_demand(stock.maximum_copy_bytes).unwrap();let turn=cursor.close_step(g);assert_eq!(quoted,turn.work);cpu=cpu.checked_sub(turn.work).unwrap();let p=turn.result.unwrap().progress();stock.maximum_items-=p.copied_items;stock.maximum_copy_bytes-=p.copied_bytes;stock.maximum_capacity_bytes-=p.retained_capacity_bytes;stock.maximum_release_bytes-=p.released_bytes;
 }
 assert!(cursor.terminal_is_empty());let turn=cursor.close_step(Default::default());assert_eq!(turn.work,law["expected"]["cachedCpu"].as_u64().unwrap());turn.result.unwrap();
 for _ in 0..law["maximumTurns"].as_u64().unwrap(){
  if source.terminal_is_empty(){break}
  let d=source.next_close_demand(stock.maximum_copy_bytes).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
  assert!(g.maximum_copy_bytes<=stock.maximum_copy_bytes&&g.maximum_capacity_bytes<=stock.maximum_capacity_bytes&&g.maximum_release_bytes<=stock.maximum_release_bytes&&g.maximum_depth<=stock.maximum_depth);
  let turn=source.close_step(g);cpu=cpu.checked_sub(turn.work).unwrap();let p=turn.result.unwrap().progress();stock.maximum_items-=p.copied_items;stock.maximum_copy_bytes-=p.copied_bytes;stock.maximum_capacity_bytes-=p.retained_capacity_bytes;stock.maximum_release_bytes-=p.released_bytes;
 }
 assert!(source.terminal_is_empty());assert!(cpu>0);
}

#[derive(crate::RetireOwned)]
struct RefusingProjectionSource(u8);
struct RefusingProjectionCursor{closing:bool}
impl SnapshotProjection for RefusingProjectionSource{
 type Output=Self;type Projection=RefusingProjectionCursor;
 fn projection_birth_demand()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
 fn projection_birth_work_demand()->Result<u64,ValueError>{Ok(1)}
 fn begin_projection(grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<(Self::Projection,RetainedCloneProgress),ValueError>{if grant.maximum_items==0||grant.maximum_depth==0{return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("fixture birth denied")),work:0}}crate::RetainedWorkOutcome{result:Ok((RefusingProjectionCursor{closing:false},RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
}
impl SnapshotProjectionCursor<RefusingProjectionSource> for RefusingProjectionCursor{
 fn demands(&self,_:RetainedCloneRef<'_,RefusingProjectionSource>,_:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{depth:1,..Default::default()})}
 fn advance_work_demand(&self,_:RetainedCloneRef<'_,RefusingProjectionSource>,_:usize)->Result<u64,ValueError>{Ok(1)}
 fn advance(&mut self,_:RetainedCloneRef<'_,RefusingProjectionSource>,_:usize,grant:RetainedCloneGrant)->crate::OriginalRetirementOutcome{if grant.maximum_items==0||grant.maximum_depth==0{return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}let primary=ValueError::literal(ValueRefusalKind::InvalidValue,"actual projection source");let receipt=ValueError::literal(ValueRefusalKind::OwnershipLimit,"actual projection receipt");crate::RetainedWorkOutcome{result:Err(crate::OriginalRetirementFailure::from_primary_and_receipt(primary,receipt).with_retained_progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
 fn ready(&self)->bool{false}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{Err(ValueError::invariant("fixture output absent"))}
 fn take_work_demand(&self)->Result<u64,ValueError>{Err(ValueError::invariant("fixture output absent"))}
 fn take(&mut self,_:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(RefusingProjectionSource,RetainedCloneProgress)>,ValueError>{crate::RetainedWorkOutcome{result:Err(ValueError::invariant("fixture output absent")),work:0}}
 fn begin_close(&mut self){self.closing=true}
 fn close_demands(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 fn close_work_demand(&self,_:usize)->Result<u64,ValueError>{Ok(0)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let _=effect.grant();crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
 fn terminal_is_empty(&self)->bool{self.closing}
}
#[test]
fn original_projection_forwards_the_real_child_work_and_independent_refusals(){
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64};let admission=RetainedCloneSource::<RefusingProjectionSource>::admit_owned(RefusingProjectionSource(7),(),grant);let(mut source,_)=admission.result.map_err(|(error,_,_)|error).unwrap();let mut field=ProjectionField::<RefusingProjectionSource>::default();let born=field.advance(source.borrow(),4096,grant);assert_eq!(born.work,2);born.result.unwrap();let denied=field.advance(source.borrow(),4096,RetainedCloneGrant{maximum_items:0,..grant});assert_eq!(denied.work,0);assert_eq!(denied.result.unwrap().progress(),Default::default());let turn=field.advance(source.borrow(),4096,grant);assert_eq!(turn.work,1);let failure=turn.result.unwrap_err();assert_eq!(failure.primary().unwrap().message,"actual projection source");assert_eq!(failure.receipt_refusal().unwrap().message,"actual projection receipt");assert_eq!(failure.retained_progress().copied_items,1);let mut failure_owner=crate::retirement::controlled::ControlledRetirement::new(failure).unwrap();while !failure_owner.terminal_is_empty(){failure_owner.step(grant).unwrap();}field.begin_close();while !field.terminal_is_empty(){field.close_step(grant).result.unwrap();}while !source.terminal_is_empty(){source.close_step(grant).result.unwrap();}
}
