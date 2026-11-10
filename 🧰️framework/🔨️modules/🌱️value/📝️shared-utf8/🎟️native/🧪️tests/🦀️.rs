//! 🧪️ A real shared actor leaves its input slot only through one funded native receipt and is then paid closed.
use semio_framework_value_shared_utf8::*;
#[path="../../🧪️tests/⏱️effect/🦀️.rs"]
mod effect;
use effect::*;
#[test]fn native_shared_actor_moves_from_input_to_output_in_one_funded_receipt_and_is_paid_closed(){
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:8};
 let text="native shared actor".to_owned();let pointer=text.as_ptr();let mut input=Some(text);let mut output=None;let mut b=bank();
 let denied=RetainedCloneGrant{maximum_items:0,..grant};
 let turn=run(&mut b,denied,|effect|SharedUtf8::receive_original(&mut input,&mut output,effect));assert_eq!(turn.result.unwrap(),RetainedCloneProgress::default());assert!(input.is_some()&&output.is_none());
 let turn=run(&mut b,grant,|effect|SharedUtf8::receive_original(&mut input,&mut output,effect));let progress=turn.result.unwrap();assert!(progress.fits(grant));assert_eq!(turn.work,1);
 assert!(input.is_none());let output=output.take().unwrap();assert_eq!(output.as_ptr(),pointer);
 let mut owner=semio_framework_value_shared_utf8::retirement::controlled::ControlledRetirement::new(output).ok().unwrap();
 for _ in 0..100000{if owner.terminal_is_empty(){break}let turn=run(&mut b,grant,|effect|settled(owner.step(effect)));turn.result.unwrap();}
 assert!(owner.terminal_is_empty());
}
