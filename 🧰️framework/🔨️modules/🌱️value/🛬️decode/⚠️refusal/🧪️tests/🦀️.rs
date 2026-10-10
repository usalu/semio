//! 🧱️ Actual native refusals preserve original zero authority without allocating prose.
use crate::{NativeDecodeControl,NativeEncodeControl,ValueError,retained_clone::RetainedCloneProgress};

#[test]
fn original_native_refusal_forwarded_cause_moves_the_exact_original_backing(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["forwarded"];
 macro_rules! direction{($control:ident,$allocation:ty)=>{
  let message=law["message"].as_str().unwrap().to_owned();let pointer=message.as_ptr();let mut original=Some(ValueError::new(crate::ValueRefusalKind::InvalidValue,message));let mut callback=|_|true;
  let mut allocation=|request:$allocation|{assert_eq!(request.bytes,1);assert_eq!(request.owned_bytes,0);assert_eq!(request.next_owned_bytes,1);assert_eq!(request.maximum_bytes,1);Err(original.take().unwrap())};
  let mut control=$control::new_forwarded(law["maximumBytes"].as_u64().unwrap()as usize,&mut callback,&mut allocation);let(result,physical)=crate::value::observe_retirement_allocations(||control.charge(law["chargeBytes"].as_u64().unwrap()as usize));
  assert_eq!(physical,(0,0));assert_eq!(control.owned_bytes(),0);let error=result.unwrap_err();assert_eq!(error.kind.as_str(),law["kind"].as_str().unwrap());assert_eq!(error.message,law["message"].as_str().unwrap());assert_eq!(error.message.as_ptr(),pointer);assert!(matches!(error.message,std::borrow::Cow::Owned(_)));assert_eq!(error.retained_progress(),RetainedCloneProgress::default());drop(control);assert!(original.is_none());
 };}
 direction!(NativeDecodeControl,crate::native_decoding::NativeDecodeAllocation);
 direction!(NativeEncodeControl,crate::native_encoding::NativeEncodeAllocation);
 eprintln!("[DEBUG] Both original forwarded native refusals move exact existing owned cause backing without allocating, copying, releasing or renewing the original cumulative receipt");
}

#[test]
fn original_native_refusal_literal_controls_preserve_zero_physical_authority(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for case in fixture["cases"].as_array().unwrap(){
  let operation=case["operation"].as_str().unwrap();let mut accepted=|_|operation!="checkpoint";let maximum=fixture["maximumBytes"].as_u64().unwrap()as usize;
  let((result,owned),physical)=crate::value::observe_retirement_allocations(||{
   if case["direction"]=="decoding"{let mut control=NativeDecodeControl::new(maximum,&mut accepted);let result=match operation{
    "charge"=>control.charge(1),"vectorOverflow"=>control.allocate_vec::<u16>(usize::MAX).map(|_|()),"text"=>control.copy_text("x").map(|_|()),"depth"=>control.scoped_depth(0,|_|Ok::<(),ValueError>(())),
    "noRecipient"=>control.with_retirement_owner::<(),ValueError>(0,|_|unreachable!()),"invalidUtf8"=>control.borrow_text(&[255]).map(|_|()),"checkpoint"=>control.checkpoint(),_=>unreachable!()
   };(result,control.owned_bytes())}else{let mut accepted=|_|operation!="checkpoint";let mut control=NativeEncodeControl::new(maximum,&mut accepted);let result=match operation{
    "charge"=>control.charge(1),"vectorOverflow"=>control.allocate_vec::<u16>(usize::MAX).map(|_|()),"text"=>control.copy_text("x").map(|_|()),"depth"=>control.scoped_depth(0,|_|Ok::<(),ValueError>(())),
    "noRecipient"=>control.with_retirement_owner::<(),ValueError>(0,|_|unreachable!()),"appendBytes"=>control.append_bytes(&mut Vec::new(),&[1]),"checkpoint"=>control.checkpoint(),_=>unreachable!()
   };(result,control.owned_bytes())}
  });
  assert_eq!(physical,(0,0));assert_eq!(owned,0);let error=result.unwrap_err();assert!(matches!(error.message,std::borrow::Cow::Borrowed(_)));assert_eq!(error.kind.as_str(),case["kind"].as_str().unwrap());assert_eq!(error.message,case["message"].as_str().unwrap());assert_eq!(error.retained_progress(),RetainedCloneProgress::default());
 }
 eprintln!("[DEBUG] Actual14 original native refusal sites retain zero capacity and borrowed literal causes, exact allocator0/release0 and unchanged original zero receipts");
}
