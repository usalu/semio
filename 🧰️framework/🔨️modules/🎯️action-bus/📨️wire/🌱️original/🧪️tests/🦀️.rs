use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,OriginalTurnFailure,StepContext,default_now_us,admit_original_construction,OriginalConstructionDemand,close_original_retained_step};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand};
const TURN:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:16<<20,maximum_release_bytes:16<<20,maximum_depth:128};
fn caller()->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OperationId(20751),Generation(52),RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<29,maximum_release_bytes:1<<29,maximum_depth:128},default_now_us,u64::MAX,1<<30)}
fn event<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,g:RetainedCloneGrant,f:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(g,decoder,f));let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result.unwrap()}
fn original(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
fn close_demand(input:&RetainedToolWireInput)->RetirementDemand{RetirementDemand{copy_bytes:input.next_close_copy_byte_demand().unwrap(),release_bytes:input.next_close_release_byte_demand().unwrap(),depth:input.next_close_depth_demand().unwrap(),..Default::default()}}
fn denied(d:RetirementDemand,axis:usize)->Option<RetainedCloneGrant>{let mut g=TURN;match axis{0=>g.maximum_items=0,1 if d.copy_bytes>0=>g.maximum_copy_bytes=d.copy_bytes-1,2 if d.capacity_bytes>0=>g.maximum_capacity_bytes=d.capacity_bytes-1,3 if d.release_bytes>0=>g.maximum_release_bytes=d.release_bytes-1,4 if d.depth>0=>g.maximum_depth=d.depth-1,_=>return None}Some(g)}
#[doc(hidden)]
pub fn original_command_wire_backing_is_admitted_before_pages_and_denied_close_preserves_the_same_owner(){
 let law:semio_framework_value::serde_json::Value=semio_framework_value::serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){let bytes=row["bytes"].as_u64().unwrap()as usize;let pages=row["pages"].as_u64().unwrap()as usize;
  for cut in law["cancelAfterPages"].as_array().unwrap().iter().map(|cut|Some(cut.as_u64().unwrap()as usize)).chain([None]){
   let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let mut input=None;let d=RetainedToolWireInput::original_birth_demands(bytes,bytes).unwrap();
   for axis in 0..5{let Some(g)=denied(d,axis)else{continue};assert!(event(&mut caller,&mut decoder,g,|_|Ok(RetainedToolWireInput::admit_original(bytes,bytes,g)?)).is_none());assert!(input.is_none());assert_eq!(caller.receipt(),Default::default());}
   event(&mut caller,&mut decoder,TURN,|cx|admit_original_construction(cx,&mut input,original(d),|_|Ok(RetainedToolWireInput::admit_original(bytes,bytes,TURN)?.unwrap())));
   let input=input.as_mut().unwrap();assert_eq!(input.pages.capacity(),pages);let pointer=input.pages.as_ptr();
   for page in 0..pages{if cut==Some(page){break}let offset=page*TOOL_WIRE_PAGE_BYTES;let count=(bytes-offset).min(TOOL_WIRE_PAGE_BYTES);let d=RetirementDemand{copy_bytes:3*size_of::<ToolWirePage>()+count+3*size_of::<usize>(),depth:1,..Default::default()};event(&mut caller,&mut decoder,TURN,|cx|{let mut done=None;admit_original_construction(cx,&mut done,original(d),|_|{let mut scratch=[0;TOOL_WIRE_PAGE_BYTES];for(index,byte)in scratch[..count].iter_mut().enumerate(){*byte=((offset+index)*37+19)as u8}input.admit_page(ToolWirePage::try_copy_from(&scratch[..count]).unwrap()).unwrap();Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;Ok(())});assert_eq!(input.pages.as_ptr(),pointer);}
   if cut.is_none(){event(&mut caller,&mut decoder,TURN,|cx|{let d=RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()};let mut done=None;admit_original_construction(cx,&mut done,original(d),|_|{input.seal().unwrap();Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;Ok(())});assert_eq!(input.page_count(),pages);for page in 0..pages{for(index,byte)in input.page(page).unwrap().iter().enumerate(){assert_eq!(*byte,((page*TOOL_WIRE_PAGE_BYTES+index)*37+19)as u8);}}}else{caller.cancel().cancel_now()}
   let d=close_demand(input);let state=(input.closing,input.sealed,input.admitted_bytes,input.pages.as_ptr(),input.pages.len(),input.pages.capacity());
   for axis in 0..5{let Some(g)=denied(d,axis)else{continue};event(&mut caller,&mut decoder,g,|cx|cx.with_original_retirement(|cx|{let p=input.close_step(g)?.progress();cx.consume_retained(p)?;cx.consume_fuel(p.copied_items as u64);Ok(())}));assert_eq!(caller.receipt(),Default::default());assert_eq!((input.closing,input.sealed,input.admitted_bytes,input.pages.as_ptr(),input.pages.len(),input.pages.capacity()),state);assert_eq!(close_demand(input),d);}
   event(&mut caller,&mut decoder,TURN,|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|grant|{let step=input.close_step(grant)?;Ok(RetainedCloneStep::Complete(step.progress()))})?;Ok(())}));assert!(input.terminal_is_empty());
   while !decoder.terminal_is_empty(){caller.turn(TURN,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(32768)?;close_original_retained_step(cx,d,|g|decoder.close_step(g))?;Ok(())})).unwrap();}caller.retire(TURN);
  }
 }
}

#[cfg(test)]
#[test]
fn original_command_wire_law(){original_command_wire_backing_is_admitted_before_pages_and_denied_close_preserves_the_same_owner()}
