//! 🧪️ Testkit borrows actual ready host output and returns every original owner under the same finite caller.
use super::*;
pub use semio_framework_trace::HeapWitness;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneStep};
use semio_plugin_job_receiving::original_steps::{receipt,retired};
fn clock()->Option<u64>{Some(1)}
/// 🎫️ Quotes one closing turn: the turn grant widened to the origin refresh plus the owner's own next retirement demand.
pub fn closing(grant:RetainedCloneGrant,demand:RetirementDemand)->RetainedCloneGrant{
    let business=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
    let quoted=semio_framework_job::original_return_refresh_turn_quote().and_then(|quote|quote.beneath(business,1)).expect("bounded closing quote").0;
    RetainedCloneGrant{maximum_items:grant.maximum_items.max(quoted.maximum_items),maximum_copy_bytes:grant.maximum_copy_bytes.max(quoted.maximum_copy_bytes),maximum_capacity_bytes:grant.maximum_capacity_bytes.max(quoted.maximum_capacity_bytes),maximum_release_bytes:grant.maximum_release_bytes.max(quoted.maximum_release_bytes),maximum_depth:grant.maximum_depth.max(quoted.maximum_depth)}
}
/// 🎫️ Quotes one turn's grant from the owner's own demands: its fixed state, one input slice, one parsed node per declared item plus the text bytes, settled through the construction turn quote.
pub fn grant<M:RetireOwned+Send+'static>(declaration:&Declaration<M>,input_bytes:usize)->RetainedCloneGrant{
    let bound=input_bytes.max(2);let(depth,nodes)=match declaration.projection{Projection::Contributions(policy)=>(policy.maximum_depth,policy.maximum_items),Projection::RawText=>(1,0)};
    let owned=size_of::<OriginalHostConfiguration<M>>().max(bound.saturating_add(nodes.saturating_mul(size_of::<DslValue>())));
    let demand=semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:size_of::<(OriginalHostConfiguration<M>,Option<DslValue>,Option<DslValue>)>()+64+2*size_of::<usize>()+bound,capacity_bytes:owned,release_bytes:owned,depth};
    semio_framework_job::original_construction_settled_turn_quote::<Box<OriginalHostConfiguration<M>>>(demand,1).expect("bounded construction quote").0
}
fn turn<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(grant,decoder,run));
    let progress=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes),"host test binary requires its actual HeapWitness allocator");
    result.unwrap()
}
/// ⚖️ A real original host producer lends its ready mutation before paid source, decoder and caller closure.
pub fn with_original_host_configuration<M:RetireOwned+Send+'static>(declaration:&'static Declaration<M>,mut args:Option<DslValue>,check:impl FnOnce(&M)){
    let g=grant(declaration,args.as_ref().and_then(|args|args.get("json")).and_then(DslValue::as_str).map_or(2,str::len));let mut caller=OriginalJobFixtureCaller::admit(OperationId(941),Generation(43),RetainedCloneGrant{maximum_items:1<<24,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096},clock,u64::MAX,1<<24);
    let mut decoder=caller.admitted_decoder();let mut source=None;let mut admission=Default::default();
    for _ in 0..1000000{if turn(&mut caller,&mut decoder,g,|cx|OriginalHostConfiguration::admit(declaration,&mut args,cx,&mut admission,&mut source)){break}}assert!(source.is_some());assert!(args.is_none());
    for index in 0..1000000{if source.as_ref().unwrap().ready(){break}turn(&mut caller,&mut decoder,g,|cx|source.as_mut().unwrap().advance(cx));assert!(index<999999,"original host producer remained pending");}
    check(source.as_ref().unwrap().owners().unwrap().mutation.as_ref().unwrap());
    for index in 0..1000000{
        if source.is_none(){break}
        let owner=source.as_ref().unwrap();let step_grant=closing(g,if owner.terminal_is_empty(){OriginalHostConfiguration::<M>::release_demands()}else{owner.close_demands().expect("bounded close quote")});
        turn(&mut caller,&mut decoder,step_grant,|cx|cx.with_original_retirement(|cx|{
            if source.as_ref().unwrap().terminal_is_empty(){let d=OriginalHostConfiguration::<M>::release_demands();retired(cx,d,|_|{drop(source.take());Ok(RetainedCloneStep::Complete(receipt(d)))})?;}else{source.as_mut().unwrap().close_step(cx)?;}Ok(())
        }));assert!(index<999999,"original host source retirement remained pending");
    }
    while !decoder.terminal_is_empty(){let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(g,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(g.maximum_copy_bytes)?;decoder.close_on_source(cx)?;Ok(())})));let progress=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));result.unwrap();}
    caller.retire(g);
}
