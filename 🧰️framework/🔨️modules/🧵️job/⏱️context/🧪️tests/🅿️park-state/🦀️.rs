//! 🅿️ Parking hands out only a snapshot: it leaves no fuel and exactly the posted grant, and restoring or returning never changes either.
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation,OriginalTurnFailure};
use semio_framework_value::RetainedCloneGrant;
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:128}}
fn quantum()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:128}}
#[test]
fn restoring_and_returning_parked_state_change_neither_fuel_nor_grant(){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(7611),Generation(3),stock(),||Some(0),u64::MAX,1000000);
 caller.turn(quantum(),|cx|{
  let fuel=cx.fuel_remaining();let grant=cx.retained_grant();let posted=cx.retained_progress();let state=cx.park_state();
  cx.park_restore(state);assert_eq!((cx.fuel_remaining(),cx.retained_grant()),(fuel,grant));
  cx.park_return(9,Some(state.clock()));assert_eq!((cx.fuel_remaining(),cx.retained_grant(),cx.retained_progress()),(fuel,grant,posted));
  Ok::<_,OriginalTurnFailure>(())
 }).unwrap();
 caller.retire(quantum());
}
