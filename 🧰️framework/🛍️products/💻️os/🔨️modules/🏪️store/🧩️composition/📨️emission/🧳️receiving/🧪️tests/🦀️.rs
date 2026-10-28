use super::*;
use semio_framework_value::ErasedSnapshotRetirement;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)}}
fn observe<R>(f:impl FnOnce()->R)->(R,semio_framework_trace::HeapAllocationObservation){semio_framework_trace::observe_heap_allocations_on_this_thread(f)}
fn physical(p:RetainedCloneProgress,h:semio_framework_trace::HeapAllocationObservation,g:RetainedCloneGrant){assert!(p.fits(g));assert_eq!((p.retained_capacity_bytes,p.released_bytes),(h.requested_bytes,h.released_bytes));}
struct Construction(RetainedCloneGrant);
impl semio_framework_value_native_owned::native_decoding::NativeDecodeConstructionRecipient for Construction{
 fn grant(&mut self,d:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{if !retained_clone_grant_admits(self.0,d){return Err(invalid())}Ok(self.0)}
 fn accept(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),ValueError>{if g!=self.0||!p.fits(g){return Err(invalid())}Ok(())}
}
fn run(row:&serde_json::Value,cut:Option<usize>){
 let originals:Vec<Vec<u8>>=row.as_array().unwrap().iter().map(|v|format!("op{}",serde_json::to_string(v).unwrap()).into_bytes()).collect();
 let pointer=originals.as_ptr();let wire_pointers:Vec<_>=originals.iter().map(|v|v.as_ptr()).collect();let mut originals=Some(originals);
 let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<u64>>=Arc::new(semio_framework_value_factory::OwnedValueRetirementFactory::<u64>::default());
 let codec=ArtifactOperationWire::<u64>::canonical().with_original_receiving();let mut accepted=|_|true;
 let mut recipient=semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();let mut control=NativeDecodeControl::new_retained(&mut accepted);control.install_retirement_recipient(&mut recipient).unwrap();
 let d=OriginalMemberBatchReceiving::<u64>::admission_demands();let g=RetainedCloneGrant{maximum_copy_bytes:4096,..grant(d)};
 let ((mut owner,p),heap)=observe(||OriginalMemberBatchReceiving::admit(&mut originals,codec,&factory,&mut control,g).unwrap().unwrap());physical(p,heap,g);assert!(originals.is_none());
 let mut turns=0;let mut output=None;
 loop{
  if cut==Some(turns)||owner.ready(){break}assert!(turns<100000);
  let d=owner.demands(4096).unwrap();let g=grant(d);
  for axis in 0..5{
   if axis==1&&d.copy_bytes==0||axis==2&&d.capacity_bytes==0||axis==3&&d.release_bytes==0{continue}
   let denied=match axis{0=>RetainedCloneGrant{maximum_items:0,..g},1=>RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g},2=>RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..g},3=>RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..g},_=>RetainedCloneGrant{maximum_depth:d.depth-1,..g}};
   let (step,heap)=observe(||if owner.settling(){control.with_retirement(g,|r|owner.settle(r,denied))}else{owner.advance(&mut control,denied)}.unwrap());physical(step.progress(),heap,denied);assert_eq!(step.progress(),Default::default());assert_eq!(owner.demands(4096).unwrap(),d);
  }
  let mut construction=Construction(g);let (step,heap)=observe(||if owner.settling(){control.with_retirement(g,|r|owner.settle(r,g))}else{control.with_construction_recipient(&mut construction,|c|owner.advance(c,g))}.unwrap());physical(step.progress(),heap,g);assert!(step.progress()!=Default::default()||owner.ready(),"[DEBUG] original member receiver made no progress at turn={turns}, settling={}, demand={d:?}",owner.settling());turns+=1;
 }
 if cut.is_none(){assert!(owner.ready());let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:64};let(step,heap)=observe(||owner.take(&mut output,&mut originals,&control,g).unwrap());physical(step.progress(),heap,g);assert_eq!(originals.as_ref().unwrap().as_ptr(),pointer);for(wire,pointer)in originals.as_ref().unwrap().iter().zip(wire_pointers){assert_eq!(wire.as_ptr(),pointer)}assert_eq!(serde_json::to_value(output.as_ref().unwrap().mutations::<u64>().unwrap()).unwrap(),*row);}
 owner.begin_close();for _ in 0..100000{if owner.terminal_is_empty(){break}let d=owner.close_demands(4096).unwrap();let g=grant(d);let(step,heap)=observe(||control.with_retirement(g,|r|owner.close_step(r,g)).unwrap());physical(step.progress(),heap,g);}assert!(owner.terminal_is_empty());let bytes=owner.allocation_bytes();let((),heap)=observe(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,bytes));
 drop(control);assert!(recipient.terminal_is_empty());
 if let Some(batch)=output.as_mut(){for _ in 0..100000{if batch.terminal_is_empty(){break}let d=batch.next_demands(4096).unwrap();let g=grant(d);let(step,heap)=observe(||batch.close_granted(g).unwrap());physical(step.progress(),heap,g);}assert!(batch.terminal_is_empty());}
}
#[test]
fn original_member_wire_batch_preserves_sources_and_exact_typed_receipts(){let corpus:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();for row in corpus["cases"].as_array().unwrap(){run(row,None);for cut in corpus["cancelPrefixes"].as_array().unwrap(){run(row,Some(cut.as_u64().unwrap()as usize));}}}
