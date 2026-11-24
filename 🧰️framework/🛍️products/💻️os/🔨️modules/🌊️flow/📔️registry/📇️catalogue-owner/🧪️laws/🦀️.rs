//! 🧪️ The catalogue owner captures its registry only through the turn's own Source, so a maintenance pump turn that carries one never refuses with a missing original Source.
use semio_framework_job::{Generation,OperationId,OriginalJobFixtureCaller,OriginalTurnFailure};
use semio_framework_value::RetainedCloneGrant;
use semio_os_flow_operators::registry::{FlowOperatorCatalogueOwner,flow_extension_registry,flow_extension_registry_generation};
fn now()->Option<u64>{Some(0)}
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:128}}
fn quantum()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:1<<16,maximum_capacity_bytes:1<<16,maximum_release_bytes:1<<16,maximum_depth:128}}
fn drive(caller:&mut OriginalJobFixtureCaller,mut run:impl FnMut(&mut semio_framework_job::StepContext<'_>)->Result<bool,OriginalTurnFailure>)->Result<(),String>{
 for _ in 0..100000{match caller.turn(quantum(),|cx|run(cx)){Ok(true)=>return Ok(()),Ok(false)=>{},Err(failure)=>return Err(failure.primary.message().to_string())}}
 Err("catalogue owner did not settle".into())
}
#[test]
fn catalogue_derivation_captures_the_registry_under_the_turns_own_source(){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(7711),Generation(1),stock(),now,u64::MAX,1_000_000);
 let mut owner=FlowOperatorCatalogueOwner::new(flow_extension_registry(),flow_extension_registry_generation());
 let derived=drive(&mut caller,|cx|owner.step(cx));
 owner.begin_close();
 let closed=drive(&mut caller,|cx|owner.close(cx).map(|_|owner.terminal_is_empty()));
 derived.expect("catalogue derivation ran under the turn's own original Source");closed.expect("catalogue owner closes");
 assert!(owner.terminal_is_empty());
 caller.retire_exact();
}
#[test]
fn a_panicking_body_with_a_non_empty_owner_unwinds_without_aborting(){
 let panicked=std::panic::catch_unwind(||{let _owner=FlowOperatorCatalogueOwner::new(flow_extension_registry(),flow_extension_registry_generation());panic!("[DEBUG] expected test-body failure")});
 assert!(panicked.is_err());
 let dropped=std::panic::catch_unwind(||drop(FlowOperatorCatalogueOwner::new(flow_extension_registry(),flow_extension_registry_generation())));
 assert!(dropped.is_err(),"outside unwinding a non-empty owner still violates its terminal law");
}
