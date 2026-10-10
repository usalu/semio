//! 📸️ Same original app workspace and ARC1 bytes survive independent denials and paid metadata closure.
use super::*;
use semio_framework_trace::observe_heap_allocations_on_this_thread;
#[test]
fn original_command_checkpoint_cursor_preserves_every_arc1_byte_and_zero_copy_metadata(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 let original=law["originalGrant"].as_array().unwrap();let grant=RetainedCloneGrant{maximum_items:original[0].as_u64().unwrap()as usize,maximum_copy_bytes:original[1].as_u64().unwrap()as usize,maximum_capacity_bytes:original[2].as_u64().unwrap()as usize,maximum_release_bytes:original[3].as_u64().unwrap()as usize,maximum_depth:original[4].as_u64().unwrap()as usize};
 for row in law["cases"].as_array().unwrap(){
  let work:Vec<u8>=row["work"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect();let foreign=work.clone();let values:[u64;5]=std::array::from_fn(|index|row["values"][index].as_u64().unwrap());let work_phase=row["workPhase"].as_bool().unwrap();let read=|work:&Vec<u8>,index:usize|work.get(index).copied();
  let mut expected=vec![0;48+work.len()];expected[..4].copy_from_slice(b"ARC1");expected[4]=3;expected[5]=u8::from(work_phase);for(index,value)in values.into_iter().enumerate(){expected[8+index*8..16+index*8].copy_from_slice(&value.to_le_bytes())}expected[48..].copy_from_slice(&work);
  for cut in[0,1,17,512,usize::MAX]{
   let mut target=[MaybeUninit::uninit();512];let mut other=[MaybeUninit::uninit();512];let target_pointer=target.as_ptr();let work_pointer=work.as_ptr();let mut cursor=ArtifactCommandCheckpointCursor::new(work_phase,values);let mut copied=0;let mut turns=0;
   while !cursor.is_complete()&&turns<cut{
    let(demand,heap)=observe_heap_allocations_on_this_thread(||cursor.advance_demands(&work,&target,read).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    for denied in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
     let before=cursor.written();let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.advance_one(&work,&mut target,read,denied).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(cursor.written(),before);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    }
    if demand.copy_bytes>0{let before=cursor.written();let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.advance_one(&work,&mut target,read,RetainedCloneGrant{maximum_copy_bytes:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(cursor.written(),before);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
    let incoming=RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes,..grant};let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.advance_one(&work,&mut target,read,incoming).unwrap());let progress=step.progress();assert!(progress.fits(incoming));assert_eq!(progress.copied_bytes,demand.copy_bytes);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));copied+=progress.copied_bytes;turns+=1;
    let(bytes,heap)=observe_heap_allocations_on_this_thread(||cursor.bytes(&target).unwrap());assert_eq!(bytes,&expected[..cursor.written()]);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(target.as_ptr(),target_pointer);assert_eq!(work.as_ptr(),work_pointer);assert!(cursor.advance_one(&foreign,&mut target,read,grant).is_err());assert!(cursor.advance_one(&work,&mut other,read,grant).is_err());
   }
   assert_eq!(copied,cursor.written());if cut==usize::MAX{assert!(cursor.is_complete());assert_eq!(cursor.bytes(&target).unwrap(),expected);assert_eq!(copied,expected.len());}
   let before=cursor.written();let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.close_step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(cursor.written(),before);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.close_step(RetainedCloneGrant{maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,..grant}).unwrap());assert!(cursor.terminal_is_empty());assert_eq!(step.progress().copied_bytes,0);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(_,heap)=observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] original ARC1 bytes={} cut={cut} turns={turns} copy={copied} metadata0 sameWork=true sameTarget=true terminalDrop0",expected.len());
  }
 }
}

