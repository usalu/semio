//! 🩹️ LAW: a selected original birth (failure spare or child) runs on its own turn before the general close effect, so closing a ticket never refuses "selected original birth must run before the general retirement effect".
use super::{retire_controlled,retire_ticket_controlled};
use semio_framework_job::{Generation,OperationId,OriginalJobFixtureCaller,RetainedCloneGrant};
use semio_framework_value::{close_factory_ticket_controlled,retirement::controlled::ControlledRetirement};
use semio_framework_value_testing_allocator as _;

type Ticket=ControlledRetirement<String>;

fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:4096}}
fn turn_grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:1<<16,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64}}
fn ticket()->Option<Box<Ticket>>{Some(Box::new(ControlledRetirement::new("λ".repeat(2048)).expect("ticket payload")))}

/// 🧪️ Drives `ticket` through `retire_ticket_controlled` and returns the number of turns that advanced it; the first turn only runs the birth.
#[test]
fn a_ticket_runs_its_selected_birth_on_its_own_turn_then_closes_without_refusal(){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(8801),Generation(2),stock(),||Some(1),u64::MAX,1<<20);
 let mut decoder=caller.admitted_decoder();caller.establish_decoder_receipt(&mut decoder,turn_grant());let mut ticket=ticket();
 let none=RetainedCloneGrant{maximum_items:0,..turn_grant()};
 assert!(!caller.turn_with_decoder(none,&mut decoder,|cx|retire_ticket_controlled(cx,&mut ticket)).expect("an unfunded turn is not a refusal"),"an unfunded turn advances nothing");
 assert!(ticket.is_some());
 assert!(caller.turn_with_decoder(turn_grant(),&mut decoder,|cx|retire_ticket_controlled(cx,&mut ticket)).expect("the birth turn"),"the first funded turn runs the selected birth");
 assert!(ticket.as_ref().is_some_and(|t|!semio_framework_value::ErasedSnapshotRetirement::terminal_is_empty(&**t)),"the birth turn leaves the ticket open");
 let mut turns=1;
 while ticket.is_some(){assert!(turns<4096,"the ticket closes within bounded turns");caller.turn_with_decoder(turn_grant(),&mut decoder,|cx|retire_ticket_controlled(cx,&mut ticket)).expect("a later turn closes without the skipped-birth refusal");turns+=1;}
 assert!(turns>1);
 caller.close_decoder(&mut decoder,turn_grant());caller.retire(stock());
}

/// 🧪️ The effect-shaped close runs only after the birth turn; a refused-birth ordering would invoke it first and fail.
#[test]
fn an_effect_shaped_close_waits_for_its_birth_turn_and_then_runs_once_per_funded_turn(){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(8802),Generation(2),stock(),||Some(1),u64::MAX,1<<20);
 let mut decoder=caller.admitted_decoder();caller.establish_decoder_receipt(&mut decoder,turn_grant());let mut ticket=ticket();let mut runs=0usize;
 assert!(caller.turn_with_decoder(turn_grant(),&mut decoder,|cx|retire_controlled(cx,|effect,control|{runs+=1;close_factory_ticket_controlled(&mut ticket,effect.grant().maximum_copy_bytes,effect,control)})).expect("the birth turn"));
 assert_eq!(runs,0,"the close effect is not invoked on the birth turn");
 for turns in 0..4096{
  if ticket.is_none(){break}assert!(turns<4095,"the ticket closes within bounded turns");
  caller.turn_with_decoder(turn_grant(),&mut decoder,|cx|retire_controlled(cx,|effect,control|{runs+=1;close_factory_ticket_controlled(&mut ticket,effect.grant().maximum_copy_bytes,effect,control)})).expect("a later turn closes without the skipped-birth refusal");
 }
 assert!(runs>=1&&ticket.is_none());
 caller.close_decoder(&mut decoder,turn_grant());caller.retire(stock());
}
