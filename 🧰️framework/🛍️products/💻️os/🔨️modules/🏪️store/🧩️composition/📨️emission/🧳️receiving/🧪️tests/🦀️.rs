use super::*;
use semio_framework_value::ErasedSnapshotRetirement;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:2,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)}}
fn observe<R>(f:impl FnOnce()->R)->(R,semio_framework_trace::HeapAllocationObservation){semio_framework_trace::observe_heap_allocations_on_this_thread(f)}
fn physical(p:RetainedCloneProgress,h:semio_framework_trace::HeapAllocationObservation,g:RetainedCloneGrant){assert!(p.fits(g));assert_eq!((p.retained_capacity_bytes,p.released_bytes),(h.requested_bytes,h.released_bytes));}
use semio_framework_value_clone_source_testing::drive::drive;
use semio_framework_value_testing_admission::Authorities;
type Owner=Box<dyn MemberOriginalBatchReceiving>;
const WORK:u64=1<<20;
const STALL_TURNS:usize=64;
fn advance(payer:&mut Authorities,control:&mut NativeDecodeControl<'_>,owner:&mut Owner,g:RetainedCloneGrant)->(RetainedCloneStep,semio_framework_trace::HeapAllocationObservation){
 let mut heap=None;control.admit_turn_capacity(g.maximum_capacity_bytes).unwrap();
 let poll=payer.nested(control,g,|control,_|{let(poll,observed)=observe(||owner.advance(control,WORK,g));heap=Some(observed);poll});
 let RetainedWorkPoll::Complete(turn)=poll else{panic!("original member receiver advance stayed pending")};
 (turn.result.unwrap_or_else(|failure|panic!("original member receiver advance failed: {:?}",failure.primary)),heap.expect("observed advance"))
}
fn retire(control:&mut NativeDecodeControl<'_>,owner:&mut Owner,g:RetainedCloneGrant,settle:bool,pendings:&mut usize)->RetainedCloneStep{
 let turn=drive(g,WORK,|effect|{let poll=control.with_retirement(g,|r|Ok::<_,ValueError>(if settle{owner.settle(r,effect)}else{owner.close_step(r,effect)})).unwrap();if matches!(poll,RetainedWorkPoll::Pending{..}){*pendings+=1}poll});
 turn.result.unwrap_or_else(|failure|panic!("original member receiver retirement failed: {:?}",failure.primary()))
}
fn run(row:&serde_json::Value,cut:Option<usize>)->usize{
 let originals:Vec<Vec<u8>>=row.as_array().unwrap().iter().map(|v|format!("op{}",serde_json::to_string(v).unwrap()).into_bytes()).collect();
 let pointer=originals.as_ptr();let wire_pointers:Vec<_>=originals.iter().map(|v|v.as_ptr()).collect();let mut originals=Some(originals);
 let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<u64>>=Arc::new(semio_framework_value_factory::OwnedValueRetirementFactory::<u64>::default());
 let codec=ArtifactOperationWire::<u64>::canonical().with_original_receiving();let mut accepted=|_|true;
 let mut recipient=semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();let mut control=NativeDecodeControl::new_retained(&mut accepted);control.install_retirement_recipient(&mut recipient).unwrap();
 let mut payer=Authorities::new();
 let d=OriginalMemberBatchReceiving::<u64>::admission_demands();let g=RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.max(4096),..grant(d)};
 let mut heap=None;let poll=payer.nested(&mut control,g,|control,_|{let(result,observed)=observe(||OriginalMemberBatchReceiving::admit(&mut originals,codec,&factory,control,g));heap=Some(observed);RetainedWorkPoll::<_,ValueError>::Complete(RetainedWorkOutcome{result,work:1})});
 let RetainedWorkPoll::Complete(turn)=poll else{panic!("original member admission stayed pending")};
 let(mut owner,p)=turn.result.unwrap().expect("original member receiver admitted");physical(p,heap.expect("observed admission"),g);assert!(originals.is_none());
 let mut pendings=0;let mut turns=0;let mut output=None;let mut previous=None;let mut still=0usize;let mut last=RetainedCloneProgress::default();
 loop{
  if cut==Some(turns)||owner.ready(){break}
  let d=owner.demands(4096).unwrap();if previous==Some(d){still+=1}else{previous=Some(d);still=0}assert!(still<STALL_TURNS,"original member receiver demand stayed {d:?} for {STALL_TURNS} turns at turn={turns}, settling={}, last progress {last:?}",owner.settling());let g=grant(d);
  for axis in 0..5{
   if axis==1&&d.copy_bytes==0||axis==2&&d.capacity_bytes==0||axis==3&&d.release_bytes==0{continue}
   let denied=match axis{0=>RetainedCloneGrant{maximum_items:0,..g},1=>RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g},2=>RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..g},3=>RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..g},_=>RetainedCloneGrant{maximum_depth:d.depth-1,..g}};
   if owner.settling(){let step=retire(&mut control,&mut owner,denied,true,&mut pendings);assert_eq!(step.progress(),Default::default());}else{let(step,heap)=advance(&mut payer,&mut control,&mut owner,denied);physical(step.progress(),heap,denied);assert_eq!(step.progress(),Default::default());}
   assert_eq!(owner.demands(4096).unwrap(),d);
  }
  let progress=if owner.settling(){retire(&mut control,&mut owner,g,true,&mut pendings).progress()}else{let(step,heap)=advance(&mut payer,&mut control,&mut owner,g);physical(step.progress(),heap,g);step.progress()};
  assert!(progress!=Default::default()||owner.ready(),"original member receiver made no progress at turn={turns}, settling={}, demand={d:?}",owner.settling());last=progress;turns+=1;
 }
 if cut.is_none(){assert!(owner.ready());let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:64};let(step,heap)=observe(||owner.take(&mut output,&mut originals,&control,g).unwrap());physical(step.progress(),heap,g);assert_eq!(originals.as_ref().unwrap().as_ptr(),pointer);for(wire,pointer)in originals.as_ref().unwrap().iter().zip(wire_pointers){assert_eq!(wire.as_ptr(),pointer)}assert_eq!(serde_json::to_value(output.as_ref().unwrap().mutations::<u64>().unwrap()).unwrap(),*row);}
 owner.begin_close();for _ in 0..100000{if owner.terminal_is_empty(){break}let d=owner.close_demands(4096).unwrap();retire(&mut control,&mut owner,grant(d),false,&mut pendings);}assert!(owner.terminal_is_empty());let bytes=owner.allocation_bytes();let((),heap)=observe(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,bytes));
 drop(control);assert!(recipient.terminal_is_empty());
 if let Some(batch)=output.as_mut(){for _ in 0..100000{if batch.terminal_is_empty(){break}let d=batch.next_demands(4096).unwrap();let g=grant(d);let mut actual=None;let(turn,_)=semio_framework_value::testing::with_effect(g,WORK,&mut actual,|effect|observe(||batch.close_granted(effect)));let(step,heap)=(turn.0.unwrap(),turn.1);physical(step.progress(),heap,g);}assert!(batch.terminal_is_empty());}
 pendings
}
#[test]
fn original_member_wire_batch_preserves_sources_and_exact_typed_receipts(){let corpus:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();for row in corpus["cases"].as_array().unwrap(){run(row,None);for cut in corpus["cancelPrefixes"].as_array().unwrap(){run(row,Some(cut.as_u64().unwrap()as usize));}}}
/// 🔁️ A pending original handback keeps its resident route, re-enters as recovery and returns each wire exactly once.
#[test]
fn original_member_close_recovers_every_pending_original_handback_exactly_once(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();let mut pendings=0;
 for row in corpus["cases"].as_array().unwrap(){pendings+=run(row,None);for cut in corpus["cancelPrefixes"].as_array().unwrap(){pendings+=run(row,Some(cut.as_u64().unwrap()as usize));}}
 assert!(pendings>0,"no original handback ever stayed pending");
}
