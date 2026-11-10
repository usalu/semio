//! 🛬️ Original flat arrays construct one packed page at a time through the caller's native receiving authority.
use super::*;
use semio_framework_value::{NativeDecodeControl,OriginalRetirementCloseLoan,OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneRef,retained_clone_grant_admits}};
use semio_framework_value_native_owned::{NativeReceivingFailure,OriginalEffectReply,OriginalEffectReplyComplete,OriginalReceivingPrimary,owned::{NativeAdvanceSelection,NativeListSource,NativeReceive,NativeReceiver,OriginalSourceTurnWitness,native_close_settle,native_output_demand,native_output_take,native_receiver_close_start,native_record_pure_take}};
use std::mem::ManuallyDrop;

#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Bind,Count,Initialize,RowBirth,RowAdvance,RowTake,RowClose,Push,Build,Finish,Take,Ready,Spent}

pub struct RasterCoverageReceiver<I:NativeListSource+?Sized> where RasterSelectionSpan:NativeReceive<I::Row>{
    binding:Option<RetainedCloneBinding>,builder:ManuallyDrop<Option<RasterSelectionSpansBuilder>>,child:ManuallyDrop<Option<<RasterSelectionSpan as NativeReceive<I::Row>>::Receiver>>,row:Option<RasterSelectionSpan>,output:ManuallyDrop<Option<RasterSelectionSpans>>,retirement:RetainedCloneClose,count:usize,index:usize,phase:Phase,closing:bool,input:std::marker::PhantomData<fn(&I)>,
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"packed coverage receiving requires its live original array owner")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(invalid)?;Ok(demand)}
fn child_grant(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}}

