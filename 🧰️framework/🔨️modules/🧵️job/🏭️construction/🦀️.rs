//! 🏗️ The original Job decoder owns generic construction, candidate custody and complete physical receipts.
use super::{StepContext,OriginalTurnFailure,Generation,OperationId,OriginalPayloadSourceIdentity};
use semio_framework_value::{ValueError, ValueRefusalKind, RetirementDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
/// 📏️ One defining owner declares all five independent currencies before a native birth.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalConstructionDemand{pub items:usize,pub copy_bytes:usize,pub capacity_bytes:usize,pub release_bytes:usize,pub depth:usize}
impl OriginalConstructionDemand{
 pub fn fits(self,grant:RetainedCloneGrant)->bool{self.items==1&&self.depth>0&&self.items<=grant.maximum_items&&self.copy_bytes<=grant.maximum_copy_bytes&&self.capacity_bytes<=grant.maximum_capacity_bytes&&self.release_bytes<=grant.maximum_release_bytes&&self.depth<=grant.maximum_depth}
 fn physical(self)->RetirementDemand{RetirementDemand{copy_bytes:self.copy_bytes,capacity_bytes:self.capacity_bytes,release_bytes:self.release_bytes,depth:self.depth}}
}
/// 🧷️ A zero-effect witness borrows original source identity without issuing any construction or work authority.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalSourceBinding{operation:OperationId,generation:Generation,payload:OriginalPayloadSourceIdentity,clock:Option<u64>,clock_source:usize,deadline:u64}
impl OriginalSourceBinding{
 /// 👁️ Captures only identity already supplied by the original caller and its native source ledger.
 pub fn borrow(context:&StepContext<'_>)->Self{Self{operation:context.operation(),generation:context.generation(),payload:context.payload_authority_identity(),clock:context.original_clock_binding(),clock_source:context.clock_source()as usize,deadline:context.original_absolute_deadline_us()}}
 /// 🔒️ Checks the same roots and original clock without deriving or enlarging a turn grant.
 pub fn matches(self,context:&StepContext<'_>)->bool{self==Self::borrow(context)}
 pub fn operation(self)->OperationId{self.operation}
 pub fn generation(self)->Generation{self.generation}
}
/// 🎟️ Retains a born candidate before forwarding actual allocation and any complete refusal to its original recipient.
pub fn admit_original_construction<O>(context:&mut StepContext<'_>,output:&mut Option<O>,demand:OriginalConstructionDemand,construct:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<(O,RetainedCloneProgress),ValueError>)->Result<bool,OriginalTurnFailure>{

    if output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original native construction requires an empty candidate slot").into());}
    if context.receiving_issued_grant().is_some()||context.receiving_child_progress()!=RetainedCloneProgress::default(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original native construction cannot replace an outstanding construction scope").into());}
    if !context.retained_recipient_is_admitted(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original native construction recipient already exceeds its turn").into());}
    if context.is_cancelled()||context.should_yield()||!demand.fits(context.retained_grant()){return Ok(false);}
    use semio_framework_value_native_owned::{native_decoding::NativeConstructionStepFailure, native_decoding::NativeConstructionStepResult};
    use super::OriginalReceivingPrimary;
    let result=context.with_native_receiving::<_,NativeConstructionStepFailure<OriginalReceivingPrimary>>(|control|control.receive_construction_step_typed(demand.physical(),|control,grant|{
        if control.owned_bytes().checked_add(demand.capacity_bytes).is_none_or(|next|next>control.maximum_bytes()){
            return NativeConstructionStepResult { result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original native construction exceeds its decoder allocation scope").into()),progress:Default::default() };
        }
        let (progress,primary)=match construct(control){
            Ok((candidate,progress))=>{*output=Some(candidate);(progress,None)},
            Err(error)=>(error.retained_progress(),Some(error)),
        };
        let quote_refusal=(!progress.fits(grant)||progress.copied_items>demand.items||progress.copied_bytes>demand.copy_bytes||progress.retained_capacity_bytes>demand.capacity_bytes||progress.released_bytes>demand.release_bytes).then(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original native construction exceeded its physical quote").with_retained_progress(progress));
        let allocation_refusal=control.record_constructed_allocation(progress.retained_capacity_bytes).err().map(|error|error.with_retained_progress(progress));
        let result=if primary.is_some()||quote_refusal.is_some()||allocation_refusal.is_some(){Err(OriginalReceivingPrimary { primary,quote_refusal,allocation_refusal,progress })}else{Ok(RetainedCloneStep::Complete(progress))};
        NativeConstructionStepResult { result,progress }
    }).into_step_result());
    let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
    let reconciled=match context.receiving_issued_grant(){Some(issued)=>context.consume_receiving_retained(issued,progress),None=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original native construction omitted its issued receiving scope"))};
    context.consume_fuel(progress.copied_items as u64);
    match (result,reconciled){
        (Ok(step),Ok(()))=>Ok(matches!(step,RetainedCloneStep::Complete(_))&&output.is_some()),
        (Err(error),Ok(()))=>Err(error.into()),
        (Ok(_),Err(error))=>Err(error.into()),
        (Err(error),Err(receipt))=>{let mut failure=OriginalTurnFailure::from(error);failure.context_refusal=Some(receipt);Err(failure)}
    }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
