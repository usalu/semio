use super::*;
use std::cell::Cell;
use std::mem::{MaybeUninit,size_of};
use semio_framework_value::RetirementDemand;

struct OriginalSpan<'a>{bytes:&'a[u8],reads:&'a Cell<usize>,depth:usize}
impl JobPayloadStagedSource for OriginalSpan<'_>{
 fn byte_len(&self)->usize{self.bytes.len()}
 fn copy_byte_demand(&self)->Result<usize,JobPayloadAdmissionFault>{Ok(self.bytes.len())}
 fn depth_demand(&self)->Result<usize,JobPayloadAdmissionFault>{Ok(self.depth)}
 fn write_bytes(&self,target:&mut[MaybeUninit<u8>])->Result<(),JobPayloadAdmissionFault>{self.reads.set(self.reads.get()+1);for(slot,byte)in target.iter_mut().zip(self.bytes){slot.write(*byte);}Ok(())}
}
struct OriginalFixture{caller:OriginalJobFixtureCaller,decoder:OriginalJobDecodeOwner}
impl OriginalFixture{
 const STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1_000_000,maximum_copy_bytes:1<<28,maximum_capacity_bytes:1<<28,maximum_release_bytes:1<<28,maximum_depth:32};
 fn new()->Self{Self{caller:OriginalJobFixtureCaller::admit(OperationId(9918),Generation(3),Self::STOCK,||Some(0),u64::MAX,1_000_000),decoder:OriginalJobDecodeOwner::empty()}}
 fn turn<T>(&mut self,grant:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{self.caller.turn_with_decoder(grant,&mut self.decoder,run).expect("same original staged payload turn")}
 fn close_writer(&mut self,writer:&mut RetainedJobPayloadWriter){writer.begin_close();while !writer.terminal_is_empty(){let d=writer.retirement_demands().unwrap();self.turn(exact(d),|cx|close_original_retained_step(cx,d,|g|writer.close_step(g)));}}
 fn close_payload(&mut self,payload:&mut RetainedJobPayload){while !payload.terminal_is_empty(){let d=payload.retirement_demands().unwrap();self.turn(exact(d),|cx|close_original_retained_step(cx,d,|g|payload.close_step(g)));}}
 fn finish(mut self){while !self.decoder.terminal_is_empty(){let d=self.decoder.retirement_demands(4096).unwrap();self.caller.turn(exact(d),|cx|close_original_retained_step(cx,d,|g|self.decoder.close_step(g))).unwrap();}self.caller.retire(Self::STOCK);}
}
fn exact(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)}}
fn observe<T>(run:impl FnOnce()->T)->(T,(usize,usize)){let(v,h)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);assert!(!h.overflowed);(v,(h.requested_bytes,h.released_bytes))}
fn writer_demand(frontier:usize,writer:&Option<RetainedJobPayloadWriter>,source:&OriginalSpan<'_>)->RetirementDemand{match frontier{0=>RetainedJobPayloadWriter::birth_demand(),1=>writer.as_ref().unwrap().staged_page_birth_demand().unwrap(),2=>writer.as_ref().unwrap().staged_source_demand(source,size_of::<usize>()).unwrap(),_=>writer.as_ref().unwrap().staged_commit_demand(size_of::<[usize;3]>()).unwrap()}}
fn writer_turn(frontier:usize,writer:&mut Option<RetainedJobPayloadWriter>,source:&OriginalSpan<'_>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{Ok(match frontier{0=>{*writer=RetainedJobPayloadWriter::new_funded(JobPayloadStream::CheckpointState,cx)?;writer.is_some()},1=>writer.as_mut().unwrap().begin_staged_page_funded(cx)?,2=>writer.as_mut().unwrap().write_staged_source_funded(cx,source,size_of::<usize>())?,_=>writer.as_mut().unwrap().commit_staged_page_funded(cx,size_of::<[usize;3]>())?})}

#[test]
fn original_staged_projection_admits_source_and_writer_frontiers_independently(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⛽️staged-projection/🔣️.json")).unwrap();
 let bytes:Vec<u8>=fixture["sourceBytes"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect();
 for frontier in 0..4{for axis in fixture["deniedAxes"].as_array().unwrap().iter().map(|v|v.as_str().unwrap()).chain(["exact"]){
  let mut original=OriginalFixture::new();let reads=Cell::new(0);let source=OriginalSpan{bytes:&bytes,reads:&reads,depth:fixture["sourceDepth"].as_u64().unwrap()as usize};let mut writer=None;
  for prefix in 0..frontier{let d=writer_demand(prefix,&writer,&source);assert!(original.turn(exact(d),|cx|writer_turn(prefix,&mut writer,&source,cx)));}
  let d=writer_demand(frontier,&writer,&source);let mut grant=exact(d);
  match axis{"items"=>grant.maximum_items=0,"copyBytes" if d.copy_bytes>0=>grant.maximum_copy_bytes-=1,"capacityBytes" if d.capacity_bytes>0=>grant.maximum_capacity_bytes-=1,"depth"=>grant.maximum_depth-=1,"cancelled"=>original.caller.cancel().cancel_now(),"exact"=>{},_=>{if let Some(w)=writer.as_mut(){original.close_writer(w);}original.finish();continue;}}
  let before=(writer.as_ref().map(RetainedJobPayloadWriter::page_count),writer.as_ref().and_then(RetainedJobPayloadWriter::staged_page_len),writer.as_ref().and_then(RetainedJobPayloadWriter::staged_backing_identity),reads.get());
  let(admitted,heap)=observe(||original.turn(grant,|cx|writer_turn(frontier,&mut writer,&source,cx)));
  if axis=="exact"{assert!(admitted);assert_eq!(heap,(d.capacity_bytes,d.release_bytes));assert_eq!(original.caller.receipt().copied_bytes,d.copy_bytes);assert_eq!(original.caller.receipt().copied_items,1);}
  else{assert!(!admitted);assert_eq!(heap,(0,0));assert_eq!(original.caller.receipt(),Default::default());assert_eq!((writer.as_ref().map(RetainedJobPayloadWriter::page_count),writer.as_ref().and_then(RetainedJobPayloadWriter::staged_page_len),writer.as_ref().and_then(RetainedJobPayloadWriter::staged_backing_identity),reads.get()),before);}
  if let Some(w)=writer.as_mut(){original.close_writer(w);}original.finish();
 }}
 let mut original=OriginalFixture::new();let reads=Cell::new(0);let source=OriginalSpan{bytes:&bytes,reads:&reads,depth:3};let mut writer=None;
 for frontier in 0..4{let d=writer_demand(frontier,&writer,&source);assert!(original.turn(exact(d),|cx|writer_turn(frontier,&mut writer,&source,cx)));}
 assert_eq!(reads.get(),1);let mut output=None;assert!(original.turn(exact(RetainedJobPayloadWriter::finish_demand()),|cx|RetainedJobPayloadWriter::finish_funded(&mut writer,&mut output,cx).map_err(Into::into)));
 let mut payload=output.unwrap();assert_eq!(payload.page(0),Some(bytes.as_slice()));original.close_payload(&mut payload);original.finish();
}

#[test]
fn original_staged_projection_reuses_its_retained_refused_backing_without_allocation(){
 let mut original=OriginalFixture::new();let mut writer=original.turn(exact(RetainedJobPayloadWriter::birth_demand()),|cx|RetainedJobPayloadWriter::new_funded(JobPayloadStream::CheckpointState,cx).map_err(Into::into)).unwrap();
 let mut source=None;let demand=OriginalConstructionDemand{items:1,copy_bytes:size_of::<Option<JobPayloadPageSource>>(),capacity_bytes:JOB_PAYLOAD_PAGE_BYTES,release_bytes:0,depth:1};
 assert!(original.turn(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:0,maximum_depth:1},|cx|admit_original_construction(cx,&mut source,demand,|_|Ok((JobPayloadPageSource::new(),RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()})))));
 let pointer=source.as_ref().unwrap().backing_identity();let handoff=RetirementDemand{copy_bytes:2*size_of::<Option<JobPayloadPageSource>>(),depth:1,..Default::default()};
 original.turn(exact(handoff),|cx|{*writer.rejected=source.take();cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:handoff.copy_bytes,..Default::default()})?;Ok(())});
 let demand=writer.staged_page_birth_demand().unwrap();assert_eq!(demand.capacity_bytes,0);let(admitted,heap)=observe(||original.turn(exact(demand),|cx|writer.begin_staged_page_funded(cx).map_err(Into::into)));assert!(admitted);assert_eq!(heap,(0,0));assert_eq!(writer.staged_backing_identity(),Some(pointer));assert_eq!(original.caller.receipt().retained_capacity_bytes,0);
 original.close_writer(&mut writer);original.finish();
}

