//! 🧪️ Original boxed edits match Node framed SHA256 through exact four-axis ownership and cancellation.
use super::*;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use crate::retained_sources::{stage,extracted,closed,polled,stepped};

fn grant(demand:RetirementDemand)->RetainedCloneGrant{assert!(demand.copy_bytes+demand.capacity_bytes<=4096);RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}
fn hex(bytes:[u8;32])->String{bytes.iter().map(|byte|format!("{byte:02x}")).collect()}

#[test]
fn canonical_native_source_handoff_refuses_extracted_original_without_losing_custody(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let edit=Box::new(semio_framework_pack_json::from_json_str::<Edit<bool>>(&serde_json::to_string(&cases["cases"][0]["edit"]).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
 let pointer=edit.as_ref()as*const Edit<bool>;let id=edit.id.as_ptr();
 let admitted=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:131072,maximum_depth:4096};
 let mut source=stage(edit);
 let extracted=extracted(&mut source,admitted).expect("original authority transfer");let mut source=Some(source);
 let(result,heap)=observe(||ArtifactCanonicalEditSealCursor::admit_source(&mut source,admitted));assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(source.is_some());assert_eq!(extracted.as_ref()as*const Edit<bool>,pointer);assert_eq!(extracted.id.as_ptr(),id);
 let mut source=source.unwrap();for turn in 0..100000{if source.terminal_is_empty(){break;}let(step,_)=closed(&mut source,admitted);assert!(step.progress().fits(admitted));assert!(turn<99999||source.terminal_is_empty());}assert!(source.terminal_is_empty());
 let edit=extracted;assert_eq!(edit.as_ref()as*const Edit<bool>,pointer);assert_eq!(edit.id.as_ptr(),id);let mut owner=ControlledRetirement::new(edit).unwrap_or_else(|(error,_)|panic!("extracted original retirement: {error}"));for turn in 0..100000{if owner.terminal_is_empty(){break;}let(step,_)=stepped(&mut owner,admitted);assert!(step.progress().fits(admitted));assert!(turn<99999||owner.terminal_is_empty());}assert!(owner.terminal_is_empty());let(_,heap)=observe(||{drop(owner);drop(source);});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
}

#[test]
fn canonical_native_edit_seal_preserves_original_box_and_fields_after_framed_digest_and_cancellation(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for(index,row)in cases["cases"].as_array().unwrap().iter().enumerate(){
  let mut pauses:Vec<(Option<usize>,Option<Phase>,bool)>=fixture["pauses"].as_array().unwrap().iter().map(|value|(Some(value.as_u64().unwrap()as usize),None,false)).collect();
  for phase in [Phase::Header,Phase::HashStart,Phase::Hash,Phase::TakeOwner,Phase::SourceClose,Phase::Finalize,Phase::Ready]{pauses.push((None,Some(phase),false));}pauses.push((None,None,true));pauses.push((None,None,false));
  for(pause,phase_pause,pending_pause)in pauses{for _ in[()]{
   let mut original=Some(Box::new(semio_framework_pack_json::from_json_str::<Edit<bool>>(&serde_json::to_string(&row["edit"]).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()));
   original.as_mut().unwrap().id.reserve(8192);let pointer=original.as_ref().unwrap().as_ref()as*const Edit<bool>;let id=original.as_ref().unwrap().id.as_ptr();let forwards=original.as_ref().unwrap().forwards.as_ptr();
   let(mut cursor,progress,admitted,heap)={
    let source=stage(original.take().unwrap());let mut source=Some(source);
    let(demand,heap)=observe(||ArtifactCanonicalEditSealCursor::<bool>::source_constructor_demand());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(demand.capacity_bytes,0);assert_eq!(demand.release_bytes,0);let admitted=grant(demand);
    for refused in[RetainedCloneGrant::default(),RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..admitted},RetainedCloneGrant{maximum_depth:demand.depth-1,..admitted}]{let(result,heap)=observe(||ArtifactCanonicalEditSealCursor::admit_source(&mut source,refused).unwrap());assert!(result.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(source.as_ref().unwrap().borrow().get().as_ref()as*const Edit<bool>,pointer);assert_eq!(source.as_ref().unwrap().borrow().get().id.as_ptr(),id);}
    let(result,heap)=observe(||ArtifactCanonicalEditSealCursor::admit_source(&mut source,admitted).unwrap());assert!(source.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(cursor,progress)=result.unwrap();(cursor,progress,admitted,heap)
   };assert!(original.is_none());assert!(progress.fits(admitted));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
   let mut cancelled=false;let mut turns=0;
   while !cursor.terminal_is_empty(){
    assert!(turns<100000,"original native seal exceeded fixed turn bound");
    if !cancelled&&(pause==Some(turns)||phase_pause==Some(cursor.phase)||pending_pause&&cursor.hash_pending){cursor.begin_close();cancelled=true;}
    if cursor.is_ready(){
     assert!(!cancelled);assert_eq!(cursor.canonical_length(),fixture["rows"][index]["canonicalLength"].as_u64().unwrap());assert_eq!(hex(cursor.digest().unwrap()),fixture["rows"][index]["digest"].as_str().unwrap());
     let(result,heap)=observe(||cursor.take_edit(RetainedCloneGrant::default()).unwrap());assert!(result.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
     let(demand,heap)=observe(||cursor.next_take_demand());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let admitted=grant(demand);let(result,heap)=observe(||cursor.take_edit(admitted).unwrap());let(edit,_,progress)=result.unwrap();assert!(progress.fits(admitted));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(edit.as_ref()as*const Edit<bool>,pointer);assert_eq!(edit.id.as_ptr(),id);assert_eq!(edit.forwards.as_ptr(),forwards);
     let admitted=grant(copy(size_of::<ControlledRetirement<Box<Edit<bool>>>>()+size_of::<Box<Edit<bool>>>()));let(owner,heap)=observe(||ControlledRetirement::new(edit).unwrap_or_else(|(error,_)|panic!("original returned edit: {error}")));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(admitted.maximum_copy_bytes>0);let mut owner=owner;
     for turn in 0..100000{if owner.terminal_is_empty(){break;}let work=owner.next_copy_byte_demand().unwrap();let admitted=grant(RetirementDemand{copy_bytes:work,capacity_bytes:owner.next_capacity_byte_demand(work).unwrap(),release_bytes:owner.next_release_byte_demand().unwrap(),depth:owner.next_depth_demand().unwrap()});let(step,_)=stepped(&mut owner,admitted);assert!(step.progress().fits(admitted));assert!(turn<99999||owner.terminal_is_empty());}assert!(owner.terminal_is_empty());let(_,heap)=observe(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));break;
    }
    let(demand,heap)=observe(||cursor.next_demand().unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let admitted=grant(demand);
     let work_contract:serde_json::Value=serde_json::from_str(include_str!("work.json")).unwrap();
     let(zero,spent)=polled(RetainedCloneGrant::default(),|effect|cursor.advance(effect));assert_eq!(zero.progress(),RetainedCloneProgress::default());assert_eq!(cursor.last_turn_work(),u128::from(work_contract["deniedWork"].as_u64().unwrap()));assert_eq!(spent,(0,0));
    for refused in [RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..admitted},RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..admitted},RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..admitted},RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..admitted}]{if refused==admitted{continue;}let(step,spent)=polled(refused,|effect|cursor.advance(effect));assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(spent,(0,0));}
     let bound=cursor.next_work_demand().unwrap();let(step,_)=polled(admitted,|effect|cursor.advance(effect));assert!(cursor.last_turn_work()<=u128::from(bound));assert!(step.progress().fits(admitted));turns+=1;
   }
   assert!(cursor.terminal_is_empty());let(_,heap)=observe(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  }}
 }
}

#[test]
fn canonical_seal_terminal_polling_preserves_one_shot_original_transfer(){
 let contract:serde_json::Value=serde_json::from_str(include_str!("terminal.json")).unwrap();
 let hashes:serde_json::Value=serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for(index,row)in cases["cases"].as_array().unwrap().iter().enumerate(){for cancel in [false,true]{
  let edit=Box::new(semio_framework_pack_json::from_json_str::<Edit<bool>>(&serde_json::to_string(&row["edit"]).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
  let pointer=edit.as_ref()as*const Edit<bool>;let id=edit.id.as_ptr();let mut source=Some(stage(edit));
  let(mut cursor,_)=ArtifactCanonicalEditSealCursor::admit_source(&mut source,grant(ArtifactCanonicalEditSealCursor::<bool>::source_constructor_demand())).unwrap().unwrap();
  if cancel{cursor.begin_close();}
  let mut transferred=None;let mut transfers=0;
  for turn in 0..contract["maximumTurns"].as_u64().unwrap(){
   if cursor.terminal_is_empty(){break;}
   if cursor.is_ready(){
    assert!(!cancel);let admitted=grant(cursor.next_take_demand());
    let(edit,digest,receipt)=cursor.take_edit(admitted).unwrap().unwrap();assert!(receipt.fits(admitted));
    assert_eq!(edit.as_ref()as*const Edit<bool>,pointer);assert_eq!(edit.id.as_ptr(),id);
    assert_eq!(hex(digest),hashes["rows"][index]["digest"].as_str().unwrap());transfers+=1;transferred=Some(edit);
   }else{let admitted=grant(cursor.next_demand().unwrap());assert!(polled(admitted,|effect|cursor.advance(effect)).0.progress().fits(admitted));}
   assert!(turn+1<contract["maximumTurns"].as_u64().unwrap()||cursor.terminal_is_empty());
  }
  assert!(cursor.terminal_is_empty());assert!(!cursor.is_ready());assert!(cursor.digest().is_none());
  for _ in 0..contract["terminalPolls"].as_u64().unwrap(){
   assert_eq!(cursor.next_demand().unwrap(),RetirementDemand::default());
   for admitted in [grant(RetirementDemand::default()),RetainedCloneGrant::default()]{
    let(step,spent)=polled(admitted,|effect|cursor.advance(effect));
    assert!(matches!(step,RetainedCloneStep::Complete(_)));assert_eq!(serde_json::json!({"copied_items":step.progress().copied_items,"copied_bytes":step.progress().copied_bytes,"retained_capacity_bytes":step.progress().retained_capacity_bytes,"released_bytes":step.progress().released_bytes}),contract["terminalProgress"]);assert_eq!(spent,(0,0));
    assert!(cursor.take_edit(admitted).unwrap().is_none());
   }
  }
  assert_eq!(transfers,if cancel{0}else{contract["expectedTransfers"].as_u64().unwrap()});
  if let Some(edit)=transferred{let mut owner=ControlledRetirement::new(edit).unwrap_or_else(|(error,_)|panic!("returned original retirement: {error}"));for _ in 0..contract["maximumTurns"].as_u64().unwrap(){if owner.terminal_is_empty(){break;}let body=owner.next_copy_byte_demand().unwrap();let admitted=grant(RetirementDemand{copy_bytes:body,capacity_bytes:owner.next_capacity_byte_demand(body).unwrap(),release_bytes:owner.next_release_byte_demand().unwrap(),depth:owner.next_depth_demand().unwrap()});assert!(stepped(&mut owner,admitted).0.progress().fits(admitted));}assert!(owner.terminal_is_empty());}
 }}
}
