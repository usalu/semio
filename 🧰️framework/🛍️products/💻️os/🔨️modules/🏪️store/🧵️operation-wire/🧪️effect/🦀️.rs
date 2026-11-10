//! 🧪️ Operation-wire fixtures run each original turn inside one actual original Work effect.
use semio_framework_value::{RetainedCloneGrant,RetainedWorkOutcome};
pub use semio_framework_value::testing::{fx,settled,with_effect};
/// 🪪️ Binds one owning alias of a whole retained source inside one original effect.
pub fn alias<T:semio_framework_value::retirement::RetireOwned+Sync+'static>(source:&semio_framework_value::retained_clone::RetainedCloneSource<T>)->(semio_framework_value::retained_clone::RetainedOwnedProjection<T>,semio_framework_value::RetainedCloneProgress){
 let demand=semio_framework_pack_json::retained_alias_demand(source.borrow()).unwrap();
 fx(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1)},|effect|semio_framework_pack_json::retained_alias(source.borrow(),effect)).unwrap().expect("funded alias")
}
type Construct<'a,O>=&'a mut dyn FnMut(&mut semio_framework_value::native_decoding::construction::NativeConstructionScope<'_,'_>,&mut Option<usize>)->RetainedWorkOutcome<(O,semio_framework_value::RetainedCloneProgress)>;
/// 🏗️ Drives one original construction through every admission turn, then retires its control metadata, and sums the physical receipts.
pub fn construct_ready<O>(caller:&mut semio_framework_job::OriginalJobFixtureCaller,decoder:&mut semio_framework_job::OriginalJobDecodeOwner,grant:RetainedCloneGrant,demand:semio_framework_job::OriginalConstructionDemand,output:&mut Option<O>,construct:Construct<'_,O>)->(semio_framework_job::OriginalConstructionStatus,usize,usize){
 use semio_framework_job::{OriginalConstructionAdmission,OriginalConstructionStatus,admit_original_construction};
 let mut admission=OriginalConstructionAdmission::default();let(mut requested,mut released)=(0,0);let mut status=OriginalConstructionStatus::Pending;
 for _ in 0..64{status=caller.turn_with_decoder(grant,decoder,|cx|admit_original_construction(cx,output,&mut admission,demand,1,&mut *construct)).unwrap();let receipt=caller.receipt();requested+=receipt.retained_capacity_bytes;released+=receipt.released_bytes;if status!=OriginalConstructionStatus::Pending{break}}
 for _ in 0..64{let done=caller.turn_with_decoder(grant,decoder,|cx|admission.close_step(cx)).unwrap();let receipt=caller.receipt();requested+=receipt.retained_capacity_bytes;released+=receipt.released_bytes;if done{break}}
 assert!(admission.terminal_is_empty());(status,requested,released)
}
