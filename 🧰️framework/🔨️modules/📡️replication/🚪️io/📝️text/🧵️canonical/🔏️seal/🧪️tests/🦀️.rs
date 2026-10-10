//! 🧪️ Original boxed edits match Node framed SHA256 through exact four-axis ownership and cancellation.
use super::*;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;

fn grant(demand:RetirementDemand)->RetainedCloneGrant{assert!(demand.copy_bytes+demand.capacity_bytes<=4096);RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}
fn hex(bytes:[u8;32])->String{bytes.iter().map(|byte|format!("{byte:02x}")).collect()}

#[test]
fn canonical_native_source_handoff_refuses_extracted_original_without_losing_custody(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
 let edit=Box::new(semio_framework_pack_json::from_json_str::<Edit<bool>>(&serde_json::to_string(&cases["cases"][0]["edit"]).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
 let pointer=edit.as_ref()as*const Edit<bool>;let id=edit.id.as_ptr();
 let admitted=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:131072,maximum_depth:4096};
 let(mut source,_)=RetainedCloneSource::admit_owned(edit,(),admitted).unwrap_or_else(|(error,_,_)|panic!("original live source: {error}"));
 let extracted=source.take_owner().unwrap();let mut source=Some(source);
 let(result,heap)=observe(||ArtifactCanonicalEditSealCursor::admit_source(&mut source,admitted));assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(source.is_some());assert_eq!(extracted.as_ref().as_ref()as*const Edit<bool>,pointer);assert_eq!(extracted.id.as_ptr(),id);
 let mut source=source.unwrap();for turn in 0..100000{if source.terminal_is_empty(){break;}let(step,heap)=observe(||source.close_step(admitted).unwrap());assert!(step.progress().fits(admitted));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(turn<99999||source.terminal_is_empty());}assert!(source.terminal_is_empty());
 let edit=Arc::try_unwrap(extracted).unwrap_or_else(|_|panic!("paid original source aliases closed"));assert_eq!(edit.as_ref()as*const Edit<bool>,pointer);assert_eq!(edit.id.as_ptr(),id);let mut owner=ControlledRetirement::new(edit).unwrap_or_else(|(error,_)|panic!("extracted original retirement: {error}"));for turn in 0..100000{if owner.terminal_is_empty(){break;}let(step,heap)=observe(||owner.step(admitted).unwrap());assert!(step.progress().fits(admitted));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(turn<99999||owner.terminal_is_empty());}assert!(owner.terminal_is_empty());let(_,heap)=observe(||{drop(owner);drop(source);});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 println!("[DEBUG] native extracted original source refuses admitted handoff heap0 and retains exact Box/id pointers; fixed4096 source/owner closure and terminal drop0");
}

#[test]
fn canonical_native_edit_seal_preserves_original_box_and_fields_after_framed_digest_and_cancellation(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
 for(index,row)in cases["cases"].as_array().unwrap().iter().enumerate(){
  let mut pauses:Vec<(Option<usize>,Option<Phase>,bool)>=fixture["pauses"].as_array().unwrap().iter().map(|value|(Some(value.as_u64().unwrap()as usize),None,false)).collect();
  for phase in [Phase::Header,Phase::HashStart,Phase::Hash,Phase::TakeOwner,Phase::SourceClose,Phase::Unwrap,Phase::Finalize,Phase::Ready]{pauses.push((None,Some(phase),false));}pauses.push((None,None,true));pauses.push((None,None,false));
  for(pause,phase_pause,pending_pause)in pauses{for source_mode in[false,true]{
   let mut original=Some(Box::new(semio_framework_pack_json::from_json_str::<Edit<bool>>(&serde_json::to_string(&row["edit"]).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()));
   original.as_mut().unwrap().id.reserve(8192);let pointer=original.as_ref().unwrap().as_ref()as*const Edit<bool>;let id=original.as_ref().unwrap().id.as_ptr();let forwards=original.as_ref().unwrap().forwards.as_ptr();
   let(mut cursor,progress,admitted,heap)=if source_mode{
    let birth=RetainedCloneSource::<Box<Edit<bool>>>::owned_constructor_demand::<()>();let source_grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:birth.capacity_bytes,maximum_depth:birth.depth,..Default::default()};let((source,receipt),heap)=observe(||RetainedCloneSource::admit_owned(original.take().unwrap(),(),source_grant).unwrap_or_else(|(error,_,_)|panic!("same admitted source: {error}")));assert!(receipt.fits(source_grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));let mut source=Some(source);
    let(demand,heap)=observe(||ArtifactCanonicalEditSealCursor::<bool>::source_constructor_demand());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(demand.capacity_bytes,0);assert_eq!(demand.release_bytes,0);let admitted=grant(demand);
    for refused in[RetainedCloneGrant::default(),RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..admitted},RetainedCloneGrant{maximum_depth:demand.depth-1,..admitted}]{let(result,heap)=observe(||ArtifactCanonicalEditSealCursor::admit_source(&mut source,refused).unwrap());assert!(result.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(source.as_ref().unwrap().borrow().get().as_ref()as*const Edit<bool>,pointer);assert_eq!(source.as_ref().unwrap().borrow().get().id.as_ptr(),id);}
    let(result,heap)=observe(||ArtifactCanonicalEditSealCursor::admit_source(&mut source,admitted).unwrap());assert!(source.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(cursor,progress)=result.unwrap();(cursor,progress,admitted,heap)
   }else{
   let demand=ArtifactCanonicalEditSealCursor::<bool>::constructor_demand();let admitted=grant(demand);
   for refused in [RetainedCloneGrant::default(),RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..admitted},RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..admitted},RetainedCloneGrant{maximum_depth:demand.depth-1,..admitted}]{let(result,heap)=observe(||ArtifactCanonicalEditSealCursor::admit_original(&mut original,refused).unwrap());assert!(result.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(original.as_ref().unwrap().as_ref()as*const Edit<bool>,pointer);assert_eq!(original.as_ref().unwrap().id.as_ptr(),id);}
   let(result,heap)=observe(||ArtifactCanonicalEditSealCursor::admit_original(&mut original,admitted).unwrap());let(cursor,progress)=result.unwrap();(cursor,progress,admitted,heap)
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
     for turn in 0..100000{if owner.terminal_is_empty(){break;}let work=owner.next_copy_byte_demand().unwrap();let admitted=grant(RetirementDemand{copy_bytes:work,capacity_bytes:owner.next_capacity_byte_demand(work).unwrap(),release_bytes:owner.next_release_byte_demand().unwrap(),depth:owner.next_depth_demand().unwrap()});let(step,heap)=observe(||owner.step(admitted).unwrap());assert!(step.progress().fits(admitted));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(turn<99999||owner.terminal_is_empty());}assert!(owner.terminal_is_empty());let(_,heap)=observe(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));break;
    }
    let(demand,heap)=observe(||cursor.next_demand().unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let admitted=grant(demand);
    let(zero,heap)=observe(||cursor.advance(RetainedCloneGrant::default()).unwrap());assert_eq!(zero.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    for refused in [RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..admitted},RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..admitted},RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..admitted},RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..admitted}]{if refused==admitted{continue;}let(step,heap)=observe(||cursor.advance(refused).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
    let(step,heap)=observe(||cursor.advance(admitted).unwrap());assert!(step.progress().fits(admitted));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));turns+=1;
   }
   assert!(cursor.terminal_is_empty());let(_,heap)=observe(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   println!("[DEBUG] native original edit seal case={index} same_source={source_mode} pause={pause:?} phase={} pending={pending_pause} cancelled={cancelled} turns={turns} NodeSHA256/Serde digest exact; original Box/id/forwards pointers; zero/below/full 4096 heap and terminal drop0",phase_pause.map_or(0,|phase|phase as u8+1));
  }}
 }
}
