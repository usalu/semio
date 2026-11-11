//! 🧪️ Provenance text copies and every abandoned partial cursor have exact physical receipts.
use super::*;
use crate::original_steps::{admission_grant,finished,step_grant};
use semio_os_kernel::MutationOrigin;
use semio_os_kernel as store;
use crate::original_steps::{retired,grant_of,birth_grant};
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,default_now_us};
use semio_framework_value::RetainedCloneGrant;
thread_local!{static IDLE:std::cell::Cell<u32>=const{std::cell::Cell::new(0)}}
fn idle(p:semio_framework_value::RetainedCloneProgress){IDLE.with(|n|{if p==Default::default(){n.set(n.get()+1);assert!(n.get()<=16,"original turns neither progress nor charge: stalled before {} turns",n.get())}else{n.set(0)}})}
fn finish(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){for index in 0..4096{if decoder.terminal_is_empty(){return}let grant=grant_of(decoder.retirement_demands(4096).unwrap());caller.turn(grant,|cx|cx.with_original_retirement(|cx|{decoder.close_on_source(cx)?;Ok(())})).unwrap();assert!(index<4095,"original decoder close stalled");}}
fn bind(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){let stock=semio_framework_value::RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:128};for _ in 0..16{if caller.turn(stock,|cx|Ok(decoder.admit_step(cx)?)).unwrap(){break}}assert!(decoder.source_loan_admitted(),"original decoder source loan is admitted before any birth");}
fn turn<R>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,g:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->R{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(g,decoder,run));let p=caller.receipt();idle(p);assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes),"actual original provenance receipt {p:?}");result.unwrap()}
fn born_within<O>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,d:RetirementDemand,mut admit:impl FnMut(&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>){let grant=birth_grant::<O>(d).unwrap();for index in 0..64{if turn(caller,decoder,grant,&mut admit){return}assert!(index<63,"original birth did not settle on its exact quote")}}
#[test]fn original_provenance_copies_preserve_all_variants_and_close_every_cancelled_cursor(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let count=law["count"].as_u64().unwrap()as usize;
 for kind in 0..3{for cut in law["cancelPrefixes"].as_array().unwrap().iter().map(|n|Some(n.as_u64().unwrap()as usize)).chain([None]){
  let original=match kind{0=>MutationOrigin::Owner,1=>MutationOrigin::Contributed{plugin_id:law["pluginId"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap()as usize),mutation_id:store::SchemaId(law["mutationId"].as_str().unwrap().into()),payload_hash:store::PayloadHash([7;32])},_=>MutationOrigin::Transaction{initiator:store::ForeignTarget{artifact_id:law["initiator"]["artifactId"].as_str().unwrap().into(),artifact_kind:law["initiator"]["artifactKind"].as_str().unwrap().into(),dialect:Some(law["initiator"]["dialect"].as_str().unwrap().into())}}};let oracle=original.clone();let mut original=Some(original);
  let g=admission_grant(size_of::<OriginalValueCopies<MutationOrigin>>());let body=g.maximum_copy_bytes;let mut caller=OriginalJobFixtureCaller::admit(OperationId(382),Generation(8),crate::original_steps::STOCK,default_now_us,u64::MAX,1<<30);let mut decoder=caller.admitted_decoder();caller.retain_dispatch().unwrap();let mut owner=None;
  let mut admission=Default::default();assert!((0..64).any(|_|turn(&mut caller,&mut decoder,g,|cx|OriginalValueCopies::<MutationOrigin>::admit(&mut original,count,cx,&mut admission,&mut owner))));assert!(original.is_none());
  for index in 0..262144{if cut==Some(index){break}let q=owner.as_ref().unwrap().advance_grant(caller.fuel_remaining()).unwrap();let ready=turn(&mut caller,&mut decoder,q,|cx|owner.as_mut().unwrap().advance(cx));if ready{break}assert!(index<262143,"original provenance did not settle");}
  let mut output=None;if cut.is_none(){assert!((0..64).any(|_|turn(&mut caller,&mut decoder,OriginalValueCopies::<MutationOrigin>::take_grant().unwrap(),|cx|owner.as_mut().unwrap().take(cx,&mut output))));let rows=output.as_ref().unwrap();assert_eq!(rows.len(),count);for row in rows{assert_eq!(row,&oracle);}if let MutationOrigin::Contributed{plugin_id:source,..}=&oracle{for row in rows{let MutationOrigin::Contributed{plugin_id,..}=row else{unreachable!()};assert_ne!(source.as_ptr(),plugin_id.as_ptr());assert_eq!(serde_json::from_str::<String>(&serde_json::to_string(plugin_id).unwrap()).unwrap(),law["pluginId"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap()as usize));}}}else{caller.cancel().cancel_now();}
  for index in 0..262144{if owner.as_ref().unwrap().terminal_is_empty(){break}let q=owner.as_ref().unwrap().close_grant().unwrap();turn(&mut caller,&mut decoder,q,|cx|cx.with_original_retirement(|cx|owner.as_mut().unwrap().close_step(cx)));assert!(index<262143,"original provenance cancellation did not settle");}
  turn(&mut caller,&mut decoder,step_grant(OriginalValueCopies::<MutationOrigin>::release_demands()),|cx|cx.with_original_retirement(|cx|{let d=OriginalValueCopies::<MutationOrigin>::release_demands();retired(cx,d,|_|{owner.take();Ok(RetainedCloneStep::Complete(receipt(d)))})?;Ok(())}));
  if output.is_some(){let mut retirement=RetainedCloneClose::default();for index in 0..262144{if output.is_none()&&retirement.is_empty(){break}let d=retirement.next_owner_demands::<VecDeque<MutationOrigin>>(output.is_some(),&None,body).unwrap();turn(&mut caller,&mut decoder,step_grant(d),|cx|cx.with_original_retirement(|cx|{if retirement.is_empty(){begun(cx,d,|effect|retirement.begin_granted(&mut output,effect))?;}else{retired_poll(cx,d,|effect|retirement.step_granted(body,effect))?;}Ok(())}));assert!(index<262143);}}
  finished(caller,&mut decoder);
 }}
}
#[test]
fn underfunded_admission_stays_pending_and_a_funded_turn_resumes_it(){
 let full=admission_grant(size_of::<OriginalValueCopies<MutationOrigin>>());let mut starved=full;starved.maximum_items-=1;
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(386),Generation(12),crate::original_steps::STOCK,default_now_us,u64::MAX,1<<30);let mut decoder=caller.admitted_decoder();caller.retain_dispatch().unwrap();
 let mut original=Some(MutationOrigin::Owner);let mut owner=None;let mut admission=Default::default();
 assert!(!turn(&mut caller,&mut decoder,starved,|cx|OriginalValueCopies::<MutationOrigin>::admit(&mut original,3,cx,&mut admission,&mut owner)));assert!(owner.is_none()&&original.is_some());
 for n in 0..8{let r=turn(&mut caller,&mut decoder,full,|cx|OriginalValueCopies::<MutationOrigin>::admit(&mut original,3,cx,&mut admission,&mut owner));if r{break}}assert!(owner.is_some());assert!(original.is_none()&&owner.is_some()&&admission.terminal_is_empty());
 for index in 0..262144{if owner.as_ref().unwrap().terminal_is_empty(){break}let q=owner.as_ref().unwrap().close_grant().unwrap();turn(&mut caller,&mut decoder,q,|cx|cx.with_original_retirement(|cx|owner.as_mut().unwrap().close_step(cx)));assert!(index<262143,"original provenance cancellation did not settle");}
 turn(&mut caller,&mut decoder,step_grant(OriginalValueCopies::<MutationOrigin>::release_demands()),|cx|cx.with_original_retirement(|cx|{let d=OriginalValueCopies::<MutationOrigin>::release_demands();retired(cx,d,|_|{owner.take();Ok(RetainedCloneStep::Complete(receipt(d)))})?;Ok(())}));
 finished(caller,&mut decoder);
}