fn boxed_demand(owner:&Option<Box<RetainedPayloadBuilder>>,source:&OriginalSpan<'_>)->RetirementDemand{match owner.as_ref(){None=>RetainedPayloadBuilder::boxed_birth_demands(),Some(b)if !b.is_initialized()=>b.initialization_demand(),Some(b)if b.staged_page_len().is_none()&&b.original().unwrap().page_count()==0=>b.staged_page_birth_demands().unwrap(),Some(b)if b.staged_page_len()==Some(0)=>b.staged_source_demands(source,0).unwrap(),Some(b)if b.staged_page_len().is_some()=>b.staged_commit_demands().unwrap(),_=>RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()}}}
fn boxed_turn(owner:&mut Option<Box<RetainedPayloadBuilder>>,source:&OriginalSpan<'_>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
 let Some(b)=owner.as_mut()else{*owner=RetainedPayloadBuilder::admit_boxed(JobPayloadStream::Preview,cx)?;return Ok(false)};
 if !b.is_initialized(){b.advance_initialization(cx)?;return Ok(false)}if b.staged_page_len().is_none()&&b.original().unwrap().page_count()==0{b.begin_staged_page_funded(cx)?;return Ok(false)}if b.staged_page_len()==Some(0){b.write_staged_source_funded(cx,source,0)?;return Ok(false)}if b.staged_page_len().is_some(){b.commit_staged_page_funded(cx)?;return Ok(false)}Ok(b.seal(cx)?)
}
#[test]
fn original_boxed_staged_payload_initializes_and_projects_under_fixed_small_copy_grants(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⛽️staged-projection/🔣️.json")).unwrap();let law=&fixture["construction"];let bytes:Vec<u8>=fixture["sourceBytes"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect();
 for prefix in law["cancelPrefixes"].as_array().unwrap(){
  let mut original=OriginalFixture::new();let reads=Cell::new(0);let source=OriginalSpan{bytes:&bytes,reads:&reads,depth:3};let mut owner:Option<Box<RetainedPayloadBuilder>>=None;let(mut complete,mut born,mut released,mut turns)=(false,0,0,0);
  while turns<prefix.as_u64().unwrap()as usize&&!complete{
   let d=boxed_demand(&owner,&source);assert!(d.copy_bytes<=law["maximumCopyBytes"].as_u64().unwrap()as usize);let pointer=owner.as_ref().map(|v|&**v as*const _);let page=owner.as_ref().and_then(|v|v.staged_backing_identity());let prior_reads=reads.get();
   for axis in 0..5{let mut g=exact(d);match axis{0=>g.maximum_items=0,1 if d.copy_bytes>0=>g.maximum_copy_bytes-=1,2 if d.capacity_bytes>0=>g.maximum_capacity_bytes-=1,3 if d.release_bytes>0=>g.maximum_release_bytes-=1,4=>g.maximum_depth-=1,_=>continue}let(_,heap)=observe(||original.turn(g,|cx|boxed_turn(&mut owner,&source,cx)));assert_eq!(heap,(0,0));assert_eq!(original.caller.receipt(),Default::default());assert_eq!(owner.as_ref().map(|v|&**v as*const _),pointer);assert_eq!(owner.as_ref().and_then(|v|v.staged_backing_identity()),page);assert_eq!(reads.get(),prior_reads);}
   let(done,heap)=observe(||original.turn(exact(d),|cx|boxed_turn(&mut owner,&source,cx)));complete=done;assert_eq!(original.caller.receipt().copied_bytes,d.copy_bytes);assert_eq!(original.caller.receipt().copied_items,1);assert_eq!(heap,(d.capacity_bytes,d.release_bytes));born+=heap.0;released+=heap.1;turns+=1;
  }
  if complete{assert_eq!(owner.as_ref().unwrap().published().unwrap().page(0),Some(bytes.as_slice()));}
  original.caller.cancel().cancel_now();let d=boxed_demand(&owner,&source);let pointer=owner.as_ref().map(|v|&**v as*const _);let prior_reads=reads.get();
  let(_,heap)=observe(||original.turn(exact(d),|cx|boxed_turn(&mut owner,&source,cx)));assert_eq!(heap,(0,0));assert_eq!(original.caller.receipt(),Default::default());assert_eq!(owner.as_ref().map(|v|&**v as*const _),pointer);assert_eq!(reads.get(),prior_reads);
  while owner.as_ref().is_some_and(|v|!v.terminal_is_empty()){let b=owner.as_mut().unwrap();let d=b.retirement_demands().unwrap();let(step,heap)=observe(||original.turn(exact(d),|cx|close_original_retained_step(cx,d,|g|b.close_step_granted(g))));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}
  if owner.is_some(){let d=RetainedPayloadBuilder::boxed_release_demands();let(_,heap)=observe(||original.turn(exact(d),|cx|close_original_retained_step(cx,d,|_|{drop(owner.take());Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))})));assert_eq!(heap.1,d.release_bytes);released+=heap.1;}
  assert_eq!(born,released);original.finish();
 }
}
