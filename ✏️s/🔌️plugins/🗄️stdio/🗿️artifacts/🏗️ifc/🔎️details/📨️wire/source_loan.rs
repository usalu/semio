//! 🪢️ One original-funded projection leases the render host's actual immutable source Arc.
use super::{IfcSnapshot,wire::SnapshotSource};
use semio_framework_job::{OriginalSourceBinding,OriginalTurnFailure,StepContext,receive_original_retained_work};
use super::effected;
use semio_framework_value::{OriginalRetirementFailure,RetainedAllocationEffect,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,RetainedClone,RetainedCloneCursor,RetainedCloneRef,RetainedCloneProgress,StagedSourceAdmission,retained_clone_grant_admits},retirement::arc_custody::ArcLease};
use std::{mem::size_of,sync::Arc};

/// 🔗️ The host's shared snapshot leases its own payload as the original clone source.
pub type SnapshotLease=ArcLease<IfcSnapshot>;

/// 🪙️ Runs one step inside the turn's own original effect and settles its receipt and performed work once; `None` means the quote was not admitted and nothing ran.
pub fn funded_turn(cx:&mut StepContext<'_>,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,ValueError>)->Result<Option<RetainedCloneStep>,OriginalTurnFailure>{
    let grant=cx.retained_grant();
    let Some((turn,performed))=cx.with_original_effect(grant,|_,effect|run(effect))? else{return Ok(None)};
    let overrun=turn.work!=performed;
    let step=receive_original_retained_work(cx,turn)?;
    if overrun{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IFC admission work differs from its original effect").into());}
    Ok(Some(step))
}

#[derive(Clone,Copy)]
struct Binding{original:OriginalSourceBinding,pointer:usize,revision:[u8;32]}
semio_framework_value::artifact_retire_leaf!(Binding);
type Admission=StagedSourceAdmission<IfcSnapshot,SnapshotLease>;
#[derive(Default,semio_framework_value::RetireOwned)]
pub struct SourceLoan{admission:Option<Admission>,source:Option<SnapshotSource>,binding:Option<Binding>}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"staging.original-source-loan")}
impl SourceLoan{
    fn metadata_bytes()->usize{2*size_of::<Self>()+size_of::<SnapshotLease>()}
    fn ready(&self)->bool{self.admission.is_none()&&self.source.is_some()}
    pub fn demands(&self)->Result<RetirementDemand,ValueError>{
        if self.ready(){return Ok(Default::default());}
        match self.admission.as_ref(){
            Some(admission)=>{let mut demand=admission.next_demand()?;demand.copy_bytes=demand.copy_bytes.max(Self::metadata_bytes());Ok(demand)},
            None=>Ok(RetirementDemand{copy_bytes:Self::metadata_bytes(),depth:1,..Default::default()}),
        }
    }
    pub fn source(&self)->Option<&SnapshotSource>{if self.ready(){self.source.as_ref()}else{None}}
    pub fn advance(&mut self,owner:&Arc<IfcSnapshot>,expected:&IfcSnapshot,revision:[u8;32],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        let pointer=expected as *const _ as usize;
        if !std::ptr::eq(owner.as_ref(),expected)||self.binding.is_some_and(|binding|!binding.original.matches(cx)||binding.pointer!=pointer||binding.revision!=revision){return Err(invalid().into());}
        if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"staging source loan cancelled").into());}
        if self.ready(){return Ok(true);}
        let demand=self.demands()?;
        if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),demand){return Ok(false);}
        let progress=RetainedCloneProgress{copied_items:1,copied_bytes:Self::metadata_bytes(),..Default::default()};
        let Some(admission)=self.admission.as_mut()else{
            self.admission=Some(Admission::borrowed(SnapshotLease::new(Arc::clone(owner))));
            self.binding=Some(Binding{original:OriginalSourceBinding::borrow(cx),pointer,revision});
            cx.consume_retained(progress)?;cx.consume_fuel(1);
            return Ok(false);
        };
        if admission.terminal_is_empty(){
            if self.source.is_none(){return Err(invalid().into());}
            self.admission=None;
            cx.consume_retained(progress)?;cx.consume_fuel(1);
            return Ok(true);
        }
        let source=&mut self.source;
        funded_turn(cx,|effect|admission.advance(source,effect))?;
        Ok(false)
    }
}