impl<I:NativeListSource+?Sized> RasterCoverageReceiver<I> where RasterSelectionSpan:NativeReceive<I::Row>{
    fn source<'a>(&self,original:RetainedCloneRef<'a,I>)->Result<RetainedCloneRef<'a,I::Row>,ValueError>{original.get().row(self.index)?;let index=self.index;Ok(original.project(index,move|input|input.row(index).expect("validated original span rank")))}
    fn builder(&self)->Result<&RasterSelectionSpansBuilder,ValueError>{self.builder.as_ref().ok_or_else(invalid)}
    fn child(&self)->Result<&<RasterSelectionSpan as NativeReceive<I::Row>>::Receiver,ValueError>{self.child.as_ref().ok_or_else(invalid)}
}
impl<I:NativeListSource+?Sized> NativeReceive<I> for RasterSelectionSpans where RasterSelectionSpan:NativeReceive<I::Row>{
    type Receiver=RasterCoverageReceiver<I>;
    fn receiver()->Self::Receiver{RasterCoverageReceiver{binding:None,builder:ManuallyDrop::new(None),child:ManuallyDrop::new(None),row:None,output:ManuallyDrop::new(None),retirement:Default::default(),count:0,index:0,phase:Phase::Bind,closing:false,input:std::marker::PhantomData}}
}
impl<I:NativeListSource+?Sized> NativeReceiver<I,RasterSelectionSpans> for RasterCoverageReceiver<I> where RasterSelectionSpan:NativeReceive<I::Row>{
    fn demands(&self,original:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
        if self.closing||matches!(self.phase,Phase::Ready|Phase::Spent){return Err(invalid());}original.check_binding(&self.binding)?;
        match self.phase{
            Phase::Bind=>Ok(fixed(if original.check_binding(&self.binding)?{0}else{original.binding_copy_bytes()})),
            Phase::Count=>Ok(fixed(2*size_of::<usize>())),
            Phase::Initialize=>Ok(fixed(size_of::<RasterSelectionSpansBuilder>())),
            Phase::RowBirth=>Ok(fixed(size_of::<<RasterSelectionSpan as NativeReceive<I::Row>>::Receiver>())),
            Phase::RowAdvance=>if self.child()?.output_ready(){Ok(fixed(0))}else{nested(self.child()?.demands(self.source(original)?,body,maximum_work_units)?)},
            Phase::RowTake=>nested(self.child()?.take_demand()?),
            Phase::RowClose=>{let child=self.child()?;if child.terminal_is_empty(){Ok(fixed(size_of::<<RasterSelectionSpan as NativeReceive<I::Row>>::Receiver>()))}else if child.is_closing(){nested(child.close_demands(body)?)}else{nested(child.begin_close_demand()?)}},
            Phase::Push=>self.builder()?.push_demands(),
            Phase::Build=>if self.builder()?.output_ready()||self.builder()?.accepts_row(){Ok(fixed(0))}else{self.builder()?.demands()},
            Phase::Finish=>self.builder()?.finish_demands(),
            Phase::Take=>self.builder()?.take_demands(),
            _=>Err(invalid()),
        }
    }
    fn select_advance(&self,original:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<NativeAdvanceSelection,ValueError>{
        if self.closing||matches!(self.phase,Phase::Ready|Phase::Spent){return Err(invalid());}
        Ok(match self.phase{
            Phase::Bind=>NativeAdvanceSelection::Local{work:if original.check_binding(&self.binding)?{1}else{original.binding_work_demand(&self.binding)?}},
            Phase::RowAdvance=>if self.child()?.output_ready(){NativeAdvanceSelection::Local{work:1}}else{self.child()?.select_advance(self.source(original)?,body,maximum_work_units)?},
            Phase::RowTake=>NativeAdvanceSelection::Local{work:self.child()?.take_work_demand()?},
            Phase::RowClose=>{let child=self.child()?;NativeAdvanceSelection::Local{work:if child.terminal_is_empty(){1}else if child.is_closing(){child.close_work_demand(body)?}else{child.begin_close_work_demand()?}}},
            Phase::Build if !self.builder()?.output_ready()&&!self.builder()?.accepts_row()=>NativeAdvanceSelection::Local{work:self.builder()?.advance_work_demand()?},
            _=>NativeAdvanceSelection::Local{work:1},
        })
    }
    fn advance_nested(&mut self,original:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>{
        let refuse=||RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0});
        if self.closing||self.phase!=Phase::RowAdvance||self.child.as_ref().is_none_or(|child|child.output_ready()){return refuse()}
        let source=match self.source(original){Ok(source)=>source,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};
        self.child.as_mut().map_or_else(refuse,|child|child.advance_nested(source,control,witness,body,maximum_work_units,child_grant(grant)).map(|step|RetainedCloneStep::Progress(step.progress())))
    }
    fn advance_local<'reply>(&mut self,original:RetainedCloneRef<'_,I>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
        if self.phase==Phase::RowAdvance&&!self.closing{let source=self.source(original)?;let child=self.child.as_mut().ok_or_else(invalid)?;if !child.output_ready(){return child.advance_local(source,body,effect,reply)}}
        let g=effect.grant();
        let result:Result<Option<RetainedCloneStep>,OriginalReceivingPrimary>=(||{
            let demand=self.demands(original,body,effect.maximum_work_units())?;if !retained_clone_grant_admits(g,demand){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
            if self.phase==Phase::Bind&&self.binding.is_none(){let turn=original.bind(&mut self.binding,effect);return turn.result.map(|p|Some(RetainedCloneStep::Progress(p.unwrap_or_default()))).map_err(Into::into);}
            match self.phase{
                Phase::RowTake=>{let mut taken=None;let mut slot=None;{let inner=OriginalEffectReply::borrow(&mut slot)?;self.child.as_mut().ok_or_else(invalid)?.take(&mut taken,effect,inner)?;}let Some(step)=slot.take()else{return Ok(None)};let step=step?;if taken.is_some(){self.row=taken;self.phase=Phase::RowClose;}return Ok(Some(RetainedCloneStep::Progress(step.progress())));},
                Phase::RowClose if !self.child()?.terminal_is_empty()=>{let child=self.child.as_mut().ok_or_else(invalid)?;let mut inner=effect.nested(child_grant(g));let poll=if child.is_closing(){native_close_settle(&mut inner,|effect,reply|child.close_step(body,effect,reply))}else{native_close_settle(&mut inner,|effect,reply|child.begin_close_into(effect,reply))};return match poll{RetainedWorkPoll::Pending{..}=>Ok(None),RetainedWorkPoll::Complete(turn)=>turn.result.map(|step|Some(RetainedCloneStep::Progress(step.progress()))).map_err(Into::into)};},
                Phase::Push=>{let step=self.builder.as_mut().ok_or_else(invalid)?.push(&mut self.row,effect)?;if self.row.is_none(){self.index+=1;self.phase=if self.index==self.count{Phase::Finish}else if self.builder()?.accepts_row(){Phase::RowBirth}else{Phase::Build};}return Ok(Some(step));},
                Phase::Build if !self.builder()?.output_ready()&&!self.builder()?.accepts_row()=>{return match self.builder.as_mut().ok_or_else(invalid)?.advance(effect)?{None=>Ok(None),Some(settled)=>settled.map(Some).map_err(Into::into)};},
                Phase::Finish=>{let step=self.builder.as_mut().ok_or_else(invalid)?.finish(effect)?;self.phase=Phase::Build;return Ok(Some(step));},
                Phase::Take=>{return match self.builder.as_mut().ok_or_else(invalid)?.take(effect)?{Some(output)=>{*self.output=Some(output);self.phase=Phase::Ready;Ok(Some(RetainedCloneStep::Complete(receipt(demand))))},None=>Ok(Some(RetainedCloneStep::Progress(Default::default())))};},
                _=>{},
            }
            if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"packed coverage action exceeded its original work").into());}
            match self.phase{
                Phase::Bind=>self.phase=Phase::Count,
                Phase::Count=>{self.count=original.get().row_count()?;if self.count>16_777_216{return Err(invalid().into());}self.phase=Phase::Initialize;},
                Phase::Initialize=>{*self.builder=Some(RasterSelectionSpansBuilder::new(self.count)?);self.phase=if self.count==0{Phase::Finish}else{Phase::RowBirth};},
                Phase::RowBirth=>{*self.child=Some(<RasterSelectionSpan as NativeReceive<I::Row>>::receiver());self.phase=Phase::RowAdvance;},
                Phase::RowAdvance=>self.phase=Phase::RowTake,
                Phase::RowClose=>{drop(self.child.take());self.phase=Phase::Push;},
                Phase::Build=>self.phase=if self.builder()?.output_ready(){Phase::Take}else{Phase::RowBirth},
                _=>return Err(invalid().into()),
            }
            Ok(Some(RetainedCloneStep::Progress(receipt(demand))))
        })();
        Ok(match result{Ok(Some(step))=>reply.complete(Ok(step)),Ok(None)=>reply.pending(),Err(error)=>reply.complete(Err(error))})
    }
    fn advance_pending(&self)->bool{self.child.as_ref().is_some_and(|child|child.advance_pending())}
    fn output_ready(&self)->bool{!self.closing&&self.phase==Phase::Ready&&self.output.is_some()}
    fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid());}native_output_demand::<RasterSelectionSpans>(0)}
    fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand().map(|_|1)}
    fn take<'reply>(&mut self,output:&mut Option<RasterSelectionSpans>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{if !self.output_ready(){return Ok(reply.complete(Err(invalid().into())))}let turn=native_output_take(&mut self.output,output,0,effect);if matches!(&turn,RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(_)),..})){self.phase=Phase::Spent;}native_record_pure_take(turn,reply)}
    native_receiver_close_start!(closing);
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.closing{return Err(invalid());}
        if let Some(child)=self.child.as_ref(){return if child.terminal_is_empty(){Ok(fixed(size_of::<<RasterSelectionSpan as NativeReceive<I::Row>>::Receiver>()))}else if child.is_closing(){nested(child.close_demands(body)?)}else{nested(child.begin_close_demand()?)};}
        if let Some(builder)=self.builder.as_ref(){return if builder.terminal_is_empty(){Ok(fixed(size_of::<RasterSelectionSpansBuilder>()))}else if builder.is_closing(){builder.close_demands(body)}else{Ok(builder.begin_close_demand())};}
        if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<RasterSelectionSpans>(false,&None,body);}
        if self.output.is_some(){return self.retirement.next_owner_demands::<RasterSelectionSpans>(true,&None,body);}
        if self.row.is_some(){return Ok(fixed(size_of::<RasterSelectionSpan>()));}
        RetainedCloneBinding::close_demand(&self.binding,body)
    }
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if let Some(child)=self.child.as_ref(){return if child.terminal_is_empty(){Ok(1)}else if child.is_closing(){child.close_work_demand(body)}else{child.begin_close_work_demand()}}if let Some(builder)=self.builder.as_ref(){return if builder.terminal_is_empty(){Ok(1)}else if builder.is_closing(){builder.close_work_demand(body)}else{Ok(builder.begin_close_work_demand())}}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<RasterSelectionSpans>(false,&None,body)}if self.output.is_some(){return self.retirement.next_owner_work::<RasterSelectionSpans>(true,&None,body)}if self.row.is_some(){return Ok(1)}RetainedCloneBinding::close_work_demand(&self.binding,body)}
    fn close_step<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.close_routed(None,body,effect,reply,false)}
    fn recover_close_step<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.close_routed(None,body,effect,reply,true)}
    fn requires_original_birth(&self)->bool{if let Some(child)=self.child.as_ref(){return !child.terminal_is_empty()&&child.requires_original_birth()}if let Some(builder)=self.builder.as_ref(){return !builder.terminal_is_empty()&&builder.requires_original_birth()}self.retirement.requires_original_birth()}
    fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{if grant.maximum_items==0||grant.maximum_depth==0{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}if let Some(child)=self.child.as_mut(){return child.close_original_birth_step(original,child_grant(grant))}if let Some(builder)=self.builder.as_mut(){return builder.close_original_birth_step(original,grant)}self.retirement.step_original_birth(original,grant)}
    fn requires_original_control(&self)->bool{if let Some(child)=self.child.as_ref(){return !child.terminal_is_empty()&&child.requires_original_control()}if let Some(builder)=self.builder.as_ref(){return !builder.terminal_is_empty()&&builder.requires_original_control()}self.retirement.requires_original_control()}
    fn close_original_step<'reply>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{if self.requires_original_birth(){return Err(ValueError::invariant("settle original birth first"))}self.close_routed(Some(original),body,effect,reply,false)}
    fn recover_close_original_step<'reply>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.close_routed(Some(original),body,effect,reply,true)}
    fn terminal_is_empty(&self)->bool{self.child.is_none()&&self.builder.is_none()&&self.row.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.binding.is_none()}
}
impl<I:NativeListSource+?Sized> RasterCoverageReceiver<I> where RasterSelectionSpan:NativeReceive<I::Row>{
    fn close_routed<'reply>(&mut self,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{
        if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
        let g=effect.grant();let d=self.close_demands(body)?;if !recover&&!retained_clone_grant_admits(g,d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
        if let Some(child)=self.child.as_mut(){
            if child.terminal_is_empty(){if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"packed coverage child release exceeded its original work").into())))}drop(self.child.take());return Ok(reply.complete(Ok(RetainedCloneStep::Progress(receipt(d)))))}
            let mut inner=effect.nested(child_grant(g));
            return match(child.is_closing(),original,recover){(false,_,false)=>child.begin_close_into(&mut inner,reply.progress()),(false,_,true)=>child.recover_begin_close_into(&mut inner,reply.progress()),(true,Some(loan),false)=>child.close_original_step(loan,body,&mut inner,reply.progress()),(true,Some(loan),true)=>child.recover_close_original_step(loan,body,&mut inner,reply.progress()),(true,None,false)=>child.close_step(body,&mut inner,reply.progress()),(true,None,true)=>child.recover_close_step(body,&mut inner,reply.progress())};
        }
        if let Some(builder)=self.builder.as_mut(){
            if builder.terminal_is_empty(){if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"packed coverage builder release exceeded its original work").into())))}drop(self.builder.take());return Ok(reply.complete(Ok(RetainedCloneStep::Progress(receipt(d)))))}
            if !builder.is_closing(){let turn=builder.begin_close_paid(effect);return Ok(reply.complete(turn.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into)))}
            return match(original,recover){(Some(loan),false)=>builder.close_original_into(loan,body,effect,reply.progress()),(Some(loan),true)=>builder.recover_close_original_into(loan,body,effect,reply.progress()),(None,false)=>builder.close_into(body,effect,reply.progress()),(None,true)=>builder.recover_close_into(body,effect,reply.progress())};
        }
        if !self.retirement.is_empty(){return match(original,recover){(Some(loan),false)=>self.retirement.step_original_into(loan,body,effect,reply),(Some(loan),true)=>self.retirement.recover_step_original_into(loan,body,effect,reply),(None,false)=>self.retirement.step_into(body,effect,reply),(None,true)=>self.retirement.recover_step_into(body,effect,reply)}}
        if recover{return Ok(reply.pending())}
        if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,effect);return Ok(reply.complete(turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}
        if self.row.is_some(){if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"packed coverage row release exceeded its original work").into())))}self.row=None;return Ok(reply.complete(Ok(RetainedCloneStep::Progress(receipt(d)))))}
        Ok(match RetainedCloneBinding::close_one(&mut self.binding,body,effect){RetainedWorkPoll::Pending{..}=>reply.pending(),RetainedWorkPoll::Complete(turn)=>reply.complete(turn.result)})
    }
}
impl<I:NativeListSource+?Sized> Drop for RasterCoverageReceiver<I> where RasterSelectionSpan:NativeReceive<I::Row>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"packed coverage native receiving retains original owners until funded closure");}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

#[path="📥️owned/🦀️.rs"]mod owned;
