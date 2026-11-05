//! 🛬️ Original flat arrays construct one packed page at a time through the caller's native receiving authority.
use super::*;
use semio_framework_value::{NativeDecodeControl,RetainedWorkOutcome,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneRef,retained_clone_grant_admits}};
use semio_framework_value_native_owned::{OriginalReceivingPrimary,owned::{NativeListSource,NativeReceive,NativeReceiver,receiving_capacity}};
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
    fn demands(&self,original:RetainedCloneRef<'_,I>,body:usize)->Result<RetirementDemand,ValueError>{
        if self.closing||matches!(self.phase,Phase::Ready|Phase::Spent){return Err(invalid());}original.check_binding(&self.binding)?;
        match self.phase{
            Phase::Bind=>Ok(fixed(if original.check_binding(&self.binding)?{0}else{original.binding_copy_bytes()})),
            Phase::Count=>Ok(fixed(2*size_of::<usize>())),
            Phase::Initialize=>Ok(fixed(size_of::<RasterSelectionSpansBuilder>())),
            Phase::RowBirth=>Ok(fixed(size_of::<<RasterSelectionSpan as NativeReceive<I::Row>>::Receiver>())),
            Phase::RowAdvance=>if self.child()?.output_ready(){Ok(fixed(0))}else{nested(self.child()?.demands(self.source(original)?,body)?)},
            Phase::RowTake=>nested(self.child()?.take_demand()?),
            Phase::RowClose=>if self.child()?.terminal_is_empty(){Ok(fixed(size_of::<<RasterSelectionSpan as NativeReceive<I::Row>>::Receiver>()))}else{nested(self.child()?.close_demands(body)?)},
            Phase::Push=>self.builder()?.push_demands(),
            Phase::Build=>if self.builder()?.output_ready()||self.builder()?.accepts_row(){Ok(fixed(0))}else{self.builder()?.demands()},
            Phase::Finish=>self.builder()?.finish_demands(),
            Phase::Take=>self.builder()?.take_demands(),
            _=>Err(invalid()),
        }
    }
    fn advance(&mut self,original:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{
        let mut work=0;let result=(||{
            let demand=self.demands(original,body)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}control.checkpoint()?;receiving_capacity(control,demand.capacity_bytes)?;
            match self.phase{
                Phase::Bind=>{if self.binding.is_none(){let turn=original.bind(&mut self.binding,grant);let turn=control.receive_retained_work(grant,RetainedWorkOutcome{result:turn.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())),work:turn.work});work=turn.work;return turn.result.map_err(Into::into);}self.phase=Phase::Count;},
                Phase::Count=>{self.count=original.get().row_count()?;if self.count>16_777_216{return Err(invalid().into());}self.phase=Phase::Initialize;},
                Phase::Initialize=>{*self.builder=Some(RasterSelectionSpansBuilder::new(self.count)?);self.phase=if self.count==0{Phase::Finish}else{Phase::RowBirth};},
                Phase::RowBirth=>{*self.child=Some(<RasterSelectionSpan as NativeReceive<I::Row>>::receiver());self.phase=Phase::RowAdvance;},
                Phase::RowAdvance=>{if self.child()?.output_ready(){self.phase=Phase::RowTake;}else{let source=self.source(original)?;let turn=self.child.as_mut().ok_or_else(invalid)?.advance(source,control,body,child_grant(grant));work=turn.work;return turn.result;}},
                Phase::RowTake=>{let turn=self.child.as_mut().ok_or_else(invalid)?.take(child_grant(grant));work=turn.work;if let Some((row,p))=turn.result?{self.row=Some(row);self.child.as_mut().ok_or_else(invalid)?.begin_close();self.phase=Phase::RowClose;return Ok(RetainedCloneStep::Progress(p));}return Ok(RetainedCloneStep::Progress(Default::default()));},
                Phase::RowClose=>{if !self.child()?.terminal_is_empty(){let turn=self.child.as_mut().ok_or_else(invalid)?.close_step(child_grant(grant));let turn=control.receive_retained_work(grant,turn);work=turn.work;return turn.result;}drop(self.child.take());self.phase=Phase::Push;},
                Phase::Push=>{let turn=self.builder.as_mut().ok_or_else(invalid)?.push(&mut self.row,grant);let turn=control.receive_retained_work(grant,RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work});work=turn.work;if self.row.is_none(){self.index+=1;self.phase=if self.index==self.count{Phase::Finish}else if self.builder()?.accepts_row(){Phase::RowBirth}else{Phase::Build};}return turn.result;},
                Phase::Build=>{if self.builder()?.output_ready(){self.phase=Phase::Take;}else if self.builder()?.accepts_row(){self.phase=Phase::RowBirth;}else{let turn=self.builder.as_mut().ok_or_else(invalid)?.advance(grant);let turn=control.receive_retained_work(grant,turn);work=turn.work;return turn.result;}},
                Phase::Finish=>{let turn=self.builder.as_mut().ok_or_else(invalid)?.finish(grant);let turn=control.receive_retained_work(grant,RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work});work=turn.work;if turn.result.is_ok(){self.phase=Phase::Build;}return turn.result;},
                Phase::Take=>{let turn=self.builder.as_mut().ok_or_else(invalid)?.take(grant);work=turn.work;if let Some((output,p))=turn.result?{*self.output=Some(output);self.phase=Phase::Ready;return Ok(RetainedCloneStep::Complete(p));}return Ok(RetainedCloneStep::Progress(Default::default()));},
                _=>return Err(invalid().into()),
            }
            work=1;Ok(RetainedCloneStep::Progress(receipt(demand)))
        })();RetainedWorkOutcome{result,work}
    }
    fn output_ready(&self)->bool{!self.closing&&self.phase==Phase::Ready&&self.output.is_some()}
    fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid());}Ok(fixed(size_of::<RasterSelectionSpans>()))}
    fn take(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(RasterSelectionSpans,RetainedCloneProgress)>>{let result=(||{let d=self.take_demand()?;if !retained_clone_grant_admits(grant,d){return Ok(None);}self.phase=Phase::Spent;Ok(Some((self.output.take().ok_or_else(invalid)?,receipt(d))))})();let work=u64::from(result.as_ref().is_ok_and(Option::is_some));RetainedWorkOutcome{result,work}}
    fn begin_close(&mut self){self.closing=true;if let Some(child)=self.child.as_mut(){child.begin_close();}if let Some(builder)=self.builder.as_mut(){builder.begin_close();}}
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if let Some(child)=self.child.as_ref(){return if child.terminal_is_empty(){Ok(fixed(size_of::<<RasterSelectionSpan as NativeReceive<I::Row>>::Receiver>()))}else{nested(child.close_demands(body)?)};}
        if let Some(builder)=self.builder.as_ref(){return if builder.terminal_is_empty(){Ok(fixed(size_of::<RasterSelectionSpansBuilder>()))}else{builder.close_demands(body)};}
        if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<RasterSelectionSpans>(false,&None,body);}
        if self.output.is_some(){return self.retirement.next_owner_demands::<RasterSelectionSpans>(true,&None,body);}
        if self.row.is_some(){return Ok(fixed(size_of::<RasterSelectionSpan>()));}
        RetainedCloneBinding::close_demand(&self.binding,body)
    }
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
        let mut work=0;let result=(||{let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
            if let Some(child)=self.child.as_mut(){if !child.terminal_is_empty(){let turn=child.close_step(child_grant(grant));work=turn.work;return turn.result;}drop(self.child.take());work=1;return Ok(RetainedCloneStep::Progress(receipt(d)));}
            if let Some(builder)=self.builder.as_mut(){if !builder.terminal_is_empty(){let turn=builder.close_step(grant);work=turn.work;return turn.result;}drop(self.builder.take());work=1;return Ok(RetainedCloneStep::Progress(receipt(d)));}
            if !self.retirement.is_empty(){let turn=self.retirement.step_granted(grant);work=turn.work;return turn.result;}
            if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,grant);work=turn.work;return turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default())));}
            if self.row.take().is_some(){work=1;return Ok(RetainedCloneStep::Progress(receipt(d)));}
            let turn=RetainedCloneBinding::close_one(&mut self.binding,grant);work=turn.work;turn.result
        })();RetainedWorkOutcome{result,work}
    }
    fn terminal_is_empty(&self)->bool{self.child.is_none()&&self.builder.is_none()&&self.row.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.binding.is_none()}
}
impl<I:NativeListSource+?Sized> Drop for RasterCoverageReceiver<I> where RasterSelectionSpan:NativeReceive<I::Row>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"packed coverage native receiving retains original owners until funded closure");}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

#[path="📥️owned/🦀️.rs"]mod owned;