/// 🪢️ Runs one reply-shaped callee and returns its recorded result; `None` means the callee suspended.
pub fn replied<E>(effect:&mut RetainedAllocationEffect<'_>,action:impl for<'r> FnOnce(&mut RetainedAllocationEffect<'_>,OriginalEffectReply<'r,RetainedCloneStep,E>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>)->Result<Option<Result<RetainedCloneStep,E>>,ValueError>{
    let mut slot=None;let reply=OriginalEffectReply::borrow(&mut slot)?;action(&mut *effect,reply)?;Ok(slot.take())
}
/// 🔁️ One clone-cursor turn over a borrowed source; a suspended reply is recovered on the next turn through `recover`.
pub fn advance_cursor<T:RetainedClone,C:RetainedCloneCursor<T>>(cx:&mut StepContext<'_>,cursor:&mut C,source:RetainedCloneRef<'_,T>,body:usize,recover:&mut bool)->Result<(),OriginalTurnFailure>{
    let again=*recover;
    let Some(reply)=effected(cx,|effect|{let maximum=effect.maximum_work_units();if again{replied(effect,|effect,reply|cursor.recover_advance_into(source,body,maximum,effect,reply))}else{replied(effect,|effect,reply|cursor.advance_into(source,body,maximum,effect,reply))}})? else{return Ok(())};
    settle_cursor(cx,reply?,recover)
}
/// ♻️ One clone-cursor close turn; a suspended reply is recovered on the next turn through `recover`.
pub fn close_cursor<T:RetainedClone,C:RetainedCloneCursor<T>>(cx:&mut StepContext<'_>,cursor:&mut C,body:usize,recover:&mut bool)->Result<(),OriginalTurnFailure>{
    let again=*recover;
    let Some(reply)=effected(cx,|effect|if again{replied(effect,|effect,reply|cursor.recover_close_into(body,effect,reply))}else{replied(effect,|effect,reply|cursor.close_into(body,effect,reply))})? else{return Ok(())};
    settle_cursor(cx,reply?,recover)
}
/// 🧾️ Settles one recorded reply: a completed step is debited, a suspension arms the recovery flag.
pub fn settle_cursor(cx:&mut StepContext<'_>,reply:Option<Result<RetainedCloneStep,OriginalRetirementFailure>>,recover:&mut bool)->Result<(),OriginalTurnFailure>{
    match reply{
        Some(result)=>{*recover=false;match result.map_err(OriginalRetirementFailure::into_value_error){Ok(step)=>{cx.consume_retained(step.progress())?;Ok(())},Err(error)=>Err(OriginalTurnFailure::debit_original(error,cx))}},
        None=>{*recover=true;Ok(())},
    }
}
/// 📤️ Moves a completed clone cursor's output into the caller's empty slot; `false` means the quote was not admitted.
pub fn take_cursor<T:RetainedClone,C:RetainedCloneCursor<T>>(cx:&mut StepContext<'_>,cursor:&mut C,output:&mut Option<T>)->Result<bool,OriginalTurnFailure>{
    let Some(reply)=effected(cx,|effect|replied(effect,|effect,reply|cursor.take(output,effect,reply)))? else{return Ok(false)};
    match reply?{
        Some(Ok(step))=>{cx.consume_retained(step.progress())?;Ok(true)},
        Some(Err(error))=>Err(OriginalTurnFailure::debit_original(error,cx)),
        None=>Err(ValueError::invariant("IFC clone cursor output handoff suspended").into()),
    }
}
/// ♻️ One effect-shaped clone-cursor close whose reply is folded into the owner outcome; a suspended reply is recovered on the next turn through `recover`.
pub fn closed_cursor<T:RetainedClone,C:RetainedCloneCursor<T>>(effect:&mut RetainedAllocationEffect<'_>,cursor:&mut C,body:usize,recover:&mut bool)->RetainedWorkOutcome{
    let again=*recover;
    let reply=if again{replied(effect,|effect,reply|cursor.recover_close_into(body,effect,reply))}else{replied(effect,|effect,reply|cursor.close_into(body,effect,reply))};
    let result=match reply{
        Ok(Some(result))=>{*recover=false;result.map_err(OriginalRetirementFailure::into_value_error)},
        Ok(None)=>{*recover=true;Ok(RetainedCloneStep::Progress(Default::default()))},
        Err(error)=>Err(error),
    };
    RetainedWorkOutcome{result,work:effect.performed_work()}
}
