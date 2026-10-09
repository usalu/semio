//! 🔐️ Detached local receipts require the genuine original recipient in both native directions.
use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::controlled::ControlledRetirement};
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn grant(value:&serde_json::Value)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:value["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:value["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:value["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:value["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:value["maximumDepth"].as_u64().unwrap()as usize}}
macro_rules! original_local_law{
 ($name:ident,$control:path,$recipient:path,$hop:ident)=>{
  #[test]fn $name(){
   use $control as Control;use $recipient as Recipient;
   let law=fixture();let mut accepted=|_|true;let mut native=Control::new(law["maximumNativeBytes"].as_u64().unwrap()as usize,&mut accepted);
   let recipient_bytes=std::mem::size_of::<Recipient>();native.charge(recipient_bytes).unwrap();let mut recipient=Box::new(Recipient::new());let pointer=recipient.as_ref()as *const Recipient;
   native.install_retirement_recipient(&mut recipient).unwrap();native.charge(law["initialBytes"].as_u64().unwrap()as usize).unwrap();
   let wrapper=std::mem::size_of::<ControlledRetirement<String>>();
   let error=native.with_retirement_owner::<(),ValueError>(wrapper,|native|{let text=native.copy_text(law["text"].as_str().unwrap()).unwrap();assert_eq!(text.len(),law["textBytes"].as_u64().unwrap()as usize);let owner=Box::new(ControlledRetirement::new(text).unwrap());(Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original local output retained")),Some(owner))}).unwrap_err();
   assert_eq!(error.kind,ValueRefusalKind::InvalidValue);let before=native.owned_bytes();assert_eq!(before,recipient_bytes+law["initialBytes"].as_u64().unwrap()as usize+wrapper+law["textBytes"].as_u64().unwrap()as usize);
   let(detached,_)=native.pause_with_retirement().unwrap().detach();assert!(recipient.has_owner());assert_eq!(detached.owned_bytes(),before);assert_eq!(recipient.as_ref()as *const Recipient,pointer);
   let mut replacement=Recipient::new();let(error,detached)=match detached.bind(&mut replacement){Err(refused)=>refused,Ok(_)=>panic!("replacement recipient acquired original receipt")};assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);assert!(!replacement.has_owner());assert!(recipient.has_owner());assert_eq!(detached.owned_bytes(),before);
   let mut receipt=detached.bind(&mut recipient).map_err(|(error,_)|error).unwrap();let mut accepted=|_|true;let denied=grant(&law["deniedGrant"]);let step=receipt.$hop(&mut accepted,|native|native.close_retirement_recipient(denied)).unwrap();assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(receipt.owned_bytes(),before);
   let closing=grant(&law["closeGrant"]);let mut released=0;let mut born=0;let mut terminal=false;
   for _ in 0..law["maximumTurns"].as_u64().unwrap(){let done=receipt.$hop(&mut accepted,|native|{let step=native.close_retirement_recipient(closing)?;assert!(step.progress().fits(closing));released+=step.progress().released_bytes;born+=step.progress().retained_capacity_bytes;Ok(!native.has_retirement_owner())}).unwrap();if done{terminal=true;break}}
   assert!(terminal);let(detached,_)=receipt.detach();assert_eq!(detached.owned_bytes(),before+born);assert!(!recipient.has_owner());assert_eq!(recipient.as_ref()as *const Recipient,pointer);assert_eq!(released,wrapper+law["textBytes"].as_u64().unwrap()as usize+born);
   let oracle=serde_json::to_string(law["text"].as_str().unwrap()).unwrap();assert_eq!(serde_json::from_str::<String>(&oracle).unwrap().len(),law["textBytes"].as_u64().unwrap()as usize);
   assert!(closing.maximum_release_bytes>=recipient_bytes);drop(recipient);
   eprintln!("[DEBUG] {} preserves original occupied recipient identity, cumulative allocation, denied-close custody and exact real returned backing across detached local hops",stringify!($name));
  }
 }
}
original_local_law!(native_local_original_decode_recipient_survives_detachment,crate::native_decoding::NativeDecodeControl,crate::native_decoding::NativeDecodeRetirementRecipient,decode);
original_local_law!(native_local_original_encode_recipient_survives_detachment,crate::native_encoding::NativeEncodeControl,crate::native_encoding::NativeEncodeRetirementRecipient,encode);
