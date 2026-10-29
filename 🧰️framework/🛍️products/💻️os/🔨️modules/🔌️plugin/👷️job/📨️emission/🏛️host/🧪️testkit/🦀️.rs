//! 🧪️ Testkit borrows actual ready host output and returns every original owner under the same finite caller.
use super::*;
pub use semio_framework_trace::HeapWitness;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation};
use semio_framework_value::RetainedCloneGrant;
fn clock()->Option<u64>{Some(1)}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:16<<20,maximum_release_bytes:16<<20,maximum_depth:4096}}
fn turn<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(grant(),decoder,run));
    let progress=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes),"host test binary requires its actual HeapWitness allocator");
    result.unwrap()
}
/// ⚖️ A real original host producer lends its ready mutation before paid source, decoder and caller closure.
pub fn with_original_host_configuration<M:RetireOwned+Send+'static>(declaration:&'static Declaration<M>,mut args:Option<DslValue>,check:impl FnOnce(&M)){
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(941),Generation(43),RetainedCloneGrant{maximum_items:1<<24,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096},clock,u64::MAX,1<<24);
    let mut decoder=OriginalJobDecodeOwner::empty();let mut source=None;
    turn(&mut caller,&mut decoder,|cx|{assert!(OriginalHostConfiguration::admit(declaration,&mut args,cx,&mut source)?);Ok(())});assert!(args.is_none());
    for index in 0..1000000{if source.as_ref().unwrap().ready(){break}turn(&mut caller,&mut decoder,|cx|source.as_mut().unwrap().advance(cx));assert!(index<999999,"original host producer remained pending");}
    check(source.as_ref().unwrap().owners().unwrap().mutation.as_ref().unwrap());
    for index in 0..1000000{
        if source.is_none(){break}
        turn(&mut caller,&mut decoder,|cx|cx.with_original_retirement(|cx|{
            if source.as_ref().unwrap().terminal_is_empty(){let d=OriginalHostConfiguration::<M>::release_demands();close_original_retained_step(cx,d,|_|{drop(source.take());Ok(RetainedCloneStep::Complete(receipt(d)))})?;}else{source.as_mut().unwrap().close_step(cx)?;}Ok(())
        }));assert!(index<999999,"original host source retirement remained pending");
    }
    while !decoder.terminal_is_empty(){let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant(),|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(32768)?;close_original_retained_step(cx,d,|grant|decoder.close_step(grant))?;Ok(())})));let progress=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));result.unwrap();}
    caller.retire(grant());
}
