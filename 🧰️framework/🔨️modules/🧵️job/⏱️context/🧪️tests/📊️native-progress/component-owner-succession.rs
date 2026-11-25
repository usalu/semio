//! 🛰️ A component host never hands published headers back, so its Source acknowledges settled scope rows itself and successive owners never exhaust the fixed escrow table.
use super::{caller,checkpoint,quantum,stock};
use semio_framework_job::OriginalJobFixtureCaller;

/// 🔌️ Switches the Source to Core-wire delivery with local acknowledgement, as the plugin actor does before any claim.
fn component(caller:&mut OriginalJobFixtureCaller){
 caller.bind_source_epoch(1);let mut ready=false;
 for _ in 0..64{caller.turn(stock(),|cx|{cx.acknowledge_return_rows_locally();ready=cx.admit_original_core_wire_delivery()?&&cx.admit_original_escrow_publisher()?;Ok(())}).unwrap();if ready{break}}
 assert!(ready,"Core-wire delivery admits within its funded turns");
}

#[test]
fn successive_component_owners_reclaim_their_settled_scope_rows(){
 let mut caller=caller();component(&mut caller);
 for owner in 0..4*semio_framework_job_payload::ORIGINAL_RETURN_ESCROW_CELLS{
  let mut decoder=caller.admitted_decoder();checkpoint(&mut caller,&mut decoder);
  for _ in 0..4096{if decoder.terminal_is_empty(){break}caller.bare_turn(quantum(),|cx|cx.with_original_retirement(|cx|{decoder.close_on_source(cx)?;Ok(())})).unwrap_or_else(|error|panic!("owner {owner}: {error:?}"));}
  assert!(decoder.terminal_is_empty(),"owner {owner} never retired");
 }
 caller.retire(quantum());
}
