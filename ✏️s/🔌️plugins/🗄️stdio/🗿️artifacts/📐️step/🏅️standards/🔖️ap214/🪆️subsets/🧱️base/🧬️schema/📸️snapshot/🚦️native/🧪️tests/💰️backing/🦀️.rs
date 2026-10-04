//! 💰️ Inline roots and single-slot typed boxes expose their actual backing contract.
use super::*;
#[test]
fn sqlite_snapshot_step_inline_root_and_typed_box_have_only_concrete_backing(){
 let vector:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🪶️sqlite/💰️backing/🔣️.json")).unwrap();
 let mut progress=|_|true;let mut control=NativeDecodeControl::new(0,&mut progress);
 let frame=Frame{schema:String::new(),description:Vec::new(),implementation_level:String::new(),file_name:String::new(),timestamp:String::new(),author:Vec::new(),organization:Vec::new(),preprocessor_version:String::new(),originating_system:String::new(),authorization:String::new(),file_schema:Vec::new(),entities:Vec::new(),values:Vec::new()};
 let result=reconstruct(frame,&mut control).expect("an inline root must not request heap backing");assert_eq!(control.owned_bytes(),0);assert_eq!(vector["typedConstruction"]["inlineRoot"],"noBackingRequest");close(result);
 let bytes=std::mem::size_of::<StepValue>();
 let mut progress=|_|true;let mut control=NativeDecodeControl::new(bytes,&mut progress);let mut slot=control.allocate_vec::<StepValue>(1).unwrap();slot.push(StepValue::Real(f64::from_bits(0x7ff8000000000042)));let address=slot.as_ptr();let value=box_slot(slot).unwrap();assert_eq!(std::ptr::from_ref(value.as_ref()),address);assert_eq!(control.owned_bytes(),bytes);assert_eq!(vector["typedConstruction"]["typedBox"],"onePaidSlot");assert!(matches!(*value,StepValue::Real(real)if real.to_bits()==0x7ff8000000000042));close(value);
 let mut progress=|_|true;let mut control=NativeDecodeControl::new(bytes-1,&mut progress);assert_eq!(control.allocate_vec::<StepValue>(1).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(control.owned_bytes(),0);
}

