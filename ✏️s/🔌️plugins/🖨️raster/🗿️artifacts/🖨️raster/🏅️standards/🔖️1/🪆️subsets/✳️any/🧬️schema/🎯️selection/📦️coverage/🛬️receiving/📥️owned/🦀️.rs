//! 📥️ Flat DSL coverage rows feed the same granted packed builder as original native arrays.
use super::super::*;
use semio_framework_dsl_record::{DslOwnedReceivingField,OwnedFieldReceiving,FieldValue,receiving::owned::*};
use semio_framework_value::{NativeDecodeControl,RetainedAllocationEffect,RetainedCloneGrant,RetainedWorkOutcome,RetainedWorkPoll,retained_clone::OriginalEffectReply};
use std::mem::ManuallyDrop;

pub struct RasterCoverageFieldReceiving{source:OwnedSlot<FieldValue>,current:OwnedSlot<FieldValue>,row:OwnedSlot<RasterSelectionSpan>,output:OwnedSlot<RasterSelectionSpans>,child:OwnedReceivingSlot<RasterSelectionSpan>,builder:ManuallyDrop<Option<RasterSelectionSpansBuilder>>,index:usize,length:usize,phase:u8,closing:bool,failed:bool}
impl Default for RasterCoverageFieldReceiving{fn default()->Self{Self{source:Default::default(),current:Default::default(),row:Default::default(),output:Default::default(),child:OwnedReceivingSlot::new(),builder:ManuallyDrop::new(None),index:0,length:0,phase:0,closing:false,failed:false}}}
impl DslOwnedReceivingField for RasterSelectionSpans{type Cursor=RasterCoverageFieldReceiving;fn receiving_cursor()->Self::Cursor{Self::Cursor::default()}}
/// 🪢️ Settles one reply-shaped builder close action into an owned closing turn.
fn builder_close(builder:&mut RasterSelectionSpansBuilder,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OwnedClose{
    if !builder.is_closing(){return done(effect,builder.begin_close_paid(effect).map(|step|step.progress()).map_err(Into::into));}
    let mut slot=None;let ran=OriginalEffectReply::borrow(&mut slot).and_then(|reply|builder.close_into(body,effect,reply).map(|_|()));
    match(ran,slot.take()){(Err(error),_)=>done(effect,Err(error.into())),(Ok(()),Some(settled))=>done(effect,settled.map(|step|step.progress())),(Ok(()),None)=>RetainedWorkPoll::Pending{work:effect.performed_work()}}
}
impl RasterCoverageFieldReceiving{
    fn builder(&self)->Result<&RasterSelectionSpansBuilder,ValueError>{self.builder.as_ref().ok_or_else(invariant)}
    fn selected(&mut self,input:&mut Option<FieldValue>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OwnedPoll{
        if self.closing||self.failed{return done(effect,Err(invariant().into()));}
        if self.phase==11{return idle(effect);}
        let demand=match self.demands(input,body){Ok(d)=>d,Err(error)=>return done(effect,Err(error.into()))};
        if !admits(effect,demand){return idle(effect);}
        let g=effect.grant();
        match self.phase{
            2 if !self.child.is_ready()=>{let poll=self.child.advance_local(&mut self.current.value,body,&mut effect.nested(child_grant(g)));return settle(effect,poll);},
            3 if self.row.value.is_none()=>{let poll=self.child.take_into(&mut self.row.value,&mut effect.nested(child_grant(g)));return settle(effect,normal(poll));},
            4 if !self.child.terminal_is_empty()=>{let poll=self.child.close_step(&mut effect.nested(child_grant(g)));return settle(effect,receiving(poll));},
            5=>{let result=match self.builder.as_mut(){Some(builder)=>builder.push(&mut self.row.value,effect),None=>Err(invariant())};if result.is_ok()&&self.row.value.is_none(){self.index+=1;self.phase=if self.index==self.length{7}else if self.builder().is_ok_and(|b|b.accepts_row()){1}else{6};}return done(effect,result.map(|step|step.progress()).map_err(Into::into));},
            6 if !self.builder()?.output_ready()&&!self.builder()?.accepts_row()=>{return match self.builder.as_mut().ok_or_else(invariant).and_then(|builder|builder.advance(effect)){Ok(None)=>RetainedWorkPoll::Pending{work:effect.performed_work()},Ok(Some(settled))=>done(effect,settled.map(|step|step.progress()).map_err(Into::into)),Err(error)=>done(effect,Err(error.into()))};},
            7=>{let result=self.builder.as_mut().ok_or_else(invariant).and_then(|builder|builder.finish(effect));if result.is_ok(){self.phase=6;}return done(effect,result.map(|step|step.progress()).map_err(Into::into));},
            8=>{let result=self.builder.as_mut().ok_or_else(invariant).and_then(|builder|builder.take(effect));return match result{Ok(Some(output))=>{*self.output.value=Some(output);self.phase=9;done(effect,Ok(receipt(demand)))},Ok(None)=>idle(effect),Err(error)=>done(effect,Err(error.into()))};},
            9 if self.builder.as_ref().is_some_and(|builder|!builder.terminal_is_empty())=>{let poll=builder_close(self.builder.as_mut().unwrap(),body,effect);return settle(effect,receiving(poll));},
            10 if !self.source.terminal_is_empty()=>{let poll=self.source.close_step(effect);return settle(effect,receiving(poll));},
            _=>{},
        }
        local(effect,demand,||{match self.phase{
            0=>{let length=match input.as_ref(){Some(FieldValue::List(rows)|FieldValue::Tuple(rows))=>rows.len(),_=>return Err(invalid())};let builder=RasterSelectionSpansBuilder::new(length)?;self.length=length;*self.source.value=input.take();*self.builder=Some(builder);self.phase=if length==0{7}else{1};},
            1=>{let Some(FieldValue::List(rows)|FieldValue::Tuple(rows))=self.source.value.as_mut()else{return Err(invariant())};*self.current.value=Some(std::mem::replace(rows.get_mut(self.index).ok_or_else(invariant)?,FieldValue::Absent));self.phase=2;},
            2=>self.phase=3,3=>self.phase=4,
            4=>{self.child=OwnedReceivingSlot::new();self.phase=5;},
            6=>self.phase=if self.builder()?.output_ready(){8}else{1},
            9=>{drop(self.builder.take());self.phase=10;},10=>self.phase=11,
            _=>return Err(invariant()),
        }Ok(())})
    }
}
impl OwnedFieldReceiving<RasterSelectionSpans> for RasterCoverageFieldReceiving{
    fn demands(&self,_source:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>{
        if self.closing||self.failed{return Err(invariant());}
        match self.phase{
            0=>Ok(payload::<RasterSelectionSpansBuilder>()),1=>Ok(payload::<FieldValue>()),
            2 if !self.child.is_ready()=>self.child.demands(&self.current.value,body).and_then(nested),
            3 if self.row.value.is_none()=>nested(nested(payload::<RasterSelectionSpan>())?),
            4 if !self.child.terminal_is_empty()=>self.child.close_demands(body).and_then(nested),
            4=>Ok(payload::<OwnedReceivingSlot<RasterSelectionSpan>>()),
            5=>self.builder()?.push_demands(),
            6 if !self.builder()?.output_ready()&&!self.builder()?.accepts_row()=>self.builder()?.demands(),
            7=>self.builder()?.finish_demands(),8=>self.builder()?.take_demands(),
            9 if self.builder.as_ref().is_some_and(|builder|!builder.terminal_is_empty())=>{let builder=self.builder()?;if builder.is_closing(){builder.close_demands(body)}else{Ok(builder.begin_close_demand())}},
            9=>Ok(payload::<RasterSelectionSpansBuilder>()),
            10 if !self.source.terminal_is_empty()=>self.source.close_demands(body),
            11=>Ok(Default::default()),_=>Ok(metadata::<()>()),
        }
    }
    fn select_advance(&self,_source:&Option<FieldValue>,body:usize,maximum_work_units:u64)->Result<OwnedAdvanceSelection,ValueError>{
        if self.closing||self.failed{return Err(invariant());}
        let local=|work:u64|Ok(OwnedAdvanceSelection::Local{work});
        match self.phase{
            2 if !self.child.is_ready()=>self.child.select_advance(&self.current.value,body,maximum_work_units),
            3 if self.row.value.is_none()=>local(self.child.take_work_demand()?),
            4 if !self.child.terminal_is_empty()=>local(self.child.close_work_demand(body)?),
            6 if !self.builder()?.output_ready()&&!self.builder()?.accepts_row()=>local(self.builder()?.advance_work_demand()?),
            9 if self.builder.as_ref().is_some_and(|builder|!builder.terminal_is_empty())=>{let builder=self.builder()?;local(if builder.is_closing(){builder.close_work_demand(body)?}else{builder.begin_close_work_demand()})},
            10 if !self.source.terminal_is_empty()=>local(self.source.close_work_demand(body)?),
            11=>local(0),_=>local(1),
        }
    }
    fn advance_nested(&mut self,_source:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->OwnedPoll{if self.failed||self.closing||self.phase!=2||self.child.is_ready(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invariant().into()),work:0});}let poll=self.child.advance_nested(&mut self.current.value,native,body,maximum_work_units,child_grant(g));poison(&mut self.failed,&poll);poll}
    fn advance_local(&mut self,source:&mut Option<FieldValue>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OwnedPoll{let poll=self.selected(source,body,effect);poison(&mut self.failed,&poll);poll}
    fn is_ready(&self)->bool{!self.closing&&!self.failed&&self.phase==11&&self.output.value.is_some()}
    fn take_into(&mut self,output:&mut Option<RasterSelectionSpans>,effect:&mut RetainedAllocationEffect<'_>)->OwnedTake{if !self.is_ready(){return done(effect,Err(invariant()));}move_into(&mut self.output.value,output,effect)}
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.terminal_is_empty(){return Ok(Default::default());}if !self.closing{return Ok(metadata::<()>());}
        if !self.child.terminal_is_empty(){return self.child.close_demands(body).and_then(nested);}
        if !self.current.terminal_is_empty(){return self.current.close_demands(body);}
        if !self.row.terminal_is_empty(){return self.row.close_demands(body);}
        if let Some(builder)=self.builder.as_ref(){return if builder.terminal_is_empty(){Ok(payload::<RasterSelectionSpansBuilder>())}else if builder.is_closing(){builder.close_demands(body)}else{Ok(builder.begin_close_demand())};}
        if !self.output.terminal_is_empty(){return self.output.close_demands(body);}self.source.close_demands(body)
    }
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
        if self.terminal_is_empty(){return Ok(0);}if !self.closing{return Ok(1);}
        if !self.child.terminal_is_empty(){return self.child.close_work_demand(body);}
        if !self.current.terminal_is_empty(){return self.current.close_work_demand(body);}
        if !self.row.terminal_is_empty(){return self.row.close_work_demand(body);}
        if let Some(builder)=self.builder.as_ref(){return if builder.terminal_is_empty(){Ok(1)}else if builder.is_closing(){builder.close_work_demand(body)}else{Ok(builder.begin_close_work_demand())};}
        if !self.output.terminal_is_empty(){return self.output.close_work_demand(body);}self.source.close_work_demand(body)
    }
    fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OwnedClose{
        if self.terminal_is_empty(){return idle(effect);}
        let g=effect.grant();let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return done(effect,Err(error.into()))};if !admits(effect,d){return idle(effect);}
        if !self.closing{return local_close(effect,d,||{self.closing=true;Ok(())});}
        if !self.child.terminal_is_empty(){let poll=self.child.close_step(&mut effect.nested(child_grant(g)));return settle(effect,poll);}
        if !self.current.terminal_is_empty(){return self.current.close_step(effect);}
        if !self.row.terminal_is_empty(){return self.row.close_step(effect);}
        if let Some(builder)=self.builder.as_mut(){if builder.terminal_is_empty(){return local_close(effect,d,||{drop(self.builder.take());Ok(())});}let poll=builder_close(builder,g.maximum_copy_bytes,effect);return settle(effect,poll);}
        if !self.output.terminal_is_empty(){return self.output.close_step(effect);}self.source.close_step(effect)
    }
    fn terminal_is_empty(&self)->bool{self.child.terminal_is_empty()&&self.current.terminal_is_empty()&&self.row.terminal_is_empty()&&self.builder.is_none()&&self.output.terminal_is_empty()&&self.source.terminal_is_empty()}
}
impl Drop for RasterCoverageFieldReceiving{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"packed coverage field receiving retains original rows until funded closure");}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
